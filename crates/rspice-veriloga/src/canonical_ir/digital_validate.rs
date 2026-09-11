//! Integrity checks at the digital artifact boundary; never run per event.

use std::collections::{BTreeMap, HashSet};

use super::digital::*;
use super::{
    CANONICAL_IR_SCHEMA_VERSION, CfgTerminator, CfgValueKind, CfgValueType, CompilerPhase,
    DigitalWait, IrDiagnostic, IrValidationResult,
};

fn error(message: impl Into<String>) -> Vec<IrDiagnostic> {
    vec![IrDiagnostic::global_error(CompilerPhase::Artifact, message)]
}

impl CanonicalDigitalPlan {
    pub(super) fn seal(mut self) -> Result<Self, Vec<IrDiagnostic>> {
        self.validate_structure()?;
        self.content_identity = self.compute_identity()?;
        Ok(self)
    }

    /// Validate decoded or edited plans before sharing them with a runtime.
    /// The runtime may then compare the stored identity at constant cost.
    pub fn validate(&self) -> IrValidationResult {
        self.validate_structure()?;
        if self.content_identity != self.compute_identity()? {
            return Err(error("digital plan content identity is stale"));
        }
        Ok(())
    }

    fn compute_identity(&self) -> Result<[u8; 32], Vec<IrDiagnostic>> {
        if self.is_empty() {
            return Ok([0; 32]);
        }
        // Stream the canonical field order into the hash without allocating a
        // second serialized copy of a potentially large elaborated design.
        struct DigestWriter(blake3::Hasher);
        impl std::io::Write for DigestWriter {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                self.0.update(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut writer = DigestWriter(blake3::Hasher::new_derive_key(
            "RSpice canonical digital plan",
        ));
        serde_json::to_writer(
            &mut writer,
            &(
                CANONICAL_IR_SCHEMA_VERSION,
                &self.timing,
                &self.signals,
                &self.processes,
                &self.drivers,
                &self.analog_probes,
            ),
        )
        .map_err(|detail| error(format!("cannot hash digital plan: {detail}")))?;
        // JSON maps all non-finite floats to null. Preserve the exact real
        // constant bits as well, including NaN payloads and signed zero.
        for process in &self.processes {
            for value in &process.function.values {
                if let CfgValueKind::RealConstant(number) = value.kind {
                    writer.0.update(&number.to_bits().to_le_bytes());
                }
            }
        }
        Ok(*writer.0.finalize().as_bytes())
    }

    fn validate_structure(&self) -> IrValidationResult {
        self.timing.validate().map_err(error)?;
        for process in &self.processes {
            process.time_scale.validate().map_err(error)?;
            if process.time_scale.precision_exponent() < self.timing.precision_exponent {
                return Err(error(
                    "digital design precision is coarser than a process's module precision",
                ));
            }
        }
        let mut names = HashSet::new();
        for (index, signal) in self.signals.iter().enumerate() {
            if usize::from(signal.id) != index
                || signal.name.is_empty()
                || !names.insert(&signal.name)
            {
                return Err(error(
                    "digital signals must have dense IDs and unique nonempty names",
                ));
            }
            let valid_width = if signal.kind.is_real() {
                signal.width == 0 && signal.bounds.is_none()
            } else {
                let declared = signal
                    .bounds
                    .map_or(Some(1), |(msb, lsb)| msb.abs_diff(lsb).checked_add(1));
                signal.width > 0
                    && signal.width <= crate::semantic::MAX_DIGITAL_VECTOR_WIDTH
                    && declared == Some(u64::from(signal.width))
            };
            if !valid_width {
                return Err(error(format!(
                    "digital signal '{}' has invalid width or bounds",
                    signal.name
                )));
            }
        }
        for (index, probe) in self.analog_probes.iter().enumerate() {
            if usize::from(probe.id) != index
                || probe.access.is_empty()
                || probe.positive.is_empty()
                || probe.negative.as_ref().is_some_and(|name| name.is_empty())
            {
                return Err(error(
                    "digital analog probes must have dense IDs and nonempty access/net names",
                ));
            }
        }
        let mut drivers = BTreeMap::new();
        let mut driver_counts = vec![0u32; self.signals.len()];
        for driver in &self.drivers {
            let Some(signal) = self.signal(driver.id.signal) else {
                return Err(error("digital driver names an undeclared signal"));
            };
            let count = &mut driver_counts[usize::from(signal.id)];
            if driver.target.signal != signal.id
                || signal.procedurally_assignable
                || driver.id.index != *count
                || self
                    .process(driver.process)
                    .is_none_or(|p| p.kind != DigitalProcessKind::ContinuousAssign)
                || (signal.kind.is_real() && driver.target.select != DigitalWriteSelect::Whole)
                || (signal.kind == DigitalSignalKind::Real(DigitalRealResolution::Single)
                    && *count != 0)
            {
                return Err(error(
                    "digital driver has inconsistent target, index, process, or resolution",
                ));
            }
            *count += 1;
            drivers.insert(driver.id, driver);
        }
        let check_terms = |terms: &[DigitalSensitivityTerm]| {
            if terms.iter().any(|term| self.signal(term.signal).is_none()) {
                Err(error("digital sensitivity names an undeclared signal"))
            } else {
                Ok(())
            }
        };
        let mut written_drivers = HashSet::new();
        for (index, process) in self.processes.iter().enumerate() {
            if usize::from(process.id) != index {
                return Err(error("digital processes must have dense IDs"));
            }
            let function = &process.function;
            function
                .validate()
                .map_err(|detail| error(format!("digital process {}: {detail}", index)))?;
            if !function.block(function.entry).params.is_empty() {
                return Err(error("digital process entry must not take arguments"));
            }
            if let Some(sensitivity) = &process.static_sensitivity {
                check_terms(&sensitivity.terms)?;
            }
            let check_wait = |wait: &DigitalWait| -> IrValidationResult {
                let wait = if let DigitalWait::Repeat { count, event } = wait {
                    if !matches!(
                        function.value(*count).value_type,
                        CfgValueType::FourState { .. }
                    ) {
                        return Err(error(
                            "repeat event count must be normalized four-state data",
                        ));
                    }
                    if !matches!(
                        event.as_ref(),
                        DigitalWait::Event(_) | DigitalWait::Expressions(_)
                    ) {
                        return Err(error("repeat must contain one event control"));
                    }
                    event.as_ref()
                } else {
                    wait
                };
                match wait {
                    DigitalWait::Event(terms) => {
                        if terms.is_empty() {
                            return Err(error("event wait must have sensitivity terms"));
                        }
                        check_terms(terms)
                    }
                    DigitalWait::Expressions(terms) => check_expressions(function, terms),
                    DigitalWait::Delay(value) => {
                        if !matches!(
                            function.value(*value).value_type,
                            CfgValueType::Integer | CfgValueType::FourState { .. }
                        ) {
                            return Err(error(
                                "digital delay must contain converted integer ticks",
                            ));
                        }
                        Ok(())
                    }
                    DigitalWait::Repeat { .. } => unreachable!("nested repeat rejected"),
                }
            };
            for block in &function.blocks {
                match &block.terminator {
                    CfgTerminator::Wait { wait, .. } => check_wait(wait)?,
                    CfgTerminator::Branch { condition, .. } => {
                        if !matches!(
                            function.value(*condition).value_type,
                            CfgValueType::FourState { .. }
                        ) {
                            return Err(error("digital branch must have a four-state condition"));
                        }
                    }
                    _ => {}
                }
            }
            for value in &function.values {
                let kind = &value.kind;
                if !kind.is_digital()
                    && !matches!(
                        kind,
                        CfgValueKind::RealConstant(_) | CfgValueKind::BlockParameter
                    )
                {
                    return Err(error("analog value kind in digital process"));
                }
                if matches!(
                    value.value_type,
                    CfgValueType::Boolean | CfgValueType::Lanes(_)
                ) {
                    return Err(error("analog value type in digital process"));
                }
                match kind {
                    CfgValueKind::DigitalRepeatCount { input, .. } => {
                        let expected = match function.value(*input).value_type {
                            CfgValueType::FourState { width } => CfgValueType::FourState { width },
                            CfgValueType::Integer | CfgValueType::Real => {
                                CfgValueType::FourState { width: 32 }
                            }
                            _ => return Err(error("repeat count has the wrong value domain")),
                        };
                        if value.value_type != expected {
                            return Err(error("repeat count has the wrong output width"));
                        }
                    }
                    CfgValueKind::DigitalDelayTicks { input, .. } => {
                        if value.value_type != (CfgValueType::FourState { width: 64 })
                            || !matches!(
                                function.value(*input).value_type,
                                CfgValueType::Integer
                                    | CfgValueType::Real
                                    | CfgValueType::FourState { .. }
                            )
                        {
                            return Err(error(
                                "digital delay conversion has the wrong value domain or width",
                            ));
                        }
                    }
                    CfgValueKind::DigitalBitSelect {
                        input,
                        index,
                        bounds,
                        ..
                    } => {
                        let width = bounds.0.abs_diff(bounds.1).checked_add(1);
                        if value.value_type != (CfgValueType::FourState { width: 1 })
                            || !matches!(
                                function.value(*index).value_type,
                                CfgValueType::FourState { .. } | CfgValueType::Integer
                            )
                            || !matches!(function.value(*input).value_type, CfgValueType::FourState { width: w } if Some(u64::from(w)) == width)
                        {
                            return Err(error(
                                "digital bit select has inconsistent input, index or declared bounds",
                            ));
                        }
                    }
                    CfgValueKind::DigitalIntegerToReal { input, .. } => {
                        if value.value_type != CfgValueType::Real
                            || !matches!(
                                function.value(*input).value_type,
                                CfgValueType::Integer | CfgValueType::FourState { .. }
                            )
                        {
                            return Err(error(
                                "integer-to-real conversion has inconsistent value domains",
                            ));
                        }
                    }
                    CfgValueKind::DigitalRealToInteger { input, width } => {
                        if *width == 0
                            || *width > crate::semantic::MAX_DIGITAL_VECTOR_WIDTH
                            || value.value_type != (CfgValueType::FourState { width: *width })
                            || function.value(*input).value_type != CfgValueType::Real
                        {
                            return Err(error(
                                "real-to-integer conversion has inconsistent value domains or width",
                            ));
                        }
                    }
                    CfgValueKind::DigitalTime { query } => {
                        if value.value_type != query.value_type() {
                            return Err(error(
                                "digital time query has the wrong value domain or width",
                            ));
                        }
                    }
                    CfgValueKind::DigitalSignalRead { signal }
                    | CfgValueKind::DigitalRealSignalRead { signal } => {
                        let Some(signal) = self.signal(*signal) else {
                            return Err(error("digital read names an undeclared signal"));
                        };
                        let expected = if signal.kind.is_real() {
                            CfgValueType::Real
                        } else {
                            CfgValueType::FourState {
                                width: signal.width,
                            }
                        };
                        if value.value_type != expected
                            || signal.kind.is_real()
                                != matches!(kind, CfgValueKind::DigitalRealSignalRead { .. })
                        {
                            return Err(error(
                                "digital signal read has the wrong value domain or width",
                            ));
                        }
                    }
                    CfgValueKind::DigitalAnalogPotential { probe }
                        if self.analog_probe(*probe).is_none() =>
                    {
                        return Err(error("digital read names an undeclared analog probe"));
                    }
                    CfgValueKind::DigitalBlockingWrite { target, .. }
                    | CfgValueKind::DigitalNonblockingWrite { target, .. } => {
                        if self
                            .signal(target.signal)
                            .is_none_or(|signal| !signal.procedurally_assignable)
                        {
                            return Err(error(
                                "digital procedural write must target a declared variable",
                            ));
                        }
                        if let CfgValueKind::DigitalNonblockingWrite {
                            wait: Some(wait), ..
                        } = kind
                        {
                            check_wait(wait)?;
                        }
                        if let CfgValueKind::DigitalNonblockingWrite { region, .. } = kind
                            && *region != DigitalSchedulingRegion::NonBlockingAssign
                        {
                            return Err(error(
                                "digital nonblocking write has the wrong scheduling region",
                            ));
                        }
                    }
                    CfgValueKind::DigitalDriverWrite { driver, target, .. } => {
                        if drivers.get(driver).is_none_or(|declared| {
                            declared.process != process.id || declared.target != *target
                        }) {
                            return Err(error(
                                "digital driver write does not match its declaration",
                            ));
                        }
                        written_drivers.insert(*driver);
                    }
                    _ => {}
                }
            }
        }
        if written_drivers.len() != self.drivers.len() {
            return Err(error("digital driver has no corresponding write"));
        }
        Ok(())
    }
}

/// Trace only the expression graph. A process's writes and resume parameters
/// must never be replayed while observing an event. Uses an explicit stack so
/// source-controlled expression depth cannot exhaust the host stack.
pub(crate) fn event_expression_schedule(
    function: &super::CfgFunction,
    root: super::ids::ValueId,
) -> Result<(Vec<super::ids::ValueId>, Vec<super::ids::DigitalSignalId>), String> {
    let mut states = vec![0u8; function.values.len()];
    let mut stack = vec![(root, false)];
    let mut order = Vec::new();
    let mut dependencies = std::collections::BTreeSet::new();
    while let Some((id, finish)) = stack.pop() {
        let index = usize::from(id);
        let Some(value) = function.values.get(index) else {
            return Err("event expression names an absent value".into());
        };
        if finish {
            states[index] = 2;
            order.push(id);
            continue;
        }
        match states[index] {
            2 => continue,
            1 => return Err("event expression contains a value cycle".into()),
            _ => {}
        }
        if value.value_type == CfgValueType::Effect
            || matches!(
                value.kind,
                CfgValueKind::BlockParameter | CfgValueKind::DigitalAnalogPotential { .. }
            )
            || !(value.kind.is_digital() || matches!(value.kind, CfgValueKind::RealConstant(_)))
        {
            return Err(
                "event expression needs event bindings for process-local or analog-owned storage"
                    .into(),
            );
        }
        if let CfgValueKind::DigitalSignalRead { signal }
        | CfgValueKind::DigitalRealSignalRead { signal } = value.kind
        {
            dependencies.insert(signal);
        }
        states[index] = 1;
        stack.push((id, true));
        for operand in value.kind.operands().into_iter().rev() {
            stack.push((operand, false));
        }
    }
    Ok((order, dependencies.into_iter().collect()))
}

fn check_expressions(
    function: &super::CfgFunction,
    terms: &[DigitalEventExpression],
) -> IrValidationResult {
    if terms.is_empty() {
        return Err(error("event expression list must not be empty"));
    }
    for term in terms {
        event_expression_schedule(function, term.value).map_err(error)?;
        if term.edge.is_some() && function.value(term.value).value_type == CfgValueType::Real {
            return Err(error("edge event expression must have a bit-valued result"));
        }
    }
    Ok(())
}

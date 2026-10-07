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

/// Validate source bounds independently from normalized linked storage.
fn packed_write_type(
    function: &super::CfgFunction,
    signal: &DigitalSignal,
    bounds: (i64, i64),
    select: DigitalArrayWriteSelect,
) -> Result<CfgValueType, Vec<IrDiagnostic>> {
    if !signal.procedurally_assignable {
        return Err(error(
            "digital packed write must target a declared variable",
        ));
    }
    if !signal.kind.is_real() {
        let authored = super::VectorBounds {
            msb: bounds.0,
            lsb: bounds.1,
        };
        let actual = signal.declared_range();
        let normalized = super::VectorBounds {
            msb: i64::from(signal.width) - 1,
            lsb: 0,
        };
        if authored.width() != signal.width || (actual != authored && actual != normalized) {
            return Err(error(
                "digital packed write has inconsistent authored bounds",
            ));
        }
    }
    packed_selection_type(
        function,
        signal.kind.is_real(),
        signal.width,
        bounds,
        select,
    )
}

/// Selection typing shared by stored writes and pure four-state updates.
fn packed_selection_type(
    function: &super::CfgFunction,
    real: bool,
    width: u32,
    bounds: (i64, i64),
    select: DigitalArrayWriteSelect,
) -> Result<CfgValueType, Vec<IrDiagnostic>> {
    Ok(match select {
        DigitalArrayWriteSelect::Whole => {
            if real {
                CfgValueType::Real
            } else {
                CfgValueType::FourState { width }
            }
        }
        DigitalArrayWriteSelect::Bit { index, .. } => {
            if real
                || !matches!(
                    function.value(index).value_type,
                    CfgValueType::Real | CfgValueType::Integer | CfgValueType::FourState { .. }
                )
            {
                return Err(error(
                    "digital bit write requires integral storage and a numeric selector",
                ));
            }
            CfgValueType::FourState { width: 1 }
        }
        DigitalArrayWriteSelect::Part { msb, lsb } => {
            let width = DigitalWriteSelect::Part { msb, lsb }.checked_width(super::VectorBounds {
                msb: bounds.0,
                lsb: bounds.1,
            });
            if real || width.is_none() {
                return Err(error(
                    "digital packed part write has an invalid element type, width or direction",
                ));
            }
            CfgValueType::FourState {
                width: width.unwrap(),
            }
        }
    })
}

/// Fixed procedural/driver selections are part of the artifact, not SSA data.
fn fixed_write_type(
    signal: &DigitalSignal,
    select: &DigitalWriteSelect,
) -> Result<CfgValueType, Vec<IrDiagnostic>> {
    if signal.kind.is_real() {
        return if *select == DigitalWriteSelect::Whole {
            Ok(CfgValueType::Real)
        } else {
            Err(error(
                "digital fixed write cannot select bits of real storage",
            ))
        };
    }
    let width = select
        .checked_width(signal.declared_range())
        .ok_or_else(|| error("digital fixed write has an invalid selection width or direction"))?;
    Ok(CfgValueType::FourState { width })
}

fn check_fixed_write(
    signal: &DigitalSignal,
    select: &DigitalWriteSelect,
    effect: CfgValueType,
    rhs: CfgValueType,
) -> IrValidationResult {
    let expected = fixed_write_type(signal, select)?;
    // Fixed writes resize four-state/32-bit integer data at execution. Real
    // conversion is explicit in the CFG and cannot be inferred from a target.
    let valid_rhs = if expected == CfgValueType::Real {
        rhs == CfgValueType::Real
    } else {
        matches!(rhs, CfgValueType::Integer)
            || matches!(rhs, CfgValueType::FourState { width }
                if width > 0 && width <= crate::semantic::MAX_DIGITAL_VECTOR_WIDTH)
    };
    if effect != CfgValueType::Effect || !valid_rhs {
        return Err(error(
            "digital fixed write has an invalid effect or RHS type",
        ));
    }
    Ok(())
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
                &self.arrays,
                &self.processes,
                &self.drivers,
                &self.analog_probes,
            ),
        )
        .map_err(|detail| error(format!("cannot hash digital plan: {detail}")))?;
        // JSON maps all non-finite floats to null. Preserve the exact real
        // constant bits as well, including NaN payloads and signed zero.
        for process in &self.processes {
            for function in process_functions(&process.function) {
                for value in &function.values {
                    if let CfgValueKind::RealConstant(number) = value.kind {
                        writer.0.update(&number.to_bits().to_le_bytes());
                    }
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
        let mut local_declarations = HashSet::new();
        for (index, signal) in self.signals.iter().enumerate() {
            if usize::from(signal.id) != index
                || signal.name.is_empty()
                || !names.insert(&signal.name)
            {
                return Err(error(
                    "digital signals must have dense IDs and unique nonempty names",
                ));
            }
            if let Some(local) = &signal.local {
                if !signal.procedurally_assignable
                    || local.name.is_empty()
                    || self.process(local.process).is_none()
                    || (local.element.is_none()
                        && !local_declarations.insert((local.process, local.declaration)))
                {
                    return Err(error(
                        "digital local storage requires a unique declaration and valid owning process",
                    ));
                }
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
            if signal.integer
                && (signal.kind.is_real()
                    || !signal.signed
                    || !signal.procedurally_assignable
                    || signal.width != 32
                    || signal.bounds != Some((31, 0)))
            {
                return Err(error(format!(
                    "digital integer '{}' must be a signed [31:0] variable",
                    signal.name
                )));
            }
            if let Some(value) = &signal.initial_value {
                let valid = signal.procedurally_assignable
                    && match value {
                        super::digital::DigitalInitialValue::FourState(value) => {
                            !signal.kind.is_real() && value.width() == signal.width
                        }
                        super::digital::DigitalInitialValue::Real(value) => {
                            signal.kind.is_real() && value.is_finite()
                        }
                    };
                if !valid {
                    return Err(error(format!(
                        "digital signal '{}' has an incompatible declaration initializer",
                        signal.name
                    )));
                }
            }
            if !valid_width {
                return Err(error(format!(
                    "digital signal '{}' has invalid width or bounds",
                    signal.name
                )));
            }
        }
        let mut arrays = HashSet::new();
        let mut array_cells = vec![false; self.signals.len()];
        for array in &self.arrays {
            let range = array
                .storage
                .cell_range()
                .ok_or_else(|| error("digital array has invalid storage extent"))?;
            if array.name.is_empty()
                || !names.insert(&array.name)
                || array.bounds.0.min(array.bounds.1) != array.storage.lower
                || array.bounds.0.abs_diff(array.bounds.1).checked_add(1)
                    != Some(u64::from(array.storage.len))
                || !arrays.insert(array.storage)
            {
                return Err(error(
                    "digital arrays require unique names, storage and matching declared bounds",
                ));
            }
            let first = self
                .signal(array.storage.base)
                .ok_or_else(|| error("digital array names absent storage"))?;
            if let Some(local) = &first.local {
                if !local_declarations.insert((local.process, local.declaration)) {
                    return Err(error(
                        "digital local arrays require unique declaration identities",
                    ));
                }
            }
            for slot in range {
                let cell = self
                    .signal(super::ids::DigitalSignalId::new(slot))
                    .ok_or_else(|| error("digital array exceeds declared signal storage"))?;
                let occupied = &mut array_cells[slot as usize];
                let index = array.storage.lower + i64::from(slot - array.storage.base.index());
                let local_matches = match (&first.local, &cell.local) {
                    (None, None) => true,
                    (Some(first), Some(local)) => {
                        local.process == first.process
                            && local.declaration == first.declaration
                            && local.name == first.name
                            && local.element == Some(index)
                    }
                    _ => false,
                };
                if !local_matches {
                    return Err(error(
                        "digital array cells have inconsistent local ownership or indices",
                    ));
                }
                if *occupied
                    || !cell.procedurally_assignable
                    || cell.name != format!("{}[{index}]", array.name)
                    || (
                        cell.kind,
                        cell.width,
                        cell.bounds,
                        cell.signed,
                        cell.integer,
                    ) != (
                        first.kind,
                        first.width,
                        first.bounds,
                        first.signed,
                        first.integer,
                    )
                {
                    return Err(error(
                        "digital array cells must be distinct, consistently named variables of one element type",
                    ));
                }
                *occupied = true;
            }
        }
        if self.signals.iter().enumerate().any(|(slot, signal)| {
            signal
                .local
                .as_ref()
                .is_some_and(|local| local.element.is_some())
                && !array_cells[slot]
        }) {
            return Err(error("digital local array element has no owning array"));
        }
        for (index, probe) in self.analog_probes.iter().enumerate() {
            if usize::from(probe.id) != index
                || probe.access.is_empty()
                || match &probe.target {
                    super::digital::DigitalAnalogProbeTarget::Nodes { positive, negative } => {
                        positive.is_empty() || negative.as_ref().is_some_and(|name| name.is_empty())
                    }
                    super::digital::DigitalAnalogProbeTarget::Branch { name }
                    | super::digital::DigitalAnalogProbeTarget::Variable { name } => {
                        name.is_empty()
                    }
                }
            {
                return Err(error(
                    "digital analog probes must have dense IDs and nonempty access/net names",
                ));
            }
        }
        for probe in &self.analog_probes {
            use super::digital::{DigitalAnalogProbeTarget, DigitalAnalogQuantity};
            let variable = matches!(
                probe.quantity,
                DigitalAnalogQuantity::RealVariable | DigitalAnalogQuantity::IntegerVariable
            );
            if variable != matches!(probe.target, DigitalAnalogProbeTarget::Variable { .. }) {
                return Err(error("analog read quantity does not match its target kind"));
            }
        }
        let mut drivers = BTreeMap::new();
        let mut driver_counts = vec![0u32; self.signals.len()];
        for driver in &self.drivers {
            let Some(signal) = self.signal(driver.id.signal) else {
                return Err(error("digital driver names an undeclared signal"));
            };
            fixed_write_type(signal, &driver.target.select)?;
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
            } else if terms
                .iter()
                .any(|term| term.edge.is_some() && self.signal(term.signal).unwrap().kind.is_real())
            {
                Err(error("digital edge sensitivity requires integral storage"))
            } else {
                Ok(())
            }
        };
        let mut written_drivers = HashSet::new();
        for (index, process) in self.processes.iter().enumerate() {
            if usize::from(process.id) != index {
                return Err(error("digital processes must have dense IDs"));
            }
            for function in process_functions(&process.function) {
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
                                return Err(error(
                                    "digital branch must have a four-state condition",
                                ));
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
                        CfgValueKind::DigitalExpression {
                            function: expression,
                            result,
                        } => {
                            expression
                                .validate()
                                .map_err(|detail| error(format!("event expression: {detail}")))?;
                            expression_dependencies(expression, *result).map_err(error)?;
                            if value.value_type != expression.value(*result).value_type {
                                return Err(error(
                                    "event expression result has the wrong value domain or width",
                                ));
                            }
                        }
                        CfgValueKind::DigitalRealSelect {
                            condition,
                            then_value,
                            else_value,
                        } => {
                            if value.value_type != CfgValueType::Real
                                || !matches!(
                                    function.value(*condition).value_type,
                                    CfgValueType::FourState { .. }
                                )
                                || function.value(*then_value).value_type != CfgValueType::Real
                                || function.value(*else_value).value_type != CfgValueType::Real
                            {
                                return Err(error("real selection has inconsistent value domains"));
                            }
                        }
                        CfgValueKind::DigitalRepeatCount { input, .. } => {
                            let expected = match function.value(*input).value_type {
                                CfgValueType::FourState { width } => {
                                    CfgValueType::FourState { width }
                                }
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
                                    CfgValueType::FourState { .. }
                                        | CfgValueType::Integer
                                        | CfgValueType::Real
                                )
                                || !matches!(function.value(*input).value_type, CfgValueType::FourState { width: w } if Some(u64::from(w)) == width)
                            {
                                return Err(error(
                                    "digital bit select has inconsistent input, index or declared bounds",
                                ));
                            }
                        }
                        CfgValueKind::DigitalPackedUpdate {
                            input,
                            bounds,
                            select,
                            value: rhs,
                        } => {
                            let width = bounds
                                .0
                                .abs_diff(bounds.1)
                                .checked_add(1)
                                .filter(|width| {
                                    *width <= u64::from(crate::semantic::MAX_DIGITAL_VECTOR_WIDTH)
                                })
                                .ok_or_else(|| {
                                    error("digital packed update has invalid input bounds")
                                })? as u32;
                            let input_type = CfgValueType::FourState { width };
                            if function.value(*input).value_type != input_type
                                || value.value_type != input_type
                            {
                                return Err(error(
                                    "digital packed update has inconsistent input/output type or bounds",
                                ));
                            }
                            let selected =
                                packed_selection_type(function, false, width, *bounds, *select)?;
                            if function.value(*rhs).value_type != selected {
                                return Err(error(
                                    "digital packed update has an incompatible replacement type",
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
                        CfgValueKind::DigitalArrayRead { array, index, .. }
                        | CfgValueKind::DigitalArrayBlockingWrite { array, index, .. }
                        | CfgValueKind::DigitalArrayNonblockingWrite { array, index, .. } => {
                            if !arrays.contains(array) {
                                return Err(error(
                                    "digital array operation must name a declared array's complete storage",
                                ));
                            }
                            if !matches!(
                                function.value(*index).value_type,
                                CfgValueType::Real
                                    | CfgValueType::Integer
                                    | CfgValueType::FourState { .. }
                            ) {
                                return Err(error(
                                    "digital array operation has an invalid index type",
                                ));
                            }
                            let signal = self.signal(array.base).expect("validated array storage");
                            let element_type = if signal.kind.is_real() {
                                CfgValueType::Real
                            } else {
                                CfgValueType::FourState {
                                    width: signal.width,
                                }
                            };
                            match kind {
                                CfgValueKind::DigitalArrayRead { .. } => {
                                    if value.value_type != element_type {
                                        return Err(error(
                                            "digital array read has the wrong element type",
                                        ));
                                    }
                                }
                                CfgValueKind::DigitalArrayBlockingWrite {
                                    value: rhs,
                                    select,
                                    bounds,
                                    ..
                                }
                                | CfgValueKind::DigitalArrayNonblockingWrite {
                                    value: rhs,
                                    select,
                                    bounds,
                                    ..
                                } => {
                                    let expected =
                                        packed_write_type(function, signal, *bounds, *select)?;
                                    if value.value_type != CfgValueType::Effect
                                        || function.value(*rhs).value_type != expected
                                    {
                                        return Err(error(
                                            "digital array write has the wrong effect or element type",
                                        ));
                                    }
                                }
                                _ => unreachable!(),
                            }
                            if let CfgValueKind::DigitalArrayNonblockingWrite {
                                region, wait, ..
                            } = kind
                            {
                                if *region != DigitalSchedulingRegion::NonBlockingAssign {
                                    return Err(error(
                                        "digital array nonblocking write has the wrong scheduling region",
                                    ));
                                }
                                if let Some(wait) = wait {
                                    check_wait(wait)?;
                                }
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
                        CfgValueKind::DigitalAnalogVariable { probe, array_index } => {
                            use super::digital::DigitalAnalogQuantity;
                            let Some(declaration) = self.analog_probe(*probe) else {
                                return Err(error(
                                    "digital read names an undeclared analog variable",
                                ));
                            };
                            let expected = match declaration.quantity {
                                DigitalAnalogQuantity::RealVariable => CfgValueType::Real,
                                DigitalAnalogQuantity::IntegerVariable => {
                                    CfgValueType::FourState { width: 32 }
                                }
                                _ => {
                                    return Err(error(
                                        "digital variable read names a physical probe",
                                    ));
                                }
                            };
                            if value.value_type != expected {
                                return Err(error(
                                    "digital analog variable read has the wrong type",
                                ));
                            }
                            if let Some(selection) = array_index {
                                if selection.len == 0
                                    || selection.len > 65_536
                                    || selection
                                        .lower
                                        .checked_add(i64::from(selection.len) - 1)
                                        .is_none()
                                    || !matches!(
                                        function.value(selection.index).value_type,
                                        CfgValueType::Real
                                            | CfgValueType::Integer
                                            | CfgValueType::FourState { .. }
                                    )
                                {
                                    return Err(error(
                                        "digital analog array read has an invalid index or extent",
                                    ));
                                }
                                let DigitalAnalogProbeTarget::Variable { name } =
                                    &declaration.target
                                else {
                                    return Err(error(
                                        "digital analog array read requires variable probes",
                                    ));
                                };
                                let suffix = format!("[{}]", selection.lower);
                                let Some(name) = name.strip_suffix(&suffix) else {
                                    return Err(error(
                                        "digital analog array read has an inconsistent lower bound",
                                    ));
                                };
                                let base = usize::from(*probe);
                                let Some(end) = base.checked_add(selection.len as usize) else {
                                    return Err(error(
                                        "digital analog array probe range overflows",
                                    ));
                                };
                                let Some(probes) = self.analog_probes.get(base..end) else {
                                    return Err(error(
                                        "digital analog array read exceeds its probe table",
                                    ));
                                };
                                for (offset, element) in probes.iter().enumerate() {
                                    let expected_name =
                                        format!("{name}[{}]", selection.lower + offset as i64);
                                    if element.quantity != declaration.quantity
                                        || !matches!(&element.target, DigitalAnalogProbeTarget::Variable { name } if name == &expected_name)
                                    {
                                        return Err(error(
                                            "digital analog array read has inconsistent element bindings",
                                        ));
                                    }
                                }
                            }
                        }
                        CfgValueKind::DigitalAnalogPotential { probe }
                        | CfgValueKind::DigitalAnalogFlow { probe } => {
                            let Some(probe) = self.analog_probe(*probe) else {
                                return Err(error("digital read names an undeclared analog probe"));
                            };
                            let expected =
                                if matches!(kind, CfgValueKind::DigitalAnalogPotential { .. }) {
                                    super::digital::DigitalAnalogQuantity::Potential
                                } else {
                                    super::digital::DigitalAnalogQuantity::Flow
                                };
                            if probe.quantity != expected || value.value_type != CfgValueType::Real
                            {
                                return Err(error(
                                    "digital analog read has the wrong physical quantity or value type",
                                ));
                            }
                        }
                        CfgValueKind::DigitalBitBlockingWrite {
                            signal,
                            index,
                            signed,
                            bounds,
                            value: rhs,
                        }
                        | CfgValueKind::DigitalBitNonblockingWrite {
                            signal,
                            index,
                            signed,
                            bounds,
                            value: rhs,
                            ..
                        } => {
                            let Some(signal) = self.signal(*signal) else {
                                return Err(error("digital bit write names an undeclared signal"));
                            };
                            let expected = packed_write_type(
                                function,
                                signal,
                                *bounds,
                                DigitalArrayWriteSelect::Bit {
                                    index: *index,
                                    signed: *signed,
                                },
                            )?;
                            if value.value_type != CfgValueType::Effect
                                || function.value(*rhs).value_type != expected
                            {
                                return Err(error(
                                    "digital bit write has the wrong effect or value type",
                                ));
                            }
                            if let CfgValueKind::DigitalBitNonblockingWrite {
                                region, wait, ..
                            } = kind
                            {
                                if *region != DigitalSchedulingRegion::NonBlockingAssign {
                                    return Err(error(
                                        "digital bit nonblocking write has the wrong scheduling region",
                                    ));
                                }
                                if let Some(wait) = wait {
                                    check_wait(wait)?;
                                }
                            }
                        }
                        CfgValueKind::DigitalBlockingWrite { target, value: rhs }
                        | CfgValueKind::DigitalNonblockingWrite {
                            target, value: rhs, ..
                        } => {
                            let signal = self
                                .signal(target.signal)
                                .filter(|signal| signal.procedurally_assignable)
                                .ok_or_else(|| {
                                    error(
                                        "digital procedural write must target a declared variable",
                                    )
                                })?;
                            check_fixed_write(
                                signal,
                                &target.select,
                                value.value_type,
                                function.value(*rhs).value_type,
                            )?;
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
                        CfgValueKind::DigitalDriverWrite {
                            driver,
                            target,
                            value: rhs,
                        } => {
                            if drivers.get(driver).is_none_or(|declared| {
                                declared.process != process.id || declared.target != *target
                            }) {
                                return Err(error(
                                    "digital driver write does not match its declaration",
                                ));
                            }
                            let signal = self.signal(target.signal).ok_or_else(|| {
                                error("digital driver names an undeclared signal")
                            })?;
                            check_fixed_write(
                                signal,
                                &target.select,
                                value.value_type,
                                function.value(*rhs).value_type,
                            )?;
                            written_drivers.insert(*driver);
                        }
                        _ => {}
                    }
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
                CfgValueKind::BlockParameter
                    | CfgValueKind::DigitalArrayBlockingWrite { .. }
                    | CfgValueKind::DigitalBitBlockingWrite { .. }
                    | CfgValueKind::DigitalBitNonblockingWrite { .. }
                    | CfgValueKind::DigitalArrayNonblockingWrite { .. }
                    | CfgValueKind::DigitalAnalogPotential { .. }
                    | CfgValueKind::DigitalAnalogFlow { .. }
                    | CfgValueKind::DigitalAnalogVariable { .. }
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
        if let CfgValueKind::DigitalArrayRead { array, .. } = value.kind {
            let range = array
                .cell_range()
                .ok_or("event expression has invalid array storage")?;
            dependencies.extend(range.map(super::ids::DigitalSignalId::new));
        }
        if let CfgValueKind::DigitalExpression { function, result } = &value.kind {
            function.validate().map_err(|error| error.to_string())?;
            dependencies.extend(expression_dependencies(function, *result)?);
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

/// Embedded expression functions have one level of independent SSA scope.
/// Nested embedded functions are rejected before walking their contents.
fn process_functions(function: &super::CfgFunction) -> impl Iterator<Item = &super::CfgFunction> {
    std::iter::once(function).chain(
        function
            .values
            .iter()
            .filter_map(|value| match &value.kind {
                CfgValueKind::DigitalExpression { function, .. } => Some(function.as_ref()),
                _ => None,
            }),
    )
}

/// Validate pure, acyclic observation and collect every possible signal input,
/// including inputs of branches that are currently inactive. Selection changes
/// can make any of them relevant on the next observation.
fn expression_dependencies(
    function: &super::CfgFunction,
    result: super::ids::ValueId,
) -> Result<Vec<super::ids::DigitalSignalId>, String> {
    let Some(output) = function.values.get(usize::from(result)) else {
        return Err("event expression names an absent result".into());
    };
    if !matches!(
        output.value_type,
        CfgValueType::Real | CfgValueType::Integer | CfgValueType::FourState { .. }
    ) {
        return Err("event expression must return scalar data".into());
    }
    if !function.block(function.entry).params.is_empty() {
        return Err("event expression must not capture process arguments".into());
    }
    let mut dependencies = std::collections::BTreeSet::new();
    for value in &function.values {
        if matches!(
            value.kind,
            CfgValueKind::DigitalExpression { .. }
                | CfgValueKind::DigitalArrayBlockingWrite { .. }
                | CfgValueKind::DigitalBitBlockingWrite { .. }
                | CfgValueKind::DigitalBitNonblockingWrite { .. }
                | CfgValueKind::DigitalArrayNonblockingWrite { .. }
                | CfgValueKind::DigitalBlockingWrite { .. }
                | CfgValueKind::DigitalNonblockingWrite { .. }
                | CfgValueKind::DigitalDriverWrite { .. }
                | CfgValueKind::DigitalAnalogPotential { .. }
                | CfgValueKind::DigitalAnalogFlow { .. }
                | CfgValueKind::DigitalAnalogVariable { .. }
        ) || !matches!(
            value.value_type,
            CfgValueType::Real | CfgValueType::Integer | CfgValueType::FourState { .. }
        ) || !(value.kind.is_digital()
            || matches!(
                value.kind,
                CfgValueKind::RealConstant(_) | CfgValueKind::BlockParameter
            ))
        {
            return Err("event expression must be pure and needs supported event bindings".into());
        }
        if let CfgValueKind::DigitalSignalRead { signal }
        | CfgValueKind::DigitalRealSignalRead { signal } = value.kind
        {
            dependencies.insert(signal);
        }
        if let CfgValueKind::DigitalArrayRead { array, .. } = value.kind {
            let range = array
                .cell_range()
                .ok_or("event expression has invalid array storage")?;
            dependencies.extend(range.map(super::ids::DigitalSignalId::new));
        }
    }
    let mut incoming = vec![0usize; function.blocks.len()];
    let mut returns = 0;
    let mut definition = None;
    for block in &function.blocks {
        match &block.terminator {
            CfgTerminator::Return => returns += 1,
            CfgTerminator::Jump { .. } | CfgTerminator::Branch { .. } => {}
            _ => return Err("event expression must not suspend".into()),
        }
        if block.params.contains(&result) || block.instructions.iter().any(|i| i.result == result) {
            definition = Some(block.id);
        }
        for successor in block.successors() {
            incoming[usize::from(successor)] += 1;
        }
    }
    let mut ready: Vec<_> = incoming
        .iter()
        .enumerate()
        .filter_map(|(i, &n)| (n == 0).then_some(super::ids::BlockId::from(i)))
        .collect();
    let mut visited = 0;
    while let Some(block) = ready.pop() {
        visited += 1;
        for successor in function.block(block).successors() {
            let count = &mut incoming[usize::from(successor)];
            *count -= 1;
            if *count == 0 {
                ready.push(successor);
            }
        }
    }
    if visited != function.blocks.len() || returns == 0 {
        return Err("event expression control flow must be acyclic and return a value".into());
    }
    // Every return must have passed the result's definition. Constants may be
    // unplaced leaves; all other results have a defining block checked above.
    if let Some(definition) = definition {
        let mut seen = vec![false; function.blocks.len()];
        let mut pending = vec![function.entry];
        while let Some(block) = pending.pop() {
            if block == definition || std::mem::replace(&mut seen[usize::from(block)], true) {
                continue;
            }
            if matches!(function.block(block).terminator, CfgTerminator::Return) {
                return Err("event expression result is not defined on every return path".into());
            }
            pending.extend(function.block(block).successors());
        }
    } else if !matches!(
        output.kind,
        CfgValueKind::RealConstant(_)
            | CfgValueKind::IntegerConstant(_)
            | CfgValueKind::FourStateConstant(_)
    ) {
        return Err("event expression result has no definition".into());
    }
    Ok(dependencies.into_iter().collect())
}

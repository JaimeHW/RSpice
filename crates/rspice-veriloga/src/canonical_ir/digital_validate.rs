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
            for block in &function.blocks {
                match &block.terminator {
                    CfgTerminator::Wait {
                        wait: DigitalWait::Event(terms),
                        ..
                    } => check_terms(terms)?,
                    CfgTerminator::Wait {
                        wait: DigitalWait::Delay(value),
                        ..
                    } => {
                        if !matches!(
                            function.value(*value).value_type,
                            CfgValueType::Integer | CfgValueType::FourState { .. }
                        ) {
                            return Err(error(
                                "digital delay must have an integer or four-state operand",
                            ));
                        }
                    }
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

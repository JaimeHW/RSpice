use super::*;

impl Engine {
    /// Run .STEP parametric sweep
    ///
    /// Executes multiple simulations with different parameter values.
    /// Returns all results indexed by step values.
    pub fn run_step(
        &self,
        netlist: &Netlist,
        param_name: &str,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        validate_step_values(values)?;

        let mut results = Vec::with_capacity(values.len());
        let mut any_binding = false;

        for &value in values {
            let (modified_netlist, rebuilt) =
                Self::create_perturbed_netlist(netlist, param_name, value)?;
            any_binding |= rebuilt > 0;

            match self.run_dc_op(&modified_netlist) {
                Ok(result) => results.push((value, result)),
                Err(e) => return Err(step_point_error("PARAM", param_name, value, e)),
            }
        }

        if netlist.source_text.is_some() && !any_binding {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        Ok(results)
    }

    /// Run `.STEP` command execution for PARAM/DEVICE/MODEL targets.
    pub fn run_step_command(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        match step_cmd.target {
            StepTarget::Param => self.run_step(netlist, &step_cmd.name, values),
            StepTarget::Device => self.run_step_device(
                netlist,
                &step_cmd.name,
                step_cmd.param_name.as_deref(),
                values,
            ),
            StepTarget::Model => self.run_step_model(
                netlist,
                &step_cmd.name,
                step_cmd.param_name.as_deref(),
                values,
            ),
            StepTarget::Temp => Err(SimulationError::Circuit(
                "Engine `.STEP TEMP` execution is handled via temperature-configured runs"
                    .to_string(),
            )),
        }
    }

    pub(in crate::engine::advanced) fn run_step_device(
        &self,
        netlist: &Netlist,
        device_name: &str,
        param_name: Option<&str>,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        validate_step_values(values)?;

        let device_idx = netlist
            .elements
            .iter()
            .position(|e| e.name.eq_ignore_ascii_case(device_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP DEVICE target '{}' not found in netlist",
                    device_name
                ))
            })?;

        let mut results = Vec::with_capacity(values.len());
        for &value in values {
            let mut stepped = netlist.clone();
            let element = stepped.elements.get_mut(device_idx).ok_or_else(|| {
                SimulationError::Circuit("Internal step device index error".to_string())
            })?;
            Self::apply_device_step_value(&mut element.kind, param_name, value)?;

            match self.run_dc_op(&stepped) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    let target = format!(
                        "{}{}",
                        device_name,
                        param_name.map(|p| format!(".{}", p)).unwrap_or_default()
                    );
                    return Err(step_point_error("DEVICE", &target, value, e));
                }
            }
        }

        Ok(results)
    }

    pub(in crate::engine::advanced) fn run_step_model(
        &self,
        netlist: &Netlist,
        model_name: &str,
        param_name: Option<&str>,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        validate_step_values(values)?;

        let param_name = param_name.ok_or_else(|| {
            SimulationError::Circuit(format!(
                ".STEP MODEL {} requires an explicit parameter name",
                model_name
            ))
        })?;

        let model_idx = netlist
            .models
            .iter()
            .position(|m| m.name.eq_ignore_ascii_case(model_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP MODEL target '{}' not found in netlist",
                    model_name
                ))
            })?;

        let param_upper = param_name.to_ascii_uppercase();
        let mut results = Vec::with_capacity(values.len());
        for &value in values {
            let mut stepped = netlist.clone();
            let model = stepped.models.get_mut(model_idx).ok_or_else(|| {
                SimulationError::Circuit("Internal step model index error".to_string())
            })?;

            if let Some((_, v)) = model
                .params
                .iter_mut()
                .find(|(name, _)| name.eq_ignore_ascii_case(&param_upper))
            {
                *v = value;
            } else {
                model.params.push((param_upper.clone(), value));
            }

            match self.run_dc_op(&stepped) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    let target = format!("{}.{}", model_name, param_upper);
                    return Err(step_point_error("MODEL", &target, value, e));
                }
            }
        }

        Ok(results)
    }

    pub(in crate::engine::advanced) fn apply_device_step_value(
        kind: &mut ElementKind,
        param_name: Option<&str>,
        value: Value,
    ) -> Result<(), SimulationError> {
        let param_upper = param_name.map(|p| p.trim().to_ascii_uppercase());
        let matches_param = |aliases: &[&str]| -> bool {
            match &param_upper {
                None => true,
                Some(name) => aliases.iter().any(|alias| name.eq_ignore_ascii_case(alias)),
            }
        };

        match kind {
            ElementKind::Resistor {
                value: r,
                model,
                instance_params,
                ..
            } => {
                if !matches_param(&["R", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported resistor step parameter; use R or VALUE".to_string(),
                    ));
                }
                *r = value;
                if model.is_some() {
                    if let Some((_, existing)) = instance_params
                        .iter_mut()
                        .find(|(name, _)| name.eq_ignore_ascii_case("R"))
                    {
                        *existing = value;
                    } else {
                        instance_params.push(("R".to_string(), value));
                    }
                }
                Ok(())
            }
            ElementKind::Capacitor { value: c, .. } => {
                if !matches_param(&["C", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported capacitor step parameter; use C or VALUE".to_string(),
                    ));
                }
                *c = value;
                Ok(())
            }
            ElementKind::Inductor { value: l, .. }
            | ElementKind::JilesAthertonInductor { value: l, .. } => {
                if !matches_param(&["L", "VALUE", "INDUCTANCE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported inductor step parameter; use L or VALUE".to_string(),
                    ));
                }
                *l = value;
                Ok(())
            }
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                if !matches_param(&["DC", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported source step parameter; use DC or VALUE".to_string(),
                    ));
                }
                Self::set_source_dc_value(spec, value)
            }
            ElementKind::Vcvs { gain, .. } | ElementKind::Cccs { gain, .. } => {
                if !matches_param(&["GAIN", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported controlled-source step parameter; use GAIN".to_string(),
                    ));
                }
                *gain = value;
                Ok(())
            }
            ElementKind::Vccs {
                transconductance, ..
            } => {
                if !matches_param(&["GM", "TRANSCONDUCTANCE", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported VCCS step parameter; use GM".to_string(),
                    ));
                }
                *transconductance = value;
                Ok(())
            }
            ElementKind::Ccvs {
                transresistance, ..
            } => {
                if !matches_param(&["RM", "TRANSRESISTANCE", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported CCVS step parameter; use RM".to_string(),
                    ));
                }
                *transresistance = value;
                Ok(())
            }
            ElementKind::Coupling { coefficient, .. } => {
                if !matches_param(&["K", "COUPLING", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported coupling step parameter; use K".to_string(),
                    ));
                }
                *coefficient = value;
                Ok(())
            }
            ElementKind::TransmissionLine {
                z0, td, freq, nl, ..
            } => {
                match param_upper.as_deref() {
                    None => *z0 = Some(value),
                    Some("Z0") | Some("VALUE") => *z0 = Some(value),
                    Some("TD") => *td = Some(value),
                    Some("F") | Some("FREQ") => *freq = Some(value),
                    Some("NL") => *nl = Some(value),
                    Some(other) => {
                        return Err(SimulationError::Circuit(format!(
                            "Unsupported transmission-line step parameter '{}' (use Z0, TD, FREQ, NL)",
                            other
                        )));
                    }
                }
                Ok(())
            }
            _ => Err(SimulationError::Circuit(
                "Unsupported .STEP DEVICE target for this element type".to_string(),
            )),
        }
    }

    pub(in crate::engine::advanced) fn set_source_dc_value(
        spec: &mut SourceSpec,
        value: Value,
    ) -> Result<(), SimulationError> {
        match spec {
            SourceSpec::Dc(v) => {
                *v = value;
                Ok(())
            }
            SourceSpec::DcAc { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            SourceSpec::DcTransient { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            SourceSpec::DcAcTransient { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            _ => Err(SimulationError::Circuit(
                "Stepping source VALUE/DC is supported for DC, DC+AC, DC+transient, and DC+AC+transient source definitions only"
                    .to_string(),
            )),
        }
    }
}

fn step_point_error(
    target_kind: &str,
    target_name: &str,
    value: Value,
    error: SimulationError,
) -> SimulationError {
    SimulationError::Circuit(format!(
        ".STEP {target_kind} {target_name} = {value} failed: {error}"
    ))
}

fn validate_step_values(values: &[Value]) -> Result<(), SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::Circuit(
            ".STEP produced no sweep values".to_string(),
        ));
    }

    if let Some((index, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(SimulationError::Circuit(format!(
            ".STEP value at index {index} must be finite, got {value}"
        )));
    }

    Ok(())
}

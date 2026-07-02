use super::*;
use crate::netlist::{ModelDef, StepSweep};

#[derive(Clone, Copy)]
enum DeviceStepResolution {
    Device(usize),
    Model(usize),
}

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
        if matches!(step_cmd.sweep, StepSweep::Data { .. }) {
            let stepped_netlists = self.step_netlists_for_command(netlist, step_cmd, values)?;
            let mut results = Vec::with_capacity(stepped_netlists.len());
            for (index, stepped_netlist) in stepped_netlists {
                match self.run_dc_op(&stepped_netlist) {
                    Ok(result) => results.push((index, result)),
                    Err(e) => return Err(step_point_error("DATA", &step_cmd.name, index, e)),
                }
            }
            return Ok(results);
        }

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
            StepTarget::Temp => self.run_step_temp(netlist, values),
        }
    }

    pub(crate) fn step_netlists_for_command(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        values: &[Value],
    ) -> Result<Vec<(Value, Netlist)>, SimulationError> {
        if let StepSweep::Data { table_name } = &step_cmd.sweep {
            return Self::data_step_netlists_for_table(netlist, table_name);
        }

        validate_step_values(values)?;

        let mut stepped = Vec::with_capacity(values.len());
        let mut any_binding = false;
        for &value in values {
            let (netlist, bindings) =
                Self::step_netlist_for_command_value(netlist, step_cmd, value)?;
            any_binding |= bindings > 0;
            stepped.push((value, netlist));
        }

        if step_cmd.target == StepTarget::Param && netlist.source_text.is_some() && !any_binding {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                step_cmd.name
            )));
        }

        Ok(stepped)
    }

    fn data_step_netlists_for_table(
        netlist: &Netlist,
        table_name: &str,
    ) -> Result<Vec<(Value, Netlist)>, SimulationError> {
        let table = netlist
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case(table_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(".STEP DATA table '{table_name}' not found"))
            })?;
        if table.params.is_empty() {
            return Err(SimulationError::Circuit(format!(
                ".STEP DATA table '{}' has no parameter columns",
                table.name
            )));
        }
        if table.rows.is_empty() {
            return Err(SimulationError::Circuit(format!(
                ".STEP DATA table '{}' has no rows",
                table.name
            )));
        }

        let mut stepped = Vec::with_capacity(table.rows.len());
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != table.params.len() {
                return Err(SimulationError::Circuit(format!(
                    ".STEP DATA table '{}' row {} has {} value(s), expected {}",
                    table.name,
                    row_index,
                    row.len(),
                    table.params.len()
                )));
            }
            if let Some(value) = row.iter().find(|value| !value.is_finite()) {
                return Err(SimulationError::Circuit(format!(
                    ".STEP DATA table '{}' row {} contains non-finite value {}",
                    table.name, row_index, value
                )));
            }

            let overrides = table
                .params
                .iter()
                .cloned()
                .zip(row.iter().copied())
                .collect::<Vec<_>>();
            let (netlist, _) = Self::create_perturbed_netlist_multi(netlist, &overrides)?;
            stepped.push((row_index as Value, netlist));
        }

        Ok(stepped)
    }

    fn step_netlist_for_command_value(
        netlist: &Netlist,
        step_cmd: &StepCommand,
        value: Value,
    ) -> Result<(Netlist, usize), SimulationError> {
        match step_cmd.target {
            StepTarget::Param => Self::create_perturbed_netlist(netlist, &step_cmd.name, value),
            StepTarget::Device => {
                let mut stepped = netlist.clone();
                match Self::resolve_device_or_model_step_target(
                    &stepped,
                    &step_cmd.name,
                    step_cmd.param_name.as_deref(),
                )? {
                    DeviceStepResolution::Device(device_idx) => {
                        let element = stepped.elements.get_mut(device_idx).ok_or_else(|| {
                            SimulationError::Circuit("Internal step device index error".to_string())
                        })?;
                        Self::apply_device_step_value(
                            &mut element.kind,
                            step_cmd.param_name.as_deref(),
                            value,
                        )?;
                    }
                    DeviceStepResolution::Model(model_idx) => {
                        let param_name = step_cmd.param_name.as_deref().ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                ".STEP MODEL {} requires an explicit parameter name",
                                step_cmd.name
                            ))
                        })?;
                        let model = stepped.models.get_mut(model_idx).ok_or_else(|| {
                            SimulationError::Circuit("Internal step model index error".to_string())
                        })?;
                        Self::apply_model_step_value(model, param_name, value);
                    }
                }
                Self::mark_ast_stepped_netlist(&mut stepped);
                Ok((stepped, 1))
            }
            StepTarget::Model => {
                let param_name = step_cmd.param_name.as_deref().ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        ".STEP MODEL {} requires an explicit parameter name",
                        step_cmd.name
                    ))
                })?;

                let mut stepped = netlist.clone();
                let model = Self::find_step_model_mut(&mut stepped, &step_cmd.name)?;
                Self::apply_model_step_value(model, param_name, value);
                Self::mark_ast_stepped_netlist(&mut stepped);
                Ok((stepped, 1))
            }
            StepTarget::Temp => Self::step_temperature_netlist(netlist, value),
        }
    }

    fn step_temperature_netlist(
        netlist: &Netlist,
        value: Value,
    ) -> Result<(Netlist, usize), SimulationError> {
        let vt = thermal_voltage_celsius(value);
        let overrides = [
            ("TEMP".to_string(), value),
            ("TEMPER".to_string(), value),
            ("VT".to_string(), vt),
        ];
        let (mut stepped, bindings) = Self::create_perturbed_netlist_multi(netlist, &overrides)?;
        apply_temperature_scalars(&mut stepped, value, vt);
        Ok((stepped, bindings.max(1)))
    }

    pub(in crate::engine::advanced) fn run_step_temp(
        &self,
        netlist: &Netlist,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        if values.is_empty() {
            return Ok(Vec::new());
        }

        let mut results = Vec::with_capacity(values.len());
        for &value in values {
            let (stepped, _) = Self::step_temperature_netlist(netlist, value)?;

            match self.run_dc_op(&stepped) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    log::warn!("Step TEMP = {} failed: {}", value, e);
                }
            }
        }

        Ok(results)
    }

    pub(in crate::engine::advanced) fn run_step_device(
        &self,
        netlist: &Netlist,
        device_name: &str,
        param_name: Option<&str>,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        validate_step_values(values)?;

        let resolved = Self::resolve_device_or_model_step_target(netlist, device_name, param_name)?;

        let mut results = Vec::with_capacity(values.len());
        for &value in values {
            let mut stepped = netlist.clone();
            let target = format!(
                "{}{}",
                device_name,
                param_name.map(|p| format!(".{}", p)).unwrap_or_default()
            );
            match resolved {
                DeviceStepResolution::Device(device_idx) => {
                    let element = stepped.elements.get_mut(device_idx).ok_or_else(|| {
                        SimulationError::Circuit("Internal step device index error".to_string())
                    })?;
                    Self::apply_device_step_value(&mut element.kind, param_name, value)
                        .map_err(|e| step_point_error("DEVICE", &target, value, e))?;
                }
                DeviceStepResolution::Model(model_idx) => {
                    let param_name = param_name.ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            ".STEP MODEL {device_name} requires an explicit parameter name"
                        ))
                    })?;
                    let model = stepped.models.get_mut(model_idx).ok_or_else(|| {
                        SimulationError::Circuit("Internal step model index error".to_string())
                    })?;
                    Self::apply_model_step_value(model, param_name, value);
                }
            }
            Self::mark_ast_stepped_netlist(&mut stepped);

            match self.run_dc_op(&stepped) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
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

        let mut results = Vec::with_capacity(values.len());
        for &value in values {
            let mut stepped = netlist.clone();
            let model = Self::find_step_model_mut(&mut stepped, model_name)?;
            Self::apply_model_step_value(model, param_name, value);
            Self::mark_ast_stepped_netlist(&mut stepped);

            match self.run_dc_op(&stepped) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    let target = format!("{}.{}", model_name, param_name.to_ascii_uppercase());
                    return Err(step_point_error("MODEL", &target, value, e));
                }
            }
        }

        Ok(results)
    }

    fn resolve_device_or_model_step_target(
        netlist: &Netlist,
        target_name: &str,
        param_name: Option<&str>,
    ) -> Result<DeviceStepResolution, SimulationError> {
        if let Some(device_idx) = netlist
            .elements
            .iter()
            .position(|e| e.name.eq_ignore_ascii_case(target_name))
        {
            return Ok(DeviceStepResolution::Device(device_idx));
        }

        if param_name.is_some()
            && let Some(model_idx) = netlist
                .models
                .iter()
                .position(|m| m.name.eq_ignore_ascii_case(target_name))
        {
            return Ok(DeviceStepResolution::Model(model_idx));
        }

        Err(SimulationError::Circuit(format!(
            ".STEP DEVICE target '{}' not found in netlist",
            target_name
        )))
    }

    fn find_step_model_mut<'a>(
        netlist: &'a mut Netlist,
        model_name: &str,
    ) -> Result<&'a mut ModelDef, SimulationError> {
        netlist
            .models
            .iter_mut()
            .find(|model| model.name.eq_ignore_ascii_case(model_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP MODEL target '{}' not found in netlist",
                    model_name
                ))
            })
    }

    fn apply_model_step_value(model: &mut ModelDef, param_name: &str, value: Value) {
        let param_upper = param_name.to_ascii_uppercase();
        if let Some((_, existing)) = model
            .params
            .iter_mut()
            .find(|(name, _)| name.eq_ignore_ascii_case(&param_upper))
        {
            *existing = value;
        } else {
            model.params.push((param_upper, value));
        }
    }

    fn mark_ast_stepped_netlist(netlist: &mut Netlist) {
        netlist.source_text = None;
        netlist.source_path = None;
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
        let set_instance_param =
            |instance_params: &mut Vec<(String, Value)>, name: &str, value: Value| {
                if let Some((_, existing)) = instance_params
                    .iter_mut()
                    .find(|(param, _)| param.eq_ignore_ascii_case(name))
                {
                    *existing = value;
                } else {
                    instance_params.push((name.to_ascii_uppercase(), value));
                }
            };
        let set_instance_param_alias = |instance_params: &mut Vec<(String, Value)>,
                                        aliases: &[&str],
                                        canonical_name: &str,
                                        value: Value| {
            if let Some((name, existing)) = instance_params.iter_mut().find(|(param, _)| {
                aliases
                    .iter()
                    .any(|alias| param.eq_ignore_ascii_case(alias))
            }) {
                *name = canonical_name.to_ascii_uppercase();
                *existing = value;
            } else {
                instance_params.push((canonical_name.to_ascii_uppercase(), value));
            }
        };

        match kind {
            ElementKind::Resistor {
                value: r,
                model,
                instance_params,
                ..
            } => {
                match param_upper.as_deref() {
                    None | Some("R") | Some("VALUE") => {
                        *r = value;
                        if model.is_some() {
                            set_instance_param(instance_params, "R", value);
                        }
                    }
                    Some(param_name) => {
                        set_instance_param(instance_params, param_name, value);
                    }
                }
                Ok(())
            }
            ElementKind::Capacitor {
                value: c,
                initial_voltage,
                instance_params,
                ..
            } => {
                match param_upper.as_deref() {
                    None | Some("C") | Some("CAP") | Some("VALUE") | Some("CAPACITANCE") => {
                        *c = value;
                        set_instance_param_alias(
                            instance_params,
                            &["C", "CAP", "VALUE", "CAPACITANCE"],
                            "C",
                            value,
                        );
                    }
                    Some("IC") => {
                        *initial_voltage = Some(value);
                        set_instance_param(instance_params, "IC", value);
                    }
                    Some("L") | Some("LENGTH") => {
                        set_instance_param_alias(instance_params, &["L", "LENGTH"], "L", value);
                    }
                    Some("W") | Some("WIDTH") => {
                        set_instance_param_alias(instance_params, &["W", "WIDTH"], "W", value);
                    }
                    Some("M") | Some("MULT") => {
                        set_instance_param_alias(instance_params, &["M", "MULT"], "M", value);
                    }
                    Some("SCALE") | Some("TEMP") | Some("DTEMP") | Some("TC1") | Some("TC2") => {
                        set_instance_param(
                            instance_params,
                            param_upper.as_deref().expect("param is present"),
                            value,
                        );
                    }
                    Some(_) => {
                        return Err(SimulationError::Circuit(
                            "Unsupported capacitor step parameter; use C, VALUE, CAP, CAPACITANCE, M, MULT, SCALE, L, W, TEMP, DTEMP, TC1, TC2, or IC"
                                .to_string(),
                        ));
                    }
                }
                Ok(())
            }
            ElementKind::Inductor {
                value: l,
                model,
                instance_params,
                ..
            } => {
                match param_upper.as_deref() {
                    None | Some("L") | Some("IND") | Some("VALUE") | Some("INDUCTANCE") => {
                        *l = value;
                        if model.is_some() {
                            set_instance_param(instance_params, "L", value);
                        }
                    }
                    Some("M") | Some("MULT") | Some("SCALE") | Some("TEMP") | Some("DTEMP")
                    | Some("TC1") | Some("TC2") => {
                        set_instance_param(
                            instance_params,
                            param_upper.as_deref().expect("param is present"),
                            value,
                        );
                    }
                    Some(_) => {
                        return Err(SimulationError::Circuit(
                            "Unsupported inductor step parameter; use L, VALUE, M, MULT, SCALE, TEMP, DTEMP, TC1, or TC2"
                                .to_string(),
                        ));
                    }
                }
                Ok(())
            }
            ElementKind::JilesAthertonInductor { value: l, .. } => {
                if !matches_param(&["L", "VALUE", "INDUCTANCE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported Jiles-Atherton inductor step parameter; use L or VALUE"
                            .to_string(),
                    ));
                }
                *l = value;
                Ok(())
            }
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                Self::set_source_step_value(spec, param_upper.as_deref(), value)
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

    fn set_source_step_value(
        spec: &mut SourceSpec,
        param_name: Option<&str>,
        value: Value,
    ) -> Result<(), SimulationError> {
        match param_name {
            None | Some("DC") | Some("VALUE") => Self::set_source_dc_value(spec, value),
            Some("VHI" | "VLO" | "TD" | "TR" | "TF" | "TSAMPLE") => {
                Self::set_pat_source_parameter(spec, param_name.unwrap(), value)
            }
            Some(other) => Err(SimulationError::Circuit(format!(
                "Unsupported source step parameter '{other}'; use DC, VALUE, or PAT parameters VHI, VLO, TD, TR, TF, TSAMPLE"
            ))),
        }
    }

    fn set_pat_source_parameter(
        spec: &mut SourceSpec,
        param_name: &str,
        value: Value,
    ) -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "PAT source step parameter '{param_name}' must be finite, got {value}"
            )));
        }
        match spec {
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::set_pat_source_parameter(transient, param_name, value)
            }
            SourceSpec::Pat {
                vhi,
                vlo,
                delay,
                rise,
                fall,
                sample,
                ..
            } => {
                match param_name {
                    "VHI" => *vhi = value,
                    "VLO" => *vlo = value,
                    "TD" => *delay = value,
                    "TR" => {
                        if value <= 0.0 {
                            return Err(SimulationError::Circuit(
                                "PAT TR must be positive".to_string(),
                            ));
                        }
                        *rise = value;
                    }
                    "TF" => {
                        if value <= 0.0 {
                            return Err(SimulationError::Circuit(
                                "PAT TF must be positive".to_string(),
                            ));
                        }
                        *fall = value;
                    }
                    "TSAMPLE" => {
                        if value <= 0.0 {
                            return Err(SimulationError::Circuit(
                                "PAT TSAMPLE must be positive".to_string(),
                            ));
                        }
                        *sample = value;
                    }
                    _ => unreachable!("validated PAT parameter"),
                }
                Ok(())
            }
            _ => Err(SimulationError::Circuit(format!(
                "Source step parameter '{param_name}' requires a PAT source"
            ))),
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

fn apply_temperature_scalars(netlist: &mut Netlist, temp_c: Value, vt: Value) {
    netlist.options.temp = Some(temp_c);
    netlist.params.set("TEMP", temp_c);
    netlist.params.set("TEMPER", temp_c);
    netlist.params.set("VT", vt);
}

fn thermal_voltage_celsius(temp_c: Value) -> Value {
    crate::constants::thermal_voltage(crate::analysis::temperature::celsius_to_kelvin(temp_c))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{SimulationConfig, SpiceDialect};
    use crate::netlist::{AnalysisCommand, Netlist};

    fn first_step_command(netlist: &Netlist) -> &StepCommand {
        netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step),
                _ => None,
            })
            .expect("step command parsed")
    }

    #[test]
    fn device_step_updates_capacitor_multiplicity_parameter() {
        let netlist = Netlist::parse(
            "\
capacitor m step
V1 in 0 0
C3 in 0 C=32u M=1.25
.STEP C3:M 1.25 2.5 1.25
.END
",
        )
        .expect("deck parses");
        let step = first_step_command(&netlist);
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });

        let stepped = engine
            .step_netlists_for_command(&netlist, step, &[2.5])
            .expect("capacitor multiplicity step materializes");
        let stepped_netlist = &stepped[0].1;
        let capacitance = engine
            .resolved_capacitor_value(stepped_netlist, "C3")
            .expect("capacitance resolves")
            .expect("C3 exists");

        assert!(
            (capacitance - 80.0e-6).abs() < 1.0e-18,
            "resolved capacitance {capacitance}"
        );
    }

    #[test]
    fn device_step_capacitor_value_replaces_existing_alias() {
        let netlist = Netlist::parse(
            "\
capacitor value alias step
V1 in 0 0
C3 in 0 C=32u M=2
.STEP C3:VALUE 16u 16u 1u
.END
",
        )
        .expect("deck parses");
        let step = first_step_command(&netlist);
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });

        let stepped = engine
            .step_netlists_for_command(&netlist, step, &[16.0e-6])
            .expect("capacitor value step materializes");
        let stepped_netlist = &stepped[0].1;
        let capacitance = engine
            .resolved_capacitor_value(stepped_netlist, "C3")
            .expect("capacitance resolves")
            .expect("C3 exists");

        assert!(
            (capacitance - 32.0e-6).abs() < 1.0e-18,
            "resolved capacitance {capacitance}"
        );
    }
}

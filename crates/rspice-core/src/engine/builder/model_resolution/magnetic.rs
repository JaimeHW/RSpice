use super::*;

pub(in crate::engine::builder) fn positive_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    param_label: &str,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        if !value.is_finite() || value <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Jiles-Atherton model '{}' has invalid {}={} (must be finite and > 0)",
                model_def.name, param_label, value
            )));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub(in crate::engine::builder) fn unit_interval_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    param_label: &str,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(SimulationError::Circuit(format!(
                "Jiles-Atherton model '{}' has invalid {}={} (must be finite and within [0, 1])",
                model_def.name, param_label, value
            )));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub(in crate::engine::builder) fn nonnegative_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    param_label: &str,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        if !value.is_finite() || value < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Jiles-Atherton model '{}' has invalid {}={} (must be finite and >= 0)",
                model_def.name, param_label, value
            )));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub(in crate::engine::builder) fn resolve_jiles_atherton_model_params(
    model_def: &crate::netlist::ModelDef,
    nominal_inductance: f64,
) -> Result<crate::device::passive::JilesAthertonParams, SimulationError> {
    let mut params = crate::device::passive::JilesAthertonParams::default();
    let mut explicit_turns = false;

    if let Some(ms) = positive_model_param(model_def, &["MS"], "MS")? {
        params.ms = ms;
    }
    if let Some(a) = positive_model_param(model_def, &["A"], "A")? {
        params.a = a;
    }
    if let Some(k) = positive_model_param(model_def, &["K"], "K")? {
        params.k = k;
    }
    if let Some(c) = unit_interval_model_param(model_def, &["C"], "C")? {
        params.c = c;
    }
    if let Some(alpha) = nonnegative_model_param(model_def, &["ALPHA"], "ALPHA")? {
        params.alpha = alpha;
    }
    if let Some(area) = positive_model_param(model_def, &["AREA", "ACORE", "COREAREA"], "AREA")? {
        params.area = area;
    }
    if let Some(length) = positive_model_param(model_def, &["LENGTH", "LEN", "PATHLEN"], "LENGTH")?
    {
        params.length = length;
    }
    if let Some(n_turns) = positive_model_param(model_def, &["N", "NT", "NTURNS", "TURNS"], "N")? {
        params.n_turns = n_turns;
        explicit_turns = true;
    }

    if !explicit_turns && nominal_inductance.is_finite() && nominal_inductance > 0.0 {
        let base = params.base_inductance();
        if base.is_finite() && base > 0.0 {
            let turns_scale = (nominal_inductance / base).sqrt();
            if turns_scale.is_finite() && turns_scale > 0.0 {
                params.n_turns *= turns_scale;
            }
        }
    }

    Ok(params)
}

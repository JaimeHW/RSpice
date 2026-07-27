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

/// Resolve Xyce's level-1 nonlinear mutual-inductor (`Core`) model.
///
/// Unlike a standalone Jiles-Atherton inductor card, Xyce interprets the
/// numeric value on each winding as its number of turns and supplies `AREA`,
/// `PATH`, and `GAP` in centimetres/centimetres².  Keep that distinction
/// explicit so a Core card cannot silently acquire the standalone resolver's
/// inferred-turns or SI-geometry semantics.
pub(in crate::engine::builder) fn resolve_xyce_core_model_params(
    model_def: &crate::netlist::ModelDef,
    winding_turns: f64,
) -> Result<crate::device::passive::JilesAthertonParams, SimulationError> {
    if !winding_turns.is_finite() || winding_turns <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Xyce Core model '{}' has invalid winding turns {}",
            model_def.name, winding_turns
        )));
    }

    // Defaults are the canonical values from Xyce's MutIndNonLin model
    // metadata (N_DEV_MutIndNonLin.C), expressed in the SI units consumed by
    // the native runtime below.
    let mut params = crate::device::passive::JilesAthertonParams {
        ms: 1.0e6,
        a: 1000.0,
        k: 500.0,
        c: 0.2,
        alpha: 5.0e-5,
        area: 0.1e-4,
        length: 1.0e-2,
        n_turns: winding_turns,
        gap: 0.0,
        xyce_core: true,
        xyce_core_level2: false,
        delta_v: 0.1,
        v_inf: 1.0,
        delta_v_scaling: 1.0e3,
        beta_h: 1.0e-4,
        beta_m: 3.125e-5,
    };

    if let Some(ms) = positive_model_param(model_def, &["MS"], "MS")? {
        params.ms = ms;
    }
    if let Some(a) = positive_model_param(model_def, &["A"], "A")? {
        params.a = a;
    }
    if let Some(k) = positive_model_param(model_def, &["K", "KIRR"], "K")? {
        params.k = k;
    }
    if let Some(c) = unit_interval_model_param(model_def, &["C"], "C")? {
        params.c = c;
    }
    if let Some(alpha) = nonnegative_model_param(model_def, &["ALPHA"], "ALPHA")? {
        params.alpha = alpha;
    }
    if let Some(area_cm2) = positive_model_param(model_def, &["AREA", "ACORE", "COREAREA"], "AREA")?
    {
        params.area = area_cm2 * 1.0e-4;
    }
    if let Some(path_cm) =
        positive_model_param(model_def, &["PATH", "LENGTH", "LEN", "PATHLEN"], "PATH")?
    {
        params.length = path_cm * 1.0e-2;
    }
    if let Some(gap_cm) = nonnegative_model_param(model_def, &["GAP"], "GAP")? {
        params.gap = gap_cm * 1.0e-2;
    }
    if let Some(beta_h) = positive_model_param(model_def, &["BETAH"], "BETAH")? {
        params.beta_h = beta_h;
    }
    if let Some(beta_m) = positive_model_param(model_def, &["BETAM"], "BETAM")? {
        params.beta_m = beta_m;
    }
    // Xyce has two distinct nonlinear mutual-inductor device contracts.  A
    // CORE model with LEVEL=1 (or with no LEVEL, whose K-device default is
    // level 1) is routed to MutIndNonLin; LEVEL=2 is routed to
    // MutIndNonLin2.  The constitutive curve is shared, but the voltage
    // direction scaling and accepted-state equation are not interchangeable.
    let level = if let Some(level) = model_param(&model_def.params, &["LEVEL"]) {
        if !level.is_finite() || level < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Xyce Core model '{}' has invalid LEVEL={level} (must be finite and >= 0)",
                model_def.name
            )));
        }
        level
    } else {
        1.0
    };
    params.xyce_core_level2 = level >= 2.0;
    if let Some(delta_v) = positive_model_param(model_def, &["DELV"], "DELV")? {
        params.delta_v = delta_v;
    }
    if let Some(v_inf) = positive_model_param(model_def, &["VINF"], "VINF")? {
        params.v_inf = v_inf;
    }
    if let Some(delta_v_scaling) = positive_model_param(model_def, &["DELVSCALING"], "DELVSCALING")?
    {
        params.delta_v_scaling = delta_v_scaling;
    }

    Ok(params)
}

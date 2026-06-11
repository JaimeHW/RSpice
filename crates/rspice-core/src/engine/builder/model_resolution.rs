#![allow(clippy::needless_range_loop)]
use super::*;

mod capacitors;
mod inductors;
mod magnetic;
mod resistors;
mod transmission;
mod xspice;

pub(super) use capacitors::*;
pub(super) use inductors::*;
pub(super) use magnetic::*;
pub(super) use resistors::*;
pub(super) use transmission::*;
pub(super) use xspice::*;

pub(super) fn model_param(params: &[(String, f64)], names: &[&str]) -> Option<f64> {
    params.iter().find_map(|(name, value)| {
        if names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            Some(*value)
        } else {
            None
        }
    })
}

pub(super) fn instance_param(params: &[(String, f64)], names: &[&str]) -> Option<f64> {
    params.iter().find_map(|(name, value)| {
        if names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            Some(*value)
        } else {
            None
        }
    })
}

fn normalize_temperature_param_to_celsius(value: f64) -> f64 {
    if value > 200.0 {
        crate::analysis::temperature::kelvin_to_celsius(value)
    } else {
        value
    }
}

/// Build the expression-evaluation context for a passive instance (R/C/L),
/// honoring instance `TEMP`/`DTEMP` overrides and the model card's `TNOM`.
///
/// Returns `(context, instance_temperature_celsius, tnom_celsius)`.
fn resolve_passive_eval_context(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> Result<(crate::netlist::ParamContext, f64, f64), SimulationError> {
    let mut current_temp_c = crate::analysis::temperature::kelvin_to_celsius(temperature_kelvin);
    if let Some(temp) = instance_param(instance_params, &["TEMP"]) {
        current_temp_c = normalize_temperature_param_to_celsius(temp);
    } else if let Some(dtemp) = instance_param(instance_params, &["DTEMP"]) {
        current_temp_c += dtemp;
    }

    let base_tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let Some(model_def) = model_def else {
        let mut ctx = netlist.params.clone();
        ctx.set("TEMP", current_temp_c);
        ctx.set("TEMPER", current_temp_c);
        ctx.set("TNOM", base_tnom_c);
        return Ok((ctx, current_temp_c, base_tnom_c));
    };

    let initial_ctx = build_model_eval_context(netlist, model_def, current_temp_c, base_tnom_c);
    let model_tnom_c = resolve_model_param(model_def, &["TNOM"], &initial_ctx)?
        .map(normalize_temperature_param_to_celsius)
        .unwrap_or(base_tnom_c);

    let ctx = if (model_tnom_c - base_tnom_c).abs() > f64::EPSILON {
        build_model_eval_context(netlist, model_def, current_temp_c, model_tnom_c)
    } else {
        initial_ctx
    };

    Ok((ctx, current_temp_c, model_tnom_c))
}

fn build_model_eval_context(
    netlist: &Netlist,
    model_def: &crate::netlist::ModelDef,
    current_temp_c: f64,
    tnom_c: f64,
) -> crate::netlist::ParamContext {
    let mut ctx = netlist.params.clone();
    ctx.set("TEMP", current_temp_c);
    ctx.set("TEMPER", current_temp_c);
    ctx.set("TNOM", tnom_c);

    for (name, value) in &model_def.params {
        ctx.set(name, *value);
    }

    let mut pending = model_def.expr_params.clone();
    let mut progress = true;
    while !pending.is_empty() && progress {
        progress = false;
        let mut unresolved = Vec::new();
        for (name, expr) in pending {
            if let Ok(value) = crate::netlist::expr::eval_expression(&expr, &ctx) {
                ctx.set(&name, value);
                progress = true;
            } else {
                unresolved.push((name, expr));
            }
        }
        pending = unresolved;
    }

    ctx
}

fn resolve_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    ctx: &crate::netlist::ParamContext,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        return Ok(Some(value));
    }

    for candidate in names {
        if let Some((name, expr)) = model_def
            .expr_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(candidate))
        {
            let value = crate::netlist::expr::eval_expression(expr, ctx).map_err(|e| {
                SimulationError::Circuit(format!(
                    "Model '{}' parameter '{}' could not be resolved: {}",
                    model_def.name, name, e
                ))
            })?;
            return Ok(Some(value));
        }
    }

    Ok(None)
}

fn model_binning_param(model_def: &crate::netlist::ModelDef, name: &str) -> Option<f64> {
    model_param(&model_def.params, &[name])
}

fn model_matches_geometry(model_def: &crate::netlist::ModelDef, width: f64, length: f64) -> bool {
    let lmin = model_binning_param(model_def, "LMIN");
    let lmax = model_binning_param(model_def, "LMAX");
    let wmin = model_binning_param(model_def, "WMIN");
    let wmax = model_binning_param(model_def, "WMAX");

    if lmin.is_none() && lmax.is_none() && wmin.is_none() && wmax.is_none() {
        return false;
    }

    let within_l = lmin.is_none_or(|min| length >= min) && lmax.is_none_or(|max| length <= max);
    let within_w = wmin.is_none_or(|min| width >= min) && wmax.is_none_or(|max| width <= max);
    within_l && within_w
}

fn model_bin_range_size(model_def: &crate::netlist::ModelDef) -> f64 {
    let l_range = match (
        model_binning_param(model_def, "LMIN"),
        model_binning_param(model_def, "LMAX"),
    ) {
        (Some(min), Some(max)) => max - min,
        _ => f64::MAX,
    };
    let w_range = match (
        model_binning_param(model_def, "WMIN"),
        model_binning_param(model_def, "WMAX"),
    ) {
        (Some(min), Some(max)) => max - min,
        _ => f64::MAX,
    };
    l_range + w_range
}

pub(super) fn resolve_bjt_type_from_model(model_type: &str) -> Option<crate::netlist::BjtType> {
    if model_type.eq_ignore_ascii_case("NPN") {
        Some(crate::netlist::BjtType::Npn)
    } else if model_type.eq_ignore_ascii_case("PNP") {
        Some(crate::netlist::BjtType::Pnp)
    } else {
        None
    }
}

pub(super) fn resolve_mos_type_from_model(model_type: &str) -> Option<crate::netlist::MosType> {
    if model_type.eq_ignore_ascii_case("NMOS") {
        Some(crate::netlist::MosType::Nmos)
    } else if model_type.eq_ignore_ascii_case("PMOS") {
        Some(crate::netlist::MosType::Pmos)
    } else {
        None
    }
}

pub(super) fn resolve_jfet_type_from_model(model_type: &str) -> Option<crate::netlist::JfetType> {
    if model_type.eq_ignore_ascii_case("NJF") {
        Some(crate::netlist::JfetType::Njf)
    } else if model_type.eq_ignore_ascii_case("PJF") {
        Some(crate::netlist::JfetType::Pjf)
    } else {
        None
    }
}

pub(super) fn resolve_mesfet_type_from_model(
    model_type: &str,
) -> Option<crate::netlist::MesfetType> {
    if model_type.eq_ignore_ascii_case("NMF") || model_type.eq_ignore_ascii_case("NHFET") {
        Some(crate::netlist::MesfetType::Nmf)
    } else if model_type.eq_ignore_ascii_case("PMF") || model_type.eq_ignore_ascii_case("PHFET") {
        Some(crate::netlist::MesfetType::Pmf)
    } else {
        None
    }
}

pub(super) fn find_model_def<'a>(
    netlist: &'a Netlist,
    model_name: &str,
) -> Option<&'a crate::netlist::ModelDef> {
    netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))
}

pub(super) fn find_binned_model_def<'a>(
    netlist: &'a Netlist,
    model_name: &str,
    instance_params: &[(String, f64)],
) -> Option<&'a crate::netlist::ModelDef> {
    let exact = find_model_def(netlist, model_name);
    if exact.is_some() {
        return exact;
    }

    let width = instance_param(instance_params, &["W", "WIDTH"])?;
    let length = instance_param(instance_params, &["L", "LENGTH"])?;
    let prefix = format!("{model_name}.");

    netlist
        .models
        .iter()
        .filter(|model_def| {
            model_def.name.len() > prefix.len()
                && model_def.name[..prefix.len()].eq_ignore_ascii_case(&prefix)
                && model_matches_geometry(model_def, width, length)
        })
        .min_by(|left, right| {
            model_bin_range_size(left)
                .partial_cmp(&model_bin_range_size(right))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

pub(super) fn expected_model_type_text(expected_types: &[&str]) -> String {
    match expected_types {
        [] => String::new(),
        [single] => (*single).to_string(),
        [left, right] => format!("{left} or {right}"),
        _ => expected_types.join(", "),
    }
}

pub(super) fn ensure_model_type(
    element_kind: &str,
    element_name: &str,
    model_name: &str,
    model_def: &crate::netlist::ModelDef,
    expected_types: &[&str],
) -> Result<(), SimulationError> {
    if expected_types
        .iter()
        .any(|kind| model_def.model_type.eq_ignore_ascii_case(kind))
    {
        return Ok(());
    }

    let expected = expected_model_type_text(expected_types);
    Err(SimulationError::Circuit(format!(
        "{} '{}' references model '{}' with incompatible type '{}'; expected {}",
        element_kind, element_name, model_name, model_def.model_type, expected
    )))
}

pub(super) fn map_switch_state(state: crate::netlist::SwitchState) -> crate::device::SwitchState {
    match state {
        crate::netlist::SwitchState::On => crate::device::SwitchState::On,
        crate::netlist::SwitchState::Off => crate::device::SwitchState::Off,
    }
}

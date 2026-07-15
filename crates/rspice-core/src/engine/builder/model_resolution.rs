#![allow(clippy::needless_range_loop)]
use super::*;

mod capacitors;
mod inductors;
mod magnetic;
mod resistors;
mod transmission;
mod xspice;

pub(crate) use capacitors::XYCE_DEFAULT_CAPACITOR_AGE_DEGRADATION;
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
    // Netlist TEMP/TNOM parameters are specified in Celsius. Device internals
    // convert to Kelvin explicitly at their API boundary.
    value
}

fn set_temperature_scalars(ctx: &mut crate::netlist::ParamContext, temp_c: f64, tnom_c: f64) {
    ctx.set("TEMP", temp_c);
    ctx.set("TEMPER", temp_c);
    ctx.set("TNOM", tnom_c);
    ctx.set(
        "VT",
        crate::constants::thermal_voltage(crate::analysis::temperature::celsius_to_kelvin(temp_c)),
    );
}

pub(super) fn base_eval_context(netlist: &Netlist) -> crate::netlist::ParamContext {
    let mut ctx = netlist.params.clone();
    ctx.set(
        "GMIN",
        netlist.options.gmin.unwrap_or(crate::constants::GMIN),
    );
    ctx
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
        let mut ctx = base_eval_context(netlist);
        set_temperature_scalars(&mut ctx, current_temp_c, base_tnom_c);
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
    let mut ctx = base_eval_context(netlist);
    set_temperature_scalars(&mut ctx, current_temp_c, tnom_c);

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

fn canonical_supported_model_param<'a>(name: &str, supported: &'a [&'a str]) -> Option<&'a str> {
    supported
        .iter()
        .copied()
        .find(|candidate| name.eq_ignore_ascii_case(candidate))
}

pub(super) fn resolve_supported_model_params_upper_map(
    netlist: &Netlist,
    model_def: &crate::netlist::ModelDef,
    element_kind: &str,
    element_name: &str,
    model_name: &str,
    supported: &[&str],
    temperature_kelvin: f64,
) -> Result<std::collections::HashMap<String, f64>, SimulationError> {
    let (ctx, _, _) =
        resolve_passive_eval_context(netlist, Some(model_def), &[], temperature_kelvin)?;
    let mut params = std::collections::HashMap::new();

    for (name, value) in &model_def.params {
        if let Some(param) = canonical_supported_model_param(name, supported) {
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "{} '{}' model '{}' uses non-finite switch parameter {}={}",
                    element_kind, element_name, model_name, param, value
                )));
            }
            params.insert(param.to_string(), *value);
        }
    }

    for (name, value) in &model_def.string_params {
        if let Some(param) = canonical_supported_model_param(name, supported) {
            return Err(SimulationError::Circuit(format!(
                "{} '{}' model '{}' uses non-numeric switch parameter {}=\"{}\"; switch parameters must be finite numeric values",
                element_kind, element_name, model_name, param, value
            )));
        }
    }

    for (name, values) in &model_def.string_vector_params {
        if let Some(param) = canonical_supported_model_param(name, supported) {
            return Err(SimulationError::Circuit(format!(
                "{} '{}' model '{}' uses non-numeric switch parameter {}={:?}; switch parameters must be finite numeric values",
                element_kind, element_name, model_name, param, values
            )));
        }
    }

    for (name, expr) in &model_def.expr_params {
        if let Some(param) = canonical_supported_model_param(name, supported) {
            let value = crate::netlist::expr::eval_expression(expr, &ctx).map_err(|err| {
                SimulationError::Circuit(format!(
                    "{} '{}' model '{}' switch parameter {} could not be resolved: {}",
                    element_kind, element_name, model_name, param, err
                ))
            })?;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "{} '{}' model '{}' uses non-finite switch parameter {}={}",
                    element_kind, element_name, model_name, param, value
                )));
            }
            params.insert(param.to_string(), value);
        }
    }

    Ok(params)
}

fn model_binning_param(model_def: &crate::netlist::ModelDef, name: &str) -> Option<f64> {
    model_param(&model_def.params, &[name])
}

fn model_matches_geometry(
    model_def: &crate::netlist::ModelDef,
    instance_params: &[(String, f64)],
) -> bool {
    let lmin = model_binning_param(model_def, "LMIN");
    let lmax = model_binning_param(model_def, "LMAX");
    let wmin = model_binning_param(model_def, "WMIN");
    let wmax = model_binning_param(model_def, "WMAX");
    let nfinmin = model_binning_param(model_def, "NFINMIN");
    let nfinmax = model_binning_param(model_def, "NFINMAX");

    if lmin.is_none()
        && lmax.is_none()
        && wmin.is_none()
        && wmax.is_none()
        && nfinmin.is_none()
        && nfinmax.is_none()
    {
        return false;
    }

    let length = instance_param(instance_params, &["L", "LENGTH"]);
    let width = instance_param(instance_params, &["W", "WIDTH"]);
    let nfin = instance_param(instance_params, &["NFIN"]);

    bin_range_contains(length, lmin, lmax)
        && bin_range_contains(width, wmin, wmax)
        && bin_range_contains(nfin, nfinmin, nfinmax)
}

fn bin_range_contains(value: Option<f64>, min: Option<f64>, max: Option<f64>) -> bool {
    if min.is_none() && max.is_none() {
        return true;
    }
    let Some(value) = value else {
        return false;
    };
    min.is_none_or(|min| value >= min) && max.is_none_or(|max| value < max)
}

fn model_bin_range_size(model_def: &crate::netlist::ModelDef) -> f64 {
    bin_range_size(
        model_binning_param(model_def, "LMIN"),
        model_binning_param(model_def, "LMAX"),
    ) + bin_range_size(
        model_binning_param(model_def, "WMIN"),
        model_binning_param(model_def, "WMAX"),
    ) + bin_range_size(
        model_binning_param(model_def, "NFINMIN"),
        model_binning_param(model_def, "NFINMAX"),
    )
}

fn bin_range_size(min: Option<f64>, max: Option<f64>) -> f64 {
    match (min, max) {
        (Some(min), Some(max)) => max - min,
        (Some(_), None) | (None, Some(_)) => f64::MAX / 4.0,
        (None, None) => 0.0,
    }
}

pub(super) fn resolve_bjt_type_from_model(model_type: &str) -> Option<crate::netlist::BjtType> {
    if model_type.eq_ignore_ascii_case("NPN") {
        Some(crate::netlist::BjtType::Npn)
    } else if model_type.eq_ignore_ascii_case("PNP") || is_lpnp_bjt_model_type(model_type) {
        Some(crate::netlist::BjtType::Pnp)
    } else {
        None
    }
}

pub(super) fn is_lpnp_bjt_model_type(model_type: &str) -> bool {
    model_type.eq_ignore_ascii_case("LPNP")
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

pub(super) fn is_vdmos_model_type(model_type: &str) -> bool {
    model_type.eq_ignore_ascii_case("VDMOS")
        || model_type.eq_ignore_ascii_case("NVDMOS")
        || model_type.eq_ignore_ascii_case("PVDMOS")
        || model_type.eq_ignore_ascii_case("VDMOSN")
        || model_type.eq_ignore_ascii_case("VDMOSP")
}

pub(super) fn resolve_vdmos_type_from_model(
    model_type: &str,
    params: &HashMap<String, f64>,
) -> Option<crate::netlist::MosType> {
    if model_type.eq_ignore_ascii_case("NVDMOS") || model_type.eq_ignore_ascii_case("VDMOSN") {
        return Some(crate::netlist::MosType::Nmos);
    }
    if model_type.eq_ignore_ascii_case("PVDMOS") || model_type.eq_ignore_ascii_case("VDMOSP") {
        return Some(crate::netlist::MosType::Pmos);
    }
    if !model_type.eq_ignore_ascii_case("VDMOS") {
        return None;
    }

    let flag_set = |names: &[&str]| {
        names
            .iter()
            .any(|name| params.get(*name).copied().is_some_and(|value| value != 0.0))
    };

    if flag_set(&["PCHAN", "PCHANNEL", "PMOS"]) {
        Some(crate::netlist::MosType::Pmos)
    } else {
        Some(crate::netlist::MosType::Nmos)
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

    let prefix = format!("{model_name}.");

    netlist
        .models
        .iter()
        .filter(|model_def| {
            model_def.name.len() > prefix.len()
                && model_def.name[..prefix.len()].eq_ignore_ascii_case(&prefix)
                && model_matches_geometry(model_def, instance_params)
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

#[cfg(test)]
mod tests {
    use super::bin_range_contains;

    #[test]
    fn model_bin_ranges_are_lower_inclusive_and_upper_exclusive() {
        assert!(bin_range_contains(Some(1.0), Some(1.0), Some(2.0)));
        assert!(bin_range_contains(Some(1.5), Some(1.0), Some(2.0)));
        assert!(!bin_range_contains(Some(2.0), Some(1.0), Some(2.0)));
        assert!(!bin_range_contains(Some(0.5), Some(1.0), Some(2.0)));
        assert!(bin_range_contains(Some(2.0), Some(1.0), None));
        assert!(bin_range_contains(Some(1.0), None, Some(2.0)));
        assert!(!bin_range_contains(None, Some(1.0), Some(2.0)));
    }
}

use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TransmissionLineModelParams {
    pub(super) z0: Option<f64>,
    pub(super) td: Option<f64>,
    pub(super) freq: Option<f64>,
    pub(super) nl: Option<f64>,
    pub(super) r: Option<f64>,
    pub(super) g: Option<f64>,
    pub(super) len: Option<f64>,
    pub(super) alpha: Option<f64>,
    pub(super) atten: Option<f64>,
}

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

pub(super) fn resolve_resistor_instance_value(
    netlist: &Netlist,
    element_name: &str,
    value: f64,
    model_name: Option<&str>,
    instance_params: &[(String, f64)],
) -> Result<f64, SimulationError> {
    let mut resistance = instance_param(instance_params, &["R", "VALUE"]);
    if resistance.is_none() && value.is_finite() && value > 0.0 {
        resistance = Some(value);
    }

    if let Some(model_name) = model_name {
        let model_def = find_model_def(netlist, model_name).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Resistor '{}' references unknown model '{}'",
                element_name, model_name
            ))
        })?;
        ensure_model_type(
            "Resistor",
            element_name,
            model_name,
            model_def,
            &["R", "RES", "RESISTOR"],
        )?;

        if resistance.is_none() {
            resistance = model_param(&model_def.params, &["R", "RES", "R0", "VALUE"]);
        }

        if resistance.is_none() {
            let rsh = model_param(&model_def.params, &["RSH", "SHEETRES", "SHEETR"]).ok_or_else(
                || {
                    SimulationError::Circuit(format!(
                        "Resistor '{}' model '{}' requires R/RES or RSH with geometry",
                        element_name, model_name
                    ))
                },
            )?;

            if !rsh.is_finite() || rsh <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' model '{}' has invalid RSH={} (must be finite and > 0)",
                    element_name, model_name, rsh
                )));
            }

            let squares = if let Some(nsq) =
                instance_param(instance_params, &["NRS", "NRSQ", "NSQ", "SQUARES"])
            {
                nsq
            } else if let Some(nsq) =
                model_param(&model_def.params, &["NRS", "NRSQ", "NSQ", "SQUARES"])
            {
                nsq
            } else {
                let l = instance_param(instance_params, &["L", "LENGTH"])
                    .or_else(|| model_param(&model_def.params, &["L", "LENGTH"]))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Resistor '{}' model '{}' requires L/LENGTH when using RSH",
                            element_name, model_name
                        ))
                    })?;
                let w = instance_param(instance_params, &["W", "WIDTH"])
                    .or_else(|| model_param(&model_def.params, &["W", "WIDTH", "DEFW"]))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Resistor '{}' model '{}' requires W/WIDTH (or DEFW) when using RSH",
                            element_name, model_name
                        ))
                    })?;
                let narrow = model_param(&model_def.params, &["NARROW"]).unwrap_or(0.0);
                let l_eff = l - narrow;
                let w_eff = w - narrow;
                if !l_eff.is_finite() || !w_eff.is_finite() || l_eff <= 0.0 || w_eff <= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "Resistor '{}' has invalid effective geometry (L={}, W={}, NARROW={})",
                        element_name, l, w, narrow
                    )));
                }
                l_eff / w_eff
            };

            if !squares.is_finite() || squares <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' has invalid number of squares {}",
                    element_name, squares
                )));
            }
            resistance = Some(rsh * squares);
        }
    }

    let mut resolved = resistance.ok_or_else(|| {
        if model_name.is_some() {
            SimulationError::Circuit(format!(
                "Resistor '{}' model-based value could not be resolved",
                element_name
            ))
        } else {
            SimulationError::Circuit(format!(
                "Resistor '{}' has no valid resistance value",
                element_name
            ))
        }
    })?;

    if let Some(mult) = instance_param(instance_params, &["M", "MULT"]) {
        if !mult.is_finite() || mult <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' has invalid multiplicity M={} (must be finite and > 0)",
                element_name, mult
            )));
        }
        resolved /= mult;
    }

    if !resolved.is_finite() || resolved <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' resolved to invalid resistance {}",
            element_name, resolved
        )));
    }

    Ok(resolved)
}

pub(super) fn resolve_tline_model_params(
    netlist: &Netlist,
    model_name: &str,
) -> Option<TransmissionLineModelParams> {
    let model = netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))?;

    let mut params = TransmissionLineModelParams {
        z0: model_param(&model.params, &["Z0", "ZO"]),
        td: model_param(&model.params, &["TD", "TDELAY"]),
        freq: model_param(&model.params, &["F", "FREQ"]),
        nl: model_param(&model.params, &["NL"]),
        r: model_param(&model.params, &["R", "R0"]),
        g: model_param(&model.params, &["G", "G0"]),
        len: model_param(&model.params, &["LEN", "LENGTH"]),
        alpha: model_param(&model.params, &["ALPHA"]),
        atten: model_param(&model.params, &["ATTEN", "ATTENDB", "LOSSDB"]),
    };

    let l = model_param(&model.params, &["L", "L0"]);
    let c = model_param(&model.params, &["C", "C0"]);
    let len = params.len;

    if params.z0.is_none() {
        if let (Some(l), Some(c)) = (l, c) {
            if l > 0.0 && c > 0.0 {
                params.z0 = Some((l / c).sqrt());
            }
        }
    }

    if params.td.is_none() {
        if let (Some(f), Some(nl)) = (params.freq, params.nl) {
            if f > 0.0 {
                params.td = Some(nl / f);
            }
        }
    }

    if params.td.is_none() {
        if let (Some(l), Some(c), Some(len)) = (l, c, len) {
            if l > 0.0 && c > 0.0 && len > 0.0 {
                params.td = Some(len * (l * c).sqrt());
            }
        }
    }

    Some(params)
}

pub(super) fn tline_model_attenuation(params: TransmissionLineModelParams, z0: f64) -> Option<f64> {
    let len = params.len.unwrap_or(1.0).max(0.0);

    // Explicit alpha (Np/unit length) takes precedence.
    if let Some(alpha) = params.alpha {
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    // ATTEN/ATTENDB: interpret <=1 as linear ratio, otherwise as dB.
    if let Some(atten) = params.atten {
        if atten.is_finite() && atten >= 0.0 {
            if atten <= 1.0 {
                return Some(atten);
            }
            let db_total = if params.len.is_some() {
                atten * len
            } else {
                atten
            };
            return Some(10_f64.powf(-db_total / 20.0));
        }
    }

    // Derive from primary RLGC line loss when available.
    let r = params.r.unwrap_or(0.0).max(0.0);
    let g = params.g.unwrap_or(0.0).max(0.0);
    if (r > 0.0 || g > 0.0) && z0.is_finite() && z0 > 0.0 {
        let alpha = r / (2.0 * z0) + g * z0 / 2.0;
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    None
}

pub(super) fn positive_model_param(
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

pub(super) fn unit_interval_model_param(
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

pub(super) fn nonnegative_model_param(
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

pub(super) fn resolve_jiles_atherton_model_params(
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
    use super::resolve_mesfet_type_from_model;

    #[test]
    fn resolve_mesfet_type_accepts_hfet_aliases() {
        assert_eq!(
            resolve_mesfet_type_from_model("NHFET"),
            Some(crate::netlist::MesfetType::Nmf)
        );
        assert_eq!(
            resolve_mesfet_type_from_model("PHFET"),
            Some(crate::netlist::MesfetType::Pmf)
        );
        assert_eq!(
            resolve_mesfet_type_from_model("NMF"),
            Some(crate::netlist::MesfetType::Nmf)
        );
        assert_eq!(
            resolve_mesfet_type_from_model("PMF"),
            Some(crate::netlist::MesfetType::Pmf)
        );
        assert_eq!(resolve_mesfet_type_from_model("UNKNOWN"), None);
    }
}

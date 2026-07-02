//! Model-policy and native-device selection helpers for circuit construction.

use super::*;

/// Construct a Jiles-Atherton (magnetic-core) inductor instance and add it,
/// together with its linear runtime companion, to the circuit.
pub(super) fn add_jiles_atherton_inductor_element(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &crate::netlist::Element,
    value: f64,
    model: &str,
    initial_current: Option<f64>,
) -> Result<(), SimulationError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Jiles-Atherton inductor '{}' has invalid inductance value {}",
            element.name, value
        )));
    }

    let np = circuit.get_or_create_node(&element.nodes[0]);
    let nn = circuit.get_or_create_node(&element.nodes[1]);
    let branch = circuit.allocate_branch_named(&element.name);

    let model_def = find_model_def(netlist, model).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Jiles-Atherton inductor '{}' references unknown model '{}'",
            element.name, model
        ))
    })?;
    ensure_model_type(
        "Jiles-Atherton inductor",
        &element.name,
        model,
        model_def,
        &["CORE", "JA", "JILES", "JILESATHERTON"],
    )?;

    let params = resolve_jiles_atherton_model_params(model_def, value)?;
    let mut ja = crate::device::passive::JilesAthertonInductor::new(element.name.clone(), np, nn)
        .with_params(params);
    if let Some(ic) = initial_current {
        ja.set_initial_current(ic);
    }

    let effective_l = ja.effective_inductance();
    let runtime_l = if effective_l.is_finite() && effective_l > 0.0 {
        effective_l
    } else {
        value
    };

    if let Some(ic) = initial_current {
        circuit
            .inductors
            .add_with_ic(element.name.clone(), np, nn, branch, runtime_l, ic);
    } else {
        circuit
            .inductors
            .add(element.name.clone(), np, nn, branch, runtime_l);
    }

    let inductor_index = circuit.inductors.len().saturating_sub(1);
    circuit.add_jiles_atherton_inductor(inductor_index, branch, ja);

    Ok(())
}

/// `true` when a model card's type names a magnetic-core (Jiles-Atherton)
/// model rather than a linear inductor card.
pub(super) fn is_magnetic_core_model_type(model_type: &str) -> bool {
    model_type.eq_ignore_ascii_case("CORE")
        || model_type.eq_ignore_ascii_case("JA")
        || model_type.eq_ignore_ascii_case("JILES")
        || model_type.eq_ignore_ascii_case("JILESATHERTON")
}

/// MOS model levels with a native bulk-MOSFET implementation: Berkeley
/// MOS1/MOS2/MOS3/MOS6 (1/2/3/6), ngspice MOS9 (9), and the legacy BSIM1/BSIM2
/// ports (4/5). Levels 8/49 plus Xyce-selected level 9 (BSIM3v3.3), 14/54
/// (BSIM4 v4.8) and 55-57 (BSIM3-SOI) are routed to dedicated devices before
/// this check applies.
pub(super) fn native_bulk_mos_level(level: i32) -> bool {
    matches!(level, 1 | 2 | 3 | 4 | 5 | 6 | 9)
}

pub(super) fn has_any_model_param(
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
    names: &[&str],
) -> bool {
    names.iter().any(|name| {
        params.keys().any(|param| param.eq_ignore_ascii_case(name))
            || expr_params
                .iter()
                .any(|(param, _)| param.eq_ignore_ascii_case(name))
            || string_params
                .iter()
                .any(|(param, _)| param.eq_ignore_ascii_case(name))
    })
}

// These are dispatch signatures, not complete model parameter inventories.
// BestAvailable chooses Xyce-style BSIM3 LEVEL=9 only for a decisive BSIM3
// surface, while ngspice-shaped LEVEL=9 remains MOS9.
pub(super) const LEVEL9_BSIM3_VERSION_PARAM: &[&str] = &["VERSION"];
pub(super) const LEVEL9_BSIM3_STRONG_SIGNATURE_PARAMS: &[&str] = &[
    "NCH", "NDEP", "TOXE", "TOXM", "DVT0", "DVT1", "DVT2", "NLX", "W0", "A0", "AGS", "B0", "B1",
    "KETA", "A1", "A2", "RDSW", "PRWG", "PRWB", "WR", "PCLM", "PDIBLC1", "PDIBLC2", "PDIBLCB",
    "DROUT", "PSCBE1", "PSCBE2", "PVAG", "MOBMOD", "CAPMOD", "NQSMOD", "ACNQSMOD", "NOIMOD", "ACM",
    "CALCACM", "BINUNIT", "PARAMCHK",
];
pub(super) const LEVEL9_MOS9_SIGNATURE_PARAMS: &[&str] = &[
    "VTO", "VT0", "VTH0", "KP", "U0", "UO", "GAMMA", "PHI", "ETA", "THETA", "KAPPA", "DELTA",
    "NFS", "VMAX", "XJ", "TOX", "LD", "XL", "XW", "WD", "RD", "RS", "RSH", "CBD", "CBS", "CJ",
    "MJ", "CJSW", "MJSW", "PB", "FC", "CGSO", "CGDO", "CGBO", "IS", "JS", "TPG",
];

pub(super) fn level9_selects_bsim3(
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
    dialect: SpiceDialect,
) -> bool {
    match dialect {
        SpiceDialect::Xyce => true,
        SpiceDialect::Ngspice => false,
        SpiceDialect::BestAvailable => {
            let has_strong_bsim3_signature = has_any_model_param(
                params,
                expr_params,
                string_params,
                LEVEL9_BSIM3_STRONG_SIGNATURE_PARAMS,
            );
            let has_bsim3_version = has_any_model_param(
                params,
                expr_params,
                string_params,
                LEVEL9_BSIM3_VERSION_PARAM,
            );
            let has_mos9_signature = has_any_model_param(
                params,
                expr_params,
                string_params,
                LEVEL9_MOS9_SIGNATURE_PARAMS,
            );
            has_strong_bsim3_signature || (has_bsim3_version && !has_mos9_signature)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Bsim3VersionFamily {
    V32(String),
    V33OrLater,
    UnsupportedPre33(String),
}

pub(super) fn bsim3_level8_49_version_family(
    params: &HashMap<String, f64>,
    string_params: &[(String, String)],
) -> Option<Bsim3VersionFamily> {
    if let Some(version) = params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("VERSION"))
        .map(|(_, value)| *value)
    {
        let requested = format!("{version}");
        return Some(if approx_any(version, &[3.2, 3.22, 3.23, 3.24]) {
            Bsim3VersionFamily::V32(requested)
        } else if version < 3.3 {
            Bsim3VersionFamily::UnsupportedPre33(requested)
        } else {
            Bsim3VersionFamily::V33OrLater
        });
    }

    let version = string_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("VERSION"))
        .map(|(_, value)| value.trim());
    version.map(|value| {
        let normalized = value.trim_matches('"').trim_matches('\'').trim();
        if matches!(
            normalized,
            "3.2" | "3.20" | "3.2.0" | "3.22" | "3.2.2" | "3.23" | "3.2.3" | "3.24" | "3.2.4"
        ) {
            Bsim3VersionFamily::V32(normalized.to_string())
        } else if normalized.parse::<f64>().is_ok_and(|version| version < 3.3) {
            Bsim3VersionFamily::UnsupportedPre33(normalized.to_string())
        } else {
            Bsim3VersionFamily::V33OrLater
        }
    })
}

pub(super) fn approx_any(value: f64, candidates: &[f64]) -> bool {
    candidates
        .iter()
        .any(|candidate| (value - candidate).abs() <= 1.0e-12)
}

pub(super) fn native_ekv3_level(level: i32) -> bool {
    level == 301
}

pub(super) fn native_ekv26_level(level: i32) -> bool {
    level == 260
}

pub(super) fn known_advanced_mos_level_without_native(level: i32) -> bool {
    matches!(
        level,
        58 | 61
            | 62
            | 68
            | 70
            | 73
            | 77
            | 102
            | 103
            | 104
            | 107
            | 108
            | 109
            | 110
            | 111
            | 1000
            | 1031
            | 2000
            | 2001
            | 2002
            | 70450
            | 70470
            | 10240
    )
}

pub(super) fn missing_advanced_mos_builtin_error(
    element_name: &str,
    model_name: &str,
    level: i32,
) -> SimulationError {
    let descriptor = mos_level_descriptor(level);
    SimulationError::Circuit(format!(
        "MOSFET '{element_name}': model '{model_name}' requests {descriptor}, but no generated \
         Verilog-A builtin for that exact advanced compact-model family is available in this \
         build. Advanced CMC/HiSIM model families must not run through the simplified MOS \
         approximation; add or enable the exact Verilog-A-to-Rust codegen builtin with \
         reference-backed validation before running this card."
    ))
}

pub(super) fn bjt_level_matches(level: f64, expected: f64) -> bool {
    level.is_finite() && level == expected
}

pub(super) fn is_native_vbic_bjt_level(level: f64) -> bool {
    bjt_level_matches(level, 4.0)
        || bjt_level_matches(level, 9.0)
        || bjt_level_matches(level, 11.0)
        || bjt_level_matches(level, 12.0)
        || bjt_level_matches(level, 13.0)
}

pub(super) fn known_unsupported_bjt_family(level: f64) -> Option<(&'static str, &'static str)> {
    if bjt_level_matches(level, 23.0) {
        return Some(("Xyce HBT_X", "Q level 23"));
    }
    if bjt_level_matches(level, 8.0) {
        return Some(("ngspice HICUM/L2", "Q level 8"));
    }
    if bjt_level_matches(level, 230.0) {
        return Some(("Xyce HICUM/L0", "Q level 230"));
    }
    if bjt_level_matches(level, 234.0) {
        return Some(("Xyce HICUM/L2", "Q level 234"));
    }
    if bjt_level_matches(level, 504.0) || bjt_level_matches(level, 505.0) {
        return Some(("MEXTRAM", "Q level 504/505"));
    }
    None
}

pub(super) fn supported_diode_level(level: f64) -> bool {
    bjt_level_matches(level, 0.0) || bjt_level_matches(level, 1.0)
}

pub(super) fn supported_bjt_level(level: f64) -> bool {
    legacy_gummel_poon_bjt_level(level) || is_native_vbic_bjt_level(level)
}

pub(super) fn legacy_gummel_poon_bjt_level(level: f64) -> bool {
    bjt_level_matches(level, 0.0) || bjt_level_matches(level, 1.0) || bjt_level_matches(level, 2.0)
}

pub(super) fn bjt_level_descriptor(level: f64) -> String {
    if level.is_finite() && (level - level.round()).abs() <= 1e-9 {
        format!("LEVEL={:.0}", level.round())
    } else {
        format!("LEVEL={level}")
    }
}

pub(super) fn validate_bjt_model_level(
    element_name: &str,
    model: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
) -> Result<(), SimulationError> {
    for (name, expr) in expr_params {
        if name.eq_ignore_ascii_case("LEVEL") {
            return Err(SimulationError::Circuit(format!(
                "BJT '{element_name}': model '{model}' has unresolved LEVEL={expr}; \
                 LEVEL selectors must be finite numeric literals"
            )));
        }
    }
    for (name, value) in string_params {
        if name.eq_ignore_ascii_case("LEVEL") {
            return Err(SimulationError::Circuit(format!(
                "BJT '{element_name}': model '{model}' has non-numeric LEVEL=\"{value}\"; \
                 LEVEL selectors must be finite numeric literals"
            )));
        }
    }

    let level = params.get("LEVEL").copied();
    if let Some(level_value) = level {
        if !level_value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "BJT '{element_name}': model '{model}' has invalid LEVEL={level_value}"
            )));
        }
    }

    let native_vbic_level = level.is_some_and(is_native_vbic_bjt_level);
    reject_unsupported_vbic13_params(
        element_name,
        model,
        params,
        expr_params,
        string_params,
        native_vbic_level,
    )?;

    if native_vbic_level {
        reject_deferred_native_bjt_model_params(
            element_name,
            model,
            "VBIC",
            params,
            expr_params,
            string_params,
        )?;
    }

    let Some(level) = level else {
        return Ok(());
    };

    if !supported_bjt_level(level) {
        if let Some((family, selector)) = known_unsupported_bjt_family(level) {
            let descriptor = bjt_level_descriptor(level);
            return Err(SimulationError::Circuit(format!(
                "BJT '{element_name}': model '{model}' requests {family} {descriptor} ({selector}), \
                 which has no native implementation. Advanced CMC BJT families are future \
                 Verilog-A-to-Rust codegen targets and must not fall back to any other BJT model \
                 family or runtime Verilog-A."
            )));
        }

        let descriptor = bjt_level_descriptor(level);
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': model '{model}' requests {descriptor}, which has no native \
             implementation. Supported BJT model levels: legacy Gummel-Poon (no LEVEL or \
             LEVEL=1/2), and LEVEL=4/9/11/12/13 (VBIC). Advanced CMC BJT models are generated \
             from Verilog-A rather than hand-written here."
        )));
    }

    Ok(())
}

pub(super) fn reject_deferred_native_bjt_model_params(
    element_name: &str,
    model: &str,
    family: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
) -> Result<(), SimulationError> {
    for (name, expr) in expr_params {
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': native {family} model '{model}' uses unresolved model parameter {name}={expr}; \
             native {family} model parameters must be finite numeric literals"
        )));
    }
    for (name, value) in string_params {
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': native {family} model '{model}' uses non-numeric model parameter {name}=\"{value}\"; \
             native {family} model parameters must be finite numeric literals"
        )));
    }
    for (name, value) in params {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "BJT '{element_name}': native {family} model '{model}' uses non-finite model parameter {name}={value}; \
                 native {family} model parameters must be finite numeric literals"
            )));
        }
    }

    Ok(())
}

pub(super) const VBIC13_PARAMS: &[&str] =
    &["VBBE", "NBBE", "IBBE", "TVBBE1", "TVBBE2", "TNBBE", "EBBE"];

pub(super) fn canonical_vbic13_param(name: &str) -> Option<&'static str> {
    VBIC13_PARAMS
        .iter()
        .copied()
        .find(|param| param.eq_ignore_ascii_case(name))
}

pub(super) fn push_unique_vbic13_param(list: &mut Vec<&'static str>, param: &'static str) {
    if !list.contains(&param) {
        list.push(param);
    }
}

pub(super) fn reject_unsupported_vbic13_params(
    element_name: &str,
    model: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
    native_vbic_level: bool,
) -> Result<(), SimulationError> {
    let mut present: Vec<&'static str> = Vec::new();
    for param in VBIC13_PARAMS
        .iter()
        .copied()
        .filter(|param| params.contains_key(*param))
    {
        push_unique_vbic13_param(&mut present, param);
    }
    for (name, _) in expr_params {
        if let Some(param) = canonical_vbic13_param(name) {
            push_unique_vbic13_param(&mut present, param);
        }
    }
    for (name, _) in string_params {
        if let Some(param) = canonical_vbic13_param(name) {
            push_unique_vbic13_param(&mut present, param);
        }
    }
    if present.is_empty() {
        return Ok(());
    }

    let present_list = present.join(", ");
    if !native_vbic_level {
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': model '{model}' uses VBIC13 parameter(s) {present_list}; \
             these are unsupported on legacy GP routing, accepted only on native VBIC LEVEL=4, \
             LEVEL=9, LEVEL=11, LEVEL=12, or LEVEL=13 cards, and must not be silently ignored"
        )));
    }

    for (name, expr) in expr_params {
        let Some(param) = canonical_vbic13_param(name) else {
            continue;
        };
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': model '{model}' uses unresolved VBIC13 parameter {param}={expr}; \
             accepted VBIC13 compatibility parameters must be finite numeric literals"
        )));
    }
    for (name, value) in string_params {
        let Some(param) = canonical_vbic13_param(name) else {
            continue;
        };
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': model '{model}' uses non-numeric VBIC13 parameter {param}=\"{value}\"; \
             accepted VBIC13 compatibility parameters must be finite numeric literals"
        )));
    }

    for param in &present {
        if !params.contains_key(*param) {
            continue;
        }
        let value = params
            .get(*param)
            .copied()
            .expect("present VBIC13 parameter has a value");
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "BJT '{element_name}': model '{model}' uses non-finite VBIC13 parameter {param}={value}; \
                 accepted VBIC13 parameters must be finite numeric literals"
            )));
        }
    }

    let vbbe = params.get("VBBE").copied().unwrap_or(0.0);
    if vbbe < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': model '{model}' uses VBIC13 parameter VBBE={vbbe}; \
             VBBE must be nonnegative for native VBIC13 reverse B-E breakdown"
        )));
    }

    let nbbe = params.get("NBBE").copied().unwrap_or(1.0);
    if nbbe <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': model '{model}' uses VBIC13 parameter NBBE={nbbe}; \
             NBBE must be positive for native VBIC13 reverse B-E breakdown"
        )));
    }

    let ibbe = params.get("IBBE").copied().unwrap_or(1e-6);
    if ibbe <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "BJT '{element_name}': model '{model}' uses VBIC13 parameter IBBE={ibbe}; \
             IBBE must be positive for native VBIC13 reverse B-E breakdown"
        )));
    }

    if vbbe > 0.0 {
        for (name, expr) in expr_params {
            if name.eq_ignore_ascii_case("WBE") {
                return Err(SimulationError::Circuit(format!(
                    "BJT '{element_name}': model '{model}' uses unresolved WBE={expr} with active \
                     VBIC13 reverse B-E breakdown; WBE must be a finite numeric literal for the \
                     native split path"
                )));
            }
        }
        for (name, value) in string_params {
            if name.eq_ignore_ascii_case("WBE") {
                return Err(SimulationError::Circuit(format!(
                    "BJT '{element_name}': model '{model}' uses non-numeric WBE=\"{value}\" with \
                     active VBIC13 reverse B-E breakdown; WBE must be a finite numeric literal for \
                     the native split path"
                )));
            }
        }
        let wbe = params.get("WBE").copied().unwrap_or(1.0);
        if !wbe.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "BJT '{element_name}': model '{model}' uses active VBIC13 reverse B-E breakdown \
                 with WBE={wbe}; WBE must be finite for the native split path"
            )));
        }
    }

    Ok(())
}

pub(super) fn validate_diode_model_level(
    element_name: &str,
    model: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
) -> Result<(), SimulationError> {
    for (name, expr) in expr_params {
        if name.eq_ignore_ascii_case("LEVEL") {
            return Err(SimulationError::Circuit(format!(
                "Diode '{element_name}': model '{model}' has unresolved LEVEL={expr}; \
                 LEVEL selectors must be finite numeric literals"
            )));
        }
    }
    for (name, value) in string_params {
        if name.eq_ignore_ascii_case("LEVEL") {
            return Err(SimulationError::Circuit(format!(
                "Diode '{element_name}': model '{model}' has non-numeric LEVEL=\"{value}\"; \
                 LEVEL selectors must be finite numeric literals"
            )));
        }
    }

    let Some(level) = params.get("LEVEL").copied() else {
        return Ok(());
    };
    if !level.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "Diode '{element_name}': model '{model}' has invalid LEVEL={level}"
        )));
    }
    if !supported_diode_level(level) {
        let descriptor = bjt_level_descriptor(level);
        return Err(SimulationError::Circuit(format!(
            "Diode '{element_name}': model '{model}' requests {descriptor}, which has no native \
             implementation. Supported diode model levels: legacy SPICE diode (no LEVEL, LEVEL=0, \
             or LEVEL=1). Advanced CMC diode models will be generated from Verilog-A rather than \
             hand-written here."
        )));
    }

    Ok(())
}

pub(super) const VSWITCH_MODEL_PARAMS: &[&str] = &[
    "VT", "VH", "VON", "VOFF", "VHON", "VHOFF", "ON", "OFF", "ONH", "OFFH", "RON", "ROFF", "SMOOTH",
];
pub(super) const ISWITCH_MODEL_PARAMS: &[&str] = &[
    "IT", "IH", "ION", "IOFF", "IHON", "IHOFF", "ON", "OFF", "ONH", "OFFH", "RON", "ROFF", "SMOOTH",
];
pub(super) const GENERIC_SWITCH_MODEL_PARAMS: &[&str] =
    &["ON", "OFF", "ONH", "OFFH", "RON", "ROFF"];

pub(super) fn native_mos_instance_multiplier(
    element_name: &str,
    family: &str,
    instance_params: &[(String, f64)],
) -> Result<f64, SimulationError> {
    let mut multiplier = None;
    for (alias, value) in instance_params
        .iter()
        .filter(|(name, _)| name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT"))
    {
        if !value.is_finite() || *value <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{element_name}': native {family} instance parameter {alias}={value} must be finite and > 0"
            )));
        }
        if multiplier.is_none() {
            multiplier = Some(*value);
        }
    }
    Ok(multiplier.unwrap_or(1.0))
}

/// BSIM4 instance selector parsing mirrors ngspice's `floor(value + 0.5)`
/// coercion, then fails closed if the effective selector is not native.
pub(super) fn native_bsim4v8_instance_selector(
    element_name: &str,
    family: &str,
    instance_params: &[(String, f64)],
    param: &str,
    default: i32,
    allowed: &[i32],
) -> Result<(i32, bool), SimulationError> {
    let Some((_, raw)) = instance_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(param))
    else {
        return Ok((default, false));
    };
    let supported = allowed
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    if !raw.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance selector {param}={raw} \
             must be a finite numeric selector (supported values: {supported})"
        )));
    }
    let rounded = (*raw + 0.5).floor();
    if !rounded.is_finite() || rounded < f64::from(i32::MIN) || rounded > f64::from(i32::MAX) {
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance selector {param}={raw} \
             rounds outside the finite integer range (supported values: {supported})"
        )));
    }
    let value = rounded as i32;
    if allowed.contains(&value) {
        Ok((value, true))
    } else {
        Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance selector {param}={raw} \
             must round to a finite integer in the supported set ({supported}); effective {value}"
        )))
    }
}

pub(super) fn native_mos_instance_integer_selector_reset_to_default(
    element_name: &str,
    family: &str,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    param: &str,
    default: i32,
    allowed: &[i32],
) -> Result<(i32, bool), SimulationError> {
    if let Some((_, expr)) = deferred_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(param))
    {
        let supported = allowed
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance selector {param}={expr} \
             must be a finite integer (supported values: {supported})"
        )));
    }
    let Some((_, raw)) = instance_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(param))
    else {
        return Ok((default, false));
    };
    let supported = allowed
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    if !raw.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance selector {param}={raw} \
             must be a finite numeric selector (supported values: {supported})"
        )));
    }
    let rounded = (*raw + 0.5).floor();
    if !rounded.is_finite() || rounded < f64::from(i32::MIN) || rounded > f64::from(i32::MAX) {
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance selector {param}={raw} \
             rounds outside the finite integer range (supported values: {supported})"
        )));
    }
    let value = rounded as i32;
    if allowed.contains(&value) {
        Ok((value, true))
    } else {
        Ok((default, true))
    }
}

pub(super) const B3SOI_DEBUG_SUPPORTED: &str = "-1 or any non-negative 32-bit integer";

pub(super) fn native_b3soi_debug_mod(
    element_name: &str,
    family: &str,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
) -> Result<Option<i32>, SimulationError> {
    for (name, expr) in deferred_params {
        if name.eq_ignore_ascii_case("DEBUG") {
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{element_name}': native {family} instance selector DEBUG={expr} \
                 must be a finite integer (supported values: {B3SOI_DEBUG_SUPPORTED})"
            )));
        }
    }

    let mut debug_mod = None;
    for (_, raw) in instance_params
        .iter()
        .filter(|(name, _)| name.eq_ignore_ascii_case("DEBUG"))
    {
        if !raw.is_finite() || raw.trunc() != *raw || *raw < -1.0 || *raw > f64::from(i32::MAX) {
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{element_name}': native {family} instance selector DEBUG={raw} \
                 must be a finite integer in the supported range ({B3SOI_DEBUG_SUPPORTED})"
            )));
        }
        if debug_mod.is_none() {
            debug_mod = Some(*raw as i32);
        }
    }
    Ok(debug_mod)
}

pub(super) fn checked_integer_model_level(
    device_kind: &str,
    element_name: &str,
    model: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
) -> Result<Option<i32>, SimulationError> {
    for (name, expr) in expr_params {
        if name.eq_ignore_ascii_case("LEVEL") {
            return Err(SimulationError::Circuit(format!(
                "{device_kind} '{element_name}': model '{model}' has unresolved LEVEL={expr}; LEVEL selectors must be finite integers"
            )));
        }
    }
    for (name, value) in string_params {
        if name.eq_ignore_ascii_case("LEVEL") {
            return Err(SimulationError::Circuit(format!(
                "{device_kind} '{element_name}': model '{model}' has non-numeric LEVEL=\"{value}\"; LEVEL selectors must be finite integers"
            )));
        }
    }

    let Some(level) = params.get("LEVEL").copied() else {
        return Ok(None);
    };

    let rounded = level.round();
    if !level.is_finite() || (level - rounded).abs() > 1e-9 {
        return Err(SimulationError::Circuit(format!(
            "{device_kind} '{element_name}': model '{model}' has invalid LEVEL={level}; LEVEL selectors must be finite integers"
        )));
    }

    Ok(Some(rounded as i32))
}

pub(super) fn reject_deferred_native_mos_model_params(
    element_name: &str,
    model: &str,
    family: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
) -> Result<(), SimulationError> {
    for (name, expr) in expr_params {
        let param = name.to_ascii_uppercase();
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} model '{model}' uses unresolved model parameter {param}={expr}; \
             native {family} model parameters must be finite numeric literals"
        )));
    }
    for (name, value) in string_params {
        let param = name.to_ascii_uppercase();
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} model '{model}' uses non-numeric model parameter {param}=\"{value}\"; \
             native {family} model parameters must be finite numeric literals"
        )));
    }
    for (name, value) in params {
        if !value.is_finite() {
            let param = name.to_ascii_uppercase();
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{element_name}': native {family} model '{model}' uses non-finite model parameter {param}={value}; \
                 native {family} model parameters must be finite numeric literals"
            )));
        }
    }

    Ok(())
}

pub(super) fn native_bsim3_model_params_upper_map(
    element_name: &str,
    model: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
) -> Result<HashMap<String, f64>, SimulationError> {
    let mut resolved = params.clone();

    for (name, expr) in expr_params {
        let param = name.to_ascii_uppercase();
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native BSIM3 model '{model}' uses unresolved model parameter {param}={expr}; \
             native BSIM3 model parameters must be finite numeric literals, except dotted VERSION metadata"
        )));
    }

    for (name, value) in string_params {
        let param = name.to_ascii_uppercase();
        if param == "VERSION" {
            let version = parse_dotted_version_metadata(value).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "MOSFET '{element_name}': native BSIM3 model '{model}' has invalid VERSION=\"{value}\"; \
                     VERSION metadata must be a dotted numeric version such as 3.3.0"
                ))
            })?;
            resolved.insert(param, version);
            continue;
        }

        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native BSIM3 model '{model}' uses non-numeric model parameter {param}=\"{value}\"; \
             native BSIM3 model parameters must be finite numeric literals, except dotted VERSION metadata"
        )));
    }

    for (name, value) in &resolved {
        if !value.is_finite() {
            let param = name.to_ascii_uppercase();
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{element_name}': native BSIM3 model '{model}' uses non-finite model parameter {param}={value}; \
                 native BSIM3 model parameters must be finite numeric literals"
            )));
        }
    }

    Ok(resolved)
}

pub(super) fn native_bsim4_model_params_upper_map(
    element_name: &str,
    model: &str,
    params: &HashMap<String, f64>,
    expr_params: &[(String, String)],
    string_params: &[(String, String)],
) -> Result<HashMap<String, f64>, SimulationError> {
    let mut resolved = params.clone();

    for (name, expr) in expr_params {
        let param = name.to_ascii_uppercase();
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native BSIM4 model '{model}' uses unresolved model parameter {param}={expr}; \
             native BSIM4 model parameters must be finite numeric literals, except dotted VERSION metadata"
        )));
    }

    for (name, value) in string_params {
        let param = name.to_ascii_uppercase();
        if param == "VERSION" {
            let version = parse_dotted_version_metadata(value).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "MOSFET '{element_name}': native BSIM4 model '{model}' has invalid VERSION=\"{value}\"; \
                     VERSION metadata must be a dotted numeric version such as 4.6.1"
                ))
            })?;
            resolved.insert(param, version);
            continue;
        }

        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native BSIM4 model '{model}' uses non-numeric model parameter {param}=\"{value}\"; \
             native BSIM4 model parameters must be finite numeric literals, except dotted VERSION metadata"
        )));
    }

    for (name, value) in &resolved {
        if !value.is_finite() {
            let param = name.to_ascii_uppercase();
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{element_name}': native BSIM4 model '{model}' uses non-finite model parameter {param}={value}; \
                 native BSIM4 model parameters must be finite numeric literals"
            )));
        }
    }

    Ok(resolved)
}

fn parse_dotted_version_metadata(value: &str) -> Option<f64> {
    let normalized = value.trim().trim_matches('"').trim_matches('\'').trim();
    if normalized.is_empty() {
        return None;
    }
    if let Ok(version) = normalized.parse::<f64>() {
        return version.is_finite().then_some(version);
    }

    let mut parts = normalized.split('.');
    let major = parts.next()?;
    let minor = parts.next()?;
    let patch_parts = parts.collect::<Vec<_>>();
    if patch_parts.is_empty()
        || major.is_empty()
        || minor.is_empty()
        || patch_parts.iter().any(|part| part.is_empty())
        || !major.chars().all(|ch| ch.is_ascii_digit())
        || !minor.chars().all(|ch| ch.is_ascii_digit())
        || !patch_parts
            .iter()
            .all(|part| part.chars().all(|ch| ch.is_ascii_digit()))
    {
        return None;
    }

    let compact = format!("{major}.{minor}{}", patch_parts.join(""));
    compact
        .parse::<f64>()
        .ok()
        .filter(|version| version.is_finite())
}

/// Warn about nodes with no conductive path to ground: the unconditional
/// matrix gmin keeps such systems numerically solvable, so without this
/// notice a forgotten connection simulates silently with a meaningless bias.
/// Capacitors and current sources do not conduct DC; every other element's
/// terminals are treated as connected, which never produces a false alarm.
pub(super) fn warn_floating_nodes(flat_elements: &[crate::netlist::Element]) {
    use crate::netlist::ElementKind;
    use std::collections::HashMap;

    let canonical = |node: &str| -> String {
        let lower = node.to_ascii_lowercase();
        if lower == "gnd" {
            "0".to_string()
        } else {
            lower
        }
    };

    let mut parent: HashMap<String, String> = HashMap::new();
    fn find(parent: &mut HashMap<String, String>, node: &str) -> String {
        let mut current = node.to_string();
        loop {
            let up = parent
                .entry(current.clone())
                .or_insert_with(|| current.clone())
                .clone();
            if up == current {
                return current;
            }
            let grand = parent.get(&up).cloned().unwrap_or_else(|| up.clone());
            parent.insert(current, grand);
            current = up;
        }
    }

    let mut all_nodes: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for element in flat_elements {
        let conducts = !matches!(
            element.kind,
            ElementKind::Capacitor { .. } | ElementKind::CurrentSource(_)
        );
        let nodes: Vec<String> = element.nodes.iter().map(|n| canonical(n)).collect();
        for node in &nodes {
            if seen.insert(node.clone()) {
                all_nodes.push(node.clone());
            }
        }
        if conducts && nodes.len() >= 2 {
            let first = find(&mut parent, &nodes[0]);
            for other in &nodes[1..] {
                let root = find(&mut parent, other);
                if root != first {
                    parent.insert(root, first.clone());
                }
            }
        }
    }

    if !seen.contains("0") {
        // No ground reference at all; other validation owns that report.
        return;
    }
    let ground_root = find(&mut parent, "0");
    let floating: Vec<&String> = all_nodes
        .iter()
        .filter(|node| node.as_str() != "0" && find(&mut parent, node) != ground_root)
        .collect();
    if floating.is_empty() {
        return;
    }
    let shown: Vec<&str> = floating.iter().take(8).map(|s| s.as_str()).collect();
    let suffix = if floating.len() > shown.len() {
        format!(" (and {} more)", floating.len() - shown.len())
    } else {
        String::new()
    };
    log::warn!(
        "node(s) {}{} have no conductive path to ground (capacitors and current sources \
         do not conduct DC); their bias is set by the matrix gmin only and is not \
         physically meaningful",
        shown.join(", "),
        suffix
    );
}

/// Diagnostic descriptor for well-known MOS model levels.
pub(super) fn mos_level_descriptor(level: i32) -> String {
    match level {
        3 => "LEVEL=3 (MOS3)".to_string(),
        8 | 49 | 53 => format!("LEVEL={level} (BSIM3v3)"),
        9 => "LEVEL=9 (ngspice MOS9 / Xyce BSIM3)".to_string(),
        14 | 54 => format!("LEVEL={level} (BSIM4)"),
        18 => "LEVEL=18 (VDMOS)".to_string(),
        58 => "LEVEL=58 (ngspice B4SOI)".to_string(),
        61 | 68 => format!("LEVEL={level} (HiSIM2)"),
        62 | 73 => format!("LEVEL={level} (HiSIM_HV)"),
        70 => "LEVEL=70 (B4SOI 4.6)".to_string(),
        77 => "LEVEL=77 (BSIM6)".to_string(),
        102 => "LEVEL=102 (PSP102)".to_string(),
        103 => "LEVEL=103 (PSP103)".to_string(),
        107 => "LEVEL=107 (BSIM-CMG 107)".to_string(),
        108 => "LEVEL=108 (BSIM-CMG 108)".to_string(),
        109 => "LEVEL=109 (BSIM-CMG 109)".to_string(),
        110 => "LEVEL=110 (BSIM-CMG 110)".to_string(),
        111 => "LEVEL=111 (BSIM-CMG 111.2.1)".to_string(),
        260 => "LEVEL=260 (EKV 2.6)".to_string(),
        301 => "LEVEL=301 (EKV3)".to_string(),
        1031 => "LEVEL=1031 (PSP103 thermal)".to_string(),
        2000 => "LEVEL=2000 (MVS 2.0.0 ETSOI)".to_string(),
        2001 => "LEVEL=2001 (MVS 2.0.0 HEMT)".to_string(),
        70450 => "LEVEL=70450 (B4SOI 4.5)".to_string(),
        70470 => "LEVEL=70470 (B4SOI 4.7)".to_string(),
        10240 => "LEVEL=10240 (L-UTSOI)".to_string(),
        _ => format!("LEVEL={level}"),
    }
}

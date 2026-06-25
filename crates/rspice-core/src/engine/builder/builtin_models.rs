use super::*;

/// Embedded transistor model library used for fallback model resolution.
const BUILTIN_TRANSISTOR_LIB: &str = include_str!("../../../../../models/spice/transistor.lib");
/// Embedded diode model library used for fallback model resolution.
const BUILTIN_DIODE_LIB: &str = include_str!("../../../../../models/spice/diode.lib");

/// Lazily parsed builtin BJT model parameter map (MODEL_NAME -> params).
pub(in crate::engine::builder) fn builtin_bjt_model_map()
-> &'static HashMap<String, HashMap<String, f64>> {
    static BJT_MODELS: OnceLock<HashMap<String, HashMap<String, f64>>> = OnceLock::new();
    BJT_MODELS.get_or_init(|| {
        let mut map = HashMap::new();
        let Ok(netlist) = crate::netlist::parse_netlist(BUILTIN_TRANSISTOR_LIB) else {
            log::warn!("Failed to parse embedded transistor library for BJT fallback models");
            return map;
        };

        for model in netlist.models {
            if model.model_type.eq_ignore_ascii_case("NPN")
                || model.model_type.eq_ignore_ascii_case("PNP")
            {
                map.insert(
                    model.name.to_uppercase(),
                    model_params_upper_map(&model.params),
                );
            }
        }
        map
    })
}

/// Lazily parsed builtin diode model parameter map (MODEL_NAME -> params).
pub(in crate::engine::builder) fn builtin_diode_model_map()
-> &'static HashMap<String, HashMap<String, f64>> {
    static DIODE_MODELS: OnceLock<HashMap<String, HashMap<String, f64>>> = OnceLock::new();
    DIODE_MODELS.get_or_init(|| {
        let mut map = HashMap::new();
        let Ok(netlist) = crate::netlist::parse_netlist(BUILTIN_DIODE_LIB) else {
            log::warn!("Failed to parse embedded diode library for fallback models");
            return map;
        };

        for model in netlist.models {
            if model.model_type.eq_ignore_ascii_case("D")
                || model.model_type.eq_ignore_ascii_case("DIODE")
            {
                map.insert(
                    model.name.to_uppercase(),
                    model_params_upper_map(&model.params),
                );
            }
        }
        map
    })
}

pub(in crate::engine::builder) fn model_params_upper_map(
    params: &[(String, f64)],
) -> HashMap<String, f64> {
    params
        .iter()
        .map(|(name, value)| (name.to_ascii_uppercase(), *value))
        .collect()
}

pub(in crate::engine::builder) fn is_bsimsoi_level(level: i32) -> bool {
    matches!(level, 10 | 55..=57)
}

const B3SOI_DEFAULT_L: f64 = 5.0e-6;
const B3SOI_DEFAULT_NPEAK: f64 = 1.7e17;
const B3SOI_DEFAULT_NLX: f64 = 1.74e-7;
const B3SOI_DEFAULT_TSI: f64 = 1.0e-7;
const B3SOI_DEFAULT_VBS0PD: f64 = 0.0;
const B3SOI_DEFAULT_VBS0FD: f64 = 0.5;
const B3SOI_DEFAULT_VBSA: f64 = 0.0;
const B3SOI_EPS_SI: f64 = 1.03594e-10;

pub(in crate::engine::builder) fn native_bsimsoi_level_for(
    level: i32,
    params: &HashMap<String, f64>,
    instance_params: &[(String, f64)],
) -> Result<Option<i32>, String> {
    if matches!(level, 55..=57) {
        return Ok(Some(level));
    }
    if level != 10 {
        return Ok(None);
    }

    let soi_mod = instance_param(instance_params, &["SOIMOD"])
        .or_else(|| params.get("SOIMOD").copied())
        .unwrap_or(0.0);
    if !soi_mod.is_finite() || (soi_mod - soi_mod.round()).abs() > 1.0e-12 {
        return Err(format!(
            "requests Xyce BSIMSOI3 LEVEL=10 with invalid SOIMOD={soi_mod}; supported native mappings are SOIMOD=0/1/2/3"
        ));
    }

    match soi_mod.round() as i32 {
        0 => Ok(Some(57)),
        1 => Ok(Some(56)),
        2 => Ok(Some(55)),
        3 => bsimsoi_auto_selected_level(params, instance_params).map(Some),
        value => Err(format!(
            "requests Xyce BSIMSOI3 LEVEL=10 with unsupported SOIMOD={value}; supported native mappings are SOIMOD=0/1/2/3"
        )),
    }
}

fn bsimsoi_model_param(params: &HashMap<String, f64>, key: &str, default: f64) -> f64 {
    params.get(key).copied().unwrap_or(default)
}

fn bsimsoi_auto_selected_level(
    params: &HashMap<String, f64>,
    instance_params: &[(String, f64)],
) -> Result<i32, String> {
    let l = instance_param(instance_params, &["L"])
        .or_else(|| params.get("L").copied())
        .unwrap_or(B3SOI_DEFAULT_L);
    let npeak = bsimsoi_model_param(params, "NCH", B3SOI_DEFAULT_NPEAK);
    let nlx = bsimsoi_model_param(params, "NLX", B3SOI_DEFAULT_NLX);
    let tsi = bsimsoi_model_param(params, "TSI", B3SOI_DEFAULT_TSI);
    let vbsa = bsimsoi_model_param(params, "VBSA", B3SOI_DEFAULT_VBSA);
    let vbs0pd = bsimsoi_model_param(params, "VBS0PD", B3SOI_DEFAULT_VBS0PD);
    let vbs0fd = bsimsoi_model_param(params, "VBS0FD", B3SOI_DEFAULT_VBS0FD);

    if !l.is_finite() || l <= 0.0 {
        return Err(format!(
            "requests Xyce BSIMSOI3 LEVEL=10 SOIMOD=3 auto-selection with invalid L={l}; L must be finite and positive"
        ));
    }
    if !npeak.is_finite()
        || !nlx.is_finite()
        || !tsi.is_finite()
        || tsi <= 0.0
        || !vbsa.is_finite()
        || !vbs0pd.is_finite()
        || !vbs0fd.is_finite()
    {
        return Err(
            "requests Xyce BSIMSOI3 LEVEL=10 SOIMOD=3 auto-selection with non-finite selector parameters"
                .to_string(),
        );
    }

    let csi = B3SOI_EPS_SI / tsi;
    let qsi =
        crate::device::mosfet::b3soi::common::CHARGE_Q * npeak * (1.0 + nlx / l) * 1.0e6 * tsi;
    let vbs0t = 0.8 - 0.5 * qsi / csi + vbsa;

    if vbs0t > vbs0fd {
        Ok(55)
    } else if vbs0t < vbs0pd {
        Ok(57)
    } else {
        Ok(56)
    }
}

pub(in crate::engine::builder) fn positive_or_one(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        1.0
    }
}

pub(in crate::engine::builder) fn jfet_extrinsic_resistance_scale(
    jfet: &crate::device::Jfet,
) -> f64 {
    let multiplier = positive_or_one(jfet.m);
    let area = if matches!(jfet.params.channel_model, JfetChannelModel::Hfet1) {
        1.0
    } else {
        positive_or_one(jfet.area)
    };
    area * multiplier
}

pub(in crate::engine::builder) fn scaled_extrinsic_resistance(resistance: f64, scale: f64) -> f64 {
    if resistance.is_finite() && resistance > 0.0 {
        resistance / positive_or_one(scale)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::native_bsimsoi_level_for;
    use std::collections::HashMap;

    fn params(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect()
    }

    fn instance_params(pairs: &[(&str, f64)]) -> Vec<(String, f64)> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect()
    }

    #[test]
    fn level10_soimod_selects_native_soi_family() {
        assert_eq!(
            native_bsimsoi_level_for(10, &params(&[]), &[]).unwrap(),
            Some(57)
        );
        assert_eq!(
            native_bsimsoi_level_for(10, &params(&[("SOIMOD", 0.0)]), &[]).unwrap(),
            Some(57)
        );
        assert_eq!(
            native_bsimsoi_level_for(10, &params(&[("SOIMOD", 1.0)]), &[]).unwrap(),
            Some(56)
        );
        assert_eq!(
            native_bsimsoi_level_for(10, &params(&[("SOIMOD", 2.0)]), &[]).unwrap(),
            Some(55)
        );
    }

    #[test]
    fn explicit_ngspice_soi_levels_route_directly() {
        for level in [55, 56, 57] {
            assert_eq!(
                native_bsimsoi_level_for(level, &params(&[("SOIMOD", 99.0)]), &[]).unwrap(),
                Some(level)
            );
        }
    }

    #[test]
    fn level10_soimod_auto_selects_native_soi_family() {
        assert_eq!(
            native_bsimsoi_level_for(10, &params(&[("SOIMOD", 3.0)]), &[]).unwrap(),
            Some(57)
        );
        assert_eq!(
            native_bsimsoi_level_for(
                10,
                &params(&[("SOIMOD", 3.0), ("VBS0PD", -1.0), ("VBS0FD", 0.5)]),
                &[]
            )
            .unwrap(),
            Some(56)
        );
        assert_eq!(
            native_bsimsoi_level_for(10, &params(&[("SOIMOD", 3.0), ("VBS0FD", -1.0)]), &[])
                .unwrap(),
            Some(55)
        );

        let instance_override = instance_params(&[("SOIMOD", 3.0)]);
        assert_eq!(
            native_bsimsoi_level_for(
                10,
                &params(&[("SOIMOD", 0.0), ("VBS0FD", -1.0)]),
                &instance_override
            )
            .unwrap(),
            Some(55)
        );
    }

    #[test]
    fn level10_invalid_soimod_values_are_rejected() {
        let fractional = native_bsimsoi_level_for(10, &params(&[("SOIMOD", 1.5)]), &[])
            .expect_err("fractional SOIMOD must be rejected");
        assert!(fractional.contains("invalid SOIMOD=1.5"), "{fractional}");
    }
}

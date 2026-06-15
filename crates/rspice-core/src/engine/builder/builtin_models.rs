use super::*;

/// Embedded transistor model library used for fallback model resolution.
const BUILTIN_TRANSISTOR_LIB: &str = include_str!("../../../models/spice/transistor.lib");
/// Embedded diode model library used for fallback model resolution.
const BUILTIN_DIODE_LIB: &str = include_str!("../../../models/spice/diode.lib");

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
    matches!(level, 55..=57)
}

pub(in crate::engine::builder) fn is_physical_mesa_mesfet_level(level: f64) -> bool {
    level.is_finite() && matches!(level.round() as i32, 2..=4)
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

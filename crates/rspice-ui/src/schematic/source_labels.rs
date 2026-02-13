//! Source Value Label Formatting
//!
//! Provides concise, descriptive source labels for schematic rendering without
//! changing netlist serialization semantics.

use crate::state::{Component, ComponentType};
use std::borrow::Cow;
use std::collections::HashMap;

/// Format the component value text for schematic display.
///
/// Non-source components return `component.value` directly (borrowed).
/// Source components return a normalized, descriptive label (owned).
pub fn component_value_label(component: &Component) -> Cow<'_, str> {
    if !component.kind.is_source() {
        return Cow::Borrowed(component.value.as_str());
    }

    Cow::Owned(format_source_label(component))
}

fn format_source_label(component: &Component) -> String {
    let params = crate::properties::parse_params_string(&component.params);
    let primary = primary_or_default(component.value.as_str(), "0");

    match component.kind {
        ComponentType::VoltageSource => format!("DC {} V", primary_or_default(primary, "0")),
        ComponentType::CurrentSource => format!("DC {} A", primary_or_default(primary, "0")),
        ComponentType::VoltageSourceAc => format_ac_label(&params, primary, "V"),
        ComponentType::CurrentSourceAc => format_ac_label(&params, primary, "A"),
        ComponentType::VoltageSourcePulse => format_pulse_label(&params, primary, true),
        ComponentType::CurrentSourcePulse => format_pulse_label(&params, primary, false),
        ComponentType::VoltageSourceSin => format_sin_label(&params, primary, true),
        ComponentType::CurrentSourceSin => format_sin_label(&params, primary, false),
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
            format_pwl_label(&params, primary)
        }
        ComponentType::VoltageSourceExp => format_exp_label(&params, primary, true),
        ComponentType::CurrentSourceExp => format_exp_label(&params, primary, false),
        ComponentType::VoltageSourceSffm => format_sffm_label(&params, primary),
        ComponentType::CurrentSourceNoise => format_noise_label(&params, primary),
        _ => primary_or_default(component.value.as_str(), "0").to_string(),
    }
}

fn format_ac_label(params: &HashMap<String, String>, primary: &str, unit: &str) -> String {
    let mag = get_param_with_aliases(params, &["ac", "acmag", "ac_mag"], primary, "1");
    let phase = get_param_with_aliases(params, &["acphase", "phase"], "", "0");
    if phase == "0" {
        format!("AC {} {}", mag, unit)
    } else {
        format!("AC {} {} angle {} deg", mag, unit, phase)
    }
}

fn format_pulse_label(params: &HashMap<String, String>, primary: &str, is_voltage: bool) -> String {
    let unit = if is_voltage { "V" } else { "A" };
    let low = get_param_with_aliases(
        params,
        if is_voltage { &["v1"] } else { &["i1"] },
        primary,
        "0",
    );
    let high = get_param_with_aliases(params, if is_voltage { &["v2"] } else { &["i2"] }, "", "1");
    let width = get_param_with_aliases(params, &["pw"], "", "1u");
    let period = get_param_with_aliases(params, &["per", "period"], "", "2u");
    format!(
        "PULSE {}->{} {} (PW={}, PER={})",
        low, high, unit, width, period
    )
}

fn format_sin_label(params: &HashMap<String, String>, primary: &str, is_voltage: bool) -> String {
    let (offset_keys, amp_keys, unit) = if is_voltage {
        (&["vo"][..], &["va"][..], "V")
    } else {
        (&["io"][..], &["ia"][..], "A")
    };
    let offset = get_param_with_aliases(params, offset_keys, primary, "0");
    let amp = get_param_with_aliases(params, amp_keys, "", "1");
    let freq = get_param_with_aliases(params, &["freq", "f"], "", "1k");
    let phase = get_param_with_aliases(params, &["phase"], "", "0");
    if phase == "0" {
        format!("SIN {} {} @ {} (offset {})", amp, unit, freq, offset)
    } else {
        format!(
            "SIN {} {} @ {} (offset {}, phase={} deg)",
            amp, unit, freq, offset, phase
        )
    }
}

fn format_pwl_label(params: &HashMap<String, String>, primary: &str) -> String {
    let data = get_param_with_aliases(params, &["pwl_data"], primary, "0 0 1u 1");
    let summary = summarize_pwl(data);
    format!("PWL {}", summary)
}

fn format_exp_label(params: &HashMap<String, String>, primary: &str, is_voltage: bool) -> String {
    let unit = if is_voltage { "V" } else { "A" };
    let low = get_param_with_aliases(
        params,
        if is_voltage { &["v1"] } else { &["i1"] },
        primary,
        "0",
    );
    let high = get_param_with_aliases(params, if is_voltage { &["v2"] } else { &["i2"] }, "", "1");
    let td1 = get_param_with_aliases(params, &["td1"], "", "0");
    let tau1 = get_param_with_aliases(params, &["tau1"], "", "1u");
    format!(
        "EXP {}->{} {} (TD1={}, TAU1={})",
        low, high, unit, td1, tau1
    )
}

fn format_sffm_label(params: &HashMap<String, String>, primary: &str) -> String {
    let offset = get_param_with_aliases(params, &["vo"], primary, "0");
    let amp = get_param_with_aliases(params, &["va"], "", "1");
    let fc = get_param_with_aliases(params, &["fc"], "", "1k");
    let fs = get_param_with_aliases(params, &["fs"], "", "10");
    format!("SFFM {} V (offset {}, fc={}, fs={})", amp, offset, fc, fs)
}

fn format_noise_label(params: &HashMap<String, String>, primary: &str) -> String {
    let dc = get_param_with_aliases(params, &["dc"], primary, "0");
    let noise_type = get_param_with_aliases(params, &["noise_type"], "", "white");
    let noise_val = get_param_with_aliases(params, &["noiseval"], "", "1e-24");
    format!("NOISE {}={} A^2/Hz (DC {} A)", noise_type, noise_val, dc)
}

fn summarize_pwl(data: &str) -> String {
    let tokens: Vec<&str> = data.split_whitespace().collect();
    if tokens.len() < 2 || tokens.len() % 2 != 0 {
        return format!("({})", data);
    }
    let points = tokens.len() / 2;
    let t0 = tokens[0];
    let t_last = tokens[tokens.len() - 2];

    if points <= 4 && data.len() <= 48 {
        format!("({})", data)
    } else {
        format!("({} pts, {}->{})", points, t0, t_last)
    }
}

fn get_param_with_aliases<'a>(
    params: &'a HashMap<String, String>,
    keys: &[&str],
    primary_fallback: &'a str,
    default: &'a str,
) -> &'a str {
    for key in keys {
        if let Some(value) = params.get(*key).map(String::as_str) {
            let trimmed = value.trim();
            if !trimmed.is_empty() && !looks_like_placeholder(trimmed) {
                return trimmed;
            }
        }
    }

    let trimmed_primary = primary_fallback.trim();
    if !trimmed_primary.is_empty() && !looks_like_placeholder(trimmed_primary) {
        trimmed_primary
    } else {
        default
    }
}

fn primary_or_default<'a>(value: &'a str, default: &'a str) -> &'a str {
    let trimmed = value.trim();
    if trimmed.is_empty() || looks_like_placeholder(trimmed) {
        default
    } else {
        trimmed
    }
}

fn looks_like_placeholder(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "value"
            | "dc"
            | "dcoffset"
            | "offset"
            | "initial"
            | "amplitude"
            | "frequency"
            | "freq"
            | "vo"
            | "io"
            | "v1"
            | "i1"
            | "ac"
            | "acmag"
            | "ac_mag"
            | "pwl_data"
            | "noiseval"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};

    fn source(kind: ComponentType, value: &str, params: &str) -> Component {
        let mut c = Component::new(1, kind, Point::new(0, 0));
        c.value = value.to_string();
        c.params = params.to_string();
        c
    }

    #[test]
    fn test_non_source_uses_raw_value() {
        let mut c = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        c.value = "10k".to_string();
        assert_eq!(component_value_label(&c), Cow::Borrowed("10k"));
    }

    #[test]
    fn test_dc_voltage_source_label() {
        let c = source(ComponentType::VoltageSource, "5", "");
        assert_eq!(component_value_label(&c).as_ref(), "DC 5 V");
    }

    #[test]
    fn test_ac_source_label_with_phase() {
        let c = source(ComponentType::VoltageSourceAc, "1", "acphase=90");
        assert_eq!(component_value_label(&c).as_ref(), "AC 1 V angle 90 deg");
    }

    #[test]
    fn test_sin_voltage_label_prefers_named_params() {
        let c = source(
            ComponentType::VoltageSourceSin,
            "0",
            "va=2.5 freq=10k vo=0.5 phase=45",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "SIN 2.5 V @ 10k (offset 0.5, phase=45 deg)"
        );
    }

    #[test]
    fn test_sin_current_label_uses_current_keys() {
        let c = source(
            ComponentType::CurrentSourceSin,
            "0",
            "ia=3m freq=1k io=1m phase=0",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "SIN 3m A @ 1k (offset 1m)"
        );
    }

    #[test]
    fn test_pulse_voltage_label_uses_per_alias() {
        let c = source(
            ComponentType::VoltageSourcePulse,
            "0",
            "v1=0 v2=1.8 pw=5n per=10n",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "PULSE 0->1.8 V (PW=5n, PER=10n)"
        );
    }

    #[test]
    fn test_pulse_current_label_uses_i_keys() {
        let c = source(
            ComponentType::CurrentSourcePulse,
            "0",
            "i1=0 i2=2m pw=10n period=20n",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "PULSE 0->2m A (PW=10n, PER=20n)"
        );
    }

    #[test]
    fn test_pwl_label_summarizes_long_data() {
        let c = source(
            ComponentType::VoltageSourcePwl,
            "",
            "pwl_data=\"0 0 1n 1 2n 0 3n 1 4n 0 5n 1\"",
        );
        assert_eq!(component_value_label(&c).as_ref(), "PWL (6 pts, 0->5n)");
    }

    #[test]
    fn test_exp_label() {
        let c = source(
            ComponentType::VoltageSourceExp,
            "",
            "v1=0 v2=5 td1=1u tau1=10n td2=10u tau2=10n",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "EXP 0->5 V (TD1=1u, TAU1=10n)"
        );
    }

    #[test]
    fn test_sffm_label() {
        let c = source(
            ComponentType::VoltageSourceSffm,
            "",
            "vo=0 va=1 fc=100k fs=1k",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "SFFM 1 V (offset 0, fc=100k, fs=1k)"
        );
    }

    #[test]
    fn test_noise_label() {
        let c = source(
            ComponentType::CurrentSourceNoise,
            "0",
            "noise_type=flicker noiseval=2e-24 dc=1m",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "NOISE flicker=2e-24 A^2/Hz (DC 1m A)"
        );
    }

    #[test]
    fn test_placeholder_primary_is_sanitized() {
        let c = source(ComponentType::VoltageSourceSin, "dcoffset", "va=1 freq=1k");
        assert_eq!(
            component_value_label(&c).as_ref(),
            "SIN 1 V @ 1k (offset 0)"
        );
    }
}

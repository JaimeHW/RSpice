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
        ComponentType::VoltageSource => format_dc_source_label(&params, primary, "V"),
        ComponentType::CurrentSource => format_dc_source_label(&params, primary, "A"),
        ComponentType::VoltageSourceAc => format_ac_source_label(&params, primary, "V"),
        ComponentType::CurrentSourceAc => format_ac_source_label(&params, primary, "A"),
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

fn format_dc_source_label(params: &HashMap<String, String>, primary: &str, unit: &str) -> String {
    let mut lines = vec![format!("DC: {} {}", primary_or_default(primary, "0"), unit)];

    if let Some(ac) = get_param_optional(params, &["ac", "acmag", "ac_mag"]) {
        if !is_default_value(ac, "0") {
            lines.push(format!("AC: {} {}", ac, unit));
        }
    }
    if let Some(phase) = get_param_optional(params, &["acphase", "phase"]) {
        if !is_default_value(phase, "0") {
            lines.push(format!("Phase: {} deg", phase));
        }
    }

    push_parasitic_lines(&mut lines, params);
    lines.join("\n")
}

fn format_ac_source_label(params: &HashMap<String, String>, primary: &str, unit: &str) -> String {
    let mag = get_param_with_aliases(params, &["ac", "acmag", "ac_mag"], primary, "1");
    let mut lines = vec![format!("AC: {} {}", mag, unit)];

    if let Some(phase) = get_param_optional(params, &["acphase", "phase"]) {
        if !is_default_value(phase, "0") {
            lines.push(format!("Phase: {} deg", phase));
        }
    }
    if let Some(dc) = get_param_optional(params, &["dc"]) {
        if !is_default_value(dc, "0") {
            lines.push(format!("DC: {} {}", dc, unit));
        }
    }

    push_parasitic_lines(&mut lines, params);
    lines.join("\n")
}

fn format_pulse_label(params: &HashMap<String, String>, primary: &str, is_voltage: bool) -> String {
    let unit = if is_voltage { "V" } else { "A" };
    let low_default = "0";
    let high_default = if is_voltage { "1" } else { "1m" };

    let low = get_param_with_aliases(
        params,
        if is_voltage { &["v1"] } else { &["i1"] },
        primary,
        low_default,
    );
    let high = get_param_with_aliases(
        params,
        if is_voltage { &["v2"] } else { &["i2"] },
        "",
        high_default,
    );
    let width = get_param_with_aliases(params, &["pw"], "", "1u");
    let period = get_param_with_aliases(params, &["per", "period"], "", "2u");

    let mut lines = vec![
        "PULSE".to_string(),
        format!("Low: {} {}", low, unit),
        format!("High: {} {}", high, unit),
        format!("PW: {}", width),
        format!("PER: {}", period),
    ];

    if let Some(td) = get_param_optional(params, &["td"]) {
        if !is_default_value(td, "0") {
            lines.push(format!("TD: {}", td));
        }
    }
    if let Some(tr) = get_param_optional(params, &["tr"]) {
        if !is_default_value(tr, "1n") {
            lines.push(format!("TR: {}", tr));
        }
    }
    if let Some(tf) = get_param_optional(params, &["tf"]) {
        if !is_default_value(tf, "1n") {
            lines.push(format!("TF: {}", tf));
        }
    }

    lines.join("\n")
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

    let mut lines = vec![
        "SIN".to_string(),
        format!("Amp: {} {}", amp, unit),
        format!("Freq: {}", freq),
    ];

    if !is_default_value(offset, "0") {
        lines.push(format!("Offset: {} {}", offset, unit));
    }
    if let Some(phase) = get_param_optional(params, &["phase"]) {
        if !is_default_value(phase, "0") {
            lines.push(format!("Phase: {} deg", phase));
        }
    }
    if let Some(td) = get_param_optional(params, &["td"]) {
        if !is_default_value(td, "0") {
            lines.push(format!("TD: {}", td));
        }
    }
    if let Some(theta) = get_param_optional(params, &["theta"]) {
        if !is_default_value(theta, "0") {
            lines.push(format!("Theta: {}", theta));
        }
    }

    lines.join("\n")
}

fn format_pwl_label(params: &HashMap<String, String>, primary: &str) -> String {
    let data = get_param_with_aliases(params, &["pwl_data"], primary, "0 0 1u 1");
    let (points, t0, tn) = summarize_pwl(data);

    let mut lines = vec![
        "PWL".to_string(),
        format!("Pts: {}", points),
        format!("T0: {}", t0),
        format!("Tn: {}", tn),
    ];

    if let Some(td) = get_param_optional(params, &["td"]) {
        if !is_default_value(td, "0") {
            lines.push(format!("TD: {}", td));
        }
    }
    if let Some(repeat) = get_param_optional(params, &["repeat"]) {
        if is_truthy(repeat) {
            lines.push("Repeat: on".to_string());
        }
    }

    lines.join("\n")
}

fn format_exp_label(params: &HashMap<String, String>, primary: &str, is_voltage: bool) -> String {
    let unit = if is_voltage { "V" } else { "A" };
    let low_default = "0";
    let high_default = if is_voltage { "1" } else { "1m" };

    let low = get_param_with_aliases(
        params,
        if is_voltage { &["v1"] } else { &["i1"] },
        primary,
        low_default,
    );
    let high = get_param_with_aliases(
        params,
        if is_voltage { &["v2"] } else { &["i2"] },
        "",
        high_default,
    );
    let td1 = get_param_with_aliases(params, &["td1"], "", "0");
    let tau1 = get_param_with_aliases(params, &["tau1"], "", "1u");
    let td2 = get_param_with_aliases(params, &["td2"], "", "5u");
    let tau2 = get_param_with_aliases(params, &["tau2"], "", "1u");

    [
        "EXP".to_string(),
        format!("Low: {} {}", low, unit),
        format!("High: {} {}", high, unit),
        format!("TD1: {}", td1),
        format!("TAU1: {}", tau1),
        format!("TD2: {}", td2),
        format!("TAU2: {}", tau2),
    ]
    .join("\n")
}

fn format_sffm_label(params: &HashMap<String, String>, primary: &str) -> String {
    let offset = get_param_with_aliases(params, &["vo"], primary, "0");
    let amp = get_param_with_aliases(params, &["va"], "", "1");
    let fc = get_param_with_aliases(params, &["fc"], "", "1k");
    let mdi = get_param_with_aliases(params, &["mdi"], "", "1");
    let fs = get_param_with_aliases(params, &["fs"], "", "10");

    let mut lines = vec![
        "SFFM".to_string(),
        format!("Amp: {} V", amp),
        format!("Fc: {}", fc),
        format!("Fs: {}", fs),
        format!("Mdi: {}", mdi),
    ];

    if !is_default_value(offset, "0") {
        lines.push(format!("Offset: {} V", offset));
    }

    lines.join("\n")
}

fn format_noise_label(params: &HashMap<String, String>, primary: &str) -> String {
    let dc = get_param_with_aliases(params, &["dc"], primary, "0");
    let noise_type = get_param_with_aliases(params, &["noise_type"], "", "white");
    let noise_val = get_param_with_aliases(params, &["noiseval"], "", "1e-24");

    let mut lines = vec![
        "NOISE".to_string(),
        format!("Type: {}", noise_type),
        format!("Density: {} A^2/Hz", noise_val),
    ];

    if !is_default_value(dc, "0") {
        lines.push(format!("DC: {} A", dc));
    }
    if let Some(kf) = get_param_optional(params, &["kf"]) {
        if !is_default_value(kf, "0") {
            lines.push(format!("KF: {}", kf));
        }
    }
    if let Some(af) = get_param_optional(params, &["af"]) {
        if !is_default_value(af, "1") {
            lines.push(format!("AF: {}", af));
        }
    }

    lines.join("\n")
}

fn push_parasitic_lines(lines: &mut Vec<String>, params: &HashMap<String, String>) {
    if let Some(rs) = get_param_optional(params, &["rs"]) {
        if !is_default_value(rs, "0") {
            lines.push(format!("Rs: {} ohm", rs));
        }
    }
    if let Some(rp) = get_param_optional(params, &["rp"]) {
        if !is_default_value(rp, "inf")
            && !is_default_value(rp, "infinity")
            && !is_default_value(rp, "1e309")
        {
            lines.push(format!("Rp: {} ohm", rp));
        }
    }
    if let Some(cpar) = get_param_optional(params, &["cpar"]) {
        if !is_default_value(cpar, "0") {
            lines.push(format!("Cpar: {} F", cpar));
        }
    }
}

fn summarize_pwl(data: &str) -> (usize, String, String) {
    let tokens: Vec<&str> = data.split_whitespace().collect();
    if tokens.len() < 2 || !crate::utils::numeric::is_multiple_of(tokens.len(), 2) {
        return (0, "-".to_string(), "-".to_string());
    }
    let points = tokens.len() / 2;
    let t0 = tokens[0].to_string();
    let t_last = tokens[tokens.len() - 2].to_string();
    (points, t0, t_last)
}

fn get_param_optional<'a>(params: &'a HashMap<String, String>, keys: &[&str]) -> Option<&'a str> {
    for key in keys {
        if let Some(value) = params.get(*key).map(String::as_str) {
            let trimmed = value.trim();
            if !trimmed.is_empty() && !looks_like_placeholder(trimmed) {
                return Some(trimmed);
            }
        }
    }
    None
}

fn get_param_with_aliases<'a>(
    params: &'a HashMap<String, String>,
    keys: &[&str],
    primary_fallback: &'a str,
    default: &'a str,
) -> &'a str {
    if let Some(value) = get_param_optional(params, keys) {
        return value;
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

fn is_default_value(value: &str, default: &str) -> bool {
    let v = value.trim();
    let d = default.trim();
    if v.eq_ignore_ascii_case(d) {
        return true;
    }

    match (parse_numeric(v), parse_numeric(d)) {
        (Some(a), Some(b)) => {
            if !a.is_finite() || !b.is_finite() {
                return a == b;
            }
            let scale = a.abs().max(b.abs());
            let abs_tol = 1e-30;
            let rel_tol = 1e-12;
            (a - b).abs() <= abs_tol + rel_tol * scale
        }
        _ => false,
    }
}

fn parse_numeric(value: &str) -> Option<f64> {
    crate::properties::parse_engineering_value(value)
        .ok()
        .or_else(|| value.parse::<f64>().ok())
}

fn is_truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
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
        assert_eq!(component_value_label(&c).as_ref(), "DC: 5 V");
    }

    #[test]
    fn test_dc_current_source_with_ac_phase_and_parasitics() {
        let c = source(
            ComponentType::CurrentSource,
            "2m",
            "ac=1m acphase=45 rs=10 rp=1e6 cpar=5p",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "DC: 2m A\nAC: 1m A\nPhase: 45 deg\nRs: 10 ohm\nRp: 1e6 ohm\nCpar: 5p F"
        );
    }

    #[test]
    fn test_ac_source_label_with_phase() {
        let c = source(ComponentType::VoltageSourceAc, "1", "acphase=90");
        assert_eq!(component_value_label(&c).as_ref(), "AC: 1 V\nPhase: 90 deg");
    }

    #[test]
    fn test_ac_source_includes_non_default_dc_offset() {
        let c = source(ComponentType::CurrentSourceAc, "3m", "dc=200u");
        assert_eq!(component_value_label(&c).as_ref(), "AC: 3m A\nDC: 200u A");
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
            "SIN\nAmp: 2.5 V\nFreq: 10k\nOffset: 0.5 V\nPhase: 45 deg"
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
            "SIN\nAmp: 3m A\nFreq: 1k\nOffset: 1m A"
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
            "PULSE\nLow: 0 V\nHigh: 1.8 V\nPW: 5n\nPER: 10n"
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
            "PULSE\nLow: 0 A\nHigh: 2m A\nPW: 10n\nPER: 20n"
        );
    }

    #[test]
    fn test_pulse_non_default_timing_fields_are_appended() {
        let c = source(
            ComponentType::VoltageSourcePulse,
            "0",
            "v1=0 v2=1.2 pw=2n per=8n td=1n tr=2n tf=3n",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "PULSE\nLow: 0 V\nHigh: 1.2 V\nPW: 2n\nPER: 8n\nTD: 1n\nTR: 2n\nTF: 3n"
        );
    }

    #[test]
    fn test_pwl_label_summarizes_long_data() {
        let c = source(
            ComponentType::VoltageSourcePwl,
            "",
            "pwl_data=\"0 0 1n 1 2n 0 3n 1 4n 0 5n 1\"",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "PWL\nPts: 6\nT0: 0\nTn: 5n"
        );
    }

    #[test]
    fn test_pwl_repeat_and_delay_are_displayed_when_enabled() {
        let c = source(
            ComponentType::CurrentSourcePwl,
            "",
            "pwl_data=\"0 0 1u 1\" td=100n repeat=true",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "PWL\nPts: 2\nT0: 0\nTn: 1u\nTD: 100n\nRepeat: on"
        );
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
            "EXP\nLow: 0 V\nHigh: 5 V\nTD1: 1u\nTAU1: 10n\nTD2: 10u\nTAU2: 10n"
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
            "SFFM\nAmp: 1 V\nFc: 100k\nFs: 1k\nMdi: 1"
        );
    }

    #[test]
    fn test_sffm_includes_non_default_offset() {
        let c = source(
            ComponentType::VoltageSourceSffm,
            "",
            "vo=0.5 va=2 fc=100k fs=1k mdi=0.2",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "SFFM\nAmp: 2 V\nFc: 100k\nFs: 1k\nMdi: 0.2\nOffset: 0.5 V"
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
            "NOISE\nType: flicker\nDensity: 2e-24 A^2/Hz\nDC: 1m A"
        );
    }

    #[test]
    fn test_noise_includes_non_default_flicker_coefficients() {
        let c = source(
            ComponentType::CurrentSourceNoise,
            "0",
            "noise_type=flicker noiseval=2e-24 kf=1e-21 af=2",
        );
        assert_eq!(
            component_value_label(&c).as_ref(),
            "NOISE\nType: flicker\nDensity: 2e-24 A^2/Hz\nKF: 1e-21\nAF: 2"
        );
    }

    #[test]
    fn test_placeholder_primary_is_sanitized() {
        let c = source(ComponentType::VoltageSourceSin, "dcoffset", "va=1 freq=1k");
        assert_eq!(
            component_value_label(&c).as_ref(),
            "SIN\nAmp: 1 V\nFreq: 1k"
        );
    }

    #[test]
    fn test_primary_placeholder_falls_back_to_default_on_dc_source() {
        let c = source(ComponentType::VoltageSource, "value", "");
        assert_eq!(component_value_label(&c).as_ref(), "DC: 0 V");
    }

    #[test]
    fn test_placeholder_aliases_do_not_override_fallback_value() {
        let c = source(ComponentType::VoltageSourceAc, "2", "ac=acmag");
        assert_eq!(component_value_label(&c).as_ref(), "AC: 2 V");
    }

    #[test]
    fn test_numeric_default_detection_with_equivalent_formats() {
        assert!(is_default_value("1e-6", "1u"));
        assert!(is_default_value("0.0", "0"));
        assert!(!is_default_value("1e-5", "1u"));
        assert!(!is_default_value("1e-21", "0"));
    }
}

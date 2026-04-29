use super::*;

pub(super) fn truncate_legend_trace_name(
    painter: &Painter,
    text: &str,
    font: FontId,
    max_width: f32,
) -> String {
    const ELLIPSIS: &str = "...";
    if text.is_empty() || max_width <= 0.0 {
        return String::new();
    }
    let text_width = measure_text_width(painter, text, font.clone(), Color32::WHITE);
    if text_width <= max_width {
        return text.to_owned();
    }
    let ellipsis_width = measure_text_width(painter, ELLIPSIS, font.clone(), Color32::WHITE);
    if ellipsis_width >= max_width {
        return ELLIPSIS.to_owned();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut low = 0usize;
    let mut high = chars.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let prefix: String = chars.iter().take(mid).collect();
        let candidate = format!("{prefix}{ELLIPSIS}");
        let width = measure_text_width(painter, &candidate, font.clone(), Color32::WHITE);
        if width <= max_width {
            low = mid;
        } else {
            high = mid.saturating_sub(1);
        }
    }

    let prefix: String = chars.iter().take(low).collect();
    format!("{prefix}{ELLIPSIS}")
}

pub(super) fn format_optional_value(value: Option<f64>, unit: &str) -> String {
    value
        .map(|v| axis::format_with_si_prefix(v, unit, 4))
        .unwrap_or_else(|| "--".to_string())
}

pub(super) fn format_optional_time(value: Option<f64>) -> String {
    value
        .map(axis::format_time)
        .unwrap_or_else(|| "--".to_string())
}

pub(super) fn format_optional_freq(value: Option<f64>) -> String {
    value
        .map(axis::format_frequency)
        .unwrap_or_else(|| "--".to_string())
}

//! Parametric Sweep Configuration Panel
//!
//! UI component for configuring and running parametric sweeps.
//! Supports linear, logarithmic, and list-based parameter variations.

use dioxus::prelude::*;

use crate::theme::Theme;

/// Sweep type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SweepType {
    Linear,
    Logarithmic,
    List,
}

impl std::fmt::Display for SweepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SweepType::Linear => write!(f, "Linear"),
            SweepType::Logarithmic => write!(f, "Log"),
            SweepType::List => write!(f, "List"),
        }
    }
}

/// Sweep parameter configuration
#[derive(Debug, Clone, PartialEq)]
pub struct SweepConfig {
    /// Parameter name (e.g., "R1", "C1", "VIN")
    pub parameter: String,
    /// Sweep type
    pub sweep_type: SweepType,
    /// Start value
    pub start: f64,
    /// End value
    pub end: f64,
    /// Number of steps
    pub steps: usize,
    /// List of values (for List sweep type)
    pub values: Vec<f64>,
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self {
            parameter: String::new(),
            sweep_type: SweepType::Linear,
            start: 1.0,
            end: 10.0,
            steps: 10,
            values: vec![],
        }
    }
}

impl SweepConfig {
    /// Generate sweep values based on configuration
    pub fn generate_values(&self) -> Vec<f64> {
        match self.sweep_type {
            SweepType::Linear => {
                let step = (self.end - self.start) / (self.steps.max(1) - 1) as f64;
                (0..self.steps)
                    .map(|i| self.start + step * i as f64)
                    .collect()
            }
            SweepType::Logarithmic => {
                if self.start <= 0.0 || self.end <= 0.0 {
                    return vec![];
                }
                let log_start = self.start.ln();
                let log_end = self.end.ln();
                let log_step = (log_end - log_start) / (self.steps.max(1) - 1) as f64;
                (0..self.steps)
                    .map(|i| (log_start + log_step * i as f64).exp())
                    .collect()
            }
            SweepType::List => self.values.clone(),
        }
    }
}

/// Props for sweep panel
#[derive(Props, Clone, PartialEq)]
pub struct SweepPanelProps {
    /// Whether the panel is visible
    pub visible: bool,
    /// Callback to close the panel
    pub on_close: EventHandler<()>,
    /// Callback when sweep should run
    pub on_run_sweep: EventHandler<SweepConfig>,
    /// Panel position (managed by parent for global drag)
    #[props(default = (60, 50))]
    pub position: (i32, i32),
    /// Callback when drag starts
    #[props(default)]
    pub on_drag_start: EventHandler<(i32, i32)>,
}

/// Parametric sweep configuration panel
#[component]
pub fn SweepPanel(props: SweepPanelProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let mut config = use_signal(SweepConfig::default);
    let mut param_input = use_signal(|| "R1".to_string());
    let mut start_input = use_signal(|| "1k".to_string());
    let mut end_input = use_signal(|| "100k".to_string());
    let mut steps_input = use_signal(|| "10".to_string());
    let mut list_input = use_signal(|| "1k, 10k, 100k".to_string());

    if !props.visible {
        return rsx! {};
    }

    let (x, y) = props.position;
    let sweep_type = config.read().sweep_type;

    rsx! {
        div {
            class: "sweep-panel",
            style: "
                position: absolute;
                left: {x}px;
                top: {y}px;
                width: 320px;
                background: {th.bg_secondary()};
                border: 1px solid {th.border()};
                border-radius: {Theme::RADIUS_MD};
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
                z-index: 100;
                overflow: hidden;
            ",

            // Draggable Header
            div {
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: {Theme::SPACING_SM} {Theme::SPACING_MD};
                    border-bottom: 1px solid {th.border()};
                    background: {th.surface()};
                    cursor: move;
                    user-select: none;
                ",
                onmousedown: {
                    let position = props.position;
                    move |e| {
                        let offset_x = e.client_coordinates().x as i32 - position.0;
                        let offset_y = e.client_coordinates().y as i32 - position.1;
                        props.on_drag_start.call((offset_x, offset_y));
                    }
                },

                span {
                    style: "
                        font-weight: 600;
                        font-size: {Theme::FONT_SIZE_SM};
                        color: {th.text_primary()};
                    ",
                    "Parametric Sweep"
                }

                button {
                    style: "
                        background: none;
                        border: none;
                        color: {th.text_muted()};
                        cursor: pointer;
                        padding: 4px;
                        font-size: 14px;
                    ",
                    onmousedown: move |e| e.stop_propagation(),
                    onclick: move |_| props.on_close.call(()),
                    "✕"
                }
            }

            // Content
            div {
                style: "padding: {Theme::SPACING_MD};",

                // Parameter input
                FormField {
                    label: "Parameter",
                    input {
                        r#type: "text",
                        placeholder: "R1, C1, VIN...",
                        value: "{param_input}",
                        style: "{input_style(&th)}",
                        oninput: move |e| {
                            param_input.set(e.value());
                        },
                    }
                }

                // Sweep type selector
                FormField {
                    label: "Sweep Type",
                    div {
                        style: "display: flex; gap: 8px;",

                        SweepTypeButton {
                            label: "Linear",
                            active: sweep_type == SweepType::Linear,
                            onclick: move |_| {
                                config.write().sweep_type = SweepType::Linear;
                            },
                        }
                        SweepTypeButton {
                            label: "Log",
                            active: sweep_type == SweepType::Logarithmic,
                            onclick: move |_| {
                                config.write().sweep_type = SweepType::Logarithmic;
                            },
                        }
                        SweepTypeButton {
                            label: "List",
                            active: sweep_type == SweepType::List,
                            onclick: move |_| {
                                config.write().sweep_type = SweepType::List;
                            },
                        }
                    }
                }

                // Range inputs (for Linear/Log)
                if sweep_type != SweepType::List {
                    div {
                        style: "display: flex; gap: {Theme::SPACING_SM};",

                        div {
                            style: "flex: 1;",
                            FormField {
                                label: "Start",
                                input {
                                    r#type: "text",
                                    value: "{start_input}",
                                    style: "{input_style(&th)}",
                                    oninput: move |e| {
                                        start_input.set(e.value());
                                    },
                                }
                            }
                        }

                        div {
                            style: "flex: 1;",
                            FormField {
                                label: "End",
                                input {
                                    r#type: "text",
                                    value: "{end_input}",
                                    style: "{input_style(&th)}",
                                    oninput: move |e| {
                                        end_input.set(e.value());
                                    },
                                }
                            }
                        }

                        div {
                            style: "flex: 0 0 60px;",
                            FormField {
                                label: "Steps",
                                input {
                                    r#type: "number",
                                    value: "{steps_input}",
                                    min: "2",
                                    max: "100",
                                    style: "{input_style(&th)}",
                                    oninput: move |e| {
                                        steps_input.set(e.value());
                                    },
                                }
                            }
                        }
                    }
                }

                // List input (for List type)
                if sweep_type == SweepType::List {
                    FormField {
                        label: "Values (comma-separated)",
                        input {
                            r#type: "text",
                            placeholder: "1k, 10k, 100k",
                            value: "{list_input}",
                            style: "{input_style(&th)}",
                            oninput: move |e| {
                                list_input.set(e.value());
                            },
                        }
                    }
                }

                // Preview
                div {
                    style: "
                        margin-top: {Theme::SPACING_MD};
                        padding: {Theme::SPACING_SM};
                        background: {th.bg_primary()};
                        border-radius: {Theme::RADIUS_SM};
                        font-size: 11px;
                        color: {th.text_secondary()};
                    ",

                    div {
                        style: "margin-bottom: 4px; font-weight: 500;",
                        "Preview:"
                    }

                    {
                        let values = config.read().generate_values();
                        let preview: String = values.iter()
                            .take(5)
                            .map(|v| format_value(*v))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let suffix = if values.len() > 5 {
                            format!("... ({} total)", values.len())
                        } else {
                            String::new()
                        };
                        rsx! {
                            span {
                                style: "font-family: monospace;",
                                "{preview}{suffix}"
                            }
                        }
                    }
                }

                // Run button
                button {
                    style: "
                        width: 100%;
                        margin-top: {Theme::SPACING_MD};
                        padding: {Theme::SPACING_SM} {Theme::SPACING_MD};
                        background: {th.accent_primary()};
                        border: none;
                        border-radius: {Theme::RADIUS_SM};
                        color: white;
                        font-size: {Theme::FONT_SIZE_SM};
                        font-weight: 600;
                        cursor: pointer;
                    ",
                    onclick: move |_| {
                        // Build config from current input values
                        let mut cfg = config.read().clone();
                        cfg.parameter = param_input.read().clone();
                        cfg.start = parse_value(&start_input.read());
                        cfg.end = parse_value(&end_input.read());
                        cfg.steps = steps_input.read().parse().unwrap_or(10);
                        cfg.values = parse_list(&list_input.read());
                        props.on_run_sweep.call(cfg);
                    },
                    "▶ Run Sweep"
                }
            }
        }
    }
}

/// Form field wrapper
#[component]
fn FormField(label: &'static str, children: Element) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "margin-bottom: {Theme::SPACING_SM};",

            label {
                style: "
                    display: block;
                    font-size: 11px;
                    color: {th.text_secondary()};
                    margin-bottom: 4px;
                ",
                "{label}"
            }

            {children}
        }
    }
}

/// Sweep type toggle button
#[component]
fn SweepTypeButton(
    label: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let bg = if active {
        th.accent_primary()
    } else {
        th.surface()
    };
    let color = if active { "#ffffff" } else { th.text_primary() };

    rsx! {
        button {
            style: "
                flex: 1;
                padding: 6px 12px;
                background: {bg};
                border: 1px solid {th.border()};
                border-radius: {Theme::RADIUS_SM};
                color: {color};
                font-size: 11px;
                cursor: pointer;
                transition: all {Theme::TRANSITION_FAST};
            ",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

/// Input style helper
fn input_style(th: &Theme) -> String {
    format!(
        "width: 100%; padding: 6px 8px; background: {}; border: 1px solid {}; border-radius: {}; color: {}; font-size: 12px; font-family: {}; outline: none;",
        th.bg_primary(),
        th.border(),
        Theme::RADIUS_SM,
        th.text_primary(),
        Theme::FONT_MONO
    )
}

/// Parse a value with SI prefix (e.g., "10k" -> 10000)
fn parse_value(s: &str) -> f64 {
    let s = s.trim();
    if s.is_empty() {
        return 0.0;
    }

    // Check for "meg" suffix first (before single-char suffixes)
    if s.to_lowercase().ends_with("meg") {
        return s[..s.len() - 3].parse::<f64>().unwrap_or(0.0) * 1e6;
    }

    // Check for SI suffix
    let (num_str, multiplier) = if let Some(c) = s.chars().last() {
        match c.to_ascii_lowercase() {
            'f' => (&s[..s.len() - 1], 1e-15),
            'p' => (&s[..s.len() - 1], 1e-12),
            'n' => (&s[..s.len() - 1], 1e-9),
            'u' | 'µ' => (&s[..s.len() - 1], 1e-6),
            'm' => (&s[..s.len() - 1], 1e-3),
            'k' => (&s[..s.len() - 1], 1e3),
            'g' => (&s[..s.len() - 1], 1e9),
            _ => (s, 1.0),
        }
    } else {
        (s, 1.0)
    };

    num_str.parse::<f64>().unwrap_or(0.0) * multiplier
}

/// Parse a comma-separated list of values
fn parse_list(s: &str) -> Vec<f64> {
    s.split(',')
        .map(|v| parse_value(v.trim()))
        .filter(|v| *v != 0.0 || s.contains("0"))
        .collect()
}

/// Format value with SI prefix
fn format_value(v: f64) -> String {
    let abs = v.abs();
    if abs >= 1e9 {
        format!("{:.2}G", v / 1e9)
    } else if abs >= 1e6 {
        format!("{:.2}M", v / 1e6)
    } else if abs >= 1e3 {
        format!("{:.2}k", v / 1e3)
    } else if abs >= 1.0 {
        format!("{:.2}", v)
    } else if abs >= 1e-3 {
        format!("{:.2}m", v * 1e3)
    } else if abs >= 1e-6 {
        format!("{:.2}µ", v * 1e6)
    } else if abs >= 1e-9 {
        format!("{:.2}n", v * 1e9)
    } else if abs >= 1e-12 {
        format!("{:.2}p", v * 1e12)
    } else {
        format!("{:.2e}", v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert!((parse_value("10k") - 10000.0).abs() < 0.1);
        assert!((parse_value("4.7u") - 4.7e-6).abs() < 1e-9);
        assert!((parse_value("100n") - 100e-9).abs() < 1e-12);
        assert!((parse_value("2.2meg") - 2.2e6).abs() < 1e3);
    }

    #[test]
    fn test_linear_sweep() {
        let config = SweepConfig {
            sweep_type: SweepType::Linear,
            start: 1.0,
            end: 10.0,
            steps: 10,
            ..Default::default()
        };
        let values = config.generate_values();
        assert_eq!(values.len(), 10);
        assert!((values[0] - 1.0).abs() < 0.001);
        assert!((values[9] - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_log_sweep() {
        let config = SweepConfig {
            sweep_type: SweepType::Logarithmic,
            start: 1.0,
            end: 1000.0,
            steps: 4,
            ..Default::default()
        };
        let values = config.generate_values();
        assert_eq!(values.len(), 4);
        assert!((values[0] - 1.0).abs() < 0.001);
        assert!((values[1] - 10.0).abs() < 0.1);
        assert!((values[2] - 100.0).abs() < 1.0);
        assert!((values[3] - 1000.0).abs() < 1.0);
    }
}

//! Verilog-A Model Inspector Component
//!
//! Collapsible panel for displaying compiled Verilog-A model details.
//! Shows terminals, parameters, and compilation status.

use dioxus::prelude::*;

use crate::theme::Theme;

/// Compiled Verilog-A model information for display
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VerilogAModelInfo {
    /// Model/module name
    pub name: String,
    /// Source file path
    pub source_path: String,
    /// Whether compilation succeeded
    pub is_compiled: bool,
    /// Compilation error message (if any)
    pub error: Option<String>,
    /// Terminal names
    pub terminals: Vec<String>,
    /// Parameter definitions
    pub parameters: Vec<ParameterInfo>,
    /// Number of internal nodes
    pub internal_nodes: usize,
}

/// Parameter definition
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    /// Default value
    pub default_value: f64,
    /// Minimum value constraint
    pub min: Option<f64>,
    /// Maximum value constraint
    pub max: Option<f64>,
}

/// Convert from compiled model to display info
impl From<&rspice_veriloga::CompiledModel> for VerilogAModelInfo {
    fn from(model: &rspice_veriloga::CompiledModel) -> Self {
        let terminals = model.terminal_names.iter().map(|t| t.to_string()).collect();

        let parameters = model
            .parameters
            .iter()
            .map(|p| ParameterInfo {
                name: p.name.to_string(),
                default_value: p.default,
                min: p.min,
                max: p.max,
            })
            .collect();

        Self {
            name: model.name.to_string(),
            source_path: String::new(),
            is_compiled: true,
            error: None,
            terminals,
            parameters,
            internal_nodes: model.internal_nodes,
        }
    }
}

/// Props for VerilogAInspector
#[derive(Props, Clone, PartialEq)]
pub struct VerilogAInspectorProps {
    /// Model information to display
    pub model: VerilogAModelInfo,
    /// Whether panel is expanded
    #[props(default = true)]
    pub expanded: bool,
    /// Callback when expansion state changes
    #[props(default)]
    pub on_toggle: Option<EventHandler<bool>>,
    /// Callback when parameter value is edited
    #[props(default)]
    pub on_param_change: Option<EventHandler<(String, f64)>>,
}

/// Verilog-A Model Inspector Panel
///
/// A premium, collapsible panel showing detailed information about a compiled
/// Verilog-A model including terminals and parameters.
#[component]
pub fn VerilogAInspector(props: VerilogAInspectorProps) -> Element {
    let theme = use_context::<Theme>();
    let mut is_expanded = use_signal(|| props.expanded);

    // Clone props for use in closures
    let model = props.model.clone();
    let on_toggle = props.on_toggle.clone();

    let toggle_expansion = move |_| {
        let new_state = !*is_expanded.read();
        is_expanded.set(new_state);
        if let Some(handler) = &on_toggle {
            handler.call(new_state);
        }
    };

    // Pre-compute styles
    let bg_secondary = theme.bg_secondary();
    let bg_tertiary = theme.bg_tertiary();
    let border = theme.border();
    let surface = theme.surface();
    let text_primary = theme.text_primary();
    let text_secondary = theme.text_secondary();
    let text_muted = theme.text_muted();
    let border_subtle = theme.border_subtle();
    let accent_success = theme.accent_success();
    let accent_error = theme.accent_error();
    let accent_primary = theme.accent_primary();

    let panel_style = format!(
        "background: linear-gradient(135deg, {bg_secondary}ee, {bg_tertiary}dd); \
         backdrop-filter: blur(12px); \
         border: 1px solid {border}; \
         border-radius: {}; \
         margin: 8px; \
         overflow: hidden; \
         transition: all {} ease; \
         box-shadow: 0 4px 24px rgba(0,0,0,0.2);",
        Theme::RADIUS_LG,
        Theme::TRANSITION_NORMAL,
    );

    let expanded = *is_expanded.read();
    let header_bg = if expanded { surface } else { "transparent" };
    let header_border = if expanded { border } else { "transparent" };

    let header_style = format!(
        "display: flex; \
         align-items: center; \
         justify-content: space-between; \
         padding: 12px 16px; \
         cursor: pointer; \
         user-select: none; \
         background: {header_bg}; \
         border-bottom: 1px solid {header_border}; \
         transition: background {} ease;",
        Theme::TRANSITION_FAST,
    );

    let chevron_rotation = if expanded { 90 } else { 0 };
    let status_color = if model.is_compiled {
        accent_success
    } else {
        accent_error
    };
    let status_label = if model.is_compiled {
        "Compiled"
    } else {
        "Error"
    };

    rsx! {
        div {
            style: "{panel_style}",

            // Header
            div {
                style: "{header_style}",
                onclick: toggle_expansion,

                // Model name and status
                div {
                    style: "display: flex; align-items: center; gap: 12px;",

                    // Chevron icon
                    span {
                        style: format!("font-size: 12px; color: {text_secondary}; transition: transform {trans}; transform: rotate({chevron_rotation}deg);", trans = Theme::TRANSITION_FAST),
                        "▶"
                    }

                    // Model name
                    span {
                        style: format!("font-family: {font}; font-size: {size}; font-weight: 600; color: {text_primary};", font = Theme::FONT_MONO, size = Theme::FONT_SIZE_BASE),
                        "{model.name}"
                    }

                    // Status badge
                    span {
                        style: "padding: 2px 8px; background: {status_color}22; color: {status_color}; \
                                border: 1px solid {status_color}44; border-radius: 12px; \
                                font-size: 11px; font-weight: 500; text-transform: uppercase; letter-spacing: 0.5px;",
                        "{status_label}"
                    }
                }

                // Terminal count badge
                div {
                    style: format!("padding: 4px 8px; background: {surface}; border-radius: 4px; font-size: {size};", size = Theme::FONT_SIZE_SM),
                    span {
                        style: "color: {text_secondary};",
                        "{model.terminals.len()} terminals"
                    }
                }
            }

            // Expandable content
            if expanded {
                div {
                    style: "padding: 16px;",

                    // Error display
                    if let Some(ref error) = model.error {
                        div {
                            style: format!("padding: 12px 16px; background: {accent_error}15; border: 1px solid {accent_error}33; \
                                    border-radius: {radius}; color: {accent_error}; font-family: {font}; font-size: {size}; \
                                    margin-bottom: 16px; white-space: pre-wrap;",
                                    radius = Theme::RADIUS_MD, font = Theme::FONT_MONO, size = Theme::FONT_SIZE_SM),
                            "{error}"
                        }
                    }

                    // Terminals section
                    if !model.terminals.is_empty() {
                        div {
                            style: "margin-bottom: 16px;",

                            // Section header
                            div {
                                style: format!("display: flex; align-items: center; gap: 8px; margin-bottom: 8px; \
                                        font-size: {size}; font-weight: 500; color: {text_secondary};", size = Theme::FONT_SIZE_SM),
                                span { "◉" }
                                span { "Terminals" }
                            }

                            // Terminal badges
                            div {
                                style: "display: flex; flex-wrap: wrap; gap: 8px;",
                                for terminal in model.terminals.iter() {
                                    div {
                                        style: format!("display: flex; align-items: center; gap: 6px; \
                                                padding: 6px 12px; background: {surface}; \
                                                border: 1px solid {border}; border-radius: 6px; \
                                                font-family: {font}; font-size: {size};", font = Theme::FONT_MONO, size = Theme::FONT_SIZE_SM),
                                        span { style: "color: {accent_primary};", "↔" }
                                        span { style: "color: {text_primary};", "{terminal}" }
                                    }
                                }
                            }
                        }
                    }

                    // Parameters section
                    if !model.parameters.is_empty() {
                        div {
                            style: "margin-bottom: 16px;",

                            // Section header
                            div {
                                style: format!("display: flex; align-items: center; gap: 8px; margin-bottom: 8px; \
                                        font-size: {size}; font-weight: 500; color: {text_secondary};", size = Theme::FONT_SIZE_SM),
                                span { "⚙" }
                                span { "Parameters" }
                            }

                            // Parameter rows
                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",
                                for param in model.parameters.iter() {
                                    ParameterRow {
                                        info: param.clone(),
                                        on_change: props.on_param_change.clone(),
                                    }
                                }
                            }
                        }
                    }

                    // Internal nodes info
                    if model.internal_nodes > 0 {
                        div {
                            style: format!("font-size: {size}; color: {text_muted};", size = Theme::FONT_SIZE_SM),
                            "Internal nodes: {model.internal_nodes}"
                        }
                    }

                    // Source file info
                    if !model.source_path.is_empty() {
                        div {
                            style: format!("margin-top: 16px; padding-top: 12px; border-top: 1px solid {border_subtle}; \
                                    font-size: {size}; color: {text_muted};", size = Theme::FONT_SIZE_SM),
                            "Source: {model.source_path}"
                        }
                    }
                }
            }
        }
    }
}

/// Parameter row with editable value
#[component]
fn ParameterRow(info: ParameterInfo, on_change: Option<EventHandler<(String, f64)>>) -> Element {
    let theme = use_context::<Theme>();
    let mut value = use_signal(|| info.default_value.to_string());

    let info_name = info.name.clone();
    let info_default = info.default_value;

    let handle_change = {
        let info_name = info_name.clone();
        move |evt: Event<FormData>| {
            let new_value = evt.value().to_string();
            value.set(new_value.clone());
            if let Some(handler) = &on_change {
                if let Ok(v) = new_value.parse::<f64>() {
                    handler.call((info_name.clone(), v));
                }
            }
        }
    };

    // Pre-compute theme colors
    let bg_tertiary = theme.bg_tertiary();
    let text_primary = theme.text_primary();
    let text_muted = theme.text_muted();
    let bg_primary = theme.bg_primary();
    let border = theme.border();
    let text_secondary = theme.text_secondary();

    // Format range string
    let range_str = match (info.min, info.max) {
        (Some(min), Some(max)) => format!(" [{}, {}]", min, max),
        (Some(min), None) => format!(" [{}, ∞)", min),
        (None, Some(max)) => format!(" (-∞, {}]", max),
        (None, None) => String::new(),
    };

    let current_value = value.read().clone();
    let is_default = current_value == info_default.to_string();

    rsx! {
        div {
            style: "display: grid; grid-template-columns: 1fr 120px 60px; gap: 12px; \
                    align-items: center; padding: 8px 12px; background: {bg_tertiary}; border-radius: 6px;",

            // Parameter name and range
            div {
                div {
                    style: format!("font-family: {font}; font-size: {size}; color: {text_primary}; font-weight: 500;", font = Theme::FONT_MONO, size = Theme::FONT_SIZE_SM),
                    "{info_name}"
                }
                div {
                    style: "font-size: 11px; color: {text_muted};",
                    "default: {info_default}{range_str}"
                }
            }

            // Editable value
            input {
                r#type: "text",
                value: "{current_value}",
                oninput: handle_change,
                style: format!("width: 100%; padding: 6px 10px; background: {bg_primary}; \
                        border: 1px solid {border}; border-radius: 4px; color: {text_primary}; \
                        font-family: {font}; font-size: {size}; outline: none; transition: border-color {trans};",
                        font = Theme::FONT_MONO, size = Theme::FONT_SIZE_SM, trans = Theme::TRANSITION_FAST),
            }

            // Reset button (only show if modified)
            if !is_default {
                button {
                    onclick: move |_| value.set(info_default.to_string()),
                    style: "padding: 4px 8px; background: transparent; border: 1px solid {border}; \
                            border-radius: 4px; color: {text_secondary}; font-size: 11px; cursor: pointer;",
                    "Reset"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_default() {
        let info = VerilogAModelInfo::default();
        assert!(info.name.is_empty());
        assert!(!info.is_compiled);
        assert!(info.terminals.is_empty());
    }

    #[test]
    fn test_parameter_info() {
        let param = ParameterInfo {
            name: "r".to_string(),
            default_value: 1000.0,
            min: Some(0.0),
            max: None,
        };
        assert_eq!(param.name, "r");
        assert_eq!(param.default_value, 1000.0);
        assert_eq!(param.min, Some(0.0));
    }

    #[test]
    fn test_model_info_with_terminals() {
        let info = VerilogAModelInfo {
            name: "resistor".to_string(),
            source_path: "resistor.va".to_string(),
            is_compiled: true,
            error: None,
            terminals: vec!["p".to_string(), "n".to_string()],
            parameters: vec![ParameterInfo {
                name: "r".to_string(),
                default_value: 1000.0,
                min: Some(0.0),
                max: None,
            }],
            internal_nodes: 0,
        };
        assert_eq!(info.terminals.len(), 2);
        assert_eq!(info.parameters.len(), 1);
    }

    #[test]
    fn test_model_info_error_state() {
        let info = VerilogAModelInfo {
            name: "broken_model".to_string(),
            source_path: "broken.va".to_string(),
            is_compiled: false,
            error: Some("Syntax error on line 5".to_string()),
            terminals: vec![],
            parameters: vec![],
            internal_nodes: 0,
        };
        assert!(!info.is_compiled);
        assert!(info.error.is_some());
        assert_eq!(info.error.as_ref().unwrap(), "Syntax error on line 5");
    }

    #[test]
    fn test_parameter_with_full_range() {
        let param = ParameterInfo {
            name: "temperature".to_string(),
            default_value: 300.0,
            min: Some(0.0),
            max: Some(500.0),
        };
        assert_eq!(param.min, Some(0.0));
        assert_eq!(param.max, Some(500.0));
    }

    #[test]
    fn test_multiple_parameters() {
        let info = VerilogAModelInfo {
            name: "complex_device".to_string(),
            source_path: "".to_string(),
            is_compiled: true,
            error: None,
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            parameters: vec![
                ParameterInfo {
                    name: "r1".to_string(),
                    default_value: 100.0,
                    min: Some(0.0),
                    max: None,
                },
                ParameterInfo {
                    name: "r2".to_string(),
                    default_value: 200.0,
                    min: None,
                    max: Some(1e6),
                },
                ParameterInfo {
                    name: "k".to_string(),
                    default_value: 1.0,
                    min: Some(0.0),
                    max: Some(1.0),
                },
            ],
            internal_nodes: 2,
        };
        assert_eq!(info.terminals.len(), 3);
        assert_eq!(info.parameters.len(), 3);
        assert_eq!(info.internal_nodes, 2);
    }

    #[test]
    fn test_compiled_model_conversion() {
        use rspice_veriloga::VerilogACompiler;

        let va_source = r#"
            nature electrical; units = "V"; access = V; abstol = 1e-12; endnature
            nature current; units = "A"; access = I; abstol = 1e-12; endnature
            discipline electrical; potential electrical; flow current; enddiscipline
            module test_res(p, n);
                inout p, n; electrical p, n;
                parameter real g = 0.001;
                analog I(p, n) <+ g * V(p, n);
            endmodule
        "#;

        let compiler = VerilogACompiler::default();
        let model = compiler.compile(va_source).unwrap();

        // Convert to display info
        let info: VerilogAModelInfo = (&model).into();

        assert_eq!(info.name, "test_res");
        assert!(info.is_compiled);
        assert!(info.error.is_none());
        assert_eq!(info.terminals.len(), 2);
        assert!(info.terminals.contains(&"p".to_string()));
        assert!(info.terminals.contains(&"n".to_string()));
        assert_eq!(info.parameters.len(), 1);
        assert_eq!(info.parameters[0].name, "g");
        assert_eq!(info.parameters[0].default_value, 0.001);
    }

    #[test]
    fn test_model_info_equality() {
        let info1 = VerilogAModelInfo {
            name: "res".to_string(),
            source_path: "".to_string(),
            is_compiled: true,
            error: None,
            terminals: vec!["p".to_string()],
            parameters: vec![],
            internal_nodes: 0,
        };
        let info2 = info1.clone();

        assert_eq!(info1, info2);
    }
}

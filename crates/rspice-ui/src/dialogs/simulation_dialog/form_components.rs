//! Form Helper Components
//!
//! Reusable form input components for the simulation dialog.

use dioxus::prelude::*;

use crate::theme::Theme;

// =============================================================================
// FormRow Component
// =============================================================================

/// A form row with label, help text, and input field
#[component]
pub fn FormRow(label: &'static str, help: &'static str, children: Element) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "display: grid; grid-template-columns: 140px 1fr; gap: 12px; align-items: center; margin-bottom: 14px;",
            div {
                label {
                    style: "font-size: 13px; font-weight: 500; color: {th.text_secondary()};",
                    "{label}"
                }
                div {
                    style: "font-size: 11px; color: {th.text_muted()}; margin-top: 2px;",
                    "{help}"
                }
            }
            {children}
        }
    }
}

// =============================================================================
// FormInput Component
// =============================================================================

/// A text input with optional suffix (e.g., "Hz", "V", "s")
#[component]
pub fn FormInput(
    value: String,
    placeholder: &'static str,
    suffix: &'static str,
    onchange: EventHandler<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px;",
            input {
                r#type: "text",
                style: "flex: 1; padding: 8px 10px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 13px; font-family: 'JetBrains Mono', 'Fira Code', monospace;",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| onchange.call(e.value()),
            }
            if !suffix.is_empty() {
                span {
                    style: "color: {th.text_muted()}; font-size: 12px; min-width: 20px;",
                    "{suffix}"
                }
            }
        }
    }
}

// =============================================================================
// TabButton Component
// =============================================================================

/// A select/dropdown component for form options
#[component]
pub fn FormSelect(value: String, options: Vec<String>, onchange: EventHandler<String>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        select {
            style: "flex: 1; padding: 8px 10px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 13px; cursor: pointer;",
            value: "{value}",
            onchange: move |e| onchange.call(e.value()),
            for opt in options.iter() {
                option {
                    value: "{opt}",
                    selected: *opt == value,
                    "{opt}"
                }
            }
        }
    }
}

// =============================================================================
// TabButton Component
// =============================================================================

/// A horizontal tab button with enabled indicator
#[component]
pub fn TabButton(
    label: &'static str,
    active: bool,
    enabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let bg = if active {
        th.bg_secondary().to_string()
    } else {
        "transparent".to_string()
    };
    let color = if enabled {
        th.text_primary()
    } else {
        th.text_muted()
    };
    let weight = if active { "600" } else { "400" };
    let border_color = if active {
        th.accent_primary().to_string()
    } else {
        "transparent".to_string()
    };
    let dot_color = th.accent_success();

    rsx! {
        button {
            style: "padding: 10px 16px; border: none; background: {bg}; color: {color}; font-size: 13px; font-weight: {weight}; cursor: pointer; border-bottom: 2px solid {border_color}; transition: all 0.15s ease;",
            onclick: move |e| onclick.call(e),
            "{label}"
            if enabled {
                span {
                    style: "margin-left: 6px; width: 8px; height: 8px; background: {dot_color}; border-radius: 50%; display: inline-block;",
                }
            }
        }
    }
}

// =============================================================================
// SidebarButton Component
// =============================================================================

/// Props for SidebarButton
#[derive(Props, Clone, PartialEq)]
pub struct SidebarButtonProps {
    pub label: &'static str,
    pub active: bool,
    pub enabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

/// A sidebar navigation button with enabled indicator
#[component]
pub fn SidebarButton(props: SidebarButtonProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let bg = if props.active {
        th.bg_secondary().to_string()
    } else {
        "transparent".to_string()
    };
    let color = if props.enabled {
        th.text_primary()
    } else {
        th.text_secondary()
    };
    let weight = if props.active { "500" } else { "400" };
    let left_border = if props.active {
        format!("3px solid {}", th.accent_primary())
    } else {
        "3px solid transparent".to_string()
    };

    // Use CSS-based styling instead of conditional rendering
    // This ensures the element always exists and updates reliably
    let (dot_bg, dot_border) = if props.enabled {
        (th.accent_success().to_string(), "none".to_string())
    } else {
        (
            "transparent".to_string(),
            format!("1px solid {}", th.border()),
        )
    };

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 8px; width: 100%; padding: 10px 12px; border: none; border-left: {left_border}; background: {bg}; color: {color}; font-size: 12px; font-weight: {weight}; text-align: left; cursor: pointer; transition: all 0.15s ease;",
            onclick: move |e| props.onclick.call(e),
            span {
                style: "width: 6px; height: 6px; background: {dot_bg}; border: {dot_border}; border-radius: 50%; flex-shrink: 0; transition: background 0.15s ease;",
            }
            "{props.label}"
        }
    }
}

// =============================================================================
// Utilities
// =============================================================================

/// Format a value for display in form fields using engineering notation
///
/// Converts values to human-readable SI prefix notation:
/// - 1e9 → "1G"
/// - 1e6 → "1MEG"
/// - 1e3 → "1k"
/// - 1e-3 → "1m"
/// - 1e-6 → "1u"
/// - etc.
pub fn format_value(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }

    let abs_val = v.abs();

    // Use engineering notation for convenient editing
    if abs_val >= 1e9 {
        format!("{}G", v / 1e9)
    } else if abs_val >= 1e6 {
        format!("{}MEG", v / 1e6)
    } else if abs_val >= 1e3 {
        format!("{}k", v / 1e3)
    } else if abs_val >= 1.0 {
        format!("{}", v)
    } else if abs_val >= 1e-3 {
        format!("{}m", v * 1e3)
    } else if abs_val >= 1e-6 {
        format!("{}u", v * 1e6)
    } else if abs_val >= 1e-9 {
        format!("{}n", v * 1e9)
    } else if abs_val >= 1e-12 {
        format!("{}p", v * 1e12)
    } else {
        format!("{:e}", v)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_zero() {
        assert_eq!(format_value(0.0), "0");
    }

    #[test]
    fn test_format_value_giga() {
        assert_eq!(format_value(1e9), "1G");
        assert_eq!(format_value(2.5e9), "2.5G");
    }

    #[test]
    fn test_format_value_mega() {
        assert_eq!(format_value(1e6), "1MEG");
        assert_eq!(format_value(10e6), "10MEG");
    }

    #[test]
    fn test_format_value_kilo() {
        assert_eq!(format_value(1e3), "1k");
        assert_eq!(format_value(4.7e3), "4.7k");
    }

    #[test]
    fn test_format_value_unity() {
        assert_eq!(format_value(1.0), "1");
        assert_eq!(format_value(100.0), "100");
    }

    #[test]
    fn test_format_value_milli() {
        assert_eq!(format_value(1e-3), "1m");
        // Use 0.001 * 5 = 0.005 to test non-1 milli values
        assert_eq!(format_value(5e-3), "5m");
    }

    #[test]
    fn test_format_value_micro() {
        assert_eq!(format_value(1e-6), "1u");
        assert_eq!(format_value(4.7e-6), "4.7u");
    }

    #[test]
    fn test_format_value_nano() {
        assert_eq!(format_value(1e-9), "1n");
        assert_eq!(format_value(100e-9), "100n");
    }

    #[test]
    fn test_format_value_pico() {
        assert_eq!(format_value(1e-12), "1p");
        assert_eq!(format_value(10e-12), "10p");
    }

    #[test]
    fn test_format_value_very_small() {
        // Values smaller than pico use scientific notation
        let result = format_value(1e-15);
        assert!(result.contains("e"));
    }

    #[test]
    fn test_format_value_negative() {
        assert_eq!(format_value(-1e3), "-1k");
        assert_eq!(format_value(-1e-6), "-1u");
    }
}

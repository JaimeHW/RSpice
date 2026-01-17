//! Component Library Browser
//!
//! Searchable sidebar for browsing and placing components.

use dioxus::prelude::*;

use crate::state::{use_canvas_focus, ComponentType, Rotation, SchematicState, Tool};
use crate::theme::Theme;

/// Component category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCategory {
    Passives,
    Semiconductors,
    Sources,
}

impl ComponentCategory {
    fn name(&self) -> &'static str {
        match self {
            ComponentCategory::Passives => "Passive Components",
            ComponentCategory::Semiconductors => "Semiconductors",
            ComponentCategory::Sources => "Sources",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ComponentCategory::Passives => "⚡",
            ComponentCategory::Semiconductors => "💎",
            ComponentCategory::Sources => "🔋",
        }
    }

    fn components(&self) -> &'static [ComponentType] {
        match self {
            ComponentCategory::Passives => &[
                ComponentType::Resistor,
                ComponentType::Capacitor,
                ComponentType::Inductor,
            ],
            ComponentCategory::Semiconductors => &[
                ComponentType::Diode,
                ComponentType::NpnBjt,
                ComponentType::PnpBjt,
                ComponentType::Nmos,
                ComponentType::Pmos,
            ],
            ComponentCategory::Sources => &[
                ComponentType::VoltageSource,
                ComponentType::VoltageSourceAc,
                ComponentType::VoltageSourceSin,
                ComponentType::VoltageSourcePulse,
                ComponentType::CurrentSource,
                ComponentType::Ground,
            ],
        }
    }
}

/// Get display info for a component type
fn component_info(kind: ComponentType) -> (&'static str, &'static str, &'static str) {
    // Returns: (name, description, shortcut)
    match kind {
        ComponentType::Resistor => ("Resistor", "R - Resistance element", "R"),
        ComponentType::Capacitor => ("Capacitor", "C - Stores charge", "C"),
        ComponentType::Inductor => ("Inductor", "L - Stores energy in magnetic field", "L"),
        ComponentType::Diode => ("Diode", "D - Allows current in one direction", "D"),
        ComponentType::NpnBjt => ("NPN BJT", "Q - NPN Bipolar Junction Transistor", "Q"),
        ComponentType::PnpBjt => ("PNP BJT", "Q - PNP Bipolar Junction Transistor", ""),
        ComponentType::Nmos => ("N-MOSFET", "M - N-Channel MOSFET", "M"),
        ComponentType::Pmos => ("P-MOSFET", "M - P-Channel MOSFET", ""),
        ComponentType::VoltageSource => ("DC Voltage", "V - DC Voltage Source", "V"),
        ComponentType::VoltageSourceAc => ("AC Voltage", "V - AC Voltage Source", ""),
        ComponentType::VoltageSourceSin => ("Sine Source", "V - Sinusoidal Voltage", ""),
        ComponentType::VoltageSourcePulse => ("Pulse Source", "V - Pulse Voltage", ""),
        ComponentType::CurrentSource => ("Current Source", "I - Current Source", "I"),
        ComponentType::Ground => ("Ground", "0 - Ground reference", "G"),
    }
}

/// Props for ComponentLibrary
#[derive(Props, Clone, PartialEq)]
pub struct ComponentLibraryProps {
    /// Schematic state for setting the tool
    pub schematic: Signal<SchematicState>,
    /// Whether the library is visible
    #[props(default = true)]
    pub visible: bool,
}

/// Component Library Browser sidebar
#[component]
pub fn ComponentLibrary(props: ComponentLibraryProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Search filter state
    let mut search = use_signal(String::new);

    // Expanded category state
    let mut expanded = use_signal(|| vec![true, true, true]); // All expanded by default

    if !props.visible {
        return rsx! {};
    }

    let search_lower = search.read().to_lowercase();
    let categories = [
        ComponentCategory::Passives,
        ComponentCategory::Semiconductors,
        ComponentCategory::Sources,
    ];

    rsx! {
        div {
            style: "
                width: 220px;
                background: {th.bg_secondary()};
                border-right: 1px solid {th.border()};
                display: flex;
                flex-direction: column;
                height: 100%;
                overflow: hidden;
            ",

            // Header
            div {
                style: "
                    padding: 12px;
                    border-bottom: 1px solid {th.border()};
                    font-weight: 600;
                    color: {th.text_primary()};
                    font-size: 13px;
                ",
                "📦 Component Library"
            }

            // Search input
            div {
                style: "padding: 8px 12px;",
                input {
                    r#type: "text",
                    placeholder: "🔍 Search components...",
                    value: "{search}",
                    style: "
                        width: 100%;
                        padding: 6px 10px;
                        background: {th.bg_primary()};
                        border: 1px solid {th.border()};
                        border-radius: 6px;
                        color: {th.text_primary()};
                        font-size: 12px;
                        outline: none;
                        box-sizing: border-box;
                    ",
                    oninput: move |e| search.set(e.value().clone()),
                }
            }

            // Categories
            div {
                style: "flex: 1; overflow-y: auto; padding: 4px 0;",

                for (cat_idx, category) in categories.iter().enumerate() {
                    // Filter components by search
                    {
                        let components: Vec<_> = category.components()
                            .iter()
                            .filter(|c| {
                                let (name, desc, _) = component_info(**c);
                                search_lower.is_empty()
                                    || name.to_lowercase().contains(&search_lower)
                                    || desc.to_lowercase().contains(&search_lower)
                            })
                            .collect();

                        if components.is_empty() && !search_lower.is_empty() {
                            rsx! {}
                        } else {
                            let is_expanded = *expanded.read().get(cat_idx).unwrap_or(&true);
                            let cat_icon = category.icon();
                            let cat_name = category.name();

                            rsx! {
                                // Category header
                                div {
                                    style: "
                                        display: flex;
                                        align-items: center;
                                        padding: 8px 12px;
                                        cursor: pointer;
                                        user-select: none;
                                        color: {th.text_secondary()};
                                        font-size: 11px;
                                        font-weight: 600;
                                        text-transform: uppercase;
                                        letter-spacing: 0.5px;
                                    ",
                                    onclick: move |_| {
                                        let mut exp = expanded.write();
                                        if let Some(v) = exp.get_mut(cat_idx) {
                                            *v = !*v;
                                        }
                                    },
                                    span { style: "margin-right: 6px;", "{cat_icon}" }
                                    span { style: "flex: 1;", "{cat_name}" }
                                    span {
                                        style: "font-size: 10px; opacity: 0.6;",
                                        if is_expanded { "▼" } else { "▶" }
                                    }
                                }

                                // Component list
                                if is_expanded {
                                    for comp_type in components {
                                        ComponentItem {
                                            kind: *comp_type,
                                            schematic: props.schematic,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Footer with tip
            div {
                style: "
                    padding: 8px 12px;
                    border-top: 1px solid {th.border()};
                    font-size: 10px;
                    color: {th.text_muted()};
                    opacity: 0.8;
                ",
                "Click to select, then click canvas to place"
            }
        }
    }
}

/// Individual component item in the library
#[component]
fn ComponentItem(kind: ComponentType, schematic: Signal<SchematicState>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let canvas_focus = use_canvas_focus(); // Get canvas focus at component level

    let (name, _desc, shortcut) = component_info(kind);
    let is_active = matches!(schematic.read().tool, Tool::Place(k) if k == kind);

    let bg: &str = if is_active {
        &th.accent_primary()
    } else {
        "transparent"
    };
    let text_color: &str = if is_active {
        "#ffffff"
    } else {
        &th.text_primary()
    };
    let shortcut_bg: &str = if is_active {
        "rgba(255,255,255,0.2)"
    } else {
        &th.bg_tertiary()
    };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                padding: 6px 12px 6px 24px;
                cursor: pointer;
                background: {bg};
                transition: background 0.1s;
            ",
            onclick: move |_| {
                schematic.write().tool = Tool::Place(kind);
                schematic.write().preview_rotation = Rotation::R0;  // Reset rotation for new component
                // Focus canvas so keyboard shortcuts work immediately
                canvas_focus.read().focus();
            },
            span {
                style: "flex: 1; font-size: 12px; color: {text_color};",
                "{name}"
            }
            if !shortcut.is_empty() {
                span {
                    style: "
                        font-size: 10px;
                        color: {text_color};
                        opacity: 0.7;
                        background: {shortcut_bg};
                        padding: 1px 5px;
                        border-radius: 3px;
                        font-family: monospace;
                    ",
                    "{shortcut}"
                }
            }
        }
    }
}

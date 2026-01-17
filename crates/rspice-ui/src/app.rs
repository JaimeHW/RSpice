//! Application Root Component
//!
//! The main application component that sets up the window layout,
//! routing, and global state providers.

use dioxus::prelude::*;

use crate::components::{Panel, Toolbar};
use crate::state::{ComponentType, SchematicState, SimulationState, Tool};
use crate::theme::Theme;
use crate::views::{Console, Netlist, Schematic, Waveform};

/// Active editor tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorTab {
    #[default]
    Schematic,
    Netlist,
}

/// Root application component
#[component]
pub fn App() -> Element {
    // Initialize global state
    let theme = use_signal(|| Theme::DARK);
    let sim_state = use_signal(SimulationState::default);
    let schematic_state = use_signal(SchematicState::default);
    let active_tab = use_signal(|| EditorTab::Schematic);

    // Provide contexts to all children
    use_context_provider(|| theme);
    use_context_provider(|| sim_state);
    use_context_provider(|| schematic_state);

    let th = theme.read();

    rsx! {
        // Global styles
        style { {global_styles(&th)} }

        // Main application container
        div {
            class: "app-container",
            style: "
                display: flex;
                flex-direction: column;
                width: 100vw;
                height: 100vh;
                background: {th.bg_primary()};
                color: {th.text_primary()};
                font-family: {Theme::FONT_FAMILY};
                font-size: {Theme::FONT_SIZE_BASE};
                overflow: hidden;
            ",

            // Top toolbar
            Toolbar {}

            // Main content area
            div {
                class: "main-content",
                style: "
                    display: flex;
                    flex: 1;
                    overflow: hidden;
                ",

                // Center - Editor area with tabs
                div {
                    class: "editor-area",
                    style: "
                        display: flex;
                        flex-direction: column;
                        flex: 1;
                        overflow: hidden;
                    ",

                    // Editor tabs header
                    EditorTabBar {
                        active_tab: active_tab,
                    }

                    // Editor content based on active tab
                    match *active_tab.read() {
                        EditorTab::Schematic => rsx! {
                            // Schematic editor (main)
                            div {
                                style: "flex: 1; overflow: hidden; min-height: 0;",
                                Schematic {}
                            }

                            // Resizable divider
                            div {
                                style: "
                                    height: 4px;
                                    background: {th.border()};
                                    cursor: row-resize;
                                "
                            }

                            // Waveform viewer (bottom)
                            div {
                                style: "height: 200px; min-height: 100px; overflow: hidden;",
                                Waveform {}
                            }
                        },
                        EditorTab::Netlist => rsx! {
                            // Netlist editor (top)
                            div {
                                style: "flex: 1; min-height: 200px; overflow: hidden;",
                                Netlist {}
                            }

                            // Resizable divider
                            div {
                                style: "
                                    height: 4px;
                                    background: {th.border()};
                                    cursor: row-resize;
                                "
                            }

                            // Waveform viewer (bottom)
                            div {
                                style: "flex: 1; min-height: 200px; overflow: hidden;",
                                Waveform {}
                            }
                        },
                    }
                }

                // Right sidebar - Properties
                Panel {
                    title: "Properties",
                    width: "250px",
                    position: "right",
                    PropertiesPanel {}
                }
            }

            // Bottom console
            Console {}
        }
    }
}

/// Editor tab bar component
#[component]
fn EditorTabBar(active_tab: Signal<EditorTab>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            class: "editor-tab-bar",
            style: "
                display: flex;
                align-items: center;
                height: 32px;
                padding: 0 {Theme::SPACING_SM};
                background: {th.bg_tertiary()};
                border-bottom: 1px solid {th.border()};
                gap: 2px;
            ",

            // Schematic tab
            EditorTabButton {
                label: "Schematic",
                active: *active_tab.read() == EditorTab::Schematic,
                onclick: move |_| active_tab.set(EditorTab::Schematic),
            }

            // Netlist tab
            EditorTabButton {
                label: "Netlist",
                active: *active_tab.read() == EditorTab::Netlist,
                onclick: move |_| active_tab.set(EditorTab::Netlist),
            }
        }
    }
}

/// Single editor tab button
#[component]
fn EditorTabButton(
    label: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let bg = if active {
        th.bg_secondary()
    } else {
        "transparent"
    };
    let color = if active {
        th.text_primary()
    } else {
        th.text_muted()
    };
    let border = if active {
        format!("border-bottom: 2px solid {};", th.accent_primary())
    } else {
        "border-bottom: 2px solid transparent;".to_string()
    };

    rsx! {
        button {
            style: "
                padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                background: {bg};
                color: {color};
                font-size: {Theme::FONT_SIZE_SM};
                border: none;
                border-radius: {Theme::RADIUS_SM} {Theme::RADIUS_SM} 0 0;
                {border}
                cursor: pointer;
                transition: all 0.15s ease;
            ",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

/// Component library sidebar content
#[component]
fn ComponentLibrary() -> Element {
    let theme: Signal<Theme> = use_context();
    let schematic: Signal<SchematicState> = use_context();
    let th = theme.read();

    let categories: &[(&str, &[(&str, ComponentType)])] = &[
        (
            "Basic",
            &[
                ("Resistor", ComponentType::Resistor),
                ("Capacitor", ComponentType::Capacitor),
                ("Inductor", ComponentType::Inductor),
            ],
        ),
        (
            "Semiconductors",
            &[
                ("Diode", ComponentType::Diode),
                ("NPN BJT", ComponentType::NpnBjt),
                ("PNP BJT", ComponentType::PnpBjt),
                ("NMOS", ComponentType::Nmos),
                ("PMOS", ComponentType::Pmos),
            ],
        ),
        (
            "Sources",
            &[
                ("V DC", ComponentType::VoltageSource),
                ("I DC", ComponentType::CurrentSource),
                ("V AC", ComponentType::VoltageSourceAc),
                ("V Pulse", ComponentType::VoltageSourcePulse),
                ("V Sin", ComponentType::VoltageSourceSin),
            ],
        ),
        ("Special", &[("Ground", ComponentType::Ground)]),
    ];

    rsx! {
        div {
            style: "padding: {Theme::SPACING_SM};",

            // Search box
            input {
                r#type: "text",
                placeholder: "Search components...",
                style: "
                    width: 100%;
                    padding: {Theme::SPACING_SM};
                    background: {th.surface()};
                    border: 1px solid {th.border()};
                    border-radius: {Theme::RADIUS_SM};
                    color: {th.text_primary()};
                    font-size: {Theme::FONT_SIZE_SM};
                    outline: none;
                    margin-bottom: {Theme::SPACING_MD};
                "
            }

            // Category tree
            for (category, items) in categories.iter() {
                div {
                    style: "margin-bottom: {Theme::SPACING_MD};",

                    // Category header
                    div {
                        style: "
                            font-weight: 600;
                            color: {th.text_secondary()};
                            font-size: {Theme::FONT_SIZE_SM};
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                            margin-bottom: {Theme::SPACING_XS};
                        ",
                        "{category}"
                    }

                    // Items
                    for (name, comp_type) in items.iter() {
                        ComponentLibraryItem {
                            name: *name,
                            comp_type: *comp_type,
                        }
                    }
                }
            }
        }
    }
}

/// Single component library item
#[component]
fn ComponentLibraryItem(name: &'static str, comp_type: ComponentType) -> Element {
    let theme: Signal<Theme> = use_context();
    let mut schematic: Signal<SchematicState> = use_context();
    let th = theme.read();

    let current_tool = schematic.read().tool;
    let is_active = matches!(current_tool, Tool::Place(t) if t == comp_type);

    let bg = if is_active {
        th.surface_hover()
    } else {
        "transparent"
    };
    let color = if is_active {
        th.accent_primary()
    } else {
        th.text_primary()
    };

    rsx! {
        div {
            style: "
                padding: {Theme::SPACING_XS} {Theme::SPACING_SM};
                border-radius: {Theme::RADIUS_SM};
                cursor: pointer;
                transition: background {Theme::TRANSITION_FAST};
                background: {bg};
                color: {color};
            ",
            onclick: move |_| {
                schematic.write().tool = Tool::Place(comp_type);
            },
            "{name}"
        }
    }
}

/// Properties panel content
#[component]
fn PropertiesPanel() -> Element {
    let theme: Signal<Theme> = use_context();
    let schematic: Signal<SchematicState> = use_context();
    let th = theme.read();

    let sch = schematic.read();
    let selection = &sch.selection;

    // Get selected component info
    let selected_component = if selection.components.len() == 1 {
        let id = selection.components[0];
        sch.components.iter().find(|c| c.id == id)
    } else {
        None
    };

    rsx! {
        div {
            style: "
                padding: {Theme::SPACING_MD};
            ",

            if let Some(comp) = selected_component {
                // Component properties
                div {
                    style: "
                        font-weight: 600;
                        color: {th.text_primary()};
                        font-size: {Theme::FONT_SIZE_SM};
                        margin-bottom: {Theme::SPACING_MD};
                    ",
                    "{comp.kind.display_name()}"
                }

                // Name field
                PropertyField {
                    label: "Name",
                    value: comp.name.clone(),
                }

                // Value field
                PropertyField {
                    label: "Value",
                    value: comp.value.clone(),
                }

                // Position
                div {
                    style: "
                        display: flex;
                        gap: {Theme::SPACING_SM};
                        margin-bottom: {Theme::SPACING_SM};
                    ",

                    div {
                        style: "flex: 1;",
                        PropertyField {
                            label: "X",
                            value: format!("{}", comp.pos.x),
                        }
                    }

                    div {
                        style: "flex: 1;",
                        PropertyField {
                            label: "Y",
                            value: format!("{}", comp.pos.y),
                        }
                    }
                }

                // Rotation
                PropertyField {
                    label: "Rotation",
                    value: format!("{}°", comp.rotation.degrees()),
                }
            } else if !selection.is_empty() {
                div {
                    style: "
                        color: {th.text_secondary()};
                        font-size: {Theme::FONT_SIZE_SM};
                    ",
                    {format!("{} items selected", selection.components.len() + selection.wires.len())}
                }
            } else {
                div {
                    style: "
                        color: {th.text_muted()};
                        font-size: {Theme::FONT_SIZE_SM};
                        text-align: center;
                    ",
                    "Select a component to view its properties"
                }
            }
        }
    }
}

/// Property field component
#[component]
fn PropertyField(label: &'static str, value: String) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "margin-bottom: {Theme::SPACING_SM};",

            label {
                style: "
                    display: block;
                    font-size: 11px;
                    color: {th.text_muted()};
                    margin-bottom: 2px;
                ",
                "{label}"
            }

            input {
                r#type: "text",
                value: "{value}",
                style: "
                    width: 100%;
                    padding: 4px 8px;
                    background: {th.surface()};
                    border: 1px solid {th.border()};
                    border-radius: {Theme::RADIUS_SM};
                    color: {th.text_primary()};
                    font-size: {Theme::FONT_SIZE_SM};
                    outline: none;
                "
            }
        }
    }
}

/// Global CSS styles
fn global_styles(theme: &Theme) -> String {
    format!(
        r#"
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        html, body {{
            width: 100%;
            height: 100%;
            overflow: hidden;
        }}

        ::-webkit-scrollbar {{
            width: 8px;
            height: 8px;
        }}

        ::-webkit-scrollbar-track {{
            background: {bg};
        }}

        ::-webkit-scrollbar-thumb {{
            background: {border};
            border-radius: 4px;
        }}

        ::-webkit-scrollbar-thumb:hover {{
            background: {border_hover};
        }}

        ::selection {{
            background: {accent}40;
        }}
        "#,
        bg = theme.bg_secondary(),
        border = theme.border(),
        border_hover = theme.surface_hover(),
        accent = theme.accent_primary(),
    )
}

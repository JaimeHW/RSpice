//! Application Root Component
//!
//! The main application component that sets up the window layout,
//! routing, and global state providers.

use dioxus::prelude::*;

use crate::components::{Panel, Toolbar};
use crate::state::SimulationState;
use crate::theme::Theme;
use crate::views::{Console, Netlist, Waveform};

/// Root application component
#[component]
pub fn App() -> Element {
    // Initialize global state
    let theme = use_signal(|| Theme::DARK);
    let sim_state = use_signal(SimulationState::default);

    // Provide theme context to all children
    use_context_provider(|| theme);
    use_context_provider(|| sim_state);

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

                // Left sidebar - Component library
                Panel {
                    title: "Components",
                    width: "250px",
                    position: "left",
                    ComponentLibrary {}
                }

                // Center - Editor area
                div {
                    class: "editor-area",
                    style: "
                        display: flex;
                        flex-direction: column;
                        flex: 1;
                        overflow: hidden;
                    ",

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
                }

                // Right sidebar - Properties (future)
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

/// Component library sidebar content
#[component]
fn ComponentLibrary() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let categories = [
        ("Basic", vec!["Resistor", "Capacitor", "Inductor"]),
        (
            "Semiconductors",
            vec!["Diode", "NPN BJT", "PNP BJT", "NMOS", "PMOS"],
        ),
        ("Sources", vec!["V DC", "I DC", "V AC", "V Pulse", "V Sin"]),
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
            for (category, items) in categories {
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
                    for item in items {
                        div {
                            style: "
                                padding: {Theme::SPACING_XS} {Theme::SPACING_SM};
                                border-radius: {Theme::RADIUS_SM};
                                cursor: pointer;
                                transition: background {Theme::TRANSITION_FAST};
                            ",
                            onmouseenter: move |_| {},
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}

/// Properties panel content
#[component]
fn PropertiesPanel() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "
                padding: {Theme::SPACING_MD};
                color: {th.text_secondary()};
                font-size: {Theme::FONT_SIZE_SM};
                text-align: center;
            ",
            "Select a component to view its properties"
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

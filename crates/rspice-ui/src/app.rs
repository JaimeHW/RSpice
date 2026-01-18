//! Application Root Component
//!
//! The main application component that sets up the window layout,
//! routing, and global state providers.

use dioxus::prelude::*;

use crate::components::{Panel, Toolbar};
use crate::state::{SchematicState, SimulationState};
use crate::theme::Theme;
use crate::views::{Console, Netlist, Schematic, Waveform};

/// Active editor tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorTab {
    #[default]
    Schematic,
    Netlist,
}

/// Wrapper type for waveform visibility (needed for distinct context type)
#[derive(Clone, Copy)]
pub struct WaveformVisible(pub bool);

/// Wrapper type for console visibility (needed for distinct context type)
#[derive(Clone, Copy)]
pub struct ConsoleVisible(pub bool);

/// Root application component
#[component]
pub fn App() -> Element {
    // Initialize global state
    let theme = use_signal(|| Theme::DARK);
    let sim_state = use_signal(SimulationState::default);
    let schematic_state = use_signal(SchematicState::default);
    let active_tab = use_signal(|| EditorTab::Schematic);

    // Resizable pane heights (in pixels)
    let mut waveform_height = use_signal(|| 200.0_f64);
    let mut console_height = use_signal(|| 120.0_f64);

    // Panel visibility state (using wrapper types for distinct context lookup)
    let waveform_visible = use_signal(|| WaveformVisible(true));
    let console_visible = use_signal(|| ConsoleVisible(true));

    // Drag state for resizable dividers: None = not dragging, Some("waveform" or "console")
    let mut resize_dragging: Signal<Option<&'static str>> = use_signal(|| None);
    let mut resize_start_y = use_signal(|| 0.0_f64);
    let mut resize_start_height = use_signal(|| 0.0_f64);

    // Provide contexts to all children
    use_context_provider(|| theme);
    use_context_provider(|| sim_state);
    use_context_provider(|| schematic_state);
    use_context_provider(|| waveform_visible);
    use_context_provider(|| console_visible);

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

            // Global mouse handlers for resize dragging
            onmousemove: move |evt| {
                if let Some(target) = *resize_dragging.read() {
                    let current_y = evt.page_coordinates().y;
                    let delta = current_y - *resize_start_y.read();
                    let start_h = *resize_start_height.read();

                    // Note: For waveform (top resizes down), delta is inverted
                    // For console (bottom resizes up), delta is normal
                    let new_height = match target {
                        "waveform" => (start_h - delta).max(100.0).min(600.0),
                        "console" => (start_h - delta).max(60.0).min(400.0),
                        _ => start_h,
                    };

                    match target {
                        "waveform" => waveform_height.set(new_height),
                        "console" => console_height.set(new_height),
                        _ => {},
                    }
                }
            },
            onmouseup: move |_| {
                resize_dragging.set(None);
            },

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

                            // Only show waveform viewer when simulation results exist AND visible
                            // This matches professional simulator behavior (LTspice)
                            if !sim_state.read().waveforms.is_empty() && waveform_visible.read().0 {
                                // Resizable divider for waveform pane
                                div {
                                    style: "
                                        height: 6px;
                                        background: {th.border()};
                                        cursor: row-resize;
                                        transition: background 0.15s;
                                    ",
                                    onmouseenter: move |_| {},
                                    onmousedown: move |evt| {
                                        resize_dragging.set(Some("waveform"));
                                        resize_start_y.set(evt.page_coordinates().y);
                                        resize_start_height.set(*waveform_height.read());
                                    },
                                }

                                // Waveform viewer (bottom) - dynamic height
                                div {
                                    style: "height: {waveform_height}px; min-height: 100px; overflow: hidden;",
                                    Waveform {}
                                }
                            }
                        },
                        EditorTab::Netlist => rsx! {
                            // Netlist editor (top)
                            div {
                                style: "flex: 1; min-height: 200px; overflow: hidden;",
                                Netlist {}
                            }

                            // Only show waveform viewer when simulation results exist AND visible
                            if !sim_state.read().waveforms.is_empty() && waveform_visible.read().0 {
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

            // Console section (only visible if not hidden)
            if console_visible.read().0 {
                // Resizable divider for console pane
                div {
                    style: "
                        height: 6px;
                        background: {th.border()};
                        cursor: row-resize;
                        transition: background 0.15s;
                    ",
                    onmousedown: move |evt| {
                        resize_dragging.set(Some("console"));
                        resize_start_y.set(evt.page_coordinates().y);
                        resize_start_height.set(*console_height.read());
                    },
                }

                // Bottom console with dynamic height
                div {
                    style: "height: {console_height}px; min-height: 60px; overflow: hidden;",
                    Console {}
                }
            }
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

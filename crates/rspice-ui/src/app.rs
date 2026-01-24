//! Application Root Component
//!
//! The main application component that sets up the window layout,
//! routing, and global state providers.

use std::sync::Arc;

use dioxus::prelude::*;
use rspice_core::library::LibraryManager;

use crate::components::{Panel, ProjectBrowser, SimulationDialog, Toolbar};
use crate::dialogs::{SimulationOptions, SimulationOptionsDialog};
use crate::services::cross_probing::CrossProbeManager;
use crate::state::display_settings::SchematicDisplaySettings;
use crate::state::hierarchy::HierarchyManager;
use crate::state::simulation_command::SimulationConfig;
use crate::state::{DocumentManager, SchematicState, SimulationState};
use crate::theme::Theme;
use crate::views::{Console, Schematic, SchematicToolbar, Waveform};

/// Wrapper type for waveform visibility (needed for distinct context type)
#[derive(Clone, Copy)]
pub struct WaveformVisible(pub bool);

/// Wrapper type for console visibility (needed for distinct context type)
#[derive(Clone, Copy)]
pub struct ConsoleVisible(pub bool);

/// Wrapper type for library browser visibility (needed for distinct context type)
#[derive(Clone, Copy)]
pub struct BrowserVisible(pub bool);

/// Wrapper type for simulation dialog visibility (needed for distinct context type)
#[derive(Clone, Copy)]
pub struct SimDialogVisible(pub bool);

/// Wrapper type for simulation options dialog visibility (needed for distinct context type)
#[derive(Clone, Copy)]
pub struct SimOptionsVisible(pub bool);

/// Root application component
#[component]
pub fn App() -> Element {
    // Initialize global state
    let theme = use_signal(|| Theme::DARK);
    let sim_state = use_signal(SimulationState::default);
    let schematic_state = use_signal(SchematicState::default);

    // Display settings for schematic rendering (label visibility, fonts, etc.)
    let display_settings = use_signal(SchematicDisplaySettings::default);

    // Simulation configuration (from dialog)
    let mut sim_config = use_signal(SimulationConfig::default);
    let mut sim_dialog_visible = use_signal(|| SimDialogVisible(false));

    // Simulation options (advanced solver/convergence settings)
    let mut sim_options = use_signal(SimulationOptions::default);
    let mut sim_options_visible = use_signal(|| SimOptionsVisible(false));

    // Resizable pane heights (in pixels)
    let mut waveform_height = use_signal(|| 200.0_f64);
    let mut console_height = use_signal(|| 120.0_f64);

    // Panel visibility state (using wrapper types for distinct context lookup)
    let waveform_visible = use_signal(|| WaveformVisible(true));
    let console_visible = use_signal(|| ConsoleVisible(true));
    let browser_visible = use_signal(|| BrowserVisible(false)); // Default hidden to maximize canvas

    // Drag state for resizable dividers: None = not dragging, Some("waveform" or "console")
    let mut resize_dragging: Signal<Option<&'static str>> = use_signal(|| None);
    let mut resize_start_y = use_signal(|| 0.0_f64);
    let mut resize_start_height = use_signal(|| 0.0_f64);

    // Initialize component library manager (embedded libraries parsed at startup)
    let library_manager = use_signal(|| Arc::new(LibraryManager::new()));

    // Multi-document interface: manage open documents
    let doc_manager = use_signal(DocumentManager::default);

    // Cross-probing manager for schematic ↔ waveform coordination
    let cross_probe = use_signal(CrossProbeManager::new);

    // Hierarchy manager for Library/Cell/View navigation
    // Initialize with built-in primitives library
    let hierarchy_manager = use_signal(|| {
        let mut mgr = HierarchyManager::new();
        mgr.create_primitives_library();
        mgr
    });

    // Provide contexts to all children
    use_context_provider(|| theme);
    use_context_provider(|| sim_state);
    use_context_provider(|| schematic_state);
    use_context_provider(|| library_manager);
    use_context_provider(|| waveform_visible);
    use_context_provider(|| console_visible);
    use_context_provider(|| sim_config);
    use_context_provider(|| sim_dialog_visible);
    use_context_provider(|| sim_options);
    use_context_provider(|| sim_options_visible);
    use_context_provider(|| doc_manager);
    use_context_provider(|| cross_probe);
    use_context_provider(|| display_settings);
    use_context_provider(|| hierarchy_manager);
    use_context_provider(|| browser_visible);

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

            // Main layout: Icon rail on left (full height) + Content area on right
            div {
                class: "main-layout",
                style: "
                    display: flex;
                    flex: 1;
                    overflow: hidden;
                ",

                // Left icon rail (VSCode style) - FULL HEIGHT from toolbar to bottom
                div {
                    class: "icon-rail",
                    style: "
                        display: flex;
                        flex-direction: column;
                        width: 42px;
                        min-width: 42px;
                        background: {th.surface()};
                        border-right: 1px solid {th.border()};
                        padding: 8px 0;
                        align-items: center;
                    ",

                    // === TOP SECTION: Navigation panels ===

                    // Library browser toggle
                    {
                        let mut browser_vis = browser_visible;
                        let is_active = browser_visible.read().0;
                        // Use a brighter surface for inactive buttons so they stand out
                        let btn_bg = if is_active { th.accent_primary().to_string() } else { "#3a3e4a".to_string() };
                        let btn_border = if is_active { th.accent_primary().to_string() } else { "#555963".to_string() };
                        let btn_color = if is_active { "#ffffff".to_string() } else { th.text_primary().to_string() };
                        rsx! {
                            button {
                                class: "rail-btn",
                                style: "
                                    width: 32px;
                                    height: 32px;
                                    border: 2px solid {btn_border};
                                    border-radius: 6px;
                                    background: {btn_bg};
                                    color: {btn_color};
                                    cursor: pointer;
                                    font-size: 16px;
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                    margin-bottom: 6px;
                                    transition: all 0.15s ease;
                                    box-shadow: 0 2px 4px rgba(0,0,0,0.25);
                                ",
                                title: "Library Browser (Ctrl+Shift+L)",
                                onclick: move |_| {
                                    let current = browser_vis.read().0;
                                    browser_vis.set(BrowserVisible(!current));
                                },
                                // Folder icon for library
                                "📁"
                            }
                        }
                    }

                    // === SPACER: Push bottom section to bottom ===
                    div { style: "flex: 1;" }

                    // === BOTTOM SECTION: Auxiliary panels ===

                    // Console toggle
                    {
                        let mut console_vis = console_visible;
                        let is_active = console_visible.read().0;
                        // Use brighter surface for inactive buttons
                        let btn_bg = if is_active { th.accent_primary().to_string() } else { "#3a3e4a".to_string() };
                        let btn_border = if is_active { th.accent_primary().to_string() } else { "#555963".to_string() };
                        let btn_color = if is_active { "#ffffff".to_string() } else { th.text_primary().to_string() };
                        rsx! {
                            button {
                                class: "rail-btn",
                                style: "
                                    width: 32px;
                                    height: 32px;
                                    border: 2px solid {btn_border};
                                    border-radius: 6px;
                                    background: {btn_bg};
                                    color: {btn_color};
                                    cursor: pointer;
                                    font-size: 16px;
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                    margin-bottom: 6px;
                                    transition: all 0.15s ease;
                                    box-shadow: 0 2px 4px rgba(0,0,0,0.25);
                                ",
                                title: "Toggle Console",
                                onclick: move |_| {
                                    let current = console_vis.read().0;
                                    console_vis.set(ConsoleVisible(!current));
                                },
                                // Terminal/console icon
                                "⌨"
                            }
                        }
                    }

                    // Waveform toggle
                    {
                        let mut waveform_vis = waveform_visible;
                        let sim = sim_state.read();
                        let has_waveforms = !sim.waveforms.is_empty();
                        let is_active = waveform_visible.read().0 && has_waveforms;
                        drop(sim);
                        // Use brighter surface for inactive buttons
                        let btn_bg = if is_active {
                            th.accent_primary().to_string()
                        } else if has_waveforms {
                            "#3a3e4a".to_string()
                        } else {
                            "#2d3038".to_string()
                        };
                        let btn_border = if is_active {
                            th.accent_primary().to_string()
                        } else if has_waveforms {
                            "#555963".to_string()
                        } else {
                            "#404550".to_string()
                        };
                        let btn_color = if !has_waveforms {
                            th.text_muted().to_string()
                        } else if is_active {
                            "#ffffff".to_string()
                        } else {
                            th.text_primary().to_string()
                        };
                        let cursor = if has_waveforms { "pointer" } else { "not-allowed" };
                        let opacity = if has_waveforms { "1" } else { "0.5" };
                        rsx! {
                            button {
                                class: "rail-btn",
                                style: "
                                    width: 32px;
                                    height: 32px;
                                    border: 2px solid {btn_border};
                                    border-radius: 6px;
                                    background: {btn_bg};
                                    color: {btn_color};
                                    cursor: {cursor};
                                    opacity: {opacity};
                                    font-size: 18px;
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                    transition: all 0.15s ease;
                                    box-shadow: 0 2px 4px rgba(0,0,0,0.25);
                                ",
                                title: if has_waveforms { "Toggle Waveform Viewer" } else { "No waveforms available" },
                                disabled: !has_waveforms,
                                onclick: move |_| {
                                    if has_waveforms {
                                        let current = waveform_vis.read().0;
                                        waveform_vis.set(WaveformVisible(!current));
                                    }
                                },
                                // Waveform/wave icon
                                "∿"
                            }
                        }
                    }
                }

                // Right side: All content (main area + bottom panels)
                // This is a flex column that the icon rail sits beside
                div {
                    class: "content-area",
                    style: "
                        display: flex;
                        flex-direction: column;
                        flex: 1;
                        overflow: hidden;
                    ",

                    // Main content row (sidebars + editor)
                    div {
                        class: "main-content",
                        style: "
                            display: flex;
                            flex: 1;
                            overflow: hidden;
                        ",

                        // Left sidebar - Project Browser (Library/Cell/View tree)
                        // Conditionally rendered based on browser_visible state
                        if browser_visible.read().0 {
                            div {
                                style: "
                                    width: 220px;
                                    border-right: 1px solid {th.border()};
                                    display: flex;
                                    flex-direction: column;
                                    overflow: hidden;
                                ",
                                ProjectBrowser {}
                            }
                        }

                        // Center - Editor area (schematic only - waveform moved to full-width below)
                        div {
                            class: "editor-area",
                            style: "
                                display: flex;
                                flex-direction: column;
                                flex: 1;
                                overflow: hidden;
                            ",

                            // Tool selection bar (Select, Wire, Probe, etc.)
                            SchematicToolbar { schematic: schematic_state }

                            // Schematic editor (main) - includes document tabs internally
                            div {
                                style: "flex: 1; overflow: hidden; min-height: 0;",
                                Schematic {}
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

                    // Waveform viewer - FULL WIDTH spanning under all sidebars
                    // This matches industry standard layout
                    // Only show when simulation results exist AND visible
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

                        // Waveform viewer (full width) - dynamic height
                        div {
                            style: "height: {waveform_height}px; min-height: 100px; overflow: hidden;",
                            Waveform {}
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

            // Simulation setup dialog (modal, rendered on top)
            SimulationDialog {
                visible: sim_dialog_visible.read().0,
                config: sim_config.read().clone(),
                on_confirm: move |new_config| {
                    sim_config.set(new_config);
                    sim_dialog_visible.set(SimDialogVisible(false));
                },
                on_cancel: move |_| {
                    sim_dialog_visible.set(SimDialogVisible(false));
                },
            }

            // Simulation options dialog (advanced solver/convergence settings)
            SimulationOptionsDialog {
                options: sim_options.read().clone(),
                is_open: sim_options_visible.read().0,
                on_close: move |_| {
                    sim_options_visible.set(SimOptionsVisible(false));
                },
                on_save: move |new_options| {
                    sim_options.set(new_options);
                },
            }
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

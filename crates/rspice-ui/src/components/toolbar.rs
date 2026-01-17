//! Toolbar Component
//!
//! Main application toolbar with file operations and simulation controls.

use dioxus::prelude::*;

use super::button::{Button, ButtonVariant};
use super::icons::{Icon, IconType};
use crate::state::SimulationState;
use crate::theme::Theme;

/// Main application toolbar
#[component]
pub fn Toolbar() -> Element {
    let theme: Signal<Theme> = use_context();
    let mut sim_state: Signal<SimulationState> = use_context();
    let th = theme.read();

    let is_running = sim_state.read().is_running;

    rsx! {
        div {
            class: "toolbar",
            style: "
                display: flex;
                align-items: center;
                height: 48px;
                padding: 0 {Theme::SPACING_MD};
                background: {th.bg_secondary()};
                border-bottom: 1px solid {th.border()};
                gap: {Theme::SPACING_MD};
            ",

            // Logo / App name
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: {Theme::SPACING_SM};
                    font-size: {Theme::FONT_SIZE_LG};
                    font-weight: 700;
                    color: {th.accent_primary()};
                    padding-right: {Theme::SPACING_MD};
                    border-right: 1px solid {th.border()};
                ",
                "⚡ RSpice"
            }

            // File operations group
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::File, size: 16 } },
                    "New"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::FolderOpen, size: 16 } },
                    "Open"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Save, size: 16 } },
                    "Save"
                }
            }

            // Divider
            ToolbarDivider {}

            // Edit operations
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Undo, size: 16 } },
                    ""
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Redo, size: 16 } },
                    ""
                }
            }

            // Spacer
            div { style: "flex: 1;" }

            // Simulation controls
            ToolbarGroup {
                if is_running {
                    Button {
                        variant: ButtonVariant::Danger,
                        icon: rsx! { Icon { icon: IconType::Stop, size: 16, color: "#ffffff".to_string() } },
                        onclick: move |_| {
                            sim_state.write().is_running = false;
                            log::info!("Simulation stopped");
                        },
                        "Stop"
                    }
                } else {
                    Button {
                        variant: ButtonVariant::Success,
                        icon: rsx! { Icon { icon: IconType::Play, size: 16, color: "#ffffff".to_string() } },
                        onclick: move |_| {
                            sim_state.write().is_running = true;
                            log::info!("Simulation started");
                        },
                        "Run"
                    }
                }
            }

            // Divider
            ToolbarDivider {}

            // View controls
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::ZoomIn, size: 16 } },
                    ""
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::ZoomOut, size: 16 } },
                    ""
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::FitToScreen, size: 16 } },
                    ""
                }
            }

            // Settings
            Button {
                variant: ButtonVariant::Ghost,
                icon: rsx! { Icon { icon: IconType::Settings, size: 18 } },
                ""
            }
        }
    }
}

/// Toolbar button group
#[component]
fn ToolbarGroup(children: Element) -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 2px;
            ",
            {children}
        }
    }
}

/// Vertical divider between toolbar sections
#[component]
fn ToolbarDivider() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "
                width: 1px;
                height: 24px;
                background: {th.border()};
                margin: 0 {Theme::SPACING_XS};
            "
        }
    }
}

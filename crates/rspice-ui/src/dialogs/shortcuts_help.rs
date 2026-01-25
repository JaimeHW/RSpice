//! Keyboard Shortcuts Help Dialog
//!
//! Reference dialog displaying all available keyboard shortcuts organized by category.
//! Essential for professional EDA tools to help users work efficiently.

use dioxus::prelude::*;

use crate::state::shortcuts::{ShortcutCategory, ShortcutRegistry};
use crate::theme::Theme;

//=============================================================================
// Shortcuts Help Dialog Component
//=============================================================================

/// Props for the shortcuts help dialog
#[derive(Props, Clone, PartialEq)]
pub struct ShortcutsHelpDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Handler for closing the dialog
    pub on_close: EventHandler<()>,
}

/// Keyboard shortcuts reference dialog
#[component]
pub fn ShortcutsHelpDialog(props: ShortcutsHelpDialogProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let theme = use_context::<Signal<Theme>>();
    let registry = ShortcutRegistry::default();
    let grouped = registry.actions_by_category();

    rsx! {
        // Backdrop
        div {
            class: "shortcuts-dialog-backdrop",
            style: "
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.7);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 10000;
            ",
            onclick: move |_| props.on_close.call(()),

            // Dialog
            div {
                class: "shortcuts-dialog",
                style: format!(
                    "
                    background: {};
                    border: 1px solid {};
                    border-radius: 12px;
                    padding: 24px;
                    width: 700px;
                    max-height: 80vh;
                    overflow-y: auto;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
                    ",
                    theme.read().bg_secondary(),
                    theme.read().border()
                ),
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    style: "
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        margin-bottom: 24px;
                    ",

                    h2 {
                        style: format!(
                            "
                            font-size: 20px;
                            font-weight: 600;
                            color: {};
                            margin: 0;
                            ",
                            theme.read().text_primary()
                        ),
                        "⌨️ Keyboard Shortcuts"
                    }

                    button {
                        style: format!(
                            "
                            background: none;
                            border: none;
                            color: {};
                            font-size: 24px;
                            cursor: pointer;
                            padding: 4px 8px;
                            border-radius: 4px;
                            ",
                            theme.read().text_muted()
                        ),
                        onclick: move |_| props.on_close.call(()),
                        "×"
                    }
                }

                // Shortcut categories in columns
                div {
                    style: "
                        display: grid;
                        grid-template-columns: repeat(2, 1fr);
                        gap: 24px;
                    ",

                    for (category, actions) in grouped {
                        ShortcutCategory {
                            category: category,
                            actions: actions.iter().map(|(a, b)| (*a, b.to_vec())).collect(),
                        }
                    }
                }

                // Footer
                div {
                    style: format!(
                        "
                        margin-top: 24px;
                        padding-top: 16px;
                        border-top: 1px solid {};
                        text-align: center;
                        font-size: 12px;
                        color: {};
                        ",
                        theme.read().border_subtle(),
                        theme.read().text_muted()
                    ),
                    "Press Ctrl+/ to show this dialog"
                }
            }
        }
    }
}

//=============================================================================
// Helper Components
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct ShortcutCategoryProps {
    category: ShortcutCategory,
    actions: Vec<(
        crate::state::shortcuts::ShortcutAction,
        Vec<crate::state::shortcuts::KeyBinding>,
    )>,
}

#[component]
fn ShortcutCategory(props: ShortcutCategoryProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        div {
            // Category header
            h3 {
                style: format!(
                    "
                    font-size: 14px;
                    font-weight: 600;
                    color: {};
                    margin: 0 0 12px 0;
                    padding-bottom: 8px;
                    border-bottom: 1px solid {};
                    ",
                    theme.read().accent_primary(),
                    theme.read().border_subtle()
                ),
                "{props.category.display_name()}"
            }

            // Actions list
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                for (action, bindings) in &props.actions {
                    ShortcutRow {
                        action_name: action.display_name().to_string(),
                        shortcut: bindings.first().map(|b| b.display()).unwrap_or_default(),
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ShortcutRowProps {
    action_name: String,
    shortcut: String,
}

#[component]
fn ShortcutRow(props: ShortcutRowProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: space-between;
                align-items: center;
            ",

            span {
                style: format!("color: {}; font-size: 13px;", theme.read().text_secondary()),
                "{props.action_name}"
            }

            span {
                style: format!(
                    "
                    background: {};
                    color: {};
                    font-family: {};
                    font-size: 11px;
                    padding: 3px 8px;
                    border-radius: 4px;
                    min-width: 60px;
                    text-align: center;
                    ",
                    theme.read().surface(),
                    theme.read().text_primary(),
                    Theme::FONT_MONO
                ),
                "{props.shortcut}"
            }
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_category_all_have_display_names() {
        for category in ShortcutCategory::all() {
            assert!(!category.display_name().is_empty());
        }
    }
}

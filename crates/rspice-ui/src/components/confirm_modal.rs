//! Confirmation Modal Component
//!
//! Professional modal dialog for confirming destructive actions like
//! discarding unsaved changes.

use dioxus::prelude::*;

use super::button::{Button, ButtonVariant};
use crate::theme::Theme;

/// Result of the unsaved changes dialog
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaveDialogResult {
    /// Save changes before proceeding
    Save,
    /// Discard changes and proceed
    DontSave,
    /// Cancel the operation
    Cancel,
}

/// Confirmation modal for unsaved changes
#[component]
pub fn UnsavedChangesModal(
    /// Whether the modal is visible
    visible: bool,
    /// Optional filename to display
    filename: Option<String>,
    /// Callback when user makes a choice
    on_result: EventHandler<SaveDialogResult>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    if !visible {
        return rsx! {};
    }

    let display_name = filename.unwrap_or_else(|| "Untitled".to_string());

    rsx! {
        // Backdrop
        div {
            style: "
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.6);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 1000;
            ",
            onclick: move |_| {
                on_result.call(SaveDialogResult::Cancel);
            },

            // Modal dialog - compact professional design
            div {
                style: "
                    background: {th.bg_secondary()};
                    border: 1px solid {th.border()};
                    border-radius: {Theme::RADIUS_MD};
                    padding: 16px 20px;
                    width: 360px;
                    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
                ",
                onclick: move |e| {
                    e.stop_propagation();
                },

                // Title
                div {
                    style: "
                        font-size: 14px;
                        font-weight: 600;
                        color: {th.text_primary()};
                        margin-bottom: 8px;
                    ",
                    "Save Changes?"
                }

                // Message
                div {
                    style: "
                        font-size: 13px;
                        color: {th.text_secondary()};
                        margin-bottom: 16px;
                        line-height: 1.4;
                    ",
                    "Do you want to save changes to \"{display_name}\"?"
                }

                // Buttons - full width row with proper spacing
                div {
                    style: "
                        display: flex;
                        gap: 8px;
                    ",

                    // Cancel on the left
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            on_result.call(SaveDialogResult::Cancel);
                        },
                        "Cancel"
                    }

                    // Spacer to push action buttons right
                    div { style: "flex: 1;" }

                    // Don't Save
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            on_result.call(SaveDialogResult::DontSave);
                        },
                        "Don't Save"
                    }

                    // Save - primary action
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            on_result.call(SaveDialogResult::Save);
                        },
                        "Save"
                    }
                }
            }
        }
    }
}

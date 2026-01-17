//! Component Edit Modal
//!
//! Modal popup for editing component properties (name, value, parameters).

use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;

use crate::state::{ComponentType, SchematicState};
use crate::theme::Theme;

/// Props for ComponentEditModal
#[derive(Props, Clone, PartialEq)]
pub struct ComponentEditProps {
    /// ID of component being edited
    pub component_id: u64,
    /// Screen position for the modal
    pub position: (f64, f64),
    /// Callback when editing is complete
    pub on_save: EventHandler<(String, String, String)>,
    /// Callback when editing is cancelled
    pub on_cancel: EventHandler<()>,
    /// Schematic state for reading component data
    pub schematic: Signal<SchematicState>,
}

/// Modal popup for editing component properties
#[component]
pub fn ComponentEditModal(props: ComponentEditProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Get initial values from component
    let schematic = props.schematic.read();
    let comp = schematic
        .components
        .iter()
        .find(|c| c.id == props.component_id);

    let (initial_name, initial_value, initial_params, comp_type) = match comp {
        Some(c) => (
            c.name.clone(),
            c.value.clone(),
            c.params.clone(),
            Some(c.kind),
        ),
        None => (String::new(), String::new(), String::new(), None),
    };
    drop(schematic);

    // Local state for editing
    let mut name = use_signal(|| initial_name.clone());
    let mut value = use_signal(|| initial_value.clone());
    let mut params = use_signal(|| initial_params.clone());

    // Position the modal near the component
    let (x, y) = props.position;
    let left = (x - 150.0).max(10.0);
    let top = (y + 20.0).max(10.0);

    // Get type label
    let type_label = comp_type.map(|t| format!("{:?}", t)).unwrap_or_default();

    rsx! {
        // Backdrop
        div {
            style: "
                position: fixed;
                inset: 0;
                background: rgba(0, 0, 0, 0.5);
                z-index: 1000;
            ",
            onclick: move |_| props.on_cancel.call(()),
        }

        // Modal
        div {
            style: "
                position: fixed;
                left: {left}px;
                top: {top}px;
                background: {th.bg_secondary()};
                border: 1px solid {th.border()};
                border-radius: 8px;
                padding: 16px;
                min-width: 280px;
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
                z-index: 1001;
            ",
            // Prevent click from closing modal
            onclick: move |e| e.stop_propagation(),

            onkeydown: move |e| {
                match e.key() {
                    Key::Escape => props.on_cancel.call(()),
                    Key::Enter if !e.modifiers().shift() => {
                        props.on_save.call((
                            name.read().clone(),
                            value.read().clone(),
                            params.read().clone(),
                        ));
                    }
                    _ => {}
                }
            },

            // Header
            div {
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 16px;
                    padding-bottom: 8px;
                    border-bottom: 1px solid {th.border()};
                ",
                span {
                    style: "font-weight: 600; color: {th.text_primary()}; font-size: 14px;",
                    "Edit Component"
                }
                span {
                    style: "font-size: 11px; color: {th.text_muted()}; background: {th.bg_tertiary()}; padding: 2px 6px; border-radius: 4px;",
                    "{type_label}"
                }
            }

            // Form fields
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                // Name field
                div {
                    label {
                        style: "display: block; font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Reference Designator"
                    }
                    input {
                        r#type: "text",
                        value: "{name}",
                        placeholder: "R1, C2, etc.",
                        style: "
                            width: 100%;
                            padding: 8px 10px;
                            background: {th.bg_primary()};
                            border: 1px solid {th.border()};
                            border-radius: 6px;
                            color: {th.text_primary()};
                            font-size: 13px;
                            font-family: {Theme::FONT_MONO};
                            outline: none;
                            box-sizing: border-box;
                        ",
                        autofocus: true,
                        oninput: move |e| name.set(e.value().clone()),
                    }
                }

                // Value field
                div {
                    label {
                        style: "display: block; font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Value"
                    }
                    input {
                        r#type: "text",
                        value: "{value}",
                        placeholder: "1k, 10u, 5V, etc.",
                        style: "
                            width: 100%;
                            padding: 8px 10px;
                            background: {th.bg_primary()};
                            border: 1px solid {th.border()};
                            border-radius: 6px;
                            color: {th.text_primary()};
                            font-size: 13px;
                            font-family: {Theme::FONT_MONO};
                            outline: none;
                            box-sizing: border-box;
                        ",
                        oninput: move |e| value.set(e.value().clone()),
                    }
                }

                // Parameters field
                div {
                    label {
                        style: "display: block; font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "SPICE Parameters"
                    }
                    input {
                        r#type: "text",
                        value: "{params}",
                        placeholder: "model=xxx, tc=0.01, etc.",
                        style: "
                            width: 100%;
                            padding: 8px 10px;
                            background: {th.bg_primary()};
                            border: 1px solid {th.border()};
                            border-radius: 6px;
                            color: {th.text_primary()};
                            font-size: 13px;
                            font-family: {Theme::FONT_MONO};
                            outline: none;
                            box-sizing: border-box;
                        ",
                        oninput: move |e| params.set(e.value().clone()),
                    }
                }
            }

            // Buttons
            div {
                style: "
                    display: flex;
                    justify-content: flex-end;
                    gap: 8px;
                    margin-top: 16px;
                    padding-top: 12px;
                    border-top: 1px solid {th.border()};
                ",

                button {
                    style: "
                        padding: 8px 16px;
                        background: transparent;
                        border: 1px solid {th.border()};
                        border-radius: 6px;
                        color: {th.text_secondary()};
                        font-size: 12px;
                        cursor: pointer;
                    ",
                    onclick: move |_| props.on_cancel.call(()),
                    "Cancel"
                }

                button {
                    style: "
                        padding: 8px 16px;
                        background: #3b82f6;
                        border: none;
                        border-radius: 6px;
                        color: white;
                        font-size: 12px;
                        font-weight: 500;
                        cursor: pointer;
                    ",
                    onclick: move |_| {
                        props.on_save.call((
                            name.read().clone(),
                            value.read().clone(),
                            params.read().clone(),
                        ));
                    },
                    "Save"
                }
            }

            // Help text
            div {
                style: "margin-top: 12px; font-size: 10px; color: {th.text_muted()}; opacity: 0.7;",
                "Press Enter to save, Escape to cancel"
            }
        }
    }
}

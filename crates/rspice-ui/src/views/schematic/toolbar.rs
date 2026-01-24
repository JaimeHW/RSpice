//! Schematic Toolbar Component
//!
//! Tool selection buttons and view options for the schematic editor.

use dioxus::prelude::*;

use crate::state::{SchematicState, Tool};
use crate::theme::Theme;

/// Schematic toolbar with tool buttons and view options
#[component]
pub fn SchematicToolbar(schematic: Signal<SchematicState>) -> Element {
    let theme: Signal<Theme> = use_context();
    let mut display_settings: Signal<crate::state::display_settings::SchematicDisplaySettings> =
        use_context();
    let th = theme.read();
    let tool = schematic.read().tool;
    let settings = display_settings.read();

    // Current pin visibility state for toggle button
    let pins_visible = matches!(
        settings.show_pin_names,
        crate::state::display_settings::PinNameVisibility::Always
    );
    let pin_btn_label = if pins_visible {
        "📍 Pins ✓"
    } else {
        "📍 Pins"
    };

    let pin_bg = if pins_visible {
        th.accent_primary()
    } else {
        th.surface()
    };
    let pin_color = if pins_visible {
        "#fff"
    } else {
        th.text_primary()
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; height: 32px; padding: 0 8px; background: {th.bg_tertiary()}; border-bottom: 1px solid {th.border()}; gap: 4px;",
            // Tool buttons
            ToolBtn { label: "↖ Select", active: matches!(tool, Tool::Select), onclick: move |_| schematic.write().tool = Tool::Select }
            ToolBtn { label: "— Wire", active: matches!(tool, Tool::Wire), onclick: move |_| schematic.write().tool = Tool::Wire }
            ToolBtn { label: "⚡ Probe", active: matches!(tool, Tool::Probe), onclick: move |_| {
                let mut s = schematic.write();
                s.selection.clear();
                s.tool = Tool::Probe;
            }}
            ToolBtn { label: "🏷 Label", active: matches!(tool, Tool::Label), onclick: move |_| {
                let mut s = schematic.write();
                s.selection.clear();
                s.tool = Tool::Label;
            }}
            div { style: "width: 1px; height: 18px; background: {th.border()}; margin: 0 4px;" }
            button { style: "padding: 4px 8px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer;", onclick: move |_| schematic.write().rotate_selection(), "⟳ Rotate" }
            button { style: "padding: 4px 8px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer;", onclick: move |_| schematic.write().delete_selection(), "🗑 Delete" }

            // View section divider
            div { style: "width: 1px; height: 18px; background: {th.border()}; margin: 0 8px;" }
            span { style: "font-size: 11px; color: {th.text_muted()}; margin-right: 4px;", "View:" }

            // Pin names toggle (View option)
            button {
                style: "padding: 4px 8px; background: {pin_bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {pin_color}; font-size: 12px; cursor: pointer;",
                title: "Toggle terminal pin names",
                onclick: move |_| {
                    let mut ds = display_settings.write();
                    ds.show_pin_names = if matches!(ds.show_pin_names, crate::state::display_settings::PinNameVisibility::Always) {
                        crate::state::display_settings::PinNameVisibility::Hidden
                    } else {
                        crate::state::display_settings::PinNameVisibility::Always
                    };
                },
                "{pin_btn_label}"
            }

            // DC Annotation toggle
            {
                let mut sim_state: Signal<crate::state::SimulationState> = use_context();
                let mode = sim_state.read().dc_annotations.mode;
                let dc_label = match mode {
                    crate::state::dc_annotation::AnnotationMode::Hidden => "DC: Off",
                    crate::state::dc_annotation::AnnotationMode::Voltages => "DC: V",
                    crate::state::dc_annotation::AnnotationMode::Currents => "DC: I",
                    crate::state::dc_annotation::AnnotationMode::All => "DC: All",
                };
                let dc_active = !matches!(mode, crate::state::dc_annotation::AnnotationMode::Hidden);
                let dc_bg = if dc_active { th.accent_primary() } else { th.surface() };
                let dc_color = if dc_active { "#fff".to_string() } else { th.text_primary().to_string() };

                rsx! {
                    button {
                        style: "padding: 4px 8px; background: {dc_bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {dc_color}; font-size: 12px; cursor: pointer;",
                        title: "Cycle DC annotation mode",
                        onclick: move |_| {
                            let current_mode = sim_state.read().dc_annotations.mode;
                            let new_mode = current_mode.cycle();
                            sim_state.write().dc_annotations.mode = new_mode;
                        },
                        "{dc_label}"
                    }
                }
            }

            div { style: "flex: 1;" }
            span { style: "font-size: 12px; color: {th.text_muted()};", {format!("{} components, {} wires", schematic.read().components.len(), schematic.read().wires.len())} }
        }
    }
}

/// Tool button component
#[component]
fn ToolBtn(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (bg, col) = if active {
        (th.accent_primary(), "#fff")
    } else {
        (th.surface(), th.text_primary())
    };
    rsx! { button { style: "padding: 4px 8px; background: {bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {col}; font-size: 12px; cursor: pointer;", onclick: move |e| onclick.call(e), "{label}" } }
}

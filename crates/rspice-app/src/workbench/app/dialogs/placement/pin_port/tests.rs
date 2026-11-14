//! What Create pins states, arms and refuses.
//!
//! The claims worth pinning here are the ones a reader cannot check by eye:
//! that a batch is split the way the model splits names, that a coercion moves
//! the field the reader did not touch rather than refusing the one they did,
//! that pressing the primary changes nothing in the document, and that the
//! whole surface fits on a phone with the worst draft anyone could type into
//! it.

use super::*;
use crate::state::{Point, PortDirection, PortDiscipline, PortSignalType};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

/// Draw the form as its dialog step does.
pub(crate) fn render(app: &mut RSpiceApp, ctx: &egui::Context) {
    render_pin_port_dialog(&mut app.state.design.pin_port, &mut app.state.model, ctx);
}

/// The batch the form holds, over what it reads.
pub(crate) fn draft(state: &AppState) -> Draft {
    super::draft(&state.design.pin_port, &state.model)
}

pub(crate) fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_100.0, 850.0),
        )),
        events,
        ..Default::default()
    }
}

pub(crate) fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

#[test]
fn supply_coerces_signal_to_power_and_power_coerces_direction() {
    use rspice_schematic_editor::pin_placement::PinPlacementFields;

    let directions = [
        PortDirection::In,
        PortDirection::Out,
        PortDirection::InOut,
        PortDirection::Supply,
    ];
    let signals = [
        PortSignalType::Analog,
        PortSignalType::Logic,
        PortSignalType::Power,
    ];
    let mut state = crate::state::SchematicState::default();
    let mut placeable = |direction: PortDirection, signal: PortSignalType, picked: &str| {
        let pending = crate::state::PendingPortPlacement::from_contract(
            format!("PIN_{}", state.next_interface_order()),
            direction,
            signal,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        state
            .place_pending_port(Point::origin(), pending)
            .unwrap_or_else(|error| {
                panic!("picking {picked} settled on {direction:?}/{signal:?}: {error}")
            });
    };
    for was_direction in directions {
        for was_signal in signals {
            for picked in directions {
                let mut fields = PinPlacementFields {
                    direction: was_direction,
                    signal_type: was_signal,
                    ..Default::default()
                };
                fields.set_direction(picked);
                placeable(fields.direction, fields.signal_type, &format!("{picked:?}"));
            }
            for picked in signals {
                let mut fields = PinPlacementFields {
                    direction: was_direction,
                    signal_type: was_signal,
                    ..Default::default()
                };
                fields.set_signal_type(picked);
                placeable(fields.direction, fields.signal_type, &format!("{picked:?}"));
            }
        }
    }
}

/// Every labelled node the form publishes, with its role and bounds, after a
/// few frames at the desktop size.
pub(crate) fn published(app: &mut RSpiceApp) -> Vec<(egui::accesskit::Role, String, egui::Rect)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(app, ctx));
        nodes.extend(
            output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes,
        );
    }
    nodes
        .into_values()
        .filter_map(|node| {
            let b = node.bounds()?;
            // A plain label publishes its text as the value.
            let text = node
                .label()
                .filter(|label| !label.is_empty())
                .or_else(|| node.value())?;
            Some((
                node.role(),
                text.to_owned(),
                egui::Rect::from_min_max(
                    egui::pos2(b.x0 as f32, b.y0 as f32),
                    egui::pos2(b.x1 as f32, b.y1 as f32),
                ),
            ))
        })
        .collect()
}

// ============================================================================
// Renders for a human to look at
// ============================================================================

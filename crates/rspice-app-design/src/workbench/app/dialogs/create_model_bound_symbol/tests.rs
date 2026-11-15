//! Create-symbol dialog tests.

use egui::Context;

#[cfg(test)]
use super::controller::{inferred_model_pins, parse_target};
use super::state::*;
#[cfg(test)]
use crate::state::model_library::ModelType;
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

/// Draw the dialog as its dialog step does.
pub fn render(
    draft: &mut CreateModelBoundSymbolDialogState,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    ctx: &Context,
) {
    super::render::render_create_model_bound_symbol_dialog(draft, model, workbench, ctx);
}

pub fn node_bounds(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    role: egui::accesskit::Role,
    label: &str,
) -> egui::accesskit::Rect {
    nodes
        .iter()
        .find(|(_, node)| node.role() == role && node.label() == Some(label))
        .and_then(|(_, node)| node.bounds())
        .unwrap_or_else(|| panic!("missing {role:?} node {label}"))
}

#[test]
fn target_parser_requires_one_canonical_library_cell_identity() {
    assert_eq!(parse_target("work / ota").unwrap(), ("work", "ota"));
    assert!(parse_target("work").is_err());
    assert!(parse_target("work / ota / symbol").is_err());
    assert!(parse_target("work / 12bad").is_err());
}

#[test]
fn inferred_standard_model_contracts_remain_ordered_and_review_gated() {
    let mos = inferred_model_pins(ModelType::Nmos);
    assert_eq!(
        mos.iter().map(|pin| pin.name.as_str()).collect::<Vec<_>>(),
        ["D", "G", "S", "B"]
    );
    assert_eq!(mos[1].electrical_type, CreateSymbolPinType::AnalogInput);
    assert!(inferred_model_pins(ModelType::Other).is_empty());
}

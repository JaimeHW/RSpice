//! What the canvas marks for the current finding beyond its own badge.

use super::*;
use crate::state::{Cell, CellViewRef, ComponentType, Library, NetLabel, Point, View, ViewType};
use rspice_design::drc::DrcViolationType;

/// One source of the loop [`draw_source_loop`] draws.
pub(crate) struct LoopSource {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) position: Point,
}

/// Three voltage sources drawn round one loop, `a` to `b` to `0` and back to
/// `a`, each joined by labels on its terminals, in `work/top`, beside an empty
/// `work/aux`. `top` is open and checked; the loop is the one finding that
/// involves the three.
pub(crate) fn draw_source_loop(
    state: &mut AppState,
) -> (CellViewRef, CellViewRef, Vec<LoopSource>) {
    let mut library = Library::new("work");
    for name in ["top", "aux"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
    }
    state.library_manager.add_library(library);
    let top = CellViewRef::new("work", "top", "schematic");
    let aux = CellViewRef::new("work", "aux", "schematic");
    state.open_workspace_view(top.clone());

    let mut label = 9000;
    let mut sources = Vec::new();
    for (index, (plus, minus)) in [("a", "b"), ("b", "0"), ("0", "a")].into_iter().enumerate() {
        let id = state.schematic.add_component(
            ComponentType::VoltageSource,
            Point::new(300 * index as i32, 0),
        );
        let component = state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("the source is placed")
            .clone();
        for (pin, node) in [("+", plus), ("-", minus)] {
            let point = component
                .terminal_positions()
                .into_iter()
                .find_map(|(name, point)| (name == pin).then_some(point))
                .expect("a voltage source has both terminals");
            label += 1;
            state
                .schematic
                .document_mut_for_test()
                .net_labels
                .push(NetLabel::new(label, point, node));
        }
        sources.push(LoopSource {
            id,
            name: component.name.clone(),
            position: component.pos,
        });
    }
    state.sync_active_schematic_to_workspace();
    let result = state
        .check_active_cellview()
        .expect("the checks run on top");
    assert_eq!(
        result
            .violations()
            .iter()
            .filter(|finding| finding.violation_type == DrcViolationType::SourceToSource)
            .count(),
        1,
        "one loop: {:?}",
        result.violations()
    );
    (top, aux, sources)
}

/// The identity the published loop finding carries.
pub(crate) fn loop_finding(state: &AppState, subject: &CellViewRef) -> uuid::Uuid {
    let crate::workbench::app_state::DesignCheckStatus::Current(receipt) =
        state.design_check_status(subject)
    else {
        panic!("the check of the subject is current");
    };
    receipt
        .result
        .violations()
        .iter()
        .zip(state.design_rule_finding_identities(subject))
        .find_map(|(finding, identity)| {
            (finding.violation_type == DrcViolationType::SourceToSource).then_some(identity)
        })
        .expect("the loop is published")
}

/// With a loop current, every source in it is marked where it is drawn, in
/// the finding's own severity colour; with nothing current, nothing is.
#[test]
fn every_source_in_the_current_loop_is_marked() {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    let (top, _, sources) = draw_source_loop(&mut state);
    assert_eq!(related_marks(&state), None, "no finding is current");

    state
        .diagnostics
        .set_current_finding(Some(loop_finding(&state, &top)));
    let (severity, mut marked) = related_marks(&state).expect("the loop's sources are marked");

    let mut drawn: Vec<Point> = sources.iter().map(|source| source.position).collect();
    marked.sort_by_key(|point| (point.x, point.y));
    drawn.sort_by_key(|point| (point.x, point.y));
    assert_eq!(
        marked, drawn,
        "each of the three sources, where it is drawn"
    );
    assert_eq!(
        severity,
        DrcViolationType::SourceToSource.default_severity()
    );
}

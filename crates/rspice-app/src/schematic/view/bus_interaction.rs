//! App inputs for the editor's shared bus-tap preview and click query.

use crate::{state::Point, workbench::app_state::AppState};
pub(super) use rspice_schematic_editor::view::bus_interaction::BusTapCandidateError;
use rspice_schematic_editor::view::bus_interaction::{self, BusTapCandidate};

pub(super) fn resolve_bus_tap_candidate_on_active_sheet(
    state: &AppState,
    requested: Point,
    source_hit_radius: i32,
) -> Result<BusTapCandidate, BusTapCandidateError> {
    bus_interaction::resolve_bus_tap_candidate(
        &super::schematic_design_view(state),
        state
            .schematic
            .session
            .editor
            .pending_bus_tap
            .as_ref()
            .map(|placement| &placement.configuration),
        requested,
        source_hit_radius,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Bus, BusDeclaration, BusSlice, BusTapOrientation, Wire};
    #[test]
    fn non_lattice_source_projection_preview_always_commits_exactly() {
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().buses.push(
            Bus::segment(
                1,
                Point::new(0, 0),
                Point::new(10, 3),
                Some(declaration.clone()),
            )
            .unwrap(),
        );
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(2, Point::new(20, 3), Point::new(20, 12)));
        let pending = crate::state::PendingBusTap::new(
            declaration,
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Automatic,
        )
        .unwrap();
        state.schematic.session.editor.pending_bus_tap = Some(
            rspice_schematic_editor::session::bus::PendingBusTapPlacement::new(
                pending.clone(),
                crate::workbench::app::schematic_editor_request_source(&state),
            ),
        );

        let candidate =
            resolve_bus_tap_candidate_on_active_sheet(&state, Point::new(5, 2), 6).unwrap();
        assert_eq!(candidate.bus_point, Point::new(10, 3));
        assert!(state.schematic.document().buses[0].contains_point(candidate.bus_point));
        assert!(
            state
                .schematic
                .place_configured_bus_tap(
                    candidate.bus_id,
                    candidate.bus_point,
                    candidate.connection_point,
                    &pending,
                )
                .is_ok()
        );
    }
}

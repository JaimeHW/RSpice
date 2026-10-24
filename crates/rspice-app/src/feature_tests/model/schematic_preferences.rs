//! Personal schematic editing behaviour applied to a whole session's open
//! schematic and to the schematics it creates.

use crate::state::{Point, WireRoutingMode};
use crate::workbench::ChoicePreference;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::schematic_preferences::{
    ANY_ANGLE_ROUTING, MANUAL_JUNCTION_DOTS, SELECTION_CROSSING,
};

/// Changing the row changes the open schematic: its next T-join gets a
/// dot or not according to the setting, with no new document involved.
#[test]
fn junction_dots_follow_the_preference_on_the_open_schematic() {
    for (stored, dots) in [(0, 1), (MANUAL_JUNCTION_DOTS, 0)] {
        let mut state = AppState::default();
        state
            .model
            .preferences
            .set_choice(ChoicePreference::WireJunctionBehavior, stored)
            .expect("a stored junction choice");
        state.apply_schematic_editing_preferences();
        state
            .model
            .schematic
            .add_wire(vec![Point::new(0, 0), Point::new(40, 0)]);
        state.model.schematic.start_wire(Point::new(20, 40));
        state.model.schematic.extend_wire(Point::new(20, 0));
        state.model.schematic.finish_wire(|_| false);
        assert_eq!(state.model.schematic.document().junctions.len(), dots);
    }
}

/// The routing a new schematic starts with is the preference's, not the
/// session's last: an orthogonal preference does not inherit angled
/// routing from the session, and an any-angle one opens free routing on a
/// document that permits it.
#[test]
fn new_schematic_routing_comes_from_the_preference() {
    use crate::state::WireJunctionPolicy;

    let mut state = AppState::default();
    state.design.schematic_routing_mode = WireRoutingMode::Diagonal;
    let created = state.new_schematic_document();
    assert_eq!(
        created.session.editor.wire_drawing.routing_mode,
        WireRoutingMode::HorizontalFirst
    );
    assert_eq!(
        created.document().document_policy.wire_junctions,
        WireJunctionPolicy::OrthogonalAutomatic
    );

    state
        .model
        .preferences
        .set_choice(ChoicePreference::WireJunctionBehavior, ANY_ANGLE_ROUTING)
        .expect("a stored routing choice");
    state.design.schematic_routing_mode = WireRoutingMode::HorizontalFirst;
    let created = state.new_schematic_document();
    assert_eq!(
        created.session.editor.wire_drawing.routing_mode,
        WireRoutingMode::Diagonal
    );
    assert_eq!(
        created.session.editor.bus_drawing.routing_mode,
        WireRoutingMode::Diagonal
    );
    assert_eq!(
        created.document().document_policy.wire_junctions,
        WireJunctionPolicy::AnyAngle
    );
}

#[test]
fn drag_selection_follows_the_preference_on_the_open_schematic() {
    let mut state = AppState::default();
    for (stored, policy) in SELECTION_CROSSING.into_iter().enumerate() {
        state
            .model
            .preferences
            .set_choice(ChoicePreference::SelectionCrossingPolicy, stored)
            .expect("a stored selection choice");
        state.apply_schematic_editing_preferences();
        assert_eq!(
            state.model.schematic.session.editor.selection_crossing,
            policy
        );
    }
}

//! Personal schematic editing behaviour, read from Preferences.
//!
//! Junction dots and drag selection belong to the person editing, not to the
//! drawing, so they come from Preferences for whatever schematic is open and
//! a change applies at once. Wire routing is a default instead: it seeds a
//! new schematic, and View ▸ Grid, snap and wire routing changes it for the
//! session.
//!
//! One stored choice holds both halves of the junction-and-routing setting,
//! so the values an earlier build wrote keep their meaning: 0 automatic dots
//! with orthogonal routing, 1 manual dots, 2 any-angle routing, and 3 both.

use super::AppState;
use crate::state::{SelectionCrossingPolicy, WireRoutingMode};
use crate::workbench::{ChoicePreference, UserPreferences};

/// The bit of [`ChoicePreference::WireJunctionBehavior`] that says the user
/// places junction dots.
pub(crate) const MANUAL_JUNCTION_DOTS: usize = 1;
/// The bit of [`ChoicePreference::WireJunctionBehavior`] that says new
/// schematics route at any angle.
pub(crate) const ANY_ANGLE_ROUTING: usize = 2;

/// Drag-selection behaviour in stored index order.
pub(crate) const SELECTION_CROSSING: [SelectionCrossingPolicy; 3] = [
    SelectionCrossingPolicy::Directional,
    SelectionCrossingPolicy::EnclosedOnly,
    SelectionCrossingPolicy::Intersecting,
];

impl UserPreferences {
    /// Whether an edit that makes a T-join adds its junction dot.
    #[must_use]
    pub(crate) fn automatic_junction_dots(&self) -> bool {
        self.choice(ChoicePreference::WireJunctionBehavior) & MANUAL_JUNCTION_DOTS == 0
    }

    /// Whether new schematics route wires at any angle.
    #[must_use]
    pub(crate) fn any_angle_wire_routing(&self) -> bool {
        self.choice(ChoicePreference::WireJunctionBehavior) & ANY_ANGLE_ROUTING != 0
    }

    /// Which objects a drag box selects.
    #[must_use]
    pub(crate) fn selection_crossing(&self) -> SelectionCrossingPolicy {
        SELECTION_CROSSING
            .get(self.choice(ChoicePreference::SelectionCrossingPolicy))
            .copied()
            .unwrap_or_default()
    }

    /// The routing a new schematic starts with: any angle, or the session's
    /// own orthogonal mode (horizontal or vertical first) when it has one.
    #[must_use]
    pub(crate) fn new_schematic_routing_mode(&self, session: WireRoutingMode) -> WireRoutingMode {
        if self.any_angle_wire_routing() {
            WireRoutingMode::Diagonal
        } else if session.is_orthogonal() {
            session
        } else {
            WireRoutingMode::HorizontalFirst
        }
    }
}

impl AppState {
    /// Put the personal editing behaviour on the open schematic. Run at the
    /// top of every frame, so a changed Preferences row applies to the open
    /// schematic before its next edit.
    pub(crate) fn apply_schematic_editing_preferences(&mut self) {
        let editor = &mut self.schematic.session.editor;
        editor.automatic_junctions = self.ui.preferences.automatic_junction_dots();
        editor.selection_crossing = self.ui.preferences.selection_crossing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    /// Changing the row changes the open schematic: its next T-join gets a
    /// dot or not according to the setting, with no new document involved.
    #[test]
    fn junction_dots_follow_the_preference_on_the_open_schematic() {
        for (stored, dots) in [(0, 1), (MANUAL_JUNCTION_DOTS, 0)] {
            let mut state = AppState::default();
            state
                .ui
                .preferences
                .set_choice(ChoicePreference::WireJunctionBehavior, stored)
                .expect("a stored junction choice");
            state.apply_schematic_editing_preferences();
            state
                .schematic
                .add_wire(vec![Point::new(0, 0), Point::new(40, 0)]);
            state.schematic.start_wire(Point::new(20, 40));
            state.schematic.extend_wire(Point::new(20, 0));
            state.schematic.finish_wire(|_| false);
            assert_eq!(state.schematic.document().junctions.len(), dots);
        }
    }

    /// The values an earlier build stored keep their meaning: 1 was manual
    /// dots with orthogonal routing, and 2 was any-angle routing, whose dots
    /// were automatic.
    #[test]
    fn stored_junction_choices_keep_their_meaning() {
        let mut preferences = UserPreferences::default();
        for (stored, automatic, any_angle) in [
            (0, true, false),
            (1, false, false),
            (2, true, true),
            (3, false, true),
        ] {
            preferences
                .set_choice(ChoicePreference::WireJunctionBehavior, stored)
                .expect("a stored junction choice");
            assert_eq!(preferences.automatic_junction_dots(), automatic, "{stored}");
            assert_eq!(preferences.any_angle_wire_routing(), any_angle, "{stored}");
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
        state.ui.schematic_routing_mode = WireRoutingMode::Diagonal;
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
            .ui
            .preferences
            .set_choice(ChoicePreference::WireJunctionBehavior, ANY_ANGLE_ROUTING)
            .expect("a stored routing choice");
        state.ui.schematic_routing_mode = WireRoutingMode::HorizontalFirst;
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
                .ui
                .preferences
                .set_choice(ChoicePreference::SelectionCrossingPolicy, stored)
                .expect("a stored selection choice");
            state.apply_schematic_editing_preferences();
            assert_eq!(state.schematic.session.editor.selection_crossing, policy);
        }
    }
}

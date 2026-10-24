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

use super::model::Model;
use crate::state::{NetNamingPolicy, SelectionCrossingPolicy, WireRoutingMode};
use crate::workbench::preferences::{ChoicePreference, UserPreferences};

/// The bit of [`ChoicePreference::WireJunctionBehavior`] that says the user
/// places junction dots.
pub const MANUAL_JUNCTION_DOTS: usize = 1;
/// The bit of [`ChoicePreference::WireJunctionBehavior`] that says new
/// schematics route at any angle.
pub const ANY_ANGLE_ROUTING: usize = 2;

/// Drag-selection behaviour in stored index order.
pub(crate) const SELECTION_CROSSING: [SelectionCrossingPolicy; 3] = [
    SelectionCrossingPolicy::Directional,
    SelectionCrossingPolicy::EnclosedOnly,
    SelectionCrossingPolicy::Intersecting,
];

/// The characters a net name may use, in stored index order, with the words
/// Preferences and the sheet inspector name each set by. Each fits the
/// inspector's value column at its default width whole.
pub const NET_NAME_CHARACTERS: [(NetNamingPolicy, &str); 2] = [
    (
        NetNamingPolicy::StrictCaseSensitive,
        "A\u{2013}Z a\u{2013}z 0\u{2013}9 _.$:/![]<>-",
    ),
    (
        NetNamingPolicy::SpiceCompatibleRelaxed,
        "Any printable but spaces",
    ),
];

impl UserPreferences {
    /// Whether an edit that makes a T-join adds its junction dot.
    #[must_use]
    pub fn automatic_junction_dots(&self) -> bool {
        self.choice(ChoicePreference::WireJunctionBehavior) & MANUAL_JUNCTION_DOTS == 0
    }

    /// Whether new schematics route wires at any angle.
    #[must_use]
    pub fn any_angle_wire_routing(&self) -> bool {
        self.choice(ChoicePreference::WireJunctionBehavior) & ANY_ANGLE_ROUTING != 0
    }

    /// Which objects a drag box selects.
    #[must_use]
    pub fn selection_crossing(&self) -> SelectionCrossingPolicy {
        SELECTION_CROSSING
            .get(self.choice(ChoicePreference::SelectionCrossingPolicy))
            .copied()
            .unwrap_or_default()
    }

    /// The characters a new schematic's net names may use.
    #[must_use]
    pub fn new_schematic_net_naming(&self) -> NetNamingPolicy {
        NET_NAME_CHARACTERS
            .get(self.choice(ChoicePreference::NetNamingPolicy))
            .map(|(policy, _)| *policy)
            .unwrap_or_default()
    }

    /// The routing a new schematic starts with: any angle, or the session's
    /// own orthogonal mode (horizontal or vertical first) when it has one.
    #[must_use]
    pub fn new_schematic_routing_mode(&self, session: WireRoutingMode) -> WireRoutingMode {
        if self.any_angle_wire_routing() {
            WireRoutingMode::Diagonal
        } else if session.is_orthogonal() {
            session
        } else {
            WireRoutingMode::HorizontalFirst
        }
    }
}

impl Model {
    /// Put the personal editing behaviour on the open schematic. Run at the
    /// top of every frame, so a changed Preferences row applies to the open
    /// schematic before its next edit.
    pub fn apply_schematic_editing_preferences(&mut self) {
        let editor = &mut self.schematic.session.editor;
        editor.automatic_junctions = self.preferences.automatic_junction_dots();
        editor.selection_crossing = self.preferences.selection_crossing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

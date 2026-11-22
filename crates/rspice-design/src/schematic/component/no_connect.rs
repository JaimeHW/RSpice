//! The pins an author marked as meant to stay open.
//!
//! A no-connect is a statement about one terminal of one placed part, as
//! KiCad's and Virtuoso's marks are: the pin is left open on purpose. It
//! joins nothing and changes no node, since the deck writes the pin as the
//! node of its own it always was. It changes what the design check says
//! about the pin, and the sheet draws a cross at the pin's end.

use super::Component;
use crate::resolved_symbol::ResolvedCellSymbol;
use rspice_design_model::Point;

impl Component {
    /// Whether the author marked `pin` as meant to stay open. A pin name
    /// matches whatever its case, as the deck matches it.
    pub fn is_no_connect(&self, pin: &str) -> bool {
        self.no_connect_pins
            .iter()
            .any(|flagged| flagged.eq_ignore_ascii_case(pin))
    }

    /// The pins marked no-connect, by terminal name.
    pub fn no_connect_pins(&self) -> impl Iterator<Item = &str> {
        self.no_connect_pins.iter().map(String::as_str)
    }

    /// Whether any pin of the part is marked no-connect.
    pub fn has_no_connects(&self) -> bool {
        !self.no_connect_pins.is_empty()
    }

    /// Mark `pin` no-connect, or clear its mark. Says whether that changed
    /// anything.
    pub fn set_no_connect(&mut self, pin: &str, flagged: bool) -> bool {
        if flagged {
            !self.is_no_connect(pin) && self.no_connect_pins.insert(pin.to_owned())
        } else {
            let before = self.no_connect_pins.len();
            self.no_connect_pins
                .retain(|marked| !marked.eq_ignore_ascii_case(pin));
            self.no_connect_pins.len() != before
        }
    }

    /// Carry every mark onto the pin `pin_for` names for it, and drop each
    /// mark whose pin it names none for. Returns the dropped pins' names.
    ///
    /// A part that changes what it is keeps its marks on the pins it still
    /// has, and only there: a mark left on a name the part no longer has
    /// would come back unasked on any pin that later takes that name.
    pub fn carry_no_connects(
        &mut self,
        mut pin_for: impl FnMut(&str) -> Option<String>,
    ) -> Vec<String> {
        let mut dropped = Vec::new();
        let marks = std::mem::take(&mut self.no_connect_pins);
        for mark in marks {
            match pin_for(&mark) {
                Some(pin) => {
                    self.set_no_connect(&pin, true);
                }
                None => dropped.push(mark),
            }
        }
        dropped
    }

    /// Where the sheet draws a no-connect cross: the end of each marked pin,
    /// with the pins resolved as `terminal_positions_resolved` resolves them.
    pub fn no_connect_terminals(&self, resolved_symbol: Option<&ResolvedCellSymbol>) -> Vec<Point> {
        if self.no_connect_pins.is_empty() {
            return Vec::new();
        }
        self.terminal_positions_resolved(resolved_symbol)
            .into_iter()
            .filter(|(pin, _)| self.is_no_connect(pin))
            .map(|(_, point)| point)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::schematic::component::Component;
    use crate::schematic::component_type::ComponentType;
    use crate::schematic::rotation::Rotation;
    use rspice_design_model::Point;

    fn resistor() -> Component {
        Component::new(7, ComponentType::Resistor, Point::new(100, 100))
    }

    #[test]
    fn a_mark_is_set_once_cleared_once_and_matches_any_case() {
        let mut part = Component::new(7, ComponentType::Nmos, Point::new(0, 0));
        assert!(part.set_no_connect("B", true));
        assert!(
            !part.set_no_connect("b", true),
            "the same pin is marked once"
        );
        assert!(part.is_no_connect("b"));
        assert_eq!(part.no_connect_pins().collect::<Vec<_>>(), ["B"]);
        assert!(part.set_no_connect("b", false));
        assert!(!part.set_no_connect("B", false));
        assert!(!part.has_no_connects());
    }

    #[test]
    fn the_cross_stays_on_its_pin_when_the_part_turns() {
        let mut part = resistor();
        part.set_no_connect("-", true);
        let minus = |part: &Component| {
            part.terminal_positions()
                .into_iter()
                .find(|(pin, _)| *pin == "-")
                .map(|(_, point)| point)
                .unwrap()
        };
        assert_eq!(part.no_connect_terminals(None), [minus(&part)]);
        part.rotation = Rotation::R90;
        assert_eq!(part.no_connect_terminals(None), [minus(&part)]);
        part.mirror_h = true;
        assert_eq!(part.no_connect_terminals(None), [minus(&part)]);
    }

    #[test]
    fn marks_survive_a_save_and_a_part_with_none_writes_nothing() {
        let mut part = resistor();
        assert!(!serde_json::to_string(&part).unwrap().contains("no_connect"));
        part.set_no_connect("+", true);
        let saved = serde_json::to_string(&part).unwrap();
        let reopened: Component = serde_json::from_str(&saved).unwrap();
        assert!(reopened.is_no_connect("+"));
        assert!(!reopened.is_no_connect("-"));
    }

    #[test]
    fn carried_marks_follow_the_pin_map_and_report_what_was_dropped() {
        let mut part = Component::new(7, ComponentType::Nmos, Point::new(0, 0));
        part.set_no_connect("B", true);
        part.set_no_connect("G", true);
        let dropped = part.carry_no_connects(|pin| (pin == "G").then(|| "g".to_owned()));
        assert_eq!(dropped, ["B"]);
        assert_eq!(part.no_connect_pins().collect::<Vec<_>>(), ["g"]);
    }
}

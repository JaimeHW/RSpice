//! Whether another cell fits where an instance is placed: the check a
//! configuration binding to another cell is made against, when it is entered
//! and every time the hierarchy resolves.
//!
//! Pins meet by name, as Virtuoso's netlister connects a switch master's
//! terminals: the cell bound in must declare every pin the placement
//! connects, at the same width, in a direction that does not contradict the
//! placed one. A pin of its own the placement does not connect is left
//! unconnected, as that netlister leaves a switch master's extra terminal,
//! and is said, never dropped in silence.

use crate::schematic::port::{PortDirection, PortSpec};

/// How one cell's pins meet the pins an instance places.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PortFit {
    /// Placed pins the cell does not declare.
    missing: Vec<String>,
    /// Placed pins the cell declares at another width: the pin as placed,
    /// and its width placed and declared.
    widths: Vec<(String, usize, usize)>,
    /// Placed pins whose direction the cell contradicts: the pin, and its
    /// direction placed and declared.
    directions: Vec<(String, PortDirection, PortDirection)>,
    /// The cell's pins the placement does not connect, as the cell names
    /// them.
    unplaced: Vec<String>,
}

/// Measure `target`'s pins against the pins an instance places.
pub fn port_fit(placed: &[PortSpec], target: &[PortSpec]) -> PortFit {
    let mut fit = PortFit::default();
    for pin in placed {
        let Some(declared) = target.iter().find(|port| same_pin(port, pin)) else {
            fit.missing.push(pin.name.clone());
            continue;
        };
        if declared.width() != pin.width() {
            fit.widths
                .push((pin.name.clone(), pin.width(), declared.width()));
        }
        if contradicts(pin.direction, declared.direction) {
            fit.directions
                .push((pin.name.clone(), pin.direction, declared.direction));
        }
    }
    fit.unplaced = target
        .iter()
        .filter(|port| !placed.iter().any(|pin| same_pin(pin, port)))
        .map(|port| port.name.clone())
        .collect();
    fit
}

impl PortFit {
    /// Whether the cell can run in the placement's place.
    pub fn fits(&self) -> bool {
        self.missing.is_empty() && self.widths.is_empty() && self.directions.is_empty()
    }

    /// The cell's pins the placement leaves unconnected.
    pub fn unplaced(&self) -> &[String] {
        &self.unplaced
    }

    /// Which pins keep the cell out, each named, or `None` when it fits.
    pub fn refusal(&self) -> Option<String> {
        if self.fits() {
            return None;
        }
        let mut parts = Vec::with_capacity(3);
        if !self.missing.is_empty() {
            parts.push(format!(
                "it has no pin {}",
                or_list(&self.missing.iter().map(String::as_str).collect::<Vec<_>>())
            ));
        }
        for (pin, placed, declared) in &self.widths {
            parts.push(format!(
                "{pin} is {placed} wide where its pin is {declared} wide"
            ));
        }
        for (pin, placed, declared) in &self.directions {
            parts.push(format!(
                "{pin} is {} where its pin is {}",
                direction_word(*placed),
                direction_word(*declared)
            ));
        }
        Some(parts.join("; "))
    }

    /// What resolution says of a fitting cell that leaves pins of its own
    /// unconnected, or `None` when it connects them all.
    pub fn unconnected_warning(&self, cell: &str) -> Option<String> {
        (!self.unplaced.is_empty()).then(|| {
            format!(
                "{cell} pin{} {} {} left unconnected: the instance does not place {}",
                if self.unplaced.len() == 1 { "" } else { "s" },
                self.unplaced.join(", "),
                if self.unplaced.len() == 1 {
                    "is"
                } else {
                    "are"
                },
                if self.unplaced.len() == 1 {
                    "it"
                } else {
                    "them"
                },
            )
        })
    }
}

/// Whether two pins are one pin: the same name, a vector by its name
/// without its range, without regard to case.
pub fn same_pin(left: &PortSpec, right: &PortSpec) -> bool {
    pin_stem(&left.name).eq_ignore_ascii_case(&pin_stem(&right.name))
}

/// The name a pin is matched by: a vector's name without its range.
pub fn pin_stem(name: &str) -> String {
    rspice_design_model::bus::declared_vector(name)
        .map_or_else(|| name.trim().to_owned(), |vector| vector.name)
}

/// An input meeting an output, or a supply meeting a signal, is a
/// contradiction. A pin that states no direction states nothing to
/// contradict: `InOut` is also what an unclassified pin reads as.
fn contradicts(placed: PortDirection, declared: PortDirection) -> bool {
    placed != declared && placed != PortDirection::InOut && declared != PortDirection::InOut
}

fn direction_word(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::In => "an input",
        PortDirection::Out => "an output",
        PortDirection::InOut => "bidirectional",
        PortDirection::Supply => "a supply",
    }
}

fn or_list(names: &[&str]) -> String {
    match names {
        [] => String::new(),
        [only] => (*only).to_owned(),
        [rest @ .., last] => format!("{} or {last}", rest.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pin(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    /// The same pins in another order, in other case, fit: pins meet by
    /// name, never by place.
    #[test]
    fn pins_meet_by_name_in_any_order() {
        let placed = [pin("IN", PortDirection::In), pin("OUT", PortDirection::Out)];
        let target = [pin("out", PortDirection::Out), pin("in", PortDirection::In)];
        let fit = port_fit(&placed, &target);
        assert!(fit.fits());
        assert_eq!(fit.refusal(), None);
        assert!(fit.unplaced().is_empty());
    }

    /// Every pin that keeps a cell out is named, with what differs.
    #[test]
    fn a_misfit_names_each_pin_and_what_differs() {
        let placed = [
            pin("IN", PortDirection::In),
            pin("OUT", PortDirection::Out),
            pin("VDD", PortDirection::Supply),
            pin("D[7:0]", PortDirection::In),
        ];
        let target = [
            pin("IN", PortDirection::Out),
            pin("D[3:0]", PortDirection::In),
        ];
        let fit = port_fit(&placed, &target);
        assert!(!fit.fits());
        assert_eq!(
            fit.refusal().as_deref(),
            Some(
                "it has no pin OUT or VDD; D[7:0] is 8 wide where its pin is 4 wide; \
                 IN is an input where its pin is an output"
            )
        );
    }

    /// A pin of the cell's own the placement does not connect does not keep
    /// it out; it is left unconnected, and said so.
    #[test]
    fn an_extra_pin_fits_and_is_left_unconnected() {
        let placed = [
            pin("A", PortDirection::InOut),
            pin("B", PortDirection::InOut),
        ];
        let target = [
            pin("A", PortDirection::In),
            pin("B", PortDirection::Out),
            pin("TRIM", PortDirection::In),
        ];
        let fit = port_fit(&placed, &target);
        assert!(fit.fits(), "a bidirectional pin contradicts no direction");
        assert_eq!(fit.unplaced(), ["TRIM"]);
        assert_eq!(
            fit.unconnected_warning("vendor/filter").as_deref(),
            Some("vendor/filter pin TRIM is left unconnected: the instance does not place it")
        );
    }
}

//! Pin roles of an imported subcircuit, read from what the vendor wrote.
//!
//! A `.SUBCKT` port list is positional: `1 2 99 50 45` says nothing about
//! which node is the output. Vendors say it twice, though — in pin names
//! (`IN+ IN- VCC VEE OUT`) and in the comment block above the definition
//! (`* Node assignments`, `* CONNECTIONS:`, `* pin order:`). A role is taken
//! from the name when the name says one, and from the comment otherwise. A
//! pin neither names is bidirectional, which places it as before.

use rspice_design_model::port::{PinRole, PortDirection};

use crate::symbol_generation::{InterfacePin, PinMark};

/// Comment headings that introduce a vendor pin-order list.
const PIN_ORDER_HEADINGS: &[&str] = &[
    "node assignments",
    "node assignment",
    "connections",
    "pin order",
    "pinout",
    "pin out",
    "pins",
];

/// Roles for `pins`, in interface order, from their names and the comment
/// paragraph the parser attached to the definition.
pub fn infer_pin_roles(pins: &[String], description: Option<&str>) -> Vec<PinRole> {
    let listed = description.and_then(|text| pin_order_roles(text, pins.len()));
    pins.iter()
        .enumerate()
        .map(|(index, pin)| {
            role_from_name(pin)
                .or_else(|| listed.as_ref().map(|roles| roles[index]))
                .unwrap_or(PinRole::Bidirectional)
        })
        .collect()
}

/// The roles a vendor pin-order list states, when it states one for every
/// pin. Staircase blocks (`| | positive supply`) flatten to `|`-separated
/// entries; one-line lists separate entries by commas or spaces.
fn pin_order_roles(description: &str, count: usize) -> Option<Vec<PinRole>> {
    let lower = description.to_ascii_lowercase();
    let start = PIN_ORDER_HEADINGS
        .iter()
        .filter_map(|heading| lower.rfind(heading).map(|at| at + heading.len()))
        .max()?;
    let list = description[start..].trim_start_matches([':', ' ', '\t', '-', '=']);
    let separator = if list.contains('|') {
        '|'
    } else if list.contains(',') {
        ','
    } else {
        ' '
    };
    let entries = list
        .split(separator)
        .map(entry_text)
        .filter(|entry| !entry.is_empty())
        .take(count)
        .collect::<Vec<_>>();
    if count == 0 || entries.len() < count {
        return None;
    }
    entries.into_iter().map(role_from_name).collect()
}

/// An entry without its leading pin number: `1 = +IN`, `2: IN-`, `3) OUT`.
fn entry_text(entry: &str) -> &str {
    let entry = entry.trim();
    let rest = entry.trim_start_matches(|c: char| c.is_ascii_digit());
    if rest.len() < entry.len() {
        let rest = rest.trim_start();
        if let Some(stripped) = rest.strip_prefix(['=', ':', ')', '.', '-']) {
            return stripped.trim();
        }
    }
    entry
}

/// Positive supply names; the ones marked as prefixes also name `vdd2` or
/// `vdd_core`.
const POSITIVE_SUPPLIES: &[&str] = &[
    "V+", "VCC", "VDD", "VS+", "+VS", "VPOS", "+V", "VPLUS", "AVDD", "VDDA", "DVDD", "VDDD",
    "VDDIO", "VPWR", "VBAT",
];
const POSITIVE_SUPPLY_PREFIXES: &[&str] = &["VDD", "VCC", "AVDD", "DVDD"];
const NEGATIVE_SUPPLIES: &[&str] = &[
    "V-", "VEE", "VSS", "VS-", "-VS", "VNEG", "-V", "VMINUS", "AVSS", "VSSA", "DVSS", "VSSD",
];
const NEGATIVE_SUPPLY_PREFIXES: &[&str] = &["VSS", "VEE", "AVSS", "DVSS"];
/// `COM` is exact: `COMP` is a comparator's output.
const GROUNDS: &[&str] = &[
    "GND", "AGND", "DGND", "PGND", "GROUND", "COM", "0", "VGND", "GNDA", "GNDD",
];
const GROUND_PREFIXES: &[&str] = &["GND"];
const CLOCKS: &[&str] = &[
    "CLK", "CLOCK", "CK", "CLKIN", "PHI", "PHI1", "PHI2", "CLKB", "CLKN",
];

/// A name as the tables spell it: upper case, without spaces or `_`.
fn compact_name(name: &str) -> String {
    name.chars()
        .filter(|c| !c.is_whitespace() && *c != '_')
        .collect::<String>()
        .to_ascii_uppercase()
}

/// Whether `compact` is one of `names`, or one of `prefixes` followed by
/// more of a name.
fn named(compact: &str, names: &[&str], prefixes: &[&str]) -> bool {
    names.contains(&compact)
        || prefixes.iter().any(|prefix| {
            compact.strip_prefix(prefix).is_some_and(|rest| {
                rest.chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphanumeric())
            })
        })
}

/// The role and mark a generated symbol gives one port, by the first rule
/// that matches: a thermal discipline; a supply or ground name, whatever the
/// declared direction (a Verilog-A rail can only be `inout`); a declared
/// input, inverting or not as its name says; a declared output; a
/// bidirectional port whose name states a signal role; anything else is
/// bidirectional. A clock name marks an input.
pub fn interface_pin(name: &str, direction: PortDirection, thermal: bool) -> InterfacePin {
    if thermal {
        return InterfacePin {
            role: PinRole::Bidirectional,
            mark: PinMark::Thermal,
        };
    }
    let named_role = role_from_name(name);
    let role = match (named_role, direction) {
        (Some(role @ (PinRole::PositiveSupply | PinRole::NegativeSupply | PinRole::Ground)), _) => {
            role
        }
        (
            Some(role @ (PinRole::NonInvertingInput | PinRole::InvertingInput)),
            PortDirection::In,
        ) => role,
        (_, PortDirection::In) => PinRole::Input,
        (_, PortDirection::Out) => PinRole::Output,
        (_, PortDirection::Supply) => PinRole::PositiveSupply,
        (Some(role), PortDirection::InOut) => role,
        (None, PortDirection::InOut) => PinRole::Bidirectional,
    };
    let clock = matches!(role, PinRole::Input) && CLOCKS.contains(&compact_name(name).as_str());
    InterfacePin {
        role,
        mark: if clock {
            PinMark::Clock
        } else {
            PinMark::Plain
        },
    }
}

/// The role a pin name or a description phrase states, if any.
pub fn role_from_name(name: &str) -> Option<PinRole> {
    let compact = compact_name(name);
    if named(&compact, POSITIVE_SUPPLIES, POSITIVE_SUPPLY_PREFIXES) {
        return Some(PinRole::PositiveSupply);
    }
    if named(&compact, NEGATIVE_SUPPLIES, NEGATIVE_SUPPLY_PREFIXES) {
        return Some(PinRole::NegativeSupply);
    }
    if named(&compact, GROUNDS, GROUND_PREFIXES) {
        return Some(PinRole::Ground);
    }
    if CLOCKS.contains(&compact.as_str()) {
        return Some(PinRole::Input);
    }
    let role = match compact.as_str() {
        "IN+" | "+IN" | "INP" | "VIN+" | "+VIN" | "VINP" | "NONINV" | "NINV" | "NI" | "PLUS"
        | "+" | "INPLUS" => PinRole::NonInvertingInput,
        "IN-" | "-IN" | "INN" | "INM" | "VIN-" | "-VIN" | "VINN" | "VINM" | "INV" | "MINUS"
        | "-" | "INMINUS" => PinRole::InvertingInput,
        "OUT" | "VOUT" | "VO" | "OUTPUT" | "O" | "OUT+" | "OUT-" | "OUTP" | "OUTN" | "Q" | "QB"
        | "QN" | "Y" | "Z" => PinRole::Output,
        "EN" | "ENABLE" | "SHDN" | "SD" | "SHUTDOWN" | "IN" | "VIN" | "REF" | "VREF" => {
            PinRole::Input
        }
        _ => return role_from_phrase(&compact),
    };
    Some(role)
}

/// The role a vendor's spelled-out description states: `noninverting
/// input`, `NEGATIVE POWER SUPPLY`, `output`.
fn role_from_phrase(compact: &str) -> Option<PinRole> {
    let words = compact
        .chars()
        .filter(char::is_ascii_alphabetic)
        .collect::<String>();
    let supply = ["SUPPLY", "POWER", "RAIL"]
        .iter()
        .any(|word| words.contains(word));
    let role = if words.contains("NONINVERTING") {
        PinRole::NonInvertingInput
    } else if words.contains("INVERTING") {
        PinRole::InvertingInput
    } else if supply && words.contains("POSITIVE") {
        PinRole::PositiveSupply
    } else if supply && words.contains("NEGATIVE") {
        PinRole::NegativeSupply
    } else if words.contains("GROUND") {
        PinRole::Ground
    } else if words.contains("OUTPUT") {
        PinRole::Output
    } else if words.contains("INPUT") || words.contains("ENABLE") || words.contains("SHUTDOWN") {
        PinRole::Input
    } else {
        return None;
    };
    Some(role)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pins(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| (*name).to_owned()).collect()
    }

    const OPAMP: [PinRole; 5] = [
        PinRole::NonInvertingInput,
        PinRole::InvertingInput,
        PinRole::PositiveSupply,
        PinRole::NegativeSupply,
        PinRole::Output,
    ];

    #[test]
    fn names_state_the_roles_of_a_named_interface() {
        assert_eq!(
            infer_pin_roles(&pins(&["IN+", "IN-", "VCC", "VEE", "OUT"]), None),
            OPAMP
        );
        assert_eq!(
            infer_pin_roles(&pins(&["inp", "inn", "v+", "gnd", "vout"]), None)[3],
            PinRole::Ground
        );
    }

    #[test]
    fn a_staircase_pin_order_block_names_numbered_pins() {
        // The comment paragraph as the library parser flattens it.
        let description = "Copyright header. Node Assignments noninverting input | inverting \
                           input | | positive supply | | | negative supply | | | | output | | | \
                           | |";
        assert_eq!(
            infer_pin_roles(&pins(&["1", "2", "99", "50", "45"]), Some(description)),
            OPAMP
        );
        let ti = "CONNECTIONS: NON-INVERTING INPUT | INVERTING INPUT | | POSITIVE POWER SUPPLY \
                  | | | NEGATIVE POWER SUPPLY | | | | OUTPUT | | | | |";
        assert_eq!(
            infer_pin_roles(&pins(&["3", "2", "7", "4", "6"]), Some(ti)),
            OPAMP
        );
    }

    #[test]
    fn one_line_pin_order_lists_are_read_with_or_without_numbers() {
        assert_eq!(
            infer_pin_roles(
                &pins(&["1", "2", "3", "4", "5"]),
                Some("Generic op amp. pin order: IN+ IN- V+ V- OUT")
            ),
            OPAMP
        );
        assert_eq!(
            infer_pin_roles(
                &pins(&["1", "2", "3", "4", "5"]),
                Some("Pins: 1 = +IN, 2 = -IN, 3 = VCC, 4 = VEE, 5 = OUT")
            ),
            OPAMP
        );
    }

    #[test]
    fn a_list_that_does_not_cover_every_pin_states_nothing() {
        let roles = infer_pin_roles(
            &pins(&["1", "2", "3", "4", "5"]),
            Some("pin order: IN+ IN- OUT"),
        );
        assert!(roles.iter().all(|role| *role == PinRole::Bidirectional));
    }

    #[test]
    fn a_name_wins_over_the_comment() {
        let roles = infer_pin_roles(
            &pins(&["OUT", "2", "3"]),
            Some("pin order: input, inverting input, noninverting input"),
        );
        assert_eq!(
            roles,
            [
                PinRole::Output,
                PinRole::InvertingInput,
                PinRole::NonInvertingInput
            ]
        );
    }
}

//! The symbol view a cell keeps generated from its interface.
//!
//! A generated symbol is derived, not drawn. It carries the `generated`
//! marker and, under `ports`, the interface it was derived from as
//! `name:direction` entries joined by spaces. The first hand edit clears the
//! marker, and the symbol is its author's from then on.
//!
//! A bus pin carries a colon of its own, `D[7:0]:inout`, so an entry's
//! direction is whatever follows its last colon outside brackets. Every writer
//! and every reader of `ports` goes through this one codec; two spellings of
//! it are how `D[7:0]` came to be read back as `D[7`.

use rspice_design_model::port::{PortDirection, PortSpec};
use rspice_design_model::symbol_pin::SymbolElectricalType;

use super::SymbolDocument;
use crate::library::View;

/// The one symbol view a cell keeps generated.
///
/// Every reader of the marker resolves the view under this exact name, so a
/// symbol view created under any other name is its author's from the start.
pub const GENERATED_SYMBOL_VIEW: &str = "symbol";

/// Metadata key whose presence marks a symbol view as generated.
pub const GENERATED_MARKER_KEY: &str = "generated";

/// Metadata key holding the interface a generated symbol was derived from.
pub const GENERATED_PORTS_KEY: &str = "ports";

/// Whether `view` is a generated symbol rather than an authored one.
pub fn is_generated_symbol(view: &View) -> bool {
    view.metadata.contains_key(GENERATED_MARKER_KEY)
}

/// `ports` as the `ports` metadata value.
pub fn encode_ports(ports: &[PortSpec]) -> String {
    ports
        .iter()
        .map(|port| format!("{}:{}", port.name, port.direction.keyword()))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The interface a `ports` metadata value states, in order.
pub fn decode_ports(encoded: &str) -> Vec<PortSpec> {
    encoded.split_whitespace().filter_map(decode_port).collect()
}

/// One `name:direction` entry.
///
/// The direction is the text after the last colon outside `[...]` or `<...>`,
/// so a range's own colon stays in the name. An entry with no such colon is
/// a name with the default direction, `inout`.
pub fn decode_port(entry: &str) -> Option<PortSpec> {
    let entry = entry.trim();
    let (name, direction) = match last_top_level_colon(entry) {
        Some(index) => (&entry[..index], PortDirection::parse(&entry[index + 1..])),
        None => (entry, PortDirection::InOut),
    };
    let name = name.trim();
    (!name.is_empty()).then(|| PortSpec {
        name: name.to_owned(),
        direction,
    })
}

/// Why a typed port entry was refused by [`parse_port_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortEntryError {
    /// The entry has nothing before its direction.
    MissingName,
    /// The text after the entry's last top-level colon names no direction.
    UnknownDirection(String),
}

/// One `name:direction` entry a person typed, split by the rule
/// [`decode_port`] reads stored metadata with.
///
/// Stored metadata is read generously: a direction this codec does not know
/// reads as `inout`, because the value was written by something. A person's
/// entry is not: `inp:inn` is a typo, not an inout port named `inp`. So the
/// direction must be one this codec writes, or `input` or `output`, and an
/// entry with no direction is the default `inout` as before.
pub fn parse_port_entry(entry: &str) -> Result<PortSpec, PortEntryError> {
    let entry = entry.trim();
    let (name, direction) = match last_top_level_colon(entry) {
        Some(index) => {
            let keyword = entry[index + 1..].trim();
            let direction = match keyword.to_ascii_lowercase().as_str() {
                "in" | "input" => PortDirection::In,
                "out" | "output" => PortDirection::Out,
                "inout" => PortDirection::InOut,
                "supply" => PortDirection::Supply,
                _ => return Err(PortEntryError::UnknownDirection(keyword.to_owned())),
            };
            (&entry[..index], direction)
        }
        None => (entry, PortDirection::InOut),
    };
    let name = name.trim();
    if name.is_empty() {
        return Err(PortEntryError::MissingName);
    }
    Ok(PortSpec {
        name: name.to_owned(),
        direction,
    })
}

fn last_top_level_colon(entry: &str) -> Option<usize> {
    let mut depth = 0_usize;
    let mut last = None;
    for (index, character) in entry.char_indices() {
        match character {
            '[' | '<' => depth += 1,
            ']' | '>' => depth = depth.saturating_sub(1),
            ':' if depth == 0 => last = Some(index),
            _ => {}
        }
    }
    last
}

/// The symbol generated for `ports`, each pin typed by `electrical` where it
/// names a type and by its direction where it does not.
pub fn generated_symbol_document(
    ports: &[PortSpec],
    electrical: impl Fn(&PortSpec) -> Option<SymbolElectricalType>,
) -> SymbolDocument {
    let mut document = SymbolDocument::generated_from_ports(ports);
    for port in ports {
        if let Some(electrical) = electrical(port)
            && let Some(pin) = document.pin_mut(&port.name)
        {
            pin.electrical_type = Some(electrical);
        }
    }
    document
}

/// Make `view` the generated symbol for `ports`: its document, its marker and
/// the interface it was derived from, in one write.
pub fn store_generated_symbol(
    view: &mut View,
    ports: &[PortSpec],
    electrical: impl Fn(&PortSpec) -> Option<SymbolElectricalType>,
) -> Result<(), String> {
    generated_symbol_document(ports, electrical).store_in_view(view)?;
    view.metadata
        .insert(GENERATED_MARKER_KEY.to_owned(), "ports".to_owned());
    view.metadata
        .insert(GENERATED_PORTS_KEY.to_owned(), encode_ports(ports));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::ViewType;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    /// The first-colon split read `D[7:0]:inout` back as a port named `D[7`.
    #[test]
    fn a_bus_pin_round_trips_through_the_ports_metadata() {
        let ports = [
            port("D[7:0]", PortDirection::InOut),
            port("A<3:0>", PortDirection::In),
            port("en", PortDirection::In),
            port("q", PortDirection::Out),
        ];

        let encoded = encode_ports(&ports);

        assert_eq!(encoded, "D[7:0]:inout A<3:0>:in en:in q:out");
        assert_eq!(decode_ports(&encoded), ports);
    }

    #[test]
    fn an_entry_without_a_direction_is_an_inout_name() {
        assert_eq!(
            decode_ports("D[7:0] vdd:supply"),
            [
                port("D[7:0]", PortDirection::InOut),
                port("vdd", PortDirection::Supply),
            ]
        );
        assert_eq!(decode_port(":in"), None);
    }

    /// Stored metadata reads an unknown direction as inout; a typed entry
    /// refuses it, while a bus range's own colon still stays in the name.
    #[test]
    fn a_typed_entry_refuses_a_direction_the_codec_does_not_write() {
        assert_eq!(
            parse_port_entry("d[7:0]:inout"),
            Ok(port("d[7:0]", PortDirection::InOut))
        );
        assert_eq!(
            parse_port_entry("inp:input"),
            Ok(port("inp", PortDirection::In))
        );
        assert_eq!(
            parse_port_entry("vss"),
            Ok(port("vss", PortDirection::InOut))
        );
        assert_eq!(
            parse_port_entry("inp:inn"),
            Err(PortEntryError::UnknownDirection("inn".to_owned()))
        );
        assert_eq!(
            decode_port("inp:inn"),
            Some(port("inp", PortDirection::InOut))
        );
        assert_eq!(parse_port_entry(":out"), Err(PortEntryError::MissingName));
    }

    #[test]
    fn a_stored_generated_symbol_carries_its_marker_ports_and_pin_types() {
        let ports = [
            port("d[3:0]", PortDirection::InOut),
            port("mode", PortDirection::In),
        ];
        let mut view = View::new(GENERATED_SYMBOL_VIEW, ViewType::Symbol);

        store_generated_symbol(&mut view, &ports, |port| {
            (port.name == "mode").then_some(SymbolElectricalType::Logic)
        })
        .expect("the symbol stores");

        assert!(is_generated_symbol(&view));
        assert_eq!(
            view.metadata.get(GENERATED_PORTS_KEY).map(String::as_str),
            Some("d[3:0]:inout mode:in")
        );
        let document = SymbolDocument::load_from_view(&view).expect("the symbol loads");
        let electrical = |name: &str| document.pin(name).and_then(|pin| pin.electrical_type);
        assert_eq!(document.pins.len(), 2);
        assert_eq!(electrical("d[3:0]"), Some(SymbolElectricalType::Analog));
        assert_eq!(electrical("mode"), Some(SymbolElectricalType::Logic));
    }
}

//! The one deck spelling of a vector-bus bit.
//!
//! A schematic authors and displays bus members Virtuoso-style: `A<3:0>` for
//! the declaration, `A<3>` for one bit. That spelling cannot be what the deck
//! carries. `<` and `>` are not identifier characters in the engine's lexer,
//! so they arrive as standalone tokens, and the probe parser keeps only the
//! identifier and number pieces of a `v(...)` operand: `.PRINT TRAN V(A<3>)`
//! reaches the engine as `v(a3)`, silently naming an unrelated node instead of
//! failing. `[` and `]` collapse the same way, and an XSPICE port written
//! `A<3>` is rejected outright.
//!
//! The deck therefore spells one bit `A#3`. `#` is an identifier character, so
//! a bit name is a single token that no path splits, folds, or drops — element
//! cards, `.SUBCKT` ports, subcircuit instances, XSPICE ports and
//! `.PRINT`/`.SAVE` probes all carry it intact. `#` is also outside every
//! charset a name can be authored
//! in here: a bus base name admits only alphanumerics, `_`, `.` and `$`, and
//! the strict net-naming policy admits only `_.$:/![]<>-` beyond
//! alphanumerics. One bus's bits can therefore never alias another's, nor a
//! net a user named.
//!
//! Bit names belong to the deck alone. Every one is produced here, and a result
//! vector is rendered back to the authored `<n>` form through
//! [`display_bit_name`] — the waveform-naming boundary is wired later in the
//! vector-nets wave, so until then a `#` name reaching a viewer is a gap in that
//! wiring, not a second spelling. The engine upper-cases identifiers, so the
//! vector arrives as `DATA#3`; the round trip rewrites the delimiters and leaves
//! the base exactly as the engine spelled it.

use crate::state::BusDeclaration;

/// Deck spelling of bit `index` of the vector net named `base`.
pub(crate) fn deck_bit_name(base: &str, index: u32) -> String {
    format!("{base}#{index}")
}

/// Split a deck bit name into the base net name and the bit index.
///
/// A name is a bit name only in the exact shape [`deck_bit_name`] emits: one
/// `#`, a non-empty base, and a canonical decimal index. Any other name yields
/// `None` rather than being reinterpreted as a bit of something.
pub(crate) fn split_deck_bit(deck: &str) -> Option<(&str, u32)> {
    let (base, digits) = deck.split_once('#')?;
    if base.is_empty() || digits.is_empty() {
        return None;
    }
    if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if digits.len() > 1 && digits.starts_with('0') {
        return None;
    }
    Some((base, digits.parse().ok()?))
}

/// Authored spelling of a deck bit name, or `None` for any other name.
pub(crate) fn display_bit_name(deck: &str) -> Option<String> {
    let (base, index) = split_deck_bit(deck)?;
    Some(format!("{base}<{index}>"))
}

/// Deck spellings of every member of `declaration`, in declaration order.
///
/// The declaration owns the order — the order [`BusDeclaration::members`]
/// expands, which runs from the declared MSB toward the declared LSB. A
/// descending `DATA<3:0>` therefore yields bits 3, 2, 1, 0 and an ascending
/// `ADDR<0:3>` yields 0, 1, 2, 3; both are the order a port list must follow.
/// The base is passed separately because the deck's net name is resolved by
/// the generator, while the declaration only supplies the range.
pub(crate) fn deck_bit_names(base: &str, declaration: &BusDeclaration) -> Vec<String> {
    declaration
        .members()
        .into_iter()
        .map(|member| deck_bit_name(base, member.index))
        .collect()
}

#[cfg(test)]
mod tests {
    use rspice_core::netlist::{ElementKind, Netlist, SaveSignal, XspicePort};

    use super::*;

    /// Every deck position a bit name can occupy, in one deck: element nodes,
    /// a `.SUBCKT` port list, a subcircuit instance, an XSPICE port, and both
    /// probe directives. `A`, `A3` and `A#3` are all present so the bit
    /// namespace is proven distinct from the names it must never collide with.
    const BIT_NAME_DECK: &str = "vector bit names\n\
         .SUBCKT CHILD D#3 D#2 D#1 D#0\n\
         RC D#3 D#0 1k\n\
         .ENDS\n\
         R1 A#3 0 1k\n\
         r2 data#3 0 1k\n\
         R3 A 0 1k\n\
         R4 A3 0 1k\n\
         X1 D#3 D#2 D#1 D#0 CHILD\n\
         A1 A#3 out gain\n\
         .model gain gain(gain=2)\n\
         .PRINT TRAN V(A#3)\n\
         .SAVE V(A#3)\n\
         .TRAN 1n 10n\n\
         .END\n";

    fn nodes_of(netlist: &Netlist, element: &str) -> Vec<String> {
        netlist
            .elements
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(element))
            .unwrap_or_else(|| panic!("{element} is in the parsed deck"))
            .nodes
            .clone()
    }

    #[test]
    fn deck_bit_names_survive_the_engine() {
        let netlist = Netlist::parse(BIT_NAME_DECK).expect("a deck of bit names parses");

        // An element card carries the bit name whole, and case folding leaves
        // the `#` and the index alone: the engine's canonical spelling of one
        // bit is exactly `deck_bit_name` applied to the upper-cased base.
        assert_eq!(nodes_of(&netlist, "R1"), ["A#3", "0"]);
        assert_eq!(
            nodes_of(&netlist, "R2"),
            [deck_bit_name("DATA", 3), "0".to_owned()]
        );

        // A bit is a node of its own, distinct from the base and from the
        // index-concatenated name the authored `<3>` spelling degrades into.
        assert_eq!(nodes_of(&netlist, "R3"), ["A", "0"]);
        assert_eq!(nodes_of(&netlist, "R4"), ["A3", "0"]);

        // The port list of a subcircuit and its instance agree bit for bit.
        let child = netlist
            .subcircuits
            .iter()
            .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("CHILD"))
            .expect("CHILD is defined");
        let declaration = BusDeclaration::parse("D<3:0>").expect("D<3:0> is a bus");
        assert_eq!(child.ports, deck_bit_names("D", &declaration));
        assert_eq!(nodes_of(&netlist, "X1"), child.ports);

        // An XSPICE port takes a bit name; the same position rejects `<`.
        let ports = nodes_of_xspice(&netlist, "A1");
        assert_eq!(ports.first(), Some(&XspicePort::Analog("A#3".to_owned())));

        // Both probe directives keep the bit name; the engine lower-cases a
        // probe operand, so the round trip renders `A#3` back as authored.
        assert_eq!(
            netlist.saves.signals,
            vec![
                SaveSignal::Voltage("a#3".to_owned()),
                SaveSignal::Voltage("a#3".to_owned()),
            ]
        );
        assert_eq!(display_bit_name("A#3").as_deref(), Some("A<3>"));
    }

    fn nodes_of_xspice(netlist: &Netlist, element: &str) -> Vec<XspicePort> {
        match &netlist
            .elements
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(element))
            .unwrap_or_else(|| panic!("{element} is in the parsed deck"))
            .kind
        {
            ElementKind::Xspice { ports, .. } => ports.clone(),
            other => panic!("{element} is not an XSPICE instance: {other:?}"),
        }
    }

    /// The evidence behind the `#` decision: the authored delimiters are not
    /// merely unconventional in a deck, they are destroyed by it — and only on
    /// some of the paths a bit name has to cross, which is why reading an
    /// element card alone would suggest they were safe.
    #[test]
    fn authored_delimiters_do_not_survive_the_probe_and_xspice_paths() {
        // An element card does reassemble them, so a bus that is only ever
        // wired looks fine right up to the point where it must be observed.
        for (card, node) in [("R1 A<3> 0 1k", "A<3>"), ("R1 A[3] 0 1k", "A[3]")] {
            let netlist = Netlist::parse(&format!("card\n{card}\n.END\n"))
                .expect("an element card reassembles a delimited node name");
            assert_eq!(nodes_of(&netlist, "R1"), [node, "0"]);
        }

        // A probe operand keeps only its identifier and number pieces, so all
        // three of these name the same node: the authored delimiters are
        // dropped rather than rejected, and `V(A<3>)` silently becomes a probe
        // on `A3` — a node the same deck may legitimately declare.
        for (probe, expected) in [
            ("V(A#3)", "a#3"),
            ("V(A<3>)", "a3"),
            ("V(A[3])", "a3"),
            ("V(A3)", "a3"),
        ] {
            let deck = format!("probe\nR1 A 0 1k\n.PRINT TRAN {probe}\n.TRAN 1n 10n\n.END\n");
            let netlist = Netlist::parse(&deck).expect("the probe deck parses");
            assert_eq!(
                netlist.saves.signals,
                vec![SaveSignal::Voltage(expected.to_owned())],
                "{probe} did not reach the engine as v({expected})"
            );
        }

        // An XSPICE port is the one place that refuses them outright.
        let error =
            Netlist::parse("angle port\nA1 A<3> out gain\n.model gain gain(gain=2)\n.END\n")
                .expect_err("an XSPICE port spelled with angle brackets is rejected");
        assert!(
            error
                .to_string()
                .contains("XSPICE port requires a node name, found '<'"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn deck_and_display_spellings_round_trip() {
        let deck = deck_bit_name("DATA", 3);
        assert_eq!(deck, "DATA#3");
        assert_eq!(split_deck_bit(&deck), Some(("DATA", 3)));
        assert_eq!(display_bit_name(&deck).as_deref(), Some("DATA<3>"));
        assert_eq!(
            display_bit_name(&deck_bit_name("A", 0)).as_deref(),
            Some("A<0>")
        );
    }

    #[test]
    fn only_a_canonical_single_hash_index_reads_as_a_bit_name() {
        for other in [
            "A", "A#", "#3", "A#3#4", "A#x", "A#-1", "A#03", "A<3>", "A[3]",
        ] {
            assert_eq!(split_deck_bit(other), None, "{other} is not a bit name");
            assert_eq!(
                display_bit_name(other),
                None,
                "{other} must not be rewritten"
            );
        }
    }

    #[test]
    fn declaration_order_drives_the_emitted_bit_order() {
        let descending = BusDeclaration::parse("DATA<3:0>").expect("DATA<3:0> is a bus");
        assert_eq!(
            deck_bit_names("DATA", &descending),
            ["DATA#3", "DATA#2", "DATA#1", "DATA#0"]
        );

        let ascending = BusDeclaration::parse("ADDR[0:3]").expect("ADDR[0:3] is a bus");
        assert_eq!(
            deck_bit_names("ADDR", &ascending),
            ["ADDR#0", "ADDR#1", "ADDR#2", "ADDR#3"]
        );
    }
}

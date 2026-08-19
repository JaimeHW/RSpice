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
//! charset a name can be authored in here: a bus base name admits only
//! alphanumerics, `_`, `.` and `$`, and the strict net-naming policy admits
//! only `_.$:/![]<>-` beyond alphanumerics. One bus's bits can therefore never
//! alias another's, nor a net a user named.
//!
//! Bit names belong to the deck alone, and every one is produced here — and
//! every one is read back here too. [`display_bit_name`] is the only inverse:
//! a surface that shows a result vector to a designer renders the deck name
//! through it instead of re-splitting the `#` itself, so the two spellings can
//! never drift apart. The inverse takes the notation the vector was declared
//! in, because a deck bit records the index and not the delimiters: rendering
//! `DATA#3` as `DATA<3>` in a document that declared `DATA[3:0]` would quote a
//! spelling that document does not contain.

use crate::state::BusNotation;

/// Deck spelling of bit `index` of the vector net named `base`.
pub(crate) fn deck_bit_name(base: &str, index: u32) -> String {
    format!("{base}#{index}")
}

/// Authored spelling of a deck bit name, or `None` when the name is not one.
///
/// The deck spelling is unambiguous by construction: `#` cannot occur in an
/// authored name, so the last one separates a base from its index and nothing
/// else can produce that shape. A name with no `#`, an empty base, or a
/// non-numeric index is an ordinary node and stays exactly as it is.
pub(crate) fn display_bit_name(deck_name: &str, notation: BusNotation) -> Option<String> {
    let (base, index) = deck_name.rsplit_once('#')?;
    if base.is_empty() || index.is_empty() || !index.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let (open, close) = notation.delimiters();
    Some(format!("{base}{open}{index}{close}"))
}

#[cfg(test)]
mod tests {
    use rspice_core::netlist::{ElementKind, Netlist, SaveSignal, XspicePort};

    use super::*;
    use crate::state::BusDeclaration;

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

        // The port list of a subcircuit and its instance agree bit for bit, in
        // the order the declaration expands: from the declared MSB toward the
        // declared LSB.
        let child = netlist
            .subcircuits
            .iter()
            .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("CHILD"))
            .expect("CHILD is defined");
        let declaration = BusDeclaration::parse("D<3:0>").expect("D<3:0> is a bus");
        let declared: Vec<String> = declaration
            .members()
            .into_iter()
            .map(|member| deck_bit_name("D", member.index))
            .collect();
        assert_eq!(declared, ["D#3", "D#2", "D#1", "D#0"]);
        assert_eq!(child.ports, declared);
        assert_eq!(nodes_of(&netlist, "X1"), child.ports);

        // An XSPICE port takes a bit name; the same position rejects `<`.
        let ports = nodes_of_xspice(&netlist, "A1");
        assert_eq!(ports.first(), Some(&XspicePort::Analog("A#3".to_owned())));

        // Both probe directives keep the bit name whole; the engine lower-cases
        // a probe operand and drops nothing else.
        assert_eq!(
            netlist.saves.signals,
            vec![
                SaveSignal::Voltage("a#3".to_owned()),
                SaveSignal::Voltage("a#3".to_owned()),
            ]
        );
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

    #[test]
    fn every_deck_bit_name_renders_back_to_the_name_it_was_authored_from() {
        for (declaration, notation) in [
            ("DATA<15:0>", BusNotation::Angle),
            ("DATA[15:0]", BusNotation::Square),
        ] {
            let declaration = BusDeclaration::parse(declaration).expect("a bus declaration");
            for member in declaration.members() {
                let deck = deck_bit_name(&declaration.name, member.index);
                assert_eq!(
                    display_bit_name(&deck, notation),
                    Some(member.to_string()),
                    "{deck}"
                );
            }
        }

        // The inverse is exact over the whole index space and every base a bus
        // name admits — digits, `_`, `.` and `$` included. Only the delimiters
        // come from the declaration; the base and the index come back
        // untouched.
        for base in ["A", "DATA", "afe.bias$sense", "b2b_9", "_x1"] {
            for index in [0, 1, 9, 10, 4_294_967_295] {
                for (notation, expected) in [
                    (BusNotation::Square, format!("{base}[{index}]")),
                    (BusNotation::Angle, format!("{base}<{index}>")),
                ] {
                    let deck = deck_bit_name(base, index);
                    assert_eq!(display_bit_name(&deck, notation), Some(expected), "{deck}");
                }
            }
        }

        // Anything that is not a bit name is left alone rather than reshaped.
        for ordinary in ["out", "0", "net12", "A#", "#3", "A#x", "A#3x"] {
            assert_eq!(
                display_bit_name(ordinary, BusNotation::Square),
                None,
                "{ordinary}"
            );
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
}

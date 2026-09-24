//! One spelling of a declared range's deck bits, for the dialogs that show one
//! before it exists.
//!
//! A range a designer types is one pin or one tap; what reaches the engine is a
//! list of scalar bits. Both placement dialogs preview that list while it is
//! still a draft, and both render it here, so a preview can never quote a bit
//! the deck will not carry: the names come from
//! [`crate::simulation::netlist_gen::deck_bit_name`] — the one owner of the
//! deck's `#` spelling — and their order comes from the declaration that
//! expanded them.

use crate::simulation::netlist_gen::deck_bit_name;

/// Bits a preview lists in full.
///
/// A wider range keeps both ends and elides the middle: the ends are what
/// identify a range, and a thousand names in a row identify nothing.
const LISTED_IN_FULL: usize = 8;
const KEPT_AT_HEAD: usize = 4;
const KEPT_AT_TAIL: usize = 3;

/// The deck bits `indices` of the vector named `base` resolve to, as one line.
pub(super) fn deck_bits(base: &str, indices: impl IntoIterator<Item = u32>) -> String {
    let names: Vec<String> = indices
        .into_iter()
        .map(|index| deck_bit_name(base, index))
        .collect();
    if names.len() <= LISTED_IN_FULL {
        return names.join(" ");
    }
    format!(
        "{} \u{2026} {}",
        names[..KEPT_AT_HEAD].join(" "),
        names[names.len() - KEPT_AT_TAIL..].join(" ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BusDeclaration;

    fn bits(declaration: &str) -> String {
        let declaration = BusDeclaration::parse(declaration).expect("a bus declaration");
        deck_bits(
            &declaration.name,
            declaration.members().into_iter().map(|member| member.index),
        )
    }

    #[test]
    fn a_preview_lists_every_bit_it_can_and_elides_only_the_middle() {
        assert_eq!(bits("EN[1:0]"), "EN#1 EN#0");
        assert_eq!(
            bits("DATA[7:0]"),
            "DATA#7 DATA#6 DATA#5 DATA#4 DATA#3 DATA#2 DATA#1 DATA#0"
        );

        // Past the line's budget both ends survive, in declaration order, and
        // only the middle is replaced.
        let wide = bits("DATA[15:0]");
        assert_eq!(
            wide,
            "DATA#15 DATA#14 DATA#13 DATA#12 \u{2026} DATA#2 DATA#1 DATA#0"
        );
        assert!(!wide.contains("DATA#8"));

        // The declared direction is the preview's order, not a sorted one.
        assert_eq!(bits("ADDR<0:3>"), "ADDR#0 ADDR#1 ADDR#2 ADDR#3");
    }

    /// The bits are the deck's, not a second spelling that could drift: every
    /// name the preview shows is the one `deck_bit_name` produces.
    #[test]
    fn every_previewed_bit_is_the_name_the_deck_carries() {
        let declaration = BusDeclaration::parse("DATA[3:0]").expect("a bus declaration");
        let previewed = bits("DATA[3:0]");
        for member in declaration.members() {
            assert!(
                previewed.contains(&deck_bit_name(&declaration.name, member.index)),
                "{previewed}"
            );
        }
    }
}

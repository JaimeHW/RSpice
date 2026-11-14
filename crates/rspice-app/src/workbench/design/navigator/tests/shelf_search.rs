//! The Component shelf's ranked search: what a reader typing a query gets
//! first, and what they never get at all.

use super::super::shelf::search::{MatchTier, ShelfQuery, ShelfTerms};

#[test]
fn exact_beats_a_word_prefix_beats_a_part_number() {
    let tier = |query: &str, terms: ShelfTerms<'_>| {
        ShelfQuery::new(query).rank(&terms).map(|rank| rank.tier)
    };
    let row = ShelfTerms {
        label: "Zener Diode",
        keywords: &["avalanche"],
        part_numbers: &["RSPICE_ZENER"],
    };
    assert_eq!(tier("zener diode", row), Some(MatchTier::Exact));
    let row = ShelfTerms {
        label: "Zener Diode",
        keywords: &["avalanche"],
        part_numbers: &["RSPICE_ZENER"],
    };
    assert_eq!(tier("diode", row), Some(MatchTier::WordPrefix));
    let part = || ShelfTerms {
        label: "",
        keywords: &[],
        part_numbers: &["D1N4148"],
    };
    assert_eq!(tier("d1n", part()), Some(MatchTier::PartNumberPrefix));
    assert_eq!(tier("4148", part()), Some(MatchTier::PartNumberSubstring));
    // Separators fold away on both sides.
    assert_eq!(tier("d-1n_4148", part()), Some(MatchTier::PartNumberPrefix));
}

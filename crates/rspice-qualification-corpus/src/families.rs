//! The corpus case families. Each module authors drafts for one analysis
//! domain; every draft carries its own engine-independent oracle.

pub mod dc;
pub mod dynamics;
pub mod frequency;
pub mod limits;
pub mod op;
pub mod physics;

use crate::capture::CaseDraft;

/// Every draft in the corpus, unordered; the assembler sorts by identifier.
pub fn all() -> Vec<CaseDraft> {
    let mut drafts = op::drafts();
    drafts.extend(dc::drafts());
    drafts.extend(dynamics::drafts());
    drafts.extend(frequency::drafts());
    drafts.extend(limits::drafts());
    drafts
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::capture::realize;

    /// Every category floor the authored families claim to meet, checked
    /// structurally: a family module that silently drops out of `all()`
    /// fails here even though every remaining draft still realizes.
    #[test]
    fn authored_families_meet_their_category_floors() {
        let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
        for draft in all() {
            *counts.entry(draft.primary_category).or_default() += 1;
            for category in &draft.extra_categories {
                *counts.entry(category).or_default() += 1;
            }
        }
        for category in crate::contract::REQUIRED_CATEGORIES {
            let count = counts.get(category).copied().unwrap_or(0);
            assert!(count >= 10, "category {category} covers {count} < 10 cases");
        }
    }

    /// The corpus red/green loop: every authored draft must realize against
    /// the live engine, meaning its independent oracle agrees with the
    /// engine inside its own tolerance and its identifiers are coherent.
    /// A physics mistake in any family fails here, naming the case.
    #[test]
    fn every_family_draft_realizes_against_the_live_engine() {
        let drafts = all();
        assert!(!drafts.is_empty());
        let mut ids = BTreeSet::new();
        let mut spec_digests = BTreeSet::new();
        for draft in drafts {
            let id = draft.id.clone();
            let case = realize(draft).unwrap_or_else(|error| panic!("{error}"));
            assert!(ids.insert(id.clone()), "duplicate case id {id}");
            assert!(
                spec_digests.insert(
                    case["case_spec_sha256"]
                        .as_str()
                        .expect("digest")
                        .to_owned()
                ),
                "duplicate case specification for {id}"
            );
        }
    }
}

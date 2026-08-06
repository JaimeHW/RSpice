//! The corpus case families. Each module authors drafts for one analysis
//! domain; every draft carries its own engine-independent oracle.

pub mod dc;
pub mod op;
pub mod physics;

use crate::capture::CaseDraft;

/// Every draft in the corpus, unordered; the assembler sorts by identifier.
pub fn all() -> Vec<CaseDraft> {
    let mut drafts = op::drafts();
    drafts.extend(dc::drafts());
    drafts
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::capture::realize;

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

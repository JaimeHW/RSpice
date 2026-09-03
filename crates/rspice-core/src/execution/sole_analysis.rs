//! Canonical identity for a run that authored exactly one analysis card.
//!
//! Analysis identities belong to the canonical [`DeckPlan`], and a frontend
//! that ran a deck reads them from it. A frontend convenience call that solves
//! one analysis directly authors no deck, and yet its result still has to name
//! an identity in the shared result document. That identity is not a guess:
//! a run with a single card of a family *is* that family's first ordinal, so
//! this is the plan's own answer for the one-card case rather than a second
//! numbering scheme.
//!
//! It is deliberately the only way to mint an identity outside the planner:
//! anything with more than one card of a family must ask the plan, because
//! only the plan knows the authored order.
//!
//! [`DeckPlan`]: super::DeckPlan

use super::plan::{AnalysisInstanceId, AnalysisKind};

/// The identity of the only `kind` card of a run that authored exactly one.
pub const fn sole_analysis_identity(kind: AnalysisKind) -> AnalysisInstanceId {
    analysis_instance_identity(kind, 0)
}

/// The identity of the `ordinal`-th (zero-based) authored `kind` card.
///
/// This is for a frontend that already holds a canonical ordinal — one it read
/// off the plan, or one it assigned in the plan's own authored order — and
/// needs the identity back as a value. It does not decide the ordering: a
/// frontend that has a [`DeckPlan`] reads identities from it directly.
///
/// [`DeckPlan`]: super::DeckPlan
pub const fn analysis_instance_identity(kind: AnalysisKind, ordinal: u32) -> AnalysisInstanceId {
    AnalysisInstanceId::new(kind, ordinal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::Netlist;
    use crate::resource::ResourceLimits;

    #[test]
    fn the_sole_identity_is_the_one_the_planner_assigns_a_single_card_deck() {
        // The claim this function makes is that it agrees with the planner,
        // so it is checked against the planner rather than against a literal.
        for (source, kind) in [
            (
                "single ac card\nV1 in 0 AC 1\nR1 in 0 1k\n.ac dec 2 1 10\n.end\n",
                AnalysisKind::Ac,
            ),
            (
                "single tran card\nV1 in 0 DC 1\nR1 in 0 1k\n.tran 1u 10u\n.end\n",
                AnalysisKind::Tran,
            ),
            (
                "single op card\nV1 in 0 DC 1\nR1 in 0 1k\n.op\n.end\n",
                AnalysisKind::Op,
            ),
        ] {
            let netlist = Netlist::parse_validated(source).expect("the fixture parses");
            let plan = super::super::DeckPlan::from_netlist(&netlist, &ResourceLimits::default())
                .expect("the fixture plans");
            let planned = plan
                .analyses()
                .iter()
                .find(|analysis| analysis.id().kind() == kind)
                .expect("the fixture plans its one card")
                .id();
            assert_eq!(planned, sole_analysis_identity(kind));
            assert_eq!(planned.tag(), format!("{}-001", kind.tag()));
        }
    }
}

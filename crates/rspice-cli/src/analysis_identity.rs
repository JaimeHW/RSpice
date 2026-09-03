//! Canonical analysis identity for the post-process families the authored
//! plan does not name.
//!
//! Every physical analysis the CLI publishes takes its artifact namespace from
//! the `AnalysisInstanceId` the canonical planner minted for it. Two families
//! have no planned slot: `.FOUR` and `.FFT` are attached to the transient they
//! post-process rather than being independent cards, so `DeckPlan` deliberately
//! pairs them with `None`.
//!
//! `AnalysisInstanceId` is also deliberately not constructible outside
//! `rspice-core`. This module therefore mints those two families' identities
//! from a one-family `DeckPlan` — the same canonical minting the planner uses
//! for everything else — so no artifact writer formats `fft-NNN` or `four-NNN`
//! by hand and two spellings of one identity cannot drift apart.

use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisRequest, DeckPlan};

/// Mint `count` canonical identities of one post-process family, in the deck's
/// authored order.
///
/// The identities are the planner's own: the `n`th entry is what `DeckPlan`
/// would assign the `n`th authored card of that family.
pub(crate) fn post_process_ids(
    kind: AnalysisKind,
    count: usize,
) -> Result<Vec<AnalysisInstanceId>, rspice_core::execution::DeckPlanError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let requests = std::iter::repeat_with(|| AnalysisRequest::new(kind))
        .take(count)
        .collect::<Vec<_>>();
    let plan = DeckPlan::new(Vec::new(), requests)?;
    Ok(plan
        .analyses()
        .iter()
        .map(rspice_core::execution::PlannedAnalysis::id)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_process_identities_are_one_based_and_dense() {
        let ids = post_process_ids(AnalysisKind::Fft, 3).expect("three FFT identities");
        assert_eq!(
            ids.iter().map(|id| id.tag()).collect::<Vec<_>>(),
            ["fft-001", "fft-002", "fft-003"]
        );
        assert!(ids.iter().all(|id| id.kind() == AnalysisKind::Fft));
    }

    #[test]
    fn fourier_identities_use_the_planner_tag() {
        let ids = post_process_ids(AnalysisKind::Fourier, 7).expect("seven Fourier identities");
        assert_eq!(ids.first().map(|id| id.tag()).as_deref(), Some("four-001"));
        assert_eq!(ids.last().map(|id| id.tag()).as_deref(), Some("four-007"));
    }

    #[test]
    fn an_empty_request_mints_nothing() {
        assert!(
            post_process_ids(AnalysisKind::Fourier, 0)
                .expect("empty request succeeds")
                .is_empty()
        );
    }

    #[test]
    fn a_longer_request_extends_the_shorter_one_without_renumbering() {
        let short = post_process_ids(AnalysisKind::Fft, 2).expect("two FFT identities");
        let long = post_process_ids(AnalysisKind::Fft, 5).expect("five FFT identities");
        assert_eq!(long.len(), 5);
        assert_eq!(&long[..short.len()], short.as_slice());
    }
}

//! Destructive review before a plan removal that orphans something.
//!
//! Four registries share one review, and each has to answer the same three
//! questions: a removal with dependents stops and leaves the record standing
//! when the reader cancels, a confirmed removal commits with the same receipt
//! the immediate route writes, and a removal that orphans nothing never opens a
//! modal at all. These press the actual Remove control rather than calling the
//! handler, because "which route commits a plan mutation" is exactly what was
//! wrong: three of the four committed on the first click.
//!
//! The analysis registry answers a fourth question the other three do not. Its
//! dependents are the one case the plan's own transaction *refuses*, so a
//! review staged on them could only ever be confirmed into a refusal notice —
//! that arm states the refusal instead, and only the retained-results arm is
//! still a question.

use crate::workbench::simulate::test_host::SimulateTestHost;

// ---------------------------------------------------------------- variables

// ------------------------------------------------------------------ outputs

// ----------------------------------------------------------- capture groups

// ------------------------------------------------------- analysis regression

/// The hop is called what the other hop to the same place is called.
#[test]
fn the_refusal_and_the_options_panel_name_the_same_hop_the_same_way() {
    assert_eq!(
        crate::workbench::app::REVEAL_BLOCKER,
        super::super::advanced_options::REVEAL_ACTION,
        "two surfaces sending the reader to the Analyses page must say so in one wording"
    );
}

/// Retained results, and nothing bound to it: still reviewed, still committed.
///
/// This is the arm where the confirmation is a real question. The removal will
/// succeed, and what it costs is the attribution of datasets that stay in the
/// project either way — so the reader decides, and the answer is applied on the
/// surface's next frame rather than by the modal.
#[test]
fn a_removal_costing_only_retained_runs_still_stages_and_commits() {
    use super::super::lifecycle::{apply_confirmed_analysis_removal, remove_analysis_instance};
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::simulation::plan::AnalysisKind;
    use crate::state::{AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun};

    let mut harness = SimulateTestHost::harness();
    let ac = harness
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .insert(AnalysisKind::Ac)
        .expect("AC inserts")
        .0;
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_provenance(
            AnalysisResultProvenance::new(
                ac,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5a; 32]),
                Vec::new(),
            )
            .expect("test provenance is valid"),
        ),
    );
    harness.model.simulation.retained.runs = vec![run].into();

    remove_analysis_instance(&mut harness.cx(), ac);
    assert_eq!(
        harness.state.plan_removal_review.target,
        Some(crate::workbench::app::PlanRemovalTarget::Analysis(ac)),
        "a removal that orphans retained results must still be reviewed"
    );
    assert!(
        harness.state.plan_removal_refusal.analysis.is_none(),
        "a removal the plan will perform is a question, not a refusal"
    );
    let stated: Vec<(String, String)> = harness
        .state
        .plan_removal_review
        .consequences
        .iter()
        .map(|consequence| (consequence.fact.clone(), consequence.value.clone()))
        .collect();
    assert_eq!(
        stated,
        vec![
            (
                "Retained runs holding its results".to_owned(),
                "1".to_owned()
            ),
            ("Analyses bound to it".to_owned(), "none".to_owned()),
        ],
        "the review states what removal costs and what it does not"
    );
    assert!(
        harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(ac)
            .is_some(),
        "staging a review must not remove the analysis"
    );

    harness.state.plan_removal_review.confirmed = true;
    apply_confirmed_analysis_removal(&mut harness.cx());
    assert!(
        harness.state.plan_removal_review.target.is_none(),
        "the review must close once its answer has been applied"
    );
    assert!(
        harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(ac)
            .is_none(),
        "the confirmed removal must reach the plan's own transaction"
    );
    let status = harness.model.analysis_lifecycle_status.message().to_owned();
    assert!(
        status.contains("Remove committed"),
        "the commit must leave the receipt the immediate route writes, got: {status}"
    );
    apply_confirmed_analysis_removal(&mut harness.cx());
    assert_eq!(
        harness.model.analysis_lifecycle_status.message().to_owned(),
        status,
        "an answered review must not be applied a second time"
    );
}

/// A removal with nothing behind it still goes on the click that asked.
#[test]
fn an_analysis_with_no_dependents_and_no_results_is_removed_at_once() {
    use super::super::lifecycle::remove_analysis_instance;
    use crate::simulation::plan::AnalysisKind;

    let mut harness = SimulateTestHost::harness();
    let ac = harness
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .insert(AnalysisKind::Ac)
        .expect("AC inserts")
        .0;

    remove_analysis_instance(&mut harness.cx(), ac);
    assert!(
        harness.state.plan_removal_review.target.is_none()
            && harness.state.plan_removal_refusal.analysis.is_none(),
        "an analysis nothing depends on must not raise a modal"
    );
    assert!(
        harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(ac)
            .is_none(),
        "it must be removed by the action that asked for it"
    );
}

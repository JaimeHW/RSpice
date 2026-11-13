//! Singleton projection and frozen-plan cases.

use super::*;
use crate::analysis_draft::AnalysisDraft;
use crate::analysis_kind::AnalysisKind;
use crate::drafts::{DistoDraft, NoiseDraft};

#[test]
fn compatibility_projection_preserves_noise_and_disto_owned_sweeps() {
    let mut setup = SimulationSetup::new();
    let noise = AnalysisDraft::Noise(NoiseDraft {
        points: "77".to_owned(),
        sweep: crate::config::NoiseSweepType::Linear,
        ..NoiseDraft::default()
    });
    setup.apply_analysis_draft_projection(&noise);
    assert_eq!(setup.ac.points, "77");
    assert_eq!(setup.ac.sweep, 2);
    let captured = setup.legacy_analysis_draft(AnalysisKind::Noise);
    let AnalysisDraft::Noise(captured) = captured else {
        panic!("noise projection must retain its kind");
    };
    assert_eq!(captured.points, "77");

    let mut disto = DistoDraft::default();
    disto.sweep.points = "31".to_owned();
    disto.f2_over_f1 = "0.9".to_owned();
    setup.apply_analysis_draft_projection(&AnalysisDraft::Disto(disto));
    let captured = setup.legacy_analysis_draft(AnalysisKind::Disto);
    let AnalysisDraft::Disto(captured) = captured else {
        panic!("DISTO projection must retain its kind");
    };
    assert_eq!(captured.sweep.points, "31");
    assert_eq!(captured.f2_over_f1, "0.9");
}

#[test]
fn frozen_projection_applies_the_exact_transitive_prerequisite_closure() {
    let mut setup = SimulationSetup::new();
    let plan = setup.analysis_plan.as_mut().expect("stable plan exists");
    let (op, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts");
    plan.edit(op, |draft| {
        let AnalysisDraft::OperatingPoint(draft) = draft else {
            panic!("expected OP draft");
        };
        draft.temperature = "88".to_owned();
    })
    .expect("OP edits");
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(draft) = draft else {
            panic!("expected PSS draft");
        };
        draft.fund_freq = "7Meg".to_owned();
        // A driven solve needs a tone, and only the design can name one.
        draft.tone_sources = "VSRC".to_owned();
    })
    .expect("PSS edits");
    plan.bind_dependency(pss, AnalysisKind::OperatingPoint, op)
        .expect("PSS binds OP");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    plan.bind_dependency(pac, AnalysisKind::Pss, pss)
        .expect("PAC binds PSS");
    let frozen = plan.freeze().expect("plan freezes");
    let frozen_pac = frozen
        .instances()
        .iter()
        .find(|instance| instance.id() == pac)
        .expect("PAC freezes");

    let projection = setup
        .frozen_instance_projection(&frozen, frozen_pac)
        .expect("transitive closure projects");

    assert_eq!(projection.op.temperature, "88");
    assert_eq!(projection.pss.fund_freq, "7Meg");
    assert_eq!(
        projection.analysis_order,
        vec![AnalysisKind::Pac.singleton_index()]
    );
}

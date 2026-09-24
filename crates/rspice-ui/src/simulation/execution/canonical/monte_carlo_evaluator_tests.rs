//! Canonical Monte Carlo tests preserve saved base-analysis and postprocessor identity.

use super::*;
use crate::product::ObjectRevision;
use crate::simulation::runner::study::{
    StudyAnalysis, StudyOperatingPoint, StudyPostprocess, StudyQpssConfig, StudyRunConfig,
};

fn base(analysis: StudyAnalysis) -> StudyRunConfig {
    StudyRunConfig {
        postprocess: None,
        constraints: vec![],
        objective_terms: vec![],
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        analysis,
        analysis_line: ".qpss".into(),
        numeric_options: ".options RELTOL=1e-6".into(),
        measurements: vec!["tuple:1,-1:magnitude:V(out)".into()],
        histogram_bins: 10,
    }
}

#[test]
fn monte_carlo_evaluator_compatibility_preserves_saved_op_and_solver_identity() {
    let mut op = OpConfig::default();
    op.initial_guess = OpInitialGuess::PreviousCompatible;
    op.node_initialization = OpNodeInitialization::IgnoreIcAndNodeset;
    op.previous_state = Some(OpPreviousState {
        source_content_digest: ContentDigest::from_bytes([1; 32]),
        producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
        producer_result_digest: ContentDigest::from_bytes([3; 32]),
        node_names: vec!["out".into()],
        branch_names: vec![],
        solution: vec![0.5],
    });
    op.validate_for_execution().unwrap();
    let spec = crate::simulation::plan::QpssDraft {
        tones: "1k, 1414.213562373095".into(),
        harmonics: "1, 1".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let mut base = base(StudyAnalysis::Qpss(Box::new(StudyQpssConfig {
        request: spec,
        operating_point: StudyOperatingPoint {
            instance_id: AnalysisInstanceId::new(),
            source_revision: ObjectRevision::INITIAL,
            config: op,
            numeric_options: ".options GMIN=0".into(),
        },
    })));
    base.analysis.validate().unwrap();
    let expected = monte_carlo_evaluator_digest(&base);
    let prepared = |base: &StudyRunConfig| {
        analysis_config_digest(
            ".mc 2",
            &AnalysisSpec::MonteCarlo {
                variation_source: crate::simulation::dialog::McVariationSource::ParameterTolerance,
                params: vec![],
            },
            None,
            &SpecExecutionOptions {
                study_base: Some(base.clone()),
                ..Default::default()
            },
            None,
        )
    };
    let original_task = prepared(&base);
    base.source_revision = base.source_revision.next().unwrap();
    let StudyAnalysis::Qpss(config) = &mut base.analysis else {
        unreachable!()
    };
    config.operating_point.source_revision = config.operating_point.source_revision.next().unwrap();
    assert_eq!(monte_carlo_evaluator_digest(&base), expected);
    assert_ne!(
        prepared(&base),
        original_task,
        "dispatch still binds exact authored revisions"
    );

    let changes: &[fn(&mut StudyOperatingPoint)] = &[
        |op| op.instance_id = AnalysisInstanceId::new(),
        |op| op.numeric_options.push_str(" ABSTOL=1e-13"),
        |op| op.config.temperature_celsius += 1.0,
        |op| {
            op.config
                .previous_state
                .as_mut()
                .unwrap()
                .source_content_digest = ContentDigest::from_bytes([4; 32])
        },
        |op| {
            op.config
                .previous_state
                .as_mut()
                .unwrap()
                .producer_snapshot_digest = ContentDigest::from_bytes([4; 32])
        },
        |op| {
            op.config
                .previous_state
                .as_mut()
                .unwrap()
                .producer_result_digest = ContentDigest::from_bytes([4; 32])
        },
        |op| op.config.previous_state.as_mut().unwrap().solution[0] = 0.6,
    ];
    for change in changes {
        let mut changed = base.clone();
        let StudyAnalysis::Qpss(config) = &mut changed.analysis else {
            unreachable!()
        };
        change(&mut config.operating_point);
        assert_ne!(monte_carlo_evaluator_digest(&changed), expected);
    }
    let mut changed = base.clone();
    changed.measurements[0] = "tuple:1,0:magnitude:V(out)".into();
    assert_ne!(monte_carlo_evaluator_digest(&changed), expected);
    let mut changed = base.clone();
    let StudyAnalysis::Qpss(config) = &mut changed.analysis else {
        unreachable!()
    };
    let AnalysisSpec::Qpss { controls, .. } = &mut config.request else {
        unreachable!()
    };
    controls.voltage_absolute_tolerance *= 2.0;
    assert_ne!(monte_carlo_evaluator_digest(&changed), expected);
}

#[test]
fn monte_carlo_evaluator_compatibility_keeps_postprocessor_configuration() {
    let mut base = base(
        AnalysisConfig::Transient(crate::simulation::config::TransientAnalysisConfig {
            step_time: 1e-6,
            stop_time: 10e-3,
            ..Default::default()
        })
        .into(),
    );
    base.analysis_line = ".four 1k V(out)".into();
    base.measurements = vec!["bin:1:magnitude:V(out)".into()];
    base.postprocess = Some(StudyPostprocess {
        producer_instance_id: AnalysisInstanceId::new(),
        producer_source_revision: ObjectRevision::INITIAL,
        producer_analysis_line: ".tran 1u 10m".into(),
        producer_numeric_options: ".options RELTOL=1e-6".into(),
        request: AnalysisSpec::Fourier {
            fundamental_freq: 1e3,
            num_harmonics: 3,
            num_periods: 1,
            output_node: "out".into(),
            output_ref: "0".into(),
            additional_outputs: vec![],
            start_time: 0.0,
            stop_time: 10e-3,
            compute_thd: true,
            normalize: false,
        },
        periodic_options: None,
    });
    base.postprocess
        .as_ref()
        .unwrap()
        .request
        .validate()
        .unwrap();
    let expected = monte_carlo_evaluator_digest(&base);
    base.source_revision = base.source_revision.next().unwrap();
    base.postprocess.as_mut().unwrap().producer_source_revision = base.source_revision;
    base.histogram_bins = 7;
    assert_eq!(monte_carlo_evaluator_digest(&base), expected);
    for change in [
        (|post: &mut StudyPostprocess| post.producer_instance_id = AnalysisInstanceId::new())
            as fn(&mut StudyPostprocess),
        |post| post.producer_numeric_options.push_str(" METHOD=GEAR"),
        |post| post.producer_analysis_line.push_str(" UIC"),
        |post| {
            let AnalysisSpec::Fourier { normalize, .. } = &mut post.request else {
                unreachable!()
            };
            *normalize = true;
        },
    ] {
        let mut changed = base.clone();
        change(changed.postprocess.as_mut().unwrap());
        assert_ne!(monte_carlo_evaluator_digest(&changed), expected);
    }
}

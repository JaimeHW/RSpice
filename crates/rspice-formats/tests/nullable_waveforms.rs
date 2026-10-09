#![cfg(feature = "project-results")]

use rspice_app_types::product::ProjectId;
use rspice_formats::project_results::{
    ProjectSimulationResults, ProjectSimulationResultsData, ProjectSimulationRun,
    ProjectWaveformData,
};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::run::{ExecutionTarget, SimulationRun, SimulationRunLifecycle};
use rspice_results::run_receipt::SimulationRunProvenance;
use rspice_results::waveform::RetainedWaveform;

fn project(sample: f64, complex: bool) -> ProjectSimulationResults {
    let mut waveform =
        RetainedWaveform::new("out", vec![0.0, 1.0, 2.0], vec![1.0, sample, -0.0]).with_unit("V");
    if complex {
        waveform = waveform.with_complex_components(
            "out",
            vec![1.0, sample, -0.0],
            vec![0.0, sample, 0.0],
        );
    }
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    let mut analysis =
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![waveform]);
    analysis.import_source = Some(rspice_results::result_import::ResultImportSource {
        source_name: "gaps.raw".into(),
        format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
        coordinate: None,
    });
    run.add_analysis(analysis);
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.lifecycle = SimulationRunLifecycle::Completed;
    ProjectSimulationResults::from(ProjectSimulationResultsData {
        runs: vec![ProjectSimulationRun::from_run(&run, |waveform| {
            ProjectWaveformData::from_waveform(waveform, "#ff0000".into(), true)
        })],
        next_run_id: 1,
        ..Default::default()
    })
}

#[test]
fn project_roundtrip_preserves_unavailable_samples_and_binds_their_identity() {
    for complex in [false, true] {
        let current = project(f64::NAN, complex);
        current.validate().unwrap();
        let encoded = serde_json::to_string(&current).unwrap();
        assert!(encoded.contains("[1.0,null,-0.0]"));
        let restored: ProjectSimulationResults = serde_json::from_str(&encoded).unwrap();
        restored.validate().unwrap();
        assert_eq!(current, restored);
        let waveform = &restored.runs[0].analyses[0].waveforms[0];
        assert!(waveform.y[1].is_nan());
        assert_eq!(waveform.y[2].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(waveform.clone().into_waveform().sample(1), None);
        assert_ne!(
            current.runs[0].dataset_content_digest,
            project(0.0, complex).runs[0].dataset_content_digest
        );
        // Different NaN payloads describe the same absence and have stable hashes.
        assert_eq!(
            current.runs[0].dataset_content_digest,
            project(f64::from_bits(0x7ff8_0000_0000_0123), complex).runs[0].dataset_content_digest
        );
        let mut legacy = restored;
        legacy.schema_version = 42;
        assert!(legacy.validate().is_err());
        assert!(
            legacy
                .migrate_to_current(ProjectId::new())
                .unwrap_err()
                .contains("unavailable")
        );
        assert_eq!(legacy.schema_version, 42);
    }
    let mut legacy_dense = project(0.0, false);
    let digest = legacy_dense.runs[0].dataset_content_digest.clone();
    legacy_dense.schema_version = 42;
    legacy_dense.migrate_to_current(ProjectId::new()).unwrap();
    legacy_dense.validate().unwrap();
    assert_eq!(legacy_dense.schema_version, 44);
    assert_eq!(legacy_dense.runs[0].dataset_content_digest, digest);
}

#[test]
fn project_rejects_infinities_missing_coordinates_and_partial_complex_samples() {
    for sample in [f64::INFINITY, f64::NEG_INFINITY] {
        let invalid = project(sample, false);
        assert!(invalid.validate().is_err());
        assert!(serde_json::to_string(&invalid).is_err());
    }
    let mut invalid = project(f64::NAN, true);
    std::sync::Arc::make_mut(
        &mut invalid.runs[0].analyses[0].waveforms[0]
            .complex
            .as_mut()
            .unwrap()
            .imag,
    )[1] = 0.0;
    assert!(invalid.validate().unwrap_err().contains("availability"));
    let mut invalid = project(f64::NAN, false);
    std::sync::Arc::make_mut(&mut invalid.runs[0].analyses[0].waveforms[0].x)[1] = f64::NAN;
    assert!(invalid.validate().is_err());
}

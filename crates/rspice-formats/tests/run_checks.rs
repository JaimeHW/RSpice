//! A run's checks (HDL severity-task records and `.ASSERT` reports) through
//! the project results format, result schema v45.
#![cfg(feature = "project-results")]

use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::config::{HdlOrigin, HdlPoint, HdlSeverity};
use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};
use rspice_formats::project_results::{
    ProjectSimulationResults, ProjectSimulationResultsData, ProjectSimulationRun,
};
use rspice_results::analysis_result::{AnalysisResult, HdlDiagnostic, RunChecks};
use rspice_results::analysis_type::AnalysisType;
use rspice_results::run::{ExecutionTarget, SimulationRun, SimulationRunLifecycle};
use rspice_results::run_receipt::SimulationRunProvenance;
use rspice_vocabulary::product::ProjectId;

fn checks() -> RunChecks {
    RunChecks {
        hdl: vec![HdlDiagnostic {
            severity: HdlSeverity::Error,
            text: "code 3: step 0.0091 V is outside [5 mV, 7 mV]".into(),
            point: HdlPoint::Transient { time: 3.99e-4 },
            origin: HdlOrigin {
                scope: "xtb".into(),
                file: "tb_ldo_fail.vams".into(),
                line: 40,
            },
        }],
        assertions: vec![
            AssertionReport {
                name: "a".into(),
                analysis: AssertAnalysis::Tran,
                target: "V(out)".into(),
                instance: None,
                level: AssertLevel::Error,
                message: Some("over half".into()),
                origin: NetlistSourceLocation::in_file("rc.cir", 5),
                violations: vec![AssertViolation {
                    start: Some(6.931_477_04e-7),
                    end: None,
                    extreme: Some(0.993),
                    at: Some(5e-6),
                }],
                unknown_stretches: 2,
                error: None,
            },
            AssertionReport {
                name: "mpsat".into(),
                analysis: AssertAnalysis::Op,
                target: "@mp[vds]".into(),
                instance: Some("mp".into()),
                level: AssertLevel::Notice,
                message: None,
                origin: NetlistSourceLocation::in_memory(9),
                violations: Vec::new(),
                unknown_stretches: 0,
                error: Some("@mp[vds] is not in the result".into()),
            },
        ],
    }
}

fn persisted(checks: RunChecks) -> ProjectSimulationRun {
    let mut analysis: AnalysisResult =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0);
    analysis.checks = checks;
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.add_analysis(analysis);
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.lifecycle = SimulationRunLifecycle::Completed;
    ProjectSimulationRun::from_run(&run, |_| unreachable!("no waveforms"))
}

#[test]
fn run_checks_survive_the_project_file_and_are_sealed_into_the_result_digest() {
    let with = persisted(checks());
    let without = persisted(RunChecks::default());
    assert!(without.analyses[0].checks.is_missing());
    assert_ne!(
        with.analyses[0].result_data_digest, without.analyses[0].result_data_digest,
        "the checks are part of the result's identity"
    );
    let current = ProjectSimulationResults::from(ProjectSimulationResultsData {
        runs: vec![with],
        next_run_id: 1,
        ..Default::default()
    });
    current.validate().unwrap();
    assert_eq!(current.schema_version, 45);
    let json = serde_json::to_string(&current).unwrap();
    assert!(json.contains("\"checks\":"), "{json}");
    let restored: ProjectSimulationResults = serde_json::from_str(&json).unwrap();
    restored.validate().unwrap();
    assert_eq!(restored.runs, current.runs);
    let analysis = restored.runs[0].analyses[0]
        .clone()
        .into_analysis()
        .unwrap();
    assert_eq!(analysis.checks, checks());

    // A result without checks writes no field.
    let json = serde_json::to_string(&without).unwrap();
    assert!(!json.contains("\"checks\":"), "{json}");
}

#[test]
fn checks_relabelled_as_a_legacy_document_are_refused_and_a_v44_one_migrates() {
    for version in [1, 30, 43, 44] {
        let mut project = ProjectSimulationResults::from(ProjectSimulationResultsData {
            schema_version: version,
            runs: vec![persisted(checks())],
            ..Default::default()
        });
        let error = project.migrate_to_current(ProjectId::new()).unwrap_err();
        assert!(error.contains("run checks"), "v{version}: {error}");
        assert_eq!(project.schema_version, version, "the refusal is atomic");
    }
    // A v44 document has no checks, and its digests stay valid.
    let legacy = persisted(RunChecks::default());
    let mut project = ProjectSimulationResults::from(ProjectSimulationResultsData {
        schema_version: 44,
        runs: vec![legacy.clone()],
        next_run_id: 1,
        ..Default::default()
    });
    project.migrate_to_current(ProjectId::new()).unwrap();
    project.validate().unwrap();
    assert_eq!(project.schema_version, 45);
    assert_eq!(
        project.runs[0].dataset_content_digest,
        legacy.dataset_content_digest
    );
}

#[test]
fn an_unknown_assertion_level_or_a_non_finite_violation_is_refused() {
    let mut run = persisted(checks());
    let mut bad_level = run.clone();
    if let Some(checks) = bad_level.analyses[0].checks.as_mut() {
        checks.assertions[0].level = "fatal".into();
    }
    let project = ProjectSimulationResults::from(ProjectSimulationResultsData {
        runs: vec![bad_level],
        next_run_id: 1,
        ..Default::default()
    });
    let error = project.validate().unwrap_err();
    assert!(error.contains("unknown assertion level"), "{error}");

    if let Some(checks) = run.analyses[0].checks.as_mut() {
        checks.assertions[0].violations[0].extreme = Some(f64::NAN);
    }
    let project = ProjectSimulationResults::from(ProjectSimulationResultsData {
        runs: vec![run],
        next_run_id: 1,
        ..Default::default()
    });
    assert!(project.validate().is_err());
}

#![cfg(feature = "result-csv")]
use rspice_core::{
    CurrentImpulseOwner, CurrentImpulseTrace, ImpulseDerivative, VoltageImpulsePoint,
    VoltageImpulseTrace,
};
use rspice_formats::result_csv::{TypedCsvSummary, encode_typed_result_csv};
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::current_impulses::CurrentImpulseHistoryEvidence;
use rspice_results::result_digest::ResultDigestEncoding;
use rspice_results::voltage_impulses::VoltageImpulseHistoryEvidence;

fn history() -> VoltageImpulseHistoryEvidence {
    VoltageImpulseHistoryEvidence {
        start_time_s: 0.0,
        stop_time_s: 1.0,
        delivery_complete: true,
        traces: vec![VoltageImpulseTrace {
            node_name: "out".into(),
            complete: true,
            points: vec![VoltageImpulsePoint {
                time: 0.3,
                volt_seconds: -0.002,
            }],
            derivatives: vec![ImpulseDerivative {
                time: 0.3,
                order: 2,
                coefficient: -1e-21,
            }],
        }],
    }
}

fn analysis(voltage: Option<VoltageImpulseHistoryEvidence>) -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::Transient, "voltage observations", 0.0)
        .with_result_payload(AnalysisResultPayload::TransientEvents {
            current_impulses: Some(CurrentImpulseHistoryEvidence {
                start_time_s: 0.0,
                stop_time_s: 1.0,
                delivery_complete: true,
                traces: vec![CurrentImpulseTrace {
                    owner: CurrentImpulseOwner::Branch {
                        branch_name: "H1".into(),
                    },
                    complete: true,
                    points: vec![],
                    derivatives: vec![ImpulseDerivative {
                        time: 0.3,
                        order: 1,
                        coefficient: 2e-21,
                    }],
                }],
            }),
            voltage_impulses: voltage,
            digital_traces: vec![],
            real_traces: vec![],
            digital_buses: vec![],
        })
}

#[test]
fn csv_preserves_voltage_actions_and_both_kinds_of_derivative() {
    let encoded = encode_typed_result_csv(&analysis(Some(history()))).unwrap();
    assert_eq!(
        encoded.summary,
        TypedCsvSummary::ImpulseEvents {
            node_count: 0,
            current_history_count: 1,
            voltage_history_count: 1,
            impulse_count: 3,
        }
    );
    let mut reader = csv::Reader::from_reader(encoded.contents.as_bytes());
    let header = reader.headers().unwrap().clone();
    let col = |name| header.iter().position(|field| field == name).unwrap();
    let rows: Vec<_> = reader.records().collect::<Result<_, _>>().unwrap();
    let current = rows
        .iter()
        .find(|row| &row[col("domain")] == "current_impulse" && &row[col("record")] == "derivative")
        .unwrap();
    assert_eq!(&current[col("derivative_order")], "1");
    assert_eq!(
        current[col("derivative_coefficient_si")]
            .parse::<f64>()
            .unwrap(),
        2e-21
    );
    assert_eq!(&current[col("charge_coulombs")], "");
    assert_eq!(&current[col("volt_seconds")], "");
    let voltage = rows
        .iter()
        .find(|row| &row[col("domain")] == "voltage_impulse" && &row[col("record")] == "event")
        .unwrap();
    assert_eq!(voltage[col("volt_seconds")].parse::<f64>().unwrap(), -0.002);
    assert_eq!(&voltage[col("charge_coulombs")], "");
    let derivative = rows
        .iter()
        .find(|row| &row[col("domain")] == "voltage_impulse" && &row[col("record")] == "derivative")
        .unwrap();
    assert_eq!(&derivative[col("derivative_order")], "2");
    assert_eq!(
        derivative[col("derivative_coefficient_si")]
            .parse::<f64>()
            .unwrap(),
        -1e-21
    );
    assert_eq!(&derivative[col("volt_seconds")], "");
    assert_eq!(
        rows.iter()
            .filter(|row| &row[col("record")] == "coverage")
            .count(),
        2
    );
}

#[test]
fn every_voltage_history_field_participates_in_identity_and_retention_accounting() {
    let digest = |value| {
        analysis(value)
            .result_data_ref()
            .digest(ResultDigestEncoding::CURRENT)
    };
    let original = digest(Some(history()));
    assert_ne!(original, digest(None));
    for case in 0..9 {
        let mut changed = history();
        match case {
            0 => changed.delivery_complete = false,
            1 => changed.start_time_s = 0.1,
            2 => changed.stop_time_s = 2.0,
            3 => changed.traces[0].node_name = "ref".into(),
            4 => changed.traces[0].complete = false,
            5 => changed.traces[0].points[0].time = 0.4,
            6 => changed.traces[0].points[0].volt_seconds = 0.002,
            7 => changed.traces[0].derivatives[0].order = 3,
            8 => changed.traces[0].derivatives[0].coefficient = 1e-21,
            _ => unreachable!(),
        }
        changed.validate().unwrap();
        assert_ne!(original, digest(Some(changed)), "case {case}");
    }
    let retained = analysis(Some(history()));
    let legacy = analysis(None);
    assert!(
        retained.result_data_ref().retained_data_bytes()
            > legacy.result_data_ref().retained_data_bytes() + 40
    );
}

#[cfg(feature = "project-results")]
#[test]
fn project_schema_rejects_voltage_evidence_relabelled_as_a_legacy_document() {
    use rspice_app_types::product::ProjectId;
    use rspice_formats::project_results::{
        ProjectSimulationResults, ProjectSimulationResultsData, ProjectSimulationRun,
    };
    use rspice_results::run::{ExecutionTarget, SimulationRun, SimulationRunLifecycle};
    use rspice_results::run_receipt::SimulationRunProvenance;
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.add_analysis(analysis(Some(history())));
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.lifecycle = SimulationRunLifecycle::Completed;
    let persisted = ProjectSimulationRun::from_run(&run, |_| unreachable!("no waveforms"));
    let current = ProjectSimulationResults::from(ProjectSimulationResultsData {
        runs: vec![persisted.clone()],
        next_run_id: 1,
        ..Default::default()
    });
    current.validate().unwrap();
    let json = serde_json::to_string(&current).unwrap();
    let restored: ProjectSimulationResults = serde_json::from_str(&json).unwrap();
    restored.validate().unwrap();
    assert_eq!(restored.runs, current.runs);
    for version in [1, 15, 29, 39, 40] {
        let mut project = ProjectSimulationResults::from(ProjectSimulationResultsData {
            schema_version: version,
            runs: vec![persisted.clone()],
            ..Default::default()
        });
        let error = project.migrate_to_current(ProjectId::new()).unwrap_err();
        assert!(error.contains("voltage impulse"), "v{version}: {error}");
        assert_eq!(
            project.schema_version, version,
            "migration refusal is atomic"
        );
    }
    // Existing v40 digests remain valid when voltage histories are absent.
    run.analyses[0] = analysis(None);
    let legacy = ProjectSimulationRun::from_run(&run, |_| unreachable!("no waveforms"));
    let mut project = ProjectSimulationResults::from(ProjectSimulationResultsData {
        schema_version: 40,
        runs: vec![legacy.clone()],
        next_run_id: 1,
        ..Default::default()
    });
    project.migrate_to_current(ProjectId::new()).unwrap();
    project.validate().unwrap();
    assert_eq!(project.schema_version, 43);
    assert_eq!(
        project.runs[0].dataset_content_digest,
        legacy.dataset_content_digest
    );
}

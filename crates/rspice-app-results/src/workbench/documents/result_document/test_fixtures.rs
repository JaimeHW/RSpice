//! Evidence and test doubles that both this feature's tests and the
//! application's tests build from.
//!
//! Each has one body, so the two suites cannot drift apart. Everything here is
//! retained evidence, encoded bytes or a recording double: nothing holds or
//! builds a host, because each suite builds its own.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use super::*;
use crate::product::{ContentDigest, ObjectRevision, SimulationPlanId};
use crate::state::{
    AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultSourceDomain, AnalysisType,
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRunLifecycle,
    SimulationRunProvenance,
};
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub fn soa_analysis() -> AnalysisResult {
    use crate::state::{
        AnalysisResultFamilyMetadata, SoaEvaluationEvidence, SoaParameterEvidence,
        SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
    };
    let time = vec![0.0, 1.0e-9];
    AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: time.clone() })
        .with_waveforms(vec![
            RetainedTrace::new(
                "SOA_VIOLATION_COUNT",
                time.clone(),
                vec![0.0, 0.0],
                "#ffbd2e",
            ),
            // The stress history the producer now retains, named exactly as
            // the sheet addresses it.
            RetainedTrace::new(
                crate::results::safety::soa_stress_waveform_name(
                    "M1",
                    crate::results::safety::SoAParameter::Vds,
                ),
                time,
                vec![2.0, 3.0],
                "#00aaff",
            ),
        ])
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: None,
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.0,
                worst_time_s: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                // 3.0 V against a 3.3 V limit is inside the warning band,
                // which the payload validator derives rather than trusts.
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            // A non-passing rule must carry the exact event at its worst
            // point; the validator refuses a verdict with nothing behind it.
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.0,
                time_s: 1.0e-9,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        })
}

pub fn optimization_analysis() -> AnalysisResult {
    use crate::state::AnalysisResultFamilyMetadata;
    AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            best_objectives: Vec::new(),
            best_constraints: Vec::new(),
            iterations: vec![0.0, 1.0],
            best_cost: 0.25,
            best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
            converged: true,
        })
        // The names the optimization runner emits: one OPT_COST trace over
        // the iteration axis, plus OPT_<variable> per design variable.
        .with_waveforms(vec![
            RetainedTrace::new("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25], "#ffbd2e"),
            RetainedTrace::new("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6], "#00aaff"),
        ])
}

/// A committed event history with all three kinds of row the Events sheet
/// draws: a scalar conductor, a real-valued event node, and two declared
/// buses over their own member traces.
///
/// The buses are here rather than in a second fixture because every gate that
/// reads this one is a gate about the sheet — the fit gate below measures the
/// radix bar and the per-row disclosures, which a history declaring no bus
/// does not paint at all.
pub fn events_analysis() -> AnalysisResult {
    use crate::state::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
    };
    let trace = |name: &str, points: &[(f64, u8)]| DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: points
            .iter()
            .map(|(time_s, value_code)| DigitalEventPointEvidence {
                time_s: *time_s,
                value_code: *value_code,
            })
            .collect(),
    };
    let bus = |name: &str| DigitalBusEvidence {
        name: name.to_owned(),
        msb: 1,
        lsb: 0,
        members: vec![format!("{name}[1]"), format!("{name}[0]")],
        source: DigitalBusSourceEvidence::Import,
    };
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
        AnalysisResultPayload::TransientEvents {
            voltage_impulses: None,
            current_impulses: None,
            digital_traces: vec![
                trace("clk", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 12)]),
                trace("addr[1]", &[(0.0, 0), (1.0e-9, 1)]),
                trace("addr[0]", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 0)]),
                // One bit the run never states before its first change, so a
                // word the sheet cannot spell as a number is on screen too.
                trace("data[1]", &[(5.0e-10, 2)]),
                trace("data[0]", &[(0.0, 1), (1.0e-9, 12)]),
            ],
            real_traces: vec![RealEventTraceEvidence {
                node_name: "level".to_owned(),
                points: vec![RealEventPointEvidence {
                    time_s: 2.5e-10,
                    value: 0.75,
                }],
            }],
            digital_buses: vec![bus("addr"), bus("data")],
        },
    )
}

pub fn network_matrix_analysis(mixed: bool) -> AnalysisResult {
    let mut traces = Vec::new();
    let ports = if mixed { 4 } else { 2 };
    for row in 0..ports {
        for column in 0..ports {
            let name = if mixed {
                format!(
                    "S{}{}{}{}",
                    if row < 2 { "d" } else { "c" },
                    if column < 2 { "d" } else { "c" },
                    row % 2 + 1,
                    column % 2 + 1
                )
            } else {
                format!("S{}{}", row + 1, column + 1)
            };
            let value = if row == column { 0.25 } else { 0.01 };
            traces.push(
                RetainedTrace::new(
                    format!("|{name}|"),
                    vec![1e6, 2e6],
                    vec![value; 2],
                    "#00aaff",
                )
                .with_complex_components(name, vec![value; 2], vec![0.0; 2]),
            );
        }
    }
    AnalysisResult::new(1, AnalysisType::SParameter, "Network")
        .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm: vec![50.0; ports],
            noise_reference_temperature_kelvin: None,
        })
        .with_waveforms(traces)
}

pub fn analysis(kind: AnalysisType, id: u64, samples: usize) -> AnalysisResult {
    let (name, value) = match kind {
        AnalysisType::Noise => ("onoise", 1.0e-12),
        AnalysisType::Disto => ("HD2", 1.0),
        _ => ("V(out)", 1.0),
    };
    AnalysisResult::new(id, kind, name).with_waveforms(vec![RetainedTrace::new(
        name,
        (1..=samples)
            .map(|sample| sample as f64)
            .collect::<Vec<_>>(),
        vec![value; samples],
        "#55aaff",
    )])
}

pub fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

pub fn completed_prepared_run(
    plan_id: SimulationPlanId,
    project_revision: ObjectRevision,
    source_digest: ContentDigest,
) -> SimulationRun {
    let task = PreparedRunTaskReceipt::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        Vec::new(),
        5,
        digest(0x44),
    )
    .expect("task receipt");
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(plan_id),
        project_revision,
        prepared_snapshot_digest: digest(0x41),
        source_content_digest: source_digest,
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![task],
    })
    .expect("run receipt");
    let mut run = SimulationRun::new(1);
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("run provenance");
    run.mark_running().expect("running lifecycle");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("completed lifecycle");
    run
}

#[derive(Debug)]
pub struct MockExportWorkflowIo {
    pub datasets: RefCell<Vec<crate::io::WaveformDataset>>,
    pub paths: RefCell<Vec<PathBuf>>,
    pub text_files: RefCell<Vec<(PathBuf, String)>>,
    pub byte_files: RefCell<Vec<(PathBuf, Vec<u8>, String)>>,
    pub dialog_titles: RefCell<Vec<String>>,
    pub saved_paths_are_reopenable: bool,
    /// Why the save dialog could not open, when it cannot.
    pub dialog_error: Option<String>,
}

impl Default for MockExportWorkflowIo {
    fn default() -> Self {
        Self {
            datasets: RefCell::default(),
            paths: RefCell::default(),
            text_files: RefCell::default(),
            byte_files: RefCell::default(),
            dialog_titles: RefCell::default(),
            saved_paths_are_reopenable: true,
            dialog_error: None,
        }
    }
}

impl ExportWorkflowIo for MockExportWorkflowIo {
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        self.dialog_titles
            .borrow_mut()
            .push(config.title.to_owned());
        if let Some(error) = &self.dialog_error {
            return Err(error.clone());
        }
        Ok(Some(PathBuf::from(config.default_name)))
    }

    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        self.text_files
            .borrow_mut()
            .push((path.to_path_buf(), contents.to_owned()));
        Ok(())
    }

    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String> {
        self.datasets.borrow_mut().push(dataset.clone());
        self.paths.borrow_mut().push(path.to_path_buf());
        Ok(())
    }

    fn write_bytes_file_observed(
        &self,
        destination: &crate::workbench::workflows::export_workflow::ObservedExportDestination,
        contents: &[u8],
        mime_type: &str,
    ) -> Result<(), String> {
        self.byte_files.borrow_mut().push((
            destination.path().to_path_buf(),
            contents.to_vec(),
            mime_type.to_owned(),
        ));
        Ok(())
    }

    fn saved_paths_are_reopenable(&self) -> bool {
        self.saved_paths_are_reopenable
    }
}

pub fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> RetainedTrace {
    RetainedTrace::new(name.to_owned(), x, y, "#4f81bd")
}

pub fn generated_fst() -> Vec<u8> {
    use fst_writer::{
        FstFileType, FstInfo, FstScopeType, FstSignalType, FstVarDirection, FstVarType,
    };
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-result-import-{}-{nonce}.fst",
        std::process::id()
    ));
    let info = FstInfo {
        start_time: 0,
        timescale_exponent: -9,
        version: "RSpice import fixture".to_owned(),
        date: "2026-08-31".to_owned(),
        file_type: FstFileType::Verilog,
    };
    let mut header = fst_writer::open_fst(&path, &info).expect("open FST fixture");
    header
        .scope("top", "top", FstScopeType::Module)
        .expect("FST scope");
    let clock = header
        .var(
            "clock",
            FstSignalType::bit_vec(1),
            FstVarType::Wire,
            FstVarDirection::Implicit,
            None,
        )
        .expect("FST signal");
    header.up_scope().expect("FST upscope");
    let mut body = header.finish().expect("FST header");
    for tick in 0..96_u64 {
        body.time_change(tick).expect("FST time change");
        body.signal_change(clock, if tick & 1 == 0 { b"0" } else { b"1" })
            .expect("FST signal change");
    }
    body.finish().expect("FST finish");
    let bytes = std::fs::read(&path).expect("read FST fixture");
    let _ = std::fs::remove_file(&path);
    bytes
}

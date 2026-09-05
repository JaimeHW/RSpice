//! The export registry is a closed contract, not a menu that grew.
//!
//! Ten ids, each governing its own availability, so a format cannot ship
//! without a rule for when it is offered. The rest pin what each sheet
//! actually writes: a report or table exports the evidence it rendered, and a
//! saved path stays reopenable.

use super::*;

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType,
    ComplexResultValue, FloquetOrbitKindEvidence, FloquetSpectrumCertificateEvidence,
    FloquetSpectrumEvidence, FloquetStabilityVerdictEvidence, PssFloquetMultiplierEvidence,
    PstbFloquetModeEvidence, PstbStabilityClassificationEvidence, ReliabilityCheckpointEvidence,
    ReliabilityDeviceEvidence, ReliabilityShiftEvidence, ReliabilityStressEvidence,
    SensitivityResultMode, SensitivityResultRow, SimulationRun, SoaEvaluationEvidence,
    SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence, WaveformData,
};

#[derive(Debug)]
struct MockExportWorkflowIo {
    datasets: RefCell<Vec<crate::io::WaveformDataset>>,
    paths: RefCell<Vec<PathBuf>>,
    text_files: RefCell<Vec<(PathBuf, String)>>,
    byte_files: RefCell<Vec<(PathBuf, Vec<u8>, String)>>,
    dialog_titles: RefCell<Vec<String>>,
    saved_paths_are_reopenable: bool,
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
        }
    }
}

impl ExportWorkflowIo for MockExportWorkflowIo {
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        self.dialog_titles
            .borrow_mut()
            .push(config.title.to_owned());
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

fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> WaveformData {
    WaveformData::new(name.to_owned(), x, y, "#4f81bd")
}

fn floquet_certificate(problem_order: u64) -> FloquetSpectrumCertificateEvidence {
    FloquetSpectrumCertificateEvidence {
        problem_order,
        max_backward_error: 0.0,
        qualification_tolerance:
            FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(problem_order)
                .unwrap(),
    }
}

fn dc_op_with_one_node() -> crate::state::DcOpResult {
    crate::state::DcOpResult {
        node_voltages: vec![crate::state::OperatingPointValue {
            name: "V(out)".to_owned(),
            value: 1.25,
            unit: "V".to_owned(),
        }],
        ..crate::state::DcOpResult::default()
    }
}

#[test]
fn export_registry_matches_the_ten_contract_ids_and_governs_availability() {
    assert_eq!(
        ResultExportFormat::ALL.map(ResultExportFormat::canonical_id),
        [
            "rspice-result-bundle",
            "rspice-dataset-bundle",
            "csv-rfc4180",
            "tsv",
            "touchstone-v2",
            "hdf5",
            "numpy-npy",
            "numpy-npz",
            "matlab-v5",
            "vcd",
        ]
    );
    for format in ResultExportFormat::ALL {
        let availability = result_export_format_availability(format);
        if matches!(
            format,
            ResultExportFormat::RSpiceResultBundle
                | ResultExportFormat::RSpiceDatasetBundle
                | ResultExportFormat::CsvRfc4180
                | ResultExportFormat::Tsv
                | ResultExportFormat::TouchstoneV2
                | ResultExportFormat::NumpyNpy
                | ResultExportFormat::NumpyNpz
                | ResultExportFormat::Vcd
        ) {
            assert!(availability.is_ok(), "{}", format.canonical_id());
        } else {
            let error = availability.expect_err("encoder is governed unavailable");
            assert!(error.contains(format.canonical_id()), "{error}");
            assert!(error.contains("verified lossless encoder"), "{error}");
        }
    }
}

fn complex_waveform(
    name: &str,
    source_name: &str,
    x: Vec<f64>,
    display_y: Vec<f64>,
    real: Vec<f64>,
    imag: Vec<f64>,
) -> WaveformData {
    WaveformData::new(name.to_owned(), x, display_y, "#4f81bd").with_complex_components(
        source_name.to_owned(),
        real,
        imag,
    )
}

fn last_log_message(state: &AppState) -> String {
    state
        .log_buffer
        .entries()
        .last()
        .expect("a user-facing log line is emitted")
        .message
        .clone()
}

fn state_with_typed_result(analysis: AnalysisResult) -> AppState {
    let viewer = match analysis.analysis_type {
        AnalysisType::DcOp => crate::workbench::ResultViewer::Op,
        AnalysisType::PoleZero => crate::workbench::ResultViewer::PoleZero,
        AnalysisType::Sensitivity => crate::workbench::ResultViewer::Contribution,
        AnalysisType::Tf => crate::workbench::ResultViewer::TransferFunction,
        AnalysisType::Reliability => crate::workbench::ResultViewer::Reliability,
        AnalysisType::Soa => crate::workbench::ResultViewer::Soa,
        AnalysisType::Optimization => crate::workbench::ResultViewer::Optimization,
        kind if kind.is_time_domain() => crate::workbench::ResultViewer::Waves,
        kind if kind.is_bode_response() || kind.is_raw_frequency_curve() => {
            crate::workbench::ResultViewer::Bode
        }
        _ => crate::workbench::ResultViewer::Table,
    };
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, viewer);
    state
}

fn activate_result_document(state: &mut AppState, viewer: crate::workbench::ResultViewer) {
    let dataset_id = state
        .simulation
        .active_run()
        .expect("test retains an active result run")
        .dataset_id;
    state.workbench.documents.activate(
        crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
    );
    state.ui.results.viewer = viewer;
}

fn bind_active_fft_authority(state: &mut AppState) {
    let authority = {
        let run = state
            .simulation
            .active_run()
            .expect("test retains an active result run");
        let analysis = state
            .simulation
            .active_analysis()
            .expect("test retains an active result analysis");
        crate::workbench::app_state::SpecializedViewerCacheProvenance::for_analysis(
            run.dataset_id,
            analysis,
        )
    };
    state.bind_specialized_viewer_cache(crate::workbench::app_state::ActiveViewer::Fft, authority);
}

#[test]
fn report_and_table_sheets_export_the_evidence_they_render() {
    let mut manifest_state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0], vec![0.0, 1.0])]),
    );
    activate_result_document(
        &mut manifest_state,
        crate::workbench::ResultViewer::Manifest,
    );
    let manifest_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut manifest_state, &manifest_io);
    let manifest = &manifest_io.text_files.borrow()[0].1;
    assert!(manifest.contains("dataset_id"));
    assert!(manifest.contains("inventory,,,Transient"));

    let mut op = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
    op.dc_op = Some(crate::state::DcOpResult {
        node_voltages: vec![crate::state::OperatingPointValue {
            name: "V(out)".to_owned(),
            value: 1.25,
            unit: "V".to_owned(),
        }],
        ..crate::state::DcOpResult::default()
    });
    let mut op_state = state_with_typed_result(op);
    let op_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut op_state, &op_io);
    let op_csv = &op_io.text_files.borrow()[0].1;
    assert!(op_csv.contains("node_voltage,V(out),,,value,1.25000000000000000e0,V"));

    let mut specs_state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.5)]),
    );
    specs_state.workspace.specs.push(crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max V(out)".to_owned(),
        min: Some(1.0),
        max: Some(2.0),
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    });
    activate_result_document(&mut specs_state, crate::workbench::ResultViewer::Specs);
    let specs_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut specs_state, &specs_io);
    let specs = &specs_io.text_files.borrow()[0].1;
    assert!(specs.contains("gain,max V(out),1.50000000000000000e0"));
    assert!(specs.contains(",pass,"));

    let optimization = AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            iterations: vec![0.0, 1.0],
            best_cost: 0.25,
            best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
            converged: true,
        })
        .with_waveforms(vec![
            waveform("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25]),
            waveform("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6]),
        ]);
    let mut optimization_state = state_with_typed_result(optimization);
    let optimization_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut optimization_state, &optimization_io);
    let optimization = &optimization_io.text_files.borrow()[0].1;
    assert!(optimization.contains("converged,true"));
    assert!(optimization.contains("iteration,cost,w"));
    assert!(optimization.contains("1.00000000000000000e0,2.50000000000000000e-1"));
}

/// A derived sheet must export the curve it drew, not the samples it was
/// derived from.
///
/// The export routed on the retained payload alone, so exporting from the
/// FFT sheet wrote the transient — the input to the spectrum, never the
/// spectrum. The distribution and the eye had the same gap, and none of
/// the three is retained anywhere the payload path can see.
#[test]
fn csv_export_from_a_derived_sheet_publishes_what_that_sheet_draws() {
    use crate::analysis::fft::data::{FftData, SpectrumNormalization};
    use crate::analysis::histogram::data::{Histogram, HistogramBin};

    let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.0])]);
    let mut state = state_with_typed_result(transient);
    state.analysis.fft_state.normalization = SpectrumNormalization::Peak;
    state
        .analysis
        .fft_state
        .load_data(FftData::from_spectrum_with_normalization(
            "V(out)",
            &[0.0, 1.0e3, 2.0e3],
            &[1.0, 0.5, 0.25],
            &[0.0, 0.1, 0.2],
            4_000.0,
            SpectrumNormalization::Peak,
        ));
    bind_active_fft_authority(&mut state);
    state.ui.results.viewer = crate::workbench::ResultViewer::Fft;
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("rspice-spectrum.csv"));
    let spectrum = files[0].1.clone();
    drop(files);
    assert!(
        spectrum.contains("frequency_hz,magnitude,magnitude_db,phase_rad"),
        "{spectrum}"
    );
    assert!(spectrum.contains("2.00000000000000000e3"), "{spectrum}");
    assert!(
        !spectrum.contains("V(out)\ntime"),
        "the transient leaked into a spectrum export: {spectrum}"
    );

    // The distribution exports its bins, not the Monte-Carlo waveform.
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_waveforms(vec![waveform(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.9, 1.1],
        )]),
    );
    state.analysis.histogram_state.load_histogram(Histogram {
        name: "V(out)".to_owned(),
        bins: vec![HistogramBin {
            lower: 0.9,
            upper: 1.0,
            count: 3,
            weight: 3.0,
        }],
        total_count: 3,
        total_weight: 3.0,
        underflow: 0,
        overflow: 0,
        data_min: 0.9,
        data_max: 1.0,
    });
    state.ui.results.viewer = crate::workbench::ResultViewer::Hist;
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("rspice-distribution.csv"));
    assert!(
        files[0].1.contains("bin_lower,bin_upper,count,weight"),
        "{}",
        files[0].1
    );
}

#[test]
fn unavailable_fft_export_never_falls_through_to_the_source_transient() {
    use crate::analysis::fft::data::{FftData, SpectrumNormalization};

    let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.0])]);
    let mut state = state_with_typed_result(transient);
    // A data-only partial transaction must not count as an exportable FFT.
    state.analysis.fft_state.data = Some(FftData::from_spectrum_with_normalization(
        "partial",
        &[0.0, 1.0e3, 2.0e3],
        &[1.0, 0.5, 0.25],
        &[0.0, 0.1, 0.2],
        4_000.0,
        SpectrumNormalization::Peak,
    ));
    bind_active_fft_authority(&mut state);
    state.ui.results.viewer = crate::workbench::ResultViewer::Fft;
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    assert!(io.text_files.borrow().is_empty());
    assert!(io.datasets.borrow().is_empty());
    assert!(last_log_message(&state).contains("derived analysis is incomplete"));
}

#[test]
fn fft_export_rejects_complete_but_unbound_or_stale_cache_evidence() {
    use crate::analysis::fft::data::{FftData, SpectrumNormalization};

    for stale in [false, true] {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.0])]);
        let mut state = state_with_typed_result(transient);
        state
            .analysis
            .fft_state
            .load_data(FftData::from_spectrum_with_normalization(
                "V(stale)",
                &[0.0, 1.0e3, 2.0e3],
                &[1.0, 0.5, 0.25],
                &[0.0, 0.1, 0.2],
                4_000.0,
                SpectrumNormalization::Peak,
            ));
        if stale {
            let dataset_id = state.simulation.active_run().unwrap().dataset_id;
            let foreign = AnalysisResult::new(999, AnalysisType::Transient, "FOREIGN");
            let authority =
                crate::workbench::app_state::SpecializedViewerCacheProvenance::for_analysis(
                    dataset_id, &foreign,
                );
            state.bind_specialized_viewer_cache(
                crate::workbench::app_state::ActiveViewer::Fft,
                authority,
            );
        }
        state.ui.results.viewer = crate::workbench::ResultViewer::Fft;
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        assert!(io.text_files.borrow().is_empty());
        assert!(io.datasets.borrow().is_empty());
        assert!(last_log_message(&state).contains("does not belong to the displayed result"));
    }
}

/// A sheet that plots a retained vector keeps the payload path: its
/// export already is what it shows, and a second writer would be a
/// second answer to the same question.
#[test]
fn csv_export_from_a_retained_sheet_is_unchanged_by_the_derived_route() {
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 2.0],
        )]),
    );
    state.ui.results.viewer = crate::workbench::ResultViewer::Waves;
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    assert!(
        io.text_files.borrow().is_empty(),
        "a waveform sheet must still go through the waveform dataset writer"
    );
    assert_eq!(io.datasets.borrow().len(), 1);
}

#[test]
fn csv_export_publishes_exact_pole_zero_evidence_without_pseudo_waveforms() {
    let analysis = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
        .with_result_payload(AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: -1.0,
            imaginary: 2.0,
        }],
        zeros: vec![ComplexResultValue {
            real: -3.0,
            imaginary: 0.0,
        }],
        pole_evidence: crate::state::PoleZeroRootSetEvidence::Qualified {
            certificate: crate::state::PoleZeroSpectrumCertificate {
                problem_order: 1,
                infinite_count: 0,
                max_backward_error: 1.0e-14,
                qualification_tolerance:
                    crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                        .unwrap(),
            },
        },
        zero_evidence: crate::state::PoleZeroRootSetEvidence::Qualified {
            certificate: crate::state::PoleZeroSpectrumCertificate {
                problem_order: 1,
                infinite_count: 0,
                max_backward_error: 2.0e-14,
                qualification_tolerance:
                    crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                        .unwrap(),
            },
        },
        gain: Some(4.25),
    });
    let mut state = state_with_typed_result(analysis);
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    assert!(io.datasets.borrow().is_empty());
    assert_eq!(io.dialog_titles.borrow().as_slice(), &["Export Result CSV"]);
    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("pole-zero.csv"));
    assert_eq!(
        files[0].1,
        concat!(
            "record,index,real_rad_per_s,imaginary_rad_per_s,value\n",
            "gain,,,,4.25000000000000000e0\n",
            "pole_evidence,,,,qualified\n",
            "pole_problem_order,,,,1\n",
            "pole_infinite_count,,,,0\n",
            "pole_max_backward_error,,,,9.99999999999999999e-15\n",
            "pole_qualification_tolerance,,,,2.84217094304040074e-14\n",
            "zero_evidence,,,,qualified\n",
            "zero_problem_order,,,,1\n",
            "zero_infinite_count,,,,0\n",
            "zero_max_backward_error,,,,2.00000000000000000e-14\n",
            "zero_qualification_tolerance,,,,2.84217094304040074e-14\n",
            "pole,0,-1.00000000000000000e0,2.00000000000000000e0,\n",
            "zero,0,-3.00000000000000000e0,0.00000000000000000e0,\n",
        )
    );
}

#[test]
fn csv_export_marks_unavailable_pole_zero_gain_explicitly() {
    let analysis = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
        .with_result_payload(AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: -1.0,
            imaginary: 2.0,
        }],
        zeros: Vec::new(),
        pole_evidence: crate::state::PoleZeroRootSetEvidence::Qualified {
            certificate: crate::state::PoleZeroSpectrumCertificate {
                problem_order: 1,
                infinite_count: 0,
                max_backward_error: 1.0e-14,
                qualification_tolerance:
                    crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                        .unwrap(),
            },
        },
        zero_evidence: crate::state::PoleZeroRootSetEvidence::NotRequested,
        gain: None,
    });
    let mut state = state_with_typed_result(analysis);
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("pole-zero.csv"));
    assert_eq!(
        files[0].1,
        concat!(
            "record,index,real_rad_per_s,imaginary_rad_per_s,value\n",
            "gain,,,,unavailable\n",
            "pole_evidence,,,,qualified\n",
            "pole_problem_order,,,,1\n",
            "pole_infinite_count,,,,0\n",
            "pole_max_backward_error,,,,9.99999999999999999e-15\n",
            "pole_qualification_tolerance,,,,2.84217094304040074e-14\n",
            "zero_evidence,,,,not requested\n",
            "pole,0,-1.00000000000000000e0,2.00000000000000000e0,\n",
        )
    );
}

#[test]
fn csv_export_publishes_canonical_sensitivity_rows_and_basis() {
    let analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS").with_result_payload(
        AnalysisResultPayload::Sensitivity {
            output: "V(out), differential".to_owned(),
            result_mode: SensitivityResultMode::Ac {
                frequency_hz: 10_000.0,
            },
            rows: vec![SensitivityResultRow {
                parameter: "width".to_owned(),
                raw: 2.0,
                normalized: 0.5,
            }],
        },
    );
    let mut state = state_with_typed_result(analysis);
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("sensitivity.csv"));
    assert_eq!(
        files[0].1,
        concat!(
            "parameter,raw_sensitivity,normalized_sensitivity,output,mode,frequency_hz\n",
            "width,2.00000000000000000e0,5.00000000000000000e-1,\"V(out), differential\",ac,1.00000000000000000e4\n",
        )
    );
}

#[test]
fn csv_export_publishes_exact_reliability_device_and_shift_evidence() {
    let analysis = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_family_metadata(AnalysisResultFamilyMetadata::Reliability { years: vec![10.0] })
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![ReliabilityDeviceEvidence {
                device_id: "M1".to_owned(),
                stress: ReliabilityStressEvidence {
                    average_gate_stress_v: 1.2,
                    average_drain_stress_v: 1.8,
                    average_temperature_k: 358.15,
                    duration_s: 3_600.0,
                },
                checkpoints: vec![ReliabilityCheckpointEvidence {
                    years: 10.0,
                    shift: ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.0125,
                        mobility_shift: -0.003,
                        drain_source_resistance_shift: 0.001,
                    },
                }],
            }],
        });
    let mut state = state_with_typed_result(analysis);
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("reliability-evidence.csv"));
    assert!(
        files[0]
            .1
            .starts_with("device,lifetime_years,average_gate_stress_v")
    );
    assert!(
        files[0]
            .1
            .contains("M1,1.00000000000000000e1,1.19999999999999996e0")
    );
    assert!(files[0].1.contains("1.25000000000000007e-2"));
}

#[test]
fn typed_csv_exports_every_pss_multiplier_and_its_certificate() {
    let payload = AnalysisResultPayload::PssFloquet {
        period_s: Some(2.0),
        fundamental_frequency_hz: Some(0.5),
        iterations: Some(4),
        residual_norm: Some(1.0e-12),
        multipliers: vec![
            PssFloquetMultiplierEvidence {
                multiplier: ComplexResultValue {
                    real: 0.5,
                    imaginary: 0.0,
                },
            },
            PssFloquetMultiplierEvidence {
                multiplier: ComplexResultValue {
                    real: 0.25,
                    imaginary: 0.0,
                },
            },
        ],
        floquet_evidence: FloquetSpectrumEvidence::Qualified {
            certificate: floquet_certificate(2),
        },
        orbit_kind: FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
    };
    let analysis = AnalysisResult::new(1, AnalysisType::Pss, "PSS")
        .with_waveforms(vec![waveform("display-only", vec![0.0], vec![1.0])])
        .with_result_payload(payload);

    let csv = prepare_typed_result_csv(&analysis).unwrap();

    assert_eq!(csv.default_name, "pss-floquet-evidence.csv");
    assert_eq!(
        csv.contents
            .lines()
            .filter(|line| line.starts_with("multiplier,"))
            .count(),
        2
    );
    assert!(
        csv.contents
            .contains("authenticated_complete_multiplier_count,2,count")
    );
    assert!(csv.contents.contains("certificate_problem_order,2,"));
    assert!(csv.contents.contains("floquet_evidence_json"));
    assert!(csv.contents.contains("qualified"));
}

#[test]
fn typed_csv_exports_complete_pstb_modes_and_all_stability_provenance() {
    let first = ComplexResultValue {
        real: 0.5,
        imaginary: 0.0,
    };
    let second = ComplexResultValue {
        real: 0.25,
        imaginary: 0.0,
    };
    let payload = AnalysisResultPayload::Pstb {
        period_s: Some(2.0),
        fundamental_frequency_hz: Some(0.5),
        stability_threshold: Some(1.0 + 1.0e-6),
        probe_instance: Some("LPROBE".to_owned()),
        detect_subharmonics: Some(false),
        modes: vec![
            PstbFloquetModeEvidence {
                multiplier: first,
                exponent: ComplexResultValue {
                    real: first.real.ln() / 2.0,
                    imaginary: 0.0,
                },
                probe_participation: 0.25,
                is_unstable: false,
                is_trivial: false,
                subharmonic_order: None,
            },
            PstbFloquetModeEvidence {
                multiplier: second,
                exponent: ComplexResultValue {
                    real: second.real.ln() / 2.0,
                    imaginary: 0.0,
                },
                probe_participation: 0.75,
                is_unstable: false,
                is_trivial: false,
                subharmonic_order: None,
            },
        ],
        floquet_evidence: FloquetSpectrumEvidence::Qualified {
            certificate: floquet_certificate(2),
        },
        orbit_kind: FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        stability_classification: PstbStabilityClassificationEvidence::Stable,
        min_stability_margin_db: Some(-20.0 * first.real.log10()),
        max_multiplier_magnitude: Some(first.real),
        num_unstable: Some(0),
        subharmonics: Vec::new(),
        converged: Some(true),
        iterations: Some(0),
    };
    let analysis = AnalysisResult::new(1, AnalysisType::Pstb, "PSTB")
        .with_waveforms(vec![waveform("bounded-display", vec![1.0], vec![0.5])])
        .with_result_payload(payload);

    let csv = prepare_typed_result_csv(&analysis).unwrap();

    assert_eq!(csv.default_name, "pstb-floquet-evidence.csv");
    assert_eq!(
        csv.contents
            .lines()
            .filter(|line| line.starts_with("mode,"))
            .count(),
        2,
        "display truncation must not truncate the authoritative mode export"
    );
    for field in [
        "stability_threshold",
        "probe_instance",
        "detect_subharmonics",
        "stability_verdict",
        "stability_classification",
        "min_stability_margin_db",
        "max_multiplier_magnitude",
        "num_unstable",
        "subharmonics",
        "certificate_problem_order",
    ] {
        assert!(csv.contents.contains(field), "missing {field}");
    }
    assert!(csv.contents.contains("LPROBE"));
    assert!(
        csv.contents
            .contains("authenticated_complete_mode_count,2,count")
    );
}

#[test]
fn typed_csv_rejects_payloads_that_contradict_their_retained_axis() {
    let analysis = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_family_metadata(AnalysisResultFamilyMetadata::Reliability { years: vec![1.0] })
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![ReliabilityDeviceEvidence {
                device_id: "M1".to_owned(),
                stress: ReliabilityStressEvidence {
                    average_gate_stress_v: 1.2,
                    average_drain_stress_v: 1.8,
                    average_temperature_k: 358.15,
                    duration_s: 3_600.0,
                },
                checkpoints: vec![ReliabilityCheckpointEvidence {
                    years: 10.0,
                    shift: ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.0125,
                        mobility_shift: -0.003,
                        drain_source_resistance_shift: 0.001,
                    },
                }],
            }],
        });
    let state = state_with_typed_result(analysis);

    assert!(prepare_typed_result_csv(state.simulation.active_analysis().unwrap()).is_none());
}

#[test]
fn csv_export_publishes_complete_soa_rules_and_exact_events() {
    let analysis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: vec![1.0e-6] })
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![SoaEvaluationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.2,
                worst_time_s: 1.0e-6,
                sample_count: 1,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.2,
                time_s: 1.0e-6,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        });
    let mut state = state_with_typed_result(analysis);
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("soa-evidence.csv"));
    assert!(files[0].1.contains("evaluation,M1,vds"));
    assert!(files[0].1.contains("Maximum drain-source voltage,warning"));
    assert!(files[0].1.contains("event,M1,vds"));
}

#[test]
fn bode_export_uses_the_displayed_frequency_analysis_not_the_global_selector() {
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
    let ac =
        AnalysisResult::new(2, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![waveform(
            "|V(out)|",
            vec![1.0e3, 1.0e4, 1.0e5],
            vec![0.1, 0.2, 0.3],
        )]);

    let mut run = SimulationRun::new(7);
    run.add_analysis(transient.clone());
    run.add_analysis(ac);

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    // Deliberately leave the simulation selector on TRAN while the Results
    // document displays Bode. Export authority is the document, not this
    // unrelated ordinal.
    state.simulation.active_analysis_idx = Some(0);
    state
        .simulation
        .replace_waveforms(transient.waveforms.clone());
    activate_result_document(&mut state, crate::workbench::ResultViewer::Bode);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    let datasets = io.datasets.borrow();
    assert_eq!(datasets.len(), 1);
    let dataset = &datasets[0];
    assert_eq!(
        dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
        Some("frequency")
    );
    assert_eq!(dataset.signal_names(), vec!["|V(out)|"]);
    assert_eq!(dataset.point_count(), 3);
    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .map(|signal| signal.data.as_slice()),
        Some(&[1.0e3, 1.0e4, 1.0e5][..])
    );
    assert!(dataset.get_signal("V(out)").is_none());
}

#[test]
fn waves_export_preserves_every_displayed_analysis_and_independent_axes() {
    let tran_a = AnalysisResult::new(1, AnalysisType::Transient, "TRAN A")
        .with_waveforms(vec![waveform("V(a)", vec![0.0, 1.0], vec![1.0, 2.0])]);
    let tran_b =
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN B").with_waveforms(vec![waveform(
            "V(b)",
            vec![10.0, 20.0, 30.0],
            vec![3.0, 4.0, 5.0],
        )]);
    let mut run = SimulationRun::new(7);
    run.add_analysis(tran_a);
    run.add_analysis(tran_b);
    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state, &io);

    assert!(io.datasets.borrow().is_empty());
    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("rspice-displayed-results.csv"));
    let csv = &files[0].1;
    assert!(csv.starts_with("dataset_id,analysis_sequence,analysis_label"));
    assert!(csv.contains(",1,TRAN A,TR,V(a),display,1,1.00000000000000000e0"));
    assert!(csv.contains(",2,TRAN B,TR,V(b),display,2,3.00000000000000000e1"));
}

#[test]
fn csv_export_preserves_single_analysis_axis_shape() {
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);

    let mut run = SimulationRun::new(7);
    run.add_analysis(transient.clone());

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    state
        .simulation
        .replace_waveforms(transient.waveforms.clone());
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    let datasets = io.datasets.borrow();
    assert_eq!(datasets.len(), 1);
    let dataset = &datasets[0];
    assert_eq!(
        dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
        Some("time")
    );
    assert_eq!(dataset.signal_names(), vec!["V(out)"]);
    assert_eq!(dataset.point_count(), 2);
}

#[test]
fn csv_export_uses_the_displayed_analysis_without_a_global_analysis_selector() {
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
    let mut run = SimulationRun::new(7);
    run.add_analysis(transient.clone());

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state
        .simulation
        .replace_waveforms(transient.waveforms.clone());
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert_eq!(io.datasets.borrow().len(), 1);
    assert_eq!(io.datasets.borrow()[0].signal_names(), vec!["V(out)"]);
}

#[test]
fn csv_export_fails_closed_when_the_active_analysis_has_no_samples() {
    let mut run = SimulationRun::new(7);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "Transient"));

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.datasets.borrow().is_empty());
    assert_eq!(last_log_message(&state), NO_SAMPLES_MESSAGE);
}

#[test]
fn csv_export_rejects_single_analysis_divergent_x_axes() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]),
            waveform("V(in)", vec![10.0, 20.0], vec![5.0, 6.0]),
        ]);

    let mut run = SimulationRun::new(7);
    run.add_analysis(transient);

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.datasets.borrow().is_empty());
    assert!(
        state
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains("different x-axis samples"))
    );
}

#[test]
fn csv_export_ignores_divergent_axes_owned_by_an_inactive_analysis() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0], vec![0.0, 1.0]),
            waveform("V(in)", vec![10.0, 20.0], vec![5.0, 6.0]),
        ]);
    let ac =
        AnalysisResult::new(2, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![waveform(
            "|V(out)|",
            vec![1.0e3, 1.0e4],
            vec![0.1, 0.2],
        )]);

    let mut run = SimulationRun::new(7);
    run.add_analysis(transient);
    run.add_analysis(ac);

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(1);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Bode);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    let datasets = io.datasets.borrow();
    assert_eq!(datasets.len(), 1);
    let dataset = &datasets[0];
    assert_eq!(
        dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
        Some("frequency")
    );
    assert_eq!(dataset.signal_names(), vec!["|V(out)|"]);
}

#[test]
fn csv_export_includes_complex_real_and_imaginary_columns() {
    let current_real = f64::from_bits(1);
    let current_imag = -f64::from_bits(2);
    let ac = AnalysisResult::new(1, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![
        complex_waveform(
            "|V(out)|",
            "V(out)",
            vec![1.0e3, 1.0e4],
            vec![1.0, 2.0],
            vec![0.8, 1.6],
            vec![0.6, 1.2],
        ),
        complex_waveform(
            "|I(V1)[sb=+0]|",
            "I(V1)[sb=+0]",
            vec![1.0e3, 1.0e4],
            vec![5.0e-4, current_real],
            vec![-5.0e-4, current_real],
            vec![0.0, current_imag],
        )
        .with_unit("A"),
    ]);

    let mut run = SimulationRun::new(7);
    run.add_analysis(ac);

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Bode);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    let datasets = io.datasets.borrow();
    assert_eq!(datasets.len(), 1);
    let dataset = &datasets[0];
    assert_eq!(
        dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
        Some("frequency")
    );
    assert_eq!(
        dataset.signal_names(),
        vec![
            "|V(out)|",
            "re(V(out))",
            "im(V(out))",
            "|I(V1)[sb=+0]|",
            "re(I(V1)[sb=+0])",
            "im(I(V1)[sb=+0])",
        ]
    );
    assert_eq!(
        dataset
            .get_signal("re(V(out))")
            .map(|signal| signal.data.as_slice()),
        Some(&[0.8, 1.6][..])
    );
    assert_eq!(
        dataset
            .get_signal("im(V(out))")
            .map(|signal| signal.data.as_slice()),
        Some(&[0.6, 1.2][..])
    );
    let current_real_signal = dataset
        .get_signal("re(I(V1)[sb=+0])")
        .expect("exact current real components are exported");
    let current_imag_signal = dataset
        .get_signal("im(I(V1)[sb=+0])")
        .expect("exact current imaginary components are exported");
    assert_eq!(current_real_signal.unit, "A");
    assert_eq!(current_imag_signal.unit, "A");
    assert_eq!(
        current_real_signal.data[1].to_bits(),
        current_real.to_bits()
    );
    assert_eq!(
        current_imag_signal.data[1].to_bits(),
        current_imag.to_bits()
    );
}

#[test]
fn engineering_export_preference_dispatches_compatible_touchstone() {
    let frequency = vec![1.0e6, 2.0e6];
    let waveforms = ["S11", "S12", "S21", "S22"]
        .into_iter()
        .enumerate()
        .map(|(index, name)| {
            complex_waveform(
                &format!("|{name}|"),
                name,
                frequency.clone(),
                vec![0.1 + index as f64 * 0.1, 0.2 + index as f64 * 0.1],
                vec![0.01 + index as f64 * 0.1, 0.02 + index as f64 * 0.1],
                vec![0.001 + index as f64 * 0.01, 0.002 + index as f64 * 0.01],
            )
        })
        .collect::<Vec<_>>();
    let ac = AnalysisResult::new(1, AnalysisType::SParameter, "S-parameters")
        .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm: vec![50.0, 75.0],
        })
        .with_waveforms(waveforms);
    let mut run = SimulationRun::new(7);
    run.add_analysis(ac);

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Smith);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 1)
        .unwrap();

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.datasets.borrow().is_empty());
    assert_eq!(io.dialog_titles.borrow().as_slice(), &["Export Touchstone"]);
    let text_files = io.text_files.borrow();
    assert_eq!(text_files.len(), 1);
    assert_eq!(text_files[0].0, PathBuf::from("waveforms.snp"));
    assert!(text_files[0].1.contains("[Version] 2.0"));
    assert!(text_files[0].1.contains("[Reference] 5e1 7.5e1"));
    assert!(text_files[0].1.contains("# Hz S RI R 50"));
    assert!(text_files[0].1.contains("1e6"));
}

#[test]
fn engineering_export_preference_dispatches_exact_tsv() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform(
                "V(out)",
                vec![0.0, 1.0e-6],
                vec![0.0, 1.234_567_890_123_456],
            ),
        ]);
    let mut state = state_with_typed_result(transient);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 2)
        .expect("TSV preference");
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert_eq!(
        io.dialog_titles.borrow().as_slice(),
        &["Export Waveform TSV"]
    );
    let files = io.text_files.borrow();
    assert_eq!(files[0].0, PathBuf::from("waveforms.tsv"));
    assert!(files[0].1.starts_with("time\tV(out)\n"));
    assert!(files[0].1.contains("1.234567890123456"));
}

#[test]
fn engineering_export_ui_publishes_reopenable_native_real_and_complex_bundles() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0e-6, 2.0e-6], vec![0.0, 1.25, -0.5]),
        ]);
    let mut result_state = state_with_typed_result(transient);
    result_state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 3)
        .expect("result bundle preference");
    let result_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut result_state, &result_io);
    assert_eq!(
        result_io.dialog_titles.borrow().as_slice(),
        &["Export RSpice Result Bundle"]
    );
    let result_files = result_io.byte_files.borrow();
    assert_eq!(result_files.len(), 1);
    assert_eq!(result_files[0].0, PathBuf::from("waveforms.rspiceresult"));
    assert_eq!(result_files[0].2, "application/vnd.rspice.result+zip");
    let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
        "waveforms.rspiceresult",
        &result_files[0].1,
    )
    .expect("reopen result bundle");
    assert_eq!(reopened.analysis_type, AnalysisType::Transient);
    assert_eq!(reopened.waveforms[0].name, "V(out)");
    assert_eq!(reopened.waveforms[0].y.as_ref(), &[0.0, 1.25, -0.5]);
    drop(result_files);

    let ac = AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![complex_waveform(
        "|V(out)|",
        "V(out)",
        vec![1.0e3, 2.0e3, 4.0e3],
        vec![1.0, 2.0, 0.5],
        vec![0.75, -1.5, 0.25],
        vec![0.25, 0.5, -0.125],
    )]);
    let mut dataset_state = state_with_typed_result(ac);
    dataset_state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 4)
        .expect("dataset bundle preference");
    let dataset_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut dataset_state, &dataset_io);
    assert_eq!(
        dataset_io.dialog_titles.borrow().as_slice(),
        &["Export RSpice Dataset Bundle"]
    );
    let dataset_files = dataset_io.byte_files.borrow();
    assert_eq!(dataset_files.len(), 1);
    assert_eq!(dataset_files[0].0, PathBuf::from("waveforms.rspicedata"));
    assert_eq!(dataset_files[0].2, "application/vnd.rspice.dataset+zip");
    let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
        "waveforms.rspicedata",
        &dataset_files[0].1,
    )
    .expect("reopen dataset bundle");
    assert_eq!(reopened.analysis_type, AnalysisType::Ac);
    assert_eq!(reopened.waveforms.len(), 1);
    // The retained identity remains `V(out)` in the complex payload. The
    // ordinary waveform name describes the magnitude trace the result viewer
    // materializes from those authoritative rectangular samples.
    assert_eq!(reopened.waveforms[0].name, "|V(out)|");
    let complex = reopened.waveforms[0]
        .complex
        .as_ref()
        .expect("complex components reopen");
    assert_eq!(complex.source_name, "V(out)");
    assert_eq!(complex.real.as_ref(), &[0.75, -1.5, 0.25]);
    assert_eq!(complex.imag.as_ref(), &[0.25, 0.5, -0.125]);
}

#[test]
fn flat_export_refuses_mismatched_sample_lengths_without_opening_a_picker() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0]),
        ]);
    let mut state = state_with_typed_result(transient);
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);
    assert!(io.dialog_titles.borrow().is_empty());
    let message = last_log_message(&state);
    assert!(
        message.contains("retained-evidence verification failed"),
        "{message}"
    );
    assert!(message.contains("coordinates"), "{message}");
}

#[test]
fn incompatible_touchstone_result_is_rejected_before_save_picker() {
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
    let mut run = SimulationRun::new(7);
    run.add_analysis(transient);

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 1)
        .unwrap();

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.text_files.borrow().is_empty());
    assert!(last_log_message(&state).contains("not compatible"));
}

#[test]
fn displayed_digits_never_reduce_csv_source_precision() {
    let exact = 1.234_567_890_123_456_f64;
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform("V(out)", vec![0.0], vec![exact])]);
    let mut run = SimulationRun::new(7);
    run.add_analysis(transient);

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
    state
        .ui
        .preferences
        .set_scalar(
            crate::workbench::ScalarPreference::DisplayedSignificantDigits,
            3,
        )
        .unwrap();

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    let datasets = io.datasets.borrow();
    assert_eq!(datasets[0].get_signal("V(out)").unwrap().data, vec![exact]);
}

#[test]
fn csv_export_reports_browser_download_start_without_claiming_file_written() {
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
    let mut run = SimulationRun::new(7);
    run.add_analysis(transient.clone());

    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    state
        .simulation
        .replace_waveforms(transient.waveforms.clone());
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

    let io = MockExportWorkflowIo {
        saved_paths_are_reopenable: false,
        ..Default::default()
    };
    action_export_csv_with_io(&mut state, &io);

    assert_eq!(
        last_log_message(&state),
        "CSV download started: waveforms.csv (1 signals, 2 points; confirm the browser accepted the download)"
    );
}

/// A run with two traces, one of them hidden by the reader, exported from a
/// view showing `analyses` analyses.
fn hidden_trace_state(analyses: usize) -> AppState {
    let mut run = SimulationRun::new(7);
    for index in 0..analyses {
        run.add_analysis(
            AnalysisResult::new(
                index as u64 + 1,
                AnalysisType::Transient,
                format!("Transient {index}"),
            )
            .with_waveforms(vec![
                waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2]),
                waveform("V(mid)", vec![0.0, 1.0e-6], vec![0.0, 0.6]),
            ]),
        );
    }
    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    let waveforms = state.simulation.runs[0].analyses[0].waveforms.clone();
    state.simulation.replace_waveforms(waveforms);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
    // Hide the second trace of every displayed analysis, exactly as a legend
    // chip or a navigator check-mark does.
    for index in 0..analyses {
        crate::workbench::documents::result_document::toggle_visibility(&mut state, index, 1);
    }
    state
}

/// Hiding a trace changes the export, whatever the view is showing.
///
/// The two routes disagreed. The long-form route read the dataset's own
/// `visible` flag and never saw the reader's override; the single-analysis
/// route read no visibility at all and wrote every retained trace. So hiding
/// a trace altered the file only when the viewer happened to be displaying
/// two analyses or more — and even then only if the flag had been written
/// into the dataset rather than into the session.
#[test]
fn a_hidden_trace_stays_out_of_the_export_for_one_analysis_and_for_many() {
    let mut single = hidden_trace_state(1);
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut single, &io);
    assert_eq!(io.datasets.borrow().len(), 1);
    assert_eq!(io.datasets.borrow()[0].signal_names(), vec!["V(out)"]);

    let mut stacked = hidden_trace_state(2);
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut stacked, &io);
    let written = io.text_files.borrow();
    let contents = &written.last().expect("long-form CSV is written").1;
    assert!(contents.contains(",V(out),display,"), "{contents}");
    assert!(!contents.contains(",V(mid),"), "{contents}");
}

/// Hiding every trace refuses the export and says so.
#[test]
fn an_export_with_every_trace_hidden_names_the_hidden_traces() {
    let mut state = hidden_trace_state(1);
    crate::workbench::documents::result_document::toggle_visibility(&mut state, 0, 0);

    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.datasets.borrow().is_empty());
    assert_eq!(last_log_message(&state), ALL_TRACES_HIDDEN_MESSAGE);
}

/// The operating-point export refuses, rather than handing back whatever the
/// payload router finds on the same analysis.
///
/// The OP sheet renders for any analysis retaining a DC solution, so a
/// transient carrying `dc_op` shows it. Its export returned `None` and the
/// chain below wrote that transient's *event history* under the menu item
/// the reader pressed on the operating-point sheet.
#[test]
fn an_operating_point_export_that_cannot_be_produced_states_why() {
    fn op_state(analysis: AnalysisResult) -> AppState {
        let mut run = SimulationRun::new(7);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Op);
        state
    }

    let mut failed = AnalysisResult::new(1, AnalysisType::DcOp, "Operating point");
    failed.dc_op = Some(dc_op_with_one_node());
    failed.success = false;
    failed.error_message = Some("gmin stepping did not converge".to_owned());
    let mut state = op_state(failed);
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);
    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.text_files.borrow().is_empty());
    let message = last_log_message(&state);
    assert!(
        message.contains("no analysis in 'Run 7") && message.contains("completed successfully"),
        "{message}"
    );
    assert!(
        message.contains("gmin stepping did not converge"),
        "{message}"
    );

    // A successful transient whose DC solution the OP sheet renders: the
    // sheet's export does not apply, and the event history must not stand in
    // for it.
    let mut transient = AnalysisResult::new(2, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
    transient.dc_op = Some(dc_op_with_one_node());
    let mut state = op_state(transient);
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);
    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.text_files.borrow().is_empty());
    let message = last_log_message(&state);
    assert!(
        message.contains("operating point cannot be exported"),
        "{message}"
    );
}

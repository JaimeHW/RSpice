//! SPICE rawfile export of a real run, read back by RSpice's own importer.

use super::*;
use rspice_app_runs::simulation::controller::test_execution::run_manual_batch;

const TRANSIENT_DECK: &str = "rc step
V1 in 0 PULSE(0 1 0 1n 1n 5u 10u)
R1 in out 1k
C1 out 0 1n
.tran 10n 2u
.end
";

const AC_DECK: &str = "rc lowpass
V1 in 0 AC 1
R1 in out 1k
C1 out 0 1n
.ac dec 5 1k 10meg
.end
";

/// Index of each encoding in the stored export choice.
const RAW_BINARY: usize = 10;
const RAW_ASCII: usize = 11;

fn state_with_run(deck: &str) -> ResultsTestHost {
    let run = run_manual_batch(deck);
    assert_eq!(run.analyses.len(), 1, "one analysis per fixture deck");
    state_with_typed_result(run.analyses[0].clone())
}

/// The file "Export dataset…" writes in `preference`, and its media type.
fn exported(state: &mut ResultsTestHost, preference: usize) -> (Vec<u8>, String) {
    state
        .model
        .preferences
        .set_choice(
            crate::workbench::ChoicePreference::EngineeringExport,
            preference,
        )
        .expect("a rawfile preference");
    let io = MockExportWorkflowIo::default();
    action_export_dataset_with_io(
        &mut state.results_parts(),
        &io,
        &DatasetExportChoice::default(),
    );
    assert_eq!(
        io.dialog_titles.borrow().as_slice(),
        &["Export SPICE Rawfile"],
        "{}",
        last_log_message(state)
    );
    let files = io.byte_files.borrow();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].0, PathBuf::from("waveforms.raw"));
    (files[0].1.clone(), files[0].2.clone())
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// A transient exported as a rawfile, binary or ASCII, reopens in RSpice with
/// every retained sample bit for bit, under its own name and unit.
#[test]
fn a_transient_rawfile_reopens_with_every_sample_exact() {
    let mut state = state_with_run(TRANSIENT_DECK);
    let retained = state.model.simulation.retained.runs[0].analyses[0]
        .waveforms
        .clone();
    assert!(retained.iter().any(|waveform| waveform.name == "OUT"));
    for (preference, mime) in [
        (RAW_BINARY, "application/octet-stream"),
        (RAW_ASCII, "text/plain;charset=utf-8"),
    ] {
        let (bytes, media_type) = exported(&mut state, preference);
        assert_eq!(media_type, mime);
        assert!(bytes.starts_with(b"Title: "), "a rawfile header");
        let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
            "waveforms.raw",
            &bytes,
        )
        .expect("RSpice reads its own rawfile");
        assert_eq!(reopened.analysis_type, AnalysisType::Transient);
        assert_eq!(reopened.coordinate_unit.as_deref(), Some("s"));
        assert_eq!(reopened.waveforms.len(), retained.len());
        for (back, original) in reopened.waveforms.iter().zip(&retained) {
            assert_eq!(back.name, original.name);
            assert_eq!(back.unit, original.unit, "{}", original.name);
            assert_eq!(bits(&back.x), bits(&original.x), "{}", original.name);
            assert_eq!(bits(&back.y), bits(&original.y), "{}", original.name);
        }
    }
}

/// An AC sweep reopens with its retained rectangular samples exact: the
/// rawfile is a complex plot of them, as ngspice writes one.
#[test]
fn an_ac_rawfile_reopens_with_its_complex_samples_exact() {
    let mut state = state_with_run(AC_DECK);
    let retained = state.model.simulation.retained.runs[0].analyses[0]
        .waveforms
        .clone();
    let out = retained
        .iter()
        .find_map(|waveform| {
            waveform
                .complex
                .as_ref()
                .filter(|complex| complex.source_name == "V(OUT)")
        })
        .expect("the AC run retains V(OUT) as complex samples");
    for preference in [RAW_BINARY, RAW_ASCII] {
        let (bytes, _) = exported(&mut state, preference);
        assert!(
            bytes
                .windows(b"Flags: complex".len())
                .any(|window| window == b"Flags: complex")
        );
        let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
            "waveforms.raw",
            &bytes,
        )
        .expect("RSpice reads its own rawfile");
        assert_eq!(reopened.analysis_type, AnalysisType::Ac);
        let back = reopened
            .waveforms
            .iter()
            .find_map(|waveform| {
                waveform
                    .complex
                    .as_ref()
                    .filter(|complex| complex.source_name == "V(OUT)")
            })
            .expect("V(OUT) reopens complex");
        assert_eq!(bits(&back.real), bits(&out.real));
        assert_eq!(bits(&back.imag), bits(&out.imag));
    }
}

/// A result the rawfile has no plot for is refused by name before a
/// destination is asked for.
#[test]
fn an_operating_point_is_refused_before_the_file_picker() {
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::DcOp, "Operating point")
            .with_dc_op(dc_op_with_one_node()),
    );
    state
        .model
        .preferences
        .set_choice(
            crate::workbench::ChoicePreference::EngineeringExport,
            RAW_BINARY,
        )
        .unwrap();
    let io = MockExportWorkflowIo::default();
    action_export_dataset_with_io(
        &mut state.results_parts(),
        &io,
        &DatasetExportChoice::default(),
    );
    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.byte_files.borrow().is_empty());
    let message = refusal(&state);
    assert!(
        message.contains("SPICE rawfile") || message.contains("samples"),
        "{message}"
    );
}

/// A transient with two analog traces and event histories: `q1`
/// drawn as a lane through its visible `D(q1)` column, `q0` and the real node
/// `level` kept only as event history, and the bus `q` over both bits.
fn mixed_transient() -> AnalysisResult {
    use crate::state::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
    };
    let digital = |node: &str, points: &[(f64, u8)]| DigitalEventTraceEvidence {
        node_name: node.to_owned(),
        points: points
            .iter()
            .map(|&(time_s, value_code)| DigitalEventPointEvidence { time_s, value_code })
            .collect(),
    };
    AnalysisResult::new(1, AnalysisType::Transient, "Mixed")
        .with_waveforms(vec![
            waveform("V(a)", vec![0.0, 1e-9], vec![0.0, 1.8]),
            waveform("D(q1)", vec![0.0, 1e-9], vec![0.0, 1.0]),
            waveform("V(b)", vec![0.0, 1e-9], vec![0.25, 0.5]),
        ])
        .with_result_payload(AnalysisResultPayload::TransientEvents {
            current_impulses: None,
            voltage_impulses: None,
            digital_traces: vec![
                digital("q1", &[(0.0, 0), (5e-10, 1)]),
                digital("q0", &[(0.0, 1), (7e-10, 0)]),
            ],
            real_traces: vec![RealEventTraceEvidence {
                node_name: "level".to_owned(),
                points: vec![RealEventPointEvidence {
                    time_s: 2e-10,
                    value: 0.75,
                }],
            }],
            digital_buses: vec![DigitalBusEvidence {
                name: "q".to_owned(),
                msb: 1,
                lsb: 0,
                members: vec!["q1".to_owned(), "q0".to_owned()],
                source: DigitalBusSourceEvidence::Engine,
            }],
        })
}

/// The rawfile follows the form's signal choice for traces and event lanes
/// alike: "Plotted" writes the visible traces and the lanes on the plot, "All"
/// every trace and every event node with its bus. Each file imports back into
/// RSpice as one transient carrying exactly the event histories it wrote.
#[test]
fn a_rawfile_follows_the_signal_choice_and_reimports_with_its_event_lanes() {
    use crate::workbench::workflows::result_import_workflow::{
        commit_result_import_draft, stage_imported_result_dataset,
    };
    for (signals, traces, digital, real, buses) in [
        (
            ExportedSignals::Plotted,
            &["V(a)", "D(q1)"][..],
            &["q1"][..],
            &[][..],
            &[][..],
        ),
        (
            ExportedSignals::All,
            &["V(a)", "D(q1)", "V(b)"][..],
            &["q1", "q0"][..],
            &["level"][..],
            &["q"][..],
        ),
    ] {
        let mut state = state_with_typed_result(mixed_transient());
        // The reader hides V(b).
        crate::workbench::documents::result_document::toggle_visibility(
            &mut state.results_parts(),
            0,
            2,
        );
        state
            .model
            .preferences
            .set_choice(
                crate::workbench::ChoicePreference::EngineeringExport,
                RAW_BINARY,
            )
            .unwrap();
        let io = MockExportWorkflowIo::default();
        let choice = DatasetExportChoice {
            signals,
            ..DatasetExportChoice::default()
        };
        action_export_dataset_with_io(&mut state.results_parts(), &io, &choice);
        let bytes = {
            let files = io.byte_files.borrow();
            assert_eq!(files.len(), 1, "{signals:?}: {}", last_log_message(&state));
            files[0].1.clone()
        };

        stage_imported_result_dataset(&mut state.results_parts(), "waveforms.raw", &bytes)
            .unwrap_or_else(|error| panic!("{signals:?}: {error}"));
        assert!(state.results.result_import.plots.is_none(), "one analysis");
        commit_result_import_draft(&mut state.results_parts()).unwrap();
        let run = state
            .model
            .simulation
            .active_run()
            .expect("the import is shown");
        assert_eq!(run.label, "Imported · waveforms.raw");
        let [analysis] = run.analyses.as_slice() else {
            panic!("{signals:?}: one analysis, got {}", run.analyses.len());
        };
        assert_eq!(analysis.analysis_type, AnalysisType::Transient);
        let names = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, traces, "{signals:?}");
        let Some(AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            digital_buses,
            ..
        }) = &analysis.result_payload
        else {
            panic!("{signals:?}: the event lanes import as event history");
        };
        let digital_names = digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect::<Vec<_>>();
        let real_names = real_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect::<Vec<_>>();
        let bus_names = digital_buses
            .iter()
            .map(|bus| bus.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            (
                digital_names.as_slice(),
                real_names.as_slice(),
                bus_names.as_slice()
            ),
            (digital, real, buses),
            "{signals:?}"
        );
        // The histories come back change for change, strength and all.
        let original = mixed_transient();
        let Some(AnalysisResultPayload::TransientEvents {
            digital_traces: written,
            ..
        }) = &original.result_payload
        else {
            unreachable!()
        };
        for trace in digital_traces {
            let source = written
                .iter()
                .find(|source| source.node_name == trace.node_name)
                .unwrap();
            assert_eq!(trace.points, source.points, "{}", trace.node_name);
        }
    }
}

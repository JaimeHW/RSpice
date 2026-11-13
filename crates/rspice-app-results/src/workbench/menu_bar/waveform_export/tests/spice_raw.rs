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
    action_export_csv_with_io(&mut state.results_parts(), &io);
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
    action_export_csv_with_io(&mut state.results_parts(), &io);
    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.byte_files.borrow().is_empty());
    let message = refusal(&state);
    assert!(
        message.contains("SPICE rawfile") || message.contains("samples"),
        "{message}"
    );
}

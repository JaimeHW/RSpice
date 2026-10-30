//! Result imports reviewed, committed and carried through a project reload
//! by a whole application.

use super::result_import_adapters::apply_imported_result_dataset;
use crate::io::file_exchange::MAX_RESULT_DATASET_BYTES;
use crate::state::AnalysisResultFamilyMetadata;
use crate::state::AnalysisType;
use crate::state::SimulationRunLifecycle;
use crate::state::SimulationRunProvenance;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::import::ResultImportStage;
use crate::workbench::workflows::result_import_workflow::*;

fn loaded_project_state() -> AppState {
    let mut state = AppState::default();
    let baseline =
        crate::workbench::lifecycle::project_lifecycle::snapshot(&state).expect("project snapshot");
    crate::workbench::app_state::services_access::accept_loaded_project(&mut state, baseline, None);
    state
}

fn assert_nullable_import_round_trip(name: &str, bytes: &[u8]) {
    let mut state = loaded_project_state();
    stage_imported_result_dataset(&mut state.results_parts(), name, bytes).unwrap();
    assert_eq!(state.results.result_import.waveforms.len(), 1);
    assert_eq!(state.results.result_import.waveforms[0].sample(1), None);
    commit_result_import_draft(&mut state.results_parts()).unwrap();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let project = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(
        project.file.simulation_results_warning.is_none(),
        "{:?}",
        project.file.simulation_results_warning
    );
    let simulation =
        crate::io::simulation_state_from_results(project.file.simulation_results).unwrap();
    let restored = &simulation.active_run().unwrap().analyses[0];
    restored.validate_retained_evidence().unwrap();
    assert_eq!(restored.waveforms[0].x.as_ref(), &[0.0, 1.0, 2.0]);
    assert_eq!(restored.waveforms[0].sample(1), None);
    assert_eq!(restored.waveforms[0].y[2].to_bits(), (-0.0_f64).to_bits());
    let projected = rspice_formats::waveform_io::result::project_waveforms(
        restored,
        &[&restored.waveforms[0]],
        false,
    )
    .unwrap();
    let csv = rspice_formats::WaveformWriter::new(rspice_formats::WaveformFormat::Csv)
        .write_text(&projected)
        .unwrap();
    let parsed = parse_result_dataset("gaps.csv", csv.as_bytes()).unwrap();
    assert_eq!(parsed.waveforms[0].sample(1), None);
    assert_eq!(parsed.waveforms[0].y[2].to_bits(), (-0.0_f64).to_bits());
    use rspice_formats::native_bundle::{
        NativeBundleKind, encode_native_bundle, result::project_native_bundle,
    };
    let native = project_native_bundle(
        restored,
        &[&restored.waveforms[0]],
        rspice_formats::WaveformDomain::Transient,
    )
    .unwrap();
    let bytes =
        encode_native_bundle(NativeBundleKind::Dataset, &native, MAX_RESULT_DATASET_BYTES).unwrap();
    let parsed = parse_result_dataset("gaps.rspicedata", &bytes).unwrap();
    assert_eq!(parsed.waveforms[0].sample(1), None);
    assert_eq!(parsed.waveforms[0].unit.as_deref(), Some("V"));
}

#[test]
fn imported_angles_and_temperatures_retain_normalized_values_and_units() {
    let csv = b"time [s],phase [rad],angle [deg],ambient [degC],absolute [K],legacy [C],ratio [%],gain [dB],unstated\n0,3.141592653589793,-90,25,25,-273.15,12.5,-20,7\n";
    let parsed = parse_result_dataset("units.csv", csv).unwrap();
    let expected = [
        (180.0, Some("°")),
        (-90.0, Some("°")),
        (298.15, Some("K")),
        (25.0, Some("K")),
        (0.0, Some("K")),
        (0.125, Some("1")),
        (-20.0, Some("dB")),
        (7.0, None),
    ];
    for (waveform, (value, unit)) in parsed.waveforms.iter().zip(expected) {
        assert_eq!(waveform.y.as_slice(), &[value], "{}", waveform.name);
        assert_eq!(waveform.unit.as_deref(), unit, "{}", waveform.name);
    }
    let mut state = loaded_project_state();
    apply_imported_result_dataset(&mut state.results_parts(), "units.csv", csv).unwrap();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let project = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(project.file.simulation_results_warning.is_none());
    let simulation =
        crate::io::simulation_state_from_results(project.file.simulation_results).unwrap();
    let restored = &simulation.active_run().unwrap().analyses[0];
    restored.validate_retained_evidence().unwrap();
    for (before, after) in parsed.waveforms.iter().zip(&restored.waveforms) {
        assert_eq!(after.name, before.name);
        assert_eq!(after.y, before.y);
        assert_eq!(after.unit, before.unit);
    }
    let sweep = parse_result_dataset(
        "temperature.csv",
        b"temperature [degC],V(out) [V]\n-273.15,0\n25,1\n",
    )
    .unwrap();
    assert_eq!(sweep.analysis_type, AnalysisType::DcSweep);
    assert_eq!(sweep.waveforms[0].x.as_slice(), &[0.0, 298.15]);
    assert_eq!(sweep.coordinate_unit.as_deref(), Some("K"));

    let error =
        parse_result_dataset("overflow.csv", b"time [s],angle [rad]\n0,1e308\n").unwrap_err();
    assert!(error.contains("angle") && error.contains("overflows after unit conversion"));
}

#[test]
fn raw_coordinate_units_survive_import_review_project_reload_and_waveform_projection() {
    for (name, kind, unit, canonical, end) in [
        ("time", "time", "ns", "s", 1e-9),
        ("bias", "current", "mA", "A", 1e-3),
        ("parameter", "value", "widgets", "widgets", 1.0),
    ] {
        let mut bytes = b"Title: axis\nPlotname: axis\n".to_vec();
        rspice_core::io::ltspice_raw::write_raw_table_layout_metadata(
            &mut bytes,
            &[],
            &[Some(unit.to_owned()), Some("V".to_owned())],
            None,
        )
        .unwrap();
        bytes.extend_from_slice(format!("Flags: real\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 {name} {kind}\n1 V(out) voltage\nValues:\n0 0 3\n1 1 4\n").as_bytes());
        let mut state = loaded_project_state();
        stage_imported_result_dataset(&mut state.results_parts(), "axis.raw", &bytes).unwrap();
        assert_eq!(
            state.results.result_import.coordinate_unit.as_deref(),
            Some(canonical)
        );
        assert_eq!(
            state.results.result_import.waveforms[0].x.as_slice(),
            &[0.0, end]
        );
        commit_result_import_draft(&mut state.results_parts()).unwrap();
        let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
        let text = crate::io::project_io::serialize_project_file(&project).unwrap();
        let project = crate::io::project_io::load_project_text(&text, None).unwrap();
        assert!(project.file.simulation_results_warning.is_none());
        let simulation =
            crate::io::simulation_state_from_results(project.file.simulation_results).unwrap();
        let restored = &simulation.active_run().unwrap().analyses[0];
        let coordinate = restored.imported_coordinate().unwrap();
        assert_eq!(coordinate.name, name);
        assert_eq!(coordinate.unit.as_deref(), Some(canonical));
        assert_eq!(restored.waveforms[0].x.as_slice(), &[0.0, end]);
        let projected = rspice_formats::waveform_io::result::project_waveforms(
            restored,
            &[&restored.waveforms[0]],
            false,
        )
        .unwrap();
        let axis = projected.x_signal.unwrap();
        assert_eq!(axis.name, name);
        assert_eq!(axis.unit, canonical);
        assert_eq!(axis.data, [0.0, end]);
        let domain = if restored.analysis_type == AnalysisType::Transient {
            rspice_formats::WaveformDomain::Transient
        } else {
            rspice_formats::WaveformDomain::DcSweep
        };
        use rspice_formats::native_bundle::{
            NativeBundleKind, NativeBundleReadLimits, decode_native_bundle, encode_native_bundle,
            result::project_native_bundle,
        };
        let bundle = project_native_bundle(restored, &[&restored.waveforms[0]], domain).unwrap();
        let bytes =
            encode_native_bundle(NativeBundleKind::Dataset, &bundle, MAX_RESULT_DATASET_BYTES)
                .unwrap();
        let reimported = decode_native_bundle(
            &bytes,
            NativeBundleKind::Dataset,
            NativeBundleReadLimits {
                max_members: 2,
                max_expanded_bytes: MAX_RESULT_DATASET_BYTES,
                max_member_bytes: MAX_RESULT_DATASET_BYTES,
                max_rows: MAX_RESULT_ROWS,
                max_columns: MAX_RESULT_COLUMNS,
                max_numeric_values: MAX_RESULT_VALUES,
            },
        )
        .unwrap();
        assert_eq!(reimported.coordinate_name, name);
        assert_eq!(reimported.coordinate_unit.as_deref(), Some(canonical));
        assert_eq!(reimported.coordinate, [0.0, end]);
    }
}

#[test]
fn nullable_raw_survives_review_retention_project_reload_and_reexport() {
    let mut bytes = b"Title: gaps\nPlotname: Transient Analysis\n".to_vec();
    rspice_core::io::ltspice_raw::write_raw_table_layout_metadata(
        &mut bytes,
        &[],
        &[Some("s".into()), Some("V".into()), Some("1".into())],
        None,
    )
    .unwrap();
    bytes.extend_from_slice(b"Flags: real\nNo. Variables: 3\nNo. Points: 3\nVariables:\n0 time time\n1 V(out) nullable_real:voltage\n2 Valid(V(out)) nullable_validity:voltage\nValues:\n0 0 1 1\n1 1 0 0\n2 2 -0 1\n");
    assert_nullable_import_round_trip("gaps.raw", &bytes);
}

#[test]
fn nullable_hdf5_survives_review_retention_project_reload_and_reexport() {
    use rustyhdf5::{AttrValue, FileBuilder};
    let mut file = FileBuilder::new();
    let mut group = file.create_group("converted");
    for (key, value) in [
        ("section_type", "table"),
        ("coordinate_type", "time"),
        ("coordinate_unit", "ns"),
        ("independent_name", "time"),
        ("signal_0000_name", "V(out)"),
        ("signal_0000_type", "nullable_real:voltage"),
        ("signal_0000_unit", "V"),
        ("signal_0001_name", "Valid(V(out))"),
        ("signal_0001_type", "nullable_validity:voltage"),
        ("signal_0001_unit", "1"),
    ] {
        group.set_attr(key, AttrValue::String(value.to_owned()));
    }
    group.set_attr("signal_count", AttrValue::I64(2));
    group
        .create_dataset("independent")
        .with_f64_data(&[0.0, 1e9, 2e9]);
    group
        .create_dataset("signal_0000")
        .with_f64_data(&[1.0, 0.0, -0.0]);
    group
        .create_dataset("signal_0001")
        .with_f64_data(&[1.0, 0.0, 1.0]);
    file.add_group(group.finish());
    assert_nullable_import_round_trip("gaps.h5", &file.finish().unwrap());
}

#[test]
fn delimited_metadata_survives_review_project_reload_and_reexport() {
    use rspice_formats::{
        SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter,
    };
    let mut dataset = WaveformDataset::new("Exact units");
    let mut axis = WaveformSignal::new("time", SignalType::Time);
    axis.unit = "ns".into();
    axis.data = vec![0.0, 2.0, 4.0];
    dataset.set_x(axis);
    for (name, unit, values) in [
        ("charge [C]", "C", vec![1.0, f64::NAN, -0.0]),
        ("V(unstated)", "", vec![2.0, 3.0, 4.0]),
    ] {
        let mut signal = WaveformSignal::new(name, SignalType::Unknown);
        signal.unit = unit.into();
        signal.data = values;
        dataset.add_signal(signal);
    }
    let writer = WaveformWriter::new(WaveformFormat::Csv);
    let text = writer.write_text(&dataset).unwrap();
    let mut state = loaded_project_state();
    apply_imported_result_dataset(&mut state.results_parts(), "units.csv", text.as_bytes())
        .unwrap();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let project = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(project.file.simulation_results_warning.is_none());
    let simulation =
        crate::io::simulation_state_from_results(project.file.simulation_results).unwrap();
    let restored = &simulation.active_run().unwrap().analyses[0];
    restored.validate_retained_evidence().unwrap();
    let projected = rspice_formats::waveform_io::result::project_waveforms(
        restored,
        &[&restored.waveforms[0], &restored.waveforms[1]],
        false,
    )
    .unwrap();
    assert_eq!(
        projected.signals[1].unit, "",
        "an unstated unit cannot be inferred from V(...) in its name"
    );
    let text = writer.write_text(&projected).unwrap();
    let parsed = parse_result_dataset("restored.csv", text.as_bytes()).unwrap();
    assert_eq!(parsed.coordinate_unit.as_deref(), Some("s"));
    assert_eq!(parsed.waveforms[0].x.as_ref(), &[0.0, 2e-9, 4e-9]);
    assert_eq!(parsed.waveforms[0].name, "charge [C]");
    assert_eq!(parsed.waveforms[0].unit.as_deref(), Some("C"));
    assert_eq!(parsed.waveforms[0].sample(0), Some(1.0));
    assert_eq!(parsed.waveforms[0].sample(1), None);
    assert_eq!(parsed.waveforms[0].y[2].to_bits(), (-0.0_f64).to_bits());
    assert_eq!(parsed.waveforms[1].unit, None);
}

#[test]
fn import_review_cannot_reinterpret_a_current_coordinate_as_time() {
    let mut state = loaded_project_state();
    stage_imported_result_dataset(
        &mut state.results_parts(),
        "bias.csv",
        b"bias [mA],V(out) [V]\n0,0\n1,2\n",
    )
    .unwrap();
    state.results.result_import.analysis_type = AnalysisType::Transient;
    let error = commit_result_import_draft(&mut state.results_parts()).unwrap_err();
    assert!(error.contains("incompatible"), "{error}");
    assert!(state.model.simulation.retained.runs.is_empty());
    assert!(state.results.result_import.open);
}

#[test]
fn touchstone_v2_import_retains_complex_matrix_and_reference_authority() {
    let mut state = loaded_project_state();
    apply_imported_result_dataset(
        &mut state.results_parts(),
        "network.ts",
        b"[Version] 2.0\n[Number of Ports] 2\n[Number of Frequencies] 2\n# MHz S RI R 50\n[Reference] 50 75\n[Two-Port Data Order] 21_12\n[Network Data]\n1 0.1 0 0.2 0 0.3 0 0.4 0\n2 0.5 0 0.6 0 0.7 0 0.8 0\n[End]\n",
    )
    .expect("Touchstone import succeeds");

    let analysis = state
        .model
        .simulation
        .active_analysis()
        .expect("active import");
    assert_eq!(analysis.analysis_type, AnalysisType::SParameter);
    assert_eq!(analysis.waveforms.len(), 4);
    assert_eq!(analysis.waveforms[0].x.as_slice(), &[1.0e6, 2.0e6]);
    assert!(analysis.waveforms.iter().all(|waveform| {
        waveform.name.starts_with("|S")
            && waveform
                .complex
                .as_ref()
                .is_some_and(|complex| complex.real.len() == 2 && complex.imag.len() == 2)
    }));
    assert_eq!(
        analysis.family_metadata,
        Some(AnalysisResultFamilyMetadata::SParameter {
            noise_reference_temperature_kelvin: None,
            reference_impedances_ohm: vec![50.0, 75.0],
        })
    );
    assert!(analysis.validate_retained_evidence().is_ok());
    assert_eq!(
        state.results.results.session.viewer,
        crate::workbench::ResultViewer::Smith
    );
    assert_eq!(
        state.results.result_import.source_format_id, "",
        "successful commit discards the runtime import draft"
    );
}

#[test]
fn touchstone_noise_import_commits_independent_axes_and_persists_units() {
    let mut state = loaded_project_state();
    apply_imported_result_dataset(&mut state.results_parts(), "noise.ts", b"[Version] 2.0\n# MHz S RI R 75\n[Number of Ports] 2\n[Two-Port Data Order] 21_12\n[Number of Frequencies] 2\n[Number of Noise Frequencies] 1\n[Reference] 25 100\n[Network Data]\n1 0 0 1 0 0 0 0 0\n3 0 0 1 0 0 0 0 0\n[Noise Data]\n2 3 0 0 15\n[End]\n").unwrap();
    let analysis = &state
        .model
        .simulation
        .retained
        .runs
        .last()
        .unwrap()
        .analyses[0];
    analysis.validate_retained_evidence().unwrap();
    assert_eq!(analysis.waveforms.len(), 7);
    let rn = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "Rn")
        .unwrap();
    assert_eq!(rn.x.as_slice(), [2e6]);
    assert_eq!(rn.y.as_slice(), [15.0]);
    assert_eq!(rn.unit.as_deref(), Some("Ω"));
    let gamma = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "|Sopt|")
        .unwrap();
    assert!((gamma.complex.as_ref().unwrap().real[0] - 0.5).abs() < 1e-14);
    let network = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "|S21|")
        .unwrap();
    assert_eq!(network.x.as_slice(), [1e6, 3e6]);
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let project = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(project.file.simulation_results_warning.is_none());
    let simulation =
        crate::io::simulation_state_from_results(project.file.simulation_results).unwrap();
    let restored = &simulation.active_run().unwrap().analyses[0];
    restored.validate_retained_evidence().unwrap();
    assert_eq!(
        restored.family_metadata,
        Some(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm: vec![25.0, 100.0],
            noise_reference_temperature_kelvin: Some(290.0),
        })
    );
    assert_eq!(
        restored
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "Rn")
            .unwrap()
            .x,
        rn.x
    );
}

#[test]
fn exhausted_run_sequence_retains_the_import_draft_and_existing_dataset() {
    let mut state = loaded_project_state();
    state.model.simulation.retained.next_run_id = u64::MAX - 1;
    apply_imported_result_dataset(
        &mut state.results_parts(),
        "last.csv",
        b"time [s],V(out) [V]\n0,0\n1,1\n",
    )
    .expect("the last sequence can be imported");
    assert_eq!(state.model.simulation.active_run().unwrap().id, u64::MAX);
    let baseline = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let workspace = state.workbench.workspace;
    let document = state.workbench.documents.active(workspace).cloned();
    stage_imported_result_dataset(
        &mut state.results_parts(),
        "next.csv",
        b"time [s],V(out) [V]\n0,2\n1,3\n",
    )
    .expect("reviewing a draft requires no sequence");
    let selected = state.results.result_import.selected_signals.clone();

    for _ in 0..2 {
        let error = commit_result_import_draft(&mut state.results_parts()).unwrap_err();
        assert!(error.contains("run sequence is exhausted"));
        assert!(state.results.result_import.open);
        assert_eq!(state.results.result_import.source_name, "next.csv");
        assert_eq!(state.results.result_import.selected_signals, selected);
        assert_eq!(state.workbench.workspace, workspace);
        assert_eq!(
            state.workbench.documents.active(workspace),
            document.as_ref()
        );
        assert_eq!(
            crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
                .unwrap()
                .file
                .simulation_results,
            baseline.file.simulation_results
        );
    }
}

#[test]
fn imported_dataset_is_unattributed_stable_immutable_and_saveable() {
    let mut state = loaded_project_state();
    assert!(
        !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
        "accepted baseline starts clean"
    );
    apply_imported_result_dataset(
        &mut state.results_parts(),
        "external.csv",
        b"time [s],V(out) [V]\n0,0\n1,1\n",
    )
    .expect("import succeeds");

    let run = state
        .model
        .simulation
        .active_run()
        .expect("imported run selected");
    assert_eq!(run.lifecycle, SimulationRunLifecycle::LegacyUnknown);
    assert_eq!(run.elapsed_time, 0.0);
    assert!(run.job_id.is_none());
    assert!(run.execution_target.is_none());
    assert!(matches!(
        run.provenance(),
        Some(SimulationRunProvenance::LegacyUnattributed)
    ));
    assert_eq!(run.analyses.len(), 1);
    assert_eq!(
        state
            .model
            .simulation
            .active_analysis()
            .map(|analysis| analysis.analysis_type),
        Some(AnalysisType::Transient)
    );
    assert_ne!(run.run_id.to_string(), run.dataset_id.to_string());
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Results
    );
    assert_eq!(
        state
            .workbench
            .documents
            .active(crate::workbench::state::Workspace::Results),
        Some(&crate::workbench::state::WorkspaceDocumentId::ResultDataset(run.dataset_id))
    );
    assert!(
        crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
        "retaining external evidence must dirty canonical result history"
    );
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project)
        .expect("imported evidence must be saveable");
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(restored.file.simulation_results_warning.is_none());
    assert_eq!(
        restored.file.simulation_results,
        project.file.simulation_results
    );
    let restored =
        crate::io::simulation_state_from_results(restored.file.simulation_results).unwrap();
    let run = restored.active_run().unwrap();
    assert_eq!(run.lifecycle, SimulationRunLifecycle::LegacyUnknown);
    assert_eq!(run.elapsed_time, 0.0);
    assert!(run.execution_identity().is_none());
    assert!(run.execution_target.is_none());
    assert!(run.prepared_receipt().is_none());
    assert_eq!(run.analyses[0].waveforms[0].y.as_slice(), [0.0, 1.0]);
}

#[test]
fn event_source_import_format_and_name_survive_project_publication() {
    let mut state = loaded_project_state();
    apply_imported_result_dataset(&mut state.results_parts(), "capture.vcd",
        b"$timescale 1ns $end\n$scope module top $end\n$var wire 1 ! clk $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\n#1\n1!\n")
        .unwrap();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(restored.file.simulation_results_warning.is_none());
    let json = serde_json::to_value(&restored.file.simulation_results).unwrap();
    let source = &json["runs"][0]["analyses"][0]["import_source"];
    assert_eq!(source["format"], "vcd");
    assert_eq!(source["source_name"], "capture.vcd");
}

#[test]
fn staging_and_discarding_import_never_mutates_retained_history() {
    let mut state = loaded_project_state();
    let initial_run_count = state.model.simulation.retained.runs.len();
    let initial_workspace = state.workbench.workspace;

    stage_imported_result_dataset(
        &mut state.results_parts(),
        "external.csv",
        b"time [s],V(out) [V]\n0,0\n1,1\n",
    )
    .expect("stage succeeds");

    assert!(state.results.result_import.open);
    assert_eq!(state.results.result_import.stage, ResultImportStage::Detect);
    assert_eq!(
        state.model.simulation.retained.runs.len(),
        initial_run_count
    );
    assert_eq!(state.workbench.workspace, initial_workspace);
    assert!(
        !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
        "a review draft is presentation state, not project history"
    );

    discard_result_import_draft(&mut state.results_parts());
    assert!(!state.results.result_import.open);
    assert_eq!(
        state.model.simulation.retained.runs.len(),
        initial_run_count
    );
    assert_eq!(state.workbench.workspace, initial_workspace);
    assert!(!crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state));
}

#[test]
fn validation_failure_retains_draft_and_leaves_history_unchanged() {
    let mut state = loaded_project_state();
    stage_imported_result_dataset(
        &mut state.results_parts(),
        "external.csv",
        b"time [s],V(out) [V]\n0,0\n1,1\n",
    )
    .expect("stage succeeds");
    state.results.result_import.selected_signals.fill(false);

    let error =
        commit_result_import_draft(&mut state.results_parts()).expect_err("empty mapping rejected");
    assert!(error.contains("at least one signal"), "{error}");
    assert!(state.results.result_import.open);
    assert!(state.model.simulation.retained.runs.is_empty());
    assert!(!crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state));
}

#[test]
fn commit_imports_only_explicitly_selected_signals() {
    let mut state = loaded_project_state();
    stage_imported_result_dataset(
        &mut state.results_parts(),
        "external.csv",
        b"time [s],V(out) [V],I(VDD) [A]\n0,0,1\n1,1,2\n",
    )
    .expect("stage succeeds");
    state.results.result_import.selected_signals[1] = false;

    commit_result_import_draft(&mut state.results_parts()).expect("commit succeeds");

    let analysis = state
        .model
        .simulation
        .active_analysis()
        .expect("active import");
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].name, "V(out)");
    assert!(!state.results.result_import.open);
}

#[test]
fn remapping_zero_coordinate_data_to_ac_is_rejected_before_commit() {
    let mut state = loaded_project_state();
    stage_imported_result_dataset(
        &mut state.results_parts(),
        "external.csv",
        b"time,V(out) [V]\n0,0\n1,1\n",
    )
    .expect("stage succeeds");
    state.results.result_import.analysis_type = AnalysisType::Ac;

    let error = commit_result_import_draft(&mut state.results_parts())
        .expect_err("invalid frequency rejected");
    assert!(error.contains("greater than zero"), "{error}");
    assert!(state.model.simulation.retained.runs.is_empty());
}

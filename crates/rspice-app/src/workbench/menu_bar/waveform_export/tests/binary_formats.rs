//! Binary engineering exports: native bundles, value change dumps, NumPy,
//! HDF5 and MATLAB files.
//!
//! Each picker index reaches its own encoder and writes bytes that reopen
//! through the result importer. A result the container cannot carry, or one
//! whose file would exceed the re-import budget, is refused before a
//! destination is chosen, so nothing is written.

use super::*;

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
    assert_eq!(reopened.waveforms[0].unit, None);
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
    assert_eq!(reopened.waveforms[0].unit, None);
    let complex = reopened.waveforms[0]
        .complex
        .as_ref()
        .expect("complex components reopen");
    assert_eq!(complex.source_name, "V(out)");
    assert_eq!(complex.real.as_ref(), &[0.75, -1.5, 0.25]);
    assert_eq!(complex.imag.as_ref(), &[0.25, 0.5, -0.125]);
}

#[test]
fn native_bundle_preserves_saved_output_alias_units_in_real_and_complex_samples() {
    for (analysis_type, waveform) in [
        (
            AnalysisType::DcSweep,
            waveform(
                "Source current [forward]",
                vec![0.0, 0.5, 1.0],
                vec![0.0, -0.0005, -0.001],
            )
            .with_unit("A"),
        ),
        (
            AnalysisType::Ac,
            complex_waveform(
                "|Current alias|",
                "Current alias",
                vec![1e3, 2e3],
                vec![1.0, 2.0],
                vec![0.6, 1.2],
                vec![0.8, 1.6],
            )
            .with_unit("A"),
        ),
    ] {
        for preference in [3, 4] {
            let analysis = AnalysisResult::new(1, analysis_type, "Aliased current")
                .with_waveforms(vec![waveform.clone()]);
            let mut state = state_with_typed_result(analysis);
            state
                .ui
                .preferences
                .set_choice(
                    crate::workbench::ChoicePreference::EngineeringExport,
                    preference,
                )
                .unwrap();
            let io = MockExportWorkflowIo::default();
            action_export_csv_with_io(&mut state, &io);
            let files = io.byte_files.borrow();
            assert_eq!(files.len(), 1, "{}", last_log_message(&state));
            let reopened =
                crate::workbench::workflows::result_import_workflow::parse_result_dataset(
                    files[0].0.to_str().unwrap(),
                    &files[0].1,
                )
                .unwrap();
            assert_eq!(reopened.waveforms.len(), 1);
            assert_eq!(reopened.waveforms[0].unit.as_deref(), Some("A"));
            assert_eq!(reopened.waveforms[0].x, waveform.x);
            if let Some(original) = &waveform.complex {
                let restored = reopened.waveforms[0].complex.as_ref().unwrap();
                assert_eq!(restored.real, original.real);
                assert_eq!(restored.imag, original.imag);
            } else {
                assert_eq!(reopened.waveforms[0].y, waveform.y);
            }
        }
    }
}

#[test]
fn binary_waveform_exports_enforce_reimport_budget_before_destination_selection() {
    // Shared source arrays keep the fixture small, while a clone-per-column
    // projection would allocate more than the application's 64 MiB admission limit.
    let prototype = waveform(
        "prototype",
        (0..10_000).map(f64::from).collect(),
        vec![1.0; 10_000],
    );
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Wide result").with_waveforms(
        (0..840)
            .map(|index| {
                let mut trace = prototype.clone();
                trace.data.name = format!("signal{index}");
                trace
            })
            .collect(),
    );
    for preference in [6, 7, 8, 9] {
        let mut state = state_with_typed_result(analysis.clone());
        state
            .ui
            .preferences
            .set_choice(
                crate::workbench::ChoicePreference::EngineeringExport,
                preference,
            )
            .unwrap();
        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);
        assert!(
            io.dialog_titles.borrow().is_empty(),
            "preference {preference}"
        );
        assert!(io.byte_files.borrow().is_empty());
        let message = refusal(&state);
        assert!(
            message.contains("retains 8410000 numeric values") && message.contains("8388608"),
            "{message}"
        );
    }
}

#[test]
fn binary_waveform_exports_count_container_bytes_at_the_sample_budget_boundary() {
    // Exactly 64 MiB of f64 samples fits the admission budget, but every
    // supported container adds headers that put its file over the byte limit.
    let rows = 524_288;
    let prototype = waveform(
        "prototype",
        (0..rows).map(f64::from).collect(),
        vec![1.0; rows as usize],
    );
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Byte boundary").with_waveforms(
        (0..15)
            .map(|index| {
                let mut trace = prototype.clone();
                trace.data.name = format!("signal{index}");
                trace
            })
            .collect(),
    );
    for preference in [6, 7, 8, 9] {
        let mut state = state_with_typed_result(analysis.clone());
        state
            .ui
            .preferences
            .set_choice(
                crate::workbench::ChoicePreference::EngineeringExport,
                preference,
            )
            .unwrap();
        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);
        assert!(
            io.dialog_titles.borrow().is_empty(),
            "preference {preference}"
        );
        assert!(io.byte_files.borrow().is_empty());
        let message = last_log_message(&state);
        assert!(
            message.contains("encoded result") && message.contains("67108864 bytes"),
            "{message}"
        );
    }
}

/// The picker index is the durable contract between the preference and the
/// encoder. These are the three added with the new encoders, and each asserts
/// the bytes that actually reached the IO backend rather than only that a
/// dispatch arm was taken.
#[test]
fn engineering_export_preference_dispatches_the_three_new_encoders() {
    let events = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![waveform(
            "V(out)",
            vec![0.0, 1.0e-8, 2.0e-8],
            vec![0.0, 1.0, 0.0],
        )])
        .with_result_payload(AnalysisResultPayload::TransientEvents {
            voltage_impulses: None,
            current_impulses: None,
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "d".to_owned(),
                points: vec![
                    crate::state::DigitalEventPointEvidence {
                        time_s: 0.0,
                        value_code: 0,
                    },
                    crate::state::DigitalEventPointEvidence {
                        time_s: 1.0e-8,
                        value_code: 1,
                    },
                ],
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        });
    let mut vcd_state = state_with_typed_result(events);
    vcd_state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 5)
        .expect("VCD preference");
    let vcd_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut vcd_state, &vcd_io);
    assert_eq!(
        vcd_io.dialog_titles.borrow().as_slice(),
        &["Export Value Change Dump"]
    );
    let vcd_files = vcd_io.byte_files.borrow();
    assert_eq!(vcd_files.len(), 1);
    assert_eq!(vcd_files[0].0, PathBuf::from("events.vcd"));
    assert_eq!(vcd_files[0].2, "text/plain;charset=utf-8");
    let dump = String::from_utf8(vcd_files[0].1.clone()).expect("a dump is ASCII text");
    assert!(dump.contains("$scope module events $end"), "{dump}");
    assert!(dump.contains("$var wire 1 ! d $end"), "{dump}");
    drop(vcd_files);

    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0e-6, 2.0e-6], vec![0.0, 1.25, -0.5]),
        ]);
    let mut npy_state = state_with_typed_result(transient.clone());
    npy_state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 6)
        .expect("NumPy array preference");
    let npy_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut npy_state, &npy_io);
    assert_eq!(
        npy_io.dialog_titles.borrow().as_slice(),
        &["Export NumPy Array"]
    );
    let npy_files = npy_io.byte_files.borrow();
    assert_eq!(npy_files[0].0, PathBuf::from("waveforms.npy"));
    assert_eq!(npy_files[0].2, "application/octet-stream");
    let npy_reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
        "waveforms.npy",
        &npy_files[0].1,
    )
    .expect("reopen the array");
    assert_eq!(npy_reopened.waveforms[0].y.as_ref(), &[0.0, 1.25, -0.5]);
    drop(npy_files);

    let mut npz_state = state_with_typed_result(transient);
    npz_state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 7)
        .expect("NumPy archive preference");
    let npz_io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut npz_state, &npz_io);
    assert_eq!(
        npz_io.dialog_titles.borrow().as_slice(),
        &["Export NumPy Archive"]
    );
    let npz_files = npz_io.byte_files.borrow();
    assert_eq!(npz_files[0].0, PathBuf::from("waveforms.npz"));
    assert_eq!(npz_files[0].2, "application/zip");
    let npz_reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
        "waveforms.npz",
        &npz_files[0].1,
    )
    .expect("reopen the archive");
    assert_eq!(npz_reopened.coordinate_name, "time");
    // An archive keeps the names an array cannot.
    assert_eq!(npz_reopened.waveforms[0].name, "V(out)");
    assert_eq!(npz_reopened.waveforms[0].y.as_ref(), &[0.0, 1.25, -0.5]);
}

#[test]
fn engineering_export_preference_dispatches_an_hdf5_dataset() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0e-6, 2.0e-6], vec![0.0, 1.25, -0.5]),
        ]);
    let mut state = state_with_typed_result(transient);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 8)
        .expect("HDF5 preference");
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert_eq!(
        io.dialog_titles.borrow().as_slice(),
        &["Export HDF5 Dataset"]
    );
    let files = io.byte_files.borrow();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].0, PathBuf::from("waveforms.h5"));
    assert_eq!(files[0].2, "application/x-hdf5");
    assert_eq!(
        files[0].1[..8],
        [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a],
        "the HDF5 signature"
    );
    let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
        "waveforms.h5",
        &files[0].1,
    )
    .expect("reopen the dataset");
    assert_eq!(reopened.coordinate_name, "time");
    assert_eq!(reopened.coordinate_unit.as_deref(), Some("s"));
    assert_eq!(reopened.waveforms[0].name, "V(out)");
    assert_eq!(reopened.waveforms[0].y.as_ref(), &[0.0, 1.25, -0.5]);
}

#[test]
fn hdf5_export_ui_retains_imported_coordinate_names_and_units() {
    use rspice_results::result_import::{
        ResultImportCoordinate, ResultImportFormat, ResultImportSource,
    };

    for (kind, name, unit) in [
        (AnalysisType::Transient, "Elapsed time", Some("s")),
        (AnalysisType::Transient, "Clock", None),
        (AnalysisType::DcSweep, "bias", Some("A")),
        (AnalysisType::DcSweep, "control", None),
        (AnalysisType::Ac, "Test frequency", Some("Hz")),
        (AnalysisType::Ac, "Tone", None),
    ] {
        let trace = if kind == AnalysisType::Ac {
            complex_waveform(
                "|out|",
                "out",
                vec![1.0, 2.0],
                vec![1.0, 2.0],
                vec![-0.0, 4.0],
                vec![2.0, -1.0],
            )
        } else {
            waveform("out", vec![1.0, 2.0], vec![-0.0, 4.0])
        };
        let mut analysis =
            AnalysisResult::new(1, kind, "Imported coordinate").with_waveforms(vec![trace]);
        analysis.import_source = Some(ResultImportSource {
            source_name: "original.h5".into(),
            format: ResultImportFormat::Hdf5,
            coordinate: Some(ResultImportCoordinate {
                name: name.into(),
                unit: unit.map(str::to_owned),
            }),
        });
        let mut state = state_with_typed_result(analysis);
        state
            .ui
            .preferences
            .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 8)
            .unwrap();
        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);
        let files = io.byte_files.borrow();
        assert_eq!(files.len(), 1, "{}", last_log_message(&state));
        let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
            "waveforms.h5",
            &files[0].1,
        )
        .unwrap();
        assert_eq!(reopened.coordinate_name, name);
        assert_eq!(reopened.coordinate_unit.as_deref(), unit);
        assert_eq!(reopened.analysis_type, kind);
        assert_eq!(reopened.waveforms[0].x.as_ref(), &[1.0, 2.0]);
        assert_eq!(reopened.waveforms[0].unit, None);
        if kind == AnalysisType::Ac {
            let components = reopened.waveforms[0].complex.as_ref().unwrap();
            assert_eq!(components.real[0].to_bits(), (-0.0_f64).to_bits());
            assert_eq!(components.imag.as_ref(), &[2.0, -1.0]);
        } else {
            assert_eq!(reopened.waveforms[0].y[0].to_bits(), (-0.0_f64).to_bits());
        }
    }
}

#[test]
fn a_domain_the_hdf5_layout_has_no_section_for_refuses_and_writes_nothing() {
    let noise =
        AnalysisResult::new(1, AnalysisType::Noise, "Noise (1Hz-100Hz)").with_waveforms(vec![
            waveform(
                "V(onoise)",
                vec![1.0, 10.0, 100.0],
                vec![1.0e-9, 2.0e-9, 3.0e-9],
            ),
        ]);
    let mut state = state_with_typed_result(noise);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 8)
        .expect("HDF5 preference");
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.byte_files.borrow().is_empty());
    assert!(io.text_files.borrow().is_empty());
    // The refusal names the three sections HDF5 carries.
    let message = refusal(&state);
    assert!(
        message.contains("a transient, a DC sweep or an AC sweep"),
        "{message}"
    );
}

#[test]
fn binary_export_workflows_preserve_mixed_real_and_complex_traces_in_all_domains() {
    use rspice_results::result_import::{ResultImportFormat, ResultImportSource};

    for kind in [
        AnalysisType::Transient,
        AnalysisType::DcSweep,
        AnalysisType::Ac,
    ] {
        for preference in [8, 9] {
            let mut analysis = AnalysisResult::new(1, kind, "Imported complex samples")
                .with_waveforms(vec![
                    complex_waveform(
                        "|out|",
                        "out",
                        vec![1.0, 2.0],
                        vec![2.0, 17.0_f64.sqrt()],
                        vec![-0.0, 4.0],
                        vec![2.0, -1.0],
                    ),
                    waveform("scalar", vec![1.0, 2.0], vec![3.0, -0.0]),
                ]);
            analysis.import_source = Some(ResultImportSource {
                source_name: "original.h5".into(),
                format: ResultImportFormat::Hdf5,
                coordinate: None,
            });
            let mut state = state_with_typed_result(analysis);
            state
                .ui
                .preferences
                .set_choice(
                    crate::workbench::ChoicePreference::EngineeringExport,
                    preference,
                )
                .unwrap();
            let io = MockExportWorkflowIo::default();
            action_export_csv_with_io(&mut state, &io);
            let files = io.byte_files.borrow();
            assert_eq!(files.len(), 1, "{}", last_log_message(&state));
            let parsed = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
                files[0].0.to_str().unwrap(),
                &files[0].1,
            )
            .unwrap();
            assert_eq!(parsed.analysis_type, kind);
            assert_eq!(parsed.waveforms.len(), 2);
            let complex = parsed.waveforms[0]
                .complex
                .as_ref()
                .expect("retained components survive");
            assert_eq!(complex.source_name, "out");
            assert_eq!(complex.real[0].to_bits(), (-0.0_f64).to_bits());
            assert_eq!(complex.real[1], 4.0);
            assert_eq!(complex.imag.as_ref(), &[2.0, -1.0]);
            assert_eq!(parsed.waveforms[1].name, "scalar");
            assert!(parsed.waveforms[1].complex.is_none());
            assert_eq!(parsed.waveforms[1].y[1].to_bits(), (-0.0_f64).to_bits());
        }
    }
}

#[test]
fn a_transient_without_event_evidence_refuses_the_dump_and_writes_nothing() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0e-6, 2.0e-6], vec![0.0, 1.25, -0.5]),
        ]);
    let mut state = state_with_typed_result(transient);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 5)
        .expect("VCD preference");
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.byte_files.borrow().is_empty());
    assert!(io.text_files.borrow().is_empty());
    // The refusal names what a dump carries.
    let message = refusal(&state);
    assert!(message.contains("event timelines"), "{message}");
}

#[test]
fn engineering_export_preference_dispatches_a_matlab_v5_file() {
    let transient =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("V(out)", vec![0.0, 1.0e-6, 2.0e-6], vec![0.0, 1.25, -0.5]),
        ]);
    let mut state = state_with_typed_result(transient);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 9)
        .expect("MATLAB preference");
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert_eq!(
        io.dialog_titles.borrow().as_slice(),
        &["Export MATLAB v5 File"]
    );
    let files = io.byte_files.borrow();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].0, PathBuf::from("waveforms.mat"));
    assert_eq!(files[0].2, "application/x-matlab-data");
    assert!(
        files[0].1.starts_with(b"MATLAB 5.0 MAT-file"),
        "the Level 5 header signature"
    );
    let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
        "waveforms.mat",
        &files[0].1,
    )
    .expect("reopen the dataset");
    assert_eq!(reopened.coordinate_name, "time");
    // MAT names are MATLAB identifiers, so the reader sees the sanitised one.
    assert_eq!(reopened.waveforms[0].name, "V_out_");
    assert_eq!(reopened.waveforms[0].y.as_ref(), &[0.0, 1.25, -0.5]);
}

#[test]
fn a_domain_the_matlab_layout_has_no_coordinate_for_refuses_and_writes_nothing() {
    let noise =
        AnalysisResult::new(1, AnalysisType::Noise, "Noise (1Hz-100Hz)").with_waveforms(vec![
            waveform(
                "V(onoise)",
                vec![1.0, 10.0, 100.0],
                vec![1.0e-9, 2.0e-9, 3.0e-9],
            ),
        ]);
    let mut state = state_with_typed_result(noise);
    state
        .ui
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 9)
        .expect("MATLAB preference");
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);

    assert!(io.dialog_titles.borrow().is_empty());
    assert!(io.byte_files.borrow().is_empty());
    assert!(io.text_files.borrow().is_empty());
    // The refusal names the three coordinates a MAT file can be read back by.
    let message = refusal(&state);
    assert!(
        message.contains("a transient on 'time'") && message.contains("an AC sweep on 'frequency'"),
        "{message}"
    );
}

//! Authored outputs must retain every DC member through the prepared run path.
//! Ordinary sweeps are the control; nested and retraced sweeps have the same
//! requested signals but must not collapse their members into one waveform.

use super::fixtures::output;
use super::*;
use crate::state::{AnalysisResultPayload, DcSweepFamily, OutputSelectionMode, SimulationRun};

const DECK: &str = "Saved DC family\nV1 in 0 0\nV2 out 0 1e-7\nR1 in out 1k\n.end\n";

fn spec(nested: bool, retraced: bool) -> AnalysisSpec {
    AnalysisSpec::DcSweep {
        source_name: "V1".to_owned(),
        start: 1.0,
        stop: 0.0,
        step: -0.5,
        source2: nested.then(|| "V2".to_owned()),
        start2: nested.then_some(1e-7),
        stop2: nested.then_some(3e-7),
        step2: nested.then_some(1e-7),
        hysteresis: retraced,
    }
}

fn run(spec: AnalysisSpec, outputs: &[SavedOutput]) -> SimulationRun {
    run_with_selection(spec, outputs, OutputSelectionMode::ExplicitOnly)
}

fn run_with_selection(
    spec: AnalysisSpec,
    outputs: &[SavedOutput],
    output_selection_mode: OutputSelectionMode,
) -> SimulationRun {
    let line = if matches!(spec, AnalysisSpec::DcOp { .. }) {
        ".op"
    } else {
        ".dc V1 1 0 -0.5"
    };
    super::fixtures::run(
        DECK,
        "Authored DC family",
        spec,
        line,
        outputs,
        output_selection_mode,
    )
}

fn assert_outputs(spec: AnalysisSpec, member_count: usize) {
    let outputs = [
        output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Output voltage",
            "V(out)",
        ),
        output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Source current",
            "I(V1)",
        ),
        output(
            SavedOutputKind::DerivedExpression,
            "Voltage sum",
            "V(in) + V(out)",
        ),
        output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Difference",
            "V(in,out)",
        ),
    ];
    let run = run(spec, &outputs);
    run.validate_provenance().unwrap();
    assert!(run.success);
    assert_eq!(run.analyses.len(), 1);
    let analysis = &run.analyses[0];
    assert!(analysis.success, "{:?}", analysis.error_message);
    assert_eq!(analysis.saved_output_receipts.len(), outputs.len());
    assert_eq!(
        analysis.waveforms.len(),
        outputs.len() * member_count,
        "every authored output must retain every solved member: {:?}",
        analysis.saved_output_receipts
    );
    for output in outputs {
        assert_eq!(
            analysis
                .waveforms
                .iter()
                .filter(|wave| wave.name.starts_with(&output.name))
                .count(),
            member_count
        );
    }
    let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
        panic!("DC output projection must retain its solved member identities");
    };
    for member in 0..member_count {
        let bias = match &evidence.family {
            DcSweepFamily::Nested { values, .. } => values[member],
            _ => 1e-7,
        };
        for (name, unit) in [
            ("Output voltage", Some("V")),
            ("Source current", Some("A")),
            ("Voltage sum", None),
            ("Difference", Some("V")),
        ] {
            let waveform = analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == evidence.member_trace_name(name, member))
                .unwrap();
            assert_eq!(waveform.x.as_slice(), &[0.0, 0.5, 1.0]);
            assert_eq!(waveform.unit.as_deref(), unit);
            for (&x, &actual) in waveform.x.iter().zip(waveform.y.iter()) {
                let expected = match name {
                    "Output voltage" => bias,
                    "Source current" => (bias - x) / 1000.0,
                    "Voltage sum" => x + bias,
                    "Difference" => x - bias,
                    _ => unreachable!(),
                };
                assert!(
                    (actual - expected).abs() < 1e-12,
                    "{name} member {member} at {x}: {actual} != {expected}"
                );
            }
        }
    }
    analysis.validate_retained_evidence().unwrap();
}

#[test]
fn ordinary_dc_materializes_the_same_authored_outputs() {
    assert_outputs(spec(false, false), 1);
}

#[test]
fn nested_dc_materializes_every_authored_output_for_each_member() {
    assert_outputs(spec(true, false), 3);
}

#[test]
fn retraced_dc_materializes_every_authored_output_for_each_branch() {
    assert_outputs(spec(false, true), 2);
}

#[test]
fn ascending_and_single_secondary_point_dc_families_keep_their_output_members() {
    for mut spec in [spec(true, false), spec(false, true)] {
        if let AnalysisSpec::DcSweep {
            start, stop, step, ..
        } = &mut spec
        {
            *start = 0.0;
            *stop = 1.0;
            *step = 0.5;
        }
        let count = if matches!(
            &spec,
            AnalysisSpec::DcSweep {
                source2: Some(_),
                ..
            }
        ) {
            3
        } else {
            2
        };
        assert_outputs(spec, count);
    }
    let mut spec = spec(true, false);
    if let AnalysisSpec::DcSweep { stop2, .. } = &mut spec {
        *stop2 = Some(1e-7);
    }
    assert_outputs(spec, 1);
}

#[test]
fn dc_family_save_policies_keep_requested_members_and_hide_internal_sources() {
    for selection in [
        OutputSelectionMode::ExplicitOnly,
        OutputSelectionMode::SaveAll,
    ] {
        for policy in [
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPolicy::OnDemandFromRetainedState,
            SavedOutputPolicy::FailureDiagnosticsOnly,
        ] {
            let mut output = output(SavedOutputKind::RawVoltageOrCurrent, "Chosen", "V(out)");
            output.save_policy = policy;
            output.display_intent = crate::state::SavedOutputDisplayIntent::DataBrowserOnly;
            let run = run_with_selection(spec(true, false), &[output], selection);
            run.validate_provenance().unwrap();
            let analysis = &run.analyses[0];
            assert!(analysis.success);
            assert!(
                analysis.waveforms.iter().all(|waveform| !waveform.visible),
                "{selection:?} {policy:?} exposed internal or data-browser-only curves"
            );
            let status = &analysis.saved_output_receipts[0].status;
            match policy {
                SavedOutputPolicy::OnDemandFromRetainedState => {
                    assert_eq!(status, &SavedOutputMaterializationStatus::Deferred);
                    assert!(!analysis.waveforms.is_empty());
                }
                SavedOutputPolicy::FailureDiagnosticsOnly => {
                    assert_eq!(
                        status,
                        &SavedOutputMaterializationStatus::SuppressedOnSuccess
                    );
                    if selection == OutputSelectionMode::ExplicitOnly {
                        assert!(analysis.waveforms.is_empty());
                    }
                }
                _ => {
                    assert_eq!(status.materialized_waveforms().count(), 3);
                    assert!(status.materialized_waveforms().all(|(_, count)| count == 3));
                    if selection == OutputSelectionMode::ExplicitOnly {
                        assert_eq!(analysis.waveforms.len(), 3);
                    }
                }
            }
        }
    }
}

#[test]
fn dc_family_materialization_is_atomic_when_the_last_member_collides() {
    for spec in [spec(true, false), spec(false, true)] {
        let mut output = output(SavedOutputKind::RawVoltageOrCurrent, "Chosen", "V(out)");
        output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
        let run = run(spec, &[output]);
        let mut analysis = run.analyses[0].clone();
        let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
            unreachable!()
        };
        let collision_name = evidence.member_trace_name("Chosen", evidence.member_count() - 1);
        analysis.waveforms.push(
            WaveformData::new(&collision_name, vec![0.0, 0.5, 1.0], vec![9.0; 3], "#fff")
                .with_unit("V"),
        );
        let names = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.name.clone())
            .collect::<Vec<_>>();
        let owners = analysis
            .waveforms
            .iter()
            .map(|waveform| (Arc::clone(&waveform.x), Arc::clone(&waveform.y)))
            .collect::<Vec<_>>();
        assert!(
            materialize_deferred_saved_output(&mut analysis, 0)
                .unwrap_err()
                .contains("collides")
        );
        assert_eq!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Deferred
        );
        assert_eq!(
            analysis
                .waveforms
                .iter()
                .map(|waveform| &waveform.name)
                .collect::<Vec<_>>(),
            names.iter().collect::<Vec<_>>()
        );
        for (waveform, (x, y)) in analysis.waveforms.iter().zip(owners) {
            assert!(Arc::ptr_eq(&waveform.x, &x));
            assert!(Arc::ptr_eq(&waveform.y, &y));
        }
    }
}

#[test]
fn deferred_dc_family_shares_each_exact_source_and_rebuilds_every_display_cache() {
    for spec in [spec(true, false), spec(false, true)] {
        let mut output = output(SavedOutputKind::RawVoltageOrCurrent, "Chosen", "V(out)");
        output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
        output.stored_precision = SavedOutputPrecision::DisplayCacheWithFullSourcePrecision;
        let run = run(spec, &[output]);
        let mut analysis = run.analyses[0].clone();
        let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
            unreachable!()
        };
        let evidence = Arc::clone(evidence);
        let quantity = evidence
            .quantities
            .iter()
            .find(|quantity| quantity.unit() == "V" && quantity.name().eq_ignore_ascii_case("out"))
            .unwrap();
        let sources = (0..evidence.member_count())
            .map(|member| {
                analysis
                    .waveforms
                    .iter()
                    .find(|waveform| waveform.name == evidence.trace_name(quantity, member))
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<_>>();
        materialize_deferred_saved_output(&mut analysis, 0).unwrap();
        analysis.validate_retained_evidence().unwrap();
        let members = analysis.saved_output_receipts[0]
            .status
            .materialized_waveforms()
            .collect::<Vec<_>>();
        assert_eq!(members.len(), sources.len());
        for ((name, count), source) in members.into_iter().zip(sources) {
            let waveform = analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == name)
                .unwrap();
            assert_eq!(count as usize, source.x.len());
            assert!(Arc::ptr_eq(&waveform.x, &source.x));
            assert!(Arc::ptr_eq(&waveform.y, &source.y));
            assert!(waveform.display_cache.is_some());
        }
        let names = analysis.saved_output_receipts[0]
            .status
            .materialized_waveforms()
            .map(|(name, _)| name.to_owned())
            .collect::<HashSet<_>>();
        for waveform in &mut analysis.waveforms {
            if names.contains(&waveform.name) {
                waveform.x = Arc::new(vec![0.0, 0.25, 1.0]);
            }
        }
        assert!(
            analysis
                .validate_retained_evidence()
                .unwrap_err()
                .contains("retained solved primary axis")
        );
    }
}

#[test]
fn operating_point_outputs_resolve_retained_voltages_currents_and_expressions() {
    let outputs = [
        output(SavedOutputKind::RawVoltageOrCurrent, "Voltage", "V(out)"),
        output(SavedOutputKind::RawVoltageOrCurrent, "Current", "I(V1)"),
        output(
            SavedOutputKind::DerivedExpression,
            "Difference",
            "V(out) - V(in)",
        ),
    ];
    let run = run(AnalysisSpec::dc_op(), &outputs);
    run.validate_provenance().unwrap();
    let analysis = &run.analyses[0];
    assert!(analysis.success);
    assert!(analysis.dc_op.is_some());
    assert_eq!(
        analysis.waveforms.len(),
        3,
        "{:?}",
        analysis.saved_output_receipts
    );
    for (index, (name, expected, unit)) in [
        ("Voltage", 1e-7, Some("V")),
        ("Current", 1e-10, Some("A")),
        ("Difference", 1e-7, None),
    ]
    .into_iter()
    .enumerate()
    {
        let waveform = &analysis.waveforms[index];
        assert_eq!(waveform.name, name);
        assert_eq!(waveform.x.as_slice(), &[0.0]);
        assert!((waveform.y[0] - expected).abs() < 1e-16);
        assert_eq!(waveform.unit.as_deref(), unit);
        assert_eq!(
            analysis.saved_output_receipts[index]
                .status
                .materialized_waveforms()
                .collect::<Vec<_>>(),
            vec![(name, 1)]
        );
    }
    analysis.validate_retained_evidence().unwrap();
}

fn stored(run: SimulationRun) -> crate::io::project_io::ProjectSimulationResults {
    let mut state = crate::state::SimulationState::default();
    state.next_run_id = run.id;
    state.runs = vec![run].into();
    crate::io::project_io::ProjectSimulationResults::from_state(&state)
}

#[test]
fn dc_family_receipts_round_trip_every_member_and_reject_schema_downgrades() {
    let mut output = output(
        SavedOutputKind::RawVoltageOrCurrent,
        "Current alias",
        "I(V1)",
    );
    output.stored_precision = SavedOutputPrecision::DisplayCacheWithFullSourcePrecision;
    let run = run(spec(true, false), &[output]);
    let digest = run.analyses[0].result_data_digest();
    let snapshot = stored(run);
    snapshot.validate().unwrap();
    let restored: crate::io::project_io::ProjectSimulationResults =
        serde_json::from_slice(&serde_json::to_vec(&snapshot).unwrap()).unwrap();
    let mut state = crate::state::SimulationState::default();
    restored.apply_to_state(&mut state).unwrap();
    let analysis = &state.runs[0].analyses[0];
    assert_eq!(analysis.result_data_digest(), digest);
    assert_eq!(analysis.waveforms.len(), 3);
    assert!(
        analysis
            .waveforms
            .iter()
            .all(|waveform| waveform.display_cache.is_some()
                && waveform.unit.as_deref() == Some("A"))
    );
    for schema in [21, 22] {
        let mut downgraded = snapshot.clone();
        downgraded.schema_version = schema;
        for receipt in &mut downgraded.runs[0].analyses[0].saved_output_receipts {
            receipt.source_bindings = None;
        }
        assert!(
            downgraded
                .migrate_to_current(crate::product::ProjectId::new())
                .unwrap_err()
                .contains("DC output families")
        );
    }
}

#[test]
fn schema_22_saved_outputs_authenticate_and_reseal_their_result_digests() {
    for nested in [false, true] {
        let mut output = output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Voltage alias",
            "V(out)",
        );
        if nested {
            output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
        }
        let mut run = run(spec(nested, false), &[output]);
        run.analyses[0].saved_output_receipts[0].source_bindings = None;
        let digest = run.analyses[0].legacy_v12_result_data_digest();
        let dataset_digest = run.legacy_v12_dataset_content_digest();
        let mut snapshot = stored(run);
        snapshot.schema_version = 22;
        snapshot.runs[0].analyses[0].result_data_digest =
            crate::io::project_io::PersistedField::Value(digest);
        snapshot.runs[0].dataset_content_digest =
            crate::io::project_io::PersistedField::Value(dataset_digest);
        let mut corrupted = snapshot.clone();
        Arc::make_mut(&mut corrupted.runs[0].analyses[0].waveforms[0].y)[0] += 0.1;
        assert!(
            corrupted
                .migrate_to_current(crate::product::ProjectId::new())
                .unwrap_err()
                .contains("digest")
        );
        snapshot
            .migrate_to_current(crate::product::ProjectId::new())
            .unwrap();
        assert_eq!(snapshot.schema_version, 26);
        let mut state = crate::state::SimulationState::default();
        snapshot.apply_to_state(&mut state).unwrap();
        assert_ne!(state.runs[0].analyses[0].result_data_digest(), digest);
    }
}

#[test]
fn dc_family_receipts_reject_missing_reordered_and_misrepresented_raw_members() {
    let run = run(
        spec(true, false),
        &[output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Current alias",
            "I(V1)",
        )],
    );
    let original = &run.analyses[0];
    original.validate_retained_evidence().unwrap();
    for mutation in 0..7 {
        let mut analysis = original.clone();
        let SavedOutputMaterializationStatus::MaterializedDcFamily { members } =
            &mut analysis.saved_output_receipts[0].status
        else {
            unreachable!()
        };
        match mutation {
            0 => {
                members.pop();
            }
            1 => members.swap(0, 1),
            2 => members[0].sample_count += 1,
            3 => members[0].waveform_name.push_str(" changed"),
            4 => analysis.waveforms[0].unit = Some("V".to_owned()),
            5 => analysis.waveforms[0].x = Arc::new(vec![0.0, 0.25, 1.0]),
            6 => {
                for waveform in &mut analysis.waveforms {
                    waveform.x = Arc::new(vec![1.0, 0.5, 0.0]);
                }
            }
            _ => unreachable!(),
        }
        assert!(
            analysis.validate_retained_evidence().is_err(),
            "accepted mutation {mutation}"
        );
        assert_ne!(analysis.result_data_digest(), original.result_data_digest());
    }
}

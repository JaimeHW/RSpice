use super::*;

/// The reader's accept set is the engine's own label vocabulary, so nothing the
/// engine can name is refused on write. This is the property that failed: the
/// reader carried a hand-copied subset, and every family that had outgrown it
/// wrote a project the reader would not take back.
#[test]
fn the_project_reader_accepts_every_label_the_engine_can_emit() {
    for label in rspice_core::circuit::OP_LABELS {
        require_static_label(label.as_str(), "device_op label")
            .unwrap_or_else(|error| panic!("{error}"));
    }
}

/// One prepared-task provenance record as a project writes it: a small-signal
/// result depending on `parent`, with `state` attached when it has one.
fn state_provenance(
    parent: AnalysisInstanceId,
    state: Option<AnalysisResultTransientState>,
) -> Result<AnalysisResultProvenance, String> {
    AnalysisResultProvenance::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x2a; 32]),
        vec![parent],
    )?
    .with_transient_state(state)
}

/// A result linearized around a transient state keeps its parent and its
/// time through the project file, bit for bit.
#[test]
fn a_transient_state_survives_the_project_file() {
    let parent = AnalysisInstanceId::new();
    let state = AnalysisResultTransientState {
        parent,
        time_s: 5.5e-4,
    };
    let provenance = state_provenance(parent, Some(state)).expect("the provenance builds");
    let text = serde_json::to_string(&ProjectAnalysisResultProvenance::from(&provenance))
        .expect("the provenance writes");
    assert!(text.contains("\"transient_state\""), "{text}");
    let read: ProjectAnalysisResultProvenance =
        serde_json::from_str(&text).expect("the provenance reads");
    read.validate().expect("the read provenance validates");
    let restored = read.into_provenance().expect("the provenance restores");
    assert_eq!(restored, provenance);
    assert_eq!(restored.transient_state(), Some(&state));
}

/// A transient that solved one point of a `.STEP` sweep keeps the point and
/// its coordinate through the project file, bit for bit; a result that names
/// no member writes no field.
#[test]
fn a_family_member_survives_the_project_file() {
    use rspice_results::provenance::{AnalysisResultFamilyMember, AnalysisResultStepCoordinate};
    let member = AnalysisResultFamilyMember::StepPoint {
        index: 1,
        coordinate: vec![AnalysisResultStepCoordinate {
            name: "r".into(),
            value: 2e3,
        }],
    };
    let provenance = state_provenance(AnalysisInstanceId::new(), None)
        .and_then(|provenance| provenance.with_family_member(Some(member.clone())))
        .expect("the provenance builds");
    let text = serde_json::to_string(&ProjectAnalysisResultProvenance::from(&provenance))
        .expect("the provenance writes");
    assert!(text.contains("\"family_member\""), "{text}");
    let read: ProjectAnalysisResultProvenance =
        serde_json::from_str(&text).expect("the provenance reads");
    read.validate().expect("the read provenance validates");
    let restored = read.into_provenance().expect("the provenance restores");
    assert_eq!(restored.family_member(), Some(&member));
    assert_eq!(restored, provenance);

    let plain = state_provenance(AnalysisInstanceId::new(), None).unwrap();
    let text = serde_json::to_string(&ProjectAnalysisResultProvenance::from(&plain)).unwrap();
    assert!(!text.contains("family_member"), "{text}");

    let unnamed = AnalysisResultFamilyMember::StepPoint {
        index: 0,
        coordinate: vec![AnalysisResultStepCoordinate {
            name: "r".into(),
            value: f64::NAN,
        }],
    };
    assert!(plain.with_family_member(Some(unnamed)).is_err());
}

/// Two points of a design-variable sweep keep their swept values through the
/// project file, exact text in declaration order, so they read back as the
/// two points they were; a point that swept nothing writes no field, and a
/// stored value with no name is refused.
#[test]
fn a_pvt_points_swept_values_survive_the_project_file() {
    use rspice_results::provenance::{AnalysisResultSweptAxis, AnalysisResultSweptValue};
    let swept = |cload: &str| {
        vec![
            AnalysisResultSweptValue {
                axis: AnalysisResultSweptAxis::Parameter,
                name: "CLOAD".to_owned(),
                value: cload.to_owned(),
            },
            AnalysisResultSweptValue {
                axis: AnalysisResultSweptAxis::Source,
                name: "VBIAS".to_owned(),
                value: "1.5".to_owned(),
            },
        ]
    };
    let point = |cload: &str| {
        AnalysisResultPvtPoint::new("TT", Some(1.8), 27.0, None, false)
            .and_then(|point| point.with_swept_values(swept(cload)))
            .expect("the point builds")
    };
    let round_trip = |point: AnalysisResultPvtPoint| {
        let provenance = state_provenance(AnalysisInstanceId::new(), None)
            .expect("the provenance builds")
            .with_pvt_point(Some(point));
        let text = serde_json::to_string(&ProjectAnalysisResultProvenance::from(&provenance))
            .expect("the provenance writes");
        let read: ProjectAnalysisResultProvenance =
            serde_json::from_str(&text).expect("the provenance reads");
        read.validate().expect("the read provenance validates");
        let restored = read.into_provenance().expect("the provenance restores");
        assert_eq!(restored, provenance);
        (text, restored)
    };

    let (text, small) = round_trip(point("2p"));
    assert!(text.contains("\"swept_values\""), "{text}");
    let (_, large) = round_trip(point("20p"));
    let small = small.pvt_point().expect("the small point restores");
    let large = large.pvt_point().expect("the large point restores");
    assert_eq!(small.swept_values(), swept("2p").as_slice());
    assert_ne!(small, large);

    let plain = AnalysisResultPvtPoint::new("TT", Some(1.8), 27.0, None, true).unwrap();
    let (text, _) = round_trip(plain);
    assert!(!text.contains("swept_values"), "{text}");

    let unnamed = vec![AnalysisResultSweptValue {
        axis: AnalysisResultSweptAxis::Parameter,
        name: " ".to_owned(),
        value: "2p".to_owned(),
    }];
    let stored = ProjectAnalysisResultPvtPoint {
        process: "TT".to_owned(),
        supply_voltage: None,
        temperature_celsius: 27.0,
        corner_contract: None,
        nominal: false,
        swept_values: unnamed,
    };
    let error = stored.into_point().expect_err("an unnamed swept value");
    assert!(error.contains("no name"), "{error}");
}

/// A project written before results named a transient state reads as it
/// always did: every result in it was linearized at the operating point, and
/// a result that names no state writes no field.
#[test]
fn a_project_written_before_transient_states_still_reads() {
    let provenance = state_provenance(AnalysisInstanceId::new(), None).expect("builds");
    let text = serde_json::to_string(&ProjectAnalysisResultProvenance::from(&provenance))
        .expect("the provenance writes");
    assert!(!text.contains("transient_state"), "{text}");
    let read: ProjectAnalysisResultProvenance =
        serde_json::from_str(&text).expect("an older record reads");
    read.validate().expect("an older record validates");
    let restored = read.into_provenance().expect("an older record restores");
    assert_eq!(restored.transient_state(), None);
    assert_eq!(restored, provenance);
}

/// A state whose parent is not among the result's dependencies, or whose
/// time is not a time, is refused rather than persisted.
#[test]
fn a_transient_state_names_one_of_its_dependencies_and_a_finite_time() {
    let parent = AnalysisInstanceId::new();
    let stranger = AnalysisResultTransientState {
        parent: AnalysisInstanceId::new(),
        time_s: 1e-6,
    };
    let error = state_provenance(parent, Some(stranger)).expect_err("a stranger parent");
    assert!(error.contains("not one of its dependencies"), "{error}");
    for time_s in [f64::NAN, f64::INFINITY, -1e-9] {
        let error = state_provenance(
            parent,
            Some(AnalysisResultTransientState { parent, time_s }),
        )
        .expect_err("not a time");
        assert!(error.contains("not a finite time"), "{time_s}: {error}");
    }
}

/// A run's log is read back from the project as it was written, and a run
/// with nothing in its log writes no log at all, so a project from before
/// runs kept one is the same file.
#[test]
fn a_runs_log_survives_the_project_file_and_an_empty_one_is_not_written() {
    use rspice_vocabulary::diagnostics::{LogSeverity, LogSource};
    use rspice_vocabulary::run_log::RunLogLine;

    let mut run = SimulationRun::<AnalysisResult>::new(
        4,
        1_760_613_845.0,
        rspice_results::run::ExecutionTarget::LocalDesktop,
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("a fresh run takes provenance");
    let persist = |run: &SimulationRun| {
        ProjectSimulationRun::from_run::<_, RetainedWaveform>(run, |_| {
            unreachable!("the run retained no analyses")
        })
    };
    let written = serde_json::to_value(persist(&run)).expect("serializes");
    assert!(written.get("log").is_none(), "{written}");

    run.log.push(RunLogLine {
        unix_ms: 1_760_613_846_020,
        severity: LogSeverity::Warning,
        source: LogSource::Hdl,
        message: "tb.v:14: $error: output stuck low".to_owned(),
    });
    let text = serde_json::to_string(&persist(&run)).expect("serializes");
    let read: ProjectSimulationRun = serde_json::from_str(&text).expect("deserializes");
    assert_eq!(read.into_run().expect("restores").log, run.log);
}

/// A field is written as `Some` so RON can read it back; a saved project's
/// JSON must not change with it. A present field is still a bare value there,
/// and an explicit null still null.
#[test]
fn a_persisted_field_writes_project_json_as_a_bare_value_or_null() {
    let present = PersistedField::Value("ab".to_owned());
    assert_eq!(serde_json::to_string(&present).unwrap(), "\"ab\"");
    assert_eq!(
        serde_json::to_string(&PersistedField::<String>::Null).unwrap(),
        "null"
    );
    let read: PersistedField<String> = serde_json::from_str("\"ab\"").unwrap();
    assert_eq!(read, present);
}

/// A snapshot for a reader in another memory, as a browser's hardcopy worker is, carries a stored
/// event history's changes: a reference would name a segment that reader does not hold. A saved
/// payload still names the segment.
#[test]
fn a_snapshot_for_another_memory_carries_the_changes_of_its_stored_event_histories() {
    use rspice_results::events::RealEventPointEvidence;
    use rspice_results::events::points::EventPoints;
    use rspice_results::store_reader::{Segment, SegmentBytes, SegmentEvents, StoreReader};
    use rspice_results_store::{
        Axis, EventChange, EventValue, MemoryIo, RunMeta, RunStatus, Schema, SegmentReader,
        SegmentWriter, Signal, SignalKind,
    };

    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![
            Signal::new("v(a)", "V", SignalKind::Real),
            Signal::new("e(ctl)", "V", SignalKind::RealEvent),
        ],
    };
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &RunMeta::default()).unwrap();
    writer.push_rows(&[0.0, 1.0], &[0.0, 1.0]).unwrap();
    let changes: Vec<EventChange> = (0..4_u32)
        .map(|i| EventChange {
            signal: 1,
            time: f64::from(i) * 0.25,
            sequence: i,
            value: EventValue::Real(f64::from(i)),
            strength: 0,
        })
        .collect();
    writer.push_events(&changes).unwrap();
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    let reader = SegmentReader::open(Box::new(io) as SegmentBytes).unwrap();
    let segment = Segment::kept(StoreReader::new(reader), std::path::PathBuf::new()).shared();
    let history: EventPoints<RealEventPointEvidence> =
        EventPoints::stored(SegmentEvents::new(segment, 1, "e(ctl)".to_owned()), 0.0).unwrap();

    let write = |carrying: bool| {
        let mut out = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut out);
        let mut payload = || stored_values::serialize_payload(&history, &mut serializer);
        if carrying {
            carrying_rows(payload).unwrap();
        } else {
            payload().unwrap();
        }
        String::from_utf8(out).unwrap()
    };
    let digest = receipt.digest.to_string();
    let named = write(false);
    assert!(named.contains(&digest), "{named}");
    let carried = write(true);
    assert!(!carried.contains(&digest), "{carried}");
    let read: EventPoints<RealEventPointEvidence> =
        carrying_rows(|| serde_json::from_str(&carried)).unwrap();
    assert_eq!(read.len(), changes.len());
    assert_eq!(
        serde_json::to_string(&*read).unwrap(),
        serde_json::to_string(&*history).unwrap()
    );
}

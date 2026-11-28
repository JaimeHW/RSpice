//! A finished analysis written to the results store: every series becomes a window of a
//! segment's column with the same samples, waveforms on one axis share one segment and one axis
//! window, and an axis that does not ascend is kept as a column of its own.

use super::*;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::store_reader::SegmentColumn;
use rspice_results::waveform::{StoredWindow, WaveformValues, WindowLead};

fn bits(values: &WaveformValues) -> Vec<u64> {
    values.samples(..).map(f64::to_bits).collect()
}

/// An imported AC result of two complex traces on one frequency axis, one with a sample its source
/// left unavailable, and a DC trace on a sweep that runs down, each axis its own allocation as a
/// converter makes them.
fn analysis() -> RetainedAnalysisResult<RetainedWaveform> {
    let frequencies = || (0..50).map(|k| 10.0 * 1.2_f64.powi(k)).collect::<Vec<_>>();
    let magnitude: Vec<f64> = (0..50).map(|k| 1.0 / (1.0 + k as f64)).collect();
    let real: Vec<f64> = (0..50).map(|k| (k as f64).cos()).collect();
    let imag: Vec<f64> = (0..50).map(|k| -(k as f64).sin()).collect();
    let mut nan = magnitude.clone();
    nan[7] = f64::NAN;
    let sweep: Vec<f64> = (0..21).map(|k| 5.0 - 0.5 * k as f64).collect();
    let mut result = RetainedAnalysisResult::new(1, AnalysisType::Ac, "AC", 0.0);
    result.waveforms = vec![
        RetainedWaveform::new("V(out)", frequencies(), magnitude.clone())
            .with_unit("V")
            .with_complex_components("V(out)", real.clone(), imag.clone()),
        RetainedWaveform::new("V(in)", frequencies(), nan).with_unit("V"),
        RetainedWaveform::new(
            "I(V1)",
            sweep.clone(),
            sweep.iter().map(|v| v / 1e3).collect::<Vec<_>>(),
        )
        .with_unit("A"),
    ];
    result.import_source = Some(rspice_results::result_import::ResultImportSource {
        source_name: "ac.raw".into(),
        format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
        coordinate: None,
        family_member: None,
        event_dump: None,
    });
    result
}

fn assert_kept(make: fn(&mut RetainedAnalysisResult<RetainedWaveform>) -> Result<(), String>) {
    let before = analysis();
    let mut after = before.clone();
    make(&mut after).expect("the store keeps the samples");
    for (left, right) in before.waveforms.iter().zip(&after.waveforms) {
        assert!(right.x.is_stored() && right.y.is_stored(), "{}", right.name);
        assert_eq!(bits(&right.x), bits(&left.x), "{}", right.name);
        assert_eq!(bits(&right.y), bits(&left.y), "{}", right.name);
        assert_eq!(
            right
                .complex
                .as_ref()
                .map(|c| (bits(&c.real), bits(&c.imag))),
            left.complex
                .as_ref()
                .map(|c| (bits(&c.real), bits(&c.imag))),
        );
        assert_eq!(right.unit, left.unit);
    }
    let [ac_out, ac_in, dc] = &after.waveforms[..] else {
        panic!("three waveforms");
    };
    assert!(
        WaveformValues::ptr_eq(&ac_out.x, &ac_in.x),
        "one axis window for the waveforms on one axis"
    );
    assert!(
        ac_out.x.is_ascending_axis(),
        "an ascending axis is the segment's own"
    );
    assert!(
        !dc.x.is_ascending_axis(),
        "a sweep that runs down is a column, never read as an ascending axis"
    );
    let segment = |values: &WaveformValues| {
        values
            .stored_window()
            .and_then(|(source, _)| source.identity())
            .map(|(digest, _)| digest)
            .expect("a stored window names its segment")
    };
    assert_eq!(segment(&ac_out.y), segment(&ac_in.y));
    assert_ne!(segment(&ac_out.y), segment(&dc.y));
    // Storing again changes nothing.
    let mut again = after.clone();
    make(&mut again).expect("nothing left to keep");
    assert!(WaveformValues::ptr_eq(
        &again.waveforms[0].y,
        &after.waveforms[0].y
    ));
    assert!(!holds_samples_in_memory(&after));
    assert!(holds_samples_in_memory(&before));
}

#[test]
fn a_finished_analysis_keeps_every_sample_in_the_process_store() {
    assert_kept(store_analysis);
}

#[test]
fn a_finished_analysis_keeps_every_sample_in_a_store_held_in_memory() {
    assert_kept(store_analysis_in_memory);
}

/// A waveform of 20,000 rows read from a segment whose `END` was lost: its rows are readable and
/// it states no digest.
fn waveform_of_a_segment_without_its_end() -> RetainedWaveform {
    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![Signal::new("V(out)", "V", SignalKind::Real)],
    };
    /// Memory a test reads back after the writer that wrote it is gone.
    #[derive(Clone, Default)]
    struct SharedIo(Arc<std::sync::Mutex<MemoryIo>>);
    use rspice_results_store::SegmentIo;
    impl SegmentIo for SharedIo {
        fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
            self.0.lock().unwrap().read_at(offset, buf)
        }
        fn write_at(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
            self.0.lock().unwrap().write_at(offset, data)
        }
        fn truncate(&mut self, len: u64) -> std::io::Result<()> {
            self.0.lock().unwrap().truncate(len)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.0.lock().unwrap().flush()
        }
        fn size(&self) -> std::io::Result<u64> {
            self.0.lock().unwrap().size()
        }
    }
    let io = SharedIo::default();
    let mut writer = SegmentWriter::create(io.clone(), &schema, &RunMeta::default()).unwrap();
    for row in 0..20_000_u32 {
        let t = f64::from(row) * 1e-9;
        writer.push_row(t, &[(t * 1e6).sin()]).unwrap();
    }
    // The run stops here: what it synced is on disk, its `END` is not.
    writer.sync().unwrap();
    drop(writer);
    let bytes = io.0.lock().unwrap().bytes().to_vec();
    let reader =
        SegmentReader::open(Box::new(MemoryIo::from_bytes(bytes)) as SegmentBytes).unwrap();
    assert_eq!(
        *reader.state(),
        rspice_results_store::SegmentState::Recovered
    );
    let segment = Segment::kept(StoreReader::new(reader), std::path::PathBuf::new()).shared();
    let window = StoredWindow {
        first: 0,
        rows: segment.rows(),
        lead: WindowLead::None,
    };
    assert!(window.rows > 0);
    let column = |slot, name: &str| {
        WaveformValues::stored(
            SegmentColumn::new(Arc::clone(&segment), slot, name.to_owned()),
            window,
        )
        .unwrap()
    };
    RetainedWaveform::new("V(out)", column(None, "time"), column(Some(0), "V(out)")).with_unit("V")
}

#[test]
fn a_waveform_of_a_segment_that_states_no_digest_is_kept_again_where_a_file_can_name_it() {
    let mut analysis = RetainedAnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0);
    analysis.waveforms = vec![waveform_of_a_segment_without_its_end()];
    let before = analysis.waveforms[0].clone();
    assert!(before.y.is_stored());
    assert!(
        rspice_results::store_reader::ColumnReference::of(&before.y).is_none(),
        "a segment without its end names no digest"
    );
    assert!(holds_samples_in_memory(&analysis), "a file cannot name it");

    store_analysis_in_memory(&mut analysis).unwrap();
    let after = &analysis.waveforms[0];
    assert!(!holds_samples_in_memory(&analysis));
    for (kept, read) in [(&after.x, &before.x), (&after.y, &before.y)] {
        assert!(rspice_results::store_reader::ColumnReference::of(kept).is_some());
        assert_eq!(bits(kept), bits(read));
    }
    assert!(
        after.x.is_ascending_axis(),
        "its time axis is the new segment's own"
    );
}

/// A solver result with an unavailable sample does not validate: the store refuses it and its
/// samples stay in memory, where every save and restore refuses them too.
#[test]
fn a_result_that_does_not_validate_stays_in_memory() {
    let mut analysis = analysis();
    analysis.import_source = None;
    let before = analysis.clone();
    let refused = store_analysis_in_memory(&mut analysis).expect_err("a solver NaN is refused");
    assert!(refused.contains("missing sample"), "{refused}");
    assert!(holds_samples_in_memory(&analysis));
    for (left, right) in before.waveforms.iter().zip(&analysis.waveforms) {
        assert!(
            !right.x.is_stored() && !right.y.is_stored(),
            "{}",
            right.name
        );
        assert_eq!(bits(&right.y), bits(&left.y), "{}", right.name);
    }
}

/// A copy of a run's analyses holding their waveforms alone, without the
/// colour and tick the run keeps with each, made a share at a time as a
/// frame makes it, digests as the run does.
#[test]
fn analyses_copied_apart_from_their_run_digest_as_the_run_does() {
    let traced = |analysis: RetainedAnalysisResult<RetainedWaveform>| {
        let mut traced: RetainedAnalysisResult<rspice_results::waveform::RetainedTrace> =
            analysis.without_waveforms();
        traced.waveforms = analysis
            .waveforms
            .into_iter()
            .map(|waveform| rspice_results::waveform::RetainedTrace {
                data: waveform,
                color: "#123456".to_owned(),
                visible: false,
            })
            .collect();
        traced
    };
    let mut second = analysis();
    second.id = 2;
    let analyses = vec![traced(analysis()), traced(second)];
    let mut copy: Vec<RetainedAnalysisResult<RetainedWaveform>> = Vec::new();
    for analysis in &analyses {
        let mut copied = analysis.without_waveforms();
        for trace in &analysis.waveforms {
            copied.waveforms.push(trace.data.clone());
        }
        copy.push(copied);
    }
    assert_eq!(analyses_digest(&copy), analyses_digest(&analyses));
    let mut other = copy.clone();
    other[1].waveforms.pop();
    assert_ne!(analyses_digest(&other), analyses_digest(&analyses));
}

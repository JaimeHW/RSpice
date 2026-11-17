use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use rspice_results_store::{EventChange, EventSpan, EventValue};

use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_result::AnalysisResult;
use crate::analysis_type::AnalysisType;
use crate::events::points::{EventHistorySource, EventPoints};
use crate::events::{RealEventPointEvidence, RealEventTraceEvidence};
use crate::result_digest::ResultDigestEncoding;
use crate::waveform::{ColumnSource, RetainedWaveform, StoredWindow, WaveformValues, WindowLead};

/// A column or event signal of a store that states its digest, counting its reads; `fails`
/// makes every read fail as a deleted store's does.
struct Store {
    values: Vec<f64>,
    index: u64,
    fails: bool,
    reads: AtomicUsize,
}

impl Store {
    fn new(values: &[f64], index: u64) -> Arc<Self> {
        Self::with(values, index, false)
    }

    fn failing(values: &[f64], index: u64) -> Arc<Self> {
        Self::with(values, index, true)
    }

    fn with(values: &[f64], index: u64, fails: bool) -> Arc<Self> {
        Arc::new(Self {
            values: values.to_vec(),
            index,
            fails,
            reads: AtomicUsize::new(0),
        })
    }

    fn reads(&self) -> usize {
        self.reads.load(Ordering::SeqCst)
    }

    fn change(&self, index: usize) -> EventChange {
        EventChange {
            signal: 0,
            time: index as f64,
            sequence: index as u32,
            value: EventValue::Real(self.values[index]),
            strength: 0,
        }
    }

    fn read_failure(&self) -> Result<(), String> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        if self.fails {
            return Err("the segment was deleted".to_owned());
        }
        Ok(())
    }
}

impl ColumnSource for Store {
    fn name(&self) -> &str {
        "v(out)"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.read_failure()?;
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some(([3; 32], self.index))
    }
}

impl EventHistorySource for Store {
    fn name(&self) -> &str {
        "e(out)"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn count(&self) -> Result<u64, String> {
        Ok(self.values.len() as u64)
    }

    fn before(&self, t: f64) -> Result<u64, String> {
        Ok((0..self.values.len()).filter(|&i| (i as f64) < t).count() as u64)
    }

    fn span(&self, t0: f64, t1: f64) -> Result<EventSpan, String> {
        self.read_failure()?;
        let times = 0..self.values.len();
        Ok(EventSpan {
            before: times
                .clone()
                .rev()
                .find(|&i| (i as f64) < t0)
                .map(|i| self.change(i)),
            within: times
                .clone()
                .filter(|&i| (i as f64) >= t0 && (i as f64) <= t1)
                .map(|i| self.change(i))
                .collect(),
            after: times
                .clone()
                .find(|&i| (i as f64) > t1)
                .map(|i| self.change(i)),
        })
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some(([3; 32], self.index))
    }
}

fn window(source: &Arc<Store>, first: u64, rows: u64) -> WaveformValues {
    let window = StoredWindow {
        first,
        rows,
        lead: WindowLead::None,
    };
    WaveformValues::stored(source.clone(), window).unwrap()
}

fn transient(
    time: &WaveformValues,
    ys: &[WaveformValues],
    events: EventPoints<RealEventPointEvidence>,
) -> AnalysisResult {
    let mut result = AnalysisResult::new(1, AnalysisType::Transient, "tran", 0.0);
    result.waveforms = ys
        .iter()
        .enumerate()
        .map(|(index, y)| RetainedWaveform::new(format!("v({index})"), time.clone(), y.clone()))
        .collect();
    result.result_payload = Some(AnalysisResultPayload::TransientEvents {
        current_impulses: None,
        voltage_impulses: None,
        digital_traces: Vec::new(),
        real_traces: vec![RealEventTraceEvidence {
            node_name: "e(out)".to_owned(),
            points: events,
        }],
        digital_buses: Vec::new(),
        hdl_sources: Vec::new(),
    });
    result
}

/// A file that outlives a run's store, such as a study's results, holds the run's samples
/// itself. Read into memory, a stored result is the same samples and changes, held in memory and
/// digested as those values are; the waveforms that shared one stored time axis share one copy,
/// read once.
#[test]
fn a_stored_result_read_into_memory_holds_the_same_samples_and_digests_as_them() {
    let time = Store::new(&[0.0, 1.0, 2.0, 3.0, 4.0], 0);
    let a = Store::new(&[10.0, 11.0, 12.0, 13.0, 14.0], 1);
    let b = Store::new(&[20.0, 21.0, 22.0, 23.0, 24.0], 2);
    let signal = Store::new(&[0.5, 1.5, 2.5], 3);
    let axis = window(&time, 1, 3);
    let mut stored = transient(
        &axis,
        &[window(&a, 1, 3), window(&b, 1, 3)],
        EventPoints::stored(signal.clone(), 1.0).unwrap(),
    );
    let memory = transient(
        &WaveformValues::from(vec![1.0, 2.0, 3.0]),
        &[
            WaveformValues::from(vec![11.0, 12.0, 13.0]),
            WaveformValues::from(vec![21.0, 22.0, 23.0]),
        ],
        [(1.0, 1.5), (2.0, 2.5)]
            .into_iter()
            .map(|(time_s, value)| RealEventPointEvidence { time_s, value })
            .collect(),
    );
    let digest = |result: &AnalysisResult| {
        result
            .result_data_ref()
            .digest(ResultDigestEncoding::CURRENT)
    };
    assert_ne!(
        digest(&stored),
        digest(&memory),
        "a stored result digests by its store's identity"
    );

    stored.read_stored_into_memory().unwrap();

    for values in stored
        .waveforms
        .iter()
        .flat_map(|waveform| [&waveform.x, &waveform.y])
    {
        assert!(!values.is_stored(), "a window still reads the store");
    }
    let Some(AnalysisResultPayload::TransientEvents { real_traces, .. }) = &stored.result_payload
    else {
        panic!("the events payload went");
    };
    assert!(!real_traces[0].points.is_stored());
    assert!(
        WaveformValues::ptr_eq(&stored.waveforms[0].x, &stored.waveforms[1].x),
        "the waveforms that shared a time axis share its copy"
    );
    assert_eq!(time.reads(), 1, "the shared time axis was read once");
    assert_eq!(digest(&stored), digest(&memory));
    assert_eq!(
        stored.waveforms[1].y.try_values().unwrap(),
        &vec![21.0, 22.0, 23.0]
    );

    // A result held in memory already is left as it is.
    let mut again = stored.clone();
    again.read_stored_into_memory().unwrap();
    assert!(WaveformValues::ptr_eq(
        &again.waveforms[0].y,
        &stored.waveforms[0].y
    ));
}

/// A store that cannot give its rows back fails the result read into memory and names the
/// column, rather than leaving unavailable samples where it wrote values.
#[test]
fn a_store_that_cannot_be_read_fails_the_result_read_into_memory() {
    let time = Store::new(&[0.0, 1.0, 2.0], 0);
    let deleted = Store::failing(&[5.0, 6.0, 7.0], 1);
    let mut stored = transient(
        &window(&time, 0, 3),
        &[window(&deleted, 0, 3)],
        EventPoints::from(Vec::new()),
    );
    let failure = stored.read_stored_into_memory().unwrap_err();
    assert_eq!(failure.column, "v(out)");
    assert!(failure.cause.contains("deleted"), "{failure}");

    let signal = Store::failing(&[0.5, 1.5], 2);
    let mut stored = transient(
        &WaveformValues::from(vec![0.0]),
        &[WaveformValues::from(vec![1.0])],
        EventPoints::stored(signal, 0.0).unwrap(),
    );
    assert!(stored.read_stored_into_memory().is_err());
}

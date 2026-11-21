use std::sync::{Arc, Mutex};

use rspice_core::abort_signal::{AbortSignal, TransientSample};
use rspice_results::sampling::SampleInterpolation;
use rspice_simulation_contract::config::{AnalysisConfig, TransientAnalysisConfig};

use super::*;
use crate::results::SimulationResult;

/// A step into an RC low-pass of a 1 ms time constant, solved in steps of at most 0.5 us, so the
/// run writes several chunks: V(out) = 1 - exp(-t / 1 ms).
const DECK: &str = "live segment
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 1u
.tran 0.5u 5m 0 0.5u
.end
";

const TAU: f64 = 1e-3;

/// Opens the run's live segment at chosen accepted points, as the host does while it runs.
struct Opener {
    slot: crate::results_store::LiveSegmentSlot,
    seen: Mutex<Vec<LiveRows>>,
}

impl AbortSignal for Opener {
    fn is_aborted(&self) -> bool {
        false
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        if sample.points % 2_500 != 0 {
            return;
        }
        let path = self.slot.lock().unwrap().clone();
        let path = path.expect("the running transient names its segment");
        if let Some(rows) = open_live_rows(&path).expect("a growing segment opens") {
            self.seen.lock().unwrap().push(rows);
        }
    }
}

fn values(values: &WaveformValues) -> Vec<f64> {
    values.read_whole().unwrap().into_owned()
}

#[test]
fn a_running_transient_is_drawn_from_the_rows_its_segment_holds() {
    let slot = crate::results_store::LiveSegmentSlot::default();
    let _installed = crate::results_store::install_live_segment(Arc::clone(&slot));
    let opener = Opener {
        slot: Arc::clone(&slot),
        seen: Mutex::new(Vec::new()),
    };
    let config = AnalysisConfig::Transient(TransientAnalysisConfig {
        stop_time: 5e-3,
        step_time: 0.5e-6,
        start_time: 0.0,
        max_timestep: Some(0.5e-6),
        uic: false,
    });
    let result = super::super::super::EngineBridge::new()
        .run_with_abort(&config, DECK, &opener)
        .unwrap();
    assert!(
        slot.lock().unwrap().is_none(),
        "the slot is cleared once the run is done writing"
    );
    let SimulationResult::Transient {
        time, waveforms, ..
    } = result
    else {
        panic!("a transient");
    };
    let final_time = values(&time);
    let final_out = values(
        &waveforms
            .iter()
            .find(|(name, _)| {
                name.eq_ignore_ascii_case("out") || name.eq_ignore_ascii_case("V(out)")
            })
            .unwrap_or_else(|| panic!("V(out) among {:?}", waveforms.keys().collect::<Vec<_>>()))
            .1
            .y_values,
    );
    let seen = opener.seen.into_inner().unwrap();
    assert!(
        seen.iter().any(|live| live.rows > 0),
        "the rows are readable while the run writes them"
    );
    let mut previous = 0;
    for live in &seen {
        assert!(live.rows >= previous, "the segment only grows");
        previous = live.rows;
        let out = live
            .waveforms
            .iter()
            .find(|waveform| {
                waveform.name.eq_ignore_ascii_case("out")
                    || waveform.name.eq_ignore_ascii_case("V(out)")
            })
            .expect("each saved node is a waveform");
        assert_eq!(out.unit.as_deref(), Some("V"));
        assert!(out.x.is_stored() && out.y.is_stored());
        let x = values(&out.x);
        let y = values(&out.y);
        assert_eq!(x.len() as u64, live.rows);
        assert_eq!(x.last().copied(), Some(live.last_time));
        // The rows written so far are the run's own first rows, value for value.
        assert_eq!(x[..], final_time[..x.len()]);
        assert_eq!(y[..], final_out[..y.len()]);
        for (&t, &v) in x.iter().zip(&y).skip(1) {
            let expected = 1.0 - (-(t - 1e-9).max(0.0) / TAU).exp();
            assert!(
                (v - expected).abs() < 1e-4,
                "V(out) at {t}: {v} vs {expected}"
            );
        }
        // A view over every row reaches the last row written, which the segment's last chunk
        // holds: the charging curve's greatest value is its last.
        let (first, last) = (x[0], *x.last().unwrap());
        match out
            .y
            .tile_over(&out.x, first, last, 16)
            .expect("a stored column tiles")
        {
            ColumnTile::Envelope { max, .. } => {
                assert_eq!(max.last().copied(), y.last().copied());
            }
            ColumnTile::Samples { y: tiled, .. } => {
                assert_eq!(tiled.last().copied(), y.last().copied());
            }
        }
        // A cursor between the last two rows reads between them.
        if x.len() >= 2 {
            let n = x.len();
            let at = 0.5 * (x[n - 2] + x[n - 1]);
            let read = out
                .y
                .value_at(&out.x, at, SampleInterpolation::Linear)
                .expect("a stored column reads at a cursor");
            assert!((read - 0.5 * (y[n - 2] + y[n - 1])).abs() < 1e-12);
        }
    }
}

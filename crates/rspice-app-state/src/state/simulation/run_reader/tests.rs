//! The reader service reads stored series off the calling thread and holds at most its budget,
//! the runs on screen keeping their reads longest.

use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;
use crate::state::simulation::run_store::store_analysis_in_memory;
use rspice_results::analysis_result::AnalysisResult as RetainedAnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::sampling::SampleInterpolation;
use rspice_results::waveform::RetainedWaveform;

const ROWS: usize = 20_000;

/// Run `run`'s transient of two signals on one axis, kept in a store held in memory.
fn stored_run(run: u64) -> RetainedAnalysisResult<RetainedWaveform> {
    let time: Vec<f64> = (0..ROWS).map(|row| row as f64 * 1e-9).collect();
    let mut analysis = RetainedAnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0);
    analysis.waveforms = (0..2)
        .map(|signal| {
            let y: Vec<f64> = time
                .iter()
                .map(|t| (t * 1e6 * (signal + 1) as f64).sin() + run as f64)
                .collect();
            RetainedWaveform::new(format!("V(n{signal})"), time.clone(), y)
        })
        .collect();
    store_analysis_in_memory(&mut analysis).unwrap();
    analysis
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn the_memory_budget_holds_with_twenty_retained_runs_and_runs_on_screen_go_last() {
    const BUDGET: u64 = 1 << 20;
    let runs: Vec<_> = (0..20).map(stored_run).collect();
    let reader = RunReader::new(BUDGET);
    let arrived = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&arrived);
    // A waker that takes its time, as a busy frame loop does: a read that settled has had its
    // frame asked for all the same.
    reader.set_waker(move || {
        std::thread::sleep(std::time::Duration::from_millis(2));
        counted.fetch_add(1, Ordering::Relaxed);
    });
    reader.show([19]);
    let whole = Read::Range {
        first: 0,
        count: ROWS,
    };
    let key = |run: usize, signal: usize| {
        ReadKey::of(run as u64, &runs[run].waveforms[signal].y, whole).unwrap()
    };
    // The run on screen is read first, so it holds the oldest reads.
    for run in std::iter::once(19).chain(0..19) {
        for waveform in &runs[run].waveforms {
            assert_eq!(
                reader.request(run as u64, &waveform.y, &waveform.x, whole),
                None,
                "a stored read is not made on the asking thread"
            );
            reader.settle();
            assert!(
                reader.loaded_bytes() <= BUDGET,
                "{} bytes loaded over a budget of {BUDGET}",
                reader.loaded_bytes()
            );
        }
    }
    assert_eq!(
        arrived.load(Ordering::Relaxed),
        40,
        "each read asked for a frame"
    );

    for signal in 0..2 {
        let Some(ReadValue::Samples(samples)) = reader.get(&key(19, signal)) else {
            panic!("the run on screen keeps its reads");
        };
        let expected = runs[19].waveforms[signal].y.read_range(0..ROWS).unwrap();
        assert_eq!(bits(&samples), bits(&expected));
    }
    assert!(reader.get(&key(18, 1)).is_some(), "the latest read is held");
    assert!(
        reader.get(&key(0, 0)).is_none(),
        "the oldest read of a run off screen went first"
    );
    let first = reader.request(
        18,
        &runs[18].waveforms[1].y,
        &runs[18].waveforms[1].x,
        whole,
    );
    assert!(
        matches!(first, Some(ReadValue::Samples(_))),
        "a held read comes back at once"
    );

    // A point and a tile read the run's own samples.
    let (x, y) = (&runs[3].waveforms[0].x, &runs[3].waveforms[0].y);
    let at = Read::Point {
        x: x.read_range(100..101).unwrap()[0],
        interpolation: SampleInterpolation::Linear,
    };
    reader.request(3, y, x, at);
    let tile = Read::Tile {
        x0: 0.0,
        x1: 1e-5,
        pixels: 100,
    };
    reader.request(3, y, x, tile);
    reader.settle();
    assert_eq!(
        reader.get(&ReadKey::of(3, y, at).unwrap()),
        Some(ReadValue::Point(Some(y.read_range(100..101).unwrap()[0])))
    );
    assert!(matches!(
        reader.get(&ReadKey::of(3, y, tile).unwrap()),
        Some(ReadValue::Tile(_))
    ));

    // A smaller budget lets go at once, and a read larger than it is refused with a reason.
    reader.set_budget(64 * 1024);
    assert!(reader.loaded_bytes() <= 64 * 1024);
    reader.request(5, &runs[5].waveforms[0].y, &runs[5].waveforms[0].x, whole);
    reader.settle();
    let Some(ReadValue::Failed(reason)) = reader.get(&key(5, 0)) else {
        panic!("a read past the budget is refused");
    };
    assert!(reason.contains("more than"), "{reason}");
    assert!(reader.loaded_bytes() <= 64 * 1024);

    reader.release_run(3);
    assert!(reader.get(&ReadKey::of(3, y, at).unwrap()).is_none());
}

#[test]
fn samples_held_in_memory_are_read_as_they_are_asked() {
    let reader = RunReader::new(1 << 20);
    let x: WaveformValues = vec![0.0, 1.0, 2.0].into();
    let y: WaveformValues = vec![4.0, 5.0, 6.0].into();
    assert_eq!(
        reader.request(1, &y, &x, Read::Range { first: 1, count: 5 }),
        Some(ReadValue::Samples(vec![5.0, 6.0].into()))
    );
    assert_eq!(reader.loaded_bytes(), 0, "memory reads are not the cache's");
}

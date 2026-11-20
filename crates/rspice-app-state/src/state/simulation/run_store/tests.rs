//! A finished analysis written to the results store: every series becomes a window of a
//! segment's column with the same samples, waveforms on one axis share one segment and one axis
//! window, and an axis that does not ascend is kept as a column of its own.

use super::*;
use rspice_results::analysis_type::AnalysisType;

fn bits(values: &WaveformValues) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// An AC result of two complex traces on one frequency axis, and a DC trace on a sweep that runs
/// down, each axis its own allocation as a converter makes them.
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

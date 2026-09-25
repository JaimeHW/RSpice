//! Pure frequency-response/Bode summary extraction shared by Results and
//! netlist summaries.

use std::sync::Arc;

use crate::product::AnalysisInstanceId;

use super::{AnalysisResult, AnalysisType, SharedWaveformValues, SimulationRun, WaveformData};

pub use rspice_results::bode::AcBodeMetrics;
#[cfg(test)]
use rspice_results::bode::low_frequency_gain_is_dc;
use rspice_results::bode::metrics_from_curves;

#[derive(Debug, Clone, PartialEq)]
pub struct AcBodeSummary {
    pub signal: String,
    pub frequency: SharedWaveformValues,
    pub gain_db: SharedWaveformValues,
    pub phase_deg: Option<SharedWaveformValues>,
    pub metrics: AcBodeMetrics,
    pub analysis_index: usize,
    pub mag_index: usize,
    pub phase_index: Option<usize>,
}

/// Which retained traces a frequency response is made of, and nothing more.
///
/// The shape question — is there a magnitude trace with a phase trace that
/// matches it? — is answered from names and indices alone, in time
/// proportional to the number of traces rather than the number of samples.
/// [`AcBodeSummary`] is the shape plus the measurement: a decibel conversion
/// of the whole magnitude vector, an unwrapped copy of the phase, and every
/// crossing search over both. Asking the shape question through the summary
/// meant a caller who only wanted to know whether a Bode sheet could be
/// offered paid for all of it — once per analysis in the run, on every frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcBodeShape {
    pub signal: String,
    pub analysis_index: usize,
    pub mag_index: usize,
    pub phase_index: Option<usize>,
}

/// The run's normal frequency response: the first one carrying traces.
fn first_response(run: &SimulationRun) -> Option<(usize, &AnalysisResult)> {
    run.analyses.iter().enumerate().find(|(_, analysis)| {
        analysis.analysis_type.is_bode_response() && !analysis.waveforms.is_empty()
    })
}

/// The response produced by one exact prepared analysis instance.
///
/// Analysis kind and display label are deliberately not used as identity:
/// both can be identical when a run contains multiple AC configurations.
fn response_by_source_instance(
    run: &SimulationRun,
    source_instance_id: AnalysisInstanceId,
) -> Option<(usize, &AnalysisResult)> {
    run.analyses.iter().enumerate().find(|(_, analysis)| {
        analysis
            .provenance
            .as_ref()
            .is_some_and(|provenance| provenance.source_instance_id() == source_instance_id)
    })
}

/// The response a result browser's selection names.
///
/// A selected AC result with current provenance is re-resolved through its
/// stable prepared-instance identity. Legacy results, which predate that
/// identity, remain addressable by their run-local index. When the current
/// selection is not a frequency response, the run's normal response fallback
/// is retained.
fn selected_response(
    run: &SimulationRun,
    selected_analysis_index: Option<usize>,
) -> Option<(usize, &AnalysisResult)> {
    let Some(analysis_index) = selected_analysis_index else {
        return first_response(run);
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return first_response(run);
    };
    if !analysis.analysis_type.is_bode_response() {
        return first_response(run);
    }
    match analysis.provenance.as_ref() {
        Some(provenance) => response_by_source_instance(run, provenance.source_instance_id()),
        None => Some((analysis_index, analysis)),
    }
}

/// Resolve the frequency response selected in a result browser; see
/// [`selected_response`].
pub fn ac_bode_summary_for_selection(
    run: &SimulationRun,
    selected_analysis_index: Option<usize>,
) -> Option<AcBodeSummary> {
    let (analysis_index, analysis) = selected_response(run, selected_analysis_index)?;
    ac_bode_summary_for_analysis(analysis, analysis_index)
}

/// The shape of the response a result browser's selection names, without
/// measuring it; see [`AcBodeShape`].
pub fn ac_bode_shape_for_selection(
    run: &SimulationRun,
    selected_analysis_index: Option<usize>,
) -> Option<AcBodeShape> {
    let (analysis_index, analysis) = selected_response(run, selected_analysis_index)?;
    ac_bode_shape_for_analysis(analysis, analysis_index)
}

/// Which of one analysis' retained traces form a frequency response.
///
/// No sample is read: the answer is the magnitude trace this module would
/// measure and the phase trace named after it, if the run retained one.
pub fn ac_bode_shape_for_analysis(
    analysis: &AnalysisResult,
    analysis_index: usize,
) -> Option<AcBodeShape> {
    if !analysis.analysis_type.is_bode_response() || analysis.waveforms.is_empty() {
        return None;
    }
    let (signal, mag_index, phase_index) = if analysis.analysis_type == AnalysisType::Stb {
        let mag_index = analysis
            .waveforms
            .iter()
            .position(|waveform| waveform.name == "Loop Gain (dB)")?;
        let phase_index = analysis
            .waveforms
            .iter()
            .position(|waveform| waveform.name == "Loop Phase (deg)");
        ("Loop Gain".to_owned(), mag_index, phase_index)
    } else {
        let (mag_index, mag) = select_magnitude_trace(&analysis.waveforms)?;
        let signal = mag
            .name
            .trim_start_matches('|')
            .trim_end_matches('|')
            .to_owned();
        let phase_name = format!("phase({signal})");
        let phase_index = analysis
            .waveforms
            .iter()
            .position(|waveform| waveform.name == phase_name);
        (signal, mag_index, phase_index)
    };
    Some(AcBodeShape {
        signal,
        analysis_index,
        mag_index,
        phase_index,
    })
}

pub fn ac_bode_summary_for_analysis(
    analysis: &AnalysisResult,
    analysis_index: usize,
) -> Option<AcBodeSummary> {
    // A translated multi-tone transfer is not a scalar feedback loop.
    // Its signed offset axis does not support ordinary AC bandwidth/margin claims.
    if matches!(
        analysis.analysis_type,
        AnalysisType::Qpac | AnalysisType::Qpxf
    ) {
        return None;
    }
    let AcBodeShape {
        signal,
        analysis_index,
        mag_index,
        phase_index,
    } = ac_bode_shape_for_analysis(analysis, analysis_index)?;

    let mag = analysis.waveforms.get(mag_index)?;
    // STB retains decibels; every other response retains linear magnitude.
    let gain_db = if analysis.analysis_type == AnalysisType::Stb {
        Arc::clone(&mag.y)
    } else {
        magnitude_to_db(&mag.y)
    };
    let frequency = Arc::clone(&mag.x);
    let phase_deg = phase_index
        .and_then(|index| analysis.waveforms.get(index))
        .map(|waveform| Arc::clone(&waveform.y));
    let metrics = metrics_from_curves(
        frequency.as_slice(),
        gain_db.as_slice(),
        phase_deg.as_ref().map(|values| values.as_slice()),
    );

    Some(AcBodeSummary {
        signal,
        frequency,
        gain_db,
        phase_deg,
        metrics,
        analysis_index,
        mag_index,
        phase_index,
    })
}

fn select_magnitude_trace(waveforms: &[WaveformData]) -> Option<(usize, &WaveformData)> {
    waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform.name.starts_with('|'))
        .max_by_key(|(_, waveform)| waveform.visible)
}

fn magnitude_to_db(magnitude: &[f64]) -> SharedWaveformValues {
    Arc::new(
        magnitude
            .iter()
            .map(|&m| 20.0 * m.log10())
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision};

    fn ac_analysis(waveforms: Vec<WaveformData>) -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(waveforms)
    }

    fn response_analysis(
        analysis_type: AnalysisType,
        waveforms: Vec<WaveformData>,
    ) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, analysis_type.short_label()).with_waveforms(waveforms)
    }

    fn wave(name: &str, x: &[f64], y: &[f64], visible: bool) -> WaveformData {
        let mut waveform = WaveformData::new(name, x.to_vec(), y.to_vec(), "#fff");
        waveform.visible = visible;
        waveform
    }

    fn assert_close(actual: Option<f64>, expected: f64) {
        let actual = actual.expect("metric exists");
        assert!(
            (actual - expected).abs() < 1.0e-9,
            "expected {expected}, got {actual}"
        );
    }

    // -------------------------------------------------------------------
    // Analytic oracles
    //
    // Margins are checked against closed-form loop transmissions and against
    // exactly log-affine synthetic curves, never against a recorded output of
    // this module. The closed-form crossings are located by bisection on the
    // analytic response itself, which shares no code with the extraction
    // under test.
    // -------------------------------------------------------------------

    /// `L(s) = 10^(k_db/20) / Π(1 + s/pᵢ)` — a real multi-pole loop
    /// transmission with closed-form magnitude and phase.
    struct PoleLoop {
        k_db: f64,
        poles: &'static [f64],
    }

    impl PoleLoop {
        fn gain_db(&self, f: f64) -> f64 {
            self.k_db
                - self
                    .poles
                    .iter()
                    .map(|p| 10.0 * (1.0 + (f / p).powi(2)).log10())
                    .sum::<f64>()
        }

        fn phase_deg(&self, f: f64) -> f64 {
            -self
                .poles
                .iter()
                .map(|p| (f / p).atan().to_degrees())
                .sum::<f64>()
        }

        /// Bisect in log-frequency for the root of `probe`, which must change
        /// sign exactly once over `[lo, hi]`.
        fn bisect(&self, lo: f64, hi: f64, probe: impl Fn(&Self, f64) -> f64) -> f64 {
            let (mut l, mut h) = (lo.log10(), hi.log10());
            assert!(
                probe(self, 10f64.powf(l)) * probe(self, 10f64.powf(h)) < 0.0,
                "oracle bracket does not straddle the root"
            );
            for _ in 0..200 {
                let m = 0.5 * (l + h);
                if probe(self, 10f64.powf(l)) * probe(self, 10f64.powf(m)) <= 0.0 {
                    h = m;
                } else {
                    l = m;
                }
            }
            10f64.powf(0.5 * (l + h))
        }

        fn unity_gain(&self, lo: f64, hi: f64) -> f64 {
            self.bisect(lo, hi, |loop_, f| loop_.gain_db(f))
        }

        fn phase_crossing(&self, level: f64, lo: f64, hi: f64) -> f64 {
            self.bisect(lo, hi, move |loop_, f| loop_.phase_deg(f) - level)
        }
    }

    fn decade_sweep(f0: f64, f1: f64, per_decade: usize) -> Vec<f64> {
        let (l0, l1) = (f0.log10(), f1.log10());
        let steps = ((l1 - l0) * per_decade as f64).round() as usize;
        (0..=steps)
            .map(|i| 10f64.powf(l0 + (l1 - l0) * i as f64 / steps as f64))
            .collect()
    }

    /// A `lin` sweep: `points` samples spaced evenly *in frequency*, which is
    /// what the STB, PAC, PNOISE and PXF dialogs emit.
    fn linear_sweep(f0: f64, f1: f64, points: usize) -> Vec<f64> {
        (0..points)
            .map(|i| f0 + (f1 - f0) * i as f64 / (points - 1) as f64)
            .collect()
    }

    /// The exact range `imag.atan2(real).to_degrees()` produces: `(-180, 180]`.
    fn wrap_deg(phase: f64) -> f64 {
        let wrapped = (phase + 180.0).rem_euclid(360.0) - 180.0;
        if wrapped == -180.0 { 180.0 } else { wrapped }
    }

    /// Build the retained trace pair exactly as `results_convert` does: linear
    /// magnitude under `|…|`, and a `(-180, 180]`-wrapped phase.
    fn wrapped_response(frequency: &[f64], gain_db: &[f64], phase_deg: &[f64]) -> AnalysisResult {
        let magnitude = gain_db
            .iter()
            .map(|db| 10f64.powf(db / 20.0))
            .collect::<Vec<_>>();
        let wrapped = phase_deg.iter().copied().map(wrap_deg).collect::<Vec<_>>();
        ac_analysis(vec![
            wave("|V(out)|", frequency, &magnitude, true),
            wave("phase(V(out))", frequency, &wrapped, true),
        ])
    }

    fn loop_response(loop_: &PoleLoop, frequency: &[f64]) -> AnalysisResult {
        let gain = frequency
            .iter()
            .map(|&f| loop_.gain_db(f))
            .collect::<Vec<_>>();
        let phase = frequency
            .iter()
            .map(|&f| loop_.phase_deg(f))
            .collect::<Vec<_>>();
        wrapped_response(frequency, &gain, &phase)
    }

    #[track_caller]
    fn assert_within(actual: Option<f64>, expected: f64, tolerance: f64, what: &str) {
        let actual = actual.unwrap_or_else(|| panic!("{what} was not reported at all"));
        assert!(
            (actual - expected).abs() <= tolerance,
            "{what}: expected {expected}, got {actual} (tolerance {tolerance})"
        );
    }

    #[track_caller]
    fn assert_relative(actual: Option<f64>, expected: f64, tolerance: f64, what: &str) {
        let actual = actual.unwrap_or_else(|| panic!("{what} was not reported at all"));
        let error = (actual - expected).abs() / expected.abs();
        assert!(
            error <= tolerance,
            "{what}: expected {expected}, got {actual} (relative error {error:e})"
        );
    }

    /// A two-pole loop can never reach -180°, so it has no gain margin at all.
    /// Reporting one would be an invention; the unity-gain frequency and phase
    /// margin still have to match the closed form.
    #[test]
    fn two_pole_loop_matches_closed_form_and_reports_no_gain_margin() {
        let loop_ = PoleLoop {
            k_db: 80.0,
            poles: &[10.0, 2.0e4],
        };
        let frequency = decade_sweep(1.0, 1.0e7, 10);
        let analysis = loop_response(&loop_, &frequency);

        let ugf = loop_.unity_gain(1.0, 1.0e7);
        let pm = 180.0 + loop_.phase_deg(ugf);

        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;

        assert_relative(metrics.ugf, ugf, 5.0e-3, "unity-gain frequency");
        assert_within(metrics.pm_deg, pm, 0.5, "phase margin");
        assert_eq!(metrics.f180, None, "a two-pole loop never reaches -180°");
        assert_eq!(
            metrics.gm_db, None,
            "no -180° crossing means no gain margin"
        );
    }

    /// The headline case. A three-pole loop's phase passes -180° between two
    /// samples, so the retained wrapped trace jumps from ≈-180° to ≈+180°.
    /// f₁₈₀ and the gain margin exist and must be found.
    #[test]
    fn three_pole_loop_recovers_margins_across_the_wrapped_phase_jump() {
        let loop_ = PoleLoop {
            k_db: 60.0,
            poles: &[10.0, 1.0e4, 1.0e5],
        };
        let frequency = decade_sweep(1.0, 1.0e7, 10);
        let analysis = loop_response(&loop_, &frequency);

        let ugf = loop_.unity_gain(1.0, 1.0e7);
        let pm = 180.0 + loop_.phase_deg(ugf);
        let f180 = loop_.phase_crossing(-180.0, 1.0e3, 1.0e6);
        let gm = -loop_.gain_db(f180);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");
        // The retained trace really does wrap: this is what the extraction has
        // to see through.
        let phase = summary.phase_deg.as_ref().expect("phase trace");
        assert!(
            phase.windows(2).any(|w| (w[1] - w[0]).abs() > 180.0),
            "the oracle trace must contain a wrapped jump"
        );
        let metrics = summary.metrics;

        assert_relative(metrics.ugf, ugf, 5.0e-3, "unity-gain frequency");
        assert_within(metrics.pm_deg, pm, 0.5, "phase margin");
        assert_relative(metrics.f180, f180, 5.0e-3, "f180");
        assert_within(metrics.gm_db, gm, 0.1, "gain margin");
    }

    /// The same defect with every interpolation error removed: both curves are
    /// exactly affine in log-frequency, so log-consistent extraction is exact
    /// to machine precision. The wrap sits between the second and third
    /// samples, and the true response is unstable — a reading of "stable"
    /// here is the failure this pins.
    #[test]
    fn wrapped_jump_between_samples_yields_the_exact_unstable_margins() {
        // slope -80°/decade, -15 dB/decade
        let frequency = [1.0e2, 1.0e3, 1.0e4, 1.0e5];
        let gain_db = [20.0, 5.0, -10.0, -25.0];
        let phase_deg = [-90.0, -170.0, -250.0, -330.0];
        let analysis = wrapped_response(&frequency, &gain_db, &phase_deg);

        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;

        // gain = 0 dB at log10 f = 3 + 5/15
        assert_relative(metrics.ugf, 10f64.powf(3.0 + 1.0 / 3.0), 1.0e-12, "UGF");
        // phase at UGF = -170 - 80/3, so the loop is unstable by 16.7°.
        assert_within(metrics.pm_deg, 180.0 - 170.0 - 80.0 / 3.0, 1.0e-9, "PM");
        // phase = -180° at log10 f = 3 + 10/80
        assert_relative(metrics.f180, 10f64.powf(3.125), 1.0e-12, "f180");
        // gain at f180 = 5 - 15/8
        assert_within(metrics.gm_db, -(5.0 - 15.0 / 8.0), 1.0e-9, "GM");
    }

    /// Phase and gain are read at the located crossings the same way the
    /// crossings themselves are located: log-in-frequency. A linear-in-
    /// frequency read of a log-affine curve is wrong by a fixed, large amount.
    #[test]
    fn crossing_samples_are_read_log_consistently_with_the_crossing_search() {
        // -20 dB/decade, -50°/decade, no wrap anywhere in range.
        let frequency = [1.0e2, 1.0e3, 1.0e4, 1.0e5];
        let gain_db = [30.0, 10.0, -10.0, -30.0];
        let phase_deg = [-20.0, -70.0, -120.0, -170.0];
        let analysis = wrapped_response(&frequency, &gain_db, &phase_deg);

        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;

        // gain = 0 dB at log10 f = 3.5, where the phase is exactly -95°.
        assert_relative(metrics.ugf, 10f64.powf(3.5), 1.0e-12, "UGF");
        assert_within(metrics.pm_deg, 85.0, 1.0e-9, "PM");
        assert_eq!(metrics.f180, None, "the phase never reaches -180°");
    }

    /// Every -180° crossing of an affine phase trace, paired with the signed
    /// gain margin there, computed from the closed forms the fixture is built
    /// from rather than from anything this module produces.
    fn closed_form_margins(phase_deg: &[f64], gain_at: impl Fn(f64) -> f64) -> Vec<(f64, f64)> {
        phase_deg
            .windows(2)
            .enumerate()
            .filter(|(_, w)| (w[0] + 180.0) * (w[1] + 180.0) < 0.0)
            .map(|(i, w)| {
                let log_f = i as f64 + (-180.0 - w[0]) / (w[1] - w[0]);
                (log_f, -gain_at(log_f))
            })
            .collect()
    }

    /// A conditionally stable loop dips below -180° while the gain is still
    /// above 0 dB and comes back. Every -180° crossing is a real one; the card
    /// reports the crossing that *binds* — the smallest gain change that
    /// reaches instability, `min |GM|` — and f₁₈₀ names that same crossing.
    ///
    /// This loop is healthy with 2 dB in hand. Reporting the deepest crossing
    /// instead reads -26.7 dB, which is the whole conditionally stable hump
    /// mistaken for a defect.
    #[test]
    fn conditionally_stable_loop_reports_the_binding_gain_margin() {
        // Both curves are exactly affine in log-frequency: gain falls
        // -10 dB/decade from 40 dB, and the phase is affine per decade.
        let gain_at = |log_f: f64| 40.0 - 10.0 * log_f;
        let phase_deg = [-100.0, -160.0, -220.0, -220.0, -160.0, -260.0];
        let frequency = (0..phase_deg.len())
            .map(|i| 10f64.powi(i as i32))
            .collect::<Vec<_>>();
        let gain_db = (0..phase_deg.len())
            .map(|i| gain_at(i as f64))
            .collect::<Vec<_>>();
        let analysis = wrapped_response(&frequency, &gain_db, &phase_deg);

        let margins = closed_form_margins(&phase_deg, gain_at);
        assert_eq!(margins.len(), 3, "the fixture must invert three times");
        let binding = margins
            .iter()
            .copied()
            .min_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .expect("a binding crossing");
        let deepest = margins
            .iter()
            .copied()
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .expect("a deepest crossing");
        assert_ne!(
            binding.0, deepest.0,
            "the fixture must separate the binding crossing from the deepest one"
        );

        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;

        // One 0 dB crossing, landing exactly on the 10 kHz sample.
        assert_relative(metrics.ugf, 1.0e4, 1.0e-12, "UGF");
        assert_within(metrics.pm_deg, 180.0 + phase_deg[4], 1.0e-9, "PM");
        assert_within(metrics.gm_db, binding.1, 1.0e-9, "GM");
        assert_relative(metrics.f180, 10f64.powf(binding.0), 1.0e-12, "f180");
    }

    /// The binding crossing is the one nearest unity gain, not the one with the
    /// friendlier sign. Both -180° crossings here are already past the margin,
    /// and the shallower one is still the number to act on: it takes 13.3 dB of
    /// gain reduction to clear it and 23.3 dB to clear the other.
    #[test]
    fn the_binding_gain_margin_is_the_shallowest_when_every_crossing_is_negative() {
        let gain_at = |log_f: f64| 30.0 - 10.0 * log_f;
        let phase_deg = [-100.0, -220.0, -160.0, -170.0];
        let frequency = (0..phase_deg.len())
            .map(|i| 10f64.powi(i as i32))
            .collect::<Vec<_>>();
        let gain_db = (0..phase_deg.len())
            .map(|i| gain_at(i as f64))
            .collect::<Vec<_>>();
        let analysis = wrapped_response(&frequency, &gain_db, &phase_deg);

        let margins = closed_form_margins(&phase_deg, gain_at);
        assert_eq!(margins.len(), 2, "the fixture must invert twice");
        assert!(
            margins.iter().all(|&(_, gm)| gm < 0.0),
            "both crossings must be past the margin, so sign cannot select one"
        );
        let binding = margins
            .iter()
            .copied()
            .min_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .expect("a binding crossing");
        assert_ne!(
            binding.0, margins[0].0,
            "the binding crossing must not be the deepest one"
        );

        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;

        assert_relative(metrics.f180, 10f64.powf(binding.0), 1.0e-12, "f180");
        assert_within(metrics.gm_db, binding.1, 1.0e-9, "GM");
    }

    /// Gain can pass through 0 dB more than once. The phase margin is measured
    /// at the crossing that binds — the smallest `|PM|` — and the unity-gain
    /// frequency names that same crossing, so the two rows never describe
    /// different points on the curve.
    #[test]
    fn multiple_unity_gain_crossings_report_the_binding_phase_margin() {
        // Gain dips through 0 dB, comes back up, and falls through again.
        let gain_db = [20.0, 10.0, -10.0, 10.0, -20.0];
        let phase_deg = [-20.0, -60.0, -100.0, -140.0, -190.0];
        let frequency = (0..gain_db.len())
            .map(|i| 10f64.powi(i as i32))
            .collect::<Vec<_>>();
        let analysis = wrapped_response(&frequency, &gain_db, &phase_deg);

        // Every 0 dB crossing of the affine gain, with the phase there.
        let crossings = gain_db
            .windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] * w[1] < 0.0)
            .map(|(i, w)| {
                let t = w[0] / (w[0] - w[1]);
                let log_f = i as f64 + t;
                let phase = phase_deg[i] + t * (phase_deg[i + 1] - phase_deg[i]);
                (log_f, 180.0 + phase)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            crossings.len(),
            3,
            "the fixture must cross unity three times"
        );
        let binding = crossings
            .iter()
            .copied()
            .min_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .expect("a binding crossing");
        assert_ne!(
            binding.0, crossings[0].0,
            "the fixture must separate the binding crossing from the first one"
        );

        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;

        assert_relative(metrics.ugf, 10f64.powf(binding.0), 1.0e-12, "UGF");
        assert_within(metrics.pm_deg, binding.1, 1.0e-9, "PM");
    }

    /// `gain_db.first()` is the gain at the first swept frequency. It is only
    /// the DC gain when the sweep provably starts flat.
    #[test]
    fn low_frequency_gain_claims_dc_only_when_the_first_decade_is_flat() {
        let loop_ = PoleLoop {
            k_db: 60.0,
            poles: &[10.0],
        };

        let below_pole = decade_sweep(1.0e-2, 1.0e4, 10);
        let flat = ac_bode_summary_for_analysis(&loop_response(&loop_, &below_pole), 0)
            .expect("AC summary")
            .metrics;
        assert!(
            flat.adc_is_dc,
            "a sweep starting two decades below the pole is flat over its first decade"
        );
        assert_within(flat.adc_db, 60.0, 1.0e-3, "A_dc");

        let above_pole = decade_sweep(1.0e3, 1.0e6, 10);
        let rolled_off = ac_bode_summary_for_analysis(&loop_response(&loop_, &above_pole), 0)
            .expect("AC summary")
            .metrics;
        assert!(
            !rolled_off.adc_is_dc,
            "a sweep starting two decades above the pole is mid-rolloff, not DC"
        );
    }

    /// A `lin` sweep spans its first decade while placing exactly one sample
    /// inside it, and one sample is not evidence of anything: the flatness
    /// window would be comparing that gain against itself.
    ///
    /// This is the same false DC claim the span check exists to stop, reached
    /// through the sweep kind the STB/PAC/PNOISE/PXF dialogs emit rather than
    /// through a short sweep.
    #[test]
    fn a_lone_first_decade_sample_never_proves_flatness() {
        let loop_ = PoleLoop {
            k_db: 60.0,
            poles: &[10.0],
        };
        // 100 points from 1 kHz to 1 MHz: ~10.09 kHz apart, so the second
        // sample already sits above 10 kHz and only f_min lands in the first
        // decade. The sweep opens two decades above the pole, where the gain
        // is ≈20 dB — 40 dB below the DC gain it would be labelled as.
        let frequency = linear_sweep(1.0e3, 1.0e6, 100);
        assert_eq!(
            frequency.iter().filter(|&&f| f <= 1.0e4).count(),
            1,
            "the fixture must place exactly one sample in the first decade"
        );

        let metrics = ac_bode_summary_for_analysis(&loop_response(&loop_, &frequency), 0)
            .expect("AC summary")
            .metrics;

        assert_within(metrics.adc_db, 20.0, 1.0e-3, "A(f_min)");
        assert!(
            !metrics.adc_is_dc,
            "a single first-decade sample proves no flatness, so A(f_min) is \
             not the DC gain"
        );
    }

    /// The minimal shape of the same defect, with the sweep reduced to the two
    /// samples the span check needs: the first decade holds one of them, and
    /// the gain is visibly falling.
    #[test]
    fn one_sample_in_the_first_decade_is_not_a_flat_first_decade() {
        let frequency = [1.0, 100.0];
        let gain_db = [60.0, 40.0];

        assert!(!low_frequency_gain_is_dc(&frequency, &gain_db));

        let analysis = wrapped_response(&frequency, &gain_db, &[0.0, -90.0]);
        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;
        assert!(!metrics.adc_is_dc);
    }

    /// A NaN frequency never reads as a proven DC gain.
    ///
    /// Both span guards are written `f.is_nan() || f <op> bound` rather than
    /// `!(f <op> bound)`. The forms agree, and this pins the arm that carries
    /// the meaning: without it a NaN endpoint fails every comparison, passes
    /// both guards, and the summary claims A(f_min) is the DC gain on a sweep
    /// whose extent is not known. "Not provably DC" is the answer this
    /// function owes when it cannot tell.
    #[test]
    fn a_nan_frequency_endpoint_is_never_a_proven_dc_gain() {
        // Flat and decade-spanning but for the NaN, so nothing except the
        // guard under test can be what refuses these.
        assert!(low_frequency_gain_is_dc(
            &[1.0, 10.0, 100.0],
            &[60.0, 59.95, 40.0]
        ));
        assert!(
            !low_frequency_gain_is_dc(&[f64::NAN, 10.0, 100.0], &[60.0, 59.95, 40.0]),
            "a NaN f_min passed the positivity guard"
        );
        assert!(
            !low_frequency_gain_is_dc(&[1.0, 10.0, f64::NAN], &[60.0, 59.95, 40.0]),
            "a NaN f_max passed the decade-span guard"
        );
    }

    /// The other side of the threshold: two samples in the first decade are a
    /// real comparison, so a genuinely flat one still reports DC. Requiring a
    /// second sample must not cost the sweeps that always had one.
    #[test]
    fn two_flat_first_decade_samples_still_report_dc() {
        let frequency = [1.0, 10.0, 100.0];
        let gain_db = [60.0, 59.95, 40.0];

        assert!(low_frequency_gain_is_dc(&frequency, &gain_db));

        let analysis = wrapped_response(&frequency, &gain_db, &[0.0, -5.0, -90.0]);
        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("AC summary")
            .metrics;
        assert!(metrics.adc_is_dc);
        assert_within(metrics.adc_db, 60.0, 1.0e-9, "A_dc");
    }

    #[test]
    fn ac_summary_selects_magnitude_phase_pair_and_computes_margins() {
        let frequency = [1.0, 10.0, 100.0, 1000.0];
        let magnitude = [10.0, 1.0, 0.1, 0.01];
        let phase = [-45.0, -135.0, -180.0, -225.0];
        let analysis = ac_analysis(vec![
            wave("|V(out)|", &frequency, &magnitude, true),
            wave("phase(V(out))", &frequency, &phase, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 3).expect("AC summary");

        assert_eq!(summary.signal, "V(out)");
        assert_eq!(summary.analysis_index, 3);
        assert_eq!(summary.mag_index, 0);
        assert_eq!(summary.phase_index, Some(1));
        assert_close(summary.metrics.adc_db, 20.0);
        assert_close(summary.metrics.ugf, 10.0);
        assert_close(summary.metrics.pm_deg, 45.0);
        assert_close(summary.metrics.f180, 100.0);
        assert_close(summary.metrics.gm_db, 20.0);
        assert_close(summary.metrics.f3db, 10f64.powf(0.15));
        assert_eq!(summary.metrics.gain_extremes, (-40.0, 20.0));
        assert_eq!(summary.metrics.phase_extremes, Some((-225.0, -45.0)));
    }

    #[test]
    fn ac_summary_omits_phase_metrics_when_phase_trace_is_missing() {
        let frequency = [1.0, 10.0, 100.0];
        let magnitude = [2.0, 1.0, 0.5];
        let analysis = ac_analysis(vec![wave("|V(out)|", &frequency, &magnitude, true)]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert!(summary.phase_deg.is_none());
        assert_eq!(summary.metrics.pm_deg, None);
        assert_eq!(summary.metrics.f180, None);
        assert_eq!(summary.metrics.gm_db, None);
    }

    #[test]
    fn pac_summary_uses_the_same_exact_complex_response_projection() {
        let analysis = response_analysis(
            AnalysisType::Pac,
            vec![
                wave("|V(out,0)|", &[1.0, 10.0], &[10.0, 1.0], true),
                wave("phase(V(out,0))", &[1.0, 10.0], &[-90.0, -135.0], true),
            ],
        );

        let summary = ac_bode_summary_for_analysis(&analysis, 2).expect("PAC summary");

        assert_eq!(summary.signal, "V(out,0)");
        assert_eq!(summary.analysis_index, 2);
        assert_close(summary.metrics.adc_db, 20.0);
        assert_close(summary.metrics.ugf, 10.0);
    }

    #[test]
    fn stb_summary_preserves_retained_decibels_without_double_conversion() {
        let analysis = response_analysis(
            AnalysisType::Stb,
            vec![
                wave(
                    "Loop Gain (dB)",
                    &[1.0, 10.0, 100.0],
                    &[40.0, 0.0, -20.0],
                    true,
                ),
                wave(
                    "Loop Phase (deg)",
                    &[1.0, 10.0, 100.0],
                    &[-90.0, -135.0, -180.0],
                    true,
                ),
            ],
        );

        let summary = ac_bode_summary_for_analysis(&analysis, 4).expect("STB summary");

        assert_eq!(summary.signal, "Loop Gain");
        assert_eq!(summary.gain_db.as_slice(), &[40.0, 0.0, -20.0]);
        assert_close(summary.metrics.adc_db, 40.0);
        assert_close(summary.metrics.ugf, 10.0);
        assert_close(summary.metrics.pm_deg, 45.0);
        assert_close(summary.metrics.f180, 100.0);
        assert_close(summary.metrics.gm_db, 20.0);
    }

    #[test]
    fn pstb_mode_samples_never_impersonate_a_frequency_response() {
        let analysis = response_analysis(
            AnalysisType::Pstb,
            vec![wave(
                "Stability Margin (dB)",
                &[0.0, 1.0],
                &[12.0, 4.0],
                true,
            )],
        );

        assert_eq!(ac_bode_summary_for_analysis(&analysis, 0), None);
    }

    #[test]
    fn ac_summary_prefers_visible_magnitude_trace() {
        let frequency = [1.0, 10.0];
        let hidden = [1.0, 1.0];
        let visible = [10.0, 1.0];
        let analysis = ac_analysis(vec![
            wave("|V(in)|", &frequency, &hidden, false),
            wave("|V(out)|", &frequency, &visible, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert_eq!(summary.signal, "V(out)");
        assert_eq!(summary.mag_index, 1);
    }

    #[test]
    fn ac_summary_uses_last_matching_magnitude_when_visibility_ties() {
        let frequency = [1.0, 10.0];
        let first = [1.0, 1.0];
        let last = [10.0, 1.0];
        let analysis = ac_analysis(vec![
            wave("|V(first)|", &frequency, &first, true),
            wave("|V(last)|", &frequency, &last, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert_eq!(summary.signal, "V(last)");
        assert_eq!(summary.mag_index, 1);
    }

    #[test]
    fn an_unbound_selection_falls_back_to_the_first_ac_analysis_with_waveforms() {
        let frequency = [1.0, 10.0];
        let magnitude = [10.0, 1.0];
        let mut run = SimulationRun::new(7);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "OP"));
        run.add_analysis(ac_analysis(vec![wave(
            "|V(out)|", &frequency, &magnitude, true,
        )]));

        let summary = ac_bode_summary_for_selection(&run, None).expect("AC summary");

        assert_eq!(summary.analysis_index, 1);
        assert_eq!(summary.signal, "V(out)");
    }

    #[test]
    fn the_fallback_does_not_skip_an_unusable_first_ac_analysis() {
        let frequency = [1.0, 10.0];
        let magnitude = [10.0, 1.0];
        let mut run = SimulationRun::new(7);
        run.add_analysis(ac_analysis(vec![wave(
            "phase(V(in))",
            &frequency,
            &magnitude,
            true,
        )]));
        run.add_analysis(ac_analysis(vec![wave(
            "|V(out)|", &frequency, &magnitude, true,
        )]));

        assert_eq!(ac_bode_summary_for_selection(&run, None), None);
    }

    #[test]
    fn ac_selection_resolves_two_same_kind_results_by_source_instance() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();
        let snapshot = ContentDigest::from_bytes([0x42; 32]);
        let frequency = [1.0, 10.0];
        let mut run = SimulationRun::new(9);
        run.add_analysis(
            ac_analysis(vec![wave("|V(low_band)|", &frequency, &[10.0, 1.0], true)])
                .with_provenance(
                    super::super::AnalysisResultProvenance::new(
                        first_id,
                        ObjectRevision::INITIAL,
                        snapshot,
                        Vec::new(),
                    )
                    .expect("first provenance"),
                ),
        );
        run.add_analysis(
            ac_analysis(vec![wave(
                "|V(high_band)|",
                &frequency,
                &[100.0, 10.0],
                true,
            )])
            .with_provenance(
                super::super::AnalysisResultProvenance::new(
                    second_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    Vec::new(),
                )
                .expect("second provenance"),
            ),
        );

        let (first_index, first) = response_by_source_instance(&run, first_id).expect("first AC");
        let first = ac_bode_summary_for_analysis(first, first_index).expect("first summary");
        let (second_index, second) =
            response_by_source_instance(&run, second_id).expect("second AC");
        let second = ac_bode_summary_for_analysis(second, second_index).expect("second summary");
        assert_eq!(
            (first.analysis_index, first.signal.as_str()),
            (0, "V(low_band)")
        );
        assert_eq!(
            (second.analysis_index, second.signal.as_str()),
            (1, "V(high_band)")
        );

        assert_eq!(
            ac_bode_summary_for_selection(&run, Some(0))
                .expect("selected first AC")
                .signal,
            "V(low_band)"
        );
        assert_eq!(
            ac_bode_summary_for_selection(&run, Some(1))
                .expect("selected second AC")
                .signal,
            "V(high_band)"
        );
    }
}

//! Eye diagram presentation and application integration.
//!
//! Core owns waveform folding and eye measurements; results owns retained mask
//! geometry and compliance calculations. This module coordinates viewer state.

pub mod bar;
pub mod state;
pub mod view;

pub use rspice_results::engine_evidence::EyeTrace;
pub use rspice_results::engine_evidence::{
    Bathtub, BerEstimate, ClockTrigger, ClockTriggerRejection, DEFAULT_OPENING_BER, DualDiracFit,
    EyeData, EyeDataBuilder, EyeLevel, EyeMeasureOptions, EyeMeasurements, EyeOpening,
    MAX_EYE_LEVELS, MIN_EYE_LEVELS, UiEstimateRejection, calculate_eye_measurements,
    calculate_eye_measurements_with, centring_delay, clock_trigger, crossing_phase_at,
    estimate_unit_interval, fold_anchor,
};
pub use state::{
    EyeClockSignal, EyeDiagramState, EyeFold, EyeSettings, EyeTimebase, EyeTimebaseKey,
    EyeTimebaseProvenance, OPENING_RATES, parse_eye_timebase,
};

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eye_diagram::state::{EyeMask, MaskShape};

    /// Trapezoidal NRZ, one bit per unit interval, 20–80 % edge of 50 ps.
    ///
    /// Deliberately the same stimulus shape the core oracles use: this test
    /// is about the *mask* geometry meeting the folded eye, and it would
    /// prove nothing against a signal invented to suit it.
    fn prbs7_waveform(low: f64, high: f64) -> (Vec<f64>, Vec<f64>) {
        const UI: f64 = 1e-9;
        const DT: f64 = 10e-12;
        const RAMP: f64 = 50e-12 / 0.6;
        let t_start = 0.137e-9;

        let mut lfsr: u8 = 0x7f;
        let bits: Vec<bool> = (0..300)
            .map(|_| {
                let bit = ((lfsr >> 6) ^ (lfsr >> 5)) & 1;
                lfsr = ((lfsr << 1) | bit) & 0x7f;
                bit == 1
            })
            .collect();

        let level = |bit: bool| if bit { high } else { low };
        let mut events: Vec<(f64, f64)> = Vec::new();
        for n in 1..bits.len() {
            if bits[n] != bits[n - 1] {
                events.push((t_start + n as f64 * UI, level(bits[n])));
            }
        }

        let t_end = t_start + 302.0 * UI;
        let count = (t_end / DT) as usize;
        let mut time = Vec::with_capacity(count);
        let mut signal = Vec::with_capacity(count);
        let mut cursor = 0usize;
        let mut held = level(bits[0]);
        for index in 0..count {
            let t = index as f64 * DT;
            while cursor < events.len() && t >= events[cursor].0 + 0.5 * RAMP {
                held = events[cursor].1;
                cursor += 1;
            }
            let value = match events.get(cursor) {
                Some(&(edge, next)) if t > edge - 0.5 * RAMP => {
                    let alpha = ((t - (edge - 0.5 * RAMP)) / RAMP).clamp(0.0, 1.0);
                    held + alpha * (next - held)
                }
                _ => held,
            };
            time.push(t);
            signal.push(value);
        }
        (time, signal)
    }

    fn folded_eye(time: &[f64], signal: &[f64]) -> EyeData {
        let estimate = estimate_unit_interval(time, signal).expect("PRBS has a bit period");
        EyeDataBuilder::new()
            .bit_period(estimate.unit_interval)
            .ui_count(2)
            .skip_initial(2)
            .fold_anchor(fold_anchor(
                estimate.mean_crossing_phase,
                estimate.unit_interval,
            ))
            .build(time, signal)
    }

    /// The default mask is a keep-out over the eye *opening*. A clean PRBS7
    /// eye must not touch it — which is only true once the fold puts the
    /// opening at the window centre, where the mask sits.
    #[test]
    fn a_clean_prbs7_eye_clears_the_default_compliance_mask() {
        let (time, signal) = prbs7_waveform(-0.4, 0.4);
        let data = folded_eye(&time, &signal);
        assert!(
            data.trace_count() > 100,
            "{} acquisitions",
            data.trace_count()
        );

        let mut state = EyeDiagramState::default();
        state.load_data(data);
        state.set_show_mask(true);

        let mask = &state.settings().mask;
        assert!(mask.total_samples > 0, "the mask tested nothing");
        assert_eq!(mask.violation_count, 0, "a clean eye touched the mask");
        let clearance = mask.clearance.expect("a tested mask has a clearance");
        assert!(clearance > 0.0, "clean eye reported {clearance} clearance");
    }

    /// A mask drawn for a single-ended 0 to 1.2 V eye sits on its 0.6 V
    /// threshold and counts exactly the samples the fixture puts in the
    /// opening: three on one acquisition that sags to mid-level at the
    /// sampling instant.
    #[test]
    fn a_single_ended_eye_reports_violations_only_where_the_fixture_puts_them() {
        let (time, signal) = prbs7_waveform(0.0, 1.2);
        let mut data = folded_eye(&time, &signal);
        let clean = data.trace_count();
        data.add_trace(EyeTrace::new(vec![0.95, 1.0, 1.05], vec![0.62, 0.6, 0.58]));

        let mut state = EyeDiagramState::default();
        state.adopt_settings(&EyeSettings {
            mask: EyeMask {
                shape: MaskShape::Hexagon,
                width_ui: 0.5,
                flat_ui: 0.2,
                height: Some(0.4),
                ..EyeMask::default()
            },
            show_mask: true,
            ..EyeSettings::default()
        });
        state.load_data(data);

        let frame = state.mask_frame();
        assert!((frame.threshold - 0.6).abs() < 1e-9, "{frame:?}");
        let mask = &state.settings().mask;
        assert_eq!(mask.violation_count, 3, "of {} acquisitions", clean + 1);
        assert!(mask.clearance.is_some_and(|clearance| clearance < 0.0));
    }

    /// Enabling the mask tests the acquisitions that are loaded now. The
    /// verdict used to be latched at load time, so a mask switched on after
    /// the eye arrived reported `0 / 0` — an untested mask reading as a pass.
    #[test]
    fn toggling_the_mask_tests_the_currently_loaded_acquisitions() {
        let mut data = EyeData::new(1e-9, 2);
        data.v_low = -0.4;
        data.v_high = 0.4;
        data.swing = 0.8;
        data.v_cross = 0.0;
        // A trace that sits at the crossing level right through the opening.
        data.add_trace(EyeTrace::new(vec![0.9, 1.0, 1.1], vec![0.0, 0.0, 0.0]));

        let mut state = EyeDiagramState::default();
        state.load_data(data);
        assert_eq!(
            state.settings().mask.total_samples,
            0,
            "mask off, nothing tested"
        );

        state.set_show_mask(true);
        let mask = &state.settings().mask;
        assert_eq!(mask.total_samples, 3);
        assert_eq!(mask.violation_count, 3);
        let clearance = mask.clearance.expect("a tested mask has a clearance");
        assert!(
            clearance < 0.0,
            "a violated mask reported {clearance} clearance"
        );

        state.set_show_mask(false);
        assert_eq!(state.settings().mask.total_samples, 0);
        assert_eq!(state.settings().mask.clearance, None);
    }
}

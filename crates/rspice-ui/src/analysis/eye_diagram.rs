//! Eye Diagram Viewer
//!
//! Commercial-grade eye diagram visualization for signal integrity analysis.
//!
//! # Where the engine lives
//!
//! Eye construction and measurement are simulation mathematics, not
//! presentation, so they live in `rspice_core::analysis::signal_integrity`
//! next to the jitter decomposition and bathtub curves that consume the same
//! crossings. This module owns only what the viewer adds on top: display
//! mode, colour map, cursors, mask polygons, and the persistence cache.
//!
//! The engine types are re-exported here so viewer code keeps one import path.
//!
//! # Features
//!
//! - Overlay of signal transitions aligned to bit period
//! - Persistence/density display mode
//! - Eye opening measurements (height, width, area)
//! - Jitter calculation (RJ, DJ, TJ)
//! - Rise/fall time measurement
//! - Q-factor from BER
//! - Mask testing support
//!
//! # Architecture
//!
//! Follows Cadence-style signal integrity analysis workflow.

pub(crate) mod state;

#[cfg(test)]
pub use rspice_core::analysis::signal_integrity::EyeTrace;
pub use rspice_core::analysis::signal_integrity::{
    EyeData, EyeDataBuilder, EyeMeasurements, UiEstimateRejection, calculate_eye_measurements,
    crossing_phase_at, estimate_unit_interval, find_edges, fold_anchor,
};
pub use state::{
    EyeDiagramState, EyeRateEditor, EyeTimebase, EyeTimebaseKey, EyeTimebaseProvenance,
    parse_eye_timebase,
};

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::eye_diagram::state::EyeMask;

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
    /// eye at the levels the mask was authored against must not touch it —
    /// which is only true once the fold puts the opening at the window
    /// centre, where the mask sits.
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
        state.mask = EyeMask::default();
        state.load_data(data);
        state.set_show_mask(true);

        assert!(state.mask.total_samples > 0, "the mask tested nothing");
        assert_eq!(
            state.mask.violation_count, 0,
            "a clean eye touched the opening mask"
        );
        let margin = state.mask.margin.expect("a tested mask has a margin");
        assert!(margin > 0.0, "clean eye reported {margin} margin");
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
        assert_eq!(state.mask.total_samples, 0, "mask off, nothing tested");

        state.set_show_mask(true);
        assert_eq!(state.mask.total_samples, 3);
        assert_eq!(state.mask.violation_count, 3);
        assert_eq!(state.mask.pass_rate(), Some(0.0));
        let margin = state.mask.margin.expect("a tested mask has a margin");
        assert!(margin < 0.0, "a violated mask reported {margin} margin");

        state.set_show_mask(false);
        assert_eq!(state.mask.total_samples, 0);
        assert_eq!(state.mask.pass_rate(), None, "0/0 must not read as a pass");
        assert_eq!(state.mask.margin, None);
    }

    /// An untested mask has no verdict. Reporting 100 % for an empty test is
    /// how a compliance number comes to mean nothing.
    #[test]
    fn an_untested_mask_reports_no_pass_rate() {
        let mask = EyeMask::default();
        assert_eq!(mask.pass_rate(), None);
        assert_eq!(mask.margin, None);
    }
}

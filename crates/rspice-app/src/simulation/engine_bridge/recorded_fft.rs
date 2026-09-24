//! Carrying the `.FFT` spectra of one transient solve out of the engine.
//!
//! The engine evaluates every `.fft` card the deck holds inside the transient
//! and returns the spectra on its result. This module is the whole of the
//! Studio's part: pair each returned spectrum with the card it came from, and
//! copy the engine's own columns. No transform, window or normalisation is
//! computed here or anywhere else in this crate.

use std::sync::Arc;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::TransientFftResult;

use crate::simulation::config::FftRequest;
use crate::simulation::results::RecordedFftSpectrum;
use crate::simulation::runner::SimulationError;

/// Convert the spectra one transient returned, keyed by their own cards.
///
/// The key is the deck's parsed request written back out by the one card
/// writer, so the consumer — which holds only its own authored card — derives
/// the same string by parsing it with core. Order is never the key.
pub(super) fn recorded_spectra(
    netlist: &rspice_core::Netlist,
    spectra: &[TransientFftResult],
    abort: &dyn AbortSignal,
) -> Result<Vec<Arc<RecordedFftSpectrum>>, SimulationError> {
    if spectra.is_empty() && netlist.fft_analyses.is_empty() {
        return Ok(Vec::new());
    }
    if spectra.len() != netlist.fft_analyses.len() {
        return Err(SimulationError::SolverError(format!(
            "transient engine returned {} FFT spectra for {} .FFT cards",
            spectra.len(),
            netlist.fft_analyses.len()
        )));
    }
    let mut recorded = Vec::with_capacity(spectra.len());
    for (analysis, spectrum) in netlist.fft_analyses.iter().zip(spectra) {
        super::ensure_not_aborted(abort)?;
        let request_key = FftRequest::from_core(analysis).to_card();
        let evidence = crate::state::FftSpectrumEvidence::from(spectrum);
        let mut frequency = Vec::with_capacity(spectrum.bins.len());
        let mut real = Vec::with_capacity(spectrum.bins.len());
        let mut imaginary = Vec::with_capacity(spectrum.bins.len());
        for bin in &spectrum.bins {
            frequency.push(bin.frequency);
            real.push(bin.real);
            imaginary.push(bin.imaginary);
        }
        let recorded_spectrum = RecordedFftSpectrum {
            request_key,
            evidence,
            frequency,
            real,
            imaginary,
        };
        recorded_spectrum
            .validate()
            .map_err(SimulationError::SolverError)?;
        recorded.push(Arc::new(recorded_spectrum));
    }
    Ok(recorded)
}

#[cfg(test)]
mod tests {
    use crate::simulation::config::FftRequest;
    use crate::simulation::results::SimulationResult;

    fn run(deck: &str, stop: f64) -> SimulationResult {
        let netlist = rspice_core::Netlist::parse(deck).expect("the deck parses");
        let bridge = crate::simulation::engine_bridge::EngineBridge::new();
        let config = crate::simulation::config::TransientAnalysisConfig {
            stop_time: stop,
            step_time: 5.0e-6,
            ..Default::default()
        };
        bridge
            .run_transient(&netlist, &config, &rspice_core::abort_signal::NoAbort)
            .expect("the transient with its bound card runs")
    }

    #[test]
    fn an_fft_of_a_sine_reports_its_line_at_the_authored_frequency() {
        // Eight exact periods of a 1 kHz sine over an 8 ms record, sampled at
        // 256 points: the line lands on bin 8 = 1000 Hz with no scalloping, so
        // the only deviation from amplitude 1 is the window's own image term.
        // `epsilon` is the closed-form bound |W[2M]| / (N * CG) of each
        // window, computed offline from the window definitions.
        const CASES: [(&str, f64, f64); 4] = [
            ("RECT", 1.000_000_000, 1.0e-12),
            ("HANN", 0.498_046_875, 1.540e-5),
            ("BLACKMAN", 0.418_359_375, 6.459e-6),
            ("HARRIS", 0.357_348_867, 1.071e-6),
        ];
        for (window, coherent_gain, epsilon) in CASES {
            let request = FftRequest {
                output: "V(out)".to_owned(),
                start: Some(0.0),
                stop: Some(8.0e-3),
                points: 256,
                format: Some(crate::simulation::config::FftFormatChoice::Unnormalized),
                window: window.to_owned(),
                ..FftRequest::default()
            };
            let deck = format!(
                "recorded fft oracle\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 5u 8m\n{}\n.end\n",
                request.to_card()
            );
            let SimulationResult::Transient { spectra, .. } = run(&deck, 8.0e-3) else {
                panic!("a transient result");
            };
            let [spectrum] = spectra.as_slice() else {
                panic!("one recorded spectrum for {window}");
            };
            assert_eq!(
                spectrum.request_key,
                request.engine_key().expect("the key round-trips"),
                "{window}"
            );
            let evidence = &spectrum.evidence;
            assert!(evidence.accurate_sampling, "{window}");
            assert_eq!(evidence.window, window);
            assert!(
                (evidence.coherent_gain - coherent_gain).abs() < 1.0e-9,
                "{window}: coherent gain {} is not {coherent_gain}",
                evidence.coherent_gain
            );
            assert!(
                (evidence.frequency_resolution_hz - 125.0).abs() < 1.0e-9,
                "{window}"
            );
            assert_eq!(spectrum.frequency.len(), 129, "{window}");
            assert_eq!(spectrum.frequency[8], 1000.0, "{window}");
            let magnitude = spectrum.real[8].hypot(spectrum.imaginary[8]);
            assert!(
                (magnitude - 1.0).abs() <= epsilon * 1.001 + 1.0e-9,
                "{window}: |{magnitude} - 1| exceeds {epsilon}"
            );
            let phase = spectrum.imaginary[8].atan2(spectrum.real[8]).to_degrees();
            assert!((phase + 90.0).abs() < 1.0e-3, "{window}: phase {phase}");
        }
    }

    #[test]
    fn a_transient_that_carries_no_card_records_no_spectrum() {
        let deck = "no fft here\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 5u 8m\n.end\n";
        let SimulationResult::Transient { spectra, .. } = run(deck, 8.0e-3) else {
            panic!("a transient result");
        };
        assert!(spectra.is_empty());
    }

    #[test]
    fn a_card_past_the_transient_stop_time_fails_that_transient() {
        // Why a card may only ever be in the deck of the transient it is bound
        // to, in the engine's own words. Nothing about this is observation:
        // the solve does not run at all.
        let deck = "recorded fft past the stop time\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n\
                    .tran 5u 2m\n.fft V(out) NP=256 WINDOW=RECT START=0 STOP=8m\n.end\n";
        let netlist = rspice_core::Netlist::parse(deck).expect("the deck parses");
        let bridge = crate::simulation::engine_bridge::EngineBridge::new();
        let config = crate::simulation::config::TransientAnalysisConfig {
            stop_time: 2.0e-3,
            step_time: 5.0e-6,
            ..Default::default()
        };
        let error = bridge
            .run_transient(&netlist, &config, &rspice_core::abort_signal::NoAbort)
            .expect_err("the engine refuses the whole transient");
        assert!(
            error
                .to_string()
                .contains("STOP 0.008 exceeds transient stop time 0.002"),
            "{error}"
        );
    }
}

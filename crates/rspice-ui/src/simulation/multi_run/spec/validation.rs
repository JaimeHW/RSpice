//! Specification validation.
//!
//! Rejects a specification the engine could not execute — a missing
//! dependency, an empty sweep, an output that names nothing — before a run
//! starts rather than partway through.
//!
//! The refusals themselves live one file per family, in the same families
//! `simulation/runner/spec/` dispatches by, because that is the grain at
//! which they are edited: a kind that learns a parameter learns a bound on
//! it, and the two belong in one place. What is left here is the routing.

mod config;
mod device;
mod frequency;
mod periodic;
mod post_process;
mod time_domain;

use super::AnalysisSpec;

impl AnalysisSpec {
    /// Validate analysis parameters.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            AnalysisSpec::DcOp { .. } => time_domain::validate(self),
            AnalysisSpec::DcSweep { .. } => config::validate(self),
            AnalysisSpec::Ac { .. } => config::validate(self),
            AnalysisSpec::AcData { .. } => config::validate(self),
            AnalysisSpec::Disto { .. } => periodic::validate(self),
            AnalysisSpec::Transient { .. } => time_domain::validate(self),
            AnalysisSpec::Noise { .. } => config::validate(self),
            AnalysisSpec::Pss { .. } => periodic::validate(self),
            AnalysisSpec::PssSpectrum { .. } => periodic::validate(self),
            AnalysisSpec::HarmonicBalance { .. } => periodic::validate(self),
            AnalysisSpec::Sensitivity { .. } => config::validate(self),
            AnalysisSpec::PoleZero { .. } => config::validate(self),
            AnalysisSpec::Stb { .. } => frequency::validate(self),
            AnalysisSpec::SParameter { .. } => frequency::validate(self),
            AnalysisSpec::Envelope { .. } => periodic::validate(self),
            AnalysisSpec::Fourier { .. } => post_process::validate(self),
            AnalysisSpec::Fft { .. } => post_process::validate(self),
            AnalysisSpec::Reliability { .. } => device::validate(self),
            AnalysisSpec::Optimization { .. } => device::validate(self),
            AnalysisSpec::Soa { .. } => device::validate(self),
            AnalysisSpec::Qpss { .. } => periodic::validate(self),
            AnalysisSpec::Hbsp { .. } => periodic::validate(self),
            AnalysisSpec::Psp { .. } => periodic::validate(self),
            AnalysisSpec::Hbnoise { .. } => periodic::validate(self),
            AnalysisSpec::Qpac { .. } => periodic::validate(self),
            AnalysisSpec::Qpxf { .. } => periodic::validate(self),
            AnalysisSpec::Qpnoise { .. } => periodic::validate(self),
            AnalysisSpec::TransientNoise { .. } => time_domain::validate(self),
            AnalysisSpec::DcMismatch { .. } => device::validate(self),
            AnalysisSpec::Tf { .. } => frequency::validate(self),
            // Nothing to refuse. Four of these are dispatched from a periodic
            // solve whose own specification carries every bound, two are
            // expansions authorized before a task exists, and the legacy
            // operating point carries no policy at all.
            AnalysisSpec::LegacyDcOp
            | AnalysisSpec::Pac
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pstb
            | AnalysisSpec::MonteCarlo { .. }
            | AnalysisSpec::Parametric
            | AnalysisSpec::Corner => Ok(()),
        }
    }
}

/// A specification that reached the wrong family validator.
///
/// The router above names a family for every variant, so this is a routing
/// fault and not a configuration anyone can correct. It is still a refusal
/// rather than a panic: the only thing on the other side of it is an
/// operator standing in front of a studio.
fn misrouted_specification(family: &str, spec: &AnalysisSpec) -> String {
    format!(
        "{} is not a {family} analysis",
        spec.run_type().display_name()
    )
}

#[cfg(test)]
mod tests {
    use super::periodic::validate_periodic_mixed_mode_ports;
    use super::*;
    use crate::simulation::multi_run::{
        EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve, HbToneSpec,
        PssMethod, TfAccuracy, TfNormalization,
    };

    #[test]
    fn transient_spec_rejects_non_finite_stop_and_step_times() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for stop_field in [false, true] {
                let spec = AnalysisSpec::Transient {
                    stop_time: if stop_field { value } else { 1.0e-6 },
                    step_time: if stop_field { 1.0e-9 } else { value },
                    start_time: 0.0,
                    max_timestep: None,
                    uic: false,
                };
                assert!(
                    spec.validate().is_err(),
                    "non-finite transient input was accepted: {spec:?}"
                );
            }
        }
    }

    #[test]
    fn periodic_mixed_mode_requires_complete_equal_impedance_pairs() {
        let port = |name: &str, z0: Option<f64>| super::super::SpPort {
            node_pos: name.to_owned(),
            node_neg: "0".to_owned(),
            z0,
        };

        validate_periodic_mixed_mode_ports(&[port("P1", Some(50.0)), port("P2", Some(50.0))])
            .expect("an equal-impedance physical-port pair is valid");

        let odd = validate_periodic_mixed_mode_ports(&[
            port("P1", Some(50.0)),
            port("P2", Some(50.0)),
            port("P3", Some(50.0)),
        ])
        .expect_err("an unpaired physical port must be rejected");
        assert!(odd.contains("even number"));

        let mismatch =
            validate_periodic_mixed_mode_ports(&[port("P1", Some(50.0)), port("P2", Some(75.0))])
                .expect_err("unequal explicit impedances do not define an orthonormal pair");
        assert!(mismatch.contains("unequal explicit reference impedances"));
    }

    fn tf_spec(output_expression: &str) -> AnalysisSpec {
        AnalysisSpec::Tf {
            input_source: "VIN_DIFF".to_owned(),
            output_expression: output_expression.to_owned(),
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: TfNormalization::None,
            accuracy: TfAccuracy::Balanced,
        }
    }

    #[test]
    fn tf_validation_accepts_only_exact_probe_grammar_and_one_source_token() {
        for valid in ["V(out)", "v(out,ref)", "I(Vsense)"] {
            assert!(tf_spec(valid).validate().is_ok(), "{valid}");
        }
        for invalid in [
            "",
            "out",
            "V()",
            "V(out,)",
            "V(a,b,c)",
            "I(V1,V2)",
            "P(R1)",
            " V(out)",
            "V(out) ",
            "V (out)",
            "V( out)",
            "V(out, ref)",
            "V((out))",
            "V(out) extra",
        ] {
            assert!(tf_spec(invalid).validate().is_err(), "{invalid}");
        }

        let mut invalid_source = tf_spec("V(out)");
        let AnalysisSpec::Tf { input_source, .. } = &mut invalid_source else {
            unreachable!()
        };
        *input_source = " VIN_DIFF".to_owned();
        assert!(invalid_source.validate().is_err());
        let AnalysisSpec::Tf { input_source, .. } = &mut invalid_source else {
            unreachable!()
        };
        *input_source = "VIN DIFF".to_owned();
        assert!(invalid_source.validate().is_err());
    }

    #[test]
    fn tf_validation_rejects_an_all_disabled_result_contract() {
        let mut spec = tf_spec("V(out)");
        let AnalysisSpec::Tf {
            transfer_gain,
            input_resistance,
            output_resistance,
            ..
        } = &mut spec
        else {
            unreachable!()
        };
        *transfer_gain = false;
        *input_resistance = false;
        *output_resistance = false;

        assert!(
            spec.validate()
                .expect_err("TF must retain at least one scalar")
                .contains("requires transfer gain")
        );
    }

    #[test]
    fn a_pss_spectrum_must_retain_at_least_one_harmonic() {
        assert!(
            AnalysisSpec::PssSpectrum { num_harmonics: 0 }
                .validate()
                .is_err()
        );
        assert!(
            AnalysisSpec::PssSpectrum { num_harmonics: 20 }
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn a_pss_spectrum_is_its_own_run_type_and_reads_as_a_coefficient_spectrum() {
        use crate::simulation::multi_run::AnalysisRunType;

        let spectrum = AnalysisSpec::PssSpectrum { num_harmonics: 20 };
        // Its own run type, not the PSS one: the two are separate retained
        // analyses and a shared type would collapse them in every label and
        // availability check that keys off it.
        assert_eq!(spectrum.run_type(), AnalysisRunType::PssSpectrum);
        assert_eq!(spectrum.run_type().display_name(), "PSS Spectrum");
    }

    #[test]
    fn legacy_pss_specs_receive_compatible_execution_defaults() {
        let spec: AnalysisSpec = serde_json::from_str(
            r#"{"Pss":{"fundamental_freq":1000000.0,"num_harmonics":9,"tolerance":0.000001}}"#,
        )
        .expect("legacy PSS spec deserializes");

        assert_eq!(
            spec,
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                fundamental_freq: 1.0e6,
                // A legacy spec that named no tone restores as one that named
                // no tone: only the design can supply that name.
                tone_sources: Vec::new(),
                tstab_periods: 20,
                points_per_period: 512,
                tolerance: 1.0e-6,
                oscillator_mode: false,
                oscillator_node: None,
                num_harmonics: 9,
                integration_method: None,
                tstab: 0.0,
                max_iterations: 100,
                abstol: 1.0e-12,
                damping: 1.0,
                max_period_change: 0.1,
                verbose: false,
            }
        );
    }

    #[test]
    fn pss_validation_requires_an_explicit_autonomous_probe() {
        let spec = AnalysisSpec::Pss {
            method: PssMethod::Shooting,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VCLK".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-6,
            oscillator_mode: true,
            oscillator_node: None,
            num_harmonics: 9,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        };

        assert!(spec.validate().is_err());
    }

    #[test]
    fn pss_validation_rejects_autonomous_harmonic_balance_before_dispatch() {
        let spec = AnalysisSpec::Pss {
            method: PssMethod::HarmonicBalance,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VCLK".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-6,
            oscillator_mode: true,
            oscillator_node: Some("out".to_owned()),
            num_harmonics: 9,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        };

        let error = spec
            .validate()
            .expect_err("autonomous harmonic balance must fail validation");
        assert!(error.contains("HB-PSS"));
    }

    #[test]
    fn legacy_envelope_spec_migrates_without_inventing_a_source_binding() {
        let spec: AnalysisSpec = serde_json::from_str(
            r#"{"Envelope":{"fundamental_freq":1000000.0,"stop_time":0.01,"num_harmonics":9,"max_step":0.000001}}"#,
        )
        .expect("legacy Envelope spec deserializes");

        assert_eq!(
            spec,
            AnalysisSpec::Envelope {
                fundamental_freq: 1.0e6,
                additional_carrier_tones: Vec::new(),
                stop_time: 0.01,
                num_harmonics: 9,
                envelope_step: Some(1.0e-6),
                modulation_sources: Vec::new(),
                initial_periodic_solve: EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
                adaptive_mode: EnvelopeAdaptiveMode::FixedEnvelopeStep,
                extraction_path: EnvelopeExtractionPath::Projection,
            }
        );
        assert!(spec.validate().is_ok());
    }

    #[test]
    fn envelope_validation_rejects_duplicate_tones_and_invalid_source_names() {
        let spec = |additional_carrier_tones, modulation_sources| AnalysisSpec::Envelope {
            fundamental_freq: 1.0e6,
            additional_carrier_tones,
            stop_time: 0.01,
            num_harmonics: 9,
            envelope_step: Some(1.0e-6),
            modulation_sources,
            initial_periodic_solve: EnvelopeInitialPeriodicSolve::HarmonicBalance,
            adaptive_mode: EnvelopeAdaptiveMode::Enabled,
            extraction_path: EnvelopeExtractionPath::Projection,
        };

        assert!(
            spec(vec![1.0e6], vec!["VIN_AM".to_owned()])
                .validate()
                .unwrap_err()
                .contains("unique")
        );
        assert!(
            spec(Vec::new(), vec![" VIN_AM".to_owned()])
                .validate()
                .unwrap_err()
                .contains("trimmed")
        );
        assert!(
            spec(vec![2.0e6], vec!["VIN_AM".to_owned(), "vin_am".to_owned()])
                .validate()
                .unwrap_err()
                .contains("unique")
        );
        assert!(
            spec(Vec::new(), Vec::new())
                .validate()
                .unwrap_err()
                .contains("required")
        );
    }

    #[test]
    fn fourier_validation_rejects_non_finite_or_negative_windows() {
        let spec = |fundamental_freq, start_time, stop_time| AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics: 9,
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            additional_outputs: Vec::new(),
            start_time,
            stop_time,
            compute_thd: true,
            normalize: false,
        };

        assert!(spec(f64::NAN, 0.0, 1.0).validate().is_err());
        assert!(spec(1.0, f64::NAN, 1.0).validate().is_err());
        assert!(spec(1.0, -1.0, 1.0).validate().is_err());
        assert!(spec(1.0, 0.0, f64::INFINITY).validate().is_err());
        assert!(spec(1.0, 0.0, 1.0).validate().is_ok());
    }

    #[test]
    fn fourier_validation_rejects_a_reference_on_a_current_accessor() {
        let spec = AnalysisSpec::Fourier {
            fundamental_freq: 1.0,
            num_harmonics: 9,
            output_node: "I(V1)".to_owned(),
            output_ref: "0".to_owned(),
            additional_outputs: Vec::new(),
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd: true,
            normalize: false,
        };

        assert_eq!(
            spec.validate().expect_err("current references are invalid"),
            "Fourier current output must not specify a voltage reference"
        );
    }

    fn hb_spec(collocation_points: Option<usize>) -> AnalysisSpec {
        AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1.0e6, 3)],
            reltol: 1.0e-6,
            abstol: 1.0e-12,
            max_iterations: 40,
            damping: 1.0,
            min_damping: 0.01,
            oversample: 2,
            collocation_points,
            max_mixing_order: 3,
            use_krylov: false,
            gmres_restart: 20,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose: false,
        }
    }

    #[test]
    fn hb_validation_rejects_undersized_exact_grid() {
        let err = hb_spec(Some(5)).validate().expect_err("grid is undersized");
        assert!(err.contains("at least 7 points"));
    }

    #[test]
    fn hb_validation_rejects_non_finite_tolerances() {
        assert!(hb_spec_with_reltol(f64::NAN).validate().is_err());
    }

    fn hb_spec_with_reltol(reltol: f64) -> AnalysisSpec {
        let mut spec = hb_spec(None);
        let AnalysisSpec::HarmonicBalance {
            reltol: configured, ..
        } = &mut spec
        else {
            unreachable!();
        };
        *configured = reltol;
        spec
    }
}

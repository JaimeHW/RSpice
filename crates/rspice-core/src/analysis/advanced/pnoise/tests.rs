//! PNoise Integration Tests
//!
//! Comprehensive tests for the complete phase noise analysis workflow.

#[cfg(test)]
mod integration_tests {
    use crate::analysis::advanced::pnoise::config::{PnoiseConfig, PnoiseSideband, PnoiseSweep};
    use crate::analysis::advanced::pnoise::floquet::{
        FloquetAnalyzer, FloquetMode, FloquetModeType,
    };
    use crate::analysis::advanced::pnoise::result::{PhaseNoisePoint, PnoiseResult};
    use crate::analysis::advanced::pnoise::solver::{DeviceNoise, NoisePsd, PnoiseSolver};
    use num_complex::Complex64;

    // =========================================================================
    // End-to-End Workflow Tests
    // =========================================================================

    #[test]
    fn test_complete_pnoise_workflow() {
        // 1. Configure analysis
        let config = PnoiseConfig::new("vco_out", 1e3, 10e6)
            .with_sidebands(PnoiseSideband::Both)
            .with_max_sidebands(10)
            .with_reference_freq(1e9)
            .with_points_per_decade(5);

        assert!(config.validate().is_ok());

        // 2. Create solver
        let mut solver = PnoiseSolver::new(config, 1e9, 3);

        // 3. Add noise sources
        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0, 1], 1000.0, 300.0));
        solver
            .state_mut()
            .add_device_noise(DeviceNoise::shot("D1", vec![1, 2], 1e-3));
        solver
            .state_mut()
            .add_device_noise(DeviceNoise::flicker("M1", vec![2], 1e-24, 2.0, 1e-3));

        // 4. Run analysis
        let result = solver.compute();
        assert!(result.is_ok());

        let pn = result.unwrap();

        // 5. Verify result
        assert!(pn.converged);
        assert!(!pn.spectral_points.is_empty());
        assert_eq!(pn.output_node, "vco_out");
        assert_eq!(pn.carrier_freq, 1e9);

        // Should have contributions from all 3 devices
        assert_eq!(pn.contributors.len(), 3);
    }

    #[test]
    fn test_pnoise_with_jitter_calculation() {
        let config = PnoiseConfig::new("osc", 100.0, 10e6).with_jitter_integration(1e3, 10e6);

        let mut solver = PnoiseSolver::new(config, 2.5e9, 2);

        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0], 50.0, 300.0));

        let result = solver.compute().unwrap();

        assert!(result.rms_jitter.is_some());
        assert!(result.rms_phase_error.is_some());
        assert!(result.jitter_bandwidth == Some((1e3, 10e6)));

        // Jitter should be positive
        assert!(result.rms_jitter.unwrap() > 0.0);
    }

    // =========================================================================
    // Frequency Response Tests - Verify Correct Physical Behavior
    // =========================================================================

    #[test]
    fn test_pnoise_1_over_f_noise_slope() {
        // With 1/f flicker noise through 1/f² transfer function,
        // total phase noise ∝ 1/f³ (-30 dB/decade)
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        // Flicker noise: S_n(f) = Kf / f
        let kf = 1e-20;
        let noise_1k = kf / 1e3;
        let noise_10k = kf / 10e3;

        let phase_psd_1k = analyzer.noise_to_phase_transfer(1e3, noise_1k, 0);
        let phase_psd_10k = analyzer.noise_to_phase_transfer(10e3, noise_10k, 0);

        let dbc_1k = analyzer.phase_psd_to_dbc(phase_psd_1k);
        let dbc_10k = analyzer.phase_psd_to_dbc(phase_psd_10k);
        let slope = dbc_1k - dbc_10k;

        // 1/f noise × 1/f² TF = 1/f³ = 30 dB/decade
        assert!(
            slope > 25.0 && slope < 35.0,
            "1/f noise slope should be ~30 dB/decade, got {}",
            slope
        );
    }

    #[test]
    fn test_pnoise_white_noise_slope() {
        // With white (thermal) noise through 1/f² transfer function,
        // phase noise ∝ 1/f² (-20 dB/decade)
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        let white_noise = 1e-20; // Constant PSD

        let phase_psd_1m = analyzer.noise_to_phase_transfer(1e6, white_noise, 0);
        let phase_psd_10m = analyzer.noise_to_phase_transfer(10e6, white_noise, 0);

        let dbc_1m = analyzer.phase_psd_to_dbc(phase_psd_1m);
        let dbc_10m = analyzer.phase_psd_to_dbc(phase_psd_10m);
        let slope = dbc_1m - dbc_10m;

        // White noise × 1/f² TF = 1/f² = 20 dB/decade
        assert!(
            slope > 15.0 && slope < 25.0,
            "White noise slope should be ~20 dB/decade, got {}",
            slope
        );
    }

    #[test]
    fn test_transfer_function_20db_per_decade() {
        // The Hajimiri-Lee transfer function should give -20 dB/decade
        // |H(f)|² ∝ 1/f², so |H| ∝ 1/f = -20 dB/decade
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        let tf_1k = analyzer.transfer_function(1e3, 0);
        let tf_10k = analyzer.transfer_function(10e3, 0);

        let mag_ratio = tf_1k.magnitude / tf_10k.magnitude;
        // Should be 10 (one decade)
        assert!(
            (mag_ratio - 10.0).abs() < 1.0,
            "TF should drop 20 dB/decade (10x per decade), got ratio {}",
            mag_ratio
        );
    }

    // =========================================================================
    // Floquet Mode Tests
    // =========================================================================

    #[test]
    fn test_floquet_mode_classification() {
        let modes = [
            FloquetMode::new(0, Complex64::new(0.0, 0.0), vec![Complex64::new(1.0, 0.0)]),
            FloquetMode::new(1, Complex64::new(-1e6, 0.0), vec![Complex64::new(0.5, 0.5)]),
            FloquetMode::new(2, Complex64::new(1e5, 0.0), vec![Complex64::new(0.0, 1.0)]),
        ];

        assert_eq!(modes[0].mode_type, FloquetModeType::Phase);
        assert_eq!(modes[1].mode_type, FloquetModeType::Stable);
        assert_eq!(modes[2].mode_type, FloquetModeType::Unstable);
    }

    #[test]
    fn test_floquet_transfer_function_frequency_dependence() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        // Get transfer functions at different offsets
        let tf_1k = analyzer.transfer_function(1e3, 0);
        let tf_1m = analyzer.transfer_function(1e6, 0);

        // Higher offset should have lower magnitude (1/f)
        assert!(tf_1k.offset_freq == 1e3);
        assert!(tf_1m.offset_freq == 1e6);
        assert!(tf_1k.magnitude > tf_1m.magnitude);
    }

    // =========================================================================
    // Result Analysis Tests
    // =========================================================================

    #[test]
    fn test_spot_noise_extraction() {
        let mut result = PnoiseResult::new(1e9, "vco");

        result.add_point(PhaseNoisePoint::new(1e3, -80.0));
        result.add_point(PhaseNoisePoint::new(10e3, -100.0));
        result.add_point(PhaseNoisePoint::new(100e3, -120.0));
        result.add_point(PhaseNoisePoint::new(1e6, -140.0));
        result.converged = true;

        let summary = result.spot_noise_summary();

        assert!(summary.contains_key("1kHz"));
        assert!(summary.contains_key("10kHz"));
        assert!(summary.contains_key("100kHz"));
        assert!(summary.contains_key("1MHz"));

        assert!((summary["1kHz"] - (-80.0)).abs() < 0.1);
        assert!((summary["1MHz"] - (-140.0)).abs() < 0.1);
    }

    #[test]
    fn test_phase_noise_interpolation_log() {
        let mut result = PnoiseResult::new(1e9, "out");

        // -20 dB/decade slope (typical 1/f^2 region)
        result.add_point(PhaseNoisePoint::new(1e3, -80.0));
        result.add_point(PhaseNoisePoint::new(10e3, -100.0));
        result.add_point(PhaseNoisePoint::new(100e3, -120.0));

        // Interpolate at 3.16 kHz (half decade from 1kHz)
        if let Some(pn) = result.phase_noise_at(3.16e3) {
            // Should be around -90 dBc/Hz (halfway between -80 and -100 in log scale)
            assert!(
                pn > -95.0 && pn < -85.0,
                "Interpolated value {} should be ~-90",
                pn
            );
        }
    }

    // =========================================================================
    // Noise Model Tests
    // =========================================================================

    #[test]
    fn test_combined_noise_model() {
        let psd = NoisePsd::Combined {
            white: 1e-20,
            kf: 1e-14,
            af: 1.0,
            i_dc: 1e-3,
        };

        // At low frequency, flicker dominates
        let low_f = psd.at(1.0); // 1 Hz
        assert!(low_f > 1e-18);

        // At high frequency, white dominates
        let high_f = psd.at(1e9); // 1 GHz
        assert!((high_f - 1e-20).abs() / 1e-20 < 0.01);

        // Corner frequency should be calculable
        let fc = psd.corner_freq().unwrap();
        assert!(fc > 0.0);
    }

    #[test]
    fn test_thermal_noise_temperature_scaling() {
        let noise_300k = DeviceNoise::thermal("R1", vec![0], 1000.0, 300.0);
        let noise_400k = DeviceNoise::thermal("R2", vec![0], 1000.0, 400.0);

        let psd_300 = noise_300k.psd_at(1e6);
        let psd_400 = noise_400k.psd_at(1e6);

        // PSD scales linearly with temperature
        let ratio = psd_400 / psd_300;
        assert!((ratio - 400.0 / 300.0).abs() < 0.01);
    }

    #[test]
    fn test_shot_noise_current_scaling() {
        let noise_1ma = DeviceNoise::shot("D1", vec![0], 1e-3);
        let noise_4ma = DeviceNoise::shot("D2", vec![0], 4e-3);

        let psd_1 = noise_1ma.psd_at(1e6);
        let psd_4 = noise_4ma.psd_at(1e6);

        // PSD scales linearly with current
        let ratio = psd_4 / psd_1;
        assert!((ratio - 4.0).abs() < 0.01);
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_pnoise_very_high_carrier() {
        let config = PnoiseConfig::new("out", 1e3, 1e6);
        let mut solver = PnoiseSolver::new(config, 100e9, 2); // 100 GHz carrier

        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0], 50.0, 300.0));

        let result = solver.compute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pnoise_very_low_offset() {
        let config = PnoiseConfig::new("out", 0.1, 1e3).with_sweep(PnoiseSweep::log(0.1, 1e3, 5));

        let mut solver = PnoiseSolver::new(config, 1e9, 2);

        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0], 1000.0, 300.0));

        let result = solver.compute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pnoise_single_point_sweep() {
        let config = PnoiseConfig::new("out", 10e3, 10e3).with_sweep(PnoiseSweep::list(vec![10e3]));

        let mut solver = PnoiseSolver::new(config, 1e9, 1);

        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0], 100.0, 300.0));

        let result = solver.compute();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().spectral_points.len(), 1);
    }

    #[test]
    fn test_pnoise_many_noise_sources() {
        let config = PnoiseConfig::new("out", 1e3, 1e6).with_sweep(PnoiseSweep::log(1e3, 1e6, 3));

        let mut solver = PnoiseSolver::new(config, 1e9, 5);

        // Add 10 noise sources
        for i in 0..10 {
            solver.state_mut().add_device_noise(DeviceNoise::thermal(
                &format!("R{}", i),
                vec![i % 5],
                1000.0 * (i + 1) as f64,
                300.0,
            ));
        }

        let result = solver.compute();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().contributors.len(), 10);
    }

    // =========================================================================
    // ISF and PPV Tests
    // =========================================================================

    #[test]
    fn test_isf_rms_for_sinusoidal() {
        // For sinusoidal ISF cos(ωt), Γ_rms = 1/√2
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer = analyzer.with_num_states(2);
        analyzer.compute_approximate_isf();

        // Check that ISF RMS is approximately 1/√2
        let expected_rms = 1.0 / 2.0_f64.sqrt();
        for rms in analyzer.isf_rms() {
            assert!(
                (*rms - expected_rms).abs() < 0.01,
                "ISF RMS should be ~0.707, got {}",
                rms
            );
        }
    }

    #[test]
    fn test_phase_sensitivity_periodicity() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer = analyzer.with_num_states(2);
        analyzer.compute_approximate_isf();

        // PPV at t=0 should equal PPV at t=1 (periodic)
        let ppv_start = analyzer.phase_sensitivity(0.0);
        let ppv_end = analyzer.phase_sensitivity(1.0);

        for (a, b) in ppv_start.iter().zip(ppv_end.iter()) {
            let diff = (a - b).norm();
            assert!(diff < 0.1, "PPV should be periodic, diff = {}", diff);
        }
    }
}

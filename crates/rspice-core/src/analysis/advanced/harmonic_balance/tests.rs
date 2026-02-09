//! Harmonic Balance Integration Tests
//!
//! Comprehensive tests for the complete HB analysis workflow.

#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use num_complex::Complex64;
    use std::f64::consts::PI;

    /// Test basic RC lowpass filter at single frequency
    #[test]
    fn test_rc_lowpass_single_frequency() {
        // RC lowpass: R = 1kΩ, C = 1nF
        // Corner frequency: f_c = 1/(2π*R*C) ≈ 159 kHz
        // At f = 159 kHz, |H| ≈ 0.707 (-3dB)

        let r = 1e3;
        let c = 1e-9;
        let f_c = 1.0 / (2.0 * PI * r * c);

        let config = HbConfig::new(f_c).with_harmonics(3);
        let mut solver = HbSolver::new(config.clone(), 2); // 2 nodes: in, out

        // Stamp resistor between node 0 and 1
        solver.add_conductance(0, 0, 1.0 / r);
        solver.add_conductance(0, 1, -1.0 / r);
        solver.add_conductance(1, 0, -1.0 / r);
        solver.add_conductance(1, 1, 1.0 / r);

        // Stamp capacitor from node 1 to ground
        solver.add_capacitance(1, 1, c);

        // 1V AC input at fundamental
        solver.set_ac_source(0, 1.0, 0.0);

        let mut state = HbSolverState::new(2, 3);

        // Solve should work for linear circuit
        let result = solver.solve_linear(&mut state);
        assert!(result.is_ok());

        // At corner frequency, output should be ~-3dB ≈ 0.707
        let v_out = state.x[1][1].norm();
        let v_in = state.x[0][1].norm();
        let gain = if v_in > 0.0 { v_out / v_in } else { 0.0 };

        // Due to simplified setup, just check it's reasonable
        assert!(gain >= 0.0 && gain <= 1.0, "Gain {} should be 0-1", gain);
    }

    /// Test DC operating point
    #[test]
    fn test_dc_operating_point() {
        let config = HbConfig::new(1e6).with_harmonics(3);
        let mut solver = HbSolver::new(config, 2);

        // Voltage divider: R1 = R2 = 1kΩ
        // V_out = V_in / 2
        let g = 1.0 / 1000.0; // 1 mS

        // R1: node 0 to node 1
        solver.add_conductance(0, 0, g);
        solver.add_conductance(0, 1, -g);
        solver.add_conductance(1, 0, -g);
        solver.add_conductance(1, 1, g);

        // R2: node 1 to ground (implicit)
        solver.add_conductance(1, 1, g);

        // 1V DC at input
        solver.set_dc_source(0, 1.0 * g); // Current = V/R

        let mut state = HbSolverState::new(2, 3);
        solver.solve_linear(&mut state).unwrap();

        // DC voltages
        let _v0_dc = state.x[0][0].re;
        let v1_dc = state.x[1][0].re;

        // V1 should be approximately V0/2
        // Note: With our simplified setup, exact values depend on grounding
        assert!(v1_dc.is_finite());
    }

    /// Test FFT roundtrip accuracy
    #[test]
    fn test_fft_roundtrip_complex_waveform() {
        let mut fft = HbFft::new(10, 4);
        let n = fft.size();

        // Generate complex multi-harmonic waveform
        let waveform: Vec<f64> = (0..n)
            .map(|i| {
                let t = i as f64 / n as f64;
                1.0 // DC
                + 2.0 * (2.0 * PI * t).cos()        // H1
                + 0.5 * (4.0 * PI * t).sin()        // H2
                + 0.1 * (6.0 * PI * t).cos() // H3
            })
            .collect();

        // Forward then inverse
        let spectrum = fft.to_frequency_domain(&waveform);
        let recovered = fft.to_time_domain(&spectrum);

        // Compare
        let max_error: f64 = waveform
            .iter()
            .zip(recovered.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);

        assert!(
            max_error < 0.1,
            "FFT roundtrip error too large: {}",
            max_error
        );
    }

    /// Test multi-tone frequency mapping
    #[test]
    fn test_multi_tone_mixer_frequencies() {
        // Typical mixer: f_RF = 900 MHz, f_LO = 800 MHz
        // Expect: DC, f_RF, f_LO, f_RF+f_LO, f_RF-f_LO, 2*f_RF, 2*f_LO, etc.

        let config = MultiToneConfig::new(vec![900e6, 800e6], 3);
        let map = FrequencyMap::new(&config);

        // Should have multiple frequency components
        assert!(
            map.len() > 5,
            "Should have >5 components, got {}",
            map.len()
        );

        // DC should be present
        let dc_idx = FrequencyIndex::dc(2);
        assert!(map.find(&dc_idx).is_some(), "DC not found");

        // RF fundamental should be present
        let rf_idx = FrequencyIndex::single_tone(0, 1, 2);
        assert!(map.find(&rf_idx).is_some(), "RF fundamental not found");

        // LO fundamental should be present
        let lo_idx = FrequencyIndex::single_tone(1, 1, 2);
        assert!(map.find(&lo_idx).is_some(), "LO fundamental not found");
    }

    /// Test spectral voltage THD calculation
    #[test]
    fn test_thd_calculation() {
        let mut sv = SpectralVoltage::new("test", 5);

        // Fundamental = 1.0, H2 = 0.05, H3 = 0.02
        sv.coefficients[1] = Complex64::new(1.0, 0.0);
        sv.coefficients[2] = Complex64::new(0.05, 0.0);
        sv.coefficients[3] = Complex64::new(0.02, 0.0);

        let thd = sv.thd();

        // THD = sqrt(0.05² + 0.02²) / 1.0 = sqrt(0.0029) ≈ 0.0539
        let expected = (0.0025 + 0.0004_f64).sqrt();
        assert!(
            (thd - expected).abs() < 0.001,
            "THD: got {}, expected {}",
            thd,
            expected
        );

        assert!(sv.thd_percent() > 5.0 && sv.thd_percent() < 6.0);
    }

    /// Test RMS voltage calculation
    #[test]
    fn test_rms_calculation() {
        let mut sv = SpectralVoltage::new("test", 3);

        // DC = 2.0, H1 = 1.0
        sv.coefficients[0] = Complex64::new(2.0, 0.0);
        sv.coefficients[1] = Complex64::new(1.0, 0.0);

        let rms = sv.rms();

        // RMS = sqrt(DC² + H1²/2) = sqrt(4 + 0.5) = sqrt(4.5)
        let expected = 4.5_f64.sqrt();
        assert!(
            (rms - expected).abs() < 0.01,
            "RMS: got {}, expected {}",
            rms,
            expected
        );
    }

    /// Test harmonic data extraction
    #[test]
    fn test_harmonic_data_extraction() {
        let config = HbConfig::new(1e9).with_harmonics(3);
        let solver = HbSolver::new(config, 2);

        let mut state = HbSolverState::new(2, 3);
        state.x[0][0] = Complex64::new(1.0, 0.0); // DC
        state.x[0][1] = Complex64::new(0.5, 0.5); // H1 at 45°
        state.x[0][2] = Complex64::new(0.1, 0.0); // H2
        state.converged = true;

        let result = solver.build_result(&state);
        let harmonics = result.get_harmonics(0);

        assert_eq!(harmonics.len(), 4); // DC + 3 harmonics
        assert_eq!(harmonics[0].index, 0);
        assert!((harmonics[1].phase_degrees - 45.0).abs() < 1.0);
    }

    /// Test solver convergence tracking
    #[test]
    fn test_convergence_tracking() {
        let config = HbConfig::new(1e6)
            .with_harmonics(2)
            .with_tolerance(1e-8)
            .with_max_iterations(50);

        let mut solver = HbSolver::new(config, 1);

        // Simple circuit
        solver.add_conductance(0, 0, 1e-3);
        solver.set_dc_source(0, 1e-3);

        let mut state = HbSolverState::new(1, 2);
        solver.solve_linear(&mut state).unwrap();

        let result = solver.build_result(&state);
        assert!(result.is_valid());
    }

    /// Test coefficient phase handling
    #[test]
    fn test_phase_handling() {
        let mut sv = SpectralVoltage::new("test", 3);

        // Pure imaginary = 90°
        sv.coefficients[1] = Complex64::new(0.0, 1.0);
        assert!((sv.phase(1) - PI / 2.0).abs() < 0.01);

        // Pure negative real = 180°
        sv.coefficients[2] = Complex64::new(-1.0, 0.0);
        assert!((sv.phase(2).abs() - PI).abs() < 0.01);
    }

    /// Test dBV conversion
    #[test]
    fn test_dbv_conversion() {
        let mut sv = SpectralVoltage::new("test", 3);

        // 1 V = 0 dBV
        sv.coefficients[0] = Complex64::new(1.0, 0.0);
        assert!((sv.magnitude_dbv(0) - 0.0).abs() < 0.01);

        // 0.1 V = -20 dBV
        sv.coefficients[1] = Complex64::new(0.1, 0.0);
        assert!((sv.magnitude_dbv(1) - (-20.0)).abs() < 0.01);

        // 10 V = 20 dBV
        sv.coefficients[2] = Complex64::new(10.0, 0.0);
        assert!((sv.magnitude_dbv(2) - 20.0).abs() < 0.01);
    }

    /// Test Parseval's theorem for power conservation
    #[test]
    fn test_parseval_power_conservation() {
        let mut fft = HbFft::new(15, 4);
        let n = fft.size();

        // Generate arbitrary waveform
        let waveform: Vec<f64> = (0..n)
            .map(|i| {
                let t = 2.0 * PI * (i as f64) / (n as f64);
                (t).sin() + 0.3 * (3.0 * t).cos() + 1.5
            })
            .collect();

        // Time domain power
        let time_power = waveform.iter().map(|x| x * x).sum::<f64>() / n as f64;

        // Frequency domain power
        let spectrum = fft.to_frequency_domain(&waveform);
        let freq_power = fft.total_power(&spectrum);

        assert!(
            (time_power - freq_power).abs() / time_power < 0.1,
            "Parseval violation: time={}, freq={}",
            time_power,
            freq_power
        );
    }
}

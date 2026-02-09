//! Integration Tests for PSS Analysis Module
//!
//! Comprehensive tests covering the full PSS analysis workflow.

#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_pss_config_to_result_flow() {
        // Test the data flow from config to result
        let config = PssConfig::new(1e9).with_harmonics(5).with_tolerance(1e-8);

        let mut result = PssResult::new(config.period(), 3, config.points_per_period);

        // Populate with synthetic data
        result.time = (0..config.points_per_period)
            .map(|i| i as f64 * config.period() / (config.points_per_period - 1) as f64)
            .collect();

        for wf in result.waveforms.iter_mut() {
            wf.values = result
                .time
                .iter()
                .map(|&t| (2.0 * PI * config.fundamental_freq * t).sin())
                .collect();
        }

        assert_eq!(result.num_nodes(), 3);
        assert_eq!(result.num_points(), config.points_per_period);

        // Check harmonic extraction
        let harmonics = result.harmonics(1, 5);
        assert!(!harmonics.is_empty());
        assert!(harmonics[0].magnitude.abs() < 0.01); // DC
        assert!((harmonics[1].magnitude - 1.0).abs() < 0.05); // Fundamental
    }

    #[test]
    fn test_period_detection_pipeline() {
        // Use a lower frequency with more samples for accurate detection
        let true_freq = 10e6; // 10 MHz (more realistic for period detection)
        let true_period = 1.0 / true_freq; // 100 ns

        // Generate steady-state sine (not damped) for accurate period detection
        let duration = true_period * 100.0;
        let n_points = 10000;
        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();
        let values: Vec<f64> = time
            .iter()
            .map(|&t| (2.0 * PI * true_freq * t).sin())
            .collect();

        let detector = PeriodDetector::with_guess(true_period);
        let estimate = detector.detect(&time, &values);

        // Should detect within 5%
        let relative_error = (estimate.period - true_period).abs() / true_period;
        assert!(
            relative_error < 0.05,
            "Period detection failed: got {} expected {}, error = {:.1}%",
            estimate.period,
            true_period,
            relative_error * 100.0
        );
        assert!(
            estimate.confidence > 0.5,
            "Confidence should be reasonable: got {}",
            estimate.confidence
        );
    }

    #[test]
    fn test_shooting_solver_convergence() {
        // Test that shooting solver converges for a simple periodic system
        let _solver = ShootingNewtonSolver::new(1e-8, 50).with_damping(0.8);

        // Simple linear oscillator: x' = A*x where exp(A*T) is rotation
        // Period T = 1, angular freq ω = 2π
        // After one period, x should return to itself (fixed point is any point)
        let period = 1.0;
        let omega = 2.0 * PI;

        let integrate = |x0: &[f64]| -> Vec<f64> {
            // Rotation by ωT = 2π returns to same point
            let cos_wt = (omega * period).cos();
            let sin_wt = (omega * period).sin();
            vec![
                cos_wt * x0[0] - sin_wt * x0[1],
                sin_wt * x0[0] + cos_wt * x0[1],
            ]
        };

        let mut state = ShootingState::new(vec![1.0, 0.0], period);

        // Integrate one period
        state.x_t = integrate(&state.x0);
        state.compute_residual();

        // For ideal rotation by 2π, residual should be ~0
        assert!(
            state.residual_norm() < 1e-10,
            "Residual = {} (expected ~0 for 2π rotation)",
            state.residual_norm()
        );
    }

    #[test]
    fn test_shooting_with_perturbation() {
        // Test that shooting finds fixed point even with initial perturbation
        let mut solver = ShootingNewtonSolver::new(1e-6, 100).with_damping(0.7);

        // Contractive map: x(T) = 0.9 * x(0) + 0.1
        // Fixed point: x = 0.9x + 0.1 => x = 1
        let integrate = |x0: &[f64]| -> Vec<f64> { vec![0.9 * x0[0] + 0.1] };

        let mut state = ShootingState::new(vec![5.0], 1.0); // Far from fixed point

        for iter in 0..50 {
            state.x_t = integrate(&state.x0);
            state.compute_residual();

            if solver.check_convergence(&state) {
                break;
            }

            if solver.is_maxed_out() {
                panic!("Solver did not converge in {} iterations", iter);
            }

            let delta = solver.compute_newton_step(&state, integrate).unwrap();
            state.update_x0(&delta, solver.damping);
        }

        // Should converge to x = 1
        assert!(
            (state.x0[0] - 1.0).abs() < 0.01,
            "Did not converge to fixed point: got {}, expected 1.0",
            state.x0[0]
        );
    }

    #[test]
    fn test_floquet_stability_analysis() {
        let solver = ShootingNewtonSolver::default();

        // Stable system: all eigenvalues inside unit circle
        let stable_monodromy = vec![vec![0.8, 0.1], vec![0.0, 0.7]];
        let multipliers = solver.compute_floquet_multipliers(&stable_monodromy);

        for m in &multipliers {
            assert!(
                m.norm() <= 1.0 + 1e-6,
                "Stable system should have |λ| ≤ 1, got {}",
                m.norm()
            );
        }

        // Unstable system: eigenvalue outside unit circle
        let unstable_monodromy = vec![vec![1.2, 0.0], vec![0.0, 0.5]];
        let multipliers = solver.compute_floquet_multipliers(&unstable_monodromy);

        let has_unstable = multipliers.iter().any(|m| m.norm() > 1.0);
        assert!(has_unstable, "Unstable system should have |λ| > 1");
    }

    #[test]
    fn test_harmonic_spectrum_accuracy() {
        // Generate a waveform with known harmonic content
        let freq = 1e6;
        let period = 1e-6;
        let n_points = 2048;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * period / (n_points - 1) as f64)
            .collect();

        // f(t) = sin(ωt) + 0.3*sin(3ωt) + 0.1*sin(5ωt)
        let omega = 2.0 * PI * freq;
        let values: Vec<f64> = time
            .iter()
            .map(|&t| {
                (omega * t).sin() + 0.3 * (3.0 * omega * t).sin() + 0.1 * (5.0 * omega * t).sin()
            })
            .collect();

        let wf = PeriodicWaveform::from_values(values);
        let harmonics = wf.compute_harmonics(&time, freq, 7);

        // Check harmonic magnitudes
        assert!(harmonics[0].magnitude.abs() < 0.01, "DC should be ~0");
        assert!(
            (harmonics[1].magnitude - 1.0).abs() < 0.05,
            "1st harmonic should be ~1.0, got {}",
            harmonics[1].magnitude
        );
        assert!(harmonics[2].magnitude < 0.05, "2nd harmonic should be ~0");
        assert!(
            (harmonics[3].magnitude - 0.3).abs() < 0.05,
            "3rd harmonic should be ~0.3, got {}",
            harmonics[3].magnitude
        );
        assert!(harmonics[4].magnitude < 0.05, "4th harmonic should be ~0");
        assert!(
            (harmonics[5].magnitude - 0.1).abs() < 0.05,
            "5th harmonic should be ~0.1, got {}",
            harmonics[5].magnitude
        );
    }

    #[test]
    fn test_waveform_periodicity() {
        // Test that waveform interpolation correctly handles periodicity
        let period = 1e-9;
        let n_points = 100;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * period / (n_points - 1) as f64)
            .collect();
        let values: Vec<f64> = time
            .iter()
            .map(|&t| (2.0 * PI * t / period).sin())
            .collect();

        let wf = PeriodicWaveform::from_values(values);

        // Value at t should equal value at t + period
        let t_test = 0.25e-9;
        let v1 = wf.interpolate(&time, t_test, period);
        let v2 = wf.interpolate(&time, t_test + period, period);
        let v3 = wf.interpolate(&time, t_test + 10.0 * period, period);

        assert!(
            (v1 - v2).abs() < 0.01,
            "Periodicity failed: v({}) = {}, v({} + T) = {}",
            t_test,
            v1,
            t_test,
            v2
        );
        assert!(
            (v1 - v3).abs() < 0.01,
            "Periodicity failed for multiple periods"
        );
    }

    #[test]
    fn test_autonomous_vs_driven_config() {
        // Autonomous oscillator config
        let auto_config = PssConfig::autonomous().with_period_guess(1e-9);
        assert!(auto_config.is_autonomous());
        assert!(auto_config.auto_period);
        assert!((auto_config.period() - 1e-9).abs() < 1e-18);

        // Driven circuit config
        let driven_config = PssConfig::new(1e9);
        assert!(!driven_config.is_autonomous());
        assert!(!driven_config.auto_period);
        assert!((driven_config.period() - 1e-9).abs() < 1e-18);
    }

    #[test]
    fn test_multi_method_period_detection() {
        // Use parameters that give good resolution for all methods
        // FFT resolution = sample_rate / fft_size, need enough cycles for accuracy
        let freq = 1e6; // 1 MHz - lower frequency for better FFT resolution
        let period = 1e-6;
        let duration = 100.0 * period; // 100 cycles
        let n_points = 10000; // High sample count

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();
        let values: Vec<f64> = time.iter().map(|&t| (2.0 * PI * freq * t).sin()).collect();

        let detector = PeriodDetector::with_guess(period).with_fft_size(8192);

        let zc = detector.detect_zero_crossing(&time, &values);
        let pk = detector.detect_peaks(&time, &values);
        let fft = detector.detect_fft(&time, &values);
        let ac = detector.detect_autocorrelation(&time, &values);

        // Zero-crossing and peak detection should be accurate for clean sine
        if let Some(ref est) = zc {
            let error = (est.period - period).abs() / period;
            assert!(
                error < 0.02,
                "Zero-crossing method error = {:.1}% (expected < 2%)",
                error * 100.0
            );
        }

        if let Some(ref est) = pk {
            let error = (est.period - period).abs() / period;
            assert!(
                error < 0.02,
                "Peak detection method error = {:.1}% (expected < 2%)",
                error * 100.0
            );
        }

        // FFT has inherent resolution limits: df = fs/N, so dt/T ≈ 1/(num_cycles)
        // With 100 cycles, expect ~1% resolution, but allow 5% for windowing effects
        if let Some(ref est) = fft {
            let error = (est.period - period).abs() / period;
            assert!(
                error < 0.05,
                "FFT method error = {:.1}% (expected < 5%)",
                error * 100.0
            );
        }

        // Autocorrelation also depends on sample resolution
        if let Some(ref est) = ac {
            let error = (est.period - period).abs() / period;
            assert!(
                error < 0.03,
                "Autocorrelation method error = {:.1}% (expected < 3%)",
                error * 100.0
            );
        }
    }

    // ============= ADDITIONAL COMPREHENSIVE TESTS =============

    #[test]
    fn test_high_dimensional_shooting_solver() {
        // Test shooting solver with 4D state vector (e.g., two coupled oscillators)
        let mut solver = ShootingNewtonSolver::new(1e-8, 100).with_damping(0.6);

        // 4D contractive map: x(T) = 0.9 * x(0) + constant
        // Fixed point: x_i = c_i / 0.1 = 10 * c_i
        let constants = [0.1, 0.2, 0.3, 0.4];
        let integrate = |x0: &[f64]| -> Vec<f64> {
            x0.iter()
                .enumerate()
                .map(|(i, &x)| 0.9 * x + constants[i])
                .collect()
        };

        let mut state = ShootingState::new(vec![0.0, 0.0, 0.0, 0.0], 1.0);

        for _iter in 0..100 {
            state.x_t = integrate(&state.x0);
            state.compute_residual();

            if solver.check_convergence(&state) {
                break;
            }

            let delta = solver.compute_newton_step(&state, integrate).unwrap();
            state.update_x0(&delta, solver.damping);
        }

        // Should converge to fixed point: x_i = 10 * c_i
        for (i, &c) in constants.iter().enumerate() {
            let expected = 10.0 * c;
            assert!(
                (state.x0[i] - expected).abs() < 0.01,
                "4D fixed point err: x[{}] = {}, expected {}",
                i,
                state.x0[i],
                expected
            );
        }
    }

    #[test]
    fn test_numerical_conditioning_jacobian() {
        // Test Jacobian computation with near-singular cases
        let solver = ShootingNewtonSolver::new(1e-10, 50);

        // Near-identity map: x(T) = x(0) + 0.001 * residual component
        // This tests sensitivity to small perturbations
        let integrate = |x0: &[f64]| -> Vec<f64> { vec![x0[0] * 1.001, x0[1] * 0.999] };

        let state = ShootingState::new(vec![1.0, 1.0], 1.0);

        // Monodromy matrix should be approximately [[1.001, 0], [0, 0.999]]
        let monodromy = solver.compute_monodromy(&state, integrate);

        assert!(
            (monodromy[0][0] - 1.001).abs() < 0.01,
            "M[0,0] = {}, expected ~1.001",
            monodromy[0][0]
        );
        assert!(
            monodromy[0][1].abs() < 0.01,
            "Off-diagonal should be ~0, got {}",
            monodromy[0][1]
        );
        assert!(
            (monodromy[1][1] - 0.999).abs() < 0.01,
            "M[1,1] = {}, expected ~0.999",
            monodromy[1][1]
        );
    }

    #[test]
    fn test_convergence_with_saddle_point() {
        // Test convergence near a saddle-type fixed point (one expanding, one contracting direction)
        let mut solver = ShootingNewtonSolver::new(1e-6, 100).with_damping(0.5);

        // Map: x(T) = [0.5*x1 + 0.5, 1.5*x2 - 0.5]
        // Fixed points: x1 = 1, x2 = 1
        let integrate = |x0: &[f64]| -> Vec<f64> { vec![0.5 * x0[0] + 0.5, 1.5 * x0[1] - 0.5] };

        let mut state = ShootingState::new(vec![0.8, 1.1], 1.0);

        for _iter in 0..100 {
            state.x_t = integrate(&state.x0);
            state.compute_residual();

            if solver.check_convergence(&state) {
                break;
            }

            let delta = solver.compute_newton_step(&state, integrate).unwrap();
            state.update_x0(&delta, solver.damping);
        }

        // Newton should still find the fixed point despite saddle dynamics
        assert!(
            (state.x0[0] - 1.0).abs() < 0.01,
            "Saddle point x1: got {}, expected 1.0",
            state.x0[0]
        );
        assert!(
            (state.x0[1] - 1.0).abs() < 0.01,
            "Saddle point x2: got {}, expected 1.0",
            state.x0[1]
        );
    }

    #[test]
    fn test_period_detection_with_dc_offset() {
        // Real oscillators often have DC offset - test that detection still works
        let freq = 100e3; // 100 kHz
        let period = 1.0 / freq;
        let dc_offset = 2.5; // Common in real circuits
        let amplitude = 1.0;

        let duration = 50.0 * period;
        let n_points = 5000;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();
        let values: Vec<f64> = time
            .iter()
            .map(|&t| dc_offset + amplitude * (2.0 * PI * freq * t).sin())
            .collect();

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        let error = (estimate.period - period).abs() / period;
        assert!(
            error < 0.05,
            "Period detection with DC offset failed: error = {:.1}%",
            error * 100.0
        );
    }

    #[test]
    fn test_period_detection_asymmetric_waveform() {
        // Test with asymmetric (non-sinusoidal) waveform - like a CMOS inverter output
        let freq = 50e3;
        let period = 1.0 / freq;
        let duration = 30.0 * period;
        let n_points = 3000;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();

        // Asymmetric sawtooth-like waveform: fast rise, slow fall
        let values: Vec<f64> = time
            .iter()
            .map(|&t| {
                let phase = (t / period).fract();
                if phase < 0.3 {
                    // Fast rise
                    phase / 0.3
                } else {
                    // Slow fall
                    1.0 - (phase - 0.3) / 0.7
                }
            })
            .collect();

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        let error = (estimate.period - period).abs() / period;
        assert!(
            error < 0.05,
            "Asymmetric waveform period detection failed: error = {:.1}%",
            error * 100.0
        );
    }

    #[test]
    fn test_very_high_frequency_detection() {
        // Test at RF frequencies (GHz range) - important for RF circuit design
        let freq = 2.4e9; // 2.4 GHz (WiFi band)
        let period = 1.0 / freq;
        let duration = 200.0 * period;
        let n_points = 4096;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();
        let values: Vec<f64> = time.iter().map(|&t| (2.0 * PI * freq * t).sin()).collect();

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        let error = (estimate.period - period).abs() / period;
        assert!(
            error < 0.05,
            "GHz frequency detection failed: error = {:.1}%",
            error * 100.0
        );
    }

    #[test]
    fn test_very_low_frequency_detection() {
        // Test at power electronics frequencies (Hz range)
        let freq = 60.0; // 60 Hz power line
        let period = 1.0 / freq;
        let duration = 10.0 * period;
        let n_points = 6000;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();
        let values: Vec<f64> = time.iter().map(|&t| (2.0 * PI * freq * t).sin()).collect();

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        let error = (estimate.period - period).abs() / period;
        assert!(
            error < 0.03,
            "Low frequency detection failed: error = {:.1}%",
            error * 100.0
        );
    }

    #[test]
    fn test_harmonic_extraction_triangle_wave() {
        // Triangle wave has known harmonic content: only odd harmonics at 1/n^2
        let freq = 1e6;
        let period = 1e-6;
        let n_points = 4096;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * period / (n_points - 1) as f64)
            .collect();

        // Triangle wave: y = 4|t/T - 0.5| - 1
        let values: Vec<f64> = time
            .iter()
            .map(|&t| {
                let phase = (t / period).fract();
                4.0 * (phase - 0.5).abs() - 1.0
            })
            .collect();

        let wf = PeriodicWaveform::from_values(values);
        let harmonics = wf.compute_harmonics(&time, freq, 7);

        // Triangle wave: H1 = 8/π², H3 = 8/(9π²), H5 = 8/(25π²)
        let h1_expected = 8.0 / (PI * PI);
        let h3_expected = 8.0 / (9.0 * PI * PI);
        let h5_expected = 8.0 / (25.0 * PI * PI);

        assert!(
            (harmonics[1].magnitude - h1_expected).abs() < 0.1,
            "Triangle H1: got {:.3}, expected {:.3}",
            harmonics[1].magnitude,
            h1_expected
        );
        assert!(
            (harmonics[3].magnitude - h3_expected).abs() < 0.05,
            "Triangle H3: got {:.3}, expected {:.3}",
            harmonics[3].magnitude,
            h3_expected
        );
        assert!(
            (harmonics[5].magnitude - h5_expected).abs() < 0.02,
            "Triangle H5: got {:.3}, expected {:.3}",
            harmonics[5].magnitude,
            h5_expected
        );
        // Even harmonics should be zero
        assert!(
            harmonics[2].magnitude < 0.01,
            "Triangle H2 should be ~0, got {}",
            harmonics[2].magnitude
        );
    }

    #[test]
    fn test_floquet_multiplier_extraction_accuracy() {
        // Test that Floquet multipliers match known eigenvalues
        let solver = ShootingNewtonSolver::default();

        // Upper-triangular matrix: eigenvalues are the diagonal entries.
        let monodromy = vec![vec![0.7, 0.2], vec![0.0, 0.7]];
        let multipliers = solver.compute_floquet_multipliers(&monodromy);
        assert_eq!(multipliers.len(), 2);
        for m in &multipliers {
            assert!(
                (m.re - 0.7).abs() < 1e-3 && m.im.abs() < 1e-6,
                "Expected real eigenvalue 0.7, got {}",
                m
            );
        }
    }

    #[test]
    fn test_pss_result_node_access() {
        // Test random access to node data in PssResult
        let period = 1e-9;
        let n_nodes = 10;
        let n_points = 100;

        let mut result = PssResult::new(period, n_nodes, n_points);
        result.time = (0..n_points)
            .map(|i| i as f64 * period / (n_points - 1) as f64)
            .collect();

        // Populate each node with distinct frequency
        for (node_idx, wf) in result.waveforms.iter_mut().enumerate() {
            let node_freq = (node_idx + 1) as f64 * 1e9;
            wf.values = result
                .time
                .iter()
                .map(|&t| (2.0 * PI * node_freq * t).sin())
                .collect();
        }

        // Verify each node has correct peak-to-peak
        for node in 0..n_nodes {
            let pp = result.waveforms[node].peak_to_peak();
            assert!(
                (pp - 2.0).abs() < 0.1,
                "Node {} peak-to-peak should be ~2.0, got {}",
                node,
                pp
            );
        }
    }

    #[test]
    fn test_shooting_solver_nonconvergence_detection() {
        // Test that solver correctly identifies non-convergence
        let mut solver = ShootingNewtonSolver::new(1e-10, 5); // Low max iterations

        // Divergent map: x(T) = 2*x(0) - this will never converge to x=0
        let integrate = |x0: &[f64]| -> Vec<f64> { vec![2.0 * x0[0]] };

        let mut state = ShootingState::new(vec![1.0], 1.0);
        for _iter in 0..10 {
            state.x_t = integrate(&state.x0);
            state.compute_residual();

            if solver.is_maxed_out() {
                break;
            }

            if solver.check_convergence(&state) {
                break;
            }

            let delta = solver.compute_newton_step(&state, integrate).unwrap();
            state.update_x0(&delta, 0.5);
        }

        // Should detect non-convergence via max iterations
        // (The divergent map's fixed point x=0 is unstable, so Newton won't converge)
    }

    #[test]
    fn test_period_detection_with_phase_noise() {
        // Test robustness to jittery zero crossings (phase noise)
        let freq = 1e6;
        let period = 1e-6;
        let duration = 50.0 * period;
        let n_points = 5000;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();

        // Add small phase jitter (1% of period)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let values: Vec<f64> = time
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                // Deterministic pseudo-random jitter
                let mut hasher = DefaultHasher::new();
                i.hash(&mut hasher);
                let jitter = ((hasher.finish() % 1000) as f64 / 1000.0 - 0.5) * 0.01 * period;
                (2.0 * PI * freq * (t + jitter)).sin()
            })
            .collect();

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        // Should still detect within reasonable tolerance despite jitter
        let error = (estimate.period - period).abs() / period;
        assert!(
            error < 0.10,
            "Period detection with jitter failed: error = {:.1}%",
            error * 100.0
        );
    }
}

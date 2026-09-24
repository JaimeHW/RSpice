//! Verify that authored AC grids reach the engine unchanged.

use crate::simulation::config::{AcAnalysisConfig, AcSweepType};
use crate::simulation::results::SimulationResult;
use crate::simulation::{AnalysisConfig, EngineBridge};

#[test]
fn single_frequency_and_zero_start_lin_run_the_requested_grid() {
    for (kind, start, stop, count, expected) in [
        (AcSweepType::Linear, 0.0, 0.0, 1, vec![0.0]),
        (
            AcSweepType::Linear,
            0.0,
            1000.0,
            3,
            vec![0.0, 500.0, 1000.0],
        ),
        (AcSweepType::Decade, 1000.0, 1000.0, 10, vec![1000.0]),
        (AcSweepType::Octave, 1000.0, 1000.0, 4, vec![1000.0]),
    ] {
        let config = AcAnalysisConfig {
            sweep_type: kind,
            start_freq: start,
            stop_freq: stop,
            num_points: count,
        };
        config.validate().unwrap();
        let result = EngineBridge::new()
            .run(
                &AnalysisConfig::Ac(config),
                "AC divider\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
            )
            .unwrap();
        let SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } = result
        else {
            panic!("expected AC");
        };
        assert_eq!(frequencies, expected);
        let trace = &waveforms["V(OUT)"];
        assert!(
            trace
                .y_values
                .iter()
                .all(|value| (value - 0.5).abs() < 1e-12)
        );
        assert!(
            trace
                .y_imag
                .as_ref()
                .unwrap()
                .iter()
                .all(|value| value.abs() < 1e-12)
        );
    }
}

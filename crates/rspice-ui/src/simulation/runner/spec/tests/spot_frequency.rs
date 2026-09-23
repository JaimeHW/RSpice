use super::*;
use crate::simulation::config::{NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType};
use crate::simulation::multi_run::FrequencySweep;
use crate::simulation::runner::worker_contract::{WorkerAnalysisSpec, round_trip_response_for_test};

#[test]
fn single_frequency_noise_stb_and_disto_execute_and_publish_one_point() {
    let run = |spec: AnalysisSpec, circuit: &str| {
        let request = WorkerAnalysisSpec::try_from(&spec).unwrap();
        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::from(request),
            SpecExecutionOptions::default(),
            circuit,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::NoAbort,
        )
        .unwrap();
        round_trip_response_for_test(result)
    };
    for (sweep, noise_sweep) in [
        (FrequencySweep::Linear, NoiseSweepType::Linear),
        (FrequencySweep::Decade, NoiseSweepType::Decade),
        (FrequencySweep::Octave, NoiseSweepType::Octave),
    ] {
        let noise = run(
            AnalysisSpec::Noise {
                output_node: "out".into(),
                reference_node: "0".into(),
                input_source: "VIN".into(),
                start_freq: 1000.0,
                stop_freq: 1000.0,
                points_per_decade: 1,
                sweep: noise_sweep,
                explicit_frequencies: None,
                data_table_name: None,
                contribution_detail: NoiseContributionDetail::Top50,
                integration_mode: NoiseIntegrationMode::Enabled,
                temperature: 300.0,
            },
            "spot noise\nVIN in 0 DC 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
        );
        let SimulationResult::Noise {
            frequencies,
            output_noise,
            input_noise,
            ..
        } = noise
        else {
            panic!("expected noise result");
        };
        assert_eq!(frequencies, [1000.0]);
        let expected = 4.0 * rspice_core::constants::K_BOLTZMANN * 300.0 * 500.0;
        assert!((output_noise[0] / expected - 1.0).abs() < 1e-9);
        assert!((input_noise.unwrap()[0] / (expected * 4.0) - 1.0).abs() < 1e-9);

        let stability = run(
            AnalysisSpec::Stb {
                probe_node: "VPROBE".into(),
                start_freq: 1000.0,
                stop_freq: 1000.0,
                points_per_decade: 1,
                sweep,
                compute_nyquist: true,
            },
            "spot stability\nE1 EO 0 CTRL 0 -1000\nVPROBE EO X 0\nR1 X CTRL 1k\nC1 CTRL 0 159.154943091895n\n.end\n",
        );
        let SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } = stability
        else {
            panic!("expected stability result");
        };
        assert_eq!(frequencies, [1000.0]);
        let expected_gain = 20.0 * (1000.0_f64 / 2.0_f64.sqrt()).log10();
        assert!((waveforms["Loop Gain (dB)"].y_values[0] - expected_gain).abs() < 1e-6);
        assert_eq!(
            waveforms[crate::simulation::results::STB_NYQUIST_CONTOUR_WAVEFORM]
                .y_values
                .len(),
            1
        );

        let distortion = run(
            AnalysisSpec::Disto {
                start_freq: 1000.0,
                stop_freq: 1000.0,
                points_per_unit: 1,
                sweep,
                f2_over_f1: None,
            },
            "spot distortion\nVIN in 0 DISTOF1 1 0\nR1 in out 1k\nR2 out 0 1k\n.end\n",
        );
        let SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } = distortion
        else {
            panic!("expected distortion result");
        };
        assert_eq!(frequencies, [1000.0]);
        let f1 = waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out) F1"))
            .unwrap()
            .1;
        let thd = waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out) THD"))
            .unwrap()
            .1;
        assert!((f1.y_values[0] - 0.5).abs() < 1e-12);
        assert_eq!(thd.y_values, [0.0]);
    }
}

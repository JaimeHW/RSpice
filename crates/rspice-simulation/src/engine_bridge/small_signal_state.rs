//! Small-signal cards around a transient state over the engine bridge.
//!
//! A prepared task whose card carries `ACTIMES=` or `CHECKPOINT=` holds its
//! parent's `.TRAN` card and its own (`execution::headless_tasks`), and a
//! `CHECKPOINT=` task carries the image its card names. The card runs
//! through the engine's state entry, which runs the transient to the state,
//! or restores the image, and linearizes there; its one result is projected
//! as the task's configuration projects a run at the operating point.

use std::path::Path;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{SmallSignalCardResult, TransientCheckpoint};
use rspice_core::netlist::{AnalysisCommand, SmallSignalStateSource};
use rspice_simulation_contract::config::AnalysisConfig;

use super::{EngineBridge, SimulationInput, ensure_not_aborted};
use crate::error::SimulationError;
use crate::results::SimulationResult;
use crate::transient_checkpoint::TransientCheckpointInput;

/// How far the task's noise temperature may sit from the circuit's, where
/// the card runs, before the two are different temperatures.
const TEMPERATURE_TOLERANCE: f64 = 1.0e-12;

impl EngineBridge {
    /// Run `config` around the transient state its card in `netlist` names,
    /// or `None` when the analysis is not one a card around a state serves.
    pub(super) fn run_at_transient_state(
        &self,
        config: &AnalysisConfig,
        netlist: &rspice_core::Netlist,
        abort: &dyn AbortSignal,
    ) -> Option<Result<SimulationResult, SimulationError>> {
        matches!(config, AnalysisConfig::Ac(_) | AnalysisConfig::Noise(_))
            .then(|| self.run_card_at_transient_state(config, netlist, None, abort))
    }

    /// Run `config` around the state `checkpoint` holds, which the task's
    /// `CHECKPOINT=` card names.
    pub(crate) fn run_at_checkpoint_state(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
        environment: Option<crate::runner::AnalysisExecutionEnvironment>,
        checkpoint: &TransientCheckpointInput,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let netlist = self.materialize_input(
            &SimulationInput {
                config,
                netlist_str,
                source_path,
                supply_corner: None,
                environment,
            },
            abort,
        )?;
        if !matches!(config, AnalysisConfig::Ac(_) | AnalysisConfig::Noise(_)) {
            return Err(SimulationError::InvalidConfig(
                "Only an AC or noise task linearizes around a checkpoint's state".into(),
            ));
        }
        let checkpoint = checkpoint.decode(self.resource_limits(), abort)?;
        self.run_card_at_transient_state(config, &netlist, Some(&checkpoint), abort)
    }

    fn run_card_at_transient_state(
        &self,
        config: &AnalysisConfig,
        netlist: &rspice_core::Netlist,
        checkpoint: Option<&TransientCheckpoint>,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let refuse = |message: &str| {
            Err(SimulationError::InvalidConfig(format!(
                "A small-signal task around a transient state {message}"
            )))
        };
        let [request] = netlist.small_signal_states.as_slice() else {
            return refuse("holds one card around one state");
        };
        let index = request.analysis_index;
        let same_kind = matches!(
            (config, netlist.analyses.get(index)),
            (AnalysisConfig::Ac(_), Some(AnalysisCommand::Ac { .. }))
                | (
                    AnalysisConfig::Noise(_),
                    Some(AnalysisCommand::Noise { .. })
                )
        );
        if !same_kind {
            return refuse("runs the card its analysis names");
        }
        match (&request.source, checkpoint) {
            (SmallSignalStateSource::TransientTimes(times), None) if times.len() == 1 => {}
            (SmallSignalStateSource::TransientTimes(_), None) => {
                return refuse("is one state, at one time");
            }
            (SmallSignalStateSource::CheckpointFile(_), Some(_)) => {}
            (SmallSignalStateSource::CheckpointFile(name), None) => {
                return refuse(&format!(
                    "carries the image of its checkpoint file {name}, and this one carries none"
                ));
            }
            (SmallSignalStateSource::TransientTimes(_), Some(_)) => {
                return refuse("names its times or a checkpoint, not both");
            }
        }
        let engine = self.engine_for_netlist(netlist);
        if let AnalysisConfig::Noise(noise) = config {
            let circuit = engine.config().temperature;
            if (noise.temperature_kelvin - circuit).abs() > TEMPERATURE_TOLERANCE * circuit {
                return Err(SimulationError::InvalidConfig(format!(
                    "A noise card around a transient state runs at the circuit's temperature, \
                     {circuit} K, and the task asks for {} K",
                    noise.temperature_kelvin
                )));
            }
        }
        let state = match (&request.source, checkpoint) {
            (SmallSignalStateSource::TransientTimes(times), _) => {
                let mut states = engine
                    .run_small_signal_at_transient_times(netlist, index, times, abort)
                    .map_err(|error| self.translate_error(error))?;
                let (Some(state), true) = (states.pop(), states.is_empty()) else {
                    return refuse("publishes one result");
                };
                state
            }
            (SmallSignalStateSource::CheckpointFile(_), Some(checkpoint)) => engine
                .run_small_signal_at_checkpoint(netlist, index, checkpoint, abort)
                .map_err(|error| self.translate_error(error))?,
            (SmallSignalStateSource::CheckpointFile(_), None) => {
                return refuse("carries the image of its checkpoint file");
            }
        };
        match (config, state.result) {
            (AnalysisConfig::Ac(_), SmallSignalCardResult::Ac(points)) => {
                let frequencies = points.iter().map(|point| point.frequency).collect();
                Self::finish_ac_run(netlist, frequencies, points, abort)
            }
            (AnalysisConfig::Noise(noise), SmallSignalCardResult::Noise(points)) => {
                Self::finish_noise_run(netlist, noise, points, abort)
            }
            _ => refuse("publishes the result its analysis names"),
        }
    }
}

#[cfg(test)]
mod tests {
    use rspice_core::NoAbort;
    use rspice_simulation_contract::config::{AcAnalysisConfig, AcSweepType};

    use super::*;

    /// A diode charging a capacitor through a resistor: the diode's
    /// conductance falls as the capacitor charges, so the state at 20 us
    /// linearizes differently from the operating point.
    const DECK: &str = "* diode-RC charging\n\
        v1 in 0 pulse(0 1 0 100n 100n 1 2) ac 1\n\
        r1 in a 10k\n\
        d1 a out dmod\n\
        c1 out 0 1n\n\
        r2 out 0 1meg\n\
        .model dmod d is=1e-14 n=1.2 cjo=5p\n\
        .tran 0.1u 20u 0 0.1u\n";

    fn ac_config() -> AnalysisConfig {
        AnalysisConfig::Ac(AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 2,
            start_freq: 1e3,
            stop_freq: 1e6,
        })
    }

    fn v_out(result: &SimulationResult) -> Vec<(f64, f64)> {
        let SimulationResult::Ac { waveforms, .. } = result else {
            panic!("an AC result: {result:?}");
        };
        let waveform = waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("v(out)"))
            .map(|(_, waveform)| waveform)
            .expect("v(out)");
        let imaginary = waveform.y_imag.as_ref().expect("a complex waveform");
        waveform
            .y_values
            .iter()
            .copied()
            .zip(imaginary.iter().copied())
            .collect()
    }

    #[test]
    fn an_ac_card_at_a_state_publishes_the_engines_state_result() {
        let netlist =
            rspice_core::Netlist::parse(&format!("{DECK}.ac dec 2 1k 1meg actimes=20u\n.end\n"))
                .expect("deck parses");
        let bridge = EngineBridge::new();
        let result = bridge
            .run_at_transient_state(&ac_config(), &netlist, &NoAbort)
            .expect("an AC task is served around a state")
            .expect("the state run succeeds");
        let SmallSignalStateSource::TransientTimes(times) = &netlist.small_signal_states[0].source
        else {
            panic!("the card names a time");
        };
        let expected = rspice_core::Engine::default()
            .run_small_signal_at_transient_times(&netlist, 1, times, &NoAbort)
            .expect("the engine runs the state");
        let SmallSignalCardResult::Ac(points) = &expected[0].result else {
            panic!("an AC state result");
        };
        let out = points[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("node out");
        let expected: Vec<(f64, f64)> = points
            .iter()
            .map(|point| (point.voltages[out].re, point.voltages[out].im))
            .collect();
        assert_eq!(v_out(&result), expected);

        let at_operating_point =
            rspice_core::Netlist::parse(&format!("{DECK}.ac dec 2 1k 1meg\n.end\n"))
                .expect("deck parses");
        let operating = bridge
            .dispatch_analysis(&ac_config(), &at_operating_point, &NoAbort)
            .expect("the operating-point run succeeds");
        assert_ne!(
            v_out(&operating),
            expected,
            "the state at 20 us is not the operating point"
        );
    }

    #[test]
    fn a_noise_card_at_a_state_runs_at_the_circuits_temperature() {
        let netlist = rspice_core::Netlist::parse(&format!(
            "{DECK}.noise v(out) v1 dec 2 1k 1meg actimes=20u\n.end\n"
        ))
        .expect("deck parses");
        let bridge = EngineBridge::new();
        let config = |temperature_kelvin| {
            AnalysisConfig::Noise(rspice_simulation_contract::config::NoiseAnalysisConfig {
                output_node: "out".to_owned(),
                reference_node: "0".to_owned(),
                input_source: "v1".to_owned(),
                sweep_type: AcSweepType::Decade,
                num_points: 2,
                start_freq: 1e3,
                stop_freq: 1e6,
                explicit_frequencies: None,
                data_table_name: None,
                contribution_detail: Default::default(),
                integration_mode: Default::default(),
                temperature_kelvin,
            })
        };
        let circuit = bridge.engine_for_netlist(&netlist).config().temperature;
        let result = bridge
            .run_at_transient_state(&config(circuit), &netlist, &NoAbort)
            .expect("a noise task is served around a state")
            .expect("the state run succeeds");
        let SimulationResult::Noise {
            frequencies,
            output_noise,
            ..
        } = &result
        else {
            panic!("a noise result: {result:?}");
        };
        assert_eq!(frequencies.len(), output_noise.len());
        assert!(output_noise.iter().all(|density| *density > 0.0));
        let error = bridge
            .run_at_transient_state(&config(circuit + 50.0), &netlist, &NoAbort)
            .expect("a noise task is served around a state")
            .expect_err("another temperature is refused");
        assert!(
            error.to_string().contains("circuit's temperature"),
            "{error}"
        );
    }

    #[test]
    fn a_state_task_runs_only_its_own_card_at_one_time() {
        let bridge = EngineBridge::new();
        for (card, reason) in [
            (".ac dec 2 1k 1meg actimes=10u,20u", "one time"),
            (
                ".noise v(out) v1 dec 2 1k 1meg actimes=20u",
                "its analysis names",
            ),
        ] {
            let netlist =
                rspice_core::Netlist::parse(&format!("{DECK}{card}\n.end\n")).expect("deck parses");
            let error = bridge
                .run_at_transient_state(&ac_config(), &netlist, &NoAbort)
                .expect("an AC task is served around a state")
                .expect_err("the task is refused");
            assert!(error.to_string().contains(reason), "{card}: {error}");
        }
        let netlist =
            rspice_core::Netlist::parse(&format!("{DECK}.ac dec 2 1k 1meg actimes=20u\n.end\n"))
                .expect("deck parses");
        let transient = AnalysisConfig::Transient(Default::default());
        assert!(
            bridge
                .run_at_transient_state(&transient, &netlist, &NoAbort)
                .is_none(),
            "a transient task runs its own analysis"
        );
    }
}

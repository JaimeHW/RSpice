//! Verilog-A noise processes sampled on the periodic orbit.
//!
//! A `white_noise` or `flicker_noise` call of a runtime or generated instance
//! is one noise process (see `crate::engine::transient::device_noise::processes`):
//! one unit train reaching the circuit through its injections. Periodic noise
//! evaluates every process at each collocation sample of the carrier, as
//! `.noise` evaluates it at the operating point. A white process's power
//! times its squared gain is the intensity `s(t)` of a cyclostationary
//! current; a flicker process's is the power of the amplitude `u(t)` that
//! modulates one stationary `1/f^ef` process, as the native junction flicker
//! laws are sampled.
//!
//! Folding needs a process to reach one pair of node rows through a gain that
//! is real and the same at every frequency, and to keep its row pair and its
//! spectral law over the orbit. A process that does not (a noise table, a
//! filtered or multi-row injection, one entering a branch equation) is
//! refused by instance and mechanism before any folding.

use super::bjt::{NativeNoiseWaveform, sampled_noise_sources};
use super::*;
use crate::engine::UnsupportedCapabilityError;
use crate::engine::transient::device_noise::processes::{
    CollectedNoiseProcess, NoiseProcessKey, NoiseProcessSpectrum, collect_noise_processes,
    has_structural_noise_processes,
};
use std::collections::BTreeMap;

/// The token every refused Verilog-A noise process carries: the same one
/// the structural cyclostationary gaps raise.
const COLORED_NOISE: &str = "analysis.pnoise.colored_noise";

/// `process` refused by instance, module and mechanism.
fn refused(process: &CollectedNoiseProcess, reason: &str) -> SimulationError {
    UnsupportedCapabilityError::new(
        COLORED_NOISE,
        format!(
            "periodic noise cannot fold Verilog-A noise process '{}' of instance '{}' (module \
             '{}'): {reason}",
            process.mechanism, process.instance, process.module
        ),
    )
    .for_instance(process.instance.clone())
    .for_module(process.module.clone())
    .into()
}

/// One sample of one process: its rows, its frequency exponent and its power
/// times its squared gain.
fn sampled(
    process: &CollectedNoiseProcess,
    shifted: &CollectedNoiseProcess,
    num_nodes: usize,
) -> Result<([usize; 2], Option<Value>, ScaledNonnegative), SimulationError> {
    let [injection] = process.injections.as_slice() else {
        return Err(refused(
            process,
            "one noise train reaches more than one pair of circuit rows",
        ));
    };
    if injection.gain.im != 0.0
        || shifted
            .injections
            .first()
            .is_none_or(|other| other.gain != injection.gain)
    {
        return Err(refused(
            process,
            "it reaches the circuit through a gain that varies with frequency",
        ));
    }
    if injection.node_pos > num_nodes || injection.node_neg > num_nodes {
        return Err(refused(
            process,
            "it enters a branch equation, which periodic noise folding does not address",
        ));
    }
    let exponent = match process.spectrum {
        NoiseProcessSpectrum::White => None,
        NoiseProcessSpectrum::Flicker { exponent } => Some(exponent),
        NoiseProcessSpectrum::Table { .. } => {
            return Err(refused(
                process,
                "a noise table has no cyclostationary form",
            ));
        }
    };
    let gain = injection.gain.re.abs();
    let power = checked_scaled_positive_product(
        &[process.psd, gain, gain],
        &format!("periodic noise process '{}'", process.describe()),
    )?;
    Ok(([injection.node_pos, injection.node_neg], exponent, power))
}

impl Engine {
    /// Every runtime and generated Verilog-A noise process, sampled over the
    /// retained orbit.
    pub(in crate::engine::hb) fn veriloga_periodic_noise_sources(
        circuit: &CircuitData,
        solver: &mut HbSolver,
        state: &HbSolverState,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PeriodicNoiseSource>, SimulationError> {
        if !has_structural_noise_processes(circuit) {
            return Ok(Vec::new());
        }
        let num_nodes = circuit.num_nodes();
        // A gain the same at two unrelated frequencies, and real, has no
        // reactive or filtered part.
        let frequency = solver.fundamental_freq();
        let mut waveforms: BTreeMap<NoiseProcessKey, NativeNoiseWaveform> = BTreeMap::new();
        let mut failure = None;
        let mut sample = |time: usize, count: usize, solution: &[Value]| {
            let processes = collect_noise_processes(circuit, solution, frequency)?;
            let shifted = collect_noise_processes(circuit, solution, 2.5 * frequency)?;
            for process in &processes {
                let Some(other) = shifted.iter().find(|other| other.key() == process.key()) else {
                    return Err(refused(
                        process,
                        "it is active at one frequency and not another",
                    ));
                };
                let (nodes, exponent, power) = sampled(process, other, num_nodes)?;
                let waveform =
                    waveforms
                        .entry(process.key())
                        .or_insert_with(|| NativeNoiseWaveform {
                            name: process.describe(),
                            nodes,
                            frequency_exponent: exponent,
                            samples: vec![ScaledNonnegative::ZERO; count],
                        });
                if waveform.nodes != nodes {
                    return Err(refused(
                        process,
                        "it moves between circuit rows over the orbit",
                    ));
                }
                if waveform.frequency_exponent != exponent {
                    return Err(refused(process, "its spectral law changes over the orbit"));
                }
                waveform.samples[time] = power;
            }
            Ok(())
        };
        let visited = solver.visit_periodic_samples(state, abort, |time, count, solution| {
            sample(time, count, solution).map_err(|error| {
                let message = error.to_string();
                failure = Some(error);
                crate::analysis::HbError::InvalidCircuit(message)
            })
        });
        if let Some(error) = failure {
            return Err(error);
        }
        visited.map_err(|error| match error {
            crate::analysis::HbError::Aborted => SimulationError::Aborted,
            error => SimulationError::Circuit(format!(
                "Verilog-A periodic-noise sampling failed: {error}"
            )),
        })?;
        sampled_noise_sources(
            solver,
            waveforms
                .into_values()
                .filter(|waveform| waveform.nodes[0] != waveform.nodes[1])
                .collect(),
            abort,
        )
    }
}

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
//! Folding needs a process to reach the circuit through gains that are real
//! and the same at every frequency, and to keep the rows it reaches, their
//! proportion and its spectral law over the orbit. The rows may be any set of
//! node and branch rows: a contribution whose current a flow probe also reads
//! reaches the probe's row as well, and one noise function may feed several
//! branch equations (LRM 2.4 section 4.6.4.6). A process that does not (a
//! noise table, a filtered injection, rows that move or change proportion)
//! is refused by instance and mechanism before any folding.

use super::bjt::{NativeNoiseWaveform, sampled_noise_sources};
use super::*;
use crate::engine::UnsupportedCapabilityError;
use crate::engine::transient::device_noise::processes::{
    CollectedNoiseProcess, GeneratedOrbitSample, NoiseProcessKey, NoiseProcessSpectrum,
    collect_noise_processes_on_orbit, has_structural_noise_processes,
};
use rspice_veriloga_runtime::GeneratedPeriodicFrame;
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

/// The rows one sample of one process reaches: the pair of one-based node
/// rows its unit current flows between, and every further one-based row,
/// node or branch, with its weight relative to that current. A process that
/// reaches one pair of node rows has no further rows; one that reaches any
/// other set has the pair `[0, 0]` and every row among the further ones.
#[derive(Clone, Debug, PartialEq)]
struct ProcessRows {
    nodes: [usize; 2],
    further: Vec<(usize, Value)>,
}

impl ProcessRows {
    /// Whether these are the rows `waveform` reaches, in its proportion up to
    /// rounding.
    fn reach_as(&self, waveform: &NativeNoiseWaveform) -> bool {
        self.nodes == waveform.nodes
            && self.further.len() == waveform.further_rows.len()
            && self.further.iter().zip(&waveform.further_rows).all(
                |(&(row, weight), &(other_row, other_weight))| {
                    row == other_row
                        && (weight - other_weight).abs()
                            <= 64.0 * Value::EPSILON * weight.abs().max(other_weight.abs())
                },
            )
    }
}

/// One sample of one process: the rows it reaches, its frequency exponent,
/// and its power times its squared gain; `None` when its injections cancel
/// and it reaches no row at this sample.
///
/// A process's injection vector is the sum of its injections' gains over
/// the rows each enters, and periodic noise needs only that vector: the
/// output noise of a unit train reaching the circuit through `u` is
/// `|yᴴu|²` per sideband for the adjoint `y`, whatever rows `u` reaches. A
/// process that reaches one pair of node rows keeps that pair; any other is
/// scaled to the weight of its first row, which the power carries.
fn sampled(
    process: &CollectedNoiseProcess,
    shifted: &CollectedNoiseProcess,
    num_nodes: usize,
) -> Result<Option<(ProcessRows, Option<Value>, ScaledNonnegative)>, SimulationError> {
    if process.injections.len() != shifted.injections.len()
        || process
            .injections
            .iter()
            .zip(&shifted.injections)
            .any(|(injection, other)| injection.gain.im != 0.0 || other.gain != injection.gain)
    {
        return Err(refused(
            process,
            "it reaches the circuit through a gain that varies with frequency",
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
    let name = || format!("periodic noise process '{}'", process.describe());
    if let [injection] = process.injections.as_slice()
        && injection.node_pos <= num_nodes
        && injection.node_neg <= num_nodes
    {
        let gain = injection.gain.re.abs();
        let power = checked_scaled_positive_product(&[process.psd, gain, gain], &name())?;
        let rows = ProcessRows {
            nodes: [injection.node_pos, injection.node_neg],
            further: Vec::new(),
        };
        return Ok(Some((rows, exponent, power)));
    }
    // The injection vector over one-based rows, ground dropped.
    let mut weights: Vec<(usize, Value)> = Vec::with_capacity(2 * process.injections.len());
    for injection in &process.injections {
        for (row, gain) in [
            (injection.node_pos, injection.gain.re),
            (injection.node_neg, -injection.gain.re),
        ] {
            if row == 0 {
                continue;
            }
            match weights.iter_mut().find(|(existing, _)| *existing == row) {
                Some((_, weight)) => *weight += gain,
                None => weights.push((row, gain)),
            }
        }
    }
    weights.retain(|&(_, weight)| weight != 0.0);
    weights.sort_unstable_by_key(|&(row, _)| row);
    let (rows, gain) = match weights.as_slice() {
        [] => return Ok(None),
        &[(row, weight)] if row <= num_nodes => (
            ProcessRows {
                nodes: [row, 0],
                further: Vec::new(),
            },
            weight.abs(),
        ),
        &[(first, weight), (second, other)] if second <= num_nodes && weight == -other => (
            ProcessRows {
                nodes: [first, second],
                further: Vec::new(),
            },
            weight.abs(),
        ),
        &[(_, scale), ..] => (
            ProcessRows {
                nodes: [0, 0],
                further: weights
                    .iter()
                    .map(|&(row, weight)| (row, weight / scale))
                    .collect(),
            },
            scale.abs(),
        ),
    };
    let power = checked_scaled_positive_product(&[process.psd, gain, gain], &name())?;
    Ok(Some((rows, exponent, power)))
}

/// Each generated instance as one sample of the orbit holds it, in the
/// circuit's order: harmonic balance's sampled copy of an instance with noise
/// processes (see [`GeneratedOrbitSample`]), the periodic state's row of each
/// carried operator site's output, and the frame that supplies those outputs
/// to the instance's noise evaluation. An instance with no carried site has
/// no rows and no frame.
struct GeneratedNoiseSamples {
    #[cfg(feature = "veriloga-builtins-base")]
    instances: Vec<Option<crate::device::veriloga_builtins::BuiltinVerilogAInstance>>,
    rows: Vec<Vec<usize>>,
    outputs: Vec<Value>,
    frames: Vec<Option<GeneratedPeriodicFrame>>,
}

impl GeneratedNoiseSamples {
    fn new(circuit: &CircuitData, solver: &HbSolver) -> Result<Self, SimulationError> {
        // Without generated models nothing is pushed.
        #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(unused_mut))]
        let mut samples = Self {
            #[cfg(feature = "veriloga-builtins-base")]
            instances: Vec::new(),
            rows: Vec::new(),
            outputs: Vec::new(),
            frames: Vec::new(),
        };
        #[cfg(feature = "veriloga-builtins-base")]
        for device in circuit.generated_veriloga_devices().iter() {
            let unsampled = || {
                SimulationError::Circuit(format!(
                    "periodic noise found no periodic sampler for generated Verilog-A instance \
                     '{}'",
                    device.instance_name
                ))
            };
            if !device.has_grouped_noise_processes() {
                samples.instances.push(None);
                samples.rows.push(Vec::new());
                samples.frames.push(None);
                continue;
            }
            samples.instances.push(Some(
                solver
                    .generated_sampled_instance(&device.instance_name)
                    .ok_or_else(unsampled)?
                    .clone(),
            ));
            let sites = device.harmonic_balance_periodic_sites().len();
            if sites == 0 {
                samples.rows.push(Vec::new());
                samples.frames.push(None);
                continue;
            }
            let rows = solver
                .generated_carried_output_rows(&device.instance_name)
                .filter(|rows| rows.len() == sites)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic noise found no carried operator rows for generated Verilog-A \
                         instance '{}'",
                        device.instance_name
                    ))
                })?;
            samples
                .outputs
                .resize(samples.outputs.len().max(sites), 0.0);
            samples.rows.push(rows);
            samples
                .frames
                .push(Some(GeneratedPeriodicFrame::new(sites)));
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        let _ = (circuit, solver);
        Ok(samples)
    }

    /// The orbit sample [`Self::prepare`] last supplied.
    fn orbit(&self) -> GeneratedOrbitSample<'_> {
        GeneratedOrbitSample {
            #[cfg(feature = "veriloga-builtins-base")]
            instances: &self.instances,
            frames: &self.frames,
        }
    }

    /// Supply each frame its sites' outputs at `solution`, one sample.
    fn prepare(&mut self, solution: &[Value]) {
        for (rows, frame) in self.rows.iter().zip(&mut self.frames) {
            let Some(frame) = frame else {
                continue;
            };
            let outputs = &mut self.outputs[..rows.len()];
            for (output, &row) in outputs.iter_mut().zip(rows) {
                *output = solution[row];
            }
            frame.prepare(outputs, &[]);
        }
    }
}

impl Engine {
    /// Every runtime and generated Verilog-A noise process, sampled over the
    /// retained orbit. A generated model's noise is evaluated on harmonic
    /// balance's own sampled copy of the instance, at the orbit's value of
    /// every operator site harmonic balance carries as an unknown of its own,
    /// at each sample, as harmonic balance's sampler of the model is.
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
        let mut generated = GeneratedNoiseSamples::new(circuit, solver)?;
        let mut failure = None;
        let mut sample = |time: usize, count: usize, solution: &[Value]| {
            generated.prepare(solution);
            let orbit = generated.orbit();
            let processes = collect_noise_processes_on_orbit(circuit, solution, frequency, orbit)?;
            let shifted =
                collect_noise_processes_on_orbit(circuit, solution, 2.5 * frequency, orbit)?;
            for process in &processes {
                let Some(other) = shifted.iter().find(|other| other.key() == process.key()) else {
                    return Err(refused(
                        process,
                        "it is active at one frequency and not another",
                    ));
                };
                let Some((rows, exponent, power)) = sampled(process, other, num_nodes)? else {
                    continue;
                };
                let waveform =
                    waveforms
                        .entry(process.key())
                        .or_insert_with(|| NativeNoiseWaveform {
                            name: process.describe(),
                            nodes: rows.nodes,
                            further_rows: rows.further.clone(),
                            frequency_exponent: exponent,
                            samples: vec![ScaledNonnegative::ZERO; count],
                        });
                if !rows.reach_as(waveform) {
                    return Err(refused(
                        process,
                        "it moves between circuit rows, or changes its proportion across them, \
                         over the orbit",
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
                .filter(|waveform| {
                    waveform.nodes[0] != waveform.nodes[1] || !waveform.further_rows.is_empty()
                })
                .collect(),
            abort,
        )
    }
}

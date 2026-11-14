//! Periodic noise around an event orbit, in the time domain.
//!
//! PNOISE reads the linearized period PAC builds (see [`super::pac`]) through
//! its transpose.
//!
//! - **The adjoint.** For an output `y = Σₘ cₘᵀ·x(tₘ)` of the periodic
//!   response to an input whose period-to-period factor is `e^{jωT}`, the
//!   transposed recurrence run back from the end of the period,
//!   `λₖ = Aₖ⁻ᵀ·(Bₖ₊₁ᵀ·λₖ₊₁ + cₖ₊₁)`, gives `∂y/∂sₖ = λₖ`: how much of `y`
//!   a current injected over step `k` makes. Its closure is the transpose
//!   of PAC's: `(e^{jωT}I − Mᵀ)·ν = B₀ᵀ·λ₀`, with `ν` the term the period's
//!   last step receives from the next period's first, and `λ₀` that of a
//!   first sweep with `ν = 0`.
//! - **The sources.** Every device noise source is read at every accepted
//!   point at the orbit's own bias, so a switch's thermal noise follows its
//!   state through the period. Over step `k` a white source injects its
//!   average current, of variance `S/(2h)`; a source's share of the output is
//!   `T·Σₖ |λₖᵀ·e|²·Sₖ/hₖ`, with `Sₖ` weighted as the step weights its
//!   conductance (trapezoidal, or the step's end where a switch changes).
//!   By Parseval over the sidebands this is the noise every sideband
//!   `ω + mω₀` folds onto `ω`, all of them and not a window of them.
//! - **The output.** Sampled at a phase, `cₘ` reads the output at the sample
//!   (interpolated between the points around it), and the result is the
//!   one-sided spectrum of the sample sequence on `[0, f₀/2]`. Otherwise `cₘ`
//!   is the trapezoidal weight of `(1/T)∫x(t)·e^{−j(ω+kω₀)t}dt` for the output
//!   sideband `k`, and the result is the time-averaged spectrum at
//!   `ω + kω₀`.
//! - **Scope.** Thermal, shot and white sources, whose density does not
//!   depend on frequency. A source whose density does, a correlated pair, a
//!   source on a device's private noise nodes, a switch whose control
//!   carries noise where it changes (its crossing would move, and that
//!   impulse is not formed), and edge or delay sampling are refused by name.
//! - **Certification.** The sweep's endpoints and middle are recomputed on
//!   the bisected mesh and must agree within the run's relative voltage
//!   tolerance, applied to the noise amplitude.

use super::pac::{
    LinearPeriod, SMALL_SIGNAL_MESH_UNCERTIFIED, input_excitation, output_selection,
    probe_frequencies, reproduce_boundary, small_signal_refusal, solve_complex,
};
use super::{EVENT_CARRIER, MESH_REFINEMENTS, boundary};
use crate::abort_signal::AbortSignal;
use crate::analysis::noise::{NoiseInputQuantity, NoiseSourceType};
use crate::config::SpiceDialect;
use crate::engine::{
    Engine, PeriodicNoiseRequest, PeriodicNoiseSamplePoint, PeriodicNoiseSampling,
    PeriodicNoiseSamplingEvidence, PnoiseAnalysisResult, PssOperatingPoint, SimulationError,
};
use crate::identity::AnalysisKind;
use crate::{CircuitData, Complex64, Netlist, Value};
use std::f64::consts::TAU;

/// A switch's control is taken to carry no noise when its noise power at the
/// change is below this fraction of the output's.
const QUIET_CONTROL: Value = 1.0e-12;

fn refusal(token: &'static str, detail: String, instance: Option<&str>) -> SimulationError {
    small_signal_refusal(AnalysisKind::PNoise, token, detail, instance)
}

/// One device noise source at one accepted point: the one-based unknowns its
/// current enters and leaves (zero is ground) and its one-sided density.
#[derive(Clone, Copy, Debug)]
pub(super) struct PointNoise {
    pos: usize,
    neg: usize,
    density: Value,
}

/// What reads the device noise sources at every accepted point, and the
/// labels the first point fixed.
pub(super) struct NoiseCatalog {
    dialect: SpiceDialect,
    temperature: Value,
    labels: std::cell::RefCell<Option<Vec<String>>>,
}

impl NoiseCatalog {
    fn new(dialect: SpiceDialect, temperature: Value) -> Self {
        Self {
            dialect,
            temperature,
            labels: std::cell::RefCell::new(None),
        }
    }

    fn labels(&self) -> Vec<String> {
        self.labels.borrow().clone().unwrap_or_default()
    }

    /// Every source of `circuit`, prepared for small signal at `solution`.
    pub(super) fn collect(
        &self,
        circuit: &CircuitData,
        solution: &[Value],
    ) -> Result<Vec<PointNoise>, SimulationError> {
        let collected = Engine::try_collect_noise_sources(circuit, solution, self.dialect)?;
        if let Some(pair) = collected.correlated.first() {
            return Err(refusal(
                EVENT_CARRIER,
                format!(
                    "PNOISE on an event orbit folds independent white sources, and '{}' has a \
                     correlated noise pair",
                    pair.identity.device
                ),
                Some(&pair.identity.device),
            ));
        }
        let unknowns = circuit.matrix_size();
        let mut points = Vec::with_capacity(collected.elementary.len());
        let mut labels = Vec::with_capacity(collected.elementary.len());
        for (source, &absolute) in collected
            .elementary
            .iter()
            .zip(&collected.elementary_absolute_temperatures)
        {
            let device = source.identity.device.as_str();
            if source.node_pos > unknowns || source.node_neg > unknowns {
                return Err(refusal(
                    EVENT_CARRIER,
                    format!(
                        "PNOISE on an event orbit reads noise on the circuit's own unknowns, and \
                         '{device}' injects it on private noise nodes"
                    ),
                    Some(device),
                ));
            }
            let temperature = Engine::elementary_noise_temperature(self.temperature, absolute);
            let density = Engine::evaluated_noise_density(source, 1.0, temperature)?;
            let white = matches!(
                source.noise_type,
                NoiseSourceType::Thermal | NoiseSourceType::Shot | NoiseSourceType::White
            );
            if !white
                && (density != 0.0
                    || Engine::evaluated_noise_density(source, 1.0e6, temperature)? != 0.0)
            {
                return Err(refusal(
                    EVENT_CARRIER,
                    format!(
                        "PNOISE on an event orbit folds white sources, and the {:?} noise of \
                         '{device}' depends on frequency",
                        source.noise_type
                    ),
                    Some(device),
                ));
            }
            points.push(PointNoise {
                pos: source.node_pos,
                neg: source.node_neg,
                density: if white { density } else { 0.0 },
            });
            labels.push(Engine::noise_source_label(&source.identity));
        }
        let mut fixed = self.labels.borrow_mut();
        match fixed.as_ref() {
            None => *fixed = Some(labels),
            Some(first) if *first == labels => {}
            Some(first) => {
                return Err(SimulationError::Circuit(format!(
                    "PNOISE on the event orbit found its noise sources change along the period: \
                     {first:?} became {labels:?}"
                )));
            }
        }
        Ok(points)
    }
}

/// The analysis's output.
#[derive(Clone, Copy, Debug)]
enum Observation {
    /// The output at this absolute time, seconds into the period.
    Sample(Value),
    /// The output's sideband `k` of the time-averaged spectrum.
    Sideband(i32),
}

/// One offset's noise at the output.
struct OffsetNoise {
    /// Each source's share, in catalog order.
    shares: Vec<Value>,
    /// The output's response to a unit input at the input sideband, when one
    /// is named.
    gain: Option<Complex64>,
}

impl OffsetNoise {
    fn total(&self) -> Value {
        self.shares.iter().sum()
    }
}

impl LinearPeriod {
    /// The weight `y` puts on every accepted point's output, with the period's
    /// first point folded onto its last as the next period's.
    fn forcing(
        &self,
        omega: Value,
        fundamental: Value,
        observation: Observation,
    ) -> Vec<Complex64> {
        let points = self.phases.len();
        let rotation = Complex64::from_polar(1.0, omega * self.period);
        let mut weights = vec![Complex64::new(0.0, 0.0); points];
        match observation {
            Observation::Sample(time) => {
                let start = self.phases[0];
                let time = start + (time - start).rem_euclid(self.period);
                let index = self
                    .phases
                    .partition_point(|phase| *phase <= time)
                    .clamp(1, points - 1);
                let (t0, t1) = (self.phases[index - 1], self.phases[index]);
                let share = if t1 > t0 {
                    ((time - t0) / (t1 - t0)).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                weights[index - 1] += Complex64::new(1.0 - share, 0.0);
                weights[index] += Complex64::new(share, 0.0);
            }
            Observation::Sideband(k) => {
                let frequency = omega + Value::from(k) * fundamental;
                for index in 1..points {
                    let (t0, t1) = (self.phases[index - 1], self.phases[index]);
                    let half = 0.5 * (t1 - t0) / self.period;
                    weights[index - 1] += Complex64::from_polar(half, -frequency * t0);
                    weights[index] += Complex64::from_polar(half, -frequency * t1);
                }
            }
        }
        let first = std::mem::replace(&mut weights[0], Complex64::new(0.0, 0.0));
        weights[points - 1] += first / rotation;
        weights
    }

    /// One backward sweep of the transposed recurrence under `forcing`,
    /// starting from `closure` (the next period's term into the last step),
    /// offering every step's `λₖ` to `visit`. Returns `B₀ᵀ·λ₀`.
    fn sweep_back(
        &self,
        forcing: &[Complex64],
        output: (usize, Option<usize>),
        closure: Option<&[Complex64]>,
        mut visit: impl FnMut(usize, &[Complex64]),
    ) -> Vec<Complex64> {
        let n = self.n;
        let mut next = closure.map_or_else(|| vec![Complex64::new(0.0, 0.0); n], <[_]>::to_vec);
        let mut re = vec![0.0; n];
        let mut im = vec![0.0; n];
        for (k, step) in self.steps.iter().enumerate().rev() {
            let weight = forcing[k + 1];
            next[output.0] += weight;
            if let Some(reference) = output.1 {
                next[reference] -= weight;
            }
            for (row, value) in next.iter().enumerate() {
                re[row] = value.re;
                im[row] = value.im;
            }
            let (re_l, im_l) = (step.a.solve_transposed(&re), step.a.solve_transposed(&im));
            let lambda: Vec<Complex64> = re_l
                .iter()
                .zip(&im_l)
                .map(|(re, im)| Complex64::new(*re, *im))
                .collect();
            visit(k, &lambda);
            for (col, entry) in next.iter_mut().enumerate() {
                *entry = (0..n).map(|row| lambda[row] * step.b[row * n + col]).sum();
            }
        }
        next
    }

    /// The noise `observation` sees at offset `omega`, and the input's gain
    /// into it when `input` names the input sideband.
    fn noise(
        &self,
        omega: Value,
        fundamental: Value,
        observation: Observation,
        output: (usize, Option<usize>),
        input: Option<i32>,
    ) -> Result<OffsetNoise, SimulationError> {
        let n = self.n;
        let forcing = self.forcing(omega, fundamental, observation);
        let first = self.sweep_back(&forcing, output, None, |_, _| {});
        let rotation = Complex64::from_polar(1.0, omega * self.period);
        let mut system: Vec<Vec<Complex64>> = (0..n)
            .map(|row| {
                (0..n)
                    .map(|col| {
                        let identity = if row == col {
                            rotation
                        } else {
                            Complex64::new(0.0, 0.0)
                        };
                        identity - self.monodromy[col * n + row]
                    })
                    .collect()
            })
            .collect();
        let closure = solve_complex(&mut system, first).ok_or_else(|| {
            refusal(
                EVENT_CARRIER,
                format!(
                    "PNOISE on the event orbit is singular at {:.6e} Hz: e^(jωT) is a Floquet \
                     multiplier of the orbit",
                    omega / TAU
                ),
                None,
            )
        })?;
        let sources = self.noise.first().map_or(0, Vec::len);
        let mut shares = vec![0.0; sources];
        let mut gain = Complex64::new(0.0, 0.0);
        let input_frequency = input.map(|m| omega + Value::from(m) * fundamental);
        self.sweep_back(&forcing, output, Some(&closure), |k, lambda| {
            let h = self.phases[k + 1] - self.phases[k];
            let end = if self.steps[k].changes.is_empty() {
                0.5
            } else {
                1.0
            };
            let at = |node: usize| {
                if node == 0 {
                    Complex64::new(0.0, 0.0)
                } else {
                    lambda[node - 1]
                }
            };
            for (share, (before, after)) in shares
                .iter_mut()
                .zip(self.noise[k].iter().zip(&self.noise[k + 1]))
            {
                let density = (1.0 - end) * before.density + end * after.density;
                if density != 0.0 {
                    *share +=
                        self.period * (at(after.pos) - at(after.neg)).norm_sqr() * density / h;
                }
            }
            if let Some(frequency) = input_frequency {
                let u0 = Complex64::from_polar(1.0, frequency * self.phases[k]);
                let u1 = Complex64::from_polar(1.0, frequency * self.phases[k + 1]);
                let step = &self.steps[k];
                for (row, &entry) in self.excitation.iter().enumerate() {
                    if entry != 0.0 {
                        let theta = step.theta[row];
                        gain += lambda[row] * entry * (u1 * theta + u0 * (1.0 - theta));
                    }
                }
            }
        });
        Ok(OffsetNoise {
            shares,
            gain: input.map(|_| gain),
        })
    }

    /// Refuse an orbit with a switch whose control carries noise where it
    /// changes: its crossing moves with that noise, and the impulse that adds
    /// is not formed.
    fn refuse_noisy_crossings(
        &self,
        circuit: &CircuitData,
        omega: Value,
        fundamental: Value,
        output_noise: Value,
    ) -> Result<(), SimulationError> {
        for (index, step) in self.steps.iter().enumerate() {
            for &switch in &step.changes {
                let (name, positive, negative) =
                    if let Some(voltage) = circuit.vswitches.get(switch) {
                        (
                            voltage.name.as_str(),
                            voltage.ctrl_pos,
                            Some(voltage.ctrl_neg),
                        )
                    } else {
                        let current = &circuit.iswitches[switch - circuit.vswitches.len()];
                        let Some(branch) = current.ctrl_branch.filter(|branch| *branch > 0) else {
                            continue;
                        };
                        (current.name.as_str(), branch, None)
                    };
                let control = match (positive, negative) {
                    (0, Some(0) | None) => continue,
                    (0, Some(negative)) => (negative - 1, None),
                    (positive, Some(negative)) if negative > 0 => {
                        (positive - 1, Some(negative - 1))
                    }
                    (positive, _) => (positive - 1, None),
                };
                let time = self.phases[index + 1];
                let noise = self
                    .noise(omega, fundamental, Observation::Sample(time), control, None)?
                    .total();
                if noise > QUIET_CONTROL * output_noise {
                    return Err(refusal(
                        EVENT_CARRIER,
                        format!(
                            "PNOISE on the event orbit refuses switch '{name}': its control \
                             carries {noise:.3e} of noise power per hertz where it changes at \
                             {:.6e} s, so its crossing moves with the noise and the impulse \
                             that adds is not formed",
                            time.rem_euclid(self.period)
                        ),
                        Some(name),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Engine {
    /// PNOISE around an orbit event shooting solved.
    pub(in crate::engine) fn run_event_pnoise(
        &self,
        netlist: &Netlist,
        request: &PeriodicNoiseRequest<'_>,
        operating_point: &PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PnoiseAnalysisResult, SimulationError> {
        let pss_config = operating_point.config();
        operating_point.authenticate_event_orbit_for_reuse(netlist, &self.config, pss_config)?;
        let PeriodicNoiseRequest {
            sampling,
            offsets,
            output_node,
            output_ref,
            input_source,
            max_sideband: _,
            sidebands,
        } = *request;
        let fundamental_freq = operating_point.analysis().result.frequency;
        let fundamental = TAU * fundamental_freq;
        if offsets.is_empty() {
            return Err(SimulationError::Circuit(
                "pnoise frequency sweep is empty".to_string(),
            ));
        }
        if let Some((index, offset)) = offsets
            .iter()
            .copied()
            .enumerate()
            .find(|(_, offset)| !offset.is_finite() || *offset < 0.0)
        {
            return Err(SimulationError::Circuit(format!(
                "pnoise offset frequencies must be finite and non-negative, got offsets[{index}]={offset}"
            )));
        }
        let observation = match sampling {
            None => Observation::Sideband(sidebands.output),
            Some(request @ PeriodicNoiseSampling::Phase { phase_degrees }) => {
                request.validate().map_err(SimulationError::Circuit)?;
                let highest = offsets.iter().copied().fold(0.0, Value::max);
                if sidebands.output != 0
                    || crate::analysis::pnoise::offset_exceeds_sampled_band(
                        highest,
                        fundamental_freq,
                    )
                {
                    return Err(SimulationError::Circuit(format!(
                        "sampled PNOISE requires output sideband zero and offsets no higher than \
                         half the carrier frequency (output sideband {}, highest offset \
                         {highest:.15e} Hz, half the carrier {:.15e} Hz)",
                        sidebands.output,
                        fundamental_freq * 0.5
                    )));
                }
                Observation::Sample(phase_degrees.rem_euclid(360.0) / 360.0 * pss_config.period())
            }
            Some(_) => {
                return Err(refusal(
                    EVENT_CARRIER,
                    "PNOISE on an event orbit samples its output at a phase; a crossing's timing \
                     (edge or delay sampling) is not formed there"
                        .to_string(),
                    None,
                ));
            }
        };
        let prepared = self.prepare_event_orbit(netlist, pss_config, None, abort)?;
        if prepared.basis.names() != operating_point.shooting_state_basis() {
            return Err(refusal(
                EVENT_CARRIER,
                format!(
                    "PNOISE cannot reproduce the event orbit: its coordinates at the reproduced \
                     boundary are {:?}, the retained orbit's {:?}",
                    prepared.basis.names(),
                    operating_point.shooting_state_basis()
                ),
                None,
            ));
        }
        let circuit = &prepared.image.circuit;
        #[cfg(feature = "veriloga")]
        if let Some(device) = circuit.veriloga_devices().iter().next() {
            return Err(refusal(
                EVENT_CARRIER,
                format!(
                    "PNOISE on an event orbit linearizes switches and the native devices between \
                     their changes; the operator of Verilog-A instance '{}' around its events is \
                     not formed",
                    device.name
                ),
                Some(&device.name.to_string()),
            ));
        }
        let node_names = circuit.node_names_sorted();
        let output = output_selection(
            Some(output_node),
            output_ref,
            netlist,
            &node_names,
            "PNOISE",
        )?
        .ok_or_else(|| SimulationError::Circuit("PNOISE requires an output node".into()))?;
        let excitation = match input_source {
            Some(name) => input_excitation(circuit, name, "PNOISE")?,
            None => vec![0.0; circuit.matrix_size()],
        };
        let input_quantity = input_source.map(|name| {
            if circuit
                .voltage_sources
                .names
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(name.trim()))
            {
                NoiseInputQuantity::Voltage
            } else {
                NoiseInputQuantity::Current
            }
        });
        let input = input_source.map(|_| sidebands.input);
        let structure = self.build_matrix(circuit)?;
        let orbit = prepared.orbit(netlist, pss_config, abort);
        let z = operating_point.shooting_state();
        let (start, mesh) = reproduce_boundary(
            &orbit,
            &prepared.image,
            z,
            &prepared.mesh,
            AnalysisKind::PNoise,
        )?;
        let reltol = prepared.mesh_tolerance.0;
        let catalog = NoiseCatalog::new(self.config.spice_dialect, self.config.temperature);
        let total = |period: &LinearPeriod, offset: Value| -> Result<Value, SimulationError> {
            Ok(period
                .noise(TAU * offset, fundamental, observation, output, None)?
                .total())
        };
        let probes = probe_frequencies(offsets);
        let mut coarse_mesh = boundary::bisect(&mesh, start.time);
        let mut coarse = orbit.linear_period(
            &start,
            z,
            &coarse_mesh,
            &structure,
            &excitation,
            Some(&catalog),
        )?;
        let mut refinements = 1;
        let fine = loop {
            refinements += 1;
            let fine_mesh = boundary::bisect(&coarse_mesh, start.time);
            let fine = orbit.linear_period(
                &start,
                z,
                &fine_mesh,
                &structure,
                &excitation,
                Some(&catalog),
            )?;
            let mut worst = (0.0, probes[0]);
            for &offset in &probes {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let (a, b) = (total(&coarse, offset)?.sqrt(), total(&fine, offset)?.sqrt());
                let value = (a - b).abs() / (reltol * b).max(Value::MIN_POSITIVE);
                if value > worst.0 {
                    worst = (value, offset);
                }
            }
            if worst.0 <= 1.0 {
                break fine;
            }
            if refinements > MESH_REFINEMENTS {
                return Err(refusal(
                    SMALL_SIGNAL_MESH_UNCERTIFIED,
                    format!(
                        "PNOISE on the event orbit cannot certify its mesh: after {refinements} \
                         bisections its noise at {:.6e} Hz still moves by {:.3e} times its \
                         tolerance",
                        worst.1, worst.0
                    ),
                    None,
                ));
            }
            coarse = fine;
            coarse_mesh = fine_mesh;
        };
        fine.refuse_noisy_crossings(
            circuit,
            TAU * offsets[0],
            fundamental,
            total(&fine, offsets[0])?,
        )?;
        let labels = catalog.labels();
        let mut contributors: Vec<(String, Vec<Value>)> = Vec::new();
        let mut slot_of = Vec::with_capacity(labels.len());
        for label in &labels {
            let slot = match contributors.iter().position(|(name, _)| name == label) {
                Some(slot) => slot,
                None => {
                    contributors.push((label.clone(), Vec::with_capacity(offsets.len())));
                    contributors.len() - 1
                }
            };
            slot_of.push(slot);
        }
        let mut output_noise = Vec::with_capacity(offsets.len());
        let mut input_noise = input.map(|_| Vec::with_capacity(offsets.len()));
        for &offset in offsets {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let noise = fine.noise(TAU * offset, fundamental, observation, output, input)?;
            let total = noise.total();
            if !total.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "PNOISE on the event orbit has a non-finite output noise at {offset:.6e} Hz"
                )));
            }
            let mut merged = vec![0.0; contributors.len()];
            for (&slot, share) in slot_of.iter().zip(&noise.shares) {
                merged[slot] += share;
            }
            for ((_, values), value) in contributors.iter_mut().zip(merged) {
                values.push(value);
            }
            output_noise.push(total);
            if let (Some(values), Some(gain)) = (input_noise.as_mut(), noise.gain) {
                let referred = total / gain.norm_sqr();
                if !referred.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "PNOISE on the event orbit cannot refer its noise to the input at \
                         {offset:.6e} Hz: the input's gain into the output is {gain}"
                    )));
                }
                values.push(referred);
            }
        }
        let sampling = match (sampling, observation) {
            (Some(request), Observation::Sample(time)) => Some(sample_evidence(
                request,
                operating_point,
                &node_names,
                output,
                time,
            )?),
            _ => None,
        };
        Ok(PnoiseAnalysisResult {
            sampling,
            input_quantity,
            sidebands,
            frequencies: offsets.to_vec(),
            output_noise,
            contributors,
            input_noise,
            fundamental_freq,
            converged: true,
            integrated_output_noise: None,
            integrated_input_noise: None,
        })
    }
}

/// The sample the orbit holds at the sampling phase, and its slew.
fn sample_evidence(
    request: &PeriodicNoiseSampling,
    operating_point: &PssOperatingPoint,
    node_names: &[String],
    output: (usize, Option<usize>),
    time: Value,
) -> Result<PeriodicNoiseSamplingEvidence, SimulationError> {
    let result = &operating_point.analysis().result;
    let trace = |unknown: usize| -> Result<&[Value], SimulationError> {
        let name = &node_names[unknown];
        result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .map(|index| result.waveforms[index].values.as_slice())
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "sampled PNOISE cannot read node {name} on the event orbit"
                ))
            })
    };
    let index = result
        .time
        .partition_point(|t| *t <= time)
        .clamp(1, result.time.len().saturating_sub(1).max(1));
    let (t0, t1) = (result.time[index - 1], result.time[index]);
    let read = |values: &[Value]| {
        let (v0, v1) = (values[index - 1], values[index]);
        let slope = if t1 > t0 { (v1 - v0) / (t1 - t0) } else { 0.0 };
        (v0 + slope * (time - t0), slope)
    };
    let (mut voltage, mut slew) = read(trace(output.0)?);
    if let Some(reference) = output.1 {
        let (v, s) = read(trace(reference)?);
        voltage -= v;
        slew -= s;
    }
    let evidence = PeriodicNoiseSamplingEvidence {
        carrier_frequency_hz: result.frequency,
        request: request.clone(),
        output: PeriodicNoiseSamplePoint {
            node: node_names[output.0].clone(),
            reference: output.1.map(|reference| node_names[reference].clone()),
            phase_degrees: match request {
                PeriodicNoiseSampling::Phase { phase_degrees } => phase_degrees.rem_euclid(360.0),
                _ => (time / result.period * 360.0).rem_euclid(360.0),
            },
            voltage,
            slew_volts_per_second: slew,
        },
        reference: None,
        nominal_delay_seconds: None,
    };
    evidence.validate().map_err(SimulationError::Circuit)?;
    Ok(evidence)
}

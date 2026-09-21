//! Complete physical noise catalog on an authenticated independent-phase orbit.
use super::*;
use crate::ResourceLimits;
use crate::analysis::harmonic_balance::{ScaledNonnegative, normalize_scaled_noise_waveform};
use crate::analysis::quasi_periodic::{
    QuasiPeriodicNoiseSource as Source, QuasiPeriodicNoiseSpectrum as Spectrum,
    QuasiPeriodicTransform,
};
use crate::engine::hb::pnoise::{
    NativeNoiseWaveforms, checked_scaled_positive_product, pnoise_physical_constants,
};

fn error(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("QPNOISE: {}", message.into()))
}
fn core_error(error: SimulationError) -> QuasiPeriodicError {
    match error {
        SimulationError::Aborted => QuasiPeriodicError::Aborted,
        SimulationError::ResourceLimit(limit) => QuasiPeriodicError::ResourceLimit(limit),
        other => QuasiPeriodicError::InvalidCircuit(other.to_string()),
    }
}
fn direction(positive: usize, negative: usize, nodes: usize) -> Vec<(usize, Complex64)> {
    if positive == negative {
        return vec![];
    }
    let mut values = Vec::new();
    if positive < nodes {
        values.push((positive, Complex64::ONE));
    }
    if negative < nodes {
        values.push((negative, -Complex64::ONE));
    }
    values
}
fn white(name: String, injections: Vec<(usize, Complex64)>, density: ScaledNonnegative) -> Source {
    Source {
        name,
        injections,
        spectrum: Spectrum::White {
            density: vec![density.mantissa],
            binary_scale_exponent: density.exponent,
        },
    }
}
fn add_scale(density: ScaledNonnegative, scale: i32) -> Result<ScaledNonnegative, SimulationError> {
    Ok(ScaledNonnegative {
        mantissa: density.mantissa,
        exponent: density
            .exponent
            .checked_add(scale)
            .ok_or_else(|| error("noise coefficient exponent exceeds signed range"))?,
    })
}
#[derive(Default)]
struct Catalog {
    sources: Vec<Source>,
    values: usize,
}
impl Catalog {
    fn check(&self, engine: &Engine, extra: usize) -> Result<(), SimulationError> {
        engine.ensure_result_values(self.values.saturating_add(extra))
    }
    fn remaining(&self, engine: &Engine) -> ResourceLimits {
        let mut limits = engine.config.resource_limits.clone();
        limits.max_result_values = limits
            .max_result_values
            .min(32_000_000)
            .saturating_sub(self.values);
        limits
    }
    fn push(&mut self, engine: &Engine, source: Source) -> Result<(), SimulationError> {
        if source.injections.is_empty() {
            return Ok(());
        }
        let spectrum = match &source.spectrum {
            Spectrum::White { density, .. } => density.len(),
            Spectrum::PowerLaw {
                modulation,
                modulation_lattices,
                ..
            } => modulation.len().saturating_mul(2).saturating_add(
                modulation_lattices.as_ref().map_or(0, |tuples| {
                    tuples.iter().fold(0usize, |s, t| s.saturating_add(t.len()))
                }),
            ),
        };
        let values = spectrum
            .saturating_add(source.injections.len().saturating_mul(3))
            .saturating_add(8);
        self.check(engine, values)?;
        self.values = self.values.saturating_add(values);
        self.sources.push(source);
        Ok(())
    }
}

impl Engine {
    /// Physical source evidence for the exact retained QPSS producer. This
    /// authenticates the orbit; numerical projection still requires QPNOISE's
    /// adjoint solve, and this catalog alone is not an output noise result.
    pub fn qpss_noise_sources_with_abort(
        &self,
        netlist: &Netlist,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Source>, SimulationError> {
        check_abort(abort)?;
        let engine = self.resolved_for_netlist(netlist);
        let producer = state::Producer::capture(netlist, &engine.config, point.config())?;
        let grid = engine.validate_qpss_operating_point_with_abort(netlist, point, abort)?;
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::ensure_no_mixed_signal_analysis(&circuit, "QPNOISE")?;
        let mut solver = engine.qpss_circuit_solver(&circuit, &grid)?;
        let sources = engine.prepare_quasi_periodic_noise_sources(
            &circuit,
            &mut solver,
            point,
            grid,
            abort,
        )?;
        if producer != state::Producer::capture(netlist, &engine.config, point.config())? {
            return Err(error("noise producer changed during sampling"));
        }
        check_abort(abort)?;
        Ok(sources)
    }

    pub(super) fn prepare_quasi_periodic_noise_sources(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        point: &QpssOperatingPoint,
        grid: Arc<QuasiPeriodicGrid>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Source>, SimulationError> {
        self.validate_cyclostationary_noise_circuit(circuit, "qpnoise")?;
        let mut catalog = Catalog::default();
        self.quasi_periodic_resistor_noise(circuit, solver, point, &grid, &mut catalog, abort)?;
        let compact = solver
            .quasi_periodic_device_noise_sources_with_abort(
                grid.clone(),
                point.complete_spectra(),
                self.config.temperature,
                pnoise_physical_constants(self.config.spice_dialect),
                &catalog.remaining(self),
                abort,
            )
            .map_err(numerical_error)?;
        for source in compact {
            check_abort(abort)?;
            catalog.push(self, source)?;
        }
        self.quasi_periodic_native_noise(solver, point, grid, &mut catalog, abort)?;
        Ok(catalog.sources)
    }

    fn quasi_periodic_resistor_noise(
        &self,
        circuit: &CircuitData,
        solver: &HbSolver,
        point: &QpssOperatingPoint,
        grid: &QuasiPeriodicGrid,
        catalog: &mut Catalog,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let nodes = circuit.num_nodes();
        let ambient = self.config.temperature;
        let kb = pnoise_physical_constants(self.config.spice_dialect).boltzmann;
        let shunt = circuit.global_shunt_conductance();
        if !shunt.is_finite() || shunt < 0.0 {
            return Err(error("RSHUNT conductance must be finite and nonnegative"));
        }
        if shunt > 0.0 {
            let q = checked_scaled_positive_product(&[4.0, kb, ambient, shunt], "QPNOISE RSHUNT")?;
            for node in 0..nodes {
                check_abort(abort)?;
                if !circuit.has_global_shunt_at(node) {
                    continue;
                }
                let name = point
                    .node_names()
                    .get(node)
                    .ok_or_else(|| error("RSHUNT node table differs from retained QPSS"))?;
                catalog.push(
                    self,
                    white(
                        format!("RSHUNT:{name} thermal"),
                        vec![(node, Complex64::ONE)],
                        q,
                    ),
                )?;
            }
        }
        for i in 0..circuit.resistors.len() {
            check_abort(abort)?;
            if !circuit.resistors.noisy[i] {
                continue;
            }
            let name = &circuit.resistors.names[i];
            let g = circuit.resistors.small_signal_conductance(i);
            if !g.is_finite() || g < 0.0 {
                return Err(error(format!(
                    "resistor '{name}' has invalid noise conductance"
                )));
            }
            if g == 0.0 {
                continue;
            }
            let temperature = circuit.resistor_noise_temperature(i, ambient);
            if !temperature.is_finite() || temperature <= 0.0 {
                return Err(error(format!(
                    "resistor '{name}' noise temperature must be finite and positive"
                )));
            }
            let pos = Self::hb_node_to_solver_index(circuit.resistors.stamps[i].pp.row, nodes);
            let neg = Self::hb_node_to_solver_index(circuit.resistors.stamps[i].nn.row, nodes);
            let injections = direction(pos, neg, nodes);
            let q = checked_scaled_positive_product(&[4.0, kb, temperature, g], name)?;
            catalog.push(
                self,
                white(format!("{name} thermal"), injections.clone(), q),
            )?;
            if let Some(crate::circuit::ResistorFlickerNoise {
                coefficient,
                binary_scale,
                af: 2.0,
                ef,
            }) = circuit.resistors.flicker[i]
                && coefficient != 0.0
            {
                catalog.check(self, grid.len().saturating_mul(2).saturating_add(14))?;
                let dcg = circuit.resistors.conductances[i];
                let q = add_scale(
                    checked_scaled_positive_product(&[coefficient, dcg, dcg], name)?,
                    binary_scale,
                )?;
                let mut modulation = Vec::with_capacity(grid.len());
                for k in 0..grid.len() {
                    if k.is_multiple_of(256) {
                        check_abort(abort)?;
                    }
                    let v = |row: usize| {
                        if row >= nodes {
                            Complex64::ZERO
                        } else {
                            point.complete_spectra()[row][k]
                        }
                    };
                    let value = v(pos) - v(neg);
                    if !value.re.is_finite() || !value.im.is_finite() {
                        return Err(error("resistor modulation overflowed"));
                    }
                    modulation.push(value);
                }
                catalog.push(
                    self,
                    Source {
                        name: format!("{name} flicker"),
                        injections,
                        spectrum: Spectrum::PowerLaw {
                            coefficient: q.mantissa,
                            binary_scale_exponent: q.exponent,
                            exponent: ef,
                            modulation,
                            modulation_lattices: None,
                        },
                    },
                )?;
            }
        }
        let branches = solver
            .try_periodic_mna_branch_names()
            .map_err(|e| error(e.to_string()))?;
        for i in 0..circuit.resistor_branches.len() {
            check_abort(abort)?;
            if !circuit.resistor_branches.noisy[i] {
                continue;
            }
            let name = &circuit.resistor_branches.names[i];
            let r = circuit.resistor_branches.small_signal_resistances[i];
            if !r.is_finite() || r < 0.0 {
                return Err(error(format!(
                    "resistor '{name}' has invalid noise resistance"
                )));
            }
            if r == 0.0 {
                continue;
            }
            let temperature = circuit.resistor_branches.noise_temperature(i, ambient);
            if !temperature.is_finite() || temperature <= 0.0 {
                return Err(error(format!(
                    "resistor '{name}' noise temperature must be finite and positive"
                )));
            }
            let branch = circuit.resistor_branches.branch_indices[i]
                .checked_sub(1)
                .ok_or_else(|| error("invalid resistor branch ordinal"))?;
            if branches.get(branch) != Some(name) || point.branch_names().get(branch) != Some(name)
            {
                return Err(error("resistor branch differs from retained exact MNA"));
            }
            let injections = vec![(nodes + branch, Complex64::ONE)];
            // Series Thevenin noise: the resistor's own branch current includes
            // its intrinsic fluctuation, which a Norton-only probe would miss.
            let q = checked_scaled_positive_product(&[4.0, kb, temperature, r], name)?;
            catalog.push(
                self,
                white(format!("{name} thermal"), injections.clone(), q),
            )?;
            if let Some(crate::circuit::ResistorFlickerNoise {
                coefficient,
                binary_scale,
                af: 2.0,
                ef,
            }) = circuit.resistor_branches.flicker[i]
                && coefficient != 0.0
            {
                catalog.check(self, grid.len().saturating_mul(2).saturating_add(11))?;
                let q = add_scale(
                    checked_scaled_positive_product(&[coefficient, r, r], name)?,
                    binary_scale,
                )?;
                let modulation = point.complete_spectra()[nodes + branch].clone();
                catalog.push(
                    self,
                    Source {
                        name: format!("{name} flicker"),
                        injections,
                        spectrum: Spectrum::PowerLaw {
                            coefficient: q.mantissa,
                            binary_scale_exponent: q.exponent,
                            exponent: ef,
                            modulation,
                            modulation_lattices: None,
                        },
                    },
                )?;
            }
        }
        Ok(())
    }

    fn quasi_periodic_native_noise(
        &self,
        solver: &mut HbSolver,
        point: &QpssOperatingPoint,
        grid: Arc<QuasiPeriodicGrid>,
        catalog: &mut Catalog,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let mut frames = None;
        solver
            .visit_quasi_periodic_native_bjt_samples_with_abort(
                grid.clone(),
                point.complete_spectra(),
                &catalog.remaining(self),
                abort,
                |phase, count, bjts, solution, remaining| {
                    let frames = frames.get_or_insert_with(|| {
                        NativeNoiseWaveforms::new(
                            self.config.temperature,
                            remaining.max_result_values,
                        )
                    });
                    frames
                        .sample(self, phase, count, bjts, solution, abort)
                        .map_err(core_error)
                },
            )
            .map_err(numerical_error)?;
        let Some(frames) = frames else {
            return Ok(());
        };
        let mut frame_values = frames.waveforms.iter().fold(0usize, |n, w| {
            n.saturating_add(w.samples.len().saturating_mul(2))
        });
        catalog.check(
            self,
            frame_values.saturating_add(grid.sample_count().saturating_mul(8)),
        )?;
        let mut transform =
            QuasiPeriodicTransform::new_with_abort(grid.clone(), abort).map_err(numerical_error)?;
        for mut frame in frames.waveforms {
            check_abort(abort)?;
            let own = frame.samples.len().saturating_mul(2);
            frame_values = frame_values.saturating_sub(own);
            let injections = direction(
                frame.nodes[0].checked_sub(1).unwrap_or(usize::MAX),
                frame.nodes[1].checked_sub(1).unwrap_or(usize::MAX),
                point.node_names().len(),
            );
            if injections.is_empty() {
                continue;
            }
            let mut nonzero = false;
            for (i, sample) in frame.samples.iter_mut().enumerate() {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                nonzero |= sample.mantissa != 0.0;
                if frame.frequency_exponent.is_some() {
                    sample.mantissa = (sample.mantissa
                        * if sample.exponent.rem_euclid(2) == 0 {
                            1.0
                        } else {
                            2.0
                        })
                    .sqrt();
                    sample.exponent = sample.exponent.div_euclid(2);
                }
            }
            if !nonzero {
                continue;
            }
            let extra = frame_values
                .saturating_add(own)
                .saturating_add(frame.samples.len())
                .saturating_add(grid.sample_count().saturating_mul(8));
            catalog.check(self, extra)?;
            let (samples, exponent) = normalize_scaled_noise_waveform(&frame.samples)
                .map_err(|e| error(format!("{}: {e}", frame.name)))?;
            check_abort(abort)?;
            let spectrum = if let Some(power) = frame.frequency_exponent {
                let mut limits = catalog.remaining(self);
                limits.max_result_values = limits.max_result_values.saturating_sub(extra);
                let full = transform
                    .to_complete_real_spectrum_with_abort(&samples, &limits, abort)
                    .map_err(numerical_error)?;
                Spectrum::PowerLaw {
                    coefficient: 1.0,
                    exponent: power,
                    binary_scale_exponent: exponent
                        .checked_mul(2)
                        .ok_or_else(|| error("native amplitude exponent exceeds signed range"))?,
                    modulation: full.coefficients,
                    modulation_lattices: Some(full.tuples),
                }
            } else {
                Spectrum::White {
                    density: samples,
                    binary_scale_exponent: exponent,
                }
            };
            catalog.push(
                self,
                Source {
                    name: frame.name,
                    injections,
                    spectrum,
                },
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;

//! Zero-origin primitives on the signed independent-tone lattice.
use super::*;
use crate::analysis::harmonic_balance::solver::integrals::forced::coordinate_group;
use crate::analysis::quasi_periodic::{QuasiPeriodicSampleSpectrum, QuasiPeriodicTransform};
use crate::{ResourceKind, ResourceLimitError};

fn check_abort(abort: &dyn AbortSignal) -> Result<(), Error> {
    if abort.is_aborted() {
        Err(Error::Aborted)
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub(in crate::analysis::harmonic_balance::solver) struct QuasiPrescribedIntegrals {
    grid: Arc<QuasiPeriodicGrid>,
    // Some(empty) marks a prescribed coordinate in retained mode.
    primitives: Vec<Option<Primitive>>,
    retained: bool,
    scale: Value,
}

#[derive(Debug)]
struct Primitive {
    spectrum: QuasiPeriodicSampleSpectrum,
    samples: Vec<Value>,
}

fn primitive_value(spectrum: &QuasiPeriodicSampleSpectrum, phases: &[Value]) -> Value {
    // Real conjugate pairs, anchored at theta=0. Writing cos(theta)-1 also
    // preserves the exact zero origin when large terms would otherwise cancel.
    spectrum
        .tuples
        .iter()
        .zip(&spectrum.coefficients)
        .take(spectrum.coefficients.len() / 2)
        .map(|(tuple, coefficient)| {
            let phase = tuple
                .iter()
                .zip(phases)
                .map(|(n, phase)| *n as Value * phase)
                .sum::<Value>();
            let (sin, cos) = phase.sin_cos();
            2.0 * (coefficient.re * (cos - 1.0) - coefficient.im * sin)
        })
        .sum()
}

impl QuasiPrescribedIntegrals {
    pub(in crate::analysis::harmonic_balance::solver) fn known_input(&self, index: usize) -> bool {
        !self.retained && self.primitives.get(index).is_some_and(Option::is_some)
    }

    /// The circuit coordinate contains retained modes, even when the complete
    /// primitive needed omitted modes to establish its zero-origin constant.
    pub(in crate::analysis::harmonic_balance::solver) fn project_input(
        &self,
        index: usize,
        target: &mut [Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        if !self.known_input(index) || target.len() != self.grid.len() {
            return Err(Error::InvalidCircuit(
                "prescribed input differs from the producer basis".into(),
            ));
        }
        target.fill(Complex64::ZERO);
        let primitive = self.primitives[index].as_ref().expect("qualified input");
        for (i, (tuple, &value)) in primitive
            .spectrum
            .tuples
            .iter()
            .zip(&primitive.spectrum.coefficients)
            .enumerate()
        {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if let Some(slot) = self.grid.index_of(tuple) {
                target[slot] = value;
            }
        }
        Ok(())
    }

    pub(in crate::analysis::harmonic_balance::solver) fn sample(
        &self,
        phases: &[Value],
        integrals: &[Value],
    ) -> Vec<Option<(Value, Value)>> {
        let mut sample = Some(0);
        let mut stride = 1;
        for (&phase, &size) in phases.iter().zip(self.grid.dimensions()) {
            let index = (phase * size as Value / std::f64::consts::TAU).round() as usize;
            if index >= size || phase != std::f64::consts::TAU * index as Value / size as Value {
                sample = None;
                break;
            }
            sample = sample.map(|sample| sample + index * stride);
            stride *= size;
        }
        self.primitives
            .iter()
            .enumerate()
            .map(|(i, primitive)| {
                primitive.as_ref().map(|primitive| {
                    (
                        if self.retained {
                            integrals[i]
                        } else {
                            sample.map_or_else(
                                || primitive_value(&primitive.spectrum, phases),
                                |i| primitive.samples[i],
                            )
                        },
                        self.scale,
                    )
                })
            })
            .collect()
    }
}

impl HbSolver {
    pub(super) fn prepare_quasi_periodic_carrier(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        config: &QuasiPeriodicSolveConfig,
        sources: &[Vec<Complex64>],
        seed: Option<&[Vec<Complex64>]>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(Option<Vec<Vec<Complex64>>>, ResourceLimits, usize), Error> {
        let (mut remaining, mut needed) = self.prepare_quasi_periodic_integrals_with_inputs(
            grid.clone(),
            limits,
            false,
            Some(sources),
            None,
            abort,
        )?;
        let n = self.unknowns();
        let seed_values = n.saturating_mul(grid.len().saturating_mul(2).saturating_add(4));
        let input_values = |inputs: &[Option<Vec<Complex64>>]| {
            inputs.len().saturating_mul(4).saturating_add(
                inputs
                    .iter()
                    .flatten()
                    .map(|row| row.capacity().saturating_mul(2))
                    .sum::<usize>(),
            )
        };
        let mut inputs: Vec<Option<Vec<Complex64>>> = Vec::new();
        let mut prepared: Option<Vec<Vec<Complex64>>> = None;
        let mut iterations = 0;
        while needed.iter().any(|&needed| needed) && iterations < config.max_iterations {
            check_abort(abort)?;
            let initially_prescribed = self
                .quasi_prescribed_integrals
                .as_ref()
                .map_or(0, |cache| cache.primitives.iter().flatten().count());
            let retained_seed = if prepared.is_some() { seed_values } else { 0 };
            let mut preparing = remaining.clone();
            preparing.max_result_values = preparing
                .max_result_values
                .saturating_sub(input_values(&inputs))
                .saturating_sub(retained_seed)
                .saturating_sub(needed.len());
            let mut bounded = config.clone();
            bounded.max_iterations -= iterations;
            let driven = self
                .nonlinear_driven_spectra(
                    grid.clone(),
                    &bounded,
                    sources,
                    prepared.as_deref().or(seed),
                    &needed,
                    &preparing,
                    abort,
                )
                .map_err(|error| match error {
                    Error::ConvergenceFailed {
                        iterations: used,
                        merit,
                    } => Error::ConvergenceFailed {
                        iterations: iterations + used,
                        merit,
                    },
                    other => other,
                })?;
            // Release the flags before the next preparation allocates its set.
            drop(std::mem::take(&mut needed));
            iterations += driven.iterations;
            if driven.spectra.iter().all(Option::is_none) {
                break;
            }
            if inputs.is_empty() {
                inputs = driven.spectra;
            } else {
                for (target, value) in inputs.iter_mut().zip(driven.spectra) {
                    if value.is_some() {
                        *target = value;
                    }
                }
            }
            let driven_values = input_values(&inputs);
            let mut preparing = limits.clone();
            preparing.max_result_values = preparing
                .max_result_values
                .saturating_sub(driven_values)
                .saturating_sub(retained_seed);
            let (available, unresolved) = self.prepare_quasi_periodic_integrals_with_inputs(
                grid.clone(),
                &preparing,
                false,
                Some(sources),
                Some(&inputs),
                abort,
            )?;
            let primitive_values = preparing
                .max_result_values
                .saturating_sub(available.max_result_values);
            remaining = limits.clone();
            remaining.max_result_values =
                remaining.max_result_values.saturating_sub(primitive_values);
            needed = unresolved;
            let now_prescribed = self
                .quasi_prescribed_integrals
                .as_ref()
                .map_or(0, |cache| cache.primitives.iter().flatten().count());
            if now_prescribed <= initially_prescribed {
                // Solving one input does not qualify a feedback integral whose
                // constant still belongs to the joint solve. Preserve its seed.
                break;
            }
            ResourceLimitError::ensure(
                ResourceKind::ResultValues,
                seed_values
                    .saturating_add(driven_values)
                    .saturating_add(needed.len()),
                remaining.max_result_values,
            )?;
            let prepared = prepared.get_or_insert_with(|| {
                seed.map_or_else(|| vec![vec![Complex64::ZERO; grid.len()]; n], <[_]>::to_vec)
            });
            for (target, values) in prepared.iter_mut().zip(&inputs) {
                check_abort(abort)?;
                if let Some(values) = values {
                    target.clone_from(values);
                }
            }
            if let Some(cache) = &self.quasi_prescribed_integrals {
                let physical = self.num_nodes + self.physical_branch_count();
                for (index, target) in prepared[physical..].iter_mut().enumerate() {
                    if cache.known_input(index) {
                        cache.project_input(index, target, abort)?;
                    }
                }
            }
            // Every continuing pass has qualified another primitive, so both
            // dependency depth and the shared Newton budget bound this loop.
        }
        if prepared.is_some() {
            remaining.max_result_values = remaining.max_result_values.saturating_sub(seed_values);
        }
        Ok((prepared, remaining, iterations))
    }

    pub(super) fn prepare_quasi_periodic_integrals(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        limits: &ResourceLimits,
        retained: bool,
        sources: Option<&[Vec<Complex64>]>,
        driven: Option<&[Option<Vec<Complex64>>]>,
        abort: &dyn AbortSignal,
    ) -> Result<ResourceLimits, Error> {
        self.prepare_quasi_periodic_integrals_with_inputs(
            grid, limits, retained, sources, driven, abort,
        )
        .map(|(limits, _)| limits)
    }

    fn prepare_quasi_periodic_integrals_with_inputs(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        limits: &ResourceLimits,
        retained: bool,
        sources: Option<&[Vec<Complex64>]>,
        driven: Option<&[Option<Vec<Complex64>>]>,
        abort: &dyn AbortSignal,
    ) -> Result<(ResourceLimits, Vec<bool>), Error> {
        check_abort(abort)?;
        // Reusing this registry for a producer after a consumer must not keep
        // the consumer's fixed-perturbation mode or a previous tone basis.
        self.quasi_prescribed_integrals = None;
        let mut plans = self
            .behavioral_sources
            .prescribed_integral_rates()
            .map_err(Error::InvalidCircuit)?;
        let capacitor_expressions = self
            .periodic_capacitors
            .iter()
            .filter(|cap| cap.expression.program.sdt_count != 0)
            .map(|cap| cap.expression.clone())
            .collect::<Vec<_>>();
        for expression in &capacitor_expressions {
            expression
                .append_prescribed_integral_rates(&mut plans)
                .map_err(Error::InvalidCircuit)?;
        }
        if plans.is_empty() {
            return Ok((limits.clone(), Vec::new()));
        }
        let budget = |values| {
            ResourceLimitError::ensure(
                ResourceKind::ResultValues,
                values,
                limits.max_result_values.min(32_000_000),
            )
            .map_err(Error::from)
        };
        let input_values = if retained { 0 } else { self.unknowns() };
        budget(plans.len().saturating_add(input_values))?;
        let mut unresolved = vec![false; input_values];
        let mut primitives: Vec<Option<Primitive>> = Vec::with_capacity(plans.len());
        let mut values = plans.len();
        let mut transform = None;
        let mut forced: Vec<Option<Vec<Value>>> = Vec::new();
        let mut groups = Vec::new();
        let mut forced_values = input_values;
        if !retained && plans.iter().any(|plan| plan.dependencies().is_none()) {
            let sources = sources.ok_or_else(|| {
                Error::InvalidConfig(
                    "driven integral preparation requires complete source spectra".into(),
                )
            })?;
            if sources.len() != self.unknowns() || sources.iter().any(|row| row.len() != grid.len())
            {
                return Err(Error::InvalidConfig(
                    "driven integral source spectra differ from the MNA tone basis".into(),
                ));
            }
            let topology = self
                .num_nodes
                .saturating_add(self.exact_mna_branches().len())
                .saturating_add(1)
                .saturating_mul(12);
            budget(values.saturating_add(input_values).saturating_add(topology))?;
            let mut forest = self
                .forced_voltage_forest(false, abort)
                .map_err(device_error)?;
            forest
                .retain_coordinates(plans.iter().flat_map(|plan| plan.coordinates()), abort)
                .map_err(device_error)?;
            groups = forest.groups;
            forced_values = input_values
                .saturating_add(topology)
                .saturating_add(forest.nodes.len().saturating_mul(grid.sample_count()));
            budget(
                values
                    .saturating_add(forced_values)
                    .saturating_add(grid.sample_count().saturating_mul(8)),
            )?;
            let projection =
                transform.insert(QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?);
            forced.resize(self.num_nodes, None);
            for node in forest.nodes {
                check_abort(abort)?;
                let mut samples = projection
                    .to_real_samples_with_abort(&sources[self.num_nodes + node.branch], abort)?;
                for (i, value) in samples.iter_mut().enumerate() {
                    if i.is_multiple_of(256) {
                        check_abort(abort)?;
                    }
                    *value = node.sign * *value
                        + node
                            .parent
                            .map_or(0.0, |p| forced[p].as_ref().map_or(0.0, |values| values[i]));
                    if !value.is_finite() {
                        return Err(Error::Numerical(
                            "driven node voltage overflowed during integral preparation".into(),
                        ));
                    }
                }
                forced[node.node] = Some(samples);
            }
        }
        if !retained && let Some(driven) = driven {
            if driven.len() != self.unknowns()
                || driven.iter().flatten().any(|row| row.len() != grid.len())
            {
                return Err(Error::InvalidConfig(
                    "nonlinear input trajectories differ from the periodic basis".into(),
                ));
            }
            if transform.is_none() {
                transform = Some(QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?);
            }
            forced.resize(driven.len(), None);
            groups.extend((groups.len()..driven.len()).map(|i| i + 1));
            for (row, spectrum) in driven.iter().enumerate() {
                let Some(spectrum) = spectrum else {
                    continue;
                };
                if !plans
                    .iter()
                    .any(|plan| plan.coordinates().any(|index| index == row))
                {
                    continue;
                }
                if coordinate_group(&groups, &forced, row).is_none() {
                    continue;
                }
                if forced[row].is_none() {
                    forced_values = forced_values.saturating_add(grid.sample_count());
                }
                budget(
                    values
                        .saturating_add(forced_values)
                        .saturating_add(grid.sample_count().saturating_mul(8)),
                )?;
                forced[row] = None;
                forced[row] = Some(
                    transform
                        .as_mut()
                        .expect("allocated transform")
                        .to_real_samples_with_abort(spectrum, abort)?,
                );
                groups[row] = 0;
            }
        }
        if !retained
            && plans.iter().any(|plan| {
                plan.dependencies_with_offsets(|i| coordinate_group(&groups, &forced, i))
                    .is_none()
            })
        {
            let sources = sources.expect("validated producer source spectra");
            let coordinates = self.linear_driven_spectra(
                &self.behavioral_sources,
                grid.frequencies_hz(),
                |row, k| sources[row][k],
                |row| {
                    coordinate_group(&groups, &forced, row).is_some()
                        && plans
                            .iter()
                            .any(|plan| plan.coordinates().any(|index| index == row))
                },
                limits
                    .max_result_values
                    .min(32_000_000)
                    .saturating_sub(values)
                    .saturating_sub(forced_values)
                    .saturating_sub(grid.sample_count().saturating_mul(8)),
                abort,
            )?;
            let temporary = coordinates.len().saturating_mul(4).saturating_add(
                coordinates
                    .iter()
                    .flatten()
                    .map(|row| row.len().saturating_mul(2))
                    .sum::<usize>(),
            );
            forced.resize(coordinates.len(), None);
            groups.extend((groups.len()..coordinates.len()).map(|i| i + 1));
            for (row, spectrum) in coordinates.iter().enumerate() {
                let Some(spectrum) = spectrum else { continue };
                if coordinate_group(&groups, &forced, row).is_none() {
                    continue;
                }
                if forced[row].is_none() {
                    forced_values = forced_values.saturating_add(grid.sample_count());
                }
                budget(
                    values
                        .saturating_add(forced_values)
                        .saturating_add(temporary)
                        .saturating_add(grid.sample_count().saturating_mul(8)),
                )?;
                forced[row] = None;
                forced[row] = Some(
                    transform
                        .as_mut()
                        .expect("allocated transform")
                        .to_real_samples_with_abort(spectrum, abort)?,
                );
                groups[row] = 0;
            }
        }
        for plan in plans {
            check_abort(abort)?;
            let dependencies = if retained {
                plan.dependencies()
            } else {
                plan.dependencies_with_offsets(|i| coordinate_group(&groups, &forced, i))
            };
            let Some(dependencies) = dependencies else {
                if !retained {
                    for coordinate in plan.coordinates() {
                        if coordinate_group(&groups, &forced, coordinate).is_some() {
                            unresolved[coordinate] = true;
                        }
                    }
                }
                primitives.push(None);
                continue;
            };
            if dependencies
                .iter()
                .any(|&i| primitives.get(i).is_none_or(Option::is_none))
            {
                primitives.push(None);
                continue;
            }
            if retained {
                // The producer owns the entire primitive and its constant.
                // Never reconstruct or re-anchor a retained orbit here.
                primitives.push(Some(Primitive {
                    spectrum: QuasiPeriodicSampleSpectrum {
                        tuples: Vec::new(),
                        coefficients: Vec::new(),
                    },
                    samples: Vec::new(),
                }));
                continue;
            }
            budget(
                values
                    .saturating_add(forced_values)
                    .saturating_add(grid.sample_count().saturating_mul(8)),
            )?;
            if transform.is_none() {
                transform = Some(QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?);
            }
            let mut samples = Vec::with_capacity(grid.sample_count());
            let mut rate_scale = 0.0_f64;
            for sample in 0..grid.sample_count() {
                check_abort(abort)?;
                let phases = grid.phases(sample).expect("bounded phase sample");
                let value = plan
                    .sample_with_coordinates(
                        0.0,
                        &phases,
                        |i| {
                            primitives[i]
                                .as_ref()
                                .expect("qualified dependency")
                                .samples[sample]
                        },
                        |i| {
                            forced
                                .get(i)
                                .and_then(Option::as_ref)
                                .map_or(0.0, |values| values[sample])
                        },
                    )
                    .map_err(Error::InvalidCircuit)?;
                rate_scale = rate_scale.max(value.abs());
                samples.push(value);
            }
            let mut projection_limits = limits.clone();
            projection_limits.max_result_values = limits
                .max_result_values
                .saturating_sub(values)
                .saturating_sub(forced_values);
            let mut spectrum = transform
                .as_mut()
                .expect("allocated transform")
                .to_complete_real_spectrum_with_abort(&samples, &projection_limits, abort)?;
            values = values
                .saturating_add(
                    spectrum
                        .coefficients
                        .len()
                        .saturating_mul(2 + grid.dimensions().len()),
                )
                .saturating_add(grid.sample_count());
            budget(
                values
                    .saturating_add(forced_values)
                    .saturating_add(grid.sample_count().saturating_mul(8)),
            )?;
            let dc = spectrum.coefficients.len() / 2;
            let roundoff = 64.0 * Value::EPSILON * (grid.sample_count() as Value).log2().max(1.0);
            if rate_scale > 0.0 && spectrum.coefficients[dc].norm() / rate_scale > roundoff {
                return Err(Error::InvalidCircuit(format!(
                    "prescribed integral '{}' has nonzero mean input {:e}; its zero-origin primitive cannot be quasiperiodic",
                    plan.name, spectrum.coefficients[dc].re,
                )));
            }
            for k in 0..dc {
                check_abort(abort)?;
                let reflected = spectrum.coefficients.len() - 1 - k;
                let rate = spectrum.coefficients[k];
                let omega = std::f64::consts::TAU
                    * grid.frequency_relative_to(0.0, &spectrum.tuples[dc], &spectrum.tuples[k])?;
                if omega == 0.0 && (rate_scale == 0.0 || rate.norm() / rate_scale <= roundoff) {
                    spectrum.coefficients[k] = Complex64::ZERO;
                    spectrum.coefficients[reflected] = Complex64::ZERO;
                    continue;
                }
                if !omega.is_finite() || omega == 0.0 {
                    return Err(Error::InvalidCircuit(format!(
                        "prescribed integral '{}' has an unrepresentable angular frequency",
                        plan.name
                    )));
                }
                // Scalar division avoids squaring an extreme angular frequency.
                let primitive = Complex64::new(rate.im / omega, -rate.re / omega);
                spectrum.coefficients[k] = primitive;
                spectrum.coefficients[reflected] = primitive.conj();
            }
            spectrum.coefficients[dc] = Complex64::new(
                -2.0 * spectrum.coefficients[..dc]
                    .iter()
                    .map(|c| c.re)
                    .sum::<Value>(),
                0.0,
            );
            if spectrum
                .coefficients
                .iter()
                .any(|c| !c.re.is_finite() || !c.im.is_finite())
            {
                return Err(Error::InvalidCircuit(format!(
                    "prescribed integral '{}' has unrepresentable spectral values",
                    plan.name
                )));
            }
            let samples = transform
                .as_mut()
                .expect("allocated transform")
                .complete_real_samples_with_abort(&spectrum, abort)?;
            primitives.push(Some(Primitive { spectrum, samples }));
        }
        let mut remaining = limits.clone();
        if primitives.iter().any(Option::is_some) {
            remaining.max_result_values = remaining.max_result_values.saturating_sub(values);
            self.quasi_prescribed_integrals = Some(QuasiPrescribedIntegrals {
                scale: grid
                    .config()
                    .frequencies_hz
                    .iter()
                    .copied()
                    .fold(Value::INFINITY, Value::min),
                grid,
                primitives,
                retained,
            });
        }
        Ok((remaining, unresolved))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};
    use crate::analysis::quasi_periodic::{QuasiPeriodicGridConfig, QuasiPeriodicSampling};
    use crate::device::behavioral::{BehavioralSources, BehavioralVoltageSource};

    #[test]
    fn qpss_prescribed_preparation_bounds_complete_spectra_and_cancels_sampling() {
        let limits = ResourceLimits::default();
        let mut config =
            QuasiPeriodicGridConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
        config.sampling = QuasiPeriodicSampling::Exact(vec![64, 64]);
        let grid = Arc::new(QuasiPeriodicGrid::new_with_abort(config, &limits, &NoAbort).unwrap());
        let sources = BehavioralSources {
            voltage_sources: vec![
                BehavioralVoltageSource::new("BV".into(), 1, 0, 1, "sdt(cos(2*pi*1k*time))")
                    .unwrap(),
            ],
            current_sources: vec![],
        };
        let mut solver = HbSolver::try_new(HbConfig::new(17.0).with_harmonics(1), 1).unwrap();
        solver
            .try_add_periodic_constitutive_port_branch(1, 0, 1, "BV")
            .unwrap();
        solver
            .set_quasi_periodic_behavioral_sources(&sources, &grid)
            .unwrap();
        let limited = ResourceLimits {
            max_result_values: 8 * grid.sample_count() + 10,
            ..limits.clone()
        };
        assert!(matches!(
            solver.prepare_quasi_periodic_integrals(
                grid.clone(),
                &limited,
                false,
                None,
                None,
                &NoAbort
            ),
            Err(Error::ResourceLimit(_))
        ));
        assert!(solver.quasi_prescribed_integrals.is_none());
        assert!(matches!(
            solver.prepare_quasi_periodic_integrals(
                grid,
                &limits,
                false,
                None,
                None,
                &CountingAbort::new(32)
            ),
            Err(Error::Aborted)
        ));
        assert!(solver.quasi_prescribed_integrals.is_none());
    }
}

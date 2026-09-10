//! Analytic, prescribed behavioral forcing for the constant MNA descriptor.
//! Trial-time derivatives are prepared once before Newton assembly. Source
//! mappings then read immutable samples without allocating Taylor workspaces
//! or silently evaluating a solution-dependent expression on empty bindings.

use super::*;
use crate::expr::{
    CompiledExpr, Context, TimeDerivativeError, TimeDerivatives, TimeEnclosure, TimeInterval,
};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Derivatives {
    initial: Arc<Vec<Value>>,
    samples: [(Value, Arc<Vec<Value>>); 3],
}

#[derive(Debug, Clone, Default)]
pub(super) struct BehavioralForcing {
    sources: BTreeMap<(PrescribedSource, usize), Derivatives>,
}

fn source(
    circuit: &CircuitData,
    kind: PrescribedSource,
    index: usize,
) -> Result<(&str, &CompiledExpr, Context<'_>), SimulationError> {
    let (name, program) = match kind {
        PrescribedSource::BehavioralVoltage => {
            let source = &circuit.behavioral_sources.voltage_sources[index];
            (source.name.as_str(), source.prescribed_time_program())
        }
        PrescribedSource::BehavioralCurrent => {
            let source = &circuit.behavioral_sources.current_sources[index];
            (source.name.as_str(), source.prescribed_time_program())
        }
        _ => {
            return Err(SimulationError::Circuit(
                "PSS behavioral forcing has an invalid source kind".to_owned(),
            ));
        }
    };
    let (program, context) = program.ok_or_else(|| {
        SimulationError::Circuit(format!("PSS source {name} is not prescribed time forcing"))
    })?;
    Ok((name, program, context))
}

fn derivative_error(
    name: &str,
    time: Value,
    order: usize,
    error: TimeDerivativeError,
) -> SimulationError {
    match error {
        TimeDerivativeError::Aborted => SimulationError::Aborted,
        TimeDerivativeError::Resource(error) => SimulationError::ResourceLimit(error),
        error => SimulationError::Circuit(format!(
            "PSS coupled state constraint requires a finite analytic derivative through order {order} for source {name} at t={time:e}: {error}"
        )),
    }
}

impl BehavioralForcing {
    pub(super) fn new(
        circuit: &CircuitData,
        orders: &BTreeMap<(PrescribedSource, usize), usize>,
        retained_words: &mut usize,
        max_values: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        // Retained initial/three trial arrays, staged replacements and map/
        // Arc headers remain charged during coefficient and interval work.
        for (&(kind, _), &order) in orders {
            if kind.is_behavioral() {
                *retained_words = retained_words
                    .saturating_add(order.saturating_add(1).saturating_mul(8).saturating_add(64));
            }
        }
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::ResultValues,
            *retained_words,
            max_values,
        )?;
        let mut sources = BTreeMap::new();
        for (&(kind, index), &order) in orders {
            if !kind.is_behavioral() {
                continue;
            }
            let (name, program, context) = source(circuit, kind, index)?;
            let derivatives = TimeDerivatives::new(
                program,
                order,
                max_values.saturating_sub(*retained_words),
                abort,
            )
            .and_then(|plan| plan.evaluate(&context, abort))
            .map_err(|error| derivative_error(name, 0.0, order, error))?;
            let initial = Arc::new(derivatives);
            sources.insert(
                (kind, index),
                Derivatives {
                    samples: std::array::from_fn(|_| (0.0, initial.clone())),
                    initial,
                },
            );
        }
        Ok(Self { sources })
    }

    pub(super) fn value(
        &self,
        value: ForestValue,
        time: Value,
        extra: usize,
    ) -> Result<Value, SimulationError> {
        let (kind, index, order) = value.source().ok_or_else(|| {
            SimulationError::Circuit("PSS behavioral forcing is not a source".to_owned())
        })?;
        let values = self.sources.get(&(kind, index)).and_then(|source| {
            if time == 0.0 {
                Some(&source.initial)
            } else {
                source
                    .samples
                    .iter()
                    .find(|(sample_time, _)| *sample_time == time)
                    .map(|(_, values)| values)
            }
        });
        order
            .checked_add(extra)
            .and_then(|order| values.and_then(|values| values.get(order)))
            .copied()
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "PSS behavioral derivative was not prepared at t={time:e}"
                ))
            })
    }

    pub(super) fn prepare(
        &mut self,
        circuit: &CircuitData,
        times: [Value; 3],
        available: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        for (&(kind, index), cache) in &mut self.sources {
            let (name, program, mut context) = source(circuit, kind, index)?;
            let order = cache.initial.len() - 1;
            let plan = TimeDerivatives::new(program, order, available, abort)
                .map_err(|error| derivative_error(name, times[0], order, error))?;
            let mut samples: [(Value, Arc<Vec<Value>>); 3] =
                std::array::from_fn(|_| (0.0, cache.initial.clone()));
            for (index, &time) in times.iter().enumerate() {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let values = if time == 0.0 {
                    cache.initial.clone()
                } else if let Some((_, values)) = cache
                    .samples
                    .iter()
                    .chain(&samples[..index])
                    .find(|(sample_time, _)| *sample_time == time)
                {
                    values.clone()
                } else {
                    context.time = time;
                    Arc::new(
                        plan.evaluate(&context, abort)
                            .map_err(|error| derivative_error(name, time, order, error))?,
                    )
                };
                samples[index] = (time, values);
            }
            cache.samples = samples;
        }
        Ok(())
    }

    pub(super) fn ensure_regular(
        &self,
        circuit: &CircuitData,
        period: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        for (&(kind, index), cache) in &self.sources {
            let (name, program, context) = source(circuit, kind, index)?;
            let order = cache.initial.len() - 1;
            let error = || {
                SimulationError::Circuit(format!(
                    "PSS coupled state constraint could not certify a regular periodic derivative through order {order} for source {name}"
                ))
            };
            let periodic = match kind {
                PrescribedSource::BehavioralVoltage => circuit.behavioral_sources.voltage_sources
                    [index]
                    .has_periodic_time_dependence(period, false),
                PrescribedSource::BehavioralCurrent => circuit.behavioral_sources.current_sources
                    [index]
                    .has_periodic_time_dependence(period, false),
                _ => false,
            };
            if !periodic {
                return Err(error());
            }
            if order == 0 {
                continue;
            }
            let mut enclosure = TimeEnclosure::new(program, period).ok_or_else(error)?;
            let mut pending = vec![(
                TimeInterval {
                    lower: 0.0,
                    upper: period,
                },
                0,
            )];
            let mut work = 0_usize;
            while let Some((interval, depth)) = pending.pop() {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if enclosure.regular_on(interval, &context, order, abort) {
                    continue;
                }
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                work += 1;
                // Finite deterministic proof work; unresolved domains are
                // diagnosed, never admitted by sampling their derivatives.
                if depth >= 20 || work >= 4096 {
                    return Err(error());
                }
                let middle = interval.lower + 0.5 * (interval.upper - interval.lower);
                if middle <= interval.lower || middle >= interval.upper {
                    return Err(error());
                }
                pending.push((
                    TimeInterval {
                        lower: middle,
                        upper: interval.upper,
                    },
                    depth + 1,
                ));
                pending.push((
                    TimeInterval {
                        lower: interval.lower,
                        upper: middle,
                    },
                    depth + 1,
                ));
            }
        }
        Ok(())
    }
}

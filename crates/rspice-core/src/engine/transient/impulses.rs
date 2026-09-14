//! Bounded, explicitly covered current observations before joint acceptance.
use super::*;
use crate::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};
use state_commit::physical_event::{PhysicalDeviceImpulses, PreparedPhysicalEvent};

fn failure(message: impl std::fmt::Display) -> SimulationError {
    SimulationError::Circuit(format!("physical current impulse observation: {message}"))
}

fn allocated<T>(count: usize) -> Result<Vec<T>, SimulationError> {
    let mut values = Vec::new();
    values.try_reserve_exact(count).map_err(failure)?;
    Ok(values)
}

fn copied(name: &str) -> Result<String, SimulationError> {
    let mut value = String::new();
    value.try_reserve_exact(name.len()).map_err(failure)?;
    value.push_str(name);
    Ok(value)
}

pub(super) struct Plan {
    num_nodes: usize,
    solved_branches: usize,
    capacitors: Vec<Option<usize>>,
    bjt_terminals: Vec<[usize; 4]>,
}

pub(super) fn initialize(
    result: &mut TransientResult,
    circuit: &crate::CircuitData,
    derived: &[DerivedTransientBranchCurrent],
    options: &charge_event::EventOptions,
    flux_tolerance: Value,
    retained_values: usize,
    abort: &dyn AbortSignal,
) -> Result<(Plan, usize), SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if result.current_impulses.is_some() {
        return Err(failure("history was already initialized"));
    }
    // Coverage is a physical claim. Run the complete admitted model census
    // before declaring any owner known, including on a checkpoint resume.
    charge_event::circuit::PreparedEventCircuit::new(circuit, flux_tolerance, options, abort)?;
    let solved_branches = circuit.num_branches();
    if result.num_nodes != circuit.num_nodes()
        || result.branch_names.len() != solved_branches.saturating_add(derived.len())
    {
        return Err(failure("unaligned current owner inventory"));
    }
    let bytes =
        |length: usize| 1usize.saturating_add(length.div_ceil(std::mem::size_of::<Value>()));
    let mut added_values = result.branch_names.iter().fold(0usize, |count, name| {
        count.saturating_add(bytes(name.len()))
    });
    for model in &circuit.bjts.devices {
        added_values = added_values
            .saturating_add(bytes(model.name.len().saturating_add(2)).saturating_mul(4));
    }
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::ResultValues,
        retained_values.saturating_add(added_values),
        options.limits.max_result_values,
    )?;
    let count = result
        .branch_names
        .len()
        .saturating_add(circuit.bjts.len().saturating_mul(4));
    let mut traces = allocated(count)?;
    for name in &result.branch_names {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        traces.push(CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: copied(name)?,
            },
            complete: true,
            points: Vec::new(),
        });
    }
    let mut capacitors = allocated(circuit.capacitors.len())?;
    capacitors.resize(circuit.capacitors.len(), None);
    for (ordinal, branch) in derived.iter().enumerate() {
        match branch.kind {
            DerivedTransientBranchCurrentKind::LinearCapacitor => {
                let target = capacitors
                    .get_mut(branch.index)
                    .ok_or_else(|| failure("unknown capacitor owner"))?;
                if target.replace(solved_branches + ordinal).is_some() {
                    return Err(failure("duplicate capacitor owner"));
                }
            }
            DerivedTransientBranchCurrentKind::LinearResistor
            | DerivedTransientBranchCurrentKind::IndependentCurrentSource => {}
            _ => {
                return Err(failure(
                    "derived current has no qualified physical impulse owner",
                ));
            }
        }
    }
    let mut bjt_terminals = allocated(circuit.bjts.len())?;
    for model in &circuit.bjts.devices {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let first = traces.len();
        for parameter in ["ic", "ib", "ie", "is"] {
            traces.push(CurrentImpulseTrace {
                owner: CurrentImpulseOwner::DeviceLead {
                    device_name: copied(&model.name)?,
                    parameter: copied(parameter)?,
                },
                complete: true,
                points: Vec::new(),
            });
        }
        bjt_terminals.push(std::array::from_fn(|index| first + index));
    }
    crate::transient_observation::validate_current_impulse_traces(
        Some(&traces),
        result.time.first().copied(),
        result.time.last().copied(),
        result.branch_names.iter().map(String::as_str),
    )
    .map_err(failure)?;
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    result.current_impulses = Some(traces);
    Ok((
        Plan {
            num_nodes: result.num_nodes,
            solved_branches,
            capacitors,
            bjt_terminals,
        },
        added_values,
    ))
}

impl Plan {
    fn source_index(&self, coordinate: usize) -> Result<usize, SimulationError> {
        coordinate
            .checked_sub(self.num_nodes)
            .filter(|&index| index < self.solved_branches)
            .ok_or_else(|| failure("impulse coordinate is not a solved branch"))
    }

    pub(super) fn prepare<'a>(
        &self,
        result: &'a mut TransientResult,
        event: &PreparedPhysicalEvent,
        retained_values: usize,
        limits: &crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Prepared<'a>, SimulationError> {
        let (capacitors, terminals): (&[Value], &[[Value; 4]]) = match event.device_impulses() {
            PhysicalDeviceImpulses::Continuous => (&[], &[]),
            PhysicalDeviceImpulses::Jumps {
                capacitors,
                bjt_terminals,
            } => {
                if capacitors.len() != self.capacitors.len()
                    || bjt_terminals.len() != self.bjt_terminals.len()
                {
                    return Err(failure("unaligned physical device impulse population"));
                }
                (capacitors, bjt_terminals)
            }
        };
        let sources = event
            .impulses()
            .map(|(coordinate, charge)| self.source_index(coordinate).map(|index| (index, charge)));
        let capacitors = capacitors
            .iter()
            .enumerate()
            .filter_map(|(index, &charge)| self.capacitors[index].map(|owner| Ok((owner, charge))));
        let terminals = terminals
            .iter()
            .zip(&self.bjt_terminals)
            .flat_map(|(charges, owners)| {
                charges
                    .iter()
                    .zip(owners)
                    .map(|(&charge, &owner)| Ok((owner, charge)))
            });
        prepare(
            result,
            event.time(),
            sources.chain(capacitors).chain(terminals),
            retained_values,
            limits,
            abort,
        )
    }
}

/// Exclusive ownership keeps the prepared target unchanged until acceptance.
/// Dropping publishes nothing; commit has no allocation or fallible work.
#[must_use]
pub(super) struct Prepared<'a> {
    target: &'a mut Vec<CurrentImpulseTrace>,
    additions: Vec<(usize, CurrentImpulsePoint)>,
    added_values: usize,
}

fn prepare<'a>(
    result: &'a mut TransientResult,
    time: Value,
    impulses: impl Iterator<Item = Result<(usize, Value), SimulationError>>,
    retained_values: usize,
    limits: &crate::resource::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Prepared<'a>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if !time.is_finite() || time < 0.0 || result.time.last().is_none_or(|previous| time < *previous)
    {
        return Err(failure("invalid accepted event time"));
    }
    let target = result
        .current_impulses
        .as_mut()
        .ok_or_else(|| failure("physical history is unavailable"))?;
    let mut additions: Vec<(usize, CurrentImpulsePoint)> = Vec::new();
    let mut added_values = 0usize;
    for impulse in impulses {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let (index, charge_coulombs) = impulse?;
        let trace = target
            .get_mut(index)
            .filter(|trace| trace.complete)
            .ok_or_else(|| failure("impulse owner has no complete physical coverage"))?;
        if !charge_coulombs.is_finite() {
            return Err(failure("nonfinite charge"));
        }
        if charge_coulombs == 0.0 {
            continue;
        }
        if additions.iter().any(|(owner, _)| *owner == index) {
            return Err(failure("duplicate current impulse in one accepted event"));
        }
        if trace.points.last().is_some_and(|point| point.time >= time) {
            return Err(failure("impulse would replay or precede an accepted event"));
        }
        added_values = added_values.saturating_add(2);
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::ResultValues,
            retained_values.saturating_add(added_values),
            limits.max_result_values,
        )?;
        additions.try_reserve(1).map_err(failure)?;
        trace.points.try_reserve(1).map_err(failure)?;
        additions.push((
            index,
            CurrentImpulsePoint {
                time,
                charge_coulombs,
            },
        ));
    }
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    Ok(Prepared {
        target,
        additions,
        added_values,
    })
}

impl Prepared<'_> {
    pub(super) fn commit(self) -> usize {
        for (trace, point) in self.additions {
            self.target[trace].points.push(point);
        }
        self.added_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};

    fn result() -> TransientResult {
        TransientResult {
            time: vec![0.0],
            step_sizes: vec![0.0],
            num_nodes: 1,
            node_names: vec!["n".into()],
            voltages: vec![vec![0.0]],
            branch_names: vec!["Vdrive".into(), "Rzero".into(), "Cderived".into()],
            branch_currents: vec![vec![0.0]; 3],
            current_impulses: Some(
                ["Vdrive", "Rzero"]
                    .into_iter()
                    .map(|name| CurrentImpulseTrace {
                        owner: CurrentImpulseOwner::Branch {
                            branch_name: name.into(),
                        },
                        complete: true,
                        points: Vec::new(),
                    })
                    .collect(),
            ),
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    fn charges<'a>(
        output: &'a mut TransientResult,
        time: Value,
        values: &[(usize, Value)],
        retained: usize,
        limits: &crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Prepared<'a>, SimulationError> {
        prepare(
            output,
            time,
            values.iter().copied().map(Ok),
            retained,
            limits,
            abort,
        )
    }

    #[test]
    fn impulse_preparation_drop_publishes_nothing_and_commit_preserves_smallest_charge() {
        let limits = crate::resource::ResourceLimits::default();
        let mut result = result();
        let before = result.current_impulses.clone();
        drop(
            charges(
                &mut result,
                0.0,
                &[(0, -1e-12), (1, 0.0)],
                0,
                &limits,
                &NoAbort,
            )
            .unwrap(),
        );
        assert_eq!(result.current_impulses, before);
        assert_eq!(
            charges(&mut result, 0.0, &[(0, -1e-12)], 0, &limits, &NoAbort)
                .unwrap()
                .commit(),
            2
        );
        assert_eq!(
            charges(
                &mut result,
                1.0,
                &[(0, Value::from_bits(1)), (1, 1e-12)],
                2,
                &limits,
                &NoAbort
            )
            .unwrap()
            .commit(),
            4
        );
        let traces = result.current_impulses.unwrap();
        assert_eq!(traces[0].points[0].charge_coulombs, -1e-12);
        assert_eq!(traces[0].points[1].charge_coulombs.to_bits(), 1);
        assert_eq!(traces[1].points[0].charge_coulombs, 1e-12);
        assert!(traces.iter().all(|trace| trace.complete));
    }

    #[test]
    fn impulse_preparation_refuses_budget_cancellation_and_bad_identity_without_publication() {
        let mut output = result();
        let before = output.current_impulses.clone();
        let retained = Engine::transient_result_value_count(&output);
        let tight = crate::resource::ResourceLimits {
            max_result_values: retained + 1,
            ..Default::default()
        };
        assert!(matches!(
            charges(&mut output, 0.0, &[(0, 1.0)], retained, &tight, &NoAbort),
            Err(SimulationError::ResourceLimit(_))
        ));
        assert_eq!(output.current_impulses, before);
        let limits = crate::resource::ResourceLimits::default();
        for values in [
            vec![(2, 1.0)],
            vec![(usize::MAX, 1.0)],
            vec![(0, Value::NAN)],
            vec![(0, 1.0), (0, 2.0)],
        ] {
            assert!(charges(&mut output, 0.0, &values, retained, &limits, &NoAbort).is_err());
            assert_eq!(output.current_impulses, before);
        }
        let census = CountingAbort::new(usize::MAX);
        drop(
            charges(
                &mut output,
                0.0,
                &[(0, 1.0), (1, 2.0)],
                retained,
                &limits,
                &census,
            )
            .unwrap(),
        );
        for poll in 0..census.count() {
            let abort = CountingAbort::new(poll);
            assert!(matches!(
                charges(
                    &mut output,
                    0.0,
                    &[(0, 1.0), (1, 2.0)],
                    retained,
                    &limits,
                    &abort
                ),
                Err(SimulationError::Aborted)
            ));
            assert_eq!(output.current_impulses, before);
            assert_eq!(abort.polls_after_abort(), 0);
        }
        output.current_impulses.as_mut().unwrap()[0].complete = false;
        assert!(charges(&mut output, 0.0, &[(0, 1.0)], retained, &limits, &NoAbort).is_err());
        let plan = Plan {
            num_nodes: 1,
            solved_branches: 2,
            capacitors: Vec::new(),
            bjt_terminals: Vec::new(),
        };
        assert_eq!(plan.source_index(1).unwrap(), 0);
        assert_eq!(plan.source_index(2).unwrap(), 1);
        assert!(plan.source_index(0).is_err());
        assert!(
            plan.source_index(3).is_err(),
            "a derived report slot is not an MNA source"
        );
    }

    #[test]
    fn impulse_preparation_rejects_seam_replay_and_keeps_zero_events_sparse() {
        let limits = crate::resource::ResourceLimits::default();
        let mut output = result();
        let before = output.current_impulses.clone();
        assert_eq!(
            charges(
                &mut output,
                0.0,
                &[(0, 0.0), (1, -0.0)],
                0,
                &limits,
                &NoAbort
            )
            .unwrap()
            .commit(),
            0
        );
        assert_eq!(output.current_impulses, before);
        charges(&mut output, 0.0, &[(0, 1.0)], 0, &limits, &NoAbort)
            .unwrap()
            .commit();
        let before = output.current_impulses.clone();
        for time in [0.0, -1.0, Value::INFINITY, Value::NAN] {
            assert!(charges(&mut output, time, &[(0, 1.0)], 2, &limits, &NoAbort).is_err());
            assert_eq!(output.current_impulses, before);
        }
    }
}

//! A generated compact model's `idt` as a shooting coordinate.
//!
//! An integral's value is a state of its own: no node voltage or charge at
//! the period start fixes it. Each `idt` a generated model carries (harmonic
//! balance carries the same sites as unknowns of their own) is therefore a
//! coordinate after the expression integrals. A period starts with every
//! integral held at its coordinate: the consistency solve evaluates the model
//! with each `idt`'s output held there, and the seed that starts the model's
//! accepted history holds it too, then starts the integral's history at it,
//! with the integrand it saw as the previous input, as a transient's first
//! point starts one at its initial condition. The steps then integrate it by
//! the model's own companion rule, and the coordinate's image is the
//! integral the last step accepted.
use super::*;

#[derive(Debug, Clone, Default)]
pub(in crate::engine::pss) struct PssGeneratedIntegrals {
    /// Each generated device that carries integrals, with how many, in device
    /// order. A build without generated models has none.
    #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(dead_code))]
    devices: Vec<(usize, usize)>,
    dimension: usize,
}

impl PssGeneratedIntegrals {
    pub(super) fn new(circuit: &CircuitData) -> Self {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let devices: Vec<(usize, usize)> = circuit
                .generated_veriloga_devices()
                .iter()
                .map(|device| device.periodic_integral_count())
                .enumerate()
                .filter(|&(_, count)| count > 0)
                .collect();
            let dimension = devices.iter().map(|&(_, count)| count).sum();
            Self { devices, dimension }
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            let _ = circuit;
            Self::default()
        }
    }

    pub(in crate::engine::pss) fn dimension(&self) -> usize {
        self.dimension
    }

    /// Each carried integral's device and its place among that device's
    /// integrals, in coordinate order.
    #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(dead_code))]
    pub(in crate::engine::pss) fn sites(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.devices
            .iter()
            .flat_map(|&(device, count)| (0..count).map(move |index| (device, index)))
    }

    /// Each device that carries integrals, with its first coordinate's place
    /// among the integrals and how many it carries.
    #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(dead_code))]
    pub(in crate::engine::pss) fn devices(
        &self,
    ) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        self.devices.iter().scan(0, |first, &(device, count)| {
            let start = *first;
            *first += count;
            Some((device, start, count))
        })
    }

    pub(super) fn names(&self, circuit: &CircuitData) -> Vec<String> {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let devices: Vec<_> = circuit.generated_veriloga_devices().iter().collect();
            self.sites()
                .map(|(device, index)| format!("X:{}:idt:{index}", devices[device].instance_name()))
                .collect()
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            let _ = circuit;
            Vec::new()
        }
    }

    /// Every carried integral's accepted value, in coordinate order.
    pub(in crate::engine::pss) fn extract(&self, circuit: &CircuitData) -> Vec<Value> {
        if self.dimension == 0 {
            return Vec::new();
        }
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let devices: Vec<_> = circuit.generated_veriloga_devices().iter().collect();
            self.devices
                .iter()
                .flat_map(|&(device, _)| devices[device].accepted_integrals())
                .collect()
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            let _ = circuit;
            Vec::new()
        }
    }

    /// Hold each carried integral's output at `values` through the
    /// evaluations that follow (see `BuiltinVerilogAInstance::hold_integrals`).
    pub(in crate::engine::pss) fn hold(&self, circuit: &mut CircuitData, values: &[Value]) {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let mut held = self.devices().peekable();
            for (index, device) in circuit
                .generated_veriloga_devices_mut()
                .iter_mut()
                .enumerate()
            {
                if let Some(&(_, first, count)) = held.peek().filter(|entry| entry.0 == index) {
                    device.hold_integrals(&values[first..first + count], None);
                    held.next();
                }
            }
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        let _ = (circuit, values);
    }

    /// Seed every generated model's accepted state at `solution`, as
    /// `CircuitData::seed_generated_veriloga_bias` does, with each carried
    /// integral held at `values` and its history started there.
    pub(super) fn seed(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        values: &[Value],
    ) -> Result<(), SimulationError> {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            self.hold(circuit, values);
            let seeded = circuit
                .seed_generated_veriloga_bias(solution)
                .map_err(SimulationError::Circuit);
            let mut held = self.devices().peekable();
            let mut started = Ok(());
            for (index, device) in circuit
                .generated_veriloga_devices_mut()
                .iter_mut()
                .enumerate()
            {
                if let Some(&(_, first, count)) = held.peek().filter(|entry| entry.0 == index) {
                    held.next();
                    if seeded.is_ok() && started.is_ok() {
                        started = device
                            .start_held_integrals(&values[first..first + count])
                            .map_err(SimulationError::Circuit);
                    } else {
                        device.release_held_integrals();
                    }
                }
            }
            seeded.and(started)
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            let _ = (circuit, solution, values);
            Ok(())
        }
    }
}

/// How the carried integrals couple to the circuit at one accepted point,
/// each by its place among the integrals: the step's companion rule
/// integrates `ẏ = u(x, y)` beside `F(x, y) + Q̇(x) + s(t) = 0`.
#[derive(Debug, Clone, Default)]
pub(in crate::engine::pss) struct PssIntegralCoupling {
    /// `(integral, column, ∂u/∂x)`: each integrand's partial along a
    /// zero-based circuit unknown.
    pub(in crate::engine::pss) input: Vec<(usize, usize, Value)>,
    /// `(row, integral, ∂F/∂y)`: each model row's partial with respect to an
    /// integral's output.
    pub(in crate::engine::pss) output: Vec<(usize, usize, Value)>,
    /// `(integral, integral, ∂u/∂y)`: one integrand's partial with respect to
    /// another integral's output.
    pub(in crate::engine::pss) cross: Vec<(usize, usize, Value)>,
}

impl PssGeneratedIntegrals {
    /// The couplings at each of `points` (a solution and the integrals there),
    /// read off each model with its integrals held: a held stamp gives each
    /// integrand's partials, and a stamp seeding one integral's output along
    /// a lane its integrand was differentiated along differs from the held
    /// one, in that lane's column, by the partials with respect to that
    /// output. `None` when an integral has no such lane: its output's
    /// partials cannot be read so, and the caller keeps finite differences.
    ///
    /// A model whose charges read an integral's output directly would need
    /// those partials too; none of the shipped ones does (PSP's non-quasi-
    /// static charges read it through the node voltage its contribution
    /// sets), and the period map's directional check refuses the result
    /// where one did.
    pub(in crate::engine::pss) fn couplings<'a>(
        &self,
        circuit: &CircuitData,
        points: impl Iterator<Item = (&'a [Value], &'a [Value])> + Clone,
    ) -> Result<Option<Vec<PssIntegralCoupling>>, SimulationError> {
        #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(unused_mut))]
        let mut couplings: Vec<PssIntegralCoupling> = Vec::new();
        #[cfg(feature = "veriloga-builtins-base")]
        {
            use crate::device::veriloga_builtins::{
                GeneratedAnalysisKind, GeneratedDdtCoefficients,
            };
            let num_nodes = circuit.num_nodes();
            let size = circuit.matrix_size();
            let simparams = circuit.generated_simulation_parameters;
            let devices: Vec<_> = circuit.generated_veriloga_devices().iter().collect();
            couplings.resize_with(points.clone().count(), Default::default);
            let mut lanes = Vec::new();
            for (device, first, count) in self.devices() {
                let mut probe = devices[device].clone();
                let mut rows: Vec<usize> = probe
                    .nodes
                    .iter()
                    .filter(|&&node| node > 0)
                    .map(|&node| node - 1)
                    .chain(
                        probe
                            .branches
                            .iter()
                            .filter(|&&branch| branch > 0)
                            .map(|&branch| num_nodes + branch - 1),
                    )
                    .collect();
                rows.sort_unstable();
                rows.dedup();
                let triplets: Vec<(usize, usize, Value)> = rows
                    .iter()
                    .flat_map(|&row| rows.iter().map(move |&col| (row, col, 0.0)))
                    .collect();
                let mut sink =
                    StaticMatrix::from_triplets(size, size, &triplets).map_err(|error| {
                        SimulationError::Circuit(format!("integral probe pattern: {error}"))
                    })?;
                let mut rhs = vec![0.0; size];
                probe.link_static_stamps(&sink, num_nodes);
                probe.set_timepoint(0.0, 0.0, GeneratedDdtCoefficients::inactive());
                probe.set_analysis_step(false, false);
                let mut stamp =
                    |probe: &mut crate::device::veriloga_builtins::BuiltinVerilogAInstance,
                     sink: &mut StaticMatrix,
                     solution: &[Value]|
                     -> Result<(), SimulationError> {
                        sink.clear_values();
                        rhs.fill(0.0);
                        probe
                        .stamp_probe(
                            sink,
                            &mut rhs,
                            solution,
                            num_nodes,
                            GeneratedAnalysisKind::Tran,
                            simparams,
                        )
                        .map_err(|error| {
                            SimulationError::Circuit(format!(
                                "generated Verilog-A device '{}' integral probe failed: {error}",
                                probe.instance_name()
                            ))
                        })
                    };
                let mut seeds: Vec<(u32, usize)> = Vec::with_capacity(count);
                let mut held_columns = vec![0.0; count * rows.len()];
                let mut held_cross = vec![0.0; count * count];
                for (coupling, (solution, integrals)) in couplings.iter_mut().zip(points.clone()) {
                    let values = &integrals[first..first + count];
                    probe.hold_integrals(values, None);
                    stamp(&mut probe, &mut sink, solution)?;
                    seeds.clear();
                    for index in 0..count {
                        probe.held_integrand_lanes(index, &mut lanes);
                        let mut seed = None;
                        for &lane in &lanes {
                            let Some(column) = probe.lane_unknown(lane, num_nodes) else {
                                continue;
                            };
                            if let Some(partial) = probe.held_integrand_partial(index, lane) {
                                coupling.input.push((first + index, column, partial));
                            }
                            if seed.is_none() && rows.binary_search(&column).is_ok() {
                                seed = Some((lane, column));
                            }
                        }
                        let Some(seed) = seed else {
                            probe.release_held_integrals();
                            return Ok(None);
                        };
                        seeds.push(seed);
                    }
                    for (index, &(lane, column)) in seeds.iter().enumerate() {
                        for (row, col, value) in sink.stored_entries() {
                            if col == column
                                && let Ok(slot) = rows.binary_search(&row)
                            {
                                held_columns[index * rows.len() + slot] = value;
                            }
                        }
                        for other in 0..count {
                            held_cross[index * count + other] =
                                probe.held_integrand_partial(other, lane).unwrap_or(0.0);
                        }
                    }
                    for (index, &(lane, column)) in seeds.iter().enumerate() {
                        probe.hold_integrals(values, Some((index, lane)));
                        stamp(&mut probe, &mut sink, solution)?;
                        for (row, col, value) in sink.stored_entries() {
                            if col == column
                                && let Ok(slot) = rows.binary_search(&row)
                            {
                                let partial = value - held_columns[index * rows.len() + slot];
                                if partial != 0.0 {
                                    coupling.output.push((row, first + index, partial));
                                }
                            }
                        }
                        for other in 0..count {
                            let partial = probe.held_integrand_partial(other, lane).unwrap_or(0.0)
                                - held_cross[index * count + other];
                            if partial != 0.0 {
                                coupling.cross.push((first + other, first + index, partial));
                            }
                        }
                    }
                }
                probe.release_held_integrals();
            }
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        let _ = (circuit, points);
        Ok(Some(couplings))
    }
}

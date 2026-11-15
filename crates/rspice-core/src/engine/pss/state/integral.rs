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
//!
//! An integral whose integrand is identically zero at the instance's
//! parameters (a non-quasi-static collocation point PSP's SWNQS leaves out)
//! keeps its initial condition for good, in the transient as here. It is a
//! constant of the model, not a coordinate: as one, it would add a Floquet
//! multiplier of exactly one that nothing in the circuit has.
use super::*;

/// The shooting basis's name for a generated instance's `slot`-th `idt`.
#[cfg(feature = "veriloga-builtins-base")]
pub(in crate::engine) fn generated_integral_coordinate_name(instance: &str, slot: usize) -> String {
    format!("X:{instance}:idt:{slot}")
}

#[derive(Debug, Clone, Default)]
pub(in crate::engine::pss) struct PssGeneratedIntegrals {
    /// Each generated device whose integrals can move, in device order, with
    /// their places among its integrals. A build without generated models
    /// has none.
    #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(dead_code))]
    devices: Vec<(usize, Vec<usize>)>,
    dimension: usize,
}

impl PssGeneratedIntegrals {
    pub(super) fn new(circuit: &CircuitData) -> Self {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let devices: Vec<(usize, Vec<usize>)> = circuit
                .generated_veriloga_devices()
                .iter()
                .enumerate()
                .filter(|(_, device)| device.periodic_integral_count() > 0)
                .map(|(index, device)| (index, moving_integrals(circuit, device)))
                .filter(|(_, sites)| !sites.is_empty())
                .collect();
            let dimension = devices.iter().map(|(_, sites)| sites.len()).sum();
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
            .flat_map(|(device, sites)| sites.iter().map(move |&site| (*device, site)))
    }

    /// Each device that carries integrals, with its first coordinate's place
    /// among the integrals and the places among its own integrals of those
    /// it carries.
    #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(dead_code))]
    pub(in crate::engine::pss) fn devices(
        &self,
    ) -> impl Iterator<Item = (usize, usize, &[usize])> + '_ {
        self.devices.iter().scan(0, |first, (device, sites)| {
            let start = *first;
            *first += sites.len();
            Some((*device, start, sites.as_slice()))
        })
    }

    pub(super) fn names(&self, circuit: &CircuitData) -> Vec<String> {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let devices: Vec<_> = circuit.generated_veriloga_devices().iter().collect();
            self.sites()
                .map(|(device, index)| {
                    generated_integral_coordinate_name(devices[device].instance_name(), index)
                })
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
                .flat_map(|(device, sites)| {
                    let accepted = devices[*device].accepted_integrals();
                    sites.iter().map(move |&site| accepted[site])
                })
                .collect()
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            let _ = circuit;
            Vec::new()
        }
    }

    /// Hold each carried integral's output at `values` through the
    /// evaluations that follow (see `BuiltinVerilogAInstance::hold_integrals`),
    /// and each constant one at its accepted value. Returns every held
    /// device's integrals, in device order.
    #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(unused_variables))]
    pub(in crate::engine::pss) fn hold(
        &self,
        circuit: &mut CircuitData,
        values: &[Value],
    ) -> Vec<Vec<Value>> {
        #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(unused_mut))]
        let mut held = Vec::with_capacity(self.devices.len());
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let mut carried = self.devices().peekable();
            for (index, device) in circuit
                .generated_veriloga_devices_mut()
                .iter_mut()
                .enumerate()
            {
                if let Some(&(_, first, sites)) = carried.peek().filter(|entry| entry.0 == index) {
                    carried.next();
                    let mut integrals = device.accepted_integrals();
                    for (&site, &value) in sites.iter().zip(&values[first..first + sites.len()]) {
                        integrals[site] = value;
                    }
                    device.hold_integrals(&integrals, None);
                    held.push(integrals);
                }
            }
        }
        held
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
            let held = self.hold(circuit, values);
            let seeded = circuit
                .seed_generated_veriloga_bias(solution)
                .map_err(SimulationError::Circuit);
            let mut carried = self.devices().zip(&held).peekable();
            let mut started = Ok(());
            for (index, device) in circuit
                .generated_veriloga_devices_mut()
                .iter_mut()
                .enumerate()
            {
                if let Some(&(_, integrals)) = carried.peek().filter(|entry| entry.0.0 == index) {
                    carried.next();
                    if seeded.is_ok() && started.is_ok() {
                        started = device
                            .start_held_integrals(integrals)
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

/// A generated model evaluated on a copy, alone, into a sink over the rows it
/// reaches, with no companion rule: what its stamps and its held integrands
/// read at a given solution.
#[cfg(feature = "veriloga-builtins-base")]
struct HeldProbe {
    device: crate::device::veriloga_builtins::BuiltinVerilogAInstance,
    /// The zero-based solution rows the model reaches, ascending.
    rows: Vec<usize>,
    sink: StaticMatrix,
    rhs: Vec<Value>,
    num_nodes: usize,
    simparams: rspice_veriloga_runtime::GeneratedSimulationParameters,
}

#[cfg(feature = "veriloga-builtins-base")]
impl HeldProbe {
    fn new(
        circuit: &CircuitData,
        device: &crate::device::veriloga_builtins::BuiltinVerilogAInstance,
    ) -> Result<Self, SimulationError> {
        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        let mut device = device.clone();
        let mut rows: Vec<usize> = device
            .nodes
            .iter()
            .filter(|&&node| node > 0)
            .map(|&node| node - 1)
            .chain(
                device
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
        let sink = StaticMatrix::from_triplets(size, size, &triplets).map_err(|error| {
            SimulationError::Circuit(format!("integral probe pattern: {error}"))
        })?;
        device.link_static_stamps(&sink, num_nodes);
        device.set_timepoint(
            0.0,
            0.0,
            crate::device::veriloga_builtins::GeneratedDdtCoefficients::inactive(),
        );
        device.set_analysis_step(false, false);
        Ok(Self {
            device,
            rows,
            sink,
            rhs: vec![0.0; size],
            num_nodes,
            simparams: circuit.generated_simulation_parameters,
        })
    }

    fn stamp(&mut self, solution: &[Value]) -> Result<(), SimulationError> {
        self.sink.clear_values();
        self.rhs.fill(0.0);
        self.device
            .stamp_probe(
                &mut self.sink,
                &mut self.rhs,
                solution,
                self.num_nodes,
                crate::device::veriloga_builtins::GeneratedAnalysisKind::Tran,
                self.simparams,
            )
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "generated Verilog-A device '{}' integral probe failed: {error}",
                    self.device.instance_name()
                ))
            })
    }
}

/// The places among `device`'s integrals of those that can move: every one
/// but those whose integrand, and its every partial, is exactly zero both at
/// the zero solution with the integrals at their accepted values and at a
/// generic one, each unknown at 0.1 and each integral 0.1 above. A model that
/// cannot be evaluated at either keeps them all.
#[cfg(feature = "veriloga-builtins-base")]
fn moving_integrals(
    circuit: &CircuitData,
    device: &crate::device::veriloga_builtins::BuiltinVerilogAInstance,
) -> Vec<usize> {
    let count = device.periodic_integral_count();
    let every = || (0..count).collect();
    let Ok(mut probe) = HeldProbe::new(circuit, device) else {
        return every();
    };
    let accepted = device.accepted_integrals();
    let mut moving = vec![false; count];
    let mut lanes = Vec::new();
    for level in [0.0, 0.1] {
        let solution = vec![level; circuit.matrix_size()];
        let values: Vec<Value> = accepted.iter().map(|value| value + level).collect();
        probe.device.hold_integrals(&values, None);
        if probe.stamp(&solution).is_err() {
            return every();
        }
        for (site, moves) in moving.iter_mut().enumerate() {
            probe.device.held_integrand_lanes(site, &mut lanes);
            *moves |= probe.device.held_integrand(site) != 0.0
                || lanes.iter().any(|&lane| {
                    probe
                        .device
                        .held_integrand_partial(site, lane)
                        .is_some_and(|partial| partial != 0.0)
                });
        }
    }
    (0..count).filter(|&site| moving[site]).collect()
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
            let num_nodes = circuit.num_nodes();
            let devices: Vec<_> = circuit.generated_veriloga_devices().iter().collect();
            couplings.resize_with(points.clone().count(), Default::default);
            let mut lanes = Vec::new();
            for (device, first, sites) in self.devices() {
                let mut probe = HeldProbe::new(circuit, devices[device])?;
                let constants = probe.device.accepted_integrals();
                let rows = std::mem::take(&mut probe.rows);
                let count = sites.len();
                let mut seeds: Vec<(u32, usize)> = Vec::with_capacity(count);
                let mut held_columns = vec![0.0; count * rows.len()];
                let mut held_cross = vec![0.0; count * count];
                for (coupling, (solution, integrals)) in couplings.iter_mut().zip(points.clone()) {
                    let mut values = constants.clone();
                    for (&site, &value) in sites.iter().zip(&integrals[first..first + count]) {
                        values[site] = value;
                    }
                    probe.device.hold_integrals(&values, None);
                    probe.stamp(solution)?;
                    seeds.clear();
                    for (index, &site) in sites.iter().enumerate() {
                        probe.device.held_integrand_lanes(site, &mut lanes);
                        let mut seed = None;
                        for &lane in &lanes {
                            let Some(column) = probe.device.lane_unknown(lane, num_nodes) else {
                                continue;
                            };
                            if let Some(partial) = probe.device.held_integrand_partial(site, lane) {
                                coupling.input.push((first + index, column, partial));
                            }
                            if seed.is_none() && rows.binary_search(&column).is_ok() {
                                seed = Some((lane, column));
                            }
                        }
                        let Some(seed) = seed else {
                            return Ok(None);
                        };
                        seeds.push(seed);
                    }
                    for (index, &(lane, column)) in seeds.iter().enumerate() {
                        for (row, col, value) in probe.sink.stored_entries() {
                            if col == column
                                && let Ok(slot) = rows.binary_search(&row)
                            {
                                held_columns[index * rows.len() + slot] = value;
                            }
                        }
                        for (other, &site) in sites.iter().enumerate() {
                            held_cross[index * count + other] = probe
                                .device
                                .held_integrand_partial(site, lane)
                                .unwrap_or(0.0);
                        }
                    }
                    for (index, &(lane, column)) in seeds.iter().enumerate() {
                        probe
                            .device
                            .hold_integrals(&values, Some((sites[index], lane)));
                        probe.stamp(solution)?;
                        for (row, col, value) in probe.sink.stored_entries() {
                            if col == column
                                && let Ok(slot) = rows.binary_search(&row)
                            {
                                let partial = value - held_columns[index * rows.len() + slot];
                                if partial != 0.0 {
                                    coupling.output.push((row, first + index, partial));
                                }
                            }
                        }
                        for (other, &site) in sites.iter().enumerate() {
                            let partial = probe
                                .device
                                .held_integrand_partial(site, lane)
                                .unwrap_or(0.0)
                                - held_cross[index * count + other];
                            if partial != 0.0 {
                                coupling.cross.push((first + other, first + index, partial));
                            }
                        }
                    }
                }
            }
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        let _ = (circuit, points);
        Ok(Some(couplings))
    }
}

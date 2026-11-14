//! The shooting coordinates of an event orbit, read from and written into
//! live images.
//!
//! An image splits into two parts. `z`, the continuous coordinates the
//! Newton iteration solves for, is the accepted MNA solution (every node
//! voltage and branch current, from which a segment takes the native
//! reactive histories exactly as a run takes them from its starting point),
//! followed by each runtime Verilog-A instance's integrator states (`ddt`
//! charges, `idt` outputs) and its real-valued retained variables. `σ`, the
//! combinatorial state the iteration holds fixed, is everything else the
//! image carries; it is compared through a [`DiscreteFingerprint`]: switch
//! states, the integer-valued retained and event-written Verilog-A variables
//! and the sides of their `cross` detectors.
//!
//! A retained Verilog-A variable whose accepted value is an exact integer is
//! discrete: a level an event block set (an output state, a count, a flag).

use super::EVENT_BASIS_PREFIX;
use crate::engine::SimulationError;
use crate::engine::transient::state_capture::CapturedTransientState;
use crate::{CircuitData, Value};

/// What a coordinate measures, for its tolerances.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CoordinateUnit {
    Volt,
    Ampere,
    /// A Verilog-A state or variable, in the units its module gives it.
    Model,
}

impl CoordinateUnit {
    /// The scale a coordinate's finite-difference step is measured against.
    pub(super) fn scale(self, value: Value) -> Value {
        match self {
            Self::Volt => value.abs().max(1.0),
            Self::Ampere => value.abs().max(1.0e-3),
            Self::Model => value.abs().max(1.0e-12),
        }
    }
}

/// What an accepted point holds of the discrete state and of the events
/// around it, read without changing the circuit.
#[derive(Clone, Debug)]
pub(super) struct Probe {
    pub(super) time: Value,
    /// On-state of every voltage and then current switch.
    switches: Vec<u64>,
    /// Every runtime Verilog-A instance's retained variables, in device order.
    retained: Vec<Value>,
    /// Every `cross` detector's expression value and side, in device order.
    pub(super) crossings: Vec<(Value, i8)>,
    /// A Verilog-A instance with a `transition` or `slew` ramp in flight.
    pub(super) ramping: Option<String>,
}

#[cfg(test)]
impl Probe {
    /// A point with no discrete state and nothing pending.
    pub(super) fn quiet(time: Value) -> Self {
        Self {
            time,
            switches: Vec::new(),
            retained: Vec::new(),
            crossings: Vec::new(),
            ramping: None,
        }
    }
}

/// One runtime Verilog-A instance's place in `z`.
#[cfg(feature = "veriloga")]
#[derive(Clone, Debug)]
struct ModelCoordinates {
    /// Index among the circuit's runtime Verilog-A devices.
    device: usize,
    /// Integrator state slots in `z`.
    states: Vec<usize>,
    /// Real-valued retained variable slots in `z`.
    reals: Vec<usize>,
}

/// The coordinate layout of one orbit, fixed at its boundary image.
#[derive(Clone, Debug)]
pub(super) struct ShootingBasis {
    unknowns: usize,
    #[cfg(feature = "veriloga")]
    models: Vec<ModelCoordinates>,
    names: Vec<String>,
    units: Vec<CoordinateUnit>,
    /// How many switches lead the discrete fingerprint.
    switches: usize,
    /// Indices into [`Probe::retained`] of the discrete retained variables.
    discrete_retained: Vec<usize>,
    discrete_names: Vec<String>,
}

/// The discrete state of an accepted point, as comparable values named by
/// [`ShootingBasis::discrete_name`].
#[derive(Clone, Debug, PartialEq)]
pub(super) struct DiscreteFingerprint {
    pub(super) values: Vec<u64>,
}

impl DiscreteFingerprint {
    /// The first discrete quantity two fingerprints disagree on.
    pub(super) fn first_difference(&self, other: &Self) -> Option<usize> {
        self.values
            .iter()
            .zip(&other.values)
            .position(|(left, right)| left != right)
            .or_else(|| (self.values.len() != other.values.len()).then_some(0))
    }
}

/// Read the discrete state and the event context of `circuit` at `time`.
pub(super) fn probe(time: Value, circuit: &CircuitData) -> Result<Probe, SimulationError> {
    let mut switches = Vec::with_capacity(circuit.vswitches.len() + circuit.iswitches.len());
    for switch in &circuit.vswitches {
        switches.push(switch.state() as u64);
    }
    for switch in &circuit.iswitches {
        switches.push(switch.state() as u64);
    }
    let mut probe = Probe {
        time,
        switches,
        retained: Vec::new(),
        crossings: Vec::new(),
        ramping: None,
    };
    probe_models(time, circuit, &mut probe)?;
    Ok(probe)
}

#[cfg(feature = "veriloga")]
fn probe_models(
    time: Value,
    circuit: &CircuitData,
    probe: &mut Probe,
) -> Result<(), SimulationError> {
    if circuit.veriloga_devices().is_empty() {
        return Ok(());
    }
    let states = circuit
        .runtime_veriloga_checkpoint_states()
        .map_err(SimulationError::Circuit)?;
    for (checkpoint, live) in states.iter().zip(circuit.veriloga_devices().iter()) {
        let accepted = &checkpoint.accepted;
        for slot in retained_slots(live) {
            probe
                .retained
                .push(accepted.variables.get(slot).copied().unwrap_or(Value::NAN));
        }
        for detector in &accepted.cross_detectors {
            probe.crossings.push((detector.value, detector.side));
        }
        let ramping = accepted.transition_filters.iter().any(|filter| {
            filter
                .active
                .as_ref()
                .is_some_and(|segment| segment.end_time > time)
                || !filter.pending.is_empty()
        }) || accepted
            .slew_filters
            .iter()
            .any(|filter| filter.next_corner_time.is_some_and(|corner| corner > time));
        if ramping && probe.ramping.is_none() {
            probe.ramping = Some(live.name.to_string());
        }
    }
    Ok(())
}

#[cfg(not(feature = "veriloga"))]
fn probe_models(
    _time: Value,
    _circuit: &CircuitData,
    _probe: &mut Probe,
) -> Result<(), SimulationError> {
    Ok(())
}

/// The variable slots of a Verilog-A instance that keep their value from the
/// last accepted point, in slot order: the inputs some path reads before
/// assigning them, and the variables an event block or `@(initial_step)`
/// writes (a toggle an event flips is accepted state even where no path
/// reads it first).
#[cfg(feature = "veriloga")]
fn retained_slots(device: &crate::device::veriloga::VerilogADevice) -> Vec<usize> {
    let retained = device.retained_variables();
    device
        .variables()
        .enumerate()
        .filter(|(slot, (name, _))| {
            let base = name.split_once('[').map_or(*name, |(base, _)| base);
            retained.contains(&base) || device.discrete_state_value(*slot).is_some()
        })
        .map(|(slot, _)| slot)
        .collect()
}

impl ShootingBasis {
    /// Lay out the coordinates of `image`.
    pub(super) fn of(image: &CapturedTransientState) -> Result<Self, SimulationError> {
        let circuit = &image.circuit;
        let unknowns = image.solution.len();
        let num_nodes = circuit.num_nodes();
        let node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();
        let mut names = Vec::with_capacity(unknowns);
        let mut units = Vec::with_capacity(unknowns);
        for index in 0..unknowns {
            if index < num_nodes {
                let node = node_names
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| format!("node {}", index + 1));
                names.push(format!("{EVENT_BASIS_PREFIX}V({node})"));
                units.push(CoordinateUnit::Volt);
            } else {
                let branch = index - num_nodes;
                let name = branch_names
                    .get(branch)
                    .cloned()
                    .unwrap_or_else(|| format!("branch {}", branch + 1));
                names.push(format!("{EVENT_BASIS_PREFIX}I({name})"));
                units.push(CoordinateUnit::Ampere);
            }
        }
        let discrete_names = circuit
            .vswitches
            .iter()
            .map(|switch| format!("switch {}", switch.name))
            .chain(
                circuit
                    .iswitches
                    .iter()
                    .map(|switch| format!("switch {}", switch.name)),
            )
            .collect();
        let mut basis = Self {
            unknowns,
            #[cfg(feature = "veriloga")]
            models: Vec::new(),
            names,
            units,
            switches: circuit.vswitches.len() + circuit.iswitches.len(),
            discrete_retained: Vec::new(),
            discrete_names,
        };
        basis.lay_out_models(circuit)?;
        Ok(basis)
    }

    #[cfg(feature = "veriloga")]
    fn lay_out_models(&mut self, circuit: &CircuitData) -> Result<(), SimulationError> {
        if circuit.veriloga_devices().is_empty() {
            return Ok(());
        }
        let states = circuit
            .runtime_veriloga_checkpoint_states()
            .map_err(SimulationError::Circuit)?;
        let mut flat = 0;
        let mut cross_names = Vec::new();
        for (device, (checkpoint, live)) in states
            .iter()
            .zip(circuit.veriloga_devices().iter())
            .enumerate()
        {
            let instance = live.name.to_string();
            let accepted = &checkpoint.accepted;
            let mut coordinates = ModelCoordinates {
                device,
                states: Vec::new(),
                reals: Vec::new(),
            };
            for (slot, initialized) in accepted.state_initialized.iter().enumerate() {
                if *initialized {
                    coordinates.states.push(slot);
                    self.names
                        .push(format!("{EVENT_BASIS_PREFIX}{instance}: state {slot}"));
                    self.units.push(CoordinateUnit::Model);
                }
            }
            let variable_names: Vec<&str> = live.variables().map(|(name, _)| name).collect();
            for slot in retained_slots(live) {
                let name = variable_names.get(slot).copied().unwrap_or("?");
                let value = accepted.variables.get(slot).copied().unwrap_or(Value::NAN);
                if value.is_finite() && value.fract() == 0.0 && value.abs() < 9.007e15 {
                    self.discrete_retained.push(flat);
                    self.discrete_names.push(format!("{instance}.{name}"));
                } else {
                    coordinates.reals.push(slot);
                    self.names
                        .push(format!("{EVENT_BASIS_PREFIX}{instance}.{name}"));
                    self.units.push(CoordinateUnit::Model);
                }
                flat += 1;
            }
            for index in 0..accepted.cross_detectors.len() {
                cross_names.push(format!("{instance}: cross detector {index}"));
            }
            self.models.push(coordinates);
        }
        self.discrete_names.extend(cross_names);
        Ok(())
    }

    #[cfg(not(feature = "veriloga"))]
    fn lay_out_models(&mut self, _circuit: &CircuitData) -> Result<(), SimulationError> {
        Ok(())
    }

    /// How many coordinates `z` has.
    pub(super) fn dimension(&self) -> usize {
        self.names.len()
    }

    pub(super) fn name(&self, index: usize) -> &str {
        self.names
            .get(index)
            .map_or("?", |name| name.trim_start_matches(EVENT_BASIS_PREFIX))
    }

    /// Every coordinate's retained name, prefixed so a retained operating
    /// point names the engine whose coordinates they are.
    pub(super) fn names(&self) -> &[String] {
        &self.names
    }

    pub(super) fn unit(&self, index: usize) -> CoordinateUnit {
        self.units
            .get(index)
            .copied()
            .unwrap_or(CoordinateUnit::Model)
    }

    pub(super) fn discrete_name(&self, index: usize) -> &str {
        self.discrete_names.get(index).map_or("?", String::as_str)
    }

    /// Whether a discrete quantity changes with the analog state: a switch or
    /// a crossing detector. A retained variable an event block writes follows
    /// the event that wrote it.
    pub(super) fn is_state_dependent(&self, quantity: usize) -> bool {
        quantity < self.switches || quantity >= self.switches + self.discrete_retained.len()
    }

    /// Where the crossing detectors start in the fingerprint.
    pub(super) fn crossing_offset(&self) -> usize {
        self.switches + self.discrete_retained.len()
    }

    /// The discrete state a probe read.
    pub(super) fn fingerprint(&self, probe: &Probe) -> DiscreteFingerprint {
        let mut values = Vec::with_capacity(self.discrete_names.len());
        values.extend_from_slice(&probe.switches);
        values.extend(self.discrete_retained.iter().map(|&index| {
            probe
                .retained
                .get(index)
                .copied()
                .unwrap_or(Value::NAN)
                .to_bits()
        }));
        values.extend(
            probe
                .crossings
                .iter()
                .map(|&(_, side)| u64::from(side as u8)),
        );
        DiscreteFingerprint { values }
    }

    /// The coordinates `z` of `image`.
    pub(super) fn read(
        &self,
        image: &CapturedTransientState,
    ) -> Result<Vec<Value>, SimulationError> {
        if image.solution.len() != self.unknowns {
            return Err(SimulationError::Circuit(format!(
                "a shooting image holds {} unknowns where its orbit has {}",
                image.solution.len(),
                self.unknowns
            )));
        }
        let mut z = Vec::with_capacity(self.dimension());
        z.extend_from_slice(&image.solution);
        self.read_models(&image.circuit, &mut z)?;
        if z.len() != self.dimension() {
            return Err(SimulationError::Circuit(
                "a shooting image's model state changed shape over the period".to_string(),
            ));
        }
        Ok(z)
    }

    #[cfg(feature = "veriloga")]
    fn read_models(
        &self,
        circuit: &CircuitData,
        z: &mut Vec<Value>,
    ) -> Result<(), SimulationError> {
        if self.models.is_empty() {
            return Ok(());
        }
        let states = circuit
            .runtime_veriloga_checkpoint_states()
            .map_err(SimulationError::Circuit)?;
        for model in &self.models {
            let accepted = &states
                .get(model.device)
                .ok_or_else(|| {
                    SimulationError::Circuit("a Verilog-A instance left the orbit".to_string())
                })?
                .accepted;
            for &slot in &model.states {
                z.push(accepted.state_values_prev.get(slot).copied().unwrap_or(0.0));
            }
            for &slot in &model.reals {
                z.push(accepted.variables.get(slot).copied().unwrap_or(0.0));
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "veriloga"))]
    fn read_models(
        &self,
        _circuit: &CircuitData,
        _z: &mut Vec<Value>,
    ) -> Result<(), SimulationError> {
        Ok(())
    }

    /// `image` with its coordinates replaced by `z`; its discrete state is
    /// kept.
    pub(super) fn write(
        &self,
        image: &CapturedTransientState,
        z: &[Value],
    ) -> Result<CapturedTransientState, SimulationError> {
        if z.len() != self.dimension() || z.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "a shooting iterate is not a finite point of its orbit's coordinates".to_string(),
            ));
        }
        let mut written = image.clone();
        written.solution.copy_from_slice(&z[..self.unknowns]);
        self.write_models(&mut written.circuit, &z[self.unknowns..])?;
        Ok(written)
    }

    #[cfg(feature = "veriloga")]
    fn write_models(&self, circuit: &mut CircuitData, z: &[Value]) -> Result<(), SimulationError> {
        if self.models.is_empty() {
            return Ok(());
        }
        let mut states = circuit
            .runtime_veriloga_checkpoint_states()
            .map_err(SimulationError::Circuit)?;
        let mut cursor = 0;
        for model in &self.models {
            let accepted = &mut states
                .get_mut(model.device)
                .ok_or_else(|| {
                    SimulationError::Circuit("a Verilog-A instance left the orbit".to_string())
                })?
                .accepted;
            for &slot in &model.states {
                if let Some(value) = accepted.state_values_prev.get_mut(slot) {
                    *value = z[cursor];
                }
                cursor += 1;
            }
            for &slot in &model.reals {
                if let Some(value) = accepted.variables.get_mut(slot) {
                    *value = z[cursor];
                }
                cursor += 1;
            }
        }
        circuit
            .restore_runtime_veriloga_checkpoint_states(&states)
            .map_err(SimulationError::Circuit)
    }

    #[cfg(not(feature = "veriloga"))]
    fn write_models(
        &self,
        _circuit: &mut CircuitData,
        _z: &[Value],
    ) -> Result<(), SimulationError> {
        Ok(())
    }
}

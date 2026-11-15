//! A Verilog-A module's stored flux as a shooting coordinate.
//!
//! A voltage contribution that takes the time derivative of its own branch
//! current (an inductance, `V(p, n) <+ ddt(L·I(p, n))`) stores flux, and its
//! state is that current: no node voltage or charge at the period start fixes
//! it. Each such branch's current is therefore a coordinate after the
//! generated integrals, as a native inductor's winding current is a physical
//! one, for a generated model and a runtime module alike. A period starts
//! with the current pinned at its coordinate: the consistency solve frees the
//! branch's own equation with an auxiliary unknown and holds the current by a
//! row of its own, as it holds a charge's voltage by a source, so the branch
//! voltage it solves is the flux's rate there; the seed that starts the
//! model's history reads the current from that solution. The steps then
//! integrate the flux by the model's own companion rule, and the coordinate's
//! image is the current the last step accepted.
use super::*;

#[derive(Debug, Clone, Default)]
pub(in crate::engine::pss) struct PssFluxes {
    /// The zero-based solution column of each branch current that stores
    /// flux, generated models first, each model's in its own order. A
    /// branch's current and its equation share the index.
    columns: Vec<usize>,
    /// Each coordinate's name, as the shooting basis reports it.
    names: Vec<String>,
    /// The consistency solve's pins, one per branch: the zero-based row of
    /// the auxiliary unknown that frees the branch's equation, and the
    /// current it holds.
    pins: Vec<(usize, Value)>,
}

impl PssFluxes {
    pub(super) fn new(circuit: &CircuitData) -> Self {
        let (columns, names) = generated_fluxes(circuit)
            .into_iter()
            .chain(runtime_fluxes(circuit))
            .unzip();
        Self {
            columns,
            names,
            pins: Vec::new(),
        }
    }

    pub(in crate::engine::pss) fn dimension(&self) -> usize {
        self.columns.len()
    }

    pub(super) fn names(&self) -> impl Iterator<Item = String> + '_ {
        self.names.iter().cloned()
    }

    /// Each carried current in `solution`, a zero-based circuit solution.
    pub(super) fn extract<'a>(&'a self, solution: &'a [Value]) -> impl Iterator<Item = Value> + 'a {
        self.columns.iter().map(move |&column| solution[column])
    }

    /// Put each carried current at its coordinate in `solution`, a zero-based
    /// circuit solution.
    pub(super) fn set(&self, solution: &mut [Value], values: &[Value]) {
        for (&column, &value) in self.columns.iter().zip(values) {
            solution[column] = value;
        }
    }

    /// Give the consistency solve one auxiliary unknown per carried branch,
    /// each holding the current `solution` (zero-based) has there.
    pub(super) fn pin(&mut self, circuit: &mut CircuitData, solution: &[Value]) {
        let num_nodes = circuit.num_nodes();
        self.pins = self
            .columns
            .iter()
            .map(|&column| (num_nodes + circuit.allocate_branch() - 1, solution[column]))
            .collect();
    }

    /// Each pin's zero-based rows: the branch it frees and its own.
    fn pin_rows(&self) -> impl Iterator<Item = (usize, usize, Value)> + '_ {
        self.columns
            .iter()
            .zip(&self.pins)
            .map(|(&branch, &(pin, current))| (branch, pin, current))
    }

    pub(super) fn pin_pattern(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.pin_rows()
            .flat_map(|(branch, pin, _)| [(branch, pin), (pin, branch)])
    }

    /// Free each carried branch's equation by its pin's unknown, and hold the
    /// branch's current at its coordinate by the pin's row.
    pub(super) fn stamp_pins(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for (branch, pin, current) in self.pin_rows() {
            matrix.add(branch, pin, 1.0);
            matrix.add(pin, branch, 1.0);
            rhs[pin] = current;
        }
    }

    /// Whether zero-based `row` is a pin's, whose equation holds a current
    /// and whose unknown is a voltage the solve's residual certifies.
    pub(super) fn is_pin(&self, row: usize) -> bool {
        self.pins.iter().any(|&(pin, _)| row == pin)
    }
}

/// Each generated model's source branch that stores flux at its parameters,
/// by its zero-based solution column and its coordinate's name.
fn generated_fluxes(circuit: &CircuitData) -> Vec<(usize, String)> {
    #[cfg(feature = "veriloga-builtins-base")]
    if circuit.has_generated_veriloga_devices() {
        let num_nodes = circuit.num_nodes();
        let names = circuit.branch_names_sorted();
        return circuit
            .generated_veriloga_devices()
            .iter()
            .flat_map(|device| {
                device.flux_storing_branches(num_nodes, circuit.generated_simulation_parameters)
            })
            .map(|branch| (num_nodes + branch - 1, format!("I({})", names[branch - 1])))
            .collect();
    }
    let _ = circuit;
    Vec::new()
}

/// Each runtime module's branch current that a `ddt` operand reads, by its
/// zero-based solution column and its coordinate's name: the builder
/// allocated it as a node of its own, by the branch's name.
fn runtime_fluxes(circuit: &CircuitData) -> Vec<(usize, String)> {
    #[cfg(feature = "veriloga")]
    if circuit.has_veriloga_devices() {
        return circuit
            .veriloga_devices()
            .iter()
            .flat_map(|device| {
                device
                    .flux_branch_ordinals()
                    .into_iter()
                    .filter_map(move |ordinal| device.branch_current_index(ordinal))
                    .filter(|&node| node > 0)
            })
            .map(|node| {
                let name = circuit.node_name_by_id(node).unwrap_or_default();
                (node - 1, format!("I({name})"))
            })
            .collect();
    }
    let _ = circuit;
    Vec::new()
}

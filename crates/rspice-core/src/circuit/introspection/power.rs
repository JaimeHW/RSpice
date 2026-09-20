//! Conductive device power from the electrical F contribution at the accepted
//! solution. Q/dt (capacitor storage and release) is deliberately absent.
//!
//! Include every electrical prime node in sum(V * F), and separately add the
//! losses of explicitly owned, builder-lowered resistors. This is the device's
//! modeled conductive power, including its junction leakage/GMIN. It is not a
//! claim that the model includes every physical loss or a junction thermal model.

use super::*;
use crate::device::NonlinearDevice;

#[derive(Default)]
struct Sum {
    value: Value,
    correction: Value,
}
impl Sum {
    fn add(&mut self, value: Value) {
        let next = self.value + value;
        self.correction += if self.value.abs() >= value.abs() {
            (self.value - next) + value
        } else {
            (value - next) + self.value
        };
        self.value = next;
    }
    fn total(&self) -> Value {
        self.value + self.correction
    }
}

struct PowerStamp<'a> {
    solution: &'a [Value],
    nodes: usize,
    excluded: &'a [NodeId],
    rows: Vec<(NodeId, Sum)>,
    invalid: bool,
}
impl PowerStamp<'_> {
    fn add_current(&mut self, row: NodeId, current: Value) {
        // Thermal, excess-phase and charge-deficit coordinates are not volts;
        // branch-equation rows beyond the nodal prefix are not electrical KCL.
        if row == 0 || row > self.nodes || self.excluded.contains(&row) {
            return;
        }
        self.invalid |= !current.is_finite();
        if let Some((_, sum)) = self.rows.iter_mut().find(|(node, _)| *node == row) {
            sum.add(current);
        } else {
            let mut sum = Sum::default();
            sum.add(current);
            self.rows.push((row, sum));
        }
    }
    fn power(self) -> Result<Value, String> {
        let mut power = Sum::default();
        for (node, current) in self.rows {
            let voltage = self
                .solution
                .get(node - 1)
                .ok_or("power observation node outside solution")?;
            power.add(voltage * current.total());
        }
        let power = power.total();
        if self.invalid || !power.is_finite() {
            Err("non-finite conductive power observation".into())
        } else {
            Ok(power)
        }
    }
}
impl MatrixStamper for PowerStamp<'_> {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row == 0
            || col == 0
            || value == 0.0
            || (col > self.nodes && self.excluded.contains(&col))
        {
            return;
        }
        if let Some(voltage) = self.solution.get(col - 1) {
            self.add_current(row, value * voltage);
        } else {
            self.invalid = true;
        }
    }
    fn stamp_rhs(&mut self, row: NodeId, value: Value) {
        self.add_current(row, -value);
    }
}

fn conductive_power(
    device: &(impl NonlinearDevice + Clone),
    solution: &[Value],
    nodes: usize,
    excluded: &[NodeId],
) -> Result<Value, String> {
    let mut stamp = PowerStamp {
        solution,
        nodes,
        excluded,
        rows: Vec::new(),
        invalid: false,
    };
    // Native stamps can consume limiter/startup cache flags through interior
    // mutability. Observe an isolated copy so reporting cannot change the next
    // Newton iterate. These admitted native implementations write RHS through
    // MatrixStamper; none uses the legacy separate RHS argument.
    device
        .clone()
        .stamp_nonlinear(solution, &mut stamp, &mut []);
    stamp.power()
}

// Instance-IC voltage constraints are artificial startup sources, not device
// dissipation. Exclude their current columns as well as their non-KCL rows.
fn soi_exclusions(
    temp: NodeId,
    ic: &crate::device::mosfet::b3soi::common::B3SoiInstanceIc,
) -> Vec<NodeId> {
    std::iter::once(temp)
        .chain(
            ic.constraints()
                .iter()
                .flatten()
                .map(|c| c.branch_matrix_index()),
        )
        .collect()
}

impl CircuitData {
    pub(super) fn add_conductive_power_observations(
        &self,
        report: &mut DeviceOpReport,
        solution: &[Value],
    ) -> Result<(), String> {
        let mut powers = HashMap::<&str, Value>::new();
        // Static F stamps include built-in nonlinear resistors and omit charge
        // companions. Explicit exclusions are the native model's non-electrical
        // coordinates; they may control electrical rows, but contribute no V*I.
        macro_rules! sample {
            ($devices:expr, $device:ident, $excluded:expr) => {
                for $device in $devices {
                    let value = conductive_power($device, solution, self.num_nodes(), &$excluded)
                        .map_err(|error| format!("device '{}': {error}", $device.name))?;
                    powers.insert(&$device.name, value);
                }
            };
        }
        sample!(&self.mosfets.devices, d, []);
        sample!(&self.bsim3v3.devices, d, [d.node_charge_deficit]);
        sample!(&self.bsim4v8.devices, d, [d.node_charge_deficit]);
        sample!(
            &self.b3soi_fd.devices,
            d,
            soi_exclusions(d.node_temp, d.instance_ic())
        );
        sample!(
            &self.b3soi.devices,
            d,
            soi_exclusions(d.node_temp, d.instance_ic())
        );
        sample!(
            &self.b3soi_pd.devices,
            d,
            soi_exclusions(d.node_temp, d.instance_ic())
        );
        sample!(&self.ekv26s.devices, d, []);
        sample!(&self.ekv3s.devices, d, []);
        sample!(&self.vdmoses.devices, d, []);
        sample!(&self.bjts.devices, d, [d.node_rth, d.node_xf1, d.node_xf2]);
        sample!(&self.jfets, d, []);
        sample!(&self.diodes.devices, d, []);
        for (index, owner) in self.resistors.device_owners.iter().enumerate() {
            let Some(power) = owner.as_deref().and_then(|owner| powers.get_mut(owner)) else {
                continue;
            };
            let stamp = &self.resistors.stamps[index];
            let voltage = |node| {
                solution_node_voltage(solution, node)
                    .ok_or("device series resistor node outside solution")
            };
            let drop = voltage(stamp.pp.row)? - voltage(stamp.nn.row)?;
            *power += self.resistors.output_conductance(index) * drop * drop;
        }
        for entry in &mut report.entries {
            let Some(power) = powers.remove(entry.name.as_str()) else {
                continue;
            };
            if !power.is_finite() {
                return Err(format!("device '{}': non-finite series loss", entry.name));
            }
            if let Some((_, value)) = entry
                .params
                .iter_mut()
                .find(|(label, _)| *label == OpLabel::POWER.as_str())
            {
                *value = power;
            } else {
                entry.params.push((OpLabel::POWER.as_str(), power));
            }
        }
        if let Some(name) = powers.keys().next() {
            return Err(format!("device '{name}' is missing from power report"));
        }
        Ok(())
    }
}

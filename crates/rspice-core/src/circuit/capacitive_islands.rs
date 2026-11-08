//! The operating point of a capacitive island.
//!
//! A node set that no DC-conducting element ties to ground, joined to the rest
//! of the circuit only through capacitors, has no DC equation for its common
//! mode, and the nodal conditioning shunt would hold it at 0 V. Its physical
//! operating point is the one its capacitors reach charging up from rest: the
//! island's net charge is zero, so a 1 V source across two equal capacitors in
//! series puts 0.5 V on the node between them, as QSPICE, Xyce and ngspice
//! give it.

use crate::solver::StaticMatrix;
use crate::{CircuitData, Value};

/// A linear capacitor with exactly one plate on an island: the island, the
/// matrix row of that plate, the row of the far plate (`None` for ground) and
/// the capacitance.
#[derive(Clone, Copy)]
struct IslandCoupling {
    island: usize,
    near: usize,
    far: Option<usize>,
    capacitance: Value,
}

impl CircuitData {
    /// Raise the nodal floor `gmin` on a floating group its own resistors
    /// would round it away on.
    ///
    /// Against a 1 mOhm resistor's 1000 S, 1e-15 S is lost in a double: the
    /// group's rows cancel exactly and a linear circuit is refused as
    /// singular. Only this floor sets the common mode of such a group and no
    /// current leaves it through anything else, so the larger floor moves no
    /// other node, and its charge is settled afterwards as for any island.
    pub(crate) fn stamp_resistive_island_floor(&self, matrix: &mut StaticMatrix, gmin: Value) {
        const ROUNDING_MARGIN: Value = 16.0 * Value::EPSILON;
        if self.dc_floating_component_nodes.is_empty() {
            return;
        }
        let mut largest = vec![0.0_f64; self.dc_floating_component_nodes.len()];
        let by_node = &self.dc_floating_component_by_node;
        let resistors = &self.resistors;
        for (stamp, conductance) in resistors.stamps.iter().zip(&resistors.conductances) {
            if let Some(&Some(component)) = by_node.get(stamp.pp.row) {
                largest[component] = largest[component].max(conductance.abs());
            }
        }
        for (node, component) in by_node.iter().enumerate().skip(1) {
            let Some(component) = *component else {
                continue;
            };
            let raised = ROUNDING_MARGIN * largest[component] - gmin;
            let qualifies = |flags: &[bool]| flags.get(component).copied().unwrap_or(false);
            if raised > 0.0
                && qualifies(&self.dc_floating_component_is_certain)
                && qualifies(&self.dc_floating_component_is_passive)
                && self.receives_nodal_gmin(node - 1)
            {
                matrix.add(node - 1, node - 1, raised);
            }
        }
    }

    /// Shift the common mode of every capacitive island so its net charge is
    /// zero, leaving every other node as solved.
    ///
    /// An island is a certain floating component that only resistors and
    /// linear capacitors touch, that no other matrix entry reads, and that no
    /// row in `held_rows` (an initial condition's) belongs to. Its resistors
    /// carry no current, so the shift changes no current anywhere. Islands
    /// coupled to one another through capacitors are settled together; a
    /// group with no capacitive path to a solved node keeps its voltages.
    pub(crate) fn settle_capacitive_islands(
        &self,
        matrix: &StaticMatrix,
        solution: &mut [Value],
        held_rows: &[usize],
    ) {
        let node_count = self.num_nodes().min(solution.len());
        let mut island_of_row: Vec<Option<usize>> = vec![None; node_count];
        let mut islands: Vec<Vec<usize>> = Vec::new();
        for component in 0..self.dc_floating_component_nodes.len() {
            let qualifies = self
                .dc_floating_component_is_certain
                .get(component)
                .copied()
                .unwrap_or(false)
                && self
                    .dc_floating_component_is_passive
                    .get(component)
                    .copied()
                    .unwrap_or(false);
            if !qualifies {
                continue;
            }
            let rows = self.dc_floating_component_matrix_rows(component);
            if rows.is_empty()
                || rows
                    .iter()
                    .any(|row| *row >= node_count || held_rows.contains(row))
            {
                continue;
            }
            for &row in &rows {
                island_of_row[row] = Some(islands.len());
            }
            islands.push(rows);
        }
        if islands.is_empty() {
            return;
        }
        let island_at = |row: Option<usize>| row.and_then(|row| island_of_row.get(row).copied()?);

        let mut eligible = vec![true; islands.len()];
        let mut couplings = Vec::new();
        let capacitors = &self.capacitors;
        for (index, stamp) in capacitors.stamps.iter().enumerate() {
            let positive = stamp.pp.row.checked_sub(1);
            let negative = stamp.nn.row.checked_sub(1);
            let (positive_island, negative_island) = (island_at(positive), island_at(negative));
            if positive_island.is_none() && negative_island.is_none() {
                continue;
            }
            let capacitance = capacitors.capacitances[index];
            let linear = capacitors.value_expressions[index].is_none()
                && capacitors.ic_branch_indices[index].is_none()
                && capacitance.is_finite()
                && capacitance > 0.0;
            if !linear {
                for island in [positive_island, negative_island].into_iter().flatten() {
                    eligible[island] = false;
                }
                continue;
            }
            if positive_island == negative_island {
                // Both plates on one island: their charges cancel.
                continue;
            }
            for (island, near, far) in [
                (positive_island, positive, negative),
                (negative_island, negative, positive),
            ] {
                if let (Some(island), Some(near)) = (island, near) {
                    couplings.push(IslandCoupling {
                        island,
                        near,
                        far,
                        capacitance,
                    });
                }
            }
        }

        // Anything else that reads an island voltage, a controlled source's
        // control port or a behavioral expression say, would see the shift.
        for (row, column) in matrix.stored_positions() {
            let Some(island) = island_of_row.get(column).copied().flatten() else {
                continue;
            };
            if !eligible[island] || island_of_row.get(row).copied().flatten() == Some(island) {
                continue;
            }
            let capacitor_entry = couplings.iter().any(|coupling| {
                coupling.island == island && coupling.near == column && coupling.far == Some(row)
            });
            if !capacitor_entry {
                eligible[island] = false;
            }
        }

        let unknowns: Vec<usize> = (0..islands.len())
            .filter(|&island| {
                eligible[island] && couplings.iter().any(|coupling| coupling.island == island)
            })
            .collect();
        for group in coupled_groups(&unknowns, &couplings, &island_at) {
            let size = group.len();
            let position = |island: usize| group.iter().position(|&member| member == island);
            let mut system = vec![0.0; size * size];
            let mut charge = vec![0.0; size];
            for coupling in &couplings {
                let Some(equation) = position(coupling.island) else {
                    continue;
                };
                let far_voltage = coupling.far.map_or(0.0, |row| solution[row]);
                system[equation * size + equation] += coupling.capacitance;
                charge[equation] -= coupling.capacitance * (solution[coupling.near] - far_voltage);
                if let Some(far) = island_at(coupling.far).and_then(position) {
                    system[equation * size + far] -= coupling.capacitance;
                }
            }
            let Some(shift) = solve_dense(size, system, charge) else {
                continue;
            };
            for (member, delta) in group.iter().zip(shift) {
                for &row in &islands[*member] {
                    solution[row] += delta;
                }
            }
        }
    }
}

/// Partition `unknowns` into the groups that capacitors join to one another.
fn coupled_groups(
    unknowns: &[usize],
    couplings: &[IslandCoupling],
    island_at: &dyn Fn(Option<usize>) -> Option<usize>,
) -> Vec<Vec<usize>> {
    let mut group_of: Vec<usize> = (0..unknowns.len()).collect();
    fn root(group_of: &mut [usize], mut member: usize) -> usize {
        while group_of[member] != member {
            group_of[member] = group_of[group_of[member]];
            member = group_of[member];
        }
        member
    }
    for coupling in couplings {
        let near = unknowns
            .iter()
            .position(|&island| island == coupling.island);
        let far = island_at(coupling.far)
            .and_then(|island| unknowns.iter().position(|&candidate| candidate == island));
        if let (Some(near), Some(far)) = (near, far) {
            let (near, far) = (root(&mut group_of, near), root(&mut group_of, far));
            group_of[near] = far;
        }
    }
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut group_index: Vec<Option<usize>> = vec![None; unknowns.len()];
    for (member, &island) in unknowns.iter().enumerate() {
        let leader = root(&mut group_of, member);
        let index = *group_index[leader].get_or_insert_with(|| {
            groups.push(Vec::new());
            groups.len() - 1
        });
        groups[index].push(island);
    }
    groups
}

/// Solve the dense `size` x `size` system by elimination with partial
/// pivoting; `None` when it is singular, as a group of islands with no
/// capacitive path to a solved node is.
fn solve_dense(size: usize, mut system: Vec<Value>, mut rhs: Vec<Value>) -> Option<Vec<Value>> {
    let scale = system
        .iter()
        .fold(0.0_f64, |largest, value| largest.max(value.abs()));
    if !(scale.is_finite() && scale > 0.0) {
        return None;
    }
    for pivot in 0..size {
        let best = (pivot..size).max_by(|&left, &right| {
            system[left * size + pivot]
                .abs()
                .total_cmp(&system[right * size + pivot].abs())
        })?;
        if system[best * size + pivot].abs() <= 1.0e-12 * scale {
            return None;
        }
        if best != pivot {
            for column in 0..size {
                system.swap(pivot * size + column, best * size + column);
            }
            rhs.swap(pivot, best);
        }
        for row in pivot + 1..size {
            let factor = system[row * size + pivot] / system[pivot * size + pivot];
            if factor == 0.0 {
                continue;
            }
            for column in pivot..size {
                system[row * size + column] -= factor * system[pivot * size + column];
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }
    let mut solution = vec![0.0; size];
    for row in (0..size).rev() {
        let tail: Value = (row + 1..size)
            .map(|column| system[row * size + column] * solution[column])
            .sum();
        solution[row] = (rhs[row] - tail) / system[row * size + row];
    }
    solution
        .iter()
        .all(|value| value.is_finite())
        .then_some(solution)
}

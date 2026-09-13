//! Shared small-signal winding stamps for Xyce LEVEL=2 magnetic cores.

use crate::circuit::{CircuitData, XyceCoreWindingBinding};
use crate::device::passive::JilesAthertonInductor;
use crate::solver::{ComplexMatrix, SolverError};
use crate::{NodeId, Value};

impl CircuitData {
    pub(crate) fn xyce_level2_owns_inductor(&self, index: usize) -> bool {
        self.jiles_atherton_inductors
            .iter()
            .any(|binding| binding.inductor_index == index && binding.device.is_xyce_core_level2())
            || self.xyce_core_groups.iter().any(|group| {
                group.device.is_xyce_core_level2()
                    && group
                        .windings
                        .iter()
                        .any(|winding| winding.inductor_index == index)
            })
    }

    pub(crate) fn xyce_level2_owns_mutual_pair(&self, first: NodeId, second: NodeId) -> bool {
        self.xyce_core_groups.iter().any(|group| {
            group.device.is_xyce_core_level2()
                && [first, second].iter().all(|branch| {
                    group.windings.iter().any(|winding| {
                        self.inductors.branch_indices[winding.inductor_index] == *branch
                    })
                })
        })
    }

    pub(crate) fn stamp_xyce_level2_small_signal(
        &self,
        matrix: &mut ComplexMatrix,
        operating_point: &[Value],
        omega: Value,
    ) -> Result<(), SolverError> {
        for binding in &self.jiles_atherton_inductors {
            if binding.device.is_xyce_core_level2() {
                let winding = XyceCoreWindingBinding {
                    inductor_index: binding.inductor_index,
                    turns: binding.device.n_turns_for_xyce_core(),
                };
                self.stamp_xyce_level2_windings(
                    matrix,
                    operating_point,
                    omega,
                    &binding.device,
                    &[winding],
                )?;
            }
        }
        for group in &self.xyce_core_groups {
            if group.device.is_xyce_core_level2() {
                self.stamp_xyce_level2_windings(
                    matrix,
                    operating_point,
                    omega,
                    &group.device,
                    &group.windings,
                )?;
            }
        }
        Ok(())
    }

    fn stamp_xyce_level2_windings(
        &self,
        matrix: &mut ComplexMatrix,
        operating_point: &[Value],
        omega: Value,
        device: &JilesAthertonInductor,
        windings: &[XyceCoreWindingBinding],
    ) -> Result<(), SolverError> {
        let invalid = || {
            SolverError::InvalidCircuit(format!(
                "Xyce LEVEL=2 core '{}' has an undefined or unconverged DC material tangent or nonfinite small-signal coefficient",
                device.name(),
            ))
        };
        let mut ampere_turns = 0.0;
        for winding in windings {
            let ordinal = self.inductors.branch_indices[winding.inductor_index];
            let branch = self.get_branch_matrix_index(ordinal) - 1;
            ampere_turns += winding.turns * operating_point.get(branch).ok_or_else(invalid)?;
        }
        let happ = device.xyce_core_happ_from_ampere_turns(ampere_turns);
        let mid = device.xyce_core_level2_dc_mid(happ).ok_or_else(invalid)?;
        for row in windings {
            let row_branch =
                self.get_branch_matrix_index(self.inductors.branch_indices[row.inductor_index]) - 1;
            for column in windings {
                let column_branch = self
                    .get_branch_matrix_index(self.inductors.branch_indices[column.inductor_index])
                    - 1;
                // Xyce's nonlinear K-card path supplies unity COUP_VAL;
                // the authored K scalar is retained as topology metadata.
                let inductance =
                    device.xyce_core_vacuum_mutual_inductance(row.turns, column.turns, 1.0) * mid;
                let coefficient = -omega * inductance;
                if !inductance.is_finite() || !coefficient.is_finite() {
                    return Err(invalid());
                }
                matrix.add_imag(row_branch, column_branch, coefficient);
            }
        }
        Ok(())
    }
}

//! Selected unit-source transfer from the already-computed output adjoint.
use crate::analysis::noise::NoiseInputQuantity;
use crate::circuit::CircuitData;
use crate::{Complex64, SimulationError};

pub(super) struct NoiseInputReference {
    pub(super) quantity: NoiseInputQuantity,
    positive_row: Option<usize>,
    negative_row: Option<usize>,
}

impl NoiseInputReference {
    pub(super) fn resolve(circuit: &CircuitData, name: &str) -> Result<Self, SimulationError> {
        let invalid = || {
            SimulationError::Circuit(format!(
                "Noise input source '{name}' has invalid matrix coordinates"
            ))
        };
        if let Some(index) = circuit
            .voltage_sources
            .names
            .iter()
            .position(|item| item.eq_ignore_ascii_case(name))
        {
            let branch = *circuit
                .voltage_sources
                .branch_indices
                .get(index)
                .ok_or_else(invalid)?;
            if branch == 0 {
                return Err(invalid());
            }
            return Ok(Self {
                quantity: NoiseInputQuantity::Voltage,
                positive_row: Some(circuit.get_branch_matrix_index(branch) - 1),
                negative_row: None,
            });
        }
        if let Some(index) = circuit
            .current_sources
            .names
            .iter()
            .position(|item| item.eq_ignore_ascii_case(name))
        {
            return Ok(Self {
                quantity: NoiseInputQuantity::Current,
                positive_row: circuit
                    .current_sources
                    .node_neg
                    .get(index)
                    .ok_or_else(invalid)?
                    .checked_sub(1),
                negative_row: circuit
                    .current_sources
                    .node_pos
                    .get(index)
                    .ok_or_else(invalid)?
                    .checked_sub(1),
            });
        }
        Err(SimulationError::Circuit(format!(
            "Noise input source '{name}' not found (expected independent V/I source)"
        )))
    }

    /// With z=A^-T*c and unit input b, the gain c^T*A^-1*b is z^T*b.
    /// This uses the ordinary transpose, without conjugating the phasors.
    pub(super) fn gain(&self, adjoint: &[Complex64]) -> Result<Complex64, SimulationError> {
        let value = |row: Option<usize>| -> Result<Complex64, SimulationError> {
            row.map_or(Ok(Complex64::new(0.0, 0.0)), |row| {
                adjoint.get(row).copied().ok_or_else(|| {
                    SimulationError::Circuit("noise input lies outside the solved system".into())
                })
            })
        };
        Ok(value(self.positive_row)? - value(self.negative_row)?)
    }
}

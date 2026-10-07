//! Engine adapters for the shared exact numerical constraint kernel.
use super::SimulationError;
use crate::numerics::exact_constraints::ConstraintError;
pub(super) use crate::numerics::exact_constraints::{
    ConstraintDisposition, ExactElimination, ExactRow, close_descriptor, coefficient_ratio,
    integer_coefficient,
};

impl From<ConstraintError> for SimulationError {
    fn from(error: ConstraintError) -> Self {
        match error {
            ConstraintError::Aborted => Self::Aborted,
            ConstraintError::Invalid(message) => Self::Circuit(message),
            ConstraintError::ResourceLimit(error) => Self::ResourceLimit(error),
        }
    }
}

//! App integration tests for the lower-owned numerical override contract.

#[cfg(test)]
pub use rspice_simulation_contract::numeric_override::OverrideValue;
pub use rspice_simulation_contract::numeric_override::{
    AnalysisNumericOverride, NumericOverrideOption, OverrideSection, OverrideValueKind,
    SolverOwnership,
};

#[cfg(test)]
use super::AnalysisKind;
#[cfg(test)]
use rspice_simulation_contract::numeric_override::OptionPackage;
#[cfg(test)]
use rspice_simulation_contract::options::{DampingStrategy, IntegrationMethod, MatrixSolver};

#[cfg(test)]
mod tests;

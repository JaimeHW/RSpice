//! Numerical override contract imports.

#[cfg(test)]
pub use rspice_simulation_contract::numeric_override::OverrideValue;
pub use rspice_simulation_contract::numeric_override::{
    AnalysisNumericOverride, NumericOverrideOption, OverrideSection, OverrideValueKind,
    SolverOwnership,
};

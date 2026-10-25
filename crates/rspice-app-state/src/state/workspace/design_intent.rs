//! App-facing reexports of authored design variables and specifications.

#[cfg(test)]
pub use rspice_results::specification::SpecificationComparison;
pub use rspice_results::specification::{
    MeasurementReferenceSource, MissingMeasurementPolicy, MonteCarloSpecificationGate,
    NominalFailurePolicy, RegressionSpecificationPolicy, SpecEntry, SpecPointScope,
    SpecificationDefinition, SpecificationPolicy, SpecificationRole,
};
pub use rspice_simulation_contract::design_variable::{
    DesignVariable, DesignVariableDefect, DesignVariableOverridePolicy, DesignVariableRange,
    DesignVariableScope, DesignVariableSweepEligibility,
};
pub use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;

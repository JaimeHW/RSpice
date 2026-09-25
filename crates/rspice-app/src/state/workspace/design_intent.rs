//! App-facing reexports of authored design variables and specifications.

pub use rspice_simulation_contract::design_variable::{
    DesignVariable, DesignVariableDefect, DesignVariableOverridePolicy, DesignVariableRange,
    DesignVariableScope, DesignVariableSweepEligibility,
};
pub use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;
pub use rspice_simulation_contract::specification::{
    MeasurementReferenceSource, MissingMeasurementPolicy, MonteCarloSpecificationGate,
    NominalFailurePolicy, RegressionSpecificationPolicy, SpecEntry, SpecPointScope,
    SpecificationComparison, SpecificationDefinition, SpecificationPolicy, SpecificationRole,
};

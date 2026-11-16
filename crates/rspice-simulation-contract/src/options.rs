//! Authored solver policies, persistence migrations, and exact numeric field conversion.

mod compatibility;
mod connect_rules;
mod enums;
mod model;
mod si;
mod validation;

pub use compatibility::{SimulationCompatibility, SourceSimulator};
pub use connect_rules::{
    RUN_CONNECT_MODULES, RunConnectParameter, RunConnectRules, RunConnectStatement,
    RunConnectSupply,
};
pub use enums::{DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver};
pub use model::{NamedPreset, SimulationOptions};
pub(crate) use si::format_freq;
pub use si::{ParseError, format_si_value, parse_si_value};
pub use validation::ValidationError;

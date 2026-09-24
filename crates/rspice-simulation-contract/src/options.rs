//! Authored solver policies, persistence migrations, and exact numeric field conversion.

mod compatibility;
mod enums;
mod model;
mod si;
mod validation;

pub use compatibility::SimulationCompatibility;
pub use enums::{DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver};
pub use model::{NamedPreset, SimulationOptions};
pub use si::{ParseError, format_si_value, parse_si_value};
pub use validation::ValidationError;

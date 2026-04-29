//! Simulation Options - Spectre-Compatible Configuration
//!
//! Options control accuracy, convergence, algorithm selection, limits,
//! temperature, and performance behavior for UI-driven simulations.

mod enums;
mod model;
mod si;
mod state;
mod validation;

pub use enums::{DampingStrategy, IntegrationMethod, MatrixSolver};
pub use model::SimulationOptions;
pub use si::{ParseError, format_si_value, parse_si_value};
pub use state::OptionsDialogState;
pub use validation::ValidationError;

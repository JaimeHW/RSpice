//! Solver-option editor buffers backed by the portable authored policy.

mod state;
#[cfg(test)]
mod tests;

pub use rspice_simulation_contract::options::{
    DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver, SimulationCompatibility,
    SimulationOptions, format_si_value, parse_si_value,
};
pub use state::OptionsDialogState;

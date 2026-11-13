//! Solver options backed by the portable authored policy. Their editor
//! buffers are plan drafts, in `simulation::plan`.

#[cfg(test)]
mod tests;

pub use rspice_simulation_contract::options::{
    DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver, SimulationCompatibility,
    SimulationOptions, format_si_value, parse_si_value,
};

//! S-Parameter Analysis Configuration
//!
//! Configuration for RF/microwave S-parameter analysis (.sp).
//! S-parameters describe the electrical behavior of linear networks
//! in terms of incident and reflected waves.

mod config;
mod state;

pub use config::SpConfig;
#[cfg(test)]
pub use config::SpPortConfig;
#[cfg(test)]
pub use rspice_simulation_contract::sp_config::SpSweepType;
#[cfg(test)]
pub use state::TOUCHSTONE_VERSIONS;
pub use state::{
    SpDialogState, SpPortSource, TOUCHSTONE_VERSION_LABELS, port_roster_error, to_config,
};

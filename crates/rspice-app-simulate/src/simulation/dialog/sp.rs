//! S-Parameter Analysis Configuration
//!
//! Configuration for RF/microwave S-parameter analysis (.sp).
//! S-parameters describe the electrical behavior of linear networks
//! in terms of incident and reflected waves.

mod state;

#[cfg(test)]
pub use rspice_simulation_contract::sp_config::SpConfig;
#[cfg(test)]
pub use rspice_simulation_contract::sp_config::SpPortConfig;
pub use state::{SpDialogState, SpPortSource, TOUCHSTONE_VERSION_LABELS, port_roster_error};

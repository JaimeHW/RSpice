//! S-Parameter Analysis Configuration
//!
//! Configuration for RF/microwave S-parameter analysis (.sp).
//! S-parameters describe the electrical behavior of linear networks
//! in terms of incident and reflected waves.

mod config;
mod state;

#[cfg(test)]
pub use config::SpPortConfig;
pub use config::{SpConfig, SpSweepType};
#[cfg(test)]
pub use state::TOUCHSTONE_VERSIONS;
pub use state::{
    SpDialogState, SpPortSource, TOUCHSTONE_VERSION_LABELS, port_roster_error, to_config,
};

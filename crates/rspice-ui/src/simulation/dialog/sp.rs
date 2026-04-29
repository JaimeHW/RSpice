//! S-Parameter Analysis Configuration
//!
//! Configuration for RF/microwave S-parameter analysis (.sp).
//! S-parameters describe the electrical behavior of linear networks
//! in terms of incident and reflected waves.

mod config;
mod format;
mod render;
mod state;

pub use config::{SpConfig, SpPortConfig, SpSweepType};
pub use state::{SpDialogState, SpPortDialogState};

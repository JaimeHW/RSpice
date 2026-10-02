//! Frequency-response viewer availability and inspectors for retained evidence.

pub(crate) mod data;
pub mod inspector;
pub mod state;

pub use data::BodeData;
pub use state::BodePlotState;

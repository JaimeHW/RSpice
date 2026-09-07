//! Distribution binning and presentation of retained Monte Carlo samples.

pub(crate) mod data;
pub(crate) mod display;
pub(crate) mod state;

pub use data::HistogramBuilder;
pub use state::{HistogramDisplayMode, HistogramState};

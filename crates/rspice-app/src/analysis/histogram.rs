//! Distribution binning and presentation of retained Monte Carlo samples.

pub(crate) mod display;
pub(crate) mod state;

pub use rspice_results::histogram::{HistogramBuilder, HistogramDisplayMode};
pub use state::HistogramState;

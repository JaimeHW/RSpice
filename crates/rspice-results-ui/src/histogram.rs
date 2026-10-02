//! Distribution binning and presentation of retained Monte Carlo samples.

pub use rspice_results::histogram::display;
pub mod state;
pub mod view;

pub use rspice_results::histogram::{HistogramBuilder, HistogramDisplayMode};
pub use state::HistogramState;

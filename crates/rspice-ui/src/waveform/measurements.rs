//! Basic amplitude measurements over waveform sample slices.
//!
//! Single-pass statistics (Welford accumulation) shared by the Results
//! workspace's measurement tables.

mod basic;

pub use basic::{
    calculate_max, calculate_mean, calculate_min, calculate_min_max_rms, calculate_pk_pk,
    calculate_rms, calculate_std_dev,
};

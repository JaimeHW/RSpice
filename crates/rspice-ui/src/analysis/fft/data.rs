//! FFT Data Structures
//!
//! Core data types for FFT and spectrum analysis.

#![allow(clippy::type_complexity)]

mod analysis;
mod cache;
mod error;
mod fft_data;
mod normalization;
mod point;

pub use analysis::SpectrumAnalysis;
pub(crate) use error::MIN_FFT_DATA_SAMPLES;
pub use error::{FftAllocationStage, FftBuildError};
pub use fft_data::FftData;
pub use normalization::SpectrumNormalization;
pub use point::FftPoint;

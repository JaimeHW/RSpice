//! FFT Data Structures
//!
//! Core data types for FFT and spectrum analysis.

#![allow(clippy::type_complexity)]

mod analysis;
mod cache;
mod fft_data;
mod normalization;
mod point;

pub use analysis::SpectrumAnalysis;
pub use fft_data::FftData;
pub use normalization::SpectrumNormalization;
pub use point::FftPoint;

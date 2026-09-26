//! Exact result documents and viewer-independent result rules.
//!
//! Presentation and execution use these contracts without owning their data.
//! The `engine-evidence` feature retains core-backed observations and their
//! canonical validation; document and calculation consumers can omit it.

pub mod analysis_type;
pub mod bode;
pub mod calculator;
pub mod convergence_attribution;
#[cfg(feature = "engine-evidence")]
pub mod convergence_quality;
#[cfg(feature = "engine-evidence")]
pub mod current_impulses;
pub mod dc_mismatch;
pub mod dc_sweep;
#[cfg(feature = "engine-evidence")]
pub mod events;
pub mod family_measurements;
pub mod family_metadata;
pub mod fft;
#[cfg(feature = "engine-evidence")]
pub mod floquet;
pub mod histogram;
pub mod interpolation;
pub mod measurements;
pub mod monte_carlo;
#[cfg(feature = "engine-evidence")]
pub mod noise;
pub mod nyquist;
pub mod operating_point;
pub mod optimization;
#[cfg(feature = "engine-evidence")]
pub mod pole_zero;
pub mod provenance;
pub mod report_document;
pub mod result_import;
pub mod safety;
pub mod sampling;
#[cfg(feature = "engine-evidence")]
pub mod sensitivity;
pub mod simulation_values;
pub mod soa_evidence;
pub mod stability;
pub mod validation;
pub mod viewer_catalog;
pub mod visualization_document;
pub mod visualization_raster;
pub mod waveform;
pub mod yield_analysis;

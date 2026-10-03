//! Result viewer sessions over explicit retained evidence.
//!
//! These sessions own derived display caches and viewer preferences. The host
//! selects their source, validates its currentness, and coordinates simulation,
//! persistence and navigation. Numerical operations retain their existing
//! owners in `rspice-results` and `rspice-core`.

pub mod bode;
pub mod box_violin;
pub mod chrome;
pub mod derived;
pub mod events;
pub mod eye_diagram;
pub mod fft;
pub mod harmonic_balance;
pub mod histogram;
pub mod manifest;
pub mod network_matrix;
pub mod noise_contrib;
pub mod nyquist;
pub mod op_inspector;
pub mod optimization;
pub mod phase_noise;
pub mod polar;
pub mod pole_zero;
pub mod presentation;
pub mod report;
pub mod scatter;
pub mod sensitivity;
pub mod smith_chart;
pub mod soa;
pub mod specs;
pub mod strip;
pub mod studio;
pub mod table;
pub mod transfer_function;
pub mod virtual_rows;
pub mod waveform;
pub mod waves;

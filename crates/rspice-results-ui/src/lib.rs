//! Result viewer sessions over explicit retained evidence.
//!
//! These sessions own derived display caches and viewer preferences. The host
//! selects their source, validates its currentness, and coordinates simulation,
//! persistence and navigation. Numerical operations retain their existing
//! owners in `rspice-results` and `rspice-core`.

pub mod bode;
pub mod derived;
pub mod eye_diagram;
pub mod fft;
pub mod histogram;
pub mod network_matrix;
pub mod nyquist;
pub mod presentation;
pub mod smith_chart;
pub mod strip;
pub mod transfer_function;
pub mod waveform;
pub mod waves;

//! Persisted analysis drafts whose typed projections are independent of the app.

mod ac_data;
mod core;
mod fft;
mod frequency_sweep;
mod noise;
pub mod parse;

pub use ac_data::{AcDataDraft, AcDataParameterDraft};
pub use core::{AcSetup, DcSetup, DistoDraft, TranSetup};
pub use fft::FftDraft;
pub use frequency_sweep::FrequencySweepDraft;
pub use noise::NoiseDraft;

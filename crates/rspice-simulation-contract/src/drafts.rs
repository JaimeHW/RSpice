//! Persisted analysis drafts whose typed projections are independent of the app.

mod ac_data;
mod core;
mod fft;
mod frequency_sweep;
mod noise;
pub mod parse;
mod statistical;

pub use ac_data::{AcDataDraft, AcDataParameterDraft};
pub use core::{AcSetup, DcSetup, DistoDraft, TranSetup};
pub use fft::FftDraft;
pub use frequency_sweep::FrequencySweepDraft;
pub use noise::NoiseDraft;
pub use statistical::{
    DcMismatchDraft, TransientNoiseDraft, dc_mismatch_share_threshold, validate_dc_mismatch,
    validate_transient_noise,
};

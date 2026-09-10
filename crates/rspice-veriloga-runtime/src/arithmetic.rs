//! Shared scalar arithmetic and extended-range frequency intermediates.

mod scalar;
mod scaled;

pub use scalar::*;
#[doc(hidden)]
pub use scaled::ScaledValue;

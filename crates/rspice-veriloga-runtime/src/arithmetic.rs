//! Shared scalar arithmetic and extended-range frequency intermediates.

mod circular;
mod scalar;
mod scaled;

pub use circular::{IdtModOrigin, IdtModOriginCheckpoint};
pub use scalar::*;
#[doc(hidden)]
pub use scaled::ScaledValue;

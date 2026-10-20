//! Shared scalar arithmetic and extended-range frequency intermediates.

mod circular;
mod decimal;
mod scalar;
mod scaled;

pub use circular::{IdtModOrigin, IdtModOriginCheckpoint};
pub use decimal::scaled_decimal;
pub use scalar::*;
#[doc(hidden)]
pub use scaled::ScaledValue;

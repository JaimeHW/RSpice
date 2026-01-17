//! Two-terminal semiconductor device models
//!
//! Includes diodes and bipolar junction transistors (BJTs).

mod bjt;
mod diode;

pub use bjt::{Bjt, BjtType};
pub use diode::Diode;

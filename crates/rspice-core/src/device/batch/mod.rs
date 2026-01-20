//! Batch Device Processing Module
//!
//! This module provides Struct-of-Arrays (SoA) storage for semiconductor devices
//! to enable efficient SIMD batch evaluation. This is the key optimization for
//! circuits with many devices of the same type.
//!
//! # Architecture
//!
//! Instead of storing devices as an array of structs (AoS):
//! ```text
//! [{is, n, vt, node_a, node_c}, {is, n, vt, node_a, node_c}, ...]
//! ```
//!
//! We use struct of arrays (SoA):
//! ```text
//! {is: [val, val, ...], n: [val, val, ...], vt: [val, val, ...], ...}
//! ```
//!
//! This allows SIMD operations to load 4 values at once and process them
//! in parallel, significantly improving performance for large circuits.
//!
//! # Supported Devices
//!
//! - **Diodes**: Shockley equation with SIMD exponential
//! - **MOSFETs (Level 1)**: Shichman-Hodges with body effect
//! - **MOSFETs (Level 6)**: Double-exponent empirical model
//! - **BJTs**: Ebers-Moll with Gummel-Poon high-injection
//! - **JFETs**: Shichman-Hodges model
//!
//! # Usage
//!
//! Batch processing is automatically enabled when a circuit contains 4+ devices
//! of a compatible type and the `simd` feature is enabled.

mod bjt;
mod diode;
mod jfet;
mod mosfet_level1;
mod mosfet_level6;

pub use bjt::BatchBjts;
pub use diode::BatchDiodes;
pub use jfet::BatchJfets;
pub use mosfet_level1::BatchMosfets;
pub use mosfet_level6::BatchMosfetsLevel6;

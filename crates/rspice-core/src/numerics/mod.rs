//! Numerical methods shared by every analysis.
//!
//! What the analyses have in common below the level of any one of them: how a
//! derivative is discretized, and how a linear system is solved. These sit
//! beneath the circuit store and the device models, because those stamp into
//! structures this module defines.
//!
//! `integration` is here. The sparse LU, Newton loop, damping and
//! continuation currently in `crate::solver` join it as `numerics::la`.

pub mod integration;

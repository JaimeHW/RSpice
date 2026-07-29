//! Numerical methods shared by every analysis.
//!
//! What the analyses have in common below the level of any one of them: how a
//! derivative is discretized, how large a step may be, and where a step is not
//! allowed to land. These sit beneath the circuit store and the device models,
//! because those stamp into structures this module defines.
//!
//! `crate::solver` — sparse LU, the Newton loop, damping and continuation —
//! is the other half of this and sits one layer up, because it solves against
//! an assembled matrix rather than defining one.

pub mod integration;

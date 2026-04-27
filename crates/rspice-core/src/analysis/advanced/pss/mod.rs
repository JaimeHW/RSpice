//! Periodic Steady-State (PSS) Analysis
//!
//! PSS analysis finds the periodic steady-state solution of circuits driven by
//! periodic large-signal excitations. This is essential for RF circuit analysis:
//! oscillators, mixers, PLLs, and power amplifiers.
//!
//! # Algorithm: Shooting Method
//!
//! The shooting method formulates finding the periodic steady state as a
//! boundary value problem (BVP): find initial state `x(0)` such that
//! `x(T) = x(0)` where `T` is the period.
//!
//! 1. Start with initial guess `x₀`
//! 2. Run transient simulation from 0 to T
//! 3. Compute residual F(x₀) = x(T) - x₀
//! 4. If ||F|| < tolerance, done
//! 5. Compute Jacobian (Monodromy matrix) and update: x₀ ← x₀ - J⁻¹·F
//! 6. Repeat from step 2
//!
//! # SPICE Syntax
//! ```text
//! .PSS fund=1GHz nharms=9 tstab=10n
//! ```
//!
//! # Example
//! ```ignore
//! use rspice_core::analysis::advanced::pss::{PssConfig, PssAnalysis};
//!
//! let config = PssConfig::new(1e9)  // 1 GHz fundamental
//!     .with_harmonics(9)
//!     .with_tstab(10e-9);
//! ```

mod config;
mod period_detect;
mod result;
mod shooting;

pub use config::PssConfig;
pub use period_detect::{PeriodDetector, PeriodEstimate};
pub use result::{PeriodicWaveform, PssResult};
pub use shooting::{ShootingNewtonSolver, ShootingState};

//! Convergence helpers for Newton-Raphson iteration
//!
//! This module provides:
//! - GMIN stepping for difficult circuits
//! - Source stepping for convergence
//! - Linear and nonlinear solver interfaces

#![allow(clippy::too_many_arguments)]
use super::{DampingStrategy, Engine, SimulationError};
use crate::abort_signal::AbortSignal;
use crate::device::NonlinearConvergenceCriteria;
use crate::solver::{
    ArcLengthConfig, ArcLengthContinuation, PseudoTransient, SolverError, StaticMatrix,
};
use crate::{CircuitData, Value};

mod continuation;
mod damping;
mod fallback;
mod residuals;
mod solve;
mod stamping;
mod tolerances;

#[derive(Debug, Clone, Copy)]
struct NewtonDampingState {
    pub(in crate::engine::convergence) bank_rose_alpha: Value,
    pub(in crate::engine::convergence) prev_step_norm: Option<Value>,
}

impl Default for NewtonDampingState {
    fn default() -> Self {
        Self {
            bank_rose_alpha: 1.0,
            prev_step_norm: None,
        }
    }
}

impl Engine {
    const MAX_NODE_VOLTAGE: Value = 1000.0;
    const MAX_DELTA_VOLTAGE_LIMIT: Value = 0.5;
    const BANK_ROSE_ALPHA_MIN: Value = 0.1;
    const BANK_ROSE_ALPHA_MAX: Value = 1.0;
    const ARMIJO_C1: Value = 1e-4;
    const LINE_SEARCH_BACKTRACK: Value = 0.5;
    const LINE_SEARCH_MAX_ITERS: usize = 8;
    const ARC_LENGTH_MAX_STEPS: usize = 128;
    const ABORT_POLL_MASK: usize = 0x7;
    const DC_RESIDUAL_STALL_LIMIT: usize = 3;
    const DC_LIMIT_CYCLE_HISTORY: usize = 64;
    const DC_LIMIT_CYCLE_HIT_LIMIT: usize = 3;
    const DC_LIMIT_CYCLE_MIN_PERIOD: usize = 4;
    const DC_LIMIT_CYCLE_MAX_TRACKED_VALUES: usize = 1024;
    const MAX_CONTINUATION_CORRECTOR_ITERS: usize = 512;
}

//=============================================================================
// Tests
//=============================================================================

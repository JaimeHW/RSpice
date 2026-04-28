//! Transmission Line Models
//!
//! Implements lossless and lossy transmission lines for high-frequency simulation.
//!
//! # SPICE Syntax
//! ```text
//! T<name> n1+ n1- n2+ n2- Z0=<impedance> TD=<delay>
//! T1 1 0 2 0 Z0=50 TD=1ns
//! ```
//!
//! # Theory
//! A lossless transmission line is characterized by:
//! - Z0: Characteristic impedance (Î©)
//! - TD: Propagation delay (s)
//!
//! The telegrapher's equations relate voltage and current at both ends:
//! ```text
//! V1(t) + Z0*I1(t) = V2(t-TD) + Z0*I2(t-TD)
//! V2(t) + Z0*I2(t) = V1(t-TD) + Z0*I1(t-TD)
//! ```
//!
//! # Implementation
//! Uses delay buffers to store past values and interpolates for accurate delays.
//! The transmission line is modeled as dependent sources with delay.

#![allow(clippy::too_many_arguments)]
use crate::{Value, circuit::NodeId};
use std::cell::Cell;
use std::collections::VecDeque;

/// DC fallback resistance for ideal/lossless lines when no explicit series
/// resistance is available from model parameters.
const TLINE_DC_SHORT_RESISTANCE: Value = 1e-3;
/// Relative tolerance used to truncate negligible distributed-RLC kernel terms.
const DISTRIBUTED_RLC_CHOP_RELTOL: Value = 1e-3;
/// Default relative tolerance for ngspice-style LTRA straight-line compaction.
const DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT: Value = 1e-3;
/// Default absolute tolerance for ngspice-style LTRA straight-line compaction.
const DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT: Value = 1e-12;

mod delay;
mod distributed;
mod line;
mod lossy;
mod response;

use delay::DelayBuffer;
use distributed::{
    DistributedRlcKernel, distributed_rlc_coefficients, distributed_rlc_max_safe_step,
};
pub use line::TransmissionLine;
pub use lossy::LossyTransmissionLine;
pub(crate) use response::TlineTransientResponse;

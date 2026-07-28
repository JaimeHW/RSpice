//! Transfer Function Analysis (.TF)
//!
//! Computes DC small-signal transfer characteristics:
//! - Transfer gain: V(output)/V(input) or I(output)/I(input)
//! - Input impedance: Zin = V(input)/I(input)
//! - Output impedance: Zout (Thevenin equivalent at output)
//!
//! # Algorithm
//!
//! 1. Solve DC operating point to linearize nonlinear devices
//! 2. Build small-signal conductance matrix G
//! 3. **Gain**: Apply unit input, measure output
//! 4. **Zin**: Zin = Vinput / Iinput (from source current)
//! 5. **Zout**: Zero input, apply test current at output
//!
//! # Example
//!
//! ```ignore
//! .TF V(out) Vin
//! ```

#![allow(clippy::needless_range_loop)]
use crate::Value;

mod ac;
mod dc;

pub use ac::{
    AcSweepType, AcTransferAnalyzer, AcTransferConfig, AcTransferPoint, AcTransferResult,
};
pub use dc::{TransferAnalyzer, TransferFunctionConfig, TransferFunctionResult};

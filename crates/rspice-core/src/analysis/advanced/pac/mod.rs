//! Periodic AC (PAC) Analysis Module
//!
//! PAC analysis performs small-signal AC analysis around a periodic steady-state
//! (PSS) operating point. This is essential for RF circuit characterization:
//!
//! - **Mixer conversion gain**: Input-to-output transfer at different sidebands
//! - **LNA noise figure**: With periodic large-signal interference  
//! - **PLL loop gain**: Stability analysis around limit cycle
//! - **Power amplifier intermodulation**: Small-signal response vs. drive level
//!
//! # Theory
//!
//! Unlike standard AC analysis which linearizes around a DC operating point,
//! PAC linearizes around a time-varying periodic operating point from PSS.
//! This results in a Linear Time-Varying (LTV) system characterized by a
//! **conversion matrix** that maps input sidebands to output sidebands.
//!
//! For a mixer with LO frequency f₀:
//! - Input at f₀ + Δf (RF) appears at output as DC + Δf (IF)
//! - The conversion matrix element H[-1,0](Δf) is the conversion gain
//!
//! # Algorithm
//!
//! 1. Obtain PSS solution (periodic operating point and Monodromy matrix)
//! 2. Compute time-varying small-signal parameters along the period
//! 3. Apply Floquet transformation to get conversion matrices
//! 4. Sweep input frequency and compute transfer at each sideband
//!
//! # SPICE Syntax
//!
//! ```text
//! .PAC start=1Meg stop=1Gig pts=100 maxsideband=5
//! ```
//!
//! # Example
//!
//! ```ignore
//! use rspice_core::analysis::advanced::pac::{PacConfig, PacAnalyzer};
//!
//! // After running PSS...
//! let pac_config = PacConfig::new()
//!     .with_sweep(1e6, 1e9, 100)    // 1 MHz to 1 GHz, 100 points
//!     .with_sidebands(-5, 5);        // Harmonics -5 to +5
//!
//! let pac_result = engine.run_pac(&netlist, &pss_result, pac_config)?;
//!
//! // Get conversion gain from RF (sideband 1) to IF (sideband 0)
//! let conversion_gain = pac_result.transfer(1, 0, output_node);
//! ```

mod config;
mod conversion_matrix;
mod result;
mod solver;

pub use config::{PacConfig, PacSweepType};
pub use conversion_matrix::{ConversionMatrix, SidebandTransfer};
pub use result::{PacResult, PacSidebandData};
pub use solver::{PacAnalyzer, PacError};

//! VDMOS (Vertical Double-diffused MOS) Power MOSFET Model
//!
//! Implements a power MOSFET model suitable for switching power supply simulation.
//! Key differences from planar MOSFET:
//! - **Quasi-saturation**: At high currents, the lightly-doped drift region limits current
//! - **Body diode**: Integrated anti-parallel diode from source to drain
//! - **Rds(on)**: Characterized by on-resistance rather than transconductance
//!
//! # Model Parameters
//! | Parameter | Description | Default |
//! |-----------|-------------|---------|
//! | VTH | Threshold voltage | 2.0V |
//! | KP | Transconductance coefficient | 2.0 A/V² |
//! | RD | Drain drift region resistance | 0.1Ω |
//! | RS | Source metallization resistance | 0.01Ω |
//! | RG | Gate resistance | 1Ω |
//! | LAMBDA | Channel length modulation | 0.01 V⁻¹ |
//! | MTRIODE | Triode region exponent | 1.5 |
//! | RQ | Quasi-saturation resistance | 0.5Ω |
//! | VQ | Quasi-saturation voltage | 5.0V |
//! | CGS | Gate-source capacitance | 1nF |
//! | CGD | Gate-drain (Miller) capacitance | 100pF |
//! | IS | Body diode saturation current | 1e-14A |
//! | N | Body diode ideality factor | 1.5 |
//! | TT | Body diode transit time | 50ns |
//! | BV | Breakdown voltage | 100V |

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

mod device;
mod recovery;
mod thermal;
mod types;

pub use device::Vdmos;
#[allow(unused_imports)]
pub use device::VdmosIndices;
pub use recovery::DiodeRecovery;
pub use thermal::ThermalNetwork;
pub use types::{VdmosRegion, VdmosType};

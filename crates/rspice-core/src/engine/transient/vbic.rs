//! VBIC transient hidden-state snapshot helpers.

use super::*;
use crate::device::semiconductor::VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT;
use crate::device::{BjtType, NonlinearConvergenceCriteria, NonlinearDevice};

mod continuation;
mod convergence;
mod linearization;
mod snapshot_solve;
mod state_evaluation;

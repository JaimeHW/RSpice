//! AC, pole-zero, noise, sensitivity, and transfer-function regression runners.

use super::*;

#[path = "frequency_analyses/ac.rs"]
mod ac;
#[path = "frequency_analyses/common.rs"]
mod common;
#[path = "frequency_analyses/frequencies.rs"]
mod frequencies;
#[path = "frequency_analyses/noise.rs"]
mod noise;
#[path = "frequency_analyses/pole_zero.rs"]
mod pole_zero;
#[path = "frequency_analyses/sensitivity.rs"]
mod sensitivity;
#[path = "frequency_analyses/transfer.rs"]
mod transfer;

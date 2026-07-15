//! Advanced Analysis Functions
//!
//! This module provides specialized analysis types:
//! - Noise analysis (thermal, shot, flicker)
//! - Monte Carlo statistical analysis
//! - Pole-zero analysis  
//! - Sensitivity analysis
//! - Parametric step sweep

#![allow(clippy::too_many_arguments)]
use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::monte_carlo::{
    Distribution, MonteCarloResult, VariableStatistics, Xorshift128Plus,
};
use crate::analysis::noise::{
    Bsim3FlickerNoise, Bsim4FlickerNoise, CorrelatedNoisePair, NoiseContribution, NoisePort,
    NoiseResult, NoiseSource, PortNoiseCorrelationResult,
};
use crate::analysis::pole_zero::{Matrix, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult};
use crate::analysis::sensitivity::{
    AcSensitivity, AcSensitivityOutput, AcSensitivityResult, ElementDesc, ElementType, Sensitivity,
    SensitivityAnalyzer, SensitivityResult,
};
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeSnapshot,
};
use crate::netlist::{ElementKind, SourceSpec, StepCommand, StepTarget};
use crate::solver::SimulationResult;
use crate::{CircuitData, Complex64, Netlist, Value};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

mod monte_carlo;
mod noise;
mod pole_zero;
mod sensitivity;
mod step;

pub use step::{MaterializedStepRun, StepPlan, StepPlanLimits};

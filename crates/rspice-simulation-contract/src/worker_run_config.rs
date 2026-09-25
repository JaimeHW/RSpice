//! Portable worker run-configuration payloads. App-side conversions stay with execution.

use serde::{Deserialize, Serialize};

use crate::worker_spec::WorkerSweepType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerTempRunConfig {
    pub temperatures_c: Vec<f64>,
    pub base_mode: WorkerCornerBaseMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerCornerRunConfig {
    pub process_corners: Vec<WorkerCornerProcess>,
    pub voltages: Vec<f64>,
    #[serde(default)]
    pub supply_source_names: Vec<String>,
    pub temperatures_c: Vec<f64>,
    pub full_matrix: bool,
    pub nominal_voltage: Option<f64>,
    pub base_mode: WorkerCornerBaseMode,
    pub model_bindings: Vec<WorkerCornerModelBinding>,
    /// The exact points to run, when the space is a filtered one. A worker that
    /// received only the axes would expand the cross product and solve the
    /// points the declaration removed.
    #[serde(default)]
    pub points: Vec<WorkerCornerPoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorkerCornerPoint {
    pub process: WorkerCornerProcess,
    pub voltage: f64,
    pub temperature_c: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerCornerModelBinding {
    pub process: WorkerCornerProcess,
    pub source_label: String,
    pub section: Option<String>,
    pub materialized_model_cards: String,
}

impl From<&rspice_model_library::CornerModelBinding> for WorkerCornerModelBinding {
    fn from(value: &rspice_model_library::CornerModelBinding) -> Self {
        Self {
            process: WorkerCornerProcess::from(value.process),
            source_label: value.source_label.clone(),
            section: value.section.clone(),
            materialized_model_cards: value.materialized_model_cards.clone(),
        }
    }
}

impl From<WorkerCornerModelBinding> for rspice_model_library::CornerModelBinding {
    fn from(value: WorkerCornerModelBinding) -> Self {
        Self {
            process: rspice_app_types::product::ProcessCorner::from(value.process),
            source_label: value.source_label,
            section: value.section,
            materialized_model_cards: value.materialized_model_cards,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerCornerBaseMode {
    Op,
    ConfiguredOp(Box<crate::config::OpConfig>),
    DcSweep {
        #[serde(default)]
        modes: crate::config::DcSweepModes,
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
    },
    DcSweepNested {
        #[serde(default)]
        modes: crate::config::DcSweepModes,
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
        source2: String,
        start2: f64,
        stop2: f64,
        step2: f64,
    },
    Transient {
        stop_time: f64,
        step_time: f64,
    },
    TransientWindow {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: Option<f64>,
        uic: bool,
    },
    Ac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerPacRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    /// The top of the sideband range, under the wire name the single symmetric
    /// bound had: for every symmetric run the two are the same number, so an
    /// older worker's request decodes as exactly the run it described.
    #[serde(rename = "max_sideband")]
    pub sideband_max: i32,
    /// The bottom of the range. Absent on a request an older worker wrote, and
    /// absent means the symmetric `-max`.
    #[serde(default)]
    pub sideband_min: Option<i32>,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub pac_magnitude: f64,
    pub include_dc: bool,
    pub reltol: f64,
    pub abstol: f64,
    /// The carrier the run linearizes around. Absent on a request an older
    /// worker wrote, and absent is the preceding periodic solve.
    #[serde(default)]
    pub carrier: WorkerPeriodicCarrier,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerPxfRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub input_source: String,
    pub input_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub output_sideband: i32,
    pub max_sideband: i32,
    pub reltol: f64,
    pub abstol: f64,
    #[serde(default)]
    pub carrier: WorkerPeriodicCarrier,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerPnoiseRunConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<rspice_core::analysis::pnoise::PeriodicNoiseSampling>,
    #[serde(default)]
    pub input_sideband: i32,
    #[serde(default)]
    pub output_sideband: i32,
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub max_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub noise_ref: WorkerPnoiseReference,
    pub integrated_noise: bool,
    pub noise_summary: bool,
    pub reltol: f64,
    pub abstol: f64,
    #[serde(default)]
    pub carrier: WorkerPeriodicCarrier,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerPstbRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: f64,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerCornerProcess {
    TT,
    SS,
    FF,
    SF,
    FS,
}

impl From<rspice_app_types::product::ProcessCorner> for WorkerCornerProcess {
    fn from(value: rspice_app_types::product::ProcessCorner) -> Self {
        match value {
            rspice_app_types::product::ProcessCorner::TT => Self::TT,
            rspice_app_types::product::ProcessCorner::SS => Self::SS,
            rspice_app_types::product::ProcessCorner::FF => Self::FF,
            rspice_app_types::product::ProcessCorner::SF => Self::SF,
            rspice_app_types::product::ProcessCorner::FS => Self::FS,
        }
    }
}

impl From<WorkerCornerProcess> for rspice_app_types::product::ProcessCorner {
    fn from(value: WorkerCornerProcess) -> Self {
        match value {
            WorkerCornerProcess::TT => Self::TT,
            WorkerCornerProcess::SS => Self::SS,
            WorkerCornerProcess::FF => Self::FF,
            WorkerCornerProcess::SF => Self::SF,
            WorkerCornerProcess::FS => Self::FS,
        }
    }
}

/// The carrier a periodic small-signal request names, on the wire.
///
/// The engine's `FROM=` vocabulary, spelled for transport. Defaulted on read
/// because an older worker's request carries no carrier at all, and the card
/// it was written from had no `FROM=` keyword: that request asked for the
/// preceding periodic solve, which is exactly what the default is.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerPeriodicCarrier {
    #[default]
    Preceding,
    Pss,
    Hb,
}

impl From<crate::periodic_carrier::PeriodicCarrier> for WorkerPeriodicCarrier {
    fn from(value: crate::periodic_carrier::PeriodicCarrier) -> Self {
        match value {
            crate::periodic_carrier::PeriodicCarrier::Preceding => Self::Preceding,
            crate::periodic_carrier::PeriodicCarrier::Pss => Self::Pss,
            crate::periodic_carrier::PeriodicCarrier::Hb => Self::Hb,
        }
    }
}

impl From<WorkerPeriodicCarrier> for crate::periodic_carrier::PeriodicCarrier {
    fn from(value: WorkerPeriodicCarrier) -> Self {
        match value {
            WorkerPeriodicCarrier::Preceding => Self::Preceding,
            WorkerPeriodicCarrier::Pss => Self::Pss,
            WorkerPeriodicCarrier::Hb => Self::Hb,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerPnoiseReference {
    Output,
    Input,
    Phase,
}

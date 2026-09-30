//! Lossless conversions between sweep configuration and the worker contract.

use rspice_simulation_contract::worker_run_config::{
    WorkerCornerBaseMode, WorkerCornerModelBinding, WorkerCornerPoint, WorkerCornerProcess,
    WorkerCornerRunConfig, WorkerTempRunConfig,
};
use rspice_simulation_contract::worker_spec::WorkerSweepType;

impl From<&super::TempRunConfig> for WorkerTempRunConfig {
    fn from(value: &super::TempRunConfig) -> Self {
        Self {
            temperatures_c: value.temperatures_c.clone(),
            base_mode: WorkerCornerBaseMode::from(&value.base_mode),
        }
    }
}

impl From<WorkerTempRunConfig> for super::TempRunConfig {
    fn from(value: WorkerTempRunConfig) -> Self {
        Self {
            temperatures_c: value.temperatures_c,
            base_mode: super::CornerBaseMode::from(value.base_mode),
        }
    }
}

impl From<&super::CornerRunConfig> for WorkerCornerRunConfig {
    fn from(value: &super::CornerRunConfig) -> Self {
        Self {
            process_corners: value
                .process_corners
                .iter()
                .copied()
                .map(WorkerCornerProcess::from)
                .collect(),
            voltages: value.voltages.clone(),
            supply_source_names: value.supply_source_names.clone(),
            temperatures_c: value.temperatures_c.clone(),
            full_matrix: value.full_matrix,
            nominal_voltage: value.nominal_voltage,
            base_mode: WorkerCornerBaseMode::from(&value.base_mode),
            model_bindings: value
                .model_bindings
                .iter()
                .map(WorkerCornerModelBinding::from)
                .collect(),
            points: value
                .points
                .iter()
                .map(|point| WorkerCornerPoint {
                    process: WorkerCornerProcess::from(point.process),
                    voltage: point.voltage,
                    temperature_c: point.temperature_c,
                })
                .collect(),
        }
    }
}

impl From<WorkerCornerRunConfig> for super::CornerRunConfig {
    fn from(value: WorkerCornerRunConfig) -> Self {
        Self {
            process_corners: value
                .process_corners
                .into_iter()
                .map(rspice_app_types::product::ProcessCorner::from)
                .collect(),
            voltages: value.voltages,
            supply_source_names: value.supply_source_names,
            temperatures_c: value.temperatures_c,
            full_matrix: value.full_matrix,
            nominal_voltage: value.nominal_voltage,
            base_mode: super::CornerBaseMode::from(value.base_mode),
            model_bindings: value
                .model_bindings
                .into_iter()
                .map(rspice_model_library::CornerModelBinding::from)
                .collect(),
            points: value
                .points
                .into_iter()
                .map(|point| super::CornerPoint {
                    process: rspice_app_types::product::ProcessCorner::from(point.process),
                    voltage: point.voltage,
                    temperature_c: point.temperature_c,
                })
                .collect(),
        }
    }
}

impl From<&super::CornerBaseMode> for WorkerCornerBaseMode {
    fn from(value: &super::CornerBaseMode) -> Self {
        match value {
            super::CornerBaseMode::Op => Self::Op,
            super::CornerBaseMode::ConfiguredOp(config) => Self::ConfiguredOp(config.clone()),
            super::CornerBaseMode::DcSweep {
                modes,
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                modes: modes.clone(),
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
            },
            super::CornerBaseMode::DcSweepNested {
                modes,
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweepNested {
                modes: modes.clone(),
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: source2.clone(),
                start2: *start2,
                stop2: *stop2,
                step2: *step2,
            },
            super::CornerBaseMode::Transient {
                stop_time,
                step_time,
            } => Self::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
            },
            super::CornerBaseMode::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::TransientWindow {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: *start_time,
                max_timestep: *max_timestep,
                uic: *uic,
            },
            super::CornerBaseMode::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
            },
        }
    }
}

impl From<WorkerCornerBaseMode> for super::CornerBaseMode {
    fn from(value: WorkerCornerBaseMode) -> Self {
        match value {
            WorkerCornerBaseMode::Op => Self::Op,
            WorkerCornerBaseMode::ConfiguredOp(config) => Self::ConfiguredOp(config),
            WorkerCornerBaseMode::DcSweep {
                modes,
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                modes,
                source_name,
                start,
                stop,
                step,
            },
            WorkerCornerBaseMode::DcSweepNested {
                modes,
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweepNested {
                modes,
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            },
            WorkerCornerBaseMode::Transient {
                stop_time,
                step_time,
            } => Self::Transient {
                stop_time,
                step_time,
            },
            WorkerCornerBaseMode::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            },
            WorkerCornerBaseMode::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: super::CornerFrequencySweep::from(sweep),
            },
        }
    }
}

impl From<super::CornerFrequencySweep> for WorkerSweepType {
    fn from(value: super::CornerFrequencySweep) -> Self {
        match value {
            super::CornerFrequencySweep::Decade => Self::Decade,
            super::CornerFrequencySweep::Octave => Self::Octave,
            super::CornerFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for super::CornerFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

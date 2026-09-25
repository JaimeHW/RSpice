//! Operating-point editor state and compatibility migration for saved projects.

use crate::config::{
    OpAccuracy, OpAnnotation, OpConfig, OpDeviceDetail, OpHomotopy, OpInitialGuess,
    OpNodeInitialization, OpRunPointContext, OpSaveDevice, OpTemperatureMode,
};
use serde::{Deserialize, Serialize};

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpDialogState {
    #[serde(default = "missing_selection_index")]
    pub temperature_mode_idx: usize,
    #[serde(default = "default_temperature")]
    pub temperature: String,
    #[serde(default)]
    pub initial_guess_idx: usize,
    #[serde(default)]
    pub node_initialization_idx: usize,
    #[serde(default)]
    pub homotopy_idx: usize,
    #[serde(default)]
    pub annotation_idx: usize,
    #[serde(default)]
    pub device_detail_idx: usize,
    #[serde(default)]
    pub save_device_op_idx: usize,
    #[serde(default = "default_accuracy_idx")]
    pub accuracy_idx: usize,

    #[serde(default, skip_serializing)]
    pub save_all: Option<bool>,
    #[serde(default, skip_serializing)]
    pub save_op_info: Option<bool>,
    #[serde(default, skip_serializing)]
    pub source_stepping: Option<bool>,
    #[serde(default, skip_serializing)]
    pub gmin_steps: Option<String>,
    #[serde(skip)]
    pub initialized: bool,
}

impl Default for OpDialogState {
    fn default() -> Self {
        Self::from_config(&OpConfig::default())
    }
}

impl OpDialogState {
    pub fn from_config(config: &OpConfig) -> Self {
        Self {
            temperature_mode_idx: index_of(&OpTemperatureMode::ALL, config.temperature_mode),
            temperature: config.temperature_celsius.to_string(),
            initial_guess_idx: index_of(&OpInitialGuess::ALL, config.initial_guess),
            node_initialization_idx: index_of(
                &OpNodeInitialization::ALL,
                config.node_initialization,
            ),
            homotopy_idx: index_of(&OpHomotopy::ALL, config.homotopy),
            annotation_idx: index_of(&OpAnnotation::ALL, config.annotation),
            device_detail_idx: index_of(&OpDeviceDetail::ALL, config.device_detail),
            save_device_op_idx: index_of(&OpSaveDevice::ALL, config.save_device_op),
            accuracy_idx: index_of(&OpAccuracy::ALL, config.accuracy),
            save_all: None,
            save_op_info: None,
            source_stepping: None,
            gmin_steps: None,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<OpConfig, String> {
        if let Some(legacy_gmin_steps) = self.gmin_steps.as_deref()
            && legacy_gmin_steps.trim().parse::<usize>().is_err()
        {
            return Err(
                "Legacy operating-point gmin_steps must be a non-negative integer".to_owned(),
            );
        }
        let temperature_mode = selected(
            &OpTemperatureMode::ALL,
            self.temperature_mode_idx,
            "temperature",
        )?;
        let temperature_celsius = match temperature_mode {
            OpTemperatureMode::Nominal27C => 27.0,
            _ => self
                .temperature
                .parse()
                .map_err(|_| "Invalid operating-point temperature")?,
        };
        let config = OpConfig {
            temperature_mode,
            temperature_celsius,
            initial_guess: selected(
                &OpInitialGuess::ALL,
                self.initial_guess_idx,
                "initial guess",
            )?,
            node_initialization: selected(
                &OpNodeInitialization::ALL,
                self.node_initialization_idx,
                "node initialization",
            )?,
            homotopy: selected(&OpHomotopy::ALL, self.homotopy_idx, "homotopy strategy")?,
            annotation: selected(&OpAnnotation::ALL, self.annotation_idx, "annotation")?,
            device_detail: selected(
                &OpDeviceDetail::ALL,
                self.device_detail_idx,
                "device detail",
            )?,
            save_device_op: selected(
                &OpSaveDevice::ALL,
                self.save_device_op_idx,
                "device OP save",
            )?,
            accuracy: selected(&OpAccuracy::ALL, self.accuracy_idx, "accuracy")?,
            selected_devices: Vec::new(),
            previous_state: None,
            violation_devices: Vec::new(),
            violation_source_content_digest: None,
            run_point: OpRunPointContext::default(),
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::default();
        }
    }

    pub fn prepare_after_restore(&mut self) {
        // Plans written before temperature-source selection existed carried
        // an authored scalar temperature. Preserve that value as explicit;
        // treating it as the new index-zero PVT policy would silently replace
        // it with the workspace reference temperature during preparation.
        if self.temperature_mode_idx == usize::MAX {
            self.temperature_mode_idx =
                index_of(&OpTemperatureMode::ALL, OpTemperatureMode::Explicit);
        }
        if self.source_stepping == Some(true) {
            self.homotopy_idx = index_of(&OpHomotopy::ALL, OpHomotopy::SourceStepping);
        } else if self
            .gmin_steps
            .as_deref()
            .and_then(|value| value.trim().parse::<usize>().ok())
            .is_some_and(|value| value > 0)
        {
            self.homotopy_idx = index_of(&OpHomotopy::ALL, OpHomotopy::GminStepping);
        }
        if let Some(save_all) = self.save_all {
            self.annotation_idx = index_of(
                &OpAnnotation::ALL,
                if save_all {
                    OpAnnotation::VoltagesAndCurrents
                } else {
                    OpAnnotation::None
                },
            );
        }
        if let Some(save_op) = self.save_op_info {
            self.device_detail_idx = index_of(
                &OpDeviceDetail::ALL,
                if save_op {
                    OpDeviceDetail::AllDevices
                } else {
                    OpDeviceDetail::None
                },
            );
            self.save_device_op_idx = index_of(
                &OpSaveDevice::ALL,
                if save_op {
                    OpSaveDevice::Enabled
                } else {
                    OpSaveDevice::Disabled
                },
            );
        }
        clamp_index(&mut self.temperature_mode_idx, OpTemperatureMode::ALL.len());
        clamp_index(&mut self.initial_guess_idx, OpInitialGuess::ALL.len());
        clamp_index(
            &mut self.node_initialization_idx,
            OpNodeInitialization::ALL.len(),
        );
        clamp_index(&mut self.homotopy_idx, OpHomotopy::ALL.len());
        clamp_index(&mut self.annotation_idx, OpAnnotation::ALL.len());
        clamp_index(&mut self.device_detail_idx, OpDeviceDetail::ALL.len());
        clamp_index(&mut self.save_device_op_idx, OpSaveDevice::ALL.len());
        clamp_index(&mut self.accuracy_idx, OpAccuracy::ALL.len());
        self.initialized = true;
    }
}

fn selected<T: Copy>(values: &[T], index: usize, label: &str) -> Result<T, String> {
    values
        .get(index)
        .copied()
        .ok_or_else(|| format!("Operating-point {label} selection is invalid"))
}

fn index_of<T: PartialEq>(values: &[T], value: T) -> usize {
    values
        .iter()
        .position(|candidate| *candidate == value)
        .unwrap_or(0)
}

fn clamp_index(index: &mut usize, len: usize) {
    *index = (*index).min(len.saturating_sub(1));
}

fn default_temperature() -> String {
    "27".to_owned()
}
const fn missing_selection_index() -> usize {
    usize::MAX
}
const fn default_accuracy_idx() -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_editor_state_projects_to_the_default_op_contract() {
        let state = OpDialogState::default();
        assert_eq!(
            state.to_config().expect("default OP contract"),
            OpConfig::default()
        );
    }

    #[test]
    fn legacy_controls_migrate_and_never_serialize_again() {
        let mut restored: OpDialogState = serde_json::from_str(r#"{"save_all":false,"save_op_info":true,"temperature":"88","source_stepping":true,"gmin_steps":"0"}"#).unwrap();
        restored.prepare_after_restore();
        assert_eq!(restored.temperature, "88");
        assert_eq!(
            restored.temperature_mode_idx,
            index_of(&OpTemperatureMode::ALL, OpTemperatureMode::Explicit)
        );
        assert_eq!(restored.to_config().unwrap().temperature_celsius, 88.0);
        assert_eq!(restored.homotopy_idx, 1);
        assert_eq!(restored.annotation_idx, 3);
        assert_eq!(restored.device_detail_idx, 1);
        let encoded = serde_json::to_value(restored).unwrap();
        for retired in ["save_all", "save_op_info", "source_stepping", "gmin_steps"] {
            assert!(encoded.get(retired).is_none(), "{retired}");
        }
    }

    #[test]
    fn corrupt_legacy_gmin_steps_fails_closed() {
        for value in ["", "abc", "-1", "1.5"] {
            let mut restored: OpDialogState =
                serde_json::from_str(&format!(r#"{{"temperature":"27","gmin_steps":"{value}"}}"#))
                    .unwrap();
            restored.prepare_after_restore();
            assert!(
                restored.to_config().unwrap_err().contains("gmin_steps"),
                "legacy value {value:?} must not silently select a homotopy mode"
            );
        }
    }
}

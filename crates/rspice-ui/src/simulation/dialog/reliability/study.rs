//! Editable model snapshots, explicit bindings and ordered mission controls.

use super::parse_si_value;
use crate::simulation::reliability_engine::{
    ReliabilityBinding, ReliabilityMissionPhase, ReliabilityStudy, ReliabilityTransientWindow,
};
use rspice_core::analysis::reliability::{AgingModelPack, SECONDS_PER_AGING_YEAR};
#[cfg(not(target_arch = "wasm32"))]
use rspice_core::analysis::reliability::MAX_AGING_PACK_BYTES;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityBindingDraft {
    pub device: String,
    pub compact_model: String,
    pub aging_models: String,
    pub conductor_area_m2: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityMissionDraft {
    pub name: String,
    pub duration_s: String,
    pub temperature_c: String,
    pub parameters: String,
}

impl Default for ReliabilityMissionDraft {
    fn default() -> Self {
        Self {
            name: "nominal".into(),
            duration_s: SECONDS_PER_AGING_YEAR.to_string(),
            temperature_c: "27".into(),
            parameters: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityStudyDraft {
    /// The loaded content travels with the project; a mutable path is not calibration.
    pub model_pack_json: String,
    pub bindings: Vec<ReliabilityBindingDraft>,
    pub mission: Vec<ReliabilityMissionDraft>,
    pub repeat_mission: bool,
    pub transient_stress: bool,
    pub step_s: String,
    pub stop_s: String,
    pub start_s: String,
    pub max_step_s: String,
    pub use_initial_conditions: bool,
    #[serde(skip)]
    pub import_error: Option<String>,
}

impl Default for ReliabilityStudyDraft {
    fn default() -> Self {
        Self {
            model_pack_json: String::new(),
            bindings: Vec::new(),
            mission: vec![ReliabilityMissionDraft::default()],
            repeat_mission: true,
            transient_stress: false,
            step_s: "1n".into(),
            stop_s: "1u".into(),
            start_s: "0".into(),
            max_step_s: String::new(),
            use_initial_conditions: false,
            import_error: None,
        }
    }
}

impl ReliabilityStudyDraft {
    /// Import is transactional: a malformed file leaves the previous pack intact.
    pub fn import_model_pack(&mut self, json: &str) -> Result<(), String> {
        AgingModelPack::from_json(json).map_err(|error| error.to_string())?;
        self.model_pack_json = json.to_owned();
        self.import_error = None;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_model_pack(&mut self, path: &std::path::Path) -> Result<(), String> {
        use std::io::Read;
        let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
        let mut bytes = Vec::new();
        file.take(MAX_AGING_PACK_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| error.to_string())?;
        if bytes.len() > MAX_AGING_PACK_BYTES {
            return Err("Model pack exceeds 4 MiB".into());
        }
        let json = String::from_utf8(bytes)
            .map_err(|error| format!("Model pack must be UTF-8: {error}"))?;
        self.import_model_pack(&json)
    }

    pub fn from_study(study: Option<&ReliabilityStudy>) -> Self {
        let Some(study) = study else {
            return Self::default();
        };
        let mut draft = Self {
            model_pack_json: serde_json::to_string_pretty(&study.model_pack)
                .expect("validated finite model pack"),
            bindings: study
                .bindings
                .iter()
                .map(|binding| ReliabilityBindingDraft {
                    device: binding.device.clone(),
                    compact_model: binding.compact_model.clone(),
                    aging_models: binding.aging_models.join(", "),
                    conductor_area_m2: binding
                        .conductor_area_m2
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                })
                .collect(),
            mission: study
                .mission
                .iter()
                .map(|phase| ReliabilityMissionDraft {
                    name: phase.name.clone(),
                    duration_s: phase.duration_s.to_string(),
                    temperature_c: phase.temperature_c.to_string(),
                    parameters: phase
                        .parameters
                        .iter()
                        .map(|(name, value)| format!("{name}={value}"))
                        .collect::<Vec<_>>()
                        .join(", "),
                })
                .collect(),
            repeat_mission: study.repeat_mission,
            ..Self::default()
        };
        if let Some(window) = &study.transient_stress {
            draft.transient_stress = true;
            draft.step_s = window.step_s.to_string();
            draft.stop_s = window.stop_s.to_string();
            draft.start_s = window.start_s.to_string();
            draft.max_step_s = window.max_step_s.map(|v| v.to_string()).unwrap_or_default();
            draft.use_initial_conditions = window.use_initial_conditions;
        }
        draft
    }

    pub fn to_study(&self) -> Result<Option<ReliabilityStudy>, String> {
        if self.model_pack_json.trim().is_empty() {
            let mut comparison = self.clone();
            comparison.import_error = None;
            if comparison == Self::default() {
                return Ok(None);
            }
            return Err("Load an aging model pack before configuring its mission".into());
        }
        let model_pack =
            AgingModelPack::from_json(&self.model_pack_json).map_err(|error| error.to_string())?;
        let bindings = self
            .bindings
            .iter()
            .map(|binding| {
                Ok(ReliabilityBinding {
                    device: binding.device.trim().into(),
                    compact_model: binding.compact_model.trim().into(),
                    aging_models: binding
                        .aging_models
                        .split(|c: char| c == ',' || c.is_whitespace())
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                        .collect(),
                    conductor_area_m2: optional(&binding.conductor_area_m2, "Conductor area (m²)")?,
                })
            })
            .collect::<Result<_, String>>()?;
        let mission = self
            .mission
            .iter()
            .map(|phase| {
                let mut parameters = std::collections::BTreeMap::new();
                for assignment in phase
                    .parameters
                    .split([',', ';', '\n'])
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    let (name, value) = assignment.split_once('=').ok_or_else(|| {
                        "Mission overrides use NAME=value, separated by commas".to_owned()
                    })?;
                    if parameters
                        .insert(
                            name.trim().to_owned(),
                            number(value.trim(), "Mission parameter")?,
                        )
                        .is_some()
                    {
                        return Err(format!("Duplicate mission parameter {}", name.trim()));
                    }
                }
                Ok(ReliabilityMissionPhase {
                    name: phase.name.trim().into(),
                    duration_s: number(&phase.duration_s, "Phase duration (s)")?,
                    temperature_c: number(&phase.temperature_c, "Phase temperature (°C)")?,
                    parameters,
                })
            })
            .collect::<Result<_, String>>()?;
        let transient_stress = if self.transient_stress {
            Some(ReliabilityTransientWindow {
                step_s: number(&self.step_s, "Stress step (s)")?,
                stop_s: number(&self.stop_s, "Stress stop (s)")?,
                start_s: number(&self.start_s, "Stress start (s)")?,
                max_step_s: optional(&self.max_step_s, "Maximum step (s)")?,
                use_initial_conditions: self.use_initial_conditions,
            })
        } else {
            None
        };
        Ok(Some(ReliabilityStudy {
            model_pack,
            bindings,
            mission,
            repeat_mission: self.repeat_mission,
            transient_stress,
        }))
    }
}

fn number(value: &str, label: &str) -> Result<f64, String> {
    let parsed = parse_si_value(value).map_err(|error| format!("{label}: {error}"))?;
    if !parsed.is_finite() {
        return Err(format!("{label} must be finite"));
    }
    Ok(parsed)
}

fn optional(value: &str, label: &str) -> Result<Option<f64>, String> {
    if value.trim().is_empty() {
        Ok(None)
    } else {
        number(value, label).map(Some)
    }
}

//! Frozen calibration, device bindings and ordered mission-profile requests.

use super::{AgingLaw, AgingMechanism, AgingModelPack, SECONDS_PER_AGING_YEAR};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

/// Explicit assignment; no arbitrary process model is selected automatically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityBinding {
    pub device: String,
    /// Exact fresh compact-model identity, checked again against the circuit.
    pub compact_model: String,
    pub aging_models: Vec<String>,
    /// Required for EM; branch current alone is not a current density.
    pub conductor_area_m2: Option<f64>,
}

/// Representative circuit window repeated within each mission phase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityTransientWindow {
    pub step_s: f64,
    pub stop_s: f64,
    pub start_s: f64,
    pub max_step_s: Option<f64>,
    pub use_initial_conditions: bool,
}

impl ReliabilityTransientWindow {
    fn validate(&self) -> Result<(), String> {
        positive(self.step_s, "Stress step")?;
        positive(self.stop_s, "Stress stop")?;
        if !self.start_s.is_finite() || self.start_s < 0.0 || self.start_s >= self.stop_s {
            return Err("Stress start must be finite and in [0, stop)".into());
        }
        if self.step_s > self.stop_s - self.start_s {
            return Err("Stress step must not exceed the observed window".into());
        }
        if let Some(value) = self.max_step_s {
            positive(value, "Stress maximum step")?;
        }
        Ok(())
    }
}

/// A phase has its own supply/design overrides and temperature. Phase order
/// is retained, including a partial final phase at a lifetime checkpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityMissionPhase {
    pub name: String,
    pub duration_s: f64,
    pub temperature_c: f64,
    pub parameters: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityStudy {
    pub model_pack: AgingModelPack,
    pub bindings: Vec<ReliabilityBinding>,
    pub mission: Vec<ReliabilityMissionPhase>,
    /// False requires the explicit mission to cover the last lifetime checkpoint.
    pub repeat_mission: bool,
    /// None obtains constant stresses from an operating point for each phase.
    pub transient_stress: Option<ReliabilityTransientWindow>,
}

/// Complete circuit-facing reliability request, including enabled mechanisms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityRunRequest {
    pub study: ReliabilityStudy,
    pub target_years: Vec<f64>,
    pub enable_hci: bool,
    pub enable_nbti: bool,
    pub enable_em: bool,
    /// Suspend transistor aging below this gate-stress magnitude; EM is independent.
    pub min_stress_voltage: f64,
}

impl ReliabilityRunRequest {
    pub fn validate(&self) -> Result<(), String> {
        if !self.enable_hci && !self.enable_nbti && !self.enable_em {
            return Err("Reliability requires at least one enabled mechanism".into());
        }
        if !self.min_stress_voltage.is_finite() || self.min_stress_voltage < 0.0 {
            return Err("Minimum stress voltage must be finite and nonnegative".into());
        }
        self.study.validate(
            &self.target_years,
            self.enable_hci,
            self.enable_nbti,
            self.enable_em,
        )
    }
}

impl ReliabilityStudy {
    pub fn validate(&self, years: &[f64], hci: bool, nbti: bool, em: bool) -> Result<(), String> {
        self.model_pack
            .validate()
            .map_err(|error| error.to_string())?;
        if self.bindings.is_empty() || self.bindings.len() > 4096 {
            return Err("Reliability requires 1..=4096 explicit device bindings".into());
        }
        let mut devices = HashSet::new();
        let mut covered = [false; 3];
        for binding in &self.bindings {
            identifier(&binding.device, "Device")?;
            identifier(&binding.compact_model, "Compact model")?;
            if !devices.insert(binding.device.to_ascii_lowercase()) {
                return Err(format!("Duplicate reliability device {}", binding.device));
            }
            if binding.aging_models.is_empty() || binding.aging_models.len() > 3 {
                return Err(format!("{} needs 1..=3 aging models", binding.device));
            }
            if let Some(area) = binding.conductor_area_m2 {
                positive(area, "Conductor cross section")?;
            }
            let mut mechanisms = HashSet::new();
            let mut parameter_modes = BTreeMap::new();
            for id in &binding.aging_models {
                let model = self
                    .model_pack
                    .models
                    .iter()
                    .find(|model| model.id == *id)
                    .ok_or_else(|| format!("Unknown aging model '{id}' for {}", binding.device))?;
                let index = match model.mechanism {
                    AgingMechanism::Hci => 0,
                    AgingMechanism::Nbti => 1,
                    AgingMechanism::Electromigration => 2,
                };
                if ![hci, nbti, em][index] {
                    return Err(format!("Aging model '{id}' uses a disabled mechanism"));
                }
                if !mechanisms.insert(index) {
                    return Err(format!(
                        "{} has multiple models for one mechanism",
                        binding.device
                    ));
                }
                covered[index] = true;
                if index == 2 && binding.conductor_area_m2.is_none() {
                    return Err(format!(
                        "{} needs a conductor cross section for EM",
                        binding.device
                    ));
                }
                if let AgingLaw::EquivalentTimePower { parameters, .. } = &model.law {
                    for parameter in parameters {
                        let prior = parameter_modes
                            .insert(parameter.parameter.to_ascii_lowercase(), parameter.update);
                        if prior.is_some_and(|prior| prior != parameter.update) {
                            return Err(format!(
                                "{} mixes additive and relative updates of {}",
                                binding.device, parameter.parameter
                            ));
                        }
                    }
                }
            }
        }
        for (index, enabled) in [hci, nbti, em].into_iter().enumerate() {
            if enabled && !covered[index] {
                return Err(format!(
                    "Enabled {} has no bound calibration",
                    ["HCI", "NBTI", "electromigration"][index]
                ));
            }
        }
        if self.mission.is_empty() || self.mission.len() > 4096 {
            return Err("Reliability requires 1..=4096 mission phases".into());
        }
        let mut total = 0.0;
        for phase in &self.mission {
            identifier(&phase.name, "Mission phase")?;
            positive(phase.duration_s, "Mission phase duration")?;
            if !phase.temperature_c.is_finite() || phase.temperature_c <= -273.15 {
                return Err("Mission temperature must be finite and above absolute zero".into());
            }
            let mut names = HashSet::new();
            if phase.parameters.len() > 4096 {
                return Err("Too many mission parameters".into());
            }
            for (name, value) in &phase.parameters {
                identifier(name, "Mission parameter")?;
                if !value.is_finite() || !names.insert(name.to_ascii_lowercase()) {
                    return Err(format!(
                        "Mission parameter '{name}' is non-finite or duplicated"
                    ));
                }
            }
            let next = total + phase.duration_s;
            if !next.is_finite() || next == total {
                return Err("Mission duration is not representable".into());
            }
            total = next;
        }
        if years.is_empty() || years.windows(2).any(|w| w[0] >= w[1]) {
            return Err("Reliability checkpoints must be nonempty and strictly increasing".into());
        }
        for year in years {
            positive(*year, "Lifetime checkpoint")?;
            let seconds = year * SECONDS_PER_AGING_YEAR;
            positive(seconds, "Lifetime checkpoint in seconds")?;
            if !self.repeat_mission && seconds > total {
                return Err("Mission duration does not cover all lifetime checkpoints; extend it or enable repetition".into());
            }
        }
        if let Some(window) = &self.transient_stress {
            window.validate()?;
        }
        Ok(())
    }
}

fn positive(value: f64, label: &str) -> Result<(), String> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(format!("{label} must be finite and positive"))
    }
}

fn identifier(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 512
        || value.chars().any(|c| c.is_control() || c.is_whitespace())
    {
        Err(format!(
            "{label} must be a nonempty identifier without whitespace"
        ))
    } else {
        Ok(())
    }
}

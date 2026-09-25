//! Authored retention policy and immutable references to prior trial journals.
use rspice_app_types::product::ContentDigest;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
pub struct McCheckpointConfig {
    pub publish_every: NonZeroUsize,
    pub resume: Vec<ContentDigest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct McCheckpointDraft {
    pub retain_trials: bool,
    pub publish_every: String,
    pub resume: Vec<ContentDigest>,
}
impl Default for McCheckpointDraft {
    fn default() -> Self {
        Self {
            retain_trials: false,
            publish_every: "10".into(),
            resume: Vec::new(),
        }
    }
}
impl McCheckpointDraft {
    pub fn from_config(config: Option<&McCheckpointConfig>) -> Self {
        config.map_or_else(Self::default, |config| Self {
            retain_trials: true,
            publish_every: config.publish_every.to_string(),
            resume: config.resume.clone(),
        })
    }
    pub fn to_config(&self) -> Result<Option<McCheckpointConfig>, String> {
        if !self.retain_trials {
            return Ok(None);
        }
        let publish_every = self
            .publish_every
            .trim()
            .parse::<NonZeroUsize>()
            .map_err(|_| "Checkpoint interval must be a positive whole number of trials")?;
        let mut unique = std::collections::HashSet::new();
        if self.resume.iter().any(|digest| !unique.insert(*digest)) {
            return Err("A Monte Carlo checkpoint is selected more than once".into());
        }
        Ok(Some(McCheckpointConfig {
            publish_every,
            resume: self.resume.clone(),
        }))
    }
}

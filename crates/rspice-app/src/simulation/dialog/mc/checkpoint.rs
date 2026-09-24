//! Authored retention policy and immutable references to prior trial journals.
use crate::product::ContentDigest;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::dialog::mc::{McConfig, McDialogState};
    use crate::simulation::plan::AnalysisDraft;
    #[test]
    fn monte_carlo_checkpoint_controls_round_trip_and_keep_inactive_buffers() {
        let mut draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
            base_analysis: Some(crate::product::AnalysisInstanceId::new()),
            measurements: vec!["scalar:V(out)".into()],
            checkpoint: Some(McCheckpointConfig {
                publish_every: 7.try_into().unwrap(),
                resume: vec![ContentDigest::from_bytes([8; 32])],
            }),
            ..Default::default()
        }));
        for mut restored in [
            serde_json::from_str::<AnalysisDraft>(&serde_json::to_string(&draft).unwrap()).unwrap(),
            ron::from_str::<AnalysisDraft>(&ron::to_string(&draft).unwrap()).unwrap(),
        ] {
            restored.prepare_after_restore();
            let AnalysisDraft::MonteCarlo(restored) = restored else {
                panic!("MC")
            };
            let policy = restored.to_config().unwrap().checkpoint.unwrap();
            assert_eq!(policy.publish_every.get(), 7);
            assert_eq!(policy.resume, [ContentDigest::from_bytes([8; 32])]);
        }
        let AnalysisDraft::MonteCarlo(state) = &mut draft else {
            panic!("MC")
        };
        state.checkpoint.publish_every = "unfinished".into();
        assert!(state.to_config().is_err());
        state.checkpoint.retain_trials = false;
        assert!(state.to_config().unwrap().checkpoint.is_none());
        assert_eq!(state.checkpoint.resume.len(), 1);
        let mut old = serde_json::to_value(state).unwrap();
        old.as_object_mut().unwrap().remove("checkpoint");
        assert!(
            !serde_json::from_value::<McDialogState>(old)
                .unwrap()
                .checkpoint
                .retain_trials
        );
    }
}

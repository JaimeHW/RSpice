//! Persisted plan-owned output selection and retention policy.

/// Plan-level rule for choosing which simulation quantities enter retained
/// result datasets. This is intentionally separate from each output's save
/// policy: the mode chooses the set, while `SavedOutputPolicy` controls how an
/// item in that set is sampled and stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OutputSelectionMode {
    /// Every node voltage and source current the engine computes, at every
    /// level of the hierarchy, beside the plan's own outputs and probes.
    #[default]
    SaveAll,
    /// Only the plan's own outputs and the schematic's probes.
    ExplicitOnly,
}

impl OutputSelectionMode {
    pub const ALL: [Self; 2] = [Self::SaveAll, Self::ExplicitOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::SaveAll => "Save all",
            Self::ExplicitOnly => "Selected only",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::SaveAll => "Every node voltage and source current, at every level",
            Self::ExplicitOnly => "Only the plan's outputs and schematic probes",
        }
    }
}

/// Plan-owned result delivery and retention policy. These controls are part of
/// the executable plan rather than project-global UI preferences: switching a
/// plan switches the policy, and a prepared snapshot authenticates it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationSavePolicy {
    /// How the plan chooses the quantities retained in each result dataset.
    #[serde(default)]
    pub output_selection_mode: OutputSelectionMode,
    /// Maximum retained datasets produced by this plan. Golden baselines are
    /// exempt and may make the limit temporarily unenforceable.
    pub retained_dataset_limit: usize,
    /// Hard preflight ceiling for the bounded saved-output forecast.
    pub maximum_storage_bytes: u64,
    /// Whether contracts requesting live delivery may open a live stream.
    pub live_streaming_enabled: bool,
    /// Whether accepted transient samples are retained as failure diagnostics
    /// if the final solve fails or is interrupted.
    pub retain_failure_diagnostics: bool,
}

impl Default for SimulationSavePolicy {
    fn default() -> Self {
        Self {
            output_selection_mode: OutputSelectionMode::SaveAll,
            retained_dataset_limit: 20,
            maximum_storage_bytes: 10 * 1024 * 1024 * 1024,
            live_streaming_enabled: true,
            retain_failure_diagnostics: true,
        }
    }
}

impl SimulationSavePolicy {
    pub fn validate(self) -> Result<(), String> {
        if self.retained_dataset_limit == 0 || self.retained_dataset_limit > 10_000 {
            return Err("Plan retention must be from 1 through 10,000 datasets.".to_owned());
        }
        if self.maximum_storage_bytes == 0 {
            return Err("Plan saved-output storage budget must be greater than zero.".to_owned());
        }
        Ok(())
    }
}

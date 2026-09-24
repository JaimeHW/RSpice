//! Persisted plan-owned output selection and retention policy.

/// Plan-level rule for choosing which simulation quantities enter retained
/// result datasets. This is intentionally separate from each output's save
/// policy: the mode chooses the set, while `SavedOutputPolicy` controls how an
/// item in that set is sampled and stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OutputSelectionMode {
    /// Use explicit saved outputs when present; otherwise synthesize a small,
    /// deterministic set of useful top-level node voltages.
    #[default]
    Automatic,
    /// Retain only outputs explicitly owned by the simulation plan.
    ExplicitOnly,
    /// Retain every result quantity produced by the selected engine analyses.
    SaveAll,
}

impl OutputSelectionMode {
    pub const ALL: [Self; 3] = [Self::Automatic, Self::ExplicitOnly, Self::SaveAll];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Automatic => "Automatic",
            Self::ExplicitOnly => "Explicit only",
            Self::SaveAll => "Save all",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Automatic => "Explicit outputs, or a bounded top-level fallback when none exist",
            Self::ExplicitOnly => "Only plan outputs and schematic probes",
            Self::SaveAll => "Every engine-produced quantity, subject to the storage ceiling",
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
            output_selection_mode: OutputSelectionMode::Automatic,
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

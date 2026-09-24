//! Persisted per-analysis participation in a declared run space.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Which points of the declared space an analysis instance runs at.
///
/// The default is [`Self::AllPoints`], and it has to stay that: a plan authored
/// before participation existed ran every analysis at every point, and a
/// project that reloaded into any other value would narrow a run nobody
/// narrowed.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisRunAt {
    /// Every point the run set resolves.
    #[default]
    AllPoints,
    /// The reference point alone. Which point that is, is decided by
    /// `nominal_point_key` rather than by anything stored here, so the
    /// setting survives an edit that moves the reference.
    NominalPoint,
    /// Exactly the named points, by `RunSetPoint::point_key`.
    SelectedPoints(Vec<String>),
}

impl AnalysisRunAt {
    /// Whether serialization may leave the field out entirely.
    #[must_use]
    pub const fn is_all_points(&self) -> bool {
        matches!(self, Self::AllPoints)
    }

    /// How this participation is spelled in a control and in a receipt.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::AllPoints => "All run-set points",
            Self::NominalPoint => "Nominal point only",
            Self::SelectedPoints(_) => "Selected points",
        }
    }

    /// Reject a selection that names nothing, or names one point twice.
    ///
    /// An instance that runs nowhere contributes no evidence and no task, which
    /// is what disabling the instance already says plainly. Storing it as a
    /// participation instead would make a disabled analysis look enabled in
    /// every count on the page.
    pub fn validate(&self) -> Result<(), String> {
        let Self::SelectedPoints(keys) = self else {
            return Ok(());
        };
        if keys.is_empty() {
            return Err(
                "A point selection must name at least one point; an analysis that runs nowhere \
                 produces no evidence. Disable the instance instead."
                    .to_owned(),
            );
        }
        if let Some(index) = keys.iter().position(|key| key.trim().is_empty()) {
            return Err(format!(
                "Point selection entry {} is empty; every entry is one resolved point identity.",
                index + 1
            ));
        }
        let unique: HashSet<&String> = keys.iter().collect();
        if unique.len() != keys.len() {
            return Err(
                "A point selection names each point once; a repeated identity would price one \
                 point as two tasks."
                    .to_owned(),
            );
        }
        Ok(())
    }
}

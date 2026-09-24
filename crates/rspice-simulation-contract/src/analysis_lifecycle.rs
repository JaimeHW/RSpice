//! Stable analysis lifecycle and receipt command vocabulary.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Editable-plan lifecycle of one analysis instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisLifecycleState {
    Absent,
    Draft,
    Invalid,
    Ready,
    PreflightReady,
    Blocked,
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Disabled,
    Removed,
    SameState,
}

impl AnalysisLifecycleState {
    pub const fn is_executing(self) -> bool {
        matches!(self, Self::Queued | Self::Running | Self::Paused)
    }
}

impl fmt::Display for AnalysisLifecycleState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Absent => "absent",
            Self::Draft => "draft",
            Self::Invalid => "invalid",
            Self::Ready => "ready",
            Self::PreflightReady => "preflight-ready",
            Self::Blocked => "blocked",
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Disabled => "disabled",
            Self::Removed => "removed",
            Self::SameState => "same-state",
        })
    }
}

/// Stable command vocabulary recorded by plan mutation receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisLifecycleCommand {
    Insert,
    Edit,
    Rename,
    Clone,
    Enable,
    Disable,
    Reorder,
    Dependency,
    Validate,
    Preflight,
    Execute,
    Remove,
}

impl fmt::Display for AnalysisLifecycleCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Insert => "insert",
            Self::Edit => "edit",
            Self::Rename => "rename",
            Self::Clone => "clone",
            Self::Enable => "enable",
            Self::Disable => "disable",
            Self::Reorder => "reorder",
            Self::Dependency => "dependency",
            Self::Validate => "validate",
            Self::Preflight => "preflight",
            Self::Execute => "execute",
            Self::Remove => "remove",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_vocabulary_matches_the_normative_mockup_ids() {
        let states = [
            (AnalysisLifecycleState::Absent, "absent"),
            (AnalysisLifecycleState::Draft, "draft"),
            (AnalysisLifecycleState::Invalid, "invalid"),
            (AnalysisLifecycleState::Ready, "ready"),
            (AnalysisLifecycleState::PreflightReady, "preflight-ready"),
            (AnalysisLifecycleState::Blocked, "blocked"),
            (AnalysisLifecycleState::Queued, "queued"),
            (AnalysisLifecycleState::Running, "running"),
            (AnalysisLifecycleState::Paused, "paused"),
            (AnalysisLifecycleState::Completed, "completed"),
            (AnalysisLifecycleState::Failed, "failed"),
            (AnalysisLifecycleState::Cancelled, "cancelled"),
            (AnalysisLifecycleState::Disabled, "disabled"),
            (AnalysisLifecycleState::Removed, "removed"),
            (AnalysisLifecycleState::SameState, "same-state"),
        ];
        for (state, stable_id) in states {
            assert_eq!(
                serde_json::to_string(&state).expect("state serializes"),
                format!("\"{stable_id}\"")
            );
            assert_eq!(state.to_string(), stable_id);
        }

        let commands = [
            (AnalysisLifecycleCommand::Insert, "insert"),
            (AnalysisLifecycleCommand::Edit, "edit"),
            (AnalysisLifecycleCommand::Clone, "clone"),
            (AnalysisLifecycleCommand::Enable, "enable"),
            (AnalysisLifecycleCommand::Disable, "disable"),
            (AnalysisLifecycleCommand::Reorder, "reorder"),
            (AnalysisLifecycleCommand::Dependency, "dependency"),
            (AnalysisLifecycleCommand::Validate, "validate"),
            (AnalysisLifecycleCommand::Preflight, "preflight"),
            (AnalysisLifecycleCommand::Execute, "execute"),
            (AnalysisLifecycleCommand::Remove, "remove"),
        ];
        for (command, stable_id) in commands {
            assert_eq!(
                serde_json::to_string(&command).expect("command serializes"),
                format!("\"{stable_id}\"")
            );
            assert_eq!(command.to_string(), stable_id);
        }
    }
}

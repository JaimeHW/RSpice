//! Persisted run-set forecast, transaction evidence, and nominal reference.

use rspice_app_types::product::ProcessCorner;

/// What the composed space costs, exactly.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RunSetForecast {
    /// Maximum points the space resolves to. Equal to the minimum for every
    /// deterministic composition.
    pub point_count: usize,
    #[serde(default)]
    pub point_count_minimum: usize,
    #[serde(default)]
    pub point_count_maximum: usize,
    #[serde(default = "default_forecast_exact")]
    pub exact: bool,
    /// Enabled analysis instances, each contributing one task per point.
    pub enabled_analysis_count: usize,
    /// `point_count × enabled_analysis_count`.
    pub task_count: usize,
    /// Modelled solve cost, in milliseconds.
    pub cost_ms: u64,
    /// Modelled stored bytes.
    pub storage_bytes: u64,
}

const fn default_forecast_exact() -> bool {
    true
}

/// Whether the run set may be previewed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunSetStatus {
    /// No validate-and-preview has run since the last edit.
    #[default]
    NotEvaluated,
    /// The declaration is executable exactly as written.
    Ready,
    /// At least one refusal stands.
    Invalid,
}

impl RunSetStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotEvaluated => "not evaluated",
            Self::Ready => "ready",
            Self::Invalid => "invalid",
        }
    }
}

/// Whether a transaction took effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunSetReceiptStatus {
    Completed,
    Blocked,
}

impl RunSetReceiptStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Blocked => "blocked",
        }
    }
}

/// The record one transaction leaves behind.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSetReceipt {
    pub sequence: usize,
    pub action: &'static str,
    pub target_id: String,
    pub before_revision: u32,
    pub after_revision: u32,
    pub status: RunSetReceiptStatus,
    pub error_ids: Vec<&'static str>,
    /// FNV-1a/64 over the fields above, so two identical transactions produce
    /// the same digest and any difference produces a different one.
    pub digest: String,
}

impl RunSetReceipt {
    /// The one-line form the page shows.
    #[must_use]
    pub fn status_line(&self) -> String {
        match self.status {
            RunSetReceiptStatus::Completed => format!(
                "Run-set receipt #{} · {} · revision {} to {}",
                self.sequence, self.action, self.before_revision, self.after_revision
            ),
            RunSetReceiptStatus::Blocked => format!(
                "Run-set {} blocked · {} · revision {} retained",
                self.action,
                if self.error_ids.is_empty() {
                    "validation failed".to_owned()
                } else {
                    self.error_ids.join(", ")
                },
                self.before_revision
            ),
        }
    }
}

/// The plan's nominal point.
///
/// A dimension the run set does not declare still has to resolve to something
/// the executor can run; it resolves to this, so an undeclared axis means "the
/// plan's reference value" rather than a constant this module invented. It is
/// the same point the workbench chrome selects and the solver's `TEMP` option
/// carries — `ReferencePvtPoint` is this type, not a copy of it.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencePoint {
    pub process: ProcessCorner,
    pub temperature_celsius: f64,
}

impl Default for ReferencePoint {
    fn default() -> Self {
        Self {
            process: ProcessCorner::TT,
            temperature_celsius: 27.0,
        }
    }
}

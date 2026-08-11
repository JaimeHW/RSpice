//! Fail-closed validation of a run set, and the forecast it resolves to.
//!
//! Validation answers one question: may this space be previewed and dispatched
//! exactly as declared? Every refusal carries a stable identity so the page can
//! point at the control that caused it, and so a refusal keeps its meaning when
//! its wording changes.

use serde::{Deserialize, Serialize};

use super::model::{InvalidValuePolicy, RunSetCompositionMode, RunSetDimensionKind, RunSetState};

/// One refusal, identified by a stable code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunSetError {
    /// Stable code, e.g. `RUNSET-ZIP-LENGTH`.
    pub id: &'static str,
    /// The dimension the refusal is about, when it is about one.
    pub dimension_id: Option<String>,
    /// What is wrong, in the words the user needs to fix it.
    pub message: String,
}

impl RunSetError {
    fn global(id: &'static str, message: impl Into<String>) -> Self {
        Self {
            id,
            dimension_id: None,
            message: message.into(),
        }
    }

    fn about(id: &'static str, dimension_id: &str, message: impl Into<String>) -> Self {
        Self {
            id,
            dimension_id: Some(dimension_id.to_owned()),
            message: message.into(),
        }
    }
}

/// An advisory that does not block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunSetWarning {
    pub id: &'static str,
    pub message: String,
}

/// What the composed space costs, exactly.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RunSetForecast {
    /// Points the space resolves to.
    pub point_count: usize,
    /// Enabled analysis instances, each contributing one task per point.
    pub enabled_analysis_count: usize,
    /// `point_count × enabled_analysis_count`.
    pub task_count: usize,
    /// Modelled solve cost, in milliseconds.
    pub cost_ms: u64,
    /// Modelled stored bytes.
    pub storage_bytes: u64,
}

/// Whether the run set may be previewed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
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

/// The complete result of validating a run set.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSetValidation {
    pub status: RunSetStatus,
    pub errors: Vec<RunSetError>,
    pub warnings: Vec<RunSetWarning>,
    pub forecast: RunSetForecast,
}

impl RunSetValidation {
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.status == RunSetStatus::Ready
    }
}

/// Validate `state` against a plan with `enabled_analysis_count` analyses.
///
/// The analysis count is a parameter rather than a field because it is owned by
/// the analysis plan; a run set that stored its own copy would report a task
/// count the plan had already contradicted.
#[must_use]
pub fn validate(state: &RunSetState, enabled_analysis_count: usize) -> RunSetValidation {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    let mut seen_dimension_ids: Vec<&str> = Vec::new();
    let mut seen_value_ids: Vec<&str> = Vec::new();
    let mut seen_kinds: Vec<RunSetDimensionKind> = Vec::new();

    for dimension in &state.dimensions {
        if dimension.id.trim().is_empty() || seen_dimension_ids.contains(&dimension.id.as_str()) {
            errors.push(RunSetError::about(
                "RUNSET-DIMENSION-ID",
                &dimension.id,
                "Dimension identities must be present and unique.",
            ));
        }
        seen_dimension_ids.push(&dimension.id);

        for value in &dimension.values {
            if value.id.trim().is_empty() || seen_value_ids.contains(&value.id.as_str()) {
                errors.push(RunSetError::about(
                    "RUNSET-VALUE-ID",
                    &dimension.id,
                    "Value identities must be present and globally unique.",
                ));
            }
            seen_value_ids.push(&value.id);
        }

        if !dimension.enabled {
            continue;
        }

        if seen_kinds.contains(&dimension.kind) {
            errors.push(RunSetError::about(
                "RUNSET-DIMENSION-KIND",
                &dimension.id,
                format!(
                    "The engine binds one {} per point, so only one {} dimension may be enabled.",
                    dimension.kind.as_str(),
                    dimension.kind.as_str()
                ),
            ));
        }
        seen_kinds.push(dimension.kind);

        if dimension.values.is_empty() {
            errors.push(RunSetError::about(
                "RUNSET-EMPTY-DIMENSION",
                &dimension.id,
                "Enabled dimensions require at least one typed value.",
            ));
        }

        let invalid: Vec<&str> = dimension
            .values
            .iter()
            .filter(|value| value.canonical.is_none())
            .map(|value| value.lexical.as_str())
            .collect();
        if !invalid.is_empty() {
            let detail = invalid.join(", ");
            let message = match dimension.kind {
                RunSetDimensionKind::ProcessSection => format!(
                    "{} is not a process section; expected one of {}.",
                    detail,
                    super::model::PROCESS_SECTIONS.join(", ")
                ),
                RunSetDimensionKind::Supply => {
                    format!("{detail} is not a positive supply voltage.")
                }
                RunSetDimensionKind::Temperature => {
                    format!("{detail} is not a temperature above absolute zero.")
                }
            };
            match dimension.invalid_value_policy {
                InvalidValuePolicy::BlockEntireRunSet => errors.push(RunSetError::about(
                    "RUNSET-VALUE-SOURCE",
                    &dimension.id,
                    format!("{message} The dimension blocks the entire run set."),
                )),
                InvalidValuePolicy::PreserveAndBlockAffectedPoints => {
                    errors.push(RunSetError::about(
                        "RUNSET-VALUE-SOURCE",
                        &dimension.id,
                        format!("{message} Its points are blocked and no task matrix is created."),
                    ));
                }
            }
        }
    }

    let lengths: Vec<usize> = state
        .enabled_dimensions()
        .map(|dimension| dimension.values.len().max(1))
        .collect();

    if state.composition.mode == RunSetCompositionMode::Zipped {
        let mut non_scalar: Vec<usize> = lengths.iter().copied().filter(|len| *len != 1).collect();
        non_scalar.sort_unstable();
        non_scalar.dedup();
        if non_scalar.len() > 1 {
            errors.push(RunSetError::global(
                "RUNSET-ZIP-LENGTH",
                format!(
                    "Zipped dimensions require equal non-scalar lengths; this run set declares {}. \
                     Implicit cycling is prohibited.",
                    non_scalar
                        .iter()
                        .map(usize::to_string)
                        .collect::<Vec<_>>()
                        .join(" and ")
                ),
            ));
        }
    }

    let composed_count = if lengths.is_empty() {
        1
    } else if state.composition.mode == RunSetCompositionMode::Zipped {
        lengths.iter().copied().max().unwrap_or(1)
    } else {
        lengths
            .iter()
            .try_fold(1usize, |total, length| total.checked_mul(*length))
            .unwrap_or(usize::MAX)
    };

    let excluded = classify_exclusions(state);
    if !excluded.unknown.is_empty() {
        // A warning, not a refusal: widening or retyping an axis legitimately
        // strands an exclusion, and blocking the whole space until a key that
        // names nothing is hunted down would punish the edit rather than the
        // mistake. The exclusion is kept and named, so restoring the values it
        // was placed on restores it.
        warnings.push(RunSetWarning {
            id: "RUNSET-EXCLUSION-UNKNOWN",
            message: format!(
                "{} of the declared exclusions name a point this space no longer contains, so they \
                 subtract nothing: {}. They are retained by identity and apply again if those \
                 values return.",
                excluded.unknown.len(),
                excluded.unknown.join("; ")
            ),
        });
    }

    let point_count = composed_count.saturating_sub(excluded.applied);
    if point_count == 0 {
        errors.push(RunSetError::global(
            "RUNSET-ALL-POINTS-EXCLUDED",
            format!(
                "Every one of the {composed_count} composed points is excluded, so the run set \
                 resolves to nothing executable. Restore at least one point or widen an axis."
            ),
        ));
    }

    let analyses = enabled_analysis_count.max(1);
    let task_count = point_count.checked_mul(analyses);
    if task_count.is_none() {
        errors.push(RunSetError::global(
            "RUNSET-CARDINALITY-OVERFLOW",
            "Exact task cardinality exceeds safe deterministic representation.",
        ));
    }
    let task_count = task_count.unwrap_or(usize::MAX);

    if task_count > state.budgets.maximum_tasks {
        errors.push(RunSetError::global(
            "RUNSET-TASK-BUDGET",
            format!(
                "{task_count} tasks exceed the declared {} task budget.",
                state.budgets.maximum_tasks
            ),
        ));
    }

    let storage_bytes = (task_count as u64).saturating_mul(state.budgets.bytes_per_point);
    if storage_bytes > state.budgets.maximum_storage_bytes {
        errors.push(RunSetError::global(
            "RUNSET-STORAGE-BUDGET",
            format!(
                "{} exceed the declared {} storage budget.",
                super::format_bytes(storage_bytes),
                super::format_bytes(state.budgets.maximum_storage_bytes)
            ),
        ));
    }

    if state
        .enabled_dimension_of(RunSetDimensionKind::ProcessSection)
        .is_none()
        && state.dimensions.iter().any(|dimension| {
            dimension.kind == RunSetDimensionKind::ProcessSection && !dimension.enabled
        })
    {
        warnings.push(RunSetWarning {
            id: "RUNSET-NO-PROCESS-AXIS",
            message:
                "No process axis is enabled, so every point resolves through the plan's reference \
                 model section."
                    .to_owned(),
        });
    }

    if lengths.is_empty() {
        warnings.push(RunSetWarning {
            id: "RUNSET-NO-DIMENSIONS",
            message: "The run set resolves to one nominal point because no dimensions are enabled."
                .to_owned(),
        });
    }

    RunSetValidation {
        status: if errors.is_empty() {
            RunSetStatus::Ready
        } else {
            RunSetStatus::Invalid
        },
        errors,
        warnings,
        forecast: RunSetForecast {
            point_count,
            enabled_analysis_count: analyses,
            task_count,
            cost_ms: (task_count as u64).saturating_mul(state.budgets.cost_per_point_ms),
            storage_bytes,
        },
    }
}

/// How the declared exclusions stand against the current space.
#[derive(Default)]
struct ExclusionStanding {
    /// Exclusions that name a point the space contains, and so shorten it.
    applied: usize,
    /// Exclusions that name no point of the current space.
    unknown: Vec<String>,
}

/// Sort the exclusions into the ones that remove a point and the ones that no
/// longer name one.
///
/// Only a filtered composition subtracts anything, so the other modes report an
/// empty standing: their exclusions are stored, not applied, and reporting them
/// as stranded would refuse a space that is composing exactly as declared.
fn classify_exclusions(state: &RunSetState) -> ExclusionStanding {
    if state.composition.mode != RunSetCompositionMode::Filtered {
        return ExclusionStanding::default();
    }
    let mut standing = ExclusionStanding::default();
    for key in &state.composition.excluded_points {
        if super::points::contains_point_key(state, key) {
            standing.applied += 1;
        } else {
            standing.unknown.push(key.clone());
        }
    }
    standing
}

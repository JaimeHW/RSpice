//! Fail-closed validation of a run set, and the forecast it resolves to.
//!
//! Validation answers one question: may this space be previewed and dispatched
//! exactly as declared? Every refusal carries a stable identity so the page can
//! point at the control that caused it, and so a refusal keeps its meaning when
//! its wording changes.

use serde::{Deserialize, Serialize};

use super::model::{InvalidValuePolicy, RunSetCompositionMode, RunSetDimensionKind, RunSetState};
use crate::simulation::plan::AnalysisKind;

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

    pub(crate) fn push_global_error(&mut self, id: &'static str, message: impl Into<String>) {
        self.errors.push(RunSetError::global(id, message));
        self.status = RunSetStatus::Invalid;
    }
}

/// Validate the Run Set against the enabled analysis kinds and exact queue
/// cardinality of the active plan.
#[must_use]
pub fn validate_for_plan(
    state: &RunSetState,
    enabled_analysis_kinds: &[AnalysisKind],
    exact_task_count: Option<usize>,
) -> RunSetValidation {
    let mut validation =
        validate_with_task_count(state, enabled_analysis_kinds.len(), exact_task_count);
    if enabled_analysis_kinds.is_empty() {
        validation.push_global_error(
            "RUNSET-PLAN-EMPTY",
            "Enable at least one analysis instance before previewing the Run Set.",
        );
    }
    let has_axes = state.enabled_dimensions().next().is_some();
    if !has_axes {
        return validation;
    }
    for kind in enabled_analysis_kinds.iter().copied() {
        let nested_declaration = matches!(kind, AnalysisKind::Temperature | AnalysisKind::Corner);
        if nested_declaration {
            validation.push_global_error(
                "RUNSET-ANALYSIS-COMPOSITION",
                format!(
                    "{} owns an internal point declaration and cannot also execute across enabled global Run Set axes. Disable the global axes or disable this nested declaration; an implicit cross-product of two point authorities is prohibited.",
                    kind.label()
                ),
            );
        }
    }
    validation
}

/// Validate `state` against a plan with `enabled_analysis_count` analyses.
///
/// The analysis count is a parameter rather than a field because it is owned by
/// the analysis plan; a run set that stored its own copy would report a task
/// count the plan had already contradicted.
#[must_use]
pub fn validate(state: &RunSetState, enabled_analysis_count: usize) -> RunSetValidation {
    validate_with_task_count(state, enabled_analysis_count, None)
}

/// Validate with an exact plan-aware queue cardinality.
///
/// Most analyses contribute one queue task per Run Set point, so callers can
/// use [`validate`]. Temperature and Corner declarations additionally create
/// their point solves and one family-assembly task when the global space is
/// reference-only. The Simulation Studio computes that exact count from the
/// enabled drafts and supplies it here so budgets and receipts describe the
/// queue preflight will actually authorize.
#[must_use]
pub fn validate_with_task_count(
    state: &RunSetState,
    enabled_analysis_count: usize,
    exact_task_count: Option<usize>,
) -> RunSetValidation {
    #[cfg(test)]
    crate::simulation::cost_probe::record(
        crate::simulation::cost_probe::Derivation::RunSetValidation,
    );
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    let mut seen_dimension_ids: Vec<&str> = Vec::new();
    let mut seen_value_ids: Vec<&str> = Vec::new();
    let mut seen_kinds: Vec<RunSetDimensionKind> = Vec::new();
    let mut parameter_authorities: Vec<(&str, String)> = Vec::new();
    let mut source_authorities: Vec<(&str, String)> = Vec::new();
    let mut supply_authorities: Vec<(&str, Vec<String>)> = Vec::new();

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

        if !dimension.kind.allows_multiple_authorities() && seen_kinds.contains(&dimension.kind) {
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

        if dimension.kind == RunSetDimensionKind::Supply {
            match super::model::parse_supply_source_authority(&dimension.source) {
                Ok(authority) => supply_authorities.push((&dimension.id, authority)),
                Err(message) => errors.push(RunSetError::about(
                    "RUNSET-SUPPLY-BINDING",
                    &dimension.id,
                    message,
                )),
            }
        }
        if dimension.kind == RunSetDimensionKind::Parameter {
            match super::model::parse_parameter_source_authority(&dimension.source) {
                Ok(authority) => parameter_authorities.push((&dimension.id, authority)),
                Err(message) => errors.push(RunSetError::about(
                    "RUNSET-PARAMETER-BINDING",
                    &dimension.id,
                    message,
                )),
            }
        }
        if dimension.kind == RunSetDimensionKind::Source {
            match super::model::parse_source_value_authority(&dimension.source) {
                Ok(authority) => source_authorities.push((&dimension.id, authority)),
                Err(message) => errors.push(RunSetError::about(
                    "RUNSET-SOURCE-BINDING",
                    &dimension.id,
                    message,
                )),
            }
        }
        if let Some(reason) = dimension.kind.execution_blocker() {
            errors.push(RunSetError::about(
                "RUNSET-BINDING-UNAVAILABLE",
                &dimension.id,
                format!(
                    "The {} dimension is retained for project compatibility but cannot execute: {reason}. Keep it disabled; source authority {:?} was preserved unchanged.",
                    dimension.kind.as_str(),
                    dimension.source
                ),
            ));
        }

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
                RunSetDimensionKind::Parameter | RunSetDimensionKind::Source => {
                    format!("{detail} is not a finite SPICE quantity.")
                }
                RunSetDimensionKind::Frequency | RunSetDimensionKind::Time => {
                    format!("{detail} is not a positive finite quantity.")
                }
                RunSetDimensionKind::Seed => {
                    format!("{detail} is not an exactly representable unsigned seed.")
                }
                RunSetDimensionKind::Sample => {
                    format!("{detail} is not a positive, exactly representable sample count.")
                }
                RunSetDimensionKind::Model
                | RunSetDimensionKind::AnalysisSelection
                | RunSetDimensionKind::DigitalConfiguration
                | RunSetDimensionKind::ExternalDataset => {
                    format!("{detail} is not a non-empty reference.")
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

    for (index, (dimension_id, authority)) in parameter_authorities.iter().enumerate() {
        if parameter_authorities[..index]
            .iter()
            .any(|(_, existing)| existing.eq_ignore_ascii_case(authority))
        {
            errors.push(RunSetError::about(
                "RUNSET-AUTHORITY-COLLISION",
                dimension_id,
                format!(
                    "More than one enabled parameter dimension binds {authority:?}; each point authority must have exactly one owner."
                ),
            ));
        }
    }
    for (index, (dimension_id, authority)) in source_authorities.iter().enumerate() {
        if source_authorities[..index]
            .iter()
            .any(|(_, existing)| existing.eq_ignore_ascii_case(authority))
        {
            errors.push(RunSetError::about(
                "RUNSET-AUTHORITY-COLLISION",
                dimension_id,
                format!(
                    "More than one enabled source dimension binds {authority:?}; each independent source must have exactly one absolute-value owner."
                ),
            ));
        }
        if let Some((supply_id, _)) = supply_authorities.iter().find(|(_, sources)| {
            sources
                .iter()
                .any(|source| source.eq_ignore_ascii_case(authority))
        }) {
            errors.push(RunSetError::about(
                "RUNSET-AUTHORITY-COLLISION",
                dimension_id,
                format!(
                    "Source {authority:?} is bound both by this absolute-value axis and supply dimension {supply_id:?}; one point cannot apply two values to the same source."
                ),
            ));
        }
    }

    let lengths: Vec<usize> = axis_lengths(state);

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

    if state.composition.mode == RunSetCompositionMode::Conditional
        && let Err(message) = super::points::validate_conditional_predicate(state)
    {
        errors.push(RunSetError::global("RUNSET-PREDICATE", message));
    }
    if state.composition.mode == RunSetCompositionMode::Nested
        && !(1..=8).contains(&state.composition.maximum_depth)
    {
        errors.push(RunSetError::global(
            "RUNSET-NESTED-DEPTH",
            "Nested composition depth must be from 1 through 8.",
        ));
    }
    if state.composition.mode == RunSetCompositionMode::Nested
        && state.enabled_dimensions().count() > usize::from(state.composition.maximum_depth)
    {
        let dimension_count = state.enabled_dimensions().count();
        errors.push(RunSetError::global(
            "RUNSET-NESTED-DEPTH",
            format!(
                "Nested composition declares {} enabled dimensions but its maximum depth is {}. Increase the depth or disable dimensions.",
                dimension_count,
                state.composition.maximum_depth
            ),
        ));
    }
    if state.composition.mode == RunSetCompositionMode::Adaptive {
        let policy_valid = state
            .composition
            .adaptive_policy
            .as_ref()
            .is_some_and(|policy| {
                !policy.id.trim().is_empty()
                    && !policy.objective.trim().is_empty()
                    && !policy.bounds.trim().is_empty()
                    && serde_json::from_str::<serde_json::Value>(&policy.bounds).is_ok()
                    && !policy.stop_rule.trim().is_empty()
                    && policy.maximum_proposals > 0
            });
        if !policy_valid {
            errors.push(RunSetError::global(
                "RUNSET-ADAPTIVE-POLICY",
                "Adaptive composition requires a frozen policy identity, objective, seed, JSON bounds, stop rule, and positive maximum proposal count.",
            ));
        }
        // Adaptive scheduling is stateful: authorizing every possible task up
        // front would misrepresent a feedback-driven campaign as a matrix.
        errors.push(RunSetError::global(
            "RUNSET-ADAPTIVE-EXECUTOR",
            state
                .composition
                .mode
                .execution_blocker()
                .unwrap_or("Adaptive composition cannot execute in this engine build."),
        ));
    }

    let composed_count = composed_point_count(state);

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

    let deterministic_point_count = deterministic_point_count(state);
    let point_count_minimum = deterministic_point_count;
    let point_count_maximum = deterministic_point_count.saturating_add(adaptive_proposals(state));
    let point_count = point_count_maximum;
    if deterministic_point_count == 0 {
        errors.push(RunSetError::global(
            "RUNSET-ALL-POINTS-EXCLUDED",
            format!(
                "Every one of the {composed_count} composed points is excluded, so the run set \
                 resolves to nothing executable. Restore at least one point or widen an axis."
            ),
        ));
    }

    let analyses = enabled_analysis_count;
    let task_count = (state.composition.mode != RunSetCompositionMode::Adaptive)
        .then_some(exact_task_count)
        .flatten()
        .or_else(|| point_count.checked_mul(analyses));
    if task_count.is_none() {
        errors.push(RunSetError::global(
            "RUNSET-CARDINALITY-OVERFLOW",
            "Exact task cardinality exceeds safe deterministic representation.",
        ));
    }
    let task_count = task_count.unwrap_or(usize::MAX);

    let engine_task_limit = rspice_core::ResourceLimits::default().max_batch_runs;
    if task_count > engine_task_limit {
        errors.push(RunSetError::global(
            "RUNSET-ENGINE-TASK-LIMIT",
            format!(
                "{task_count} tasks exceed the engine batch limit of {engine_task_limit}. Narrow the run space or disable analyses before previewing."
            ),
        ));
    }

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
            point_count_minimum,
            point_count_maximum,
            exact: state.composition.mode != RunSetCompositionMode::Adaptive,
            enabled_analysis_count: analyses,
            task_count,
            cost_ms: super::modelled_cost_ms(task_count, state.budgets.cost_per_point_ms),
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
/// How many values each enabled axis contributes. A scalar axis counts as one.
fn axis_lengths(state: &RunSetState) -> Vec<usize> {
    state
        .enabled_dimensions()
        .map(|dimension| dimension.values.len().max(1))
        .collect()
}

/// How many points the declared space composes to, before any exclusion.
///
/// A product, not a walk: the size of a matrix follows from the lengths of its
/// axes, and a zipped composition pairs them instead, so its size is the
/// longest. Nothing here expands anything.
#[must_use]
fn composed_point_count(state: &RunSetState) -> usize {
    let lengths = axis_lengths(state);
    if lengths.is_empty() {
        1
    } else if state.composition.mode == RunSetCompositionMode::Zipped {
        lengths.iter().copied().max().unwrap_or(1)
    } else {
        lengths
            .iter()
            .try_fold(1usize, |total, length| total.checked_mul(*length))
            .unwrap_or(usize::MAX)
    }
}

/// The points the space resolves to without any adaptive proposal.
///
/// The composed size less the exclusions that apply — or, where a conditional
/// composition's predicate parses, the resolution itself, because a predicate
/// decides point by point and cannot be subtracted arithmetically.
#[must_use]
fn deterministic_point_count(state: &RunSetState) -> usize {
    if state.composition.mode == RunSetCompositionMode::Conditional
        && super::points::validate_conditional_predicate(state).is_ok()
    {
        return super::points::resolve(state).map_or(0, |points| points.len());
    }
    composed_point_count(state).saturating_sub(classify_exclusions(state).applied)
}

/// How many further points an adaptive policy may propose. None from any other
/// composition.
fn adaptive_proposals(state: &RunSetState) -> usize {
    if state.composition.mode == RunSetCompositionMode::Adaptive {
        state
            .composition
            .adaptive_policy
            .as_ref()
            .map_or(0, |policy| policy.maximum_proposals)
    } else {
        0
    }
}

/// The point count the forecast reports — the number an unnarrowed analysis is
/// priced at — without the findings around it.
///
/// The studio's workload projection needs exactly this and was running a whole
/// validation to read it: a second set of errors and warnings, allocated and
/// thrown away, on every frame that drew a task-rate card. Deriving it here
/// rather than there keeps the number the page prices with and the number the
/// page validates against the same one.
#[must_use]
pub fn forecast_point_count(state: &RunSetState) -> usize {
    deterministic_point_count(state).saturating_add(adaptive_proposals(state))
}

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

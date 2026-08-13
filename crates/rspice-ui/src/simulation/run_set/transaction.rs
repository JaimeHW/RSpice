//! Run-set edits as transactions.
//!
//! Every command either takes effect completely or leaves the state exactly as
//! it was, and either way produces a receipt naming what was attempted, the
//! revisions it moved between, and a digest of that record. A blocked command
//! keeps the invalid input visible: nothing is silently corrected, and no task
//! matrix is created.

use super::model::{
    InvalidValuePolicy, RunSetComposition, RunSetCompositionMode, RunSetDimension,
    RunSetDimensionKind, RunSetState,
};
use super::validate::{RunSetError, RunSetValidation};

/// A command against a run set.
#[derive(Debug, Clone, PartialEq)]
pub enum RunSetAction {
    /// Declare a new dimension of a kind the run set does not yet carry.
    AddDimension(RunSetDimensionKind),
    /// Rename a dimension.
    Rename { id: String, name: String },
    /// Replace a dimension's values from one authored line per value.
    SetValues { id: String, text: String },
    /// Change where a dimension's values are authored.
    SetSource { id: String, source: String },
    /// Change what an unparseable value does.
    SetInvalidValuePolicy {
        id: String,
        policy: InvalidValuePolicy,
    },
    /// Enable or disable a dimension.
    SetEnabled { id: String, enabled: bool },
    /// Remove a dimension.
    RemoveDimension { id: String },
    /// Move a dimension one place earlier or later.
    MoveDimension { id: String, later: bool },
    /// Replace the composition rule.
    SetComposition(RunSetComposition),
    /// Remove one resolved point from the space, named by its point key.
    ExcludePoint { key: String },
    /// Restore one excluded point.
    IncludePoint { key: String },
    /// Replace the execution budgets.
    SetBudgets(super::RunSetBudgets),
    /// Validate and freeze the forecast. Creates no numerical result.
    Preview,
    /// Restore the previous editable state.
    Undo,
    /// Re-apply an undone editable state.
    Redo,
}

impl RunSetAction {
    /// Stable command name recorded in the receipt.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::AddDimension(_) => "add-dimension",
            Self::Rename { .. } => "rename-dimension",
            Self::SetValues { .. } => "set-dimension-values",
            Self::SetSource { .. } => "set-dimension-source",
            Self::SetInvalidValuePolicy { .. } => "set-invalid-value-policy",
            Self::SetEnabled { .. } => "set-dimension-enabled",
            Self::RemoveDimension { .. } => "remove-dimension",
            Self::MoveDimension { .. } => "move-dimension",
            Self::SetComposition(_) => "set-composition",
            Self::ExcludePoint { .. } => "exclude-point",
            Self::IncludePoint { .. } => "include-point",
            Self::SetBudgets(_) => "set-budgets",
            Self::Preview => "preview",
            Self::Undo => "undo",
            Self::Redo => "redo",
        }
    }

    /// What the command acts on.
    fn target(&self) -> String {
        match self {
            Self::AddDimension(kind) => kind.as_str().to_owned(),
            Self::Rename { id, .. }
            | Self::SetValues { id, .. }
            | Self::SetSource { id, .. }
            | Self::SetInvalidValuePolicy { id, .. }
            | Self::SetEnabled { id, .. }
            | Self::RemoveDimension { id }
            | Self::MoveDimension { id, .. } => id.clone(),
            Self::ExcludePoint { key } | Self::IncludePoint { key } => key.clone(),
            Self::SetComposition(composition) => composition.mode.as_str().to_owned(),
            Self::SetBudgets(_) => "budgets".to_owned(),
            Self::Preview => "run-set".to_owned(),
            Self::Undo => "history".to_owned(),
            Self::Redo => "future".to_owned(),
        }
    }

    /// Whether the command changes the declared space, and so must move the
    /// revision and invalidate any frozen preview.
    fn is_mutation(&self) -> bool {
        !matches!(self, Self::Preview | Self::Undo | Self::Redo)
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

/// What a dispatch produced.
#[derive(Debug, Clone)]
pub struct RunSetTransaction {
    pub receipt: RunSetReceipt,
    /// Present for `Preview`, which is the only command that evaluates.
    pub validation: Option<RunSetValidation>,
}

impl RunSetTransaction {
    #[must_use]
    pub fn was_adopted(&self) -> bool {
        self.receipt.status == RunSetReceiptStatus::Completed
    }
}

/// Apply `action` to `state`.
///
/// A mutation is applied to a clone and adopted only if it succeeds, so a
/// refused command cannot leave a half-applied edit behind.
#[cfg(test)]
pub fn dispatch(
    state: &mut RunSetState,
    action: RunSetAction,
    enabled_analysis_count: usize,
) -> RunSetTransaction {
    dispatch_with_validation(state, action, |next| {
        super::validate(next, enabled_analysis_count)
    })
}

/// Apply an action against the exact enabled analysis kinds in the plan.
pub fn dispatch_for_plan(
    state: &mut RunSetState,
    action: RunSetAction,
    enabled_analysis_kinds: &[crate::simulation::plan::AnalysisKind],
    exact_task_count: Option<usize>,
    workload_error: Option<String>,
) -> RunSetTransaction {
    dispatch_with_validation(state, action, |next| {
        let mut validation =
            super::validate_for_plan(next, enabled_analysis_kinds, exact_task_count);
        if let Some(error) = workload_error {
            validation.push_global_error("RUNSET-PLAN-WORKLOAD", error);
        }
        validation
    })
}

fn dispatch_with_validation(
    state: &mut RunSetState,
    action: RunSetAction,
    validate_preview: impl FnOnce(&RunSetState) -> RunSetValidation,
) -> RunSetTransaction {
    let before_revision = state.revision;
    let snapshot = state.snapshot();
    let mut next = state.clone();
    let mut refusal: Option<RunSetError> = None;

    match &action {
        RunSetAction::AddDimension(kind) => {
            if next
                .dimensions
                .iter()
                .any(|dimension| dimension.kind == *kind)
            {
                refusal = Some(missing(
                    "RUNSET-DIMENSION-KIND",
                    format!(
                        "The run set already declares a {} dimension.",
                        kind.as_str()
                    ),
                ));
            } else {
                let id = format!("dimension-{:03}", next.sequence);
                next.sequence += 1;
                next.dimensions.push(RunSetDimension::new(
                    id,
                    *kind,
                    &default_values(*kind),
                    next.revision + 1,
                ));
            }
        }
        RunSetAction::Rename { id, name } => {
            refusal = with_dimension(&mut next, id, |dimension| {
                let trimmed = name.trim();
                if trimmed.is_empty() {
                    return Some(missing(
                        "RUNSET-DIMENSION-ID",
                        "A dimension name cannot be empty.",
                    ));
                }
                dimension.name = trimmed.to_owned();
                None
            });
        }
        RunSetAction::SetValues { id, text } => {
            let revision = next.revision + 1;
            refusal = with_dimension(&mut next, id, |dimension| {
                dimension.set_values_from_lines(text, revision);
                None
            });
        }
        RunSetAction::SetSource { id, source } => {
            refusal = with_dimension(&mut next, id, |dimension| {
                let trimmed = source.trim();
                if trimmed.is_empty() {
                    return Some(missing(
                        "RUNSET-VALUE-SOURCE",
                        "A dimension must name the authority its values come from.",
                    ));
                }
                dimension.source = trimmed.to_owned();
                None
            });
        }
        RunSetAction::SetInvalidValuePolicy { id, policy } => {
            refusal = with_dimension(&mut next, id, |dimension| {
                dimension.invalid_value_policy = *policy;
                None
            });
        }
        RunSetAction::SetEnabled { id, enabled } => {
            refusal = with_dimension(&mut next, id, |dimension| {
                dimension.enabled = *enabled;
                None
            });
        }
        RunSetAction::RemoveDimension { id } => match next.index_of(id) {
            Some(index) => {
                next.dimensions.remove(index);
            }
            None => refusal = Some(dimension_missing(id)),
        },
        RunSetAction::MoveDimension { id, later } => match next.index_of(id) {
            Some(index) => {
                let target = if *later {
                    index + 1
                } else {
                    index.wrapping_sub(1)
                };
                if target >= next.dimensions.len() {
                    refusal = Some(missing(
                        "RUNSET-ORDER",
                        "The dimension is already at the end of the declared order.",
                    ));
                } else {
                    next.dimensions.swap(index, target);
                }
            }
            None => refusal = Some(dimension_missing(id)),
        },
        RunSetAction::SetComposition(composition) => next.composition = composition.clone(),
        RunSetAction::ExcludePoint { key } => {
            if next.composition.mode == RunSetCompositionMode::Zipped {
                // Excluding from a zipped space would have to name a point of
                // the cross product, which is a different space from the one
                // being run. Switching modes silently would change what the
                // remaining points are, not just how many there are.
                refusal = Some(missing(
                    "RUNSET-EXCLUSION-MODE",
                    "Points are excluded from a cross product. Change the composition away from \
                     zipped before excluding one.",
                ));
            } else if !super::points::contains_point_key(&next, key) {
                refusal = Some(exclusion_unknown(
                    key,
                    "does not name a point of the declared space",
                ));
            } else if !next.composition.excluded_points.insert(key.clone()) {
                refusal = Some(exclusion_unknown(key, "is already excluded"));
            } else {
                next.composition.mode = RunSetCompositionMode::Filtered;
            }
        }
        RunSetAction::IncludePoint { key } => {
            if next.composition.excluded_points.remove(key) {
                // A filtered composition that excludes nothing is the cross
                // product under another name, and two names for one space is
                // exactly the disagreement this module refuses elsewhere.
                if next.composition.excluded_points.is_empty() {
                    next.composition.mode = RunSetCompositionMode::Cartesian;
                }
            } else {
                refusal = Some(exclusion_unknown(key, "is not excluded"));
            }
        }
        RunSetAction::SetBudgets(budgets) => {
            if budgets.maximum_tasks == 0 {
                refusal = Some(missing(
                    "RUNSET-TASK-BUDGET",
                    "A task budget of zero would refuse every run set.",
                ));
            } else {
                next.budgets = *budgets;
            }
        }
        RunSetAction::Preview => {}
        RunSetAction::Undo => match next.history.pop() {
            Some(previous) => {
                next.future.push(next.snapshot());
                next.restore(previous);
            }
            None => {
                refusal = Some(missing(
                    "RUNSET-UNDO-EMPTY",
                    "No run-set transaction is available to undo.",
                ));
            }
        },
        RunSetAction::Redo => match next.future.pop() {
            Some(future) => {
                next.history.push(next.snapshot());
                next.restore(future);
            }
            None => {
                refusal = Some(missing(
                    "RUNSET-REDO-EMPTY",
                    "No run-set transaction is available to redo.",
                ));
            }
        },
    }

    let validation = matches!(action, RunSetAction::Preview).then(|| validate_preview(&next));

    let mut error_ids: Vec<&'static str> = refusal.iter().map(|error| error.id).collect();
    if let Some(validation) = &validation {
        error_ids.extend(validation.errors.iter().map(|error| error.id));
    }
    let blocked = !error_ids.is_empty();

    if refusal.is_none() {
        if let Some(validation) = &validation {
            if validation.is_ready() {
                next.preview = Some(validation.forecast);
            } else {
                next.preview = None;
            }
        } else if action.is_mutation() {
            next.history.push(snapshot);
            next.future.clear();
            next.revision += 1;
            next.preview = None;
        }
    }

    let receipt = RunSetReceipt {
        sequence: state.receipts.len() + 1,
        action: action.name(),
        target_id: action.target(),
        before_revision,
        after_revision: if refusal.is_none() {
            next.revision
        } else {
            before_revision
        },
        status: if blocked {
            RunSetReceiptStatus::Blocked
        } else {
            RunSetReceiptStatus::Completed
        },
        error_ids: error_ids.clone(),
        digest: String::new(),
    };
    let receipt = RunSetReceipt {
        digest: digest(&receipt),
        ..receipt
    };

    if refusal.is_none() {
        *state = next;
    }
    state.receipts.push(receipt.clone());

    RunSetTransaction {
        receipt,
        validation,
    }
}

fn with_dimension(
    state: &mut RunSetState,
    id: &str,
    edit: impl FnOnce(&mut RunSetDimension) -> Option<RunSetError>,
) -> Option<RunSetError> {
    match state.index_of(id) {
        Some(index) => edit(&mut state.dimensions[index]),
        None => Some(dimension_missing(id)),
    }
}

fn dimension_missing(id: &str) -> RunSetError {
    RunSetError {
        id: "RUNSET-DIMENSION-MISSING",
        dimension_id: Some(id.to_owned()),
        message: format!("Dimension {id} does not exist."),
    }
}

/// A refusal about one exclusion, naming it by the identity it was given.
fn exclusion_unknown(key: &str, reason: &str) -> RunSetError {
    RunSetError {
        id: "RUNSET-EXCLUSION-UNKNOWN",
        dimension_id: None,
        message: format!("Point {key} {reason}."),
    }
}

fn missing(id: &'static str, message: impl Into<String>) -> RunSetError {
    RunSetError {
        id,
        dimension_id: None,
        message: message.into(),
    }
}

/// The values a newly declared dimension starts with.
///
/// A dimension with no values is refused, so adding one must produce something
/// executable rather than an immediately invalid axis.
fn default_values(kind: RunSetDimensionKind) -> Vec<&'static str> {
    match kind {
        RunSetDimensionKind::ProcessSection => vec!["TT", "SS", "FF"],
        RunSetDimensionKind::Supply => vec!["0.9", "1.0", "1.1"],
        RunSetDimensionKind::Temperature => vec!["-40", "25", "125"],
    }
}

/// FNV-1a/64 over the receipt's own fields.
fn digest(receipt: &RunSetReceipt) -> String {
    let record = format!(
        "{}|{}|{}|{}|{}|{}|{}",
        receipt.sequence,
        receipt.action,
        receipt.target_id,
        receipt.before_revision,
        receipt.after_revision,
        receipt.status.as_str(),
        receipt.error_ids.join(",")
    );
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in record.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("fnv1a64-{hash:016x}")
}

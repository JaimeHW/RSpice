//! Which declared points one analysis instance is executed at.
//!
//! The run set declares the space; a plan's analyses do not all have to cross
//! all of it. A transient that only has to hold at the reference condition
//! costs one solve, not one per corner — and until participation existed, the
//! page forecast, the point table and the dispatched queue all priced every
//! analysis at the full matrix.
//!
//! Participation is a property of the instance, and this module is its one
//! resolver. Everything that has to agree about it — the page forecast, the
//! point table, the specification advisory and the expansion that mints the
//! tasks — resolves through [`participating_point_keys`], so a disagreement
//! between the number an operator budgets against and the queue a dispatch
//! authorizes is not expressible.
//!
//! Points are named by [`RunSetPoint::point_key`] rather than by position. A
//! position moves when an axis gains a value; an identity does not, and a
//! selection has to survive an edit that did not touch the points it names.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::ReferencePoint;
use super::model::RunSetDimensionKind;
use super::points::RunSetPoint;

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
    /// [`nominal_point_key`] rather than by anything stored here, so the
    /// setting survives an edit that moves the reference.
    NominalPoint,
    /// Exactly the named points, by [`RunSetPoint::point_key`].
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

    /// The keys a selection names, empty for every other case.
    #[must_use]
    pub fn selected_keys(&self) -> &[String] {
        match self {
            Self::AllPoints | Self::NominalPoint => &[],
            Self::SelectedPoints(keys) => keys,
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

/// The point a [`AnalysisRunAt::NominalPoint`] instance runs at.
///
/// Two axes carry a reference the plan already owns — the process section and
/// the temperature — and the point is the first resolved one that sits on both.
/// Every other axis kind states no reference at all: a supply, parameter or
/// source axis is a list of values its author ordered, and the first of them is
/// the only baseline the declaration offers. So those axes are answered by
/// their own first declared value, which is what "the centre of the run set"
/// means for a space that declares no centre.
///
/// `None` when the declared space contains no such point — a temperature axis
/// of `-40, 85` against a reference of 27, say. That is a refusal rather than a
/// fallback: running such an instance at whichever point came first would
/// attribute a nominal claim to a corner.
#[must_use]
pub fn nominal_point_key(points: &[RunSetPoint<'_>], reference: ReferencePoint) -> Option<String> {
    points
        .iter()
        .find(|point| point_is_reference(point, reference))
        .map(RunSetPoint::point_key)
}

/// Whether every coordinate of `point` sits on its axis's reference value.
fn point_is_reference(point: &RunSetPoint<'_>, reference: ReferencePoint) -> bool {
    point
        .coordinates
        .iter()
        .all(|(dimension, value)| match dimension.kind {
            RunSetDimensionKind::ProcessSection => value
                .lexical
                .trim()
                .eq_ignore_ascii_case(reference.process.short_name()),
            RunSetDimensionKind::Temperature => value
                .canonical
                .is_some_and(|canonical| canonical == reference.temperature_celsius),
            // No other axis kind declares a reference, so its first authored
            // value is the baseline. The coordinate borrows the axis it came
            // from, so that first value is here rather than looked up.
            _ => dimension
                .values
                .first()
                .is_some_and(|first| first.id == value.id),
        })
}

/// Why a participation cannot be resolved against the declared space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParticipationRefusal {
    /// The point identities the selection names that the space no longer has.
    /// Empty when the refusal is not an orphaning.
    pub orphaned_keys: Vec<String>,
    pub message: String,
}

/// The points of `points` that an instance with this participation visits.
///
/// The order is the declared order, so a caller that indexes back into `points`
/// gets the positions the point table shows. Both the forecast and the
/// expansion call this and nothing else, which is what makes the number the
/// operator budgets against the queue the dispatch authorizes.
pub fn participating_point_keys(
    run_at: &AnalysisRunAt,
    points: &[RunSetPoint<'_>],
    reference: ReferencePoint,
) -> Result<Vec<String>, ParticipationRefusal> {
    match run_at {
        AnalysisRunAt::AllPoints => Ok(points.iter().map(RunSetPoint::point_key).collect()),
        AnalysisRunAt::NominalPoint => nominal_point_key(points, reference)
            .map(|key| vec![key])
            .ok_or_else(|| ParticipationRefusal {
                orphaned_keys: Vec::new(),
                message: format!(
                    "This analysis runs at the nominal point only, but the declared space contains \
                     no point on the run's reference condition ({} · {} °C). Widen its \
                     participation, or move an axis onto the reference.",
                    reference.process.short_name(),
                    reference.temperature_celsius,
                ),
            }),
        AnalysisRunAt::SelectedPoints(selected) => {
            let declared: HashSet<String> = points.iter().map(RunSetPoint::point_key).collect();
            let orphaned_keys: Vec<String> = selected
                .iter()
                .filter(|key| !declared.contains(*key))
                .cloned()
                .collect();
            if !orphaned_keys.is_empty() {
                return Err(ParticipationRefusal {
                    message: format!(
                        "This analysis is scoped to {} point{} the declared space no longer \
                         contains: {}. Re-open its point selection, or restore the axis values \
                         those points were composed from.",
                        orphaned_keys.len(),
                        if orphaned_keys.len() == 1 { "" } else { "s" },
                        orphaned_keys.join(", "),
                    ),
                    orphaned_keys,
                });
            }
            let chosen: HashSet<&String> = selected.iter().collect();
            let resolved: Vec<String> = points
                .iter()
                .map(RunSetPoint::point_key)
                .filter(|key| chosen.contains(key))
                .collect();
            if resolved.is_empty() {
                return Err(ParticipationRefusal {
                    orphaned_keys: Vec::new(),
                    message: "This analysis is scoped to a point selection that resolves to \
                              nothing in the declared space."
                        .to_owned(),
                });
            }
            Ok(resolved)
        }
    }
}

/// How many points this participation resolves to, for a caller that needs the
/// count and has no place to report a refusal.
///
/// A participation that cannot resolve counts as the whole space, because that
/// is what the run costs once the refusal is repaired by widening — and pricing
/// a refused plan at zero is how a budget silently shrinks.
#[must_use]
pub fn participating_point_count(
    run_at: &AnalysisRunAt,
    points: &[RunSetPoint<'_>],
    reference: ReferencePoint,
) -> usize {
    participating_point_keys(run_at, points, reference).map_or(points.len(), |keys| keys.len())
}

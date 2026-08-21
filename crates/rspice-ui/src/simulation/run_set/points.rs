//! Deterministic materialization of the declared points.
//!
//! The resolved point list is the same expansion the executor performs, derived
//! from the same declaration, so the table a user checks the composition
//! against and the matrix the engine runs cannot disagree. It is produced only
//! from a run set that validates: an unresolvable declaration yields no points
//! rather than a partial list that would read as a shorter run.

use super::model::{RunSetCompositionMode, RunSetDimension, RunSetState, RunSetValue};

/// One executable point: the value each enabled dimension contributes.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSetPoint<'a> {
    /// One entry per enabled dimension, in declaration order.
    pub coordinates: Vec<(&'a RunSetDimension, &'a RunSetValue)>,
}

/// Separator between the value identities a point key is built from. Value
/// identities never contain it, so a key splits back into exactly the parts it
/// was joined from.
const POINT_KEY_SEPARATOR: &str = "+";

impl RunSetPoint<'_> {
    /// The point's manifest name: each coordinate's authored value, joined.
    ///
    /// The lexical form is used rather than the canonical number because that
    /// is what the user declared and what the point table shows; a name built
    /// from the parsed value would drift from the row it labels.
    #[must_use]
    pub fn label(&self) -> String {
        self.coordinates
            .iter()
            .map(|(dimension, value)| match dimension.unit() {
                Some(unit) if !value.lexical.ends_with(unit) => {
                    format!("{} {unit}", value.lexical)
                }
                _ => value.lexical.clone(),
            })
            .collect::<Vec<_>>()
            .join(" · ")
    }

    /// The point's stable identity: its coordinates' value identities, sorted
    /// and joined.
    ///
    /// Identities rather than authored text, because a value that is retyped
    /// keeps its identity and an exclusion has to stay attached to the point it
    /// was placed on. Sorted rather than in declaration order, because moving a
    /// dimension reorders every point's coordinates without changing which
    /// combinations exist, and a key that changed there would silently release
    /// every exclusion.
    #[must_use]
    pub fn point_key(&self) -> String {
        let mut ids: Vec<&str> = self
            .coordinates
            .iter()
            .map(|(_, value)| value.id.as_str())
            .collect();
        ids.sort_unstable();
        ids.join(POINT_KEY_SEPARATOR)
    }
}

/// Whether the declared space still contains the point `key` names.
///
/// Decided from the key itself rather than by expanding the space: a key is one
/// value identity per enabled dimension, and value identities are unique across
/// the run set, so membership is a lookup per coordinate. A space of a million
/// points is therefore checked in the size of its declaration.
#[must_use]
pub(super) fn contains_point_key(state: &RunSetState, key: &str) -> bool {
    let dimensions: Vec<&RunSetDimension> = expandable_dimensions(state);
    let ids: Vec<&str> = if key.is_empty() {
        Vec::new()
    } else {
        key.split(POINT_KEY_SEPARATOR).collect()
    };
    if ids.len() != dimensions.len() {
        return false;
    }
    dimensions.into_iter().all(|dimension| {
        ids.iter()
            .filter(|id| dimension.values.iter().any(|value| value.id == **id))
            .count()
            == 1
    })
}

/// The dimensions an expansion draws coordinates from.
fn expandable_dimensions(state: &RunSetState) -> Vec<&RunSetDimension> {
    state
        .enabled_dimensions()
        .filter(|dimension| !dimension.values.is_empty())
        .collect()
}

/// Expand the enabled dimensions into the points the run executes.
///
/// Returns `None` when the declaration cannot be expanded exactly: a zipped
/// composition with unequal non-scalar lengths, or a value that does not parse.
/// Both are refusals `validate` already reports, and both must produce no list
/// rather than a guess.
///
/// A filtered composition subtracts its exclusions here, so every caller — the
/// point table, the forecast and the corner configuration — sees the same
/// shorter list. Nothing else in the module removes points.
#[must_use]
pub fn resolve(state: &RunSetState) -> Option<Vec<RunSetPoint<'_>>> {
    let mut points = compose(state)?;
    if state.composition.filters() {
        points.retain(|point| {
            !state
                .composition
                .excluded_points
                .contains(&point.point_key())
        });
    }
    if state.composition.mode == RunSetCompositionMode::Conditional {
        points.retain(|point| conditional_predicate_matches(state, point).unwrap_or(false));
    }
    // An adaptive declaration requires feedback from completed proposals and
    // therefore cannot be flattened into an authorized queue up front.
    if state.composition.mode == RunSetCompositionMode::Adaptive {
        return None;
    }
    Some(points)
}

/// Evaluate the deliberately small, deterministic conditional grammar.
/// Clauses are `<dimension-id-or-name> == <authored-value>` (or `!=`) joined by
/// `&&`. Values may be quoted. There is no implicit numeric coercion: the
/// predicate addresses exactly the declaration displayed in the point table.
fn conditional_predicate_matches(state: &RunSetState, point: &RunSetPoint<'_>) -> Result<bool, ()> {
    let predicate = state.composition.predicate.trim();
    if predicate.is_empty() {
        return Err(());
    }
    for clause in predicate.split("&&") {
        let clause = clause.trim();
        let (left, right, equal) = if let Some((left, right)) = clause.split_once("==") {
            (left, right, true)
        } else if let Some((left, right)) = clause.split_once("!=") {
            (left, right, false)
        } else {
            return Err(());
        };
        let dimension_key = left.trim();
        let expected = right
            .trim()
            .trim_matches(|character| character == '\'' || character == '"');
        let Some((_, actual)) = point.coordinates.iter().find(|(dimension, _)| {
            dimension.id == dimension_key || dimension.name == dimension_key
        }) else {
            return Err(());
        };
        let matches = actual.lexical.trim() == expected;
        if matches != equal {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) fn validate_conditional_predicate(state: &RunSetState) -> Result<(), String> {
    let predicate = state.composition.predicate.trim();
    if predicate.is_empty() {
        return Err("Conditional composition requires a predicate.".to_owned());
    }
    if state.composition.upstream_dimension_ids.is_empty() {
        return Err(
            "Conditional composition requires at least one upstream dimension identity.".to_owned(),
        );
    }
    for id in &state.composition.upstream_dimension_ids {
        if !state
            .enabled_dimensions()
            .any(|dimension| dimension.id == *id)
        {
            return Err(format!(
                "Conditional upstream dimension {id:?} is not enabled in this run set."
            ));
        }
    }
    for clause in predicate.split("&&") {
        let clause = clause.trim();
        let (left, right) = clause
            .split_once("==")
            .or_else(|| clause.split_once("!="))
            .ok_or_else(|| {
                format!(
                    "Conditional clause {clause:?} must use == or !=; clauses may be joined by &&."
                )
            })?;
        let dimension_key = left.trim();
        let dimension = state
            .enabled_dimensions()
            .find(|dimension| dimension.id == dimension_key || dimension.name == dimension_key)
            .ok_or_else(|| {
                format!("Conditional clause references unknown dimension {dimension_key:?}.")
            })?;
        if !state
            .composition
            .upstream_dimension_ids
            .contains(&dimension.id)
        {
            return Err(format!(
                "Conditional clause reads {}, which is not listed as an upstream dimension.",
                dimension.id
            ));
        }
        let expected = right
            .trim()
            .trim_matches(|character| character == '\'' || character == '"');
        if expected.is_empty() {
            return Err(format!(
                "Conditional clause for {} requires a value.",
                dimension.id
            ));
        }
    }
    Ok(())
}

/// The composed space before any exclusion is applied.
///
/// The point table draws from this rather than from [`resolve`], because a row
/// a user can put back has to still be a row. Nothing that decides what runs
/// may read it.
#[must_use]
pub fn compose(state: &RunSetState) -> Option<Vec<RunSetPoint<'_>>> {
    let dimensions: Vec<&RunSetDimension> = expandable_dimensions(state);
    if dimensions.iter().any(|dimension| {
        dimension
            .values
            .iter()
            .any(|value| value.canonical.is_none())
    }) {
        return None;
    }
    if dimensions.is_empty() {
        return Some(vec![RunSetPoint {
            coordinates: Vec::new(),
        }]);
    }

    match state.composition.mode {
        // A filtered space is the cross product; `resolve` is what subtracts
        // from it, so the expansion itself is the Cartesian one.
        //
        // Nested shares this arm on purpose, and the contract says so rather
        // than promising a traversal that does not exist. Its leaves *are* the
        // cross product's, and what it declares is the order they are visited
        // in — which this loop already produces: each dimension is folded in
        // after the ones declared before it, so the first enabled dimension
        // varies slowest (outermost) and the last varies fastest (innermost).
        // Reordering the dimensions therefore reorders a nested expansion,
        // which is the whole of what "nesting declaration" means here.
        RunSetCompositionMode::Cartesian
        | RunSetCompositionMode::Filtered
        | RunSetCompositionMode::Conditional
        | RunSetCompositionMode::Adaptive
        | RunSetCompositionMode::Nested => {
            let mut points = vec![RunSetPoint {
                coordinates: Vec::new(),
            }];
            for dimension in dimensions {
                let mut next = Vec::with_capacity(points.len() * dimension.values.len());
                for point in &points {
                    for value in &dimension.values {
                        let mut coordinates = point.coordinates.clone();
                        coordinates.push((dimension, value));
                        next.push(RunSetPoint { coordinates });
                    }
                }
                points = next;
            }
            Some(points)
        }
        RunSetCompositionMode::Zipped => {
            let mut lengths: Vec<usize> = dimensions
                .iter()
                .map(|dimension| dimension.values.len())
                .filter(|length| *length != 1)
                .collect();
            lengths.sort_unstable();
            lengths.dedup();
            if lengths.len() > 1 {
                return None;
            }
            let length = lengths.first().copied().unwrap_or(1);
            Some(
                (0..length)
                    .map(|index| RunSetPoint {
                        coordinates: dimensions
                            .iter()
                            .map(|dimension| {
                                let value = if dimension.values.len() == 1 {
                                    &dimension.values[0]
                                } else {
                                    &dimension.values[index]
                                };
                                (*dimension, value)
                            })
                            .collect(),
                    })
                    .collect(),
            )
        }
    }
}

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
}

/// Expand the enabled dimensions into points, in execution order.
///
/// Returns `None` when the declaration cannot be expanded exactly: a zipped
/// composition with unequal non-scalar lengths, or a value that does not parse.
/// Both are refusals `validate` already reports, and both must produce no list
/// rather than a guess.
#[must_use]
pub fn resolve(state: &RunSetState) -> Option<Vec<RunSetPoint<'_>>> {
    let dimensions: Vec<&RunSetDimension> = state
        .enabled_dimensions()
        .filter(|dimension| !dimension.values.is_empty())
        .collect();
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
        RunSetCompositionMode::Cartesian => {
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

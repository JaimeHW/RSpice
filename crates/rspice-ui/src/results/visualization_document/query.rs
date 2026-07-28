//! Dataset invariants, and answering a query only from data that exists.
//!
//! Every key, label, unit, and resource count is checked here before a
//! document is accepted, so downstream code can index a dataset without
//! re-validating it.  The exact-row query is deliberately exact: when the
//! requested coordinates are not present it reports that interpolation would
//! be required instead of inventing a point.

use super::*;

pub(super) fn validate_key(field: &'static str, value: &str) -> Result<(), VisualizationError> {
    if value.is_empty()
        || value.len() > MAX_VISUALIZATION_KEY_BYTES
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "._:/[]()-".contains(character))
    {
        return Err(VisualizationError::InvalidValue {
            field,
            message: format!(
                "must be a non-empty stable ASCII key of at most {MAX_VISUALIZATION_KEY_BYTES} bytes"
            ),
        });
    }
    Ok(())
}

pub(super) fn validate_label(field: &'static str, value: &str) -> Result<(), VisualizationError> {
    if value.trim().is_empty()
        || value.len() > MAX_VISUALIZATION_LABEL_BYTES
        || value.chars().any(char::is_control)
    {
        return Err(VisualizationError::InvalidValue {
            field,
            message: format!(
                "must be non-blank, contain no control characters, and not exceed {MAX_VISUALIZATION_LABEL_BYTES} bytes"
            ),
        });
    }
    Ok(())
}

pub(super) fn validate_optional_unit(
    field: &'static str,
    unit: Option<&str>,
) -> Result<(), VisualizationError> {
    if unit.is_some_and(|unit| {
        unit.trim().is_empty()
            || unit.len() > MAX_VISUALIZATION_UNIT_BYTES
            || unit.chars().any(char::is_control)
    }) {
        return Err(VisualizationError::InvalidValue {
            field,
            message: format!(
                "unit must be absent or non-blank, control-free, and at most {MAX_VISUALIZATION_UNIT_BYTES} bytes"
            ),
        });
    }
    Ok(())
}

pub(super) fn ensure_maximum_len(
    field: &'static str,
    actual: usize,
    maximum: usize,
) -> Result<(), VisualizationError> {
    if actual > maximum {
        return Err(VisualizationError::InvalidValue {
            field,
            message: format!("contains {actual} entries; the resource limit is {maximum}"),
        });
    }
    Ok(())
}

pub(super) fn checked_bounded_sum(
    field: &'static str,
    current: usize,
    additional: usize,
    maximum: usize,
) -> Result<usize, VisualizationError> {
    let total =
        current
            .checked_add(additional)
            .ok_or_else(|| VisualizationError::InvalidValue {
                field,
                message: "resource accounting overflowed the supported address space".to_owned(),
            })?;
    ensure_maximum_len(field, total, maximum)?;
    Ok(total)
}

pub(super) fn validate_aggregate_nested_resources<T: NestedResourceCount>(
    field: &'static str,
    values: &[T],
    maximum: usize,
) -> Result<(), VisualizationError> {
    values.iter().try_fold(0_usize, |total, value| {
        checked_bounded_sum(field, total, value.nested_resource_count(), maximum)
    })?;
    Ok(())
}

pub(super) fn validate_dataset_set(datasets: &[SourceDataset]) -> Result<(), VisualizationError> {
    if datasets.is_empty() || datasets.len() > MAX_VISUALIZATION_DATASETS {
        return Err(VisualizationError::InvalidValue {
            field: "visualization-document.datasets",
            message: format!(
                "a visualization document requires 1 to {MAX_VISUALIZATION_DATASETS} datasets"
            ),
        });
    }
    let mut bindings = HashMap::new();
    let mut aggregate_cells = 0_usize;
    let mut aggregate_text_bytes = 0_usize;
    for dataset in datasets {
        dataset.validate()?;
        let cells = dataset
            .rows
            .len()
            .checked_mul(dataset.columns.len())
            .ok_or_else(|| VisualizationError::InvalidValue {
                field: "visualization-document.source-cells",
                message: "aggregate source cell count overflowed the supported address space"
                    .to_owned(),
            })?;
        aggregate_cells =
            aggregate_cells
                .checked_add(cells)
                .ok_or_else(|| VisualizationError::InvalidValue {
                    field: "visualization-document.source-cells",
                    message: "aggregate source cell count overflowed the supported address space"
                        .to_owned(),
                })?;
        ensure_maximum_len(
            "visualization-document.source-cells",
            aggregate_cells,
            MAX_SOURCE_CELLS_TOTAL,
        )?;
        aggregate_text_bytes = checked_bounded_sum(
            "visualization-document.retained-source-text-bytes",
            aggregate_text_bytes,
            dataset.retained_text_bytes()?,
            MAX_SOURCE_TEXT_BYTES_TOTAL,
        )?;
        if let Some(bound) =
            bindings.insert(dataset.binding.dataset_id, dataset.binding.content_digest)
        {
            return Err(if bound == dataset.binding.content_digest {
                VisualizationError::InvalidValue {
                    field: "visualization-document.datasets",
                    message: format!("duplicate dataset {}", dataset.binding.dataset_id),
                }
            } else {
                VisualizationError::SourceDigestMismatch {
                    dataset_id: dataset.binding.dataset_id,
                    bound,
                    requested: dataset.binding.content_digest,
                }
            });
        }
    }
    Ok(())
}

pub(super) fn ensure_identity(
    identities: &mut HashSet<EntityRef>,
    entity: EntityRef,
) -> Result<(), VisualizationError> {
    identities
        .insert(entity)
        .then_some(())
        .ok_or(VisualizationError::InvalidValue {
            field: "visualization-document.identities",
            message: format!("duplicate entity identity {entity:?}"),
        })
}

pub(super) fn find_column<'a>(
    dataset: &'a SourceDataset,
    key: &str,
) -> Result<&'a SourceColumn, VisualizationError> {
    dataset
        .columns
        .iter()
        .find(|column| column.key == key)
        .ok_or_else(|| VisualizationError::ColumnNotFound(key.to_owned()))
}

pub(super) fn column_index(
    dataset: &SourceDataset,
    key: &str,
) -> Result<usize, VisualizationError> {
    dataset
        .columns
        .iter()
        .position(|column| column.key == key)
        .ok_or_else(|| VisualizationError::ColumnNotFound(key.to_owned()))
}

pub(super) fn validate_annotation(
    document: &VisualizationDocument,
    pane_id: PaneId,
    anchor: &AnnotationAnchor,
    text: &str,
) -> Result<(), VisualizationError> {
    if text.trim().is_empty()
        || text.len() > MAX_ANNOTATION_TEXT_BYTES
        || text.chars().any(char::is_control)
    {
        return Err(VisualizationError::InvalidValue {
            field: "annotation.text",
            message: format!(
                "must be non-blank, control-free, and at most {MAX_ANNOTATION_TEXT_BYTES} bytes"
            ),
        });
    }
    document.require_pane(pane_id)?;
    match anchor {
        AnnotationAnchor::Pane {
            x_fraction,
            y_fraction,
        } if !x_fraction.is_finite()
            || !y_fraction.is_finite()
            || !(0.0..=1.0).contains(x_fraction)
            || !(0.0..=1.0).contains(y_fraction) =>
        {
            Err(VisualizationError::InvalidValue {
                field: "annotation.anchor",
                message: "pane fractions must be finite and within zero through one".to_owned(),
            })
        }
        AnnotationAnchor::Trace {
            trace_id,
            coordinate,
        } => {
            coordinate.validate("annotation.coordinate")?;
            document.require_trace_in_pane(*trace_id, pane_id)
        }
        _ => Ok(()),
    }
}

pub(super) fn validate_link_members(
    document: &VisualizationDocument,
    kind: LinkKind,
    members: &[EntityRef],
) -> Result<(), VisualizationError> {
    if members.len() < 2
        || members.len() > MAX_ENTITY_REFERENCES
        || members.iter().collect::<HashSet<_>>().len() != members.len()
    {
        return Err(VisualizationError::InvalidLinkMembers(kind));
    }
    for member in members {
        document.require_entity(*member)?;
        let compatible = matches!(
            (kind, member),
            (
                LinkKind::HorizontalViewport | LinkKind::VerticalViewport,
                EntityRef::Axis(_)
            ) | (LinkKind::CursorPosition, EntityRef::Cursor(_))
        );
        if !compatible {
            return Err(VisualizationError::InvalidLinkMembers(kind));
        }
    }
    Ok(())
}

pub(super) fn query_dataset_exact(
    dataset: &SourceDataset,
    query: &ExactRowQuery,
) -> Result<ExactRow, VisualizationError> {
    let coordinate_columns: Vec<_> = dataset
        .columns
        .iter()
        .enumerate()
        .filter(|(_, column)| column.role == ColumnRole::Coordinate)
        .collect();
    if query.coordinates.len() != coordinate_columns.len() {
        return Err(VisualizationError::IncompleteCoordinateQuery);
    }
    let mut predicates = Vec::with_capacity(query.coordinates.len());
    let mut seen = HashSet::new();
    for coordinate in &query.coordinates {
        coordinate.value.validate("query.coordinate")?;
        let index = column_index(dataset, &coordinate.column)?;
        let column = &dataset.columns[index];
        if column.role != ColumnRole::Coordinate || !seen.insert(index) {
            return Err(VisualizationError::IncompleteCoordinateQuery);
        }
        if coordinate.value.value_type() != column.value_type {
            return Err(VisualizationError::ColumnTypeMismatch {
                column: column.key.clone(),
                expected: column.value_type,
                actual: coordinate.value.value_type(),
            });
        }
        predicates.push((index, coordinate));
    }
    let projection_indices: Vec<_> = query
        .projections
        .iter()
        .map(|key| {
            let index = column_index(dataset, key)?;
            if dataset.columns[index].role != ColumnRole::Signal {
                return Err(VisualizationError::ColumnNotFound(key.clone()));
            }
            Ok(index)
        })
        .collect::<Result<_, _>>()?;
    if query.projections.is_empty() {
        return Err(VisualizationError::InvalidValue {
            field: "query.projections",
            message: "at least one signal projection is required".to_owned(),
        });
    }
    if let Some((row_index, row)) = dataset.rows.iter().enumerate().find(|(_, row)| {
        predicates
            .iter()
            .all(|(index, predicate)| row.values[*index].exact_eq(&predicate.value))
    }) {
        return Ok(ExactRow {
            binding: dataset.binding,
            row_index,
            coordinates: predicates
                .iter()
                .map(|(index, _)| {
                    (
                        dataset.columns[*index].key.clone(),
                        row.values[*index].clone(),
                    )
                })
                .collect(),
            values: projection_indices
                .iter()
                .map(|index| {
                    (
                        dataset.columns[*index].key.clone(),
                        row.values[*index].clone(),
                    )
                })
                .collect(),
        });
    }
    if interpolation_is_possible(dataset, &predicates) {
        Err(VisualizationError::InterpolationRequired)
    } else {
        Err(VisualizationError::RowNotFound)
    }
}

pub(super) fn interpolation_is_possible(
    dataset: &SourceDataset,
    predicates: &[(usize, &QueryCoordinate)],
) -> bool {
    let real_predicates: Vec<_> = predicates
        .iter()
        .filter(|(_, predicate)| matches!(predicate.value, TypedValue::Real(_)))
        .collect();
    if real_predicates.len() != 1 {
        return false;
    }
    let (interpolation_index, interpolation_predicate) = real_predicates[0];
    let TypedValue::Real(target) = interpolation_predicate.value else {
        return false;
    };
    let mut lower = false;
    let mut upper = false;
    for row in &dataset.rows {
        if predicates.iter().all(|(index, predicate)| {
            *index == *interpolation_index || row.values[*index].exact_eq(&predicate.value)
        }) && let TypedValue::Real(value) = row.values[*interpolation_index]
        {
            lower |= value < target;
            upper |= value > target;
        }
    }
    lower && upper
}

pub(super) fn coordinate_indices(dataset: &SourceDataset) -> Vec<usize> {
    dataset
        .columns
        .iter()
        .enumerate()
        .filter_map(|(index, column)| (column.role == ColumnRole::Coordinate).then_some(index))
        .collect()
}

pub(super) fn rows_have_exact_coordinates(
    left: &SourceRow,
    left_indices: &[usize],
    right: &SourceRow,
    right_indices: &[usize],
) -> bool {
    left_indices
        .iter()
        .zip(right_indices)
        .all(|(left_index, right_index)| {
            left.values[*left_index].exact_eq(&right.values[*right_index])
        })
}

pub(crate) fn compare_source_datasets(
    baseline: &SourceDataset,
    candidate: &SourceDataset,
    request: &ComparisonRequest,
) -> Result<ComparisonReceipt, VisualizationError> {
    if request.baseline != baseline.binding || request.candidate != candidate.binding {
        return Err(VisualizationError::InvalidValue {
            field: "comparison.binding",
            message: "request bindings must exactly match the immutable source datasets".to_owned(),
        });
    }
    if request.signal_keys.is_empty() {
        return Err(VisualizationError::EmptyComparison);
    }
    if request.signal_keys.len() > MAX_COMPARISON_SIGNALS {
        return Err(VisualizationError::InvalidValue {
            field: "comparison.signal-keys",
            message: format!("a comparison supports at most {MAX_COMPARISON_SIGNALS} signal keys"),
        });
    }
    let mut unique_signals = HashSet::with_capacity(request.signal_keys.len());
    for signal_key in &request.signal_keys {
        validate_key("comparison.signal-key", signal_key)?;
        if !unique_signals.insert(signal_key.as_str()) {
            return Err(VisualizationError::DuplicateKey(signal_key.clone()));
        }
    }
    request.policy.tolerance.validate()?;
    let baseline_coordinates = coordinate_indices(baseline);
    let candidate_coordinates = coordinate_indices(candidate);
    if baseline_coordinates.len() != candidate_coordinates.len()
        || baseline_coordinates
            .iter()
            .zip(&candidate_coordinates)
            .any(|(left, right)| {
                baseline.columns[*left].key != candidate.columns[*right].key
                    || baseline.columns[*left].value_type != candidate.columns[*right].value_type
            })
    {
        return Err(VisualizationError::ComparisonRowsDiffer);
    }
    let row_pairs: Vec<_> = match request.policy.row_alignment {
        RowAlignmentPolicy::RequireIdentical => {
            if baseline.rows.len() != candidate.rows.len()
                || baseline
                    .rows
                    .iter()
                    .zip(&candidate.rows)
                    .any(|(left, right)| {
                        !rows_have_exact_coordinates(
                            left,
                            &baseline_coordinates,
                            right,
                            &candidate_coordinates,
                        )
                    })
            {
                return Err(VisualizationError::ComparisonRowsDiffer);
            }
            baseline.rows.iter().zip(&candidate.rows).collect()
        }
        RowAlignmentPolicy::ExactIntersection => baseline
            .rows
            .iter()
            .filter_map(|left| {
                candidate
                    .rows
                    .iter()
                    .find(|right| {
                        rows_have_exact_coordinates(
                            left,
                            &baseline_coordinates,
                            right,
                            &candidate_coordinates,
                        )
                    })
                    .map(|right| (left, right))
            })
            .collect(),
    };
    if row_pairs.is_empty() {
        return Err(VisualizationError::NoComparableRows);
    }
    let mut signal_receipts = Vec::with_capacity(request.signal_keys.len());
    for signal_key in &request.signal_keys {
        let baseline_index = column_index(baseline, signal_key)?;
        let candidate_index = column_index(candidate, signal_key)?;
        let baseline_column = &baseline.columns[baseline_index];
        let candidate_column = &candidate.columns[candidate_index];
        if baseline_column.role != ColumnRole::Signal
            || candidate_column.role != ColumnRole::Signal
            || baseline_column.value_type != ValueType::Real
            || candidate_column.value_type != ValueType::Real
        {
            return Err(VisualizationError::NonNumericComparison(signal_key.clone()));
        }
        if request.policy.require_identical_units && baseline_column.unit != candidate_column.unit {
            return Err(VisualizationError::UnitMismatch {
                signal: signal_key.clone(),
                baseline: baseline_column.unit.clone(),
                candidate: candidate_column.unit.clone(),
            });
        }
        let mut failed_rows = 0;
        let mut maximum_absolute_error = 0.0_f64;
        let mut maximum_relative_error = 0.0_f64;
        for (baseline_row, candidate_row) in &row_pairs {
            let TypedValue::Real(baseline_value) = baseline_row.values[baseline_index] else {
                unreachable!("dataset validation guarantees a real value")
            };
            let TypedValue::Real(candidate_value) = candidate_row.values[candidate_index] else {
                unreachable!("dataset validation guarantees a real value")
            };
            let absolute_error = (candidate_value - baseline_value).abs();
            let scale = baseline_value.abs().max(candidate_value.abs());
            let relative_error = if scale == 0.0 {
                0.0
            } else {
                absolute_error / scale
            };
            maximum_absolute_error = maximum_absolute_error.max(absolute_error);
            maximum_relative_error = maximum_relative_error.max(relative_error);
            let allowed = request.policy.tolerance.absolute
                + request.policy.tolerance.relative * baseline_value.abs();
            failed_rows += usize::from(absolute_error > allowed);
        }
        signal_receipts.push(SignalComparison {
            signal_key: signal_key.clone(),
            compared_rows: row_pairs.len(),
            failed_rows,
            maximum_absolute_error,
            maximum_relative_error,
        });
    }
    let disposition = if signal_receipts.iter().any(|signal| signal.failed_rows > 0) {
        ComparisonDisposition::Failed
    } else {
        ComparisonDisposition::Passed
    };
    let receipt = ComparisonReceipt {
        baseline: baseline.binding,
        candidate: candidate.binding,
        policy: request.policy,
        rows_compared: row_pairs.len(),
        signals: signal_receipts,
        disposition,
    };
    receipt.validate_structure()?;
    Ok(receipt)
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryCoordinate {
    pub column: String,
    pub value: TypedValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExactRowQuery {
    pub binding: DatasetBinding,
    pub coordinates: Vec<QueryCoordinate>,
    pub projections: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExactRow {
    pub binding: DatasetBinding,
    pub row_index: usize,
    pub coordinates: Vec<(String, TypedValue)>,
    pub values: Vec<(String, TypedValue)>,
}

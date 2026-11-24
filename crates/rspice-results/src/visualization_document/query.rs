//! Dataset invariants, and answering a query only from data that exists.
//!
//! Every key, label, unit, and resource count is checked here before a
//! document is accepted, so downstream code can index a dataset without
//! re-validating it.  The exact-row query is deliberately exact: when the
//! requested coordinates are not present it reports that interpolation would
//! be required instead of inventing a point.

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::sync::{Mutex, PoisonError};

use super::lookup::Entities;
use super::*;

/// The rows of a document's embedded tables by the exact values of the
/// columns a check names. Each table is read once for each set of columns a
/// check asks of it, the first time one asks, so a marker or a trace on a
/// table of a hundred thousand rows is checked by one look-up rather than a
/// scan of the rows, and a transaction that places many costs in proportion
/// to what it places. It is dropped when the document's own check ends: it
/// is never part of what a document holds, saves or compares.
#[derive(Default)]
pub(super) struct RowIndex(Mutex<RowTables>);

/// A table's rows by the hash of the values in some of its columns. A table
/// is keyed by its binding and its row count: the rows of one immutable
/// binding are never rewritten, only joined by another analysis' rows.
#[derive(Default)]
struct RowTables {
    hashing: RandomState,
    tables: HashMap<TableRead, RowsByHash>,
}

/// One table read for some of its columns: its binding, its row count and
/// the columns read.
type TableRead = (DatasetBinding, usize, Vec<usize>);

/// The indices of a table's rows by the hash of the values read.
type RowsByHash = HashMap<u64, Vec<usize>>;

impl RowTables {
    fn hash<'a>(&self, values: impl Iterator<Item = &'a TypedValue>) -> u64 {
        let mut hasher = self.hashing.build_hasher();
        for value in values {
            ExactCoordinateValue::from(value).hash(&mut hasher);
        }
        hasher.finish()
    }
}

impl RowIndex {
    /// Whether a row of `dataset`, an embedded table, holds exactly each
    /// value of `wanted` in the column it names.
    pub(super) fn holds(&self, dataset: &SourceDataset, wanted: &[(usize, &TypedValue)]) -> bool {
        if wanted.is_empty() {
            return !dataset.rows.is_empty();
        }
        let mut tables = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let columns: Vec<usize> = wanted.iter().map(|(column, _)| *column).collect();
        let key = tables.hash(wanted.iter().map(|(_, value)| *value));
        let table = (dataset.binding, dataset.rows.len(), columns);
        if !tables.tables.contains_key(&table) {
            let mut rows = RowsByHash::new();
            for (index, row) in dataset.rows.iter().enumerate() {
                let hash = tables.hash(table.2.iter().map(|column| &row.values[*column]));
                rows.entry(hash).or_default().push(index);
            }
            tables.tables.insert(table.clone(), rows);
        }
        tables.tables[&table].get(&key).is_some_and(|rows| {
            rows.iter().any(|&index| {
                wanted
                    .iter()
                    .all(|(column, value)| dataset.rows[index].values[*column].exact_eq(value))
            })
        })
    }

    /// Drop every table read: the check that read them has ended.
    pub(super) fn clear(&self) {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .tables
            .clear();
    }
}

impl Clone for RowIndex {
    /// A copy of a document starts with no table read.
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl PartialEq for RowIndex {
    /// What a check read is not part of a document.
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Debug for RowIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RowIndex")
    }
}

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

/// How many traces a measurement may name. One the reader authored (an
/// expression, single or over a family) names its signals in its expression,
/// as `.MEASURE` does, and references no trace: it is bound to the result,
/// never to what the pane plots. A measurement of a kind reads at least one
/// of its pane's traces.
pub(super) fn validate_measurement_traces(
    count: usize,
    authored: bool,
) -> Result<(), VisualizationError> {
    if authored && count != 0 {
        return Err(VisualizationError::InvalidValue {
            field: "measurement.traces",
            message: "an authored measurement names its signals in its expression, not traces"
                .to_owned(),
        });
    }
    if !authored && count == 0 {
        return Err(VisualizationError::InvalidValue {
            field: "measurement.traces",
            message: "a measurement reads at least one trace".to_owned(),
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

pub(super) fn validate_dataset_set(datasets: &[SourceDataset]) -> Result<(), VisualizationError> {
    if datasets.is_empty() {
        return Err(VisualizationError::InvalidValue {
            field: "visualization-document.datasets",
            message: "a visualization document reads at least one dataset".to_owned(),
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

/// Whether a trace reading `signal` against `coordinate` can be drawn: a
/// signal column against a coordinate column, or against another signal
/// column, as a transfer curve or a hysteresis loop is.
pub(super) fn trace_columns_are_drawable(
    dataset: &SourceDataset,
    signal: &str,
    coordinate: &str,
) -> Result<bool, VisualizationError> {
    let signal_role = find_column(dataset, signal)?.role;
    let coordinate_role = find_column(dataset, coordinate)?.role;
    Ok(signal_role == ColumnRole::Signal
        && (coordinate_role == ColumnRole::Coordinate
            || (coordinate_role == ColumnRole::Signal && signal != coordinate)))
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
    document: &dyn Entities,
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
            document.require_trace_in_pane(*trace_id, pane_id)?;
            if trace_contains_exact_coordinate(document, *trace_id, coordinate)? {
                Ok(())
            } else {
                Err(VisualizationError::InterpolationRequired)
            }
        }
        _ => Ok(()),
    }
}

pub(super) fn trace_contains_exact_coordinate(
    document: &dyn Entities,
    trace_id: TraceId,
    coordinate: &TypedValue,
) -> Result<bool, VisualizationError> {
    coordinate.validate("trace.coordinate")?;
    let trace = document.trace(trace_id)?;
    let dataset = document.dataset(trace.binding)?;
    validate_trace_row_predicates(dataset, &trace.row_predicates, document.rows())?;
    let coordinate_index = column_index(dataset, &trace.coordinate_key)?;
    if coordinate.value_type() != dataset.columns[coordinate_index].value_type {
        return Err(VisualizationError::ColumnTypeMismatch {
            column: trace.coordinate_key.to_string(),
            expected: dataset.columns[coordinate_index].value_type,
            actual: coordinate.value_type(),
        });
    }
    if let Some(run) = &dataset.run {
        // A run reference holds no samples, so the coordinate is checked here
        // only against the samples a report copy carries. Otherwise the host
        // that places it snaps it onto the run's samples, and the viewer reads
        // it against the run.
        let series = run.select_one(&trace.row_predicates)?;
        return Ok(match (coordinate, run.figure_samples(&series)) {
            (TypedValue::Real(x), Some(samples)) => samples
                .x()
                .iter()
                .any(|sample| sample.to_bits() == x.to_bits()),
            _ => true,
        });
    }
    let mut wanted = vec![(coordinate_index, coordinate)];
    for predicate in trace.row_predicates.iter() {
        wanted.push((column_index(dataset, &predicate.column)?, &predicate.value));
    }
    Ok(document.rows().holds(dataset, &wanted))
}

pub(super) fn validate_link_members(
    document: &dyn Entities,
    kind: LinkKind,
    members: &[EntityRef],
) -> Result<(), VisualizationError> {
    if members.len() < 2 || members.iter().collect::<HashSet<_>>().len() != members.len() {
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

pub(super) fn validate_trace_row_predicates(
    dataset: &SourceDataset,
    predicates: &[QueryCoordinate],
    rows: &RowIndex,
) -> Result<(), VisualizationError> {
    let mut resolved = Vec::with_capacity(predicates.len());
    let mut seen = HashSet::with_capacity(predicates.len());
    for predicate in predicates {
        validate_key("trace.row-predicate.column", &predicate.column)?;
        predicate.value.validate("trace.row-predicate.value")?;
        let index = column_index(dataset, &predicate.column)?;
        let column = &dataset.columns[index];
        if column.role != ColumnRole::Coordinate || !seen.insert(index) {
            return Err(VisualizationError::InvalidValue {
                field: "trace.row-predicates",
                message: "trace row predicates must name distinct coordinate columns".to_owned(),
            });
        }
        if predicate.value.value_type() != column.value_type {
            return Err(VisualizationError::ColumnTypeMismatch {
                column: column.key.clone(),
                expected: column.value_type,
                actual: predicate.value.value_type(),
            });
        }
        resolved.push((index, &predicate.value));
    }
    if let Some(run) = &dataset.run {
        run.select_one(predicates)?;
        return Ok(());
    }
    if !rows.holds(dataset, &resolved) {
        return Err(VisualizationError::InvalidValue {
            field: "trace.row-predicates",
            message: "trace row predicates select no exact source rows".to_owned(),
        });
    }
    Ok(())
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

/// Compare two authenticated source datasets under a validated request.
pub fn compare_source_datasets(
    baseline: &SourceDataset,
    candidate: &SourceDataset,
    request: &ComparisonRequest,
) -> Result<ComparisonReceipt, VisualizationError> {
    if baseline.run.is_some() || candidate.run.is_some() {
        return Err(VisualizationError::InvalidValue {
            field: "comparison.source",
            message: "a run reference holds no samples; compare the retained runs themselves"
                .to_owned(),
        });
    }
    if request.baseline != baseline.binding || request.candidate != candidate.binding {
        return Err(VisualizationError::InvalidValue {
            field: "comparison.binding",
            message: "request bindings must exactly match the immutable source datasets".to_owned(),
        });
    }
    if request.signal_keys.is_empty() {
        return Err(VisualizationError::EmptyComparison);
    }
    let mut unique_signals = HashSet::with_capacity(request.signal_keys.len());
    for signal_key in &request.signal_keys {
        validate_key("comparison.signal-key", signal_key)?;
        if !unique_signals.insert(signal_key.as_str()) {
            return Err(VisualizationError::DuplicateKey(signal_key.clone()));
        }
    }
    request.policy.tolerance.validate()?;
    request.policy.execution.validate()?;
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
        let mut tally = ErrorTally::default();
        for (baseline_row, candidate_row) in &row_pairs {
            let (TypedValue::Real(baseline_value), TypedValue::Real(candidate_value)) = (
                &baseline_row.values[baseline_index],
                &candidate_row.values[candidate_index],
            ) else {
                return Err(VisualizationError::InvalidValue {
                    field: "comparison.samples",
                    message: format!(
                        "signal '{signal_key}' contains unavailable samples; a complete numeric comparison cannot be certified"
                    ),
                });
            };
            tally.add(*baseline_value, *candidate_value, request.policy.tolerance);
        }
        signal_receipts.push(tally.into_signal(signal_key.clone()));
    }
    let disposition = if signal_receipts.iter().any(|signal| signal.failed_rows > 0) {
        ComparisonDisposition::Failed
    } else {
        ComparisonDisposition::Passed
    };
    let receipt = ComparisonReceipt {
        baseline: baseline.binding,
        candidate: candidate.binding,
        policy: request.policy.clone(),
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

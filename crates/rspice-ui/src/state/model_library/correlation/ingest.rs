//! Reading measured observations out of a correlation CSV.
//!
//! Parsing is strict on purpose: every numeric cell must be finite and in
//! range, every required column must be present, and a malformed row fails the
//! whole import rather than being skipped. A silently dropped measurement
//! would change a correlation result without changing anything visible, which
//! is the one failure mode this data cannot have.

use super::*;

pub(super) fn parse_correlation_csv(raw: &[u8]) -> CorrelationResult<Vec<CorrelationObservation>> {
    if raw.is_empty() || raw.len() > MAX_CORRELATION_SOURCE_BYTES {
        return Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            "csv",
            format!("CSV source must contain 1..={MAX_CORRELATION_SOURCE_BYTES} bytes"),
        ));
    }
    std::str::from_utf8(raw).map_err(|error| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv",
            format!("correlation CSV must be strict UTF-8: {error}"),
        )
    })?;
    let csv_source = raw.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(raw);
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(false)
        .from_reader(csv_source);
    let headers = reader.headers().map_err(csv_error)?.clone();
    if headers.is_empty() || headers.len() > MAX_CORRELATION_COLUMNS {
        return Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            "csv.headers",
            format!("CSV must contain 1..={MAX_CORRELATION_COLUMNS} columns"),
        ));
    }
    let mut header_names = BTreeSet::new();
    for (index, header) in headers.iter().enumerate() {
        require_text(&format!("csv.headers[{index}]"), header)?;
        bounded_text(&format!("csv.headers[{index}]"), header)?;
        if !header_names.insert(normalized(header)) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DuplicateIdentity,
                format!("csv.headers[{index}]"),
                "CSV header names must be unique ignoring case and surrounding whitespace",
            ));
        }
    }
    let required = [
        ("id", required_column(&headers, "id")?),
        ("quantity", required_column(&headers, "quantity")?),
        ("value", required_column(&headers, "value")?),
        ("unit", required_column(&headers, "unit")?),
    ];
    let uncertainty = optional_column(&headers, "uncertainty");
    let weight = optional_column(&headers, "weight");
    let known = required
        .iter()
        .map(|(_, index)| *index)
        .chain(uncertainty)
        .chain(weight)
        .collect::<BTreeSet<_>>();
    let mut conditions = Vec::new();
    for (index, header) in headers.iter().enumerate() {
        if known.contains(&index) {
            continue;
        }
        conditions.push((index, parse_condition_header(header)?));
    }

    let mut observations = Vec::new();
    let mut ids = BTreeSet::new();
    for (row_index, row) in reader.records().enumerate() {
        if row_index >= MAX_CORRELATION_ROWS {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ResourceLimit,
                "csv.rows",
                format!("CSV exceeds the {MAX_CORRELATION_ROWS}-row import limit"),
            ));
        }
        let row = row.map_err(csv_error)?;
        validate_cells(&row, row_index)?;
        let value = |column: usize| row.get(column).unwrap_or_default();
        let id = value(required[0].1).trim().to_owned();
        let quantity = value(required[1].1).trim().to_owned();
        let observed = parse_finite_cell(value(required[2].1), row_index, "value")?;
        let unit = value(required[3].1).trim().to_owned();
        require_text(&format!("csv.rows[{row_index}].id"), &id)?;
        require_text(&format!("csv.rows[{row_index}].quantity"), &quantity)?;
        require_text(&format!("csv.rows[{row_index}].unit"), &unit)?;
        unit_spec(&unit)?;
        if !ids.insert(normalized(&id)) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DuplicateIdentity,
                format!("csv.rows[{row_index}].id"),
                "observation IDs must be unique within the imported dataset",
            ));
        }
        let uncertainty = uncertainty.map_or(Ok(0.0), |column| {
            parse_non_negative_cell(value(column), row_index, "uncertainty")
        })?;
        let weight = weight.map_or(Ok(1.0), |column| {
            parse_positive_cell(value(column), row_index, "weight")
        })?;
        let mut coordinates = Vec::with_capacity(conditions.len());
        for (column, condition) in &conditions {
            let coordinate_value =
                parse_finite_cell(value(*column), row_index, &condition.dimension)?;
            coordinates.push(CorrelationCoordinate {
                dimension: condition.dimension.clone(),
                value: finite("csv.condition", coordinate_value)?,
                unit: condition.unit.clone(),
            });
        }
        coordinates.sort_by_key(|coordinate| normalized(&coordinate.dimension));
        let observation = CorrelationObservation {
            id,
            quantity,
            value: finite("csv.value", observed)?,
            unit,
            uncertainty: non_negative("csv.uncertainty", uncertainty)?,
            weight: non_negative("csv.weight", weight)?,
            coordinates,
        };
        observation.validate(&format!("csv.rows[{row_index}]"))?;
        observations.push(observation);
    }
    if observations.is_empty() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MissingValue,
            "csv.rows",
            "correlation CSV contains no data rows",
        ));
    }
    Ok(observations)
}

pub(super) fn required_column(headers: &StringRecord, name: &str) -> CorrelationResult<usize> {
    optional_column(headers, name).ok_or_else(|| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("required CSV column '{name}' is missing"),
        )
    })
}

pub(super) fn optional_column(headers: &StringRecord, name: &str) -> Option<usize> {
    headers
        .iter()
        .position(|header| header.trim().eq_ignore_ascii_case(name))
}

pub(super) fn parse_condition_header(
    header: &str,
) -> CorrelationResult<CorrelationHeaderCondition> {
    let Some((prefix, rest)) = header.trim().split_once(':') else {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("unknown column '{header}'; condition columns use condition:<name>[<unit>]"),
        ));
    };
    if !prefix.eq_ignore_ascii_case("condition") {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("unknown column '{header}'; condition columns use condition:<name>[<unit>]"),
        ));
    }
    let (dimension, unit) = if let Some(open) = rest.rfind('[') {
        if !rest.ends_with(']') {
            return Err(CorrelationError::new(
                CorrelationErrorCode::InvalidCsv,
                "csv.headers",
                format!("condition column '{header}' has an unterminated unit"),
            ));
        }
        (&rest[..open], &rest[open + 1..rest.len() - 1])
    } else {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("condition column '{header}' must declare a unit"),
        ));
    };
    require_text("csv.condition.dimension", dimension)?;
    require_text("csv.condition.unit", unit)?;
    unit_spec(unit)?;
    Ok(CorrelationHeaderCondition {
        dimension: dimension.trim().to_owned(),
        unit: unit.trim().to_owned(),
    })
}

#[derive(Debug)]
pub(super) struct CorrelationHeaderCondition {
    dimension: String,
    unit: String,
}

pub(super) fn validate_cells(row: &StringRecord, row_index: usize) -> CorrelationResult<()> {
    for (column, cell) in row.iter().enumerate() {
        if cell.len() > MAX_CORRELATION_TEXT_BYTES {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ResourceLimit,
                format!("csv.rows[{row_index}][{column}]"),
                format!("CSV cells are limited to {MAX_CORRELATION_TEXT_BYTES} UTF-8 bytes"),
            ));
        }
    }
    Ok(())
}

pub(super) fn parse_finite_cell(value: &str, row: usize, field: &str) -> CorrelationResult<f64> {
    let value = value.trim().parse::<f64>().map_err(|_| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a finite decimal number",
        )
    })?;
    if !value.is_finite() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a finite decimal number",
        ));
    }
    Ok(if value == 0.0 { 0.0 } else { value })
}

pub(super) fn parse_non_negative_cell(
    value: &str,
    row: usize,
    field: &str,
) -> CorrelationResult<f64> {
    let value = parse_finite_cell(value, row, field)?;
    if value < 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a non-negative value",
        ));
    }
    Ok(value)
}

pub(super) fn parse_positive_cell(value: &str, row: usize, field: &str) -> CorrelationResult<f64> {
    let value = parse_finite_cell(value, row, field)?;
    if value <= 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a value greater than zero",
        ));
    }
    Ok(value)
}

pub(super) fn csv_error(error: csv::Error) -> CorrelationError {
    CorrelationError::new(
        CorrelationErrorCode::InvalidCsv,
        "csv",
        format!("CSV parsing failed: {error}"),
    )
}

pub(super) fn validate_disposition_ledger(
    suite: &CorrelationSuite,
    metric_ids: &BTreeSet<String>,
    datasets: &[&CorrelationDatasetRevision],
) -> CorrelationResult<()> {
    let mut by_id = BTreeMap::<String, &CorrelationOutlierDisposition>::new();
    let mut current = BTreeMap::<(String, String), &CorrelationOutlierDisposition>::new();
    for (index, disposition) in suite.dispositions.iter().enumerate() {
        let path = format!("suite.dispositions[{index}]");
        disposition.validate(&path)?;
        if !metric_ids.contains(&normalized(&disposition.metric_id)) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.metric_id"),
                "outlier disposition metric does not exist",
            ));
        }
        let disposition_metric_id = normalized(&disposition.metric_id);
        let metric = suite
            .metrics
            .iter()
            .find(|metric| normalized(&metric.id) == disposition_metric_id)
            .ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.metric_id"),
                    "outlier disposition metric does not exist",
                )
            })?;
        let reference = find_ci(datasets, &metric.reference_dataset_id).ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.metric_id"),
                "outlier disposition reference dataset does not exist",
            )
        })?;
        if !reference.observations.iter().any(|observation| {
            observation
                .id
                .eq_ignore_ascii_case(&disposition.reference_observation_id)
        }) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.reference_observation_id"),
                "outlier disposition observation does not exist in the metric reference dataset",
            ));
        }
        let id_key = normalized(&disposition.id);
        if by_id.insert(id_key.clone(), disposition).is_some() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DuplicateIdentity,
                format!("{path}.id"),
                "disposition event IDs must be unique",
            ));
        }
        let subject = (
            normalized(&disposition.metric_id),
            normalized(&disposition.reference_observation_id),
        );
        match (current.get(&subject), disposition.supersedes.as_deref()) {
            (None, None) => {}
            (Some(previous), Some(supersedes))
                if previous.id.eq_ignore_ascii_case(supersedes)
                    && disposition.decided_at_unix_ms > previous.decided_at_unix_ms => {}
            (Some(previous), Some(supersedes)) if previous.id.eq_ignore_ascii_case(supersedes) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.decided_at_unix_ms"),
                    "a superseding disposition must have a strictly later decision timestamp",
                ));
            }
            (None, Some(_)) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.supersedes"),
                    "the superseded disposition is not an earlier event for this subject",
                ));
            }
            (Some(_), None) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.supersedes"),
                    "a later decision must explicitly supersede the current disposition",
                ));
            }
            (Some(_), Some(_)) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.supersedes"),
                    "a later decision must supersede the immediately prior disposition",
                ));
            }
        }
        current.insert(subject, disposition);
    }
    Ok(())
}

pub(super) fn current_dispositions(
    suite: &CorrelationSuite,
) -> CorrelationResult<BTreeMap<(String, String), &CorrelationOutlierDisposition>> {
    let latest = suite.latest_datasets()?;
    let metric_ids = suite
        .metrics
        .iter()
        .map(|metric| normalized(&metric.id))
        .collect();
    validate_disposition_ledger(suite, &metric_ids, &latest)?;
    let mut current = BTreeMap::new();
    for disposition in &suite.dispositions {
        current.insert(
            (
                normalized(&disposition.metric_id),
                normalized(&disposition.reference_observation_id),
            ),
            disposition,
        );
    }
    Ok(current)
}

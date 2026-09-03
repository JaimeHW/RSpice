//! Strict, versioned provenance contract for adapter STEP/TEMP execution.

use std::collections::{BTreeMap, HashMap, HashSet};

use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::execution::{AxisKind, numeric_run_coordinate_id};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::measure::canonical_decimal;
use crate::wire::{MAX_ENGINE_RESULT_MANIFEST_BYTES, valid_result_path};
use rspice_core::execution::bounded_io::{BoundedAbortWriter, BoundedWriteFailure};

pub const AXIS_EXECUTION_SCHEMA: &str = "rspice-axis-execution";
pub const AXIS_EXECUTION_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AxisExecutionDocument {
    pub schema: String,
    pub schema_version: u32,
    pub analysis_kind: AxisAnalysisKind,
    pub coordinate_count: usize,
    pub execution_count: usize,
    pub runs: Vec<CoordinateExecution>,
}

impl AxisExecutionDocument {
    pub fn new(
        analysis_kind: AxisAnalysisKind,
        runs: Vec<CoordinateExecution>,
    ) -> Result<Self, AxisExecutionDocumentError> {
        Self::new_with_abort(analysis_kind, runs, &NoAbort)
    }

    pub fn new_with_abort(
        analysis_kind: AxisAnalysisKind,
        runs: Vec<CoordinateExecution>,
        abort: &dyn AbortSignal,
    ) -> Result<Self, AxisExecutionDocumentError> {
        check_abort(abort)?;
        let execution_count = runs.iter().try_fold(0usize, |count, run| {
            count
                .checked_add(run.analyses.len())
                .ok_or_else(|| invalid("execution count overflows usize"))
        })?;
        let document = Self {
            schema: AXIS_EXECUTION_SCHEMA.to_owned(),
            schema_version: AXIS_EXECUTION_VERSION,
            analysis_kind,
            coordinate_count: runs.len(),
            execution_count,
            runs,
        };
        document.validate_with_abort(abort)?;
        Ok(document)
    }

    pub fn to_value(&self) -> Result<serde_json::Value, AxisExecutionDocumentError> {
        self.to_value_with_abort(&NoAbort, MAX_ENGINE_RESULT_MANIFEST_BYTES as u64)
    }

    pub fn to_value_with_abort(
        &self,
        abort: &dyn AbortSignal,
        byte_limit: u64,
    ) -> Result<serde_json::Value, AxisExecutionDocumentError> {
        self.validate_with_abort(abort)?;
        check_abort(abort)?;
        let mut writer = BoundedAbortWriter::new(abort, byte_limit);
        if let Err(error) = serde_json::to_writer(&mut writer, self) {
            return Err(map_bounded_json_error(error, &writer));
        }
        check_abort(abort)?;
        serde_json::from_slice(&writer.into_bytes())
            .map_err(AxisExecutionDocumentError::InvalidJson)
    }

    pub fn from_value(value: serde_json::Value) -> Result<Self, AxisExecutionDocumentError> {
        Self::from_value_with_abort(value, &NoAbort)
    }

    pub fn from_value_with_abort(
        value: serde_json::Value,
        abort: &dyn AbortSignal,
    ) -> Result<Self, AxisExecutionDocumentError> {
        check_abort(abort)?;
        let schema = value
            .get("schema")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| invalid("axis execution schema identity is missing"))?;
        if schema != AXIS_EXECUTION_SCHEMA {
            return Err(AxisExecutionDocumentError::WrongSchema(schema.to_owned()));
        }
        let schema_version = value
            .get("schema_version")
            .and_then(serde_json::Value::as_u64)
            .and_then(|version| u32::try_from(version).ok())
            .ok_or_else(|| invalid("axis execution schema version is missing"))?;
        if schema_version != AXIS_EXECUTION_VERSION {
            return Err(AxisExecutionDocumentError::UnsupportedVersion {
                found: schema_version,
                current: AXIS_EXECUTION_VERSION,
            });
        }
        let document: Self =
            serde_json::from_value(value).map_err(AxisExecutionDocumentError::InvalidJson)?;
        document.validate_with_abort(abort)?;
        Ok(document)
    }

    pub fn from_json_with_abort(
        json: &str,
        abort: &dyn AbortSignal,
        byte_limit: u64,
    ) -> Result<Self, AxisExecutionDocumentError> {
        check_abort(abort)?;
        if json.len() as u128 > byte_limit as u128 {
            return Err(AxisExecutionDocumentError::DocumentTooLarge {
                limit_bytes: byte_limit,
            });
        }
        let value = serde_json::from_str(json).map_err(AxisExecutionDocumentError::InvalidJson)?;
        check_abort(abort)?;
        Self::from_value_with_abort(value, abort)
    }

    pub fn validate(&self) -> Result<(), AxisExecutionDocumentError> {
        self.validate_with_abort(&NoAbort)
    }

    pub fn validate_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<(), AxisExecutionDocumentError> {
        check_abort(abort)?;
        if self.schema != AXIS_EXECUTION_SCHEMA {
            return Err(AxisExecutionDocumentError::WrongSchema(self.schema.clone()));
        }
        if self.schema_version != AXIS_EXECUTION_VERSION {
            return Err(AxisExecutionDocumentError::UnsupportedVersion {
                found: self.schema_version,
                current: AXIS_EXECUTION_VERSION,
            });
        }
        if self.runs.is_empty() || self.coordinate_count != self.runs.len() {
            return Err(invalid(
                "analysis kind and a nonempty exact coordinate set are required",
            ));
        }

        let mut coordinate_ids = HashSet::new();
        let mut coordinate_namespaces = HashSet::new();
        let mut artifact_paths = HashSet::new();
        let mut execution_count = 0usize;
        let mut expected_analysis_ids: Option<Vec<String>> = None;
        let mut expected_axes: Option<
            Vec<(AxisAssignmentKind, String, Option<StepTargetDocument>)>,
        > = None;
        let mut axis_values: Option<Vec<BTreeMap<usize, String>>> = None;
        let mut coordinate_index_tuples = HashSet::new();
        let mut semantic_occurrences = HashMap::<[u8; 16], u32>::new();
        for (run_index, run) in self.runs.iter().enumerate() {
            check_abort(abort)?;
            if run.ordinal != run_index + 1
                || !valid_coordinate_id(&run.coordinate_id)
                || run.coordinate_namespace != format!("run-{}", run.coordinate_id)
                || run.assignments.is_empty()
                || run.analyses.is_empty()
                || !coordinate_ids.insert(run.coordinate_id.clone())
                || !coordinate_namespaces.insert(run.coordinate_namespace.clone())
            {
                return Err(invalid(
                    "coordinate identity, order, axes, or analysis set is invalid",
                ));
            }
            let mut assignment_names = HashSet::new();
            for assignment in &run.assignments {
                check_abort(abort)?;
                assignment.validate()?;
                if !assignment_names.insert(assignment.name.clone()) {
                    return Err(invalid("axis assignment names are not unique"));
                }
            }
            let axes = run
                .assignments
                .iter()
                .map(|assignment| {
                    (
                        assignment.kind,
                        assignment.name.clone(),
                        assignment.target.clone(),
                    )
                })
                .collect::<Vec<_>>();
            if let Some(expected) = &expected_axes {
                if expected != &axes {
                    return Err(invalid(
                        "every coordinate must preserve the same ordered axis descriptors",
                    ));
                }
            } else {
                expected_axes = Some(axes);
                axis_values = Some(vec![BTreeMap::new(); run.assignments.len()]);
            }
            let values = axis_values
                .as_mut()
                .ok_or_else(|| invalid("axis value catalog was not initialized"))?;
            for (axis_index, assignment) in run.assignments.iter().enumerate() {
                match values[axis_index].entry(assignment.value_index) {
                    std::collections::btree_map::Entry::Vacant(entry) => {
                        entry.insert(assignment.value_decimal.clone());
                    }
                    std::collections::btree_map::Entry::Occupied(entry)
                        if entry.get() == &assignment.value_decimal => {}
                    std::collections::btree_map::Entry::Occupied(_) => {
                        return Err(invalid(
                            "one axis value index maps to inconsistent numeric values",
                        ));
                    }
                }
            }
            let index_tuple = run
                .assignments
                .iter()
                .map(|assignment| assignment.value_index)
                .collect::<Vec<_>>();
            if !coordinate_index_tuples.insert(index_tuple) {
                return Err(invalid(
                    "coordinate value-index tuples must be unique within the Cartesian plan",
                ));
            }
            let mut numeric_assignments = Vec::new();
            numeric_assignments
                .try_reserve_exact(run.assignments.len())
                .map_err(|_| invalid("unable to verify coordinate identity"))?;
            for assignment in &run.assignments {
                let value = assignment
                    .value_decimal
                    .parse::<f64>()
                    .map_err(|_| invalid("axis value is not a finite canonical decimal"))?;
                numeric_assignments.push((
                    match assignment.kind {
                        AxisAssignmentKind::Step => AxisKind::Step,
                        AxisAssignmentKind::Temperature => AxisKind::Temperature,
                    },
                    assignment.name.as_str(),
                    value,
                ));
            }
            let first_occurrence = numeric_run_coordinate_id(&numeric_assignments, 0)
                .map_err(|error| invalid(&format!("coordinate identity is invalid: {error}")))?;
            let occurrence = semantic_occurrences
                .entry(first_occurrence.semantic_bytes())
                .or_insert(0);
            let expected_coordinate_id =
                numeric_run_coordinate_id(&numeric_assignments, *occurrence)
                    .map_err(|error| invalid(&format!("coordinate identity is invalid: {error}")))?
                    .to_string();
            *occurrence = occurrence
                .checked_add(1)
                .ok_or_else(|| invalid("coordinate occurrence count overflows u32"))?;
            if run.coordinate_id != expected_coordinate_id {
                return Err(invalid(
                    "coordinate ID does not match its numeric axis assignments",
                ));
            }
            let analysis_ids = run
                .analyses
                .iter()
                .map(|analysis| analysis.analysis_id.clone())
                .collect::<Vec<_>>();
            if let Some(expected) = &expected_analysis_ids {
                if expected != &analysis_ids {
                    return Err(invalid(
                        "every coordinate must preserve the same ordered analysis identities",
                    ));
                }
            } else {
                validate_analysis_ids(self.analysis_kind, &analysis_ids)?;
                expected_analysis_ids = Some(analysis_ids);
            }
            let mut analysis_ids = HashSet::new();
            for analysis in &run.analyses {
                check_abort(abort)?;
                execution_count = execution_count
                    .checked_add(1)
                    .ok_or_else(|| invalid("execution count overflows usize"))?;
                if analysis.analysis_id.trim().is_empty()
                    || analysis.output_namespace.coordinate != run.coordinate_namespace
                    || analysis.output_namespace.analysis != analysis.analysis_id
                    || analysis.artifacts.is_empty()
                    || !analysis_ids.insert(analysis.analysis_id.clone())
                {
                    return Err(invalid("analysis identity or output namespace is invalid"));
                }
                for artifact in &analysis.artifacts {
                    check_abort(abort)?;
                    artifact.validate()?;
                    if !artifact_paths.insert(artifact.path.to_ascii_lowercase()) {
                        return Err(invalid("result artifact path is duplicated"));
                    }
                }
                let mut measurement_names = HashSet::new();
                for measurement in &analysis.measurements {
                    check_abort(abort)?;
                    measurement.validate()?;
                    if !measurement_names.insert(measurement.name.clone()) {
                        return Err(invalid("analysis measurement names are not unique"));
                    }
                }
            }
        }
        if execution_count != self.execution_count {
            return Err(invalid("execution count does not match the run contents"));
        }
        let axis_values = axis_values.ok_or_else(|| invalid("axis value catalog is missing"))?;
        let mut cardinalities = Vec::new();
        cardinalities
            .try_reserve_exact(axis_values.len())
            .map_err(|_| invalid("unable to verify coordinate cardinality"))?;
        let mut cartesian_count = 1usize;
        for values in &axis_values {
            if values.is_empty()
                || values
                    .keys()
                    .copied()
                    .enumerate()
                    .any(|(expected, actual)| expected != actual)
            {
                return Err(invalid("axis value indices must be dense from zero"));
            }
            cardinalities.push(values.len());
            cartesian_count = cartesian_count
                .checked_mul(values.len())
                .ok_or_else(|| invalid("Cartesian coordinate count overflows usize"))?;
        }
        if cartesian_count != self.coordinate_count {
            return Err(invalid(
                "coordinate set is not the complete Cartesian product",
            ));
        }
        for (run_index, run) in self.runs.iter().enumerate() {
            check_abort(abort)?;
            let mut stride = 1usize;
            for (axis_index, cardinality) in cardinalities.iter().copied().enumerate() {
                let expected = (run_index / stride) % cardinality;
                if run.assignments[axis_index].value_index != expected {
                    return Err(invalid(
                        "coordinate tuples are not in canonical first-axis-fastest order",
                    ));
                }
                stride = stride
                    .checked_mul(cardinality)
                    .ok_or_else(|| invalid("Cartesian coordinate stride overflows usize"))?;
            }
        }
        Ok(())
    }
}

/// The analysis family every coordinate of one axis execution ran.
///
/// One variant per family the executor runs. The `authored_prefix` is the core
/// `AnalysisKind::tag`, so the analysis identities this manifest validates are
/// the canonical planner's, not a second naming scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisAnalysisKind {
    OperatingPoint,
    DcSweep,
    Transient,
    AcSmallSignal,
    Noise,
    Distortion,
    TransferFunction,
    Stability,
    Sensitivity,
    PoleZero,
    MonteCarlo,
    HarmonicBalance,
    Pss,
    Pac,
    Envelope,
}

impl AxisAnalysisKind {
    /// Canonical planner tag every authored analysis identity of this family
    /// starts with.
    pub const fn authored_prefix(self) -> &'static str {
        match self {
            Self::OperatingPoint => "op",
            Self::DcSweep => "dc",
            Self::Transient => "tran",
            Self::AcSmallSignal => "ac",
            Self::Noise => "noise",
            Self::Distortion => "disto",
            Self::TransferFunction => "tf",
            Self::Stability => "stb",
            Self::Sensitivity => "sens",
            Self::PoleZero => "pz",
            Self::MonteCarlo => "mc",
            Self::HarmonicBalance => "hb",
            Self::Pss => "pss",
            Self::Pac => "pac",
            Self::Envelope => "env",
        }
    }
}

/// One typed result document an executed analysis published.
///
/// The manifest is an orchestration record, not a second result schema: it
/// names each artifact and the exact document schema and family inside it, so
/// a reader can pick the documents it understands without opening them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultDocumentReference {
    /// `results/`-relative path, exactly as the response declares it.
    pub path: String,
    /// Declared MIME type of the artifact.
    pub content_type: String,
    /// Schema identifier written inside the document.
    pub schema: String,
    /// Schema version written inside the document.
    pub schema_version: u32,
    /// Result family tag the document declares.
    pub result_kind: String,
}

impl ResultDocumentReference {
    fn validate(&self) -> Result<(), AxisExecutionDocumentError> {
        if !valid_result_path(&self.path)
            || self.content_type.trim().is_empty()
            || self.schema.trim().is_empty()
            || self.result_kind.trim().is_empty()
        {
            return Err(invalid(
                "result document reference path, content type, schema, or family is invalid",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinateExecution {
    pub ordinal: usize,
    pub coordinate_id: String,
    pub coordinate_namespace: String,
    pub assignments: Vec<AxisAssignmentDocument>,
    pub analyses: Vec<AnalysisExecution>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisExecution {
    pub analysis_id: String,
    pub output_namespace: OutputNamespaceDocument,
    pub artifacts: Vec<ResultDocumentReference>,
    pub measurements: Vec<MeasurementDocument>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputNamespaceDocument {
    pub coordinate: String,
    pub analysis: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AxisAssignmentDocument {
    pub kind: AxisAssignmentKind,
    pub name: String,
    pub value_index: usize,
    pub value_decimal: String,
    pub target: Option<StepTargetDocument>,
}

impl AxisAssignmentDocument {
    fn validate(&self) -> Result<(), AxisExecutionDocumentError> {
        let value = self
            .value_decimal
            .parse::<f64>()
            .map_err(|_| invalid("axis value is not a finite canonical decimal"))?;
        if self.name.trim().is_empty()
            || !value.is_finite()
            || canonical_decimal(value).as_deref() != Some(self.value_decimal.as_str())
        {
            return Err(invalid("axis assignment name or value is invalid"));
        }
        let expected_name = match (self.kind, self.target.as_ref()) {
            (AxisAssignmentKind::Step, Some(StepTargetDocument::Parameter { name }))
                if valid_normalized_name(name) =>
            {
                format!("param:{name}")
            }
            (AxisAssignmentKind::Step, Some(StepTargetDocument::Device { name, parameter }))
                if valid_normalized_name(name)
                    && parameter
                        .as_ref()
                        .is_none_or(|value| valid_normalized_name(value)) =>
            {
                parameter.as_ref().map_or_else(
                    || format!("device:{name}"),
                    |parameter| format!("device:{name}:{parameter}"),
                )
            }
            (AxisAssignmentKind::Step, Some(StepTargetDocument::Model { name, parameter }))
                if valid_normalized_name(name) && valid_normalized_name(parameter) =>
            {
                format!("model:{name}:{parameter}")
            }
            (AxisAssignmentKind::Temperature, None)
            | (AxisAssignmentKind::Temperature, Some(StepTargetDocument::Temperature)) => {
                "temperature".to_owned()
            }
            _ => return Err(invalid("axis kind and STEP target are inconsistent")),
        };
        if self.name != expected_name {
            return Err(invalid(
                "axis assignment name does not match its typed STEP target",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisAssignmentKind {
    Step,
    Temperature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StepTargetDocument {
    Parameter {
        name: String,
    },
    Device {
        name: String,
        parameter: Option<String>,
    },
    Model {
        name: String,
        parameter: String,
    },
    Temperature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementDocument {
    pub name: String,
    pub unit: String,
    pub value_decimal: String,
    pub sample_count: usize,
    pub series_sha256: Option<String>,
}

impl MeasurementDocument {
    fn validate(&self) -> Result<(), AxisExecutionDocumentError> {
        let value = self
            .value_decimal
            .parse::<f64>()
            .map_err(|_| invalid("measurement value is not a finite canonical decimal"))?;
        if self.name.trim().is_empty()
            || self.unit.trim().is_empty()
            || self.sample_count == 0
            || !value.is_finite()
            || canonical_decimal(value).as_deref() != Some(self.value_decimal.as_str())
            || self.series_sha256.as_ref().is_some_and(|digest| {
                digest.len() != 64
                    || !digest
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            })
            || (self.sample_count > 1 && self.series_sha256.is_none())
        {
            return Err(invalid(
                "measurement identity, value, shape, or digest is invalid",
            ));
        }
        Ok(())
    }
}

fn invalid(message: &str) -> AxisExecutionDocumentError {
    AxisExecutionDocumentError::InvalidDocument(message.to_owned())
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), AxisExecutionDocumentError> {
    if abort.is_aborted() {
        Err(AxisExecutionDocumentError::Aborted)
    } else {
        Ok(())
    }
}

fn map_bounded_json_error(
    error: serde_json::Error,
    writer: &BoundedAbortWriter<'_>,
) -> AxisExecutionDocumentError {
    match writer.failure() {
        Some(BoundedWriteFailure::Aborted) => AxisExecutionDocumentError::Aborted,
        Some(BoundedWriteFailure::ByteLimitExceeded { limit_bytes }) => {
            AxisExecutionDocumentError::DocumentTooLarge { limit_bytes }
        }
        Some(BoundedWriteFailure::AllocationFailed) => {
            invalid("unable to allocate bounded axis JSON")
        }
        None => AxisExecutionDocumentError::InvalidJson(error),
    }
}

fn valid_normalized_name(value: &str) -> bool {
    !value.is_empty() && value.trim() == value && value == value.to_ascii_lowercase()
}

fn valid_coordinate_id(value: &str) -> bool {
    if value.len() < 36 || value.as_bytes().get(32) != Some(&b'-') {
        return false;
    }
    let (semantic, occurrence) = value.split_at(32);
    let occurrence = &occurrence[1..];
    if !semantic
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        || occurrence.len() < 3
        || !occurrence.bytes().all(|byte| byte.is_ascii_digit())
        || (occurrence.len() > 3 && occurrence.starts_with('0'))
    {
        return false;
    }
    occurrence
        .parse::<u64>()
        .is_ok_and(|value| (1..=u64::from(u32::MAX) + 1).contains(&value))
}

fn validate_analysis_ids(
    kind: AxisAnalysisKind,
    ids: &[String],
) -> Result<(), AxisExecutionDocumentError> {
    let authored_prefix = kind.authored_prefix();
    let implicit_op =
        kind == AxisAnalysisKind::OperatingPoint && ids.len() == 1 && ids[0] == "implicit-op-001";
    if implicit_op {
        return Ok(());
    }
    for (index, id) in ids.iter().enumerate() {
        if id != &format!("{authored_prefix}-{:03}", index + 1) {
            return Err(invalid(
                "analysis IDs must match the requested kind and authored ordinal sequence",
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum AxisExecutionDocumentError {
    #[error("invalid axis execution JSON: {0}")]
    InvalidJson(#[source] serde_json::Error),
    #[error("unexpected axis execution schema {0:?}")]
    WrongSchema(String),
    #[error("axis execution schema version {found} is unsupported (current version is {current})")]
    UnsupportedVersion { found: u32, current: u32 },
    #[error("invalid axis execution document: {0}")]
    InvalidDocument(String),
    #[error("axis execution validation or serialization was cancelled")]
    Aborted,
    #[error("axis execution document exceeds the {limit_bytes}-byte limit")]
    DocumentTooLarge { limit_bytes: u64 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::ImmediateAbort;

    fn coordinate_id(resistance: f64) -> String {
        numeric_run_coordinate_id(&[(AxisKind::Step, "param:r", resistance)], 0)
            .expect("finite fixture coordinate")
            .to_string()
    }

    fn document() -> AxisExecutionDocument {
        let coordinate_id = coordinate_id(1_000.0);
        AxisExecutionDocument::new(
            AxisAnalysisKind::Transient,
            vec![CoordinateExecution {
                ordinal: 1,
                coordinate_namespace: format!("run-{coordinate_id}"),
                assignments: vec![AxisAssignmentDocument {
                    kind: AxisAssignmentKind::Step,
                    name: "param:r".to_owned(),
                    value_index: 0,
                    value_decimal: canonical_decimal(1_000.0).expect("finite fixture value"),
                    target: Some(StepTargetDocument::Parameter {
                        name: "r".to_owned(),
                    }),
                }],
                analyses: vec![AnalysisExecution {
                    analysis_id: "tran-001".to_owned(),
                    output_namespace: OutputNamespaceDocument {
                        coordinate: format!("run-{coordinate_id}"),
                        analysis: "tran-001".to_owned(),
                    },
                    artifacts: vec![ResultDocumentReference {
                        path: "results/run-a__tran-001.result.json".to_owned(),
                        content_type: "application/vnd.rspice.analysis-result+json;version=1"
                            .to_owned(),
                        schema: "rspice-analysis-result".to_owned(),
                        schema_version: 1,
                        result_kind: "tran".to_owned(),
                    }],
                    measurements: vec![MeasurementDocument {
                        name: "v(out)".to_owned(),
                        unit: "V".to_owned(),
                        value_decimal: "1e0".to_owned(),
                        sample_count: 1,
                        series_sha256: None,
                    }],
                }],
                coordinate_id,
            }],
        )
        .expect("valid document")
    }

    fn two_coordinate_document() -> AxisExecutionDocument {
        let mut single = document();
        let first = single.runs.remove(0);
        let mut second = first.clone();
        second.ordinal = 2;
        second.coordinate_id = coordinate_id(2_000.0);
        second.coordinate_namespace = format!("run-{}", second.coordinate_id);
        second.assignments[0].value_index = 1;
        second.assignments[0].value_decimal =
            canonical_decimal(2_000.0).expect("finite fixture value");
        for analysis in &mut second.analyses {
            analysis.output_namespace.coordinate = second.coordinate_namespace.clone();
            for artifact in &mut analysis.artifacts {
                artifact.path = artifact.path.replace("run-a", "run-b");
            }
        }
        AxisExecutionDocument::new(AxisAnalysisKind::Transient, vec![first, second])
            .expect("valid two-coordinate document")
    }

    #[test]
    fn coordinate_ids_are_recomputed_from_their_numeric_assignments() {
        let document = two_coordinate_document();
        assert_eq!(document.coordinate_count, 2);
        assert_ne!(
            document.runs[0].coordinate_id,
            document.runs[1].coordinate_id
        );

        let mut detached = document;
        let foreign_id = "fedcba9876543210fedcba9876543210-001".to_owned();
        let foreign_namespace = format!("run-{foreign_id}");
        let run = &mut detached.runs[1];
        run.coordinate_id = foreign_id;
        run.coordinate_namespace = foreign_namespace.clone();
        for analysis in &mut run.analyses {
            analysis.output_namespace.coordinate = foreign_namespace.clone();
        }
        assert!(matches!(
            detached.validate(),
            Err(AxisExecutionDocumentError::InvalidDocument(message))
                if message.contains("does not match its numeric axis assignments")
        ));
    }

    #[test]
    fn coordinates_must_form_the_complete_cartesian_product_in_canonical_order() {
        let mut swapped = two_coordinate_document();
        swapped.runs.swap(0, 1);
        swapped.runs[0].ordinal = 1;
        swapped.runs[1].ordinal = 2;
        assert!(matches!(
            swapped.validate(),
            Err(AxisExecutionDocumentError::InvalidDocument(message))
                if message.contains("canonical first-axis-fastest order")
        ));

        let mut sparse = two_coordinate_document();
        sparse.runs[1].assignments[0].value_index = 2;
        assert!(matches!(
            sparse.validate(),
            Err(AxisExecutionDocumentError::InvalidDocument(message))
                if message.contains("dense from zero")
        ));

        assert!(matches!(
            two_coordinate_document().validate_with_abort(&ImmediateAbort),
            Err(AxisExecutionDocumentError::Aborted)
        ));
    }

    #[test]
    fn current_version_round_trips_and_future_version_fails_closed() {
        let document = document();
        let value = document.to_value().expect("serialize");
        assert_eq!(
            AxisExecutionDocument::from_value(value.clone()).unwrap(),
            document
        );

        let mut future = value;
        future["schema_version"] = serde_json::json!(2);
        assert!(matches!(
            AxisExecutionDocument::from_value(future),
            Err(AxisExecutionDocumentError::UnsupportedVersion {
                found: 2,
                current: 1
            })
        ));
    }

    #[test]
    fn malformed_id_target_and_cross_coordinate_analysis_drift_fail_closed() {
        let valid = document();

        let mut bad_coordinate = valid.clone();
        bad_coordinate.runs[0].coordinate_id = "ABC-001".to_owned();
        assert!(bad_coordinate.validate().is_err());

        let mut bad_target = valid.clone();
        bad_target.runs[0].assignments[0].name = "param:not-r".to_owned();
        assert!(bad_target.validate().is_err());

        let mut bad_analysis = valid.clone();
        bad_analysis.runs[0].analyses[0].analysis_id = "ac-001".to_owned();
        bad_analysis.runs[0].analyses[0].output_namespace.analysis = "ac-001".to_owned();
        assert!(bad_analysis.validate().is_err());

        let mut reordered = valid;
        let mut second = reordered.runs[0].clone();
        second.ordinal = 2;
        second.coordinate_id = "fedcba9876543210fedcba9876543210-001".to_owned();
        second.coordinate_namespace = format!("run-{}", second.coordinate_id);
        second.assignments[0].value_index = 1;
        for analysis in &mut second.analyses {
            analysis.output_namespace.coordinate = second.coordinate_namespace.clone();
            for artifact in &mut analysis.artifacts {
                artifact.path = artifact.path.replace("run-a", "run-b");
            }
        }
        second.analyses[0].analysis_id = "tran-002".to_owned();
        second.analyses[0].output_namespace.analysis = "tran-002".to_owned();
        reordered.runs.push(second);
        reordered.coordinate_count = 2;
        reordered.execution_count = 2;
        assert!(reordered.validate().is_err());

        let mut axis_drift = document();
        let mut second = axis_drift.runs[0].clone();
        second.ordinal = 2;
        second.coordinate_id = "fedcba9876543210fedcba9876543210-001".to_owned();
        second.coordinate_namespace = format!("run-{}", second.coordinate_id);
        second.assignments[0].value_index = 1;
        second.assignments[0].name = "param:s".to_owned();
        second.assignments[0].target = Some(StepTargetDocument::Parameter {
            name: "s".to_owned(),
        });
        for analysis in &mut second.analyses {
            analysis.output_namespace.coordinate = second.coordinate_namespace.clone();
            for artifact in &mut analysis.artifacts {
                artifact.path = artifact.path.replace("run-a", "run-c");
            }
        }
        axis_drift.runs.push(second);
        axis_drift.coordinate_count = 2;
        axis_drift.execution_count = 2;
        assert!(axis_drift.validate().is_err());
    }
}

//! Strict, versioned provenance contract for adapter STEP/TEMP execution.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::measure::canonical_decimal;
use crate::wire::valid_result_path;

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
        document.validate()?;
        Ok(document)
    }

    pub fn to_value(&self) -> Result<serde_json::Value, AxisExecutionDocumentError> {
        self.validate()?;
        serde_json::to_value(self).map_err(AxisExecutionDocumentError::InvalidJson)
    }

    pub fn from_value(value: serde_json::Value) -> Result<Self, AxisExecutionDocumentError> {
        #[derive(Deserialize)]
        struct Header {
            schema: String,
            schema_version: u32,
        }
        let header: Header = serde_json::from_value(value.clone())
            .map_err(AxisExecutionDocumentError::InvalidJson)?;
        if header.schema != AXIS_EXECUTION_SCHEMA {
            return Err(AxisExecutionDocumentError::WrongSchema(header.schema));
        }
        if header.schema_version != AXIS_EXECUTION_VERSION {
            return Err(AxisExecutionDocumentError::UnsupportedVersion {
                found: header.schema_version,
                current: AXIS_EXECUTION_VERSION,
            });
        }
        let document: Self =
            serde_json::from_value(value).map_err(AxisExecutionDocumentError::InvalidJson)?;
        document.validate()?;
        Ok(document)
    }

    pub fn validate(&self) -> Result<(), AxisExecutionDocumentError> {
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
        let mut coordinate_index_tuples = HashSet::new();
        for (run_index, run) in self.runs.iter().enumerate() {
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
                for path in &analysis.artifacts {
                    if !valid_result_path(path) || !artifact_paths.insert(path.to_ascii_lowercase())
                    {
                        return Err(invalid("result artifact path is invalid or duplicated"));
                    }
                }
                let mut measurement_names = HashSet::new();
                for measurement in &analysis.measurements {
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
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisAnalysisKind {
    OperatingPoint,
    DcSweep,
    Transient,
    AcSmallSignal,
    Noise,
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
    pub artifacts: Vec<String>,
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
                digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
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
    let authored_prefix = match kind {
        AxisAnalysisKind::OperatingPoint => "op",
        AxisAnalysisKind::DcSweep => "dc",
        AxisAnalysisKind::Transient => "tran",
        AxisAnalysisKind::AcSmallSignal => "ac",
        AxisAnalysisKind::Noise => "noise",
    };
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document() -> AxisExecutionDocument {
        AxisExecutionDocument::new(
            AxisAnalysisKind::Transient,
            vec![CoordinateExecution {
                ordinal: 1,
                coordinate_id: "0123456789abcdef0123456789abcdef-001".to_owned(),
                coordinate_namespace: "run-0123456789abcdef0123456789abcdef-001".to_owned(),
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
                        coordinate: "run-0123456789abcdef0123456789abcdef-001".to_owned(),
                        analysis: "tran-001".to_owned(),
                    },
                    artifacts: vec!["results/run-a__tran-001.csv".to_owned()],
                    measurements: vec![MeasurementDocument {
                        name: "v(out)".to_owned(),
                        unit: "V".to_owned(),
                        value_decimal: "1e0".to_owned(),
                        sample_count: 1,
                        series_sha256: None,
                    }],
                }],
            }],
        )
        .expect("valid document")
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
            for path in &mut analysis.artifacts {
                *path = path.replace("run-a", "run-b");
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
            for path in &mut analysis.artifacts {
                *path = path.replace("run-a", "run-c");
            }
        }
        axis_drift.runs.push(second);
        axis_drift.coordinate_count = 2;
        axis_drift.execution_count = 2;
        assert!(axis_drift.validate().is_err());
    }
}

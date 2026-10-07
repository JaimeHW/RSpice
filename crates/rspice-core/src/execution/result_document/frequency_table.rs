//! Table provenance refers to the primary axes instead of duplicating coordinates.
use super::*;
use crate::engine::{FrequencyDataResult, FrequencyDataTarget};

/// One authored column and the physical coordinate axis carrying its values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrequencyTableColumn {
    pub name: String,
    pub target: FrequencyDataTarget,
    pub axis: String,
}

/// Ordered table bindings and completion evidence for one frequency study.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrequencyTableMetadata {
    pub table_name: String,
    pub columns: Vec<FrequencyTableColumn>,
    pub requested_rows: usize,
    #[serde(with = "finish_wire")]
    pub finish: Option<crate::ModelFinish>,
}

// Keep the document's strict, camelCase contract independent of the engine
// type's general-purpose serialization. A remote definition avoids copying
// model/instance strings just to format this small piece of provenance.
mod finish_wire {
    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(
        remote = "crate::ModelFinish",
        rename_all = "camelCase",
        deny_unknown_fields
    )]
    struct Finish {
        instance: String,
        model: String,
        site: u32,
        #[serde(with = "Location")]
        point: crate::ModelFinishPoint,
        diagnostic_level: u8,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(
        remote = "crate::ModelFinishPoint",
        tag = "kind",
        rename_all = "snake_case",
        deny_unknown_fields
    )]
    enum Location {
        Initialization,
        OperatingPoint,
        Transient { time: f64 },
        DcSweep { value: f64 },
        Frequency { frequency: f64 },
    }

    pub(super) fn serialize<S: serde::Serializer>(
        value: &Option<crate::ModelFinish>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Borrowed<'a>(#[serde(with = "Finish")] &'a crate::ModelFinish);
        value.as_ref().map(Borrowed).serialize(serializer)
    }

    pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<crate::ModelFinish>, D::Error> {
        #[derive(Deserialize)]
        struct Owned(#[serde(with = "Finish")] crate::ModelFinish);
        Option::<Owned>::deserialize(deserializer).map(|value| value.map(|value| value.0))
    }
}

impl FrequencyTableMetadata {
    pub(super) fn value_count(&self) -> usize {
        // Requested rows, plus the finish site's identity, diagnostic level,
        // and (where present) physical completion coordinate.
        1 + self.finish.as_ref().map_or(0, |finish| {
            2 + usize::from(matches!(
                finish.point,
                crate::ModelFinishPoint::Frequency { .. }
                    | crate::ModelFinishPoint::Transient { .. }
                    | crate::ModelFinishPoint::DcSweep { .. }
            ))
        })
    }
}

impl AnalysisResultDocument {
    /// Project AC table rows without losing parameter or device overrides.
    pub fn from_ac_table(
        analysis: AnalysisInstanceId,
        result: &FrequencyDataResult<crate::analysis::AcResult>,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        attach(Self::from_ac(analysis, &result.points)?, result)
    }

    /// Project noise table rows with their authored order and row bindings.
    pub fn from_noise_table(
        analysis: AnalysisInstanceId,
        result: &FrequencyDataResult<crate::analysis::NoiseResult>,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        attach(Self::from_noise(analysis, &result.points)?, result)
    }
}

fn malformed(detail: impl Into<String>) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "frequency table",
        detail: detail.into(),
    }
}

fn attach<T>(
    mut builder: AnalysisResultDocumentBuilder,
    result: &FrequencyDataResult<T>,
) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
    let mut columns = Vec::with_capacity(result.columns.len());
    for column in &result.columns {
        let axis = if column.target == FrequencyDataTarget::Frequency {
            let frequency = builder
                .axes
                .first()
                .ok_or_else(|| malformed("missing frequency axis"))?;
            if !matches!(&frequency.values, AxisValues::Real { values } if *values == column.values)
            {
                return Err(malformed("table frequencies disagree with solved points"));
            }
            frequency.name.clone()
        } else {
            let axis = ResultAxis::new(
                format!("data({})", column.name),
                &column.name,
                coordinate_kind(&column.target),
                column.target.unit(),
                AxisValues::Real {
                    values: column.values.clone(),
                },
            )?;
            let name = axis.name.clone();
            builder.axes.push(axis);
            name
        };
        columns.push(FrequencyTableColumn {
            name: column.name.clone(),
            target: column.target.clone(),
            axis,
        });
    }
    builder.frequency_table = Some(FrequencyTableMetadata {
        table_name: result.table_name.clone(),
        columns,
        requested_rows: result.requested_rows,
        finish: result.finish.clone(),
    });
    Ok(builder)
}

fn coordinate_kind(target: &FrequencyDataTarget) -> ResultAxisKind {
    if matches!(target, FrequencyDataTarget::Parameter(name) if name == "TEMP") {
        ResultAxisKind::Temperature
    } else {
        ResultAxisKind::SweepValue
    }
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let Some(table) = &document.frequency_table else {
        return Ok(());
    };
    if document.schema_version < 11 {
        return Err(malformed("frequency tables require document version 11"));
    }
    if !matches!(
        document.result_kind,
        AnalysisResultKind::Ac | AnalysisResultKind::Noise
    ) {
        return Err(malformed("table coordinates require an AC or noise result"));
    }
    require_name("frequency table name", &table.table_name)?;
    if document.point_count == 0
        || table.requested_rows < document.point_count
        || (table.finish.is_none() && table.requested_rows != document.point_count)
    {
        return Err(malformed(
            "accepted rows disagree with the table's completion evidence",
        ));
    }
    if let Some(finish) = &table.finish {
        require_name("finished model", &finish.model)?;
        require_name("finished instance", &finish.instance)?;
        match finish.point {
            crate::ModelFinishPoint::Frequency { frequency }
                if frequency.is_finite() && frequency >= 0.0 => {}
            crate::ModelFinishPoint::Initialization | crate::ModelFinishPoint::OperatingPoint => {}
            _ => {
                return Err(malformed(
                    "table completion has an invalid analysis coordinate",
                ));
            }
        }
    }
    let frequency = document
        .axes
        .first()
        .ok_or_else(|| malformed("missing frequency axis"))?;
    if frequency.kind != ResultAxisKind::Frequency || frequency.unit != SignalUnit::Hertz {
        return Err(malformed(
            "the primary table axis must be frequency in hertz",
        ));
    }
    let AxisValues::Real { values } = &frequency.values else {
        return Err(malformed("table frequencies must be real coordinates"));
    };
    for (index, value) in values.iter().enumerate() {
        if index.is_multiple_of(ABORT_POLL_STRIDE) {
            check_abort(abort)?;
        }
        if *value < 0.0 || (*value == 0.0 && document.result_kind == AnalysisResultKind::Noise) {
            return Err(malformed("invalid physical frequency"));
        }
    }
    let mut names = BTreeSet::new();
    let mut targets = BTreeSet::new();
    let mut axes = BTreeSet::new();
    let mut frequency_columns = 0usize;
    let axis_by_name = document
        .axes
        .iter()
        .map(|axis| (&axis.name, axis))
        .collect::<std::collections::BTreeMap<_, _>>();
    for column in &table.columns {
        check_abort(abort)?;
        require_name("table column", &column.name)?;
        let axis = axis_by_name
            .get(&column.axis)
            .copied()
            .ok_or_else(|| malformed(format!("column '{}' has no coordinate axis", column.name)))?;
        if !names.insert(column.name.to_ascii_uppercase())
            || !targets.insert(&column.target)
            || !axes.insert(&column.axis)
        {
            return Err(malformed(
                "table columns, targets and coordinate axes must be distinct",
            ));
        }
        let canonical = |name: &str| -> Result<(), ResultDocumentError> {
            require_name("table target", name)?;
            if name != name.to_ascii_uppercase() {
                return Err(malformed(
                    "table targets must use canonical uppercase names",
                ));
            }
            Ok(())
        };
        match &column.target {
            FrequencyDataTarget::Frequency => {
                frequency_columns += 1;
                if axis != frequency {
                    return Err(malformed("frequency binding points to another axis"));
                }
            }
            FrequencyDataTarget::Parameter(name) => canonical(name)?,
            FrequencyDataTarget::DeviceParameter {
                device_name,
                parameter_name,
            } => {
                canonical(device_name)?;
                canonical(parameter_name)?;
            }
        }
        if column.target != FrequencyDataTarget::Frequency
            && (axis.kind != coordinate_kind(&column.target)
                || axis.unit != column.target.unit()
                || !matches!(axis.values, AxisValues::Real { .. }))
        {
            return Err(malformed("table bindings require real sweep coordinates"));
        }
    }
    if frequency_columns != 1 || axes.len() != document.axes.len() {
        return Err(malformed(
            "table columns must describe every axis and exactly one frequency",
        ));
    }
    Ok(())
}

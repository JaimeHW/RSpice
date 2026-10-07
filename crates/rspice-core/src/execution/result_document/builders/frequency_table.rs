//! Project tables without dropping physical row coordinates or completion.
use std::collections::BTreeSet;

use super::super::frequency_table::{
    completion_value_count, malformed as error, validate_completion, validate_target,
};
use super::*;
use crate::engine::{FrequencyDataResult, FrequencyDataTarget};
use crate::{AbortSignal, NoAbort, ResourceKind, ResourceLimitError, ResourceLimits};

fn admit(count: usize, limits: &ResourceLimits) -> Result<(), ResultDocumentError> {
    ResourceLimitError::ensure(ResourceKind::ResultValues, count, limits.max_result_values)
        .map_err(ResultDocumentError::ResourceLimit)
}

// Validate borrowed evidence and admit its projected numerical storage before
// cloning coordinates or constructing signal columns. No row netlists are needed.
fn validate_source<T>(
    result: &FrequencyDataResult<T>,
    frequency: impl Fn(&T) -> f64,
    channels: impl Fn(&T) -> (&[String], &[Complex64], &[String], &[Complex64]),
    positive_frequency: bool,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<usize, ResultDocumentError> {
    super::super::check_abort(abort)?;
    super::super::require_name("frequency table name", &result.table_name)?;
    let rows = result.points.len();
    validate_completion(result.requested_rows, rows, result.finish.as_ref())?;
    let first = result
        .points
        .first()
        .ok_or_else(|| error("table has no accepted rows"))?;
    let (nodes, _, branches, _) = channels(first);
    let count = rows
        .saturating_mul(
            result
                .columns
                .len()
                .saturating_add(nodes.len().saturating_add(branches.len()).saturating_mul(2)),
        )
        .saturating_add(completion_value_count(result.finish.as_ref()));
    admit(count, limits)?;
    ResourceLimitError::ensure(
        ResourceKind::AnalysisPoints,
        rows,
        limits.max_analysis_points,
    )
    .map_err(ResultDocumentError::ResourceLimit)?;
    let mut names = BTreeSet::new();
    let mut targets = BTreeSet::new();
    let mut frequencies = None;
    for column in &result.columns {
        super::super::check_abort(abort)?;
        super::super::require_name("frequency table column", &column.name)?;
        validate_target(&column.target)?;
        if !names.insert(column.name.to_ascii_uppercase()) || !targets.insert(&column.target) {
            return Err(error("table columns have duplicate names or targets"));
        }
        if column.values.len() != rows {
            return Err(error("table coordinate length differs from accepted rows"));
        }
        for (index, &value) in column.values.iter().enumerate() {
            if index.is_multiple_of(super::super::ABORT_POLL_STRIDE) {
                super::super::check_abort(abort)?;
            }
            if !value.is_finite() {
                return Err(error("table coordinate is not finite"));
            }
        }
        if column.target == FrequencyDataTarget::Frequency {
            frequencies = Some(&column.values);
        }
    }
    let frequencies = frequencies.ok_or_else(|| error("table has no frequency column"))?;
    for (point, &expected_frequency) in result.points.iter().zip(frequencies) {
        super::super::check_abort(abort)?;
        let (point_nodes, voltages, point_branches, currents) = channels(point);
        if point_nodes != nodes
            || point_branches != branches
            || voltages.len() != nodes.len()
            || currents.len() != branches.len()
        {
            return Err(error(
                "table points do not share one complete signal schema",
            ));
        }
        let value = frequency(point);
        if value != expected_frequency {
            return Err(error("table frequency disagrees with its solved point"));
        }
        if value < 0.0 || (positive_frequency && value == 0.0) {
            return Err(error("table has an invalid analysis frequency"));
        }
    }
    Ok(count)
}

impl AnalysisResultDocument {
    /// Project a table's AC points with every physical coordinate and completion.
    pub fn from_ac_table(
        analysis: AnalysisInstanceId,
        result: &FrequencyDataResult<AcResult>,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        Self::from_ac_table_with_limits_and_abort(
            analysis,
            result,
            &ResourceLimits::default(),
            &NoAbort,
        )
    }

    /// Bounded, cancellable table projection; limits include every retained axis.
    pub fn from_ac_table_with_limits_and_abort(
        analysis: AnalysisInstanceId,
        result: &FrequencyDataResult<AcResult>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        validate_source(
            result,
            |point| point.frequency,
            |point| {
                (
                    &point.node_names,
                    &point.voltages,
                    &point.branch_names,
                    &point.currents,
                )
            },
            false,
            limits,
            abort,
        )?;
        super::super::frequency_table::attach(
            Self::ac_projection(analysis, &result.points, abort)?,
            result,
            abort,
        )
    }

    /// Project noise spectra and contributions with physical table-row identity.
    pub fn from_noise_table(
        analysis: AnalysisInstanceId,
        result: &FrequencyDataResult<NoiseResult>,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        Self::from_noise_table_with_limits_and_abort(
            analysis,
            result,
            &ResourceLimits::default(),
            &NoAbort,
        )
    }

    /// Bounded, cancellable projection including contribution-series storage.
    pub fn from_noise_table_with_limits_and_abort(
        analysis: AnalysisInstanceId,
        result: &FrequencyDataResult<NoiseResult>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        let base = validate_source(
            result,
            |point| point.frequency,
            |point| {
                (
                    &point.node_names,
                    &point.voltages,
                    &point.branch_names,
                    &point.currents,
                )
            },
            true,
            limits,
            abort,
        )?;
        let rows = result.points.len();
        let base = base.saturating_add(rows.saturating_mul(3));
        admit(base, limits)?;
        let mut contributors = BTreeSet::new();
        for point in &result.points {
            super::super::check_abort(abort)?;
            for contribution in &point.contributions {
                super::super::check_abort(abort)?;
                let key = (
                    &contribution.identity.device,
                    &contribution.identity.mechanism,
                );
                if !contributors.contains(&key) {
                    admit(
                        base.saturating_add(
                            rows.saturating_mul(3)
                                .saturating_mul(contributors.len().saturating_add(1)),
                        ),
                        limits,
                    )?;
                    contributors.insert(key);
                }
            }
        }
        super::super::frequency_table::attach(
            Self::noise_projection(analysis, &result.points, abort)?,
            result,
            abort,
        )
    }
}

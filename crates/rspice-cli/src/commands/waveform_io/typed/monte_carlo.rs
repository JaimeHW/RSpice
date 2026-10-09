//! One row per retained trial, with statistics and campaign context preserved.

use super::{ColumnData, ExportTable, PayloadProjection, conversion_error, exact_integer_sample};
use crate::{cli::CliError, commands::report_identity::encode_exact};
use rspice_core::{
    ResourceLimits,
    execution::{
        AnalysisResultDocument, SignalUnit,
        result_document::{MonteCarloPayload, ScalarValue},
    },
};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub(super) fn table(
    path: &Path,
    document: &AnalysisResultDocument,
    payload: &MonteCarloPayload,
    limits: ResourceLimits,
) -> Result<ExportTable, CliError> {
    if document.point_count() != 0 || !document.axes().is_empty() || !document.signals().is_empty()
    {
        return Err(conversion_error(
            path,
            "Monte Carlo requires a payload-based population",
        ));
    }
    let population = payload
        .successful_trial_indices
        .as_ref()
        .map(Vec::len)
        .or_else(|| {
            payload
                .statistics
                .first()
                .map(|variable| variable.samples.len())
        });
    let samples = population.unwrap_or(0);
    if payload
        .statistics
        .iter()
        .any(|variable| variable.samples.len() != samples)
    {
        return Err(conversion_error(
            path,
            "Monte Carlo variables do not cover the same retained trials",
        ));
    }
    let points = samples.max(1);
    super::enforce_table_value_limits(path, points, limits)?;
    let (scale_name, provenance, scale) = if samples == 0 {
        (
            "report",
            if population.is_some() {
                "empty"
            } else {
                "unretained"
            },
            vec![0.0],
        )
    } else if let Some(indices) = &payload.successful_trial_indices {
        let values = indices.iter().map(|&index| exact_integer_sample(index as u64)
            .ok_or_else(|| conversion_error(path, format!("Monte Carlo trial index {index} cannot be represented exactly by a numeric flat table"))))
            .collect::<Result<Vec<_>, _>>()?;
        ("trial_index", "recorded", values)
    } else {
        (
            "sample_index",
            "unknown",
            (0..samples).map(|index| index as f64).collect(),
        )
    };
    let mut columns = Vec::new();
    let mut projection = PayloadProjection {
        path,
        points,
        columns: &mut columns,
        limits,
        width: 1,
    };
    projection.mc_identity(format!("trial_coordinates({provenance})"))?;
    let scalar_names: HashSet<_> = document
        .scalars()
        .iter()
        .map(|scalar| scalar.name().to_ascii_lowercase())
        .collect();
    let mut folded = HashMap::<String, usize>::new();
    let mut identities = HashSet::new();
    for variable in &payload.statistics {
        if !identities.insert(&variable.name) {
            return Err(conversion_error(
                path,
                "Monte Carlo repeats a variable identity",
            ));
        }
        *folded
            .entry(variable.name.to_ascii_lowercase())
            .or_default() += 1;
    }
    for variable in &payload.statistics {
        let identity = encode_exact(&variable.name);
        projection.mc_identity(format!("variable({identity})"))?;
        if samples != 0 {
            let folded_name = variable.name.to_ascii_lowercase();
            let name = if folded[&folded_name] > 1
                || scalar_names.contains(&folded_name)
                || folded_name.starts_with("mc:")
                || folded_name == scale_name
            {
                format!("mc:sample({identity})")
            } else {
                variable.name.clone()
            };
            projection.push(name, SignalUnit::Unspecified, 1, || {
                ColumnData::optional_real(variable.samples.clone())
            })?;
        }
        for (metric, value) in [
            ("mean", variable.mean),
            ("standard_deviation", variable.standard_deviation),
            ("minimum", variable.minimum),
            ("maximum", variable.maximum),
        ] {
            projection.mc_optional(
                format!("mc:{metric}({identity})"),
                SignalUnit::Unspecified,
                value,
                (samples == 0).then_some(1),
            )?;
        }
        for (index, &count) in variable.histogram.iter().enumerate() {
            projection.mc_count(
                format!("mc:histogram({identity},{index})"),
                count as u64,
                SignalUnit::Dimensionless,
            )?;
        }
        for (index, &value) in variable.bin_edges.iter().enumerate() {
            projection.constant(
                format!("mc:bin_edge({identity},{index})"),
                SignalUnit::Unspecified,
                value,
            )?;
        }
        // Empty vectors are information, too; their lengths distinguish them
        // from a lost histogram and prevent exact integer counts being rounded.
        projection.mc_count(
            format!("mc:histogram_bins({identity})"),
            variable.histogram.len() as u64,
            SignalUnit::Dimensionless,
        )?;
        projection.mc_count(
            format!("mc:histogram_edges({identity})"),
            variable.bin_edges.len() as u64,
            SignalUnit::Dimensionless,
        )?;
    }
    let confidence_states: HashMap<_, _> = document
        .scalars()
        .iter()
        .filter_map(|scalar| {
            let variable = scalar.name().strip_prefix("mean_confidence_state:")?;
            let ScalarValue::Text { value } = scalar.value() else {
                return None;
            };
            Some((variable, value.as_str()))
        })
        .collect();
    for scalar in document.scalars() {
        let name = scalar.name().to_string();
        let unit = scalar.unit().cloned().unwrap_or(SignalUnit::Unspecified);
        projection.mc_identity(format!(
            "unit({},{})",
            encode_exact(&name),
            encode_exact(&scalar.unit().map(SignalUnit::symbol).unwrap_or_default())
        ))?;
        match scalar.value() {
            ScalarValue::Real { value } => {
                let confidence_variable = name
                    .strip_prefix("mean_confidence_lower:")
                    .or_else(|| name.strip_prefix("mean_confidence_upper:"));
                let reason = match confidence_variable
                    .and_then(|variable| confidence_states.get(variable).copied())
                {
                    Some("available") if value.is_some() && samples >= 2 => None,
                    Some("insufficient_samples") if value.is_none() && samples < 2 => Some(2),
                    Some("unrepresentable") if value.is_none() && samples >= 2 => Some(3),
                    Some(_) => {
                        return Err(conversion_error(
                            path,
                            "Monte Carlo confidence availability contradicts the retained population or bounds",
                        ));
                    }
                    None => None,
                };
                projection.mc_optional(name, unit, *value, reason)?;
            }
            ScalarValue::Count { value } => {
                if name == "successful_runs" && population.is_some() && *value != samples as u64 {
                    return Err(conversion_error(
                        path,
                        "Monte Carlo successful run count disagrees with retained samples",
                    ));
                }
                projection.mc_count(name, *value, unit)?;
            }
            ScalarValue::Integer { value } => {
                projection.mc_identity(format!("integer({},{value})", encode_exact(&name)))?;
                if let Some(value) = exact_integer_sample(*value) {
                    projection.constant(name, unit, value)?;
                }
            }
            ScalarValue::Boolean { value } => {
                projection.mc_identity(format!("boolean({},{value})", encode_exact(&name)))?;
                projection.constant(name, unit, f64::from(u8::from(*value)))?;
            }
            ScalarValue::Text { value } => projection.mc_identity(format!(
                "text({},{})",
                encode_exact(&name),
                encode_exact(value)
            ))?,
            ScalarValue::Complex { value } => projection.push(name, unit, 2, || {
                ColumnData::optional_complex(vec![
                    value.map(|sample| rspice_core::Complex64::new(
                        sample.real,
                        sample.imaginary
                    ));
                    points
                ])
            })?,
            ScalarValue::Unavailable { reason } => {
                projection.mc_optional(name.clone(), unit, None, None)?;
                projection.constant(
                    format!("{name}:unavailable({})", reason.tag()),
                    SignalUnit::Dimensionless,
                    1.0,
                )?;
            }
        }
    }
    let mut names = HashSet::from([scale_name.to_ascii_lowercase()]);
    for column in &columns {
        if !names.insert(column.name.to_ascii_lowercase()) {
            return Err(conversion_error(
                path,
                format!(
                    "Monte Carlo quantities project to duplicate column '{}'",
                    column.name
                ),
            ));
        }
    }
    Ok(ExportTable {
        scale_unit: Some("1".into()),
        analysis: "monte_carlo".into(),
        plot_name: "Monte Carlo Samples".into(),
        scale_name: scale_name.into(),
        scale_type: "index".into(),
        scale,
        columns,
    })
}

impl PayloadProjection<'_> {
    fn mc_identity(&mut self, identity: String) -> Result<(), CliError> {
        self.constant(
            format!("mc:identity:{identity}"),
            SignalUnit::Dimensionless,
            1.0,
        )
    }

    fn mc_count(&mut self, name: String, value: u64, unit: SignalUnit) -> Result<(), CliError> {
        self.mc_identity(format!("count({},{value})", encode_exact(&name)))?;
        if let Some(value) = exact_integer_sample(value) {
            self.constant(name, unit, value)?;
        }
        Ok(())
    }

    fn mc_optional(
        &mut self,
        name: String,
        unit: SignalUnit,
        value: Option<f64>,
        reason: Option<u8>,
    ) -> Result<(), CliError> {
        let points = self.points;
        self.push(name.clone(), unit, 1, || {
            ColumnData::optional_real(vec![value; points])
        })?;
        if let Some(reason) = reason {
            if value.is_some() {
                return Err(super::conversion_error(
                    self.path,
                    "Monte Carlo marks a defined quantity unavailable",
                ));
            }
            self.constant(
                format!("{name}:monte_carlo_status"),
                SignalUnit::Dimensionless,
                f64::from(reason),
            )?;
        }
        Ok(())
    }
}

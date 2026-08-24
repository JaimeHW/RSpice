//! Typed `.DATA` column resolution shared by table-driven analyses.
//!
//! Xyce resolves a column as an artificial analysis quantity, then as a
//! declared scalar parameter, and finally as a device parameter. Resolving
//! this once per table prevents AC, noise, CLI, and conformance frontends from
//! assigning different meanings to the same authored row.

use std::collections::BTreeSet;

use super::{Engine, SimulationError};
use crate::abort_signal::AbortSignal;
use crate::netlist::FrequencyDataPoint;
use crate::{Netlist, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
enum FrequencyDataTarget {
    Frequency(String),
    Parameter(String),
    DeviceParameter {
        device_name: String,
        parameter_name: String,
    },
}

#[derive(Debug, Clone)]
pub(in crate::engine) struct FrequencyDataOverridePlan {
    authored_columns: Vec<String>,
    targets: Vec<FrequencyDataTarget>,
}

impl FrequencyDataOverridePlan {
    pub(in crate::engine) fn resolve(
        netlist: &Netlist,
        points: &[FrequencyDataPoint],
    ) -> Result<Self, SimulationError> {
        let first = points.first().ok_or_else(|| {
            SimulationError::Circuit("frequency .DATA table has no rows".to_owned())
        })?;
        let authored_columns = first
            .overrides
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        if points.iter().any(|point| {
            point.overrides.len() != authored_columns.len()
                || point
                    .overrides
                    .iter()
                    .zip(&authored_columns)
                    .any(|((actual, _), expected)| !actual.eq_ignore_ascii_case(expected))
        }) {
            return Err(SimulationError::Circuit(
                "frequency .DATA rows do not share one stable column schema".to_owned(),
            ));
        }

        let mut targets = Vec::with_capacity(authored_columns.len());
        let mut canonical_targets = BTreeSet::new();
        for column in &authored_columns {
            let target = Self::resolve_column(netlist, column)?;
            let canonical = match &target {
                FrequencyDataTarget::Frequency(_) => "ARTIFICIAL:FREQUENCY".to_owned(),
                FrequencyDataTarget::Parameter(name) => format!("PARAM:{name}"),
                FrequencyDataTarget::DeviceParameter {
                    device_name,
                    parameter_name,
                } => format!("DEVICE:{device_name}:{parameter_name}"),
            };
            if !canonical_targets.insert(canonical.clone()) {
                return Err(SimulationError::Circuit(format!(
                    "frequency .DATA column '{column}' duplicates canonical target '{canonical}'"
                )));
            }
            targets.push(target);
        }
        Ok(Self {
            authored_columns,
            targets,
        })
    }

    fn resolve_column(
        netlist: &Netlist,
        authored_column: &str,
    ) -> Result<FrequencyDataTarget, SimulationError> {
        let column = authored_column.trim();
        if column.eq_ignore_ascii_case("FREQ") || column.eq_ignore_ascii_case("HERTZ") {
            return Ok(FrequencyDataTarget::Frequency(column.to_ascii_uppercase()));
        }
        if netlist.params.has_any_parameter_binding(column) {
            return Ok(FrequencyDataTarget::Parameter(column.to_ascii_uppercase()));
        }

        let (device_name, explicit_parameter) = match column.rsplit_once(':') {
            Some((device, parameter))
                if !device.is_empty()
                    && !parameter.is_empty()
                    && !device.contains(':')
                    && !parameter.contains(':') =>
            {
                (device, Some(parameter))
            }
            Some(_) => {
                return Err(SimulationError::Circuit(format!(
                    "frequency .DATA column '{authored_column}' has an invalid device-parameter target; expected device:param"
                )));
            }
            None => (column, None),
        };
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(device_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "frequency .DATA column '{authored_column}' does not resolve to an analysis quantity, declared parameter, or top-level device"
                ))
            })?;
        let parameter_name = Engine::canonical_device_parameter(&element.kind, explicit_parameter);
        Ok(FrequencyDataTarget::DeviceParameter {
            device_name: element.name.to_ascii_uppercase(),
            parameter_name,
        })
    }

    pub(in crate::engine) fn canonical_overrides(
        &self,
        point: &FrequencyDataPoint,
    ) -> Result<Vec<(String, Value)>, SimulationError> {
        if point.overrides.len() != self.targets.len()
            || point
                .overrides
                .iter()
                .zip(&self.authored_columns)
                .any(|((actual, _), expected)| !actual.eq_ignore_ascii_case(expected))
        {
            return Err(SimulationError::Circuit(
                "frequency .DATA row changed after its target plan was resolved".to_owned(),
            ));
        }
        Ok(point
            .overrides
            .iter()
            .zip(&self.targets)
            .filter_map(|((_, value), target)| match target {
                FrequencyDataTarget::Frequency(name) => Some((name.clone(), *value)),
                FrequencyDataTarget::Parameter(name) => Some((name.clone(), *value)),
                FrequencyDataTarget::DeviceParameter {
                    device_name,
                    parameter_name,
                } => Some((format!("{device_name}:{parameter_name}"), *value)),
            })
            .collect())
    }
}

pub(in crate::engine) fn materialize_frequency_data_row_with_abort(
    netlist: &Netlist,
    plan: &FrequencyDataOverridePlan,
    point: &FrequencyDataPoint,
    abort: &dyn AbortSignal,
) -> Result<Netlist, SimulationError> {
    let overrides = plan.canonical_overrides(point)?;
    let (row, _) = Engine::create_perturbed_netlist_multi_with_abort(netlist, &overrides, abort)?;
    Ok(row)
}

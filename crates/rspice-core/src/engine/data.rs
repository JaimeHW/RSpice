//! Shared validation, coordinate retention and execution for frequency tables.
//!
//! Resolve columns once using the Xyce analysis/parameter/device precedence.
//! Both compact results and the compatibility APIs execute the same row loop.
use std::collections::BTreeSet;

use super::{Engine, SimulationError};
use crate::abort_signal::AbortSignal;
use crate::resource::{ResourceKind, ResourceLimitError};
use crate::{Netlist, Value};

mod result;
pub use result::{FrequencyDataColumn, FrequencyDataResult, FrequencyDataTarget};

#[cfg(test)]
mod tests;

pub(super) struct FrequencyDataOptions {
    pub analysis: &'static str,
    pub positive_frequency: bool,
    pub retain_netlists: bool,
    pub default_temperature: Option<Value>,
}

struct FrequencyDataOverridePlan {
    columns: Vec<FrequencyDataColumn>,
    override_names: Vec<String>,
}

fn reserve<T>(
    values: &mut Vec<T>,
    count: usize,
    object: &'static str,
) -> Result<(), SimulationError> {
    values
        .try_reserve_exact(count)
        .map_err(|source| SimulationError::Allocation { object, source })
}

// Row solvers see the remaining allowance. Report failures against the full
// caller budget, including values retained from earlier rows.
fn cumulative_limit(
    error: SimulationError,
    resource: ResourceKind,
    retained: usize,
    limit: usize,
) -> SimulationError {
    match error {
        SimulationError::ResourceLimit(error) if error.resource == resource => ResourceLimitError {
            resource,
            requested: error.requested.saturating_add(retained),
            limit,
        }
        .into(),
        other => other,
    }
}

impl FrequencyDataOverridePlan {
    fn resolve(
        netlist: &Netlist,
        names: &[String],
        rows: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let mut columns = Vec::new();
        let mut override_names = Vec::new();
        reserve(&mut columns, names.len(), "frequency table columns")?;
        reserve(&mut override_names, names.len(), "frequency table targets")?;
        let mut canonical_targets = BTreeSet::new();
        for name in names {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let target = Self::resolve_column(netlist, name)?;
            let (canonical, override_name) = match &target {
                FrequencyDataTarget::Frequency => {
                    ("ARTIFICIAL:FREQUENCY".to_owned(), name.to_ascii_uppercase())
                }
                FrequencyDataTarget::Parameter(name) => (format!("PARAM:{name}"), name.clone()),
                FrequencyDataTarget::DeviceParameter {
                    device_name,
                    parameter_name,
                } => (
                    format!("DEVICE:{device_name}:{parameter_name}"),
                    format!("{device_name}:{parameter_name}"),
                ),
            };
            if !canonical_targets.insert(canonical.clone()) {
                return Err(SimulationError::Circuit(format!(
                    "frequency .DATA column '{name}' duplicates canonical target '{canonical}'"
                )));
            }
            let mut values = Vec::new();
            reserve(&mut values, rows, "frequency table coordinates")?;
            columns.push(FrequencyDataColumn {
                name: name.clone(),
                target,
                values,
            });
            override_names.push(override_name);
        }
        Ok(Self {
            columns,
            override_names,
        })
    }

    fn resolve_column(
        netlist: &Netlist,
        authored_column: &str,
    ) -> Result<FrequencyDataTarget, SimulationError> {
        let column = authored_column.trim();
        if column.eq_ignore_ascii_case("FREQ") || column.eq_ignore_ascii_case("HERTZ") {
            return Ok(FrequencyDataTarget::Frequency);
        }
        // TEMP is a physical row coordinate even when the parser is using its
        // implicit default and has no authored parameter/option binding for it.
        if column.eq_ignore_ascii_case("TEMP") {
            return Ok(FrequencyDataTarget::Parameter("TEMP".into()));
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
}

impl Engine {
    /// Validate all input rows before solving, then charge retained coordinates
    /// and earlier results against every subsequent row's solver budget.
    pub(super) fn run_frequency_data<T>(
        &self,
        netlist: &Netlist,
        table_name: &str,
        options: FrequencyDataOptions,
        abort: &dyn AbortSignal,
        solve_row: impl Fn(
            &Engine,
            &Netlist,
            Value,
            &dyn AbortSignal,
        ) -> Result<Vec<T>, SimulationError>,
        value_count: fn(&T) -> usize,
    ) -> Result<(Vec<Netlist>, FrequencyDataResult<T>), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.ensure_valid_configuration()?;
        let table = netlist
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case(table_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "{} DATA references unknown .DATA table '{table_name}'",
                    options.analysis
                ))
            })?;
        self.ensure_analysis_points(table.rows.len())?;
        self.ensure_batch_runs(table.rows.len())?;
        // Bound metadata before cloning any rows, column names or result storage.
        let coordinate_values = table.rows.len().saturating_mul(table.params.len());
        self.ensure_result_values(coordinate_values)?;
        let table_error =
            |error| SimulationError::Circuit(format!("{} DATA {error}", options.analysis));
        let frequency_column = table.frequency_column().map_err(table_error)?;
        for (index, row) in table.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let frequency = table
                .validate_frequency_row(index, row, frequency_column)
                .map_err(table_error)?;
            if options.positive_frequency && frequency <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "{} DATA frequencies must be strictly positive, got {frequency}",
                    options.analysis
                )));
            }
        }
        let plan =
            FrequencyDataOverridePlan::resolve(netlist, &table.params, table.rows.len(), abort)?;
        let temperature_column = plan.columns.iter().position(|column| {
            matches!(&column.target, FrequencyDataTarget::Parameter(name) if name == "TEMP")
        });
        let run_scope = crate::abort_signal::ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        Self::ensure_model_run_active(abort)?;
        let mut result = FrequencyDataResult {
            table_name: table.name.clone(),
            columns: plan.columns,
            points: Vec::new(),
            requested_rows: table.rows.len(),
            finish: None,
        };
        let mut row_netlists = Vec::new();
        reserve(
            &mut result.points,
            table.rows.len(),
            "frequency table results",
        )?;
        if options.retain_netlists {
            reserve(
                &mut row_netlists,
                table.rows.len(),
                "frequency table row netlists",
            )?;
        }
        let mut retained_values = coordinate_values;
        let mut retained_source_bytes = 0usize;
        for (row_index, values) in table.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut overrides = Vec::new();
            reserve(
                &mut overrides,
                values.len(),
                "frequency table row overrides",
            )?;
            overrides.extend(
                plan.override_names
                    .iter()
                    .cloned()
                    .zip(values.iter().copied()),
            );
            let mut limits = self.config().resource_limits;
            limits.max_expanded_source_bytes = limits
                .max_expanded_source_bytes
                .saturating_sub(retained_source_bytes);
            let (mut row, _) = Self::create_perturbed_netlist_multi_with_limits_and_abort(
                netlist, &overrides, limits, abort,
            )
            .map_err(|error| {
                cumulative_limit(
                    error,
                    ResourceKind::ExpandedSourceBytes,
                    retained_source_bytes,
                    self.config().resource_limits.max_expanded_source_bytes,
                )
            })?;
            if options.retain_netlists {
                retained_source_bytes =
                    retained_source_bytes.saturating_add(row.retained_source_bytes());
                ResourceLimitError::ensure(
                    ResourceKind::ExpandedSourceBytes,
                    retained_source_bytes,
                    self.config().resource_limits.max_expanded_source_bytes,
                )?;
            }
            // A physical study coordinate also overrides an executed control
            // option. Keep the compatibility netlist and solver in agreement.
            let temperature = temperature_column.map(|index| values[index]);
            if let Some(temperature) = temperature {
                row.options.temp = Some(temperature);
            }
            let mut config =
                self.frequency_row_config(&row, temperature, options.default_temperature);
            config.resource_limits.max_result_values -= retained_values;
            let bounded = self.try_resolved_with_config(config)?;
            let mut points = match solve_row(&bounded, &row, values[frequency_column], abort) {
                Err(SimulationError::ModelFinished(finish)) if !result.points.is_empty() => {
                    result.finish = Some(*finish);
                    break;
                }
                outcome => outcome.map_err(|error| {
                    cumulative_limit(
                        error,
                        ResourceKind::ResultValues,
                        retained_values,
                        self.config().resource_limits.max_result_values,
                    )
                })?,
            };
            if points.len() != 1 {
                return Err(SimulationError::Circuit(format!(
                    "{} DATA table '{}' row {} produced {} results, expected one",
                    options.analysis,
                    table.name,
                    row_index + 1,
                    points.len()
                )));
            }
            let point = points.remove(0);
            retained_values = retained_values.saturating_add(value_count(&point));
            self.ensure_result_values(retained_values)?;
            for (column, &value) in result.columns.iter_mut().zip(values) {
                column.values.push(value);
            }
            result.points.push(point);
            if options.retain_netlists {
                row_netlists.push(row);
            }
            if let Some(finish) = abort.model_control().and_then(|control| control.finish()) {
                result.finish = Some(finish);
                break;
            }
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok((row_netlists, result))
    }
}

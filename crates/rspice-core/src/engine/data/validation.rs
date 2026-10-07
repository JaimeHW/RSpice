//! Borrowed table admission shared by execution and frontend preflight.
use super::*;
use crate::ResourceLimits;
use crate::netlist::{AnalysisCommand, DataTable};

pub(super) struct Input<'a> {
    pub(super) table: &'a DataTable,
    pub(super) frequency_column: usize,
    pub(super) coordinate_values: usize,
}

impl Engine {
    /// Check an AC/noise DATA request without replaying parameters or solving.
    ///
    /// Validates table existence, shape, finite coordinates, the analysis's
    /// frequency domain and point/batch/result limits using borrowed rows.
    /// Device bindings and row-dependent model constraints are resolved during
    /// execution, after any preceding control mutations have taken effect.
    pub fn validate_frequency_data_request_with_abort(
        netlist: &Netlist,
        request: &AnalysisCommand,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let (table_name, analysis, positive_frequency) = match request {
            AnalysisCommand::AcData { table_name } => (table_name, ".AC", false),
            AnalysisCommand::NoiseData { table_name, .. } => (table_name, ".NOISE", true),
            _ => {
                return Err(SimulationError::Circuit(
                    "expected an AC or noise DATA request".into(),
                ));
            }
        };
        input(
            netlist,
            table_name,
            analysis,
            positive_frequency,
            limits,
            abort,
        )
        .map(|_| ())
    }
}

pub(super) fn input<'a>(
    netlist: &'a Netlist,
    table_name: &str,
    analysis: &'static str,
    positive_frequency: bool,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Input<'a>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let table = netlist
        .data_tables
        .iter()
        .find(|table| table.name.eq_ignore_ascii_case(table_name))
        .ok_or_else(|| {
            SimulationError::Circuit(format!(
                "{analysis} DATA references unknown .DATA table '{table_name}'"
            ))
        })?;
    ResourceLimitError::ensure(
        ResourceKind::AnalysisPoints,
        table.rows.len(),
        limits.max_analysis_points,
    )?;
    ResourceLimitError::ensure(
        ResourceKind::BatchRuns,
        table.rows.len(),
        limits.max_batch_runs,
    )?;
    let coordinate_values = table.rows.len().saturating_mul(table.params.len());
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        coordinate_values,
        limits.max_result_values,
    )?;
    let table_error = |error| SimulationError::Circuit(format!("{analysis} DATA {error}"));
    let frequency_column = table.frequency_column().map_err(table_error)?;
    for (index, row) in table.rows.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let frequency = table
            .validate_frequency_row(index, row, frequency_column)
            .map_err(table_error)?;
        if positive_frequency && frequency <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "{analysis} DATA frequencies must be strictly positive, got {frequency}"
            )));
        }
    }
    Ok(Input {
        table,
        frequency_column,
        coordinate_values,
    })
}

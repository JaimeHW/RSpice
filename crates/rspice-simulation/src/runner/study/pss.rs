//! Configured PSS execution on each materialized study circuit.
use super::*;
use crate::results::SimulationResult;
use rspice_simulation_contract::config::OpConfig;

pub(super) fn run(
    request_config: &StudyPssConfig,
    engine: &rspice_core::Engine,
    circuit: &rspice_core::Netlist,
    numeric_options: &str,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    run_with_circuit(request_config, engine, circuit, numeric_options, abort)
        .map(|(_, result)| result)
}

pub(super) fn run_with_circuit(
    request_config: &StudyPssConfig,
    engine: &rspice_core::Engine,
    circuit: &rspice_core::Netlist,
    numeric_options: &str,
    abort: &dyn AbortSignal,
) -> Result<(rspice_core::Netlist, SimulationResult), SimulationError> {
    let (physical, seed) =
        super::pss::run_operating_point(&request_config.operating_point, engine, circuit, abort)?;
    let temperature_kelvin = request_config.operating_point.config.temperature_celsius + 273.15;
    let pss_circuit = circuit_with_options(&physical, numeric_options, abort)?;
    let result = super::super::spec::run_pss_study_on_materialized(
        request_config.request.clone(),
        &pss_circuit,
        &seed,
        temperature_kelvin,
        abort,
    )?;
    Ok((pss_circuit, result))
}

pub(super) fn circuit_with_options(
    circuit: &rspice_core::Netlist,
    commands: &str,
    abort: &dyn AbortSignal,
) -> Result<rspice_core::Netlist, SimulationError> {
    let mut circuit = circuit.clone();
    for line in commands
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        super::super::spec::ensure_not_aborted(abort)?;
        let (keyword, arguments) = line.split_once(char::is_whitespace).ok_or_else(|| {
            SimulationError::InvalidConfig("Malformed study numerical options".into())
        })?;
        if !keyword.eq_ignore_ascii_case(".options") {
            return Err(SimulationError::InvalidConfig(
                "Study numerical overrides must be .OPTIONS cards".into(),
            ));
        }
        circuit.options = rspice_core::netlist::simulation_options_with_overrides(
            arguments,
            &circuit.params,
            &circuit.options,
            rspice_core::ResourceLimits::default().max_analysis_points,
            abort,
        )
        .map_err(|error| {
            if error.is_aborted() {
                SimulationError::Aborted
            } else {
                SimulationError::InvalidConfig(error.to_string())
            }
        })?;
    }
    Ok(circuit)
}

pub(super) fn physical_circuit(
    request_config: &StudyOperatingPoint,
    circuit: &rspice_core::Netlist,
    abort: &dyn AbortSignal,
) -> Result<(rspice_core::Netlist, OpConfig), SimulationError> {
    super::super::spec::ensure_not_aborted(abort)?;
    let mut physical = circuit.clone();
    let mut op = request_config.config.clone();
    if let (Some(supply), Some(nominal)) = (
        op.run_point.supply_voltage,
        op.run_point.nominal_supply_voltage,
    ) {
        super::super::spec::run_abort_aware_service(abort, || {
            crate::netlist_preparation::apply_voltage_corner(
                &mut physical,
                supply,
                nominal,
                &op.run_point.supply_source_names,
                abort,
            )
        })?;
        op.run_point.supply_voltage = None;
        op.run_point.nominal_supply_voltage = None;
    }
    physical.options.temp = Some(op.temperature_celsius);
    Ok((physical, op))
}

pub(super) fn run_operating_point(
    request_config: &StudyOperatingPoint,
    engine: &rspice_core::Engine,
    circuit: &rspice_core::Netlist,
    abort: &dyn AbortSignal,
) -> Result<
    (
        rspice_core::Netlist,
        rspice_core::engine::PeriodicDcOperatingPointSeed,
    ),
    SimulationError,
> {
    let (physical, op) = physical_circuit(request_config, circuit, abort)?;
    let op_circuit = circuit_with_options(&physical, &request_config.numeric_options, abort)?;
    let result = EngineBridge::run_materialized_with_abort(
        engine,
        &AnalysisConfig::DcOp(op),
        &op_circuit,
        abort,
    )?;
    let SimulationResult::DcOp(point) = result else {
        return Err(SimulationError::SolverError(
            "Periodic study OP producer returned no DC solution".into(),
        ));
    };
    let seed = rspice_core::engine::PeriodicDcOperatingPointSeed::try_new(
        point.mna_node_names,
        point.mna_branch_names,
        point.mna_solution,
    )
    .map_err(|error| SimulationError::SolverError(error.to_string()))?;
    Ok((physical, seed))
}

#[cfg(test)]
mod tests;

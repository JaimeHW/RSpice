//! Fresh configured OP and shooting solves on a single study circuit.
use super::*;
use crate::simulation::dialog::OpConfig;
use crate::simulation::multi_run::{AnalysisSpec, PssMethod};
use crate::simulation::results::SimulationResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyOperatingPoint {
    pub instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    pub config: OpConfig,
    pub numeric_options: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyPssConfig {
    pub request: AnalysisSpec,
    pub operating_point: StudyOperatingPoint,
}

impl StudyPssConfig {
    pub(super) fn validate(&self) -> Result<(), String> {
        if !matches!(
            self.request,
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                ..
            }
        ) {
            return Err("A PSS study requires a shooting PSS specification".into());
        }
        self.request.validate()?;
        self.operating_point.config.validate()
    }

    pub(super) fn validate_measurements(&self, measurements: &[String]) -> Result<(), String> {
        self.validate()?;
        let AnalysisSpec::Pss { num_harmonics, .. } = self.request else {
            unreachable!()
        };
        for request in measurements {
            let (mode, key) = request.split_once(':').unwrap_or(("meas", request));
            if mode.eq_ignore_ascii_case("last") {
                continue;
            }
            if mode.eq_ignore_ascii_case("scalar")
                && ["pss.frequency", "pss.period", "pss.iterations"]
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(key))
            {
                continue;
            }
            if mode.eq_ignore_ascii_case("bin") {
                let (index, _, signal) = crate::simulation::results::parse_study_bin(key)?;
                if index <= num_harmonics && signal.is_some() {
                    continue;
                }
                return Err(
                    "PSS bin observations need an explicit signal and a retained harmonic index"
                        .into(),
                );
            }
            return Err("PSS studies require last:signal, bin:index:quantity:signal or scalar:pss.frequency/period/iterations".into());
        }
        Ok(())
    }

    pub(super) fn run(
        &self,
        engine: &rspice_core::Engine,
        circuit: &rspice_core::Netlist,
        numeric_options: &str,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_with_circuit(engine, circuit, numeric_options, abort)
            .map(|(_, result)| result)
    }

    pub(super) fn run_with_circuit(
        &self,
        engine: &rspice_core::Engine,
        circuit: &rspice_core::Netlist,
        numeric_options: &str,
        abort: &dyn AbortSignal,
    ) -> Result<(rspice_core::Netlist, SimulationResult), SimulationError> {
        super::super::spec::ensure_not_aborted(abort)?;
        let mut physical = circuit.clone();
        let mut op = self.operating_point.config.clone();
        if let (Some(supply), Some(nominal)) = (
            op.run_point.supply_voltage,
            op.run_point.nominal_supply_voltage,
        ) {
            super::super::spec::run_abort_aware_service(abort, || {
                services::apply_voltage_corner(
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
        let temperature_kelvin = op.temperature_celsius + 273.15;
        let op_circuit =
            circuit_with_options(&physical, &self.operating_point.numeric_options, abort)?;
        let result = EngineBridge::run_materialized_with_abort(
            engine,
            &AnalysisConfig::DcOp(op),
            &op_circuit,
            abort,
        )?;
        let SimulationResult::DcOp(point) = result else {
            return Err(SimulationError::SolverError(
                "PSS study OP producer returned no DC solution".into(),
            ));
        };
        let seed = rspice_core::engine::PssDcOperatingPointSeed::try_new(
            point.mna_node_names,
            point.mna_branch_names,
            point.mna_solution,
        )
        .map_err(|error| SimulationError::SolverError(error.to_string()))?;
        let pss_circuit = circuit_with_options(&physical, numeric_options, abort)?;
        let result = super::super::spec::run_pss_study_on_materialized(
            self.request.clone(),
            &pss_circuit,
            &seed,
            temperature_kelvin,
            abort,
        )?;
        Ok((pss_circuit, result))
    }
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

#[cfg(test)]
mod tests;

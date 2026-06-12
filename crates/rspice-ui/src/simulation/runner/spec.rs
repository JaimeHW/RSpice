use std::path::Path;

use super::super::engine_bridge::EngineBridge;
use super::super::multi_run::AnalysisSpec;
use super::super::results::SimulationResult;
use super::{SimulationError, SpecExecutionOptions};

mod config;
mod device;
mod frequency;
mod periodic;
mod sweeps;

pub(super) fn run_spec_request(
    bridge: &EngineBridge,
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    abort_flag: &dyn rspice_core::abort_signal::AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    if abort_flag.is_aborted() {
        return Err(SimulationError::Aborted);
    }

    if let Some(config) = config::analysis_config_from_spec(&spec) {
        return bridge.run_with_abort_and_source_path(&config, netlist, source_path, abort_flag);
    }

    match spec {
        AnalysisSpec::MonteCarlo | AnalysisSpec::Parametric | AnalysisSpec::Corner => {
            sweeps::run_sweep_spec(spec, options, netlist)
        }
        AnalysisSpec::Reliability { .. }
        | AnalysisSpec::Optimization { .. }
        | AnalysisSpec::Soa { .. } => device::run_device_spec(spec, netlist),
        AnalysisSpec::Pss { .. }
        | AnalysisSpec::HarmonicBalance { .. }
        | AnalysisSpec::Envelope { .. }
        | AnalysisSpec::Fourier { .. }
        | AnalysisSpec::Disto { .. } => periodic::run_periodic_spec(spec, netlist),
        AnalysisSpec::SParameter { .. }
        | AnalysisSpec::Tf
        | AnalysisSpec::Pac
        | AnalysisSpec::Pxf
        | AnalysisSpec::Pnoise
        | AnalysisSpec::Stb { .. }
        | AnalysisSpec::Pstb => frequency::run_frequency_spec(spec, options, netlist),
        unsupported => Err(SimulationError::InvalidConfig(format!(
            "{:?} is not supported by SimulationRunner::start_spec",
            unsupported.run_type()
        ))),
    }
}

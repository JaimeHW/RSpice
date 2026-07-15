use std::path::{Path, PathBuf};

use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    pub(super) fn parse_netlist_with_abort_and_source_path(
        &self,
        netlist_str: &str,
        source_path: Option<&Path>,
        abort: &dyn AbortSignal,
    ) -> Result<rspice_core::Netlist, SimulationError> {
        ensure_not_aborted(abort)?;
        let parse_source = Self::netlist_parse_source(source_path);
        let parsed =
            rspice_core::Netlist::parse_with_path_and_abort(netlist_str, &parse_source, abort)
                .map_err(|error| match error {
                    rspice_core::netlist::ParseWithAbortError::Aborted => SimulationError::Aborted,
                    rspice_core::netlist::ParseWithAbortError::Parse(error) => {
                        SimulationError::ParseError(error.to_string())
                    }
                });
        ensure_not_aborted(abort)?;
        parsed
    }

    fn netlist_parse_source(source_path: Option<&Path>) -> PathBuf {
        const GENERATED_NETLIST_NAME: &str = "__rspice_ui_generated__.cir";

        match source_path {
            Some(path) if path.is_dir() => path.join(GENERATED_NETLIST_NAME),
            Some(path) => path.to_path_buf(),
            None => std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(GENERATED_NETLIST_NAME),
        }
    }

    /// Build an engine instance with netlist `.OPTIONS` layered on top of
    /// bridge base configuration.
    pub(super) fn engine_for_netlist(&self, netlist: &rspice_core::Netlist) -> rspice_core::Engine {
        let resolved = rspice_core::resolve_simulation_config(
            self.engine.config(),
            Some(&netlist.options),
            &rspice_core::SimulationConfigOverrides::default(),
        );
        rspice_core::Engine::new(resolved)
    }
}

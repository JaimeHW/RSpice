use std::path::{Path, PathBuf};

use super::EngineBridge;
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    pub(super) fn parse_netlist_with_source_path(
        &self,
        netlist_str: &str,
        source_path: Option<&Path>,
    ) -> Result<rspice_core::Netlist, SimulationError> {
        let parse_source = Self::netlist_parse_source(source_path);
        rspice_core::Netlist::parse_with_path(netlist_str, &parse_source)
            .map_err(|e| SimulationError::ParseError(e.to_string()))
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

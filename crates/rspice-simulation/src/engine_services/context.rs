//! Explicit host policy shared by advanced analysis services.

use std::path::Path;

use rspice_core::{Netlist, ResourceLimits, SimulationConfig, abort_signal::AbortSignal};

use super::ServiceRunResult;

/// Source resolution, resource ceilings, and cancellation for one service call.
/// Nested work must inherit this context instead of rebuilding default limits.
#[derive(Clone, Copy)]
pub struct ServiceContext<'a> {
    pub source_path: Option<&'a Path>,
    pub limits: ResourceLimits,
    pub abort: &'a dyn AbortSignal,
}

impl<'a> ServiceContext<'a> {
    /// Default-policy convenience for direct service tests.
    #[cfg(test)]
    pub fn with_defaults(source_path: Option<&'a Path>, abort: &'a dyn AbortSignal) -> Self {
        Self {
            source_path,
            limits: ResourceLimits::default(),
            abort,
        }
    }

    pub(crate) fn parse(self, source: &str) -> ServiceRunResult<Netlist> {
        crate::netlist_preparation::parse_runner_netlist_with_resource_limits_and_abort(
            source,
            self.source_path,
            self.limits,
            self.abort,
        )
    }

    pub(crate) fn engine_config(self, netlist: &Netlist) -> SimulationConfig {
        let mut config = super::build_engine_config(netlist, None);
        config.resource_limits = self.limits;
        config
    }
}

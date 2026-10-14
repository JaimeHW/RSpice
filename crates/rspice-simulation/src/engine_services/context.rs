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

    /// Parse `source` for a run around `bias`.
    ///
    /// Around a transient state the deck parses as its transient does, with
    /// the parser's own statistical mode rather than the services' nominal
    /// one: a checkpoint's restart identity records the mode, so only then is
    /// a state the deck's transient wrote a state of this deck, and only then
    /// does the card see the circuit whose trajectory it linearizes.
    pub(crate) fn parse_around(
        self,
        source: &str,
        bias: super::Bias<'_>,
    ) -> ServiceRunResult<Netlist> {
        match bias {
            super::Bias::OperatingPoint => self.parse(source),
            super::Bias::TransientState { .. } => {
                crate::netlist_preparation::parse_runner_netlist_with_options_and_abort(
                    source,
                    self.source_path,
                    rspice_core::netlist::NetlistParseOptions {
                        resource_limits: self.limits,
                        ..Default::default()
                    },
                    self.abort,
                )
            }
        }
    }

    pub(crate) fn engine_config(self, netlist: &Netlist) -> SimulationConfig {
        let mut config = super::build_engine_config(netlist, None);
        config.resource_limits = self.limits;
        config
    }

    /// Deck options are already resolved. Periodic run settings own the
    /// tolerance and must not be overwritten by a second option merge.
    pub(crate) fn periodic_engine(
        self,
        netlist: &Netlist,
        tolerance: f64,
        diagnostic: &str,
    ) -> ServiceRunResult<rspice_core::engine::Engine> {
        let mut config = self.engine_config(netlist);
        config.tolerance = tolerance;
        rspice_core::engine::Engine::try_new_with_resolved_config(config).map_err(|error| {
            super::ServiceRunError::from_core(diagnostic, rspice_core::SimulationError::from(error))
        })
    }
}

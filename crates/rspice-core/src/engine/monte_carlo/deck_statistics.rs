//! Reproducible replay of authored statistical expressions and model cards.

use super::*;
use crate::netlist::{NetlistParseOptions, ParameterOverride, StatisticalParamMode};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MonteCarloVariationSource {
    /// Vary eligible nominal parameters, or use native Spectre declarations.
    #[default]
    ParameterTolerance,
    /// Redraw authored statistical expressions, including model cards, and
    /// evaluate native Spectre declarations at the trial's coordinate.
    DeckStatistics,
}

/// The independent expression-sampler seed for an original zero-based trial.
/// This preserves the Studio deck-statistics stream predating the shared runner.
pub const fn monte_carlo_deck_trial_seed(base: u64, trial: usize) -> u64 {
    let mut z = base.wrapping_add((trial as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Engine {
    pub(super) fn materialize_monte_carlo_deck_statistics(
        &self,
        netlist: &Netlist,
        seed: u64,
        abort: &dyn AbortSignal,
    ) -> Result<Netlist, SimulationError> {
        let source = netlist.source_text.as_deref().ok_or_else(|| {
            SimulationError::Circuit(
                "Deck-statistics Monte Carlo requires the circuit's authored source".into(),
            )
        })?;
        let overrides = netlist
            .ast_overlay
            .parameters
            .iter()
            .map(|(name, value)| ParameterOverride {
                name: name.clone(),
                value: *value,
                global: !netlist.params.has_parameter_binding(name)
                    && netlist.params.has_any_parameter_binding(name),
                direction: false,
            })
            .collect::<Vec<_>>();
        let parse_options = NetlistParseOptions {
            retain_control_script: netlist.control_script.is_some(),
            statistical_mode: StatisticalParamMode::Sample,
            statistical_seed: Some(seed),
            expression_dialect: netlist.params.expression_dialect(),
            parameter_redefinition_policy: netlist.params.parameter_redefinition_policy(),
            parameter_redefinition_diagnostic_policy: netlist
                .params
                .parameter_redefinition_diagnostic_policy(),
            resource_limits: self.config.resource_limits,
        };
        let mut materialized = netlist
            .replay_root_source_with_parameter_overrides_and_abort(
                source,
                parse_options,
                &overrides,
                abort,
            )
            .map_err(|error| match error {
                crate::netlist::ParseWithAbortError::Aborted => SimulationError::from_abort(abort),
                crate::netlist::ParseWithAbortError::Parse(
                    crate::netlist::ParseError::ResourceLimit(error),
                ) => SimulationError::ResourceLimit(error),
                crate::netlist::ParseWithAbortError::Parse(error) => SimulationError::Netlist(
                    format!("Failed to materialize Monte Carlo statistical trial: {error}",),
                ),
            })?;
        Self::reapply_ast_overlay_with_abort(&mut materialized, &netlist.ast_overlay, abort)?;
        Ok(materialized)
    }
}

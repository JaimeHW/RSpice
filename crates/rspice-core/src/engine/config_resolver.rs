//! Simulation configuration resolution with explicit precedence.
//!
//! This module centralizes config layering for frontends:
//! 1. `base` configuration (frontend defaults / profile)
//! 2. netlist `.OPTIONS`
//! 3. explicit runtime overrides (CLI/UI/Python/etc.)

use super::{ConvergenceConfig, SimulationConfig};
use crate::Value;
use crate::analysis::IntegrationMethod;
use crate::netlist::SimulationOptions as NetlistSimulationOptions;

/// Convergence preset selection used by frontends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergencePreset {
    Fast,
    Default,
    Robust,
}

impl ConvergencePreset {
    /// Parse convergence mode text (case-insensitive).
    pub fn from_mode_name(mode: &str) -> Option<Self> {
        if mode.eq_ignore_ascii_case("fast") {
            Some(Self::Fast)
        } else if mode.eq_ignore_ascii_case("robust") {
            Some(Self::Robust)
        } else if mode.eq_ignore_ascii_case("default") {
            Some(Self::Default)
        } else {
            None
        }
    }

    /// Build the preset convergence config.
    pub fn to_convergence_config(self) -> ConvergenceConfig {
        match self {
            Self::Fast => ConvergenceConfig::fast(),
            Self::Default => ConvergenceConfig::default(),
            Self::Robust => ConvergenceConfig::robust(),
        }
    }
}

/// Optional explicit overrides to apply after netlist options.
#[derive(Debug, Clone, Default)]
pub struct SimulationConfigOverrides {
    pub temperature_kelvin: Option<Value>,
    pub max_iterations: Option<usize>,
    pub min_timestep: Option<Value>,
    pub max_timestep: Option<Value>,
    pub integration_method: Option<IntegrationMethod>,
    pub transient_trtol: Option<Value>,
    pub convergence_preset: Option<ConvergencePreset>,
    /// Legacy RELTOL knob: updates both `SimulationConfig::tolerance` and
    /// `ConvergenceConfig::voltage_reltol`.
    pub reltol: Option<Value>,
    /// Legacy ABSTOL knob: updates both voltage and current absolute tolerances.
    pub abstol: Option<Value>,
    pub voltage_abstol: Option<Value>,
    pub current_abstol: Option<Value>,
    pub charge_abstol: Option<Value>,
    pub residual_reltol: Option<Value>,
    pub gmin_initial: Option<Value>,
}

/// Resolve the final simulation config from layered sources.
///
/// Precedence: `base` < `netlist_options` < `overrides`.
pub fn resolve_simulation_config(
    base: &SimulationConfig,
    netlist_options: Option<&NetlistSimulationOptions>,
    overrides: &SimulationConfigOverrides,
) -> SimulationConfig {
    let mut resolved = base.clone();

    let mut temperature = base.temperature;
    let mut max_iterations = base.max_iterations;
    let mut transient_max_iterations = base.transient_max_iterations;
    let mut min_timestep = base.min_timestep;
    let mut max_timestep = base.max_timestep;
    let mut integration_method = base.integration_method;
    let mut transient_trtol = base.transient_trtol;
    let mut tolerance = base.tolerance;
    let mut voltage_reltol = base.convergence_config.voltage_reltol;
    let mut voltage_abstol = base.convergence_config.voltage_abstol;
    let mut current_abstol = base.convergence_config.current_abstol;
    let mut charge_abstol = base.convergence_config.charge_abstol;
    let mut residual_reltol = base.convergence_config.residual_reltol;
    let mut gmin_initial = base.convergence_config.gmin_initial;

    if let Some(opts) = netlist_options {
        if let Some(temp_celsius) = opts.temp {
            temperature = temp_celsius + 273.15;
        }
        if let Some(itl1) = opts.itl1 {
            max_iterations = itl1;
        }
        if let Some(itl4) = opts.itl4 {
            transient_max_iterations = itl4;
        }
        if let Some(method) = opts
            .method
            .as_deref()
            .and_then(parse_integration_method_option)
        {
            integration_method = method;
        }
        if let Some(trtol) = opts.trtol {
            transient_trtol = trtol;
        }
        if let Some(reltol) = opts.reltol {
            tolerance = reltol;
            voltage_reltol = reltol;
            if opts.residual_reltol.is_none() {
                residual_reltol = reltol;
            }
        }
        if let Some(vntol) = opts.vntol {
            voltage_abstol = vntol;
        }
        if let Some(iabstol) = opts.iabstol.or(opts.abstol) {
            current_abstol = iabstol;
        }
        if let Some(chgtol) = opts.chgtol {
            charge_abstol = chgtol;
        }
        if let Some(residual) = opts.residual_reltol {
            residual_reltol = residual;
        }
        if let Some(gmin) = opts.gmin {
            gmin_initial = gmin;
        }
    }

    if let Some(temp_k) = overrides.temperature_kelvin {
        temperature = temp_k;
    }
    if let Some(iters) = overrides.max_iterations {
        max_iterations = iters;
        transient_max_iterations = iters;
    }
    if let Some(min_step) = overrides.min_timestep {
        min_timestep = min_step;
    }
    if let Some(max_step) = overrides.max_timestep {
        max_timestep = max_step;
    }
    if let Some(method) = overrides.integration_method {
        integration_method = method;
    }
    if let Some(trtol) = overrides.transient_trtol {
        transient_trtol = trtol;
    }
    if let Some(reltol) = overrides.reltol {
        tolerance = reltol;
        voltage_reltol = reltol;
    }
    if let Some(abstol) = overrides.abstol {
        voltage_abstol = abstol;
        current_abstol = abstol;
    }
    if let Some(vabstol) = overrides.voltage_abstol {
        voltage_abstol = vabstol;
    }
    if let Some(iabstol) = overrides.current_abstol {
        current_abstol = iabstol;
    }
    if let Some(chgtol) = overrides.charge_abstol {
        charge_abstol = chgtol;
    }
    if let Some(residual) = overrides.residual_reltol {
        residual_reltol = residual;
    }
    if let Some(gmin) = overrides.gmin_initial {
        gmin_initial = gmin;
    }

    if let Some(preset) = overrides.convergence_preset {
        resolved.convergence_config = preset.to_convergence_config();
    }

    resolved.temperature = temperature;
    resolved.max_iterations = max_iterations;
    resolved.transient_max_iterations = transient_max_iterations;
    resolved.min_timestep = min_timestep;
    resolved.max_timestep = max_timestep;
    resolved.integration_method = integration_method;
    resolved.transient_trtol = transient_trtol;
    resolved.tolerance = tolerance;
    resolved.convergence_config.voltage_reltol = voltage_reltol;
    resolved.convergence_config.voltage_abstol = voltage_abstol;
    resolved.convergence_config.current_abstol = current_abstol;
    resolved.convergence_config.charge_abstol = charge_abstol;
    resolved.convergence_config.residual_reltol = residual_reltol;
    resolved.convergence_config.gmin_initial = gmin_initial;
    if resolved.convergence_config.gmin_target > gmin_initial {
        resolved.convergence_config.gmin_target = gmin_initial;
    }

    resolved
}

fn parse_integration_method_option(method: &str) -> Option<IntegrationMethod> {
    if method.eq_ignore_ascii_case("TRAP")
        || method.eq_ignore_ascii_case("TRAPEZOIDAL")
        || method.eq_ignore_ascii_case("TRAPEZOID")
    {
        Some(IntegrationMethod::Trapezoidal)
    } else if method.eq_ignore_ascii_case("EULER")
        || method.eq_ignore_ascii_case("BE")
        || method.eq_ignore_ascii_case("BACKWARDEULER")
    {
        Some(IntegrationMethod::BackwardEuler)
    } else if method.eq_ignore_ascii_case("GEAR")
        || method.eq_ignore_ascii_case("BDF")
        || method.eq_ignore_ascii_case("GEAR2")
    {
        Some(IntegrationMethod::Gear2)
    } else if method.eq_ignore_ascii_case("TRAPGEAR") || method.eq_ignore_ascii_case("AUTO") {
        Some(IntegrationMethod::TrapGear)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_abstol_updates_current_tolerance_only() {
        let mut base = SimulationConfig::default();
        base.convergence_config.voltage_abstol = 1.0e-6;
        base.convergence_config.current_abstol = 1.0e-12;

        let options = NetlistSimulationOptions {
            abstol: Some(5.0e-9),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());

        assert_eq!(resolved.convergence_config.voltage_abstol, 1.0e-6);
        assert_eq!(resolved.convergence_config.current_abstol, 5.0e-9);
    }

    #[test]
    fn deck_vntol_and_iabstol_remain_independent() {
        let base = SimulationConfig::default();
        let options = NetlistSimulationOptions {
            abstol: Some(5.0e-9),
            vntol: Some(2.0e-6),
            iabstol: Some(7.0e-12),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());

        assert_eq!(resolved.convergence_config.voltage_abstol, 2.0e-6);
        assert_eq!(resolved.convergence_config.current_abstol, 7.0e-12);
    }

    #[test]
    fn deck_trtol_updates_transient_tolerance_factor() {
        let mut base = SimulationConfig::default();
        base.transient_trtol = 5.0;
        let options = NetlistSimulationOptions {
            trtol: Some(2.25),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());

        assert_eq!(resolved.transient_trtol, 2.25);
    }

    #[test]
    fn explicit_trtol_override_wins_over_deck() {
        let base = SimulationConfig::default();
        let options = NetlistSimulationOptions {
            trtol: Some(2.25),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            transient_trtol: Some(4.5),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, Some(&options), &overrides);

        assert_eq!(resolved.transient_trtol, 4.5);
    }
}

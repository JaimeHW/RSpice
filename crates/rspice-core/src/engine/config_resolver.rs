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
    pub convergence_preset: Option<ConvergencePreset>,
    /// Legacy RELTOL knob: updates both `SimulationConfig::tolerance` and
    /// `ConvergenceConfig::voltage_reltol`.
    pub reltol: Option<Value>,
    /// Legacy ABSTOL knob: updates both voltage and current absolute tolerances.
    pub abstol: Option<Value>,
    pub voltage_abstol: Option<Value>,
    pub current_abstol: Option<Value>,
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
    let mut min_timestep = base.min_timestep;
    let mut max_timestep = base.max_timestep;
    let mut integration_method = base.integration_method;
    let mut tolerance = base.tolerance;
    let mut voltage_reltol = base.convergence_config.voltage_reltol;
    let mut voltage_abstol = base.convergence_config.voltage_abstol;
    let mut current_abstol = base.convergence_config.current_abstol;
    let mut residual_reltol = base.convergence_config.residual_reltol;
    let mut gmin_initial = base.convergence_config.gmin_initial;

    if let Some(opts) = netlist_options {
        if let Some(temp_celsius) = opts.temp {
            temperature = temp_celsius + 273.15;
        }
        if let Some(itl1) = opts.itl1 {
            max_iterations = itl1;
        }
        if let Some(method) = opts
            .method
            .as_deref()
            .and_then(parse_integration_method_option)
        {
            integration_method = method;
        }
        if let Some(reltol) = opts.reltol {
            tolerance = reltol;
            voltage_reltol = reltol;
            if opts.residual_reltol.is_none() {
                residual_reltol = reltol;
            }
        }
        if let Some(vntol) = opts.vntol.or(opts.abstol) {
            voltage_abstol = vntol;
        }
        if let Some(iabstol) = opts.iabstol.or(opts.abstol) {
            current_abstol = iabstol;
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
    resolved.min_timestep = min_timestep;
    resolved.max_timestep = max_timestep;
    resolved.integration_method = integration_method;
    resolved.tolerance = tolerance;
    resolved.convergence_config.voltage_reltol = voltage_reltol;
    resolved.convergence_config.voltage_abstol = voltage_abstol;
    resolved.convergence_config.current_abstol = current_abstol;
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
    use crate::analysis::IntegrationMethod;
    use crate::netlist::SimulationOptions as NetlistOptions;

    fn base_config() -> SimulationConfig {
        let mut cfg = SimulationConfig::default();
        cfg.temperature = 321.0;
        cfg.max_iterations = 77;
        cfg.min_timestep = 1e-14;
        cfg.max_timestep = 2e-4;
        cfg.integration_method = IntegrationMethod::TrapGear;
        cfg.tolerance = 8e-4;
        cfg.convergence_config.voltage_reltol = 8e-4;
        cfg.convergence_config.voltage_abstol = 7e-7;
        cfg.convergence_config.current_abstol = 6e-12;
        cfg.convergence_config.residual_reltol = 5e-4;
        cfg.convergence_config.gmin_initial = 1e-11;
        cfg.convergence_config.gmin_target = 1e-13;
        cfg
    }

    #[test]
    fn test_resolver_keeps_base_without_inputs() {
        let base = base_config();
        let resolved =
            resolve_simulation_config(&base, None, &SimulationConfigOverrides::default());
        assert!((resolved.temperature - base.temperature).abs() < 1e-15);
        assert_eq!(resolved.max_iterations, base.max_iterations);
        assert!((resolved.tolerance - base.tolerance).abs() < 1e-15);
        assert!((resolved.convergence_config.residual_reltol - 5e-4).abs() < 1e-15);
    }

    #[test]
    fn test_netlist_reltol_updates_voltage_and_legacy_tolerance() {
        let base = base_config();
        let opts = NetlistOptions {
            reltol: Some(2e-4),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert!((resolved.tolerance - 2e-4).abs() < 1e-15);
        assert!((resolved.convergence_config.voltage_reltol - 2e-4).abs() < 1e-15);
    }

    #[test]
    fn test_netlist_reltol_backfills_residual_when_not_explicit() {
        let base = base_config();
        let opts = NetlistOptions {
            reltol: Some(3e-4),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert!((resolved.convergence_config.residual_reltol - 3e-4).abs() < 1e-15);
    }

    #[test]
    fn test_netlist_explicit_residual_reltol_wins() {
        let base = base_config();
        let opts = NetlistOptions {
            reltol: Some(3e-4),
            residual_reltol: Some(9e-5),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert!((resolved.convergence_config.residual_reltol - 9e-5).abs() < 1e-15);
    }

    #[test]
    fn test_netlist_vntol_and_iabstol_map_independently() {
        let base = base_config();
        let opts = NetlistOptions {
            vntol: Some(4e-6),
            iabstol: Some(7e-12),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert!((resolved.convergence_config.voltage_abstol - 4e-6).abs() < 1e-18);
        assert!((resolved.convergence_config.current_abstol - 7e-12).abs() < 1e-24);
    }

    #[test]
    fn test_netlist_abstol_fallback_applies_to_voltage_and_current() {
        let base = base_config();
        let opts = NetlistOptions {
            abstol: Some(8e-13),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert!((resolved.convergence_config.voltage_abstol - 8e-13).abs() < 1e-24);
        assert!((resolved.convergence_config.current_abstol - 8e-13).abs() < 1e-24);
    }

    #[test]
    fn test_netlist_itl1_maps_to_max_iterations() {
        let base = base_config();
        let opts = NetlistOptions {
            itl1: Some(125),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert_eq!(resolved.max_iterations, 125);
    }

    #[test]
    fn test_netlist_temp_maps_celsius_to_kelvin() {
        let base = base_config();
        let opts = NetlistOptions {
            temp: Some(85.0),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert!((resolved.temperature - 358.15).abs() < 1e-12);
    }

    #[test]
    fn test_netlist_method_maps_to_integration_method() {
        let base = base_config();
        let opts = NetlistOptions {
            method: Some("GEAR".to_string()),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert_eq!(resolved.integration_method, IntegrationMethod::Gear2);
    }

    #[test]
    fn test_unknown_netlist_method_is_ignored() {
        let base = base_config();
        let opts = NetlistOptions {
            method: Some("NONSENSE".to_string()),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert_eq!(resolved.integration_method, base.integration_method);
    }

    #[test]
    fn test_override_reltol_wins_over_netlist() {
        let base = base_config();
        let opts = NetlistOptions {
            reltol: Some(2e-4),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            reltol: Some(9e-4),
            ..Default::default()
        };
        let resolved = resolve_simulation_config(&base, Some(&opts), &overrides);
        assert!((resolved.tolerance - 9e-4).abs() < 1e-15);
        assert!((resolved.convergence_config.voltage_reltol - 9e-4).abs() < 1e-15);
    }

    #[test]
    fn test_override_abstol_sets_both_then_specific_overrides_apply() {
        let base = base_config();
        let overrides = SimulationConfigOverrides {
            abstol: Some(3e-12),
            voltage_abstol: Some(4e-6),
            current_abstol: Some(5e-12),
            ..Default::default()
        };
        let resolved = resolve_simulation_config(&base, None, &overrides);
        assert!((resolved.convergence_config.voltage_abstol - 4e-6).abs() < 1e-18);
        assert!((resolved.convergence_config.current_abstol - 5e-12).abs() < 1e-24);
    }

    #[test]
    fn test_override_convergence_preset_preserves_tolerances() {
        let base = base_config();
        let overrides = SimulationConfigOverrides {
            convergence_preset: Some(ConvergencePreset::Fast),
            reltol: Some(1.2e-3),
            residual_reltol: Some(4e-4),
            ..Default::default()
        };
        let resolved = resolve_simulation_config(&base, None, &overrides);
        assert!(!resolved.convergence_config.gmin_stepping);
        assert!(!resolved.convergence_config.source_stepping);
        assert!((resolved.convergence_config.voltage_reltol - 1.2e-3).abs() < 1e-15);
        assert!((resolved.convergence_config.residual_reltol - 4e-4).abs() < 1e-15);
    }

    #[test]
    fn test_override_temperature_and_timesteps() {
        let base = base_config();
        let overrides = SimulationConfigOverrides {
            temperature_kelvin: Some(310.0),
            min_timestep: Some(2e-15),
            max_timestep: Some(9e-4),
            ..Default::default()
        };
        let resolved = resolve_simulation_config(&base, None, &overrides);
        assert!((resolved.temperature - 310.0).abs() < 1e-15);
        assert!((resolved.min_timestep - 2e-15).abs() < 1e-27);
        assert!((resolved.max_timestep - 9e-4).abs() < 1e-18);
    }

    #[test]
    fn test_gmin_target_is_clamped_not_to_exceed_initial() {
        let mut base = base_config();
        base.convergence_config.gmin_initial = 1e-12;
        base.convergence_config.gmin_target = 1e-15;
        let opts = NetlistOptions {
            gmin: Some(1e-16),
            ..Default::default()
        };
        let resolved =
            resolve_simulation_config(&base, Some(&opts), &SimulationConfigOverrides::default());
        assert!((resolved.convergence_config.gmin_initial - 1e-16).abs() < 1e-28);
        assert!((resolved.convergence_config.gmin_target - 1e-16).abs() < 1e-28);
    }

    #[test]
    fn test_preset_parsing() {
        assert_eq!(
            ConvergencePreset::from_mode_name("FAST"),
            Some(ConvergencePreset::Fast)
        );
        assert_eq!(
            ConvergencePreset::from_mode_name("default"),
            Some(ConvergencePreset::Default)
        );
        assert_eq!(
            ConvergencePreset::from_mode_name("Robust"),
            Some(ConvergencePreset::Robust)
        );
        assert_eq!(ConvergencePreset::from_mode_name("unknown"), None);
    }
}

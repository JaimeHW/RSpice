//! Simulation configuration resolution with explicit precedence.
//!
//! This module centralizes config layering for frontends:
//! 1. `base` configuration (frontend defaults / profile)
//! 2. netlist `.OPTIONS`
//! 3. explicit runtime overrides (CLI/UI/Python/etc.)

use super::{ConvergenceConfig, JfetLevel2Model, SimulationConfig, SpiceDialect};
use crate::Value;
use crate::netlist::SimulationOptions as NetlistSimulationOptions;
use crate::numerics::integration::{
    IntegrationMethod, TransientErrorControl, TransientLteReference,
};

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
    pub transient_lte_reltol: Option<Value>,
    pub transient_lte_abstol: Option<Value>,
    pub transient_timeint_max_timestep: Option<Value>,
    pub transient_use_device_max_timestep: Option<bool>,
    pub transient_error_control: Option<TransientErrorControl>,
    pub transient_min_steps_between_breakpoints: Option<usize>,
    pub transient_timeint_nlmin: Option<usize>,
    pub transient_timeint_nlmax: Option<usize>,
    pub transient_timeint_min_order: Option<u8>,
    pub transient_timeint_max_order: Option<u8>,
    pub transient_timesteps_reversal: Option<bool>,
    pub transient_nonlinear_reltol: Option<Value>,
    pub transient_nonlinear_abstol: Option<Value>,
    pub transient_nonlinear_deltaxtol: Option<Value>,
    pub transient_nonlinear_rhstol: Option<Value>,
    pub transient_nonlinear_max_iterations: Option<usize>,
    pub transient_enforce_device_convergence: Option<bool>,
    pub transient_nonlinear_nox: Option<bool>,
    pub transient_lte_reference: Option<TransientLteReference>,
    pub transient_new_bp_stepping: Option<bool>,
    pub ramptime: Option<Value>,
    pub digital_delay_type: Option<i64>,
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
    pub device_voltage_limiting: Option<bool>,
    pub spice_dialect: Option<SpiceDialect>,
    pub jfet_level2_model: Option<JfetLevel2Model>,
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
    let mut transient_lte_reltol = base.transient_lte_reltol;
    let mut transient_lte_abstol = base.transient_lte_abstol;
    let mut transient_timeint_max_timestep = base.transient_timeint_max_timestep;
    let mut transient_use_device_max_timestep = base.transient_use_device_max_timestep;
    let mut transient_error_control = base.transient_error_control;
    let mut transient_min_steps_between_breakpoints = base.transient_min_steps_between_breakpoints;
    let mut transient_timeint_nlmin = base.transient_timeint_nlmin;
    let mut transient_timeint_nlmax = base.transient_timeint_nlmax;
    let mut transient_timeint_min_order = base.transient_timeint_min_order;
    let mut transient_timeint_max_order = base.transient_timeint_max_order;
    let mut transient_timesteps_reversal = base.transient_timesteps_reversal;
    let mut transient_nonlinear_reltol = base.transient_nonlinear_reltol;
    let mut transient_nonlinear_abstol = base.transient_nonlinear_abstol;
    let mut transient_nonlinear_deltaxtol = base.transient_nonlinear_deltaxtol;
    let mut transient_nonlinear_rhstol = base.transient_nonlinear_rhstol;
    let mut transient_nonlinear_max_iterations = base.transient_nonlinear_max_iterations;
    let mut transient_enforce_device_convergence = base.transient_enforce_device_convergence;
    let mut transient_nonlinear_nox = base.transient_nonlinear_nox;
    let mut transient_lte_reference = base
        .transient_lte_reference
        .unwrap_or_else(|| base.spice_dialect.default_transient_lte_reference());
    let mut transient_new_bp_stepping = base.transient_new_bp_stepping;
    let mut ramptime = base.ramptime;
    let mut digital_delay_type = base.digital_delay_type;
    let mut tolerance = base.tolerance;
    let mut voltage_reltol = base.convergence_config.voltage_reltol;
    let mut voltage_abstol = base.convergence_config.voltage_abstol;
    let mut current_abstol = base.convergence_config.current_abstol;
    let mut charge_abstol = base.convergence_config.charge_abstol;
    let mut residual_reltol = base.convergence_config.residual_reltol;
    let mut gmin_initial = base.convergence_config.gmin_initial;
    let gmin_target = base.convergence_config.gmin_target;
    let mut junction_gmin_target = base.convergence_config.junction_gmin_target;
    let mut b3soi_gmin_scaling = base.b3soi_gmin_scaling;
    let mut device_voltage_limiting = base.device_voltage_limiting;
    let mut nonlinear_continuation = base.convergence_config.nonlinear_continuation;

    if let Some(opts) = netlist_options {
        if let Some(pivrel) = opts.pivrel {
            resolved.matrix_pivot_tolerance = pivrel;
        }
        if let Some(pivtol) = opts.pivtol {
            resolved.matrix_absolute_pivot_tolerance = pivtol;
        }
        if let Some(mode) = opts.nonlinear_continuation {
            nonlinear_continuation = Some(mode);
        }
        // The continuation rungs and the damping strategy are written onto
        // `resolved` here rather than held in a local: they are exactly the
        // five fields a `convergence_preset` override is made of, and a local
        // written back at the end of this function would replay the base
        // config over the preset and make that override inert.
        if let Some(enabled) = opts.gmin_stepping {
            resolved.convergence_config.gmin_stepping = enabled;
        }
        if let Some(enabled) = opts.source_stepping {
            resolved.convergence_config.source_stepping = enabled;
        }
        if let Some(enabled) = opts.pseudo_transient {
            resolved.convergence_config.pseudo_transient = enabled;
        }
        if let Some(enabled) = opts.arc_length {
            resolved.convergence_config.arc_length = enabled;
        }
        if let Some(strategy) = opts.damping_strategy {
            resolved.convergence_config.damping_strategy = strategy;
        }
        if let Some(backend) = opts.matrix_solver {
            resolved.matrix_solver = Some(backend);
        }
        if let Some(floor) = opts.timeint_min_timestep {
            min_timestep = floor;
        }
        // `MAXTIMESTEP` and `TIMEINT DELMAX` resolve to two different fields
        // on purpose: the transient engine applies both as separate clamps,
        // so a deck that states one must leave the other exactly as it found
        // it or the tighter bound would be silently replaced by the looser.
        if let Some(ceiling) = opts.max_timestep {
            max_timestep = ceiling;
        }
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
        if let Some(reltol) = opts.timeint_reltol {
            transient_lte_reltol = Some(reltol);
        }
        if let Some(abstol) = opts.timeint_abstol {
            transient_lte_abstol = Some(abstol);
        }
        if let Some(delmax) = opts.timeint_delmax {
            transient_timeint_max_timestep = Some(delmax);
        }
        if let Some(enabled) = opts.timeint_use_device_max_timestep {
            transient_use_device_max_timestep = Some(enabled);
        }
        if let Some(control) = opts.timeint_error_control {
            transient_error_control = control;
        }
        if let Some(steps) = opts.timeint_min_steps_between_breakpoints {
            transient_min_steps_between_breakpoints = Some(steps);
        }
        if let Some(nlmin) = opts.timeint_nlmin {
            transient_timeint_nlmin = nlmin;
        }
        if let Some(nlmax) = opts.timeint_nlmax {
            transient_timeint_nlmax = nlmax;
        }
        if let Some(order) = opts.timeint_min_order {
            transient_timeint_min_order = order;
        }
        if let Some(order) = opts.timeint_max_order {
            transient_timeint_max_order = order;
        }
        if let Some(enabled) = opts.timeint_timesteps_reversal {
            transient_timesteps_reversal = enabled;
        }
        if let Some(reltol) = opts.nonlin_transient_reltol {
            transient_nonlinear_reltol = Some(reltol);
        }
        if let Some(abstol) = opts.nonlin_transient_abstol {
            transient_nonlinear_abstol = Some(abstol);
        }
        if let Some(deltaxtol) = opts.nonlin_transient_deltaxtol {
            transient_nonlinear_deltaxtol = Some(deltaxtol);
        }
        if let Some(rhstol) = opts.nonlin_transient_rhstol {
            transient_nonlinear_rhstol = Some(rhstol);
        }
        if let Some(maxstep) = opts.nonlin_transient_maxstep {
            transient_nonlinear_max_iterations = Some(maxstep);
        }
        if let Some(enforce) = opts.nonlin_transient_enforce_device_convergence {
            transient_enforce_device_convergence = Some(enforce);
        }
        if let Some(nox) = opts.nonlin_transient_nox {
            transient_nonlinear_nox = Some(nox);
        }
        if let Some(reference) = opts.transient_lte_reference {
            transient_lte_reference = reference;
        }
        if let Some(enabled) = opts.transient_new_bp_stepping {
            transient_new_bp_stepping = enabled;
        }
        if let Some(value) = opts.ramptime {
            ramptime = value;
        }
        if let Some(value) = opts.digital_delay_type {
            digital_delay_type = Some(value);
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
            junction_gmin_target = gmin;
            if gmin_initial < gmin {
                gmin_initial = gmin;
            }
        }
        if let Some(scaling) = opts.b3soi_gmin_scaling {
            b3soi_gmin_scaling = scaling;
        }
        if let Some(enabled) = opts.device_voltage_limiting {
            device_voltage_limiting = enabled;
        }
        if let Some(rshunt) = opts.rshunt {
            resolved.rshunt = Some(rshunt);
        }
        if let Some(cshunt) = opts.cshunt {
            resolved.cshunt = Some(cshunt);
        }
        // Written straight onto `resolved` rather than through a local: the
        // bypass bounds have no second source to reconcile against, and a
        // stated bound applies whether or not the same card also turns bypass
        // on, so a later `.OPTIONS BYPASS` picks it up.
        if let Some(bypass) = opts.bypass {
            resolved.bypass_config.enabled = bypass;
        }
        if let Some(reltol) = opts.bypass_reltol {
            resolved.bypass_config.reltol = reltol;
        }
        if let Some(abstol) = opts.bypass_abstol {
            resolved.bypass_config.abstol = abstol;
        }
    }

    if let Some(temp_k) = overrides.temperature_kelvin {
        temperature = temp_k;
    }
    if let Some(iters) = overrides.max_iterations {
        max_iterations = iters;
        transient_max_iterations = iters;
        transient_nonlinear_max_iterations = Some(iters);
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
    if let Some(reltol) = overrides.transient_lte_reltol {
        transient_lte_reltol = Some(reltol);
    }
    if let Some(abstol) = overrides.transient_lte_abstol {
        transient_lte_abstol = Some(abstol);
    }
    if let Some(delmax) = overrides.transient_timeint_max_timestep {
        transient_timeint_max_timestep = Some(delmax);
    }
    if let Some(enabled) = overrides.transient_use_device_max_timestep {
        transient_use_device_max_timestep = Some(enabled);
    }
    if let Some(reltol) = overrides.transient_nonlinear_reltol {
        transient_nonlinear_reltol = Some(reltol);
    }
    if let Some(abstol) = overrides.transient_nonlinear_abstol {
        transient_nonlinear_abstol = Some(abstol);
    }
    if let Some(deltaxtol) = overrides.transient_nonlinear_deltaxtol {
        transient_nonlinear_deltaxtol = Some(deltaxtol);
    }
    if let Some(rhstol) = overrides.transient_nonlinear_rhstol {
        transient_nonlinear_rhstol = Some(rhstol);
    }
    if let Some(maxstep) = overrides.transient_nonlinear_max_iterations {
        transient_nonlinear_max_iterations = Some(maxstep);
    }
    if let Some(enforce) = overrides.transient_enforce_device_convergence {
        transient_enforce_device_convergence = Some(enforce);
    }
    if let Some(nox) = overrides.transient_nonlinear_nox {
        transient_nonlinear_nox = Some(nox);
    }
    if let Some(value) = overrides.ramptime {
        ramptime = value;
    }
    if let Some(value) = overrides.digital_delay_type {
        digital_delay_type = Some(value);
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
    if let Some(enabled) = overrides.device_voltage_limiting {
        device_voltage_limiting = enabled;
    }
    if let Some(dialect) = overrides.spice_dialect {
        resolved = resolved.with_spice_dialect(dialect);
        if base.transient_lte_reference.is_none()
            && netlist_options.is_none_or(|options| options.transient_lte_reference.is_none())
        {
            transient_lte_reference = dialect.default_transient_lte_reference();
        }
    }
    if let Some(model) = overrides.jfet_level2_model {
        resolved.jfet_level2_model = model;
    }
    if let Some(reference) = overrides.transient_lte_reference {
        transient_lte_reference = reference;
    }
    if let Some(enabled) = overrides.transient_new_bp_stepping {
        transient_new_bp_stepping = enabled;
    }
    if let Some(control) = overrides.transient_error_control {
        transient_error_control = control;
    }
    if let Some(steps) = overrides.transient_min_steps_between_breakpoints {
        transient_min_steps_between_breakpoints = Some(steps);
    }
    if let Some(nlmin) = overrides.transient_timeint_nlmin {
        transient_timeint_nlmin = nlmin;
    }
    if let Some(nlmax) = overrides.transient_timeint_nlmax {
        transient_timeint_nlmax = nlmax;
    }
    if let Some(order) = overrides.transient_timeint_min_order {
        transient_timeint_min_order = order;
    }
    if let Some(order) = overrides.transient_timeint_max_order {
        transient_timeint_max_order = order;
    }
    if let Some(enabled) = overrides.transient_timesteps_reversal {
        transient_timesteps_reversal = enabled;
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
    resolved.transient_lte_reltol = transient_lte_reltol;
    resolved.transient_lte_abstol = transient_lte_abstol;
    resolved.transient_timeint_max_timestep = transient_timeint_max_timestep;
    resolved.transient_use_device_max_timestep = transient_use_device_max_timestep;
    resolved.transient_error_control = transient_error_control;
    resolved.transient_min_steps_between_breakpoints = transient_min_steps_between_breakpoints;
    resolved.transient_timeint_nlmin = transient_timeint_nlmin;
    resolved.transient_timeint_nlmax = transient_timeint_nlmax;
    resolved.transient_timeint_min_order = transient_timeint_min_order;
    resolved.transient_timeint_max_order = transient_timeint_max_order;
    resolved.transient_timesteps_reversal = transient_timesteps_reversal;
    resolved.transient_nonlinear_reltol = transient_nonlinear_reltol;
    resolved.transient_nonlinear_abstol = transient_nonlinear_abstol;
    resolved.transient_nonlinear_deltaxtol = transient_nonlinear_deltaxtol;
    resolved.transient_nonlinear_rhstol = transient_nonlinear_rhstol;
    resolved.transient_nonlinear_max_iterations = transient_nonlinear_max_iterations;
    resolved.transient_enforce_device_convergence = transient_enforce_device_convergence;
    resolved.transient_nonlinear_nox = transient_nonlinear_nox;
    resolved.transient_lte_reference = Some(transient_lte_reference);
    resolved.transient_new_bp_stepping = transient_new_bp_stepping;
    resolved.ramptime = ramptime;
    resolved.digital_delay_type = digital_delay_type;
    resolved.tolerance = tolerance;
    resolved.convergence_config.voltage_reltol = voltage_reltol;
    resolved.convergence_config.voltage_abstol = voltage_abstol;
    resolved.convergence_config.current_abstol = current_abstol;
    resolved.convergence_config.charge_abstol = charge_abstol;
    resolved.convergence_config.residual_reltol = residual_reltol;
    resolved.convergence_config.gmin_initial = gmin_initial;
    resolved.convergence_config.gmin_target = gmin_target;
    resolved.convergence_config.junction_gmin_target = junction_gmin_target;
    resolved.convergence_config.nonlinear_continuation = nonlinear_continuation;
    resolved.b3soi_gmin_scaling = b3soi_gmin_scaling;
    resolved.device_voltage_limiting = device_voltage_limiting;
    if resolved.convergence_config.gmin_target > resolved.convergence_config.gmin_initial {
        resolved.convergence_config.gmin_initial = resolved.convergence_config.gmin_target;
    }
    if resolved.convergence_config.junction_gmin_target > resolved.convergence_config.gmin_initial {
        resolved.convergence_config.gmin_initial = resolved.convergence_config.junction_gmin_target;
    }

    resolved
}

pub(crate) fn parse_integration_method_option(method: &str) -> Option<IntegrationMethod> {
    if method.eq_ignore_ascii_case("TRAP")
        || method.eq_ignore_ascii_case("TRAPEZOIDAL")
        || method.eq_ignore_ascii_case("TRAPEZOID")
        || method.eq_ignore_ascii_case("ONESTEP")
        || method == "7"
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
        || method == "8"
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
    fn device_voltage_limiting_follows_base_deck_override_precedence() {
        let base = SimulationConfig {
            device_voltage_limiting: true,
            ..Default::default()
        };
        let options = NetlistSimulationOptions {
            device_voltage_limiting: Some(false),
            ..Default::default()
        };
        let deck =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());
        assert!(!deck.device_voltage_limiting);

        let overridden = resolve_simulation_config(
            &base,
            Some(&options),
            &SimulationConfigOverrides {
                device_voltage_limiting: Some(true),
                ..Default::default()
            },
        );
        assert!(overridden.device_voltage_limiting);
        assert!(SimulationConfig::default().device_voltage_limiting);
    }

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
        let base = SimulationConfig {
            transient_trtol: 5.0,
            ..Default::default()
        };
        let options = NetlistSimulationOptions {
            trtol: Some(2.25),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());

        assert_eq!(resolved.transient_trtol, 2.25);
    }

    #[test]
    fn xyce_dialect_derives_point_global_lte_when_mode_is_omitted() {
        let base = SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        };
        assert_eq!(
            base.effective_transient_min_steps_between_breakpoints(),
            None,
            "ERROPTION=0 without authored MINTIMESTEPSBP preserves the prior default"
        );

        let resolved = resolve_simulation_config(
            &base,
            Some(&NetlistSimulationOptions::default()),
            &SimulationConfigOverrides::default(),
        );

        assert_eq!(
            resolved.transient_lte_reference,
            Some(TransientLteReference::PointGlobal)
        );
    }

    #[test]
    fn timeint_lte_settings_are_separate_and_follow_override_precedence() {
        let mut base = SimulationConfig::default();
        base.convergence_config.voltage_reltol = 4.0e-3;
        base.convergence_config.voltage_abstol = 5.0e-6;
        base.convergence_config.current_abstol = 6.0e-12;
        let options = NetlistSimulationOptions {
            timeint_reltol: Some(2.0e-6),
            timeint_abstol: Some(3.0e-9),
            timeint_delmax: Some(4.0e-8),
            timeint_use_device_max_timestep: Some(false),
            transient_lte_reference: Some(TransientLteReference::SignalGlobal),
            abstol: Some(7.0e-12),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            transient_lte_reltol: Some(8.0e-7),
            transient_lte_abstol: Some(9.0e-10),
            transient_timeint_max_timestep: Some(1.0e-8),
            transient_use_device_max_timestep: Some(true),
            transient_lte_reference: Some(TransientLteReference::SignalLocal),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, Some(&options), &overrides);

        assert_eq!(resolved.transient_lte_reltol, Some(8.0e-7));
        assert_eq!(resolved.transient_lte_abstol, Some(9.0e-10));
        assert_eq!(resolved.transient_timeint_max_timestep, Some(1.0e-8));
        assert_eq!(resolved.transient_use_device_max_timestep, Some(true));
        assert_eq!(
            resolved.transient_lte_reference,
            Some(TransientLteReference::SignalLocal)
        );
        assert_eq!(resolved.convergence_config.voltage_reltol, 4.0e-3);
        assert_eq!(resolved.convergence_config.voltage_abstol, 5.0e-6);
        assert_eq!(resolved.convergence_config.current_abstol, 7.0e-12);
    }

    #[test]
    fn timeint_iteration_control_follows_override_precedence_and_given_semantics() {
        let base = SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        };
        let options = NetlistSimulationOptions {
            timeint_error_control: Some(TransientErrorControl::NonlinearIterations),
            timeint_min_steps_between_breakpoints: Some(12),
            timeint_nlmin: Some(2),
            timeint_nlmax: Some(9),
            timeint_min_order: Some(2),
            timeint_max_order: Some(2),
            timeint_timesteps_reversal: Some(true),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            transient_min_steps_between_breakpoints: Some(20),
            transient_timeint_nlmin: Some(4),
            transient_timeint_nlmax: Some(10),
            transient_timeint_min_order: Some(1),
            transient_timeint_max_order: Some(1),
            transient_timesteps_reversal: Some(false),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, Some(&options), &overrides);
        assert_eq!(
            resolved.transient_error_control,
            TransientErrorControl::NonlinearIterations
        );
        assert_eq!(resolved.transient_min_steps_between_breakpoints, Some(20));
        assert_eq!(resolved.transient_timeint_nlmin, 4);
        assert_eq!(resolved.transient_timeint_nlmax, 10);
        assert_eq!(resolved.transient_timeint_min_order, 1);
        assert_eq!(resolved.transient_timeint_max_order, 1);
        assert!(!resolved.transient_timesteps_reversal);
        assert_eq!(
            resolved.effective_transient_min_steps_between_breakpoints(),
            Some(20)
        );

        let implicit = resolve_simulation_config(
            &base,
            Some(&NetlistSimulationOptions {
                timeint_error_control: Some(TransientErrorControl::NonlinearIterations),
                ..Default::default()
            }),
            &SimulationConfigOverrides::default(),
        );
        assert_eq!(implicit.transient_min_steps_between_breakpoints, None);
        assert_eq!(
            implicit.effective_transient_min_steps_between_breakpoints(),
            Some(crate::numerics::integration::XYCE_DEFAULT_MIN_TIME_STEPS_BREAKPOINT)
        );

        let disabled = resolve_simulation_config(
            &base,
            Some(&NetlistSimulationOptions {
                timeint_error_control: Some(TransientErrorControl::NonlinearIterations),
                timeint_min_steps_between_breakpoints: Some(12),
                ..Default::default()
            }),
            &SimulationConfigOverrides {
                transient_min_steps_between_breakpoints: Some(0),
                ..Default::default()
            },
        );
        assert_eq!(disabled.transient_min_steps_between_breakpoints, Some(0));
        assert_eq!(
            disabled.effective_transient_min_steps_between_breakpoints(),
            None,
            "an explicit zero override disables mode-1's implicit ceiling"
        );
    }

    #[test]
    fn transient_nonlinear_settings_are_separate_and_follow_override_precedence() {
        let base = SimulationConfig {
            transient_nonlinear_reltol: Some(1.0e-2),
            transient_nonlinear_abstol: Some(2.0e-6),
            transient_nonlinear_deltaxtol: Some(0.33),
            transient_nonlinear_rhstol: Some(3.0e-2),
            transient_nonlinear_max_iterations: Some(20),
            transient_enforce_device_convergence: Some(true),
            ..Default::default()
        };
        let options = NetlistSimulationOptions {
            nonlin_transient_reltol: Some(4.0e-3),
            nonlin_transient_abstol: Some(5.0e-7),
            nonlin_transient_deltaxtol: Some(0.25),
            nonlin_transient_rhstol: Some(6.0e-3),
            nonlin_transient_maxstep: Some(30),
            nonlin_transient_enforce_device_convergence: Some(false),
            reltol: Some(7.0e-4),
            abstol: Some(8.0e-12),
            timeint_reltol: Some(9.0e-5),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            transient_nonlinear_reltol: Some(1.0e-3),
            transient_nonlinear_rhstol: Some(2.0e-3),
            transient_nonlinear_max_iterations: Some(40),
            transient_enforce_device_convergence: Some(true),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, Some(&options), &overrides);

        assert_eq!(resolved.transient_nonlinear_reltol, Some(1.0e-3));
        assert_eq!(resolved.transient_nonlinear_abstol, Some(5.0e-7));
        assert_eq!(resolved.transient_nonlinear_deltaxtol, Some(0.25));
        assert_eq!(resolved.transient_nonlinear_rhstol, Some(2.0e-3));
        assert_eq!(resolved.transient_nonlinear_max_iterations, Some(40));
        assert_eq!(resolved.transient_enforce_device_convergence, Some(true));
        assert_eq!(resolved.convergence_config.voltage_reltol, 7.0e-4);
        assert_eq!(resolved.convergence_config.current_abstol, 8.0e-12);
        assert_eq!(resolved.transient_lte_reltol, Some(9.0e-5));
    }

    #[test]
    fn new_breakpoint_stepping_follows_config_precedence() {
        let base = SimulationConfig {
            transient_new_bp_stepping: true,
            ..Default::default()
        };
        let options = NetlistSimulationOptions {
            transient_new_bp_stepping: Some(false),
            ..Default::default()
        };

        let from_deck =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());
        assert!(!from_deck.transient_new_bp_stepping);

        let overridden = resolve_simulation_config(
            &base,
            Some(&options),
            &SimulationConfigOverrides {
                transient_new_bp_stepping: Some(true),
                ..Default::default()
            },
        );
        assert!(overridden.transient_new_bp_stepping);
    }

    #[test]
    fn explicit_deck_lte_mode_survives_a_dialect_override() {
        let options = NetlistSimulationOptions {
            transient_lte_reference: Some(TransientLteReference::SignalGlobal),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            spice_dialect: Some(SpiceDialect::Ngspice),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&SimulationConfig::default(), Some(&options), &overrides);

        assert_eq!(resolved.spice_dialect, SpiceDialect::Ngspice);
        assert_eq!(
            resolved.transient_lte_reference,
            Some(TransientLteReference::SignalGlobal)
        );
    }

    #[test]
    fn deck_digital_delay_type_updates_xspice_policy() {
        let base = SimulationConfig::default();
        let options = NetlistSimulationOptions {
            digital_delay_type: Some(1),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());

        assert_eq!(resolved.digital_delay_type, Some(1));
    }

    #[test]
    fn deck_pivot_controls_reach_the_sparse_matrix_policy() {
        let options = NetlistSimulationOptions {
            pivrel: Some(0.125),
            pivtol: Some(2.5e-14),
            ..Default::default()
        };
        let resolved = resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&options),
            &SimulationConfigOverrides::default(),
        );
        assert_eq!(resolved.matrix_pivot_tolerance, 0.125);
        assert_eq!(resolved.matrix_absolute_pivot_tolerance, 2.5e-14);
        assert!(resolved.validate().is_ok());
    }

    #[test]
    fn xyce_numeric_timeint_method_selectors_resolve_to_active_methods() {
        assert_eq!(
            parse_integration_method_option("7"),
            Some(IntegrationMethod::Trapezoidal)
        );
        assert_eq!(
            parse_integration_method_option("ONESTEP"),
            Some(IntegrationMethod::Trapezoidal)
        );
        assert_eq!(
            parse_integration_method_option("8"),
            Some(IntegrationMethod::Gear2)
        );
    }

    #[test]
    fn deck_gmin_updates_device_junction_floor_not_final_nodal_floor() {
        let base = SimulationConfig::default();
        let options = NetlistSimulationOptions {
            gmin: Some(0.0),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());

        assert_eq!(
            resolved.convergence_config.gmin_target,
            base.convergence_config.gmin_target
        );
        assert_eq!(resolved.convergence_config.junction_gmin_target, 0.0);
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

    #[test]
    fn explicit_jfet_level2_model_override_wins_over_base() {
        let base = SimulationConfig::default();
        assert_eq!(
            base.resolved_jfet_level2_model(),
            crate::engine::JfetLevel2Model::ParkerSkellern
        );

        let overrides = SimulationConfigOverrides {
            jfet_level2_model: Some(crate::engine::JfetLevel2Model::XyceModifiedShockley),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, None, &overrides);

        assert_eq!(
            resolved.jfet_level2_model,
            crate::engine::JfetLevel2Model::XyceModifiedShockley
        );
        assert_eq!(
            resolved.resolved_jfet_level2_model(),
            crate::engine::JfetLevel2Model::XyceModifiedShockley
        );
    }

    #[test]
    fn explicit_spice_dialect_selects_ngspice_jfet_level2_model() {
        let base = SimulationConfig::default();
        let overrides = SimulationConfigOverrides {
            spice_dialect: Some(crate::engine::SpiceDialect::Ngspice),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, None, &overrides);

        assert_eq!(resolved.spice_dialect, crate::engine::SpiceDialect::Ngspice);
        assert_eq!(
            resolved.jfet_level2_model,
            crate::engine::JfetLevel2Model::DialectDefault
        );
        assert_eq!(
            resolved.resolved_jfet_level2_model(),
            crate::engine::JfetLevel2Model::ParkerSkellern
        );
    }

    #[test]
    fn explicit_spice_dialect_selects_xyce_jfet_level2_model() {
        let base = SimulationConfig::default();
        let overrides = SimulationConfigOverrides {
            spice_dialect: Some(crate::engine::SpiceDialect::Xyce),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, None, &overrides);

        assert_eq!(resolved.spice_dialect, crate::engine::SpiceDialect::Xyce);
        assert_eq!(
            resolved.jfet_level2_model,
            crate::engine::JfetLevel2Model::DialectDefault
        );
        assert_eq!(
            resolved.resolved_jfet_level2_model(),
            crate::engine::JfetLevel2Model::XyceModifiedShockley
        );
    }

    #[test]
    fn explicit_jfet_level2_model_override_wins_over_spice_dialect() {
        let base = SimulationConfig::default();
        let overrides = SimulationConfigOverrides {
            spice_dialect: Some(crate::engine::SpiceDialect::Xyce),
            jfet_level2_model: Some(crate::engine::JfetLevel2Model::ParkerSkellern),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(&base, None, &overrides);

        assert_eq!(resolved.spice_dialect, crate::engine::SpiceDialect::Xyce);
        assert_eq!(
            resolved.jfet_level2_model,
            crate::engine::JfetLevel2Model::ParkerSkellern
        );
        assert_eq!(
            resolved.resolved_jfet_level2_model(),
            crate::engine::JfetLevel2Model::ParkerSkellern
        );
    }

    #[test]
    fn deck_continuation_ladder_damping_and_solver_reach_the_engine_config() {
        let options = NetlistSimulationOptions {
            gmin_stepping: Some(false),
            source_stepping: Some(false),
            pseudo_transient: Some(false),
            arc_length: Some(true),
            damping_strategy: Some(crate::engine::DampingStrategy::Combined),
            matrix_solver: Some(crate::solver::RealSolverBackend::Klu),
            timeint_min_timestep: Some(2.0e-18),
            ..Default::default()
        };

        let resolved = resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&options),
            &SimulationConfigOverrides::default(),
        );

        assert!(!resolved.convergence_config.gmin_stepping);
        assert!(!resolved.convergence_config.source_stepping);
        assert!(!resolved.convergence_config.pseudo_transient);
        assert!(resolved.convergence_config.arc_length);
        assert_eq!(
            resolved.convergence_config.damping_strategy,
            crate::engine::DampingStrategy::Combined
        );
        assert_eq!(
            resolved.matrix_solver,
            Some(crate::solver::RealSolverBackend::Klu)
        );
        assert_eq!(resolved.min_timestep, 2.0e-18);
    }

    #[test]
    fn a_convergence_preset_still_outranks_the_decks_ladder_and_damping() {
        // The five ladder and damping fields are the entire content of a
        // preset. Resolving them through a base-seeded local would replay the
        // base config over the preset and leave `--convergence-mode` with no
        // effect at all, so this pins the precedence rather than the values.
        let options = NetlistSimulationOptions {
            gmin_stepping: Some(true),
            source_stepping: Some(true),
            pseudo_transient: Some(true),
            arc_length: Some(true),
            damping_strategy: Some(crate::engine::DampingStrategy::Combined),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            convergence_preset: Some(ConvergencePreset::Fast),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&SimulationConfig::default(), Some(&options), &overrides);

        assert!(!resolved.convergence_config.gmin_stepping);
        assert!(!resolved.convergence_config.source_stepping);
        assert!(!resolved.convergence_config.pseudo_transient);
        assert!(!resolved.convergence_config.arc_length);
        assert_eq!(
            resolved.convergence_config.damping_strategy,
            crate::engine::DampingStrategy::None
        );
    }

    #[test]
    fn the_two_deck_step_ceilings_resolve_onto_their_own_fields() {
        // The transient engine clamps against `max_timestep` and against
        // `transient_timeint_max_timestep` in turn. Resolving either onto the
        // other's field would drop one clamp, so a deck stating one key must
        // leave the other field exactly as the base config had it.
        let both = resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&NetlistSimulationOptions {
                max_timestep: Some(4.0e-9),
                timeint_delmax: Some(7.0e-9),
                ..Default::default()
            }),
            &SimulationConfigOverrides::default(),
        );
        assert_eq!(both.max_timestep, 4.0e-9);
        assert_eq!(both.transient_timeint_max_timestep, Some(7.0e-9));

        let ceiling_only = resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&NetlistSimulationOptions {
                max_timestep: Some(4.0e-9),
                ..Default::default()
            }),
            &SimulationConfigOverrides::default(),
        );
        assert_eq!(ceiling_only.max_timestep, 4.0e-9);
        assert_eq!(ceiling_only.transient_timeint_max_timestep, None);

        let delmax_only = resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&NetlistSimulationOptions {
                timeint_delmax: Some(7.0e-9),
                ..Default::default()
            }),
            &SimulationConfigOverrides::default(),
        );
        assert_eq!(
            delmax_only.max_timestep,
            SimulationConfig::default().max_timestep
        );
        assert_eq!(delmax_only.transient_timeint_max_timestep, Some(7.0e-9));
    }

    #[test]
    fn an_explicit_max_timestep_override_wins_over_the_decks_ceiling() {
        let resolved = resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&NetlistSimulationOptions {
                max_timestep: Some(4.0e-9),
                ..Default::default()
            }),
            &SimulationConfigOverrides {
                max_timestep: Some(1.0e-9),
                ..Default::default()
            },
        );

        assert_eq!(resolved.max_timestep, 1.0e-9);
    }

    #[test]
    fn an_explicit_min_timestep_override_wins_over_the_decks_floor() {
        let options = NetlistSimulationOptions {
            timeint_min_timestep: Some(2.0e-18),
            ..Default::default()
        };
        let overrides = SimulationConfigOverrides {
            min_timestep: Some(5.0e-16),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&SimulationConfig::default(), Some(&options), &overrides);

        assert_eq!(resolved.min_timestep, 5.0e-16);
    }

    #[test]
    fn deck_nonlinear_continuation_mode_propagates_to_engine_config() {
        use crate::config::NonlinearContinuationMode;

        let base = SimulationConfig::default();
        let options = NetlistSimulationOptions {
            nonlinear_continuation: Some(NonlinearContinuationMode::SimultaneousSourceStep),
            ..Default::default()
        };

        let resolved =
            resolve_simulation_config(&base, Some(&options), &SimulationConfigOverrides::default());

        assert_eq!(
            resolved.convergence_config.nonlinear_continuation,
            Some(NonlinearContinuationMode::SimultaneousSourceStep)
        );
    }
}

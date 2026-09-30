//! Shooting-PSS request validation and canonical engine configuration.

use rspice_core::Value;
use rspice_core::analysis::PssConfig;

/// Fully materialized shooting-PSS request used by the execution pipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct PssRunConfig {
    pub fundamental_freq: Value,
    pub tone_sources: Vec<String>,
    pub tstab_periods: usize,
    pub points_per_period: usize,
    pub num_harmonics: usize,
    pub tolerance: Value,
    pub oscillator_mode: bool,
    pub oscillator_node: Option<String>,
    /// Integration method for each shooting period's inner transient, or
    /// `None` for the engine's own default.
    pub integration_method: Option<rspice_core::numerics::integration::IntegrationMethod>,
    /// Stabilization window in seconds. Zero takes it from `tstab_periods`,
    /// which is `PssConfig::effective_tstab`'s own rule.
    pub tstab: Value,
    /// Shooting-Newton correction limit per integration grid.
    pub max_iterations: usize,
    /// Absolute periodicity tolerance, in each coordinate's own units,
    /// including the authored integral units for behavioral SDT states.
    pub abstol: Value,
    /// Newton damping factor in `[0.1, 1.0]`.
    pub damping: Value,
    /// Largest relative period correction one autonomous iteration takes.
    pub max_period_change: Value,
    /// Whether the engine logs its shooting convergence.
    pub verbose: bool,
}

impl PssRunConfig {
    pub fn new(
        fundamental_freq: Value,
        tone_sources: Vec<String>,
        num_harmonics: usize,
        tolerance: Value,
    ) -> Self {
        Self {
            fundamental_freq,
            tone_sources,
            tstab_periods: 20,
            points_per_period: 512,
            num_harmonics,
            tolerance,
            oscillator_mode: false,
            oscillator_node: None,
            integration_method: None,
            // The engine's own card defaults. This constructor serves the
            // compatibility entry point, which predates every one of these
            // controls and therefore asked for exactly these values.
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        }
    }
}

/// The engine configuration one shooting request resolves to.
///
/// Like [`super::build_core_hb_config`], the
/// execution-artifact contract has to recompute the configuration a frozen
/// producer specification asked for, and it must arrive at the *same* one this
/// runner hands the engine. It used to keep its own copy of this function,
/// including its own literal `100` Newton iterations, so every control this
/// request learned was a field the two copies could disagree about — and a
/// disagreement there refuses a converged periodic state as unauthenticated.
pub fn build_core_pss_config(config: &PssRunConfig) -> PssConfig {
    let mut pss_config = if config.oscillator_mode {
        PssConfig::autonomous().with_period_guess(1.0 / config.fundamental_freq)
    } else {
        PssConfig::new(config.fundamental_freq)
    }
    // Core shooting requires at least one harmonic for its internal result
    // schema. A requested retention count of zero remains exact in the
    // service identity and yields an empty public harmonic payload.
    .with_harmonics(config.num_harmonics.max(1))
    .with_tolerance(config.tolerance)
    // The literal `100` that stood here is gone: the request carries the
    // limit, and the form and the deck can both state it.
    .with_max_iterations(config.max_iterations)
    .with_tstab_periods(config.tstab_periods)
    .with_tstab(config.tstab)
    .with_damping(config.damping)
    .with_verbose(config.verbose)
    .with_points_per_period(config.points_per_period);
    if let Some(node) = config.oscillator_node.as_deref() {
        pss_config = pss_config.with_oscillator_node(node);
    }
    // Assigned rather than built: `PssConfig` has no builder for these three.
    // The method is an `Option` whose `None` is the engine's own choice, which
    // no `with_` call can express, and the other two simply never grew one.
    pss_config.integration_method = config.integration_method;
    pss_config.abstol = config.abstol;
    pss_config.max_period_change = config.max_period_change;
    pss_config
}

pub fn validate_pss_config(config: &PssRunConfig) -> Result<(), String> {
    if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
        return Err("PSS fundamental frequency must be positive".to_string());
    }
    if !config.oscillator_mode && config.tone_sources.is_empty() {
        return Err("PSS must bind at least one periodic tone source".to_owned());
    }
    for (index, source) in config.tone_sources.iter().enumerate() {
        if source.trim().is_empty() || source.chars().any(char::is_control) {
            return Err(format!("PSS tone source {} is invalid", index + 1));
        }
        if config.tone_sources[..index]
            .iter()
            .any(|prior| prior.eq_ignore_ascii_case(source))
        {
            return Err(format!("PSS tone source '{source}' is duplicated"));
        }
    }
    if config.points_per_period < 16 {
        return Err("PSS points_per_period must be at least 16".to_owned());
    }
    if config
        .num_harmonics
        .max(1)
        .checked_mul(2)
        .is_none_or(|minimum| config.points_per_period < minimum)
    {
        return Err("PSS points_per_period must be at least twice num_harmonics".to_owned());
    }
    if !config.tolerance.is_finite() || config.tolerance <= 0.0 {
        return Err("PSS tolerance must be positive".to_string());
    }
    // The engine's own bounds, asked before the solve starts rather than
    // after: `PssConfig::validate` refuses the same values, and
    // `with_damping` would silently clamp this one.
    if !config.tstab.is_finite() || config.tstab < 0.0 {
        return Err("PSS tstab must be non-negative".to_owned());
    }
    if config.max_iterations == 0 {
        return Err("PSS max iterations must be at least 1".to_owned());
    }
    if !config.abstol.is_finite() || config.abstol <= 0.0 {
        return Err("PSS abstol must be positive".to_owned());
    }
    if !config.damping.is_finite() || !(0.1..=1.0).contains(&config.damping) {
        return Err("PSS damping must be in [0.1, 1.0]".to_owned());
    }
    if !config.max_period_change.is_finite() || config.max_period_change <= 0.0 {
        return Err("PSS max period change must be positive".to_owned());
    }
    if config.oscillator_mode
        && config
            .oscillator_node
            .as_deref()
            .is_none_or(|node| node.trim().is_empty())
    {
        return Err("PSS oscillator node is required in oscillator mode".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn configured_shooting_fields_reach_the_core_solver_request() {
        let config = PssRunConfig {
            fundamental_freq: 2.0e6,
            tone_sources: vec!["VOSC".to_owned()],
            tstab_periods: 37,
            points_per_period: 1024,
            num_harmonics: 13,
            tolerance: 2.0e-5,
            oscillator_mode: true,
            oscillator_node: Some("osc".to_owned()),
            integration_method: Some(rspice_core::numerics::integration::IntegrationMethod::Gear2),
            tstab: 3.0e-9,
            max_iterations: 250,
            abstol: 1.0e-15,
            damping: 0.75,
            max_period_change: 0.25,
            verbose: true,
        };

        let core = build_core_pss_config(&config);
        assert!(core.auto_period);
        assert_eq!(core.period_guess, 0.5e-6);
        assert_eq!(core.num_harmonics, 13);
        assert_eq!(core.tolerance, 2.0e-5);
        assert_eq!(core.tstab_periods, 37);
        assert_eq!(core.points_per_period, 1024);
        assert_eq!(core.oscillator_node.as_deref(), Some("osc"));
        // Every control the request carries, in the engine's own field. The
        // Newton limit was a literal here until this lane; the rest had no
        // route to the engine at all.
        assert_eq!(core.max_iterations, 250);
        assert_eq!(core.tstab, 3.0e-9);
        assert_eq!(core.abstol, 1.0e-15);
        assert_eq!(core.damping_factor, 0.75);
        assert_eq!(core.max_period_change, 0.25);
        assert!(core.verbose);
        assert_eq!(
            core.integration_method,
            Some(rspice_core::numerics::integration::IntegrationMethod::Gear2)
        );
        // A positive window is the window: the engine's own resolution takes
        // the period count only when the time is zero.
        assert_eq!(core.effective_tstab(), 3.0e-9);
        assert!(core.validate().is_ok(), "{:?}", core.validate());
    }

    #[test]
    fn zero_harmonic_retention_keeps_a_valid_internal_shooting_contract() {
        let config = PssRunConfig {
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VCLK".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            num_harmonics: 0,
            tolerance: 1.0e-7,
            oscillator_mode: false,
            oscillator_node: None,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        };

        validate_pss_config(&config).expect("zero retained harmonics is valid");
        let core = build_core_pss_config(&config);
        assert_eq!(core.num_harmonics, 1);
        assert_eq!(config.num_harmonics, 0);
        assert!(core.validate().is_ok());
    }
}

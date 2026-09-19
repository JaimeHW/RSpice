//! Explicit solver controls for envelope periodic initialization.

use rspice_core::analysis::{HbConfig, pss::PssConfig};
use rspice_core::netlist::HbCard;
use rspice_core::numerics::integration::IntegrationMethod;

/// Portable shooting integration choice, independent of optional core features.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EnvelopeShootingIntegration {
    #[default]
    Automatic,
    BackwardEuler,
    Trapezoidal,
    Gear2,
    TrapGear,
}

impl EnvelopeShootingIntegration {
    fn core(self) -> Option<IntegrationMethod> {
        match self {
            Self::Automatic => None,
            Self::BackwardEuler => Some(IntegrationMethod::BackwardEuler),
            Self::Trapezoidal => Some(IntegrationMethod::Trapezoidal),
            Self::Gear2 => Some(IntegrationMethod::Gear2),
            Self::TrapGear => Some(IntegrationMethod::TrapGear),
        }
    }
}

/// Defaults preserve the original envelope initializer settings.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EnvelopeInitializationConfig {
    pub max_iterations: usize,
    pub reltol: f64,
    pub abstol: f64,
    pub damping: f64,
    pub verbose: bool,
    pub pss_stabilization_periods: usize,
    pub pss_points_per_period: Option<usize>,
    pub pss_integration: EnvelopeShootingIntegration,
    pub hb_min_damping: f64,
    pub hb_oversample: usize,
    pub hb_collocation_points: Option<usize>,
    pub hb_use_krylov: bool,
    pub hb_gmres_restart: usize,
    pub hb_source_stepping: bool,
    pub hb_exact_jacobian: bool,
}

impl Default for EnvelopeInitializationConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            reltol: 1e-6,
            abstol: 1e-12,
            damping: 1.0,
            verbose: false,
            pss_stabilization_periods: 20,
            pss_points_per_period: None,
            pss_integration: EnvelopeShootingIntegration::Automatic,
            hb_min_damping: 0.01,
            hb_oversample: 2,
            hb_collocation_points: None,
            hb_use_krylov: false,
            hb_gmres_restart: 30,
            hb_source_stepping: false,
            hb_exact_jacobian: true,
        }
    }
}

impl EnvelopeInitializationConfig {
    pub(crate) fn pss_config(&self, frequency: f64, harmonics: usize) -> Result<PssConfig, String> {
        if !frequency.is_finite() || frequency <= 0.0 {
            return Err("Envelope initialization frequency must be finite and positive".into());
        }
        let mut config = PssConfig::new(frequency).with_harmonics(harmonics);
        config.max_iterations = self.max_iterations;
        config.tolerance = self.reltol;
        config.abstol = self.abstol;
        config.damping_factor = self.damping;
        config.verbose = self.verbose;
        config.tstab_periods = self.pss_stabilization_periods;
        config.tstab = self.pss_stabilization_periods as f64 / frequency;
        config.points_per_period = self
            .pss_points_per_period
            .unwrap_or_else(|| harmonics.saturating_mul(16).max(256));
        config.integration_method = self.pss_integration.core();
        config.validate()?;
        Ok(config)
    }

    pub(crate) fn hb_config(&self, frequency: f64, harmonics: usize) -> Result<HbConfig, String> {
        // The same core card resolver validates the grid and every solver knob
        // for standalone HB and envelope initialization.
        HbConfig::from_hb_card(
            &HbCard {
                frequencies: vec![frequency],
                harmonics: vec![harmonics],
                automatic_collocation: self.hb_collocation_points.is_none(),
                max_iterations: Some(self.max_iterations),
                reltol: Some(self.reltol),
                abstol: Some(self.abstol),
                damping: Some(self.damping),
                verbose: Some(self.verbose),
                min_damping: Some(self.hb_min_damping),
                oversample: Some(self.hb_oversample),
                collocation_points: self.hb_collocation_points,
                use_krylov: Some(self.hb_use_krylov),
                gmres_restart: Some(self.hb_gmres_restart),
                source_stepping: Some(self.hb_source_stepping),
                use_exact_jacobian: Some(self.hb_exact_jacobian),
                ..Default::default()
            },
            &Default::default(),
        )
        .map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
        EnvelopeRunConfig, run_envelope_analysis_with_source_path_and_abort,
    };
    use super::*;
    use rspice_core::NoAbort;

    #[test]
    fn envelope_initializer_defaults_preserve_both_core_configurations() {
        let controls = EnvelopeInitializationConfig::default();
        let hb = HbConfig::new(1e6).with_harmonics(3);
        assert_eq!(controls.hb_config(1e6, 3).unwrap(), hb);
        let mut pss = PssConfig::new(1e6).with_harmonics(3).with_tstab(20e-6);
        pss.points_per_period = 256;
        // With an explicit nonzero stabilization time the legacy fallback
        // period count had no effect. The new count must also support zero.
        pss.tstab_periods = 20;
        assert_eq!(controls.pss_config(1e6, 3).unwrap(), pss);
        let mut zero = controls;
        zero.pss_stabilization_periods = 0;
        assert_eq!(zero.pss_config(1e6, 3).unwrap().effective_tstab(), 0.0);
    }

    #[test]
    fn envelope_initializer_rejects_invalid_active_solver_controls() {
        let mut cfg = EnvelopeInitializationConfig::default();
        cfg.hb_collocation_points = Some(4);
        assert!(cfg.hb_config(1e6, 3).is_err());
        assert!(cfg.pss_config(1e6, 3).is_ok());
        cfg.hb_collocation_points = Some(7);
        assert!(cfg.hb_config(1e6, 3).is_ok());
        cfg.hb_gmres_restart = 65;
        assert!(cfg.hb_config(1e6, 3).is_err());
        cfg.pss_points_per_period = Some(16);
        assert!(cfg.pss_config(1e6, 9).is_err());
        cfg.pss_points_per_period = None;
        cfg.max_iterations = 0;
        assert!(cfg.hb_config(1e6, 3).is_err());
        assert!(cfg.pss_config(1e6, 3).is_err());
    }

    #[test]
    fn envelope_initializer_controls_reach_both_solvers_and_pss_obeys_its_budget() {
        let deck = "Initializer controls\nV1 in mod SIN(0 1 1Meg)\nVmod mod 0 PWL(0 0 10u 0)\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        for method in [
            EnvelopeInitialPeriodicSolve::HarmonicBalance,
            EnvelopeInitialPeriodicSolve::PeriodicSteadyState,
        ] {
            let mut config = EnvelopeRunConfig {
                initialization: EnvelopeInitializationConfig {
                    pss_stabilization_periods: 0,
                    pss_points_per_period: Some(64),
                    damping: 0.5,
                    hb_collocation_points: Some(17),
                    hb_oversample: 4,
                    hb_gmres_restart: 12,
                    hb_use_krylov: true,
                    ..Default::default()
                },
                fundamental_freq: 1e6,
                additional_carrier_tones: vec![],
                stop_time: 4e-6,
                num_harmonics: 3,
                envelope_step: Some(0.5e-6),
                modulation_sources: vec!["Vmod".into()],
                initial_periodic_solve: method,
                adaptive_mode: EnvelopeAdaptiveMode::FixedEnvelopeStep,
                extraction_path: EnvelopeExtractionPath::Projection,
            };
            let completed =
                run_envelope_analysis_with_source_path_and_abort(deck, &config, None, &NoAbort)
                    .unwrap();
            let initialization = completed
                .convergence
                .as_ref()
                .unwrap()
                .initialization
                .as_ref()
                .unwrap();
            let omega_rc = 2.0 * std::f64::consts::PI;
            let expected = num_complex::Complex64::new(-omega_rc, -1.0) / (1.0 + omega_rc.powi(2));
            let out = &completed
                .waveforms
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("ENV(V(out))"))
                .unwrap()
                .1;
            assert!(
                out.iter().all(|value| (*value - expected).norm() < 1e-3),
                "{method:?}: {out:?}"
            );
            if method == EnvelopeInitialPeriodicSolve::PeriodicSteadyState {
                assert!(initialization.solver_iterations > 1, "{initialization:?}");
                config.initialization.max_iterations = 1;
                let failed =
                    run_envelope_analysis_with_source_path_and_abort(deck, &config, None, &NoAbort);
                assert!(failed.is_err(), "PSS ignored the one-iteration budget");
            } else {
                // The currently supported linear HB envelope circuits admit an
                // exact seed and therefore legitimately need zero corrections.
                assert_eq!(initialization.solver_iterations, 0);
            }
        }
    }
}

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
    /// Positive seconds override the cycle count; zero keeps the cycle policy.
    #[serde(skip_serializing_if = "is_zero")]
    pub pss_stabilization_time: f64,
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
            pss_stabilization_time: 0.0,
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
    pub(crate) fn pss_config(
        &self,
        carriers: &[f64],
        harmonics: usize,
    ) -> Result<PssConfig, String> {
        let (frequency, harmonics) = periodic_basis(carriers, harmonics)?;
        let mut config = PssConfig::new(frequency).with_harmonics(harmonics);
        config.max_iterations = self.max_iterations;
        config.tolerance = self.reltol;
        config.abstol = self.abstol;
        config.damping_factor = self.damping;
        config.verbose = self.verbose;
        config.tstab_periods = self.pss_stabilization_periods;
        if !self.pss_stabilization_time.is_finite() || self.pss_stabilization_time < 0.0 {
            return Err("Envelope stabilization time must be finite and nonnegative".into());
        }
        config.tstab = if self.pss_stabilization_time > 0.0 {
            self.pss_stabilization_time
        } else {
            self.pss_stabilization_periods as f64 / frequency
        };
        config.points_per_period = self
            .pss_points_per_period
            .unwrap_or_else(|| harmonics.saturating_mul(16).max(256));
        config.integration_method = self.pss_integration.core();
        config.validate()?;
        Ok(config)
    }

    pub(crate) fn hb_config(&self, carriers: &[f64], harmonics: usize) -> Result<HbConfig, String> {
        let (frequency, harmonics) = periodic_basis(carriers, harmonics)?;
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

fn is_zero(value: &f64) -> bool {
    *value == 0.0
}

/// Keep the authored spectral ceiling (first carrier × harmonic order), while
/// allowing all carriers to share a lower fundamental. Existing harmonic tone
/// lists keep their exact basis and solver configuration. The core HB resolver
/// supplies the common basis and bounded harmonic count for other lists.
fn periodic_basis(carriers: &[f64], harmonics: usize) -> Result<(f64, usize), String> {
    let Some(&first) = carriers.first() else {
        return Err("Envelope initialization requires at least one carrier".into());
    };
    let mut seen = std::collections::HashSet::new();
    for &carrier in carriers {
        if !carrier.is_finite() || carrier <= 0.0 {
            return Err("Envelope carrier tones must be finite and positive".into());
        }
        if !seen.insert(carrier.to_bits()) {
            return Err("Envelope carrier tones must be unique".into());
        }
    }
    if harmonics == 0 {
        return Err("Envelope harmonic order must be positive".into());
    }
    let ceiling = first * harmonics as f64;
    if !ceiling.is_finite() {
        return Err("Envelope periodic bandwidth is not representable".into());
    }
    if carriers.iter().any(|&carrier| {
        carrier > ceiling && carrier - ceiling > 128.0 * f64::EPSILON * carrier.max(ceiling)
    }) {
        return Err(format!(
            "Envelope carriers must fit within the first carrier's harmonic order ({ceiling} Hz); increase the harmonic order"
        ));
    }
    let closes = |carrier: f64, basis: f64| {
        let ratio = carrier / basis;
        ratio.is_finite()
            && ratio.round() >= 1.0
            && (ratio - ratio.round()).abs() <= 128.0 * f64::EPSILON * ratio.abs().max(1.0)
    };
    if carriers.iter().all(|&carrier| closes(carrier, first)) {
        return Ok((first, harmonics));
    }
    let mut orders = vec![1; carriers.len()];
    orders[0] = harmonics;
    let resolved = HbConfig::from_hb_card(
        &HbCard {
            frequencies: carriers.to_vec(),
            harmonics: orders,
            automatic_collocation: true,
            ..Default::default()
        },
        &Default::default(),
    )
    .map_err(|error| format!("Envelope periodic carrier basis: {error}"))?;
    if !carriers
        .iter()
        .all(|&carrier| closes(carrier, resolved.fundamental_freq))
    {
        return Err("Envelope carriers do not close over a representable common period".into());
    }
    Ok((resolved.fundamental_freq, resolved.num_harmonics))
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
    fn envelope_common_period_preserves_bandwidth_and_qualifies_the_grid() {
        let controls = EnvelopeInitializationConfig::default();
        for scale in [1e-9, 1.0, 1e9] {
            for (tones, order, basis, resolved_order) in [
                ([2.0, 3.0], 3, 1.0, 6),
                ([3.0, 2.0], 3, 1.0, 9),
                ([2.0, 4.0], 3, 2.0, 3),
                ([9.0, 8.0], 2, 1.0, 18),
            ] {
                let tones = tones.map(|frequency| frequency * scale);
                let hb = controls.hb_config(&tones, order).unwrap();
                let pss = controls.pss_config(&tones, order).unwrap();
                assert!((hb.fundamental_freq / (basis * scale) - 1.0).abs() < 1e-12);
                assert_eq!(hb.num_harmonics, resolved_order);
                assert_eq!(pss.fundamental_freq, hb.fundamental_freq);
                assert_eq!(pss.num_harmonics, hb.num_harmonics);
                assert!((pss.effective_tstab() * pss.fundamental_freq - 20.0).abs() < 1e-12);
            }
        }
        // Exact old configurations, including an almost integral input, keep
        // their basis rather than acquiring a spurious, very long beat period.
        assert_eq!(
            controls.hb_config(&[1e6, 2e6_f64.next_up()], 2).unwrap(),
            controls.hb_config(&[1e6], 2).unwrap()
        );
        assert!(
            controls
                .hb_config(&[2e3, 3e3], 1)
                .unwrap_err()
                .contains("harmonic order")
        );
        assert!(controls.hb_config(&[1e3, 1e3 * 2.0_f64.sqrt()], 3).is_err());
        let mut short_grid = controls;
        short_grid.hb_collocation_points = Some(7);
        short_grid.pss_points_per_period = Some(16);
        assert!(short_grid.hb_config(&[3e3, 2e3], 3).is_err());
        assert!(short_grid.pss_config(&[3e3, 2e3], 3).is_err());
    }

    #[test]
    fn envelope_commensurate_carriers_initialize_and_continue_both_solvers() {
        let deck = "Two carrier initializer\nV1 in mid SIN(0 0.8 2k)\nV2 mid mod SIN(0 0.3 3k)\nVmod mod 0 PWL(0 0.1 10m 0.1)\nR1 in out 1k\nC1 out 0 10n\n.save V(out)\n.end\n";
        for method in [
            EnvelopeInitialPeriodicSolve::HarmonicBalance,
            EnvelopeInitialPeriodicSolve::PeriodicSteadyState,
        ] {
            // Exercise the actual form-to-request conversion before dispatch.
            let mut dialog = crate::simulation::dialog::EnvelopeDialogState::from_config(
                &crate::simulation::dialog::envelope::EnvelopeConfig::default(),
            );
            dialog.carrier_tones = "2k, 3k".into();
            dialog.harmonic_order = "3".into();
            dialog.stop_time = "4m".into();
            dialog.envelope_step = "0.5m".into();
            dialog.modulation_sources = "Vmod".into();
            dialog.initial_periodic_solve_idx =
                usize::from(method == EnvelopeInitialPeriodicSolve::PeriodicSteadyState);
            dialog.adaptive_mode_idx = 1;
            dialog.initialization.pss_stabilization_periods = "0".into();
            dialog.initialization.pss_points_per_period = "64".into();
            let authored = dialog.to_config().unwrap();
            let config = EnvelopeRunConfig {
                initialization: authored.initialization,
                fundamental_freq: authored.carrier_tones[0],
                additional_carrier_tones: authored.carrier_tones[1..].to_vec(),
                stop_time: authored.stop_time,
                num_harmonics: authored.harmonic_order as usize,
                envelope_step: Some(authored.envelope_step),
                modulation_sources: authored.modulation_sources,
                initial_periodic_solve: authored.initial_periodic_solve,
                adaptive_mode: authored.adaptive_mode,
                extraction_path: authored.extraction_path,
            };
            let result =
                run_envelope_analysis_with_source_path_and_abort(deck, &config, None, &NoAbort)
                    .unwrap();
            assert_eq!(result.waveforms.len(), 2);
            assert!(result.time.len() >= 5);
            assert!(
                result
                    .convergence
                    .as_ref()
                    .unwrap()
                    .initialization
                    .is_some()
            );
            for ((frequency, amplitude), (name, values)) in
                [(2e3, 0.8), (3e3, 0.3)].into_iter().zip(&result.waveforms)
            {
                assert!(name.eq_ignore_ascii_case(&format!("ENV(V(out)@{frequency:.12e}Hz)")));
                let expected = num_complex::Complex64::new(0.0, -amplitude)
                    / num_complex::Complex64::new(1.0, std::f64::consts::TAU * frequency * 1e-5);
                for actual in values {
                    assert!(
                        (*actual - expected).norm() < 5e-4,
                        "{method:?} {name}: {actual} vs {expected}"
                    );
                }
            }
        }
    }

    #[test]
    fn envelope_initializer_defaults_preserve_both_core_configurations() {
        let controls = EnvelopeInitializationConfig::default();
        let hb = HbConfig::new(1e6).with_harmonics(3);
        assert_eq!(controls.hb_config(&[1e6], 3).unwrap(), hb);
        let mut pss = PssConfig::new(1e6).with_harmonics(3).with_tstab(20e-6);
        pss.points_per_period = 256;
        // With an explicit nonzero stabilization time the legacy fallback
        // period count had no effect. The new count must also support zero.
        pss.tstab_periods = 20;
        assert_eq!(controls.pss_config(&[1e6], 3).unwrap(), pss);
        let mut zero = controls;
        zero.pss_stabilization_periods = 0;
        assert_eq!(zero.pss_config(&[1e6], 3).unwrap().effective_tstab(), 0.0);
        zero.pss_stabilization_periods = 100;
        zero.pss_stabilization_time = 2.5e-6;
        assert_eq!(
            zero.pss_config(&[2e6, 3e6], 3).unwrap().effective_tstab(),
            2.5e-6
        );
    }

    #[test]
    fn envelope_initializer_rejects_invalid_active_solver_controls() {
        let mut cfg = EnvelopeInitializationConfig::default();
        cfg.hb_collocation_points = Some(4);
        assert!(cfg.hb_config(&[1e6], 3).is_err());
        assert!(cfg.pss_config(&[1e6], 3).is_ok());
        cfg.hb_collocation_points = Some(7);
        assert!(cfg.hb_config(&[1e6], 3).is_ok());
        cfg.hb_gmres_restart = 65;
        assert!(cfg.hb_config(&[1e6], 3).is_err());
        cfg.pss_points_per_period = Some(16);
        assert!(cfg.pss_config(&[1e6], 9).is_err());
        cfg.pss_points_per_period = None;
        for invalid in [-1.0, f64::NAN, f64::INFINITY] {
            cfg.pss_stabilization_time = invalid;
            assert!(cfg.pss_config(&[1e6], 3).is_err());
        }
        cfg.pss_stabilization_time = 0.0;
        cfg.max_iterations = 0;
        assert!(cfg.hb_config(&[1e6], 3).is_err());
        assert!(cfg.pss_config(&[1e6], 3).is_err());
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

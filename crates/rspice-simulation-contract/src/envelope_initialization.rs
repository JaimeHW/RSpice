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
    pub fn pss_config(&self, carriers: &[f64], harmonics: usize) -> Result<PssConfig, String> {
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

    pub fn hb_config(&self, carriers: &[f64], harmonics: usize) -> Result<HbConfig, String> {
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

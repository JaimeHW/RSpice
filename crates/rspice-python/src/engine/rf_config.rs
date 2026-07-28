//! Periodic and harmonic-balance configuration builders.
//!
//! PSS and HB take considerably more setup than the small-signal analyses:
//! harmonic orders, collocation grids, tone sets, and continuation windows all
//! have to be resolved and checked against each other before a run starts.
//! Keeping that here leaves the entry points in `mod.rs` readable as an API
//! surface rather than as numerics.

use super::*;

pub(super) fn resolve_hb_harmonic_orders(
    tone_count: usize,
    requested: Option<&[usize]>,
    context: &str,
) -> PyResult<Vec<usize>> {
    let Some(requested) = requested else {
        return Ok(vec![9; tone_count]);
    };
    if requested.is_empty() {
        return Err(crate::errors::value_error(format!(
            "{context} harmonic orders must not be empty"
        )));
    }
    if requested.contains(&0) {
        return Err(crate::errors::value_error(format!(
            "{context} harmonic orders must all be at least 1"
        )));
    }
    match requested.len() {
        1 => Ok(vec![requested[0]; tone_count]),
        count if count == tone_count => Ok(requested.to_vec()),
        count => Err(crate::errors::value_error(format!(
            "{context} has {tone_count} tones but {count} harmonic orders; provide one order to broadcast or one per tone"
        ))),
    }
}

pub(super) fn hb_config_from_tones(
    frequencies: &[f64],
    harmonic_orders: &[usize],
    source_names: Option<&[String]>,
) -> PyResult<HbConfig> {
    if frequencies.is_empty() {
        return Err(crate::errors::value_error(
            "HB requires at least one tone frequency",
        ));
    }
    if harmonic_orders.len() != frequencies.len() {
        return Err(crate::errors::value_error(
            "HB requires exactly one harmonic order per tone",
        ));
    }
    if let Some(names) = source_names
        && names.len() != frequencies.len()
    {
        return Err(crate::errors::value_error(format!(
            "HB has {} tones but {} source names",
            frequencies.len(),
            names.len()
        )));
    }

    let mut unique = std::collections::BTreeSet::new();
    let mut tones = Vec::with_capacity(frequencies.len());
    for (index, (&frequency, &order)) in frequencies.iter().zip(harmonic_orders).enumerate() {
        if !frequency.is_finite() || frequency <= 0.0 {
            return Err(crate::errors::value_error(format!(
                "HB tone frequency at index {index} must be positive and finite, got {frequency}"
            )));
        }
        if order == 0 {
            return Err(crate::errors::value_error(format!(
                "HB harmonic order at index {index} must be at least 1"
            )));
        }
        if !unique.insert(frequency.to_bits()) {
            return Err(crate::errors::value_error(format!(
                "HB tone frequency {frequency} is listed more than once"
            )));
        }
        let mut tone = HbTone::new(frequency, order).with_name(format!("tone{}", index + 1));
        if let Some(source_name) = source_names
            .and_then(|names| names.get(index))
            .map(String::as_str)
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            tone = tone.with_source(source_name);
        }
        tones.push(tone);
    }

    if tones.len() == 1 && tones[0].source_name.is_none() {
        Ok(HbConfig::new(tones[0].frequency).with_harmonics(tones[0].num_harmonics))
    } else {
        Ok(HbConfig::multi_tone(tones))
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn configure_hb_numerics(
    config: &mut HbConfig,
    tolerance: f64,
    abstol: f64,
    max_iterations: usize,
    damping: f64,
    min_damping: f64,
    oversample: usize,
    collocation_points: Option<usize>,
    max_mixing_order: usize,
    use_krylov: bool,
    gmres_restart: usize,
    source_stepping: bool,
    use_exact_jacobian: bool,
    verbose: bool,
) -> PyResult<()> {
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(crate::errors::value_error(format!(
            "tolerance must be positive and finite, got {tolerance}"
        )));
    }
    if !abstol.is_finite() || abstol <= 0.0 {
        return Err(crate::errors::value_error(format!(
            "abstol must be positive and finite, got {abstol}"
        )));
    }
    if max_iterations == 0 {
        return Err(crate::errors::value_error(
            "max_iterations must be at least 1",
        ));
    }
    if !damping.is_finite() || !(0.1..=1.0).contains(&damping) {
        return Err(crate::errors::value_error(format!(
            "damping must be finite and in [0.1, 1.0], got {damping}"
        )));
    }
    if !min_damping.is_finite() || min_damping <= 0.0 || min_damping > damping {
        return Err(crate::errors::value_error(format!(
            "min_damping must be finite, positive, and no greater than damping ({damping}), got {min_damping}"
        )));
    }
    if oversample == 0 {
        return Err(crate::errors::value_error("oversample must be at least 1"));
    }
    if max_mixing_order == 0 {
        return Err(crate::errors::value_error(
            "max_mixing_order must be at least 1",
        ));
    }
    if gmres_restart == 0 {
        return Err(crate::errors::value_error(
            "gmres_restart must be at least 1",
        ));
    }
    if let Some(points) = collocation_points {
        let minimum = config.minimum_collocation_points().ok_or_else(|| {
            crate::errors::value_error("HB harmonic count exceeds the addressable collocation grid")
        })?;
        if points % 2 == 0 || points < minimum {
            return Err(crate::errors::value_error(format!(
                "collocation_points must be odd and at least {minimum}, got {points}"
            )));
        }
    }

    config.tolerance = tolerance;
    config.abstol = abstol;
    config.max_iterations = max_iterations;
    config.damping = damping;
    config.min_damping = min_damping;
    config.oversample_factor = oversample;
    config.collocation_points = collocation_points;
    config.max_mixing_order = max_mixing_order;
    config.use_krylov = use_krylov;
    config.gmres_restart = gmres_restart;
    config.source_stepping = source_stepping;
    config.use_exact_jacobian = use_exact_jacobian;
    config.verbose = verbose;
    Ok(())
}

/// Build and validate a shooting PSS configuration.
///
/// Shared by every PSS entry point so the driven/autonomous rules and the
/// numerical bounds are stated once; a second copy would drift.
#[allow(clippy::too_many_arguments)]
pub(super) fn pss_config(
    fundamental_frequency: Option<f64>,
    harmonics: usize,
    tstab: f64,
    tstab_periods: Option<usize>,
    max_iterations: usize,
    tolerance: f64,
    abstol: f64,
    damping: f64,
    max_period_change: f64,
    points_per_period: usize,
    integration_method: Option<PyIntegrationMethod>,
    autonomous: bool,
    period_guess: Option<f64>,
    verbose: bool,
) -> PyResult<PssConfig> {
    if harmonics == 0 {
        return Err(crate::errors::value_error("harmonics must be at least 1"));
    }
    if let Some(frequency) = fundamental_frequency
        && (!frequency.is_finite() || frequency <= 0.0)
    {
        return Err(crate::errors::value_error(format!(
            "fundamental_frequency must be positive and finite, got {frequency}"
        )));
    }
    if let Some(period) = period_guess
        && (!period.is_finite() || period <= 0.0)
    {
        return Err(crate::errors::value_error(format!(
            "period_guess must be positive and finite, got {period}"
        )));
    }
    if !max_period_change.is_finite() || max_period_change <= 0.0 {
        return Err(crate::errors::value_error(format!(
            "max_period_change must be positive and finite, got {max_period_change}"
        )));
    }

    let mut config = if autonomous {
        PssConfig::autonomous()
    } else {
        let frequency = fundamental_frequency.ok_or_else(|| {
            crate::errors::value_error("fundamental_frequency is required for driven PSS")
        })?;
        PssConfig::new(frequency)
    };
    if autonomous {
        if let Some(period) = period_guess {
            config.period_guess = period;
            config.fundamental_freq = 1.0 / period;
        } else if let Some(frequency) = fundamental_frequency {
            config.period_guess = 1.0 / frequency;
            config.fundamental_freq = frequency;
        }
    }
    config.num_harmonics = harmonics;
    config.tstab = tstab;
    if let Some(periods) = tstab_periods {
        config.tstab_periods = periods;
    }
    config.max_iterations = max_iterations;
    config.tolerance = tolerance;
    config.abstol = abstol;
    config.damping_factor = damping;
    config.max_period_change = max_period_change;
    config.points_per_period = points_per_period;
    config.integration_method = integration_method.map(Into::into);
    config.verbose = verbose;
    config.validate().map_err(|message| {
        crate::errors::value_error(format!("invalid PSS configuration: {message}"))
    })?;
    Ok(config)
}

/// Validate a continuation window shared by the PSS and HB envelope runners.
pub(super) fn continuation_window(duration: f64, max_step: Option<f64>) -> PyResult<(f64, f64)> {
    if !duration.is_finite() || duration <= 0.0 {
        return Err(crate::errors::value_error(format!(
            "duration must be a positive finite number of seconds, got {duration}"
        )));
    }
    if let Some(step) = max_step
        && (!step.is_finite() || step <= 0.0)
    {
        return Err(crate::errors::value_error(format!(
            "max_step must be a positive finite number of seconds, got {step}"
        )));
    }
    Ok((duration, max_step.unwrap_or(duration / 50.0)))
}

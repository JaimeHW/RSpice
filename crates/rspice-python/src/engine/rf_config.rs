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
    match requested {
        [only] => Ok(vec![*only; tone_count]),
        _ if requested.len() == tone_count => Ok(requested.to_vec()),
        orders => Err(crate::errors::value_error(format!(
            "{context} has {tone_count} tones but {} harmonic orders; provide one order to broadcast or one per tone",
            orders.len()
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

    match tones.as_slice() {
        [single] if single.source_name.is_none() => {
            Ok(HbConfig::new(single.frequency).with_harmonics(single.num_harmonics))
        }
        _ => Ok(HbConfig::multi_tone(tones)),
    }
}

/// Everything a direct `.PAC` request states, before validation.
pub(super) struct PacRequest<'a> {
    pub fundamental_frequency: f64,
    pub start_frequency: f64,
    pub stop_frequency: f64,
    pub points: usize,
    pub input_source: &'a str,
    pub output_node: &'a str,
    pub variation: &'a str,
    pub sideband_min: Option<i32>,
    pub sideband_max: i32,
    pub reference_node: Option<&'a str>,
    pub reltol: f64,
    pub abstol: f64,
}

/// Build and validate the periodic-AC configuration a direct request states.
pub(super) fn pac_config(request: PacRequest<'_>) -> PyResult<PacConfig> {
    if !request.fundamental_frequency.is_finite() || request.fundamental_frequency <= 0.0 {
        return Err(crate::errors::value_error(format!(
            "fundamental_frequency must be positive and finite, got {}",
            request.fundamental_frequency
        )));
    }
    if request.input_source.trim().is_empty() {
        return Err(crate::errors::value_error("input_source must not be empty"));
    }
    if request.output_node.trim().is_empty() {
        return Err(crate::errors::value_error("output_node must not be empty"));
    }
    let sweep_type = match request.variation.to_ascii_lowercase().as_str() {
        "dec" | "decade" => PacSweepType::Decade,
        "oct" | "octave" => PacSweepType::Octave,
        "lin" | "linear" => PacSweepType::Linear,
        other => {
            return Err(crate::errors::value_error(format!(
                "variation must be 'dec', 'oct', or 'lin', got '{other}'"
            )));
        }
    };
    let sideband_min = request
        .sideband_min
        .unwrap_or_else(|| PacConfig::default().sideband_min);
    let mut config = PacConfig::new()
        .with_fundamental(request.fundamental_frequency)
        .with_sweep(
            request.start_frequency,
            request.stop_frequency,
            request.points,
        )
        .with_sweep_type(sweep_type)
        .with_sidebands(sideband_min, request.sideband_max)
        .with_tolerances(request.reltol, request.abstol)
        .with_input_source(request.input_source)
        .with_output_node(request.output_node);
    if let Some(reference) = request.reference_node {
        if reference.trim().is_empty() {
            return Err(crate::errors::value_error(
                "reference_node must not be empty",
            ));
        }
        config = config.with_output_ref(reference);
    }
    config.validate().map_err(|message| {
        crate::errors::value_error(format!("invalid PAC configuration: {message}"))
    })?;
    Ok(config)
}

/// The numerical knobs every harmonic-balance entry point exposes.
///
/// The three HB methods take the same thirteen values and hand them to the
/// same validator. Naming that set once keeps the entry points from drifting
/// into thirteen-argument calls that differ by one forgotten field.
pub(super) struct HbNumerics {
    pub tolerance: f64,
    pub abstol: f64,
    pub max_iterations: usize,
    pub damping: f64,
    pub min_damping: f64,
    pub oversample: usize,
    pub collocation_points: Option<usize>,
    pub max_mixing_order: usize,
    pub use_krylov: bool,
    pub gmres_restart: usize,
    pub source_stepping: bool,
    pub use_exact_jacobian: bool,
    pub verbose: bool,
}

/// Build the validated HB configuration a set of tones and knobs describes.
pub(super) fn hb_config(
    frequencies: &[f64],
    harmonic_orders: &[usize],
    source_names: Option<&[String]>,
    numerics: HbNumerics,
) -> PyResult<HbConfig> {
    let mut config = hb_config_from_tones(frequencies, harmonic_orders, source_names)?;
    configure_hb_numerics(&mut config, numerics)?;
    Ok(config)
}

pub(super) fn configure_hb_numerics(config: &mut HbConfig, numerics: HbNumerics) -> PyResult<()> {
    let HbNumerics {
        tolerance,
        abstol,
        max_iterations,
        damping,
        min_damping,
        oversample,
        collocation_points,
        max_mixing_order,
        use_krylov,
        gmres_restart,
        source_stepping,
        use_exact_jacobian,
        verbose,
    } = numerics;
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

/// Build and validate the authored `.PSS` card a direct call describes.
///
/// Every PSS entry point goes through this, and the shooting configuration is
/// then core's own `PssConfig::from(&PssCard)` conversion, so a `.PSS` card in
/// a deck and a `run_pss` call cannot resolve to different configurations.
#[allow(clippy::too_many_arguments)]
pub(super) fn pss_card(
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
) -> PyResult<rspice_core::netlist::PssCard> {
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

    let mut card = if autonomous {
        rspice_core::netlist::PssCard::autonomous()
    } else {
        let frequency = fundamental_frequency.ok_or_else(|| {
            crate::errors::value_error("fundamental_frequency is required for driven PSS")
        })?;
        rspice_core::netlist::PssCard::driven(frequency)
    };
    if autonomous {
        if let Some(period) = period_guess {
            card.period_guess = period;
            card.fundamental_freq = 1.0 / period;
        } else if let Some(frequency) = fundamental_frequency {
            card.period_guess = 1.0 / frequency;
            card.fundamental_freq = frequency;
        }
    }
    card.num_harmonics = harmonics;
    card.tstab = tstab;
    if let Some(periods) = tstab_periods {
        card.tstab_periods = periods;
    }
    card.max_iterations = max_iterations;
    card.tolerance = tolerance;
    card.abstol = abstol;
    card.damping_factor = damping;
    card.max_period_change = max_period_change;
    card.points_per_period = points_per_period;
    card.integration_method = integration_method.map(Into::into);
    card.verbose = verbose;
    PssConfig::from(&card).validate().map_err(|message| {
        crate::errors::value_error(format!("invalid PSS configuration: {message}"))
    })?;
    Ok(card)
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

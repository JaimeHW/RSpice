//! Validation for the periodic steady-state analyses and the small-signal
//! analyses that linearize about one.
//!
//! The same family `runner/spec/periodic.rs` dispatches, with the
//! quasi-periodic four beside it: a lattice of mixing products is the
//! multi-tone spelling of the same sideband index, and the bounds it is
//! refused on are this family's own.

use crate::simulation::multi_run::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeInitialPeriodicSolve, HbToneSpec, PssMethod, SpPort,
};

/// Validate one periodic specification.
pub(super) fn validate(spec: &AnalysisSpec) -> Result<(), String> {
    match spec {
        AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            f2_over_f1,
            ..
        } => {
            if *start_freq <= 0.0 {
                return Err("DISTO start_freq must be > 0".to_string());
            }
            if *stop_freq <= 0.0 {
                return Err("DISTO stop_freq must be > 0".to_string());
            }
            if *stop_freq <= *start_freq {
                return Err("DISTO stop_freq must be > start_freq".to_string());
            }
            if *points_per_unit == 0 {
                return Err("DISTO points_per_unit must be > 0".to_string());
            }
            if let Some(ratio) = f2_over_f1
                && (!ratio.is_finite() || *ratio <= 0.0 || *ratio >= 1.0)
            {
                return Err(
                    "DISTO f2_over_f1 must be finite and strictly between 0 and 1".to_string(),
                );
            }
            Ok(())
        }
        AnalysisSpec::Pss {
            method,
            fundamental_freq,
            tone_sources,
            tstab_periods: _,
            points_per_period,
            tolerance,
            oscillator_mode,
            oscillator_node,
            num_harmonics,
        } => {
            if *method != PssMethod::Shooting {
                return Err(
                    "Legacy HB-PSS mode is not executable; use a Harmonic Balance analysis"
                        .to_owned(),
                );
            }
            if !fundamental_freq.is_finite() || *fundamental_freq <= 0.0 {
                return Err("PSS fundamental_freq must be finite and > 0".to_string());
            }
            if !*oscillator_mode && tone_sources.is_empty() {
                return Err("PSS must bind at least one periodic tone source".to_owned());
            }
            for (index, source) in tone_sources.iter().enumerate() {
                if source.trim().is_empty() || source.chars().any(char::is_control) {
                    return Err(format!("PSS tone source {} is invalid", index + 1));
                }
                if tone_sources[..index]
                    .iter()
                    .any(|prior| prior.eq_ignore_ascii_case(source))
                {
                    return Err(format!("PSS tone source '{source}' is duplicated"));
                }
            }
            if *points_per_period < 16 {
                return Err("PSS points_per_period must be at least 16".to_owned());
            }
            if num_harmonics
                .max(&1)
                .checked_mul(2)
                .is_none_or(|minimum| *points_per_period < minimum)
            {
                return Err("PSS points_per_period must be at least twice num_harmonics".to_owned());
            }
            if !tolerance.is_finite() || *tolerance <= 0.0 {
                return Err("PSS tolerance must be finite and > 0".to_string());
            }
            if *oscillator_mode
                && oscillator_node
                    .as_deref()
                    .is_none_or(|node| node.trim().is_empty())
            {
                return Err("PSS oscillator_node must be set in oscillator mode".to_string());
            }
            Ok(())
        }
        AnalysisSpec::PssSpectrum { num_harmonics } => {
            // A spectrum of nothing is not a result. The PSS request that
            // seeds this one already refuses a count its sample rate
            // cannot resolve, so this only has to reject the empty ask.
            if *num_harmonics == 0 {
                return Err("PSS spectrum must retain at least one harmonic".to_owned());
            }
            Ok(())
        }
        AnalysisSpec::HarmonicBalance {
            tones,
            reltol,
            abstol,
            max_iterations,
            damping,
            oversample,
            collocation_points,
            max_mixing_order,
            gmres_restart,
            ..
        } => {
            if tones.is_empty() {
                return Err("HB must define at least one tone".to_string());
            }
            for (idx, tone) in tones.iter().enumerate() {
                if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                    return Err(format!("HB tone {} frequency must be > 0", idx + 1));
                }
                if tone.harmonics == 0 {
                    return Err(format!("HB tone {} harmonics must be > 0", idx + 1));
                }
            }
            if !reltol.is_finite() || *reltol <= 0.0 {
                return Err("HB reltol must be > 0".to_string());
            }
            if !abstol.is_finite() || *abstol <= 0.0 {
                return Err("HB abstol must be > 0".to_string());
            }
            if *max_iterations == 0 {
                return Err("HB max_iterations must be > 0".to_string());
            }
            if !damping.is_finite() || *damping <= 0.0 || *damping > 1.0 {
                return Err("HB damping must be in (0, 1]".to_string());
            }
            if *oversample == 0 {
                return Err("HB oversample must be > 0".to_string());
            }
            if let Some(points) = collocation_points {
                if *points == 0 || *points % 2 == 0 {
                    return Err("HB collocation_points must be a positive odd integer".to_string());
                }
                let core_tones = tones
                    .iter()
                    .map(|tone| rspice_core::analysis::HbTone::new(tone.frequency, tone.harmonics))
                    .collect();
                let core_config = rspice_core::analysis::HbConfig::multi_tone(core_tones);
                let minimum = core_config.minimum_collocation_points().ok_or_else(|| {
                    "HB harmonic count exceeds the addressable collocation grid".to_string()
                })?;
                if *points < minimum {
                    return Err(format!(
                        "HB collocation_points must contain at least {minimum} points for the configured tones"
                    ));
                }
            }
            if *max_mixing_order == 0 {
                return Err("HB max_mixing_order must be > 0".to_string());
            }
            if *gmres_restart == 0 {
                return Err("HB gmres_restart must be > 0".to_string());
            }
            Ok(())
        }
        AnalysisSpec::Envelope {
            fundamental_freq,
            additional_carrier_tones,
            stop_time,
            num_harmonics,
            envelope_step,
            modulation_sources,
            initial_periodic_solve,
            adaptive_mode,
            extraction_path: _,
        } => {
            let carrier_tones =
                std::iter::once(fundamental_freq).chain(additional_carrier_tones.iter());
            let mut seen_tones = std::collections::HashSet::new();
            for tone in carrier_tones {
                if !tone.is_finite() || *tone <= 0.0 {
                    return Err("Envelope carrier tones must be finite and > 0".to_string());
                }
                if !seen_tones.insert(tone.to_bits()) {
                    return Err("Envelope carrier tones must be unique".to_string());
                }
            }
            if !stop_time.is_finite() || *stop_time <= 0.0 {
                return Err("Envelope stop_time must be finite and > 0".to_string());
            }
            if *num_harmonics == 0 {
                return Err("Envelope num_harmonics must be > 0".to_string());
            }
            if let Some(step) = envelope_step
                && (!step.is_finite() || *step <= 0.0)
            {
                return Err("Envelope envelope_step must be finite and > 0 when set".to_string());
            }
            if envelope_step.is_some_and(|step| step > *stop_time) {
                return Err("Envelope envelope_step cannot exceed stop_time".to_string());
            }
            let legacy_source_inference = *initial_periodic_solve
                == EnvelopeInitialPeriodicSolve::TransientSpectralEstimate
                && *adaptive_mode == EnvelopeAdaptiveMode::FixedEnvelopeStep;
            if modulation_sources.is_empty() && !legacy_source_inference {
                return Err(
                    "Envelope modulation_sources are required for periodic or adaptive execution"
                        .to_string(),
                );
            }
            // Every source present in either a legacy or current request
            // has a stable, canonical identity.
            let mut seen_sources = std::collections::HashSet::new();
            for source in modulation_sources {
                let trimmed = source.trim();
                if trimmed.is_empty() || trimmed != source {
                    return Err(
                        "Envelope modulation source names must be nonempty and trimmed".to_string(),
                    );
                }
                if !seen_sources.insert(trimmed.to_ascii_lowercase()) {
                    return Err("Envelope modulation source names must be unique".to_string());
                }
            }
            Ok(())
        }
        AnalysisSpec::Qpss {
            tones,
            max_iterations,
            relative_tolerance,
            autonomous,
            oscillator_node,
        } => validate_qpss(
            tones,
            *max_iterations,
            *relative_tolerance,
            *autonomous,
            oscillator_node.as_deref(),
        ),
        AnalysisSpec::Hbsp {
            start_freq,
            stop_freq,
            points_per_unit,
            ports,
            max_sideband,
            mixed_mode,
            noise_parameters,
            ..
        } => {
            validate_periodic_network(
                *start_freq,
                *stop_freq,
                *points_per_unit,
                ports,
                *max_sideband,
            )?;
            if *mixed_mode {
                validate_periodic_mixed_mode_ports(ports)?;
            }
            if *noise_parameters {
                return Err(
                    "HBSP noise parameters require a correlated periodic-noise solve and are not implemented"
                        .to_owned(),
                );
            }
            Ok(())
        }
        AnalysisSpec::Psp {
            start_freq,
            stop_freq,
            points_per_unit,
            ports,
            max_sideband,
            mixed_mode,
            noise_parameters,
            ..
        } => {
            validate_periodic_network(
                *start_freq,
                *stop_freq,
                *points_per_unit,
                ports,
                *max_sideband,
            )?;
            if *mixed_mode {
                validate_periodic_mixed_mode_ports(ports)?;
            }
            if *noise_parameters {
                return Err(
                    "PSP noise parameters require a correlated periodic-noise solve and are not implemented"
                        .to_owned(),
                );
            }
            Ok(())
        }
        AnalysisSpec::Hbnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            output_node,
            input_source,
            max_sideband,
            noise_figure,
            ..
        } => {
            validate_frequency_sweep(*start_freq, *stop_freq, *points_per_unit)?;
            if output_node.trim().is_empty() || input_source.trim().is_empty() {
                return Err("HBNOISE requires an output node and input source".to_owned());
            }
            if *max_sideband == 0 {
                return Err("HBNOISE max_sideband must be > 0".to_owned());
            }
            if *noise_figure {
                return Err(
                    "HBNOISE noise figure requires explicit source impedance and available-noise temperature references"
                        .to_owned(),
                );
            }
            Ok(())
        }
        AnalysisSpec::Qpac {
            start_freq,
            stop_freq,
            points_per_unit,
            input_source,
            output_node,
            ..
        }
        | AnalysisSpec::Qpxf {
            start_freq,
            stop_freq,
            points_per_unit,
            input_source,
            output_node,
            ..
        } => {
            validate_frequency_sweep(*start_freq, *stop_freq, *points_per_unit)?;
            if input_source.trim().is_empty() || output_node.trim().is_empty() {
                return Err(
                    "quasi-periodic transfer requires an input source and output node".to_owned(),
                );
            }
            Ok(())
        }
        AnalysisSpec::Qpnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            output_node,
            input_source,
            lattice_min,
            lattice_max,
            ..
        } => {
            validate_frequency_sweep(*start_freq, *stop_freq, *points_per_unit)?;
            if output_node.trim().is_empty() || input_source.trim().is_empty() {
                return Err("QPNOISE requires an output node and input source".to_owned());
            }
            if lattice_min
                .iter()
                .zip(lattice_max)
                .any(|(min, max)| min > max)
            {
                return Err("QPNOISE lattice minima must not exceed maxima".to_owned());
            }
            Ok(())
        }
        other => Err(super::misrouted_specification("periodic", other)),
    }
}

pub(super) fn validate_frequency_sweep(start: f64, stop: f64, points: usize) -> Result<(), String> {
    if !start.is_finite() || start <= 0.0 || !stop.is_finite() || stop <= start {
        return Err("frequency sweep requires finite 0 < start < stop".to_owned());
    }
    if points == 0 {
        return Err("frequency sweep point count must be > 0".to_owned());
    }
    Ok(())
}

pub(super) fn validate_qpss(
    tones: &[HbToneSpec],
    max_iterations: usize,
    relative_tolerance: f64,
    autonomous: bool,
    oscillator_node: Option<&str>,
) -> Result<(), String> {
    if tones.len() < 2 {
        return Err("QPSS requires at least two tones".to_owned());
    }
    if tones
        .iter()
        .any(|tone| !tone.frequency.is_finite() || tone.frequency <= 0.0 || tone.harmonics == 0)
    {
        return Err(
            "QPSS tones require positive finite frequencies and harmonic orders".to_owned(),
        );
    }
    if max_iterations == 0 || !relative_tolerance.is_finite() || relative_tolerance <= 0.0 {
        return Err("QPSS requires positive iterations and relative tolerance".to_owned());
    }
    if autonomous && oscillator_node.is_none_or(|node| node.trim().is_empty()) {
        return Err("autonomous QPSS requires an oscillator node".to_owned());
    }
    Ok(())
}

pub(super) fn validate_periodic_network(
    start: f64,
    stop: f64,
    points: usize,
    ports: &[SpPort],
    max_sideband: usize,
) -> Result<(), String> {
    validate_frequency_sweep(start, stop, points)?;
    if max_sideband > i32::MAX as usize {
        return Err("periodic network maximum sideband exceeds the engine index range".to_owned());
    }
    for (index, port) in ports.iter().enumerate() {
        if port.node_pos.trim().is_empty() || port.node_neg.trim().is_empty() {
            return Err(format!(
                "periodic network port {} requires both nodes",
                index + 1
            ));
        }
        if port.z0.is_some_and(|z0| !z0.is_finite() || z0 <= 0.0) {
            return Err(format!(
                "periodic network port {} z0 must be > 0",
                index + 1
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_periodic_mixed_mode_ports(ports: &[SpPort]) -> Result<(), String> {
    if !ports.len().is_multiple_of(2) {
        return Err(
            "periodic mixed-mode conversion requires an even number of physical ports paired in declaration order"
                .to_owned(),
        );
    }
    for (pair_index, pair) in ports.chunks_exact(2).enumerate() {
        if let (Some(positive_z0), Some(negative_z0)) = (pair[0].z0, pair[1].z0)
            && positive_z0.to_bits() != negative_z0.to_bits()
        {
            return Err(format!(
                "periodic mixed-mode pair {} has unequal explicit reference impedances ({} and {} ohm)",
                pair_index + 1,
                positive_z0,
                negative_z0
            ));
        }
    }
    Ok(())
}

//! Structural periodicity and continuity of authored source waveforms.
//!
//! Sampling a waveform cannot prove periodicity: a nonperiodic drive can alias
//! on every shooting point. Certify its authored periods and piecewise-linear
//! continuity instead. Uncertified startup prefixes and cancelling combinations
//! of individually nonperiodic sources require a more general constraint model.

use super::*;
use crate::numerics::is_integral_cycle_count as integral_cycles;

fn periodic_linear_points(
    points: &[(Value, Value)],
    period: Value,
    delay: Value,
    repeat_from: Option<Value>,
    require_continuity: bool,
) -> bool {
    let Some(&(first_time, first_value)) = points.first() else {
        return true;
    };
    if points.iter().all(|&(_, value)| value == first_value) {
        return delay <= 0.0 || first_value == 0.0;
    }
    let Some(start) = repeat_from else {
        return false;
    };
    let start = start.max(first_time);
    let &(last, last_value) = points.last().unwrap();
    delay <= 0.0
        && delay + start <= 0.0
        && last > start
        && integral_cycles(period / (last - start))
        && VoltageSources::pwl_time_component::<false>(points, start, 0.0, None) == last_value
        && (!require_continuity
            || points.windows(2).all(|pair| {
                let dt = pair[1].0 - pair[0].0;
                dt >= 0.0
                    && (pair[0].1 == pair[1].1
                        || (dt > 0.0 && ((pair[1].1 - pair[0].1) / dt).is_finite()))
            }))
}

impl VoltageSources {
    /// Shortest authored waveform interval after timing defaults and scaling.
    /// A fixed grid must resolve these intervals before sampled convergence
    /// can reveal a pulse that would otherwise fall between both trial meshes.
    pub(super) fn minimum_pss_interval(
        spec: &crate::netlist::SourceSpec,
        context: Option<TransientSourceContext>,
        pwl: Option<&crate::device::pwl_file::PwlWaveform>,
    ) -> Option<Value> {
        use crate::netlist::SourceSpec;
        let minimum = |intervals: &[Value]| {
            intervals
                .iter()
                .copied()
                .filter(|value| *value > 0.0 && value.is_finite())
                .reduce(Value::min)
        };
        match spec {
            SourceSpec::Distortion { inner, .. }
            | SourceSpec::DcTransient {
                transient: inner, ..
            }
            | SourceSpec::DcAcTransient {
                transient: inner, ..
            }
            | SourceSpec::RfPort { inner, .. } => Self::minimum_pss_interval(inner, context, pwl),
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                width_defaults_to_zero,
                ..
            } if v1 != v2 => {
                let (_, rise, fall, width, period) = Self::resolve_pulse_timing(
                    *delay,
                    *rise,
                    *fall,
                    *width,
                    *period,
                    *width_defaults_to_zero,
                    context,
                );
                if rise <= 0.0 && fall <= 0.0 && width <= 0.0 {
                    return None;
                }
                minimum(&[rise, fall, width, period - rise - width - fall])
            }
            SourceSpec::Pwl { points, .. } => {
                crate::numerics::minimum_pwl_interval(points.iter().copied())
            }
            SourceSpec::PwlFile { .. } => {
                pwl.and_then(crate::device::pwl_file::PwlWaveform::minimum_segment_duration)
            }
            SourceSpec::Pat {
                vhi,
                vlo,
                rise,
                fall,
                sample,
                data,
                ..
            } if vhi != vlo => {
                let (first, _, _) = Self::pat_data_shape(data)?;
                if data.as_bytes()[1..].iter().all(|&bit| bit == first) {
                    return None;
                }
                let mut previous: Option<Value> = None;
                let mut interval: Option<Value> = None;
                Self::visit_pat_points(*vhi, *vlo, *rise, *fall, *sample, data, |time, _| {
                    if let Some(last) = previous {
                        let dt = time - last;
                        if dt > 0.0 && dt.is_finite() {
                            interval = Some(interval.map_or(dt, |value| value.min(dt)));
                        }
                    }
                    previous = Some(time);
                });
                interval
            }
            _ => None,
        }
    }

    /// Fastest authored sinusoidal clock, measured in cycles per PSS period.
    /// This is a necessary sampling bound, not a bandwidth or integration-error
    /// certificate: modulation and nonlinear devices can generate more harmonics.
    pub(super) fn max_authored_tone_cycles(
        spec: &crate::netlist::SourceSpec,
        period: Value,
        context: Option<TransientSourceContext>,
    ) -> Value {
        use crate::netlist::SourceSpec;
        match spec {
            SourceSpec::Distortion { inner, .. }
            | SourceSpec::DcTransient {
                transient: inner, ..
            }
            | SourceSpec::DcAcTransient {
                transient: inner, ..
            } => Self::max_authored_tone_cycles(inner, period, context),
            SourceSpec::RfPort { inner, port } => {
                let tone = port.drive_tone().map_or(0.0, |(amplitude, frequency, _)| {
                    if amplitude == 0.0 {
                        0.0
                    } else {
                        frequency.abs() * period
                    }
                });
                tone.max(Self::max_authored_tone_cycles(inner, period, context))
            }
            SourceSpec::Sin {
                amplitude,
                frequency,
                ..
            } => {
                if *amplitude == 0.0 {
                    0.0
                } else {
                    Self::resolve_sin_frequency(*frequency, context).abs() * period
                }
            }
            SourceSpec::Sffm {
                amplitude,
                carrier_freq,
                signal_freq,
                modulation_index,
                ..
            } => {
                if *amplitude == 0.0 {
                    return 0.0;
                }
                let (carrier, signal, modulation) =
                    Self::sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, context);
                carrier
                    .abs()
                    .max(if modulation == 0.0 { 0.0 } else { signal.abs() })
                    * period
            }
            SourceSpec::Am {
                modulation_offset,
                modulation_amplitude,
                modulating_freq,
                carrier_freq,
                ..
            } => {
                if *modulation_offset == 0.0 && *modulation_amplitude == 0.0 {
                    return 0.0;
                }
                let (signal, carrier) =
                    Self::am_frequencies(*modulating_freq, *carrier_freq, context);
                // AM multiplies two sinusoids, so its highest authored tone
                // is the sum sideband, not merely the faster constituent.
                (carrier.abs()
                    + if *modulation_amplitude == 0.0 {
                        0.0
                    } else {
                        signal.abs()
                    })
                    * period
            }
            SourceSpec::Dc(_)
            | SourceSpec::Ac { .. }
            | SourceSpec::DcAc { .. }
            | SourceSpec::Pulse { .. }
            | SourceSpec::Exp { .. }
            | SourceSpec::Pwl { .. }
            | SourceSpec::PwlFile { .. }
            | SourceSpec::Pat { .. }
            | SourceSpec::TrNoise { .. }
            | SourceSpec::TrRandom { .. } => 0.0,
        }
    }

    /// Autonomous shooting repeats the authored [0, T] quiet window. A
    /// startup kick later than T still acts during stabilization, and is
    /// restored at its authored time by subsequent transient continuation.
    /// Certify that window structurally, including its outgoing endpoint;
    /// an endpoint-only comparison could miss an entire pulse.
    pub(super) fn constant_waveform_over_orbit(
        spec: &crate::netlist::SourceSpec,
        period: Value,
        context: Option<TransientSourceContext>,
        pwl: Option<&crate::device::pwl_file::PwlWaveform>,
    ) -> bool {
        use crate::netlist::SourceSpec;
        match spec {
            SourceSpec::Distortion { inner, .. }
            | SourceSpec::DcTransient {
                transient: inner, ..
            }
            | SourceSpec::DcAcTransient {
                transient: inner, ..
            } => Self::constant_waveform_over_orbit(inner, period, context, pwl),
            SourceSpec::RfPort { inner, port } => {
                port.drive_tone()
                    .is_none_or(|(amplitude, frequency, _)| amplitude == 0.0 || frequency == 0.0)
                    && Self::constant_waveform_over_orbit(inner, period, context, pwl)
            }
            SourceSpec::Dc(_)
            | SourceSpec::Ac { .. }
            | SourceSpec::DcAc { .. }
            | SourceSpec::TrNoise { .. }
            | SourceSpec::TrRandom { .. } => true,
            SourceSpec::Sin {
                amplitude,
                frequency,
                delay,
                damping,
                ..
            } => {
                *amplitude == 0.0
                    || *delay > period
                    || (*damping == 0.0 && Self::resolve_sin_frequency(*frequency, context) == 0.0)
            }
            SourceSpec::Pulse { v1, v2, delay, .. } => v1 == v2 || *delay > period,
            SourceSpec::Exp {
                v1,
                v2,
                td1,
                tau1,
                td2,
                tau2,
            } => {
                let (td1, _, td2, _) = Self::resolve_exp_timing(*td1, *tau1, *td2, *tau2, context);
                v1 == v2 || (td1 > period && td2 > period)
            }
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => {
                // A repeating profile can wrap before the first changed
                // knot; qualify only the non-repeating prefix here.
                *delay > period
                    || points.first().is_none_or(|&(_, first)| {
                        (*delay <= 0.0 || first == 0.0)
                            && (points.iter().all(|&(_, value)| value == first)
                                || (repeat_from.is_none()
                                    && points.windows(2).all(|pair| {
                                        pair[0].0 + delay > period || pair[0].1 == pair[1].1
                                    })))
                    })
            }
            SourceSpec::PwlFile { delay, .. } => pwl.is_some() && *delay > period,
            SourceSpec::Pat {
                vhi, vlo, delay, ..
            } => vhi == vlo || *delay > period,
            SourceSpec::Sffm {
                offset,
                amplitude,
                delay,
                ..
            } => {
                let xyce = Self::pulse_dialect(context) == crate::config::SpiceDialect::Xyce;
                (!xyce && *delay > period)
                    || (*amplitude == 0.0 && (xyce || *delay < 0.0 || *offset == 0.0))
            }
            SourceSpec::Am {
                offset,
                modulation_offset,
                modulation_amplitude,
                delay,
                ..
            } => {
                *delay > period
                    || (*modulation_offset == 0.0
                        && *modulation_amplitude == 0.0
                        && (*delay < 0.0 || *offset == 0.0))
            }
        }
    }

    pub(super) fn periodic_waveform(
        spec: &crate::netlist::SourceSpec,
        period: Value,
        context: Option<TransientSourceContext>,
        pwl: Option<&crate::device::pwl_file::PwlWaveform>,
        require_continuity: bool,
    ) -> bool {
        use crate::netlist::SourceSpec;
        match spec {
            SourceSpec::Distortion { inner, .. }
            | SourceSpec::DcTransient {
                transient: inner, ..
            }
            | SourceSpec::DcAcTransient {
                transient: inner, ..
            } => Self::periodic_waveform(inner, period, context, pwl, require_continuity),
            SourceSpec::RfPort { inner, port } => {
                Self::periodic_waveform(inner, period, context, pwl, require_continuity)
                    && port.drive_tone().is_none_or(|(amplitude, frequency, _)| {
                        amplitude == 0.0
                            || frequency == 0.0
                            || integral_cycles(frequency.abs() * period)
                    })
            }
            SourceSpec::Dc(_)
            | SourceSpec::Ac { .. }
            | SourceSpec::DcAc { .. }
            | SourceSpec::TrNoise { .. }
            | SourceSpec::TrRandom { .. } => true,
            SourceSpec::Sin {
                amplitude,
                frequency,
                delay,
                damping,
                ..
            } => {
                let frequency = Self::resolve_sin_frequency(*frequency, context);
                *amplitude == 0.0
                    || (*damping == 0.0
                        && (frequency == 0.0
                            || (*delay <= 0.0 && integral_cycles(frequency.abs() * period))))
            }
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period: source_period,
                pulse_count,
                width_defaults_to_zero,
            } => {
                if v1 == v2 {
                    return true;
                }
                let (delay, rise, fall, width, source_period) = Self::resolve_pulse_timing(
                    *delay,
                    *rise,
                    *fall,
                    *width,
                    *source_period,
                    *width_defaults_to_zero,
                    context,
                );
                // A pulse wider than its period jumps at the wrap. A finite
                // train has a stop event and does not repeat indefinitely.
                (!require_continuity
                    || (rise > 0.0 && fall > 0.0 && rise + width + fall <= source_period))
                    && width >= 0.0
                    && (delay <= 0.0 || delay <= source_period - (rise + width + fall))
                    && (pulse_count.is_nan() || *pulse_count <= 0.0)
                    && integral_cycles(period / source_period)
            }
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => periodic_linear_points(points, period, *delay, *repeat_from, require_continuity),
            SourceSpec::PwlFile {
                time_scale,
                time_offset,
                delay,
                repeat_from,
                ..
            } => {
                let Some(pwl) = pwl else { return false };
                if *time_scale <= 0.0 || !pwl.has_finite_segment_slopes() {
                    return false;
                }
                let points: Vec<_> = pwl
                    .scaled_knot_times()
                    .map(|time| (time, pwl.value_at(time)))
                    .collect();
                periodic_linear_points(
                    &points,
                    period,
                    *delay,
                    repeat_from.map(|start| time_offset + time_scale * start),
                    require_continuity,
                )
            }
            SourceSpec::Pat {
                vhi,
                vlo,
                delay,
                rise,
                fall,
                sample,
                data,
                repeat_count,
            } => {
                if vhi == vlo {
                    return true;
                }
                let Some((first, last, count)) = Self::pat_data_shape(data) else {
                    return false;
                };
                if data.as_bytes()[1..].iter().all(|&bit| bit == first) {
                    return true;
                }
                let mut points = Vec::new();
                Self::visit_pat_points(*vhi, *vlo, *rise, *fall, *sample, data, |time, value| {
                    points.push((time, value))
                });
                // The initial half-symbol is a hold; it equals the repeated
                // profile when the end bits agree. Other startup prefixes
                // need an explicit settled source-time origin.
                first == last
                    && *repeat_count < 0
                    && *delay <= 0.0
                    && integral_cycles(period / (count as Value * sample))
                    && periodic_linear_points(
                        &points,
                        period,
                        *delay,
                        Some(0.0),
                        require_continuity,
                    )
            }
            SourceSpec::Exp { v1, v2, .. } => v1 == v2,
            SourceSpec::Sffm {
                amplitude,
                carrier_freq,
                signal_freq,
                modulation_index,
                delay,
                ..
            } => {
                let xyce = Self::pulse_dialect(context) == crate::config::SpiceDialect::Xyce;
                let (fc, fm, mdi) =
                    Self::sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, context);
                (xyce || *delay <= 0.0)
                    && Self::source_time_component::<true>(spec, 0.0, context, pwl).is_finite()
                    && (*amplitude == 0.0
                        || ((fc == 0.0 || integral_cycles(fc.abs() * period))
                            && (mdi == 0.0 || fm == 0.0 || integral_cycles(fm.abs() * period))))
            }
            SourceSpec::Am {
                modulating_freq,
                carrier_freq,
                delay,
                ..
            } => {
                let (fm, fc) = Self::am_frequencies(*modulating_freq, *carrier_freq, context);
                *delay <= 0.0
                    && integral_cycles(fm.abs() * period)
                    && integral_cycles(fc.abs() * period)
                    && Self::source_time_component::<true>(spec, 0.0, context, pwl).is_finite()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_pwl_time_scaling_preserves_periodicity_values_and_slopes() {
        for scale in [1e-30, 1e-18, 1e-12, 1.0, 1e12, 1e30] {
            let points = vec![(0.0, 0.0), (scale, 1.0), (2.0 * scale, 0.0)];
            assert!(
                periodic_linear_points(&points, 2.0 * scale, 0.0, Some(0.0), true),
                "scale={scale:e}"
            );
            let file_waveform =
                crate::device::pwl_file::PwlWaveform::new(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)])
                    .unwrap()
                    .with_scaling(scale, 1.0, 0.0, 0.0);
            let file_spec = crate::netlist::SourceSpec::PwlFile {
                path: "owned-pwl-snapshot.csv".to_owned(),
                time_scale: scale,
                value_scale: 1.0,
                time_offset: 0.0,
                value_offset: 0.0,
                delay: 0.0,
                repeat_from: Some(0.0),
            };
            assert!(
                VoltageSources::periodic_waveform(
                    &file_spec,
                    2.0 * scale,
                    None,
                    Some(&file_waveform),
                    true,
                ),
                "scaled PWLFILE at scale={scale:e}"
            );
            for phase in [0.25_f64, 0.75, 1.25, 1.75, 4.25, 4.75, 5.25, 5.75] {
                let local = phase.rem_euclid(2.0);
                let expected = if local < 1.0 { local } else { 2.0 - local };
                let slope = if local < 1.0 { 1.0 } else { -1.0 };
                assert!(
                    (VoltageSources::pwl_time_component::<false>(
                        &points,
                        phase * scale,
                        0.0,
                        Some(0.0)
                    ) - expected)
                        .abs()
                        < 1e-14
                );
                assert!(
                    (VoltageSources::pwl_time_component::<true>(
                        &points,
                        phase * scale,
                        0.0,
                        Some(0.0)
                    ) * scale
                        - slope)
                        .abs()
                        < 1e-14
                );
            }
        }
    }

    #[test]
    fn pwl_interpolation_normalizes_time_before_scaling_voltage() {
        for (duration, amplitude) in [(1e-30, 1e-300), (1e30, 1e300)] {
            let points = [(0.0, 0.0), (duration, amplitude)];
            let actual =
                VoltageSources::pwl_time_component::<false>(&points, duration * 0.5, 0.0, None);
            assert!(
                (actual / amplitude - 0.5).abs() < 1e-14,
                "duration={duration:e}, amplitude={amplitude:e}, value={actual:e}"
            );
        }
    }

    #[test]
    fn inline_pwl_repeat_boundary_has_no_artificial_endpoint_hold() {
        for scale in [1e-30, 1e-18, 1e-12, 1.0, 1e12, 1e30] {
            let points = [(0.0, 1.0), (scale, 2.0)];
            let seam = 2.0 * scale;
            assert_eq!(
                VoltageSources::pwl_time_component::<false>(&points, seam, 0.0, Some(0.0)),
                2.0
            );
            assert!(
                VoltageSources::pwl_time_component::<true>(&points, seam, 0.0, Some(0.0)).is_nan()
            );
            let after = VoltageSources::pwl_time_component::<false>(
                &points,
                seam.next_up(),
                0.0,
                Some(0.0),
            );
            assert!(
                (after - 1.0).abs() < 1e-14,
                "scale={scale:e}, after={after}"
            );
            assert!(
                (VoltageSources::pwl_time_component::<true>(
                    &points,
                    seam.next_up(),
                    0.0,
                    Some(0.0)
                ) * scale
                    - 1.0)
                    .abs()
                    < 1e-14
            );
        }
    }

    #[test]
    fn regular_waveform_admission_covers_phase_corners_modulation_and_startup_prefixes() {
        let period = 1e-6;
        for (spec, regular) in [
            ("SIN(0.2m 1m 1meg 0 0 37)", true),
            ("SIN(0.2m 1m 1meg 0.1u 0 37)", false),
            ("SIN(0.2m 1m 1.00000001meg)", false),
            ("PULSE(0.2m 1m 0.1u 0.1u 0.1u 0.3u 1u)", true),
            ("PULSE(0.2m 1m 0.7u 0.1u 0.1u 0.3u 1u)", false),
            ("PWL(0 0.2m 0.5u 1.2m 0.5u 1.2m 1u 0.2m) R=0", true),
            (
                "PAT(1m 0.2m 0 0.1u 0.1u 0.333333333333333333u b101) R=-1",
                true,
            ),
            ("PAT(1m 0.2m 0 0.1u 0.1u 0.5u b10) R=-1", false),
            ("SFFM(0 1m 2meg 0.3 1meg)", true),
            ("SFFM(0.2m 1m 2meg 0.3 1meg)", false),
            ("AM(0 1m 1m 1meg 2meg)", true),
            ("AM(0 1m 1m 1.25meg 2meg)", false),
        ] {
            let sources = super::super::tests::current_source_with_waveform(spec);
            assert_eq!(
                sources.has_regular_periodic_waveform(0, period),
                regular,
                "{spec}"
            );
            if regular {
                for fraction in [0.0, 0.03459, 0.31233, 0.52347, 0.81234, 1.0] {
                    let time = period * fraction;
                    assert!(
                        (sources.value_at_time(0, time) - sources.value_at_time(0, time + period))
                            .abs()
                            < 1e-15,
                        "{spec}, t={time:e}"
                    );
                    assert!(
                        (sources.right_derivative_at_time(0, time)
                            - sources.right_derivative_at_time(0, time + period))
                        .abs()
                            < 1e-7,
                        "{spec}, derivative at {time:e}"
                    );
                }
            }
        }
    }
}

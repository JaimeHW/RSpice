//! Structural admission for regular periodic prescribed-current waveforms.
//!
//! Sampling a waveform cannot prove periodicity: a nonperiodic drive can alias
//! on every shooting point. Certify its authored periods and piecewise-linear
//! continuity instead. Uncertified startup prefixes and cancelling combinations
//! of individually nonperiodic sources require a more general constraint model.

use super::*;

fn integral_cycles(cycles: Value) -> bool {
    cycles.is_finite()
        && cycles > 0.0
        && (cycles - cycles.round()).abs() <= (32.0 * Value::EPSILON * cycles.max(1.0)).min(1e-10)
        && cycles.round() >= 1.0
}

fn periodic_linear_points(
    points: &[(Value, Value)],
    period: Value,
    delay: Value,
    repeat_from: Option<Value>,
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
        && points.windows(2).all(|pair| {
            let dt = pair[1].0 - pair[0].0;
            dt >= 0.0
                && (pair[0].1 == pair[1].1
                    || (dt > Value::EPSILON && ((pair[1].1 - pair[0].1) / dt).is_finite()))
        })
}

impl VoltageSources {
    pub(super) fn regular_periodic_waveform(
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
            } => Self::regular_periodic_waveform(inner, period, context, pwl),
            SourceSpec::RfPort { inner, port } => {
                Self::regular_periodic_waveform(inner, period, context, pwl)
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
                rise > 0.0
                    && fall > 0.0
                    && width >= 0.0
                    && delay <= source_period - (rise + width + fall)
                    && rise + width + fall <= source_period
                    && (pulse_count.is_nan() || *pulse_count <= 0.0)
                    && integral_cycles(period / source_period)
            }
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => periodic_linear_points(points, period, *delay, *repeat_from),
            SourceSpec::PwlFile {
                time_scale,
                time_offset,
                delay,
                repeat_from,
                ..
            } => {
                let Some(pwl) = pwl else { return false };
                if *time_scale <= Value::EPSILON || !pwl.has_finite_segment_slopes() {
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
                    && periodic_linear_points(&points, period, *delay, Some(0.0))
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

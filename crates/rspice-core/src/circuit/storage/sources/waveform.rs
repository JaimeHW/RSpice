//! Shared waveform evaluation and analytic right-hand time derivatives.

use super::*;

impl VoltageSources {
    /// Detect unresolved analysis defaults through the same parameter
    /// resolvers used by evaluation. Compare parameters, never sampled
    /// waveform values (which could alias at the chosen times).
    pub(super) fn source_needs_time_basis(
        spec: &crate::netlist::SourceSpec,
        dialect: crate::config::SpiceDialect,
    ) -> bool {
        use crate::netlist::SourceSpec;
        let contexts = [(1.0, 1.0), (2.0, 3.0)].map(|(tstep, tstop)| {
            Some(TransientSourceContext {
                tstep,
                tstop,
                dialect,
                xyce_breakpoint_tolerance: Some(0.0),
                resource_limits: crate::resource::ResourceLimits::default(),
            })
        });
        match spec {
            SourceSpec::Distortion { inner, .. }
            | SourceSpec::RfPort { inner, .. }
            | SourceSpec::DcTransient {
                transient: inner, ..
            }
            | SourceSpec::AcTransient {
                transient: inner, ..
            }
            | SourceSpec::DcAcTransient {
                transient: inner, ..
            } => Self::source_needs_time_basis(inner, dialect),
            SourceSpec::Pulse {
                delay,
                rise,
                fall,
                width,
                period,
                width_defaults_to_zero,
                ..
            } => {
                let resolved = contexts.map(|context| {
                    Self::resolve_pulse_timing(
                        *delay,
                        *rise,
                        *fall,
                        *width,
                        *period,
                        *width_defaults_to_zero,
                        context,
                    )
                });
                resolved[0] != resolved[1]
            }
            SourceSpec::Sin { frequency, .. } => {
                Self::resolve_sin_frequency(*frequency, contexts[0])
                    != Self::resolve_sin_frequency(*frequency, contexts[1])
            }
            SourceSpec::Exp {
                td1,
                tau1,
                td2,
                tau2,
                ..
            } => {
                let resolved = contexts
                    .map(|context| Self::resolve_exp_timing(*td1, *tau1, *td2, *tau2, context));
                resolved[0] != resolved[1]
            }
            SourceSpec::Sffm {
                carrier_freq,
                modulation_index,
                signal_freq,
                ..
            } => {
                let resolved = contexts.map(|context| {
                    Self::sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, context)
                });
                resolved[0] != resolved[1]
            }
            SourceSpec::Am {
                modulating_freq,
                carrier_freq,
                ..
            } => {
                let resolved = contexts
                    .map(|context| Self::am_frequencies(*modulating_freq, *carrier_freq, context));
                resolved[0] != resolved[1]
            }
            SourceSpec::Dc(_)
            | SourceSpec::Ac { .. }
            | SourceSpec::DcAc { .. }
            | SourceSpec::Pwl { .. }
            | SourceSpec::PwlFile { .. }
            | SourceSpec::Pat { .. }
            | SourceSpec::TrNoise { .. }
            | SourceSpec::TrRandom { .. } => false,
        }
    }

    pub(super) fn sffm_parameters(
        carrier_freq: Value,
        modulation_index: Value,
        signal_freq: Value,
        context: Option<TransientSourceContext>,
    ) -> (Value, Value, Value) {
        if Self::pulse_dialect(context) == crate::config::SpiceDialect::Xyce {
            return (
                if carrier_freq.is_finite() {
                    carrier_freq
                } else {
                    Self::xyce_modulated_frequency_default(context)
                },
                if signal_freq.is_finite() {
                    signal_freq
                } else {
                    Self::xyce_modulated_frequency_default(context)
                },
                if modulation_index.is_finite() {
                    modulation_index
                } else {
                    0.0
                },
            );
        }
        let fc = if carrier_freq.is_finite() && carrier_freq > 0.0 {
            carrier_freq
        } else {
            Self::modulated_frequency_default(5.0, context)
        };
        let fm = if signal_freq.is_finite() && signal_freq != 0.0 {
            signal_freq
        } else {
            Self::modulated_frequency_default(500.0, context)
        };
        // ngspice limits MDI with an if/else-if chain, not a symmetric
        // clamp: a negative FM makes FC/FM negative, and any MDI above
        // that ratio lands on the ratio itself. clamp(0.0, ratio) panics.
        let ratio = fc / fm;
        let mdi = if !modulation_index.is_finite() {
            90.0_f64.min(ratio)
        } else if modulation_index > ratio {
            ratio
        } else if modulation_index < 0.0 {
            0.0
        } else {
            modulation_index
        };
        (fc, fm, mdi)
    }

    pub(super) fn am_frequencies(
        modulating_freq: Value,
        carrier_freq: Value,
        context: Option<TransientSourceContext>,
    ) -> (Value, Value) {
        let fm = if modulating_freq.is_finite() && modulating_freq > 0.0 {
            modulating_freq
        } else {
            Self::modulated_frequency_default(5.0, context)
        };
        let fc = if carrier_freq.is_finite() && carrier_freq > 0.0 {
            carrier_freq
        } else {
            Self::modulated_frequency_default(500.0, context)
        };
        (fm, fc)
    }

    #[inline]
    pub(super) fn source_time_component<const DERIVATIVE: bool>(
        spec: &crate::netlist::SourceSpec,
        time: Value,
        context: Option<TransientSourceContext>,
        pwl_waveform: Option<&crate::device::pwl_file::PwlWaveform>,
    ) -> Value {
        use crate::netlist::SourceSpec;
        use std::f64::consts::PI;

        match spec {
            SourceSpec::Distortion { inner, .. } => {
                Self::source_time_component::<DERIVATIVE>(inner, time, context, pwl_waveform)
            }
            // A port that declares a power or a frequency is a large-signal RF
            // generator, and its drive rides on whatever the source itself was
            // given. ngspice drops the source's own DC here; that would step
            // the node by the bias between the operating point and the first
            // transient sample, so the DC is kept.
            SourceSpec::RfPort { inner, port } => {
                Self::source_time_component::<DERIVATIVE>(inner, time, context, pwl_waveform)
                    + if DERIVATIVE {
                        port.drive_tone()
                            .map_or(0.0, |(amplitude, frequency, phase)| {
                                let omega = 2.0 * PI * frequency;
                                -amplitude * omega * (omega * time + phase).sin()
                            })
                    } else {
                        port.drive_at(time).unwrap_or(0.0)
                    }
            }
            SourceSpec::Dc(v) => {
                if DERIVATIVE {
                    0.0
                } else {
                    *v
                }
            }
            SourceSpec::Ac { .. } => 0.0, // AC sources are DC=0 in transient
            // TRNOISE expands into a PWL sample train before circuit
            // construction; an unexpanded spec is zero-mean by definition.
            SourceSpec::TrNoise { .. } => 0.0,
            SourceSpec::TrRandom { parameter2, .. } => {
                if DERIVATIVE {
                    0.0
                } else {
                    *parameter2
                }
            }
            SourceSpec::DcAc { dc_value, .. } => {
                if DERIVATIVE {
                    0.0
                } else {
                    *dc_value
                }
            }
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::AcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::source_time_component::<DERIVATIVE>(transient, time, context, pwl_waveform)
            }
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                pulse_count,
                width_defaults_to_zero,
            } => {
                let xyce_boundaries =
                    Self::pulse_dialect(context) == crate::config::SpiceDialect::Xyce;
                let (delay, rise, fall, width, period) = Self::resolve_pulse_timing(
                    *delay,
                    *rise,
                    *fall,
                    *width,
                    *period,
                    *width_defaults_to_zero,
                    context,
                );
                if time < delay {
                    return if DERIVATIVE { 0.0 } else { *v1 };
                }
                let t_rel = time - delay;
                // ngspice vsrcload.c: a positive eighth argument bounds the
                // waveform to that many periods, after which it holds V1 for
                // the rest of the run.
                if Self::pulse_train_has_ended(t_rel, period, *pulse_count) {
                    return if DERIVATIVE { 0.0 } else { *v1 };
                }
                let repeating_period = if xyce_boundaries {
                    period.is_finite() && period != 0.0
                } else {
                    period.is_finite() && period > 0.0
                };
                let t = if repeating_period && t_rel > period {
                    t_rel - period * (t_rel / period).floor()
                } else {
                    t_rel
                };
                if DERIVATIVE {
                    // At a corner the initializer needs the outgoing slope.
                    // An instantaneous jump has no finite derivative at the
                    // published value and cannot supply a regular flux rate.
                    let t = if repeating_period && t == period {
                        0.0
                    } else {
                        t
                    };
                    return if t < 0.0 || t > rise + width + fall {
                        0.0
                    } else if t == 0.0 && rise == 0.0 && v1 != v2 {
                        Value::NAN
                    } else if t < rise {
                        (v2 - v1) / rise
                    } else if t < rise + width {
                        0.0
                    } else if t < rise + width + fall {
                        (v1 - v2) / fall
                    } else if fall == 0.0 && v1 != v2 && xyce_boundaries {
                        Value::NAN
                    } else {
                        0.0
                    };
                }
                if xyce_boundaries {
                    // Mirror Xyce 7.10 PulseData::updateSource branch-for-
                    // branch. Its tolerance follows the transient hard
                    // minimum timestep at the current accepted time and is
                    // deliberately unrelated to the static source tstep. A
                    // real transient supplies that accepted controller state;
                    // only stateless waveform previews derive a target-time
                    // fallback because they have no accepted state.
                    let breakpoint_tolerance = context
                        .and_then(|context| context.xyce_breakpoint_tolerance)
                        .unwrap_or_else(|| 2.0 * crate::numerics::xyce_hard_min_timestep(time));
                    let rise_width = rise + width;
                    let end = rise_width + fall;
                    if t <= 0.0 || (t > end && (t - end).abs() > breakpoint_tolerance) {
                        *v1
                    } else if t > rise
                        && (t - rise).abs() > breakpoint_tolerance
                        && (t < rise_width || (t - rise_width).abs() < breakpoint_tolerance)
                    {
                        *v2
                    } else if t > 0.0 && (t < rise || (t - rise).abs() < breakpoint_tolerance) {
                        if rise != 0.0 {
                            v1 + (v2 - v1) * t / rise
                        } else {
                            *v1
                        }
                    } else if fall != 0.0 {
                        v2 + (v1 - v2) * (t - rise_width) / fall
                    } else {
                        *v2
                    }
                } else if t <= 0.0 || t >= rise + width + fall {
                    *v1
                } else if t < rise {
                    v1 + (v2 - v1) * t / rise
                } else if t < rise + width {
                    *v2
                } else if t < rise + width + fall {
                    v2 + (v1 - v2) * (t - rise - width) / fall
                } else {
                    *v1
                }
            }
            SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => {
                let frequency = Self::resolve_sin_frequency(*frequency, context);
                if time < *delay {
                    // ngspice holds VO + VA*sin(PHASE) before the delay,
                    // not the bare offset (vsrcload.c).
                    if DERIVATIVE {
                        0.0
                    } else {
                        offset + amplitude * phase.sin()
                    }
                } else {
                    let t = time - delay;
                    if DERIVATIVE {
                        let omega = 2.0 * PI * frequency;
                        let angle = omega * t + phase;
                        amplitude
                            * (-damping * t).exp()
                            * (omega * angle.cos() - damping * angle.sin())
                    } else {
                        offset
                            + amplitude
                                * (-damping * t).exp()
                                * (2.0 * PI * frequency * t + phase).sin()
                    }
                }
            }
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => Self::pwl_time_component::<DERIVATIVE>(points, time, *delay, *repeat_from),
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
                delay,
                repeat_from,
            } => {
                if let Some(waveform) = pwl_waveform {
                    return if time < *delay {
                        0.0
                    } else {
                        if DERIVATIVE {
                            waveform.right_derivative_at_repeating(time - *delay, *repeat_from)
                        } else {
                            waveform.value_at_repeating(time - *delay, *repeat_from)
                        }
                    };
                }
                let key =
                    PwlCacheKey::new(path, *time_scale, *value_scale, *time_offset, *value_offset);
                let resource_limits = context
                    .map(|context| context.resource_limits)
                    .unwrap_or_default();
                match Self::load_pwl_waveform_cached_with_limits(
                    path,
                    *time_scale,
                    *value_scale,
                    *time_offset,
                    *value_offset,
                    resource_limits,
                ) {
                    Ok(waveform) => {
                        if time < *delay {
                            0.0
                        } else {
                            if DERIVATIVE {
                                waveform.right_derivative_at_repeating(time - *delay, *repeat_from)
                            } else {
                                waveform.value_at_repeating(time - *delay, *repeat_from)
                            }
                        }
                    }
                    Err(err) => {
                        let message = format!("failed to load PWL file '{path}': {err}");
                        Self::log_pwl_error_once(
                            key,
                            &message,
                            resource_limits.max_shared_cache_bytes,
                        );
                        if DERIVATIVE {
                            Value::NAN
                        } else {
                            *value_offset
                        }
                    }
                }
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
            } => Self::pat_time_component::<DERIVATIVE>(
                *vhi,
                *vlo,
                *delay,
                *rise,
                *fall,
                *sample,
                data,
                *repeat_count,
                time,
            ),
            SourceSpec::Exp {
                v1,
                v2,
                td1,
                tau1,
                td2,
                tau2,
            } => {
                let (td1, tau1, td2, tau2) =
                    Self::resolve_exp_timing(*td1, *tau1, *td2, *tau2, context);
                if DERIVATIVE {
                    return if time < td1 {
                        0.0
                    } else {
                        let rise = (v2 - v1) * (-(time - td1) / tau1).exp() / tau1;
                        if time < td2 {
                            rise
                        } else {
                            rise - (v2 - v1) * (-(time - td2) / tau2).exp() / tau2
                        }
                    };
                }
                if time <= td1 {
                    *v1
                } else if time <= td2 {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                } else {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                        - (v2 - v1) * (1.0 - (-(time - td2) / tau2).exp())
                }
            }
            SourceSpec::Sffm {
                offset,
                amplitude,
                carrier_freq,
                modulation_index,
                signal_freq,
                delay,
                phase_modulation,
                phase_carrier,
            } => {
                let (fc, fm, mdi) =
                    Self::sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, context);
                if matches!(
                    Self::pulse_dialect(context),
                    crate::config::SpiceDialect::Xyce
                ) {
                    if DERIVATIVE {
                        let carrier = 2.0 * PI * fc;
                        let modulation = 2.0 * PI * fm;
                        return amplitude
                            * (carrier * time + mdi * (modulation * time).sin()).cos()
                            * (carrier + mdi * modulation * (modulation * time).cos());
                    }
                    return *offset
                        + *amplitude
                            * ((2.0 * PI * fc * time) + mdi * (2.0 * PI * fm * time).sin()).sin();
                }

                let t = time - delay;
                if DERIVATIVE {
                    let carrier = 2.0 * PI * fc;
                    let modulation = 2.0 * PI * fm;
                    let angle_m = modulation * t + phase_modulation.to_radians();
                    let angle = carrier * t + phase_carrier.to_radians() + mdi * angle_m.sin();
                    return if t < 0.0 {
                        0.0
                    } else if t == 0.0 && offset + amplitude * angle.sin() != 0.0 {
                        Value::NAN
                    } else {
                        amplitude * angle.cos() * (carrier + mdi * modulation * angle_m.cos())
                    };
                }
                if t <= 0.0 {
                    0.0
                } else {
                    let phasec = phase_carrier.to_radians();
                    let phasem = phase_modulation.to_radians();
                    offset
                        + amplitude
                            * ((2.0 * PI * fc * t + phasec)
                                + mdi * (2.0 * PI * fm * t + phasem).sin())
                            .sin()
                }
            }
            SourceSpec::Am {
                offset,
                modulation_offset,
                modulation_amplitude,
                modulating_freq,
                carrier_freq,
                delay,
                phase_modulation,
                phase_carrier,
            } => {
                let (fm, fc) = Self::am_frequencies(*modulating_freq, *carrier_freq, context);
                let t = time - delay;
                if DERIVATIVE {
                    let carrier = 2.0 * PI * fc;
                    let modulation = 2.0 * PI * fm;
                    let angle_m = modulation * t + phase_modulation.to_radians();
                    let angle_c = carrier * t + phase_carrier.to_radians();
                    let envelope = modulation_offset + modulation_amplitude * angle_m.sin();
                    return if t < 0.0 {
                        0.0
                    } else if t == 0.0 && offset + envelope * angle_c.sin() != 0.0 {
                        Value::NAN
                    } else {
                        modulation_amplitude * modulation * angle_m.cos() * angle_c.sin()
                            + envelope * carrier * angle_c.cos()
                    };
                }
                if t <= 0.0 {
                    0.0
                } else {
                    let phasec = phase_carrier.to_radians();
                    let phasem = phase_modulation.to_radians();
                    offset
                        + (modulation_offset
                            + modulation_amplitude * (2.0 * PI * fm * t + phasem).sin())
                            * (2.0 * PI * fc * t + phasec).sin()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analytic_current_slopes_match_interior_waveform_differences_in_both_dialects() {
        let specs = [
            "DC 0.7",
            "SIN(0.2 1.3 1meg 0.1u 1e4 37)",
            "PULSE(0.2 1.3 0.1u 0.2u 0.3u 0.4u 1.7u)",
            "EXP(0.2 1.3 0.1u 0.2u 0.7u 0.3u)",
            "SFFM(0.2 1.3 1meg 5 100k 0.1u 30 60)",
            "AM(0.1 0.5 2 20k 1meg 0.1u 45 90)",
            "PAT(1.3 0.2 0.1u 0.2u 0.3u 1u b1010) R=-1",
            "PWL(0 0.2 0.3u 1.3 0.7u -0.1 1u 0.2) R=0",
            // Omitted frequencies and edges exercise the analysis context.
            "SIN(0.2 1.3)",
            "PULSE(0.2 1.3)",
            "SFFM(0 1)",
        ];
        for dialect in [
            crate::config::SpiceDialect::Ngspice,
            crate::config::SpiceDialect::Xyce,
        ] {
            for spec in specs {
                let mut sources = super::super::tests::current_source_with_waveform(spec);
                sources.set_transient_context_with_dialect(1e-8, 4e-6, dialect);
                for time in [
                    0.043e-6, 0.153e-6, 0.453e-6, 0.853e-6, 1.153e-6, 2.753e-6, 4.153e-6,
                ] {
                    let h = 1e-12;
                    let numerical = (8.0
                        * (sources.value_at_time(0, time + h)
                            - sources.value_at_time(0, time - h))
                        - (sources.value_at_time(0, time + 2.0 * h)
                            - sources.value_at_time(0, time - 2.0 * h)))
                        / (12.0 * h);
                    let analytic = sources.right_derivative_at_time(0, time);
                    assert!(
                        (analytic - numerical).abs() < 1e-3 + 1e-7 * numerical.abs(),
                        "{dialect:?}, {spec}, t={time:e}: {analytic:e} versus {numerical:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn outgoing_slopes_use_the_next_segment_and_preserve_sinusoidal_phase() {
        use crate::netlist::SourceSpec;
        let pulse = SourceSpec::Pulse {
            v1: 0.2,
            v2: 1.4,
            delay: 1.0,
            rise: 2.0,
            fall: 3.0,
            width: 4.0,
            period: 12.0,
            pulse_count: Value::NAN,
            width_defaults_to_zero: false,
        };
        for (time, expected) in [
            (0.5, 0.0),
            (1.0, 0.6),
            (3.0, 0.0),
            (7.0, -0.4),
            (10.0, 0.0),
            (13.0, 0.6),
        ] {
            let actual = VoltageSources::source_time_component::<true>(&pulse, time, None, None);
            assert!((actual - expected).abs() < 1e-15, "pulse t={time}");
        }
        let sinusoid = SourceSpec::Sin {
            offset: 0.2,
            amplitude: 1.3,
            frequency: 2.0,
            delay: 0.1,
            damping: 0.7,
            phase: 37.0_f64.to_radians(),
        };
        assert_eq!(
            VoltageSources::source_time_component::<true>(&sinusoid, 0.05, None, None),
            0.0
        );
        let expected = 1.3
            * (std::f64::consts::TAU * 2.0 * 37.0_f64.to_radians().cos()
                - 0.7 * 37.0_f64.to_radians().sin());
        assert!(
            (VoltageSources::source_time_component::<true>(&sinusoid, 0.1, None, None) - expected)
                .abs()
                < 1e-14
        );
        let points = [(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)];
        for (time, expected) in [(0.0, 2.0), (1.0, -2.0), (2.0, 2.0), (3.0, -2.0), (4.0, 2.0)] {
            assert_eq!(
                VoltageSources::pwl_time_component::<true>(&points, time, 0.0, Some(0.0)),
                expected
            );
        }
        let rf = SourceSpec::RfPort {
            inner: Box::new(SourceSpec::Dc(0.2)),
            port: crate::netlist::SourceRfPort {
                portnum: 1,
                z0: 50.0,
                power: Some(1e-3),
                frequency: Some(2.0),
                phase: Some(90.0),
                reference_plane: None,
            },
        };
        let expected = -(0.2_f64).sqrt() * 2.0 * std::f64::consts::TAU;
        assert!(
            (VoltageSources::source_time_component::<true>(&rf, 0.0, None, None) - expected).abs()
                < 1e-14
        );
    }
}

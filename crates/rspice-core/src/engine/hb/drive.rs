use super::*;

impl Engine {
    pub(in crate::engine::hb) fn hb_source_spectrum(
        fallback_dc: Value,
        ac_mag: Value,
        ac_phase: Value,
        spec: Option<&SourceSpec>,
        config: &HbConfig,
        drive_harmonics: &[usize],
        spice_dialect: crate::engine::SpiceDialect,
    ) -> Result<HbSourceSpectrum, SimulationError> {
        let Some(spec) = spec else {
            return Ok(HbSourceSpectrum {
                dc: fallback_dc,
                harmonics: drive_harmonics
                    .iter()
                    .copied()
                    .filter(|_| ac_mag != 0.0)
                    .map(|harmonic| (harmonic, ac_mag, ac_phase))
                    .collect(),
            });
        };
        match spec {
            SourceSpec::Distortion { inner, .. } => Self::hb_source_spectrum(
                fallback_dc,
                ac_mag,
                ac_phase,
                Some(inner),
                config,
                drive_harmonics,
                spice_dialect,
            ),
            SourceSpec::RfPort { inner, port } => {
                let Some((amplitude, frequency, phase)) = port.drive_tone() else {
                    return Self::hb_source_spectrum(
                        fallback_dc,
                        ac_mag,
                        ac_phase,
                        Some(inner),
                        config,
                        drive_harmonics,
                        spice_dialect,
                    );
                };
                // A declared drive silences the AC magnitude, exactly as an
                // explicit transient waveform does below: `AC` on a port is the
                // small-signal excitation `.AC` and the S-parameter sweep read,
                // and reading it as a large-signal tone as well would drive the
                // port twice. Transient already ignores it here -- the port's
                // waveform is its DC plus this drive -- and harmonic balance is
                // the same steady state, so the two have to agree or one deck
                // describes two circuits.
                // Silencing the `ac_mag` argument is not enough: an `AC`-only
                // or `DcAc` inner spec carries its own magnitude and would
                // inject it at every drive harmonic regardless. Only the bias
                // and any real waveform survive. (`DcAcTransient` already drops
                // its AC when it recurses into the waveform.)
                let inner = match inner.as_ref() {
                    SourceSpec::Ac { .. } => SourceSpec::Dc(0.0),
                    SourceSpec::DcAc { dc_value, .. } => SourceSpec::Dc(*dc_value),
                    other => other.clone(),
                };
                let mut spectrum = Self::hb_source_spectrum(
                    fallback_dc,
                    0.0,
                    0.0,
                    Some(&inner),
                    config,
                    drive_harmonics,
                    spice_dialect,
                )?;
                if amplitude != 0.0 {
                    let harmonic = Self::hb_periodic_source_harmonic(
                        frequency,
                        config.fundamental_freq,
                        config.num_harmonics,
                        "RF port",
                    )?;
                    // Summed rather than replacing, so a port carrying both a
                    // drive and its own waveform excites the circuit with the
                    // same total that transient integrates.
                    spectrum.harmonics.push((harmonic, amplitude, phase));
                    Self::hb_merge_source_harmonics(&mut spectrum.harmonics);
                }
                Ok(spectrum)
            }
            SourceSpec::Dc(value) => Ok(HbSourceSpectrum {
                dc: *value,
                harmonics: Vec::new(),
            }),
            SourceSpec::Ac { magnitude, phase } => Ok(HbSourceSpectrum {
                dc: 0.0,
                harmonics: drive_harmonics
                    .iter()
                    .copied()
                    .map(|harmonic| (harmonic, *magnitude, *phase))
                    .collect(),
            }),
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            } => Ok(HbSourceSpectrum {
                dc: *dc_value,
                harmonics: drive_harmonics
                    .iter()
                    .copied()
                    .filter(|_| *ac_magnitude != 0.0)
                    .map(|harmonic| (harmonic, *ac_magnitude, *ac_phase))
                    .collect(),
            }),
            SourceSpec::DcTransient {
                dc_value: _,
                transient,
            }
            | SourceSpec::DcAcTransient {
                transient,
                dc_value: _,
                ac_magnitude: _,
                ac_phase: _,
            } => Self::hb_source_spectrum(
                fallback_dc,
                0.0,
                0.0,
                Some(transient),
                config,
                drive_harmonics,
                spice_dialect,
            ),
            SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => {
                for (name, value) in [
                    ("offset", *offset),
                    ("amplitude", *amplitude),
                    ("delay", *delay),
                    ("damping", *damping),
                    ("phase", *phase),
                ] {
                    if !value.is_finite() {
                        return Err(HbError::InvalidConfig(format!(
                            "HB SIN source {name} must be finite"
                        ))
                        .into());
                    }
                }
                if *amplitude == 0.0 || drive_harmonics.is_empty() {
                    return Ok(HbSourceSpectrum {
                        dc: *offset,
                        harmonics: Vec::new(),
                    });
                }
                if *damping != 0.0 {
                    return Err(HbError::InvalidConfig(
                        "HB requires periodic sources; a damped SIN waveform is not periodic"
                            .to_string(),
                    )
                    .into());
                }
                if !frequency.is_finite() || *frequency <= 0.0 {
                    return Err(HbError::InvalidConfig(
                        "HB SIN source frequency must be finite and positive".to_string(),
                    )
                    .into());
                }
                let harmonic = Self::hb_periodic_source_harmonic(
                    *frequency,
                    config.fundamental_freq,
                    config.num_harmonics,
                    "SIN",
                )?;
                // HB coefficients use cosine-reference phasors, whereas the
                // netlist SIN waveform is defined with a sine reference.
                let phase = *phase
                    - std::f64::consts::FRAC_PI_2
                    - std::f64::consts::TAU * frequency * delay;
                Ok(HbSourceSpectrum {
                    dc: *offset,
                    harmonics: vec![(harmonic, *amplitude, phase)],
                })
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
                width_defaults_to_zero: _,
            } => {
                let mut spectrum = Self::hb_pulse_source_spectrum(
                    *v1,
                    *v2,
                    *delay,
                    *rise,
                    *fall,
                    *width,
                    *period,
                    *pulse_count,
                    spec,
                    config,
                    spice_dialect,
                )?;
                if drive_harmonics.is_empty() {
                    spectrum.harmonics.clear();
                }
                Ok(spectrum)
            }
            SourceSpec::Pwl { .. } => Err(HbError::InvalidConfig(
                "HB does not yet implement exact periodic PWL source coefficients".to_string(),
            )
            .into()),
            SourceSpec::PwlFile { .. } => Err(HbError::InvalidConfig(
                "HB cannot authenticate or derive exact periodic coefficients from a PWL FILE source"
                    .to_string(),
            )
            .into()),
            SourceSpec::Pat { .. } => Err(HbError::InvalidConfig(
                "HB does not yet implement exact periodic PAT source coefficients".to_string(),
            )
            .into()),
            SourceSpec::Exp { .. } => Err(HbError::InvalidConfig(
                "HB requires periodic sources; EXP is not periodic".to_string(),
            )
            .into()),
            SourceSpec::Sffm { .. } => Err(HbError::InvalidConfig(
                "HB does not yet implement exact periodic SFFM source coefficients".to_string(),
            )
            .into()),
            SourceSpec::Am { .. } => Err(HbError::InvalidConfig(
                "HB does not yet implement exact periodic AM source coefficients".to_string(),
            )
            .into()),
            SourceSpec::TrNoise { .. } => Err(HbError::InvalidConfig(
                "HB requires deterministic periodic sources; TRNOISE is stochastic".to_string(),
            )
            .into()),
            SourceSpec::TrRandom { .. } => Err(HbError::InvalidConfig(
                "HB requires deterministic periodic sources; TRRANDOM is stochastic".to_string(),
            )
            .into()),
        }
    }

    fn hb_merge_source_harmonics(harmonics: &mut Vec<(usize, Value, Value)>) {
        let mut merged = std::collections::BTreeMap::<usize, Complex64>::new();
        for (harmonic, amplitude, phase) in harmonics.drain(..) {
            *merged.entry(harmonic).or_default() += Complex64::from_polar(amplitude, phase);
        }
        harmonics.extend(merged.into_iter().filter_map(|(harmonic, phasor)| {
            let amplitude = phasor.norm();
            (amplitude != 0.0).then_some((harmonic, amplitude, phasor.arg()))
        }));
    }

    fn hb_periodic_source_harmonic(
        source_frequency: Value,
        fundamental_frequency: Value,
        num_harmonics: usize,
        source_kind: &str,
    ) -> Result<usize, SimulationError> {
        let ratio = source_frequency / fundamental_frequency;
        let rounded = ratio.round();
        let relative_error = (ratio - rounded).abs() / rounded.abs().max(1.0);
        if !ratio.is_finite() || rounded < 1.0 || relative_error > 1.0e-9 {
            return Err(HbError::InvalidConfig(format!(
                "HB {source_kind} source frequency {source_frequency:.12e} Hz is not a positive integer harmonic of the configured fundamental {fundamental_frequency:.12e} Hz"
            ))
            .into());
        }
        let harmonic = rounded as usize;
        if harmonic > num_harmonics {
            return Err(HbError::InvalidConfig(format!(
                "HB {source_kind} source maps to harmonic {harmonic}, beyond the configured {num_harmonics} harmonics"
            ))
            .into());
        }
        Ok(harmonic)
    }

    #[allow(clippy::too_many_arguments)]
    fn hb_pulse_source_spectrum(
        v1: Value,
        v2: Value,
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        pulse_count: Value,
        pulse_spec: &SourceSpec,
        config: &HbConfig,
        spice_dialect: crate::engine::SpiceDialect,
    ) -> Result<HbSourceSpectrum, SimulationError> {
        // Harmonic balance solves for a steady state, so the drive has to be
        // periodic for the whole run. A PULSE bounded by `NP` stops after
        // that many periods and holds V1, which has no steady state to find.
        if pulse_count > 0.0 {
            return Err(HbError::InvalidConfig(
                "HB PULSE source must be periodic; its eighth argument bounds the pulse train"
                    .to_string(),
            )
            .into());
        }
        for (name, value) in [
            ("initial value", v1),
            ("pulsed value", v2),
            ("delay", delay),
            ("pulse count", pulse_count),
        ] {
            if !value.is_finite() {
                return Err(HbError::InvalidConfig(format!(
                    "HB PULSE source {name} must be finite"
                ))
                .into());
            }
        }
        if !period.is_finite() || period <= 0.0 {
            return Err(HbError::InvalidConfig(
                "HB PULSE source period must be finite and positive".to_string(),
            )
            .into());
        }
        for (name, value) in [("rise", rise), ("fall", fall), ("width", width)] {
            if !value.is_finite() || value < 0.0 {
                return Err(HbError::InvalidConfig(format!(
                    "HB PULSE source {name} time must be finite and non-negative"
                ))
                .into());
            }
        }
        let occupied = rise + width + fall;
        if !occupied.is_finite() {
            return Err(HbError::InvalidConfig(
                "HB PULSE source rise + width + fall must be finite".to_string(),
            )
            .into());
        }
        let time_tolerance = 1.0e-12 * period.abs().max(occupied.abs()).max(1.0e-30);
        if occupied > period + time_tolerance {
            return Err(HbError::InvalidConfig(format!(
                "HB PULSE source rise + width + fall ({occupied:.12e} s) exceeds its period ({period:.12e} s)"
            ))
            .into());
        }

        let source_frequency = period.recip();
        let source_harmonic = Self::hb_periodic_source_harmonic(
            source_frequency,
            config.fundamental_freq,
            config.num_harmonics,
            "PULSE",
        )?;
        let delta = v2 - v1;
        let dc = v1 + delta * (width + 0.5 * rise + 0.5 * fall) / period;
        if v2 == v1 {
            return Ok(HbSourceSpectrum {
                dc,
                harmonics: Vec::new(),
            });
        }

        if spice_dialect == crate::engine::SpiceDialect::Xyce {
            if config.tones.len() > 1 {
                return Err(HbError::InvalidConfig(
                    "Xyce-compatible multi-tone PULSE projection requires the nonuniform APFT collocation transform, which is not implemented"
                        .to_string(),
                )
                .into());
            }
            return Self::hb_xyce_collocated_pulse_source_spectrum(pulse_spec, config);
        }

        // Integrate the authored continuous, piecewise-linear waveform.  The
        // Fourier coefficient of a ramp is evaluated through its rectangular
        // derivative using sinc, which remains well-conditioned as a rise or
        // fall time approaches zero.  A zero-duration edge is its exact jump.
        // If the source repeats `source_harmonic` times per HB period, only
        // integer multiples of that HB harmonic can be non-zero.
        let delay = delay.rem_euclid(period);
        let mut harmonics = Vec::new();
        for harmonic in (source_harmonic..=config.num_harmonics).step_by(source_harmonic) {
            let source_order = harmonic / source_harmonic;
            let angular_frequency = std::f64::consts::TAU * source_order as Value / period;
            let mut derivative_integral = Complex64::new(0.0, 0.0);
            for (start, duration, change) in [(0.0, rise, 1.0), (rise + width, fall, -1.0)] {
                if duration == 0.0 {
                    derivative_integral +=
                        Complex64::from_polar(change, -angular_frequency * start);
                } else {
                    let half_angle = 0.5 * angular_frequency * duration;
                    let sinc = if half_angle == 0.0 {
                        1.0
                    } else {
                        half_angle.sin() / half_angle
                    };
                    derivative_integral += Complex64::from_polar(
                        change * sinc,
                        -angular_frequency * (start + 0.5 * duration),
                    );
                }
            }
            let coefficient = delta * derivative_integral
                / Complex64::new(0.0, std::f64::consts::TAU * source_order as Value)
                * Complex64::from_polar(1.0, -angular_frequency * delay);
            let phasor = 2.0 * coefficient;
            let amplitude = phasor.norm();
            if amplitude != 0.0 {
                harmonics.push((harmonic, amplitude, phasor.arg()));
            }
        }
        Ok(HbSourceSpectrum { dc, harmonics })
    }

    fn hb_xyce_collocated_pulse_source_spectrum(
        pulse_spec: &SourceSpec,
        config: &HbConfig,
    ) -> Result<HbSourceSpectrum, SimulationError> {
        // Xyce's single-tone HB time-domain formulation represents the
        // independent drive on the same finite collocation grid used by the
        // nonlinear residual. Preserve that projection in the Xyce dialect;
        // native and ngspice modes retain the continuous analytic Fourier
        // integral above. Using the shared FFT implementation keeps this
        // O(N log N), including for explicitly oversampled production grids.
        let collocation_points = config
            .checked_fft_size()
            .map_err(|error| HbError::InvalidConfig(error.to_string()))?;
        let hb_period = config.fundamental_freq.recip();
        let collocation_step = hb_period / collocation_points as Value;
        let samples = (0..collocation_points)
            .map(|sample| {
                crate::circuit::VoltageSources::evaluate_source_spec_at_time_with_dialect(
                    pulse_spec,
                    sample as Value * collocation_step,
                    collocation_step,
                    hb_period,
                    crate::engine::SpiceDialect::Xyce,
                )
            })
            .collect::<Vec<_>>();
        let mut fft = HbFft::try_with_size(config.num_harmonics, collocation_points)
            .map_err(|error| HbError::InvalidConfig(error.to_string()))?;
        let coefficients = fft.to_frequency_domain(&samples);
        let dc = coefficients[0].re;
        let harmonics = coefficients
            .into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(harmonic, coefficient)| {
                let phasor = 2.0 * coefficient;
                let amplitude = phasor.norm();
                (amplitude != 0.0).then_some((harmonic, amplitude, phasor.arg()))
            })
            .collect();
        Ok(HbSourceSpectrum { dc, harmonics })
    }

    /// Build node names from circuit node map
    pub(in crate::engine::hb) fn hb_build_node_names(
        &self,
        circuit: &CircuitData,
        num_nodes: usize,
    ) -> Vec<String> {
        let mut node_names = circuit.node_names_sorted();
        if node_names.len() > num_nodes {
            node_names.truncate(num_nodes);
        } else if node_names.len() < num_nodes {
            let mut synthetic_index = node_names.len() + 1;
            while node_names.len() < num_nodes {
                node_names.push(format!("n{}", synthetic_index));
                synthetic_index += 1;
            }
        }
        node_names
    }

    pub(in crate::engine::hb) fn hb_collect_drive_tones(
        config: &HbConfig,
    ) -> Result<Vec<HbDriveTone>, SimulationError> {
        if config.tones.is_empty() {
            return Ok(vec![HbDriveTone::broadcast(1)]);
        }

        if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
            return Err(HbError::InvalidConfig(
                "HB multi-tone requires a positive basis fundamental frequency".to_string(),
            )
            .into());
        }

        let mut tones: BTreeSet<(usize, Option<String>)> = BTreeSet::new();
        for tone in &config.tones {
            if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' has invalid frequency {}",
                    tone.name, tone.frequency
                ))
                .into());
            }

            let ratio = tone.frequency / config.fundamental_freq;
            let harmonic = ratio.round();
            let abs_error = (ratio - harmonic).abs();
            let rel_error = abs_error / harmonic.abs().max(1.0);

            if !harmonic.is_finite() || harmonic < 1.0 {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' does not map to a positive harmonic of f0={:.6e} Hz",
                    tone.name, config.fundamental_freq
                ))
                .into());
            }
            if rel_error > 1e-9 {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' at {:.6e} Hz is not an integer harmonic of f0={:.6e} Hz",
                    tone.name, tone.frequency, config.fundamental_freq
                ))
                .into());
            }

            let harmonic = harmonic as usize;
            if harmonic > config.num_harmonics {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' maps to harmonic {} but num_harmonics is {}",
                    tone.name, harmonic, config.num_harmonics
                ))
                .into());
            }
            let required_harmonic =
                harmonic
                    .checked_mul(tone.num_harmonics.max(1))
                    .ok_or_else(|| {
                        HbError::InvalidConfig(format!(
                            "HB tone '{}' harmonic order overflows the addressable spectrum",
                            tone.name
                        ))
                    })?;
            if required_harmonic > config.num_harmonics {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' requires common-basis harmonic {} (tone harmonic {} x order {}) but the configured spectrum stops at {}",
                    tone.name,
                    required_harmonic,
                    harmonic,
                    tone.num_harmonics,
                    config.num_harmonics
                ))
                .into());
            }
            let source_filter = tone
                .source_name
                .as_ref()
                .map(|name| name.trim())
                .filter(|name| !name.is_empty())
                .map(|name| name.to_ascii_lowercase());
            tones.insert((harmonic, source_filter));
        }

        let collected: Vec<HbDriveTone> = tones
            .into_iter()
            .map(|(harmonic, source_filter)| HbDriveTone {
                harmonic,
                source_filter,
            })
            .collect();
        if collected.is_empty() {
            Ok(vec![HbDriveTone::broadcast(1)])
        } else {
            Ok(collected)
        }
    }

    pub(in crate::engine::hb) fn hb_validate_drive_tone_sources(
        circuit: &CircuitData,
        drive_tones: &[HbDriveTone],
    ) -> Result<(), SimulationError> {
        for tone in drive_tones {
            let Some(source_filter) = tone.source_filter.as_deref() else {
                continue;
            };
            let present_in_voltage = circuit
                .voltage_sources
                .names
                .iter()
                .any(|name| source_filter.eq_ignore_ascii_case(name));
            let present_in_current = circuit
                .current_sources
                .names
                .iter()
                .any(|name| source_filter.eq_ignore_ascii_case(name));
            if !(present_in_voltage || present_in_current) {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone source '{}' is not present in circuit independent sources",
                    source_filter
                ))
                .into());
            }
        }
        Ok(())
    }

    pub(in crate::engine::hb) fn hb_drive_harmonics_for_source(
        drive_tones: &[HbDriveTone],
        source_name: &str,
    ) -> Vec<usize> {
        let mut harmonics: Vec<usize> = drive_tones
            .iter()
            .filter(|tone| tone.matches_source(source_name))
            .map(|tone| tone.harmonic)
            .collect();
        harmonics.sort_unstable();
        harmonics.dedup();
        harmonics
    }

    pub(in crate::engine::hb) fn hb_has_supported_nonlinear_devices(
        circuit: &CircuitData,
        _num_nodes: usize,
    ) -> bool {
        !circuit.diodes.is_empty()
            || !circuit.mosfets.is_empty()
            || !circuit.jfets.is_empty()
            || !circuit.vswitches.is_empty()
    }

    pub(in crate::engine::hb) fn hb_unsupported_nonlinear_device_summary(
        circuit: &CircuitData,
        _num_nodes: usize,
    ) -> Option<String> {
        let mut kinds: Vec<String> = Vec::new();
        let describe = |name: &str, count: usize| -> String {
            let noun = if count == 1 { "device" } else { "devices" };
            format!("{name} ({count} {noun})")
        };

        let reduced_diodes = circuit
            .diodes
            .devices
            .iter()
            .filter(|diode| {
                diode.level != crate::device::DiodeLevel::Legacy
                    || diode.forward_knee_current > 0.0
                    || diode.reverse_knee_current > 0.0
                    || diode.recombination_saturation_current != 0.0
                    || diode.sidewall_saturation_current != 0.0
                    || diode.sidewall_cj0 != 0.0
                    || diode.tunneling.bottom_given
                    || diode.tunneling.sidewall_given
                    || diode.tunneling.bottom != 0.0
                    || diode.tunneling.sidewall != 0.0
                    || diode.overlap_capacitance != 0.0
            })
            .count();
        if reduced_diodes > 0 {
            kinds.push(describe(
                "diodes requiring high-injection, recombination, sidewall, tunneling, overlap, or non-LEVEL=1 equations not represented by exact HB",
                reduced_diodes,
            ));
        }
        let invalid_diodes = circuit
            .diodes
            .devices
            .iter()
            .filter(|diode| {
                !diode.is.is_finite()
                    || diode.is < 0.0
                    || !diode.n.is_finite()
                    || diode.n <= 0.0
                    || !diode.vt.is_finite()
                    || diode.vt <= 0.0
                    || !(diode.n * diode.vt).is_finite()
                    || !diode.cj0.is_finite()
                    || diode.cj0 < 0.0
                    || !diode.vj.is_finite()
                    || diode.vj <= 0.0
                    || !diode.m.is_finite()
                    || !(0.0..=1.0).contains(&diode.m)
                    || !diode.fc.is_finite()
                    || !(0.0..1.0).contains(&diode.fc)
                    || !diode.tt.is_finite()
                    || diode.tt < 0.0
                    || diode.exact_hb_breakdown_parameter_error().is_some()
            })
            .count();
        if invalid_diodes > 0 {
            kinds.push(describe(
                "LEVEL=1 diodes with invalid or nonrepresentable exact-HB junction parameters",
                invalid_diodes,
            ));
        }
        if !circuit.bjts.is_empty() {
            kinds.push(describe(
                "native BJT/VBIC models whose complete Gummel-Poon/VBIC equations are not represented by exact HB",
                circuit.bjts.len(),
            ));
        }
        let invalid_mos = circuit
            .mosfets
            .devices
            .iter()
            .filter(|mos| mos.level == 1 && mos.level1_physical_parameter_error().is_some())
            .count();
        if invalid_mos > 0 {
            kinds.push(describe(
                "LEVEL=1 MOS devices with invalid or nonrepresentable physical parameters",
                invalid_mos,
            ));
        }
        let reduced_mos = circuit
            .mosfets
            .devices
            .iter()
            .filter(|mos| {
                mos.level != 1
                    || mos.body_junction_model
                        != crate::device::MosBodyJunctionModel::NgspiceReverseClamp
                    || (mos.cjsw != 0.0
                        && (mos.source_perimeter != 0.0 || mos.drain_perimeter != 0.0))
            })
            .count();
        if reduced_mos > 0 {
            kinds.push(describe(
                "classic MOS devices requiring non-LEVEL=1, non-ngspice bulk-junction, or sidewall-charge equations not represented by exact HB",
                reduced_mos,
            ));
        }
        let reduced_jfets = circuit
            .jfets
            .iter()
            .filter(|jfet| {
                jfet.params.channel_model != crate::device::JfetChannelModel::ShichmanHodges
                    || jfet.m != 1.0
                    || jfet.area != 1.0
                    || jfet.params.fc != 0.5
                    || jfet.params.n != 1.0
                    || jfet.resolved_instance_temperature() != jfet.params.tnom
            })
            .count();
        if reduced_jfets > 0 {
            kinds.push(describe(
                "JFET devices requiring non-Shichman-Hodges, geometry-scaled, temperature-scaled, or non-default junction equations not represented by exact HB",
                reduced_jfets,
            ));
        }
        let invalid_jfets = circuit
            .jfets
            .iter()
            .filter(|jfet| {
                let params = &jfet.params;
                !params.vto.is_finite()
                    || !params.beta.is_finite()
                    || params.beta < 0.0
                    || !params.lambda.is_finite()
                    || params.lambda < 0.0
                    || !params.is.is_finite()
                    || params.is < 0.0
                    || !params.cgs.is_finite()
                    || params.cgs < 0.0
                    || !params.cgd.is_finite()
                    || params.cgd < 0.0
                    || !params.pb.is_finite()
                    || params.pb <= 0.0
                    || !params.m.is_finite()
                    || !(0.0..=1.0).contains(&params.m)
            })
            .count();
        if invalid_jfets > 0 {
            kinds.push(describe(
                "JFET devices with invalid physical parameters",
                invalid_jfets,
            ));
        }
        let invalid_vswitches = circuit
            .vswitches
            .iter()
            .filter(|switch| switch.physical_parameter_error().is_some())
            .count();
        if invalid_vswitches > 0 {
            kinds.push(describe(
                "voltage-controlled switches with invalid or nonrepresentable physical parameters",
                invalid_vswitches,
            ));
        }
        let unsupported_vswitches = circuit
            .vswitches
            .iter()
            .filter(|switch| switch.vh != 0.0 || switch.uses_xyce_curve_semantics())
            .count();
        if unsupported_vswitches > 0 {
            kinds.push(describe(
                "voltage-controlled switches requiring hysteresis or Xyce ON/OFF curve semantics not represented by exact HB",
                unsupported_vswitches,
            ));
        }
        #[cfg(feature = "veriloga")]
        if circuit.has_veriloga_devices() {
            kinds.push(describe(
                "runtime Verilog-A devices without exact HB charge/noise capability metadata",
                circuit.veriloga_devices().len(),
            ));
        }

        if !circuit.bsim3v3.is_empty() {
            kinds.push(describe("native BSIM3v3", circuit.bsim3v3.len()));
        }
        if !circuit.bsim4v8.is_empty() {
            kinds.push(describe("native BSIM4", circuit.bsim4v8.len()));
        }
        if !circuit.b3soi.is_empty() {
            kinds.push(describe("native B3SOI DD", circuit.b3soi.len()));
        }
        if !circuit.b3soi_fd.is_empty() {
            kinds.push(describe("native B3SOI FD", circuit.b3soi_fd.len()));
        }
        if !circuit.b3soi_pd.is_empty() {
            kinds.push(describe("native B3SOI PD", circuit.b3soi_pd.len()));
        }
        if !circuit.xyce_memristors.is_empty() {
            kinds.push(describe(
                "native Xyce memristor",
                circuit.xyce_memristors.len(),
            ));
        }

        if !circuit.iswitches.is_empty() {
            kinds.push(describe(
                "current-controlled switches requiring exact control-branch current spectra",
                circuit.iswitches.len(),
            ));
        }
        if !circuit.generic_switches.is_empty() {
            kinds.push(format!(
                "{} generic SWITCH CONTROL device(s) (HB support for expression-controlled switches is not native yet)",
                circuit.generic_switches.len()
            ));
        }
        if kinds.is_empty() {
            None
        } else {
            Some(kinds.join(", "))
        }
    }

    /// Circuit families whose branch equations are not yet represented by the
    /// exact PAC/PNoise periodic MNA descriptor set.
    pub(in crate::engine::hb) fn hb_periodic_mna_unsupported_summary(
        circuit: &CircuitData,
    ) -> Option<String> {
        let mut kinds = Vec::new();
        if !circuit.iswitches.is_empty() {
            kinds.push("current-controlled switches requiring control-branch current spectra");
        }
        if !circuit.ekv26s.is_empty() || !circuit.ekv3s.is_empty() || !circuit.vdmoses.is_empty() {
            kinds.push("unstamped advanced semiconductor models");
        }
        if !circuit.tlines.is_empty() || !circuit.coupled_tlines.is_empty() {
            kinds.push("distributed transmission-line equations");
        }
        if !circuit.jiles_atherton_inductors.is_empty() || !circuit.xyce_core_groups.is_empty() {
            kinds.push("nonlinear magnetic-core branch equations");
        }
        if circuit.capacitors.has_solution_dependent_values() {
            kinds.push("solution-dependent capacitor charge linearizations");
        }
        if !circuit.behavioral_sources.is_empty() {
            kinds.push("behavioral-source equations");
        }
        if !circuit.xspice_instances.is_empty() {
            kinds.push("XSPICE code-model equations");
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if circuit.has_generated_veriloga_devices() {
            kinds.push("generated Verilog-A compact-model equations");
        }
        let mut represented_branches = vec![false; circuit.num_branches()];
        for &ordinal in circuit
            .voltage_sources
            .branch_indices
            .iter()
            .chain(&circuit.inductors.branch_indices)
            .chain(&circuit.resistor_branches.branch_indices)
            .chain(&circuit.vcvs.branch_indices)
            .chain(&circuit.ccvs.branch_indices)
            .chain(
                circuit
                    .multi_winding_transformers
                    .iter()
                    .flat_map(|binding| binding.branch_ordinals.iter()),
            )
        {
            if let Some(slot) = ordinal
                .checked_sub(1)
                .and_then(|index| represented_branches.get_mut(index))
            {
                *slot = true;
            }
        }
        if represented_branches.iter().any(|represented| !represented) {
            kinds.push("unrepresented MNA branch families");
        }
        if kinds.is_empty() {
            None
        } else {
            kinds.sort_unstable();
            kinds.dedup();
            Some(kinds.join(", "))
        }
    }

    #[inline]
    pub(in crate::engine::hb) fn hb_node_to_solver_index(node: usize, num_nodes: usize) -> usize {
        if node == 0 { num_nodes } else { node - 1 }
    }
}

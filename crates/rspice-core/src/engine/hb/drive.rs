use super::*;

impl Engine {
    pub(in crate::engine::hb) fn hb_source_spectrum(
        fallback_dc: Value,
        ac_mag: Value,
        ac_phase: Value,
        spec: Option<&SourceSpec>,
        config: &HbConfig,
        drive_harmonics: &[usize],
    ) -> Result<HbSourceSpectrum, SimulationError> {
        let Some(spec) = spec else {
            return Ok(HbSourceSpectrum {
                dc: fallback_dc,
                harmonics: drive_harmonics
                    .iter()
                    .copied()
                    .filter(|_| ac_mag.abs() > HB_ZERO_SENSE_TOL)
                    .map(|harmonic| (harmonic, ac_mag, ac_phase))
                    .collect(),
            });
        };
        match spec {
            SourceSpec::RfPort { inner, .. } => Self::hb_source_spectrum(
                fallback_dc,
                ac_mag,
                ac_phase,
                Some(inner),
                config,
                drive_harmonics,
            ),
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
                    .filter(|_| ac_magnitude.abs() > HB_ZERO_SENSE_TOL)
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
                if amplitude.abs() <= HB_ZERO_SENSE_TOL || drive_harmonics.is_empty() {
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
                phase,
                width_defaults_to_zero: _,
            } => {
                let mut spectrum = Self::hb_pulse_source_spectrum(
                    *v1, *v2, *delay, *rise, *fall, *width, *period, *phase, config,
                )?;
                if drive_harmonics.is_empty() {
                    spectrum.harmonics.clear();
                }
                Ok(spectrum)
            }
            _ => Ok(HbSourceSpectrum {
                dc: fallback_dc,
                harmonics: Vec::new(),
            }),
        }
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
        phase_degrees: Value,
        config: &HbConfig,
    ) -> Result<HbSourceSpectrum, SimulationError> {
        for (name, value) in [
            ("initial value", v1),
            ("pulsed value", v2),
            ("delay", delay),
            ("phase", phase_degrees),
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
        let _source_harmonic = Self::hb_periodic_source_harmonic(
            source_frequency,
            config.fundamental_freq,
            config.num_harmonics,
            "PULSE",
        )?;
        let area = v1 * period + (v2 - v1) * (width + 0.5 * rise + 0.5 * fall);
        let dc = area / period;
        if (v2 - v1).abs() <= HB_ZERO_SENSE_TOL {
            return Ok(HbSourceSpectrum {
                dc,
                harmonics: Vec::new(),
            });
        }

        let shift = delay - phase_degrees / 360.0 * period;
        let collocation_points = config.fft_size();
        let hb_period = config.fundamental_freq.recip();
        let samples: Vec<Value> = (0..collocation_points)
            .map(|sample| {
                let time = sample as Value * hb_period / collocation_points as Value;
                let local_time = (time - shift).rem_euclid(period);
                if rise > 0.0 && local_time < rise {
                    v1 + (v2 - v1) * local_time / rise
                } else if local_time < rise + width {
                    v2
                } else if fall > 0.0 && local_time < occupied {
                    v2 + (v1 - v2) * (local_time - rise - width) / fall
                } else {
                    v1
                }
            })
            .collect();
        let dc = samples.iter().sum::<Value>() / collocation_points as Value;
        let mut harmonics = Vec::new();
        for harmonic in 1..=config.num_harmonics {
            let coefficient = samples
                .iter()
                .enumerate()
                .map(|(sample, value)| {
                    let angle = -std::f64::consts::TAU * harmonic as Value * sample as Value
                        / collocation_points as Value;
                    Complex64::from_polar(*value, angle)
                })
                .sum::<Complex64>()
                / collocation_points as Value;
            let amplitude = 2.0 * coefficient.norm();
            if amplitude > HB_ZERO_SENSE_TOL {
                harmonics.push((harmonic, amplitude, coefficient.arg()));
            }
        }
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
        num_nodes: usize,
    ) -> bool {
        !circuit.diodes.is_empty()
            || !circuit.bjts.is_empty()
            || !circuit.mosfets.is_empty()
            || !circuit.jfets.is_empty()
            || !circuit.vswitches.is_empty()
            || circuit
                .iswitches
                .iter()
                .any(|sw| Self::hb_resolve_iswitch_control(circuit, sw, num_nodes).is_ok())
            || {
                #[cfg(feature = "veriloga")]
                {
                    circuit.has_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    false
                }
            }
    }

    pub(in crate::engine::hb) fn hb_unsupported_nonlinear_device_summary(
        circuit: &CircuitData,
        num_nodes: usize,
    ) -> Option<String> {
        let mut kinds: Vec<String> = Vec::new();
        let describe = |name: &str, count: usize| -> String {
            let noun = if count == 1 { "device" } else { "devices" };
            format!("{name} ({count} {noun})")
        };

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

        let unsupported_iswitch = circuit
            .iswitches
            .iter()
            .filter(|sw| Self::hb_resolve_iswitch_control(circuit, sw, num_nodes).is_err())
            .count();
        if unsupported_iswitch > 0 {
            kinds.push(format!(
                "{} current switch(es) (HB requires static control-source waveforms for ISwitch control branches)",
                unsupported_iswitch
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

    pub(in crate::engine::hb) fn hb_extract_static_source_voltage(
        spec: Option<&SourceSpec>,
        fallback_dc: Value,
    ) -> Option<Value> {
        match spec {
            None => Some(fallback_dc),
            Some(SourceSpec::RfPort { inner, .. }) => {
                Self::hb_extract_static_source_voltage(Some(inner), fallback_dc)
            }
            Some(SourceSpec::Dc(v)) => Some(*v),
            Some(SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ..
            }) if ac_magnitude.abs() <= HB_ZERO_SENSE_TOL => Some(*dc_value),
            Some(SourceSpec::DcTransient {
                dc_value,
                transient,
            }) => Self::hb_extract_static_source_voltage(Some(transient), *dc_value),
            Some(SourceSpec::DcAcTransient {
                dc_value,
                ac_magnitude,
                transient,
                ..
            }) if ac_magnitude.abs() <= HB_ZERO_SENSE_TOL => {
                Self::hb_extract_static_source_voltage(Some(transient), *dc_value)
            }
            Some(SourceSpec::Ac { magnitude, .. }) if magnitude.abs() <= HB_ZERO_SENSE_TOL => {
                Some(0.0)
            }
            Some(SourceSpec::Sin {
                offset, amplitude, ..
            }) if amplitude.abs() <= HB_ZERO_SENSE_TOL => Some(*offset),
            Some(SourceSpec::Pulse { v1, v2, .. }) if (v2 - v1).abs() <= HB_ZERO_SENSE_TOL => {
                Some(*v1)
            }
            Some(SourceSpec::Exp { v1, v2, .. }) if (v2 - v1).abs() <= HB_ZERO_SENSE_TOL => {
                Some(*v1)
            }
            Some(SourceSpec::Pwl { points, .. }) => {
                let first = points.first().map(|(_, value)| *value)?;
                if points
                    .iter()
                    .all(|(_, value)| (*value - first).abs() <= HB_ZERO_SENSE_TOL)
                {
                    Some(first)
                } else {
                    None
                }
            }
            Some(SourceSpec::PwlFile { .. }) => None,
            _ => None,
        }
    }

    pub(in crate::engine::hb) fn hb_resolve_iswitch_control(
        circuit: &CircuitData,
        sw: &crate::device::CurrentSwitch,
        num_nodes: usize,
    ) -> Result<HbCurrentSwitchControl, ()> {
        let Some(ctrl_branch_matrix_idx) = sw.ctrl_branch else {
            return Err(());
        };
        if ctrl_branch_matrix_idx <= num_nodes {
            return Err(());
        }
        let ctrl_branch_ordinal = ctrl_branch_matrix_idx - num_nodes;
        let Some(vsrc_idx) = circuit
            .voltage_sources
            .branch_indices
            .iter()
            .position(|&ordinal| ordinal == ctrl_branch_ordinal)
        else {
            return Err(());
        };

        let dc = circuit
            .voltage_sources
            .dc_values
            .get(vsrc_idx)
            .copied()
            .unwrap_or(0.0);
        let ac_mag = circuit
            .voltage_sources
            .ac_magnitudes
            .get(vsrc_idx)
            .copied()
            .unwrap_or(0.0);
        let spec = circuit
            .voltage_sources
            .source_specs
            .get(vsrc_idx)
            .and_then(|s| s.as_ref());
        if ac_mag.abs() > HB_ZERO_SENSE_TOL {
            return Err(());
        }
        let static_voltage = Self::hb_extract_static_source_voltage(spec, dc).ok_or(())?;

        let ctrl_pos =
            Self::hb_node_to_solver_index(circuit.voltage_sources.node_pos[vsrc_idx], num_nodes);
        let ctrl_neg =
            Self::hb_node_to_solver_index(circuit.voltage_sources.node_neg[vsrc_idx], num_nodes);
        Ok(HbCurrentSwitchControl {
            ctrl_pos,
            ctrl_neg,
            control_current_bias: static_voltage * HB_NORTON_G,
        })
    }

    #[inline]
    pub(in crate::engine::hb) fn hb_node_to_solver_index(node: usize, num_nodes: usize) -> usize {
        if node == 0 { num_nodes } else { node - 1 }
    }
}

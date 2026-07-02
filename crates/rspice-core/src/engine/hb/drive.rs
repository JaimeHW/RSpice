use super::*;

impl Engine {
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

    /// HB drive terms for one source. An explicit AC keyword wins; a
    /// source specified only as a SIN waveform drives its tone with the
    /// waveform amplitude and phase (Spectre-style). Small-signal AC
    /// deliberately ignores SIN waveforms, so the stored AC arrays alone
    /// are not enough here.
    #[inline]
    pub(in crate::engine::hb) fn hb_source_drive_terms(
        ac_mag: Value,
        ac_phase: Value,
        spec: Option<&SourceSpec>,
    ) -> (Value, Value) {
        if ac_mag.abs() > HB_ZERO_SENSE_TOL {
            return (ac_mag, ac_phase);
        }
        match spec {
            Some(SourceSpec::RfPort { inner, .. }) => {
                Self::hb_source_drive_terms(ac_mag, ac_phase, Some(inner))
            }
            Some(SourceSpec::Sin {
                amplitude, phase, ..
            }) => (*amplitude, *phase),
            Some(SourceSpec::DcTransient { transient, .. }) => {
                if let SourceSpec::Sin {
                    amplitude, phase, ..
                } = transient.as_ref()
                {
                    (*amplitude, *phase)
                } else {
                    (ac_mag, ac_phase)
                }
            }
            _ => (ac_mag, ac_phase),
        }
    }
}

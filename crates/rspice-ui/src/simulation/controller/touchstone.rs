use super::*;

impl SimulationController {
    pub(super) fn maybe_export_touchstone(
        &self,
        state: &mut AppState,
        result: &crate::simulation::SimulationResult,
    ) {
        let Some(crate::simulation::multi_run::AnalysisSpec::SParameter { z0, ports, .. }) =
            self.current_spec.as_ref()
        else {
            return;
        };

        let mut sp_state = state.sim_setup.sp.clone();
        sp_state.ensure_initialized();
        let sp_cfg = match sp_state.to_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                state.push_sim_message(ConsoleMessage::warning(format!(
                    "Skipping Touchstone export: invalid S-parameter settings ({})",
                    e
                )));
                return;
            }
        };
        if !sp_cfg.touchstone_export {
            return;
        }

        let run_id = state.simulation.active_run().map(|run| run.id).unwrap_or(0);
        let z0_by_port: Vec<f64> = ports.iter().map(|port| port.z0.unwrap_or(*z0)).collect();
        let dataset = match Self::build_touchstone_dataset(
            result,
            *z0,
            &z0_by_port,
            sp_cfg.touchstone_version as usize,
        ) {
            Ok(dataset) => dataset,
            Err(e) => {
                state.push_sim_message(ConsoleMessage::warning(format!(
                    "Touchstone export skipped: {}",
                    e
                )));
                return;
            }
        };
        let num_ports = dataset
            .metadata
            .get("num_ports")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(2);
        let path =
            Self::touchstone_export_path(state, run_id, self.current_analysis_idx, num_ports);

        let writer = WaveformWriter::new(WaveformFormat::Touchstone);
        match writer.write(&dataset, &path) {
            Ok(()) => state.push_sim_message(ConsoleMessage::info(format!(
                "Exported Touchstone: {}",
                path.display()
            ))),
            Err(e) => state.push_sim_message(ConsoleMessage::warning(format!(
                "Touchstone export failed: {}",
                e
            ))),
        }
    }

    pub(super) fn build_touchstone_dataset(
        result: &crate::simulation::SimulationResult,
        z0: f64,
        z0_by_port: &[f64],
        touchstone_version: usize,
    ) -> Result<WaveformDataset, String> {
        let (frequencies, waveforms) = match result {
            crate::simulation::SimulationResult::Ac {
                frequencies,
                waveforms,
                ..
            } => (frequencies, waveforms),
            _ => return Err("result is not frequency-domain S-parameter data".to_string()),
        };
        if frequencies.is_empty() {
            return Err("frequency vector is empty".to_string());
        }

        let mut entries: std::collections::HashMap<
            (usize, usize),
            &crate::simulation::results::WaveformData,
        > = std::collections::HashMap::new();
        let mut max_port = 0usize;
        for (name, waveform) in waveforms {
            let matrix_index = Self::parse_sparameter_waveform_name(name)
                .or_else(|| Self::parse_sparameter_waveform_name(&waveform.name));
            let Some((row, col)) = matrix_index else {
                continue;
            };
            if entries.insert((row, col), waveform).is_some() {
                return Err(format!(
                    "duplicate S-parameter waveform for S{}{}",
                    row, col
                ));
            }
            max_port = max_port.max(row).max(col);
        }
        if max_port < 2 {
            return Err("no complete S-parameter matrix waveforms found".to_string());
        }
        let port_references = if z0_by_port.is_empty() {
            vec![z0; max_port]
        } else if z0_by_port.len() == max_port {
            z0_by_port.to_vec()
        } else {
            return Err(format!(
                "expected {} per-port reference values, got {}",
                max_port,
                z0_by_port.len()
            ));
        };
        for (idx, value) in port_references.iter().enumerate() {
            if !value.is_finite() || *value <= 0.0 {
                return Err(format!(
                    "invalid Touchstone reference impedance for port {}",
                    idx + 1
                ));
            }
        }
        let has_non_uniform_reference = port_references
            .iter()
            .any(|value| (*value - port_references[0]).abs() > 1e-18);
        if touchstone_version < 2 && has_non_uniform_reference {
            return Err(
                "Touchstone v1 export does not support per-port reference impedance".to_string(),
            );
        }

        let mut dataset = WaveformDataset::new("S-Parameters");
        dataset.analysis = "S-Parameter".to_string();
        dataset
            .metadata
            .insert("z0".to_string(), format!("{}", port_references[0]));
        dataset.metadata.insert(
            "z0_ports".to_string(),
            port_references
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        dataset
            .metadata
            .insert("num_ports".to_string(), max_port.to_string());
        dataset.metadata.insert(
            "touchstone_version".to_string(),
            touchstone_version.to_string(),
        );

        let mut x = WaveformSignal::new("frequency", SignalType::Frequency);
        x.data = frequencies.clone();
        dataset.set_x(x);

        for row in 1..=max_port {
            for col in 1..=max_port {
                let name = Self::sparameter_name(row, col, max_port);
                let waveform = entries
                    .get(&(row, col))
                    .copied()
                    .ok_or_else(|| format!("missing {} waveform", name))?;
                let imag = waveform
                    .y_imag
                    .as_ref()
                    .ok_or_else(|| format!("{} waveform is missing imaginary component", name))?;
                if waveform.y_values.len() != frequencies.len() || imag.len() != frequencies.len() {
                    return Err(format!(
                        "{} waveform length mismatch (freq={}, re={}, im={})",
                        name,
                        frequencies.len(),
                        waveform.y_values.len(),
                        imag.len()
                    ));
                }
                Self::push_complex_signal_pair(&mut dataset, &name, waveform)?;
            }
        }

        Ok(dataset)
    }

    fn push_complex_signal_pair(
        dataset: &mut WaveformDataset,
        name: &str,
        waveform: &crate::simulation::results::WaveformData,
    ) -> Result<(), String> {
        let imag = waveform
            .y_imag
            .as_ref()
            .ok_or_else(|| format!("{} waveform is missing imaginary component", name))?;

        let mut real_signal = WaveformSignal::new(format!("{}_RE", name), SignalType::SParameter);
        real_signal.data = waveform.y_values.clone();
        dataset.add_signal(real_signal);

        let mut imag_signal = WaveformSignal::new(format!("{}_IM", name), SignalType::SParameter);
        imag_signal.data = imag.clone();
        dataset.add_signal(imag_signal);

        Ok(())
    }

    fn parse_sparameter_waveform_name(name: &str) -> Option<(usize, usize)> {
        let normalized = name.trim().to_ascii_uppercase().replace(' ', "");
        let rest = normalized.strip_prefix('S')?;
        if let Some(inner) = rest
            .strip_prefix('(')
            .and_then(|value| value.strip_suffix(')'))
        {
            let (row, col) = inner.split_once(',')?;
            let row = row.trim().parse::<usize>().ok()?;
            let col = col.trim().parse::<usize>().ok()?;
            return (row > 0 && col > 0).then_some((row, col));
        }
        if let Some((row, col)) = rest.split_once('_') {
            let row = row.trim().parse::<usize>().ok()?;
            let col = col.trim().parse::<usize>().ok()?;
            return (row > 0 && col > 0).then_some((row, col));
        }
        if rest.len() == 2 && rest.chars().all(|ch| ch.is_ascii_digit()) {
            let row = rest[0..1].parse::<usize>().ok()?;
            let col = rest[1..2].parse::<usize>().ok()?;
            return Some((row, col));
        }
        None
    }

    fn sparameter_name(row: usize, col: usize, num_ports: usize) -> String {
        if num_ports <= 9 {
            format!("S{}{}", row, col)
        } else {
            format!("S{}_{}", row, col)
        }
    }

    pub(super) fn touchstone_export_path(
        state: &AppState,
        run_id: u64,
        analysis_idx: usize,
        num_ports: usize,
    ) -> PathBuf {
        let source_path = state.schematic.current_file.as_ref();
        let (base_dir, stem) = if let Some(path) = source_path {
            let dir = path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .filter(|s| !s.is_empty())
                .unwrap_or("untitled");
            (dir, stem.to_string())
        } else {
            (
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                "untitled".to_string(),
            )
        };

        base_dir.join(format!(
            "{}_run{:04}_sp{:02}.s{}p",
            stem,
            run_id,
            analysis_idx.max(1),
            num_ports.max(2)
        ))
    }
}

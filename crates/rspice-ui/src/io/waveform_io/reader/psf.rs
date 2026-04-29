use super::*;

impl WaveformReader {
    /// Read PSF-Lite binary waveform format (`PSFL`).
    ///
    /// Supports both:
    /// - `PSFL` binary files emitted by rspice (`psf-lite`)
    /// - Cadence native PSF binary files
    /// - Cadence-style PSF ASCII exports (`psfascii`) from file or directory targets
    pub(super) fn read_psf(&self, path: &Path) -> Result<WaveformDataset, String> {
        if path.is_dir() {
            return self.read_psf_directory(path);
        }
        if !path.is_file() {
            return Err(format!(
                "PSF path '{}' is neither a file nor a directory",
                path.display()
            ));
        }

        // First try rspice PSF-Lite binary.
        match self.read_psf_lite_file(path) {
            Ok(dataset) => Ok(dataset),
            Err(psf_lite_err) => {
                // Then try Cadence native binary PSF.
                match self.read_cadence_psf_binary_file(path) {
                    Ok(dataset) => Ok(dataset),
                    Err(cadence_bin_err) => {
                        // Finally fall back to PSF ASCII text parsing.
                        match self.read_psf_ascii_file(path) {
                            Ok(dataset) => Ok(dataset),
                            Err(psf_ascii_err) => Err(format!(
                                "Failed to read PSF '{}': {}; Cadence PSF binary parse failed: {}; PSF ASCII parse failed: {}",
                                path.display(),
                                psf_lite_err,
                                cadence_bin_err,
                                psf_ascii_err
                            )),
                        }
                    }
                }
            }
        }
    }

    fn read_psf_lite_file(&self, path: &Path) -> Result<WaveformDataset, String> {
        let mut reader = PsfReader::open(path)
            .map_err(|e| format!("Failed to open PSF-Lite file '{}': {}", path.display(), e))?;

        let header = reader.header().clone();
        if header.num_traces == 0 {
            return Err("PSF-Lite file contains zero traces".to_string());
        }

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("psf"));
        dataset.analysis = "PSF-Lite".to_string();
        dataset
            .metadata
            .insert("format".to_string(), "psf-lite".to_string());
        dataset
            .metadata
            .insert("num_traces".to_string(), header.num_traces.to_string());
        dataset
            .metadata
            .insert("num_points".to_string(), header.num_points.to_string());
        dataset
            .metadata
            .insert("timestamp".to_string(), header.timestamp.to_string());

        let mut x = WaveformSignal::new("time", SignalType::Time);
        x.data = reader
            .read_trace(0)
            .map_err(|e| format!("Failed to read PSF-Lite trace 0: {}", e))?;
        dataset.set_x(x);

        for trace_idx in 1..header.num_traces {
            let mut signal =
                WaveformSignal::new(format!("trace{}", trace_idx), SignalType::Unknown);
            signal.data = reader
                .read_trace(trace_idx)
                .map_err(|e| format!("Failed to read PSF-Lite trace {}: {}", trace_idx, e))?;
            dataset.add_signal(signal);
        }

        Ok(dataset)
    }

    fn read_cadence_psf_binary_file(&self, path: &Path) -> Result<WaveformDataset, String> {
        let bytes = fs::read(path)
            .map_err(|e| format!("Failed to read PSF file '{}': {}", path.display(), e))?;

        let parsed = std::panic::catch_unwind(|| parse_cadence_psf_binary(&bytes));
        let parsed = match parsed {
            Ok(Ok(parsed)) => parsed,
            Ok(Err(e)) => {
                return Err(format!(
                    "Cadence PSF binary parser error for '{}': {}",
                    path.display(),
                    e
                ));
            }
            Err(_) => {
                return Err(format!(
                    "Cadence PSF binary parser panicked for '{}'",
                    path.display()
                ));
            }
        };

        self.cadence_psf_binary_to_dataset(path, parsed)
    }

    fn cadence_psf_binary_to_dataset(
        &self,
        path: &Path,
        parsed: ParsedCadencePsfBinary,
    ) -> Result<WaveformDataset, String> {
        let has_declared_sweeps = !parsed.sweeps.is_empty();
        let mut sweep_traces: Vec<(String, Vec<f64>)> = parsed
            .sweeps
            .into_iter()
            .filter_map(|sweep| (!sweep.values.is_empty()).then_some((sweep.name, sweep.values)))
            .collect();
        let mut real_traces: HashMap<String, Vec<f64>> = parsed
            .real_signals
            .into_iter()
            .map(|signal| (signal.name, signal.values))
            .collect();
        let complex_traces: HashMap<String, Vec<(f64, f64)>> = parsed
            .complex_signals
            .into_iter()
            .map(|signal| (signal.name, signal.values))
            .collect();

        if real_traces.is_empty() && complex_traces.is_empty() && sweep_traces.is_empty() {
            return Err(format!(
                "Cadence PSF binary file '{}' contained no traces",
                path.display()
            ));
        }

        let mut x_candidate = sweep_traces
            .iter()
            .position(|(name, _)| name.eq_ignore_ascii_case("time"))
            .or_else(|| {
                sweep_traces.iter().position(|(name, _)| {
                    name.eq_ignore_ascii_case("freq") || name.eq_ignore_ascii_case("frequency")
                })
            })
            .or_else(|| (!sweep_traces.is_empty()).then_some(0))
            .map(|idx| sweep_traces.swap_remove(idx));

        if x_candidate.is_none() {
            let x_from_real = real_traces
                .iter()
                .find(|(name, values)| name.eq_ignore_ascii_case("time") && !values.is_empty())
                .map(|(name, _)| name.clone())
                .or_else(|| {
                    real_traces
                        .iter()
                        .find(|(name, values)| {
                            (name.eq_ignore_ascii_case("freq")
                                || name.eq_ignore_ascii_case("frequency"))
                                && !values.is_empty()
                        })
                        .map(|(name, _)| name.clone())
                });
            if let Some(name) = x_from_real
                && let Some(values) = real_traces.remove(&name)
            {
                x_candidate = Some((name, values));
            }
        }

        let (x_name, x_values) = if let Some((name, values)) = x_candidate {
            (name, values)
        } else {
            let max_len = real_traces
                .values()
                .map(Vec::len)
                .chain(complex_traces.values().map(Vec::len))
                .max()
                .unwrap_or(0);
            if max_len == 0 {
                return Err(format!(
                    "Cadence PSF binary file '{}' has no usable sample vectors",
                    path.display()
                ));
            }
            (
                "index".to_string(),
                (0..max_len).map(|i| i as f64).collect(),
            )
        };

        let x_len = x_values.len();
        if x_len == 0 {
            return Err(format!(
                "Cadence PSF binary file '{}' has empty independent variable '{}'",
                path.display(),
                x_name
            ));
        }

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("psf"));
        dataset.analysis = if x_name.eq_ignore_ascii_case("time") {
            "Transient".to_string()
        } else if x_name.eq_ignore_ascii_case("freq") || x_name.eq_ignore_ascii_case("frequency") {
            "AC".to_string()
        } else if !has_declared_sweeps && x_name == "index" && x_len <= 1 {
            "DC OP".to_string()
        } else if has_declared_sweeps {
            "DC Sweep".to_string()
        } else {
            "PSF-Binary".to_string()
        };
        dataset
            .metadata
            .insert("format".to_string(), "psf-binary-cadence".to_string());
        dataset
            .metadata
            .insert("source_path".to_string(), path.display().to_string());

        let x_signal_type = if x_name.eq_ignore_ascii_case("time") {
            SignalType::Time
        } else if x_name.eq_ignore_ascii_case("freq") || x_name.eq_ignore_ascii_case("frequency") {
            SignalType::Frequency
        } else {
            SignalType::Unknown
        };
        let mut x_signal = WaveformSignal::new(x_name.clone(), x_signal_type);
        x_signal.data = x_values;
        dataset.set_x(x_signal);

        let mut real_names: Vec<_> = real_traces.keys().cloned().collect();
        real_names.sort();
        for signal_name in real_names {
            if signal_name == x_name {
                continue;
            }
            let Some(values) = real_traces.get(&signal_name) else {
                continue;
            };
            if values.len() != x_len {
                continue;
            }
            let mut signal =
                WaveformSignal::new(signal_name.clone(), Self::infer_signal_type(&signal_name));
            signal.data = values.clone();
            dataset.add_signal(signal);
        }

        let mut complex_names: Vec<_> = complex_traces.keys().cloned().collect();
        complex_names.sort();
        for signal_name in complex_names {
            let Some(values) = complex_traces.get(&signal_name) else {
                continue;
            };
            if values.len() != x_len {
                continue;
            }

            let mut real = WaveformSignal::new(
                format!("{}_RE", signal_name),
                Self::infer_complex_signal_type(&signal_name, false),
            );
            let mut imag = WaveformSignal::new(
                format!("{}_IM", signal_name),
                Self::infer_complex_signal_type(&signal_name, true),
            );
            real.data = values.iter().map(|(re, _)| *re).collect();
            imag.data = values.iter().map(|(_, im)| *im).collect();
            dataset.add_signal(real);
            dataset.add_signal(imag);
        }

        if dataset.signals.is_empty() {
            return Err(format!(
                "Cadence PSF binary file '{}' had no traces aligned to '{}'",
                path.display(),
                x_name
            ));
        }

        Ok(dataset)
    }

    fn read_psf_directory(&self, path: &Path) -> Result<WaveformDataset, String> {
        let mut candidates = Vec::new();

        // Prefer explicitly referenced run objects from logFile when available.
        let log_file = path.join("logFile");
        if log_file.is_file() {
            let file = File::open(&log_file).map_err(|e| {
                format!("Failed to open PSF logFile '{}': {}", log_file.display(), e)
            })?;
            for line in BufReader::new(file).lines() {
                let line =
                    line.map_err(|e| format!("Failed to read '{}': {}", log_file.display(), e))?;
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                for token in trimmed.split_whitespace() {
                    let token = token.trim_matches(|c| c == '"' || c == '\'');
                    if token.ends_with(".psfascii")
                        || token.ends_with(".ascii")
                        || token.ends_with(".txt")
                        || token.ends_with(".psf")
                    {
                        let candidate = path.join(token);
                        if candidate.is_file() {
                            candidates.push(candidate);
                        }
                    }
                }
            }
        }

        // Also scan direct children for common waveform payload files.
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to scan PSF directory '{}': {}", path.display(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let child_path = entry.path();
            if !child_path.is_file() {
                continue;
            }
            let Some(name) = child_path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            let lower = name.to_ascii_lowercase();
            let likely_payload = lower.ends_with(".psfascii")
                || lower.ends_with(".ascii")
                || lower.ends_with(".psf")
                || lower.contains("tran")
                || lower.contains("ac")
                || lower.contains("dc");
            if likely_payload {
                candidates.push(child_path);
            }
        }

        candidates.sort();
        candidates.dedup();

        let mut errors = Vec::new();
        for candidate in candidates {
            match self.read_psf_lite_file(&candidate) {
                Ok(dataset) => return Ok(dataset),
                Err(psf_lite_err) => match self.read_cadence_psf_binary_file(&candidate) {
                    Ok(dataset) => return Ok(dataset),
                    Err(cadence_bin_err) => match self.read_psf_ascii_file(&candidate) {
                        Ok(dataset) => return Ok(dataset),
                        Err(psf_ascii_err) => errors.push(format!(
                            "{}: {}; {}; {}",
                            candidate.display(),
                            psf_lite_err,
                            cadence_bin_err,
                            psf_ascii_err
                        )),
                    },
                },
            }
        }

        Err(format!(
            "No readable PSF waveform payload found in '{}'. Tried: {}",
            path.display(),
            if errors.is_empty() {
                "none".to_string()
            } else {
                errors.join(" | ")
            }
        ))
    }

    fn read_psf_ascii_file(&self, path: &Path) -> Result<WaveformDataset, String> {
        let file = File::open(path)
            .map_err(|e| format!("Failed to open PSF ASCII file '{}': {}", path.display(), e))?;
        let reader = BufReader::new(file);

        let mut traces: HashMap<String, Vec<f64>> = HashMap::new();
        let mut vector_name: Option<String> = None;
        let mut vector_values: Vec<f64> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Read error in '{}': {}", path.display(), e))?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }

            if let Some(active_name) = vector_name.as_deref() {
                if trimmed.starts_with(')') {
                    Self::commit_psf_ascii_sample(&mut traces, active_name, &vector_values);
                    vector_name = None;
                    vector_values.clear();
                    continue;
                }
                vector_values.extend(Self::parse_psf_ascii_numbers(trimmed));
                if trimmed.ends_with(')') {
                    Self::commit_psf_ascii_sample(&mut traces, active_name, &vector_values);
                    vector_name = None;
                    vector_values.clear();
                }
                continue;
            }

            let Some((name, rhs)) = Self::parse_psf_ascii_assignment(trimmed) else {
                continue;
            };
            if name.is_empty() {
                continue;
            }

            if rhs.starts_with('(') {
                let mut values = Self::parse_psf_ascii_numbers(rhs);
                if rhs.ends_with(')') {
                    Self::commit_psf_ascii_sample(&mut traces, &name, &values);
                } else {
                    vector_name = Some(name);
                    vector_values.append(&mut values);
                }
            } else {
                let values = Self::parse_psf_ascii_numbers(rhs);
                Self::commit_psf_ascii_sample(&mut traces, &name, &values);
            }
        }

        if let Some(active_name) = vector_name
            && !vector_values.is_empty()
        {
            Self::commit_psf_ascii_sample(&mut traces, &active_name, &vector_values);
        }

        if traces.is_empty() {
            return Err(format!(
                "PSF ASCII file '{}' contained no waveform assignments",
                path.display()
            ));
        }

        let mut x_name = None;
        for candidate in ["time", "freq", "frequency", "sweep", "sweepparam"] {
            if let Some((name, values)) = traces
                .iter()
                .find(|(name, values)| name.eq_ignore_ascii_case(candidate) && values.len() >= 2)
            {
                x_name = Some((name.clone(), values.clone()));
                break;
            }
        }
        if x_name.is_none() {
            x_name = traces
                .iter()
                .max_by_key(|(_, values)| values.len())
                .map(|(name, values)| (name.clone(), values.clone()));
        }

        let (x_name, x_values) = x_name.ok_or_else(|| {
            format!(
                "PSF ASCII file '{}' did not expose a usable independent variable",
                path.display()
            )
        })?;
        let x_len = x_values.len();
        if x_len == 0 {
            return Err(format!(
                "PSF ASCII file '{}' has empty independent variable '{}'",
                path.display(),
                x_name
            ));
        }

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("psf"));
        dataset.analysis = if x_name.eq_ignore_ascii_case("time") {
            "Transient".to_string()
        } else {
            "PSF-ASCII".to_string()
        };
        dataset
            .metadata
            .insert("format".to_string(), "psf-ascii".to_string());
        dataset
            .metadata
            .insert("source_path".to_string(), path.display().to_string());

        let mut x_signal = WaveformSignal::new(
            x_name.clone(),
            if x_name.eq_ignore_ascii_case("time") {
                SignalType::Time
            } else {
                SignalType::Frequency
            },
        );
        x_signal.data = x_values;
        dataset.set_x(x_signal);

        let mut signal_names: Vec<_> = traces.keys().cloned().collect();
        signal_names.sort();

        for signal_name in signal_names {
            if signal_name == x_name {
                continue;
            }
            let Some(values) = traces.get(&signal_name) else {
                continue;
            };
            if values.len() != x_len {
                // Keep strict x/y alignment for plotting correctness.
                continue;
            }
            let signal_type = if signal_name.to_ascii_lowercase().starts_with("v(") {
                SignalType::Voltage
            } else if signal_name.to_ascii_lowercase().starts_with("i(") {
                SignalType::Current
            } else {
                SignalType::Unknown
            };
            let mut signal = WaveformSignal::new(signal_name, signal_type);
            signal.data = values.clone();
            dataset.add_signal(signal);
        }

        if dataset.signals.is_empty() {
            return Err(format!(
                "PSF ASCII file '{}' had no signals aligned to independent variable '{}'",
                path.display(),
                x_name
            ));
        }

        Ok(dataset)
    }

    fn parse_psf_ascii_assignment(line: &str) -> Option<(String, &str)> {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('"') {
            return None;
        }
        let mut chars = trimmed.char_indices();
        chars.next()?; // opening quote
        let end_quote = chars.find_map(|(idx, ch)| (ch == '"').then_some(idx))?;
        let name = trimmed[1..end_quote].trim().to_string();
        let rhs = trimmed[end_quote + 1..].trim_start();
        let rhs = rhs.strip_prefix('=').map(str::trim_start).unwrap_or(rhs);
        Some((name, rhs))
    }

    fn parse_psf_ascii_numbers(text: &str) -> Vec<f64> {
        text.split(|c: char| c.is_ascii_whitespace() || matches!(c, ',' | '(' | ')' | ';'))
            .filter_map(|token| {
                let token = token.trim();
                if token.is_empty() {
                    return None;
                }
                token.parse::<f64>().ok()
            })
            .collect()
    }

    fn commit_psf_ascii_sample(traces: &mut HashMap<String, Vec<f64>>, name: &str, values: &[f64]) {
        match values {
            [] => {}
            [value] => {
                traces.entry(name.to_string()).or_default().push(*value);
            }
            [re, im] => {
                traces.entry(format!("{}_RE", name)).or_default().push(*re);
                traces.entry(format!("{}_IM", name)).or_default().push(*im);
            }
            _ => {
                traces
                    .entry(name.to_string())
                    .or_default()
                    .extend_from_slice(values);
            }
        }
    }

    fn infer_signal_type(name: &str) -> SignalType {
        let lower = name.to_ascii_lowercase();
        if lower.starts_with("v(") {
            SignalType::Voltage
        } else if lower.starts_with("i(") {
            SignalType::Current
        } else if lower == "time" {
            SignalType::Time
        } else if lower == "freq" || lower == "frequency" {
            SignalType::Frequency
        } else {
            SignalType::Unknown
        }
    }

    fn infer_complex_signal_type(name: &str, imag: bool) -> SignalType {
        let lower = name.to_ascii_lowercase();
        if lower.starts_with("v(") {
            if imag {
                SignalType::VoltageImag
            } else {
                SignalType::VoltageReal
            }
        } else if lower.starts_with("i(") {
            if imag {
                SignalType::CurrentImag
            } else {
                SignalType::CurrentReal
            }
        } else {
            SignalType::Unknown
        }
    }
}

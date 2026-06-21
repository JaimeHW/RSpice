use super::*;

#[derive(Debug, Clone)]
struct PsfAsciiRecord {
    line: usize,
    values: Vec<f64>,
}

#[derive(Debug)]
enum PsfAsciiTrace {
    Real(Vec<f64>),
    Complex { real: Vec<f64>, imag: Vec<f64> },
}

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
        let x_data = reader
            .read_trace(0)
            .map_err(|e| format!("Failed to read PSF-Lite trace 0: {}", e))?;
        validate_psf_lite_trace_values("trace0", &x_data)?;
        x.data = x_data;
        dataset.set_x(x);

        for trace_idx in 1..header.num_traces {
            let signal_name = format!("trace{}", trace_idx);
            let mut signal = WaveformSignal::new(&signal_name, SignalType::Unknown);
            let data = reader
                .read_trace(trace_idx)
                .map_err(|e| format!("Failed to read PSF-Lite trace {}: {}", trace_idx, e))?;
            validate_psf_lite_trace_values(&signal_name, &data)?;
            signal.data = data;
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
        validate_cadence_psf_real_trace_values(&x_name, &x_values)?;

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
                return Err(format!(
                    "Cadence PSF binary trace '{}' has {} samples but independent variable '{}' has {}",
                    signal_name,
                    values.len(),
                    x_name,
                    x_len
                ));
            }
            validate_cadence_psf_real_trace_values(&signal_name, values)?;
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
                return Err(format!(
                    "Cadence PSF binary trace '{}' has {} samples but independent variable '{}' has {}",
                    signal_name,
                    values.len(),
                    x_name,
                    x_len
                ));
            }
            validate_cadence_psf_complex_trace_values(&signal_name, values)?;

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

        let mut traces: HashMap<String, Vec<PsfAsciiRecord>> = HashMap::new();
        let mut vector_name: Option<String> = None;
        let mut vector_values: Vec<f64> = Vec::new();
        let mut vector_line = 0usize;

        for (line_idx, line) in reader.lines().enumerate() {
            let line_num = line_idx + 1;
            let line = line.map_err(|e| format!("Read error in '{}': {}", path.display(), e))?;
            let stripped = Self::strip_psf_ascii_comment(&line);
            let trimmed = stripped.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }

            if let Some(active_name) = vector_name.as_deref() {
                if trimmed.starts_with(')') {
                    Self::commit_psf_ascii_record(
                        &mut traces,
                        active_name,
                        vector_line,
                        &vector_values,
                    );
                    vector_name = None;
                    vector_values.clear();
                    vector_line = 0;
                    continue;
                }
                vector_values.extend(Self::parse_psf_ascii_numbers(
                    trimmed,
                    active_name,
                    line_num,
                )?);
                if trimmed.ends_with(')') {
                    Self::commit_psf_ascii_record(
                        &mut traces,
                        active_name,
                        vector_line,
                        &vector_values,
                    );
                    vector_name = None;
                    vector_values.clear();
                    vector_line = 0;
                }
                continue;
            }

            let Some((name, rhs)) = Self::parse_psf_ascii_assignment(trimmed) else {
                return Err(format!(
                    "PSF ASCII file '{}' line {} has unrecognized content: {}",
                    path.display(),
                    line_num,
                    trimmed
                ));
            };
            if name.is_empty() {
                continue;
            }

            if rhs.starts_with('(') {
                let mut values = Self::parse_psf_ascii_numbers(rhs, &name, line_num)?;
                if rhs.ends_with(')') {
                    Self::commit_psf_ascii_record(&mut traces, &name, line_num, &values);
                } else {
                    vector_name = Some(name);
                    vector_line = line_num;
                    vector_values.append(&mut values);
                }
            } else {
                let values = Self::parse_psf_ascii_numbers(rhs, &name, line_num)?;
                Self::commit_psf_ascii_record(&mut traces, &name, line_num, &values);
            }
        }

        if let Some(active_name) = vector_name {
            return Err(format!(
                "PSF ASCII trace '{}' line {} has unterminated vector; missing ')'",
                active_name, vector_line
            ));
        }

        if traces.is_empty() {
            return Err(format!(
                "PSF ASCII file '{}' contained no waveform assignments",
                path.display()
            ));
        }

        let mut x_name = None;
        for candidate in ["time", "freq", "frequency", "sweep", "sweepparam"] {
            if let Some((name, records)) = traces
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(candidate))
            {
                let values = Self::normalize_psf_ascii_axis(name, records, path)?;
                x_name = Some((name.clone(), values));
                break;
            }
        }
        if x_name.is_none() {
            let fallback = traces
                .iter()
                .max_by_key(|(name, records)| (Self::psf_ascii_sample_count(records), *name))
                .map(|(name, records)| (name.clone(), records.clone()));
            if let Some((name, records)) = fallback {
                let values = Self::normalize_psf_ascii_axis(&name, &records, path)?;
                x_name = Some((name, values));
            }
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
            let Some(records) = traces.get(&signal_name) else {
                continue;
            };
            if let Some(value) = Self::psf_ascii_scalar_metadata(records, x_len, &signal_name) {
                dataset
                    .metadata
                    .insert(format!("psf_scalar.{}", signal_name), value.to_string());
                continue;
            }
            match Self::normalize_psf_ascii_trace(path, &x_name, x_len, &signal_name, records)? {
                PsfAsciiTrace::Real(values) => {
                    let mut signal = WaveformSignal::new(
                        signal_name.clone(),
                        Self::infer_signal_type(&signal_name),
                    );
                    signal.data = values;
                    dataset.add_signal(signal);
                }
                PsfAsciiTrace::Complex { real, imag } => {
                    let mut real_signal = WaveformSignal::new(
                        format!("{}_RE", signal_name),
                        Self::infer_complex_signal_type(&signal_name, false),
                    );
                    real_signal.data = real;
                    dataset.add_signal(real_signal);

                    let mut imag_signal = WaveformSignal::new(
                        format!("{}_IM", signal_name),
                        Self::infer_complex_signal_type(&signal_name, true),
                    );
                    imag_signal.data = imag;
                    dataset.add_signal(imag_signal);
                }
            }
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

    fn parse_psf_ascii_numbers(
        text: &str,
        trace_name: &str,
        line_num: usize,
    ) -> Result<Vec<f64>, String> {
        let mut values = Vec::new();
        for token in
            text.split(|c: char| c.is_ascii_whitespace() || matches!(c, ',' | '(' | ')' | ';'))
        {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let value = token.parse::<f64>().map_err(|_| {
                format!(
                    "PSF ASCII trace '{}' line {} has invalid numeric token '{}'",
                    trace_name, line_num, token
                )
            })?;
            if !value.is_finite() {
                return Err(format!(
                    "PSF ASCII trace '{}' line {} has non-finite numeric token '{}'",
                    trace_name, line_num, token
                ));
            }
            values.push(value);
        }
        Ok(values)
    }

    fn commit_psf_ascii_record(
        traces: &mut HashMap<String, Vec<PsfAsciiRecord>>,
        name: &str,
        line: usize,
        values: &[f64],
    ) {
        if !values.is_empty() {
            traces
                .entry(name.to_string())
                .or_default()
                .push(PsfAsciiRecord {
                    line,
                    values: values.to_vec(),
                });
        }
    }

    fn normalize_psf_ascii_axis(
        name: &str,
        records: &[PsfAsciiRecord],
        path: &Path,
    ) -> Result<Vec<f64>, String> {
        if records.is_empty() {
            return Err(format!(
                "PSF ASCII file '{}' has empty independent variable '{}'",
                path.display(),
                name
            ));
        }

        if records.len() == 1 {
            let values = records[0].values.clone();
            if values.is_empty() {
                return Err(format!(
                    "PSF ASCII file '{}' has empty independent variable '{}'",
                    path.display(),
                    name
                ));
            }
            return Ok(values);
        }

        if records.iter().all(|record| record.values.len() == 1) {
            return Ok(records.iter().map(|record| record.values[0]).collect());
        }

        let shapes: Vec<_> = records.iter().map(|record| record.values.len()).collect();
        Err(format!(
            "PSF ASCII independent variable '{}' in '{}' has unsupported record shapes {:?}",
            name,
            path.display(),
            shapes
        ))
    }

    fn normalize_psf_ascii_trace(
        path: &Path,
        x_name: &str,
        x_len: usize,
        name: &str,
        records: &[PsfAsciiRecord],
    ) -> Result<PsfAsciiTrace, String> {
        if records.is_empty() {
            return Err(format!(
                "PSF ASCII trace '{}' in '{}' is empty",
                name,
                path.display()
            ));
        }

        if records.len() == 1 {
            let values = records[0].values.clone();
            if values.len() == x_len {
                return Ok(PsfAsciiTrace::Real(values));
            }
            if values.len() == 2 && x_len == 1 {
                return Ok(PsfAsciiTrace::Complex {
                    real: vec![values[0]],
                    imag: vec![values[1]],
                });
            }
            return Err(format!(
                "PSF ASCII trace '{}' in '{}' has {} value(s), but independent variable '{}' has {} point(s)",
                name,
                path.display(),
                values.len(),
                x_name,
                x_len
            ));
        }

        if records.len() != x_len {
            return Err(format!(
                "PSF ASCII trace '{}' in '{}' has {} records, but independent variable '{}' has {} point(s)",
                name,
                path.display(),
                records.len(),
                x_name,
                x_len
            ));
        }

        if records.iter().all(|record| record.values.len() == 1) {
            return Ok(PsfAsciiTrace::Real(
                records.iter().map(|record| record.values[0]).collect(),
            ));
        }

        if records.iter().all(|record| record.values.len() == 2) {
            return Ok(PsfAsciiTrace::Complex {
                real: records.iter().map(|record| record.values[0]).collect(),
                imag: records.iter().map(|record| record.values[1]).collect(),
            });
        }

        let shapes: Vec<_> = records
            .iter()
            .map(|record| format!("line {}: {}", record.line, record.values.len()))
            .collect();
        Err(format!(
            "PSF ASCII trace '{}' in '{}' has mixed record shapes [{}]",
            name,
            path.display(),
            shapes.join(", ")
        ))
    }

    fn psf_ascii_sample_count(records: &[PsfAsciiRecord]) -> usize {
        if records.len() == 1 {
            records[0].values.len()
        } else {
            records.len()
        }
    }

    fn psf_ascii_scalar_metadata(
        records: &[PsfAsciiRecord],
        x_len: usize,
        name: &str,
    ) -> Option<f64> {
        if x_len <= 1 || records.len() != 1 || records[0].values.len() != 1 {
            return None;
        }
        let lower = name.to_ascii_lowercase();
        let waveform_like = lower.starts_with("v(")
            || lower.starts_with("i(")
            || lower.starts_with('/')
            || lower.starts_with("s(")
            || lower.contains('(');
        (!waveform_like).then_some(records[0].values[0])
    }

    fn strip_psf_ascii_comment(line: &str) -> &str {
        let bytes = line.as_bytes();
        let mut in_quote = false;
        let mut idx = 0usize;
        while idx < bytes.len() {
            match bytes[idx] {
                b'"' => {
                    in_quote = !in_quote;
                    idx += 1;
                }
                b'#' if !in_quote => return &line[..idx],
                b'/' if !in_quote && bytes.get(idx + 1) == Some(&b'/') => return &line[..idx],
                _ => idx += 1,
            }
        }
        line
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

fn validate_psf_lite_trace_values(trace_name: &str, values: &[f64]) -> Result<(), String> {
    if let Some((idx, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(format!(
            "PSF-Lite trace '{trace_name}' sample {idx} is non-finite ({value})"
        ));
    }
    Ok(())
}

fn validate_cadence_psf_real_trace_values(trace_name: &str, values: &[f64]) -> Result<(), String> {
    if let Some((idx, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(format!(
            "Cadence PSF binary trace '{trace_name}' sample {idx} is non-finite ({value})"
        ));
    }
    Ok(())
}

fn validate_cadence_psf_complex_trace_values(
    trace_name: &str,
    values: &[(f64, f64)],
) -> Result<(), String> {
    if let Some((idx, (real, imag))) = values
        .iter()
        .enumerate()
        .find(|(_, (real, imag))| !real.is_finite() || !imag.is_finite())
    {
        return Err(format!(
            "Cadence PSF binary trace '{trace_name}' sample {idx} is non-finite ({real}, {imag})"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::PsfHeader;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is available")
            .as_nanos();
        path.push(format!("rspice-psf-ascii-{name}-{nonce}.psfascii"));
        path
    }

    fn temp_psf_lite_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is available")
            .as_nanos();
        path.push(format!("rspice-psf-lite-{name}-{nonce}.psfl"));
        path
    }

    fn write_temp_psf(name: &str, contents: &str) -> PathBuf {
        let path = temp_path(name);
        fs::write(&path, contents).expect("write temporary PSF ASCII waveform");
        path
    }

    fn write_temp_psf_lite(name: &str, traces: &[&[f64]]) -> PathBuf {
        let path = temp_psf_lite_path(name);
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&PsfHeader::MAGIC);
        bytes.extend_from_slice(&PsfHeader::VERSION.to_le_bytes());
        bytes.extend_from_slice(&(traces.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(traces[0].len() as u32).to_le_bytes());
        bytes.extend_from_slice(&0u64.to_le_bytes());
        for trace in traces {
            for value in *trace {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
        }
        fs::write(&path, bytes).expect("write temporary PSF-Lite waveform");
        path
    }

    #[test]
    fn psf_lite_reader_rejects_nonfinite_trace_values() {
        let path = write_temp_psf_lite("nonfinite-trace", &[&[0.0], &[f64::NAN]]);
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let err = reader
            .read(&path)
            .expect_err("non-finite PSF-Lite trace values must reject import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("trace1") && err.contains("non-finite"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn cadence_psf_binary_rejects_nonfinite_decoded_trace_samples() {
        let parsed = ParsedCadencePsfBinary {
            header: std::collections::HashMap::new(),
            sweeps: vec![crate::io::cadence_psf::NamedRealSignal {
                name: "time".to_string(),
                values: vec![0.0, 1.0, 2.0],
            }],
            real_signals: vec![crate::io::cadence_psf::NamedRealSignal {
                name: "gap".to_string(),
                values: vec![20.0, f64::NAN, 22.0],
            }],
            complex_signals: Vec::new(),
        };
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let err = reader
            .cadence_psf_binary_to_dataset(Path::new("decoded.psf"), parsed)
            .expect_err("non-finite decoded Cadence PSF trace values must reject import");

        assert!(
            err.contains("gap") && err.contains("non-finite"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn cadence_psf_binary_rejects_misaligned_trace_lengths_instead_of_dropping() {
        let parsed = ParsedCadencePsfBinary {
            header: std::collections::HashMap::new(),
            sweeps: vec![crate::io::cadence_psf::NamedRealSignal {
                name: "time".to_string(),
                values: vec![0.0, 1.0, 2.0],
            }],
            real_signals: vec![
                crate::io::cadence_psf::NamedRealSignal {
                    name: "ok".to_string(),
                    values: vec![10.0, 11.0, 12.0],
                },
                crate::io::cadence_psf::NamedRealSignal {
                    name: "short".to_string(),
                    values: vec![30.0, 31.0],
                },
            ],
            complex_signals: Vec::new(),
        };
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let err = reader
            .cadence_psf_binary_to_dataset(Path::new("decoded.psf"), parsed)
            .expect_err("misaligned decoded Cadence PSF traces must reject import");

        assert!(
            err.contains("short") && err.contains("time"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn psf_ascii_keeps_two_point_real_vectors_real() {
        let path = write_temp_psf("two-point-real", "\"time\" (0 1)\n\"V(out)\" (5 6)\n");
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let dataset = reader
            .read(&path)
            .expect("two-point real PSF vectors should import as real traces");
        let _ = fs::remove_file(&path);

        assert_eq!(
            dataset
                .x_signal
                .as_ref()
                .map(|signal| (signal.name.as_str(), signal.data.as_slice())),
            Some(("time", [0.0, 1.0].as_slice()))
        );
        assert_eq!(dataset.signal_names(), vec!["V(out)"]);
        assert_eq!(dataset.signals[0].data, vec![5.0, 6.0]);
    }

    #[test]
    fn psf_ascii_rejects_invalid_numeric_tokens_in_vectors() {
        let path = write_temp_psf(
            "bad-token",
            "\"time\" (0 1 2)\n\"V(out)\" (10 BAD_TOKEN 20 30)\n",
        );
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let err = reader
            .read(&path)
            .expect_err("invalid PSF numeric tokens must reject the import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("V(out)") && err.contains("BAD_TOKEN"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn psf_ascii_rejects_signal_length_mismatches() {
        let path = write_temp_psf(
            "length-mismatch",
            "\"time\" (0 1 2)\n\"V(good)\" (1 2 3)\n\"V(bad)\" (4 5)\n",
        );
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let err = reader
            .read(&path)
            .expect_err("mismatched PSF signal lengths must reject the import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("V(bad)") && err.contains("time"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn psf_ascii_rejects_unterminated_vectors_at_eof() {
        let path = write_temp_psf("unterminated-vector", "\"time\" (0 1)\n\"V(out)\" (2 3\n");
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let err = reader
            .read(&path)
            .expect_err("unterminated PSF ASCII vectors must reject import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("V(out)") && err.contains("unterminated"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn psf_ascii_rejects_unrecognized_non_comment_lines() {
        let path = write_temp_psf(
            "unrecognized-line",
            "\"time\" (0 1)\nthis is not psf syntax\n\"V(out)\" (2 3)\n",
        );
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let err = reader
            .read(&path)
            .expect_err("unrecognized PSF ASCII lines must reject import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("line 2") && err.contains("unrecognized"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn psf_ascii_preserves_scalar_numeric_metadata() {
        let path = write_temp_psf(
            "scalar-metadata",
            "\"time\" (0 1)\n\"temperature\" 27\n\"V(out)\" (5 6)\n",
        );
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let dataset = reader
            .read(&path)
            .expect("scalar PSF metadata should not invalidate aligned traces");
        let _ = fs::remove_file(&path);

        assert_eq!(dataset.signal_names(), vec!["V(out)"]);
        assert_eq!(
            dataset.metadata.get("psf_scalar.temperature"),
            Some(&"27".to_string())
        );
    }

    #[test]
    fn psf_ascii_preserves_comment_markers_inside_quoted_names() {
        let path = write_temp_psf(
            "quoted-comment-marker",
            "\"time\" (0 1)\n\"V(net#1)\" (2 3) # real comment\n\"V(net//2)\" (4 5) // real comment\n",
        );
        let reader = WaveformReader::new(WaveformFormat::Psf);

        let dataset = reader
            .read(&path)
            .expect("quoted comment markers in PSF signal names should import");
        let _ = fs::remove_file(&path);

        assert_eq!(
            dataset
                .get_signal("V(net#1)")
                .map(|signal| signal.data.as_slice()),
            Some([2.0, 3.0].as_slice())
        );
        assert_eq!(
            dataset
                .get_signal("V(net//2)")
                .map(|signal| signal.data.as_slice()),
            Some([4.0, 5.0].as_slice())
        );
    }
}

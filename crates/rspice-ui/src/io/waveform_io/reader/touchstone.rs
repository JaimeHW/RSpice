use super::*;

impl WaveformReader {
    /// Read Touchstone (`.sNp`) S-parameter file.
    ///
    /// Supports Touchstone v1 and v2 with S-parameter data in `RI`, `MA`, and `DB` formats.
    pub(super) fn read_touchstone(&self, path: &Path) -> Result<WaveformDataset, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open: {}", e))?;
        let reader = BufReader::new(file);

        // Touchstone v1 defaults when no option line is provided.
        let mut options = TouchstoneOptions {
            freq_scale_hz: 1.0e9,
            data_format: TouchstoneDataFormat::Ma,
            reference_ohms: 50.0,
        };
        let mut version = 1u32;
        let mut matrix_format = TouchstoneMatrixFormat::Full;
        let mut declared_ports = Self::touchstone_ports_from_extension(path);
        let mut declared_freqs: Option<usize> = None;
        let mut reference_values: Option<Vec<f64>> = None;
        let mut numeric_tokens: Vec<f64> = Vec::new();

        for (line_idx, line_result) in reader.lines().enumerate() {
            let line_no = line_idx + 1;
            let mut line = line_result.map_err(|e| format!("Read error: {}", e))?;
            if let Some((before_comment, _)) = line.split_once('!') {
                line = before_comment.to_string();
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with('#') {
                options = Self::parse_touchstone_option_line(trimmed, line_no)?;
                continue;
            }

            if trimmed.starts_with('[') {
                let (section, value) = Self::parse_touchstone_section_line(trimmed, line_no)?;
                match section.as_str() {
                    "version" => {
                        let parsed = value.parse::<f64>().map_err(|_| {
                            format!("Touchstone line {}: invalid [Version] '{}'", line_no, value)
                        })?;
                        if !parsed.is_finite() || parsed < 1.0 {
                            return Err(format!(
                                "Touchstone line {}: invalid [Version] '{}'",
                                line_no, value
                            ));
                        }
                        version = parsed.floor() as u32;
                    }
                    "number of ports" => {
                        declared_ports = Some(value.parse::<usize>().map_err(|_| {
                            format!(
                                "Touchstone line {}: invalid [Number of Ports] '{}'",
                                line_no, value
                            )
                        })?);
                    }
                    "number of frequencies" => {
                        declared_freqs = Some(value.parse::<usize>().map_err(|_| {
                            format!(
                                "Touchstone line {}: invalid [Number of Frequencies] '{}'",
                                line_no, value
                            )
                        })?);
                    }
                    "matrix format" => {
                        matrix_format = match value.to_ascii_lowercase().as_str() {
                            "full" => TouchstoneMatrixFormat::Full,
                            "lower" => TouchstoneMatrixFormat::Lower,
                            "upper" => TouchstoneMatrixFormat::Upper,
                            _ => {
                                return Err(format!(
                                    "Touchstone line {}: unsupported [Matrix Format] '{}'",
                                    line_no, value
                                ));
                            }
                        };
                    }
                    "reference" => {
                        reference_values =
                            Some(Self::parse_touchstone_numeric_values(value, line_no)?);
                    }
                    // Section present for clarity in v2 files; data parser is token-stream based.
                    "network data" | "end" => {}
                    _ => {}
                }
                continue;
            }

            for token in trimmed.split_whitespace() {
                if token == "+" {
                    continue;
                }
                let value = Self::parse_touchstone_numeric_token(token).ok_or_else(|| {
                    format!(
                        "Touchstone line {}: expected numeric token, got '{}'",
                        line_no, token
                    )
                })?;
                numeric_tokens.push(value);
            }
        }

        let num_ports = declared_ports
            .or_else(|| Self::infer_touchstone_ports_from_tokens(&numeric_tokens, matrix_format))
            .ok_or_else(|| "Unable to determine Touchstone port count".to_string())?;
        if num_ports == 0 {
            return Err("Touchstone [Number of Ports] must be >= 1".to_string());
        }

        let values_per_freq = Self::touchstone_values_per_frequency(num_ports, matrix_format)
            .ok_or_else(|| "Touchstone matrix dimensions overflow".to_string())?;
        if !crate::utils::numeric::is_multiple_of(numeric_tokens.len(), values_per_freq) {
            return Err(format!(
                "Touchstone numeric data length {} is not divisible by record width {}",
                numeric_tokens.len(),
                values_per_freq
            ));
        }
        let num_freqs = numeric_tokens.len() / values_per_freq;
        if num_freqs == 0 {
            return Err("Touchstone file contains no network data points".to_string());
        }
        if let Some(expected_freqs) = declared_freqs
            && expected_freqs != num_freqs
        {
            return Err(format!(
                "Touchstone [Number of Frequencies]={} but parsed {} records",
                expected_freqs, num_freqs
            ));
        }

        let z0_by_port = Self::resolve_touchstone_reference_values(
            num_ports,
            options.reference_ohms,
            reference_values.as_deref(),
        )?;

        let mut frequencies = Vec::with_capacity(num_freqs);
        let mut matrix_re = vec![vec![vec![0.0; num_freqs]; num_ports]; num_ports];
        let mut matrix_im = vec![vec![vec![0.0; num_freqs]; num_ports]; num_ports];

        let mut offset = 0usize;
        for freq_idx in 0..num_freqs {
            let freq_hz = numeric_tokens[offset] * options.freq_scale_hz;
            offset += 1;
            if !freq_hz.is_finite() || freq_hz <= 0.0 {
                return Err(format!(
                    "Touchstone frequency point {} is invalid ({})",
                    freq_idx, freq_hz
                ));
            }
            frequencies.push(freq_hz);

            // Touchstone matrix order: S11 S21 ... SN1 S12 S22 ... SN2 ... SNN.
            for col in 0..num_ports {
                match matrix_format {
                    TouchstoneMatrixFormat::Full => {
                        for row in 0..num_ports {
                            let first = numeric_tokens[offset];
                            let second = numeric_tokens[offset + 1];
                            offset += 2;
                            let (re, im) = Self::touchstone_pair_to_complex(
                                first,
                                second,
                                options.data_format,
                            );
                            matrix_re[row][col][freq_idx] = re;
                            matrix_im[row][col][freq_idx] = im;
                        }
                    }
                    TouchstoneMatrixFormat::Lower => {
                        for row in col..num_ports {
                            let first = numeric_tokens[offset];
                            let second = numeric_tokens[offset + 1];
                            offset += 2;
                            let (re, im) = Self::touchstone_pair_to_complex(
                                first,
                                second,
                                options.data_format,
                            );
                            matrix_re[row][col][freq_idx] = re;
                            matrix_im[row][col][freq_idx] = im;
                        }
                    }
                    TouchstoneMatrixFormat::Upper => {
                        for row in 0..=col {
                            let first = numeric_tokens[offset];
                            let second = numeric_tokens[offset + 1];
                            offset += 2;
                            let (re, im) = Self::touchstone_pair_to_complex(
                                first,
                                second,
                                options.data_format,
                            );
                            matrix_re[row][col][freq_idx] = re;
                            matrix_im[row][col][freq_idx] = im;
                        }
                    }
                }
            }

            match matrix_format {
                TouchstoneMatrixFormat::Full => {}
                TouchstoneMatrixFormat::Lower => {
                    for col in 0..num_ports {
                        for row in 0..col {
                            matrix_re[row][col][freq_idx] = matrix_re[col][row][freq_idx];
                            matrix_im[row][col][freq_idx] = matrix_im[col][row][freq_idx];
                        }
                    }
                }
                TouchstoneMatrixFormat::Upper => {
                    for col in 0..num_ports {
                        for row in (col + 1)..num_ports {
                            matrix_re[row][col][freq_idx] = matrix_re[col][row][freq_idx];
                            matrix_im[row][col][freq_idx] = matrix_im[col][row][freq_idx];
                        }
                    }
                }
            }
        }

        let mut dataset = WaveformDataset::new(
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("touchstone"),
        );
        dataset.analysis = "S-Parameter".to_string();
        dataset
            .metadata
            .insert("format".to_string(), "touchstone".to_string());
        dataset
            .metadata
            .insert("touchstone_version".to_string(), version.to_string());
        dataset
            .metadata
            .insert("num_ports".to_string(), num_ports.to_string());
        dataset.metadata.insert(
            "touchstone_matrix_format".to_string(),
            match matrix_format {
                TouchstoneMatrixFormat::Full => "full",
                TouchstoneMatrixFormat::Lower => "lower",
                TouchstoneMatrixFormat::Upper => "upper",
            }
            .to_string(),
        );
        dataset
            .metadata
            .insert("z0".to_string(), z0_by_port[0].to_string());
        dataset.metadata.insert(
            "z0_ports".to_string(),
            z0_by_port
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );

        let mut x_signal = WaveformSignal::new("frequency", SignalType::Frequency);
        x_signal.data = frequencies;
        dataset.set_x(x_signal);

        for row in 1..=num_ports {
            for col in 1..=num_ports {
                let base = if num_ports <= 9 {
                    format!("S{}{}", row, col)
                } else {
                    format!("S{}_{}", row, col)
                };

                let mut re_signal =
                    WaveformSignal::new(format!("{}_RE", base), SignalType::SParameter);
                re_signal.data = matrix_re[row - 1][col - 1].clone();
                dataset.add_signal(re_signal);

                let mut im_signal =
                    WaveformSignal::new(format!("{}_IM", base), SignalType::SParameter);
                im_signal.data = matrix_im[row - 1][col - 1].clone();
                dataset.add_signal(im_signal);
            }
        }

        Ok(dataset)
    }

    fn touchstone_ports_from_extension(path: &Path) -> Option<usize> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        if ext.len() >= 3
            && ext.starts_with('s')
            && ext.ends_with('p')
            && ext[1..ext.len() - 1].chars().all(|ch| ch.is_ascii_digit())
        {
            return ext[1..ext.len() - 1].parse::<usize>().ok();
        }
        None
    }

    fn touchstone_values_per_frequency(
        num_ports: usize,
        matrix_format: TouchstoneMatrixFormat,
    ) -> Option<usize> {
        let matrix_points = match matrix_format {
            TouchstoneMatrixFormat::Full => num_ports.checked_mul(num_ports)?,
            TouchstoneMatrixFormat::Lower | TouchstoneMatrixFormat::Upper => num_ports
                .checked_mul(num_ports.checked_add(1)?)?
                .checked_div(2)?,
        };
        matrix_points.checked_mul(2)?.checked_add(1)
    }

    fn infer_touchstone_ports_from_tokens(
        tokens: &[f64],
        matrix_format: TouchstoneMatrixFormat,
    ) -> Option<usize> {
        // Guard against unrealistic matrices while still supporting large N.
        const MAX_PORTS_TO_INFER: usize = 64;
        let mut best_match = None;
        for ports in 1..=MAX_PORTS_TO_INFER {
            let Some(record_width) = Self::touchstone_values_per_frequency(ports, matrix_format)
            else {
                continue;
            };
            if tokens.len() < record_width
                || !crate::utils::numeric::is_multiple_of(tokens.len(), record_width)
            {
                continue;
            }
            let num_freqs = tokens.len() / record_width;
            let freqs_valid = (0..num_freqs).all(|idx| {
                let freq = tokens[idx * record_width];
                freq.is_finite() && freq > 0.0
            });
            if freqs_valid {
                best_match = Some(ports);
            }
        }
        best_match
    }

    fn parse_touchstone_option_line(
        line: &str,
        line_no: usize,
    ) -> Result<TouchstoneOptions, String> {
        let fields: Vec<&str> = line[1..].split_whitespace().collect();
        if fields.is_empty() {
            return Err(format!("Touchstone line {}: empty option line", line_no));
        }

        let mut idx = 0usize;

        let freq_scale_hz = match fields[idx].to_ascii_lowercase().as_str() {
            "hz" => 1.0,
            "khz" => 1.0e3,
            "mhz" => 1.0e6,
            "ghz" => 1.0e9,
            other => {
                return Err(format!(
                    "Touchstone line {}: unsupported frequency unit '{}'",
                    line_no, other
                ));
            }
        };
        idx += 1;

        if idx >= fields.len() {
            return Err(format!(
                "Touchstone line {}: option line missing parameter type",
                line_no
            ));
        }
        if !fields[idx].eq_ignore_ascii_case("s") {
            return Err(format!(
                "Touchstone line {}: only S-parameter files are supported (found '{}')",
                line_no, fields[idx]
            ));
        }
        idx += 1;

        if idx >= fields.len() {
            return Err(format!(
                "Touchstone line {}: option line missing data format",
                line_no
            ));
        }
        let data_format = match fields[idx].to_ascii_lowercase().as_str() {
            "ri" => TouchstoneDataFormat::Ri,
            "ma" => TouchstoneDataFormat::Ma,
            "db" => TouchstoneDataFormat::Db,
            other => {
                return Err(format!(
                    "Touchstone line {}: unsupported data format '{}'",
                    line_no, other
                ));
            }
        };
        idx += 1;

        let mut reference_ohms = 50.0;
        if idx < fields.len() {
            if !fields[idx].eq_ignore_ascii_case("r") {
                return Err(format!(
                    "Touchstone line {}: expected 'R <reference>', found '{}'",
                    line_no, fields[idx]
                ));
            }
            idx += 1;
            if idx >= fields.len() {
                return Err(format!(
                    "Touchstone line {}: missing numeric value after 'R'",
                    line_no
                ));
            }
            reference_ohms =
                Self::parse_touchstone_numeric_token(fields[idx]).ok_or_else(|| {
                    format!(
                        "Touchstone line {}: invalid reference impedance '{}'",
                        line_no, fields[idx]
                    )
                })?;
            idx += 1;
        }

        if idx != fields.len() {
            return Err(format!(
                "Touchstone line {}: unexpected tokens in option line",
                line_no
            ));
        }
        if !reference_ohms.is_finite() || reference_ohms <= 0.0 {
            return Err(format!(
                "Touchstone line {}: reference impedance must be positive",
                line_no
            ));
        }

        Ok(TouchstoneOptions {
            freq_scale_hz,
            data_format,
            reference_ohms,
        })
    }

    fn parse_touchstone_section_line(line: &str, line_no: usize) -> Result<(String, &str), String> {
        let Some(end_bracket) = line.find(']') else {
            return Err(format!(
                "Touchstone line {}: malformed section header '{}'",
                line_no, line
            ));
        };
        if !line.starts_with('[') {
            return Err(format!(
                "Touchstone line {}: malformed section header '{}'",
                line_no, line
            ));
        }
        let section = line[1..end_bracket].trim().to_ascii_lowercase();
        if section.is_empty() {
            return Err(format!("Touchstone line {}: empty section header", line_no));
        }
        Ok((section, line[end_bracket + 1..].trim()))
    }

    fn parse_touchstone_numeric_values(value: &str, line_no: usize) -> Result<Vec<f64>, String> {
        let mut out = Vec::new();
        for token in value.split(|ch: char| ch.is_whitespace() || ch == ',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let parsed = Self::parse_touchstone_numeric_token(token).ok_or_else(|| {
                format!(
                    "Touchstone line {}: invalid numeric value '{}'",
                    line_no, token
                )
            })?;
            out.push(parsed);
        }
        if out.is_empty() {
            return Err(format!(
                "Touchstone line {}: section requires at least one numeric value",
                line_no
            ));
        }
        Ok(out)
    }

    fn parse_touchstone_numeric_token(token: &str) -> Option<f64> {
        token.replace(['D', 'd'], "e").parse::<f64>().ok()
    }

    fn resolve_touchstone_reference_values(
        num_ports: usize,
        default_reference: f64,
        override_values: Option<&[f64]>,
    ) -> Result<Vec<f64>, String> {
        let values: Vec<f64> = match override_values {
            Some(values) if values.len() == 1 => vec![values[0]; num_ports],
            Some(values) if values.len() == num_ports => values.to_vec(),
            Some(values) => {
                return Err(format!(
                    "Touchstone [Reference] count {} does not match port count {}",
                    values.len(),
                    num_ports
                ));
            }
            None => vec![default_reference; num_ports],
        };
        for (idx, value) in values.iter().enumerate() {
            if !value.is_finite() || *value <= 0.0 {
                return Err(format!(
                    "Touchstone reference impedance for port {} must be positive",
                    idx + 1
                ));
            }
        }
        Ok(values)
    }

    fn touchstone_pair_to_complex(
        first: f64,
        second: f64,
        format: TouchstoneDataFormat,
    ) -> (f64, f64) {
        match format {
            TouchstoneDataFormat::Ri => (first, second),
            TouchstoneDataFormat::Ma => {
                let angle = second.to_radians();
                (first * angle.cos(), first * angle.sin())
            }
            TouchstoneDataFormat::Db => {
                let magnitude = 10.0_f64.powf(first / 20.0);
                let angle = second.to_radians();
                (magnitude * angle.cos(), magnitude * angle.sin())
            }
        }
    }
}

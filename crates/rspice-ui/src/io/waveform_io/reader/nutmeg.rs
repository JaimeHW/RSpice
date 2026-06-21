use super::*;

impl WaveformReader {
    /// Read NUTMEG/raw format
    pub(super) fn read_nutmeg(&self, path: &Path) -> Result<WaveformDataset, String> {
        let parsed = rspice_core::compat::parse_raw_file(path)
            .map_err(|e| format!("Failed to read raw/Nutmeg '{}': {}", path.display(), e))?;
        let x_waveform = parsed.waveforms.first().ok_or_else(|| {
            format!(
                "Raw/Nutmeg file '{}' contained no waveforms",
                path.display()
            )
        })?;
        let x_variable = parsed.variables.first().ok_or_else(|| {
            format!(
                "Raw/Nutmeg file '{}' contained no variable definitions",
                path.display()
            )
        })?;
        if x_waveform.y.is_empty() {
            return Err(format!(
                "Raw/Nutmeg file '{}' has an empty independent variable '{}'",
                path.display(),
                x_waveform.name
            ));
        }
        if !x_waveform.y.iter().all(|value| value.is_finite()) {
            return Err(format!(
                "Raw/Nutmeg file '{}' has non-finite independent variable '{}'",
                path.display(),
                x_waveform.name
            ));
        }
        let x_len = x_waveform.y.len();

        let title = if parsed.header.title.trim().is_empty() {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("raw")
                .to_string()
        } else {
            parsed.header.title.clone()
        };
        let mut dataset = WaveformDataset::new(title);
        dataset.analysis = parsed.header.plotname.clone();
        dataset
            .metadata
            .insert("format".to_string(), "raw-nutmeg".to_string());
        dataset
            .metadata
            .insert("source_path".to_string(), path.display().to_string());
        dataset.metadata.insert(
            "num_variables".to_string(),
            parsed.header.no_variables.to_string(),
        );
        dataset
            .metadata
            .insert("num_points".to_string(), x_len.to_string());

        let mut x_signal = WaveformSignal::new(
            x_waveform.name.clone(),
            SignalType::from(x_variable.var_type.as_str()),
        );
        x_signal.data = x_waveform.y.clone();
        dataset.set_x(x_signal);

        for (idx, waveform) in parsed.waveforms.iter().enumerate().skip(1) {
            let variable = parsed.variables.get(idx).ok_or_else(|| {
                format!(
                    "Raw/Nutmeg file '{}' waveform '{}' has no variable definition",
                    path.display(),
                    waveform.name
                )
            })?;
            let signal_type = SignalType::from(variable.var_type.as_str());
            if waveform.y.len() != x_len {
                return Err(format!(
                    "Raw/Nutmeg signal '{}' has {} point(s), expected {}",
                    waveform.name,
                    waveform.y.len(),
                    x_len
                ));
            }
            if !waveform.y.iter().all(|value| value.is_finite()) {
                return Err(format!(
                    "Raw/Nutmeg signal '{}' contains non-finite values",
                    waveform.name
                ));
            }

            if let Some(imag) = &waveform.y_imag {
                if imag.len() != x_len {
                    return Err(format!(
                        "Raw/Nutmeg imaginary signal '{}' has {} point(s), expected {}",
                        waveform.name,
                        imag.len(),
                        x_len
                    ));
                }
                if !imag.iter().all(|value| value.is_finite()) {
                    return Err(format!(
                        "Raw/Nutmeg imaginary signal '{}' contains non-finite values",
                        waveform.name
                    ));
                }

                let mut real_signal = WaveformSignal::new(
                    format!("{}_RE", waveform.name),
                    raw_complex_signal_type(signal_type, false),
                );
                real_signal.data = waveform.y.clone();
                dataset.add_signal(real_signal);

                let mut imag_signal = WaveformSignal::new(
                    format!("{}_IM", waveform.name),
                    raw_complex_signal_type(signal_type, true),
                );
                imag_signal.data = imag.clone();
                dataset.add_signal(imag_signal);
            } else {
                let mut signal = WaveformSignal::new(waveform.name.clone(), signal_type);
                signal.data = waveform.y.clone();
                dataset.add_signal(signal);
            }
        }

        if dataset.signals.is_empty() {
            return Err(format!(
                "Raw/Nutmeg file '{}' contained no dependent signals",
                path.display()
            ));
        }

        Ok(dataset)
    }
}

fn raw_complex_signal_type(signal_type: SignalType, imag: bool) -> SignalType {
    match (signal_type, imag) {
        (SignalType::Voltage, false) | (SignalType::VoltageReal, false) => SignalType::VoltageReal,
        (SignalType::Voltage, true) | (SignalType::VoltageImag, true) => SignalType::VoltageImag,
        (SignalType::Current, false) | (SignalType::CurrentReal, false) => SignalType::CurrentReal,
        (SignalType::Current, true) | (SignalType::CurrentImag, true) => SignalType::CurrentImag,
        _ => SignalType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is available")
            .as_nanos();
        path.push(format!("rspice-nutmeg-{name}-{nonce}.raw"));
        path
    }

    fn write_temp_raw(name: &str, contents: &str) -> PathBuf {
        let path = temp_path(name);
        fs::write(&path, contents).expect("write temporary raw waveform");
        path
    }

    #[test]
    fn nutmeg_reader_rejects_empty_files() {
        let path = write_temp_raw("empty", "");
        let reader = WaveformReader::new(WaveformFormat::Nutmeg);

        let err = reader
            .read(&path)
            .expect_err("empty Nutmeg/raw files must reject import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("raw") || err.contains("header") || err.contains("empty"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn nutmeg_reader_rejects_invalid_declared_point_count() {
        let path = write_temp_raw(
            "bad-points",
            "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: nope\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0.0 1.0\n",
        );
        let reader = WaveformReader::new(WaveformFormat::Nutmeg);

        let err = reader
            .read(&path)
            .expect_err("invalid No. Points must reject import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("No. Points") || err.contains("points"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn nutmeg_reader_rejects_bad_values_without_ragged_data() {
        let path = write_temp_raw(
            "bad-value",
            "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0\ttime\ttime\n1\tV(out)\tvoltage\nValues:\n0 0.0 bad\n",
        );
        let reader = WaveformReader::new(WaveformFormat::Nutmeg);

        let err = reader
            .read(&path)
            .expect_err("bad raw data values must reject import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("bad") || err.contains("value"),
            "unexpected error: {err}"
        );
    }
}

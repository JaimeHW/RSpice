use super::*;

impl WaveformReader {
    /// Read CSV file
    pub(super) fn read_csv(&self, path: &Path) -> Result<WaveformDataset, String> {
        self.read_delimited(path, ',')
    }

    /// Read TSV file
    pub(super) fn read_tsv(&self, path: &Path) -> Result<WaveformDataset, String> {
        self.read_delimited(path, '\t')
    }

    /// Read delimited file
    fn read_delimited(&self, path: &Path, delimiter: char) -> Result<WaveformDataset, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open: {}", e))?;
        let reader = BufReader::new(file);

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or(""));
        let mut lines = reader.lines();

        // Header line
        let header = lines
            .next()
            .ok_or("Empty file")?
            .map_err(|e| format!("Read error: {}", e))?;

        let columns = parse_delimited_record(&header, delimiter, 1)?;
        if columns.is_empty() {
            return Err("No columns found".to_string());
        }
        if columns.iter().any(|column| column.trim().is_empty()) {
            return Err("Delimited waveform header contains an empty column name".to_string());
        }

        // Create signals
        let mut signals: Vec<WaveformSignal> = columns
            .iter()
            .map(|name| {
                let name = name.trim();
                let sig_type = if name.to_lowercase() == "time" {
                    SignalType::Time
                } else if name.to_lowercase().starts_with("v(") {
                    SignalType::Voltage
                } else if name.to_lowercase().starts_with("i(") {
                    SignalType::Current
                } else {
                    SignalType::Unknown
                };
                WaveformSignal::new(name, sig_type)
            })
            .collect();

        // Data lines
        for (row_idx, line) in lines.enumerate() {
            let line_num = row_idx + 2;
            let line = line.map_err(|e| format!("Read error: {}", e))?;
            if line.trim().is_empty() {
                continue;
            }
            let values = parse_delimited_record(&line, delimiter, line_num)?;
            if values.len() != signals.len() {
                return Err(format!(
                    "Delimited waveform line {} has {} column(s); expected {}",
                    line_num,
                    values.len(),
                    signals.len()
                ));
            }

            for (signal, raw_value) in signals.iter_mut().zip(values.iter()) {
                let value = raw_value.trim().parse::<f64>().map_err(|_| {
                    format!(
                        "Delimited waveform line {} column '{}' has invalid numeric value '{}'",
                        line_num, signal.name, raw_value
                    )
                })?;
                if !value.is_finite() {
                    return Err(format!(
                        "Delimited waveform line {} column '{}' has non-finite numeric value '{}'",
                        line_num, signal.name, raw_value
                    ));
                }
                signal.push(value);
            }
        }

        // First column is typically X axis
        if !signals.is_empty() {
            dataset.x_signal = Some(signals.remove(0));
            dataset.signals = signals;
        }

        Ok(dataset)
    }
}

fn parse_delimited_record(
    line: &str,
    delimiter: char,
    line_num: usize,
) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;
    let mut just_closed_quote = false;

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    let _ = chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                    just_closed_quote = true;
                }
            } else {
                field.push(ch);
            }
            continue;
        }

        if just_closed_quote {
            if ch == delimiter {
                fields.push(field.trim().to_string());
                field.clear();
                just_closed_quote = false;
            } else if !ch.is_whitespace() {
                return Err(format!(
                    "Delimited waveform line {} has unexpected character '{}' after closing quote",
                    line_num, ch
                ));
            }
            continue;
        }

        if ch == delimiter {
            fields.push(field.trim().to_string());
            field.clear();
        } else if ch == '"' {
            if field.trim().is_empty() {
                field.clear();
                in_quotes = true;
            } else {
                return Err(format!(
                    "Delimited waveform line {} has a quote inside an unquoted field",
                    line_num
                ));
            }
        } else {
            field.push(ch);
        }
    }

    if in_quotes {
        return Err(format!(
            "Delimited waveform line {} has an unterminated quoted field",
            line_num
        ));
    }

    fields.push(field.trim().to_string());
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str, extension: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is available")
            .as_nanos();
        path.push(format!("rspice-delimited-{name}-{nonce}.{extension}"));
        path
    }

    fn write_temp_file(name: &str, extension: &str, contents: &str) -> PathBuf {
        let path = temp_path(name, extension);
        fs::write(&path, contents).expect("write temporary delimited waveform");
        path
    }

    #[test]
    fn csv_reader_rejects_malformed_numeric_rows_without_desynchronizing_columns() {
        let path = write_temp_file("bad-row", "csv", "time,V(out)\n0,1\n1,bad\n2,3\n");
        let reader = WaveformReader::new(WaveformFormat::Csv);

        let err = reader
            .read(&path)
            .expect_err("malformed numeric rows must reject the import");
        let _ = fs::remove_file(&path);

        assert!(
            err.contains("line 3") && err.contains("V(out)") && err.contains("bad"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn csv_reader_handles_quoted_header_commas() {
        let path = write_temp_file("quoted-header", "csv", "time,\"V(out,ref)\"\n0,1\n1,2\n");
        let reader = WaveformReader::new(WaveformFormat::Csv);

        let dataset = reader
            .read(&path)
            .expect("quoted CSV headers should import");
        let _ = fs::remove_file(&path);

        assert_eq!(
            dataset
                .x_signal
                .as_ref()
                .map(|signal| signal.data.as_slice()),
            Some([0.0, 1.0].as_slice())
        );
        assert_eq!(dataset.signal_names(), vec!["V(out,ref)"]);
        assert_eq!(dataset.signals[0].data, vec![1.0, 2.0]);
    }
}

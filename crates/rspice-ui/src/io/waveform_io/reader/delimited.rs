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

        let columns: Vec<&str> = header.split(delimiter).collect();
        if columns.is_empty() {
            return Err("No columns found".to_string());
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
        for line in lines {
            let line = line.map_err(|e| format!("Read error: {}", e))?;
            let values: Vec<&str> = line.split(delimiter).collect();

            for (i, val) in values.iter().enumerate() {
                if i < signals.len()
                    && let Ok(v) = val.trim().parse::<f64>()
                {
                    signals[i].push(v);
                }
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

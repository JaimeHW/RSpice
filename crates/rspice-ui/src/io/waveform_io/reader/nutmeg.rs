use super::*;

impl WaveformReader {
    /// Read NUTMEG/raw format
    pub(super) fn read_nutmeg(&self, path: &Path) -> Result<WaveformDataset, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open: {}", e))?;
        let reader = BufReader::new(file);

        let mut dataset = WaveformDataset::new("");
        let mut variables: Vec<(String, SignalType)> = Vec::new();
        let mut in_header = true;
        let mut num_points = 0;
        let mut values_buffer: Vec<Vec<f64>> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Read error: {}", e))?;
            let trimmed = line.trim();

            if in_header {
                if let Some(stripped) = trimmed.strip_prefix("Title:") {
                    dataset.title = stripped.trim().to_string();
                } else if let Some(stripped) = trimmed.strip_prefix("Plotname:") {
                    dataset.analysis = stripped.trim().to_string();
                } else if trimmed.starts_with("No. Variables:") {
                    // Parse number of variables
                } else if let Some(stripped) = trimmed.strip_prefix("No. Points:") {
                    num_points = stripped.trim().parse().unwrap_or(0);
                } else if trimmed.starts_with("Variables:") {
                    // Next lines are variable definitions
                } else if trimmed.starts_with("Values:") {
                    in_header = false;
                    // Initialize buffers
                    values_buffer = vec![Vec::with_capacity(num_points); variables.len()];
                } else if trimmed.contains('\t') && !trimmed.is_empty() {
                    // Variable definition line: index\tname\ttype
                    let parts: Vec<&str> = trimmed.split('\t').collect();
                    if parts.len() >= 3 {
                        let name = parts[1].trim().to_string();
                        let sig_type = SignalType::from(parts[2].trim());
                        variables.push((name, sig_type));
                    }
                }
            } else {
                // Data section
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                for (i, val) in parts.iter().enumerate() {
                    if i < values_buffer.len()
                        && let Ok(v) = val.parse::<f64>()
                    {
                        values_buffer[i].push(v);
                    }
                }
            }
        }

        // Create signals from buffers
        for (i, (name, sig_type)) in variables.into_iter().enumerate() {
            let mut signal = WaveformSignal::new(name, sig_type);
            if i < values_buffer.len() {
                signal.data = values_buffer[i].clone();
            }

            if i == 0 {
                dataset.x_signal = Some(signal);
            } else {
                dataset.signals.push(signal);
            }
        }

        Ok(dataset)
    }
}

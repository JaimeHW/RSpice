use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchstoneFormat {
    /// Magnitude-Angle (MA)
    MagnitudeAngle,
    /// Real-Imaginary (RI)
    RealImaginary,
    /// dB-Angle (DB)
    DecibelAngle,
}

impl TouchstoneFormat {
    /// Get format string for Touchstone header
    pub fn format_string(&self) -> &'static str {
        match self {
            TouchstoneFormat::MagnitudeAngle => "MA",
            TouchstoneFormat::RealImaginary => "RI",
            TouchstoneFormat::DecibelAngle => "DB",
        }
    }
}

/// Touchstone file frequency unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchstoneFreqUnit {
    Hz,
    KHz,
    MHz,
    GHz,
}

impl TouchstoneFreqUnit {
    /// Get the multiplier to convert to Hz
    pub fn multiplier(&self) -> Value {
        match self {
            TouchstoneFreqUnit::Hz => 1.0,
            TouchstoneFreqUnit::KHz => 1e3,
            TouchstoneFreqUnit::MHz => 1e6,
            TouchstoneFreqUnit::GHz => 1e9,
        }
    }

    /// Get unit string for Touchstone header
    pub fn unit_string(&self) -> &'static str {
        match self {
            TouchstoneFreqUnit::Hz => "HZ",
            TouchstoneFreqUnit::KHz => "KHZ",
            TouchstoneFreqUnit::MHz => "MHZ",
            TouchstoneFreqUnit::GHz => "GHZ",
        }
    }
}

/// Touchstone file exporter
pub struct TouchstoneExporter {
    /// Data format
    pub format: TouchstoneFormat,
    /// Frequency unit
    pub freq_unit: TouchstoneFreqUnit,
    /// Reference impedance
    pub z0: Value,
    /// Comments to include in file
    pub comments: Vec<String>,
}

impl Default for TouchstoneExporter {
    fn default() -> Self {
        Self {
            format: TouchstoneFormat::RealImaginary,
            freq_unit: TouchstoneFreqUnit::GHz,
            z0: 50.0,
            comments: Vec::new(),
        }
    }
}

impl TouchstoneExporter {
    /// Create new exporter with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set data format
    pub fn with_format(mut self, format: TouchstoneFormat) -> Self {
        self.format = format;
        self
    }

    /// Set frequency unit
    pub fn with_freq_unit(mut self, unit: TouchstoneFreqUnit) -> Self {
        self.freq_unit = unit;
        self
    }

    /// Set reference impedance
    pub fn with_z0(mut self, z0: Value) -> Self {
        self.z0 = z0;
        self
    }

    /// Add a comment line
    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comments.push(comment.to_string());
        self
    }

    /// Export S-parameter result to Touchstone format string
    pub fn export(&self, result: &SParameterResult) -> String {
        let mut output = String::new();

        // Comments
        for comment in &self.comments {
            output.push_str(&format!("! {}\n", comment));
        }

        // Option line: # <freq_unit> S <format> R <z0>
        output.push_str(&format!(
            "# {} S {} R {:.1}\n",
            self.freq_unit.unit_string(),
            self.format.format_string(),
            self.z0
        ));

        // Data lines
        let freq_mult = self.freq_unit.multiplier();

        for s in &result.data {
            let freq = s.frequency / freq_mult;

            match result.num_ports {
                1 => {
                    // 1-port: freq S11
                    let s11 = s.get(1, 1);
                    let (v1, v2) = self.format_complex(s11);
                    output.push_str(&format!("{:.9e}\t{:.9e}\t{:.9e}\n", freq, v1, v2));
                }
                2 => {
                    // 2-port: freq S11 S21 S12 S22
                    let s11 = s.get(1, 1);
                    let s21 = s.get(2, 1);
                    let s12 = s.get(1, 2);
                    let s22 = s.get(2, 2);

                    let (s11_1, s11_2) = self.format_complex(s11);
                    let (s21_1, s21_2) = self.format_complex(s21);
                    let (s12_1, s12_2) = self.format_complex(s12);
                    let (s22_1, s22_2) = self.format_complex(s22);

                    output.push_str(&format!(
                        "{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\n",
                        freq, s11_1, s11_2, s21_1, s21_2, s12_1, s12_2, s22_1, s22_2
                    ));
                }
                _ => {
                    // N-port: more complex formatting (split across lines)
                    output.push_str(&format!("{:.9e}", freq));
                    for row in 1..=result.num_ports {
                        for col in 1..=result.num_ports {
                            let sij = s.get(row, col);
                            let (v1, v2) = self.format_complex(sij);
                            output.push_str(&format!("\t{:.9e}\t{:.9e}", v1, v2));
                        }
                    }
                    output.push('\n');
                }
            }
        }

        output
    }

    /// Format complex number according to selected format
    fn format_complex(&self, c: Complex) -> (Value, Value) {
        match self.format {
            TouchstoneFormat::RealImaginary => (c.re, c.im),
            TouchstoneFormat::MagnitudeAngle => (c.magnitude(), c.phase_deg()),
            TouchstoneFormat::DecibelAngle => (c.mag_db(), c.phase_deg()),
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

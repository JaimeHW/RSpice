use super::format::format_freq;

/// Type of frequency sweep for S-parameter analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpSweepType {
    /// Decades (logarithmic) - most common for RF
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl SpSweepType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Decade => "Decade",
            Self::Octave => "Octave",
            Self::Linear => "Linear",
        }
    }

    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }

    /// All types
    pub fn all() -> &'static [SpSweepType] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

/// Configuration for a single port
#[derive(Debug, Clone)]
pub struct SpPortConfig {
    /// Port number (1-indexed)
    pub number: u32,
    /// Positive node name
    pub node_pos: String,
    /// Negative/reference node name (0 for ground)
    pub node_neg: String,
    /// Port-specific reference impedance (overrides global Z0)
    pub z0: Option<f64>,
}

impl SpPortConfig {
    /// Create single-ended port referenced to ground
    pub fn single_ended(number: u32, node: &str) -> Self {
        Self {
            number,
            node_pos: node.to_uppercase(),
            node_neg: "0".to_string(),
            z0: None,
        }
    }

    /// Create differential port
    pub fn differential(number: u32, node_pos: &str, node_neg: &str) -> Self {
        Self {
            number,
            node_pos: node_pos.to_uppercase(),
            node_neg: node_neg.to_uppercase(),
            z0: None,
        }
    }

    /// Set port-specific impedance
    pub fn with_z0(mut self, z0: f64) -> Self {
        self.z0 = Some(z0);
        self
    }

    /// Check if differential
    pub fn is_differential(&self) -> bool {
        self.node_neg != "0"
    }
}

impl Default for SpPortConfig {
    fn default() -> Self {
        Self::single_ended(1, "IN")
    }
}

/// S-parameter analysis configuration.
#[derive(Debug, Clone)]
pub struct SpConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points (per decade/octave for log, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: SpSweepType,
    /// Reference impedance - default 50 Ohms
    pub z0: f64,
    /// Port definitions
    pub ports: Vec<SpPortConfig>,
    /// Include noise analysis
    pub do_noise: bool,
    /// Export Touchstone file
    pub touchstone_export: bool,
    /// Touchstone format version (1 or 2)
    pub touchstone_version: u32,
    /// Save all internal nodes
    pub save_all: bool,
}

impl Default for SpConfig {
    fn default() -> Self {
        Self {
            start_freq: 1e6,
            stop_freq: 10e9,
            num_points: 10,
            sweep_type: SpSweepType::Decade,
            z0: 50.0,
            ports: vec![
                SpPortConfig::single_ended(1, "IN"),
                SpPortConfig::single_ended(2, "OUT"),
            ],
            do_noise: false,
            touchstone_export: true,
            touchstone_version: 2,
            save_all: false,
        }
    }
}

impl SpConfig {
    /// Create new linear sweep config
    pub fn linear(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type: SpSweepType::Linear,
            ..Default::default()
        }
    }

    /// Create new decade sweep config
    pub fn decade(start: f64, stop: f64, points_per_decade: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points_per_decade,
            sweep_type: SpSweepType::Decade,
            ..Default::default()
        }
    }

    /// Set reference impedance
    pub fn with_z0(mut self, z0: f64) -> Self {
        self.z0 = z0;
        self
    }

    /// Set ports
    pub fn with_ports(mut self, ports: Vec<SpPortConfig>) -> Self {
        self.ports = ports;
        self
    }

    /// Add a port
    pub fn add_port(mut self, port: SpPortConfig) -> Self {
        self.ports.push(port);
        self
    }

    /// Enable noise analysis
    pub fn with_noise(mut self, enable: bool) -> Self {
        self.do_noise = enable;
        self
    }

    /// Enable Touchstone export
    pub fn with_touchstone(mut self, enable: bool, version: u32) -> Self {
        self.touchstone_export = enable;
        self.touchstone_version = version;
        self
    }

    /// Number of ports
    pub fn num_ports(&self) -> usize {
        self.ports.len()
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            SpSweepType::Decade => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            SpSweepType::Octave => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            SpSweepType::Linear => self.num_points,
        }
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".sp {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        if (self.z0 - 50.0).abs() > 0.01 {
            cmd.push_str(&format!(" z0={}", self.z0));
        }

        for port in &self.ports {
            if port.is_differential() {
                cmd.push_str(&format!(
                    " port{}=({},{})",
                    port.number, port.node_pos, port.node_neg
                ));
            } else {
                cmd.push_str(&format!(" port{}={}", port.number, port.node_pos));
            }
            if let Some(pz0) = port.z0 {
                cmd.push_str(&format!(" port{}z0={}", port.number, pz0));
            }
        }

        if self.do_noise {
            cmd.push_str(" donoise=yes");
        }

        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.start_freq <= 0.0 {
            return Err("Start frequency must be positive".to_string());
        }
        if self.stop_freq <= 0.0 {
            return Err("Stop frequency must be positive".to_string());
        }
        if self.start_freq >= self.stop_freq {
            return Err("Start frequency must be less than stop frequency".to_string());
        }
        if self.num_points == 0 {
            return Err("Number of points must be at least 1".to_string());
        }
        if self.z0 <= 0.0 {
            return Err("Reference impedance Z0 must be positive".to_string());
        }
        if self.ports.is_empty() {
            return Err("At least one port must be defined".to_string());
        }
        if self.ports.len() < 2 {
            return Err("S-parameter analysis requires at least 2 ports".to_string());
        }

        for port in &self.ports {
            if port.node_pos.is_empty() {
                return Err(format!(
                    "Port {} positive node cannot be empty",
                    port.number
                ));
            }
            if let Some(pz0) = port.z0
                && pz0 <= 0.0
            {
                return Err(format!("Port {} impedance must be positive", port.number));
            }
        }

        let mut port_nums: Vec<u32> = self.ports.iter().map(|p| p.number).collect();
        port_nums.sort();
        for i in 1..port_nums.len() {
            if port_nums[i] == port_nums[i - 1] {
                return Err(format!("Duplicate port number: {}", port_nums[i]));
            }
        }

        Ok(())
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

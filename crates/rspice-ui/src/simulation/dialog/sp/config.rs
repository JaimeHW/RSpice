//! S-parameter sweep configuration.

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
    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
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
        }
    }
}

impl SpConfig {
    /// The `.SP` directive this configuration writes into a deck.
    ///
    /// `.SP <LIN|OCT|DEC> <np> <fstart> <fstop> [<donoise>]` is the whole of
    /// the card — both in ngspice, which is where `.SP` comes from, and in
    /// this engine's parser (`parser::command_parsers::parse_sp_command`).
    ///
    /// The port table is deliberately not on it. A port is an annotation on
    /// the element standing at it (`portnum=<n> [z0=<ohms>]`, read by
    /// `analysis::s_param::ports`), and the studio's own table reaches the
    /// solver typed instead: `build_sp_spec` copies it into
    /// `AnalysisSpec::SParameter`, and `runner::spec::frequency` copies that
    /// into `svc_runner::SParameterRunConfig`. Spelling the ports here as
    /// `port1=IN` made a card no parser accepts, and because preparation
    /// parses the whole executable deck, that refused every analysis in the
    /// plan rather than just this one.
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".sp {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        if self.do_noise {
            // The card's optional trailing flag is a value, not a key.
            cmd.push_str(" 1");
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
        // An empty table is legitimate: a design drawn with RF Port components
        // declares its ports in the deck, and typing them again here would be
        // a second place for them to disagree. A partly filled table is not —
        // one port is a table someone stopped filling in.
        if self.ports.len() == 1 {
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
}

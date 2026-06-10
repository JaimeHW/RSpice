//! Transfer Function (XF) Analysis Configuration
//!
//! Configuration for small-signal transfer function analysis.
//! XF computes the voltage or current transfer from an input source
//! to any output in the circuit.

use super::options::parse_si_value;

/// Transfer function sweep type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum XfSweepType {
    #[default]
    Decade,
    Octave,
    Linear,
}

impl XfSweepType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Decade => "Decade",
            Self::Octave => "Octave",
            Self::Linear => "Linear",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
    pub fn all() -> &'static [XfSweepType] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

/// Transfer function analysis configuration
#[derive(Debug, Clone)]
pub struct XfConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: XfSweepType,
    /// Input source name
    pub input_source: String,
    /// Output node name  
    pub output_node: String,
    /// Output reference node
    pub output_ref: String,
    /// Compute group delay
    pub group_delay: bool,
    /// Compute input impedance
    pub input_impedance: bool,
    /// Compute output impedance
    pub output_impedance: bool,
}

impl Default for XfConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0,
            stop_freq: 1e9,
            num_points: 10,
            sweep_type: XfSweepType::Decade,
            input_source: "VIN".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            group_delay: false,
            input_impedance: false,
            output_impedance: false,
        }
    }
}

impl XfConfig {
    pub fn new(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            ..Default::default()
        }
    }

    pub fn with_input(mut self, source: &str) -> Self {
        self.input_source = source.to_uppercase();
        self
    }

    pub fn with_output(mut self, node: &str) -> Self {
        self.output_node = node.to_uppercase();
        self
    }

    pub fn with_group_delay(mut self, enable: bool) -> Self {
        self.group_delay = enable;
        self
    }

    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            XfSweepType::Decade => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            XfSweepType::Octave => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            XfSweepType::Linear => self.num_points,
        }
    }

    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".xf {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        if !self.input_source.is_empty() {
            cmd.push_str(&format!(" input={}", self.input_source));
        }
        if !self.output_node.is_empty() {
            if self.output_ref.is_empty() {
                cmd.push_str(&format!(" output={}", self.output_node));
            } else {
                cmd.push_str(&format!(
                    " output=({},{})",
                    self.output_node, self.output_ref
                ));
            }
        }
        if self.group_delay {
            cmd.push_str(" groupdelay=yes");
        }
        if self.input_impedance {
            cmd.push_str(" zin=yes");
        }
        if self.output_impedance {
            cmd.push_str(" zout=yes");
        }
        cmd
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.start_freq <= 0.0 {
            return Err("Start frequency must be positive".into());
        }
        if self.stop_freq <= 0.0 {
            return Err("Stop frequency must be positive".into());
        }
        if self.start_freq >= self.stop_freq {
            return Err("Start must be less than stop".into());
        }
        if self.num_points == 0 {
            return Err("Points must be at least 1".into());
        }
        if self.input_source.is_empty() {
            return Err("Input source required".into());
        }
        if self.output_node.is_empty() {
            return Err("Output node required".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct XfDialogState {
    pub start_freq: String,
    pub stop_freq: String,
    pub num_points: String,
    pub sweep_type_idx: usize,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: String,
    pub group_delay: bool,
    pub input_impedance: bool,
    pub output_impedance: bool,
    pub initialized: bool,
}

impl XfDialogState {
    pub fn from_config(config: &XfConfig) -> Self {
        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                XfSweepType::Decade => 0,
                XfSweepType::Octave => 1,
                XfSweepType::Linear => 2,
            },
            input_source: config.input_source.clone(),
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            group_delay: config.group_delay,
            input_impedance: config.input_impedance,
            output_impedance: config.output_impedance,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<XfConfig, String> {
        let start = parse_si_value(&self.start_freq).map_err(|e| format!("Bad start: {}", e))?;
        let stop = parse_si_value(&self.stop_freq).map_err(|e| format!("Bad stop: {}", e))?;
        let points: u32 = self.num_points.parse().map_err(|_| "Bad points")?;
        let sweep_type = match self.sweep_type_idx {
            0 => XfSweepType::Decade,
            1 => XfSweepType::Octave,
            _ => XfSweepType::Linear,
        };
        let config = XfConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            input_source: self.input_source.clone(),
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            group_delay: self.group_delay,
            input_impedance: self.input_impedance,
            output_impedance: self.output_impedance,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&XfConfig::default());
        }
    }

}

fn format_freq(f: f64) -> String {
    if f >= 1e9 {
        format!("{}G", f / 1e9)
    } else if f >= 1e6 {
        format!("{}Meg", f / 1e6)
    } else if f >= 1e3 {
        format!("{}k", f / 1e3)
    } else {
        format!("{}", f)
    }
}

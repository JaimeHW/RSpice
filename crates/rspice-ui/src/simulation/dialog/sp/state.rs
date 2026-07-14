use super::format::format_freq;
use super::{SpConfig, SpPortConfig, SpSweepType};
use crate::simulation::dialog::options::parse_si_value;

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpPortDialogState {
    /// Positive node name.
    pub node_pos: String,
    /// Differential mode flag.
    pub differential: bool,
    /// Negative node name for differential mode.
    pub node_neg: String,
    /// Port-specific reference impedance override enabled.
    pub z0_override: bool,
    /// Port-specific reference impedance text buffer.
    pub z0: String,
}

impl SpPortDialogState {
    pub(super) fn single_ended(node_pos: impl Into<String>) -> Self {
        Self {
            node_pos: node_pos.into(),
            differential: false,
            node_neg: "0".to_string(),
            z0_override: false,
            z0: String::new(),
        }
    }

    fn from_port_config(port: &SpPortConfig) -> Self {
        Self {
            node_pos: port.node_pos.clone(),
            differential: port.is_differential(),
            node_neg: port.node_neg.clone(),
            z0_override: port.z0.is_some(),
            z0: port.z0.map(|value| value.to_string()).unwrap_or_default(),
        }
    }

    fn to_port_config(&self, number: u32) -> Result<SpPortConfig, String> {
        let mut port = if self.differential {
            SpPortConfig::differential(number, &self.node_pos, &self.node_neg)
        } else {
            SpPortConfig::single_ended(number, &self.node_pos)
        };
        if self.z0_override {
            let z0 = parse_si_value(&self.z0)
                .map_err(|e| format!("Invalid Port {} Z0: {}", number, e))?;
            port.z0 = Some(z0);
        }
        Ok(port)
    }
}

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpDialogState {
    /// Start frequency buffer
    pub start_freq: String,
    /// Stop frequency buffer
    pub stop_freq: String,
    /// Number of points buffer
    pub num_points: String,
    /// Sweep type index (0=decade, 1=octave, 2=linear)
    pub sweep_type_idx: usize,
    /// Reference impedance buffer
    pub z0: String,
    /// Editable port definitions
    pub ports: Vec<SpPortDialogState>,
    /// Enable noise analysis
    pub do_noise: bool,
    /// Enable Touchstone export
    pub touchstone_export: bool,
    /// Touchstone format version (1 or 2).
    pub touchstone_version: u32,
    /// Initialized flag
    #[serde(skip)]
    pub initialized: bool,
}

impl SpDialogState {
    pub(super) fn default_port_node(index: usize) -> String {
        match index {
            0 => "IN".to_string(),
            1 => "OUT".to_string(),
            _ => format!("P{}", index + 1),
        }
    }

    fn ensure_min_ports(&mut self) {
        while self.ports.len() < 2 {
            let node = Self::default_port_node(self.ports.len());
            self.ports.push(SpPortDialogState::single_ended(node));
        }
    }

    /// Initialize from config
    pub fn from_config(config: &SpConfig) -> Self {
        let mut sorted_ports = config.ports.clone();
        sorted_ports.sort_by_key(|port| port.number);
        let mut port_states: Vec<SpPortDialogState> = sorted_ports
            .iter()
            .map(SpPortDialogState::from_port_config)
            .collect();
        if port_states.is_empty() {
            port_states.push(SpPortDialogState::single_ended("IN"));
            port_states.push(SpPortDialogState::single_ended("OUT"));
        }
        while port_states.len() < 2 {
            let node = Self::default_port_node(port_states.len());
            port_states.push(SpPortDialogState::single_ended(node));
        }

        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                SpSweepType::Decade => 0,
                SpSweepType::Octave => 1,
                SpSweepType::Linear => 2,
            },
            z0: config.z0.to_string(),
            ports: port_states,
            do_noise: config.do_noise,
            touchstone_export: config.touchstone_export,
            touchstone_version: config.touchstone_version.clamp(1, 2),
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<SpConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;
        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;
        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;
        let z0: f64 = self.z0.parse().map_err(|_| "Invalid Z0")?;

        let sweep_type = match self.sweep_type_idx {
            0 => SpSweepType::Decade,
            1 => SpSweepType::Octave,
            _ => SpSweepType::Linear,
        };

        let ports = self
            .ports
            .iter()
            .enumerate()
            .map(|(idx, port)| port.to_port_config((idx + 1) as u32))
            .collect::<Result<Vec<_>, _>>()?;

        let config = SpConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            z0,
            ports,
            do_noise: self.do_noise,
            touchstone_export: self.touchstone_export,
            touchstone_version: self.touchstone_version.clamp(1, 2),
            save_all: false,
        };

        config.validate()?;
        Ok(config)
    }

    /// Initialize defaults if not already
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&SpConfig::default());
        }
        self.ensure_min_ports();
    }
}

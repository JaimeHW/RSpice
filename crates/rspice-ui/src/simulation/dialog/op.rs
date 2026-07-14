//! DC Operating Point Configuration
//!
//! Configuration for DC operating point analysis (.op).
//! The simplest analysis - computes the DC bias point with all AC sources zeroed.

/// DC operating point analysis configuration
#[derive(Debug, Clone)]
pub struct OpConfig {
    /// Save all node voltages
    pub save_all: bool,
    /// Include device operating point parameters
    pub save_op_info: bool,
    /// Temperature for analysis (Celsius)
    pub temperature: f64,
    /// Enable specific node saves
    pub save_nodes: Vec<String>,
    /// Number of Gmin steps (convergence aid)
    pub gmin_steps: u32,
    /// Source stepping for convergence
    pub source_stepping: bool,
}

impl Default for OpConfig {
    fn default() -> Self {
        Self {
            save_all: true,
            save_op_info: true,
            temperature: 27.0,
            save_nodes: Vec::new(),
            gmin_steps: 0,
            source_stepping: false,
        }
    }
}

impl OpConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }

    pub fn with_save_nodes(mut self, nodes: Vec<String>) -> Self {
        self.save_nodes = nodes;
        self.save_all = false;
        self
    }

    pub fn with_gmin_steps(mut self, steps: u32) -> Self {
        self.gmin_steps = steps;
        self
    }

    pub fn to_spice(&self) -> String {
        let mut cmd = ".op".to_string();
        if self.gmin_steps > 0 {
            cmd.push_str(&format!(" gmin_steps={}", self.gmin_steps));
        }
        cmd
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.temperature < -273.15 {
            return Err("Temperature cannot be below absolute zero".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpDialogState {
    pub save_all: bool,
    pub save_op_info: bool,
    pub temperature: String,
    pub source_stepping: bool,
    pub gmin_steps: String,
    #[serde(skip)]
    pub initialized: bool,
}

impl OpDialogState {
    pub fn from_config(config: &OpConfig) -> Self {
        Self {
            save_all: config.save_all,
            save_op_info: config.save_op_info,
            temperature: format!("{}", config.temperature),
            source_stepping: config.source_stepping,
            gmin_steps: config.gmin_steps.to_string(),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<OpConfig, String> {
        let temp: f64 = self
            .temperature
            .parse()
            .map_err(|_| "Invalid temperature")?;
        let gmin: u32 = self
            .gmin_steps
            .parse()
            .map_err(|_| "Invalid GMIN step count")?;
        let config = OpConfig {
            save_all: self.save_all,
            save_op_info: self.save_op_info,
            temperature: temp,
            save_nodes: Vec::new(),
            gmin_steps: gmin,
            source_stepping: self.source_stepping,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&OpConfig::default());
        }
    }
}

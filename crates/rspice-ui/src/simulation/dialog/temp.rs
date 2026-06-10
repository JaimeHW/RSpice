//! Temperature Sweep Analysis Configuration
//!
//! Configuration for temperature sweep analysis.
//! Runs a base analysis across a range of temperatures.


/// Base analysis for temperature sweep
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TempBaseAnalysis {
    #[default]
    Op,
    Transient,
    Ac,
    Dc,
}

impl TempBaseAnalysis {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Op => "DC OP",
            Self::Transient => "Transient",
            Self::Ac => "AC",
            Self::Dc => "DC Sweep",
        }
    }
    pub fn all() -> &'static [TempBaseAnalysis] {
        &[Self::Op, Self::Transient, Self::Ac, Self::Dc]
    }
}

/// Temperature sweep configuration
#[derive(Debug, Clone)]
pub struct TempConfig {
    /// Start temperature (Celsius)
    pub temp_start: f64,
    /// Stop temperature (Celsius)
    pub temp_stop: f64,
    /// Temperature step (Celsius)
    pub temp_step: f64,
    /// Base analysis type
    pub base_analysis: TempBaseAnalysis,
    /// Specific temperatures (overrides range)
    pub specific_temps: Vec<f64>,
    /// Include corner temperatures (-40, 25, 85, 125)
    pub corner_temps: bool,
}

impl Default for TempConfig {
    fn default() -> Self {
        Self {
            temp_start: -40.0,
            temp_stop: 125.0,
            temp_step: 25.0,
            base_analysis: TempBaseAnalysis::Op,
            specific_temps: Vec::new(),
            corner_temps: false,
        }
    }
}

impl TempConfig {
    pub fn new(start: f64, stop: f64, step: f64) -> Self {
        Self {
            temp_start: start,
            temp_stop: stop,
            temp_step: step,
            ..Default::default()
        }
    }

    pub fn with_base(mut self, b: TempBaseAnalysis) -> Self {
        self.base_analysis = b;
        self
    }
    pub fn with_corner_temps(mut self) -> Self {
        self.corner_temps = true;
        self.specific_temps = vec![-40.0, 25.0, 85.0, 125.0];
        self
    }

    pub fn to_spice(&self) -> String {
        if !self.specific_temps.is_empty() {
            let temps: Vec<String> = self
                .specific_temps
                .iter()
                .map(|t| format!("{}", t))
                .collect();
            format!(".temp {}", temps.join(" "))
        } else {
            format!(
                ".temp {} {} {}",
                self.temp_start, self.temp_stop, self.temp_step
            )
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.temp_start < -273.15 || self.temp_stop < -273.15 {
            return Err("Temperature cannot be below absolute zero (-273.15°C)".into());
        }
        if self.specific_temps.is_empty() {
            if self.temp_step.abs() < 0.01 {
                return Err("Temperature step too small".into());
            }
            if (self.temp_stop > self.temp_start && self.temp_step < 0.0)
                || (self.temp_stop < self.temp_start && self.temp_step > 0.0)
            {
                return Err("Step sign must match sweep direction".into());
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn num_temps(&self) -> usize {
        if !self.specific_temps.is_empty() {
            return self.specific_temps.len();
        }
        if self.temp_step.abs() < 0.01 {
            return 1;
        }
        ((self.temp_stop - self.temp_start) / self.temp_step)
            .abs()
            .ceil() as usize
            + 1
    }
}

#[derive(Debug, Clone, Default)]
pub struct TempDialogState {
    pub temp_start: String,
    pub temp_stop: String,
    pub temp_step: String,
    pub base_idx: usize,
    pub corner_temps: bool,
    pub initialized: bool,
}

impl TempDialogState {
    pub fn from_config(config: &TempConfig) -> Self {
        Self {
            temp_start: format!("{}", config.temp_start),
            temp_stop: format!("{}", config.temp_stop),
            temp_step: format!("{}", config.temp_step),
            base_idx: match config.base_analysis {
                TempBaseAnalysis::Op => 0,
                TempBaseAnalysis::Transient => 1,
                TempBaseAnalysis::Ac => 2,
                TempBaseAnalysis::Dc => 3,
            },
            corner_temps: config.corner_temps,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<TempConfig, String> {
        let start: f64 = self.temp_start.parse().map_err(|_| "Invalid start temp")?;
        let stop: f64 = self.temp_stop.parse().map_err(|_| "Invalid stop temp")?;
        let step: f64 = self.temp_step.parse().map_err(|_| "Invalid step")?;
        let base = match self.base_idx {
            0 => TempBaseAnalysis::Op,
            1 => TempBaseAnalysis::Transient,
            2 => TempBaseAnalysis::Ac,
            _ => TempBaseAnalysis::Dc,
        };
        let mut config = TempConfig {
            temp_start: start,
            temp_stop: stop,
            temp_step: step,
            base_analysis: base,
            specific_temps: Vec::new(),
            corner_temps: self.corner_temps,
        };
        if self.corner_temps {
            config = config.with_corner_temps();
        }
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&TempConfig::default());
        }
    }

}

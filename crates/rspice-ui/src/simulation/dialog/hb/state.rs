use super::format::format_freq;
use super::{HbConfig, HbSolverType, HbToneConfig};
use crate::simulation::dialog::options::parse_si_value;

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HbToneDialogState {
    /// Tone frequency buffer.
    pub frequency: String,
    /// Harmonics buffer.
    pub harmonics: String,
    /// Optional name.
    pub name: String,
    /// Optional source routing.
    pub source: String,
}

impl HbToneDialogState {
    fn from_tone_config(tone: &HbToneConfig) -> Self {
        Self {
            frequency: format_freq(tone.frequency),
            harmonics: tone.harmonics.to_string(),
            name: tone.name.clone(),
            source: tone.source.clone().unwrap_or_default(),
        }
    }
}

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default)]
pub struct HbDialogState {
    /// Fundamental frequency buffer
    pub fundamental: String,
    /// Number of harmonics buffer
    pub harmonics: String,
    /// Primary tone name buffer
    pub fundamental_name: String,
    /// Primary tone source buffer
    pub fundamental_source: String,
    /// Oversample factor buffer
    pub oversample: String,
    /// Relative tolerance buffer
    pub reltol: String,
    /// Max iterations buffer
    pub maxiter: String,
    /// Damping factor buffer
    pub damping: String,
    /// GMRES restart buffer
    pub gmres_restart: String,
    /// Solver type index
    pub solver_idx: usize,
    /// Source stepping enabled
    pub source_stepping: bool,
    /// Additional tone rows.
    pub additional_tones: Vec<HbToneDialogState>,
    /// Initialized flag
    pub initialized: bool,
}

impl HbDialogState {
    /// Initialize from config
    pub fn from_config(config: &HbConfig) -> Self {
        Self {
            fundamental: format_freq(config.fundamental_freq),
            harmonics: config.num_harmonics.to_string(),
            fundamental_name: config.fundamental_name.clone(),
            fundamental_source: config.fundamental_source.clone().unwrap_or_default(),
            oversample: config.oversample.to_string(),
            reltol: format!("{:.0e}", config.reltol),
            maxiter: config.maxiter.to_string(),
            damping: config.damping.to_string(),
            gmres_restart: config.gmres_restart.to_string(),
            solver_idx: match config.solver {
                HbSolverType::Newton => 0,
                HbSolverType::Krylov => 1,
            },
            source_stepping: config.source_stepping,
            additional_tones: config
                .additional_tones
                .iter()
                .map(HbToneDialogState::from_tone_config)
                .collect(),
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<HbConfig, String> {
        let fundamental = parse_si_value(&self.fundamental)
            .map_err(|e| format!("Invalid fundamental frequency: {}", e))?;

        let harmonics: u32 = self
            .harmonics
            .parse()
            .map_err(|_| "Invalid harmonics count")?;

        let oversample: u32 = self
            .oversample
            .parse()
            .map_err(|_| "Invalid oversample factor")?;

        let reltol =
            parse_si_value(&self.reltol).map_err(|e| format!("Invalid tolerance: {}", e))?;

        let maxiter: u32 = self.maxiter.parse().map_err(|_| "Invalid max iterations")?;

        let damping: f64 = self.damping.parse().map_err(|_| "Invalid damping factor")?;

        let solver = match self.solver_idx {
            0 => HbSolverType::Newton,
            _ => HbSolverType::Krylov,
        };

        let gmres_restart: u32 = self
            .gmres_restart
            .parse()
            .map_err(|_| "Invalid GMRES restart")?;

        let mut config = HbConfig {
            fundamental_freq: fundamental,
            num_harmonics: harmonics,
            fundamental_name: if self.fundamental_name.trim().is_empty() {
                "tone1".to_string()
            } else {
                self.fundamental_name.trim().to_string()
            },
            fundamental_source: if self.fundamental_source.trim().is_empty() {
                None
            } else {
                Some(self.fundamental_source.trim().to_string())
            },
            additional_tones: Vec::new(),
            oversample,
            max_mixing_order: 5,
            reltol,
            abstol: 1e-12,
            maxiter,
            damping,
            solver,
            gmres_restart,
            source_stepping: self.source_stepping,
            verbose: false,
        };

        for (idx, tone) in self.additional_tones.iter().enumerate() {
            let is_empty = tone.frequency.trim().is_empty()
                && tone.harmonics.trim().is_empty()
                && tone.name.trim().is_empty()
                && tone.source.trim().is_empty();
            if is_empty {
                continue;
            }

            let freq = parse_si_value(&tone.frequency)
                .map_err(|e| format!("Invalid tone {} frequency: {}", idx + 2, e))?;
            let harm: u32 = tone
                .harmonics
                .parse()
                .map_err(|_| format!("Invalid tone {} harmonics", idx + 2))?;

            let mut tone_cfg =
                HbToneConfig::new(freq, harm).with_name(if tone.name.trim().is_empty() {
                    format!("tone{}", idx + 2)
                } else {
                    tone.name.trim().to_string()
                });
            if !tone.source.trim().is_empty() {
                tone_cfg = tone_cfg.with_source(tone.source.trim().to_string());
            }
            config.additional_tones.push(tone_cfg);
        }

        config.validate()?;
        Ok(config)
    }

    /// Initialize defaults if not already
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&HbConfig::default());
        }
    }
}

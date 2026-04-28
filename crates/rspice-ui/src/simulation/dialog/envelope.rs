//! Envelope Transient Analysis Configuration
//!
//! Efficient simulation of modulated RF signals by solving the envelope
//! instead of every carrier cycle.

use super::options::parse_si_value;
use egui::Ui;

/// Modulation type for envelope analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModulationType {
    #[default]
    Am,
    Fm,
    Pm,
    Iq,
}

impl ModulationType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Am => "AM",
            Self::Fm => "FM",
            Self::Pm => "PM",
            Self::Iq => "IQ",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Am => "am",
            Self::Fm => "fm",
            Self::Pm => "pm",
            Self::Iq => "iq",
        }
    }
    pub fn all() -> &'static [ModulationType] {
        &[Self::Am, Self::Fm, Self::Pm, Self::Iq]
    }
}

/// Envelope transient analysis configuration
#[derive(Debug, Clone)]
pub struct EnvelopeConfig {
    pub fundamental_freq: f64,
    pub stop_time: f64,
    pub num_harmonics: u32,
    pub modulation: ModulationType,
    pub max_step: f64,
    pub reltol: f64,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        Self {
            fundamental_freq: 2.4e9,
            stop_time: 10e-6,
            num_harmonics: 9,
            modulation: ModulationType::Am,
            max_step: 0.0,
            reltol: 1e-3,
        }
    }
}

impl EnvelopeConfig {
    pub fn new(fundamental: f64, stop: f64) -> Self {
        Self {
            fundamental_freq: fundamental,
            stop_time: stop,
            ..Default::default()
        }
    }
    pub fn with_harmonics(mut self, n: u32) -> Self {
        self.num_harmonics = n;
        self
    }
    pub fn with_modulation(mut self, m: ModulationType) -> Self {
        self.modulation = m;
        self
    }

    pub fn carrier_period(&self) -> f64 {
        if self.fundamental_freq > 0.0 {
            1.0 / self.fundamental_freq
        } else {
            1.0
        }
    }

    pub fn num_cycles(&self) -> u64 {
        if self.fundamental_freq > 0.0 {
            (self.stop_time * self.fundamental_freq) as u64
        } else {
            0
        }
    }

    pub fn to_spice(&self) -> String {
        format!(
            ".envlp fund={} stop={} harmonics={}",
            format_freq(self.fundamental_freq),
            format_time(self.stop_time),
            self.num_harmonics
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.fundamental_freq <= 0.0 {
            return Err("Frequency must be positive".into());
        }
        if self.stop_time <= 0.0 {
            return Err("Stop time must be positive".into());
        }
        if self.num_harmonics == 0 {
            return Err("Harmonics must be at least 1".into());
        }
        if self.reltol <= 0.0 || self.reltol >= 1.0 {
            return Err("Reltol must be 0<x<1".into());
        }
        Ok(())
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnvelopeDialogState {
    pub fundamental: String,
    pub stop_time: String,
    pub harmonics: String,
    pub modulation_idx: usize,
    pub initialized: bool,
}

impl EnvelopeDialogState {
    pub fn from_config(config: &EnvelopeConfig) -> Self {
        Self {
            fundamental: format_freq(config.fundamental_freq),
            stop_time: format_time(config.stop_time),
            harmonics: config.num_harmonics.to_string(),
            modulation_idx: match config.modulation {
                ModulationType::Am => 0,
                ModulationType::Fm => 1,
                ModulationType::Pm => 2,
                ModulationType::Iq => 3,
            },
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<EnvelopeConfig, String> {
        let fund = parse_si_value(&self.fundamental).map_err(|e| format!("Bad freq: {}", e))?;
        let stop = parse_si_value(&self.stop_time).map_err(|e| format!("Bad time: {}", e))?;
        let harm: u32 = self.harmonics.parse().map_err(|_| "Bad harmonics")?;
        let modulation = match self.modulation_idx {
            0 => ModulationType::Am,
            1 => ModulationType::Fm,
            2 => ModulationType::Pm,
            _ => ModulationType::Iq,
        };
        let config = EnvelopeConfig {
            fundamental_freq: fund,
            stop_time: stop,
            num_harmonics: harm,
            modulation,
            max_step: 0.0,
            reltol: 1e-3,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&EnvelopeConfig::default());
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();
        ui.heading("Envelope Transient");
        ui.label(egui::RichText::new("Efficient modulated signal simulation").weak());
        ui.add_space(8.0);

        egui::Grid::new("env_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label("Carrier Freq:");
                ui.add(egui::TextEdit::singleline(&mut self.fundamental).desired_width(100.0));
                ui.end_row();
                ui.label("Stop Time:");
                ui.add(egui::TextEdit::singleline(&mut self.stop_time).desired_width(100.0));
                ui.end_row();
                ui.label("Harmonics:");
                ui.add(egui::TextEdit::singleline(&mut self.harmonics).desired_width(60.0));
                ui.end_row();
            });
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

fn format_time(t: f64) -> String {
    if t >= 1e-3 {
        format!("{}m", t / 1e-3)
    } else if t >= 1e-6 {
        format!("{}u", t / 1e-6)
    } else if t >= 1e-9 {
        format!("{}n", t / 1e-9)
    } else {
        format!("{}p", t / 1e-12)
    }
}


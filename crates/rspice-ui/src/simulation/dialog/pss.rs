//! Periodic Steady State (PSS) Analysis Configuration
//!
//! Configuration for PSS analysis (.pss).
//! Finds the periodic steady-state solution for circuits with periodic behavior.

use super::options::parse_si_value;
use egui::Ui;

/// PSS solver method
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PssSolverMethod {
    #[default]
    Shooting,
    HarmonicBalance,
}

impl PssSolverMethod {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Shooting => "Shooting",
            Self::HarmonicBalance => "Harmonic Balance",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Shooting => "shooting",
            Self::HarmonicBalance => "hb",
        }
    }
    pub fn all() -> &'static [PssSolverMethod] {
        &[Self::Shooting, Self::HarmonicBalance]
    }
}

/// PSS analysis configuration
#[derive(Debug, Clone)]
pub struct PssConfig {
    /// Fundamental frequency (Hz)
    pub fund_freq: f64,
    /// Number of harmonics to compute
    pub num_harmonics: u32,
    /// Steady-state accuracy (relative)
    pub stab_tol: f64,
    /// Maximum iterations
    pub max_iter: u32,
    /// Solver method
    pub method: PssSolverMethod,
    /// Oscillator mode (auto-detect frequency)
    pub osc_mode: bool,
    /// Oscillator node (for frequency detection)
    pub osc_node: String,
    /// Time step for shooting
    pub tstab: Option<f64>,
    /// Save all harmonics
    pub save_harmonics: bool,
}

impl Default for PssConfig {
    fn default() -> Self {
        Self {
            fund_freq: 1e6,
            num_harmonics: 10,
            stab_tol: 1e-3,
            max_iter: 50,
            method: PssSolverMethod::Shooting,
            osc_mode: false,
            osc_node: String::new(),
            tstab: None,
            save_harmonics: true,
        }
    }
}

impl PssConfig {
    pub fn new(freq: f64) -> Self {
        Self {
            fund_freq: freq,
            ..Default::default()
        }
    }
    pub fn with_harmonics(mut self, n: u32) -> Self {
        self.num_harmonics = n;
        self
    }
    pub fn with_method(mut self, m: PssSolverMethod) -> Self {
        self.method = m;
        self
    }
    pub fn with_osc_mode(mut self, node: &str) -> Self {
        self.osc_mode = true;
        self.osc_node = node.into();
        self
    }

    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".pss {} harmonics={}",
            format_freq(self.fund_freq),
            self.num_harmonics
        );
        cmd.push_str(&format!(" method={}", self.method.spice_keyword()));
        cmd.push_str(&format!(" maxiter={}", self.max_iter));
        if self.osc_mode && !self.osc_node.is_empty() {
            cmd.push_str(&format!(" oscmode=yes oscnode={}", self.osc_node));
        }
        cmd
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.fund_freq <= 0.0 {
            return Err("Fundamental frequency must be positive".into());
        }
        if self.num_harmonics == 0 {
            return Err("Number of harmonics must be at least 1".into());
        }
        if self.stab_tol <= 0.0 {
            return Err("Stability tolerance must be positive".into());
        }
        if self.max_iter == 0 {
            return Err("Max iterations must be at least 1".into());
        }
        if self.osc_mode && self.osc_node.is_empty() {
            return Err("Oscillator node required in osc mode".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct PssDialogState {
    pub fund_freq: String,
    pub num_harmonics: String,
    pub max_iter: String,
    pub method_idx: usize,
    pub osc_mode: bool,
    pub osc_node: String,
    pub save_harmonics: bool,
    pub initialized: bool,
}

impl PssDialogState {
    pub fn from_config(config: &PssConfig) -> Self {
        Self {
            fund_freq: format_freq(config.fund_freq),
            num_harmonics: config.num_harmonics.to_string(),
            max_iter: config.max_iter.to_string(),
            method_idx: match config.method {
                PssSolverMethod::Shooting => 0,
                PssSolverMethod::HarmonicBalance => 1,
            },
            osc_mode: config.osc_mode,
            osc_node: config.osc_node.clone(),
            save_harmonics: config.save_harmonics,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<PssConfig, String> {
        let freq = parse_si_value(&self.fund_freq).map_err(|e| format!("Bad frequency: {}", e))?;
        let harm: u32 = self
            .num_harmonics
            .parse()
            .map_err(|_| "Invalid harmonics")?;
        let iter: u32 = self.max_iter.parse().unwrap_or(50);
        let method = match self.method_idx {
            0 => PssSolverMethod::Shooting,
            _ => PssSolverMethod::HarmonicBalance,
        };
        let config = PssConfig {
            fund_freq: freq,
            num_harmonics: harm,
            stab_tol: 1e-3,
            max_iter: iter,
            method,
            osc_mode: self.osc_mode,
            osc_node: self.osc_node.clone(),
            tstab: None,
            save_harmonics: self.save_harmonics,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PssConfig::default());
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();
        ui.heading("Periodic Steady State (PSS)");
        ui.label(egui::RichText::new("Find periodic steady-state solution").weak());
        ui.add_space(8.0);

        egui::Grid::new("pss_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label("Fundamental Freq:");
                ui.add(egui::TextEdit::singleline(&mut self.fund_freq).desired_width(100.0));
                ui.end_row();
                ui.label("Harmonics:");
                ui.add(egui::TextEdit::singleline(&mut self.num_harmonics).desired_width(60.0));
                ui.end_row();
                ui.label("Max Iterations:");
                ui.add(egui::TextEdit::singleline(&mut self.max_iter).desired_width(60.0));
                ui.end_row();
            });
        ui.add_space(4.0);
        ui.checkbox(
            &mut self.osc_mode,
            "Oscillator Mode (auto-detect frequency)",
        );
        if self.osc_mode {
            ui.horizontal(|ui| {
                ui.label("Osc Node:");
                ui.add(egui::TextEdit::singleline(&mut self.osc_node).desired_width(80.0));
            });
        }
        ui.checkbox(&mut self.save_harmonics, "Save All Harmonics");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let c = PssConfig::default();
        assert_eq!(c.fund_freq, 1e6);
    }
    #[test]
    fn test_new() {
        let c = PssConfig::new(2.4e9);
        assert_eq!(c.fund_freq, 2.4e9);
    }
    #[test]
    fn test_with_harmonics() {
        let c = PssConfig::default().with_harmonics(20);
        assert_eq!(c.num_harmonics, 20);
    }
    #[test]
    fn test_with_method() {
        let c = PssConfig::default().with_method(PssSolverMethod::HarmonicBalance);
        assert_eq!(c.method, PssSolverMethod::HarmonicBalance);
    }
    #[test]
    fn test_with_osc() {
        let c = PssConfig::default().with_osc_mode("OUT");
        assert!(c.osc_mode);
        assert_eq!(c.osc_node, "OUT");
    }
    #[test]
    fn test_validate_ok() {
        assert!(PssConfig::default().validate().is_ok());
    }
    #[test]
    fn test_validate_zero_freq() {
        let mut c = PssConfig::default();
        c.fund_freq = 0.0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_zero_harm() {
        let mut c = PssConfig::default();
        c.num_harmonics = 0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_osc_no_node() {
        let mut c = PssConfig::default();
        c.osc_mode = true;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_to_spice() {
        let s = PssConfig::default().to_spice();
        assert!(s.contains(".pss"));
    }
    #[test]
    fn test_to_spice_osc() {
        let s = PssConfig::default().with_osc_mode("VCO").to_spice();
        assert!(s.contains("oscmode=yes"));
    }
    #[test]
    fn test_method_all() {
        assert_eq!(PssSolverMethod::all().len(), 2);
    }
    #[test]
    fn test_dialog_roundtrip() {
        let s = PssDialogState::from_config(&PssConfig::default());
        assert!(s.to_config().is_ok());
    }
    #[test]
    fn test_reset() {
        let mut c = PssConfig::new(5e9);
        c.reset();
        assert_eq!(c.fund_freq, 1e6);
    }
}

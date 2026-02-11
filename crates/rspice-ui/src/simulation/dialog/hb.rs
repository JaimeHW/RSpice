//! Harmonic Balance (HB) Analysis Configuration
//!
//! Configuration for large-signal frequency-domain analysis (.hb).
//! HB finds the periodic steady-state by solving for Fourier coefficients
//! directly in the frequency domain - essential for RF/microwave design.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Single-tone and multi-tone excitation
//! - Configurable harmonic count and oversampling
//! - Newton/Krylov solver options
//! - Automated mixing product truncation
//!
//! # Example SPICE Output
//!
//! ```text
//! .hb 1G harmonics=9 oversample=2 reltol=1e-6
//! ```

use super::options::parse_si_value;
use egui::Ui;
use serde::{Deserialize, Serialize};

// =============================================================================
// HB Solver Type
// =============================================================================

/// Solver type for Harmonic Balance
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HbSolverType {
    /// Standard Newton-Raphson
    #[default]
    Newton,
    /// Krylov subspace (GMRES) for large circuits
    Krylov,
}

impl HbSolverType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Newton => "Newton-Raphson",
            Self::Krylov => "Krylov (GMRES)",
        }
    }

    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Newton => "newton",
            Self::Krylov => "krylov",
        }
    }

    /// All available solvers
    pub fn all() -> &'static [HbSolverType] {
        &[Self::Newton, Self::Krylov]
    }
}

// =============================================================================
// HB Tone Configuration
// =============================================================================

/// Configuration for a single tone in multi-tone HB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HbToneConfig {
    /// Tone frequency (Hz)
    pub frequency: f64,
    /// Number of harmonics for this tone
    pub harmonics: u32,
    /// Tone name/label
    pub name: String,
    /// Optional independent source name this tone should drive.
    pub source: Option<String>,
}

impl HbToneConfig {
    /// Create new tone
    pub fn new(frequency: f64, harmonics: u32) -> Self {
        Self {
            frequency,
            harmonics,
            name: String::new(),
            source: None,
        }
    }

    /// Set tone name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set optional source routing for this tone.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();
        self.source = if source.trim().is_empty() {
            None
        } else {
            Some(source)
        };
        self
    }
}

impl Default for HbToneConfig {
    fn default() -> Self {
        Self {
            frequency: 1e9,
            harmonics: 9,
            name: String::new(),
            source: None,
        }
    }
}

// =============================================================================
// HB Analysis Configuration
// =============================================================================

/// Harmonic Balance analysis configuration
///
/// Commercial-grade configuration matching Cadence Spectre HB parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HbConfig {
    /// Fundamental frequency (Hz) - primary tone
    pub fundamental_freq: f64,
    /// Number of harmonics (DC through Nth)
    pub num_harmonics: u32,
    /// Optional primary tone name
    pub fundamental_name: String,
    /// Optional source routing for primary tone
    pub fundamental_source: Option<String>,
    /// Additional tones for multi-tone analysis
    pub additional_tones: Vec<HbToneConfig>,
    /// Oversampling factor for FFT (anti-aliasing)
    pub oversample: u32,
    /// Maximum mixing order for multi-tone
    pub max_mixing_order: u32,
    /// Relative convergence tolerance
    pub reltol: f64,
    /// Absolute convergence tolerance
    pub abstol: f64,
    /// Maximum Newton iterations
    pub maxiter: u32,
    /// Newton damping factor (0 < damping <= 1)
    pub damping: f64,
    /// Solver type
    pub solver: HbSolverType,
    /// GMRES restart parameter (for Krylov)
    pub gmres_restart: u32,
    /// Enable source stepping for difficult convergence
    pub source_stepping: bool,
    /// Verbose logging
    pub verbose: bool,
}

impl Default for HbConfig {
    fn default() -> Self {
        Self {
            fundamental_freq: 1e9, // 1 GHz RF default
            num_harmonics: 9,      // DC through 9th harmonic
            fundamental_name: "tone1".to_string(),
            fundamental_source: None,
            additional_tones: Vec::new(),
            oversample: 2,       // 2x oversampling (Spectre default)
            max_mixing_order: 5, // Typical for 2-tone IMD
            reltol: 1e-6,        // Spectre default
            abstol: 1e-12,       // 1 pA absolute
            maxiter: 100,        // Spectre default
            damping: 1.0,        // Full Newton step
            solver: HbSolverType::Newton,
            gmres_restart: 30,
            source_stepping: false,
            verbose: false,
        }
    }
}

impl HbConfig {
    /// Create new single-tone HB config
    pub fn new(fundamental: f64, harmonics: u32) -> Self {
        Self {
            fundamental_freq: fundamental,
            num_harmonics: harmonics,
            fundamental_name: "tone1".to_string(),
            fundamental_source: None,
            ..Default::default()
        }
    }

    /// Create multi-tone HB config
    pub fn multi_tone(tones: Vec<HbToneConfig>) -> Self {
        let fundamental = tones.first().map(|t| t.frequency).unwrap_or(1e9);
        let harmonics = tones.first().map(|t| t.harmonics).unwrap_or(9);
        let additional = if tones.len() > 1 {
            tones[1..].to_vec()
        } else {
            Vec::new()
        };

        Self {
            fundamental_freq: fundamental,
            num_harmonics: harmonics,
            fundamental_name: tones
                .first()
                .map(|tone| tone.name.clone())
                .unwrap_or_else(|| "tone1".to_string()),
            fundamental_source: tones.first().and_then(|tone| tone.source.clone()),
            additional_tones: additional,
            ..Default::default()
        }
    }

    /// Set oversampling factor
    pub fn with_oversample(mut self, factor: u32) -> Self {
        self.oversample = factor.max(1);
        self
    }

    /// Set convergence tolerance
    pub fn with_tolerance(mut self, reltol: f64) -> Self {
        self.reltol = reltol;
        self
    }

    /// Set solver type
    pub fn with_solver(mut self, solver: HbSolverType) -> Self {
        self.solver = solver;
        self
    }

    /// Enable source stepping
    pub fn with_source_stepping(mut self, enable: bool) -> Self {
        self.source_stepping = enable;
        self
    }

    /// Add a tone for multi-tone analysis
    pub fn add_tone(mut self, tone: HbToneConfig) -> Self {
        self.additional_tones.push(tone);
        self
    }

    /// Check if multi-tone
    pub fn is_multi_tone(&self) -> bool {
        !self.additional_tones.is_empty()
    }

    /// Primary tone as a full tone config.
    pub fn primary_tone(&self) -> HbToneConfig {
        let mut tone = HbToneConfig::new(self.fundamental_freq, self.num_harmonics);
        tone.name = self.fundamental_name.clone();
        tone.source = self.fundamental_source.clone();
        tone
    }

    /// Get all tones (primary + additional) in execution order.
    pub fn all_tones(&self) -> Vec<HbToneConfig> {
        let mut tones = Vec::with_capacity(1 + self.additional_tones.len());
        tones.push(self.primary_tone());
        tones.extend(self.additional_tones.iter().cloned());
        tones
    }

    /// Total number of spectral components per node
    pub fn num_spectral_components(&self) -> u32 {
        if self.is_multi_tone() {
            // Approximate for multi-tone (box truncation)
            let mut count = 1_u32; // DC
            count += 2 * self.num_harmonics;
            for tone in &self.additional_tones {
                count += 2 * tone.harmonics;
            }
            count
        } else {
            // Single-tone: DC + harmonics (negative are conjugates)
            self.num_harmonics + 1
        }
    }

    /// Estimated FFT size
    pub fn fft_size(&self) -> u32 {
        let min_size = self.num_spectral_components() * self.oversample;
        min_size.next_power_of_two()
    }

    /// Fundamental period in seconds
    pub fn period(&self) -> f64 {
        if self.fundamental_freq > 0.0 {
            1.0 / self.fundamental_freq
        } else {
            1.0
        }
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".hb {} harmonics={} oversample={}",
            format_freq(self.fundamental_freq),
            self.num_harmonics,
            self.oversample
        );

        // Add multi-tone frequencies
        for (i, tone) in self.additional_tones.iter().enumerate() {
            cmd.push_str(&format!(
                " tone{}={} tone{}harm={}",
                i + 2,
                format_freq(tone.frequency),
                i + 2,
                tone.harmonics
            ));
        }

        // Convergence options
        if (self.reltol - 1e-6).abs() > 1e-10 {
            cmd.push_str(&format!(" reltol={:.0e}", self.reltol));
        }

        if self.maxiter != 100 {
            cmd.push_str(&format!(" maxiter={}", self.maxiter));
        }

        if self.solver == HbSolverType::Krylov {
            cmd.push_str(&format!(
                " solver=krylov gmres_restart={}",
                self.gmres_restart
            ));
        }

        if self.source_stepping {
            cmd.push_str(" sourcestepping=yes");
        }

        cmd
    }

    /// Generate SPICE directive (legacy compatibility helper).
    pub fn to_spice_string(&self) -> String {
        self.to_spice()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.fundamental_freq <= 0.0 {
            return Err("Fundamental frequency must be positive".to_string());
        }

        if self.num_harmonics == 0 {
            return Err("Number of harmonics must be at least 1".to_string());
        }
        if self
            .fundamental_source
            .as_deref()
            .map(str::trim)
            .is_some_and(|name| name.is_empty())
        {
            return Err("Fundamental tone source cannot be empty".to_string());
        }

        if self.oversample == 0 {
            return Err("Oversample factor must be at least 1".to_string());
        }

        if self.reltol <= 0.0 || self.reltol >= 1.0 {
            return Err("Relative tolerance must be between 0 and 1".to_string());
        }

        if self.maxiter == 0 {
            return Err("Maximum iterations must be at least 1".to_string());
        }

        if self.damping <= 0.0 || self.damping > 1.0 {
            return Err("Damping factor must be between 0 and 1".to_string());
        }

        // Validate additional tones
        for (i, tone) in self.additional_tones.iter().enumerate() {
            if tone.frequency <= 0.0 {
                return Err(format!("Tone {} frequency must be positive", i + 2));
            }
            if tone.harmonics == 0 {
                return Err(format!("Tone {} harmonics must be at least 1", i + 2));
            }
            if tone
                .source
                .as_deref()
                .map(str::trim)
                .is_some_and(|name| name.is_empty())
            {
                return Err(format!("Tone {} source cannot be empty", i + 2));
            }
        }

        Ok(())
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// Dialog State for String Buffers
// =============================================================================

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

    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("Harmonic Balance Analysis");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Large-signal frequency-domain steady-state for RF circuits")
                .weak(),
        );
        ui.add_space(12.0);

        // Fundamental Tone
        ui.group(|ui| {
            ui.label(egui::RichText::new("Fundamental Tone").strong());
            ui.add_space(4.0);

            egui::Grid::new("hb_fund_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Frequency:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fundamental)
                            .desired_width(120.0)
                            .hint_text("1G"),
                    );
                    ui.end_row();

                    ui.label("Harmonics:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.harmonics)
                            .desired_width(120.0)
                            .hint_text("9"),
                    );
                    ui.end_row();

                    ui.label("Tone Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fundamental_name)
                            .desired_width(120.0)
                            .hint_text("tone1"),
                    );
                    ui.end_row();

                    ui.label("Source:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fundamental_source)
                            .desired_width(120.0)
                            .hint_text("V1"),
                    );
                    ui.end_row();

                    ui.label("Oversample:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.oversample)
                            .desired_width(120.0)
                            .hint_text("2"),
                    );
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Additional tones.
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Additional Tones").strong());
                if ui.small_button("+ Add Tone").clicked() {
                    self.additional_tones.push(HbToneDialogState {
                        frequency: "900Meg".to_string(),
                        harmonics: "5".to_string(),
                        name: format!("tone{}", self.additional_tones.len() + 2),
                        source: String::new(),
                    });
                }
            });
            ui.add_space(4.0);

            if self.additional_tones.is_empty() {
                ui.label(
                    egui::RichText::new("No additional tones configured")
                        .small()
                        .weak(),
                );
            } else {
                let mut remove_index = None;
                for (idx, tone) in self.additional_tones.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("Tone {}:", idx + 2));
                        ui.label("F");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.frequency)
                                .desired_width(80.0)
                                .hint_text("900Meg"),
                        );
                        ui.label("H");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.harmonics)
                                .desired_width(40.0)
                                .hint_text("5"),
                        );
                        ui.label("Name");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.name)
                                .desired_width(70.0)
                                .hint_text("LO"),
                        );
                        ui.label("Source");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.source)
                                .desired_width(70.0)
                                .hint_text("VLO"),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove_index = Some(idx);
                        }
                    });
                }
                if let Some(idx) = remove_index {
                    self.additional_tones.remove(idx);
                }
            }
        });

        ui.add_space(8.0);

        // Convergence
        ui.group(|ui| {
            ui.label(egui::RichText::new("Convergence").strong());
            ui.add_space(4.0);

            egui::Grid::new("hb_conv_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Rel. Tolerance:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.reltol)
                            .desired_width(120.0)
                            .hint_text("1e-6"),
                    );
                    ui.end_row();

                    ui.label("Max Iterations:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.maxiter)
                            .desired_width(120.0)
                            .hint_text("100"),
                    );
                    ui.end_row();

                    ui.label("Damping:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.damping)
                            .desired_width(120.0)
                            .hint_text("1.0"),
                    );
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Solver Options
        ui.group(|ui| {
            ui.label(egui::RichText::new("Solver Options").strong());
            ui.add_space(4.0);

            egui::Grid::new("hb_solver_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Solver:");
                    let solvers = ["Newton-Raphson", "Krylov (GMRES)"];
                    egui::ComboBox::from_id_salt("hb_solver")
                        .selected_text(solvers[self.solver_idx])
                        .show_ui(ui, |ui| {
                            for (idx, name) in solvers.iter().enumerate() {
                                if ui.selectable_label(self.solver_idx == idx, *name).clicked() {
                                    self.solver_idx = idx;
                                }
                            }
                        });
                    ui.end_row();

                    if self.solver_idx == 1 {
                        ui.label("GMRES Restart:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.gmres_restart)
                                .desired_width(120.0)
                                .hint_text("30"),
                        );
                        ui.end_row();
                    }
                });

            ui.add_space(4.0);
            ui.checkbox(
                &mut self.source_stepping,
                "Enable Source Stepping (for difficult convergence)",
            );
        });

        // Info footer
        if let Ok(config) = self.to_config() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!(
                    "Spectral components: {} | FFT size: {}",
                    config.num_spectral_components(),
                    config.fft_size()
                ))
                .size(10.0)
                .color(egui::Color32::from_rgb(120, 125, 135)),
            );
        }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

fn format_freq(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{}G", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{}Meg", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{}k", freq / 1e3)
    } else {
        format!("{}", freq)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // HbConfig Basic Tests
    // =========================================================================

    #[test]
    fn test_config_default() {
        let cfg = HbConfig::default();
        assert_eq!(cfg.fundamental_freq, 1e9);
        assert_eq!(cfg.num_harmonics, 9);
        assert_eq!(cfg.fundamental_name, "tone1");
        assert!(cfg.fundamental_source.is_none());
        assert_eq!(cfg.oversample, 2);
        assert!(!cfg.is_multi_tone());
    }

    #[test]
    fn test_config_new() {
        let cfg = HbConfig::new(2.4e9, 15);
        assert_eq!(cfg.fundamental_freq, 2.4e9);
        assert_eq!(cfg.num_harmonics, 15);
    }

    #[test]
    fn test_config_multi_tone() {
        let tones = vec![
            HbToneConfig::new(1e9, 9).with_name("RF"),
            HbToneConfig::new(900e6, 5).with_name("LO"),
        ];
        let cfg = HbConfig::multi_tone(tones);

        assert!(cfg.is_multi_tone());
        assert_eq!(cfg.fundamental_freq, 1e9);
        assert_eq!(cfg.fundamental_name, "RF");
        assert_eq!(cfg.additional_tones.len(), 1);
    }

    #[test]
    fn test_config_builder_chain() {
        let cfg = HbConfig::new(1e9, 9)
            .with_oversample(4)
            .with_tolerance(1e-8)
            .with_solver(HbSolverType::Krylov)
            .with_source_stepping(true);

        assert_eq!(cfg.oversample, 4);
        assert_eq!(cfg.reltol, 1e-8);
        assert_eq!(cfg.solver, HbSolverType::Krylov);
        assert!(cfg.source_stepping);
    }

    #[test]
    fn test_config_add_tone() {
        let cfg = HbConfig::new(1e9, 9).add_tone(HbToneConfig::new(800e6, 5));

        assert!(cfg.is_multi_tone());
        assert_eq!(cfg.additional_tones.len(), 1);
        assert_eq!(cfg.additional_tones[0].frequency, 800e6);
    }

    #[test]
    fn test_config_all_tones_includes_primary_and_additional() {
        let cfg = HbConfig::new(1e9, 9).add_tone(HbToneConfig::new(800e6, 5).with_name("LO"));
        let tones = cfg.all_tones();
        assert_eq!(tones.len(), 2);
        assert_eq!(tones[0].frequency, 1e9);
        assert_eq!(tones[1].name, "LO");
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_valid() {
        let cfg = HbConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_frequency() {
        let mut cfg = HbConfig::default();
        cfg.fundamental_freq = 0.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_negative_frequency() {
        let mut cfg = HbConfig::default();
        cfg.fundamental_freq = -1e9;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_harmonics() {
        let mut cfg = HbConfig::default();
        cfg.num_harmonics = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_oversample() {
        let mut cfg = HbConfig::default();
        cfg.oversample = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_reltol_zero() {
        let mut cfg = HbConfig::default();
        cfg.reltol = 0.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_reltol_too_large() {
        let mut cfg = HbConfig::default();
        cfg.reltol = 1.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_maxiter() {
        let mut cfg = HbConfig::default();
        cfg.maxiter = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_damping_zero() {
        let mut cfg = HbConfig::default();
        cfg.damping = 0.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_damping_too_large() {
        let mut cfg = HbConfig::default();
        cfg.damping = 1.5;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_tone_frequency() {
        let cfg = HbConfig::new(1e9, 9).add_tone(HbToneConfig::new(-800e6, 5));
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_tone_harmonics() {
        let cfg = HbConfig::new(1e9, 9).add_tone(HbToneConfig::new(800e6, 0));
        assert!(cfg.validate().is_err());
    }

    // =========================================================================
    // SPICE Generation Tests
    // =========================================================================

    #[test]
    fn test_to_spice_single_tone() {
        let cfg = HbConfig::new(1e9, 9);
        let spice = cfg.to_spice();

        assert!(spice.starts_with(".hb"));
        assert!(spice.contains("1G"));
        assert!(spice.contains("harmonics=9"));
        assert!(spice.contains("oversample=2"));
        assert_eq!(cfg.to_spice_string(), spice);
    }

    #[test]
    fn test_to_spice_multi_tone() {
        let cfg = HbConfig::new(1e9, 9).add_tone(HbToneConfig::new(800e6, 5));
        let spice = cfg.to_spice();

        assert!(spice.contains("tone2=800Meg"));
        assert!(spice.contains("tone2harm=5"));
    }

    #[test]
    fn test_to_spice_custom_tolerance() {
        let cfg = HbConfig::new(1e9, 9).with_tolerance(1e-9);
        let spice = cfg.to_spice();

        assert!(spice.contains("reltol="));
    }

    #[test]
    fn test_to_spice_krylov_solver() {
        let cfg = HbConfig::new(1e9, 9).with_solver(HbSolverType::Krylov);
        let spice = cfg.to_spice();

        assert!(spice.contains("solver=krylov"));
        assert!(spice.contains("gmres_restart="));
    }

    #[test]
    fn test_to_spice_source_stepping() {
        let cfg = HbConfig::new(1e9, 9).with_source_stepping(true);
        let spice = cfg.to_spice();

        assert!(spice.contains("sourcestepping=yes"));
    }

    #[test]
    fn test_to_spice_custom_maxiter() {
        let mut cfg = HbConfig::default();
        cfg.maxiter = 200;
        let spice = cfg.to_spice();

        assert!(spice.contains("maxiter=200"));
    }

    // =========================================================================
    // Spectral Component Tests
    // =========================================================================

    #[test]
    fn test_num_spectral_components_single_tone() {
        let cfg = HbConfig::new(1e9, 9);
        // DC + 9 harmonics = 10
        assert_eq!(cfg.num_spectral_components(), 10);
    }

    #[test]
    fn test_num_spectral_components_multi_tone() {
        let cfg = HbConfig::new(1e9, 9).add_tone(HbToneConfig::new(800e6, 5));
        // 1 (DC) + 2*9 (primary) + 2*5 (secondary) = 29
        assert_eq!(cfg.num_spectral_components(), 29);
    }

    #[test]
    fn test_fft_size_power_of_two() {
        let cfg = HbConfig::new(1e9, 9);
        let fft_size = cfg.fft_size();
        assert!(fft_size.is_power_of_two());
    }

    #[test]
    fn test_fft_size_minimum() {
        let cfg = HbConfig::new(1e9, 9);
        let fft_size = cfg.fft_size();
        // Should be at least spectral_components * oversample
        assert!(fft_size >= cfg.num_spectral_components() * cfg.oversample);
    }

    #[test]
    fn test_period() {
        let cfg = HbConfig::new(1e9, 9);
        assert!((cfg.period() - 1e-9).abs() < 1e-20);
    }

    #[test]
    fn test_period_zero_freq() {
        let mut cfg = HbConfig::default();
        cfg.fundamental_freq = 0.0;
        assert_eq!(cfg.period(), 1.0);
    }

    // =========================================================================
    // HbSolverType Tests
    // =========================================================================

    #[test]
    fn test_solver_display_names() {
        assert!(!HbSolverType::Newton.display_name().is_empty());
        assert!(!HbSolverType::Krylov.display_name().is_empty());
    }

    #[test]
    fn test_solver_spice_keywords() {
        assert_eq!(HbSolverType::Newton.spice_keyword(), "newton");
        assert_eq!(HbSolverType::Krylov.spice_keyword(), "krylov");
    }

    #[test]
    fn test_solver_all() {
        let all = HbSolverType::all();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&HbSolverType::Newton));
        assert!(all.contains(&HbSolverType::Krylov));
    }

    // =========================================================================
    // HbToneConfig Tests
    // =========================================================================

    #[test]
    fn test_tone_config_new() {
        let tone = HbToneConfig::new(2.4e9, 7);
        assert_eq!(tone.frequency, 2.4e9);
        assert_eq!(tone.harmonics, 7);
        assert!(tone.name.is_empty());
        assert!(tone.source.is_none());
    }

    #[test]
    fn test_tone_config_with_name() {
        let tone = HbToneConfig::new(2.4e9, 7).with_name("WiFi");
        assert_eq!(tone.name, "WiFi");
    }

    #[test]
    fn test_tone_config_with_source() {
        let tone = HbToneConfig::new(2.4e9, 7).with_source("VLO");
        assert_eq!(tone.source.as_deref(), Some("VLO"));
    }

    #[test]
    fn test_tone_config_default() {
        let tone = HbToneConfig::default();
        assert_eq!(tone.frequency, 1e9);
        assert_eq!(tone.harmonics, 9);
    }

    // =========================================================================
    // Reset Tests
    // =========================================================================

    #[test]
    fn test_config_reset() {
        let mut cfg = HbConfig::new(2.4e9, 15)
            .with_oversample(8)
            .with_solver(HbSolverType::Krylov);

        cfg.reset();

        assert_eq!(cfg.fundamental_freq, 1e9);
        assert_eq!(cfg.num_harmonics, 9);
        assert_eq!(cfg.fundamental_name, "tone1");
        assert!(cfg.fundamental_source.is_none());
        assert_eq!(cfg.oversample, 2);
        assert_eq!(cfg.solver, HbSolverType::Newton);
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_format_freq_giga() {
        assert_eq!(format_freq(1e9), "1G");
        assert_eq!(format_freq(2.4e9), "2.4G");
    }

    #[test]
    fn test_format_freq_mega() {
        assert_eq!(format_freq(1e6), "1Meg");
        assert_eq!(format_freq(800e6), "800Meg");
    }

    #[test]
    fn test_format_freq_kilo() {
        assert_eq!(format_freq(1e3), "1k");
        assert_eq!(format_freq(10e3), "10k");
    }

    #[test]
    fn test_format_freq_plain() {
        assert_eq!(format_freq(100.0), "100");
    }

    // =========================================================================
    // Dialog State Tests
    // =========================================================================

    #[test]
    fn test_dialog_state_from_config() {
        let cfg = HbConfig::new(2.4e9, 15);
        let state = HbDialogState::from_config(&cfg);

        assert!(state.initialized);
        assert!(state.fundamental.contains("2.4G"));
        assert_eq!(state.harmonics, "15");
        assert_eq!(state.fundamental_name, "tone1");
    }

    #[test]
    fn test_dialog_state_to_config() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.fundamental = "1G".to_string();
        state.harmonics = "9".to_string();
        state.fundamental_name = "RF".to_string();
        state.fundamental_source = "VRF".to_string();

        let result = state.to_config();
        assert!(result.is_ok());

        let cfg = result.unwrap();
        assert_eq!(cfg.fundamental_freq, 1e9);
        assert_eq!(cfg.num_harmonics, 9);
        assert_eq!(cfg.fundamental_name, "RF");
        assert_eq!(cfg.fundamental_source.as_deref(), Some("VRF"));
    }

    #[test]
    fn test_dialog_state_invalid_frequency() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.fundamental = "invalid".to_string();

        let result = state.to_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_dialog_state_multi_tone() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.additional_tones.push(HbToneDialogState {
            frequency: "800Meg".to_string(),
            harmonics: "5".to_string(),
            name: "LO".to_string(),
            source: "VLO".to_string(),
        });

        let result = state.to_config();
        assert!(result.is_ok());

        let cfg = result.unwrap();
        assert!(cfg.is_multi_tone());
        assert_eq!(cfg.additional_tones[0].frequency, 800e6);
        assert_eq!(cfg.additional_tones[0].name, "LO");
        assert_eq!(cfg.additional_tones[0].source.as_deref(), Some("VLO"));
    }

    #[test]
    fn test_dialog_state_parses_multiple_additional_tones() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.additional_tones = vec![
            HbToneDialogState {
                frequency: "900Meg".to_string(),
                harmonics: "5".to_string(),
                name: "LO".to_string(),
                source: "VLO".to_string(),
            },
            HbToneDialogState {
                frequency: "2.1G".to_string(),
                harmonics: "3".to_string(),
                name: "AUX".to_string(),
                source: String::new(),
            },
        ];

        let cfg = state
            .to_config()
            .expect("multiple additional tones should parse");
        assert_eq!(cfg.additional_tones.len(), 2);
        assert!((cfg.additional_tones[0].frequency - 900e6).abs() < 1e-3);
        assert_eq!(cfg.additional_tones[0].source.as_deref(), Some("VLO"));
        assert!((cfg.additional_tones[1].frequency - 2.1e9).abs() < 1e-3);
        assert!(cfg.additional_tones[1].source.is_none());
    }

    #[test]
    fn test_dialog_state_skips_completely_empty_additional_row() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.additional_tones.push(HbToneDialogState::default());
        let cfg = state.to_config().expect("empty tone row should be ignored");
        assert!(cfg.additional_tones.is_empty());
    }

    #[test]
    fn test_dialog_state_ensure_initialized() {
        let mut state = HbDialogState::default();
        assert!(!state.initialized);

        state.ensure_initialized();
        assert!(state.initialized);
        assert!(!state.fundamental.is_empty());
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_with_oversample_clamps_minimum() {
        let cfg = HbConfig::new(1e9, 9).with_oversample(0);
        assert_eq!(cfg.oversample, 1);
    }

    #[test]
    fn test_single_harmonic() {
        let cfg = HbConfig::new(1e9, 1);
        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.num_spectral_components(), 2); // DC + 1 harmonic
    }

    #[test]
    fn test_large_harmonics() {
        let cfg = HbConfig::new(1e9, 100);
        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.num_spectral_components(), 101);
    }

    #[test]
    fn test_very_low_frequency() {
        let cfg = HbConfig::new(1.0, 9); // 1 Hz
        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.period(), 1.0);
    }

    #[test]
    fn test_very_high_frequency() {
        let cfg = HbConfig::new(100e9, 9); // 100 GHz
        assert!(cfg.validate().is_ok());
        assert!((cfg.period() - 1e-11).abs() < 1e-25);
    }
}

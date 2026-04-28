//! Periodic Stability (PSTB) Analysis Configuration
//!
//! Configuration for periodic stability analysis around a PSS operating point.
//! PSTB uses Floquet analysis to determine the stability of periodic circuits
//! such as oscillators and switched-capacitor filters.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Floquet multiplier computation
//! - Loop gain and phase margin
//! - Stability margin visualization
//!
//! # Example SPICE Output
//!
//! ```text
//! .pstb probe=LPROBE
//! + maxharm=10 annotate=yes
//! ```

use egui::Ui;

// =============================================================================
// PSTB Configuration
// =============================================================================

/// Periodic stability (PSTB) analysis configuration
#[derive(Debug, Clone)]
pub struct PstbConfig {
    /// Probe instance name (loop break)
    pub probe: String,
    /// Maximum harmonics for analysis
    pub max_harmonics: u32,
    /// Annotate results in schematic
    pub annotate: bool,
    /// Compute phase margin
    pub phase_margin: bool,
    /// Compute gain margin
    pub gain_margin: bool,
    /// Frequency for stability check (0 = all)
    pub check_freq: f64,
    /// Number of Floquet multipliers to compute
    pub num_multipliers: u32,
    /// Fundamental frequency from PSS
    pub fundamental_freq: f64,
}

impl Default for PstbConfig {
    fn default() -> Self {
        Self {
            probe: "LPROBE".to_string(),
            max_harmonics: 10,
            annotate: true,
            phase_margin: true,
            gain_margin: true,
            check_freq: 0.0,
            num_multipliers: 10,
            fundamental_freq: 0.0,
        }
    }
}

impl PstbConfig {
    /// Create new PSTB config with probe name
    pub fn new(probe: &str) -> Self {
        Self {
            probe: probe.to_uppercase(),
            ..Default::default()
        }
    }

    /// Set maximum harmonics
    pub fn with_harmonics(mut self, max: u32) -> Self {
        self.max_harmonics = max;
        self
    }

    /// Set number of multipliers
    pub fn with_multipliers(mut self, num: u32) -> Self {
        self.num_multipliers = num;
        self
    }

    /// Enable/disable annotation
    pub fn with_annotate(mut self, enable: bool) -> Self {
        self.annotate = enable;
        self
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(".pstb probe={}", self.probe);

        cmd.push_str(&format!(" maxharm={}", self.max_harmonics));

        if self.annotate {
            cmd.push_str(" annotate=yes");
        }

        if self.num_multipliers != 10 {
            cmd.push_str(&format!(" nmults={}", self.num_multipliers));
        }

        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.probe.is_empty() {
            return Err("Probe instance must be specified".to_string());
        }
        if self.max_harmonics == 0 {
            return Err("Maximum harmonics must be at least 1".to_string());
        }
        if self.num_multipliers == 0 {
            return Err("Number of multipliers must be at least 1".to_string());
        }
        Ok(())
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with string buffers
#[derive(Debug, Clone, Default)]
pub struct PstbDialogState {
    pub probe: String,
    pub max_harmonics: String,
    pub num_multipliers: String,
    pub annotate: bool,
    pub phase_margin: bool,
    pub gain_margin: bool,
    pub initialized: bool,
}

impl PstbDialogState {
    /// Initialize from config
    pub fn from_config(config: &PstbConfig) -> Self {
        Self {
            probe: config.probe.clone(),
            max_harmonics: config.max_harmonics.to_string(),
            num_multipliers: config.num_multipliers.to_string(),
            annotate: config.annotate,
            phase_margin: config.phase_margin,
            gain_margin: config.gain_margin,
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PstbConfig, String> {
        let max_harm: u32 = self
            .max_harmonics
            .parse()
            .map_err(|_| "Invalid harmonics")?;
        let num_mult: u32 = self
            .num_multipliers
            .parse()
            .map_err(|_| "Invalid multipliers")?;

        let config = PstbConfig {
            probe: self.probe.clone(),
            max_harmonics: max_harm,
            annotate: self.annotate,
            phase_margin: self.phase_margin,
            gain_margin: self.gain_margin,
            check_freq: 0.0,
            num_multipliers: num_mult,
            fundamental_freq: 0.0,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PstbConfig::default());
        }
    }

    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("Periodic Stability Analysis");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Floquet stability analysis around PSS (requires PSS)").weak(),
        );
        ui.add_space(12.0);

        // Probe Configuration
        ui.group(|ui| {
            ui.label(egui::RichText::new("Loop Probe").strong());
            egui::Grid::new("pstb_probe_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Probe Instance:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.probe)
                            .desired_width(120.0)
                            .hint_text("LPROBE"),
                    );
                    ui.end_row();
                    ui.label("Max Harmonics:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.max_harmonics)
                            .desired_width(60.0)
                            .hint_text("10"),
                    );
                    ui.end_row();
                    ui.label("Multipliers:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.num_multipliers)
                            .desired_width(60.0)
                            .hint_text("10"),
                    );
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Options
        ui.group(|ui| {
            ui.label(egui::RichText::new("Results").strong());
            ui.checkbox(&mut self.annotate, "Annotate schematic");
            ui.checkbox(&mut self.phase_margin, "Compute phase margin");
            ui.checkbox(&mut self.gain_margin, "Compute gain margin");
        });
    }
}

// =============================================================================
// Tests
// =============================================================================


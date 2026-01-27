//! Periodic Noise (PNoise) Analysis Configuration
//!
//! Configuration for periodic noise analysis around a PSS operating point.
//! PNoise computes noise contributions from all components, including the
//! frequency translation effects that are critical for oscillators and mixers.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Sideband noise folding
//! - Spot noise and integrated noise
//! - Phase noise and jitter calculation
//! - Per-device noise contribution
//!
//! # Example SPICE Output
//!
//! ```text
//! .pnoise dec 10 1 1Meg
//! + output=VOUT maxsideband=5 noiseref=yes
//! ```

use super::options::parse_si_value;
use egui::Ui;

// =============================================================================
// PNoise Sweep Type
// =============================================================================

/// Type of frequency sweep for PNoise analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PnoiseSweepType {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl PnoiseSweepType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Decade => "Decade",
            Self::Octave => "Octave",
            Self::Linear => "Linear",
        }
    }

    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }

    /// All types
    pub fn all() -> &'static [PnoiseSweepType] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

// =============================================================================
// Noise Reference Type
// =============================================================================

/// Type of noise reference for PNoise
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NoiseReferenceType {
    /// Output-referred noise (V/√Hz or A/√Hz)
    #[default]
    Output,
    /// Input-referred noise
    Input,
    /// Phase noise (dBc/Hz)
    Phase,
}

impl NoiseReferenceType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Output => "Output-Referred",
            Self::Input => "Input-Referred",
            Self::Phase => "Phase Noise (dBc/Hz)",
        }
    }

    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Output => "output",
            Self::Input => "input",
            Self::Phase => "phase",
        }
    }

    /// All types
    pub fn all() -> &'static [NoiseReferenceType] {
        &[Self::Output, Self::Input, Self::Phase]
    }
}

// =============================================================================
// PNoise Configuration
// =============================================================================

/// Periodic noise (PNoise) analysis configuration
///
/// Commercial-grade configuration matching Cadence Spectre PNoise parameters.
#[derive(Debug, Clone)]
pub struct PnoiseConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points (per decade for log, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: PnoiseSweepType,
    /// Maximum sideband index for noise folding
    pub max_sideband: i32,
    /// Output node name
    pub output_node: String,
    /// Reference node (ground if empty)
    pub output_ref: String,
    /// Noise reference type
    pub noise_ref: NoiseReferenceType,
    /// Include spot noise
    pub spot_noise: bool,
    /// Include integrated noise
    pub integrated_noise: bool,
    /// Noise summary (per-device contributions)
    pub noise_summary: bool,
    /// Fundamental frequency from PSS (set from PSS result)
    pub fundamental_freq: f64,
}

impl Default for PnoiseConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0, // 1 Hz (for phase noise)
            stop_freq: 1e6,  // 1 MHz
            num_points: 10,  // 10 per decade
            sweep_type: PnoiseSweepType::Decade,
            max_sideband: 5, // Sidebands to fold
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            noise_ref: NoiseReferenceType::Output,
            spot_noise: true,
            integrated_noise: false,
            noise_summary: true,
            fundamental_freq: 0.0, // Set from PSS
        }
    }
}

impl PnoiseConfig {
    /// Create new PNoise config
    pub fn new(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            ..Default::default()
        }
    }

    /// Create config for phase noise analysis
    pub fn phase_noise(offset_start: f64, offset_stop: f64) -> Self {
        Self {
            start_freq: offset_start,
            stop_freq: offset_stop,
            num_points: 10,
            sweep_type: PnoiseSweepType::Decade,
            noise_ref: NoiseReferenceType::Phase,
            spot_noise: true,
            ..Default::default()
        }
    }

    /// Set output node
    pub fn with_output(mut self, node: &str) -> Self {
        self.output_node = node.to_uppercase();
        self
    }

    /// Set noise reference type
    pub fn with_noise_ref(mut self, noise_ref: NoiseReferenceType) -> Self {
        self.noise_ref = noise_ref;
        self
    }

    /// Set sideband range
    pub fn with_sidebands(mut self, max: i32) -> Self {
        self.max_sideband = max.abs();
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: PnoiseSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Enable integrated noise calculation
    pub fn with_integrated_noise(mut self, enable: bool) -> Self {
        self.integrated_noise = enable;
        self
    }

    /// Set fundamental frequency
    pub fn with_fundamental(mut self, freq: f64) -> Self {
        self.fundamental_freq = freq;
        self
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            PnoiseSweepType::Decade => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            PnoiseSweepType::Octave => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            PnoiseSweepType::Linear => self.num_points,
        }
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".pnoise {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

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

        cmd.push_str(&format!(" maxsideband={}", self.max_sideband));

        if self.noise_ref != NoiseReferenceType::Output {
            cmd.push_str(&format!(" noiseref={}", self.noise_ref.spice_keyword()));
        }

        if self.integrated_noise {
            cmd.push_str(" integratedNoise=yes");
        }

        if self.noise_summary {
            cmd.push_str(" noiseSummary=yes");
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

        if self.max_sideband < 0 {
            return Err("Maximum sideband must be non-negative".to_string());
        }

        if self.output_node.is_empty() {
            return Err("Output node must be specified".to_string());
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
pub struct PnoiseDialogState {
    /// Start frequency buffer
    pub start_freq: String,
    /// Stop frequency buffer
    pub stop_freq: String,
    /// Points buffer
    pub num_points: String,
    /// Sweep type index
    pub sweep_type_idx: usize,
    /// Max sideband buffer
    pub max_sideband: String,
    /// Output node buffer
    pub output_node: String,
    /// Output reference buffer
    pub output_ref: String,
    /// Noise reference type index
    pub noise_ref_idx: usize,
    /// Spot noise enabled
    pub spot_noise: bool,
    /// Integrated noise enabled
    pub integrated_noise: bool,
    /// Noise summary enabled
    pub noise_summary: bool,
    /// Initialized flag
    pub initialized: bool,
}

impl PnoiseDialogState {
    /// Initialize from config
    pub fn from_config(config: &PnoiseConfig) -> Self {
        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                PnoiseSweepType::Decade => 0,
                PnoiseSweepType::Octave => 1,
                PnoiseSweepType::Linear => 2,
            },
            max_sideband: config.max_sideband.to_string(),
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            noise_ref_idx: match config.noise_ref {
                NoiseReferenceType::Output => 0,
                NoiseReferenceType::Input => 1,
                NoiseReferenceType::Phase => 2,
            },
            spot_noise: config.spot_noise,
            integrated_noise: config.integrated_noise,
            noise_summary: config.noise_summary,
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PnoiseConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;

        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;

        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;

        let max_sb: i32 = self.max_sideband.parse().map_err(|_| "Invalid sideband")?;

        let sweep_type = match self.sweep_type_idx {
            0 => PnoiseSweepType::Decade,
            1 => PnoiseSweepType::Octave,
            _ => PnoiseSweepType::Linear,
        };

        let noise_ref = match self.noise_ref_idx {
            0 => NoiseReferenceType::Output,
            1 => NoiseReferenceType::Input,
            _ => NoiseReferenceType::Phase,
        };

        let config = PnoiseConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            max_sideband: max_sb,
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            noise_ref,
            spot_noise: self.spot_noise,
            integrated_noise: self.integrated_noise,
            noise_summary: self.noise_summary,
            fundamental_freq: 0.0,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PnoiseConfig::default());
        }
    }

    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("Periodic Noise Analysis");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Noise analysis around periodic steady-state (requires PSS)")
                .weak(),
        );
        ui.add_space(12.0);

        // Frequency Sweep
        ui.group(|ui| {
            ui.label(egui::RichText::new("Offset Frequency Sweep").strong());
            ui.add_space(4.0);

            egui::Grid::new("pnoise_freq_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Start:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.start_freq)
                            .desired_width(120.0)
                            .hint_text("1"),
                    );
                    ui.end_row();

                    ui.label("Stop:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.stop_freq)
                            .desired_width(120.0)
                            .hint_text("1Meg"),
                    );
                    ui.end_row();

                    ui.label("Points:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.num_points)
                            .desired_width(120.0)
                            .hint_text("10"),
                    );
                    ui.end_row();

                    ui.label("Sweep:");
                    let sweeps = ["Decade", "Octave", "Linear"];
                    egui::ComboBox::from_id_salt("pnoise_sweep")
                        .selected_text(sweeps[self.sweep_type_idx])
                        .show_ui(ui, |ui| {
                            for (idx, name) in sweeps.iter().enumerate() {
                                if ui
                                    .selectable_label(self.sweep_type_idx == idx, *name)
                                    .clicked()
                                {
                                    self.sweep_type_idx = idx;
                                }
                            }
                        });
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Output Configuration
        ui.group(|ui| {
            ui.label(egui::RichText::new("Output Configuration").strong());
            ui.add_space(4.0);

            egui::Grid::new("pnoise_output_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Output Node:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.output_node)
                            .desired_width(120.0)
                            .hint_text("VOUT"),
                    );
                    ui.end_row();

                    ui.label("Reference:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.output_ref)
                            .desired_width(120.0)
                            .hint_text("(ground)"),
                    );
                    ui.end_row();

                    ui.label("Noise Type:");
                    let noise_types = ["Output-Referred", "Input-Referred", "Phase Noise"];
                    egui::ComboBox::from_id_salt("pnoise_ref")
                        .selected_text(noise_types[self.noise_ref_idx])
                        .show_ui(ui, |ui| {
                            for (idx, name) in noise_types.iter().enumerate() {
                                if ui
                                    .selectable_label(self.noise_ref_idx == idx, *name)
                                    .clicked()
                                {
                                    self.noise_ref_idx = idx;
                                }
                            }
                        });
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Sideband Configuration
        ui.group(|ui| {
            ui.label(egui::RichText::new("Sideband Folding").strong());
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Max Sideband:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.max_sideband)
                        .desired_width(60.0)
                        .hint_text("5"),
                );
                ui.label("(noise folding from ±N harmonics)");
            });
        });

        ui.add_space(8.0);

        // Options
        ui.group(|ui| {
            ui.label(egui::RichText::new("Options").strong());
            ui.add_space(4.0);

            ui.checkbox(&mut self.spot_noise, "Spot Noise (V/√Hz or dBc/Hz)");
            ui.checkbox(&mut self.integrated_noise, "Integrated Noise (Vrms)");
            ui.checkbox(&mut self.noise_summary, "Noise Summary (per-device)");
        });

        // Info footer
        if let Ok(config) = self.to_config() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!(
                    "~{} freq points | {} sideband folding",
                    config.total_points(),
                    config.max_sideband
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
    // PnoiseConfig Basic Tests
    // =========================================================================

    #[test]
    fn test_config_default() {
        let cfg = PnoiseConfig::default();
        assert_eq!(cfg.start_freq, 1.0);
        assert_eq!(cfg.stop_freq, 1e6);
        assert_eq!(cfg.max_sideband, 5);
        assert_eq!(cfg.output_node, "VOUT");
        assert!(cfg.spot_noise);
    }

    #[test]
    fn test_config_new() {
        let cfg = PnoiseConfig::new(10.0, 10e6, 20);
        assert_eq!(cfg.start_freq, 10.0);
        assert_eq!(cfg.stop_freq, 10e6);
        assert_eq!(cfg.num_points, 20);
    }

    #[test]
    fn test_config_phase_noise() {
        let cfg = PnoiseConfig::phase_noise(1e3, 10e6);
        assert_eq!(cfg.noise_ref, NoiseReferenceType::Phase);
        assert_eq!(cfg.start_freq, 1e3);
        assert_eq!(cfg.stop_freq, 10e6);
    }

    #[test]
    fn test_config_builder_chain() {
        let cfg = PnoiseConfig::new(1.0, 1e6, 10)
            .with_output("VCO_OUT")
            .with_noise_ref(NoiseReferenceType::Phase)
            .with_sidebands(10)
            .with_integrated_noise(true);

        assert_eq!(cfg.output_node, "VCO_OUT");
        assert_eq!(cfg.noise_ref, NoiseReferenceType::Phase);
        assert_eq!(cfg.max_sideband, 10);
        assert!(cfg.integrated_noise);
    }

    #[test]
    fn test_config_with_sweep_type() {
        let cfg = PnoiseConfig::default().with_sweep_type(PnoiseSweepType::Linear);
        assert_eq!(cfg.sweep_type, PnoiseSweepType::Linear);
    }

    #[test]
    fn test_config_with_fundamental() {
        let cfg = PnoiseConfig::default().with_fundamental(1e9);
        assert_eq!(cfg.fundamental_freq, 1e9);
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_valid() {
        let cfg = PnoiseConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_start() {
        let mut cfg = PnoiseConfig::default();
        cfg.start_freq = 0.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_negative_freq() {
        let mut cfg = PnoiseConfig::default();
        cfg.start_freq = -1.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_start_after_stop() {
        let mut cfg = PnoiseConfig::default();
        cfg.start_freq = 1e6;
        cfg.stop_freq = 1.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_points() {
        let mut cfg = PnoiseConfig::default();
        cfg.num_points = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_negative_sideband() {
        let mut cfg = PnoiseConfig::default();
        cfg.max_sideband = -1;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_empty_output() {
        let mut cfg = PnoiseConfig::default();
        cfg.output_node = String::new();
        assert!(cfg.validate().is_err());
    }

    // =========================================================================
    // SPICE Generation Tests
    // =========================================================================

    #[test]
    fn test_to_spice_basic() {
        let cfg = PnoiseConfig::default();
        let spice = cfg.to_spice();

        assert!(spice.starts_with(".pnoise"));
        assert!(spice.contains("dec"));
        assert!(spice.contains("output=VOUT"));
        assert!(spice.contains("maxsideband=5"));
    }

    #[test]
    fn test_to_spice_phase_noise() {
        let cfg = PnoiseConfig::phase_noise(1e3, 10e6);
        let spice = cfg.to_spice();

        assert!(spice.contains("noiseref=phase"));
    }

    #[test]
    fn test_to_spice_differential_output() {
        let mut cfg = PnoiseConfig::default();
        cfg.output_ref = "VSS".to_string();
        let spice = cfg.to_spice();

        assert!(spice.contains("output=(VOUT,VSS)"));
    }

    #[test]
    fn test_to_spice_integrated_noise() {
        let cfg = PnoiseConfig::default().with_integrated_noise(true);
        let spice = cfg.to_spice();

        assert!(spice.contains("integratedNoise=yes"));
    }

    #[test]
    fn test_to_spice_noise_summary() {
        let cfg = PnoiseConfig::default();
        let spice = cfg.to_spice();

        assert!(spice.contains("noiseSummary=yes"));
    }

    // =========================================================================
    // Total Points Tests
    // =========================================================================

    #[test]
    fn test_total_points_decade() {
        let cfg = PnoiseConfig::new(1.0, 1e6, 10);
        let points = cfg.total_points();
        // 6 decades * 10 points + 1 = 61
        assert!(points >= 60);
    }

    #[test]
    fn test_total_points_linear() {
        let cfg = PnoiseConfig::new(1.0, 1e6, 100).with_sweep_type(PnoiseSweepType::Linear);
        assert_eq!(cfg.total_points(), 100);
    }

    // =========================================================================
    // PnoiseSweepType Tests
    // =========================================================================

    #[test]
    fn test_sweep_type_names() {
        assert_eq!(PnoiseSweepType::Decade.display_name(), "Decade");
        assert_eq!(PnoiseSweepType::Octave.display_name(), "Octave");
        assert_eq!(PnoiseSweepType::Linear.display_name(), "Linear");
    }

    #[test]
    fn test_sweep_type_keywords() {
        assert_eq!(PnoiseSweepType::Decade.spice_keyword(), "dec");
        assert_eq!(PnoiseSweepType::Octave.spice_keyword(), "oct");
        assert_eq!(PnoiseSweepType::Linear.spice_keyword(), "lin");
    }

    #[test]
    fn test_sweep_type_all() {
        assert_eq!(PnoiseSweepType::all().len(), 3);
    }

    // =========================================================================
    // NoiseReferenceType Tests
    // =========================================================================

    #[test]
    fn test_noise_ref_names() {
        assert!(NoiseReferenceType::Output.display_name().contains("Output"));
        assert!(NoiseReferenceType::Input.display_name().contains("Input"));
        assert!(NoiseReferenceType::Phase.display_name().contains("Phase"));
    }

    #[test]
    fn test_noise_ref_keywords() {
        assert_eq!(NoiseReferenceType::Output.spice_keyword(), "output");
        assert_eq!(NoiseReferenceType::Input.spice_keyword(), "input");
        assert_eq!(NoiseReferenceType::Phase.spice_keyword(), "phase");
    }

    #[test]
    fn test_noise_ref_all() {
        assert_eq!(NoiseReferenceType::all().len(), 3);
    }

    // =========================================================================
    // Reset Test
    // =========================================================================

    #[test]
    fn test_config_reset() {
        let mut cfg = PnoiseConfig::phase_noise(1e3, 100e6).with_sidebands(20);
        cfg.reset();

        assert_eq!(cfg.max_sideband, 5);
        assert_eq!(cfg.noise_ref, NoiseReferenceType::Output);
    }

    // =========================================================================
    // Dialog State Tests
    // =========================================================================

    #[test]
    fn test_dialog_state_from_config() {
        let cfg = PnoiseConfig::phase_noise(1e3, 10e6);
        let state = PnoiseDialogState::from_config(&cfg);

        assert!(state.initialized);
        assert_eq!(state.noise_ref_idx, 2); // Phase
    }

    #[test]
    fn test_dialog_state_to_config() {
        let mut state = PnoiseDialogState::from_config(&PnoiseConfig::default());
        state.noise_ref_idx = 2; // Phase

        let result = state.to_config();
        assert!(result.is_ok());

        let cfg = result.unwrap();
        assert_eq!(cfg.noise_ref, NoiseReferenceType::Phase);
    }

    #[test]
    fn test_dialog_state_invalid_freq() {
        let mut state = PnoiseDialogState::from_config(&PnoiseConfig::default());
        state.start_freq = "invalid".to_string();

        let result = state.to_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_dialog_state_ensure_initialized() {
        let mut state = PnoiseDialogState::default();
        assert!(!state.initialized);

        state.ensure_initialized();
        assert!(state.initialized);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_vco_phase_noise_config() {
        // Typical VCO phase noise: 1 kHz to 100 MHz offset
        let cfg = PnoiseConfig::phase_noise(1e3, 100e6)
            .with_output("VCO_OUT")
            .with_sidebands(10);

        assert!(cfg.validate().is_ok());
        let spice = cfg.to_spice();
        assert!(spice.contains("noiseref=phase"));
    }

    #[test]
    fn test_lna_noise_config() {
        // LNA noise figure: 10 Hz to 10 GHz
        let cfg = PnoiseConfig::new(10.0, 10e9, 20)
            .with_output("VOUT")
            .with_noise_ref(NoiseReferenceType::Input);

        assert!(cfg.validate().is_ok());
        let spice = cfg.to_spice();
        assert!(spice.contains("noiseref=input"));
    }

    #[test]
    fn test_zero_sideband() {
        // No sideband folding
        let cfg = PnoiseConfig::default().with_sidebands(0);
        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.max_sideband, 0);
    }

    #[test]
    fn test_large_sideband() {
        // Many sidebands for accurate noise folding
        let cfg = PnoiseConfig::default().with_sidebands(50);
        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.max_sideband, 50);
    }
}

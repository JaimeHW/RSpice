//! S-Parameter Analysis Configuration
//!
//! Configuration for RF/microwave S-parameter analysis (.sp).
//! S-parameters describe the electrical behavior of linear networks
//! in terms of incident and reflected waves.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Linear, decade, and octave frequency sweeps
//! - Configurable reference impedance (Z0)
//! - Multi-port support with node specification
//! - Noise analysis option
//! - Touchstone export support
//!
//! # Example SPICE Output
//!
//! ```text
//! .sp dec 10 1Meg 10G
//! + port1=in port2=out z0=50
//! ```

use super::options::parse_si_value;
use egui::Ui;

// =============================================================================
// Frequency Sweep Type (shared with AC)
// =============================================================================

/// Type of frequency sweep for S-parameter analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpSweepType {
    /// Decades (logarithmic) - most common for RF
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl SpSweepType {
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
    pub fn all() -> &'static [SpSweepType] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

// =============================================================================
// Port Configuration
// =============================================================================

/// Configuration for a single port
#[derive(Debug, Clone)]
pub struct SpPortConfig {
    /// Port number (1-indexed)
    pub number: u32,
    /// Positive node name
    pub node_pos: String,
    /// Negative/reference node name (0 for ground)
    pub node_neg: String,
    /// Port-specific reference impedance (overrides global Z0)
    pub z0: Option<f64>,
}

impl SpPortConfig {
    /// Create single-ended port referenced to ground
    pub fn single_ended(number: u32, node: &str) -> Self {
        Self {
            number,
            node_pos: node.to_uppercase(),
            node_neg: "0".to_string(),
            z0: None,
        }
    }

    /// Create differential port
    pub fn differential(number: u32, node_pos: &str, node_neg: &str) -> Self {
        Self {
            number,
            node_pos: node_pos.to_uppercase(),
            node_neg: node_neg.to_uppercase(),
            z0: None,
        }
    }

    /// Set port-specific impedance
    pub fn with_z0(mut self, z0: f64) -> Self {
        self.z0 = Some(z0);
        self
    }

    /// Check if differential
    pub fn is_differential(&self) -> bool {
        self.node_neg != "0"
    }
}

impl Default for SpPortConfig {
    fn default() -> Self {
        Self::single_ended(1, "IN")
    }
}

// =============================================================================
// S-Parameter Configuration
// =============================================================================

/// S-parameter analysis configuration
///
/// Commercial-grade configuration matching Cadence Spectre SP parameters.
#[derive(Debug, Clone)]
pub struct SpConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points (per decade/octave for log, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: SpSweepType,
    /// Reference impedance (Ω) - default 50Ω
    pub z0: f64,
    /// Port definitions
    pub ports: Vec<SpPortConfig>,
    /// Include noise analysis
    pub do_noise: bool,
    /// Export Touchstone file
    pub touchstone_export: bool,
    /// Touchstone format version (1 or 2)
    pub touchstone_version: u32,
    /// Save all internal nodes
    pub save_all: bool,
}

impl Default for SpConfig {
    fn default() -> Self {
        Self {
            start_freq: 1e6, // 1 MHz
            stop_freq: 10e9, // 10 GHz
            num_points: 10,  // 10 points per decade
            sweep_type: SpSweepType::Decade,
            z0: 50.0, // Standard 50Ω
            ports: vec![
                SpPortConfig::single_ended(1, "IN"),
                SpPortConfig::single_ended(2, "OUT"),
            ],
            do_noise: false,
            touchstone_export: true,
            touchstone_version: 2,
            save_all: false,
        }
    }
}

impl SpConfig {
    /// Create new linear sweep config
    pub fn linear(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type: SpSweepType::Linear,
            ..Default::default()
        }
    }

    /// Create new decade sweep config
    pub fn decade(start: f64, stop: f64, points_per_decade: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points_per_decade,
            sweep_type: SpSweepType::Decade,
            ..Default::default()
        }
    }

    /// Set reference impedance
    pub fn with_z0(mut self, z0: f64) -> Self {
        self.z0 = z0;
        self
    }

    /// Set ports
    pub fn with_ports(mut self, ports: Vec<SpPortConfig>) -> Self {
        self.ports = ports;
        self
    }

    /// Add a port
    pub fn add_port(mut self, port: SpPortConfig) -> Self {
        self.ports.push(port);
        self
    }

    /// Enable noise analysis
    pub fn with_noise(mut self, enable: bool) -> Self {
        self.do_noise = enable;
        self
    }

    /// Enable Touchstone export
    pub fn with_touchstone(mut self, enable: bool, version: u32) -> Self {
        self.touchstone_export = enable;
        self.touchstone_version = version;
        self
    }

    /// Number of ports
    pub fn num_ports(&self) -> usize {
        self.ports.len()
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            SpSweepType::Decade => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            SpSweepType::Octave => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            SpSweepType::Linear => self.num_points,
        }
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".sp {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        // Reference impedance (if not 50Ω)
        if (self.z0 - 50.0).abs() > 0.01 {
            cmd.push_str(&format!(" z0={}", self.z0));
        }

        // Port definitions
        for port in &self.ports {
            if port.is_differential() {
                cmd.push_str(&format!(
                    " port{}=({},{})",
                    port.number, port.node_pos, port.node_neg
                ));
            } else {
                cmd.push_str(&format!(" port{}={}", port.number, port.node_pos));
            }
            if let Some(pz0) = port.z0 {
                cmd.push_str(&format!(" port{}z0={}", port.number, pz0));
            }
        }

        // Options
        if self.do_noise {
            cmd.push_str(" donoise=yes");
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

        if self.z0 <= 0.0 {
            return Err("Reference impedance Z0 must be positive".to_string());
        }

        if self.ports.is_empty() {
            return Err("At least one port must be defined".to_string());
        }

        if self.ports.len() < 2 {
            return Err("S-parameter analysis requires at least 2 ports".to_string());
        }

        // Validate port definitions
        for port in &self.ports {
            if port.node_pos.is_empty() {
                return Err(format!(
                    "Port {} positive node cannot be empty",
                    port.number
                ));
            }
            if let Some(pz0) = port.z0 {
                if pz0 <= 0.0 {
                    return Err(format!("Port {} impedance must be positive", port.number));
                }
            }
        }

        // Check for duplicate port numbers
        let mut port_nums: Vec<u32> = self.ports.iter().map(|p| p.number).collect();
        port_nums.sort();
        for i in 1..port_nums.len() {
            if port_nums[i] == port_nums[i - 1] {
                return Err(format!("Duplicate port number: {}", port_nums[i]));
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
#[derive(Debug, Clone, Default)]
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
    /// Port 1 node buffer
    pub port1_node: String,
    /// Port 2 node buffer
    pub port2_node: String,
    /// Port 1 is differential
    pub port1_differential: bool,
    /// Port 1 negative node (for differential)
    pub port1_neg: String,
    /// Port 2 is differential
    pub port2_differential: bool,
    /// Port 2 negative node (for differential)
    pub port2_neg: String,
    /// Enable noise analysis
    pub do_noise: bool,
    /// Enable Touchstone export
    pub touchstone_export: bool,
    /// Initialized flag
    pub initialized: bool,
}

impl SpDialogState {
    /// Initialize from config
    pub fn from_config(config: &SpConfig) -> Self {
        let port1 = config.ports.first().cloned().unwrap_or_default();
        let port2 = config
            .ports
            .get(1)
            .cloned()
            .unwrap_or_else(|| SpPortConfig::single_ended(2, "OUT"));

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
            port1_node: port1.node_pos.clone(),
            port2_node: port2.node_pos.clone(),
            port1_differential: port1.is_differential(),
            port1_neg: port1.node_neg.clone(),
            port2_differential: port2.is_differential(),
            port2_neg: port2.node_neg.clone(),
            do_noise: config.do_noise,
            touchstone_export: config.touchstone_export,
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

        // Build port 1
        let port1 = if self.port1_differential {
            SpPortConfig::differential(1, &self.port1_node, &self.port1_neg)
        } else {
            SpPortConfig::single_ended(1, &self.port1_node)
        };

        // Build port 2
        let port2 = if self.port2_differential {
            SpPortConfig::differential(2, &self.port2_node, &self.port2_neg)
        } else {
            SpPortConfig::single_ended(2, &self.port2_node)
        };

        let config = SpConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            z0,
            ports: vec![port1, port2],
            do_noise: self.do_noise,
            touchstone_export: self.touchstone_export,
            touchstone_version: 2,
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
    }

    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("S-Parameter Analysis");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("RF/microwave network characterization (S11, S21, S12, S22)")
                .weak(),
        );
        ui.add_space(12.0);

        // Frequency Range
        ui.group(|ui| {
            ui.label(egui::RichText::new("Frequency Range").strong());
            ui.add_space(4.0);

            egui::Grid::new("sp_freq_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Start:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.start_freq)
                            .desired_width(120.0)
                            .hint_text("1Meg"),
                    );
                    ui.end_row();

                    ui.label("Stop:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.stop_freq)
                            .desired_width(120.0)
                            .hint_text("10G"),
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
                    egui::ComboBox::from_id_salt("sp_sweep")
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

        // Reference Impedance
        ui.group(|ui| {
            ui.label(egui::RichText::new("Reference Impedance").strong());
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Z0:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.z0)
                        .desired_width(80.0)
                        .hint_text("50"),
                );
                ui.label("Ω");
            });
        });

        ui.add_space(8.0);

        // Port Configuration
        ui.group(|ui| {
            ui.label(egui::RichText::new("Port Configuration").strong());
            ui.add_space(4.0);

            // Port 1
            ui.horizontal(|ui| {
                ui.label("Port 1:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.port1_node)
                        .desired_width(80.0)
                        .hint_text("IN"),
                );
                ui.checkbox(&mut self.port1_differential, "Differential");
            });
            if self.port1_differential {
                ui.horizontal(|ui| {
                    ui.add_space(55.0);
                    ui.label("Neg:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.port1_neg)
                            .desired_width(80.0)
                            .hint_text("INM"),
                    );
                });
            }

            ui.add_space(4.0);

            // Port 2
            ui.horizontal(|ui| {
                ui.label("Port 2:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.port2_node)
                        .desired_width(80.0)
                        .hint_text("OUT"),
                );
                ui.checkbox(&mut self.port2_differential, "Differential");
            });
            if self.port2_differential {
                ui.horizontal(|ui| {
                    ui.add_space(55.0);
                    ui.label("Neg:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.port2_neg)
                            .desired_width(80.0)
                            .hint_text("OUTM"),
                    );
                });
            }
        });

        ui.add_space(8.0);

        // Options
        ui.group(|ui| {
            ui.label(egui::RichText::new("Options").strong());
            ui.add_space(4.0);

            ui.checkbox(&mut self.do_noise, "Include Noise Analysis (NF)");
            ui.checkbox(&mut self.touchstone_export, "Export Touchstone (.s2p)");
        });

        // Info footer
        if let Ok(config) = self.to_config() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!(
                    "{}-port | ~{} freq points",
                    config.num_ports(),
                    config.total_points()
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
    // SpConfig Basic Tests
    // =========================================================================

    #[test]
    fn test_config_default() {
        let cfg = SpConfig::default();
        assert_eq!(cfg.start_freq, 1e6);
        assert_eq!(cfg.stop_freq, 10e9);
        assert_eq!(cfg.z0, 50.0);
        assert_eq!(cfg.ports.len(), 2);
        assert!(!cfg.do_noise);
    }

    #[test]
    fn test_config_linear() {
        let cfg = SpConfig::linear(1e6, 1e9, 100);
        assert_eq!(cfg.sweep_type, SpSweepType::Linear);
        assert_eq!(cfg.num_points, 100);
    }

    #[test]
    fn test_config_decade() {
        let cfg = SpConfig::decade(1e6, 1e9, 20);
        assert_eq!(cfg.sweep_type, SpSweepType::Decade);
        assert_eq!(cfg.num_points, 20);
    }

    #[test]
    fn test_config_builder_chain() {
        let cfg = SpConfig::decade(1e6, 10e9, 10)
            .with_z0(75.0)
            .with_noise(true);

        assert_eq!(cfg.z0, 75.0);
        assert!(cfg.do_noise);
    }

    #[test]
    fn test_config_add_port() {
        let cfg = SpConfig::default().add_port(SpPortConfig::single_ended(3, "MID"));

        assert_eq!(cfg.ports.len(), 3);
        assert_eq!(cfg.ports[2].node_pos, "MID");
    }

    #[test]
    fn test_config_with_ports() {
        let ports = vec![
            SpPortConfig::single_ended(1, "RF"),
            SpPortConfig::single_ended(2, "IF"),
        ];
        let cfg = SpConfig::default().with_ports(ports);

        assert_eq!(cfg.ports.len(), 2);
        assert_eq!(cfg.ports[0].node_pos, "RF");
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_valid() {
        let cfg = SpConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_start_freq() {
        let mut cfg = SpConfig::default();
        cfg.start_freq = 0.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_negative_freq() {
        let mut cfg = SpConfig::default();
        cfg.start_freq = -1e6;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_start_after_stop() {
        let mut cfg = SpConfig::default();
        cfg.start_freq = 10e9;
        cfg.stop_freq = 1e6;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_points() {
        let mut cfg = SpConfig::default();
        cfg.num_points = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_z0() {
        let mut cfg = SpConfig::default();
        cfg.z0 = 0.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_negative_z0() {
        let mut cfg = SpConfig::default();
        cfg.z0 = -50.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_no_ports() {
        let mut cfg = SpConfig::default();
        cfg.ports.clear();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_one_port() {
        let mut cfg = SpConfig::default();
        cfg.ports = vec![SpPortConfig::single_ended(1, "IN")];
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_empty_port_node() {
        let mut cfg = SpConfig::default();
        cfg.ports[0].node_pos = String::new();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_duplicate_port_numbers() {
        let mut cfg = SpConfig::default();
        cfg.ports = vec![
            SpPortConfig::single_ended(1, "IN"),
            SpPortConfig::single_ended(1, "OUT"),
        ];
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_port_z0() {
        let mut cfg = SpConfig::default();
        cfg.ports[0].z0 = Some(-50.0);
        assert!(cfg.validate().is_err());
    }

    // =========================================================================
    // SPICE Generation Tests
    // =========================================================================

    #[test]
    fn test_to_spice_basic() {
        let cfg = SpConfig::decade(1e6, 10e9, 10);
        let spice = cfg.to_spice();

        assert!(spice.starts_with(".sp"));
        assert!(spice.contains("dec"));
        assert!(spice.contains("10"));
    }

    #[test]
    fn test_to_spice_custom_z0() {
        let cfg = SpConfig::default().with_z0(75.0);
        let spice = cfg.to_spice();

        assert!(spice.contains("z0=75"));
    }

    #[test]
    fn test_to_spice_ports() {
        let cfg = SpConfig::default();
        let spice = cfg.to_spice();

        assert!(spice.contains("port1=IN"));
        assert!(spice.contains("port2=OUT"));
    }

    #[test]
    fn test_to_spice_differential_port() {
        let ports = vec![
            SpPortConfig::differential(1, "INP", "INM"),
            SpPortConfig::single_ended(2, "OUT"),
        ];
        let cfg = SpConfig::default().with_ports(ports);
        let spice = cfg.to_spice();

        assert!(spice.contains("port1=(INP,INM)"));
    }

    #[test]
    fn test_to_spice_noise() {
        let cfg = SpConfig::default().with_noise(true);
        let spice = cfg.to_spice();

        assert!(spice.contains("donoise=yes"));
    }

    #[test]
    fn test_to_spice_port_z0() {
        let mut cfg = SpConfig::default();
        cfg.ports[0].z0 = Some(75.0);
        let spice = cfg.to_spice();

        assert!(spice.contains("port1z0=75"));
    }

    // =========================================================================
    // Total Points Tests
    // =========================================================================

    #[test]
    fn test_total_points_decade() {
        let cfg = SpConfig::decade(1e6, 1e9, 10);
        let points = cfg.total_points();
        // 3 decades * 10 points + 1 = 31
        assert!(points >= 30);
    }

    #[test]
    fn test_total_points_linear() {
        let cfg = SpConfig::linear(1e6, 1e9, 100);
        assert_eq!(cfg.total_points(), 100);
    }

    #[test]
    fn test_total_points_octave() {
        let mut cfg = SpConfig::default();
        cfg.sweep_type = SpSweepType::Octave;
        cfg.start_freq = 1e6;
        cfg.stop_freq = 8e6; // 3 octaves
        cfg.num_points = 5;

        let points = cfg.total_points();
        assert!(points >= 15);
    }

    // =========================================================================
    // SpSweepType Tests
    // =========================================================================

    #[test]
    fn test_sweep_type_display_names() {
        assert_eq!(SpSweepType::Decade.display_name(), "Decade");
        assert_eq!(SpSweepType::Octave.display_name(), "Octave");
        assert_eq!(SpSweepType::Linear.display_name(), "Linear");
    }

    #[test]
    fn test_sweep_type_spice_keywords() {
        assert_eq!(SpSweepType::Decade.spice_keyword(), "dec");
        assert_eq!(SpSweepType::Octave.spice_keyword(), "oct");
        assert_eq!(SpSweepType::Linear.spice_keyword(), "lin");
    }

    #[test]
    fn test_sweep_type_all() {
        assert_eq!(SpSweepType::all().len(), 3);
    }

    // =========================================================================
    // SpPortConfig Tests
    // =========================================================================

    #[test]
    fn test_port_single_ended() {
        let port = SpPortConfig::single_ended(1, "in");
        assert_eq!(port.number, 1);
        assert_eq!(port.node_pos, "IN");
        assert_eq!(port.node_neg, "0");
        assert!(!port.is_differential());
    }

    #[test]
    fn test_port_differential() {
        let port = SpPortConfig::differential(2, "outp", "outm");
        assert_eq!(port.number, 2);
        assert_eq!(port.node_pos, "OUTP");
        assert_eq!(port.node_neg, "OUTM");
        assert!(port.is_differential());
    }

    #[test]
    fn test_port_with_z0() {
        let port = SpPortConfig::single_ended(1, "in").with_z0(75.0);
        assert_eq!(port.z0, Some(75.0));
    }

    // =========================================================================
    // Reset Tests
    // =========================================================================

    #[test]
    fn test_config_reset() {
        let mut cfg = SpConfig::linear(1e3, 1e6, 50).with_z0(100.0);

        cfg.reset();

        assert_eq!(cfg.z0, 50.0);
        assert_eq!(cfg.sweep_type, SpSweepType::Decade);
    }

    // =========================================================================
    // Dialog State Tests
    // =========================================================================

    #[test]
    fn test_dialog_state_from_config() {
        let cfg = SpConfig::decade(1e6, 10e9, 20).with_z0(75.0);
        let state = SpDialogState::from_config(&cfg);

        assert!(state.initialized);
        assert_eq!(state.z0, "75");
        assert_eq!(state.sweep_type_idx, 0);
    }

    #[test]
    fn test_dialog_state_to_config() {
        let mut state = SpDialogState::from_config(&SpConfig::default());
        state.z0 = "100".to_string();

        let result = state.to_config();
        assert!(result.is_ok());

        let cfg = result.unwrap();
        assert_eq!(cfg.z0, 100.0);
    }

    #[test]
    fn test_dialog_state_invalid_freq() {
        let mut state = SpDialogState::from_config(&SpConfig::default());
        state.start_freq = "invalid".to_string();

        let result = state.to_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_dialog_state_differential_port() {
        let mut state = SpDialogState::from_config(&SpConfig::default());
        state.port1_differential = true;
        state.port1_neg = "INM".to_string();

        let result = state.to_config();
        assert!(result.is_ok());

        let cfg = result.unwrap();
        assert!(cfg.ports[0].is_differential());
    }

    #[test]
    fn test_dialog_state_ensure_initialized() {
        let mut state = SpDialogState::default();
        assert!(!state.initialized);

        state.ensure_initialized();
        assert!(state.initialized);
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_format_freq_giga() {
        assert_eq!(format_freq(1e9), "1G");
        assert_eq!(format_freq(10e9), "10G");
    }

    #[test]
    fn test_format_freq_mega() {
        assert_eq!(format_freq(1e6), "1Meg");
        assert_eq!(format_freq(100e6), "100Meg");
    }

    #[test]
    fn test_format_freq_kilo() {
        assert_eq!(format_freq(1e3), "1k");
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_three_port_network() {
        let ports = vec![
            SpPortConfig::single_ended(1, "IN"),
            SpPortConfig::single_ended(2, "OUT1"),
            SpPortConfig::single_ended(3, "OUT2"),
        ];
        let cfg = SpConfig::default().with_ports(ports);

        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.num_ports(), 3);
    }

    #[test]
    fn test_microwave_frequency_range() {
        let cfg = SpConfig::decade(1e9, 100e9, 10); // 1-100 GHz
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_touchstone_settings() {
        let cfg = SpConfig::default().with_touchstone(true, 2);
        assert!(cfg.touchstone_export);
        assert_eq!(cfg.touchstone_version, 2);
    }

    #[test]
    fn test_z0_75_ohm() {
        // Common for video/cable applications
        let cfg = SpConfig::default().with_z0(75.0);
        assert!(cfg.validate().is_ok());
        assert!(cfg.to_spice().contains("z0=75"));
    }

    #[test]
    fn test_z0_600_ohm() {
        // Common for audio/telephony
        let cfg = SpConfig::default().with_z0(600.0);
        assert!(cfg.validate().is_ok());
    }
}

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
            if let Some(pz0) = port.z0
                && pz0 <= 0.0
            {
                return Err(format!("Port {} impedance must be positive", port.number));
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
    fn single_ended(node_pos: impl Into<String>) -> Self {
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
    /// Editable port definitions
    pub ports: Vec<SpPortDialogState>,
    /// Enable noise analysis
    pub do_noise: bool,
    /// Enable Touchstone export
    pub touchstone_export: bool,
    /// Touchstone format version (1 or 2).
    pub touchstone_version: u32,
    /// Initialized flag
    pub initialized: bool,
}

impl SpDialogState {
    fn default_port_node(index: usize) -> String {
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

    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("S-Parameter Analysis");
        ui.add_space(4.0);
        ui.label(egui::RichText::new("RF/microwave network characterization (Sij matrix)").weak());
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

            let mut remove_index = None;
            for (idx, port) in self.ports.iter_mut().enumerate() {
                let hint = Self::default_port_node(idx);
                ui.horizontal(|ui| {
                    ui.label(format!("Port {}:", idx + 1));
                    ui.add(
                        egui::TextEdit::singleline(&mut port.node_pos)
                            .desired_width(110.0)
                            .hint_text(hint.as_str()),
                    );
                    ui.checkbox(&mut port.differential, "Differential");
                    ui.checkbox(&mut port.z0_override, "Z0");
                    if port.z0_override {
                        ui.add(
                            egui::TextEdit::singleline(&mut port.z0)
                                .desired_width(70.0)
                                .hint_text("50"),
                        );
                        ui.label("ohm");
                    }
                    if idx >= 2 && ui.small_button("Remove").clicked() {
                        remove_index = Some(idx);
                    }
                });
                if port.differential {
                    ui.horizontal(|ui| {
                        ui.add_space(55.0);
                        ui.label("Neg:");
                        ui.add(
                            egui::TextEdit::singleline(&mut port.node_neg)
                                .desired_width(110.0)
                                .hint_text("0"),
                        );
                    });
                }
                ui.add_space(4.0);
            }
            if let Some(idx) = remove_index {
                self.ports.remove(idx);
            }

            if ui.button("+ Add Port").clicked() {
                let node = Self::default_port_node(self.ports.len());
                self.ports.push(SpPortDialogState::single_ended(node));
            }
        });

        ui.add_space(8.0);

        // Options
        ui.group(|ui| {
            ui.label(egui::RichText::new("Options").strong());
            ui.add_space(4.0);

            ui.checkbox(&mut self.do_noise, "Include Noise Analysis (NF)");
            ui.checkbox(&mut self.touchstone_export, "Export Touchstone (.sNp)");
            if self.touchstone_export {
                ui.horizontal(|ui| {
                    ui.label("Touchstone version:");
                    egui::ComboBox::from_id_salt("sp_touchstone_version")
                        .selected_text(format!("v{}", self.touchstone_version.clamp(1, 2)))
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(self.touchstone_version == 1, "v1")
                                .clicked()
                            {
                                self.touchstone_version = 1;
                            }
                            if ui
                                .selectable_label(self.touchstone_version != 1, "v2")
                                .clicked()
                            {
                                self.touchstone_version = 2;
                            }
                        });
                });
            }
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


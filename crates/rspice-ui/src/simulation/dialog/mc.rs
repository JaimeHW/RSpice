//! Monte Carlo Analysis Configuration
//!
//! Configuration for Monte Carlo statistical analysis.
//! Runs multiple iterations with random parameter variations.

use egui::Ui;

/// Random distribution type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum McDistribution {
    #[default]
    Gaussian,
    Uniform,
    WorstCase,
}

impl McDistribution {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Gaussian => "Gaussian",
            Self::Uniform => "Uniform",
            Self::WorstCase => "Worst Case",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Gaussian => "gauss",
            Self::Uniform => "uniform",
            Self::WorstCase => "worstcase",
        }
    }
    pub fn all() -> &'static [McDistribution] {
        &[Self::Gaussian, Self::Uniform, Self::WorstCase]
    }
}

/// Base analysis to run for MC
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum McBaseAnalysis {
    #[default]
    Transient,
    Ac,
    Dc,
    Op,
}

impl McBaseAnalysis {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Transient => "Transient",
            Self::Ac => "AC",
            Self::Dc => "DC Sweep",
            Self::Op => "DC OP",
        }
    }
    pub fn all() -> &'static [McBaseAnalysis] {
        &[Self::Transient, Self::Ac, Self::Dc, Self::Op]
    }
}

/// Monte Carlo analysis configuration
#[derive(Debug, Clone)]
pub struct McConfig {
    /// Number of runs
    pub num_runs: u32,
    /// Random seed (0 = auto)
    pub seed: u32,
    /// Distribution type
    pub distribution: McDistribution,
    /// Base analysis type
    pub base_analysis: McBaseAnalysis,
    /// Variation percentage (sigma for Gaussian, ± for Uniform)
    pub variation_pct: f64,
    /// Include process variations
    pub process_variations: bool,
    /// Include mismatch variations  
    pub mismatch_variations: bool,
    /// Save each run
    pub save_all_runs: bool,
    /// Compute statistics
    pub compute_stats: bool,
}

impl Default for McConfig {
    fn default() -> Self {
        Self {
            num_runs: 100,
            seed: 0,
            distribution: McDistribution::Gaussian,
            base_analysis: McBaseAnalysis::Transient,
            variation_pct: 5.0,
            process_variations: true,
            mismatch_variations: true,
            save_all_runs: false,
            compute_stats: true,
        }
    }
}

impl McConfig {
    pub fn new(runs: u32) -> Self {
        Self {
            num_runs: runs,
            ..Default::default()
        }
    }
    pub fn with_distribution(mut self, d: McDistribution) -> Self {
        self.distribution = d;
        self
    }
    pub fn with_base(mut self, b: McBaseAnalysis) -> Self {
        self.base_analysis = b;
        self
    }
    pub fn with_seed(mut self, s: u32) -> Self {
        self.seed = s;
        self
    }

    pub fn to_spice(&self) -> String {
        let dist = match self.distribution {
            McDistribution::Gaussian => "GAUSS",
            McDistribution::Uniform => "UNIFORM",
            McDistribution::WorstCase => "WORSTCASE",
        };
        let spread = (self.variation_pct / 100.0).abs();
        let mut cmd = format!(".mc {} DIST {} SPREAD {:.12e}", self.num_runs, dist, spread);
        if self.seed > 0 {
            cmd.push_str(&format!(" SEED {}", self.seed));
        }
        cmd
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.num_runs == 0 {
            return Err("Number of runs must be at least 1".into());
        }
        if self.variation_pct <= 0.0 {
            return Err("Variation percentage must be positive".into());
        }
        if self.variation_pct > 100.0 {
            return Err("Variation percentage cannot exceed 100%".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct McDialogState {
    pub num_runs: String,
    pub seed: String,
    pub distribution_idx: usize,
    pub base_idx: usize,
    pub variation_pct: String,
    pub process_variations: bool,
    pub mismatch_variations: bool,
    pub save_all_runs: bool,
    pub initialized: bool,
}

impl McDialogState {
    pub fn from_config(config: &McConfig) -> Self {
        Self {
            num_runs: config.num_runs.to_string(),
            seed: config.seed.to_string(),
            distribution_idx: match config.distribution {
                McDistribution::Gaussian => 0,
                McDistribution::Uniform => 1,
                McDistribution::WorstCase => 2,
            },
            base_idx: match config.base_analysis {
                McBaseAnalysis::Transient => 0,
                McBaseAnalysis::Ac => 1,
                McBaseAnalysis::Dc => 2,
                McBaseAnalysis::Op => 3,
            },
            variation_pct: format!("{}", config.variation_pct),
            process_variations: config.process_variations,
            mismatch_variations: config.mismatch_variations,
            save_all_runs: config.save_all_runs,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<McConfig, String> {
        let runs: u32 = self.num_runs.parse().map_err(|_| "Invalid runs")?;
        let seed: u32 = self.seed.parse().unwrap_or(0);
        let pct: f64 = self
            .variation_pct
            .parse()
            .map_err(|_| "Invalid variation")?;
        let dist = match self.distribution_idx {
            0 => McDistribution::Gaussian,
            1 => McDistribution::Uniform,
            _ => McDistribution::WorstCase,
        };
        let base = match self.base_idx {
            0 => McBaseAnalysis::Transient,
            1 => McBaseAnalysis::Ac,
            2 => McBaseAnalysis::Dc,
            _ => McBaseAnalysis::Op,
        };
        let config = McConfig {
            num_runs: runs,
            seed,
            distribution: dist,
            base_analysis: base,
            variation_pct: pct,
            process_variations: self.process_variations,
            mismatch_variations: self.mismatch_variations,
            save_all_runs: self.save_all_runs,
            compute_stats: true,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&McConfig::default());
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();
        ui.heading("Monte Carlo Analysis");
        ui.label(
            egui::RichText::new("Statistical analysis with random parameter variations").weak(),
        );
        ui.add_space(8.0);

        egui::Grid::new("mc_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label("Number of Runs:");
                ui.add(egui::TextEdit::singleline(&mut self.num_runs).desired_width(80.0));
                ui.end_row();
                ui.label("Random Seed:");
                ui.add(egui::TextEdit::singleline(&mut self.seed).desired_width(80.0));
                ui.end_row();
                ui.label("Variation (%):");
                ui.add(egui::TextEdit::singleline(&mut self.variation_pct).desired_width(60.0));
                ui.end_row();
            });
        ui.add_space(8.0);
        ui.checkbox(&mut self.process_variations, "Include Process Variations");
        ui.checkbox(&mut self.mismatch_variations, "Include Mismatch Variations");
        ui.checkbox(&mut self.save_all_runs, "Save All Run Data");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let c = McConfig::default();
        assert_eq!(c.num_runs, 100);
    }
    #[test]
    fn test_new() {
        let c = McConfig::new(500);
        assert_eq!(c.num_runs, 500);
    }
    #[test]
    fn test_with_dist() {
        let c = McConfig::default().with_distribution(McDistribution::Uniform);
        assert_eq!(c.distribution, McDistribution::Uniform);
    }
    #[test]
    fn test_with_base() {
        let c = McConfig::default().with_base(McBaseAnalysis::Ac);
        assert_eq!(c.base_analysis, McBaseAnalysis::Ac);
    }
    #[test]
    fn test_with_seed() {
        let c = McConfig::default().with_seed(12345);
        assert_eq!(c.seed, 12345);
    }
    #[test]
    fn test_validate_ok() {
        assert!(McConfig::default().validate().is_ok());
    }
    #[test]
    fn test_validate_zero_runs() {
        let mut c = McConfig::default();
        c.num_runs = 0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_zero_var() {
        let mut c = McConfig::default();
        c.variation_pct = 0.0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_big_var() {
        let mut c = McConfig::default();
        c.variation_pct = 200.0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_to_spice() {
        let s = McConfig::default().to_spice();
        assert!(s.contains(".mc"));
    }
    #[test]
    fn test_to_spice_seed() {
        let s = McConfig::default().with_seed(99).to_spice();
        assert!(s.contains("SEED 99"));
    }
    #[test]
    fn test_to_spice_worst_case_distribution() {
        let s = McConfig::default()
            .with_distribution(McDistribution::WorstCase)
            .to_spice();
        assert!(s.contains("DIST WORSTCASE"));
    }
    #[test]
    fn test_dist_all() {
        assert_eq!(McDistribution::all().len(), 3);
    }
    #[test]
    fn test_base_all() {
        assert_eq!(McBaseAnalysis::all().len(), 4);
    }
    #[test]
    fn test_dialog_roundtrip() {
        let s = McDialogState::from_config(&McConfig::default());
        assert!(s.to_config().is_ok());
    }
    #[test]
    fn test_reset() {
        let mut c = McConfig::new(999);
        c.reset();
        assert_eq!(c.num_runs, 100);
    }
}

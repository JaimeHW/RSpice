use super::format::format_freq;
use serde::{Deserialize, Serialize};

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

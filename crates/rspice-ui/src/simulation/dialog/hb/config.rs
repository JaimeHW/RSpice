//! Harmonic balance solver configuration.

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
}

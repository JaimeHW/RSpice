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
    /// The `.HB` directive this configuration writes into a deck.
    ///
    /// A `.HB` card names the fundamental tones and nothing else. That is the
    /// whole of the directive in the one dialect that spells it — Xyce, whose
    /// `.HB <f1> [<f2> ...]` takes frequencies and reads the harmonic order
    /// from `.OPTIONS HBINT NUMFREQ` — and it is exactly what this engine's
    /// parser accepts (`netlist::parser::commands`, the `".HB"` arm). ngspice
    /// has no `.HB` at all, so there is no second spelling to honour.
    ///
    /// Everything else the dialog configures — harmonic counts, oversampling,
    /// tolerances, the iteration budget, the solver choice — reaches the engine
    /// on the typed channel instead, and the deck is not a second owner of it:
    /// `build_harmonic_balance_spec` copies this config into
    /// `AnalysisSpec::HarmonicBalance`, `runner::spec::periodic` copies that
    /// into `svc_runner::HbRunConfig`, and `build_core_hb_config` copies that
    /// into `rspice_core::analysis::HbConfig`. Writing those values here as
    /// well would put two authorities in the same deck, and — because one
    /// executable netlist carries every task's directive — a second HB instance
    /// would silently overwrite the first.
    pub fn to_spice(&self) -> String {
        let mut cmd = String::from(".hb");
        for frequency in std::iter::once(self.fundamental_freq)
            .chain(self.additional_tones.iter().map(|tone| tone.frequency))
        {
            cmd.push(' ');
            cmd.push_str(&format_freq(frequency));
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse the directive the way a run does: spliced into a deck and read
    /// back by the engine's own parser. Asserting on the emitted string alone
    /// is what let `.hb 1G harmonics=9 oversample=2` ship — every character of
    /// it looked plausible, and no HB plan could be prepared.
    fn parse_through_the_deck(config: &HbConfig) -> rspice_core::Netlist {
        let deck = format!(
            "hb round trip\nV1 in 0 SIN(0 1 1G)\nR1 in out 1k\nC1 out 0 1p\n{}\n.end\n",
            config.to_spice()
        );
        rspice_core::netlist::parse_netlist(&deck)
            .unwrap_or_else(|error| panic!("the emitted deck must parse: {error}\n{deck}"))
    }

    fn parsed_frequencies(config: &HbConfig) -> Vec<f64> {
        let netlist = parse_through_the_deck(config);
        let [rspice_core::netlist::AnalysisCommand::Hb { frequencies }] =
            netlist.analyses.as_slice()
        else {
            panic!("the deck must carry exactly one .HB command: {:?}", netlist.analyses);
        };
        frequencies.clone()
    }

    #[test]
    fn the_default_directive_parses_to_its_own_fundamental() {
        let config = HbConfig::default();
        assert_eq!(config.to_spice(), ".hb 1G");
        assert_eq!(parsed_frequencies(&config), vec![config.fundamental_freq]);
    }

    #[test]
    fn every_tone_frequency_reaches_the_parsed_directive() {
        // The old emitter spelled extra tones `tone2=<f> tone2harm=<n>`, which
        // no dialect reads. Frequencies are what a `.HB` card carries, and a
        // multi-tone request carries all of them.
        let config = HbConfig {
            fundamental_freq: 2.0e9,
            additional_tones: vec![
                HbToneConfig::new(1.5e6, 3).with_name("tone2"),
                HbToneConfig::new(900.0, 2).with_name("tone3"),
            ],
            ..HbConfig::default()
        };

        assert_eq!(config.to_spice(), ".hb 2G 1.5Meg 900");
        assert_eq!(parsed_frequencies(&config), vec![2.0e9, 1.5e6, 900.0]);
    }

    #[test]
    fn solver_settings_stay_off_the_directive_and_travel_typed() {
        // Harmonic counts, oversampling, tolerances and the solver choice reach
        // the engine through `AnalysisSpec::HarmonicBalance` ->
        // `svc_runner::HbRunConfig` -> `build_core_hb_config`. Restating them
        // on the card would make the deck a second authority, and the parser
        // would refuse the card outright.
        let config = HbConfig {
            num_harmonics: 15,
            oversample: 8,
            reltol: 1.0e-9,
            maxiter: 250,
            solver: HbSolverType::Krylov,
            gmres_restart: 64,
            source_stepping: true,
            ..HbConfig::default()
        };

        let directive = config.to_spice();
        for key in [
            "harmonics",
            "oversample",
            "reltol",
            "maxiter",
            "solver",
            "gmres_restart",
            "sourcestepping",
        ] {
            assert!(
                !directive.contains(key),
                "the .HB card must not state {key}: {directive}"
            );
        }
        parse_through_the_deck(&config);
    }
}

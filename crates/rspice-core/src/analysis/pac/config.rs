//! PAC Analysis Configuration
//!
//! Provides configuration types for Periodic AC analysis including:
//! - Frequency sweep parameters (linear, decade, octave)
//! - Sideband range selection
//! - Accuracy and convergence controls

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::frequency_grid::{
    FrequencyGridError, FrequencyGridScale, frequency_point_count, generate_frequency_grid,
    validate_generated_sweep,
};

//=============================================================================
// Frequency Sweep Type
//=============================================================================

/// Type of frequency sweep for PAC analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacSweepType {
    /// Linear frequency sweep
    Linear,
    /// Decade (logarithmic) frequency sweep
    #[default]
    Decade,
    /// Octave frequency sweep
    Octave,
}

//=============================================================================
// PAC Configuration
//=============================================================================

/// Configuration for Periodic AC (PAC) analysis
///
/// PAC analysis performs small-signal AC analysis around a periodic steady-state
/// operating point. The result is a conversion matrix that relates input signals
/// at one frequency/sideband to output signals at other frequencies/sidebands.
///
/// # Example
///
/// ```ignore
/// let config = PacConfig::new()
///     .with_sweep(1e6, 1e9, 100)     // 1 MHz to 1 GHz, 100 points
///     .with_sidebands(-5, 5)          // Harmonics -5 to +5
///     .with_input_source("VRF")       // RF input source
///     .with_output_node("VOUT");      // Output node
/// ```
#[derive(Debug, Clone)]
pub struct PacConfig {
    /// Start frequency for input sweep (Hz)
    pub sweep_start: Value,

    /// Stop frequency for input sweep (Hz)
    pub sweep_stop: Value,

    /// Number of frequency points (or points per decade for log sweeps)
    pub num_points: usize,

    /// Sweep type (linear, decade, octave)
    pub sweep_type: PacSweepType,

    /// Minimum output sideband index relative to input (e.g., -5 for LO - 5*f₀)
    pub sideband_min: i32,

    /// Maximum output sideband index relative to input (e.g., +5 for LO + 5*f₀)  
    pub sideband_max: i32,

    /// Name of the input source (voltage or current source providing small-signal)
    pub input_source: Option<String>,

    /// Output node name for primary result extraction
    pub output_node: Option<String>,

    /// Reference node name (default: ground = "0")
    pub output_ref: Option<String>,

    /// Relative tolerance for frequency-domain solution
    pub reltol: Value,

    /// Absolute tolerance for small-signal currents (A)
    pub abstol: Value,

    /// Publish the sideband-zero spectra. The lifted system always spans
    /// `sideband_min..=sideband_max`, because dropping a sideband from it
    /// would change the answer at the sidebands that remain; this selects
    /// what the result reports, and travels on [`PacResult::include_dc`].
    pub include_dc: bool,

    /// Amplitude of the small-signal drive applied at `input_source`, in the
    /// source's own unit. The solve itself uses a unit excitation so the
    /// conversion matrix stays a transfer function; the amplitude travels on
    /// [`PacResult::pac_magnitude`] and scales the published responses.
    pub pac_magnitude: Value,

    /// Fundamental frequency from PSS (will be set from PSS result)
    pub fundamental_freq: Value,
}

impl Default for PacConfig {
    fn default() -> Self {
        Self {
            sweep_start: 1e3, // 1 kHz default start
            sweep_stop: 1e9,  // 1 GHz default stop
            num_points: 50,   // 50 points default
            sweep_type: PacSweepType::Decade,
            sideband_min: -5, // 5 sidebands below
            sideband_max: 5,  // 5 sidebands above
            input_source: None,
            output_node: None,
            output_ref: None,
            reltol: 1e-3,  // 0.1% relative tolerance
            abstol: 1e-12, // 1 pA absolute tolerance
            include_dc: true,
            pac_magnitude: 1.0,    // Unit drive
            fundamental_freq: 0.0, // Set from PSS
        }
    }
}

impl PacConfig {
    /// Create a new PAC configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set frequency sweep parameters
    ///
    /// # Arguments
    /// * `start` - Start frequency in Hz
    /// * `stop` - Stop frequency in Hz  
    /// * `points` - Number of frequency points (total for linear, per decade for log)
    pub fn with_sweep(mut self, start: Value, stop: Value, points: usize) -> Self {
        self.sweep_start = start;
        self.sweep_stop = stop;
        self.num_points = points;
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: PacSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Set sideband range for output
    ///
    /// # Arguments
    /// * `min` - Minimum sideband index (negative for lower sidebands)
    /// * `max` - Maximum sideband index (positive for upper sidebands)
    ///
    /// # Example
    /// ```ignore
    /// // For a mixer: input at RF (sideband +1), output at IF (sideband 0)
    /// config.with_sidebands(-3, 3);  // Analyze sidebands -3 to +3
    /// ```
    pub fn with_sidebands(mut self, min: i32, max: i32) -> Self {
        self.sideband_min = min;
        self.sideband_max = max;
        self
    }

    /// Set the input source name
    ///
    /// The input source should be a small-signal source (AC magnitude)
    /// that will be swept across the frequency range.
    pub fn with_input_source(mut self, source_name: &str) -> Self {
        self.input_source = Some(source_name.to_uppercase());
        self
    }

    /// Set the output node for primary result extraction
    pub fn with_output_node(mut self, node_name: &str) -> Self {
        self.output_node = Some(node_name.to_uppercase());
        self
    }

    /// Set the reference node (default is ground)
    pub fn with_output_ref(mut self, ref_name: &str) -> Self {
        self.output_ref = Some(ref_name.to_uppercase());
        self
    }

    /// Set convergence tolerances
    pub fn with_tolerances(mut self, reltol: Value, abstol: Value) -> Self {
        self.reltol = reltol;
        self.abstol = abstol;
        self
    }

    /// Publish or withhold the sideband-zero spectra.
    pub fn with_dc(mut self, include: bool) -> Self {
        self.include_dc = include;
        self
    }

    /// Set fundamental frequency (typically from PSS result)
    pub fn with_fundamental(mut self, freq: Value) -> Self {
        self.fundamental_freq = freq;
        self
    }

    /// Generate frequency points while preserving validation and resource failures.
    pub fn frequency_points(&self) -> Result<Vec<Value>, FrequencyGridError> {
        self.try_frequency_points()
    }

    /// Generate frequency points without a cancellation source.
    pub fn try_frequency_points(&self) -> Result<Vec<Value>, FrequencyGridError> {
        self.try_frequency_points_with_abort(&NoAbort)
    }

    /// Generate frequency points with cooperative cancellation.
    pub(crate) fn try_frequency_points_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, FrequencyGridError> {
        generate_frequency_grid(
            self.sweep_start,
            self.sweep_stop,
            self.num_points,
            self.grid_scale(),
            false,
            1,
            abort,
        )
    }

    /// Number of points the configured sweep will generate, without
    /// allocating the frequency vector.
    pub fn frequency_point_count(&self) -> Result<usize, FrequencyGridError> {
        self.validate_frequency_sweep()?;
        frequency_point_count(
            self.sweep_start,
            self.sweep_stop,
            self.num_points,
            self.grid_scale(),
            1,
        )
    }

    /// Get the number of sidebands being analyzed
    pub fn num_sidebands(&self) -> usize {
        usize::try_from(i64::from(self.sideband_max) - i64::from(self.sideband_min) + 1)
            .unwrap_or(usize::MAX)
    }

    /// Get sideband indices as a vector
    pub fn sideband_indices(&self) -> Vec<i32> {
        (self.sideband_min..=self.sideband_max).collect()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        self.validate_frequency_sweep()
            .map_err(|error| error.to_string())?;
        if self.sideband_min > self.sideband_max {
            return Err("Sideband min must be <= sideband max".to_string());
        }
        if !self.reltol.is_finite()
            || !self.abstol.is_finite()
            || self.reltol <= 0.0
            || self.abstol <= 0.0
        {
            return Err("Tolerances must be positive and finite".to_string());
        }
        if !self.pac_magnitude.is_finite() || self.pac_magnitude <= 0.0 {
            return Err("PAC drive amplitude must be positive and finite".to_string());
        }
        if !self.include_dc && self.sideband_min == 0 && self.sideband_max == 0 {
            return Err(
                "PAC withholds sideband zero and analyses no other sideband, so it would \
                 publish nothing"
                    .to_string(),
            );
        }
        Ok(())
    }

    fn validate_frequency_sweep(&self) -> Result<(), FrequencyGridError> {
        validate_generated_sweep(
            self.sweep_start,
            self.sweep_stop,
            self.num_points,
            self.grid_scale(),
            false,
        )
    }

    fn grid_scale(&self) -> FrequencyGridScale {
        match self.sweep_type {
            PacSweepType::Linear => FrequencyGridScale::Linear,
            PacSweepType::Decade => FrequencyGridScale::Decade,
            PacSweepType::Octave => FrequencyGridScale::Octave,
        }
    }
}

impl From<&crate::netlist::PacCard> for PacConfig {
    /// Convert an authored `.PAC` card into the periodic-AC configuration.
    ///
    /// The conversion lives here rather than in the parser because a parsed
    /// deck sits below the analyses and may not name them. `fundamental_freq`
    /// stays zero: the runner binds it from the upstream `.PSS`/`.HB`
    /// operating point, exactly as the direct entry points do.
    fn from(card: &crate::netlist::PacCard) -> Self {
        Self {
            sweep_start: card.sweep.start_freq,
            sweep_stop: card.sweep.stop_freq,
            num_points: card.sweep.points,
            sweep_type: match card.sweep.variation {
                crate::netlist::FreqVariation::Lin => PacSweepType::Linear,
                crate::netlist::FreqVariation::Dec => PacSweepType::Decade,
                crate::netlist::FreqVariation::Oct => PacSweepType::Octave,
            },
            sideband_min: card.sideband_min,
            sideband_max: card.sideband_max,
            input_source: Some(card.input_source.to_uppercase()),
            output_node: Some(card.output_node.to_uppercase()),
            output_ref: card.output_ref.as_ref().map(|node| node.to_uppercase()),
            reltol: card.reltol,
            abstol: card.abstol,
            include_dc: card.include_dc,
            pac_magnitude: card.pac_magnitude,
            fundamental_freq: 0.0,
        }
    }
}

#[cfg(test)]
mod card_conversion_tests {
    use super::*;
    use crate::netlist::{AnalysisCommand, Netlist, PacCard};

    const CIRCUIT: &str = "pac card conversion\n\
                           V1 in 0 SIN(0 1 1G)\n\
                           R1 in out 1k\n\
                           C1 out 0 1p\n";

    fn card(cards: &str) -> PacCard {
        let netlist =
            Netlist::parse(&format!("{CIRCUIT}.HB 1G\n{cards}\n.end\n")).expect("PAC deck parses");
        match netlist.analyses.into_iter().next_back() {
            Some(AnalysisCommand::Pac(card)) => *card,
            other => panic!("expected a .PAC card, got {other:?}"),
        }
    }

    #[test]
    fn the_card_defaults_are_the_configuration_defaults() {
        // The netlist layer cannot read these constants from here, so this is
        // what keeps the two copies from drifting apart.
        let defaults = PacConfig::default();
        assert_eq!(PacCard::DEFAULT_SIDEBAND_MIN, defaults.sideband_min);
        assert_eq!(PacCard::DEFAULT_SIDEBAND_MAX, defaults.sideband_max);
        assert_eq!(PacCard::DEFAULT_RELTOL, defaults.reltol);
        assert_eq!(PacCard::DEFAULT_ABSTOL, defaults.abstol);
        assert_eq!(PacCard::DEFAULT_PAC_MAGNITUDE, defaults.pac_magnitude);
        assert_eq!(PacCard::DEFAULT_INCLUDE_DC, defaults.include_dc);
    }

    #[test]
    fn a_minimal_card_converts_to_the_direct_defaults() {
        let converted = PacConfig::from(&card(".PAC DEC 10 1k 1G INPUT=VRF OUT=V(out)"));
        let defaults = PacConfig::default();
        assert_eq!(converted.sweep_start, 1.0e3);
        assert_eq!(converted.sweep_stop, 1.0e9);
        assert_eq!(converted.num_points, 10);
        assert_eq!(converted.sweep_type, PacSweepType::Decade);
        assert_eq!(converted.sideband_min, defaults.sideband_min);
        assert_eq!(converted.sideband_max, defaults.sideband_max);
        assert_eq!(converted.reltol, defaults.reltol);
        assert_eq!(converted.abstol, defaults.abstol);
        assert_eq!(converted.include_dc, defaults.include_dc);
        assert_eq!(converted.pac_magnitude, defaults.pac_magnitude);
        assert_eq!(converted.input_source.as_deref(), Some("VRF"));
        assert_eq!(converted.output_node.as_deref(), Some("OUT"));
        assert_eq!(converted.output_ref, None);
        // Bound from the upstream operating point, never from the card.
        assert_eq!(converted.fundamental_freq, 0.0);
        assert!(converted.validate().is_ok());
    }

    #[test]
    fn every_authored_field_survives_the_conversion() {
        let converted = PacConfig::from(&card(
            ".pac lin 21 1meg 5meg input=vrf out=v(out,ref) sidebandmin=-3 \
             sidebandmax=7 reltol=1e-5 abstol=1e-15 pacmag=0.25 includedc=no from=hb",
        ));
        assert_eq!(converted.pac_magnitude, 0.25);
        assert!(!converted.include_dc);
        assert_eq!(converted.sweep_type, PacSweepType::Linear);
        assert_eq!(converted.num_points, 21);
        assert_eq!(converted.sweep_start, 1.0e6);
        assert_eq!(converted.sweep_stop, 5.0e6);
        assert_eq!(converted.sideband_min, -3);
        assert_eq!(converted.sideband_max, 7);
        assert_eq!(converted.reltol, 1e-5);
        assert_eq!(converted.abstol, 1e-15);
        assert_eq!(converted.output_node.as_deref(), Some("OUT"));
        assert_eq!(converted.output_ref.as_deref(), Some("REF"));
        assert!(converted.validate().is_ok());
    }

    #[test]
    fn a_card_the_parser_accepted_always_passes_the_configuration_validator() {
        for source in [
            ".PAC DEC 10 1k 1G INPUT=VRF OUT=out",
            ".PAC OCT 4 1k 1meg INPUT=VRF OUT=out MAXSIDEBAND=3",
            ".PAC LIN 2 1k 1k INPUT=VRF OUT=out SIDEBANDMIN=0 SIDEBANDMAX=0",
            ".PAC DEC 1 1 1G INPUT=VRF OUT=out RELTOL=1e-9 ABSTOL=1e-18",
            ".PAC DEC 10 1k 1G INPUT=VRF OUT=out PACMAG=1e-3 INCLUDEDC=NO",
        ] {
            let converted = PacConfig::from(&card(source));
            assert!(
                converted.validate().is_ok(),
                "'{source}' converted to a configuration the validator rejects: {:?}",
                converted.validate()
            );
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_non_finite_sweep_values() {
        for config in [
            PacConfig::new().with_sweep(f64::NAN, 1.0e3, 10),
            PacConfig::new().with_sweep(1.0, f64::INFINITY, 10),
            PacConfig::new().with_sweep(1.0, 1.0e3, 0),
        ] {
            assert!(
                config.validate().is_err(),
                "invalid PAC sweep config unexpectedly accepted: {config:?}"
            );
        }
    }

    #[test]
    fn validate_rejects_non_finite_tolerances() {
        for config in [
            PacConfig::new().with_tolerances(f64::NAN, 1.0e-12),
            PacConfig::new().with_tolerances(1.0e-3, f64::INFINITY),
        ] {
            assert!(
                config.validate().is_err(),
                "invalid PAC tolerance config unexpectedly accepted: {config:?}"
            );
        }
    }

    #[test]
    fn try_frequency_points_returns_validation_error_instead_of_empty_grid() {
        let config = PacConfig::new().with_sweep(1.0e6, 1.0, 10);

        let err = config
            .try_frequency_points()
            .expect_err("invalid PAC sweep should return the validation error");

        assert!(
            matches!(err, FrequencyGridError::InvalidStopFrequency),
            "unexpected PAC frequency error: {err}"
        );
        assert_eq!(
            config.frequency_points(),
            Err(FrequencyGridError::InvalidStopFrequency)
        );
    }

    #[test]
    fn frequency_grid_is_checked_fallible_and_cancellable() {
        assert_eq!(
            PacConfig::new()
                .with_sweep(1.0, 2.0, 3)
                .with_sweep_type(PacSweepType::Linear)
                .frequency_points()
                .expect("ordinary PAC grid"),
            vec![1.0, 1.5, 2.0]
        );
        assert!(matches!(
            PacConfig::new()
                .with_sweep(1.0, 2.0, usize::MAX)
                .with_sweep_type(PacSweepType::Linear)
                .frequency_points(),
            Err(FrequencyGridError::Allocation { .. })
        ));
        assert_eq!(
            PacConfig::new().try_frequency_points_with_abort(&crate::abort_signal::ImmediateAbort),
            Err(FrequencyGridError::Aborted)
        );
        assert_eq!(
            PacConfig::new()
                .with_sweep(f64::MIN_POSITIVE, f64::MAX, usize::MAX)
                .frequency_point_count(),
            Err(FrequencyGridError::PointCountOverflow)
        );
        assert_eq!(
            PacConfig::new()
                .with_sweep(1.0e3, 1.0e3, 10)
                .frequency_points()
                .expect("equal PAC endpoints remain valid"),
            vec![1.0e3]
        );
    }
}

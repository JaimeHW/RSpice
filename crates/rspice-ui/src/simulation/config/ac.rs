//! AC analysis configuration: sweep type, range, and points per decade.

// AC Analysis Configuration
//=============================================================================

/// AC sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcSweepType {
    /// Decade (logarithmic)
    #[default]
    Decade,
    /// Octave
    Octave,
    /// Linear
    Linear,
}

impl AcSweepType {
    pub(super) fn spice_name(&self) -> &'static str {
        match self {
            AcSweepType::Decade => "dec",
            AcSweepType::Octave => "oct",
            AcSweepType::Linear => "lin",
        }
    }

    /// The engine's own sweep kind, which is what its grid functions take.
    ///
    /// Crate-visible rather than config-visible: the sensitivity bridge
    /// builds its grid with the same public engine function the command line
    /// uses, and it needs the same spelling of the variation to do it.
    pub(crate) fn freq_variation(&self) -> rspice_core::netlist::FreqVariation {
        match self {
            AcSweepType::Decade => rspice_core::netlist::FreqVariation::Dec,
            AcSweepType::Octave => rspice_core::netlist::FreqVariation::Oct,
            AcSweepType::Linear => rspice_core::netlist::FreqVariation::Lin,
        }
    }
}

/// AC analysis configuration
#[derive(Debug, Clone)]
pub struct AcAnalysisConfig {
    /// Sweep type
    pub sweep_type: AcSweepType,
    /// Number of points (per decade/octave, or total for linear)
    pub num_points: usize,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
}

impl Default for AcAnalysisConfig {
    fn default() -> Self {
        Self {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
        }
    }
}

impl AcAnalysisConfig {
    /// Generate SPICE .ac command
    pub fn to_spice(&self) -> String {
        format!(
            ".ac {} {} {} {}",
            self.sweep_type.spice_name(),
            self.num_points,
            self.start_freq,
            self.stop_freq
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if !self.start_freq.is_finite() {
            errors.push("Start frequency must be finite".to_string());
        } else if self.start_freq < 0.0
            || (self.start_freq == 0.0 && self.sweep_type != AcSweepType::Linear)
        {
            errors.push(
                "Start frequency must be nonnegative for LIN and positive for DEC/OCT".to_string(),
            );
        }
        if !self.stop_freq.is_finite() {
            errors.push("Stop frequency must be finite".to_string());
        } else if self.stop_freq < 0.0 {
            errors.push("Stop frequency must be nonnegative".to_string());
        }
        if self.start_freq.is_finite()
            && self.stop_freq.is_finite()
            && self.start_freq > self.stop_freq
        {
            errors.push("Start frequency must not exceed stop frequency".to_string());
        }
        if self.num_points == 0 {
            errors.push("Number of points must be positive".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// AC analysis over a frequency axis the author states point by point.
///
/// This is `.AC DATA=<table>`, and it is a different request from a graded
/// `.AC`: there is no band and no point density, only the exact frequencies to
/// solve at. `table_name` is the `.DATA` table the card refers to — generated
/// beside the card for an authored axis, or the deck's own name when the
/// analysis was read from a hand-written deck.
#[derive(Debug, Clone, PartialEq)]
pub struct AcDataAnalysisConfig {
    /// Name of the `.DATA` table the `.AC` card reads its axis from.
    pub table_name: String,
    /// The exact frequency axis, in solve order.
    pub frequencies: Vec<f64>,
    /// Whether the axis was authored here, or imported with a deck's own table
    /// whose row order and row-local overrides belong to its author.
    pub authored: bool,
}

impl Default for AcDataAnalysisConfig {
    fn default() -> Self {
        Self {
            table_name: super::frequency_table::AC_FREQUENCY_TABLE.to_owned(),
            frequencies: Vec::new(),
            authored: true,
        }
    }
}

impl AcDataAnalysisConfig {
    /// The `.AC DATA=` card, followed by the table it reads.
    ///
    /// An imported table is already in the deck under its own name, so only
    /// the card is written; an authored axis carries its table with it.
    #[must_use]
    pub fn to_spice(&self) -> String {
        let table = self.table_name.trim();
        if !self.authored {
            return format!(".ac DATA={table}");
        }
        format!(
            ".ac DATA={table}\n{}",
            super::frequency_table::explicit_frequency_table(table, &self.frequencies)
        )
    }

    /// Validate the complete, executable table-driven configuration.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.table_name.trim().is_empty() {
            errors.push("AC DATA table name must not be empty".to_owned());
        }
        errors.extend(super::frequency_table::validate_ac_frequencies(
            &self.frequencies,
        ));
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_non_finite_frequency_bounds() {
        for (start_freq, stop_freq) in [
            (f64::NAN, 1.0),
            (f64::INFINITY, 1.0),
            (f64::NEG_INFINITY, 1.0),
            (1.0, f64::NAN),
            (1.0, f64::INFINITY),
            (1.0, f64::NEG_INFINITY),
        ] {
            let config = AcAnalysisConfig {
                start_freq,
                stop_freq,
                ..AcAnalysisConfig::default()
            };
            assert!(
                config.validate().is_err(),
                "accepted start={start_freq}, stop={stop_freq}"
            );
        }
    }

    #[test]
    fn validation_keeps_finite_positive_ordered_bounds() {
        assert!(AcAnalysisConfig::default().validate().is_ok());
    }

    /// The two analyses that sweep an authored frequency axis have to write the
    /// same table for the same axis. They did not always: the noise writer
    /// owned the column keyword, the decimal format and the `.ENDDATA` line on
    /// its own, so an AC copy of them could have drifted by any one of the
    /// three and produced a deck that swept a different axis than the form.
    #[test]
    fn the_noise_and_ac_frequency_tables_share_one_writer() {
        let frequencies = vec![10.0, 1.0e3, 1.0 / 3.0e-6];
        let ac = AcDataAnalysisConfig {
            frequencies: frequencies.clone(),
            ..AcDataAnalysisConfig::default()
        };
        let noise = crate::simulation::config::NoiseAnalysisConfig {
            output_node: "out".to_owned(),
            input_source: "V1".to_owned(),
            explicit_frequencies: Some(frequencies.clone()),
            ..crate::simulation::config::NoiseAnalysisConfig::default()
        };

        let table_body = |deck: &str| {
            deck.lines()
                .skip_while(|line| !line.starts_with(".DATA "))
                .skip(1)
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(table_body(&ac.to_spice()), table_body(&noise.to_spice()));
        assert!(table_body(&ac.to_spice()).starts_with("+ HERTZ\n"));

        // Each analysis still names its own generated table, and each card
        // refers to the table it wrote.
        assert!(
            ac.to_spice()
                .starts_with(".ac DATA=rspice_ac_frequency\n.DATA rspice_ac_frequency\n")
        );
        assert!(noise.to_spice().starts_with(
            ".noise V(out) V1 DATA=rspice_noise_frequency\n.DATA rspice_noise_frequency\n"
        ));
    }

    #[test]
    fn an_imported_table_is_referenced_and_not_rewritten() {
        let imported = AcDataAnalysisConfig {
            table_name: "pts".to_owned(),
            frequencies: vec![10.0, 1.0],
            authored: false,
        };
        assert_eq!(imported.to_spice(), ".ac DATA=pts");
        assert!(
            imported.validate().is_ok(),
            "a deck's own table keeps its author's row order"
        );
    }

    #[test]
    fn an_ac_axis_requires_finite_nonnegative_values_and_preserves_order() {
        for frequencies in [vec![], vec![-1.0], vec![f64::NAN], vec![f64::INFINITY]] {
            let config = AcDataAnalysisConfig {
                frequencies,
                ..AcDataAnalysisConfig::default()
            };
            assert!(config.validate().is_err(), "{config:?}");
        }
        let config = AcDataAnalysisConfig {
            frequencies: vec![100.0, 0.0, 10.0, 10.0],
            ..AcDataAnalysisConfig::default()
        };
        assert!(config.validate().is_ok());
    }
}

#[cfg(test)]
mod single_frequency_tests {
    use super::*;
    use crate::simulation::results::SimulationResult;
    use crate::simulation::{AnalysisConfig, EngineBridge};

    #[test]
    fn single_frequency_and_zero_start_lin_run_the_requested_grid() {
        for (kind, start, stop, count, expected) in [
            (AcSweepType::Linear, 0.0, 0.0, 1, vec![0.0]),
            (
                AcSweepType::Linear,
                0.0,
                1000.0,
                3,
                vec![0.0, 500.0, 1000.0],
            ),
            (AcSweepType::Decade, 1000.0, 1000.0, 10, vec![1000.0]),
            (AcSweepType::Octave, 1000.0, 1000.0, 4, vec![1000.0]),
        ] {
            let config = AcAnalysisConfig {
                sweep_type: kind,
                start_freq: start,
                stop_freq: stop,
                num_points: count,
            };
            config.validate().unwrap();
            let result = EngineBridge::new()
                .run(
                    &AnalysisConfig::Ac(config),
                    "AC divider\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
                )
                .unwrap();
            let SimulationResult::Ac {
                frequencies,
                waveforms,
                ..
            } = result
            else {
                panic!("expected AC");
            };
            assert_eq!(frequencies, expected);
            let trace = &waveforms["V(OUT)"];
            assert!(
                trace
                    .y_values
                    .iter()
                    .all(|value| (value - 0.5).abs() < 1e-12)
            );
            assert!(
                trace
                    .y_imag
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|value| value.abs() < 1e-12)
            );
        }
    }

    #[test]
    fn zero_frequency_remains_invalid_on_a_logarithmic_axis() {
        for kind in [AcSweepType::Decade, AcSweepType::Octave] {
            assert!(
                AcAnalysisConfig {
                    sweep_type: kind,
                    start_freq: 0.0,
                    ..Default::default()
                }
                .validate()
                .is_err()
            );
        }
    }
}

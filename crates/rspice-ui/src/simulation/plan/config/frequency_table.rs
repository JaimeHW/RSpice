//! The draft of an AC analysis over a stated frequency axis.
//!
//! A graded `.AC` draft holds a band and a point density. This one holds
//! neither: the author writes the frequencies, and the analysis solves exactly
//! those. Keeping it a separate draft rather than a fourth mode on the AC
//! sweep is what keeps one quantity to one control — an `.AC DATA=` card is a
//! different sealed request in the execution protocol
//! (`CanonicalAnalysisKind::AcData`, tag 3), and a plan kind names exactly one
//! tag.

use serde::{Deserialize, Serialize};

use crate::simulation::config::{
    AC_FREQUENCY_TABLE, AcDataAnalysisConfig, AcDataParameterColumn, parse_ac_frequency_list,
};

/// Raw form state for `.AC DATA=<table>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcDataDraft {
    #[serde(default)]
    pub from_netlist: bool,
    #[serde(default)]
    pub parameter_columns: Vec<AcDataParameterDraft>,
    /// Comma-, semicolon- or space-separated frequencies in hertz, exactly as
    /// typed. SI suffixes are accepted, so `1k` is a frequency here.
    pub frequencies: String,
    /// Name of the generated `.DATA` table the card refers to.
    ///
    /// Authored plans keep the generated default; a plan imported from a deck
    /// that named its own table retains that name so the card and the table in
    /// the deck stay the same two words.
    #[serde(default = "default_ac_frequency_table")]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcDataParameterDraft {
    pub name: String,
    pub values: String,
}

fn default_ac_frequency_table() -> String {
    AC_FREQUENCY_TABLE.to_owned()
}

impl Default for AcDataDraft {
    fn default() -> Self {
        Self {
            from_netlist: false,
            parameter_columns: Vec::new(),
            // A decade per entry across the audio band: a list that runs as
            // typed, and short enough that the author reads it as an example
            // of the form rather than as a band they have to edit down.
            frequencies: "10, 100, 1k, 10k, 100k".to_owned(),
            table_name: default_ac_frequency_table(),
        }
    }
}

impl AcDataDraft {
    /// Parse this raw draft into a complete executable configuration.
    pub fn to_config(&self) -> Result<AcDataAnalysisConfig, String> {
        let table_name = self.table_name.trim();
        if table_name.is_empty() {
            return Err("AC DATA table name is required".to_owned());
        }
        let config = AcDataAnalysisConfig {
            table_name: table_name.to_owned(),
            frequencies: if self.from_netlist {
                Vec::new()
            } else {
                parse_ac_frequency_list(&self.frequencies)?
            },
            authored: !self.from_netlist,
            parameter_columns: if self.from_netlist {
                Vec::new()
            } else {
                self.parameter_columns
                    .iter()
                    .map(|column| {
                        let values = column
                            .values
                            .split(|ch: char| ch == ',' || ch == ';' || ch.is_whitespace())
                            .filter(|value| !value.is_empty())
                            .map(|value| {
                                crate::simulation::spice_value::parse_spice_value_checked(value)
                                    .map_err(|error| format!("AC DATA {}: {error}", column.name))
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(AcDataParameterColumn {
                            name: column.name.trim().to_owned(),
                            values,
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?
            },
        };
        config.validate().map_err(|errors| errors.join("; "))?;
        Ok(config)
    }

    /// The one-line summary the plan manager shows for this instance.
    #[must_use]
    pub fn summary(&self) -> String {
        match self.to_config() {
            Ok(config) if !config.authored => format!("netlist table {}", config.table_name),
            Ok(config) => {
                let first = config.frequencies.first().copied().unwrap_or_default();
                let last = config.frequencies.last().copied().unwrap_or_default();
                format!(
                    "{} frequenc{} · {first} … {last} Hz",
                    config.frequencies.len(),
                    if config.frequencies.len() == 1 {
                        "y"
                    } else {
                        "ies"
                    }
                )
            }
            Err(_) => format!("table {}", self.table_name.trim()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_draft_runs_as_typed() {
        let config = AcDataDraft::default()
            .to_config()
            .expect("the default frequency list is executable");
        assert_eq!(config.frequencies, vec![10.0, 100.0, 1.0e3, 1.0e4, 1.0e5]);
        assert_eq!(config.table_name, AC_FREQUENCY_TABLE);
        assert!(config.authored);
    }

    #[test]
    fn an_ordered_list_keeps_repeats_and_zero_and_a_missing_table_is_refused() {
        let descending = AcDataDraft {
            frequencies: "10k, 0, 1k, 1k".to_owned(),
            ..AcDataDraft::default()
        };
        assert_eq!(
            descending.to_config().unwrap().frequencies,
            vec![10_000.0, 0.0, 1_000.0, 1_000.0]
        );
        let nameless = AcDataDraft {
            table_name: "  ".to_owned(),
            ..AcDataDraft::default()
        };
        assert_eq!(
            nameless
                .to_config()
                .expect_err("a nameless table is refused"),
            "AC DATA table name is required"
        );
    }

    #[test]
    fn the_summary_counts_the_axis_it_will_solve() {
        assert_eq!(
            AcDataDraft {
                frequencies: "1k".to_owned(),
                ..AcDataDraft::default()
            }
            .summary(),
            "1 frequency · 1000 … 1000 Hz"
        );
        assert!(
            AcDataDraft::default()
                .summary()
                .starts_with("5 frequencies")
        );
    }
}

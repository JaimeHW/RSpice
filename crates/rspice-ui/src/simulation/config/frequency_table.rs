//! The explicit frequency axis, owned once for every analysis that reads one.
//!
//! Two analyses sweep a frequency list the author states instead of a graded
//! band: `.NOISE … DATA=<table>` and `.AC DATA=<table>`. Both express it the
//! same way — a `.DATA` table with a single frequency column, referenced by
//! name from the analysis card — because that is the one spelling the engine's
//! `Netlist::frequency_data_table_points` resolver accepts.
//!
//! Three operations belong to that axis and were written per analysis before:
//! parsing the field an author types, deciding which lists are legal, and
//! writing the table the engine reads. A second copy that drifted by one
//! decimal digit, one column keyword or one ordering rule would produce a deck
//! that parses and sweeps a different axis than the form shows.

use crate::simulation::spice_value::parse_spice_value_checked;

/// Frequency column keyword written into a generated `.DATA` table.
///
/// The resolver accepts `FREQ` or `HERTZ` (`netlist::data_table`); `HERTZ` is
/// what the noise writer has always emitted, so it stays the spelling both
/// analyses share rather than changing the bytes of a card already in decks.
const FREQUENCY_COLUMN: &str = "HERTZ";

/// Table name the noise writer generates for an authored frequency list.
pub const NOISE_FREQUENCY_TABLE: &str = "rspice_noise_frequency";

/// Table name the AC writer generates for an authored frequency list.
pub const AC_FREQUENCY_TABLE: &str = "rspice_ac_frequency";

/// The `.DATA` table that carries `frequencies` as its sole swept column.
///
/// Values are written with seventeen significant decimals, the shortest form
/// that reads back as the same `f64` for every finite value, so the axis the
/// engine sweeps is the axis the form holds.
#[must_use]
pub fn explicit_frequency_table(table_name: &str, frequencies: &[f64]) -> String {
    let mut table = format!(".DATA {}\n+ {FREQUENCY_COLUMN}", table_name.trim());
    for frequency in frequencies {
        table.push_str(&format!("\n+ {frequency:.17e}"));
    }
    table.push_str("\n.ENDDATA");
    table
}

/// Whether duplicate or descending entries are refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyOrdering {
    /// Integration requires an ascending axis without repeated samples.
    StrictlyIncreasing,
    /// Solve rows in their authored order, including repeated frequencies.
    AsAuthored,
}

/// Validate an authored frequency axis, in the wording every analysis reports.
///
/// One error at most: the first offending entry is named and the scan stops,
/// because a list pasted in the wrong unit reports every entry otherwise and
/// buries the one fact the author needs.
#[must_use]
pub fn validate_explicit_frequencies(
    frequencies: &[f64],
    ordering: FrequencyOrdering,
) -> Option<String> {
    if frequencies.is_empty() {
        return Some("explicit frequency list must contain at least one value".to_owned());
    }
    let mut previous = None;
    for (index, frequency) in frequencies.iter().copied().enumerate() {
        if !frequency.is_finite() || frequency <= 0.0 {
            return Some(format!(
                "explicit frequency {} must be finite and greater than zero",
                index + 1
            ));
        }
        if matches!(ordering, FrequencyOrdering::StrictlyIncreasing)
            && previous.is_some_and(|previous| frequency <= previous)
        {
            return Some(
                "explicit frequencies must be strictly increasing without duplicates".to_owned(),
            );
        }
        previous = Some(frequency);
    }
    None
}

/// Parse and validate the comma-, semicolon- or space-separated list a form
/// field holds.
///
/// SI suffixes are accepted, because the field is where an author types `1k`.
pub fn parse_explicit_frequency_list(text: &str) -> Result<Vec<f64>, String> {
    let frequencies = parse_frequency_values(text)?;
    match validate_explicit_frequencies(&frequencies, FrequencyOrdering::StrictlyIncreasing) {
        Some(error) => Err(error),
        None => Ok(frequencies),
    }
}

fn parse_frequency_values(text: &str) -> Result<Vec<f64>, String> {
    text.split(|character: char| character == ',' || character == ';' || character.is_whitespace())
        .filter(|value| !value.is_empty())
        .map(|value| {
            parse_spice_value_checked(value)
                .map_err(|error| format!("invalid explicit frequency '{value}': {error}"))
        })
        .collect()
}

/// Parse an AC table without reordering or deduplicating its authored rows.
pub fn parse_ac_frequency_list(text: &str) -> Result<Vec<f64>, String> {
    let frequencies = parse_frequency_values(text)?;
    validate_ac_frequencies(&frequencies).map_or(Ok(frequencies), Err)
}

pub(super) fn validate_ac_frequencies(frequencies: &[f64]) -> Option<String> {
    if frequencies.is_empty() {
        return Some("explicit frequency list must contain at least one value".into());
    }
    frequencies
        .iter()
        .enumerate()
        .find_map(|(index, &frequency)| {
            (!frequency.is_finite() || frequency < 0.0).then(|| {
                format!(
                    "explicit frequency {} must be finite and nonnegative",
                    index + 1
                )
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_written_table_states_its_column_and_every_authored_row() {
        let table = explicit_frequency_table("t", &[1.0, 2.5e3]);
        let lines = table.lines().collect::<Vec<_>>();
        assert_eq!(lines[0], ".DATA t");
        assert_eq!(lines[1], "+ HERTZ");
        assert_eq!(lines[2], "+ 1.00000000000000000e0");
        assert_eq!(lines[3], "+ 2.50000000000000000e3");
        assert_eq!(lines[4], ".ENDDATA");
        assert_eq!(lines.len(), 5);
    }

    /// The written decimals have to read back as the exact authored value: the
    /// axis the engine sweeps is this text, not the vector it came from.
    #[test]
    fn every_written_row_reads_back_as_the_authored_frequency() {
        let authored = [1.0, 1.0 / 3.0, 9.876_543_21e7, f64::MIN_POSITIVE];
        let table = explicit_frequency_table("t", &authored);
        let read_back = table
            .lines()
            .skip(2)
            .filter(|line| line.starts_with("+ "))
            .map(|line| line[2..].parse::<f64>().expect("a written row parses"))
            .collect::<Vec<_>>();
        assert_eq!(read_back, authored);
    }

    #[test]
    fn a_nonpositive_or_infinite_entry_is_named_by_position() {
        for (list, expected) in [
            (vec![1.0, 0.0], "explicit frequency 2"),
            (vec![-1.0], "explicit frequency 1"),
            (vec![1.0, 2.0, f64::INFINITY], "explicit frequency 3"),
            (vec![f64::NAN], "explicit frequency 1"),
        ] {
            let error = validate_explicit_frequencies(&list, FrequencyOrdering::StrictlyIncreasing)
                .expect("the list is refused");
            assert!(error.starts_with(expected), "{error}");
        }
        assert_eq!(
            validate_explicit_frequencies(&[], FrequencyOrdering::StrictlyIncreasing),
            Some("explicit frequency list must contain at least one value".to_owned())
        );
    }

    #[test]
    fn ordering_is_required_of_an_authored_axis_and_not_of_an_imported_table() {
        let unordered = [10.0, 10.0, 1.0];
        assert!(
            validate_explicit_frequencies(&unordered, FrequencyOrdering::StrictlyIncreasing)
                .is_some()
        );
        assert!(
            validate_explicit_frequencies(&unordered, FrequencyOrdering::AsAuthored).is_none(),
            "a deck's own table keeps the author's row order"
        );
    }

    #[test]
    fn a_form_field_accepts_si_suffixes_and_either_separator() {
        assert_eq!(
            parse_explicit_frequency_list("1k, 2.5k 10k;100k").expect("the field parses"),
            vec![1.0e3, 2.5e3, 1.0e4, 1.0e5]
        );
        assert!(parse_explicit_frequency_list("  ").is_err());
        assert!(parse_explicit_frequency_list("1k, zebra").is_err());
        assert!(parse_explicit_frequency_list("10k, 1k").is_err());
    }
}

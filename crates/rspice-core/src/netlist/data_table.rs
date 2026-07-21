//! Shared validation for frequency-axis `.DATA` tables.
//!
//! Table-driven AC and noise analyses use the same Xyce contract: one named
//! table supplies a finite, positive `FREQ`/`HERTZ` axis and one value for
//! every declared parameter column on every row.  Keeping this validation in
//! the netlist layer prevents individual analysis frontends from drifting in
//! their handling of malformed tables.

use super::DataTable;
use crate::Value;
use std::fmt;

/// One validated row from a frequency-axis `.DATA` table.
#[derive(Debug, Clone, PartialEq)]
pub struct FrequencyDataPoint {
    /// The positive frequency from the table's sole `FREQ`/`HERTZ` column.
    pub frequency: Value,
    /// Authored column names paired with this row's values, in source order.
    pub overrides: Vec<(String, Value)>,
}

/// A semantic error found while resolving a table-driven frequency sweep.
#[derive(Debug, Clone, PartialEq)]
pub enum FrequencyDataTableError {
    /// The analysis references a table that is not present in the netlist.
    UnknownTable { table_name: String },
    /// The table has no declared parameter columns.
    EmptyColumns { table_name: String },
    /// The table has no data rows.
    EmptyRows { table_name: String },
    /// A parameter column is repeated case-insensitively.
    DuplicateColumn {
        table_name: String,
        column_name: String,
    },
    /// No frequency-axis column was declared.
    MissingFrequencyColumn { table_name: String },
    /// More than one frequency-axis column was declared.
    AmbiguousFrequencyColumns { table_name: String },
    /// A row does not contain one value per declared column.
    RowWidth {
        table_name: String,
        row: usize,
        actual: usize,
        expected: usize,
    },
    /// A table value is NaN or infinite.
    NonFiniteValue {
        table_name: String,
        row: usize,
        column_name: String,
        value: Value,
    },
    /// A frequency-axis value is not strictly positive and finite.
    InvalidFrequency {
        table_name: String,
        row: usize,
        frequency: Value,
    },
}

impl fmt::Display for FrequencyDataTableError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTable { table_name } => {
                write!(formatter, "references unknown .DATA table '{table_name}'")
            }
            Self::EmptyColumns { table_name } => {
                write!(formatter, "table '{table_name}' has no columns")
            }
            Self::EmptyRows { table_name } => write!(formatter, "table '{table_name}' has no rows"),
            Self::DuplicateColumn {
                table_name,
                column_name,
            } => write!(
                formatter,
                "table '{table_name}' has duplicate column '{column_name}'"
            ),
            Self::MissingFrequencyColumn { table_name } => {
                write!(
                    formatter,
                    "table '{table_name}' has no FREQ or HERTZ column"
                )
            }
            Self::AmbiguousFrequencyColumns { table_name } => write!(
                formatter,
                "table '{table_name}' has ambiguous frequency columns; expected exactly one FREQ or HERTZ column"
            ),
            Self::RowWidth {
                table_name,
                row,
                actual,
                expected,
            } => write!(
                formatter,
                "table '{table_name}' row {row} has {actual} value(s), expected {expected}"
            ),
            Self::NonFiniteValue {
                table_name,
                row,
                column_name,
                value,
            } => write!(
                formatter,
                "table '{table_name}' row {row} column '{column_name}' must be finite, got {value}"
            ),
            Self::InvalidFrequency {
                table_name,
                row,
                frequency,
            } => write!(
                formatter,
                "table '{table_name}' row {row} frequency must be positive and finite, got {frequency}"
            ),
        }
    }
}

impl std::error::Error for FrequencyDataTableError {}

impl DataTable {
    /// Validate and materialize every row using the table's frequency axis.
    pub fn frequency_points(&self) -> Result<Vec<FrequencyDataPoint>, FrequencyDataTableError> {
        if self.params.is_empty() {
            return Err(FrequencyDataTableError::EmptyColumns {
                table_name: self.name.clone(),
            });
        }
        if self.rows.is_empty() {
            return Err(FrequencyDataTableError::EmptyRows {
                table_name: self.name.clone(),
            });
        }

        let mut seen_columns = std::collections::BTreeSet::new();
        for column_name in &self.params {
            if !seen_columns.insert(column_name.to_ascii_uppercase()) {
                return Err(FrequencyDataTableError::DuplicateColumn {
                    table_name: self.name.clone(),
                    column_name: column_name.clone(),
                });
            }
        }

        let frequency_columns = self
            .params
            .iter()
            .enumerate()
            .filter_map(|(index, column_name)| {
                (column_name.eq_ignore_ascii_case("FREQ")
                    || column_name.eq_ignore_ascii_case("HERTZ"))
                .then_some(index)
            })
            .collect::<Vec<_>>();
        let frequency_column = match frequency_columns.as_slice() {
            [index] => *index,
            [] => {
                return Err(FrequencyDataTableError::MissingFrequencyColumn {
                    table_name: self.name.clone(),
                });
            }
            _ => {
                return Err(FrequencyDataTableError::AmbiguousFrequencyColumns {
                    table_name: self.name.clone(),
                });
            }
        };

        self.rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                let row_number = row_index + 1;
                if row.len() != self.params.len() {
                    return Err(FrequencyDataTableError::RowWidth {
                        table_name: self.name.clone(),
                        row: row_number,
                        actual: row.len(),
                        expected: self.params.len(),
                    });
                }
                if let Some((column_index, value)) =
                    row.iter().enumerate().find(|(_, value)| !value.is_finite())
                {
                    return Err(FrequencyDataTableError::NonFiniteValue {
                        table_name: self.name.clone(),
                        row: row_number,
                        column_name: self.params[column_index].clone(),
                        value: *value,
                    });
                }
                let frequency = row[frequency_column];
                if !frequency.is_finite() || frequency <= 0.0 {
                    return Err(FrequencyDataTableError::InvalidFrequency {
                        table_name: self.name.clone(),
                        row: row_number,
                        frequency,
                    });
                }
                Ok(FrequencyDataPoint {
                    frequency,
                    overrides: self
                        .params
                        .iter()
                        .cloned()
                        .zip(row.iter().copied())
                        .collect(),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(params: &[&str], rows: &[&[Value]]) -> DataTable {
        DataTable {
            name: "points".to_string(),
            params: params.iter().map(|param| (*param).to_string()).collect(),
            rows: rows.iter().map(|row| row.to_vec()).collect(),
        }
    }

    #[test]
    fn frequency_points_preserve_authored_rows_and_columns() {
        let points = table(&["gain", "HERTZ"], &[&[2.0, 10.0], &[3.0, 100.0]])
            .frequency_points()
            .expect("frequency table validates");
        assert_eq!(points[0].frequency, 10.0);
        assert_eq!(
            points[1].overrides,
            vec![("gain".to_string(), 3.0), ("HERTZ".to_string(), 100.0)]
        );
    }

    #[test]
    fn frequency_points_reject_duplicate_axis_and_malformed_rows() {
        let duplicate = table(&["FREQ", "hertz"], &[&[1.0, 1.0]])
            .frequency_points()
            .expect_err("duplicate frequency axes must fail");
        assert!(matches!(
            duplicate,
            FrequencyDataTableError::AmbiguousFrequencyColumns { .. }
        ));

        let malformed = table(&["FREQ", "value"], &[&[0.0]])
            .frequency_points()
            .expect_err("row width must fail before frequency validation");
        assert!(matches!(
            malformed,
            FrequencyDataTableError::RowWidth { .. }
        ));
    }
}

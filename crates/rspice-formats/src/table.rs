//! Borrowed, already-selected engineering table values for byte encoders.

/// The caller owns view selection, sorting, filtering, and displayed values.
/// Encoders only read this projection and produce format bytes.
pub trait EngineeringTableSource {
    fn column_count(&self) -> usize;
    fn row_count(&self) -> usize;
    fn column_id(&self, column: usize) -> &str;
    fn column_label(&self, column: usize) -> &str;
    fn column_unit(&self, column: usize) -> Option<&str>;
    fn numeric_value(&self, row: usize, column: usize) -> Option<f64>;
    fn display_value(&self, row: usize, column: usize) -> Option<&str>;
}

/// Encode an already-selected engineering table as CSV or TSV text.
pub fn encode_delimited_table(
    source: &impl EngineeringTableSource,
    delimiter: u8,
    include_headers: bool,
    include_units: bool,
) -> Result<String, String> {
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(Vec::new());
    if include_headers {
        writer
            .write_record((0..source.column_count()).map(|column| {
                if include_units {
                    source.column_unit(column).map_or_else(
                        || source.column_label(column).to_owned(),
                        |unit| format!("{} [{unit}]", source.column_label(column)),
                    )
                } else {
                    source.column_label(column).to_owned()
                }
            }))
            .map_err(|error| error.to_string())?;
    }
    for row in 0..source.row_count() {
        writer
            .write_record(
                (0..source.column_count())
                    .map(|column| source.display_value(row, column).unwrap_or_default()),
            )
            .map_err(|error| error.to_string())?;
    }
    let bytes = writer.into_inner().map_err(|error| error.to_string())?;
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::{EngineeringTableSource, encode_delimited_table};

    struct SelectedTable;

    impl EngineeringTableSource for SelectedTable {
        fn column_count(&self) -> usize {
            2
        }
        fn row_count(&self) -> usize {
            2
        }
        fn column_id(&self, column: usize) -> &str {
            ["time", "label"][column]
        }
        fn column_label(&self, column: usize) -> &str {
            ["Time", "Label"][column]
        }
        fn column_unit(&self, column: usize) -> Option<&str> {
            (column == 0).then_some("s")
        }
        fn numeric_value(&self, row: usize, column: usize) -> Option<f64> {
            (column == 0).then_some([1.5, 2.5][row])
        }
        fn display_value(&self, row: usize, column: usize) -> Option<&str> {
            Some([["1.5", "a,b"], ["2.5", "\"quoted\""]][row][column])
        }
    }

    #[test]
    fn selected_text_escapes_delimiters_and_quotes() {
        assert_eq!(
            encode_delimited_table(&SelectedTable, b',', true, true).unwrap(),
            "Time [s],Label\n1.5,\"a,b\"\n2.5,\"\"\"quoted\"\"\"\n"
        );
    }
}

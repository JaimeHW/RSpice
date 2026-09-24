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

/// Escape one field for the existing typed-result CSV schema.
pub fn escape_csv_field(value: &str) -> String {
    if value
        .chars()
        .any(|character| matches!(character, ',' | '"' | '\r' | '\n'))
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Convert a validated typed-result CSV document into TSV without changing its cells.
pub fn csv_to_tsv(contents: &str) -> Result<String, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(contents.as_bytes());
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(Vec::new());
    for record in reader.records() {
        let record = record.map_err(|error| format!("could not parse staged CSV rows: {error}"))?;
        writer
            .write_record(&record)
            .map_err(|error| format!("could not encode TSV row: {error}"))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| format!("could not finish TSV encoding: {}", error.error()))?;
    String::from_utf8(bytes).map_err(|error| format!("TSV encoder returned invalid UTF-8: {error}"))
}

struct ParsedLabeledCsvTable {
    sequence: String,
    label: String,
    headers: csv::StringRecord,
    rows: Vec<csv::StringRecord>,
}

/// Accumulate selected CSV tables in caller order, parsing each immediately.
pub struct CsvTableMerger {
    columns: Vec<String>,
    parsed: Vec<ParsedLabeledCsvTable>,
}

impl Default for CsvTableMerger {
    fn default() -> Self {
        Self::new()
    }
}

impl CsvTableMerger {
    pub fn new() -> Self {
        Self {
            columns: vec!["analysis_sequence".to_owned(), "analysis_label".to_owned()],
            parsed: Vec::new(),
        }
    }

    /// Parse one validated table before the caller selects the next analysis.
    pub fn push(&mut self, sequence: String, label: &str, contents: &str) -> Result<(), String> {
        let mut reader = csv::Reader::from_reader(contents.as_bytes());
        let headers = reader.headers().map_err(|error| error.to_string())?.clone();
        for header in &headers {
            if !self.columns.iter().any(|column| column == header) {
                self.columns.push(header.to_owned());
            }
        }
        let rows = reader
            .records()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        self.parsed.push(ParsedLabeledCsvTable {
            sequence,
            label: label.to_owned(),
            headers,
            rows,
        });
        Ok(())
    }

    /// Encode the union of columns and return the text and data-row count.
    pub fn finish(self) -> Result<(String, usize), String> {
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer
            .write_record(&self.columns)
            .map_err(|error| error.to_string())?;
        let mut count = 0;
        for table in self.parsed {
            let mapping = table
                .headers
                .iter()
                .map(|header| {
                    self.columns
                        .iter()
                        .position(|column| column == header)
                        .unwrap()
                })
                .collect::<Vec<_>>();
            for row in table.rows {
                let mut cells = vec![""; self.columns.len()];
                for (value, &index) in row.iter().zip(&mapping) {
                    cells[index] = value;
                }
                cells[0] = &table.sequence;
                cells[1] = &table.label;
                writer
                    .write_record(cells)
                    .map_err(|error| error.to_string())?;
                count += 1;
            }
        }
        let bytes = writer.into_inner().map_err(|error| error.to_string())?;
        Ok((
            String::from_utf8(bytes).map_err(|error| error.to_string())?,
            count,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CsvTableMerger, EngineeringTableSource, csv_to_tsv, encode_delimited_table,
        escape_csv_field,
    };

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

    #[test]
    fn typed_csv_to_tsv_preserves_quoted_cells() {
        let csv = format!(
            "name,value\n{},{}\n",
            escape_csv_field("a,b"),
            escape_csv_field("one\"two\nthree")
        );
        assert_eq!(
            csv_to_tsv(&csv).unwrap(),
            "name\tvalue\na,b\t\"one\"\"two\nthree\"\n"
        );
    }

    #[test]
    fn labeled_tables_merge_different_columns_and_quote_labels() {
        let mut merger = CsvTableMerger::new();
        merger
            .push("7".to_owned(), "run,one", "kind,value\nspectrum,1\n")
            .unwrap();
        merger
            .push("8".to_owned(), "run two", "status,kind\nready,metadata\n")
            .unwrap();
        assert_eq!(
            merger.finish().unwrap(),
            (
                "analysis_sequence,analysis_label,kind,value,status\n7,\"run,one\",spectrum,1,\n8,run two,metadata,,ready\n".to_owned(),
                2
            )
        );
    }
}

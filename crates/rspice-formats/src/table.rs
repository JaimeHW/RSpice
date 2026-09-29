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

/// Preserve CSV diagnostics, finalization failures, and invalid encoded text.
#[derive(Debug)]
pub enum DelimitedTableError {
    Csv(csv::Error),
    Finalize(std::io::Error),
    Utf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for DelimitedTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Csv(source) => source.fmt(f),
            Self::Finalize(source) => source.fmt(f),
            Self::Utf8(source) => source.fmt(f),
        }
    }
}

impl std::error::Error for DelimitedTableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Csv(source) => source,
            Self::Finalize(source) => source,
            Self::Utf8(source) => source,
        })
    }
}

/// The failed stage of a CSV-to-TSV conversion with its original cause.
#[derive(Debug)]
pub enum TsvConversionError {
    Read(csv::Error),
    Write(csv::Error),
    Finalize(std::io::Error),
    Utf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for TsvConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(source) => write!(f, "could not parse staged CSV rows: {source}"),
            Self::Write(source) => write!(f, "could not encode TSV row: {source}"),
            Self::Finalize(source) => write!(f, "could not finish TSV encoding: {source}"),
            Self::Utf8(source) => write!(f, "TSV encoder returned invalid UTF-8: {source}"),
        }
    }
}

impl std::error::Error for TsvConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Read(source) | Self::Write(source) => source,
            Self::Finalize(source) => source,
            Self::Utf8(source) => source,
        })
    }
}

/// Encode an already-selected engineering table as CSV or TSV text.
pub fn encode_delimited_table(
    source: &impl EngineeringTableSource,
    delimiter: u8,
    include_headers: bool,
    include_units: bool,
) -> Result<String, DelimitedTableError> {
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
            .map_err(DelimitedTableError::Csv)?;
    }
    for row in 0..source.row_count() {
        writer
            .write_record(
                (0..source.column_count())
                    .map(|column| source.display_value(row, column).unwrap_or_default()),
            )
            .map_err(DelimitedTableError::Csv)?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| DelimitedTableError::Finalize(error.into_error()))?;
    String::from_utf8(bytes).map_err(DelimitedTableError::Utf8)
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

/// Normalize a waveform label for existing CSV export diagnostics.
pub fn sanitize_column_label(label: &str) -> String {
    let sanitized = label
        .trim()
        .chars()
        .map(|ch| match ch {
            ',' | '\t' | '\r' | '\n' => ' ',
            _ => ch,
        })
        .collect::<String>();
    sanitized.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Convert a validated typed-result CSV document into TSV without changing its cells.
pub fn csv_to_tsv(contents: &str) -> Result<String, TsvConversionError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(contents.as_bytes());
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(Vec::new());
    for record in reader.records() {
        let record = record.map_err(TsvConversionError::Read)?;
        writer
            .write_record(&record)
            .map_err(TsvConversionError::Write)?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| TsvConversionError::Finalize(error.into_error()))?;
    String::from_utf8(bytes).map_err(TsvConversionError::Utf8)
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
    pub fn push(
        &mut self,
        sequence: String,
        label: &str,
        contents: &str,
    ) -> Result<(), DelimitedTableError> {
        let mut reader = csv::Reader::from_reader(contents.as_bytes());
        let headers = reader.headers().map_err(DelimitedTableError::Csv)?.clone();
        for header in &headers {
            if !self.columns.iter().any(|column| column == header) {
                self.columns.push(header.to_owned());
            }
        }
        let rows = reader
            .records()
            .collect::<Result<Vec<_>, _>>()
            .map_err(DelimitedTableError::Csv)?;
        self.parsed.push(ParsedLabeledCsvTable {
            sequence,
            label: label.to_owned(),
            headers,
            rows,
        });
        Ok(())
    }

    /// Encode the union of columns and return the text and data-row count.
    pub fn finish(self) -> Result<(String, usize), DelimitedTableError> {
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer
            .write_record(&self.columns)
            .map_err(DelimitedTableError::Csv)?;
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
                    .map_err(DelimitedTableError::Csv)?;
                count += 1;
            }
        }
        let bytes = writer
            .into_inner()
            .map_err(|error| DelimitedTableError::Finalize(error.into_error()))?;
        Ok((
            String::from_utf8(bytes).map_err(DelimitedTableError::Utf8)?,
            count,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CsvTableMerger, DelimitedTableError, EngineeringTableSource, TsvConversionError,
        csv_to_tsv, encode_delimited_table, escape_csv_field,
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

    #[test]
    fn malformed_table_errors_retain_csv_positions_and_causes() {
        use std::error::Error as _;

        let contents = "name,value\n\"first\nsecond\",1\nshort\n";
        let mut merger = CsvTableMerger::new();
        let merged_error = merger.push("7".into(), "run", contents).unwrap_err();
        let DelimitedTableError::Csv(source) = &merged_error else {
            panic!("expected a CSV parse error: {merged_error}");
        };
        assert_eq!(source.position().unwrap().line(), 4);
        assert!(matches!(
            source.kind(),
            csv::ErrorKind::UnequalLengths { .. }
        ));
        assert_eq!(merged_error.to_string(), source.to_string());
        assert!(merged_error.source().unwrap().is::<csv::Error>());

        let tsv_error = csv_to_tsv(contents).unwrap_err();
        let TsvConversionError::Read(source) = &tsv_error else {
            panic!("expected a CSV parse error: {tsv_error}");
        };
        assert_eq!(source.position().unwrap().line(), 4);
        assert!(matches!(
            source.kind(),
            csv::ErrorKind::UnequalLengths { .. }
        ));
        assert_eq!(
            tsv_error.to_string(),
            format!("could not parse staged CSV rows: {source}")
        );
        assert!(tsv_error.source().unwrap().is::<csv::Error>());
    }
}

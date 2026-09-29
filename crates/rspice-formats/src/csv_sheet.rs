//! Bounded CSV text sheets with physical source-line identities.

/// A decoded data row and the physical line on which its record starts.
pub type CsvSheetRow = (u64, Vec<String>);

#[derive(Debug)]
pub struct CsvSheet {
    pub headers: Vec<String>,
    pub rows: Vec<CsvSheetRow>,
}

#[derive(Debug)]
pub enum CsvSheetError {
    ByteLimit { bytes: usize, limit: usize },
    RowLimit { limit: usize },
    Empty,
    Csv(csv::Error),
}

impl CsvSheetError {
    /// Parser locations use physical lines. Resource refusals have no row.
    pub fn line(&self) -> u64 {
        match self {
            Self::Csv(source) => source.position().map_or(0, csv::Position::line),
            Self::Empty => 1,
            Self::ByteLimit { .. } | Self::RowLimit { .. } => 0,
        }
    }
}

impl std::fmt::Display for CsvSheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ByteLimit { bytes, limit } => {
                write!(f, "CSV sheet has {bytes} bytes; the limit is {limit}")
            }
            Self::RowLimit { limit } => write!(f, "CSV sheet exceeds the {limit} row limit"),
            Self::Empty => f.write_str("CSV sheet has no data rows"),
            Self::Csv(source) => write!(f, "CSV sheet is malformed: {source}"),
        }
    }
}

impl std::error::Error for CsvSheetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Csv(source) => Some(source),
            _ => None,
        }
    }
}

/// Decode a header and nonempty rows without interpreting their field meanings.
/// Byte limits include the optional BOM. All cells are trimmed, ragged rows
/// are refused, and physical source lines survive quoted multiline fields.
pub fn decode_csv_sheet(
    source: &str,
    max_bytes: usize,
    max_rows: usize,
) -> Result<CsvSheet, CsvSheetError> {
    if source.len() > max_bytes {
        return Err(CsvSheetError::ByteLimit {
            bytes: source.len(),
            limit: max_bytes,
        });
    }
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(false)
        .from_reader(source.as_bytes());
    let headers = reader
        .headers()
        .map_err(CsvSheetError::Csv)?
        .iter()
        .map(str::to_owned)
        .collect();
    let mut rows = Vec::new();
    for record in reader.records() {
        if rows.len() >= max_rows {
            return Err(CsvSheetError::RowLimit { limit: max_rows });
        }
        let record = record.map_err(CsvSheetError::Csv)?;
        let line = record.position().map_or(0, csv::Position::line);
        rows.push((line, record.iter().map(str::to_owned).collect()));
    }
    if rows.is_empty() {
        return Err(CsvSheetError::Empty);
    }
    Ok(CsvSheet { headers, rows })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_text_and_source_lines_survive_bom_and_whitespace() {
        let source = "\u{feff} name, expression, description\nR1, 1k,\" first\nsecond \"\nC1, 1u,\"quote: \"\"yes\"\"\"\n";
        let sheet = decode_csv_sheet(source, source.len(), 2).unwrap();
        assert_eq!(sheet.headers, ["name", "expression", "description"]);
        assert_eq!(
            sheet.rows,
            vec![
                (2, vec!["R1".into(), "1k".into(), "first\nsecond".into()]),
                (4, vec!["C1".into(), "1u".into(), "quote: \"yes\"".into()]),
            ]
        );
        let error = decode_csv_sheet(source, source.len() - 1, 2).unwrap_err();
        assert!(
            matches!(error, CsvSheetError::ByteLimit { bytes, limit } if bytes == source.len() && limit == source.len() - 1)
        );
    }

    #[test]
    fn malformed_empty_and_over_limit_sheets_retain_refusal_identity() {
        let ragged = "name,value\nR1,1k\nC1\n";
        let error = decode_csv_sheet(ragged, ragged.len(), 2).unwrap_err();
        assert!(matches!(error, CsvSheetError::Csv(_)));
        assert_eq!(error.line(), 3);
        // The existing reader refuses the row cap before inspecting another
        // record's error, including a malformed record beyond the cap.
        let error = decode_csv_sheet(ragged, ragged.len(), 1).unwrap_err();
        assert!(matches!(error, CsvSheetError::RowLimit { limit: 1 }));
        assert_eq!(error.line(), 0);
        let error = decode_csv_sheet("name,value\n", 100, 2).unwrap_err();
        assert!(matches!(error, CsvSheetError::Empty));
        assert_eq!(error.line(), 1);
    }
}

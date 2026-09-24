//! XLSX byte encoding for an already-selected engineering table.

use crate::table::EngineeringTableSource;
use rust_xlsxwriter::{Color, Format, Workbook};

pub struct XlsxTableOptions {
    pub include_headers: bool,
    pub include_units: bool,
    pub pinned_columns: usize,
    pub column_widths: Vec<u16>,
    pub metadata: Option<Vec<(String, String)>>,
}

pub fn encode_xlsx_table(
    source: &impl EngineeringTableSource,
    options: XlsxTableOptions,
) -> Result<Vec<u8>, String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet
        .set_name("Engineering table")
        .map_err(|error| error.to_string())?;
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x20282d))
        .set_font_color(Color::RGB(0xd7dbde));
    let row_offset = u32::from(options.include_headers);
    if options.include_headers {
        for column_index in 0..source.column_count() {
            let header = if options.include_units {
                source.column_unit(column_index).map_or_else(
                    || source.column_label(column_index).to_owned(),
                    |unit| format!("{} [{unit}]", source.column_label(column_index)),
                )
            } else {
                source.column_label(column_index).to_owned()
            };
            worksheet
                .write_string_with_format(0, column_index as u16, &header, &header_format)
                .map_err(|error| error.to_string())?;
        }
        if source.column_count() != 0 {
            worksheet
                .autofilter(
                    0,
                    0,
                    source.row_count() as u32,
                    source.column_count() as u16 - 1,
                )
                .map_err(|error| error.to_string())?;
        }
    }
    for row_index in 0..source.row_count() {
        for column_index in 0..source.column_count() {
            if let Some(number) = source.numeric_value(row_index, column_index) {
                worksheet
                    .write_number(row_index as u32 + row_offset, column_index as u16, number)
                    .map_err(|error| error.to_string())?;
            } else {
                worksheet
                    .write_string(
                        row_index as u32 + row_offset,
                        column_index as u16,
                        source
                            .display_value(row_index, column_index)
                            .unwrap_or_default(),
                    )
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    if options.include_headers {
        worksheet
            .set_freeze_panes(1, options.pinned_columns.min(source.column_count()) as u16)
            .map_err(|error| error.to_string())?;
    }
    for index in 0..source.column_count() {
        let width = options.column_widths.get(index).copied().unwrap_or(120);
        worksheet
            .set_column_width(index as u16, f64::from(width) / 7.0)
            .map_err(|error| error.to_string())?;
    }
    if let Some(metadata_rows) = options.metadata {
        let metadata = workbook.add_worksheet();
        metadata
            .set_name("RSpice provenance")
            .map_err(|error| error.to_string())?;
        for (row, (key, value)) in metadata_rows.into_iter().enumerate() {
            metadata
                .write_string(row as u32, 0, &key)
                .map_err(|error| error.to_string())?;
            metadata
                .write_string(row as u32, 1, &value)
                .map_err(|error| error.to_string())?;
        }
    }
    workbook.save_to_buffer().map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::{XlsxTableOptions, encode_xlsx_table};
    use crate::table::EngineeringTableSource;
    use std::io::{Cursor, Read};

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
            (column == 0).then_some([0.0, 1.5][row])
        }
        fn display_value(&self, row: usize, column: usize) -> Option<&str> {
            (column == 1 && row == 0).then_some("cold")
        }
    }

    #[test]
    fn workbook_retains_selected_values_and_provenance() {
        let bytes = encode_xlsx_table(
            &SelectedTable,
            XlsxTableOptions {
                include_headers: true,
                include_units: true,
                pinned_columns: 1,
                column_widths: vec![140, 120],
                metadata: Some(vec![("Grid".to_owned(), "fixture".to_owned())]),
            },
        )
        .expect("XLSX bytes");
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("XLSX archive");
        let names = archive.file_names().map(str::to_owned).collect::<Vec<_>>();
        assert!(names.iter().any(|name| name == "xl/worksheets/sheet1.xml"));
        assert!(names.iter().any(|name| name == "xl/worksheets/sheet2.xml"));
        let mut xml = String::new();
        for name in names.into_iter().filter(|name| name.ends_with(".xml")) {
            archive
                .by_name(&name)
                .expect("XML part")
                .read_to_string(&mut xml)
                .expect("XML text");
        }
        assert!(xml.contains("Time [s]"));
        assert!(xml.contains("cold"));
        assert!(xml.contains("1.5"));
        assert!(xml.contains("autoFilter"));
        assert!(xml.contains("xSplit=\"1\""));
        assert!(xml.contains("fixture"));
    }
}

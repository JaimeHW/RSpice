//! CSV for an exact, selected network matrix sample.
use crate::table::escape_csv_field as csv_field;
use rspice_results::network_matrix::NetworkMatrixTable;

pub fn encode_network_matrix_csv(table: &NetworkMatrixTable) -> String {
    let mut csv = table.columns.join(",");
    csv.push('\n');
    for row in &table.rows {
        csv.push_str(
            &row.iter()
                .map(|v| csv_field(v))
                .collect::<Vec<_>>()
                .join(","),
        );
        csv.push('\n');
    }
    csv
}

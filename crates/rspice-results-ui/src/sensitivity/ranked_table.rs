//! The ranked table beside each sensitivity chart, on the kit's table as
//! every sheet's: each column sorts, the rank and name stay put as the rest
//! scroll sideways, Ctrl+C copies the selected rows as shown, and a row's
//! menu copies the row at full precision.

use crate::fft::view::columns::Fitting;
use crate::presentation::{
    details_column_widths, details_table_room, table_row_menu, with_table_state,
};
use egui::Ui;
use rspice_ui_kit::widgets::table::{Align, Cell, Column, DataTable, RowKey};

/// One ranked table: its columns, its listed rows and what they were built
/// from.
pub(crate) struct RankedTable<'a> {
    /// Under which the table keeps its sort, selection and widths.
    pub salt: &'a str,
    /// Each column's head, and whether it holds names, which read from the
    /// left and give up width first, rather than figures.
    pub columns: &'a [(&'a str, bool)],
    /// Each listed row's identity, so a sort or a filter keeps the reader's
    /// selection on its row.
    pub keys: &'a [RowKey],
    /// Changes whenever any listed row's cells do.
    pub revision: u64,
}

/// How many of a column's longest cells its width is measured against: the
/// widths are laid out every frame, and a ranking runs to thousands of rows.
const MEASURED_CELLS: usize = 3;

/// Draw `table`, its row `row`'s cells from `cells`; the row menu's "Copy exact
/// row" puts `exact(row)` on the clipboard, every digit the run holds.
pub(crate) fn ranked_table(
    ui: &mut Ui,
    table: &RankedTable<'_>,
    cells: impl Fn(usize) -> Vec<Cell>,
    exact: impl Fn(usize) -> String,
) {
    let rows = table.keys.len();
    let widths = column_widths(ui, table, &cells);
    let columns: Vec<Column> = table
        .columns
        .iter()
        .zip(widths)
        .map(|((head, named), width)| {
            if *named {
                Column::name(*head, width)
            } else {
                Column::number(*head, width)
            }
        })
        .collect();
    let id = format!("rspice.results.sensitivity.{}", table.salt);
    let response = with_table_state(ui, table.salt, |ui, state| {
        DataTable::new(&id, &columns, rows)
            .state(state)
            .keys(table.keys)
            .freeze(2)
            .revision(table.revision)
            .show(ui, &cells)
    });
    table_row_menu(ui, table.salt, response.context_menu, |ui, row| {
        if row < rows && ui.button("Copy exact row").clicked() {
            ui.ctx().copy_text(exact(row));
            ui.close();
        }
    });
}

/// The columns' widths, measured against each column's longest cells, which
/// are found once per revision of the rows rather than every frame.
fn column_widths(
    ui: &Ui,
    table: &RankedTable<'_>,
    cells: &impl Fn(usize) -> Vec<Cell>,
) -> Vec<f32> {
    let id = ui
        .id()
        .with(("rspice.results.sensitivity.longest", table.salt));
    let cached = ui.data(|data| data.get_temp::<(u64, std::sync::Arc<Vec<Vec<String>>>)>(id));
    let longest = match cached {
        Some((revision, longest)) if revision == table.revision => longest,
        _ => {
            let longest = std::sync::Arc::new(longest_cells(
                table.columns.len(),
                (0..table.keys.len()).map(cells),
            ));
            ui.data_mut(|data| {
                data.insert_temp(id, (table.revision, std::sync::Arc::clone(&longest)));
            });
            longest
        }
    };
    let fitting: Vec<Fitting> = table
        .columns
        .iter()
        .zip(longest.iter())
        .map(|((head, named), cells)| Fitting {
            head: (*head).to_owned(),
            tooltip: None,
            align: if *named { Align::Left } else { Align::Right },
            cells: cells.iter().map(|text| (text.clone(), !named)).collect(),
        })
        .collect();
    let named: Vec<bool> = table.columns.iter().map(|(_, named)| *named).collect();
    details_column_widths(ui, &[&fitting], &named, details_table_room(ui))
}

/// Each of `columns` columns' longest few cells over `rows`, by characters:
/// the cells are set in one face, so the longest spelling is the widest.
fn longest_cells(columns: usize, rows: impl Iterator<Item = Vec<Cell>>) -> Vec<Vec<String>> {
    let mut longest: Vec<Vec<String>> = vec![Vec::new(); columns];
    for row in rows {
        for (kept, cell) in longest.iter_mut().zip(row) {
            let length = cell.text.chars().count();
            let at = kept
                .iter()
                .position(|text| text.chars().count() < length)
                .unwrap_or(kept.len());
            if at < MEASURED_CELLS {
                kept.insert(at, cell.text);
                kept.truncate(MEASURED_CELLS);
            }
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_columns_width_is_measured_against_its_longest_cells() {
        let rows = ["1", "333", "22", "4444", "55555"]
            .into_iter()
            .map(|text| vec![Cell::text(text), Cell::number(format!("{text}.0"))]);
        let longest = longest_cells(2, rows);
        assert_eq!(longest[0], ["55555", "4444", "333"]);
        assert_eq!(longest[1], ["55555.0", "4444.0", "333.0"]);
    }
}

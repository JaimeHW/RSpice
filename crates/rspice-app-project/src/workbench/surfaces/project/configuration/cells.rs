//! The Cells table: each library/cell of the expanded design once, how many
//! instances it has, what they resolved to, and the cell binding they all
//! follow. A cell binding for a cell the design does not instantiate binds
//! nothing; it stays listed, marked, so it can be removed.

use super::*;

use crate::ui::widgets::table::{Cell, Column, DataTable, RowKey, SortKey};
use crate::workbench::project::configuration_state::{
    CellTableRow, ConfigurationPageState, ConfigurationRow, ConfigurationTables, INHERITED,
};

use super::instances::{TableAction, answer, ident, lead_follows, table_height, view_list};

fn columns() -> [Column; 6] {
    [
        Column::words("Library", 104.0),
        Column::name("Cell", 120.0),
        Column::number("Instances", 84.0),
        Column::words("View found", 110.0),
        Column::name("View to use", 120.0),
        Column::name("Inherited view list", 140.0)
            .tooltip("The views a cell binding's instances go back to when it is removed"),
    ]
}

pub(super) fn cells_table(
    ui: &mut Ui,
    page: &mut ConfigurationPageState,
    tables: &ConfigurationTables,
) -> Vec<TableAction> {
    let rows = &tables.cell_rows;
    let keys = rows
        .iter()
        .map(|row| RowKey::Number(row.key))
        .collect::<Vec<_>>();
    let wanted = page.row.as_ref().and_then(|row| tables.row_key(row));
    lead_follows(&mut page.cells_table, &keys, wanted);
    let columns = columns();
    let response = DataTable::new("configuration.cells", &columns, rows.len())
        .state(&mut page.cells_table)
        .keys(&keys)
        .multi_select()
        .freeze(2)
        .max_height(table_height(ui))
        .show(ui, |index| cell_cells(&rows[index], &tables.inherited));
    let identity = |index: usize| {
        let row = &rows[index];
        ConfigurationRow::Cell {
            library: row.library.clone(),
            cell: row.cell.clone(),
        }
    };
    answer(&page.cells_table, &response, &identity)
}

/// One library/cell. The list it inherits is stated only where it has a
/// binding of its own, since that list is what removing the binding goes
/// back to; a cell that inherits says so, and the grid above states the list.
fn cell_cells(row: &CellTableRow, inherited: &str) -> Vec<Cell> {
    let own = row.to_use != INHERITED;
    let found = if !row.in_design {
        Cell::problem("not in this design", Problem::Warning)
    } else if row.found == "\u{2014}" {
        Cell::absent()
    } else {
        ident(&row.found)
    };
    vec![
        ident(&row.library),
        ident(&row.cell),
        Cell::number(row.instances.to_string()).sort_by(SortKey::Number(row.instances as f64)),
        found,
        view_list(&row.to_use),
        if own { ident(inherited) } else { Cell::empty() },
    ]
}

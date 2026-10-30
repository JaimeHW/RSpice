//! The Cells table: each library/cell of the expanded design once, how many
//! instances it has, what they resolved to, and the cell binding they all
//! follow. A cell binding for a cell the design does not instantiate binds
//! nothing; it stays listed, marked, so it can be removed.

use super::*;

use crate::state::{ConfigurationSetDefinition, LibraryManager, default_view_search_order};
use crate::ui::widgets::select;
use crate::workbench::project::configuration_state::{
    CellRow, ConfigurationRow, cell_binding, cell_view_names,
};

use super::instances::TableEdit;

const TABLE_MIN_WIDTH: f32 = 700.0;
const INHERITED: &str = "Inherited";

const COLUMNS: [(&str, f32); 6] = [
    ("Library", 0.15),
    ("Cell", 0.18),
    ("Instances", 0.09),
    ("View found", 0.16),
    ("View to use", 0.16),
    ("Inherited view list", 0.26),
];

/// One row as drawn: a cell of the design, or a cell binding that binds
/// nothing in it.
struct Row<'a> {
    library: &'a str,
    cell: &'a str,
    instances: usize,
    views_found: &'a [String],
    in_design: bool,
}

pub(super) fn cells_table(
    ui: &mut Ui,
    cells: &[CellRow],
    definition: Option<&ConfigurationSetDefinition>,
    libraries: &LibraryManager,
    selected: Option<&ConfigurationRow>,
    editable: bool,
) -> Option<TableEdit> {
    let t = Tokens::get(ui.ctx());
    let row_height = t.metrics.row_h;
    let header_height = (row_height - 4.0).max(22.0);
    let width = visible_workspace_width(ui).max(TABLE_MIN_WIDTH);
    let mut rows = cells
        .iter()
        .map(|cell| Row {
            library: &cell.library,
            cell: &cell.cell,
            instances: cell.instances,
            views_found: &cell.views_found,
            in_design: true,
        })
        .collect::<Vec<_>>();
    if let Some(definition) = definition {
        for binding in &definition.cell_bindings {
            let bound = cells.iter().any(|cell| {
                cell.library.eq_ignore_ascii_case(&binding.library)
                    && cell.cell.eq_ignore_ascii_case(&binding.cell)
            });
            if !bound {
                rows.push(Row {
                    library: &binding.library,
                    cell: &binding.cell,
                    instances: 0,
                    views_found: &[],
                    in_design: false,
                });
            }
        }
    }
    let inherited = definition.map_or_else(
        || {
            let order = default_view_search_order("schematic", false);
            let mut words = vec!["the instance's own view".to_owned()];
            words.extend(order.into_iter().skip(1));
            words.join(", then ")
        },
        |definition| definition.executable_view_policy.join(", then "),
    );
    let context = RowContext {
        definition,
        libraries,
        selected,
        inherited: &inherited,
        editable,
        width,
        height: row_height,
    };
    let mut edit = None;
    ScrollArea::horizontal()
        .id_salt("workbench.project.configuration.cells.horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(width);
            header(ui, width, header_height);
            let height = visible_workspace_height(ui).max(row_height * 4.0);
            ScrollArea::vertical()
                .id_salt("workbench.project.configuration.cells")
                .auto_shrink([false, false])
                .max_height(height)
                .show_rows(ui, row_height, rows.len(), |ui, range| {
                    ui.set_min_width(width);
                    for row in &rows[range] {
                        if let Some(row_edit) = cell_row(ui, row, &context) {
                            edit = Some(row_edit);
                        }
                    }
                });
        });
    edit
}

fn header(ui: &mut Ui, width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("Cell columns: {}", COLUMNS.map(|(name, _)| name).join(", ")),
        )
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    for (index, (name, cell)) in columns(rect).into_iter().enumerate() {
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        text(ui, ui.painter(), cell, name, &font, t.color.text_faint);
    }
}

fn columns(row: Rect) -> [(&'static str, Rect); 6] {
    let mut left = row.left();
    COLUMNS.map(|(name, share)| {
        let right = (left + row.width() * share).min(row.right());
        let cell = Rect::from_min_max(pos2(left, row.top()), pos2(right, row.bottom()));
        left = right;
        (name, cell)
    })
}

fn text(
    ui: &Ui,
    painter: &egui::Painter,
    cell: Rect,
    value: &str,
    font: &egui::FontId,
    color: Color32,
) {
    let inner = cell.shrink2(vec2(7.0, 0.0));
    let value = elide_text(ui, value, font, inner.width().max(1.0));
    painter.with_clip_rect(inner).text(
        inner.left_center(),
        Align2::LEFT_CENTER,
        value,
        font.clone(),
        color,
    );
}

/// What every row of one frame draws from.
#[derive(Clone, Copy)]
struct RowContext<'a> {
    definition: Option<&'a ConfigurationSetDefinition>,
    libraries: &'a LibraryManager,
    selected: Option<&'a ConfigurationRow>,
    inherited: &'a str,
    editable: bool,
    width: f32,
    height: f32,
}

fn cell_row(ui: &mut Ui, row: &Row<'_>, context: &RowContext<'_>) -> Option<TableEdit> {
    let RowContext {
        definition,
        libraries,
        selected,
        inherited,
        editable,
        width,
        height,
    } = *context;
    let t = Tokens::get(ui.ctx());
    let is_selected = matches!(
        selected,
        Some(ConfigurationRow::Cell { library, cell })
            if library.eq_ignore_ascii_case(row.library) && cell.eq_ignore_ascii_case(row.cell)
    );
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    let binding = definition.and_then(|definition| cell_binding(definition, row.library, row.cell));
    let to_use = binding
        .and_then(|binding| binding.executable_views.first())
        .map_or(INHERITED, String::as_str);
    let found = if row.views_found.is_empty() {
        "\u{2014}".to_owned()
    } else {
        row.views_found.join(", ")
    };
    let mut label = format!(
        "Cell {}/{}: {} instance{}; view found {found}; view to use {to_use}; inherited view list {inherited}",
        row.library,
        row.cell,
        row.instances,
        plural_suffix(row.instances)
    );
    if !row.in_design {
        label = format!(
            "{label}; {}",
            Problem::Warning.announce("not in this design")
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            is_selected,
            &label,
        )
    });
    if is_selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    theme::paint_focus_ring(ui, &response, rect);
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let cells = columns(rect);
    for (_, cell) in cells.iter().skip(1) {
        painter.vline(
            cell.left(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    text(
        ui,
        &painter,
        cells[0].1,
        row.library,
        &mono,
        t.color.text_dim,
    );
    text(ui, &painter, cells[1].1, row.cell, &mono, t.color.text);
    text(
        ui,
        &painter,
        cells[2].1,
        &row.instances.to_string(),
        &mono,
        t.color.text_dim,
    );
    let found_cell = cells[3].1;
    if row.in_design {
        text(ui, &painter, found_cell, &found, &mono, t.color.text_dim);
    } else {
        paint_problem_mark(
            &painter,
            Rect::from_center_size(
                pos2(found_cell.left() + 7.0 + MARK_SIDE * 0.5, rect.center().y),
                vec2(MARK_SIDE, MARK_SIDE),
            ),
            Problem::Warning,
        );
        text(
            ui,
            &painter,
            Rect::from_min_max(
                pos2(found_cell.left() + MARK_ADVANCE, found_cell.top()),
                found_cell.max,
            ),
            "not in this design",
            &theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    }
    let mut edit = None;
    let view_cell = cells[4].1;
    if editable {
        let mut options = vec![INHERITED.to_owned()];
        options.extend(cell_view_names(libraries, row.library, row.cell));
        let mut picked = None;
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(view_cell.shrink2(vec2(3.0, 2.0)))
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                picked = select(
                    ui,
                    &format!("configuration-cell-view-{}-{}", row.library, row.cell),
                    &format!("View to use for {}/{}", row.library, row.cell),
                    to_use,
                    &options,
                    ui.available_width(),
                );
            },
        );
        if let Some(index) = picked
            && options[index] != to_use
        {
            edit = Some(TableEdit::CellView {
                library: row.library.to_owned(),
                cell: row.cell.to_owned(),
                view: (index > 0).then(|| options[index].clone()),
            });
        }
    } else {
        text(ui, &painter, view_cell, to_use, &mono, t.color.text_dim);
    }
    text(
        ui,
        &painter,
        cells[5].1,
        inherited,
        &mono,
        t.color.text_faint,
    );
    if response.clicked() && edit.is_none() {
        edit = Some(TableEdit::Select(ConfigurationRow::Cell {
            library: row.library.to_owned(),
            cell: row.cell.to_owned(),
        }));
    }
    edit
}

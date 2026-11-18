//! Bind to cell: the picker that binds instances or cells of the
//! Configuration tables to another cell, as the Cadence Hierarchy Editor
//! binds an instance or a cell to a library, cell and view. A library, then
//! its cells and views, the ones that fit first; each one that does not fit
//! states the pins that keep it out, and Bind says why it refuses one.
//!
//! Binding is an edit of the configuration like any other on its page: Apply
//! commits it as one step of Undo.

use egui::{Context, Margin, Ui};

use super::{ConfigurationDialog, ConfigurationDialogParts, ConfigurationDialogState};
use crate::state::{CellViewRef, ConfigurationSetId};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::table::{Cell, Column, DataTable, Group, RowKey, TableState};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogHintTone, DialogSize, DialogTransactionTone, Problem, select,
};
use crate::workbench::app_state::Model;
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationRow, TargetChoice, bound_target, placed_cells,
    placed_pins, rows_label, target_choices,
};
use rspice_design::schematic::port::PortSpec;

/// The widest the library picker grows.
const LIBRARY_WIDTH: f32 = 240.0;

/// What the picker binds, and what it offers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindToCell {
    id: ConfigurationSetId,
    rows: Vec<ConfigurationRow>,
    /// The library whose cells are offered, and every library the project
    /// reaches.
    library: String,
    libraries: Vec<String>,
    /// The cells the rows place, which are not offered, and the pins they
    /// connect, which every offer is measured against.
    own: Vec<(String, String)>,
    placed: Vec<(String, Vec<PortSpec>)>,
    /// The cell views of `library` the rows can take, fitting first.
    choices: Vec<TargetChoice>,
    /// Some row runs another cell now, which the secondary undoes.
    bound: bool,
}

impl BindToCell {
    /// Offer the cells of `library`.
    fn offer(&mut self, model: &Model, library: String) {
        self.choices = target_choices(
            &model.workspace,
            &model.library_manager,
            &model.schematic,
            &library,
            &self.placed,
            &self.own,
        );
        self.library = library;
    }
}

/// Open the picker for `rows` of configuration `id`, on the library of the
/// cell they are bound to, else of the cell they place.
pub fn open_bind_to_cell(
    dialogs: &mut ConfigurationDialogState,
    model: &Model,
    configuration: &ConfigurationPageState,
    id: ConfigurationSetId,
    rows: Vec<ConfigurationRow>,
) {
    let catalog = &model.workspace.content.configuration_sets;
    let definition = configuration.definition(catalog, id);
    let current = definition.and_then(|definition| {
        rows.iter()
            .find_map(|row| bound_target(definition, row))
            .cloned()
    });
    let resolution = configuration.last_resolution();
    let own = resolution
        .as_deref()
        .map(|resolution| placed_cells(resolution, &rows))
        .unwrap_or_default();
    let placed = resolution
        .as_deref()
        .map(|resolution| placed_pins(resolution, &rows))
        .unwrap_or_default();
    let mut libraries = model
        .library_manager
        .libraries_by_key()
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>();
    libraries.sort_by_key(|name| name.to_lowercase());
    let bound = current.is_some();
    let library = current
        .map(|target| target.library)
        .or_else(|| own.first().map(|(library, _)| library.clone()))
        .or_else(|| libraries.first().cloned())
        .unwrap_or_default();
    let mut picker = BindToCell {
        id,
        bound,
        rows,
        library: String::new(),
        libraries,
        own,
        placed,
        choices: Vec::new(),
    };
    picker.offer(model, library);
    dialogs.bind_table = TableState::default();
    dialogs.refusal = None;
    dialogs.open = Some(ConfigurationDialog::BindToCell(Box::new(picker)));
}

/// Draw the picker and bind what the reader picked.
pub(super) fn bind_to_cell_dialog(ctx: &Context, parts: &mut ConfigurationDialogParts<'_>) {
    let Some(ConfigurationDialog::BindToCell(picker)) = parts.dialogs.open.as_ref() else {
        return;
    };
    let mut picker = (**picker).clone();
    let write_allowed = !parts.model.safe_mode.project_read_only();
    let picked = parts
        .dialogs
        .bind_table
        .selected_row()
        .and_then(|row| picker.choices.get(row))
        .cloned();
    let mut dialog = Dialog::new(
        format!("Bind {} to another cell", rows_label(&picker.rows)),
        "Bind",
    )
    .description(
        "Run another cell in the place of the picked instances or cells, when it holds their pins.",
    )
    .size(DialogSize::Manager)
    .flush_body()
    .overlay_scrollbar()
    .ghost("Cancel")
    .primary_enabled(write_allowed && picked.as_ref().is_some_and(|choice| choice.fit.is_ok()));
    if picker.bound {
        dialog = dialog
            .secondary("Run placed cell")
            .secondary_enabled(write_allowed);
    }
    match picked.as_ref().map(|choice| &choice.fit) {
        Some(Err(pins)) => {
            dialog = dialog
                .hint(format!("Does not fit: {pins}"))
                .hint_tone(DialogHintTone::Error);
        }
        Some(Ok(Some(unconnected))) => {
            dialog = dialog
                .hint(format!("Fits, and {unconnected}"))
                .hint_tone(DialogHintTone::Warn);
        }
        Some(Ok(None)) | None => {}
    }
    if let Some(refusal) = parts.dialogs.refusal.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "The binding was refused",
            refusal,
        );
    }
    let mut library = None;
    let mut activated = false;
    let table = &mut parts.dialogs.bind_table;
    let choice = dialog.show(ctx, |ui| {
        library = library_band(ui, &picker);
        activated = choices_table(ui, table, &picker);
    });
    if let Some(index) = library {
        let name = picker.libraries[index].clone();
        picker.offer(parts.model, name);
        parts.dialogs.bind_table = TableState::default();
        parts.dialogs.open = Some(ConfigurationDialog::BindToCell(Box::new(picker)));
        return;
    }
    let choice = if activated && write_allowed {
        DialogChoice::Primary
    } else {
        choice
    };
    let target = match choice {
        DialogChoice::Primary => match picked {
            Some(picked) if picked.fit.is_ok() => Some(CellViewRef::new(
                &picker.library,
                &picked.cell,
                &picked.view,
            )),
            _ => return,
        },
        DialogChoice::Secondary => None,
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            parts.dialogs.close();
            return;
        }
        DialogChoice::None => return,
    };
    if parts.configuration.bind_to_cell(
        &parts.model.workspace,
        &parts.model.library_manager,
        &parts.model.schematic,
        picker.id,
        &picker.rows,
        target.as_ref(),
    ) {
        parts.dialogs.close();
    } else {
        parts.dialogs.refusal = parts.configuration.refusal.take();
    }
}

/// The band over the table: the library whose cells it lists. The library
/// the reader picked, if they picked one.
fn library_band(ui: &mut Ui, picker: &BindToCell) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Library")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                let width = LIBRARY_WIDTH.min(ui.available_width()).max(120.0);
                select(
                    ui,
                    "configuration-bind-library",
                    "Library",
                    &picker.library,
                    &picker.libraries,
                    width,
                )
            })
            .inner
        })
        .inner
}

/// The cell views of the library, fitting first. Whether the reader asked
/// to bind the row they activated.
fn choices_table(ui: &mut Ui, state: &mut TableState, picker: &BindToCell) -> bool {
    let columns = [
        Column::name("Cell", 110.0),
        Column::name("View", 90.0),
        Column::words("Pins", 280.0),
    ];
    if picker.choices.is_empty() {
        let t = Tokens::get(ui.ctx());
        egui::Frame::new()
            .inner_margin(Margin::symmetric(10, 7))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "{} holds no cell but the one placed.",
                        picker.library
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
        return false;
    }
    let fitting = picker
        .choices
        .iter()
        .filter(|choice| choice.fit.is_ok())
        .count();
    let groups = match fitting {
        0 => vec![Group::new("Does not fit", 0)],
        all if all == picker.choices.len() => vec![Group::new("Fits", 0)],
        some => vec![Group::new("Fits", 0), Group::new("Does not fit", some)],
    };
    let keys = picker
        .choices
        .iter()
        .map(|choice| RowKey::Text(format!("{}/{}", choice.cell, choice.view)))
        .collect::<Vec<_>>();
    let response = DataTable::new("configuration.bind-cell", &columns, picker.choices.len())
        .state(state)
        .keys(&keys)
        .groups(&groups)
        .freeze(1)
        .show(ui, |index| {
            let choice = &picker.choices[index];
            vec![
                Cell::number(&choice.cell),
                Cell::number(&choice.view),
                match &choice.fit {
                    Ok(None) => Cell::text("fits"),
                    Ok(Some(unconnected)) => Cell::problem(unconnected, Problem::Warning),
                    Err(pins) => Cell::problem(pins, Problem::Failure),
                },
            ]
        });
    response.activate.is_some_and(|row| {
        picker
            .choices
            .get(row)
            .is_some_and(|choice| choice.fit.is_ok())
    })
}

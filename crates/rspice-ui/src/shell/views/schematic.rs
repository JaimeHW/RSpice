//! Schematic view — breadcrumb document bar with check status, above the
//! schematic canvas (document well).

use egui::Ui;

use crate::common::AppState;
use crate::schematic::symbols::SymbolLibrary;
use crate::ui::widgets::{Pill, PillState, crumb_text, docbar};

/// Render the schematic editor view.
pub fn show(ui: &mut Ui, state: &mut AppState, symbol_library: Option<&SymbolLibrary>) {
    docbar(ui, |ui| {
        breadcrumb(ui, state);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            check_status_pill(ui, state);
        });
    });

    crate::schematic::view::render_schematic_view(ui, state, symbol_library);
}

/// Breadcrumb. With a descended hierarchy, intermediate cells are clickable
/// and pop back to that level.
fn breadcrumb(ui: &mut Ui, state: &mut AppState) {
    let stack_len = state.workspace.hierarchy_stack.len();
    if stack_len <= 1 {
        let reference = &state.workspace.active_view;
        crumb_text(
            ui,
            &[
                (reference.library.as_str(), false),
                (reference.cell.as_str(), true),
                (reference.view.as_str(), false),
            ],
        );
        return;
    }

    ui.spacing_mut().item_spacing.x = 0.0;
    let library = state.workspace.active_view.library.clone();
    crumb_text(ui, &[(library.as_str(), false)]);
    let cells: Vec<String> = state
        .workspace
        .hierarchy_stack
        .iter()
        .map(|reference| reference.cell.clone())
        .collect();
    let mut focus: Option<usize> = None;
    for (index, cell) in cells.iter().enumerate() {
        crumb_text(ui, &[(" › ", false)]);
        let last = index + 1 == stack_len;
        if last {
            crumb_text(ui, &[(cell.as_str(), true)]);
        } else if ui
            .link(
                egui::RichText::new(cell)
                    .font(crate::ui::theme::mono(
                        crate::ui::tokens::FS_0,
                        crate::ui::theme::FontWeight::Regular,
                    )),
            )
            .clicked()
        {
            focus = Some(index);
        }
    }
    if let Some(index) = focus {
        state.focus_workspace_breadcrumb(index);
    }
}

/// ERC/DRC status pill from the latest check results.
fn check_status_pill(ui: &mut Ui, state: &AppState) {
    match &state.dialogs.drc_results {
        Some(_)
            if state.dialogs.drc_checked_version != state.schematic.topology_version() =>
        {
            Pill::new(PillState::Idle, "ERC stale · re-run").show(ui);
        }
        Some(result) => {
            let errors = result.errors().len();
            let warnings = result.warnings().len();
            if errors > 0 {
                Pill::new(PillState::Error, &format!("ERC {errors} errors")).show(ui);
            } else if warnings > 0 {
                Pill::new(PillState::Ok, &format!("ERC clean · {warnings} warn")).show(ui);
            } else {
                Pill::new(PillState::Ok, "ERC clean").show(ui);
            }
        }
        None => {
            Pill::new(PillState::Idle, "ERC not run").show(ui);
        }
    }
}

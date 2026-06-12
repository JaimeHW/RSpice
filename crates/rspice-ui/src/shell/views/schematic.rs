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
            selection_echo(ui, state);
        });
    });

    crate::schematic::view::render_schematic_view(ui, state, symbol_library);
}

/// Faint selection echo beside the check pill — the eye is on the canvas
/// when the selection changes, and the status bar is a saccade away.
/// Silent with nothing selected; tool narration stays in the status bar.
fn selection_echo(ui: &mut Ui, state: &AppState) {
    let selection = &state.schematic.selection;
    let text = if let Some(id) = selection.single_component() {
        state
            .schematic
            .components
            .iter()
            .find(|c| c.id == id)
            .map(|component| {
                if component.value.is_empty() {
                    format!("Sel: {}", component.name)
                } else {
                    format!("Sel: {} ({})", component.name, component.value)
                }
            })
    } else {
        let count = selection.count();
        (count > 1).then(|| format!("Sel: {count} items"))
    };
    let Some(text) = text else {
        return;
    };
    ui.add_space(10.0);
    let t = crate::ui::tokens::Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(crate::ui::theme::mono(
                crate::ui::tokens::FS_0,
                crate::ui::theme::FontWeight::Regular,
            ))
            .color(t.color.text_faint),
    );
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
    // Occurrence path: root cell, then the instance descended through at
    // each level — TOP › X1 › XB, the orientation Virtuoso users expect.
    let labels = state.workspace.occurrence_labels();
    let cells: Vec<String> = state
        .workspace
        .hierarchy_stack
        .iter()
        .map(|reference| reference.cell.clone())
        .collect();
    let mut focus: Option<usize> = None;
    for (index, label) in labels.iter().enumerate() {
        crumb_text(ui, &[(" › ", false)]);
        let last = index + 1 == stack_len;
        if last {
            crumb_text(ui, &[(label.as_str(), true)]);
            // When the crumb is an instance name, say which cell it opens.
            if cells.get(index).is_some_and(|cell| cell != label) {
                crumb_text(ui, &[(" ‹", false)]);
                crumb_text(ui, &[(cells[index].as_str(), false)]);
                crumb_text(ui, &[("›", false)]);
            }
        } else if ui
            .link(
                egui::RichText::new(label)
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

/// ERC/DRC status pill from the latest check results. Clicking it runs the
/// checks — the pill names the state, the click changes it.
fn check_status_pill(ui: &mut Ui, state: &mut AppState) {
    let response = match &state.dialogs.drc_results {
        Some(_)
            if state.dialogs.drc_checked_version != state.schematic.topology_version() =>
        {
            Pill::new(PillState::Idle, "ERC stale · re-run").show(ui)
        }
        Some(result) => {
            let errors = result.errors().len();
            let warnings = result.warnings().len();
            if errors > 0 {
                Pill::new(PillState::Error, &format!("ERC {errors} errors")).show(ui)
            } else if warnings > 0 {
                Pill::new(PillState::Ok, &format!("ERC clean · {warnings} warn")).show(ui)
            } else {
                Pill::new(PillState::Ok, "ERC clean").show(ui)
            }
        }
        None => Pill::new(PillState::Idle, "ERC not run").show(ui),
    };
    let response = response
        .interact(egui::Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text("Run design checks (Ctrl+E)");
    if response.clicked() {
        crate::common::menu_bar::run_design_rule_check(state);
    }
}

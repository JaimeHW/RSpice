//! Exact vector-interface configuration before built-in XSPICE placement.

use egui::{Context, DragValue, Grid, RichText, Ui};

use crate::state::{
    Tool, builtin_xspice_library_binding_with_vector_widths, engine_only_xspice_devices,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::app::RSpiceApp;

const EYEBROW: &str = "SCHEMATIC · BUILT-IN XSPICE";
const PRIMARY: &str = "Arm placement";
const DESCRIPTION: &str = "Choose the exact logical width of every vector port. The resulting pin order and executable connection contract are frozen with each placed instance.";
const DISCARD_TITLE: &str = "Unsaved interface changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close. No schematic object or placement tool has been changed.";

impl RSpiceApp {
    pub(in crate::workbench) fn render_builtin_xspice_placement_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.builtin_xspice_placement.open {
            return;
        }

        let validation_error = validate_draft(self).err();
        let discard_confirm = self.state.dialogs.builtin_xspice_placement.discard_confirm;
        let title = format!(
            "Configure {}",
            self.state.dialogs.builtin_xspice_placement.display_name
        );
        let mut dialog = Dialog::new(EYEBROW, &title, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation_error.is_none())
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }

        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            vector_width_form(
                ui,
                &mut self.state.dialogs.builtin_xspice_placement,
                validation_error.as_deref(),
            )
        });
        match choice {
            DialogChoice::Primary => match materialize_draft(self) {
                Ok(binding) => {
                    let label = format!("{}/{}", binding.library, binding.cell);
                    self.state.schematic.pending_library_cell = Some(binding);
                    self.state
                        .schematic
                        .arm_tool(Tool::Place(crate::state::ComponentType::CellInstance));
                    self.state.dialogs.builtin_xspice_placement.close();
                    self.state.ui.toasts.success(
                        ctx,
                        "Component placement armed",
                        format!("{label} will snap to the schematic grid."),
                    );
                    crate::schematic::view::request_schematic_canvas_focus(ctx);
                }
                Err(error) => {
                    self.state.dialogs.builtin_xspice_placement.validation_error = Some(error);
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.builtin_xspice_placement.attempt_close()
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(app: &RSpiceApp) -> Result<(), String> {
    let draft = &app.state.dialogs.builtin_xspice_placement;
    if app.state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    if draft.design_execution_epoch != app.state.design_execution_epoch
        || draft.active_schematic_epoch != app.state.active_schematic_epoch
        || draft.view_path != app.state.workspace.active_view.display_path()
    {
        return Err(
            "The active design or cell view changed. Close and reopen the device configuration."
                .to_owned(),
        );
    }
    materialize_draft(app).map(|_| ())
}

fn materialize_draft(app: &RSpiceApp) -> Result<crate::state::LibraryCellInstance, String> {
    let draft = &app.state.dialogs.builtin_xspice_placement;
    let descriptor = engine_only_xspice_devices()
        .iter()
        .find(|descriptor| descriptor.stable_id == draft.stable_id)
        .ok_or_else(|| {
            format!(
                "The built-in catalog no longer contains '{}'.",
                draft.stable_id
            )
        })?;
    builtin_xspice_library_binding_with_vector_widths(descriptor, &draft.widths)
}

fn vector_width_form(
    ui: &mut Ui,
    draft: &mut crate::workbench::app::BuiltinXspicePlacementDialogState,
    validation_error: Option<&str>,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new("Vector interface")
            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.add_space(6.0);

    let mut first = None;
    let mut edited = false;
    Grid::new("builtin-xspice-vector-widths")
        .num_columns(3)
        .spacing(egui::vec2(12.0, 8.0))
        .show(ui, |ui| {
            ui.strong("Port");
            ui.strong("Width");
            ui.strong("Executable range");
            ui.end_row();
            for port in &draft.vector_ports {
                ui.label(&port.name);
                let width = draft
                    .widths
                    .entry(port.name.clone())
                    .or_insert(port.default_width);
                let maximum = port.maximum.unwrap_or(usize::MAX);
                let fixed = port.minimum == maximum;
                let response = ui.add_enabled(
                    !fixed,
                    DragValue::new(width)
                        .range(port.minimum..=maximum)
                        .speed(1.0),
                );
                first.get_or_insert(response.id);
                edited |= response.changed();
                let range = if fixed {
                    format!("fixed at {}", port.minimum)
                } else if port.null_allowed && port.minimum == 0 {
                    format!("0–{maximum} · 0 omits/nulls the port")
                } else {
                    format!("{}–{maximum}", port.minimum)
                };
                ui.label(RichText::new(range).color(t.color.text_dim));
                ui.end_row();
            }
        });
    if edited {
        draft.mark_edited();
    }

    ui.add_space(10.0);
    ui.label(
        RichText::new(
            "Every materialized vector element (and each side of a differential element) remains a distinct connectable schematic terminal.",
        )
        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_dim),
    );
    if let Some(error) = validation_error.or(draft.validation_error.as_deref()) {
        ui.add_space(6.0);
        ui.label(
            RichText::new(error)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.err),
        );
    }
    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materialization_uses_the_exact_selected_width() {
        let descriptor = engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == "d_lut")
            .expect("lookup-table descriptor");
        let ports = crate::state::builtin_xspice_vector_ports(descriptor).expect("ports");
        let mut app = RSpiceApp::test_instance();
        app.state.dialogs.builtin_xspice_placement.open(
            descriptor.stable_id,
            descriptor.display_name,
            ports,
            app.state.design_execution_epoch,
            app.state.active_schematic_epoch,
            app.state.workspace.active_view.display_path(),
        );
        app.state
            .dialogs
            .builtin_xspice_placement
            .widths
            .insert("in".to_owned(), 6);

        let binding = materialize_draft(&app).expect("binding");
        assert_eq!(binding.terminal_order.len(), 7);
        assert_eq!(binding.builtin_xspice.unwrap().ports[0].vector_width, 6);
    }
}

//! Text-first netlist and automation-pipeline surface.

use egui::Ui;

use crate::common::RSpiceApp;
use crate::io::NetlistFormat;
use crate::ui::tokens::Tokens;

use super::super::commands::Command;
use super::super::design_system::heading;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ensure_buffer(app);
    super::super::netlist_document::prepare(&mut app.state);
    header(ui, app);
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.canvas_bg).show(ui, |ui| {
        super::super::netlist_document::show_editor(ui, &mut app.state);
    });
}

fn ensure_buffer(app: &mut RSpiceApp) {
    if let Some(source) = &app.state.workspace.netlist_source {
        if app.state.simulation.netlist_content != *source
            && !app.state.workspace.netlist_source_dirty
        {
            app.state.simulation.netlist_content = source.clone();
        }
    } else if app.state.simulation.netlist_content.trim().is_empty()
        && !app.state.schematic.components.is_empty()
    {
        crate::common::menu_bar::action_view_netlist(&mut app.state);
    }
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(14, 9))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    heading(
                        ui,
                        "Automation-pipeline owner",
                        if app.state.workspace.netlist_source.is_some() {
                            "Project-owned editable source"
                        } else {
                            "Generated netlist · immutable"
                        },
                        &format!(
                            "{} · {} lines",
                            app.state.workspace.active_display_path(),
                            app.state.simulation.netlist_content.lines().count()
                        ),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let running = app.state.simulation.is_running;
                    if ui
                        .add_enabled(
                            if running {
                                true
                            } else {
                                app.state.manual_deck_run_block_reason().is_none()
                            },
                            egui::Button::new(if running { "Stop" } else { "Run deck" }),
                        )
                        .clicked()
                    {
                        if running {
                            Command::StopSimulation.execute(app);
                        } else {
                            app.state.request_netlist_manual_deck_run();
                        }
                    }
                    if ui.button("Copy").clicked() {
                        ui.ctx()
                            .copy_text(app.state.simulation.netlist_content.clone());
                    }
                    ui.menu_button("Export", |ui| {
                        for (label, format) in [
                            ("SPICE…", NetlistFormat::Spice),
                            ("Spectre…", NetlistFormat::Spectre),
                            ("HSPICE…", NetlistFormat::Hspice),
                            ("Xyce…", NetlistFormat::Xyce),
                        ] {
                            if ui.button(label).clicked() {
                                Command::ExportNetlist(format).execute(app);
                                ui.close();
                            }
                        }
                    });
                    if app.state.workspace.netlist_source.is_some() {
                        if ui.button("Regenerate from schematic").clicked() {
                            return_to_generated_netlist(&mut app.state);
                        }
                    } else if ui.button("Make editable copy").clicked() {
                        make_editable_copy(&mut app.state);
                    }
                });
            });
        });
}

/// Explicitly transition the immutable generated artifact into project-owned
/// editable source. No editor or tuner path is permitted to call this helper.
fn make_editable_copy(state: &mut crate::common::AppState) -> bool {
    let generated = state.simulation.netlist_content.clone();
    if !state.workspace.make_netlist_editable_copy(&generated) {
        return false;
    }

    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    true
}

fn return_to_generated_netlist(state: &mut crate::common::AppState) -> bool {
    if !state.workspace.return_to_generated_netlist() {
        return false;
    }

    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    crate::common::menu_bar::action_view_netlist(state);
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_editable_copy_preserves_generated_bytes_and_marks_project_dirty() {
        let mut state = crate::common::AppState::default();
        state.simulation.netlist_content =
            "* generated\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned();
        state.ui.netlist.revision = 12;
        state.ui.netlist.completion_open = true;

        assert!(make_editable_copy(&mut state));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("* generated\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n")
        );
        assert!(state.workspace.netlist_source_path.is_none());
        assert!(state.workspace.netlist_source_dirty);
        assert!(state.workspace.any_dirty());
        assert_eq!(state.ui.netlist.revision, 13);
        assert!(!state.ui.netlist.completion_open);
    }

    #[test]
    fn make_editable_copy_is_idempotent_and_never_overwrites_owned_source() {
        let mut state = crate::common::AppState::default();
        state.simulation.netlist_content = "generated\n.end\n".to_owned();
        assert!(make_editable_copy(&mut state));
        state.workspace.mark_all_clean();
        state.simulation.netlist_content = "other generated bytes\n.end\n".to_owned();
        let revision = state.ui.netlist.revision;

        assert!(!make_editable_copy(&mut state));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("generated\n.end\n")
        );
        assert!(!state.workspace.netlist_source_dirty);
        assert_eq!(state.ui.netlist.revision, revision);
    }
}

//! Hierarchical design document surface.

use egui::{Align2, Context, Id, Order, Rect, Stroke, Ui};

use crate::common::{AppState, RSpiceApp};
use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::{WorkbenchIcon, empty_state};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.active_view_read_only() {
        read_only_banner(ui, app);
    }
    let content_rect = ui.available_rect_before_wrap();
    let canvas_document = matches!(
        app.state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    );
    match app.state.workspace.active_view_type() {
        ViewType::Schematic | ViewType::Testbench => {
            crate::schematic::view::render_schematic_view(
                ui,
                &mut app.state,
                app.symbol_library.as_ref(),
            );
        }
        ViewType::Symbol => crate::schematic::symbol_editor::show(ui, &mut app.state),
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => source_document(ui, app),
        view_type => unsupported_document(ui, app, view_type),
    }
    breadcrumb(ui.ctx(), app, content_rect);
    if canvas_document {
        canvas_check_note(ui.ctx(), app, content_rect);
    }
}

fn breadcrumb(ctx: &Context, app: &RSpiceApp, content_rect: Rect) {
    let t = Tokens::get(ctx);
    let root_library = app.state.workspace.hierarchy_stack.first().map_or(
        app.state.workspace.active_view.library.as_str(),
        |reference| reference.library.as_str(),
    );
    let mut segments = Vec::with_capacity(app.state.workspace.hierarchy_stack.len() + 2);
    segments.push(root_library.to_owned());
    segments.extend(app.state.workspace.occurrence_labels());
    segments.push(app.state.workspace.active_view.view.clone());
    let mut text = egui::text::LayoutJob::default();
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            text.append(
                " / ",
                0.0,
                egui::TextFormat {
                    font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
                    color: t.color.text_faint,
                    ..Default::default()
                },
            );
        }
        let is_view = index + 1 == segments.len();
        text.append(
            segment,
            0.0,
            egui::TextFormat {
                font_id: theme::sans(
                    tokens::FS_0,
                    if is_view {
                        FontWeight::Regular
                    } else {
                        FontWeight::Medium
                    },
                ),
                color: if is_view {
                    t.color.text_dim
                } else {
                    t.color.text
                },
                ..Default::default()
            },
        );
    }
    let maximum_frame_width = (content_rect.width() * 0.5 - 16.0).max(80.0);

    egui::Area::new(Id::new("workbench.design.canvas-breadcrumb"))
        .order(Order::Middle)
        .fixed_pos(content_rect.min + egui::vec2(10.0, 9.0))
        .constrain_to(content_rect)
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(with_alpha(t.color.bg_panel, 240))
                .stroke(Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(9, 0))
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.set_max_width((maximum_frame_width - 18.0).max(62.0));
                    ui.set_min_height(27.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(text).truncate());
                    });
                });
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CheckNoteTone {
    Ok,
    Warning,
    Error,
}

fn canvas_check_note(ctx: &Context, app: &RSpiceApp, content_rect: Rect) {
    // The canonical compact portrait composition intentionally suppresses the
    // wide engineering-status note; phone landscape (844 px in the reference)
    // and larger canvases retain it.
    if ctx.content_rect().width() <= 820.0 {
        return;
    }
    let (message, tone) = check_note_content(&app.state);
    let t = Tokens::get(ctx);
    let color = match tone {
        CheckNoteTone::Ok => t.color.ok,
        CheckNoteTone::Warning => t.color.warn,
        CheckNoteTone::Error => t.color.err,
    };

    egui::Area::new(Id::new("workbench.design.canvas-check-note"))
        .order(Order::Middle)
        .pivot(Align2::RIGHT_TOP)
        .fixed_pos(content_rect.right_top() + egui::vec2(-11.0, 10.0))
        .constrain_to(content_rect)
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(with_alpha(t.color.bg_panel, 245))
                .stroke(Stroke::new(1.0, color.gamma_multiply(0.55)))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(8, 0))
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.set_max_width((content_rect.width() * 0.5 - 32.0).max(80.0));
                    ui.set_min_height(27.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 7.0;
                        let (icon_rect, _) =
                            ui.allocate_exact_size(egui::Vec2::splat(13.0), egui::Sense::hover());
                        match tone {
                            CheckNoteTone::Ok => WorkbenchIcon::Success,
                            CheckNoteTone::Warning | CheckNoteTone::Error => WorkbenchIcon::Warning,
                        }
                        .paint(ui.painter(), icon_rect, color);
                        ui.label(
                            egui::RichText::new(message)
                                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                .color(color),
                        );
                    });
                });
        });
}

fn with_alpha(color: egui::Color32, alpha: u8) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

fn check_note_content(state: &AppState) -> (String, CheckNoteTone) {
    let Some(result) = state.dialogs.drc_results.as_ref() else {
        return (
            "Schematic checks stale · run schematic checks".to_owned(),
            CheckNoteTone::Warning,
        );
    };
    if state.dialogs.drc_checked_version != state.schematic.topology_version() {
        return (
            "Schematic checks stale · run schematic checks".to_owned(),
            CheckNoteTone::Warning,
        );
    }

    let summary = result.summary();
    let blocking = summary.critical + summary.errors;
    if blocking > 0 {
        return (
            format!("{blocking} blocking schematic findings"),
            CheckNoteTone::Error,
        );
    }
    if summary.warnings > 0 {
        return (
            format!("{} schematic advisories", summary.warnings),
            CheckNoteTone::Warning,
        );
    }
    (
        "Checks and annotations current".to_owned(),
        CheckNoteTone::Ok,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_check_note_never_calls_unrun_checks_current() {
        let mut state = AppState::default();
        assert_eq!(check_note_content(&state).1, CheckNoteTone::Warning);

        state.dialogs.drc_results = Some(crate::services::drc::DrcResult::new());
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        assert_eq!(check_note_content(&state).1, CheckNoteTone::Ok);

        state.dialogs.drc_checked_version = state.schematic.topology_version().wrapping_sub(1);
        assert_eq!(check_note_content(&state).1, CheckNoteTone::Warning);
    }
}

fn read_only_banner(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.warn.gamma_multiply(0.14))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} is read only. Create an editable copy before changing this document.",
                    app.state.workspace.active_display_path()
                ))
                .color(t.color.warn),
            );
        });
}

fn source_document(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let reference = app.state.workspace.active_view.clone();
    let contents = app
        .state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .and_then(|view| view.metadata.get("source"))
        .cloned()
        .unwrap_or_default();
    if contents.is_empty() {
        empty_state(
            ui,
            super::super::design_system::WorkbenchIcon::Netlist,
            "No source text stored",
            "Import or compile this behavioral view from the Models workspace.",
        );
        return;
    }
    let mut display = contents;
    egui::Frame::new().fill(t.color.canvas_bg).show(ui, |ui| {
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::multiline(&mut display)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .interactive(false),
        );
    });
}

fn unsupported_document(ui: &mut Ui, app: &RSpiceApp, view_type: ViewType) {
    empty_state(
        ui,
        super::super::design_system::WorkbenchIcon::File,
        &format!("{} view", view_type.display_name()),
        &format!(
            "{} is registered in the project and available for downstream integrations.",
            app.state.workspace.active_display_path()
        ),
    );
}

//! Document tabs.  Engineering documents retain stable library/cell/view
//! identity; owner workspaces use one canonical task document.

use egui::{Context, Frame, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::{DOCUMENT_BAR_H, WorkbenchIcon};
use super::super::state::Workspace;

pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    let t = Tokens::get(ctx);
    TopBottomPanel::top("workbench.document_bar")
        .exact_height(DOCUMENT_BAR_H)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            egui::ScrollArea::horizontal()
                .id_salt("workbench.document_tabs.scroll")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if matches!(
                            app.state.workbench.workspace,
                            Workspace::Design | Workspace::Models
                        ) {
                            design_documents(ui, app);
                        } else {
                            owner_document(ui, app.state.workbench.workspace);
                        }
                    });
                });
        });
}

fn design_documents(ui: &mut egui::Ui, app: &mut RSpiceApp) {
    let documents = app.state.workspace.open_views.clone();
    let active = app.state.workspace.active_view.clone();
    let mut open = None;
    let mut close = None;

    for document in documents {
        let selected = document.reference == active;
        let label = format!(
            "{}{} · {}",
            document.reference.cell,
            if document.dirty { "*" } else { "" },
            document.reference.view
        );
        let response = document_tab(
            ui,
            icon_for_view(document.view_type),
            &label,
            selected,
            true,
        );
        if response.clicked() && !selected {
            open = Some(document.reference.clone());
        }
        if !selected && !document.dirty {
            let close_rect = egui::Rect::from_center_size(
                egui::Pos2::new(response.rect.right() - 13.0, response.rect.center().y),
                Vec2::splat(18.0),
            );
            let close_response =
                ui.interact(close_rect, response.id.with("close"), egui::Sense::click());
            if close_response.hovered() {
                WorkbenchIcon::Close.paint(
                    ui.painter(),
                    close_rect.shrink(4.0),
                    Tokens::get(ui.ctx()).color.text,
                );
            }
            if close_response.clicked() {
                close = Some(document.reference.clone());
            }
        }
    }

    if let Some(reference) = open {
        app.state.open_workspace_view(reference);
        app.state.workbench.workspace = Workspace::Design;
    }
    if let Some(reference) = close {
        app.state.workspace.close_view(&reference);
    }
}

fn owner_document(ui: &mut egui::Ui, workspace: Workspace) {
    let label = match workspace {
        Workspace::Project => "Project overview",
        Workspace::Simulate => "Lab characterization · simulation plan",
        Workspace::Results => "Active result document",
        Workspace::Verify => "Verification cockpit",
        Workspace::Netlist => "Generated netlist · automation",
        Workspace::Design | Workspace::Models => unreachable!(),
    };
    let _ = document_tab(ui, workspace_icon(workspace), label, true, false);
}

fn document_tab(
    ui: &mut egui::Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    closable: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let text_width = (label.chars().count() as f32 * 6.4).clamp(80.0, 240.0);
    let width = text_width + if closable { 50.0 } else { 34.0 };
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, DOCUMENT_BAR_H), egui::Sense::click());
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::Pos2::new(rect.right(), rect.top() + 2.0),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(15.0),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 29.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(
            tokens::FS_1,
            if selected {
                FontWeight::Medium
            } else {
                FontWeight::Regular
            },
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    response
}

const fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}

const fn workspace_icon(workspace: Workspace) -> WorkbenchIcon {
    match workspace {
        Workspace::Project => WorkbenchIcon::Project,
        Workspace::Design => WorkbenchIcon::Design,
        Workspace::Simulate => WorkbenchIcon::Simulate,
        Workspace::Results => WorkbenchIcon::Results,
        Workspace::Verify => WorkbenchIcon::Verify,
        Workspace::Models => WorkbenchIcon::Models,
        Workspace::Netlist => WorkbenchIcon::Netlist,
    }
}

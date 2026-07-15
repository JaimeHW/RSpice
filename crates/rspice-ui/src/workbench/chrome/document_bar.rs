//! Document tabs. Engineering documents retain stable library/cell/view
//! identity and expose the same pointer and keyboard operations as the mockup.

use egui::containers::menu::MenuButton;
use egui::{Context, Frame, Layout, Sense, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::state::{OpenCellView, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::Workspace;

const TAB_MAX_WIDTH: f32 = 190.0;
const TAB_PADDING_X: f32 = 10.0;
const TAB_ICON_SIZE: f32 = 16.0;
const TAB_CONTENT_GAP: f32 = 7.0;
const TAB_CLOSE_SIZE: f32 = 16.0;
const TAB_DIRTY_SIZE: f32 = 6.0;
const TAB_ACTIVE_EDGE: f32 = 2.0;
const OVERFLOW_WIDTH: f32 = 31.0;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let documents = visible_documents(app);
    if !document_strip_visible(documents.len()) {
        return;
    }

    let t = Tokens::get(ctx);
    TopBottomPanel::top("workbench.document_bar")
        .exact_height(layout.document_bar_height)
        .frame(Frame::new().fill(t.color.bg_app))
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.horizontal(|ui| {
                let tabs_width = (ui.available_width() - OVERFLOW_WIDTH).max(1.0);
                ui.allocate_ui_with_layout(
                    Vec2::new(tabs_width, layout.document_bar_height),
                    Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        egui::ScrollArea::horizontal()
                            .id_salt("workbench.document_tabs.scroll")
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                            )
                            .show(ui, |ui| {
                                let tabs = ui.horizontal(|ui| {
                                    design_documents(
                                        ui,
                                        app,
                                        &documents,
                                        layout.document_bar_height,
                                    );
                                });
                                ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
                                    node.set_role(egui::accesskit::Role::TabList);
                                    node.set_label("Open documents");
                                });
                            });
                    },
                );
                document_overflow(ui, app, &documents, layout.document_bar_height);
            });
        });
}

fn visible_documents(app: &RSpiceApp) -> Vec<OpenCellView> {
    if matches!(
        app.state.workbench.workspace,
        Workspace::Design | Workspace::Models
    ) {
        app.state.workspace.open_views.clone()
    } else {
        Vec::new()
    }
}

fn document_strip_visible(document_count: usize) -> bool {
    document_count > 1
}

fn design_documents(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    documents: &[OpenCellView],
    height: f32,
) {
    let active = app.state.workspace.active_view.clone();
    let mut open = None;
    let mut close = None;
    let mut focus_index = None;
    let mut tab_ids = Vec::with_capacity(documents.len());

    for (index, document) in documents.iter().enumerate() {
        let selected = document.reference == active;
        let label = format!("{} · {}", document.reference.cell, document.reference.view);
        let tab = ui
            .push_id(("document", document.reference.key()), |ui| {
                document_tab(
                    ui,
                    DocumentTabPresentation {
                        icon: icon_for_view(document.view_type),
                        label: &label,
                        selected,
                        dirty: document.dirty,
                        closable: index > 0,
                        height,
                    },
                )
            })
            .inner;
        tab_ids.push(tab.response.id);

        if tab.close_clicked {
            close = Some((index, document.reference.clone()));
        } else if tab.response.clicked() {
            focus_index = Some(index);
            if !selected {
                open = Some(document.reference.clone());
            }
        }

        if tab.response.has_focus() {
            let key = ui.input_mut(|input| {
                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                    Some(TabKey::Previous)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                    Some(TabKey::Next)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                    Some(TabKey::First)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                    Some(TabKey::Last)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Delete) {
                    Some(TabKey::Close)
                } else {
                    None
                }
            });
            match key {
                Some(TabKey::Previous) => {
                    focus_index = Some((index + documents.len() - 1) % documents.len())
                }
                Some(TabKey::Next) => focus_index = Some((index + 1) % documents.len()),
                Some(TabKey::First) => focus_index = Some(0),
                Some(TabKey::Last) => focus_index = Some(documents.len() - 1),
                Some(TabKey::Close) if index > 0 => {
                    close = Some((index, document.reference.clone()))
                }
                Some(TabKey::Close) | None => {}
            }
        }
    }

    if let Some(index) = focus_index {
        open = Some(documents[index].reference.clone());
        ui.memory_mut(|memory| memory.request_focus(tab_ids[index]));
    }
    if let Some(reference) = open {
        app.state.open_workspace_view(reference);
        app.state.workbench.activate(Workspace::Design);
    }
    if let Some((closed_index, reference)) = close {
        app.state.open_workspace_view(reference);
        Command::CloseActiveDocument.execute(app);
        let focus_after_close = if closed_index + 1 < documents.len() {
            closed_index + 1
        } else {
            closed_index.saturating_sub(1)
        };
        ui.memory_mut(|memory| memory.request_focus(tab_ids[focus_after_close]));
    }
}

#[derive(Clone, Copy)]
enum TabKey {
    Previous,
    Next,
    First,
    Last,
    Close,
}

struct DocumentTabResponse {
    response: egui::Response,
    close_clicked: bool,
}

struct DocumentTabPresentation<'a> {
    icon: WorkbenchIcon,
    label: &'a str,
    selected: bool,
    dirty: bool,
    closable: bool,
    height: f32,
}

fn document_tab(
    ui: &mut egui::Ui,
    presentation: DocumentTabPresentation<'_>,
) -> DocumentTabResponse {
    let DocumentTabPresentation {
        icon,
        label,
        selected,
        dirty,
        closable,
        height,
    } = presentation;
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, t.color.text);
    let suffix_width = suffix_width(dirty, closable);
    let width = (TAB_PADDING_X
        + TAB_ICON_SIZE
        + TAB_CONTENT_GAP
        + galley.size().x
        + suffix_width
        + TAB_PADDING_X)
        .clamp(72.0, TAB_MAX_WIDTH);
    let sense = if selected {
        Sense::click()
    } else {
        Sense::click().difference(Sense::focusable_noninteractive())
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
        let description = match (dirty, closable) {
            (true, true) => "Unsaved changes. Press Delete to close.",
            (true, false) => "Unsaved changes.",
            (false, true) => "Press Delete to close.",
            (false, false) => "Pinned primary document.",
        };
        node.set_description(description);
    });

    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.right(), rect.top() + TAB_ACTIVE_EDGE),
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

    let icon_center_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE * 0.5;
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(icon_center_x, rect.center().y),
            Vec2::splat(TAB_ICON_SIZE),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let label_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE + TAB_CONTENT_GAP;
    let suffix_left = rect.right() - TAB_PADDING_X - suffix_width;
    let label_clip = egui::Rect::from_min_max(
        egui::pos2(label_x, rect.top()),
        egui::pos2(suffix_left.max(label_x), rect.bottom()),
    );
    ui.painter().with_clip_rect(label_clip).galley(
        egui::pos2(label_x, rect.center().y - galley.size().y * 0.5),
        galley,
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );

    let mut suffix_x = suffix_left;
    if dirty {
        let center = egui::pos2(suffix_x + TAB_DIRTY_SIZE * 0.5, rect.center().y);
        ui.painter()
            .circle_filled(center, TAB_DIRTY_SIZE * 0.5, t.color.warn);
        suffix_x += TAB_DIRTY_SIZE + TAB_CONTENT_GAP;
    }
    let mut close_clicked = false;
    if closable {
        let close_rect = egui::Rect::from_min_size(
            egui::pos2(suffix_x, rect.center().y - TAB_CLOSE_SIZE * 0.5),
            Vec2::splat(TAB_CLOSE_SIZE),
        );
        let close_response = ui.interact(
            close_rect,
            response.id.with("close"),
            Sense::click().difference(Sense::focusable_noninteractive()),
        );
        if close_response.hovered() {
            ui.painter().rect_filled(close_rect, 2.0, t.color.bg_active);
        }
        WorkbenchIcon::Close.paint(
            ui.painter(),
            close_rect.shrink(3.0),
            if close_response.hovered() {
                t.color.text
            } else {
                t.color.text_faint
            },
        );
        close_clicked = close_response.clicked();
    }
    theme::paint_focus_ring(ui, &response, rect);
    DocumentTabResponse {
        response,
        close_clicked,
    }
}

fn suffix_width(dirty: bool, closable: bool) -> f32 {
    match (dirty, closable) {
        (false, false) => 0.0,
        (true, false) => TAB_DIRTY_SIZE,
        (false, true) => TAB_CLOSE_SIZE,
        (true, true) => TAB_DIRTY_SIZE + TAB_CONTENT_GAP + TAB_CLOSE_SIZE,
    }
}

fn document_overflow(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    documents: &[OpenCellView],
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let (response, _) = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            MenuButton::from_button(
                egui::Button::new("")
                    .frame(false)
                    .min_size(Vec2::new(OVERFLOW_WIDTH, height)),
            )
            .ui(ui, |ui| {
                ui.set_min_width(220.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                let active = app.state.workspace.active_view.clone();
                for document in documents {
                    let label =
                        format!("{} · {}", document.reference.cell, document.reference.view);
                    if ui
                        .add_sized(
                            [220.0, if height >= 44.0 { 44.0 } else { 29.0 }],
                            egui::Button::selectable(document.reference == active, label),
                        )
                        .clicked()
                    {
                        app.state.open_workspace_view(document.reference.clone());
                        app.state.workbench.activate(Workspace::Design);
                        ui.close();
                    }
                }
            })
        })
        .inner;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Manage open documents",
        )
    });
    ui.painter().rect_filled(
        response.rect,
        0.0,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_app
        },
    );
    ui.painter().vline(
        response.rect.left(),
        response.rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), Vec2::splat(13.0)),
        if response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_text("Manage open documents");
}

const fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_document_does_not_consume_a_document_strip() {
        assert!(!document_strip_visible(0));
        assert!(!document_strip_visible(1));
        assert!(document_strip_visible(2));
    }

    #[test]
    fn document_tab_geometry_matches_the_mockup() {
        assert_eq!(TAB_MAX_WIDTH, 190.0);
        assert_eq!(TAB_PADDING_X, 10.0);
        assert_eq!(TAB_ICON_SIZE, 16.0);
        assert_eq!(TAB_CONTENT_GAP, 7.0);
        assert_eq!(TAB_CLOSE_SIZE, 16.0);
        assert_eq!(TAB_DIRTY_SIZE, 6.0);
        assert_eq!(OVERFLOW_WIDTH, 31.0);
        assert_eq!(tokens::FS_0, 11.0);
    }

    #[test]
    fn dirty_and_close_suffixes_never_overlap() {
        assert_eq!(suffix_width(false, false), 0.0);
        assert_eq!(suffix_width(true, false), 6.0);
        assert_eq!(suffix_width(false, true), 16.0);
        assert_eq!(suffix_width(true, true), 29.0);
    }
}

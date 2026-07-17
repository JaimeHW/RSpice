//! Project-owned engineering report authoring.
//!
//! The surface authors and saves the project-owned, versioned
//! [`ReportDocument`] graph. Route availability remains fail-closed until the
//! complete report workflow is ready for production use.

use egui::{Align2, Color32, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::common::{AppState, RSpiceApp};
use crate::results::report_document::{
    ReportDocument, ReportEdit, ReportEntityRef, ReportPageId, ReportPageUpdatePolicy,
    ReportTemplate,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, input_row, select};

use super::super::commands::Command;
use super::super::design_system::{
    WorkbenchIcon, code_inspector_property_list, code_inspector_section, code_workspace_heading,
    icon_button, property_row, workspace_title_row,
};
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const DESKTOP_BREAKPOINT: f32 = 1_020.0;
const STACK_BREAKPOINT: f32 = 820.0;
const OUTLINE_DESKTOP_WIDTH: f32 = 220.0;
const OUTLINE_TABLET_WIDTH: f32 = 180.0;
const INSPECTOR_WIDTH: f32 = 280.0;
const PANEL_GAP: f32 = 0.0;
const OUTLINE_HEADER_HEIGHT: f32 = 39.0;
const OUTLINE_ROW_HEIGHT: f32 = 34.0;
const PREVIEW_MIN_HEIGHT: f32 = 420.0;
const INSPECTOR_SECTION_HEIGHT: f32 = 29.0;
const INSPECTOR_ROW_HEIGHT: f32 = 29.0;
const INSPECTOR_SECTION_PADDING: f32 = 17.0;

const PAPER: Color32 = Color32::from_rgb(255, 255, 255);
const PAPER_PANEL: Color32 = Color32::from_rgb(246, 247, 247);
const PAPER_TEXT: Color32 = Color32::from_rgb(32, 36, 40);
const PAPER_MUTED: Color32 = Color32::from_rgb(82, 89, 94);
const PAPER_FAINT: Color32 = Color32::from_rgb(98, 105, 110);
const PAPER_BORDER: Color32 = Color32::from_rgb(205, 210, 213);
const PAPER_ACCENT: Color32 = Color32::from_rgb(122, 93, 0);

const INITIAL_PAGES: [(&str, &str); 7] = [
    ("1", "Executive summary"),
    ("2", "Design and configuration"),
    ("3", "Nominal results"),
    ("4", "PVT and yield"),
    ("5", "Reliability and regression"),
    ("6", "Physical DRC and waivers"),
    ("A", "Run manifests"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComposerLayout {
    ThreeColumn,
    TwoColumnInspectorBelow,
    Stacked,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct PaneSeparators {
    top: bool,
    right: bool,
    bottom: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ComposerPaneHeights {
    outline: f32,
    preview: f32,
    inspector: f32,
}

impl ComposerLayout {
    fn resolve(width: f32) -> Self {
        if width > DESKTOP_BREAKPOINT {
            Self::ThreeColumn
        } else if width > STACK_BREAKPOINT {
            Self::TwoColumnInspectorBelow
        } else {
            Self::Stacked
        }
    }

    fn separators(self) -> [PaneSeparators; 3] {
        match self {
            Self::ThreeColumn => [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ],
            Self::TwoColumnInspectorBelow => [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
                PaneSeparators {
                    top: true,
                    ..PaneSeparators::default()
                },
            ],
            Self::Stacked => [
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ],
        }
    }
}

fn composer_pane_heights(
    layout: ComposerLayout,
    available_height: f32,
    page_count: usize,
    page_selected: bool,
) -> ComposerPaneHeights {
    let viewport_height = if available_height.is_finite() {
        available_height.max(1.0)
    } else {
        PREVIEW_MIN_HEIGHT
    };
    let outline_content = OUTLINE_HEADER_HEIGHT + OUTLINE_ROW_HEIGHT * page_count as f32;
    let inspector_sections = if page_selected { 2.0 } else { 1.0 };
    let inspector_content = inspector_sections
        * (INSPECTOR_SECTION_HEIGHT + INSPECTOR_ROW_HEIGHT * 5.0 + INSPECTOR_SECTION_PADDING);

    match layout {
        ComposerLayout::ThreeColumn => ComposerPaneHeights {
            outline: viewport_height,
            preview: viewport_height,
            inspector: viewport_height,
        },
        ComposerLayout::TwoColumnInspectorBelow => {
            let top_content = outline_content.max(PREVIEW_MIN_HEIGHT);
            let base_height = top_content + inspector_content;
            let surplus = (viewport_height - base_height).max(0.0);
            ComposerPaneHeights {
                outline: top_content + surplus * 0.65,
                preview: top_content + surplus * 0.65,
                inspector: inspector_content + surplus * 0.35,
            }
        }
        ComposerLayout::Stacked => {
            let base_height = outline_content + PREVIEW_MIN_HEIGHT + inspector_content;
            let surplus = (viewport_height - base_height).max(0.0);
            ComposerPaneHeights {
                outline: outline_content,
                preview: PREVIEW_MIN_HEIGHT + surplus * 0.70,
                inspector: inspector_content + surplus * 0.30,
            }
        }
    }
}

pub(crate) fn open(app: &mut RSpiceApp) {
    let route = SurfaceRoute::surface(SurfaceId::ReportAuthoring);
    if let Err(error) = app
        .state
        .workbench
        .navigate(route, RouteTransitionSource::User)
    {
        app.state
            .push_user_message(crate::common::app::ConsoleMessage::warning(
                error.to_string(),
            ));
    }
}

pub(crate) fn save_document(app: &mut RSpiceApp) {
    let invalid = app
        .state
        .workspace
        .report_documents
        .iter()
        .find_map(|document| document.validate().err());
    if let Some(error) = invalid {
        app.state
            .push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                "Report document save was blocked before publication: {error}"
            )));
        return;
    }
    Command::Save.execute(app);
}

pub(crate) fn open_add_page(app: &mut RSpiceApp) {
    if active_document(&app.state).is_none() {
        return;
    }
    let next =
        active_document(&app.state).map_or(1, |document| document.pages().len().saturating_add(1));
    let editor = &mut app.state.workbench.report_authoring;
    editor.add_page_title = format!("Report page {next}");
    editor.transaction_error = None;
    editor.add_page_open = true;
}

pub(crate) fn open_page_properties(app: &mut RSpiceApp) {
    let Some(document) = active_document(&app.state) else {
        return;
    };
    let Some(page) = selected_page_id(&app.state, document).and_then(|id| document.page(id)) else {
        return;
    };
    let page_id = page.id();
    let title = page.title().to_owned();
    let template = report_template_index(document.template());
    let update_policy = page_update_policy_index(page.update_policy());
    let editor = &mut app.state.workbench.report_authoring;
    editor.page_properties_page = Some(page_id);
    editor.page_title_draft = title;
    editor.report_template_draft = template;
    editor.page_update_policy_draft = update_policy;
    editor.transaction_error = None;
    editor.page_properties_open = true;
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        ui.set_width(ui.available_width());

        if !app.state.project_lifecycle.project_open {
            workspace_title_row(ui, |ui| {
                code_workspace_heading(
                    ui,
                    "REPORT AUTHORING · NO OPEN PROJECT",
                    "Engineering report composer",
                    "Open a project before creating a project-owned report document.",
                );
            });
            return;
        }

        if let Err(error) = ensure_report_document(&mut app.state) {
            workspace_title_row(ui, |ui| {
                code_workspace_heading(
                    ui,
                    "REPORT AUTHORING · DOCUMENT ERROR",
                    "Engineering report composer",
                    &error,
                );
            });
            return;
        }

        workspace_title_row(ui, |ui| {
            code_workspace_heading(
                ui,
                "REPORT AUTHORING · ENGINEERING DRAFT",
                "Engineering report composer",
                "Author and save the versioned report document, its page order, and page properties.",
            );
        });

        let Some(document) = active_document(&app.state).cloned() else {
            return;
        };
        let selected_page = selected_page_id(&app.state, &document);
        let available = ui.available_size();
        let layout = ComposerLayout::resolve(available.x);
        let heights = composer_pane_heights(
            layout,
            available.y,
            document.pages().len(),
            selected_page.is_some(),
        );
        let [outline_separators, preview_separators, inspector_separators] = layout.separators();
        match layout {
            ComposerLayout::ThreeColumn => {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = PANEL_GAP;
                    ui.allocate_ui_with_layout(
                        Vec2::new(OUTLINE_DESKTOP_WIDTH, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            outline(
                                ui,
                                app,
                                &document,
                                selected_page,
                                outline_separators,
                            )
                        },
                    );
                    let preview_width = (available.x
                        - OUTLINE_DESKTOP_WIDTH
                        - INSPECTOR_WIDTH
                        - PANEL_GAP * 2.0)
                        .max(1.0);
                    ui.allocate_ui_with_layout(
                        Vec2::new(preview_width, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| preview(ui, &document, selected_page, preview_separators),
                    );
                    ui.allocate_ui_with_layout(
                        Vec2::new(INSPECTOR_WIDTH, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| inspector(ui, &document, selected_page, inspector_separators),
                    );
                });
            }
            ComposerLayout::TwoColumnInspectorBelow => {
                ScrollArea::vertical()
                    .id_salt("report-authoring.tablet")
                    .show(ui, |ui| {
                        let local_width = ui.available_width().max(1.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = PANEL_GAP;
                            ui.allocate_ui_with_layout(
                                Vec2::new(OUTLINE_TABLET_WIDTH, heights.outline),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    outline(
                                        ui,
                                        app,
                                        &document,
                                        selected_page,
                                        outline_separators,
                                    )
                                },
                            );
                            ui.allocate_ui_with_layout(
                                Vec2::new(
                                    (local_width - OUTLINE_TABLET_WIDTH).max(1.0),
                                    heights.preview,
                                ),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    preview(
                                        ui,
                                        &document,
                                        selected_page,
                                        preview_separators,
                                    )
                                },
                            );
                        });
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.inspector),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                inspector(
                                    ui,
                                    &document,
                                    selected_page,
                                    inspector_separators,
                                )
                            },
                        );
                    });
            }
            ComposerLayout::Stacked => {
                ScrollArea::vertical()
                    .id_salt("report-authoring.compact")
                    .show(ui, |ui| {
                        let local_width = ui.available_width().max(1.0);
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.outline),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                outline(
                                    ui,
                                    app,
                                    &document,
                                    selected_page,
                                    outline_separators,
                                )
                            },
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.preview),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                preview(
                                    ui,
                                    &document,
                                    selected_page,
                                    preview_separators,
                                )
                            },
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.inspector),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                inspector(
                                    ui,
                                    &document,
                                    selected_page,
                                    inspector_separators,
                                )
                            },
                        );
                    });
            }
        }
    });

    add_page_dialog(ui.ctx(), app);
    page_properties_dialog(ui.ctx(), app);
}

fn outline(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
        let (head, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 39.0), Sense::hover());
        ui.painter().hline(
            head.x_range(),
            head.bottom(),
            Stroke::new(1.0, t.color.border),
        );
        ui.painter().text(
            head.left_center() + Vec2::new(10.0, 0.0),
            Align2::LEFT_CENTER,
            "Report outline",
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
        let button_rect = Rect::from_center_size(
            head.right_center() - Vec2::new(19.5, 0.0),
            Vec2::new(29.0, 29.0),
        );
        let mut properties = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(button_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        if icon_button(
            &mut properties,
            WorkbenchIcon::Sliders,
            "Page properties",
            false,
            Vec2::new(29.0, 29.0),
        )
        .clicked()
        {
            open_page_properties(app);
        }

        ScrollArea::vertical()
            .id_salt("report-authoring.outline")
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for (index, page) in document.pages().iter().enumerate() {
                    let marker = page_marker(index, page.title());
                    let selected = Some(page.id()) == selected_page;
                    if outline_row(ui, marker, page.title(), selected).clicked() {
                        app.state.workbench.report_authoring.selected_page = Some(page.id());
                    }
                }
            });
    });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn outline_row(ui: &mut Ui, marker: &str, label: &str, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), selected, label)
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            Rect::from_min_max(rect.left_top(), rect.left_bottom() + Vec2::new(2.0, 0.0)),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().text(
        rect.left_center() + Vec2::new(10.0, 0.0),
        Align2::LEFT_CENTER,
        marker,
        theme::mono(tokens::FS_1, FontWeight::Medium),
        if selected {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        rect.left_center() + Vec2::new(37.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn preview(
    ui: &mut Ui,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new()
        .fill(PAPER_PANEL)
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
            ScrollArea::vertical()
                .id_salt("report-authoring.preview")
                .show(ui, |ui| {
                    let compact = ui.available_width() < 560.0;
                    let horizontal_margin = if compact { 18.0 } else { 42.0 };
                    let top_margin = if compact { 24.0 } else { 36.0 };
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: horizontal_margin as i8,
                            right: horizontal_margin as i8,
                            top: top_margin as i8,
                            bottom: 36,
                        })
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            paper_label(
                                ui,
                                "PROJECT REPORT DOCUMENT",
                                theme::mono(tokens::FS_0, FontWeight::Medium),
                                PAPER_MUTED,
                            );
                            ui.add_space(8.0);
                            paper_label(
                                ui,
                                document.title(),
                                theme::sans(26.0, FontWeight::SemiBold),
                                PAPER_TEXT,
                            );
                            ui.add_space(4.0);
                            paper_label(
                                ui,
                                &format!(
                                    "Document revision {} · {}",
                                    document.revision().get(),
                                    report_template_label(document.template()),
                                ),
                                theme::sans(tokens::FS_1, FontWeight::Regular),
                                PAPER_MUTED,
                            );

                            let page = selected_page.and_then(|id| document.page(id));
                            let page_index = page.and_then(|page| {
                                document
                                    .pages()
                                    .iter()
                                    .position(|candidate| candidate.id() == page.id())
                            });
                            let marker = page_index
                                .map(|index| page_marker(index, page.map_or("", |p| p.title())))
                                .unwrap_or("—");
                            let title = page.map_or("No report page selected", |page| page.title());
                            let description = page.map_or_else(
                                || "Select a page from the report outline.".to_owned(),
                                |page| {
                                    format!(
                                        "Page revision {} · {}",
                                        page.revision().get(),
                                        page_update_policy_label(page.update_policy())
                                    )
                                },
                            );
                            ui.add_space(24.0);
                            section_heading(ui, marker, title, &description);
                            ui.add_space(28.0);
                            summary_grid(ui, document, page, compact);
                            ui.add_space(24.0);
                            paper_label(
                                ui,
                                "Document source",
                                theme::sans(tokens::FS_3, FontWeight::SemiBold),
                                PAPER_TEXT,
                            );
                            ui.add_space(7.0);
                            paper_label(
                                ui,
                                "This source is the canonical project-owned ReportDocument. Page and document changes are applied as validated, revision-checked transactions and persisted with the project.",
                                theme::sans(tokens::FS_1, FontWeight::Regular),
                                PAPER_MUTED,
                            );
                        });
                });
        });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn section_heading(ui: &mut Ui, marker: &str, title: &str, description: &str) {
    let (line, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        line.x_range(),
        line.center().y,
        Stroke::new(1.0, PAPER_BORDER),
    );
    ui.add_space(13.0);
    ui.horizontal_top(|ui| {
        ui.set_width(ui.available_width());
        ui.allocate_ui_with_layout(
            Vec2::new(34.0, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                paper_label(
                    ui,
                    marker,
                    theme::mono(tokens::FS_2, FontWeight::SemiBold),
                    PAPER_ACCENT,
                );
            },
        );
        ui.add_space(10.0);
        ui.vertical(|ui| {
            paper_label(
                ui,
                title,
                theme::sans(15.0, FontWeight::SemiBold),
                PAPER_TEXT,
            );
            ui.add_space(4.0);
            paper_label(
                ui,
                description,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                PAPER_MUTED,
            );
        });
    });
}

fn summary_grid(
    ui: &mut Ui,
    document: &ReportDocument,
    page: Option<&crate::results::report_document::ReportPage>,
    compact: bool,
) {
    let page_sections = page.map_or(0, |page| page.sections().len());
    let page_blocks = page.map_or(0, |page| {
        page.sections()
            .iter()
            .map(|section| section.blocks().len())
            .sum()
    });
    let cells = [
        (document.pages().len().to_string(), "document pages"),
        (page_sections.to_string(), "selected-page sections"),
        (page_blocks.to_string(), "selected-page blocks"),
    ];
    if compact {
        for (value, label) in cells {
            summary_cell(ui, &value, label);
            ui.add_space(8.0);
        }
    } else {
        let width = ui.available_width();
        let cell_width = ((width - 16.0) / 3.0).max(1.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for (value, label) in cells {
                ui.allocate_ui_with_layout(
                    Vec2::new(cell_width, 76.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| summary_cell(ui, &value, label),
                );
            }
        });
    }
}

fn summary_cell(ui: &mut Ui, value: &str, label: &str) {
    egui::Frame::new()
        .fill(PAPER)
        .stroke(Stroke::new(1.0, PAPER_BORDER))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.set_min_width((ui.available_width() - 28.0).max(1.0));
            paper_label(
                ui,
                value,
                theme::sans(20.0, FontWeight::SemiBold),
                PAPER_TEXT,
            );
            ui.add_space(4.0);
            paper_label(
                ui,
                label,
                theme::sans(tokens::FS_0, FontWeight::Regular),
                PAPER_FAINT,
            );
        });
}

fn paper_label(ui: &mut Ui, text: &str, font: egui::FontId, color: Color32) {
    ui.add(
        egui::Label::new(egui::RichText::new(text).font(font).color(color))
            .wrap()
            .selectable(true),
    );
}

fn inspector(
    ui: &mut Ui,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
        ScrollArea::vertical()
            .id_salt("report-authoring.inspector")
            .show(ui, |ui| {
                code_inspector_section(ui, "Report document", None, |ui| {
                    code_inspector_property_list(ui, |ui| {
                        property_row(ui, "Title", document.title());
                        property_row(ui, "Identity", &document.id().to_string());
                        property_row(ui, "Revision", &document.revision().get().to_string());
                        property_row(ui, "Template", report_template_label(document.template()));
                        property_row(ui, "Pages", &document.pages().len().to_string());
                    });
                });
                if let Some(page) = selected_page.and_then(|id| document.page(id)) {
                    code_inspector_section(ui, "Selected page", None, |ui| {
                        code_inspector_property_list(ui, |ui| {
                            property_row(ui, "Title", page.title());
                            property_row(ui, "Identity", &page.id().to_string());
                            property_row(ui, "Revision", &page.revision().get().to_string());
                            property_row(
                                ui,
                                "Update policy",
                                page_update_policy_label(page.update_policy()),
                            );
                            property_row(ui, "Sections", &page.sections().len().to_string());
                        });
                    });
                }
            });
    });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn paint_pane_separators(ui: &Ui, rect: Rect, separators: PaneSeparators, color: Color32) {
    let stroke = Stroke::new(1.0, color);
    if separators.top {
        let y = rect.top() + 0.5;
        ui.painter().line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
    if separators.right {
        let x = rect.right() - 0.5;
        ui.painter().line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            stroke,
        );
    }
    if separators.bottom {
        let y = rect.bottom() - 0.5;
        ui.painter().line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
}

fn add_page_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.report_authoring.add_page_open {
        return;
    }
    let valid = valid_page_title(&app.state.workbench.report_authoring.add_page_title);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let choice = Dialog::new(
        "REPORTING · DOCUMENT COMPOSITION",
        "Add report page",
        "Add page",
    )
    .description("Add one versioned page to the project-owned report document.")
    .ghost("Cancel")
    .primary_enabled(valid)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let response = input_row(
            ui,
            "Page title",
            &mut app.state.workbench.report_authoring.add_page_title,
        );
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid => commit_add_page(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.report_authoring.add_page_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn commit_add_page(app: &mut RSpiceApp) {
    let title = app
        .state
        .workbench
        .report_authoring
        .add_page_title
        .trim()
        .to_owned();
    let timestamp = timestamp_unix_ms();
    let result = active_document_mut(&mut app.state).and_then(|document| {
        document
            .transact(
                document.revision(),
                vec![ReportEdit::AddPage { title }],
                timestamp,
            )
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(receipt) => {
            let created_page = receipt.created.iter().find_map(|entity| match entity {
                ReportEntityRef::Page(id) => Some(*id),
                _ => None,
            });
            app.state.workbench.report_authoring.selected_page = created_page;
            app.state.workbench.report_authoring.add_page_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty = true;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn page_properties_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.report_authoring.page_properties_open {
        return;
    }
    let valid = valid_page_title(&app.state.workbench.report_authoring.page_title_draft);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let Some(document) = active_document(&app.state).cloned() else {
        app.state.workbench.report_authoring.page_properties_open = false;
        return;
    };
    let page_options = document
        .pages()
        .iter()
        .enumerate()
        .map(|(index, page)| format!("{} · {}", page_marker(index, page.title()), page.title()))
        .collect::<Vec<_>>();
    let page_index = app
        .state
        .workbench
        .report_authoring
        .page_properties_page
        .and_then(|page_id| {
            document
                .pages()
                .iter()
                .position(|page| page.id() == page_id)
        })
        .unwrap_or_default();
    const TEMPLATE_LABELS: [&str; 3] = [
        "Release verification 4.2",
        "Design review",
        "Model qualification",
    ];
    const UPDATE_POLICY_LABELS: [&str; 2] = [
        "Refresh linked figures automatically",
        "Freeze selected figure revision",
    ];
    let choice = Dialog::new(
        "REPORTING · DOCUMENT COMPOSITION",
        "Report page properties",
        "Save page properties",
    )
    .description("Edit the selected page through one revision-checked report transaction.")
    .ghost("Cancel")
    .primary_enabled(valid)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 130.0_f32.min(ui.available_width() * 0.32);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Template"),
            );
            let selected_index = app
                .state
                .workbench
                .report_authoring
                .report_template_draft
                .min(TEMPLATE_LABELS.len() - 1);
            let options = TEMPLATE_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-page-template",
                "Report template",
                TEMPLATE_LABELS[selected_index],
                &options,
                ui.available_width(),
            ) {
                app.state.workbench.report_authoring.report_template_draft = index;
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Page"),
            );
            if let Some(index) = select(
                ui,
                "report-page-selection",
                "Report page",
                page_options
                    .get(page_index)
                    .map_or("No page", String::as_str),
                &page_options,
                ui.available_width(),
            ) && let Some(page) = document.pages().get(index)
            {
                let editor = &mut app.state.workbench.report_authoring;
                editor.page_properties_page = Some(page.id());
                editor.page_title_draft = page.title().to_owned();
                editor.page_update_policy_draft = page_update_policy_index(page.update_policy());
                editor.transaction_error = None;
            }
        });
        ui.add_space(8.0);
        let response = input_row(
            ui,
            "Page title",
            &mut app.state.workbench.report_authoring.page_title_draft,
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Update policy"),
            );
            let selected_index = app
                .state
                .workbench
                .report_authoring
                .page_update_policy_draft
                .min(UPDATE_POLICY_LABELS.len() - 1);
            let options = UPDATE_POLICY_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-page-update-policy",
                "Report page update policy",
                UPDATE_POLICY_LABELS[selected_index],
                &options,
                ui.available_width(),
            ) {
                app.state
                    .workbench
                    .report_authoring
                    .page_update_policy_draft = index;
            }
        });
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid => commit_page_properties(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.report_authoring.page_properties_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn commit_page_properties(app: &mut RSpiceApp) {
    let Some(page_id) = app.state.workbench.report_authoring.page_properties_page else {
        app.state.workbench.report_authoring.transaction_error =
            Some("The selected report page no longer exists.".to_owned());
        return;
    };
    let title = app
        .state
        .workbench
        .report_authoring
        .page_title_draft
        .trim()
        .to_owned();
    let timestamp = timestamp_unix_ms();
    let template =
        report_template_from_index(app.state.workbench.report_authoring.report_template_draft);
    let update_policy = page_update_policy_from_index(
        app.state
            .workbench
            .report_authoring
            .page_update_policy_draft,
    );
    let result = active_document_mut(&mut app.state).and_then(|document| {
        let page = document
            .page(page_id)
            .ok_or_else(|| "The selected report page no longer exists.".to_owned())?;
        let mut edits = Vec::with_capacity(3);
        if document.template() != template {
            edits.push(ReportEdit::SetTemplate { template });
        }
        let mut expected_page_revision = page.revision();
        if page.title() != title {
            edits.push(ReportEdit::UpdatePageTitle {
                page_id,
                expected_page_revision,
                title,
            });
            expected_page_revision = expected_page_revision
                .next()
                .map_err(|error| error.to_string())?;
        }
        if page.update_policy() != update_policy {
            edits.push(ReportEdit::SetPageUpdatePolicy {
                page_id,
                expected_page_revision,
                update_policy,
            });
        }
        if edits.is_empty() {
            return Ok(false);
        }
        document
            .transact(document.revision(), edits, timestamp)
            .map(|_| true)
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(changed) => {
            app.state.workbench.report_authoring.selected_page = Some(page_id);
            app.state.workbench.report_authoring.page_properties_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty |= changed;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn ensure_report_document(state: &mut AppState) -> Result<(), String> {
    let selected_is_valid = state
        .workbench
        .report_authoring
        .selected_document
        .is_some_and(|id| {
            state
                .workspace
                .report_documents
                .iter()
                .any(|doc| doc.id() == id)
        });
    if !selected_is_valid {
        state.workbench.report_authoring.selected_document = state
            .workspace
            .report_documents
            .first()
            .map(ReportDocument::id);
    }
    if state.workspace.report_documents.is_empty() {
        let mut document = ReportDocument::new("Verification report")
            .map_err(|error| format!("A report document could not be created: {error}"))?;
        let edits = INITIAL_PAGES
            .iter()
            .map(|(_, title)| ReportEdit::AddPage {
                title: (*title).to_owned(),
            })
            .collect();
        document
            .transact(document.revision(), edits, timestamp_unix_ms())
            .map_err(|error| format!("The report outline could not be created: {error}"))?;
        let document_id = document.id();
        let page_id = document.pages().first().map(|page| page.id());
        state.workspace.report_documents.push(document);
        state.workspace.report_documents_dirty = true;
        state.workbench.report_authoring.selected_document = Some(document_id);
        state.workbench.report_authoring.selected_page = page_id;
    }

    let Some(document) = active_document(state) else {
        return Err("No project-owned report document is selected.".to_owned());
    };
    let page_valid = state
        .workbench
        .report_authoring
        .selected_page
        .is_some_and(|id| document.page(id).is_some());
    if !page_valid {
        state.workbench.report_authoring.selected_page =
            document.pages().first().map(|page| page.id());
    }
    Ok(())
}

fn active_document(state: &AppState) -> Option<&ReportDocument> {
    let id = state.workbench.report_authoring.selected_document?;
    state
        .workspace
        .report_documents
        .iter()
        .find(|document| document.id() == id)
}

fn active_document_mut(state: &mut AppState) -> Result<&mut ReportDocument, String> {
    let id = state
        .workbench
        .report_authoring
        .selected_document
        .ok_or_else(|| "No report document is selected.".to_owned())?;
    state
        .workspace
        .report_documents
        .iter_mut()
        .find(|document| document.id() == id)
        .ok_or_else(|| "The selected report document no longer exists.".to_owned())
}

fn selected_page_id(state: &AppState, document: &ReportDocument) -> Option<ReportPageId> {
    state
        .workbench
        .report_authoring
        .selected_page
        .filter(|id| document.page(*id).is_some())
        .or_else(|| document.pages().first().map(|page| page.id()))
}

fn valid_page_title(title: &str) -> bool {
    let trimmed = title.trim();
    !trimmed.is_empty()
        && trimmed == title
        && trimmed.len() <= 512
        && !trimmed.chars().any(char::is_control)
}

fn report_template_index(template: ReportTemplate) -> usize {
    match template {
        ReportTemplate::ReleaseVerification42 => 0,
        ReportTemplate::DesignReview => 1,
        ReportTemplate::ModelQualification => 2,
    }
}

fn report_template_from_index(index: usize) -> ReportTemplate {
    match index {
        1 => ReportTemplate::DesignReview,
        2 => ReportTemplate::ModelQualification,
        _ => ReportTemplate::ReleaseVerification42,
    }
}

fn report_template_label(template: ReportTemplate) -> &'static str {
    match template {
        ReportTemplate::ReleaseVerification42 => "Release verification 4.2",
        ReportTemplate::DesignReview => "Design review",
        ReportTemplate::ModelQualification => "Model qualification",
    }
}

fn page_update_policy_index(policy: ReportPageUpdatePolicy) -> usize {
    match policy {
        ReportPageUpdatePolicy::RefreshLinkedAutomatically => 0,
        ReportPageUpdatePolicy::FreezeSelectedRevision => 1,
    }
}

fn page_update_policy_from_index(index: usize) -> ReportPageUpdatePolicy {
    match index {
        1 => ReportPageUpdatePolicy::FreezeSelectedRevision,
        _ => ReportPageUpdatePolicy::RefreshLinkedAutomatically,
    }
}

fn page_update_policy_label(policy: ReportPageUpdatePolicy) -> &'static str {
    match policy {
        ReportPageUpdatePolicy::RefreshLinkedAutomatically => "Refresh linked automatically",
        ReportPageUpdatePolicy::FreezeSelectedRevision => "Freeze selected revision",
    }
}

fn page_marker(index: usize, title: &str) -> &str {
    INITIAL_PAGES
        .get(index)
        .filter(|(_, expected)| *expected == title)
        .map_or("+", |(marker, _)| *marker)
}

fn timestamp_unix_ms() -> u64 {
    u64::try_from(crate::common::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_report_builder_matches_mockup_breakpoints() {
        assert_eq!(
            ComposerLayout::resolve(1_280.0),
            ComposerLayout::ThreeColumn
        );
        assert_eq!(
            ComposerLayout::resolve(1_020.0),
            ComposerLayout::TwoColumnInspectorBelow
        );
        assert_eq!(
            ComposerLayout::resolve(821.0),
            ComposerLayout::TwoColumnInspectorBelow
        );
        assert_eq!(ComposerLayout::resolve(820.0), ComposerLayout::Stacked);
        assert_eq!(ComposerLayout::resolve(390.0), ComposerLayout::Stacked);
    }

    #[test]
    fn every_report_layout_assigns_each_internal_seam_to_one_pane() {
        assert_eq!(
            ComposerLayout::ThreeColumn.separators(),
            [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ]
        );
        assert_eq!(
            ComposerLayout::TwoColumnInspectorBelow.separators(),
            [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
                PaneSeparators {
                    top: true,
                    ..PaneSeparators::default()
                },
            ]
        );
        assert_eq!(
            ComposerLayout::Stacked.separators(),
            [
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ]
        );
    }

    #[test]
    fn tablet_and_stacked_pane_heights_follow_local_space_and_document_content() {
        let tablet_short = composer_pane_heights(
            ComposerLayout::TwoColumnInspectorBelow,
            640.0,
            INITIAL_PAGES.len(),
            true,
        );
        let tablet_tall = composer_pane_heights(
            ComposerLayout::TwoColumnInspectorBelow,
            1_000.0,
            INITIAL_PAGES.len(),
            true,
        );
        assert!(tablet_tall.preview > tablet_short.preview);
        assert!(tablet_tall.inspector > tablet_short.inspector);
        assert!(tablet_short.preview + tablet_short.inspector + 0.01 >= 640.0);
        assert!(tablet_tall.preview + tablet_tall.inspector + 0.01 >= 1_000.0);

        let compact_seven =
            composer_pane_heights(ComposerLayout::Stacked, 720.0, INITIAL_PAGES.len(), true);
        let compact_twelve = composer_pane_heights(ComposerLayout::Stacked, 720.0, 12, true);
        assert!(compact_twelve.outline > compact_seven.outline);
        assert_eq!(compact_seven.preview, PREVIEW_MIN_HEIGHT);
        assert!(compact_seven.inspector > 300.0);
    }

    #[test]
    fn incomplete_publication_contract_keeps_surface_and_commands_unexposed() {
        assert!(!crate::workbench::surface_availability(SurfaceId::ReportAuthoring).can_open());
        for command in [
            Command::ReportAuthoring,
            Command::SaveReportDocument,
            Command::AddReportPage,
            Command::ReportPageProperties,
        ] {
            assert!(!crate::workbench::commands::COMMAND_REGISTRY.contains(&command));
        }
    }

    #[test]
    fn opening_report_authoring_creates_the_exact_mockup_outline() {
        let mut state = AppState::default();
        ensure_report_document(&mut state).expect("report scaffold");
        let document = active_document(&state).expect("active report");
        assert_eq!(document.pages().len(), INITIAL_PAGES.len());
        for (page, (_, expected)) in document.pages().iter().zip(INITIAL_PAGES) {
            assert_eq!(page.title(), expected);
        }
        assert!(state.workspace.report_documents_dirty);
    }

    #[test]
    fn report_scaffold_is_idempotent_and_selection_uses_stable_ids() {
        let mut state = AppState::default();
        ensure_report_document(&mut state).expect("first open");
        let document_id = state.workbench.report_authoring.selected_document;
        let page_id = state.workbench.report_authoring.selected_page;
        ensure_report_document(&mut state).expect("second open");
        assert_eq!(state.workspace.report_documents.len(), 1);
        assert_eq!(
            state.workbench.report_authoring.selected_document,
            document_id
        );
        assert_eq!(state.workbench.report_authoring.selected_page, page_id);
    }

    #[test]
    fn report_page_title_validation_matches_domain_limits() {
        assert!(valid_page_title("PVT and yield"));
        assert!(!valid_page_title(""));
        assert!(!valid_page_title(" leading"));
        assert!(!valid_page_title("trailing "));
        assert!(!valid_page_title("bad\nlabel"));
        assert!(!valid_page_title(&"x".repeat(513)));
    }

    #[test]
    fn changing_active_run_does_not_mutate_the_project_report_document() {
        let mut state = AppState::default();
        ensure_report_document(&mut state).expect("report scaffold");
        state.simulation.runs = vec![
            crate::state::SimulationRun::new(2),
            crate::state::SimulationRun::new(1),
        ];
        let document = active_document(&state).expect("report document").clone();
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_run_idx = Some(1);
        assert_eq!(active_document(&state), Some(&document));
    }
}

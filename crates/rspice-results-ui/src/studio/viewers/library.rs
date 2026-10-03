//! Viewer catalog filtering and availability-aware selection.

use super::super::widgets::{paint_bottom_rule, paint_top_rule, panel_heading};
use egui::{Color32, Frame, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui, Vec2, vec2};
use rspice_results::{
    result_presentation::ResultViewer,
    viewer_catalog::{VIEWER_DOCUMENTS, ViewerDocumentDefinition, ViewerGroup},
};
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

pub fn filter(ui: &mut Ui, query: &mut String) -> String {
    panel_heading(ui, "Viewer library", &VIEWER_DOCUMENTS.len().to_string());
    let t = Tokens::get(ui.ctx());
    Frame::NONE.inner_margin(Margin::same(8)).show(ui, |ui| {
        let response = ui.add_sized(
            [ui.available_width(), 28.0],
            egui::TextEdit::singleline(query)
                .hint_text("Filter viewers")
                .margin(Margin {
                    left: 29,
                    right: 8,
                    top: 4,
                    bottom: 4,
                })
                .desired_width(f32::INFINITY),
        );
        let icon_rect = Rect::from_center_size(
            egui::pos2(response.rect.left() + 13.5, response.rect.center().y),
            Vec2::splat(13.0),
        );
        WorkbenchIcon::Search.paint(ui.painter(), icon_rect, t.color.text_faint);
    });

    query.trim().to_ascii_lowercase()
}

pub fn show(
    ui: &mut Ui,
    query: &str,
    selected_document: &str,
    mut availability_for: impl FnMut(&ViewerDocumentDefinition) -> Result<ResultViewer, String>,
) -> Option<(&'static str, ResultViewer)> {
    let mut selected = None;
    ScrollArea::vertical()
        .id_salt("visualization.viewer-library")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for group in ViewerGroup::ALL {
                let rows: Vec<_> = VIEWER_DOCUMENTS
                    .iter()
                    .filter(|definition| definition.group == group)
                    .filter(|definition| {
                        query.is_empty()
                            || definition.title.to_ascii_lowercase().contains(query)
                            || definition.domain.to_ascii_lowercase().contains(query)
                    })
                    .collect();
                if rows.is_empty() {
                    continue;
                }
                viewer_group_heading(ui, group.label());
                for definition in rows {
                    let availability = availability_for(definition);
                    let active = selected_document == definition.id;
                    let response = viewer_library_row(
                        ui,
                        definition,
                        active,
                        availability.is_ok(),
                        availability.as_ref().err().map(String::as_str),
                    );
                    if let Ok(viewer) = availability
                        && response.clicked()
                    {
                        selected = Some((definition.id, viewer));
                    }
                }
            }
        });
    selected
}

fn viewer_group_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let heading = Frame::NONE
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 7,
            bottom: 5,
        })
        .show(ui, |ui| {
            ui.label(
                RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
        });
    paint_top_rule(ui, heading.response.rect, t.color.border);
    paint_bottom_rule(ui, heading.response.rect, t.color.border);
}

fn viewer_library_row(
    ui: &mut Ui,
    definition: &ViewerDocumentDefinition,
    active: bool,
    available: bool,
    reason: Option<&str>,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), 36.0),
        if available {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            available && ui.is_enabled(),
            active,
            definition.title,
        )
    });
    ui.painter().rect_filled(
        rect,
        0.0,
        if active {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        },
    );
    if active {
        ui.painter().vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    }
    let color = if available {
        t.color.text
    } else {
        t.color.text_faint
    };
    ui.painter().text(
        rect.left_top() + vec2(8.0, 8.0),
        egui::Align2::LEFT_TOP,
        definition.title,
        theme::sans(tokens::FS_1, FontWeight::Medium),
        color,
    );
    let detail_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let detail = elide_text_to_width(
        ui,
        if available {
            definition.domain
        } else {
            reason.unwrap_or("Unavailable")
        },
        &detail_font,
        (rect.width() - 16.0).max(1.0),
    );
    ui.painter().text(
        rect.left_bottom() + vec2(8.0, -5.0),
        egui::Align2::LEFT_BOTTOM,
        detail,
        detail_font,
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);
    if let Some(reason) = reason {
        response.on_hover_text(reason)
    } else {
        response
    }
}

fn elide_text_to_width(ui: &Ui, text: &str, font: &egui::FontId, maximum_width: f32) -> String {
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return text.to_owned();
    }
    let mut candidate = text.to_owned();
    while candidate.pop().is_some() {
        let elided = format!("{}…", candidate.trim_end());
        if ui
            .painter()
            .layout_no_wrap(elided.clone(), font.clone(), Color32::WHITE)
            .size()
            .x
            <= maximum_width
        {
            return elided;
        }
    }
    "…".to_owned()
}

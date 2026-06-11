//! Strip chrome — the 28 px header (kind tag · subtitle · legend chips ·
//! actions) over a document-well body. Every plot in the product wears this.

use egui::{Rect, Sense, Stroke, Ui, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::IconButton;

/// Height of a strip header.
pub const HEADER_HEIGHT: f32 = 28.0;

/// One legend chip's description.
pub struct LegendChip<'a> {
    /// Trace name.
    pub name: &'a str,
    /// Trace color swatch.
    pub color: egui::Color32,
    /// Whether the trace is visible.
    pub on: bool,
}

/// What strip-header interactions happened this frame.
#[derive(Default)]
pub struct StripHeaderResponse {
    /// Index of the legend chip that was clicked.
    pub legend_clicked: Option<usize>,
    /// The maximize/restore action was clicked.
    pub maximize_clicked: bool,
    /// The close action was clicked.
    pub close_clicked: bool,
    /// The FIT action (shown while zoomed) was clicked.
    pub fit_clicked: bool,
}

/// Render a strip header across the available width. `zoomed` shows the
/// FIT action that restores the automatic view.
pub fn header(
    ui: &mut Ui,
    kind: &str,
    subtitle: &str,
    legend: &[LegendChip<'_>],
    maximized: bool,
    closable: bool,
    zoomed: bool,
) -> StripHeaderResponse {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut out = StripHeaderResponse::default();

    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, HEADER_HEIGHT), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return out;
    }
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, c.bg_panel);
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, c.border),
    );

    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(10.0, 0.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let ui = &mut child;
    ui.spacing_mut().item_spacing.x = 10.0;

    // Kind tag: mono semibold, letterspaced.
    let mut job = egui::text::LayoutJob::default();
    job.append(
        kind,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: c.text,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
    if !subtitle.is_empty() {
        ui.label(
            egui::RichText::new(subtitle)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.text_faint),
        );
    }

    ui.add_space(2.0);
    ui.spacing_mut().item_spacing.x = 4.0;
    for (index, chip) in legend.iter().enumerate() {
        if legend_chip(ui, chip).clicked() {
            out.legend_clicked = Some(index);
        }
    }

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = 1.0;
        if closable {
            if IconButton::new(Icon::Close)
                .side(22.0)
                .tooltip("Close strip")
                .show(ui)
                .clicked()
            {
                out.close_clicked = true;
            }
        }
        let tip = if maximized {
            "Restore strips"
        } else {
            "Maximize strip"
        };
        if IconButton::new(Icon::ZoomFit)
            .side(22.0)
            .on(maximized)
            .tooltip(tip)
            .show(ui)
            .clicked()
        {
            out.maximize_clicked = true;
        }
        if zoomed {
            ui.add_space(4.0);
            if fit_chip(ui).clicked() {
                out.fit_clicked = true;
            }
        }
    });

    out
}

/// The FIT chip: a small mono tag shown only while the plot is zoomed;
/// clicking it restores the automatic fit-to-data view (as does a
/// double-click on the plot itself).
fn fit_chip(ui: &mut Ui) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(
            "FIT".to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            c.accent,
        )
    });
    let (rect, response) =
        ui.allocate_exact_size(vec2(galley.size().x + 14.0, 18.0), Sense::click());
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.16);
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        mix(c.bg_panel, c.bg_hover, hover),
        Stroke::new(1.0, c.accent_dim),
    );
    painter.galley(
        egui::pos2(
            rect.center().x - galley.size().x * 0.5,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        c.accent,
    );

    response
        .on_hover_text("Restore automatic view (double-click the plot does the same)")
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A legend chip: 14×3 color swatch + mono name; dimmed when off.
fn legend_chip(ui: &mut Ui, chip: &LegendChip<'_>) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(
            chip.name.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
        )
    });
    let (rect, response) = ui.allocate_exact_size(
        vec2(6.0 + 14.0 + 5.0 + galley.size().x + 6.0, 20.0),
        Sense::click(),
    );
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.16);
    let opacity = if chip.on { 1.0 } else { 0.38 };
    let painter = ui.painter();
    if hover > 0.0 {
        painter.rect_filled(
            rect,
            t.radius,
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }
    let swatch_color = if chip.on { chip.color } else { c.text_faint };
    painter.rect_filled(
        Rect::from_min_size(
            egui::pos2(rect.left() + 6.0, rect.center().y - 1.5),
            vec2(14.0, 3.0),
        ),
        1.0,
        swatch_color.gamma_multiply(opacity),
    );
    painter.galley(
        egui::pos2(
            rect.left() + 6.0 + 14.0 + 5.0,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        mix(c.text_dim, c.text, hover).gamma_multiply(opacity),
    );

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

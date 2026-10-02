//! Shared result plot navigation, readout cards and status presentation.

use egui::{Ui, WidgetInfo, WidgetType};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

/// User zoom/pan override for one plot. `None` per axis means automatic
/// fit-to-data; gestures in the plot engine populate the ranges and a
/// double-click (or the strip's FIT action) clears them.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlotView {
    /// X range override (data space).
    pub x: Option<(f64, f64)>,
    /// Y range override.
    pub y: Option<(f64, f64)>,
}

impl PlotView {
    /// Whether any axis is zoomed away from the automatic view.
    pub fn is_zoomed(&self) -> bool {
        self.x.is_some() || self.y.is_some()
    }

    /// Fold one frame's gesture result into the override.
    pub fn apply(&mut self, change: &rspice_ui_kit::plot::ViewChange) {
        if change.reset {
            *self = Self::default();
            return;
        }
        if let Some(x) = change.x {
            self.x = Some(x);
        }
        if let Some(y) = change.y {
            self.y = Some(y);
        }
    }
}

/// Forward map for the XY (equal-aspect) viewers: data → screen, matching
/// the plot engine's linear mapping.
pub fn xy_screen_pos(
    plot_rect: egui::Rect,
    point: (f64, f64),
    x_range: (f64, f64),
    y_range: (f64, f64),
) -> egui::Pos2 {
    let fx = (point.0 - x_range.0) / (x_range.1 - x_range.0);
    let fy = (point.1 - y_range.0) / (y_range.1 - y_range.0);
    egui::pos2(
        plot_rect.left() + (fx as f32) * plot_rect.width(),
        plot_rect.bottom() - (fy as f32) * plot_rect.height(),
    )
}

/// Couple an equal-aspect XY viewer's navigation ranges so wheel zoom,
/// axis-constrained pan, and zoom boxes cannot distort circles or root maps.
pub fn square_xy_view_change(
    current_x: (f64, f64),
    current_y: (f64, f64),
    change: rspice_ui_kit::plot::ViewChange,
) -> rspice_ui_kit::plot::ViewChange {
    if change.reset || (change.x.is_none() && change.y.is_none()) {
        return change;
    }
    let next_x = change.x.unwrap_or(current_x);
    let next_y = change.y.unwrap_or(current_y);
    let x_span = (next_x.1 - next_x.0).abs();
    let y_span = (next_y.1 - next_y.0).abs();
    let span = match (change.x, change.y) {
        (Some(_), None) => x_span,
        (None, Some(_)) => y_span,
        (Some(_), Some(_)) => x_span.max(y_span),
        (None, None) => unreachable!("empty view changes return above"),
    }
    .max(f64::EPSILON);
    let x_center = (next_x.0 + next_x.1) * 0.5;
    let y_center = (next_y.0 + next_y.1) * 0.5;
    rspice_ui_kit::plot::ViewChange {
        x: Some((x_center - span * 0.5, x_center + span * 0.5)),
        y: Some((y_center - span * 0.5, y_center + span * 0.5)),
        reset: false,
    }
}

/// One row of a point readout card.
pub type CardRow = (String, String);

/// The floating point-readout card the XY viewers show on hover/pin:
/// colored mono title + k/v rows, anchored beside the point and clamped
/// to the plot. Painted, not laid out — it floats over the chart.
pub fn point_card(
    ui: &Ui,
    bounds: egui::Rect,
    anchor: egui::Pos2,
    title: &str,
    title_color: egui::Color32,
    rows: &[CardRow],
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let painter = ui.painter();

    let title_galley = painter.layout_no_wrap(
        title.to_owned(),
        theme::mono(11.0, FontWeight::Medium),
        title_color,
    );
    let mut key_width = 0.0f32;
    let mut value_width = 0.0f32;
    let galleys: Vec<_> = rows
        .iter()
        .map(|(k, v)| {
            let kg = painter.layout_no_wrap(
                k.clone(),
                theme::mono(11.0, FontWeight::Regular),
                c.text_dim,
            );
            let vg =
                painter.layout_no_wrap(v.clone(), theme::mono(11.0, FontWeight::Regular), c.text);
            key_width = key_width.max(kg.size().x);
            value_width = value_width.max(vg.size().x);
            (kg, vg)
        })
        .collect();

    let (pad_x, pad_y, gap, line_h) = (9.0, 6.0, 12.0, 16.0);
    let width = (key_width + gap + value_width).max(title_galley.size().x) + pad_x * 2.0;
    let height = pad_y * 2.0 + line_h * (rows.len() as f32 + 1.0);

    let mut origin = anchor + egui::vec2(14.0, -height - 8.0);
    if origin.x + width > bounds.right() - 4.0 {
        origin.x = anchor.x - width - 14.0;
    }
    origin.y = origin.y.clamp(
        bounds.top() + 4.0,
        (bounds.bottom() - height - 4.0).max(bounds.top() + 4.0),
    );

    let rect = egui::Rect::from_min_size(origin, egui::vec2(width, height));
    painter.rect(
        rect,
        t.radius,
        c.bg_elevated,
        egui::Stroke::new(1.0, c.border_strong),
        egui::StrokeKind::Inside,
    );
    painter.galley(
        egui::pos2(origin.x + pad_x, origin.y + pad_y),
        title_galley,
        title_color,
    );
    for (i, (kg, vg)) in galleys.into_iter().enumerate() {
        let y = origin.y + pad_y + line_h * (i as f32 + 1.0);
        painter.galley(egui::pos2(origin.x + pad_x, y), kg, c.text_dim);
        painter.galley(
            egui::pos2(
                origin.x + pad_x + key_width + gap + value_width - vg.size().x,
                y,
            ),
            vg,
            c.text,
        );
    }
}

/// Dimmed names and mono values, with accent highlighting for key rows.
pub fn stat_table(ui: &mut Ui, rows: &[(&str, String, bool)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (name_width, value_width) = stat_column_widths(width);
    for (i, (name, value, highlight)) in rows.iter().enumerate() {
        let name_galley = ui.painter().layout(
            (*name).to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            if *highlight { c.text } else { c.text_dim },
            name_width,
        );
        let value_galley = ui.painter().layout(
            value.clone(),
            theme::mono(tokens::FS_1, FontWeight::Regular),
            if *highlight { c.accent } else { c.text },
            value_width,
        );
        let row_height = 25.0_f32.max(name_galley.size().y.max(value_galley.size().y) + 8.0);
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());
        if !ui.is_rect_visible(rect) {
            continue;
        }
        let painter = ui.painter();
        let name_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + 12.0, rect.top() + 4.0),
            egui::vec2(name_width, row_height - 8.0),
        );
        let value_rect = egui::Rect::from_min_size(
            egui::pos2(name_rect.right() + 8.0, rect.top() + 4.0),
            egui::vec2(value_width, row_height - 8.0),
        );
        painter
            .with_clip_rect(name_rect)
            .galley(name_rect.min, name_galley, c.text_dim);
        painter.with_clip_rect(value_rect).galley(
            egui::pos2(value_rect.right() - value_galley.size().x, value_rect.top()),
            value_galley,
            c.text,
        );
        if i + 1 < rows.len() {
            painter.hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
        }
    }
}

fn stat_column_widths(width: f32) -> (f32, f32) {
    const OUTER_INSET: f32 = 24.0;
    const GAP: f32 = 8.0;
    let content = (width - OUTER_INSET - GAP).max(0.0);
    let name = content * 0.42;
    (name, content - name)
}

/// A faint explanatory note under a right-panel section.
pub fn panel_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: 12,
            right: 12,
            top: 4,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
}

/// Parse a `#rrggbb` trace color, falling back to the palette cycle.
pub fn trace_color(hex: &str, fallback: egui::Color32) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6
        && let Ok(value) = u32::from_str_radix(hex, 16)
    {
        return egui::Color32::from_rgb(
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        );
    }
    fallback
}

/// Centered faint hint on an empty document well.
pub fn well_hint(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    let response = ui.interact(
        rect,
        ui.id().with(("result-well-status", text)),
        egui::Sense::hover(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, text));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(text);
    });
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        theme::sans(tokens::FS_2, FontWeight::Regular),
        t.color.text_faint,
    );
}

#[cfg(test)]
mod tests;

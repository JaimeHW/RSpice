//! Shared result plot navigation, readout cards and status presentation.

use egui::{Ui, WidgetInfo, WidgetType};
use rspice_ui_kit::panels::inspector::toned_inspector_copy;
use rspice_ui_kit::panels::{PANEL_ROW_H, panel_inset};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

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

/// `value` to `decimals` places with a true minus sign and no negative
/// zero, so every row of a column stands at one precision and its decimal
/// points align: a decibel to its hundredth, a share to its tenth of a
/// percent.
#[must_use]
pub fn fixed_places(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let unsigned = text.trim_start_matches('-');
    if unsigned
        .chars()
        .all(|character| matches!(character, '0' | '.'))
    {
        return unsigned.to_owned();
    }
    text.replace('-', "\u{2212}")
}

/// A decibel figure's places: to the hundredth of a decibel.
pub const DECIBEL_PLACES: usize = 2;

/// A share's places: to the tenth of a percent.
pub const SHARE_PLACES: usize = 1;

/// The least widths of a details table's columns, laid in `room`: each
/// column as wide as its head, with room for a sort mark, and its widest
/// cell when `room` holds them all whole. When it does not, the heads give
/// up the sort mark's room, which a sorted head takes from its words. When
/// even that does not fit, each `flexible` column, one that shares the spare
/// width, narrows to the shorter of its head and its widest cell, so its
/// longer words elide while a fixed column's never do. Tables stacked one
/// over another pass together and share one width per column, the widest
/// any of them needs, so they read as one grid.
pub(crate) fn details_column_widths(
    ui: &Ui,
    tables: &[&[crate::fft::view::columns::Fitting]],
    flexible: &[bool],
    room: f32,
) -> Vec<f32> {
    use crate::fft::view::columns::{Fitting, fitted_widths};
    use rspice_ui_kit::widgets::table::SORT_MARK_ROOM;
    let widest = |part: &dyn Fn(&Fitting) -> Fitting| {
        tables
            .iter()
            .map(|table| fitted_widths(ui, &table.iter().map(part).collect::<Vec<_>>()))
            .reduce(|widest, widths| {
                widest
                    .iter()
                    .zip(&widths)
                    .map(|(widest, width)| widest.max(*width))
                    .collect()
            })
            .unwrap_or_default()
    };
    let whole = widest(&Fitting::clone);
    if whole.iter().sum::<f32>() <= room {
        return whole;
    }
    // Each column's head words alone, and its widest cell alone.
    let heads: Vec<f32> = widest(&|column| Fitting {
        cells: Vec::new(),
        ..column.clone()
    })
    .into_iter()
    .map(|head| head - SORT_MARK_ROOM)
    .collect();
    let cells = widest(&|column| Fitting {
        head: String::new(),
        ..column.clone()
    });
    let tight: Vec<f32> = heads
        .iter()
        .zip(&cells)
        .map(|(head, cell)| head.max(*cell))
        .collect();
    if tight.iter().sum::<f32>() <= room {
        return tight;
    }
    tight
        .iter()
        .zip(flexible)
        .zip(heads.iter().zip(&cells))
        .map(
            |((tight, flexible), (head, cell))| {
                if *flexible { head.min(*cell) } else { *tight }
            },
        )
        .collect()
}

/// The width a details table in `ui` runs: to the panel's edges on a panel
/// body's inset.
pub(crate) fn details_table_room(ui: &Ui) -> f32 {
    rspice_ui_kit::panels::panel_bleed(ui, ui.available_rect_before_wrap()).width()
}

/// The most of a sheet's height the table under a plane takes; a short
/// table takes only its rows.
pub const TABLE_SHARE_UNDER_PLANE: f32 = 0.42;

/// A plane over the table listing what it draws, both across the sheet's
/// full width: the table shows whole rows, all it has up to its share of the
/// height, and scrolls the rest; the plane takes exactly what is left. The
/// table's head stands on the plane's X row and its last rule on `avail`'s
/// bottom edge, with nothing between.
pub fn plane_over_table(ui: &Ui, avail: egui::Rect, rows: usize) -> (egui::Rect, egui::Rect) {
    let lines = rspice_ui_kit::widgets::table::LineHeights::grid(&Tokens::get(ui.ctx()));
    let room = ((avail.height() * TABLE_SHARE_UNDER_PLANE - lines.head) / lines.row)
        .floor()
        .max(1.0);
    #[allow(clippy::cast_precision_loss)]
    let shown = (rows as f32).min(room);
    let split = avail.bottom() - (lines.head + lines.row * shown);
    (
        egui::Rect::from_min_max(avail.min, egui::pos2(avail.right(), split)),
        egui::Rect::from_min_max(egui::pos2(avail.left(), split), avail.max),
    )
}

/// Couple an equal-aspect XY viewer's navigation ranges so wheel zoom,
/// axis-constrained pan, and zoom boxes cannot distort circles or root maps.
pub fn square_xy_view_change(
    current_x: (f64, f64),
    current_y: (f64, f64),
    change: rspice_ui_kit::plot::ViewChange,
) -> rspice_ui_kit::plot::ViewChange {
    equal_scale_view_change(current_x, current_y, change, egui::Vec2::splat(1.0))
}

/// [`square_xy_view_change`] for a plane `plot` points across that is not
/// square: a zoom or pan along one axis carries the other with it at the
/// same points per unit, and a zoom box keeps all it took in.
pub fn equal_scale_view_change(
    current_x: (f64, f64),
    current_y: (f64, f64),
    change: rspice_ui_kit::plot::ViewChange,
    plot: egui::Vec2,
) -> rspice_ui_kit::plot::ViewChange {
    if change.reset || (change.x.is_none() && change.y.is_none()) {
        return change;
    }
    let next_x = change.x.unwrap_or(current_x);
    let next_y = change.y.unwrap_or(current_y);
    let (width, height) = (f64::from(plot.x.max(1.0)), f64::from(plot.y.max(1.0)));
    let along_x = (next_x.1 - next_x.0).abs() / width;
    let along_y = (next_y.1 - next_y.0).abs() / height;
    let per_point = match (change.x, change.y) {
        (Some(_), None) => along_x,
        (None, Some(_)) => along_y,
        (Some(_), Some(_)) => along_x.max(along_y),
        (None, None) => unreachable!("empty view changes return above"),
    }
    .max(f64::EPSILON);
    let x_center = (next_x.0 + next_x.1) * 0.5;
    let y_center = (next_y.0 + next_y.1) * 0.5;
    let (x_half, y_half) = (per_point * width * 0.5, per_point * height * 0.5);
    rspice_ui_kit::plot::ViewChange {
        x: Some((x_center - x_half, x_center + x_half)),
        y: Some((y_center - y_half, y_center + y_half)),
        reset: false,
    }
}

/// One row of a point readout card.
pub type CardRow = (String, String);

/// Length and thickness of the trace swatch in front of a card title, and
/// the air between the swatch and the title.
const CARD_SWATCH: egui::Vec2 = egui::vec2(12.0, 2.0);
const CARD_SWATCH_GAP: f32 = 6.0;

/// The floating point-readout card the XY viewers show on hover/pin:
/// a short line swatch in the trace's colour, a mono title in primary text,
/// then k/v rows, anchored beside the point and clamped to the plot.
/// Painted, not laid out — it floats over the chart.
pub fn point_card(
    ui: &Ui,
    bounds: egui::Rect,
    anchor: egui::Pos2,
    title: &str,
    swatch: egui::Color32,
    rows: &[CardRow],
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let painter = ui.painter();

    let title_galley = painter.layout_no_wrap(
        title.to_owned(),
        theme::mono(11.0, FontWeight::Medium),
        c.text,
    );
    let title_indent = CARD_SWATCH.x + CARD_SWATCH_GAP;
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
    let width =
        (key_width + gap + value_width).max(title_indent + title_galley.size().x) + pad_x * 2.0;
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
    let title_center_y = origin.y + pad_y + title_galley.size().y * 0.5;
    painter.rect_filled(
        egui::Rect::from_center_size(
            egui::pos2(origin.x + pad_x + CARD_SWATCH.x * 0.5, title_center_y),
            CARD_SWATCH,
        ),
        0.0,
        swatch,
    );
    painter.galley(
        egui::pos2(origin.x + pad_x + title_indent, origin.y + pad_y),
        title_galley,
        c.text,
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

/// The row and header height of a sheet's data table: 24 px at compact
/// density and 29 px relaxed where a pointer drives the sheet, as dense as
/// the signal lists of ViVA and Verdi, and a finger's target on the touch
/// build.
pub(crate) fn table_row_height(ui: &Ui) -> f32 {
    let metrics = Tokens::get(ui.ctx()).metrics;
    if metrics.is_touch() {
        metrics.touch_target()
    } else {
        metrics.row_h - 4.0
    }
}

/// A sheet table's memory between frames (its selection, sort, sideways
/// scroll and the widths the reader dragged), kept with the window under
/// `salt` and lent to `show`.
pub(crate) fn with_table_state<R>(
    ui: &mut Ui,
    salt: &str,
    show: impl FnOnce(&mut Ui, &mut rspice_ui_kit::widgets::table::TableState) -> R,
) -> R {
    let id = ui.id().with(("rspice.results.table-state", salt));
    let mut state = ui
        .data_mut(|data| data.get_temp::<rspice_ui_kit::widgets::table::TableState>(id))
        .unwrap_or_default();
    let shown = show(ui, &mut state);
    ui.data_mut(|data| data.insert_temp(id, state));
    shown
}

/// A sheet table's row menu: opened where the reader asked for it, on the
/// row they asked on, and drawn by `items` until it closes.
pub(crate) fn table_row_menu(
    ui: &Ui,
    salt: &str,
    asked: Option<rspice_ui_kit::widgets::table::ContextRequest>,
    items: impl FnOnce(&mut Ui, usize),
) {
    let id = ui.id().with(("rspice.results.table-menu", salt));
    let ctx = ui.ctx().clone();
    if let Some(asked) = asked {
        ctx.data_mut(|data| data.insert_temp(id, (asked.cell.row, asked.position)));
        egui::Popup::open_id(&ctx, id);
    }
    if !egui::Popup::is_id_open(&ctx, id) {
        return;
    }
    let Some((row, position)) = ctx.data(|data| data.get_temp::<(usize, egui::Pos2)>(id)) else {
        return;
    };
    egui::Popup::new(
        id,
        ctx,
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    .open_memory(None)
    .show(|ui| items(ui, row));
}

/// Dimmed names and mono values, the values selectable for copying. A key
/// row is set in primary text with a semibold value: weight, not colour,
/// since accent means selection. No rule between rows: the row rhythm and the
/// aligned values carry the reading, as the property rows above them do, and
/// a section's own rule is the only line that marks a group.
pub fn stat_table(ui: &mut Ui, rows: &[(&str, String, bool)]) {
    stat_table_responses(ui, rows);
}

/// [`stat_table`], returning each row's response in order, for a row to
/// carry a hover of its own.
pub fn stat_table_responses(ui: &mut Ui, rows: &[(&str, String, bool)]) -> Vec<egui::Response> {
    let mut responses = Vec::with_capacity(rows.len());
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let inset = panel_inset(ui);
    let (name_width, value_width) = stat_column_widths(width, inset);
    for (name, value, highlight) in rows {
        let value_font = theme::mono(
            tokens::FS_1,
            if *highlight {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        );
        // A value keeps one line: when it is wider than its column, the
        // name's column gives way to it rather than the value wrapping, down
        // to the name's own width. Only a value wider still wraps; a name
        // squeezed to nothing would stand one letter to a line.
        let natural = ui
            .painter()
            .layout_no_wrap(value.clone(), value_font.clone(), c.text)
            .size()
            .x;
        let name_font = theme::sans(tokens::FS_1, FontWeight::Regular);
        let name_natural = ui
            .painter()
            .layout_no_wrap((*name).to_owned(), name_font.clone(), c.text_dim)
            .size()
            .x;
        let content = name_width + value_width;
        let value_width = natural.clamp(value_width, (content - name_natural).max(value_width));
        let name_width = content - value_width;
        let name_galley = ui.painter().layout(
            (*name).to_owned(),
            name_font,
            if *highlight { c.text } else { c.text_dim },
            name_width,
        );
        let value_galley = ui
            .painter()
            .layout(value.clone(), value_font, c.text, value_width);
        let row_height = PANEL_ROW_H.max(name_galley.size().y.max(value_galley.size().y) + 8.0);
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());
        responses.push(response);
        if !ui.is_rect_visible(rect) {
            continue;
        }
        let painter = ui.painter().clone();
        let name_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + inset, rect.top() + 4.0),
            egui::vec2(name_width, row_height - 8.0),
        );
        let value_rect = egui::Rect::from_min_size(
            egui::pos2(name_rect.right() + 8.0, rect.top() + 4.0),
            egui::vec2(value_width, row_height - 8.0),
        );
        painter
            .with_clip_rect(name_rect)
            .galley(name_rect.min, name_galley, c.text_dim);
        // The value is text the reader can select and copy, as a readout
        // is read off to be used elsewhere; the name beside it is painted.
        let value_at = egui::Rect::from_min_size(
            egui::pos2(value_rect.right() - value_galley.size().x, value_rect.top()),
            value_galley.size(),
        )
        .intersect(value_rect);
        ui.new_child(
            egui::UiBuilder::new()
                .max_rect(value_at)
                .layout(egui::Layout::right_to_left(egui::Align::Min)),
        )
        .add(egui::Label::new(value_galley).selectable(true));
    }
    responses
}

/// The table's two columns inside the panel inset on both sides.
fn stat_column_widths(width: f32, inset: f32) -> (f32, f32) {
    const GAP: f32 = 8.0;
    let content = (width - 2.0 * inset - GAP).max(0.0);
    let name = content * 0.42;
    (name, content - name)
}

/// A frequency readout at `significant_digits`, in the unit Units ▸
/// Frequency display chose. The stored value is hertz either way.
pub fn frequency_readout(
    quantity_policy: &QuantityPresentationPolicy,
    hertz: f64,
    significant_digits: usize,
) -> String {
    let (scale, offset, unit) = quantity_policy.frequency_axis_transform();
    rspice_ui_kit::plot::fmt_si_significant(hertz * scale + offset, unit, significant_digits)
}

/// An angle readout at `significant_digits`, in the unit Units ▸ Angle
/// display chose. The stored value is radians either way.
/// A complex value as `a + jb`, each part in `unit` at `significant_digits`
/// with a true minus sign.
pub fn complex_readout(real: f64, imaginary: f64, unit: &str, significant_digits: usize) -> String {
    format!(
        "{} {} j{}",
        rspice_ui_kit::plot::fmt_si_significant(real, unit, significant_digits),
        if imaginary >= 0.0 { "+" } else { "\u{2212}" },
        rspice_ui_kit::plot::fmt_si_significant(imaginary.abs(), unit, significant_digits)
    )
}

pub fn angle_readout(
    quantity_policy: &QuantityPresentationPolicy,
    radians: f64,
    significant_digits: usize,
) -> String {
    use rspice_vocabulary::quantity::AngleDisplay;
    match quantity_policy.angle_display {
        AngleDisplay::Degrees => {
            rspice_ui_kit::plot::fmt_si_significant(radians.to_degrees(), "°", significant_digits)
        }
        AngleDisplay::Radians => {
            rspice_ui_kit::plot::fmt_si_significant(radians, "rad", significant_digits)
        }
    }
}

/// A faint explanatory note under a right-panel section.
///
/// The inspector's own note, in this panel's quieter tone: a viewer's notes
/// sit in the same inspector as its rows and wrap inside the same inset.
pub fn panel_note(ui: &mut Ui, text: &str) {
    let faint = Tokens::get(ui.ctx()).color.text_faint;
    toned_inspector_copy(ui, text, faint);
}

/// The colour a reader gave a trace, as drawn.
pub fn chosen_trace_color(
    color: rspice_results::visualization_document::TraceColor,
) -> egui::Color32 {
    let [red, green, blue] = color.rgb();
    egui::Color32::from_rgb(red, green, blue)
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
        t.color.canvas_text_faint,
    );
}

#[cfg(test)]
mod tests;

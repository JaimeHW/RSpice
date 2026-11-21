//! The Y axes a strip reads after its first, each a column of the gutter
//! left of the first axis' ticks, as ViVA stacks a strip's axes: its unit,
//! the colours of the curves it reads, and its ticks. Scrolled over, an
//! axis zooms about the pointer; dragged, it pans; double-clicked, it fits
//! its curves again. Its range is its own: the strip's other axes stay as
//! they are.

use egui::{Rect, Sense, Ui, pos2};
use rspice_ui_kit::plot::{Axis, fmt_si_significant};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::Tokens;

use super::StripModel;
use super::pane::SecondaryPlot;
use super::stack::{Gutter, SECONDARY_AXIS_WIDTH};

/// How far one wheel notch zooms an axis.
const ZOOM_PER_POINT: f64 = 0.003;
/// How far an arrow key pans a focused axis, as a share of its span.
const KEY_PAN_SHARE: f64 = 0.1;

/// Draw the columns of `axes` beside the plot spanning `plot_rect`, and
/// return the range each axis the reader zoomed, panned or fitted now
/// shows, by its slot: `None` to fit its curves again.
pub(crate) fn secondary_axes(
    ui: &mut Ui,
    model: &StripModel,
    axes: &[SecondaryPlot<'_>],
    plot_rect: Rect,
    gutter: Gutter,
    t: &Tokens,
) -> Vec<(usize, Option<(f64, f64)>)> {
    let mut changes = Vec::new();
    if plot_rect.height() <= 1.0 {
        return changes;
    }
    let painter = ui.painter().clone();
    let tick_font = theme::mono(rspice_ui_kit::tokens::FS_MICRO, FontWeight::Regular);
    for (column, axis) in axes.iter().enumerate() {
        let right = plot_rect.left() - gutter.value - column as f32 * SECONDARY_AXIS_WIDTH;
        let rect = Rect::from_min_max(
            pos2(right - SECONDARY_AXIS_WIDTH, plot_rect.top()),
            pos2(right, plot_rect.bottom()),
        );
        let (low, high) = axis.range;
        if !(low.is_finite() && high.is_finite()) || high <= low {
            continue;
        }
        let colours: Vec<egui::Color32> = axis
            .traces
            .iter()
            .filter_map(|index| model.traces.get(*index).map(|trace| trace.color))
            .chain(axis.expressions.iter().map(|expr| expr.color))
            .collect();
        let spine_x = rect.right() - 6.0;
        let spine = colours.first().copied().unwrap_or(t.color.text_dim);
        painter.vline(
            spine_x,
            rect.y_range(),
            egui::Stroke::new(1.0, spine.gamma_multiply(0.8)),
        );
        let to_y =
            |value: f64| rect.bottom() - ((value - low) / (high - low)) as f32 * rect.height();
        let target = ((rect.height() / 32.0) as usize).clamp(2, 6);
        for (value, _) in Axis::linear_with(low, high, "", target).ticks {
            let y = to_y(value);
            if !(rect.top() - 0.5..=rect.bottom() + 0.5).contains(&y) {
                continue;
            }
            painter.hline(
                spine_x - 3.0..=spine_x,
                y,
                egui::Stroke::new(1.0, t.color.text_dim),
            );
            painter.text(
                pos2(spine_x - 5.0, y),
                egui::Align2::RIGHT_CENTER,
                fmt_si_significant(value, "", 3),
                tick_font.clone(),
                t.color.text_dim,
            );
        }
        // The unit over the column, and a dot for each curve the axis reads
        // in that curve's colour, so the column says whose axis it is.
        let caption_y = rect.top() - 9.0;
        painter.text(
            pos2(spine_x, caption_y),
            egui::Align2::RIGHT_CENTER,
            if axis.unit.is_empty() { "1" } else { axis.unit },
            tick_font.clone(),
            t.color.text,
        );
        // As many dots as the column has room for.
        let room = ((rect.width() - 8.0) / 6.0).max(0.0) as usize;
        for (index, colour) in colours.iter().enumerate().take(room) {
            painter.circle_filled(
                pos2(rect.left() + 4.0 + index as f32 * 6.0, caption_y),
                2.5,
                *colour,
            );
        }

        let id = egui::Id::new(("rspice.results.strip-axis", model.analysis_key, axis.slot));
        let response = ui
            .interact(rect, id, Sense::click_and_drag())
            .on_hover_cursor(egui::CursorIcon::ResizeVertical);
        let unit = if axis.unit.is_empty() {
            "plain-number".to_owned()
        } else {
            axis.unit.to_owned()
        };
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Slider, true, format!("{unit} axis"))
        });
        theme::paint_focus_ring(ui, &response, rect);
        let response = response.on_hover_text(format!(
            "The {unit} axis: scroll to zoom, drag to pan, double-click to fit its curves"
        ));
        let span = high - low;
        let per_point = span / f64::from(rect.height());
        if response.double_clicked() {
            changes.push((axis.slot, None));
            continue;
        }
        if response.dragged() {
            let shift = f64::from(response.drag_delta().y) * per_point;
            if shift != 0.0 {
                changes.push((axis.slot, Some((low + shift, high + shift))));
                continue;
            }
        }
        if response.hovered() {
            let scroll = ui.input(|input| input.smooth_scroll_delta.y);
            if scroll != 0.0
                && let Some(pointer) = response.hover_pos()
            {
                // The axis takes the wheel: the strips under it do not
                // scroll as well.
                ui.input_mut(|input| input.smooth_scroll_delta.y = 0.0);
                let factor = (-f64::from(scroll) * ZOOM_PER_POINT).exp();
                let pivot = low + f64::from(rect.bottom() - pointer.y) * per_point;
                changes.push((
                    axis.slot,
                    Some((
                        pivot - (pivot - low) * factor,
                        pivot + (high - pivot) * factor,
                    )),
                ));
                continue;
            }
        }
        if response.has_focus() {
            let (up, down) = ui.input(|input| {
                (
                    input.key_pressed(egui::Key::ArrowUp),
                    input.key_pressed(egui::Key::ArrowDown),
                )
            });
            let shift = match (up, down) {
                (true, false) => span * KEY_PAN_SHARE,
                (false, true) => -span * KEY_PAN_SHARE,
                _ => 0.0,
            };
            if shift != 0.0 {
                changes.push((axis.slot, Some((low + shift, high + shift))));
            }
        }
    }
    changes
}

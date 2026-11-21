//! The gestures that arrange a waveform stack's strips: a legend entry
//! dragged onto a strip or between two, a strip's header dragged up or
//! down, the divider between two strips dragged or double-clicked, and the
//! strip and curve menus. Each makes a new arrangement through
//! [`strip_layout`], which the view keeps; a drag only remembers how far it
//! has gone until it is let go.

use egui::{Rect, Sense, Ui};
use rspice_results::result_presentation::AnalysisPresentationKey;
use rspice_results::visualization_document::Strip;
use rspice_ui_kit::theme;
use rspice_ui_kit::tokens::Tokens;

use super::StripModel;
use super::stack::StackHost;
use super::strip_layout::{self, Curve};

/// A legend entry being dragged: its curve, of `analysis`' stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurveDrag {
    pub analysis: AnalysisPresentationKey,
    pub curve: Curve,
}

/// A strip header being dragged: the strip, by its identity in the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderDrag {
    pub analysis: AnalysisPresentationKey,
    pub identity: String,
}

/// How near a strip's edge a dragged entry drops between strips rather
/// than onto one.
const GAP_REACH: f32 = 8.0;
/// The height of the band a divider answers the pointer over.
const DIVIDER_REACH: f32 = 7.0;
/// The least height a divider drag leaves either strip.
const MIN_DRAGGED_HEIGHT: f32 = 40.0;

/// Keep `edit`'s strips for `model`'s analysis, or say why it was refused.
pub(crate) fn arrange(
    host: &mut impl StackHost,
    model: &StripModel,
    edit: Result<Vec<Strip>, String>,
) {
    match edit {
        Ok(strips) => {
            let session = host.session_mut();
            if strips.is_empty() {
                session.strip_layouts.remove(&model.analysis_key);
            } else {
                session.strip_layouts.insert(model.analysis_key, strips);
            }
        }
        Err(reason) => host.refuse(reason),
    }
}

fn divider_id(model: &StripModel, index: usize) -> egui::Id {
    egui::Id::new(("rspice.results.strip-divider", model.analysis_key, index))
}

/// How far the divider under strip `index` has been dragged and not yet
/// let go, which the strips' heights take before the view keeps it.
pub(crate) fn divider_offset(ctx: &egui::Context, model: &StripModel, index: usize) -> f32 {
    ctx.data(|data| data.get_temp::<f32>(divider_id(model, index)))
        .unwrap_or(0.0)
}

/// The divider between strip `index` and the one under it, `heights` the
/// two strips' heights as drawn: a hairline seam that a drag resizes the
/// two by, a double-click evens every strip out, and arrow keys nudge.
pub(crate) fn divider(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    index: usize,
    (upper, lower): (f32, f32),
    locked: bool,
) {
    let t = Tokens::get(ui.ctx());
    let (seam, _) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            1.0_f32.min(ui.available_height().max(0.0)),
        ),
        Sense::hover(),
    );
    let reach = Rect::from_center_size(seam.center(), egui::vec2(seam.width(), DIVIDER_REACH));
    let id = divider_id(model, index);
    let response = ui.interact(reach, id, Sense::click_and_drag());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Slider,
            !locked,
            format!("Divider under strip {}", index + 1),
        )
    });
    theme::paint_focus_ring(ui, &response, reach);
    let active = !locked && (response.hovered() || response.dragged());
    ui.painter().rect_filled(
        seam,
        0.0,
        if active {
            t.color.accent
        } else {
            t.color.canvas_grid
        },
    );
    if locked {
        response.on_hover_text("A locked strip keeps its height. Unlock it in the strip's menu.");
        return;
    }
    let response = response
        .on_hover_cursor(egui::CursorIcon::ResizeRow)
        .on_hover_text("Drag to resize the strips; double-click to even them out");
    if response.double_clicked() {
        ui.ctx().data_mut(|data| data.remove::<f32>(id));
        arrange(host, model, strip_layout::equalise(model));
        return;
    }
    let mut offset = divider_offset(ui.ctx(), model, index);
    if response.dragged() {
        offset += response.drag_delta().y;
        // Each strip keeps a readable height.
        let floor = MIN_DRAGGED_HEIGHT.min(upper).min(lower);
        offset = offset.clamp(floor - upper, lower - floor);
        ui.ctx().data_mut(|data| data.insert_temp(id, offset));
    }
    let nudge = if response.has_focus() {
        ui.input(|input| {
            if input.key_pressed(egui::Key::ArrowUp) {
                -8.0
            } else if input.key_pressed(egui::Key::ArrowDown) {
                8.0
            } else {
                0.0
            }
        })
    } else {
        0.0
    };
    if response.drag_stopped() || nudge != 0.0 {
        ui.ctx().data_mut(|data| data.remove::<f32>(id));
        let offset = offset + nudge;
        if offset != 0.0 {
            arrange(
                host,
                model,
                strip_layout::resize(model, index, (upper + offset, lower - offset)),
            );
        }
    }
}

/// Where a dragged entry would land among the strips at `rects`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Landing {
    /// Onto strip `index`.
    Strip(usize),
    /// Between strips, at `index` among them.
    Gap(usize),
}

fn landing(rects: &[Rect], pointer: egui::Pos2) -> Option<Landing> {
    let last = rects.len().checked_sub(1)?;
    for (index, rect) in rects.iter().enumerate() {
        if !(rect.x_range().contains(pointer.x)) {
            return None;
        }
        if (pointer.y - rect.top()).abs() <= GAP_REACH {
            return Some(Landing::Gap(index));
        }
        if index == last && (pointer.y - rect.bottom()).abs() <= GAP_REACH {
            return Some(Landing::Gap(index + 1));
        }
        if rect.contains(pointer) {
            return Some(Landing::Strip(index));
        }
    }
    None
}

/// While a legend entry or a strip header of `model`'s stack is dragged
/// over the strips at `rects`, show where it would land, or why it cannot;
/// let go, arrange the strips so.
pub(crate) fn drops(ui: &mut Ui, host: &mut impl StackHost, model: &StripModel, rects: &[Rect]) {
    let ctx = ui.ctx().clone();
    let Some(pointer) = ctx.pointer_latest_pos() else {
        return;
    };
    let released = ctx.input(|input| input.pointer.any_released());
    let t = Tokens::get(&ctx);
    let painter = ui.painter().clone();
    if let Some(drag) = egui::DragAndDrop::payload::<CurveDrag>(&ctx)
        .filter(|drag| drag.analysis == model.analysis_key)
    {
        let own = strip_layout::place_of(model, drag.curve).map(|place| place.strip);
        let Some(landing) = landing(rects, pointer) else {
            return;
        };
        let edit = match landing {
            Landing::Strip(index) if Some(index) == own => return,
            Landing::Strip(index) => strip_layout::move_to_strip(model, drag.curve, index),
            Landing::Gap(index) => strip_layout::move_to_gap(model, drag.curve, index),
        };
        let colour = if edit.is_ok() {
            t.color.accent
        } else {
            t.color.err
        };
        match landing {
            Landing::Strip(index) => {
                painter.rect_stroke(
                    rects[index].shrink(1.0),
                    2.0,
                    egui::Stroke::new(2.0, colour),
                    egui::StrokeKind::Inside,
                );
            }
            Landing::Gap(index) => {
                let y = rects
                    .get(index)
                    .map_or_else(|| rects[index - 1].bottom(), Rect::top);
                painter.hline(rects[0].x_range(), y, egui::Stroke::new(3.0, colour));
            }
        }
        if let Err(reason) = &edit {
            egui::Tooltip::always_open(
                ctx.clone(),
                ui.layer_id(),
                egui::Id::new("rspice.results.strip-drop-refusal"),
                egui::PopupAnchor::Pointer,
            )
            .show(|ui| ui.label(reason));
        }
        if released {
            egui::DragAndDrop::clear_payload(&ctx);
            arrange(host, model, edit);
        }
        return;
    }
    if let Some(drag) = egui::DragAndDrop::payload::<HeaderDrag>(&ctx)
        .filter(|drag| drag.analysis == model.analysis_key)
    {
        let panes = model.unit_panes();
        let Some(from) = panes.iter().position(|pane| pane.unit == drag.identity) else {
            return;
        };
        if !rects.iter().any(|rect| rect.x_range().contains(pointer.x)) {
            return;
        }
        // The strip goes where the pointer is: before the first strip whose
        // middle is below it.
        let before = rects
            .iter()
            .position(|rect| pointer.y < rect.center().y)
            .unwrap_or(rects.len());
        let to = if before > from { before - 1 } else { before };
        let y = rects
            .get(before)
            .map_or_else(|| rects[rects.len() - 1].bottom(), Rect::top);
        if to != from {
            painter.hline(
                rects[0].x_range(),
                y,
                egui::Stroke::new(3.0, t.color.accent),
            );
        }
        if released {
            egui::DragAndDrop::clear_payload(&ctx);
            if to != from {
                arrange(host, model, strip_layout::reorder(model, from, to));
            }
        }
    }
}

/// One menu entry that makes `edit`: offered only when it can be made,
/// with the reason on hover when it cannot.
fn entry(
    ui: &mut Ui,
    label: &str,
    edit: Result<Vec<Strip>, String>,
    chosen: &mut Option<Vec<Strip>>,
) {
    match edit {
        Ok(strips) => {
            if ui.button(label).clicked() {
                *chosen = Some(strips);
                ui.close();
            }
        }
        Err(reason) => {
            ui.add_enabled(false, egui::Button::new(label))
                .on_disabled_hover_text(reason);
        }
    }
}

/// The strip entries of strip `index`'s menu, and the arrangement the one
/// chosen makes.
pub(crate) fn strip_menu(ui: &mut Ui, model: &StripModel, index: usize) -> Option<Vec<Strip>> {
    let mut chosen = None;
    entry(
        ui,
        "Split strip",
        strip_layout::split_strip(model, index),
        &mut chosen,
    );
    entry(
        ui,
        "Split all strips",
        strip_layout::split_all_strips(model),
        &mut chosen,
    );
    rspice_ui_kit::widgets::menu_button(ui, "Strip by", |ui| {
        entry(ui, "Unit", strip_layout::strip_by_unit(model), &mut chosen);
        entry(
            ui,
            "Trace",
            strip_layout::strip_by_trace(model),
            &mut chosen,
        );
    });
    entry(
        ui,
        "Merge with strip above",
        strip_layout::merge_with_strip_above(model, index),
        &mut chosen,
    );
    entry(
        ui,
        "Combine all strips",
        strip_layout::combine_all_strips(model),
        &mut chosen,
    );
    let locked = model
        .unit_panes()
        .get(index)
        .is_some_and(|pane| pane.locked);
    let mut lock = locked;
    if ui.checkbox(&mut lock, "Lock strip size").changed()
        && let Ok(strips) = strip_layout::set_locked(model, index, lock)
    {
        chosen = Some(strips);
        ui.close();
    }
    entry(
        ui,
        "Even out strip heights",
        strip_layout::equalise(model),
        &mut chosen,
    );
    chosen
}

/// The arranging entries of a curve's menu, and the arrangement the one
/// chosen makes.
pub(crate) fn curve_menu(ui: &mut Ui, model: &StripModel, curve: Curve) -> Option<Vec<Strip>> {
    let mut chosen = None;
    entry(
        ui,
        "Move to new strip",
        strip_layout::move_to_new_strip(model, curve),
        &mut chosen,
    );
    entry(
        ui,
        "Copy to new strip",
        strip_layout::copy_to_new_strip(model, curve),
        &mut chosen,
    );
    entry(
        ui,
        "Move to new Y axis",
        strip_layout::move_to_new_axis(model, curve),
        &mut chosen,
    );
    // The strip's other axes of the curve's unit, each by its place.
    if let Some(place) = strip_layout::place_of(model, curve) {
        let panes = model.unit_panes();
        if let Some(pane) = panes.get(place.strip) {
            let units: Vec<&str> = std::iter::once(pane.axis)
                .chain(pane.secondary.iter().map(|axis| axis.unit))
                .collect();
            let own = units.get(place.axis).copied();
            let others: Vec<usize> = units
                .iter()
                .enumerate()
                .filter(|(axis, unit)| *axis != place.axis && Some(**unit) == own)
                .map(|(axis, _)| axis)
                .collect();
            if !others.is_empty() {
                rspice_ui_kit::widgets::menu_button(ui, "Move to axis", |ui| {
                    for axis in others {
                        let label = format!("Axis {} ({})", axis + 1, units[axis]);
                        entry(
                            ui,
                            &label,
                            strip_layout::move_to_axis(model, curve, axis),
                            &mut chosen,
                        );
                    }
                });
            }
        }
    }
    chosen
}

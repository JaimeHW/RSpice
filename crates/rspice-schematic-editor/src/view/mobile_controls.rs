//! The touch strip over the canvas.
//!
//! Every button stands in for a key or a modifier a tablet has no way to
//! press: Shift+click, Ctrl+R, Esc, Ctrl+Z, F. The strip sits inside the
//! canvas against its right edge, where the right thumb rests in
//! landscape, and shows whenever the canvas is driven by touch, at any
//! width (audit G-12).

use egui::{Color32, Context, Id, Order, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::requests::EditorRequestSource;
use rspice_ui_kit::panels::WorkbenchIcon;
use rspice_ui_kit::tokens::Tokens;

const STRIP_RIGHT_INSET: f32 = 8.0;
const STRIP_PADDING: f32 = 4.0;
const STRIP_BORDER: f32 = 1.0;
/// Between two buttons of one group.
const STRIP_GAP: f32 = 4.0;
/// Between two groups: a 1 pt rule with this much space either side.
const STRIP_RULE: f32 = 1.0;
const STRIP_RULE_SPACE: f32 = 4.0;
const STRIP_ICON: f32 = 20.0;
/// A canvas at least this tall carries the tablet strip; a shorter one, a
/// phone held in landscape, carries the five-button phone strip.
pub const TABLET_STRIP_MIN_CANVAS_HEIGHT: f32 = 420.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileCanvasAction {
    /// Latch: a one-finger drag on empty canvas draws a selection box
    /// instead of panning.
    SelectBox,
    /// Latch: a tap adds to the selection or takes out of it, as
    /// Shift+click does.
    SelectAdd,
    Rotate,
    Mirror,
    Cancel,
    Undo,
    Redo,
    Fit,
}

impl MobileCanvasAction {
    const fn icon(self) -> WorkbenchIcon {
        match self {
            Self::SelectBox => WorkbenchIcon::SelectBox,
            Self::SelectAdd => WorkbenchIcon::SelectAdd,
            Self::Rotate => WorkbenchIcon::Rotate,
            Self::Mirror => WorkbenchIcon::Mirror,
            Self::Cancel => WorkbenchIcon::Close,
            Self::Undo => WorkbenchIcon::Undo,
            Self::Redo => WorkbenchIcon::Redo,
            Self::Fit => WorkbenchIcon::ZoomFit,
        }
    }

    /// The button's accessible name and tooltip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SelectBox => "Select with a box",
            Self::SelectAdd => "Add to selection",
            Self::Rotate => "Rotate clockwise",
            Self::Mirror => "Mirror",
            Self::Cancel => "Cancel",
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Fit => "Fit drawing sheet",
        }
    }

    const fn latch(self) -> bool {
        matches!(self, Self::SelectBox | Self::SelectAdd)
    }
}

/// Which strip a canvas carries, by its height.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripLayout {
    /// Eight buttons in four groups: selection, transform, operation, view.
    Tablet,
    /// Five buttons in one group; Mirror and Redo are on the long-press
    /// menu, and the box latch gives way to Cancel while an operation is
    /// live.
    Phone,
}

use MobileCanvasAction as A;
const TABLET_GROUPS: &[&[MobileCanvasAction]] = &[
    &[A::SelectBox, A::SelectAdd],
    &[A::Rotate, A::Mirror],
    &[A::Cancel, A::Undo, A::Redo],
    &[A::Fit],
];
const PHONE_GROUPS: &[&[MobileCanvasAction]] =
    &[&[A::SelectBox, A::SelectAdd, A::Rotate, A::Undo, A::Fit]];
const PHONE_LIVE_GROUPS: &[&[MobileCanvasAction]] =
    &[&[A::Cancel, A::SelectAdd, A::Rotate, A::Undo, A::Fit]];

impl StripLayout {
    pub fn for_canvas(content_rect: Rect) -> Self {
        if content_rect.height() >= TABLET_STRIP_MIN_CANVAS_HEIGHT {
            Self::Tablet
        } else {
            Self::Phone
        }
    }

    /// The groups, top to bottom. `operation_live` is whether Cancel has
    /// something to cancel.
    pub fn groups(self, operation_live: bool) -> &'static [&'static [MobileCanvasAction]] {
        match (self, operation_live) {
            (Self::Tablet, _) => TABLET_GROUPS,
            (Self::Phone, false) => PHONE_GROUPS,
            (Self::Phone, true) => PHONE_LIVE_GROUPS,
        }
    }

    /// The strip's height with buttons `button` on a side.
    fn height(self, button: f32) -> f32 {
        let groups = self.groups(false);
        let buttons: f32 = groups
            .iter()
            .map(|group| group.len() as f32 * button + (group.len() - 1) as f32 * STRIP_GAP)
            .sum();
        let rules = (groups.len() - 1) as f32 * (STRIP_RULE + 2.0 * STRIP_RULE_SPACE);
        buttons + rules + (STRIP_PADDING + STRIP_BORDER) * 2.0
    }
}

/// The two strip latches. They belong to the input device, not to a
/// document, so they live with the canvas's egui context and outlast a
/// switch of document.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TouchLatches {
    pub select_box: bool,
    pub select_add: bool,
}

fn latches_id() -> Id {
    Id::new("rspice.schematic.touch_latches")
}

pub fn touch_latches(ctx: &Context) -> TouchLatches {
    ctx.data(|data| data.get_temp(latches_id()).unwrap_or_default())
}

pub fn set_touch_latches(ctx: &Context, latches: TouchLatches) {
    ctx.data_mut(|data| data.insert_temp(latches_id(), latches));
}

impl TouchLatches {
    fn holds(self, action: MobileCanvasAction) -> bool {
        match action {
            A::SelectBox => self.select_box,
            A::SelectAdd => self.select_add,
            _ => false,
        }
    }

    fn toggle(&mut self, action: MobileCanvasAction) {
        match action {
            A::SelectBox => self.select_box = !self.select_box,
            A::SelectAdd => self.select_add = !self.select_add,
            _ => {}
        }
    }
}

/// What the strip may offer this frame. The latches are always offered.
#[derive(Debug, Clone, Copy, Default)]
pub struct MobileCanvasCapabilities {
    pub rotate: bool,
    pub mirror: bool,
    /// A route, move, placement or box is live.
    pub cancel: bool,
    pub undo: bool,
    pub redo: bool,
    pub fit: bool,
}

impl MobileCanvasCapabilities {
    fn allows(self, action: MobileCanvasAction) -> bool {
        match action {
            A::SelectBox | A::SelectAdd => true,
            A::Rotate => self.rotate,
            A::Mirror => self.mirror,
            A::Cancel => self.cancel,
            A::Undo => self.undo,
            A::Redo => self.redo,
            A::Fit => self.fit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MobileCanvasRequest {
    pub source: EditorRequestSource,
    pub action: MobileCanvasAction,
}

/// The side of a strip button: the theme's touch target, which
/// Accessibility can raise past the 44 pt floor.
fn strip_button_side(ctx: &Context) -> f32 {
    Tokens::get(ctx).metrics.touch_target()
}

/// The strip's rectangle in a canvas: against the right edge, inset 8 pt,
/// vertically centred, and kept inside the canvas. `button` is the side of
/// one strip button.
pub fn strip_rect(content_rect: Rect, button: f32) -> Rect {
    let size = Vec2::new(
        button + (STRIP_PADDING + STRIP_BORDER) * 2.0,
        StripLayout::for_canvas(content_rect).height(button),
    );
    let top = (content_rect.center().y - size.y * 0.5).max(content_rect.top());
    Rect::from_min_size(
        egui::pos2(content_rect.right() - STRIP_RIGHT_INSET - size.x, top),
        size,
    )
}

/// Paint the strip and return the button pressed this frame. A latch
/// button toggles its latch here and is reported too, so the host can say
/// what a drag now does.
pub fn show(
    ctx: &Context,
    content_rect: Rect,
    source: &EditorRequestSource,
    capabilities: MobileCanvasCapabilities,
) -> Option<MobileCanvasRequest> {
    let layout = StripLayout::for_canvas(content_rect);
    let groups = layout.groups(capabilities.cancel);
    let button = strip_button_side(ctx);
    let rect = strip_rect(content_rect, button);
    let t = Tokens::get(ctx);
    let mut latches = touch_latches(ctx);
    let mut pending_request = None;
    let area = egui::Area::new(Id::new("workbench.design.mobile-canvas-controls"))
        .order(Order::Foreground)
        .fixed_pos(rect.min)
        // `strip_rect` already keeps the strip inside the canvas. egui's own
        // constraint would clamp the first frame against a guessed size and
        // park the strip at the canvas's left edge for that frame.
        .constrain(false)
        .show(ctx, |ui| {
            let frame = egui::Frame::new()
                .fill(with_alpha(t.color.bg_panel, 245))
                .stroke(Stroke::new(STRIP_BORDER, t.color.border_strong))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::same(STRIP_PADDING as i8))
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.set_width(button);
                    for (index, group) in groups.iter().enumerate() {
                        if index > 0 {
                            ui.add_space(STRIP_RULE_SPACE);
                            let (rule, _) = ui
                                .allocate_exact_size(Vec2::new(button, STRIP_RULE), Sense::hover());
                            ui.painter().rect_filled(rule, 0.0, t.color.border);
                            ui.add_space(STRIP_RULE_SPACE);
                        }
                        for (slot, &action) in group.iter().enumerate() {
                            if slot > 0 {
                                ui.add_space(STRIP_GAP);
                            }
                            let enabled = capabilities.allows(action);
                            let latched = latches.holds(action);
                            let response = ui
                                .add_enabled_ui(enabled, |ui| strip_button(ui, action, latched))
                                .inner;
                            // Keep a held press tied to the document it began on.
                            let press_id = response.id.with("source");
                            if response.is_pointer_button_down_on()
                                && ui.input(|input| input.pointer.primary_pressed())
                            {
                                ui.data_mut(|data| data.insert_temp(press_id, source.clone()));
                            }
                            if response.clicked() {
                                let source = ui
                                    .data_mut(|data| {
                                        let pressed =
                                            data.get_temp::<EditorRequestSource>(press_id);
                                        data.remove::<EditorRequestSource>(press_id);
                                        pressed
                                    })
                                    .unwrap_or_else(|| source.clone());
                                if action.latch() {
                                    latches.toggle(action);
                                }
                                pending_request = Some(MobileCanvasRequest { source, action });
                            } else if !response.is_pointer_button_down_on() {
                                ui.data_mut(|data| {
                                    data.remove::<EditorRequestSource>(press_id);
                                });
                            }
                        }
                    }
                });
            frame.response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Other, true, STRIP_LABEL)
            });
            ui.ctx().accesskit_node_builder(frame.response.id, |node| {
                node.set_role(egui::accesskit::Role::Toolbar);
                node.set_label(STRIP_LABEL);
            });
        });
    area.response
        .widget_info(|| egui::WidgetInfo::new(egui::WidgetType::Other));
    set_touch_latches(ctx, latches);
    pending_request
}

const STRIP_LABEL: &str = "Touch editing controls";

fn strip_button(ui: &mut Ui, action: MobileCanvasAction, latched: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::splat(strip_button_side(ui.ctx())), Sense::click());
    let label = action.label();
    response.widget_info(|| {
        if action.latch() {
            egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), latched, label)
        } else {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
        }
    });
    if ui.is_rect_visible(rect) {
        let enabled = ui.is_enabled();
        let lit = enabled && (latched || response.is_pointer_button_down_on());
        let fill = if lit {
            t.color.accent_dim
        } else if enabled && response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        };
        let stroke = Stroke::new(1.0, if lit { t.color.accent } else { t.color.border });
        ui.painter()
            .rect(rect, 0.0, fill, stroke, egui::StrokeKind::Inside);
        let color = if !enabled {
            t.color.text_faint
        } else if lit {
            t.color.accent
        } else {
            t.color.text
        };
        // No focus ring: the strip is only ever pressed by a finger, and a
        // press must not leave a ring behind (pointer-focus-no-ring).
        action.icon().paint(
            ui.painter(),
            Rect::from_center_size(rect.center(), Vec2::splat(STRIP_ICON)),
            color,
        );
    }
    response.on_hover_text(label)
}

fn with_alpha(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

#[cfg(test)]
mod tests;

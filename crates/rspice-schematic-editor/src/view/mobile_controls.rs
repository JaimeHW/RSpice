//! Touch controls over the canvas.
//!
//! The on-canvas affordances a touch device needs and a mouse does not:
//! pan/zoom handles and the tool switch, positioned so a thumb can reach
//! them in landscape.

use egui::{Align2, Color32, Context, FontId, Id, Order, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::requests::EditorRequestSource;
use rspice_ui_kit::panels::WorkbenchIcon;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

const MOBILE_CANVAS_CONTROLS_BREAKPOINT: f32 = 620.0;
const MOBILE_CANVAS_CONTROLS_RIGHT_INSET: f32 = 9.0;
const MOBILE_CANVAS_CONTROLS_BOTTOM_INSET: f32 = 10.0;
const MOBILE_CANVAS_CONTROLS_PADDING: f32 = 4.0;
const MOBILE_CANVAS_CONTROLS_BORDER: f32 = 1.0;
const MOBILE_CANVAS_CONTROLS_GAP: f32 = 4.0;
const MOBILE_CANVAS_CONTROL_SIZE: f32 = 44.0;
// The mockup lets the 11 px mono guidance wrap across the four-button track.
// Two 13.75 px lines plus its 3 px/4 px block padding round to 35 points.
const MOBILE_CANVAS_GUIDANCE_HEIGHT: f32 = 35.0;
const MOBILE_CANVAS_CONTROL_COUNT: usize = 4;
const MOBILE_CANVAS_CONTROLS_INNER_WIDTH: f32 = MOBILE_CANVAS_CONTROL_SIZE
    * MOBILE_CANVAS_CONTROL_COUNT as f32
    + MOBILE_CANVAS_CONTROLS_GAP * (MOBILE_CANVAS_CONTROL_COUNT - 1) as f32;
const MOBILE_CANVAS_CONTROLS_INNER_HEIGHT: f32 =
    MOBILE_CANVAS_GUIDANCE_HEIGHT + MOBILE_CANVAS_CONTROLS_GAP + MOBILE_CANVAS_CONTROL_SIZE;
const MOBILE_CANVAS_CONTROLS_WIDTH: f32 = MOBILE_CANVAS_CONTROLS_INNER_WIDTH
    + (MOBILE_CANVAS_CONTROLS_PADDING + MOBILE_CANVAS_CONTROLS_BORDER) * 2.0;
const MOBILE_CANVAS_CONTROLS_HEIGHT: f32 = MOBILE_CANVAS_CONTROLS_INNER_HEIGHT
    + (MOBILE_CANVAS_CONTROLS_PADDING + MOBILE_CANVAS_CONTROLS_BORDER) * 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileCanvasAction {
    ZoomOut,
    ZoomFit,
    ZoomIn,
    TouchEditGuide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MobileCanvasControl {
    action: MobileCanvasAction,
    icon: Option<WorkbenchIcon>,
    accessible_label: &'static str,
}

const MOBILE_CANVAS_CONTROLS: [MobileCanvasControl; MOBILE_CANVAS_CONTROL_COUNT] = [
    MobileCanvasControl {
        action: MobileCanvasAction::ZoomOut,
        icon: Some(WorkbenchIcon::ZoomOut),
        accessible_label: "Zoom schematic out",
    },
    MobileCanvasControl {
        action: MobileCanvasAction::ZoomFit,
        icon: Some(WorkbenchIcon::ZoomFit),
        accessible_label: "Fit complete schematic",
    },
    MobileCanvasControl {
        action: MobileCanvasAction::ZoomIn,
        icon: Some(WorkbenchIcon::ZoomIn),
        accessible_label: "Zoom schematic in",
    },
    MobileCanvasControl {
        action: MobileCanvasAction::TouchEditGuide,
        icon: None,
        accessible_label: "Open touch editing guide",
    },
];

#[derive(Debug, Clone, Copy, Default)]
pub struct MobileCanvasCapabilities {
    pub zoom_out: bool,
    pub zoom_fit: bool,
    pub zoom_in: bool,
    pub touch_edit_guide: bool,
}

impl MobileCanvasCapabilities {
    fn allows(self, action: MobileCanvasAction) -> bool {
        match action {
            MobileCanvasAction::ZoomOut => self.zoom_out,
            MobileCanvasAction::ZoomFit => self.zoom_fit,
            MobileCanvasAction::ZoomIn => self.zoom_in,
            MobileCanvasAction::TouchEditGuide => self.touch_edit_guide,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MobileCanvasRequest {
    pub source: EditorRequestSource,
    pub action: MobileCanvasAction,
}

pub fn show(
    ctx: &Context,
    content_rect: Rect,
    source: &EditorRequestSource,
    capabilities: MobileCanvasCapabilities,
) -> Option<MobileCanvasRequest> {
    let viewport_width = ctx.content_rect().width();
    let control_rect = mobile_canvas_controls_rect(content_rect, viewport_width)?;
    let t = Tokens::get(ctx);
    let mut pending_request = None;
    let area = egui::Area::new(Id::new("workbench.design.mobile-canvas-controls"))
        .order(Order::Foreground)
        .fixed_pos(control_rect.min)
        .constrain_to(content_rect)
        .show(ctx, |ui| {
            let frame =
                egui::Frame::new()
                    .fill(with_alpha(t.color.bg_panel, 245))
                    .stroke(Stroke::new(
                        MOBILE_CANVAS_CONTROLS_BORDER,
                        t.color.border_strong,
                    ))
                    .corner_radius(t.radius)
                    .inner_margin(egui::Margin::same(MOBILE_CANVAS_CONTROLS_PADDING as i8))
                    .shadow(t.shadow())
                    .show(ui, |ui| {
                        ui.set_min_size(Vec2::new(
                            MOBILE_CANVAS_CONTROLS_INNER_WIDTH,
                            MOBILE_CANVAS_CONTROLS_INNER_HEIGHT,
                        ));
                        ui.set_max_size(Vec2::new(
                            MOBILE_CANVAS_CONTROLS_INNER_WIDTH,
                            MOBILE_CANVAS_CONTROLS_INNER_HEIGHT,
                        ));
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        ui.allocate_ui_with_layout(
                            Vec2::new(
                                MOBILE_CANVAS_CONTROLS_INNER_WIDTH,
                                MOBILE_CANVAS_GUIDANCE_HEIGHT,
                            ),
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                ui.add_sized(
                                Vec2::new(
                                    MOBILE_CANVAS_CONTROLS_INNER_WIDTH - 10.0,
                                    MOBILE_CANVAS_GUIDANCE_HEIGHT,
                                ),
                                egui::Label::new(egui::RichText::new(
                                    "Drag to pan \u{00b7} pinch to zoom \u{00b7} tap to select",
                                )
                                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.text_dim))
                                .wrap()
                                .halign(egui::Align::Center),
                            );
                            },
                        );
                        ui.add_space(MOBILE_CANVAS_CONTROLS_GAP);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = MOBILE_CANVAS_CONTROLS_GAP;
                            for control in MOBILE_CANVAS_CONTROLS {
                                let enabled = capabilities.allows(control.action);
                                let response = ui
                                    .add_enabled_ui(enabled, |ui| mobile_canvas_button(ui, control))
                                    .inner;
                                // Keep a held pointer press tied to its original document.
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
                                    pending_request = Some(MobileCanvasRequest {
                                        source,
                                        action: control.action,
                                    });
                                } else if !response.is_pointer_button_down_on() {
                                    ui.data_mut(|data| {
                                        data.remove::<EditorRequestSource>(press_id);
                                    });
                                }
                            }
                        });
                    });
            frame.response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Other,
                    true,
                    "Touch schematic viewport controls",
                )
            });
            ui.ctx().accesskit_node_builder(frame.response.id, |node| {
                node.set_role(egui::accesskit::Role::Toolbar);
                node.set_label("Touch schematic viewport controls");
            });
        });
    area.response
        .widget_info(|| egui::WidgetInfo::new(egui::WidgetType::Other));

    pending_request
}

pub fn mobile_canvas_controls_rect(content_rect: Rect, viewport_width: f32) -> Option<Rect> {
    if viewport_width > MOBILE_CANVAS_CONTROLS_BREAKPOINT {
        return None;
    }
    let right_bottom = content_rect.right_bottom()
        - egui::vec2(
            MOBILE_CANVAS_CONTROLS_RIGHT_INSET,
            MOBILE_CANVAS_CONTROLS_BOTTOM_INSET,
        );
    Some(Rect::from_min_size(
        right_bottom - egui::vec2(MOBILE_CANVAS_CONTROLS_WIDTH, MOBILE_CANVAS_CONTROLS_HEIGHT),
        egui::vec2(MOBILE_CANVAS_CONTROLS_WIDTH, MOBILE_CANVAS_CONTROLS_HEIGHT),
    ))
}

fn mobile_canvas_button(ui: &mut Ui, control: MobileCanvasControl) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::splat(MOBILE_CANVAS_CONTROL_SIZE), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            control.accessible_label,
        )
    });
    if ui.is_rect_visible(rect) {
        let pressed = ui.is_enabled() && response.is_pointer_button_down_on();
        let hovered = ui.is_enabled() && response.hovered();
        let fill = if pressed {
            t.color.accent_dim
        } else if hovered {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        };
        let stroke = if pressed {
            Stroke::new(1.0, t.color.accent)
        } else {
            Stroke::new(1.0, t.color.border)
        };
        ui.painter()
            .rect(rect, 0.0, fill, stroke, egui::StrokeKind::Inside);
        let color = if !ui.is_enabled() {
            t.color.text_faint
        } else if pressed {
            t.color.accent
        } else {
            t.color.text
        };
        if let Some(icon) = control.icon {
            icon.paint(
                ui.painter(),
                Rect::from_center_size(rect.center(), Vec2::splat(16.0)),
                color,
            );
        } else {
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "?",
                FontId::new(tokens::FS_3, egui::FontFamily::Proportional),
                color,
            );
        }
        theme::paint_focus_ring_outset(ui, &response, rect);
    }
    response.on_hover_text(control.accessible_label)
}

fn with_alpha(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

#[cfg(test)]
mod tests;

use egui::{Align2, Color32, Context, FontId, Id, Order, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::common::{ConsoleMessage, RSpiceApp};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::commands::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::{
    CapabilityWorkflowId, RouteTransitionSource, SurfaceRoute, route_availability,
};

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
enum MobileCanvasAction {
    Command(Command),
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
        action: MobileCanvasAction::Command(Command::ZoomOut),
        icon: Some(WorkbenchIcon::ZoomOut),
        accessible_label: "Zoom schematic out",
    },
    MobileCanvasControl {
        action: MobileCanvasAction::Command(Command::ZoomFit),
        icon: Some(WorkbenchIcon::ZoomFit),
        accessible_label: "Fit complete schematic",
    },
    MobileCanvasControl {
        action: MobileCanvasAction::Command(Command::ZoomIn),
        icon: Some(WorkbenchIcon::ZoomIn),
        accessible_label: "Zoom schematic in",
    },
    MobileCanvasControl {
        action: MobileCanvasAction::TouchEditGuide,
        icon: None,
        accessible_label: "Open touch editing guide",
    },
];

pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp, content_rect: Rect) {
    let viewport_width = ctx.content_rect().width();
    let Some(control_rect) = mobile_canvas_controls_rect(content_rect, viewport_width) else {
        return;
    };
    let t = Tokens::get(ctx);
    let mut pending_action = None;
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
                                let enabled = action_enabled(app, control.action);
                                let response = ui
                                    .add_enabled_ui(enabled, |ui| mobile_canvas_button(ui, control))
                                    .inner;
                                if response.clicked() {
                                    pending_action = Some(control.action);
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

    if let Some(action) = pending_action {
        execute_action(app, action);
    }
}

fn mobile_canvas_controls_rect(content_rect: Rect, viewport_width: f32) -> Option<Rect> {
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

fn action_enabled(app: &RSpiceApp, action: MobileCanvasAction) -> bool {
    match action {
        MobileCanvasAction::Command(command) => command.is_enabled(app),
        MobileCanvasAction::TouchEditGuide => route_availability(
            SurfaceRoute::capability_workflow(CapabilityWorkflowId::TouchEditGuide),
        )
        .can_open(),
    }
}

fn execute_action(app: &mut RSpiceApp, action: MobileCanvasAction) {
    match action {
        MobileCanvasAction::Command(command) => command.execute(app),
        MobileCanvasAction::TouchEditGuide => {
            let route = SurfaceRoute::capability_workflow(CapabilityWorkflowId::TouchEditGuide);
            if let Err(error) = app
                .state
                .workbench
                .navigate(route, RouteTransitionSource::User)
            {
                app.state
                    .push_user_message(ConsoleMessage::warning(error.to_string()));
            }
        }
    }
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
mod tests {
    use super::*;
    use crate::workbench::SurfaceId;
    use crate::workbench::state::Workspace;

    #[test]
    fn compact_geometry_matches_the_mockup_four_by_forty_four_cluster() {
        let content = Rect::from_min_size(egui::pos2(7.0, 91.0), egui::vec2(606.0, 700.0));
        let rect = mobile_canvas_controls_rect(content, 620.0)
            .expect("the controls appear at the inclusive 620 px breakpoint");

        assert_eq!(rect.width(), 198.0);
        assert_eq!(rect.height(), 93.0);
        assert_eq!(content.right() - rect.right(), 9.0);
        assert_eq!(content.bottom() - rect.bottom(), 10.0);
        assert!(mobile_canvas_controls_rect(content, 620.01).is_none());
    }

    #[test]
    fn controls_keep_the_exact_mockup_order_and_real_command_bindings() {
        assert_eq!(
            MOBILE_CANVAS_CONTROLS.map(|control| control.action),
            [
                MobileCanvasAction::Command(Command::ZoomOut),
                MobileCanvasAction::Command(Command::ZoomFit),
                MobileCanvasAction::Command(Command::ZoomIn),
                MobileCanvasAction::TouchEditGuide,
            ]
        );
        assert!(
            MOBILE_CANVAS_CONTROLS
                .iter()
                .all(|control| !control.accessible_label.is_empty())
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn rendered_controls_preserve_four_touch_targets_and_four_point_gaps() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(620.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show(ctx, &mut app, ui.max_rect()));
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("touch-control accessibility tree")
            .nodes;
        let cluster = nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Toolbar
                    && node.label() == Some("Touch schematic viewport controls")
            })
            .and_then(|(_, node)| node.bounds())
            .expect("touch-control cluster");
        assert_eq!(
            cluster.x1 - cluster.x0,
            f64::from(MOBILE_CANVAS_CONTROLS_WIDTH)
        );
        assert_eq!(
            cluster.y1 - cluster.y0,
            f64::from(MOBILE_CANVAS_CONTROLS_HEIGHT)
        );
        let mut bounds = MOBILE_CANVAS_CONTROLS.map(|control| {
            nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::Button
                        && node.label() == Some(control.accessible_label)
                })
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("missing {}", control.accessible_label))
        });
        bounds.sort_by(|left, right| left.x0.total_cmp(&right.x0));

        for bound in &bounds {
            assert_eq!(bound.x1 - bound.x0, f64::from(MOBILE_CANVAS_CONTROL_SIZE));
            assert_eq!(bound.y1 - bound.y0, f64::from(MOBILE_CANVAS_CONTROL_SIZE));
        }
        for pair in bounds.windows(2) {
            assert_eq!(
                pair[1].x0 - pair[0].x1,
                f64::from(MOBILE_CANVAS_CONTROLS_GAP)
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn actions_execute_the_schematic_camera_and_touch_guide_route() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        assert!(matches!(
            app.state.workspace.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        ));
        app.state.schematic.zoom = 1.0;

        execute_action(&mut app, MobileCanvasAction::Command(Command::ZoomOut));
        assert_eq!(app.state.schematic.zoom, 0.8);
        execute_action(&mut app, MobileCanvasAction::Command(Command::ZoomIn));
        assert_eq!(app.state.schematic.zoom, 1.0);

        assert!(!app.state.schematic.needs_fit);
        execute_action(&mut app, MobileCanvasAction::Command(Command::ZoomFit));
        assert!(app.state.schematic.needs_fit);

        execute_action(&mut app, MobileCanvasAction::TouchEditGuide);
        assert_eq!(
            app.state.workbench.current_route(),
            SurfaceRoute::capability_workflow(CapabilityWorkflowId::TouchEditGuide)
        );
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::FeatureAvailability
        );
    }
}

//! Touch geometry and accessibility independent of application routing.
use super::*;

fn source() -> EditorRequestSource {
    EditorRequestSource {
        project: rspice_app_types::product::ProjectId::new(),
        document: rspice_design_model::cell_view::CellViewRef::new("work", "amp", "schematic"),
        occurrence: None,
        design_epoch: 1,
        document_epoch: 2,
        content_version: 3,
        topology_version: 4,
        symbol_revision: 5,
        sheet: None,
    }
}

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
fn controls_keep_the_action_order_and_accessible_labels() {
    assert_eq!(
        MOBILE_CANVAS_CONTROLS.map(|control| control.action),
        [
            MobileCanvasAction::ZoomOut,
            MobileCanvasAction::ZoomFit,
            MobileCanvasAction::ZoomIn,
            MobileCanvasAction::TouchEditGuide,
        ]
    );
    assert!(
        MOBILE_CANVAS_CONTROLS
            .iter()
            .all(|control| !control.accessible_label.is_empty())
    );
}

#[test]
fn pointer_requests_keep_the_pressed_document_and_honor_disabled_controls() {
    for change in ["none", "document", "disabled"] {
        let ctx = Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let original = source();
        let mut current = original.clone();
        let mut target = None;
        let mut requested = None;
        for frame in 0..4 {
            if frame == 3 && change == "document" {
                current.document_epoch += 1;
            }
            let mut events = Vec::new();
            if frame >= 2 {
                let pos = target.expect("zoom-in button");
                events.push(egui::Event::PointerMoved(pos));
                events.push(egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: frame == 2,
                    modifiers: egui::Modifiers::NONE,
                });
            }
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(620.0, 800.0),
                    )),
                    events,
                    ..Default::default()
                },
                |root| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(root, |ui| {
                            if let Some(request) = show(
                                ui.ctx(),
                                ui.max_rect(),
                                &current,
                                MobileCanvasCapabilities {
                                    zoom_in: change != "disabled",
                                    ..Default::default()
                                },
                            ) {
                                requested = Some(request);
                            }
                        });
                },
            );
            if frame < 2 {
                target = output
                    .platform_output
                    .accesskit_update
                    .unwrap()
                    .nodes
                    .iter()
                    .find(|(_, node)| node.label() == Some("Zoom schematic in"))
                    .and_then(|(_, node)| node.bounds())
                    .map(|bounds| {
                        egui::pos2(
                            ((bounds.x0 + bounds.x1) / 2.0) as f32,
                            ((bounds.y0 + bounds.y1) / 2.0) as f32,
                        )
                    });
            }
        }
        if change == "disabled" {
            assert!(requested.is_none());
        } else {
            let request = requested.expect("enabled zoom-in request");
            assert_eq!(request.action, MobileCanvasAction::ZoomIn);
            assert_eq!(request.source, original);
        }
    }
}

#[test]
fn rendered_controls_preserve_four_touch_targets_and_four_point_gaps() {
    let ctx = Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let source = source();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(620.0, 800.0),
            )),
            ..Default::default()
        },
        |root| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(root, |ui| {
                    let ctx = ui.ctx().clone();
                    show(
                        &ctx,
                        ui.max_rect(),
                        &source,
                        MobileCanvasCapabilities {
                            zoom_out: true,
                            zoom_fit: true,
                            zoom_in: true,
                            touch_edit_guide: true,
                        },
                    );
                });
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

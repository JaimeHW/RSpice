//! Real toolbar geometry and pointer actions across pane widths and input sizes.

use super::*;
use egui::{Context, Event, Pos2};

struct Fixture {
    app: RSpiceApp,
    ctx: Context,
    width: f32,
    time: f64,
    buttons: Vec<(String, Rect)>,
    status: Vec<Rect>,
    viewport: Rect,
}

impl Fixture {
    fn new(width: f32, coarse: bool, revision: u32) -> Self {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.coarse_pointer = coarse;
        commit(
            &mut app,
            RunSetAction::AddDimension(RunSetDimensionKind::Parameter),
        );
        commit(
            &mut app,
            RunSetAction::AddDimension(RunSetDimensionKind::Parameter),
        );
        commit(&mut app, RunSetAction::Undo);
        app.state.sim_setup.run_set.revision = revision;
        let mut fixture = Self {
            app,
            ctx,
            width,
            time: 0.0,
            buttons: vec![],
            status: vec![],
            viewport: Rect::NOTHING,
        };
        fixture.frame(vec![]);
        fixture.frame(vec![]);
        fixture
    }

    fn frame(&mut self, events: Vec<Event>) {
        self.time += 0.02;
        let validation = RunSetFrame::resolve(&self.app).validation;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(self.width, 400.0))),
                events,
                time: Some(self.time),
                focused: true,
                ..Default::default()
            },
            |root| {
                self.viewport = root.ctx().content_rect();
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(root, |ui| {
                        toolbar::show(ui, &mut self.app, &validation);
                    });
            },
        );
        self.buttons = output
            .platform_output
            .accesskit_update
            .unwrap()
            .nodes
            .into_iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
            .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.bounds()?)))
            .map(|(name, rect)| {
                (
                    name,
                    Rect::from_min_max(
                        egui::pos2(rect.x0 as f32, rect.y0 as f32),
                        egui::pos2(rect.x1 as f32, rect.y1 as f32),
                    ),
                )
            })
            .collect();
        self.status.clear();
        for clipped in output.shapes {
            collect_status(&clipped.shape, &mut self.status);
        }
    }

    fn click(&mut self, name: &str) {
        let at = self
            .buttons
            .iter()
            .find(|(label, _)| label == name)
            .unwrap()
            .1
            .center();
        for pressed in [true, false] {
            let mut events = vec![
                Event::PointerMoved(at),
                Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ];
            if self.app.state.workbench.coarse_pointer {
                events.push(Event::Touch {
                    device_id: egui::TouchDeviceId(1),
                    id: egui::TouchId(1),
                    phase: if pressed {
                        egui::TouchPhase::Start
                    } else {
                        egui::TouchPhase::End
                    },
                    pos: at,
                    force: None,
                });
            }
            self.frame(events);
        }
    }
}

fn collect_status(shape: &egui::epaint::Shape, bounds: &mut Vec<Rect>) {
    match shape {
        egui::epaint::Shape::Text(text) if text.galley.job.text.starts_with("working revision") => {
            bounds.push(text.galley.rect.translate(text.pos.to_vec2()));
        }
        egui::epaint::Shape::Vec(shapes) => {
            for shape in shapes {
                collect_status(shape, bounds);
            }
        }
        _ => {}
    }
}

#[test]
fn run_set_toolbar_keeps_status_and_every_control_inside_separate_bounds() {
    for (width, zoom) in [
        (560.0, 1.0),
        (280.0, 1.0),
        (320.0, 1.0),
        (820.0, 1.0),
        (1100.0, 1.0),
        (560.0, 2.0),
        (1100.0, 1.5),
    ] {
        for coarse in [false, true] {
            for revision in [3, u32::MAX] {
                let mut fixture = Fixture::new(width, coarse, revision);
                if zoom != 1.0 {
                    fixture.ctx.set_zoom_factor(zoom);
                    fixture.frame(vec![]);
                    fixture.frame(vec![]);
                }
                assert_eq!(fixture.buttons.len(), 4);
                assert_eq!(fixture.status.len(), 1);
                let mut regions = fixture
                    .buttons
                    .iter()
                    .map(|(name, rect)| (name.as_str(), *rect))
                    .collect::<Vec<_>>();
                regions.push(("status", fixture.status[0]));
                for (index, (name, rect)) in regions.iter().enumerate() {
                    assert!(
                        fixture.viewport.contains_rect(*rect),
                        "{name} leaves {width}-point pane: {rect:?}"
                    );
                    if coarse && *name != "status" {
                        assert!(rect.height() >= 44.0, "{name} touch target: {rect:?}");
                    }
                    for (other, other_rect) in &regions[index + 1..] {
                        assert!(
                            !rect.intersects(*other_rect),
                            "{name} overlaps {other} at {width}: {rect:?} vs {other_rect:?}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn run_set_toolbar_pointer_actions_reach_their_own_controls_at_narrow_widths() {
    for width in [280.0, 560.0] {
        let mut fixture = Fixture::new(width, true, 3);
        let dimensions = fixture.app.state.sim_setup.run_set.dimensions.len();
        fixture.click("Undo");
        assert_eq!(
            fixture.app.state.sim_setup.run_set.dimensions.len(),
            dimensions - 1
        );
        fixture.click("Redo");
        assert_eq!(
            fixture.app.state.sim_setup.run_set.dimensions.len(),
            dimensions
        );
        let receipts = fixture.app.state.sim_setup.run_set.receipts.len();
        fixture.click("Validate and preview");
        assert!(fixture.app.state.sim_setup.run_set.receipts.len() > receipts);
        fixture.click("Add dimension");
        fixture.frame(vec![]);
        let choice = fixture
            .buttons
            .iter()
            .find(|(name, _)| name == "Design parameter")
            .unwrap();
        assert!(choice.1.height() >= 44.0);
        fixture.click("Design parameter");
        assert_eq!(
            fixture.app.state.sim_setup.run_set.dimensions.len(),
            dimensions + 1
        );
    }
}

//! What an analysis form's fields hold after they are typed into.
//!
//! Driven through the route rather than through the row functions: the rule
//! under test is about focus, and focus only exists once the field is a real
//! widget on a real surface. The fields are found by the name they announce,
//! which is the same name the accessibility sweep next door asserts they have.

use super::stage::show_on_stage;
use crate::simulation::plan::{AnalysisDraft, AnalysisInstance, AnalysisKind};
use crate::state::SimulationPage;
use crate::workbench::RSpiceApp;
use egui::{Pos2, Rect, pos2, vec2};

const SURFACE: (f32, f32) = (1280.0, 1400.0);

/// One studio route, driven a frame at a time with real input events.
struct Route {
    ctx: egui::Context,
    app: RSpiceApp,
    fields: Vec<(String, Rect)>,
}

impl Route {
    fn analyses(kind: AnalysisKind) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.simulate.simulation_page = SimulationPage::Analyses;
        let selected = app
            .state
            .model
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .and_then(|plan| {
                plan.instances()
                    .iter()
                    .find(|instance| instance.kind() == kind)
                    .map(AnalysisInstance::id)
            })
            .unwrap_or_else(|| panic!("the test plan holds a {kind:?}"));
        app.state.simulate.active_analysis_instance = Some(selected);
        let mut route = Self {
            ctx,
            app,
            fields: Vec::new(),
        };
        // Two settling passes: the surface resolves its content width against
        // the scrollbar track it reserves, which it only knows on a second.
        route.pass(Vec::new());
        route.pass(Vec::new());
        route
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(SURFACE.0, SURFACE.1))),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show_on_stage(ui, app));
            },
        );
        self.fields = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter(|(_, node)| node.role() == egui::accesskit::Role::TextInput)
                    .filter_map(|(_, node)| {
                        let label = node.label()?.to_owned();
                        let bounds = node.bounds()?;
                        Some((
                            label,
                            Rect::from_min_max(
                                pos2(bounds.x0 as f32, bounds.y0 as f32),
                                pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    fn click(&mut self, at: Pos2) {
        self.pass(vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ]);
        self.pass(Vec::new());
    }

    /// Select the whole of the named field, replace it with `text`, and move
    /// focus off it the way Tab does.
    fn retype(&mut self, field: &str, text: &str) {
        let rect = self
            .fields
            .iter()
            .find(|(label, _)| label == field)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| {
                panic!(
                    "the form announces no field called {field:?}; it announces {:?}",
                    self.fields
                        .iter()
                        .map(|(label, _)| label.as_str())
                        .collect::<Vec<_>>()
                )
            });
        self.click(rect.center());
        self.pass(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            egui::Event::Text(text.to_owned()),
        ]);
        self.pass(vec![egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }]);
        self.pass(Vec::new());
    }

    fn transient_stop(&self) -> String {
        let plan = self
            .app
            .state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("the plan resolves");
        let instance = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Transient)
            .expect("the plan holds a transient");
        match instance.draft() {
            AnalysisDraft::Transient(setup) => setup.stop.clone(),
            other => panic!("a transient instance holds {other:?}"),
        }
    }
}

#[test]
fn transient_checkpoint_interval_edit_reaches_the_authored_plan() {
    let mut route = Route::analyses(AnalysisKind::Transient);
    let selected = route.app.state.simulate.active_analysis_instance.unwrap();
    route
        .app
        .state
        .model
        .sim_setup
        .analysis_plan
        .as_mut()
        .unwrap()
        .edit(selected, |draft| {
            let AnalysisDraft::Transient(draft) = draft else {
                unreachable!()
            };
            draft.checkpoint = Some(Default::default());
        })
        .unwrap();
    route.pass(Vec::new());
    route.pass(Vec::new());
    route.retype("Checkpoint interval", "7n");
    let plan = route
        .app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .unwrap();
    let instance = plan
        .instances()
        .iter()
        .find(|instance| instance.id() == selected)
        .unwrap();
    let AnalysisDraft::Transient(draft) = instance.draft() else {
        unreachable!()
    };
    let interval = draft
        .checkpoint
        .as_ref()
        .unwrap()
        .capture_interval()
        .unwrap()
        .unwrap();
    assert!((interval - 7e-9).abs() < 1e-23);
}

/// A spelling only the form understands is rewritten — in the deck's own
/// notation, never a 17-digit exponent.
///
/// The field holds the text the deck is generated from, and
/// `parse_spice_value_checked` does not know the unit letters the form accepts:
/// `5ms` reaches it as an unsupported suffix, so a field left holding it reads
/// one value and runs none.
#[test]
fn a_spelling_the_deck_cannot_read_is_rewritten_in_engineering_notation() {
    let mut route = Route::analyses(AnalysisKind::Transient);
    route.retype("Stop time", "5ms");
    let rewritten = route.transient_stop();
    assert_eq!(
        rewritten,
        crate::state::format_engineering(5.0e-3),
        "the rewrite goes through the deck's own formatter"
    );
    assert!(
        !rewritten.contains('e'),
        "a 17-digit exponent is not engineering notation: {rewritten}"
    );
    assert!(
        crate::quantity::spice_value::parse_spice_value_checked(&rewritten).is_ok(),
        "whatever is left behind has to be a value the deck can read: {rewritten}"
    );
}

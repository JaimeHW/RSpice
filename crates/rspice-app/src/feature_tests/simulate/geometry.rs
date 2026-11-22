//! Where this surface puts what it paints.
//!
//! `tests` judges what the studio says and `click_tests` what a press on it
//! does. This is the third question and the one neither can ask: two things
//! laid out by two separate measurements have to agree about the space
//! between them, and only their coordinates answer that.

use super::stage::show_on_stage;
use egui::{Rect, vec2};
use rspice_app_simulate::workbench::surfaces::simulate::ANALYSIS_CATEGORY_ORDER;
use rspice_app_simulate::workbench::surfaces::simulate::lifecycle::{
    analysis_catalog_group, insert_analysis_instance,
};

use crate::workbench::RSpiceApp;
use rspice_app_state::simulation::plan::AnalysisKind;

/// One painted line of the Analyses route: its text and where it landed.
#[derive(Clone, Debug)]
struct PaintedLine {
    text: String,
    left: f32,
    right: f32,
    top: f32,
}

/// Every line of text a rendered frame painted, and where each landed.
fn painted_lines(shapes: &[egui::epaint::ClippedShape]) -> Vec<PaintedLine> {
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<PaintedLine>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push(PaintedLine {
                text: text.galley.job.text.clone(),
                left: text.pos.x,
                right: text.pos.x + text.galley.size().x,
                top: text.pos.y,
            }),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }

    let mut lines = Vec::new();
    for clipped in shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
}

/// Render the Analyses route at `width` and report every line it painted.
///
/// Positions, not just wording. The header's identity line and the
/// availability chip beside it are laid out by two separate measurements —
/// the chip is measured from the trailing inset, the line gets whatever is
/// left — and whether they agree is a question about coordinates that no
/// assertion over text alone can ask.
fn analyses_route_painted_lines(width: f32) -> Vec<PaintedLine> {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.simulate.simulation_page = rspice_app_state::state::SimulationPage::Analyses;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 900.0))),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| show_on_stage(ui, &mut app));
        },
    );
    painted_lines(&output.shapes)
}

/// The analysis header states the kind and lifecycle, never the instance's
/// UUID, and a production analysis says nothing about its availability.
///
/// The line used to read `<kind> · <id> · lifecycle <state>` beside an
/// availability chip that said "Production" in green on every default
/// analysis. The identity is internal and the default is not news, so the line
/// is `<kind> · <state>` and the trailing edge holds only the Name… action,
/// which the line stops short of. These are viewport widths; the surface
/// withholds its scrollbar's thirteen-point track from them.
#[test]
fn the_analysis_header_states_kind_and_lifecycle_short_of_its_action() {
    /// The header band: the title, the identity line and the action are the
    /// only things painted in the top 120 points of the form.
    const HEADER_BAND: f32 = 120.0;

    for width in [1016.0, 860.0] {
        let lines = analyses_route_painted_lines(width);
        let header: Vec<_> = lines
            .iter()
            .filter(|line| line.top < HEADER_BAND)
            .cloned()
            .collect();
        assert!(
            !header.iter().any(|line| line.text == "Production"),
            "a production analysis states no availability at {width}: {header:?}"
        );
        let identity = header
            .iter()
            .find(|line| line.text == "Transient · draft")
            .unwrap_or_else(|| {
                panic!("the header states the kind and lifecycle at {width}: {header:?}")
            });
        let action = header
            .iter()
            .find(|line| line.text == "Name\u{2026}")
            .unwrap_or_else(|| panic!("the header offers Name… at {width}: {header:?}"));
        assert!(
            identity.right < action.left,
            "the identity line runs to {} and the action starts at {}",
            identity.right,
            action.left
        );
    }
}

/// An option field states its value and nothing beside its caption.
///
/// Every option field once carried a tag on its caption row, and every tag
/// read `default` — including the fields holding the plan's own value, where
/// it said nothing the value did not. An empty well the engine resolves now
/// reads `default` inside itself, below its caption, and a well holding a value
/// shows only the value.
#[test]
fn an_option_field_states_default_only_inside_an_empty_well() {
    use rspice_app_state::simulation::plan::NumericOverrideOption as O;

    let lines = analyses_form_lines(AnalysisKind::Transient, 900.0)
        .expect("the default plan holds a transient");
    let caption = |label: &str| {
        lines
            .iter()
            .find(|line| line.text == label)
            .unwrap_or_else(|| panic!("the transient form draws {label:?}"))
            .clone()
    };
    // The plan states a truncation bound, so its well holds a number; the
    // plan states no LTE bound, so that well is empty.
    let valued = caption(O::Trtol.label());
    let empty = caption(O::LteReltol.label());
    let on_row = |row: &PaintedLine| {
        lines
            .iter()
            .filter(|line| (line.top - row.top).abs() < 0.5)
            .map(|line| line.text.clone())
            .collect::<Vec<_>>()
    };
    for row in [&valued, &empty] {
        let beside = on_row(row);
        assert!(
            beside
                .iter()
                .all(|text| !matches!(text.as_str(), "default" | "override" | "study default")),
            "{:?} carries a tag on its caption row: {beside:?}",
            row.text
        );
    }
    let placeholder = lines.iter().find(|line| {
        line.text == "default"
            && line.top > empty.top
            && line.top < empty.top + 40.0
            && line.left >= empty.left
    });
    assert!(
        placeholder.is_some(),
        "the empty LTE well reads `default` inside itself"
    );
    assert!(
        !lines.iter().any(|line| line.text == "default"
            && line.top > valued.top
            && line.top < valued.top + 40.0
            && (line.left - valued.left).abs() < 20.0),
        "a well holding the plan's value shows only the value"
    );
}

/// The Analyses route with one analysis of `kind` selected, at `width`.
///
/// The kind is inserted where the default plan does not hold one. `None` where
/// the plan refuses the insert, which is a kind that has no form to measure.
fn analyses_form_lines(kind: AnalysisKind, width: f32) -> Option<Vec<PaintedLine>> {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let plan = app.state.model.sim_setup.stable_analysis_plan_mut().ok()?;
    let instance = match plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == kind)
    {
        Some(instance) => instance.id(),
        None => plan.insert(kind).ok()?.0,
    };
    app.state.simulate.simulation_page = rspice_app_state::state::SimulationPage::Analyses;
    app.state.simulate.active_analysis_instance = Some(instance);
    let mut run = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 2_600.0))),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show_on_stage(ui, &mut app));
            },
        )
    };
    // The surface resolves its content width against the scrollbar track it
    // reserves, which it only knows on a second pass.
    let _ = run();
    Some(painted_lines(&run().shapes))
}

/// The window the split-route scrolling cases are drawn in.
const ROUTE_WIDTH: f32 = 1_440.0;

/// The stack's first group header, as painted: the stack's topmost line.
const FIRST_GROUP: &str = "Core analyses";

/// The Analyses route held open in a 1440-point window across many passes, so
/// a wheel's eased scroll can run its course between two measurements.
struct AnalysesRoute {
    ctx: egui::Context,
    app: RSpiceApp,
    height: f32,
}

impl AnalysesRoute {
    fn open(mut app: RSpiceApp, height: f32) -> Self {
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        app.state.simulate.simulation_page = rspice_app_state::state::SimulationPage::Analyses;
        let mut route = Self { ctx, app, height };
        route.settle(Vec::new());
        route
    }

    fn pass(&mut self, events: Vec<egui::Event>) -> Vec<PaintedLine> {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(ROUTE_WIDTH, self.height),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show_on_stage(ui, app));
            },
        );
        painted_lines(&output.shapes)
    }

    /// `events`, then passes until anything they set moving has stopped.
    fn settle(&mut self, events: Vec<egui::Event>) -> Vec<PaintedLine> {
        let mut lines = self.pass(events);
        for _ in 0..60 {
            lines = self.pass(Vec::new());
        }
        lines
    }
}

/// A plan holding one instance of every kind it accepts, which is far more
/// than a 900-point window shows, with its transient analysis selected: the
/// longest form, and one that runs past the window too.
fn crowded_plan() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test plan resolves");
    for kind in AnalysisKind::MANIFEST_ORDER {
        if plan
            .instances()
            .iter()
            .all(|instance| instance.kind() != kind)
        {
            let _ = plan.insert(kind);
        }
    }
    select_transient(&mut app.state);
    app
}

fn select_transient(state: &mut crate::workbench::AppState) {
    let plan = state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test plan resolves");
    let transient = match plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
    {
        Some(instance) => instance.id(),
        None => plan.insert(AnalysisKind::Transient).expect("transient").0,
    };
    state.simulate.active_analysis_instance = Some(transient);
}

fn top_of(lines: &[PaintedLine], text: &str) -> f32 {
    lines
        .iter()
        .find(|line| line.text == text)
        .unwrap_or_else(|| panic!("{text:?} is painted"))
        .top
}

/// An analysis made active from outside the stack is scrolled into its view.
///
/// The stack scrolls on its own now, so the analysis the catalogue inserts and
/// selects can land below what the stack shows. The catalogue's own two steps
/// are taken here — insert, then select — with the stack at its top and the
/// new row at its far end.
#[test]
fn an_analysis_made_active_below_the_stacks_view_is_scrolled_into_it() {
    let mut route = AnalysesRoute::open(crowded_plan(), 900.0);
    let first = route.settle(Vec::new());
    let top = top_of(&first, FIRST_GROUP);
    // The lowest group the plan holds a kind in.
    let last_kind = {
        let plan = route
            .app
            .state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("the test plan resolves");
        let insertable = plan
            .instances()
            .iter()
            .map(|instance| instance.kind())
            .collect::<Vec<_>>();
        ANALYSIS_CATEGORY_ORDER
            .into_iter()
            .rev()
            .find_map(|group| {
                insertable
                    .iter()
                    .copied()
                    .find(|kind| analysis_catalog_group(*kind) == group)
            })
            .expect("the plan holds an analysis")
    };
    let inserted = insert_analysis_instance(
        &mut crate::workbench::features::cx(&mut route.app),
        last_kind,
    )
    .expect("a second instance of the kind inserts");
    route.app.state.simulate.active_analysis_instance = Some(inserted);

    let after = route.settle(Vec::new());
    let moved = after
        .iter()
        .find(|line| line.text == FIRST_GROUP)
        .map(|line| line.top);
    assert!(
        moved.is_none_or(|moved| moved < top - 40.0),
        "the stack scrolled down to the new row: its first group stayed at {moved:?}"
    );
}

/// The Analyses route drawn in a 1440-point window whose docks leave the
/// centre `centre` points wide, at the window's right edge.
fn analyses_route_in_centre(centre: f32) -> Vec<PaintedLine> {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.simulate.simulation_page = rspice_app_state::state::SimulationPage::Analyses;
    let mut run = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(ROUTE_WIDTH, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        let rect = Rect::from_min_max(
                            egui::pos2(ROUTE_WIDTH - centre, 0.0),
                            egui::pos2(ROUTE_WIDTH, 900.0),
                        );
                        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                            show_on_stage(ui, &mut app);
                        });
                    });
            },
        )
    };
    let _ = run();
    painted_lines(&run().shapes)
}

/// On a desktop window the analyses list stays a column left of the
/// editor, however narrow the docks leave the centre: at 1440 with a centre
/// narrower than a phone's window the list's group stands beside the
/// editor's first field, not over it.
#[test]
fn a_desktop_window_keeps_the_analyses_list_beside_the_editor() {
    for centre in [800.0, 560.0, 480.0] {
        let lines = analyses_route_in_centre(centre);
        let group = lines
            .iter()
            .find(|line| line.text == FIRST_GROUP)
            .unwrap_or_else(|| panic!("{FIRST_GROUP:?} is painted at {centre}"));
        let field = lines
            .iter()
            .find(|line| line.text == "Stop time")
            .unwrap_or_else(|| panic!("the editor's first field is painted at {centre}"));
        assert!(
            field.left > group.right,
            "centre {centre}: the editor's field starts right of the list ({} > {})",
            field.left,
            group.right
        );
        assert!(
            field.top < group.top + 400.0,
            "centre {centre}: the editor is beside the list, not under it"
        );
    }
}

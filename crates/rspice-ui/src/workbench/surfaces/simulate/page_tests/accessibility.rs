//! What every studio surface announces, and whether it is drawn inside the
//! surface it belongs to.
//!
//! Split from the page's other route tests because these two claims are
//! measured the same way and from one fixture: both read the AccessKit tree a
//! surface publishes, one asking whether every control carries a name and the
//! other whether every control's bounds are inside the viewport. The fixture
//! they share seeds the analyses a default plan does not hold, because the
//! forms that overflowed and the controls that went unnamed were on exactly
//! those.
//!
//! What counts as a surface is wider than a route. The name sweep runs the
//! overlays and the workflow dialogs as well — the analysis catalogue and the
//! plan manager each had a nameless text field behind one — and the overflow
//! sweep opens a form for every kind the catalogue offers rather than the nine
//! the fixture starts with.

use egui::{Rect, vec2};

use super::{RENDER_VIEWPORT_HEIGHT, RSpiceApp, SimulationPage};

/// A studio fixture that reaches every analysis form and every participation
/// state the routes have a cell for.
///
/// One of each kind, a declared run space with global axes on, and the two
/// participations that are not "everywhere": a nominal-only instance and one
/// narrowed to a named subset.
fn studio_fixture() -> RSpiceApp {
    use crate::simulation::plan::{AnalysisInstance, AnalysisKind};
    use crate::simulation::run_set::{AnalysisRunAt, RunSetDimensionKind};

    let mut app = RSpiceApp::test_instance();
    {
        let plan = app
            .state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("the test instance holds a stable plan");
        for kind in [
            AnalysisKind::Ac,
            AnalysisKind::DcSweep,
            AnalysisKind::Noise,
            AnalysisKind::Stb,
            AnalysisKind::Pss,
            AnalysisKind::Temperature,
            AnalysisKind::Corner,
        ] {
            let Ok((id, _)) = plan.insert(kind) else {
                continue;
            };
            for prerequisite in kind.prerequisites() {
                if let Some(target) = plan
                    .instances()
                    .iter()
                    .find(|instance| instance.enabled() && instance.kind() == *prerequisite)
                    .map(AnalysisInstance::id)
                {
                    let _ = plan.bind_dependency(id, *prerequisite, target);
                }
            }
        }
        // Owners of an internal point expansion stay authored but disabled, so
        // the global axes below compose rather than colliding with them.
        for id in plan
            .instances()
            .iter()
            .filter(|instance| {
                matches!(
                    instance.kind(),
                    AnalysisKind::Temperature | AnalysisKind::Corner
                )
            })
            .map(AnalysisInstance::id)
            .collect::<Vec<_>>()
        {
            let _ = plan.set_enabled(id, false);
        }
    }
    for dimension in &mut app.state.sim_setup.run_set.dimensions {
        dimension.enabled = matches!(
            dimension.kind,
            RunSetDimensionKind::Temperature | RunSetDimensionKind::ProcessSection
        );
    }
    let keys: Vec<String> = crate::simulation::run_set::resolve(&app.state.sim_setup.run_set)
        .map(|points| {
            points
                .iter()
                .take(2)
                .map(crate::simulation::run_set::RunSetPoint::point_key)
                .collect()
        })
        .unwrap_or_default();
    let of_kind = |app: &RSpiceApp, kind| {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .and_then(|plan| {
                plan.instances()
                    .iter()
                    .find(|instance| instance.kind() == kind)
                    .map(AnalysisInstance::id)
            })
    };
    let noise = of_kind(&app, AnalysisKind::Noise);
    let stb = of_kind(&app, AnalysisKind::Stb);
    if let Some(plan) = app.state.sim_setup.analysis_plan.as_mut() {
        if let Some(noise) = noise {
            let _ = plan.set_run_at(noise, AnalysisRunAt::NominalPoint);
        }
        if let (Some(stb), false) = (stb, keys.is_empty()) {
            let _ = plan.set_run_at(stb, AnalysisRunAt::SelectedPoints(keys));
        }
    }
    app
}

/// The fixture with an instance of `kind` in it, and that instance selected.
///
/// The overflow gate sweeps every kind the catalogue offers, and the fixture
/// holds nine — so a kind it does not hold is inserted here, with whatever
/// prerequisites the plan asks for bound to an instance already present. `None`
/// where the plan refuses the insert outright, which is a kind that cannot be
/// on a form for this fixture and therefore has no form to measure.
fn studio_form(kind: crate::simulation::plan::AnalysisKind) -> Option<RSpiceApp> {
    use crate::simulation::plan::AnalysisInstance;

    let mut app = studio_fixture();
    app.state.workbench.simulation_page = SimulationPage::Analyses;
    let existing = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .find(|instance| instance.kind() == kind)
                .map(AnalysisInstance::id)
        });
    let selected = match existing {
        Some(id) => id,
        None => {
            let plan = app
                .state
                .sim_setup
                .analysis_plan
                .as_mut()
                .expect("the test instance holds a stable plan");
            let (id, _) = plan.insert(kind).ok()?;
            for prerequisite in kind.prerequisites() {
                if let Some(target) = plan
                    .instances()
                    .iter()
                    .find(|instance| instance.kind() == *prerequisite)
                    .map(AnalysisInstance::id)
                {
                    let _ = plan.bind_dependency(id, *prerequisite, target);
                }
            }
            id
        }
    };
    app.state.workbench.active_analysis_instance = Some(selected);
    Some(app)
}

/// Every analysis kind the fixture holds a form for.
///
/// Kinds rather than instance identities: every call to [`studio_fixture`]
/// mints fresh ones, so an identity taken from one fixture selects nothing in
/// the next and every route silently falls back to the plan's first analysis.
fn studio_analysis_kinds() -> Vec<crate::simulation::plan::AnalysisKind> {
    use crate::simulation::plan::AnalysisInstance;
    studio_fixture()
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| {
            plan.instances()
                .iter()
                .map(AnalysisInstance::kind)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// Run one studio route to a settled AccessKit tree.
///
/// Takes the fixture by value: a helper that borrowed the whole application
/// mutably would be one more of exactly the parameter the layering ratchet
/// counts, and nothing here needs the application afterwards.
fn studio_route_nodes(
    mut app: RSpiceApp,
    width: f32,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    // Two passes: the surface resolves its content width against the scrollbar
    // track it reserves, which it only knows on a second pass.
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(width, RENDER_VIEWPORT_HEIGHT),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::super::show(ui, &mut app));
                // The dialogs are drawn outside the surface, so a sweep that
                // ran only the surface reached none of their controls — which
                // is where two nameless text fields were living.
                super::super::show_workflow_dialogs(ctx, &mut app);
            },
        );
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    nodes
}

/// Every consecutive pass of one studio route, as the controls each published.
///
/// Same fixture and same host as [`studio_route_nodes`], kept apart from it
/// because that helper answers what a settled surface says and this one answers
/// whether it settles at all.
fn studio_route_passes(
    mut app: RSpiceApp,
    width: f32,
    passes: usize,
) -> Vec<Vec<(egui::accesskit::NodeId, egui::accesskit::Rect)>> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut recorded = Vec::with_capacity(passes);
    for _ in 0..passes {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(width, RENDER_VIEWPORT_HEIGHT),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::super::show(ui, &mut app));
                super::super::show_workflow_dialogs(ctx, &mut app);
            },
        );
        let mut controls: Vec<(egui::accesskit::NodeId, egui::accesskit::Rect)> = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default()
            .into_iter()
            .filter(|(_, node)| {
                matches!(
                    node.role(),
                    egui::accesskit::Role::Button
                        | egui::accesskit::Role::CheckBox
                        | egui::accesskit::Role::ComboBox
                        | egui::accesskit::Role::Link
                        | egui::accesskit::Role::TextInput
                        | egui::accesskit::Role::MultilineTextInput
                )
            })
            .filter_map(|(id, node)| node.bounds().map(|bounds| (id, bounds)))
            .collect();
        controls.sort_by_key(|(id, _)| *id);
        recorded.push(controls);
    }
    recorded
}

/// Only the frame's overlay host, with the studio surface not drawn.
///
/// This is what a reader anywhere but Simulate actually gets:
/// [`show_workflow_dialogs`](super::super::show_workflow_dialogs) runs every
/// frame on every workspace, and `simulate::show` does not run at all. A sweep
/// that drew the surface as well would let an overlay the *surface* happens to
/// draw pass as one the frame hosts, which is exactly the confusion that left
/// the analysis catalogue reachable from one route out of nine.
fn workflow_host_nodes(
    mut app: RSpiceApp,
    width: f32,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(width, RENDER_VIEWPORT_HEIGHT),
                )),
                ..Default::default()
            },
            |ctx| super::super::show_workflow_dialogs(ctx, &mut app),
        );
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    nodes
}

/// The fixture on one route, with the instance of one kind selected.
fn studio_route(
    page: SimulationPage,
    kind: Option<crate::simulation::plan::AnalysisKind>,
) -> RSpiceApp {
    use crate::simulation::plan::AnalysisInstance;
    let mut app = studio_fixture();
    app.state.workbench.simulation_page = page;
    if let Some(kind) = kind {
        let selected = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .and_then(|plan| {
                plan.instances()
                    .iter()
                    .find(|instance| instance.kind() == kind)
                    .map(AnalysisInstance::id)
            });
        assert!(
            selected.is_some(),
            "the fixture holds no {kind:?} to open a form on"
        );
        app.state.workbench.active_analysis_instance = selected;
    }
    app
}

/// Every route of the studio, and on the Analyses route every form it holds.
fn studio_routes() -> Vec<(
    SimulationPage,
    Option<crate::simulation::plan::AnalysisKind>,
)> {
    let mut routes = Vec::new();
    for page in SimulationPage::NAVIGATION {
        if page == SimulationPage::Analyses {
            routes.extend(
                studio_analysis_kinds()
                    .into_iter()
                    .map(|kind| (page, Some(kind))),
            );
        } else {
            routes.push((page, None));
        }
    }
    routes
}

/// The first enabled analysis of a fixture, for the overlays that open on one.
fn first_enabled_instance(app: &RSpiceApp) -> crate::product::AnalysisInstanceId {
    use crate::simulation::plan::AnalysisInstance;
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .find(|instance| instance.enabled())
                .map(AnalysisInstance::id)
        })
        .expect("the fixture holds an enabled analysis")
}

/// The analysis catalogue armed from every place its control is drawn.
///
/// Ten fixtures for one window, because the control that opens it is on all of
/// them: the navigator's creating action is drawn on all nine setup routes,
/// and `Command::AddAnalysis` reaches it from the Simulate menu and the palette
/// on any workspace at all. The window itself used to be drawn by the Analyses
/// rail, so nine of these ten armed a modal that no pass rendered — and
/// `palette_open` is one of the terms of `AppState::application_modal_open`, so
/// each of them silently disabled every keyboard shortcut in the application
/// with no painted dialog to press Escape on.
fn analysis_catalogue_fixtures() -> Vec<(String, RSpiceApp)> {
    use crate::workbench::state::Workspace;

    let mut fixtures = SimulationPage::NAVIGATION
        .into_iter()
        .map(|page| {
            let mut app = studio_route(page, None);
            app.state.sim_setup.palette_open = true;
            (format!("analysis catalogue · {page:?}"), app)
        })
        .collect::<Vec<_>>();
    // And from a workspace that is not Simulate at all. The frame hosts this
    // window, so standing on Results is no different from standing on a setup
    // route — which is the whole claim, and the one the command makes true.
    let mut app = studio_route(SimulationPage::Solver, None);
    app.state.workbench.activate(Workspace::Results);
    app.state.sim_setup.palette_open = true;
    fixtures.push(("analysis catalogue · Results workspace".to_owned(), app));
    fixtures
}

/// The catalogue opens wherever the reader pressed the control that arms it.
///
/// Rendered through the frame's overlay host alone, with the studio surface
/// not drawn, because that is the only pass a reader on Results or Verify
/// gets. Judged on the search field's name rather than on the window's frame:
/// the field is the control the dialog puts focus on, so a tree that publishes
/// it is a dialog a reader has actually landed inside.
#[test]
fn the_analysis_catalogue_is_hosted_by_the_frame_on_every_route() {
    let mut missing = Vec::new();
    let fixtures = analysis_catalogue_fixtures();
    assert_eq!(
        fixtures.len(),
        SimulationPage::NAVIGATION.len() + 1,
        "one fixture per setup route, plus one off the Simulate workspace"
    );
    for (surface, app) in fixtures {
        let named = workflow_host_nodes(app, 1280.0)
            .into_iter()
            .any(|(_, node)| {
                node.label()
                    .is_some_and(|label| label == super::super::ANALYSIS_CATALOG_SEARCH_LABEL)
            });
        if !named {
            missing.push(surface);
        }
    }
    assert!(
        missing.is_empty(),
        "the catalogue was armed and nothing drew it on:\n{}",
        missing.join("\n")
    );
}

/// Every overlay and dialog the studio draws over its routes, each on the route
/// it opens from.
///
/// The name sweep ran the surface and nothing else, so the catalogue the
/// palette opens, the advanced-options panel and every workflow dialog were
/// outside it — and two of the nameless text fields it exists to catch were
/// living in exactly those two places. A gate that cannot reach a surface is
/// not a gate over it.
fn studio_overlays() -> Vec<(String, RSpiceApp)> {
    use crate::workbench::state::{
        DesignVariableDraft, RenameAnalysisDraft, SavedOutputDraft, SimulationPlanManagerDraft,
        SimulationWorkflowDialog,
    };

    let mut overlays: Vec<(String, RSpiceApp)> = analysis_catalogue_fixtures();

    // The advanced-options panel, opened on one analysis from the Solver page.
    let mut app = studio_route(SimulationPage::Solver, None);
    let instance = first_enabled_instance(&app);
    super::super::advanced_options::open_for_analysis(&mut app.state.workbench, instance);
    overlays.push(("advanced options".to_owned(), app));

    // The plan manager.
    let mut app = studio_route(SimulationPage::Analyses, None);
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the fixture holds a stable plan")
        .id();
    let plan_name = app.state.sim_setup.active_plan_name().to_string();
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::PlanManager(
        SimulationPlanManagerDraft::new(plan_id, plan_name),
    ));
    overlays.push(("plan manager".to_owned(), app));

    // Renaming an analysis.
    let mut app = studio_route(SimulationPage::Analyses, None);
    let instance = first_enabled_instance(&app);
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::RenameAnalysis(
        RenameAnalysisDraft::for_instance(instance, "Transient", "Startup transient"),
    ));
    overlays.push(("rename analysis".to_owned(), app));

    // The run-points picker.
    let mut app = studio_route(SimulationPage::Analyses, None);
    let instance = first_enabled_instance(&app);
    super::super::participation::open_point_picker(&mut app.state, instance);
    overlays.push(("run points".to_owned(), app));

    // A new design variable.
    let mut app = studio_route(SimulationPage::Variables, None);
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::DesignVariable(
        DesignVariableDraft::default(),
    ));
    overlays.push(("design variable".to_owned(), app));

    // A new saved output.
    let mut app = studio_route(SimulationPage::Outputs, None);
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::SavedOutput(
        SavedOutputDraft::default(),
    ));
    overlays.push(("saved output".to_owned(), app));

    overlays
}

/// Every text field and switch in the studio announces the row that names it.
///
/// A `TextEdit` publishes no accessible name of its own, and a self-painted
/// control publishes only what it is handed — so a form of engineering fields
/// announced four controls whose name was the empty string, the analysis
/// forms' booleans announced themselves as "Enabled", and the point table
/// published twenty-seven nameless boxes. None of those can be reached, or
/// told apart, by name.
///
/// The names come from the constructors — `ui::widgets::mono_input` and
/// `page_kit::switch_row` and `page_kit::switch_cell` each take the row's own
/// label — so a new call site cannot omit one. This sweeps the routes to prove
/// no path bypasses them.
#[test]
fn every_studio_text_field_and_switch_announces_a_name() {
    let mut unnamed = Vec::new();
    let mut inputs = 0usize;
    let mut boxes = 0usize;
    let mut surfaces: Vec<(String, RSpiceApp)> = studio_routes()
        .into_iter()
        .map(|(page, kind)| {
            let label =
                kind.map_or_else(|| format!("{page:?}"), |kind| format!("{page:?}/{kind:?}"));
            (label, studio_route(page, kind))
        })
        .collect();
    // The overlays and dialogs, which the sweep did not reach: the catalogue's
    // search field and the plan manager's filter were nameless behind it.
    surfaces.extend(studio_overlays());
    let mut swept = 0usize;
    for (surface, app) in surfaces {
        swept += 1;
        for (_, node) in studio_route_nodes(app, 1280.0) {
            let role = node.role();
            if !matches!(
                role,
                egui::accesskit::Role::TextInput
                    | egui::accesskit::Role::MultilineTextInput
                    | egui::accesskit::Role::CheckBox
            ) {
                continue;
            }
            if node.bounds().is_none() {
                continue;
            }
            if role == egui::accesskit::Role::CheckBox {
                boxes += 1;
            } else {
                inputs += 1;
            }
            if node.label().is_none_or(|label| label.trim().is_empty()) {
                unnamed.push(format!("{surface}: {role:?} announces no name"));
            }
        }
    }
    unnamed.sort();
    unnamed.dedup();
    assert!(
        unnamed.is_empty(),
        "controls a reader cannot reach by name:\n{}",
        unnamed.join("\n")
    );
    // A sweep that reached nothing would pass forever, and one that quietly
    // stopped opening the overlays would pass just as well.
    assert!(
        inputs >= 12 && boxes >= 4,
        "the sweep reached {inputs} text fields and {boxes} switches; it is not \
         reaching the surfaces it claims to check"
    );
    assert!(
        swept >= studio_routes().len() + 6 + analysis_catalogue_fixtures().len(),
        "the sweep ran {swept} surfaces; every route and every overlay is one"
    );
}

/// Nothing the Analyses page offers may sit outside the surface it is drawn on.
///
/// The title row reserved a fixed width for its five actions that the group had
/// outgrown, so at the 1000-point gate the accent action was cut by the surface
/// edge and the analysis rail beside it lost its enable switches off the right.
/// A clipped control is not a control: there is no horizontal scroll on this
/// surface to reach it with.
///
/// Every analysis form, not only the one a default plan opens on. The STB and
/// Noise forms' right-hand column — Start, Points/decade, the input source —
/// was drawn a hundred points past the pane at the 1000-point gate, and this
/// gate could not see it twice over: it opened one form, and the fields it lost
/// were text inputs, which published no name and so had no AccessKit node to
/// measure until the constructors started naming them.
///
/// Measured through AccessKit rather than through painted shapes, because the
/// question is about controls: a decorative shape may legitimately be clipped,
/// and a button may not.
#[test]
fn no_analyses_page_control_is_cut_off_at_the_narrow_gate() {
    // The 1000-point gate, the widths either side of it, and the band where a
    // fixed reservation is widest relative to the surface. A row that reserves
    // a constant for its actions overflows wherever that constant plus the
    // heading's floor exceeds the surface, and where that band falls depends on
    // how wide the labels happen to be -- so this sweeps rather than sampling
    // one width.
    const GATE_WIDTHS: [f32; 6] = [620.0, 700.0, 820.0, 960.0, 1000.0, 1024.0];
    // Sub-pixel: a control resting exactly on the edge is inside it, and
    // rounding in the layout must not read as a defect.
    const TOLERANCE: f64 = 0.5;

    let mut offenders = Vec::new();
    let mut measured = 0usize;
    let mut forms = 0usize;
    for width in GATE_WIDTHS {
        for kind in crate::simulation::plan::AnalysisKind::ALL {
            let Some(app) = studio_form(kind) else {
                continue;
            };
            forms += 1;
            for (_, node) in studio_route_nodes(app, width) {
                // Only things a reader acts on. A container legitimately
                // extends past the viewport; the page scrolls vertically to
                // reach it.
                if !matches!(
                    node.role(),
                    egui::accesskit::Role::Button
                        | egui::accesskit::Role::CheckBox
                        | egui::accesskit::Role::ComboBox
                        | egui::accesskit::Role::Link
                        | egui::accesskit::Role::TextInput
                        | egui::accesskit::Role::MultilineTextInput
                ) {
                    continue;
                }
                let Some(bounds) = node.bounds() else {
                    continue;
                };
                measured += 1;
                if bounds.x1 > f64::from(width) + TOLERANCE || bounds.x0 < -TOLERANCE {
                    offenders.push(format!(
                        "{width:.0}pt surface, {kind:?} form: {:?} {:?} spans {:.1}..{:.1}",
                        node.role(),
                        node.label().unwrap_or_default(),
                        bounds.x0,
                        bounds.x1
                    ));
                }
            }
        }
    }
    offenders.sort();
    offenders.dedup();
    assert!(
        offenders.is_empty(),
        "controls drawn outside the surface they belong to:\n{}",
        offenders.join("\n")
    );
    // A sweep that measured nothing would pass forever, and one that reached
    // only the nine forms the fixture starts with would have missed the STB and
    // Noise overflow entirely -- which is how it got here.
    assert!(
        measured > 300,
        "the sweep measured only {measured} controls across {} widths and every form; \
         it is not reaching the forms it claims to check",
        GATE_WIDTHS.len()
    );
    let kinds = crate::simulation::plan::AnalysisKind::ALL.len();
    assert!(
        forms >= (kinds - 4) * GATE_WIDTHS.len(),
        "the sweep opened {forms} forms across {} widths; every one of the {kinds} kinds the \
         catalogue offers has a form, and a plan that refuses more than a handful of inserts is \
         a fixture that has stopped reaching them",
        GATE_WIDTHS.len()
    );
}

/// Every workflow dialog the studio opens comes to rest, and stays where it
/// came to rest.
///
/// A dialog measured from its content used to creep upward two points a pass
/// for as long as its body overflowed: the overflow took a scrollbar, the bar
/// narrowed the body, the narrower body wrapped taller, and the surface
/// measured from it asked for more room again. Nothing on such a dialog ever
/// stopped moving for a third of a second after it opened, and a reader
/// reaching for a control was reaching for a moving target.
///
/// Two passes of settling are allowed and no more. One is the overlay's own: a
/// content-height surface is laid out against the height its previous pass
/// resolved, so the pass that opens it is the measuring one. The other belongs
/// to the route drawn behind it, which resolves its own content against the
/// scroll track it reserves a pass later still. The dialog shell's own contract
/// — one re-measure, and never a third — is pinned where it can be measured
/// alone, in `ui::widgets::dialog`'s tests.
#[test]
fn every_studio_workflow_dialog_comes_to_rest_and_stays_there() {
    const PASSES: usize = 8;
    // Sub-pixel: rounding in the layout is not drift.
    const TOLERANCE: f64 = 0.5;
    /// The pass by which everything an overlay draws has settled, and the one
    /// every later pass is judged against.
    const SETTLED: usize = 2;

    let mut drifting = Vec::new();
    let mut swept = 0usize;
    let mut measured = 0usize;
    // Two of the studio's overlays used to be left out, each for the shell's
    // defect one level down — a scroll area whose rows were laid out against a
    // width the bar's animated reveal was still taking away. The analysis
    // catalogue's result list now reads its row track through
    // `analysis_catalog_row_space` with the gutter withheld whether or not a
    // bar is showing, and the studio surface reserves the same gutter for the
    // routes drawn into it (which is what brought the advanced-options panel
    // to rest), so the sweep covers everything.
    for (surface, app) in studio_overlays() {
        swept += 1;
        let passes = studio_route_passes(app, 1280.0, PASSES);
        let settled = &passes[SETTLED];
        measured += settled.len();
        for (index, pass) in passes.iter().enumerate().skip(SETTLED + 1) {
            let pass_number = index + 1;
            if pass.len() != settled.len() {
                drifting.push(format!(
                    "{surface}: pass {pass_number} publishes {} controls where the \
                     settled pass published {}",
                    pass.len(),
                    settled.len()
                ));
                continue;
            }
            for ((id, bounds), (settled_id, settled_bounds)) in pass.iter().zip(settled) {
                if id != settled_id {
                    drifting.push(format!(
                        "{surface}: pass {pass_number} publishes a different set of controls"
                    ));
                    continue;
                }
                let moved = [
                    (bounds.x0, settled_bounds.x0),
                    (bounds.y0, settled_bounds.y0),
                    (bounds.x1, settled_bounds.x1),
                    (bounds.y1, settled_bounds.y1),
                ]
                .into_iter()
                .any(|(now, settled)| (now - settled).abs() > TOLERANCE);
                if moved {
                    drifting.push(format!(
                        "{surface}: a control settled at \
                         {:.1}..{:.1} x {:.1}..{:.1} and pass {pass_number} drew it at \
                         {:.1}..{:.1} x {:.1}..{:.1}",
                        settled_bounds.x0,
                        settled_bounds.x1,
                        settled_bounds.y0,
                        settled_bounds.y1,
                        bounds.x0,
                        bounds.x1,
                        bounds.y0,
                        bounds.y1
                    ));
                }
            }
        }
    }
    drifting.sort();
    drifting.dedup();
    assert!(
        drifting.is_empty(),
        "overlays still moving after they had come to rest:\n{}",
        drifting.join("\n")
    );
    // A sweep that opened nothing, or measured nothing on what it opened,
    // would pass forever.
    assert_eq!(
        swept,
        studio_overlays().len(),
        "every studio overlay is one pass of this sweep"
    );
    assert!(
        measured > 100,
        "the sweep watched only {measured} controls; it is not reaching the \
         overlays it claims to check"
    );
}

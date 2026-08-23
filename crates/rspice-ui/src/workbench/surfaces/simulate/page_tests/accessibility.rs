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

    let mut overlays: Vec<(String, RSpiceApp)> = Vec::new();

    // The analysis catalogue, which a keystroke opens over the Analyses route.
    let mut app = studio_route(SimulationPage::Analyses, None);
    app.state.sim_setup.palette_open = true;
    overlays.push(("analysis catalogue".to_owned(), app));

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

/// Every text field and tick box in the studio announces the row that names it.
///
/// A `TextEdit` publishes no accessible name of its own and `egui::Checkbox`
/// publishes whichever word it is currently painting — so a form of engineering
/// fields announced four controls whose name was the empty string, the analysis
/// forms' tick boxes announced themselves as "Enabled", and the point table
/// published twenty-seven nameless boxes. None of those can be reached, or told
/// apart, by name.
///
/// The names come from the widget constructors — `ui::widgets::mono_input` and
/// `ui::widgets::tick_box` both take the row's label — so a new call site cannot
/// omit one. This sweeps the routes to prove no path bypasses them.
#[test]
fn every_studio_text_field_and_tick_box_announces_a_name() {
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
        "the sweep reached {inputs} text fields and {boxes} tick boxes; it is not \
         reaching the surfaces it claims to check"
    );
    assert!(
        swept >= studio_routes().len() + 7,
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

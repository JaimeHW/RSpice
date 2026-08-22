//! What every studio route announces, and whether it is drawn inside its
//! surface.
//!
//! Split from the page's other route tests because these two claims are
//! measured the same way and from one fixture: both read the AccessKit tree a
//! route publishes, one asking whether every control carries a name and the
//! other whether every control's bounds are inside the viewport. The fixture
//! they share holds one instance of every analysis kind, because the forms that
//! overflowed and the controls that went unnamed were on exactly the instances
//! a default plan does not hold.

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

/// Every analysis instance the fixture holds, as a route selection.
fn studio_analysis_instances() -> Vec<crate::product::AnalysisInstanceId> {
    use crate::simulation::plan::AnalysisInstance;
    studio_fixture()
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| {
            plan.instances()
                .iter()
                .map(AnalysisInstance::id)
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

/// The fixture on one route, with one analysis instance selected.
fn studio_route(
    page: SimulationPage,
    selection: Option<crate::product::AnalysisInstanceId>,
) -> RSpiceApp {
    let mut app = studio_fixture();
    app.state.workbench.simulation_page = page;
    app.state.workbench.active_analysis_instance = selection;
    app
}

/// Every route of the studio, and on the Analyses route every form it holds.
fn studio_routes() -> Vec<(SimulationPage, Option<crate::product::AnalysisInstanceId>)> {
    let mut routes = Vec::new();
    for page in SimulationPage::NAVIGATION {
        if page == SimulationPage::Analyses {
            routes.extend(
                studio_analysis_instances()
                    .into_iter()
                    .map(|id| (page, Some(id))),
            );
        } else {
            routes.push((page, None));
        }
    }
    routes
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
    for (page, selection) in studio_routes() {
        for (_, node) in studio_route_nodes(studio_route(page, selection), 1280.0) {
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
                unnamed.push(format!("{page:?}: {role:?} announces no name"));
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
    // A sweep that reached nothing would pass forever.
    assert!(
        inputs >= 8 && boxes >= 4,
        "the sweep reached {inputs} text fields and {boxes} tick boxes; it is not \
         reaching the routes it claims to check"
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
    for width in GATE_WIDTHS {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.simulation_page = SimulationPage::Analyses;
        for (_, node) in studio_route_nodes(app, width) {
            // Only things a reader acts on. A container legitimately extends
            // past the viewport; the page scrolls vertically to reach it.
            if !matches!(
                node.role(),
                egui::accesskit::Role::Button
                    | egui::accesskit::Role::CheckBox
                    | egui::accesskit::Role::ComboBox
                    | egui::accesskit::Role::Link
            ) {
                continue;
            }
            let Some(bounds) = node.bounds() else {
                continue;
            };
            if bounds.x1 > f64::from(width) + TOLERANCE || bounds.x0 < -TOLERANCE {
                offenders.push(format!(
                    "{width:.0}pt surface: {:?} {:?} spans {:.1}..{:.1}",
                    node.role(),
                    node.label().unwrap_or_default(),
                    bounds.x0,
                    bounds.x1
                ));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "controls drawn outside the surface they belong to:\n{}",
        offenders.join("\n")
    );
}

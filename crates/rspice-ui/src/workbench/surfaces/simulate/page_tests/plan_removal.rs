//! Destructive review before a plan removal that orphans something.
//!
//! Four registries share one review, and each has to answer the same three
//! questions: a removal with dependents stops and leaves the record standing
//! when the reader cancels, a confirmed removal commits with the same receipt
//! the immediate route writes, and a removal that orphans nothing never opens a
//! modal at all. These press the actual Remove control rather than calling the
//! handler, because "which route commits a plan mutation" is exactly what was
//! wrong: three of the four committed on the first click.
//!
//! The analysis registry answers a fourth question the other three do not. Its
//! dependents are the one case the plan's own transaction *refuses*, so a
//! review staged on them could only ever be confirmed into a refusal notice —
//! that arm states the refusal instead, and only the retained-results arm is
//! still a question.

use egui::vec2;

use super::{AppState, RSpiceApp, SimulationPage, pages, plan_id};
use crate::product::SimulationPlanId;
use crate::state::{
    CaptureGroup, CaptureGroupRule, DesignVariable, DesignVariableOverridePolicy,
    DesignVariableQuantity, DesignVariableScope, DesignVariableSweepEligibility, InstancePath,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SpecEntry,
};

/// One setup page, rendered with its controls announced so a test can press
/// the same button an engineer presses.
struct Registry {
    ctx: egui::Context,
    app: RSpiceApp,
    page: SimulationPage,
    controls: Vec<(String, egui::Rect)>,
}

impl Registry {
    fn open(page: SimulationPage, seed: impl FnOnce(&mut AppState)) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.simulation_page = page;
        seed(&mut app.state);
        let mut registry = Self {
            ctx,
            app,
            page,
            controls: Vec::new(),
        };
        // Twice: the first pass builds the font set and the second lays out
        // against it, and a rectangle measured before the fonts exist is not
        // the rectangle the control ends up in.
        registry.pass(Vec::new());
        registry.pass(Vec::new());
        registry
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let app = &mut self.app;
        let page = self.page;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(1400.0, 2600.0),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| pages::show(ui, app, page));
                    });
            },
        );
        self.controls = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| {
                        let label = node.label()?.to_owned();
                        let bounds = node.bounds()?;
                        Some((
                            label,
                            egui::Rect::from_min_max(
                                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    /// Press the one control announcing exactly `label`, then settle the frame
    /// the press produced.
    fn click(&mut self, label: &str) {
        let hits = self
            .controls
            .iter()
            .filter(|(announced, _)| announced == label)
            .collect::<Vec<_>>();
        assert_eq!(
            hits.len(),
            1,
            "expected exactly one control announcing {label:?}, found {}; the page announces: \
             {:#?}",
            hits.len(),
            self.controls
                .iter()
                .map(|(announced, _)| announced.as_str())
                .collect::<Vec<_>>()
        );
        let at = hits[0].1.center();
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

    fn review_open(&self) -> bool {
        self.app.state.dialogs.plan_removal_review.target.is_some()
    }

    /// Answer the review the way the dialog's primary action answers it, then
    /// run the frame that applies it. The dialog itself is drawn by the
    /// application shell, which these page-only passes do not run.
    fn confirm(&mut self) {
        assert!(self.review_open(), "no review is staged to confirm");
        self.app.state.dialogs.plan_removal_review.confirmed = true;
        self.pass(Vec::new());
    }

    /// Answer it the way Cancel answers it.
    fn cancel(&mut self) {
        assert!(self.review_open(), "no review is staged to cancel");
        self.app.state.dialogs.plan_removal_review.close();
        self.pass(Vec::new());
    }

    fn receipt(&self) -> String {
        self.app
            .state
            .workbench
            .analysis_lifecycle_status
            .message()
            .to_owned()
    }

    fn plan(&self) -> SimulationPlanId {
        plan_id(&self.app)
    }

    fn variables(&self) -> Vec<String> {
        self.app
            .state
            .workspace
            .plan_data(self.plan())
            .map(|payload| {
                payload
                    .design_variables
                    .iter()
                    .map(|variable| variable.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn outputs(&self) -> Vec<String> {
        self.app
            .state
            .workspace
            .plan_data(self.plan())
            .map(|payload| {
                payload
                    .saved_outputs
                    .iter()
                    .map(|output| output.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn groups(&self) -> Vec<String> {
        self.app
            .state
            .workspace
            .plan_data(self.plan())
            .map(|payload| {
                payload
                    .capture_groups
                    .iter()
                    .map(|group| group.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// The active plan, from the state a seeding closure is handed.
///
/// The page tests' own `plan_id` takes the whole application; a seed only needs
/// the state it writes into, and the layering ratchet is at its ceiling for
/// handlers that take more than that.
fn plan_of(state: &AppState) -> SimulationPlanId {
    state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .id()
}

/// A plan-scoped variable named `name`, selected in the registry.
fn seed_variable(state: &mut AppState, name: &str) {
    let id = plan_of(state);
    let variable = DesignVariable::new(
        name,
        "1kohm",
        DesignVariableQuantity::Resistance,
        DesignVariableScope::Testbench,
        "load resistance",
        None,
        DesignVariableSweepEligibility::FixedParameter,
        DesignVariableOverridePolicy::InheritOwnerOnly,
    )
    .expect("valid design variable");
    state
        .workspace
        .add_design_variable(id, variable)
        .expect("the plan accepts it");
    state.workbench.selected_design_variable = Some(name.to_owned());
}

/// An output named `name`, selected in the registry.
fn seed_output(state: &mut AppState, name: &str, expression: &str) {
    let id = plan_of(state);
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        name,
        expression,
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    state
        .workspace
        .add_saved_output(id, output)
        .expect("the plan accepts it");
    state.workbench.selected_saved_output = Some(name.to_owned());
}

/// Switch every analysis off, so the plan resolves a variable into nothing.
fn disable_every_analysis(state: &mut AppState) {
    let ids = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>();
    let plan = state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan");
    for id in ids {
        plan.set_enabled(id, false).expect("the plan accepts it");
    }
}

// ---------------------------------------------------------------- variables

/// Removing a variable the plan resolves is stopped, and cancelling keeps it.
///
/// It committed on the first click. A design variable is read by every analysis
/// the plan resolves it into, and nothing about the registry row said so at the
/// moment of the click — the reader lost the declaration and the expressions
/// naming it in one press, with no undo behind it.
#[test]
fn removing_a_variable_the_plan_resolves_opens_the_review() {
    let mut registry = Registry::open(SimulationPage::Variables, |state| {
        seed_variable(state, "vref")
    });
    registry.click("Remove");

    assert!(
        registry.review_open(),
        "a variable an enabled analysis resolves must be reviewed before it is removed"
    );
    assert_eq!(
        registry.variables(),
        vec!["vref".to_owned()],
        "staging a review must not remove the variable"
    );

    registry.cancel();
    assert_eq!(
        registry.variables(),
        vec!["vref".to_owned()],
        "cancelling the review must leave the registry exactly as it was"
    );
}

/// Confirming it commits the removal, through the same plan transaction.
#[test]
fn a_confirmed_variable_removal_commits_with_its_receipt() {
    let mut registry = Registry::open(SimulationPage::Variables, |state| {
        seed_variable(state, "vref")
    });
    registry.click("Remove");
    registry.confirm();

    assert!(
        registry.variables().is_empty(),
        "a confirmed removal must actually remove the variable"
    );
    assert!(
        registry.receipt().contains("Removed design variable vref"),
        "the confirmed removal must produce the registry's own receipt, got: {}",
        registry.receipt()
    );
    assert!(
        !registry.review_open(),
        "the review must close once its answer has been applied"
    );
}

/// A variable no analysis resolves is removed on the click that asks for it.
///
/// The review exists for removals that cost something. Stopping for one that
/// orphans nothing is how a reader learns to click through the ones that do.
#[test]
fn removing_a_variable_nothing_resolves_needs_no_review() {
    let mut registry = Registry::open(SimulationPage::Variables, |state| {
        disable_every_analysis(state);
        seed_variable(state, "vref");
    });
    registry.click("Remove");

    assert!(
        !registry.review_open(),
        "a variable no enabled analysis resolves must not raise a modal"
    );
    assert!(
        registry.variables().is_empty(),
        "it must be removed by the click that asked for it"
    );
    assert!(
        registry.receipt().contains("Removed design variable vref"),
        "the immediate removal keeps its receipt, got: {}",
        registry.receipt()
    );
}

// ------------------------------------------------------------------ outputs

/// Removing an output a specification is measured from is stopped.
#[test]
fn removing_an_output_a_specification_reads_opens_the_review() {
    let mut registry = Registry::open(SimulationPage::Outputs, |state| {
        seed_output(state, "vout", "V(out)");
        let id = plan_of(state);
        let payload = state
            .workspace
            .plan_data_mut(id)
            .expect("the plan has a payload");
        payload.specs.push(SpecEntry {
            measurement: "vout".to_owned(),
            expression: "MAX V(out)".to_owned(),
            min: None,
            max: Some(3.3),
            unit: "V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        });
    });
    registry.click("Remove");

    assert!(
        registry.review_open(),
        "an output a requirement is judged from must be reviewed before it is removed"
    );
    assert_eq!(registry.outputs(), vec!["vout".to_owned()]);

    registry.cancel();
    assert_eq!(
        registry.outputs(),
        vec!["vout".to_owned()],
        "cancelling the review must leave the registry exactly as it was"
    );
}

/// Confirming it commits the removal, through the same plan transaction.
#[test]
fn a_confirmed_output_removal_commits_with_its_receipt() {
    let mut registry = Registry::open(SimulationPage::Outputs, |state| {
        seed_output(state, "vout", "V(out)");
        let id = plan_of(state);
        let mut group = CaptureGroup::new("Rails").expect("group name");
        group.rules.push(CaptureGroupRule::for_scope(
            InstancePath::parse_legacy("/").expect("scope"),
        ));
        state
            .workspace
            .add_capture_group(id, group)
            .expect("the plan accepts the group");
    });
    registry.click("Remove");
    assert!(
        registry.review_open(),
        "an output a capture group holds must be reviewed before it is removed"
    );

    registry.confirm();
    assert!(
        registry.outputs().is_empty(),
        "a confirmed removal must actually remove the output"
    );
    assert!(
        registry.receipt().contains("Removed saved output vout"),
        "the confirmed removal must produce the registry's own receipt, got: {}",
        registry.receipt()
    );
}

/// An output no group holds and no requirement names is removed at once.
#[test]
fn removing_an_unheld_unread_output_needs_no_review() {
    let mut registry = Registry::open(SimulationPage::Outputs, |state| {
        seed_output(state, "vout", "V(out)")
    });
    registry.click("Remove");

    assert!(
        !registry.review_open(),
        "an output nothing in the plan is built on must not raise a modal"
    );
    assert!(
        registry.outputs().is_empty(),
        "it must be removed by the click that asked for it"
    );
    assert!(
        registry.receipt().contains("Removed saved output vout"),
        "the immediate removal keeps its receipt, got: {}",
        registry.receipt()
    );
}

// ----------------------------------------------------------- capture groups

/// A group with members, selected on the Save page.
fn seed_capture_group(state: &mut AppState, name: &str, holding: bool) {
    let id = plan_of(state);
    if holding {
        seed_output(state, "vout", "V(out)");
    }
    let mut group = CaptureGroup::new(name).expect("group name");
    group.rules.push(CaptureGroupRule::for_scope(
        InstancePath::parse_legacy("/").expect("scope"),
    ));
    group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
    let group_id = state
        .workspace
        .add_capture_group(id, group)
        .expect("the plan accepts the group");
    state.workbench.selected_capture_group = Some(group_id);
}

/// Removing a group that holds outputs is stopped, and cancelling keeps it.
///
/// It committed on the first click. Every output under the group reverts to its
/// own record's save policy, precision and streaming the instant the group
/// goes, which changes what the run stores and what it costs — and none of it
/// is legible from the strip that removes the group.
#[test]
fn removing_a_capture_group_that_holds_outputs_opens_the_review() {
    let mut registry = Registry::open(SimulationPage::Save, |state| {
        seed_capture_group(state, "Rails", true);
    });
    registry.click("Remove");

    assert!(
        registry.review_open(),
        "a group holding outputs must be reviewed before it is removed"
    );
    assert_eq!(registry.groups(), vec!["Rails".to_owned()]);

    registry.cancel();
    assert_eq!(
        registry.groups(),
        vec!["Rails".to_owned()],
        "cancelling the review must leave the group list exactly as it was"
    );
}

/// Confirming it commits the removal, through the same plan transaction.
#[test]
fn a_confirmed_capture_group_removal_commits_with_its_receipt() {
    let mut registry = Registry::open(SimulationPage::Save, |state| {
        seed_capture_group(state, "Rails", true);
    });
    registry.click("Remove");
    registry.confirm();

    assert!(
        registry.groups().is_empty(),
        "a confirmed removal must actually remove the group"
    );
    assert!(
        registry.receipt().contains("Removed capture group Rails"),
        "the confirmed removal must produce the registry's own receipt, got: {}",
        registry.receipt()
    );
    assert!(
        registry.outputs().contains(&"vout".to_owned()),
        "removing a group removes a policy, never an output"
    );
}

/// An empty group is a policy with nothing under it, and goes on one click.
#[test]
fn removing_an_empty_capture_group_needs_no_review() {
    let mut registry = Registry::open(SimulationPage::Save, |state| {
        seed_capture_group(state, "Rails", false);
    });
    registry.click("Remove");

    assert!(
        !registry.review_open(),
        "a group holding nothing must not raise a modal"
    );
    assert!(
        registry.groups().is_empty(),
        "it must be removed by the click that asked for it"
    );
    assert!(
        registry.receipt().contains("Removed capture group Rails"),
        "the immediate removal keeps its receipt, got: {}",
        registry.receipt()
    );
}

// ------------------------------------------------------- analysis regression

/// A plan with one analysis bound to another, both named distinctly.
///
/// Returns the application it built, then `(dependent, prerequisite)`. The
/// names are authored rather than left as the kind labels so an assertion about
/// a name cannot pass on a coincidence: "AC" is a word two default labels and a
/// receipt already carry.
fn bound_pair() -> (
    RSpiceApp,
    crate::product::AnalysisInstanceId,
    crate::product::AnalysisInstanceId,
) {
    use crate::simulation::plan::AnalysisKind;

    let mut app = RSpiceApp::test_instance();
    let dependent = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .insert(AnalysisKind::Ac)
        .expect("AC inserts")
        .0;
    super::super::lifecycle::apply_analysis_action(
        &mut app,
        dependent,
        super::super::AnalysisAction::RepairDependencies,
    );
    let prerequisite = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .instance(dependent)
        .expect("AC is in the plan")
        .dependencies()[0]
        .target();
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan");
    plan.set_instance_name(dependent, "Gain sweep")
        .expect("the dependent takes a name");
    plan.set_instance_name(prerequisite, "Bias point")
        .expect("the prerequisite takes a name");
    (app, dependent, prerequisite)
}

/// An analysis something is bound to is refused, not reviewed.
///
/// This route used to stage the destructive review for exactly the case the
/// plan's removal transaction refuses — same predicate, evaluated twice — so
/// the reader was asked to authorise a removal whose only possible ending was
/// a refusal notice on the next frame. Confirming did nothing, and what they
/// were told afterwards named the blocking analysis by instance id.
#[test]
fn an_analysis_something_is_bound_to_refuses_removal_instead_of_reviewing_it() {
    use super::super::lifecycle::remove_analysis_instance;

    let (mut app, dependent, prerequisite) = bound_pair();

    remove_analysis_instance(&mut app, prerequisite);

    assert!(
        app.state.dialogs.plan_removal_review.target.is_none(),
        "a removal the plan refuses must not be staged as a destructive review"
    );
    assert_eq!(
        app.state.dialogs.plan_removal_refusal.analysis,
        Some(prerequisite),
        "the refusal must be raised on the analysis whose removal was asked for"
    );
    assert!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(prerequisite)
            .is_some(),
        "stating a refusal must not remove the analysis"
    );
    assert_eq!(
        app.state.dialogs.plan_removal_refusal.label, "Bias point",
        "the refusal must name the analysis by what it shows under"
    );
    assert_eq!(
        app.state.dialogs.plan_removal_refusal.blockers,
        vec![(dependent, "Gain sweep".to_owned())],
        "the blocker must be carried by display name, and by id for the hop"
    );
    assert!(
        !app.state
            .dialogs
            .plan_removal_refusal
            .blockers
            .iter()
            .any(|(id, name)| name == &id.to_string()),
        "an opaque instance id is not a name a reader can act on"
    );
}

/// The refusal hands the reader the analysis that is in the way.
///
/// A notice that states a blocker and offers no way to reach it leaves the
/// reader to find it themselves in a stack that names it nowhere else. The hop
/// is recorded by the dialog and taken by the surface that owns the plan, the
/// same handshake a confirmed removal uses.
#[test]
fn the_refusal_hops_to_the_blocking_analysis_exactly_once() {
    use super::super::lifecycle::{apply_requested_blocker_reveal, remove_analysis_instance};

    let (mut app, dependent, prerequisite) = bound_pair();
    app.state.workbench.simulation_page = SimulationPage::Solver;
    app.state.workbench.active_analysis_instance = Some(prerequisite);

    remove_analysis_instance(&mut app, prerequisite);
    // Nothing moves until the reader asks for it.
    apply_requested_blocker_reveal(&mut app.state);
    assert_eq!(
        app.state.workbench.simulation_page,
        SimulationPage::Solver,
        "an unanswered refusal must not navigate anywhere"
    );

    app.state.dialogs.plan_removal_refusal.reveal = true;
    apply_requested_blocker_reveal(&mut app.state);
    assert_eq!(
        app.state.workbench.simulation_page,
        SimulationPage::Analyses,
        "the hop must land on the page that edits an analysis"
    );
    assert_eq!(
        app.state.workbench.active_analysis_instance,
        Some(dependent),
        "the hop must select the analysis that is blocking the removal"
    );
    assert!(
        app.state.dialogs.plan_removal_refusal.analysis.is_none(),
        "the refusal must close behind the hop it sent the reader on"
    );

    app.state.workbench.simulation_page = SimulationPage::Solver;
    apply_requested_blocker_reveal(&mut app.state);
    assert_eq!(
        app.state.workbench.simulation_page,
        SimulationPage::Solver,
        "an answered refusal must not navigate a second time"
    );
}

/// The hop is called what the other hop to the same place is called.
#[test]
fn the_refusal_and_the_options_panel_name_the_same_hop_the_same_way() {
    assert_eq!(
        crate::workbench::app::REVEAL_BLOCKER,
        super::super::advanced_options::REVEAL_ACTION,
        "two surfaces sending the reader to the Analyses page must say so in one wording"
    );
}

/// Retained results, and nothing bound to it: still reviewed, still committed.
///
/// This is the arm where the confirmation is a real question. The removal will
/// succeed, and what it costs is the attribution of datasets that stay in the
/// project either way — so the reader decides, and the answer is applied on the
/// surface's next frame rather than by the modal.
#[test]
fn a_removal_costing_only_retained_runs_still_stages_and_commits() {
    use super::super::lifecycle::{apply_confirmed_analysis_removal, remove_analysis_instance};
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::simulation::plan::AnalysisKind;
    use crate::state::{AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun};

    let mut app = RSpiceApp::test_instance();
    let ac = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .insert(AnalysisKind::Ac)
        .expect("AC inserts")
        .0;
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_provenance(
            AnalysisResultProvenance::new(
                ac,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5a; 32]),
                Vec::new(),
            )
            .expect("test provenance is valid"),
        ),
    );
    app.state.simulation.runs = vec![run];

    remove_analysis_instance(&mut app, ac);
    assert_eq!(
        app.state.dialogs.plan_removal_review.target,
        Some(crate::workbench::app::PlanRemovalTarget::Analysis(ac)),
        "a removal that orphans retained results must still be reviewed"
    );
    assert!(
        app.state.dialogs.plan_removal_refusal.analysis.is_none(),
        "a removal the plan will perform is a question, not a refusal"
    );
    let stated: Vec<(String, String)> = app
        .state
        .dialogs
        .plan_removal_review
        .consequences
        .iter()
        .map(|consequence| (consequence.fact.clone(), consequence.value.clone()))
        .collect();
    assert_eq!(
        stated,
        vec![
            (
                "Retained runs holding its results".to_owned(),
                "1".to_owned()
            ),
            ("Analyses bound to it".to_owned(), "none".to_owned()),
        ],
        "the review states what removal costs and what it does not"
    );
    assert!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(ac)
            .is_some(),
        "staging a review must not remove the analysis"
    );

    app.state.dialogs.plan_removal_review.confirmed = true;
    apply_confirmed_analysis_removal(&mut app);
    assert!(
        app.state.dialogs.plan_removal_review.target.is_none(),
        "the review must close once its answer has been applied"
    );
    assert!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(ac)
            .is_none(),
        "the confirmed removal must reach the plan's own transaction"
    );
    let status = app
        .state
        .workbench
        .analysis_lifecycle_status
        .message()
        .to_owned();
    assert!(
        status.contains("Remove committed"),
        "the commit must leave the receipt the immediate route writes, got: {status}"
    );
    apply_confirmed_analysis_removal(&mut app);
    assert_eq!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .to_owned(),
        status,
        "an answered review must not be applied a second time"
    );
}

/// A removal with nothing behind it still goes on the click that asked.
#[test]
fn an_analysis_with_no_dependents_and_no_results_is_removed_at_once() {
    use super::super::lifecycle::remove_analysis_instance;
    use crate::simulation::plan::AnalysisKind;

    let mut app = RSpiceApp::test_instance();
    let ac = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .insert(AnalysisKind::Ac)
        .expect("AC inserts")
        .0;

    remove_analysis_instance(&mut app, ac);
    assert!(
        app.state.dialogs.plan_removal_review.target.is_none()
            && app.state.dialogs.plan_removal_refusal.analysis.is_none(),
        "an analysis nothing depends on must not raise a modal"
    );
    assert!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(ac)
            .is_none(),
        "it must be removed by the action that asked for it"
    );
}

/// The transaction stays fail-closed, and says so in names.
///
/// The surface refuses a blocked removal before it opens anything, so the
/// plan's own refusal is now only reached when the plan changed underneath a
/// staged review. That is exactly when a reader has the least context to spend
/// decoding two UUIDs, so the identities in the sentence are resolved to the
/// names the analyses show under before it reaches the status strip.
#[test]
fn the_plan_transaction_refuses_a_blocked_removal_and_names_the_analyses() {
    use super::super::lifecycle::apply_confirmed_analysis_removal;

    let (mut app, dependent, prerequisite) = bound_pair();

    // A review staged against a plan that has since bound something to this
    // analysis: the modal's answer reaches the transaction, and the
    // transaction is what refuses it.
    app.state.dialogs.plan_removal_review.open(
        crate::workbench::app::PlanRemovalTarget::Analysis(prerequisite),
        "Bias point".to_owned(),
        Vec::new(),
    );
    app.state.dialogs.plan_removal_review.confirmed = true;
    apply_confirmed_analysis_removal(&mut app);

    assert!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(prerequisite)
            .is_some(),
        "the refused removal must leave the plan unchanged"
    );
    let status = app
        .state
        .workbench
        .analysis_lifecycle_status
        .message()
        .to_owned();
    assert!(
        status.contains("Remove rejected fail-closed"),
        "the transaction must still refuse fail-closed, got: {status}"
    );
    assert!(
        status.contains("Bias point") && status.contains("Gain sweep"),
        "the refusal must name both analyses by what they show under, got: {status}"
    );
    assert!(
        status.contains(&prerequisite.to_string()) && status.contains(&dependent.to_string()),
        "and must keep the identities the plan actually refused on, got: {status}"
    );
}

/// One page's confirmed answer is never taken by another page.
///
/// Every registry reads the same review, and a page that took an answer it did
/// not stage would discard the removal the page that did stage it is waiting to
/// apply — silently, because a taken answer leaves nothing behind to notice.
#[test]
fn a_registry_only_takes_the_answer_it_staged() {
    let mut app = RSpiceApp::test_instance();
    let plan = plan_id(&app);
    let variable = crate::product::DesignVariableId::new();
    app.state.dialogs.plan_removal_review.open(
        crate::workbench::app::PlanRemovalTarget::Variable { plan, id: variable },
        "vref".to_owned(),
        Vec::new(),
    );
    app.state.dialogs.plan_removal_review.confirmed = true;

    assert!(
        app.state
            .dialogs
            .plan_removal_review
            .take_confirmed_analysis()
            .is_none(),
        "the analysis stack must not claim a variable's answer"
    );
    assert!(
        app.state
            .dialogs
            .plan_removal_review
            .take_confirmed_capture_group(plan)
            .is_none(),
        "the capture-group card must not claim a variable's answer"
    );
    assert_eq!(
        app.state
            .dialogs
            .plan_removal_review
            .take_confirmed_variable(plan),
        Some(variable),
        "the registry that staged it must still find its answer"
    );
    assert!(
        app.state.dialogs.plan_removal_review.target.is_none(),
        "an answer is taken exactly once"
    );
}

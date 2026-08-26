//! Destructive review before a plan removal that orphans something.
//!
//! Four registries share one review, and each has to answer the same three
//! questions: a removal with dependents stops and leaves the record standing
//! when the reader cancels, a confirmed removal commits with the same receipt
//! the immediate route writes, and a removal that orphans nothing never opens a
//! modal at all. These press the actual Remove control rather than calling the
//! handler, because "which route commits a plan mutation" is exactly what was
//! wrong: three of the four committed on the first click.

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

/// The analysis route this review was generalized from is unchanged.
///
/// Its staging rule, its wording and its two-frame handshake are the contract
/// the other three registries were made to match, so a refactor that quietly
/// changed the arm it was copied from would have moved the target.
#[test]
fn the_analysis_route_still_stages_and_applies_as_it_did() {
    use super::super::lifecycle::{apply_confirmed_analysis_removal, remove_analysis_instance};
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
    super::super::lifecycle::apply_analysis_action(
        &mut app,
        ac,
        super::super::AnalysisAction::RepairDependencies,
    );
    let prerequisite = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .instance(ac)
        .expect("AC is in the plan")
        .dependencies()[0]
        .target();

    // The prerequisite AC is bound to: reviewed, because removing it unbinds
    // an analysis that then refuses to run.
    remove_analysis_instance(&mut app, prerequisite);
    assert!(
        app.state.dialogs.plan_removal_review.target.is_some(),
        "an analysis another one is bound to must be reviewed before it is removed"
    );
    assert!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instance(prerequisite)
            .is_some(),
        "staging a review must not remove the analysis"
    );
    assert!(
        app.state
            .dialogs
            .plan_removal_review
            .consequences
            .iter()
            .any(|consequence| consequence.fact == "Analyses bound to it"),
        "the review must still state what removal unbinds"
    );

    // The confirmed answer is taken exactly once, on the surface's next frame,
    // and it drives the plan's own removal transaction — which stays
    // fail-closed: an analysis another one is still bound to is refused by the
    // plan, and the review's job is to say so before the reader finds out.
    app.state.dialogs.plan_removal_review.confirmed = true;
    apply_confirmed_analysis_removal(&mut app);
    assert!(
        app.state.dialogs.plan_removal_review.target.is_none(),
        "the review must close once its answer has been applied"
    );
    let status = app
        .state
        .workbench
        .analysis_lifecycle_status
        .message()
        .to_owned();
    assert!(
        status.contains("Remove"),
        "the confirmed removal must reach the plan's own transaction, got: {status}"
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

    // AC itself: nothing is bound to it and it has produced nothing, so it
    // goes without a modal.
    remove_analysis_instance(&mut app, ac);
    assert!(
        app.state.dialogs.plan_removal_review.target.is_none(),
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

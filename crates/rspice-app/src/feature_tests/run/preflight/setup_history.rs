//! Undo and Redo on the Simulate setup pages: each page's committed edit is
//! one step of the setup's own history, put back exactly, and Undo follows
//! the focus: on a Simulate page it takes back the setup's last edit, and on
//! the schematic the schematic's.

use super::engine::app_with_a_moded_device;
use super::setup_page::{Page, stored_options};
use super::*;
use rspice_app_simulate::workbench::state::workflow_drafts::SimulationWorkflowDialog;
use rspice_app_state::state::SimulationPage;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// What the first part on the sheet holds, as the schematic's own history
/// restores it.
fn first_value(app: &RSpiceApp) -> String {
    app.state.model.schematic.document().components[0]
        .value
        .clone()
}

/// A Solver edit, then a newer schematic edit. On the Solver page Undo takes
/// back the Solver edit and leaves the schematic's standing, and with no
/// setup step left it reaches no further; on the schematic Undo takes back
/// the schematic's edit and leaves the setup alone. Redo follows the focus
/// the same way.
#[test]
fn undo_takes_back_the_edit_of_the_place_it_is_pressed_in() {
    let mut page = Page::open_on(app_with_a_moded_device("").0, SimulationPage::Solver);
    let setup_before = stored_options(&page.app);
    page.type_slowly("relative update", "2e-3");
    let setup_after = stored_options(&page.app);
    assert_ne!(setup_before, setup_after);

    page.app.state.workbench.activate(Workspace::Design);
    let drawn = first_value(&page.app);
    page.app
        .state
        .model
        .schematic
        .with_undo("change value", |schematic| {
            schematic.document_mut_for_test().components[0].value = "7".to_owned();
        });
    assert_eq!(first_value(&page.app), "7");

    page.app.state.workbench.activate(Workspace::Simulate);
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        stored_options(&page.app),
        setup_before,
        "the setup's edit is undone"
    );
    assert_eq!(
        first_value(&page.app),
        "7",
        "the newer schematic edit stands"
    );
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        first_value(&page.app),
        "7",
        "a Simulate page never reaches past the setup into the design"
    );

    page.app.state.workbench.activate(Workspace::Design);
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        first_value(&page.app),
        drawn,
        "the schematic's edit is undone"
    );
    assert_eq!(
        stored_options(&page.app),
        setup_before,
        "the setup is left as it is"
    );

    page.app.execute_command_key(ids::REDO);
    assert_eq!(first_value(&page.app), "7");
    assert_eq!(stored_options(&page.app), setup_before);
    page.app.state.workbench.activate(Workspace::Simulate);
    page.app.execute_command_key(ids::REDO);
    assert_eq!(stored_options(&page.app), setup_after);
    assert_eq!(first_value(&page.app), "7");
}

/// The Solver page commits through the same plan transaction, so its edits
/// are steps of the same history: a value typed a keystroke at a time is one
/// step, Undo puts back exactly the options it replaced, and Redo the ones it
/// wrote.
#[test]
fn a_solver_value_typed_a_keystroke_at_a_time_is_one_step_of_undo() {
    let mut page = Page::open_on(app_with_a_moded_device("").0, SimulationPage::Solver);
    let before = stored_options(&page.app);
    page.type_slowly("relative update", "2e-3");
    assert_eq!(page.app.state.model.sim_setup.options.reltol, 2e-3);
    let after = stored_options(&page.app);

    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        stored_options(&page.app),
        before,
        "one Undo takes the whole value back, not its last keystroke"
    );
    page.pass(Vec::new());
    assert_ne!(
        page.control("relative update").value.as_deref(),
        Some("2e-3"),
        "the page shows what the step restored"
    );
    page.app.execute_command_key(ids::REDO);
    assert_eq!(stored_options(&page.app), after);
}

/// Steps are taken back one after another, newest first, each to exactly the
/// options it replaced, and Redo makes them again in order.
#[test]
fn two_solver_edits_undo_one_after_the_other_and_redo_in_order() {
    let mut page = Page::open_on(app_with_a_moded_device("").0, SimulationPage::Solver);
    let drawn = stored_options(&page.app);
    page.type_slowly("relative update", "2e-3");
    let first = stored_options(&page.app);
    page.type_slowly("relative update", "3e-3");
    let second = stored_options(&page.app);

    page.app.execute_command_key(ids::UNDO);
    assert_eq!(stored_options(&page.app), first, "the newer edit first");
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        stored_options(&page.app),
        drawn,
        "then the older one ({:?})",
        page.app.state.prompt_text()
    );
    page.app.execute_command_key(ids::REDO);
    assert_eq!(stored_options(&page.app), first);
    page.app.execute_command_key(ids::REDO);
    assert_eq!(stored_options(&page.app), second);
}

// ---------------------------------------------------------------- analyses

/// The analyses as a step has to put them back: each one's settings, name,
/// enablement and place, without the revision that stamped them or the state
/// a run left them in.
fn analyses(app: &RSpiceApp) -> serde_json::Value {
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the plan");
    let mut value = serde_json::to_value(plan).expect("the plan serializes");
    for instance in value["instances"]
        .as_array_mut()
        .expect("the plan's instances")
    {
        let fields = instance.as_object_mut().expect("an instance");
        fields.remove("modified_revision");
        fields.remove("lifecycle");
    }
    value["instances"].take()
}

/// The id of the analysis at `position` in the list.
fn analysis_at(app: &RSpiceApp, position: usize) -> rspice_app_state::product::AnalysisInstanceId {
    app.state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the plan")
        .instances()[position]
        .id()
}

/// Make `edit` on the Simulate page, then check it is one step of Undo, as
/// [`undo_and_redo`] does.
fn one_step<T: PartialEq + std::fmt::Debug>(
    app: &mut RSpiceApp,
    edit: &str,
    read: impl Fn(&RSpiceApp) -> T,
    make: impl FnOnce(&mut RSpiceApp),
) {
    app.state.workbench.activate(Workspace::Simulate);
    let before = read(app);
    make(app);
    undo_and_redo(app, edit, read, before);
}

/// Check the edit just made on the Simulate page is one step of Undo:
/// `read` gives back exactly `before` after Undo, and what the edit left
/// after Redo.
fn undo_and_redo<T: PartialEq + std::fmt::Debug>(
    app: &mut RSpiceApp,
    edit: &str,
    read: impl Fn(&RSpiceApp) -> T,
    before: T,
) {
    let after = read(app);
    assert_ne!(before, after, "the {edit} changes the setup");
    app.execute_command_key(ids::UNDO);
    assert_eq!(
        read(app),
        before,
        "Undo takes the {edit} back exactly ({:?})",
        app.state.prompt_text()
    );
    app.execute_command_key(ids::REDO);
    assert_eq!(read(app), after, "Redo makes the {edit} again");
}

/// Open `dialog` and press its `button`, as a hand does.
fn press_in_the_dialog(app: &mut RSpiceApp, dialog: SimulationWorkflowDialog, button: &str) {
    use rspice_app_simulate::workbench::simulate::SimulateFeature;

    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    app.state.simulate.simulation_workflow = Some(dialog);
    let frame = |app: &mut RSpiceApp, events: Vec<egui::Event>| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                SimulateFeature::render_workflow_dialogs(
                    &mut crate::workbench::features::cx(app),
                    ctx,
                );
                app.after_contribution();
            },
        )
    };
    let _ = frame(app, Vec::new());
    let output = frame(app, Vec::new());
    let update = output
        .platform_output
        .accesskit_update
        .expect("the dialog's tree");
    let bounds = update
        .nodes
        .iter()
        .find(|(_, node)| node.label() == Some(button))
        .and_then(|(_, node)| node.bounds())
        .unwrap_or_else(|| panic!("the dialog offers {button}"));
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let _ = frame(
        app,
        vec![
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
        ],
    );
}

/// Adding, renaming, reordering, disabling and removing an analysis are each
/// one step of Undo on the Simulate page: Undo puts the list back exactly,
/// every analysis with its settings, name, enablement and place, and Redo
/// makes the edit again.
/// Adding, renaming, reordering, disabling and removing an analysis are each
/// one step of Undo on the Simulate page: Undo puts the list back exactly,
/// every analysis with its settings, name, enablement and place, and Redo
/// makes the edit again.
#[test]
fn each_edit_of_the_analysis_list_is_one_step_put_back_exactly() {
    use rspice_app_simulate::workbench::state::workflow_drafts::RenameAnalysisDraft;
    use rspice_app_simulate::workbench::surfaces::simulate::AnalysisAction;
    use rspice_app_simulate::workbench::surfaces::simulate::lifecycle::{
        apply_analysis_action, insert_analysis_instance,
    };
    use rspice_app_state::simulation::plan::AnalysisKind;

    fn act(app: &mut RSpiceApp, position: usize, action: AnalysisAction) {
        let id = analysis_at(app, position);
        apply_analysis_action(&mut crate::workbench::features::cx(app), id, action);
    }

    let mut app = app_with_a_moded_device("").0;
    let drawn = analyses(&app);
    one_step(&mut app, "add", analyses, |app| {
        insert_analysis_instance(&mut crate::workbench::features::cx(app), AnalysisKind::Ac)
            .expect("an AC analysis is added");
    });
    one_step(&mut app, "rename", analyses, |app| {
        let mut draft = RenameAnalysisDraft::for_instance(analysis_at(app, 1), "", "");
        draft.name = "Loop gain".to_owned();
        press_in_the_dialog(
            app,
            SimulationWorkflowDialog::RenameAnalysis(draft),
            "Apply name",
        );
    });
    one_step(&mut app, "reorder", analyses, |app| {
        act(app, 1, AnalysisAction::Earlier(0));
    });
    one_step(&mut app, "disable", analyses, |app| {
        act(app, 0, AnalysisAction::SetEnabled(false));
    });
    one_step(&mut app, "remove", analyses, |app| {
        act(app, 0, AnalysisAction::Remove);
    });
    assert_eq!(
        analyses(&app),
        drawn,
        "the analysis added, renamed, moved, disabled and removed is gone again"
    );
}

// --------------------------------------------------------------- plan data

/// The active plan's design variables, outputs, capture groups and
/// specifications, in their stored form.
fn plan_data(app: &RSpiceApp) -> String {
    let model = &app.state.model;
    let plan = model
        .sim_setup
        .stable_analysis_plan()
        .expect("the plan")
        .id();
    serde_json::to_string(&model.workspace.content.plan_data(plan))
        .expect("the plan's data serializes")
}

/// A design variable created on its page is one step: Undo takes the plan's
/// data back exactly as it stood, and Redo creates the variable again.
#[test]
fn a_variable_created_on_its_page_is_one_step_put_back_exactly() {
    use rspice_app_simulate::workbench::state::workflow_drafts::DesignVariableDraft;

    let mut app = app_with_a_moded_device("").0;
    app.state.simulate.simulation_page = SimulationPage::Variables;
    one_step(&mut app, "new variable", plan_data, |app| {
        press_in_the_dialog(
            app,
            SimulationWorkflowDialog::DesignVariable(DesignVariableDraft::default()),
            "Create variable",
        );
    });
}

// ------------------------------------------------------------------ models

/// The model libraries bound to the plan, in order, with their sections.
fn bindings(app: &RSpiceApp) -> String {
    format!("{:?}", app.state.model.sim_setup.model_bindings)
}

/// A library detached on the Models page is one step: Undo binds it again
/// where it was, with its section, and Redo detaches it again.
#[test]
fn a_library_detached_on_the_models_page_is_one_step_put_back_exactly() {
    use rspice_app_simulate::workbench::surfaces::simulate::page_models::{
        ModelBindingAction, apply_model_binding_action,
    };

    let mut app = app_with_a_moded_device("").0;
    app.state.simulate.simulation_page = SimulationPage::Models;
    assert!(
        !app.state.model.sim_setup.model_bindings.is_empty(),
        "the design binds its connect modules"
    );
    one_step(&mut app, "detach", bindings, |app| {
        apply_model_binding_action(
            &mut crate::workbench::features::cx(app),
            ModelBindingAction::Remove(0),
        );
    });
}

// -------------------------------------------------------------------- save

/// What is saved and how it streams, as the plan holds it.
fn save_policy(app: &RSpiceApp) -> String {
    format!("{:?}", app.state.model.sim_setup.save_policy)
}

/// A switch on the Save & streaming page is one step: Undo puts the policy
/// back exactly, and Redo flips it again.
#[test]
fn a_save_policy_switch_is_one_step_put_back_exactly() {
    let mut page = Page::open_on(app_with_a_moded_device("").0, SimulationPage::Save);
    let before = save_policy(&page.app);
    page.click("Permit live delivery for outputs that request it");
    undo_and_redo(&mut page.app, "streaming switch", save_policy, before);
}

// ----------------------------------------------------------------- run set

/// The run set's declared space: its axes, how they compose, its budgets and
/// the next axis identity, without the revision a restore moves on.
fn run_space(app: &RSpiceApp) -> String {
    let run_set = &app.state.model.sim_setup.run_set;
    format!(
        "{:?}",
        (
            run_set.sequence,
            &run_set.dimensions,
            &run_set.composition,
            &run_set.budgets
        )
    )
}

/// An axis added on the PVT, sweeps & variation page is one step of the
/// setup's one Undo: Undo puts the space back exactly, and Redo adds the axis
/// again.
#[test]
fn a_run_set_axis_added_on_its_page_is_one_step_put_back_exactly() {
    use rspice_app_simulate::workbench::surfaces::simulate::page_runset::commit;
    use rspice_simulation_contract::run_set::{RunSetAction, RunSetDimensionKind};

    let mut app = app_with_a_moded_device("").0;
    app.state.simulate.simulation_page = SimulationPage::RunSet;
    one_step(&mut app, "new axis", run_space, |app| {
        commit(
            &mut crate::workbench::features::cx(app),
            RunSetAction::AddDimension(RunSetDimensionKind::Parameter),
        );
    });
}

/// The reference process and temperature, and the temperature the options
/// state with them.
fn reference_pvt(app: &RSpiceApp) -> String {
    let setup = &app.state.model.sim_setup;
    format!("{:?}", (setup.reference_pvt, setup.options.temp))
}

/// A reference PVT chosen is one step: Undo puts the process and temperature
/// back exactly, and Redo chooses them again.
#[test]
fn a_reference_pvt_chosen_is_one_step_put_back_exactly() {
    let mut app = app_with_a_moded_device("").0;
    one_step(&mut app, "reference PVT", reference_pvt, |app| {
        rspice_app_simulate::workbench::simulate::chrome::commit_reference_pvt(
            &mut crate::workbench::features::cx(app),
            rspice_app_state::product::ProcessCorner::FF,
            -40.0,
        )
        .expect("the reference PVT commits");
    });
}

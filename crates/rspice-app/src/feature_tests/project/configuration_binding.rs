//! Binding an instance to another cell on the Configuration page, through
//! whole application frames: the picker offers what fits and says why the
//! rest does not, the bound cell is what the netlist and the run use, a cell
//! that stops fitting is a problem on its row and blocks the run, and Undo
//! puts the configuration back exactly.

#![cfg(not(target_arch = "wasm32"))]

use egui::accesskit::Role;

use super::configuration_editing::{
    click, disabled, nodes, page_app, paint_until_checked, pick, says, settle, shown,
};
use super::configuration_tables::{row_at, rows, select, take_from_menu};
use super::recovery_frames::offscreen;
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::configuration_state::{
    ConfigurationRow, instance_binding,
};
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_state::state::{CellViewRef, ConfigurationPlatform, ConfigurationSetOverride};
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// The design deck the run executes, as it stands.
fn deck(app: &RSpiceApp) -> Result<String, String> {
    SimulationController::prepare_design_netlist_for_inspection(&app.state.model)
        .map_err(|error| error.to_string())
}

/// The master the top-level card of `instance` names: the last word of its
/// card, outside every definition.
fn master_of(deck: &str, instance: &str) -> String {
    let needle = format!("{} ", instance.to_ascii_lowercase());
    let mut depth = 0_usize;
    deck.lines()
        .find(|line| {
            let folded = line.trim_start().to_ascii_lowercase();
            if folded.starts_with(".subckt ") {
                depth += 1;
                return false;
            }
            if folded.starts_with(".ends") {
                depth = depth.saturating_sub(1);
                return false;
            }
            depth == 0 && folded.starts_with(&needle)
        })
        .and_then(|line| line.split_whitespace().last())
        .unwrap_or_else(|| panic!("no card for {instance} in:\n{deck}"))
        .to_owned()
}

/// Bind `/XF1` to `vendor/filter` in the configuration runs use, as a
/// project that applied the binding holds it.
fn bound_xf1(app: &mut RSpiceApp) {
    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    let id = catalog
        .active_configuration_id()
        .expect("one is used for runs");
    let committed = catalog.find(id).expect("the configuration exists");
    let revision = committed.revision();
    let mut definition = committed.definition().clone();
    definition.overrides.push(ConfigurationSetOverride {
        instance_path: "/XF1".to_owned(),
        executable_views: vec!["schematic".to_owned()],
        stop_view: None,
        model_section: None,
        eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
        target: Some(rspice_design::configuration_set::BindingTarget::new(
            "vendor", "filter",
        )),
    });
    catalog
        .update(id, revision, definition)
        .expect("the binding holds");
    app.state
        .model
        .workspace
        .content
        .replace_configuration_sets(catalog)
        .expect("the catalog publishes");
}

/// XF1 places `user/filter`. Bound from its row to `vendor/filter`, picked
/// in the picker, the row states the cell it runs; applied, XF1 runs the
/// master XF2 runs, which holds the vendor's body, and the user's is no
/// longer emitted; Undo puts the configuration back exactly as it was.
#[test]
fn an_instance_bound_to_another_cell_runs_it_and_undo_restores_the_configuration() {
    let ctx = offscreen();
    let mut app = page_app();
    let before = app.state.model.workspace.content.configuration_sets.clone();
    let placed = deck(&app).expect("the reference netlists");
    let user_body = body(&placed, &master_of(&placed, "XF1"));
    let vendor_body = body(&placed, &master_of(&placed, "XF2"));
    assert!(!vendor_body.is_empty(), "the vendor's filter is defined");
    assert_ne!(user_body, vendor_body, "two filters, two bodies");

    let output = settle(&ctx, &mut app, Vec::new());
    let xf1 = row_at(&output, "Instance: XF1,");
    select(&ctx, &mut app, xf1);
    let output = take_from_menu(&ctx, &mut app, xf1, "Bind to cell", None);
    assert!(
        says(&output, "Bind /XF1 to another cell"),
        "the picker names what it binds"
    );
    let output = pick(&ctx, &mut app, &output, "Library", "vendor");
    let filter = row_at(&output, "Cell: filter, View: schematic");
    let output = select(&ctx, &mut app, filter);
    assert!(!disabled(&output, "Bind"), "a cell that fits is offered");
    click(&ctx, &mut app, &output, Role::Button, |name| name == "Bind");

    let definition = shown(&app);
    let binding = instance_binding(&definition, "/XF1").expect("XF1 has a binding of its own");
    assert_eq!(
        binding
            .target
            .as_ref()
            .map(rspice_design::configuration_set::BindingTarget::display_path)
            .as_deref(),
        Some("vendor/filter")
    );
    assert_eq!(binding.executable_views, ["schematic"]);
    let output = settle(&ctx, &mut app, Vec::new());
    let row = rows(&output)
        .into_iter()
        .find(|row| row.label.starts_with("Instance: XF1,"))
        .expect("XF1 is shown");
    assert!(
        row.label
            .contains("Cell: user/filter, Cell to use: vendor/filter"),
        "the row states the placed cell and the one it runs: {}",
        row.label
    );

    click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Apply"
    });
    paint_until_checked(&ctx, &mut app);
    let bound = deck(&app).expect("the bound design netlists");
    let master = master_of(&bound, "XF1");
    assert_eq!(
        master,
        master_of(&bound, "XF2"),
        "XF1 and XF2 run one cell:\n{bound}"
    );
    assert_eq!(
        body(&bound, &master),
        vendor_body,
        "XF1 runs the vendor's filter:\n{bound}"
    );
    assert_eq!(
        bound
            .lines()
            .filter(|line| line
                .trim_start()
                .to_ascii_lowercase()
                .starts_with(".subckt "))
            .count(),
        placed
            .lines()
            .filter(|line| line
                .trim_start()
                .to_ascii_lowercase()
                .starts_with(".subckt "))
            .count()
            - 1,
        "the user's filter is no longer emitted:\n{bound}"
    );

    assert!(app.command_key_enabled(ids::UNDO));
    app.execute_command_key(ids::UNDO);
    settle(&ctx, &mut app, Vec::new());
    assert_eq!(
        app.state.model.workspace.content.configuration_sets, before,
        "Undo restores the configuration exactly"
    );
    let undone = deck(&app).expect("netlists");
    assert_eq!(body(&undone, &master_of(&undone, "XF1")), user_body);
}

/// The lines of `master`'s definition between its header and its end.
fn body(deck: &str, master: &str) -> Vec<String> {
    let header = format!(".subckt {} ", master.to_ascii_lowercase());
    deck.lines()
        .skip_while(|line| !line.trim_start().to_ascii_lowercase().starts_with(&header))
        .skip(1)
        .take_while(|line| !line.trim_start().to_ascii_lowercase().starts_with(".ends"))
        .map(str::to_owned)
        .collect()
}

/// `user/amp` holds none of the pins XF1 places. The picker lists it under
/// what does not fit with those pins named, Bind refuses it saying the same,
/// and the binding asked for directly is refused before anything changes.
#[test]
fn a_cell_without_the_placed_pins_is_refused_with_them_named() {
    let ctx = offscreen();
    let mut app = page_app();
    let output = settle(&ctx, &mut app, Vec::new());
    let xf1 = row_at(&output, "Instance: XF1,");
    select(&ctx, &mut app, xf1);
    let output = take_from_menu(&ctx, &mut app, xf1, "Bind to cell", None);
    let amp = rows(&output)
        .into_iter()
        .find(|row| row.label.starts_with("Cell: amp, View: schematic"))
        .expect("the user library's amp is offered");
    assert!(
        amp.label.contains("it has no pin A or B"),
        "the row names the pins: {}",
        amp.label
    );
    let output = select(&ctx, &mut app, amp.at.expect("the row is on the screen"));
    assert!(
        disabled(&output, "Bind"),
        "a cell that does not fit is not bound"
    );
    assert!(
        nodes(&output).iter().any(
            |node| node.label.contains("Does not fit: it has no pin A or B")
                || node.value.contains("Does not fit: it has no pin A or B")
        ),
        "Bind says why"
    );

    let before = shown(&app);
    let id = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .active_configuration_id()
        .expect("one is used for runs");
    let model = &app.state.model;
    assert!(!app.state.project.configuration.bind_to_cell(
        &model.workspace,
        &model.library_manager,
        &model.schematic,
        id,
        &[ConfigurationRow::Instance("/XF1".to_owned())],
        Some(&CellViewRef::new("user", "amp", "schematic")),
    ));
    let refusal = app
        .state
        .project
        .configuration
        .refusal
        .clone()
        .expect("the refusal is stated");
    assert!(
        refusal.contains("does not fit /XF1: it has no pin A or B"),
        "{refusal}"
    );
    assert_eq!(shown(&app), before, "nothing changed");
}

/// A binding that fitted when it was made stops fitting when the bound cell
/// loses a pin: its row says the pins do not fit, the deck is not written,
/// and the run is blocked with the binding named.
#[test]
fn a_binding_that_stops_fitting_is_a_problem_and_never_netlists() {
    let ctx = offscreen();
    let mut app = page_app();
    bound_xf1(&mut app);
    deck(&app).expect("the binding fits as made");

    let key = CellViewRef::new("vendor", "filter", "schematic").key();
    let mut vendor = app
        .state
        .model
        .workspace
        .schematic_editor_mut(&key)
        .expect("the vendor's filter is held");
    vendor
        .editor
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.value == "B")
        .expect("the filter's B port")
        .value = "Y".to_owned();
    drop(vendor);

    let output = settle(&ctx, &mut app, Vec::new());
    let row = rows(&output)
        .into_iter()
        .find(|row| row.label.starts_with("Instance: XF1,"))
        .expect("XF1 is shown");
    assert!(
        row.label.contains("pins do not fit"),
        "the row says so: {}",
        row.label
    );
    assert!(
        deck(&app).is_err(),
        "a binding that does not fit never netlists"
    );
    let report = rspice_app_runs::workbench::preflight::collect_report(&app.state);
    let blocker = report
        .blockers
        .iter()
        .find(|issue| issue.check == "Hierarchy binding")
        .expect("the binding blocks the run");
    assert!(
        blocker
            .observed
            .contains("vendor/filter no longer fits /XF1: it has no pin B"),
        "{}",
        blocker.observed
    );
}

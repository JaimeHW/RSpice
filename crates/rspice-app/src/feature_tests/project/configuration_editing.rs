//! Editing a configuration on the Configuration page through whole
//! application frames: what the inspector inherits and keeps, which entries
//! are refused where they are made, how the tables state a binding's list,
//! and how Apply, Use for runs, Revert and Delete answer.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use egui::accesskit::Role;
use egui::{Context, Modifiers, Pos2, pos2};

use super::recovery_frames::{app_frame, offscreen};
use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference;
use rspice_app_project::workbench::app::dialogs::project::CheckpointJobHold;
use rspice_app_project::workbench::app::dialogs::project::configuration_dialogs::{
    ConfigurationDialogRequest, open_configuration_dialog,
};
use rspice_app_project::workbench::project::configuration_state::{
    ConfigurationRow, ConfigurationSelection, ConfigurationTab, instance_binding, set_cell_view,
};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::state::{
    ConfigurationCellBinding, ConfigurationPlatform, ConfigurationSetDefinition, ConfigurationSetId,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// The reference project on Project > Configuration, the configuration it
/// runs with in front.
fn page_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Configuration;
    app
}

/// One announced control: its role, name, value, whether it is offered,
/// and its centre.
struct Node {
    role: Role,
    label: String,
    value: String,
    disabled: bool,
    at: Option<Pos2>,
}

fn nodes(output: &egui::FullOutput) -> Vec<Node> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .map(|(_, node)| Node {
            role: node.role(),
            label: node.label().unwrap_or_default().to_owned(),
            value: node.value().unwrap_or_default().to_owned(),
            disabled: node.is_disabled(),
            at: node.bounds().map(|bounds| {
                pos2(
                    ((bounds.x0 + bounds.x1) / 2.0) as f32,
                    ((bounds.y0 + bounds.y1) / 2.0) as f32,
                )
            }),
        })
        .collect()
}

fn says(output: &egui::FullOutput, text: &str) -> bool {
    nodes(output)
        .iter()
        .any(|node| node.label == text || node.value == text)
}

/// Frames until what was asked for has drawn: a press lands, a popup opens.
fn settle(ctx: &Context, app: &mut RSpiceApp, events: Vec<egui::Event>) -> egui::FullOutput {
    app_frame(ctx, app, events);
    app_frame(ctx, app, Vec::new());
    app_frame(ctx, app, Vec::new())
}

/// Press and release on the control `output` announced with `role` and a
/// name for which `named` holds, then let the page answer.
fn click(
    ctx: &Context,
    app: &mut RSpiceApp,
    output: &egui::FullOutput,
    role: Role,
    named: impl Fn(&str) -> bool,
) -> egui::FullOutput {
    let all = nodes(output);
    let at = all
        .iter()
        .find(|node| node.role == role && named(&node.label))
        .and_then(|node| node.at)
        .unwrap_or_else(|| {
            panic!(
                "no {role:?} to press in {:#?}",
                all.iter()
                    .filter(|node| node.role == role)
                    .map(|node| &node.label)
                    .collect::<Vec<_>>()
            )
        });
    press_at(ctx, app, at)
}

/// Press the button named `name` that sits lowest on the screen: the one
/// in the dialog over the page, where the page has one of the same name.
fn click_lowest(
    ctx: &Context,
    app: &mut RSpiceApp,
    output: &egui::FullOutput,
    name: &str,
) -> egui::FullOutput {
    let at = nodes(output)
        .into_iter()
        .filter(|node| node.role == Role::Button && node.label == name)
        .filter_map(|node| node.at)
        .max_by(|a, b| a.y.total_cmp(&b.y))
        .unwrap_or_else(|| panic!("no {name:?} button"));
    press_at(ctx, app, at)
}

fn press_at(ctx: &Context, app: &mut RSpiceApp, at: Pos2) -> egui::FullOutput {
    settle(
        ctx,
        app,
        vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::default(),
            },
        ],
    )
}

/// Open the dropdown named `label` and pick `option` from it.
fn pick(
    ctx: &Context,
    app: &mut RSpiceApp,
    output: &egui::FullOutput,
    label: &str,
    option: &str,
) -> egui::FullOutput {
    let open = click(ctx, app, output, Role::ComboBox, |name| name == label);
    click(ctx, app, &open, Role::ListBoxOption, |name| name == option)
}

/// Whether the button `output` announced as `name` is drawn but not offered.
fn disabled(output: &egui::FullOutput, name: &str) -> bool {
    nodes(output)
        .iter()
        .any(|node| node.role == Role::Button && node.label == name && node.disabled)
}

/// Paint until no check runs, as the reader's screen would while one does.
fn paint_until_checked(ctx: &Context, app: &mut RSpiceApp) -> egui::FullOutput {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        app_frame(ctx, app, Vec::new());
        if !app.state.project.configuration.is_checking() {
            return settle(ctx, app, Vec::new());
        }
        assert!(Instant::now() < deadline, "the check never answered");
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// A second configuration made from the one runs use, under `name`, with
/// `change` made to it; runs keep using the first.
fn second_configuration(
    app: &mut RSpiceApp,
    name: &str,
    change: impl FnOnce(&mut ConfigurationSetDefinition),
) -> ConfigurationSetId {
    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    let active = catalog.active_configuration_id().unwrap();
    let mut definition = catalog.find(active).unwrap().definition().clone();
    definition.name = name.to_owned();
    change(&mut definition);
    let id = catalog.create(definition).unwrap();
    catalog.activate(active).unwrap();
    app.state
        .model
        .workspace
        .content
        .replace_configuration_sets(catalog)
        .unwrap();
    id
}

fn select_row(app: &mut RSpiceApp, row: ConfigurationRow) {
    app.state.project.configuration.row = Some(row);
}

/// The definition the page shows for the configuration in front.
fn shown(app: &RSpiceApp) -> rspice_app_state::state::ConfigurationSetDefinition {
    let catalog = &app.state.model.workspace.content.configuration_sets;
    let id = catalog
        .active_configuration_id()
        .expect("one is used for runs");
    app.state
        .project
        .configuration
        .definition(catalog, id)
        .expect("the configuration exists")
        .clone()
}

/// Edit the configuration in front directly, as a fixture.
fn edit(
    app: &mut RSpiceApp,
    change: impl FnOnce(&mut rspice_app_state::state::ConfigurationSetDefinition),
) {
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    assert!(app.state.project.configuration.edit(&catalog, id, change));
}

/// An instance a cell binding governs shows that binding's list and stop as
/// what it inherits, and a platform restriction set on it alone stores no
/// view list: the instance keeps the view the cell binding chose.
#[test]
fn an_inspector_edit_keeps_what_the_cell_binding_chose() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        definition.cell_bindings.push(ConfigurationCellBinding {
            library: "user".to_owned(),
            cell: "amp".to_owned(),
            executable_views: vec!["extracted".to_owned(), "schematic".to_owned()],
            stop_view: Some("schematic".to_owned()),
        });
    });
    select_row(&mut app, ConfigurationRow::Instance("/X1".to_owned()));
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        says(&output, "View list 1: extracted"),
        "inherits the cell binding's list"
    );
    assert!(says(&output, "View list 2: schematic"));

    let output = pick(&ctx, &mut app, &output, "Only on", "Desktop");
    let definition = shown(&app);
    let binding = instance_binding(&definition, "/X1").expect("a binding of its own");
    assert_eq!(binding.eligible_platforms, [ConfigurationPlatform::Desktop]);
    assert!(
        binding.executable_views.is_empty(),
        "a platform set alone copies no view list: {:?}",
        binding.executable_views
    );
    assert!(
        nodes(&output).iter().any(|node| node
            .label
            .starts_with("Instance /X1: cell user/amp; view found schematic; view to use Inherited; bound by Cell binding")),
        "the cell binding still binds /X1"
    );
}

/// Choosing to inherit the view list keeps the binding's platform
/// restriction: the three properties are independent.
#[test]
fn inheriting_the_view_list_keeps_the_rest_of_the_binding() {
    let ctx = offscreen();
    let mut app = page_app();
    select_row(&mut app, ConfigurationRow::Instance("/X2".to_owned()));
    let output = settle(&ctx, &mut app, Vec::new());
    let output = pick(&ctx, &mut app, &output, "Only on", "Desktop");
    click(
        &ctx,
        &mut app,
        &output,
        egui::accesskit::Role::Button,
        |name| name == "Inherit the view list",
    );
    let definition = shown(&app);
    let binding = instance_binding(&definition, "/X2").expect("the binding stays");
    assert!(binding.executable_views.is_empty());
    assert_eq!(binding.eligible_platforms, [ConfigurationPlatform::Desktop]);
}

/// A model section the configuration cannot store is refused in its field,
/// which keeps what was typed and says why; the draft and the tables stay.
/// The global view list offers no Remove on its last view.
#[test]
fn an_invalid_entry_is_refused_at_its_field_and_the_tables_stay() {
    let ctx = offscreen();
    let mut app = page_app();
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(says(&output, "View list 1: schematic"));
    assert!(
        !nodes(&output)
            .iter()
            .any(|node| node.label == "Remove schematic from View list"),
        "the global view list's only view cannot be removed"
    );
    select_row(&mut app, ConfigurationRow::Instance("/X1".to_owned()));
    settle(&ctx, &mut app, Vec::new());
    let field = egui::Id::new(("configuration-field", "instance-/X1-section"));
    ctx.memory_mut(|memory| memory.request_focus(field));
    app_frame(&ctx, &mut app, Vec::new());
    app_frame(&ctx, &mut app, vec![egui::Event::Text("t t".to_owned())]);
    let output = settle(
        &ctx,
        &mut app,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
    );
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    assert!(
        !app.state.project.configuration.is_dirty(&catalog),
        "the refused section reached the draft"
    );
    let held = app
        .state
        .project
        .configuration
        .field_edit
        .clone()
        .expect("the field keeps what was typed");
    assert_eq!(held.text, "t t");
    let refusal = held.refusal.expect("the field says why");
    assert!(
        nodes(&output)
            .iter()
            .any(|node| node.label.contains(&refusal) || node.value.contains(&refusal)),
        "{refusal:?} is said at the field"
    );
    assert!(says(
        &output,
        "Instance columns: Instance, Cell, View found, View to use, Bound by, Model section, Status"
    ));
}

/// The Cells table states a binding's whole list, and picking a view puts
/// it first instead of dropping the others.
#[test]
fn view_to_use_states_the_whole_list_and_a_pick_moves_a_view_first() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        set_cell_view(definition, "user", "amp", Some("schematic"));
        set_cell_view(definition, "user", "amp", Some("extracted"));
    });
    app.state.project.configuration.tab = ConfigurationTab::Cells;
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        nodes(&output).iter().any(|node| node.label.starts_with(
            "Cell user/amp: 2 instances; view found schematic; view to use extracted, schematic"
        )),
        "the row states every view of the binding"
    );
    pick(
        &ctx,
        &mut app,
        &output,
        "View to use for user/amp",
        "schematic",
    );
    let definition = shown(&app);
    let binding = definition
        .cell_bindings
        .iter()
        .find(|binding| binding.library == "user" && binding.cell == "amp")
        .expect("the binding stays");
    assert_eq!(binding.executable_views, ["schematic", "extracted"]);
}

/// The configuration's name is a fact in the inspector; the band's More menu
/// renames it.
#[test]
fn the_inspector_states_the_name_and_rename_is_the_one_way_to_change_it() {
    let ctx = offscreen();
    let mut app = page_app();
    let output = settle(&ctx, &mut app, Vec::new());
    let all = nodes(&output);
    assert!(
        all.iter()
            .any(|node| node.value == "Reference hierarchy" || node.label == "Reference hierarchy"),
        "the name is stated"
    );
    assert!(
        !all.iter()
            .any(|node| node.role == egui::accesskit::Role::TextInput && node.label == "Name"),
        "the inspector edits no name"
    );
    let menu = click(
        &ctx,
        &mut app,
        &output,
        egui::accesskit::Role::Button,
        |name| name == "More configuration actions",
    );
    assert!(says(&menu, "Rename\u{2026}"));
}

/// The band says when the edits are not applied, and Use for runs on a
/// configuration with edits uses what the tables show: it applies them.
#[test]
fn use_for_runs_applies_the_edits_it_was_pressed_over() {
    let ctx = offscreen();
    let mut app = page_app();
    let second = second_configuration(&mut app, "Second", |_| {});
    app.state
        .project
        .configuration
        .select(ConfigurationSelection::Configuration(second));
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    assert!(
        app.state
            .project
            .configuration
            .edit(&catalog, second, |definition| {
                definition.executable_view_policy =
                    vec!["schematic".to_owned(), "spice".to_owned()];
            })
    );
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(says(&output, "Edits not applied"));

    click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Use for runs"
    });
    let output = paint_until_checked(&ctx, &mut app);

    let catalog = &app.state.model.workspace.content.configuration_sets;
    assert_eq!(catalog.active_configuration_id(), Some(second));
    assert_eq!(
        catalog.find(second).unwrap().executable_view_policy(),
        ["schematic", "spice"],
        "the edits were applied first"
    );
    assert!(says(&output, "Used for runs"));
    assert!(!says(&output, "Edits not applied"));
}

/// While Apply is checked the band says so and offers neither Apply nor Use
/// for runs. An edit made meanwhile stops the check; the next Apply lands.
#[test]
fn apply_is_checked_off_the_frame_and_an_edit_stops_the_check() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        definition.executable_view_policy = vec!["schematic".to_owned(), "spice".to_owned()];
    });
    let before = app.state.model.workspace.content.configuration_sets.clone();
    let hold = CheckpointJobHold::default();
    app.state.project.configuration.check_hold = Some(hold.clone());
    let output = settle(&ctx, &mut app, Vec::new());

    let output = click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Apply"
    });
    assert!(
        says(&output, "Checking\u{2026}"),
        "the band says it is checking"
    );
    assert!(
        disabled(&output, "Apply"),
        "Apply is locked while it is checked"
    );
    assert_eq!(app.state.model.workspace.content.configuration_sets, before);

    edit(&mut app, |definition| {
        definition.dut_path = "/X2".to_owned()
    });
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        !says(&output, "Checking\u{2026}"),
        "the edit stopped the check"
    );
    assert!(says(&output, "Edits not applied"));
    hold.release();
    settle(&ctx, &mut app, Vec::new());
    assert_eq!(
        app.state.model.workspace.content.configuration_sets, before,
        "a stopped check lands nothing"
    );

    app.state.project.configuration.check_hold = None;
    click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Apply"
    });
    let output = paint_until_checked(&ctx, &mut app);
    let catalog = &app.state.model.workspace.content.configuration_sets;
    let id = catalog.active_configuration_id().unwrap();
    let applied = catalog.find(id).unwrap();
    assert_eq!(applied.executable_view_policy(), ["schematic", "spice"]);
    assert_eq!(applied.definition().dut_path, "/X2");
    assert!(!says(&output, "Edits not applied"));
}

/// Revert asks before it drops the edits: Keep editing keeps them.
#[test]
fn revert_asks_before_it_drops_the_edits() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        definition.dut_path = "/X2".to_owned()
    });
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let output = settle(&ctx, &mut app, Vec::new());

    let asked = click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Revert"
    });
    assert!(
        nodes(&asked)
            .iter()
            .any(|node| node.label == "Revert the edits to Reference hierarchy?"),
        "Revert asks first"
    );
    assert!(app.state.project.configuration.is_dirty(&catalog));
    let kept = click(&ctx, &mut app, &asked, Role::Button, |name| {
        name == "Keep editing"
    });
    assert!(
        app.state.project.configuration.is_dirty(&catalog),
        "Keep editing keeps them"
    );

    let asked = click(&ctx, &mut app, &kept, Role::Button, |name| name == "Revert");
    click_lowest(&ctx, &mut app, &asked, "Revert");
    assert!(
        !app.state.project.configuration.is_dirty(&catalog),
        "Revert drops them"
    );
}

/// Model section offers the sections of the technology the project
/// attaches, and picking one binds it.
#[test]
fn model_section_offers_the_attached_technology_sections() {
    let ctx = offscreen();
    let mut app = page_app();
    app.state
        .model
        .provision_test_project_technology_with_project_models(sectioned_models());
    let sections = app
        .state
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .expect("a technology is attached")
        .process_sections()
        .to_vec();
    let section = sections
        .last()
        .expect("the technology has sections")
        .clone();
    select_row(&mut app, ConfigurationRow::Instance("/X1".to_owned()));
    let output = settle(&ctx, &mut app, Vec::new());

    pick(&ctx, &mut app, &output, "Technology sections", &section);

    let definition = shown(&app);
    assert_eq!(
        instance_binding(&definition, "/X1")
            .expect("a binding of its own")
            .model_section
            .as_deref(),
        Some(section.as_str())
    );
}

/// The DUT instance is optional: its picker offers the top cell itself and
/// the instances the top cell places.
#[test]
fn the_dut_picker_offers_the_top_cell_and_its_instances() {
    let ctx = offscreen();
    let mut app = page_app();
    let output = settle(&ctx, &mut app, Vec::new());
    let open = click(&ctx, &mut app, &output, Role::ComboBox, |name| {
        name == "DUT instance"
    });
    let offered = nodes(&open)
        .into_iter()
        .filter(|node| node.role == Role::ListBoxOption)
        .map(|node| node.label)
        .collect::<Vec<_>>();
    assert!(offered.iter().any(|path| path == "/"), "{offered:?}");
    assert!(offered.iter().any(|path| path == "/X1"), "{offered:?}");
    assert!(
        offered.iter().all(|path| path.matches('/').count() <= 1),
        "only the top cell's own instances: {offered:?}"
    );

    click(&ctx, &mut app, &open, Role::ListBoxOption, |name| {
        name == "/"
    });
    assert_eq!(shown(&app).dut_path, "/");
}

/// Delete names every binding that leaves with the configuration, cell
/// bindings too, says it once, and is one step of Undo.
#[test]
fn delete_counts_every_binding_and_undo_brings_the_configuration_back() {
    let ctx = offscreen();
    let mut app = page_app();
    let spare = second_configuration(&mut app, "Spare", |definition| {
        definition.cell_bindings.push(ConfigurationCellBinding {
            library: "user".to_owned(),
            cell: "amp".to_owned(),
            executable_views: vec!["schematic".to_owned()],
            stop_view: None,
        });
    });
    open_configuration_dialog(
        &mut app.state.project.configuration_dialog,
        &app.state.model,
        ConfigurationDialogRequest::Delete(spare),
    );
    let output = settle(&ctx, &mut app, Vec::new());
    let consequence = "Spare, its 1 instance binding and its 1 cell binding leave the project. \
                       Runs that used it before are not affected, and Undo brings it back.";
    let said = announced(&output)
        .iter()
        .filter(|text| text.contains(consequence))
        .count();
    assert_eq!(said, 1, "the consequence is said once");

    click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Delete"
    });
    let catalog = &app.state.model.workspace.content.configuration_sets;
    assert!(catalog.find(spare).is_none(), "Delete took it away");

    assert!(app.command_key_enabled(ids::UNDO));
    app.execute_command_key(ids::UNDO);
    let catalog = &app.state.model.workspace.content.configuration_sets;
    let back = catalog.find(spare).expect("Undo brought it back");
    assert_eq!(back.cell_bindings().len(), 1);
}

/// Configuration changes undo and redo in turn: undoing one moves the
/// project on, and the change before it still undoes.
#[test]
fn configuration_changes_undo_and_redo_in_turn() {
    let ctx = offscreen();
    let mut app = page_app();
    let spare = second_configuration(&mut app, "Spare", |_| {});
    let extra = second_configuration(&mut app, "Extra", |_| {});
    for id in [spare, extra] {
        open_configuration_dialog(
            &mut app.state.project.configuration_dialog,
            &app.state.model,
            ConfigurationDialogRequest::Delete(id),
        );
        let output = settle(&ctx, &mut app, Vec::new());
        click(&ctx, &mut app, &output, Role::Button, |name| {
            name == "Delete"
        });
    }
    let present = |app: &RSpiceApp| {
        let catalog = &app.state.model.workspace.content.configuration_sets;
        [spare, extra].map(|id| catalog.find(id).is_some())
    };
    assert_eq!(present(&app), [false, false], "both deleted");

    for expected in [[false, true], [true, true]] {
        assert!(app.command_key_enabled(ids::UNDO));
        app.execute_command_key(ids::UNDO);
        settle(&ctx, &mut app, Vec::new());
        assert_eq!(present(&app), expected, "Undo takes back one delete");
    }
    for expected in [[false, true], [false, false]] {
        assert!(app.command_key_enabled(ids::REDO));
        app.execute_command_key(ids::REDO);
        settle(&ctx, &mut app, Vec::new());
        assert_eq!(present(&app), expected, "Redo makes one delete again");
    }
}

/// Renaming the design's top cell while the page holds edits carries the
/// edits over: they stay, they take the new root, and Apply lands them.
#[test]
fn edits_follow_a_rename_of_the_design_they_were_made_to() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        definition.executable_view_policy = vec!["schematic".to_owned(), "spice".to_owned()];
    });
    settle(&ctx, &mut app, Vec::new());

    app.state
        .rename_cell("user", "top", "top_b")
        .expect("the top cell renames");
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(says(&output, "Edits not applied"), "the edits stay");
    assert!(!says(&output, OUT_OF_DATE));
    let edited = shown(&app);
    assert_eq!(edited.root.cell, "top_b", "the edits took the rename");
    assert_eq!(edited.executable_view_policy, ["schematic", "spice"]);

    click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Apply"
    });
    let output = paint_until_checked(&ctx, &mut app);
    let catalog = &app.state.model.workspace.content.configuration_sets;
    let id = catalog.active_configuration_id().unwrap();
    let applied = catalog.find(id).unwrap();
    assert_eq!(applied.definition().root.cell, "top_b");
    assert_eq!(applied.executable_view_policy(), ["schematic", "spice"]);
    assert!(!says(&output, "Edits not applied"), "Apply landed them");
}

const OUT_OF_DATE: &str = "Edits out of date";

/// A change the project makes to a part the edits change too cannot be
/// carried over: the band says so at once, Apply is not offered, and Revert
/// takes the page to the project's change.
#[test]
fn edits_the_project_changed_under_say_so_at_once() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        definition.executable_view_policy = vec!["schematic".to_owned(), "spice".to_owned()];
    });
    settle(&ctx, &mut app, Vec::new());

    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    let id = catalog.active_configuration_id().unwrap();
    let committed = catalog.find(id).unwrap();
    let revision = committed.revision();
    let mut elsewhere = committed.definition().clone();
    elsewhere.executable_view_policy = vec!["spice".to_owned(), "schematic".to_owned()];
    catalog.update(id, revision, elsewhere).unwrap();
    app.state
        .model
        .workspace
        .content
        .replace_configuration_sets(catalog)
        .unwrap();

    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        says(&output, OUT_OF_DATE),
        "the band says so before Apply is pressed"
    );
    assert!(disabled(&output, "Apply"), "Apply cannot commit them");
    assert_eq!(
        shown(&app).executable_view_policy,
        ["schematic", "spice"],
        "the edits stay until Revert"
    );

    let asked = click(&ctx, &mut app, &output, Role::Button, |name| {
        name == "Revert"
    });
    let output = click_lowest(&ctx, &mut app, &asked, "Revert");
    assert!(!says(&output, OUT_OF_DATE));
    assert_eq!(shown(&app).executable_view_policy, ["spice", "schematic"]);
}

/// Every name, value and description one frame announced.
fn announced(output: &egui::FullOutput) -> Vec<String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .flat_map(|(_, node)| {
            [node.label(), node.value(), node.description()]
                .into_iter()
                .flatten()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
}

/// A model library with one section per process corner.
fn sectioned_models() -> Vec<u8> {
    let mut text = String::new();
    for process in ["TT", "SS", "FF", "SF", "FS"] {
        text.push_str(&format!(
            ".lib {process}\n.model nch nmos level=1\n.model pch pmos level=1\n.endl {process}\n"
        ));
    }
    text.into_bytes()
}

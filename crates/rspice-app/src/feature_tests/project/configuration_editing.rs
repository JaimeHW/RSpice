//! Editing a configuration on the Configuration page through whole
//! application frames: what the inspector inherits and keeps, which entries
//! are refused where they are made, and how the tables state a binding's
//! list.

#![cfg(not(target_arch = "wasm32"))]

use egui::{Context, Modifiers, Pos2, pos2};

use super::recovery_frames::{app_frame, offscreen};
use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference;
use rspice_app_project::workbench::project::configuration_state::{
    ConfigurationRow, ConfigurationTab, instance_binding, set_cell_view,
};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::state::{ConfigurationCellBinding, ConfigurationPlatform};
use rspice_app_state::workbench::workspace::Workspace;

/// The reference project on Project > Configuration, the configuration it
/// runs with in front.
fn page_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Configuration;
    app
}

/// One announced control: its role, name, value and centre.
struct Node {
    role: egui::accesskit::Role,
    label: String,
    value: String,
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
    role: egui::accesskit::Role,
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
    use egui::accesskit::Role;
    let open = click(ctx, app, output, Role::ComboBox, |name| name == label);
    click(ctx, app, &open, Role::ListBoxOption, |name| name == option)
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
    select_row(&mut app, ConfigurationRow::Instance("/X1".to_owned()));
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        !nodes(&output)
            .iter()
            .any(|node| node.label == "Remove schematic from View list"),
        "the global view list's only view cannot be removed"
    );
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
            .any(|node| node.label.contains(&refusal)),
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

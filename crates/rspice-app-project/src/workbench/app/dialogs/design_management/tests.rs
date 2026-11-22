//! Design management tests.

use super::*;
use crate::state::{
    CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDirection, CrossSheetPortEndpoint,
    CrossSheetSignalType, Point,
};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::tests::harness;

#[test]
fn probe_ids_participate_in_sheet_governance_and_selected_authority() {
    let mut schematic = crate::state::SchematicState::default();
    schematic.document_mut_for_test().probes.push(
        crate::state::SchematicProbe::new(
            81,
            crate::state::Point::new(10, 20),
            "V(out)",
            Some("V(out)".to_owned()),
        )
        .unwrap(),
    );
    schematic.session.editor.selection.select_only_probe(81);

    assert_eq!(all_stable_object_ids(&schematic), vec![81]);
    assert_eq!(selected_stable_object_ids(&schematic), vec![81]);
}

/// The Sheets table counts each sheet's ports and the nets leaving through
/// them separately: a net reaching two other sheets is two ports but one
/// net, and a second net through a third port is a second net.
#[test]
fn the_sheets_table_counts_ports_and_off_sheet_nets_apart() {
    let [bias, afe, output] = [SheetId::new(), SheetId::new(), SheetId::new()];
    let port = |net: &str, wire: u64, from: SheetId, to: SheetId| CrossSheetPortDefinition {
        net_name: net.to_owned(),
        first: CrossSheetPortEndpoint {
            sheet_id: from,
            anchor: CrossSheetPortAnchor::WirePoint {
                wire_id: wire,
                point: Point::origin(),
            },
        },
        second: CrossSheetPortEndpoint {
            sheet_id: to,
            anchor: CrossSheetPortAnchor::WirePoint {
                wire_id: wire + 100,
                point: Point::origin(),
            },
        },
        direction: CrossSheetPortDirection::Output,
        signal_type: CrossSheetSignalType::Analog,
        discipline: CrossSheetDiscipline::Electrical,
    };
    let ports = [
        port("VREF", 1, bias, afe),
        port("vref", 2, bias, output),
        port("BIAS", 3, bias, afe),
    ];

    assert_eq!(super::manager::sheet_crossings(&ports, bias), (3, 2));
    assert_eq!(super::manager::sheet_crossings(&ports, afe), (2, 2));
    assert_eq!(super::manager::sheet_crossings(&ports, output), (1, 1));
    assert_eq!(
        super::manager::sheet_crossings(&ports, SheetId::new()),
        (0, 0)
    );
}

/// Each audited occurrence lists the instances it places directly: not
/// itself, not the instances they place, and not an instance under another
/// whose name it begins.
#[test]
fn each_audited_occurrence_lists_the_instances_it_places() {
    let subject = |path: &str| HierarchyAuditSubject {
        instance_path: path.to_owned(),
        cell_name: "amp".to_owned(),
        design_view: "schematic".to_owned(),
        declared_fallbacks: Vec::new(),
        resolved_simulation_view: Some("schematic".to_owned()),
        fallback_used: None,
        child_instance_paths: Vec::new(),
        protected_boundary_id: None,
    };
    let mut subjects = [
        "/",
        "/X1",
        "/X1/X1",
        "/X10",
        "/X1/X2",
        "/X10/X1",
        "/X1/X1/R1",
    ]
    .map(subject)
    .to_vec();
    super::operations::list_children(&mut subjects);
    let children = |path: &str| {
        subjects
            .iter()
            .find(|subject| subject.instance_path == path)
            .map(|subject| subject.child_instance_paths.clone())
            .unwrap()
    };
    assert_eq!(children("/"), ["/X1", "/X10"]);
    assert_eq!(children("/X1"), ["/X1/X1", "/X1/X2"]);
    assert_eq!(children("/X10"), ["/X10/X1"]);
    assert_eq!(children("/X1/X1"), ["/X1/X1/R1"]);
    assert!(children("/X1/X2").is_empty());
    assert!(children("/X1/X1/R1").is_empty());

    let mut below = ["/X1/X1", "/X1/X1/R1"].map(subject).to_vec();
    super::operations::list_children(&mut below);
    assert_eq!(below[0].child_instance_paths, ["/X1/X1/R1"]);
}

/// Design management over the project feature's harness, opened on a fresh
/// project's schematic.
fn opened() -> FeatureHarness<ProjectFeature> {
    let mut harness = harness();
    harness.open_project_session();
    open_design_management_dialog(
        &mut harness.state.design_management,
        &mut harness.model,
        &harness.platform,
        &mut harness.wb,
    );
    assert!(harness.state.design_management.open, "the dialog opens");
    harness
}

/// One frame of the dialog with `events`, and the accessibility tree it
/// published.
fn frame(
    ctx: &Context,
    harness: &mut FeatureHarness<ProjectFeature>,
    events: Vec<egui::Event>,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_280.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| ProjectFeature::render_design_management_dialog(&mut harness.cx(), ctx),
    );
    output
        .platform_output
        .accesskit_update
        .map(|update| update.nodes)
        .unwrap_or_default()
}

fn themed_context() -> Context {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

fn node<'a>(
    nodes: &'a [(egui::accesskit::NodeId, egui::accesskit::Node)],
    role: egui::accesskit::Role,
    label: &str,
) -> &'a (egui::accesskit::NodeId, egui::accesskit::Node) {
    nodes
        .iter()
        .find(|(_, node)| node.role() == role && node.label() == Some(label))
        .unwrap_or_else(|| panic!("no {role:?} labelled {label:?}"))
}

fn says(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)], text: &str) -> bool {
    nodes.iter().any(|(_, node)| {
        node.label().is_some_and(|label| label.contains(text))
            || node.value().is_some_and(|value| value.contains(text))
    })
}

fn click(id: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::Click,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: id,
        data: None,
    })
}

/// A new sheet and a new variant open with an empty name, not sample text
/// that Create would use as the name. Create waits, with the footer saying
/// for what and nothing marked failed, until a name is typed; then it makes
/// the sheet or variant under the typed name.
#[test]
fn a_new_sheet_or_variant_opens_unnamed_and_create_waits_for_a_name() {
    for (page, field, primary, waiting, typed) in [
        (
            DesignManagementPage::NewSheet,
            "Sheet name",
            "Create sheet",
            "Type a name for the new sheet.",
            "Bias",
        ),
        (
            DesignManagementPage::NewVariant,
            "Name",
            "Create variant",
            "Type a name for the new variant.",
            "Fitted 85C",
        ),
    ] {
        let ctx = themed_context();
        let mut harness = opened();
        harness
            .state
            .design_management
            .reset_inputs_for_page(page, None, None);
        let mut nodes = Vec::new();
        for _ in 0..3 {
            nodes = frame(&ctx, &mut harness, Vec::new());
        }
        let (_, name) = node(&nodes, egui::accesskit::Role::TextInput, field);
        assert_eq!(name.value().unwrap_or_default(), "", "{field} opens filled");
        assert!(!says(&nodes, "Power and references"));
        assert!(!says(&nodes, "Automotive high-temperature"));
        let (_, create) = node(&nodes, egui::accesskit::Role::Button, primary);
        assert!(create.is_disabled(), "{primary} is offered with no name");
        assert!(
            says(&nodes, waiting),
            "the footer does not say what it waits for"
        );
        assert!(
            !says(&nodes, "cannot continue"),
            "an unnamed page is shown failing"
        );

        let _ = frame(
            &ctx,
            &mut harness,
            vec![egui::Event::Text(typed.to_owned())],
        );
        let nodes = frame(&ctx, &mut harness, Vec::new());
        let (create_id, create) = node(&nodes, egui::accesskit::Role::Button, primary);
        assert!(
            !create.is_disabled(),
            "{primary} waits after a name is typed"
        );
        let _ = frame(&ctx, &mut harness, vec![click(*create_id)]);

        let dialog = &harness.state.design_management;
        assert_eq!(dialog.page, DesignManagementPage::Manager);
        let draft = dialog.draft.as_ref().unwrap();
        let created = match page {
            DesignManagementPage::NewSheet => draft
                .sheet_catalog(&dialog.owner_key)
                .unwrap()
                .sheets()
                .iter()
                .map(|sheet| sheet.name().to_owned())
                .collect::<Vec<_>>(),
            _ => draft
                .variants()
                .variants()
                .iter()
                .map(|variant| variant.name().to_owned())
                .collect(),
        };
        assert!(
            created.iter().any(|name| name == typed),
            "{typed} is not among {created:?}"
        );
    }
}

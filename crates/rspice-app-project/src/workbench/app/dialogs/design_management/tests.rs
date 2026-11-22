//! Design management tests.

use super::*;
use crate::state::{
    CellViewRef, CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDirection,
    CrossSheetPortEndpoint, CrossSheetSignalType, Point,
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

/// Forty resistors, twenty on each of two sheets, every one named so that
/// renumbering changes it, with Design management open over them.
fn forty_parts_on_two_sheets() -> FeatureHarness<ProjectFeature> {
    let mut harness = harness();
    harness.open_project_session();
    let document = harness.model.schematic.document_mut_for_test();
    for id in 1..=40_u64 {
        let place = i32::try_from((id - 1) % 20).unwrap();
        document.components.push(
            crate::state::Component::new(
                id,
                crate::state::ComponentType::Resistor,
                Point::new(place % 5 * 100, place / 5 * 200),
            )
            .with_name_value(format!("R{}", 100 + id), "1k"),
        );
    }
    open_design_management_dialog(
        &mut harness.state.design_management,
        &mut harness.model,
        &harness.platform,
        &mut harness.wb,
    );
    let dialog = &mut harness.state.design_management;
    let owner = dialog.owner_key.clone();
    let sheets = dialog
        .draft
        .as_mut()
        .unwrap()
        .sheet_catalog_mut(&owner)
        .unwrap();
    let first = sheets.sheets()[0].id();
    let second = sheets
        .create_sheet_with_page_format(
            SheetDefinition {
                name: "Output".to_owned(),
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: None,
            },
            Some(first),
            SchematicSheetFormat::default(),
        )
        .unwrap();
    let revision = sheets.revision();
    sheets.assign_objects(revision, second, 21..=40).unwrap();
    dialog.reset_inputs_for_page(DesignManagementPage::RenumberPreview, None, None);
    harness
}

/// What the Renumber list says of each part it lists, by the part's name:
/// its row's reading, "R101 \u{2192} R1" or "R101 kept". The list scrolls,
/// so it is read the way a reader reads it, scrolling to its end.
fn read_renumber_list(
    ctx: &Context,
    harness: &mut FeatureHarness<ProjectFeature>,
) -> BTreeMap<String, String> {
    let mut rows = BTreeMap::new();
    let mut events = Vec::new();
    for _ in 0..60 {
        let nodes = frame(ctx, harness, std::mem::take(&mut events));
        let mut bottom = None;
        for (_, node) in &nodes {
            // A row is a label, whose words egui gives as its value.
            let Some(label) = node.value() else { continue };
            let Some((name, _)) = label
                .split_once(" \u{2192} ")
                .or_else(|| label.split_once(" kept"))
            else {
                continue;
            };
            if node.role() == egui::accesskit::Role::Label {
                rows.insert(name.to_owned(), label.to_owned());
                bottom = node.bounds().map(|bounds| {
                    egui::pos2(
                        ((bounds.x0 + bounds.x1) / 2.0) as f32,
                        ((bounds.y0 + bounds.y1) / 2.0) as f32,
                    )
                });
            }
        }
        if rows.len() == 40 {
            break;
        }
        if let Some(pointer) = bottom {
            events = vec![
                egui::Event::PointerMoved(pointer),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -120.0),
                    phase: egui::TouchPhase::Move,
                    modifiers: egui::Modifiers::NONE,
                },
            ];
        }
    }
    rows
}

/// Scroll the Renumber list back to its first row, with the pointer still
/// over it from reading it.
fn scroll_to_top(ctx: &Context, harness: &mut FeatureHarness<ProjectFeature>) {
    for _ in 0..10 {
        let _ = frame(
            ctx,
            harness,
            vec![egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Page,
                delta: egui::vec2(0.0, 10.0),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::NONE,
            }],
        );
    }
}

/// The number in a reference designator such as R17.
fn number(name: &str) -> u32 {
    name.trim_start_matches(char::is_alphabetic)
        .parse()
        .unwrap()
}

/// Renumbering forty parts on two sheets lists every part's new name before
/// anything changes, numbers the first sheet before the second, and leaves
/// the three parts whose Keep tick is set with the names they had. What the
/// list showed is what the parts are called once the change is applied.
#[test]
fn renumber_lists_every_new_name_first_and_keeps_ticked_parts() {
    let ctx = themed_context();
    let mut harness = forty_parts_on_two_sheets();

    let before = read_renumber_list(&ctx, &mut harness);
    assert_eq!(before.len(), 40, "the list does not show every part");
    let new_name = |label: &str| label.split_once(" \u{2192} ").unwrap().1.to_owned();
    let mut first_sheet = Vec::new();
    let mut second_sheet = Vec::new();
    for id in 1..=40_u64 {
        let label = &before[&format!("R{}", 100 + id)];
        let target = if id <= 20 {
            &mut first_sheet
        } else {
            &mut second_sheet
        };
        target.push(number(&new_name(label)));
    }
    assert!(
        first_sheet.iter().max() < second_sheet.iter().min(),
        "the second sheet is numbered before the first: {first_sheet:?} {second_sheet:?}"
    );

    // Tick Keep on the parts that would become R1, R2 and R3, at the top of
    // the list.
    let kept = before
        .iter()
        .filter(|(_, label)| ["R1", "R2", "R3"].contains(&new_name(label).as_str()))
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    assert_eq!(kept.len(), 3);
    scroll_to_top(&ctx, &mut harness);
    for name in &kept {
        let nodes = frame(&ctx, &mut harness, Vec::new());
        let (tick, _) = node(
            &nodes,
            egui::accesskit::Role::CheckBox,
            &format!("Keep {name}"),
        );
        let _ = frame(&ctx, &mut harness, vec![click(*tick)]);
    }
    scroll_to_top(&ctx, &mut harness);
    let shown = read_renumber_list(&ctx, &mut harness);
    assert_eq!(shown.len(), 40, "the list does not show every part");
    for name in &kept {
        assert_eq!(shown[name], format!("{name} kept"));
    }
    let promised = shown
        .iter()
        .filter(|(name, _)| !kept.contains(name))
        .map(|(name, label)| (name.clone(), new_name(label)))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(promised.len(), 37);

    let nodes = frame(&ctx, &mut harness, Vec::new());
    let (renumber, button) = node(&nodes, egui::accesskit::Role::Button, "Renumber");
    assert!(!button.is_disabled());
    let _ = frame(&ctx, &mut harness, vec![click(*renumber)]);
    assert_eq!(
        harness.state.design_management.page,
        DesignManagementPage::Manager
    );
    let nodes = frame(&ctx, &mut harness, Vec::new());
    assert!(
        says(&nodes, "37 names changed"),
        "the count includes kept parts"
    );
    let (apply, _) = node(&nodes, egui::accesskit::Role::Button, MANAGER_PRIMARY);
    let _ = frame(&ctx, &mut harness, vec![click(*apply)]);

    let names = harness
        .model
        .schematic
        .document()
        .components
        .iter()
        .map(|component| (format!("R{}", 100 + component.id), component.name.clone()))
        .collect::<BTreeMap<_, _>>();
    for name in &kept {
        assert_eq!(&names[name], name, "a kept part was renamed");
    }
    for (name, new) in &promised {
        assert_eq!(&names[name], new, "{name} is not called what the list said");
    }
}

/// A part renumbered to the name it already has is not counted as a
/// change: the list's count and the note after Renumber leave it out.
#[test]
fn a_part_that_keeps_its_name_is_not_counted_as_a_change() {
    let ctx = themed_context();
    let mut harness = harness();
    harness.open_project_session();
    for (id, name, x) in [(1, "R1", 0), (2, "R7", 100)] {
        harness
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(
                crate::state::Component::new(
                    id,
                    crate::state::ComponentType::Resistor,
                    Point::new(x, 0),
                )
                .with_name_value(name, "1k"),
            );
    }
    open_design_management_dialog(
        &mut harness.state.design_management,
        &mut harness.model,
        &harness.platform,
        &mut harness.wb,
    );
    harness.state.design_management.reset_inputs_for_page(
        DesignManagementPage::RenumberPreview,
        None,
        None,
    );
    let mut nodes = Vec::new();
    for _ in 0..3 {
        nodes = frame(&ctx, &mut harness, Vec::new());
    }
    assert!(says(&nodes, "R1 \u{2192} R1"));
    assert!(says(&nodes, "R7 \u{2192} R2"));
    assert!(says(&nodes, "1 change \u{00b7} 2 parts"));
    let (renumber, _) = node(&nodes, egui::accesskit::Role::Button, "Renumber");
    let _ = frame(&ctx, &mut harness, vec![click(*renumber)]);
    let nodes = frame(&ctx, &mut harness, Vec::new());
    assert!(
        says(&nodes, "1 name changed"),
        "a part that keeps its name is counted"
    );
}

/// The Annotation tab's Collision column names what collides with each
/// reserved range: names in it that parts outside its scope hold, and the
/// ranges that reserve the same names. Used counts the names in it that
/// parts in its scope hold.
#[test]
fn the_reserved_ranges_table_shows_real_collisions() {
    let ctx = themed_context();
    let mut harness = forty_parts_on_two_sheets();
    let dialog = &mut harness.state.design_management;
    dialog.reset_inputs_for_page(DesignManagementPage::Manager, None, None);
    dialog.active_tab = DesignManagementTab::Annotation;
    let owner = dialog.owner_key.clone();
    let draft = dialog.draft.as_mut().unwrap();
    let output = draft
        .sheet_catalog(&owner)
        .unwrap()
        .sheets()
        .iter()
        .find(|sheet| sheet.name() == "Output")
        .unwrap()
        .id();
    let range = |scope, first, last| AnnotationReservedRange {
        scope,
        prefixes: vec!["R".to_owned()],
        first,
        last,
    };
    let revision = draft.annotation().policy().revision();
    let mut definition = draft.annotation().policy().definition().clone();
    definition.reserved_ranges = vec![
        range(AnnotationRangeScope::Project, 1, 150),
        range(AnnotationRangeScope::Sheet { sheet_id: output }, 100, 199),
    ];
    draft
        .annotation_mut()
        .update_policy(revision, definition)
        .unwrap();

    let mut nodes = Vec::new();
    for _ in 0..3 {
        nodes = frame(&ctx, &mut harness, Vec::new());
    }
    // R101 to R120 sit on the first sheet, inside the range Output keeps.
    assert!(
        says(
            &nodes,
            "Output, R, 100\u{2026}199, 20, R101, R102, R103 and 17 more; overlaps / 1\u{2026}150"
        ),
        "the Output range's row does not show its collisions"
    );
    assert!(
        says(
            &nodes,
            "/, R, 1\u{2026}150, 40, overlaps Output 100\u{2026}199"
        ),
        "the project range's row does not show its overlap"
    );
}

/// `R1`, `R2` and `R3` on the open schematic, and two variants with
/// Design management open over them: Industrial leaves `R1` off, and
/// Automotive, derived from it, puts a qualified resistor of 2k in place of
/// `R2` and gives `R3` the value 4k7.
fn two_variants_over_three_parts() -> (
    FeatureHarness<ProjectFeature>,
    AssemblyVariantId,
    AssemblyVariantId,
) {
    use crate::state::{ComponentSubstitution, VariantObjectOverride, VariantQualificationState};
    let mut harness = harness();
    harness.open_project_session();
    let document = harness.model.schematic.document_mut_for_test();
    for id in 1..=3_u64 {
        document.components.push(
            crate::state::Component::new(
                id,
                crate::state::ComponentType::Resistor,
                Point::new(i32::try_from(id).unwrap() * 100, 0),
            )
            .with_name_value(format!("R{id}"), "1k"),
        );
    }
    open_design_management_dialog(
        &mut harness.state.design_management,
        &mut harness.model,
        &harness.platform,
        &mut harness.wb,
    );
    let dialog = &mut harness.state.design_management;
    let owner = dialog.owner_key.clone();
    let key = |id| SchematicObjectKey::new(&owner, id).unwrap();
    let variants = dialog.draft.as_mut().unwrap().variants_mut();
    let industrial = variants
        .create(AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::from([(
                key(1),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: String::new(),
                },
            )]),
        })
        .unwrap();
    let automotive = variants
        .create(AssemblyVariantDraft {
            name: "Automotive".to_owned(),
            parent_id: Some(industrial),
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::from([
                (
                    key(2),
                    VariantObjectOverride::Substitute {
                        replacement: ComponentSubstitution {
                            library: "qualified".to_owned(),
                            cell: "res_aec".to_owned(),
                            view: "symbol".to_owned(),
                            value_override: Some("2k".to_owned()),
                            model_section: None,
                            port_equivalence_digest: None,
                            qualification: VariantQualificationState::Current,
                        },
                    },
                ),
                (
                    key(3),
                    VariantObjectOverride::Fitted {
                        value_override: Some("4k7".to_owned()),
                    },
                ),
            ]),
        })
        .unwrap();
    (harness, industrial, automotive)
}

/// The Substitution matrix shows what each variant does to each part it
/// changes, inherited changes marked, and only shows it: its one action is
/// Close, which goes back to the Variants tab with nothing changed.
#[test]
fn the_substitution_matrix_reads_each_variants_changes_and_only_closes() {
    let ctx = themed_context();
    let (mut harness, _, _) = two_variants_over_three_parts();
    let dialog = &mut harness.state.design_management;
    dialog.active_tab = DesignManagementTab::Variants;
    dialog.reset_inputs_for_page(DesignManagementPage::VariantMatrix, None, None);
    let before = dialog.draft.clone();
    let mut nodes = Vec::new();
    for _ in 0..3 {
        nodes = frame(&ctx, &mut harness, Vec::new());
    }
    let cell = CellViewRef::from_key(&harness.state.design_management.owner_key)
        .unwrap()
        .cell;
    for row in [
        format!("R1, {cell}, DNP, DNP (inherited)"),
        format!("R2, {cell}, , qualified/res_aec \u{00b7} 2k"),
        format!("R3, {cell}, , Value 4k7"),
    ] {
        assert!(says(&nodes, &row), "the matrix has no row {row:?}");
    }
    assert!(says(&nodes, "3 parts \u{00b7} 2 variants"));
    assert!(!says(&nodes, "Apply reviewed substitutions"));
    assert!(!says(&nodes, "Missing replacement"));
    assert!(
        !nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Cancel")
        }),
        "a read-out offers Cancel"
    );
    let (close, _) = node(&nodes, egui::accesskit::Role::Button, "Close");
    let _ = frame(&ctx, &mut harness, vec![click(*close)]);
    let dialog = &harness.state.design_management;
    assert_eq!(dialog.page, DesignManagementPage::Manager);
    assert_eq!(dialog.active_tab, DesignManagementTab::Variants);
    assert_eq!(dialog.draft, before, "closing the matrix changed the draft");
}

/// Compare lists each part the Reference and the Comparison treat
/// differently, with what each does to it, the base design first; and a
/// double-click on a part shows it in Design, seen as the Comparison.
#[test]
fn compare_lists_each_difference_and_a_double_click_shows_the_part() {
    let ctx = themed_context();
    let (mut harness, _, automotive) = two_variants_over_three_parts();
    // What Compare reads is what the project has, so nothing is pending
    // and the part can be shown.
    let draft = harness.state.design_management.draft.clone().unwrap();
    harness
        .model
        .workspace
        .content
        .replace_design_management(draft.clone())
        .unwrap();
    let dialog = &mut harness.state.design_management;
    dialog.original = Some(draft);
    dialog.active_tab = DesignManagementTab::Variants;
    dialog.reset_inputs_for_page(DesignManagementPage::CompareVariants, None, None);
    assert_eq!(
        dialog.inputs.compare_reference, None,
        "Compare does not open on the base design"
    );
    dialog.inputs.compare_target = Some(automotive);
    let mut nodes = Vec::new();
    for _ in 0..3 {
        nodes = frame(&ctx, &mut harness, Vec::new());
    }
    let cell = CellViewRef::from_key(&harness.state.design_management.owner_key)
        .unwrap()
        .cell;
    for row in [
        format!("R1, {cell}, Fitted, Do not populate"),
        format!("R2, {cell}, Fitted, qualified/res_aec \u{00b7} 2k"),
        format!("R3, {cell}, Fitted, Value 4k7"),
    ] {
        assert!(says(&nodes, &row), "Compare has no row {row:?}");
    }
    assert!(says(&nodes, "3 differences"));
    assert!(!says(&nodes, "Open comparison"));

    let (_, r2) = nodes
        .iter()
        .find(|(_, node)| {
            let row = format!("R2, {cell}");
            node.label().is_some_and(|label| label.starts_with(&row))
                || node.value().is_some_and(|value| value.starts_with(&row))
        })
        .expect("R2's row");
    let bounds = r2.bounds().expect("the row is laid out");
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let _ = frame(&ctx, &mut harness, vec![egui::Event::PointerMoved(at)]);
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let _ = frame(
        &ctx,
        &mut harness,
        vec![button(true), button(false), button(true), button(false)],
    );

    assert!(
        !harness.state.design_management.open,
        "the dialog stays open"
    );
    assert_eq!(
        harness
            .model
            .chosen_assembly_variant()
            .map(|variant| variant.id()),
        Some(automotive),
        "the design is not seen as the Comparison"
    );
    let shown = harness
        .wb
        .outbox
        .take::<crate::workbench::framework::requests::ShowOnSchematic>()
        .expect("Design is not asked to show the part");
    assert_eq!(
        shown.point,
        Point::new(200, 0),
        "the canvas centres elsewhere"
    );
    assert!(
        shown.selection.components.contains(&2),
        "R2 is not selected on arrival"
    );
}

/// The Variants tab says which variant is active, counts each variant's
/// substitutes, values and DNP parts, and offers the base design as the
/// active choice beside the variants.
#[test]
fn the_variants_tab_marks_the_active_variant_and_offers_the_base_design() {
    let ctx = themed_context();
    let (mut harness, _, _) = two_variants_over_three_parts();
    let dialog = &mut harness.state.design_management;
    dialog.reset_inputs_for_page(DesignManagementPage::Manager, None, None);
    dialog.active_tab = DesignManagementTab::Variants;
    let mut nodes = Vec::new();
    for _ in 0..3 {
        nodes = frame(&ctx, &mut harness, Vec::new());
    }
    assert!(says(&nodes, "Industrial, base, 0, 0, 1, Yes"));
    assert!(says(&nodes, "Automotive, Industrial, 1, 1, 0, "));
    assert!(!says(&nodes, "Qualification"));
    let _ = node(&nodes, egui::accesskit::Role::ComboBox, "Model equivalence");

    let (active, _) = node(
        &nodes,
        egui::accesskit::Role::ComboBox,
        "Active assembly variant",
    );
    let _ = frame(&ctx, &mut harness, vec![click(*active)]);
    let nodes = frame(&ctx, &mut harness, Vec::new());
    let (base, _) = node(&nodes, egui::accesskit::Role::ListBoxOption, "Base design");
    let _ = frame(&ctx, &mut harness, vec![click(*base)]);
    let _ = frame(&ctx, &mut harness, Vec::new());
    let draft = harness.state.design_management.draft.as_ref().unwrap();
    assert_eq!(
        draft.variants().active_variant_id(),
        None,
        "the base design cannot be made active"
    );
}

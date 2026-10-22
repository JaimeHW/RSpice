//! Connectivity manager tests: the report, its repairs, and how a repair is
//! validated and revealed.

use super::*;
use crate::state::{NetLabel, NetNamingPolicy, Wire};

#[test]
fn global_alias_repairs_are_derived_from_real_label_ids() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(10, Point::new(0, 0), "VDD!"));
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(11, Point::new(1, 0), "VDD"));
    state.sync_active_schematic_to_workspace();
    let policy = ConnectivityPolicy::default();
    let projection = preview_projection(&state, &policy).expect("a design projection");
    let globals = build_global_rows(&state, &policy, &projection);
    let repairs = build_global_repairs(&state, &policy, &globals);
    assert_eq!(repairs.len(), 1);
    match repairs[0].action.as_ref().expect("action") {
        ConnectivityRepairAction::RenameLabels { canonical, labels } => {
            assert_eq!(canonical, "VDD!");
            assert_eq!(labels, &vec![(11, "VDD".to_owned())]);
        }
    }
}

#[test]
fn technology_catalog_promotes_exact_declared_aliases() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(10, Point::new(0, 0), "VCC"));
    state
        .model
        .workspace
        .content
        .connectivity
        .technology_global_nets = Some(crate::state::TechnologyGlobalNetCatalog {
        authority: "demo-pdk@1.0".to_owned(),
        nets: vec![crate::state::ConnectivityAliasGroup {
            canonical_name: "VDD".to_owned(),
            aliases: vec!["VCC".to_owned()],
        }],
    });
    state.sync_active_schematic_to_workspace();
    let policy = ConnectivityPolicy {
        global_promotion: GlobalNetPromotionPolicy::TechnologyDefinedOnly,
        ..ConnectivityPolicy::default()
    };
    validate_policy_authority(&policy, &state.model.workspace.content.connectivity).unwrap();
    let projection = preview_projection(&state, &policy).expect("a design projection");
    let globals = build_global_rows(&state, &policy, &projection);
    assert_eq!(globals.len(), 1);
    assert_eq!(globals[0].canonical, "VDD");
    assert_eq!(globals[0].state, "resolved");
    assert!(globals[0].scope.contains("demo-pdk@1.0"));
}

#[test]
fn dialect_alias_policy_uses_the_project_owned_table() {
    let mut contract = ConnectivityContract {
        dialect_aliases: Some(crate::state::DialectAliasCatalog {
            authority: "dialect-2026".to_owned(),
            groups: vec![crate::state::ConnectivityAliasGroup {
                canonical_name: "0".to_owned(),
                aliases: vec!["GND".to_owned()],
            }],
        }),
        ..ConnectivityContract::default()
    };
    contract.policy.alias_comparison = GlobalAliasComparisonPolicy::DialectCompatibility;
    validate_policy_authority(&contract.policy, &contract).unwrap();
    assert!(alias_matches(
        "0",
        "GND!",
        GlobalAliasComparisonPolicy::DialectCompatibility,
        &contract
    ));
}

#[test]
fn alias_candidate_rejects_stale_label_identity() {
    let mut state = SchematicState::default();
    state.document_mut_for_test().document_policy.net_naming = NetNamingPolicy::StrictCaseSensitive;
    state
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(3, Point::origin(), "OLD"));
    let repair = ConnectivityRepair {
        id: "global-alias:1".to_owned(),
        object: "VDD!".to_owned(),
        before: "VDD".to_owned(),
        proposed: String::new(),
        identity_effect: String::new(),
        validation: String::new(),
        action: Some(ConnectivityRepairAction::RenameLabels {
            canonical: "VDD!".to_owned(),
            labels: vec![(3, "VDD".to_owned())],
        }),
        endpoint_choices: Vec::new(),
        reveal: RevealTarget::Label {
            view_key: "user/top/schematic".to_owned(),
            id: 3,
        },
    };
    assert!(apply_repair_to_candidate(&mut state, &repair).is_err());
    assert_eq!(state.document().net_labels[0].name, "OLD");
}

#[test]
fn typed_bus_validation_preserves_declared_direction() {
    let descending = BusDeclaration::parse("DATA[7:0]").expect("valid");
    let ascending = BusDeclaration::parse("DATA[0:7]").expect("valid");
    assert_eq!(descending.direction(), BusDirection::Descending);
    assert_eq!(ascending.direction(), BusDirection::Ascending);
    assert_eq!(descending.width(), 8);
    assert_eq!(ascending.width(), 8);
}

/// Nothing is proposed: the manager opens with no declaration, so placement
/// cannot begin on a vector the user never wrote.
#[test]
fn the_bus_declaration_opens_empty_and_placement_waits_for_one() {
    let dialog = ConnectivityManagerDialogState::default();
    assert_eq!(dialog.bus_declaration, "");
    assert!(
        validate_bus_draft(&dialog).is_err(),
        "an empty declaration does not arm bus placement"
    );
}

#[test]
fn bus_draft_accepts_the_authored_declaration_and_rejects_a_malformed_one() {
    let mut dialog = ConnectivityManagerDialogState::default();
    dialog.bus_declaration = "DATA[0:7]".to_owned();
    let declaration = validate_bus_draft(&dialog).expect("the authored declaration parses");
    assert_eq!(declaration.direction(), BusDirection::Ascending);
    assert_eq!(declaration.width(), 8);

    dialog.bus_declaration = "DATA[7".to_owned();
    assert!(validate_bus_draft(&dialog).is_err());
}

/// The line under the declaration says what placement waits for, what the
/// declaration declares, or why it is refused, and nothing true of every bus.
#[test]
fn the_declaration_line_reads_off_the_declaration() {
    assert_eq!(
        bus_declaration_line("  "),
        Ok(BUS_DECLARATION_MISSING.to_owned())
    );
    assert_eq!(
        bus_declaration_line("DATA[7:0]"),
        Ok("8 members, descending".to_owned())
    );
    assert!(bus_declaration_line("DATA[7").is_err());
}

/// Every node the manager publishes, after enough passes for the surface to
/// settle. Access updates are incremental, so the latest of each is kept.
fn rendered_nodes(mut app: RSpiceApp) -> HashMap<egui::accesskit::NodeId, egui::accesskit::Node> {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut nodes = HashMap::new();
    for _ in 0..4 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| app.render_connectivity_manager_dialog(ctx),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    nodes
}

fn node_text(node: &egui::accesskit::Node) -> String {
    format!(
        "{} {}",
        node.label().unwrap_or_default(),
        node.value().unwrap_or_default()
    )
}

/// The surface's left and right edges. The modal's own node is the whole
/// screen, so they are read off the primary button, which ends 12 pt in
/// from the right edge of a surface centred on the 1440 pt screen.
fn surface_edges(
    nodes: &HashMap<egui::accesskit::NodeId, egui::accesskit::Node>,
    primary: &str,
) -> (f64, f64) {
    let right = nodes
        .values()
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some(primary))
        .and_then(egui::accesskit::Node::bounds)
        .expect("the primary button has bounds")
        .x1
        + 12.0;
    (1440.0 - right, right)
}

/// The manager's rows start on the dialog's left edge, as its title does,
/// not flush against the frame; and no section header carries a constant
/// kicker.
#[test]
fn the_manager_rows_start_on_the_dialog_edge_without_kickers() {
    let mut app = RSpiceApp::test_instance();
    open_connectivity_manager(&mut app.state);
    let nodes = rendered_nodes(app);
    let (left, _) = surface_edges(&nodes, PRIMARY);
    let signal = nodes
        .values()
        .find(|node| node_text(node).trim() == "Signal")
        .and_then(egui::accesskit::Node::bounds)
        .expect("the Signal heading has bounds");
    assert!(
        signal.x0 - left >= 11.5,
        "the table starts {} pt in from the frame",
        signal.x0 - left
    );
    assert!(
        !nodes
            .values()
            .any(|node| node_text(node).contains("hierarchy-resolved")),
        "a header still carries the constant kicker"
    );
}

/// Creating a bus is a command form: a 420 pt card asking for the
/// declaration, with no row that says the same thing for every bus.
#[test]
fn creating_a_bus_is_a_command_form_without_constant_rows() {
    let mut app = RSpiceApp::test_instance();
    open_connectivity_manager(&mut app.state);
    app.state.design.connectivity_manager.page = ConnectivityManagerPage::CreateBus;
    let nodes = rendered_nodes(app);
    let (left, right) = surface_edges(&nodes, CREATE_BUS_PRIMARY);
    assert!(
        right - left <= 424.0,
        "the form is {} pt wide",
        right - left
    );
    assert!(nodes.values().any(|node| {
        node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Declaration")
    }));
    assert!(
        !nodes
            .values()
            .any(|node| node_text(node).contains("Discipline conversion")),
        "the constant discipline row is back"
    );
}

/// The buses tab reports the schematic's own vector geometry. Nothing in a
/// project-level catalog adds a row to it.
#[test]
fn the_bus_report_is_exactly_the_schematics_typed_buses() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(10, Point::origin(), "SENSE_P"));
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(11, Point::origin(), Point::new(20, 0)));
    state.sync_active_schematic_to_workspace();

    let report = build_report(&state, &ConnectivityPolicy::default()).expect("a resolved design");

    assert_eq!(
        report.buses.len(),
        state.model.schematic.document().buses.len()
    );
}

/// Give the workspace an active configuration that cannot resolve: its DUT
/// path names an instance the expanded hierarchy has not got. The design
/// projection refuses such a configuration, so a report produced
/// afterwards would be a report on the editor buffer.
fn unresolve_configuration(state: &mut AppState) {
    let root = state.model.workspace.content.active_view.clone();
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Unresolvable DUT".to_owned(),
            root,
            dut_path: "/XABSENT".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection consumer test".to_owned(),
        })
        .expect("the fixture configuration is well formed");
}

#[test]
fn the_report_states_an_unresolved_configuration_instead_of_buffer_connectivity() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(1, Point::origin(), Point::new(20, 0)));
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(2, Point::origin(), "VOUT"));
    state.sync_active_schematic_to_workspace();
    let resolved = build_report(&state, &ConnectivityPolicy::default())
        .expect("the fixture configuration resolves");
    assert!(
        resolved.nets.iter().any(|row| row.name == "VOUT"),
        "the fixture sheet reports its net while the configuration resolves"
    );

    unresolve_configuration(&mut state);

    let reason = build_report(&state, &ConnectivityPolicy::default())
        .expect_err("an unresolvable configuration has no report");
    assert!(
        reason.contains("XABSENT"),
        "the dialog must show the projection's own reason: {reason}"
    );
}

/// An open pin is listed on the net row it stands on, and is offered as a
/// repair here. A defect inside a placed cell is neither: it is on none of
/// this view's nets, and no wire drawn here repairs it — so it is not
/// offered once per occurrence as a row with nothing to choose and nowhere
/// to go.
#[test]
fn only_this_views_findings_are_net_issues_and_repairs() {
    let mut state = AppState::default();
    let mut work = crate::state::Library::new("work");
    let mut pad = crate::state::Cell::new("pad");
    pad.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    work.add_cell(pad);
    state.model.library_manager.add_library(work);
    let mut master = SchematicState::default();
    master.add_component(crate::state::ComponentType::Resistor, Point::new(30, 0));
    state
        .model
        .workspace
        .insert_schematic_editor("work/pad/schematic".to_owned(), master);
    for x in [400, 600] {
        let mut binding = crate::state::LibraryCellInstance::new("work", "pad", "schematic");
        binding.bind_interface(&[]);
        state
            .model
            .schematic
            .add_library_cell_component(Point::new(x, 400), binding);
    }
    let resistor = state
        .model
        .schematic
        .add_component(crate::state::ComponentType::Resistor, Point::new(30, 0));
    state.sync_active_schematic_to_workspace();

    let report = build_report(&state, &ConnectivityPolicy::default()).expect("a resolved design");

    assert!(
        report
            .drc
            .violations()
            .iter()
            .any(|finding| matches!(finding.location, DrcLocation::InsideInstance { .. })),
        "the fixture's cell carries findings of its own: {:?}",
        report.drc.violations()
    );
    let open_rows = report
        .nets
        .iter()
        .filter(|row| row.component_ids == vec![resistor])
        .collect::<Vec<_>>();
    assert_eq!(open_rows.len(), 2, "one row per open leg");
    assert!(
        open_rows
            .iter()
            .all(|row| row.issue.contains("is not connected")),
        "{open_rows:?}"
    );
    assert!(
        report.repairs.iter().all(|repair| !matches!(
            &repair.reveal,
            RevealTarget::Violation(DrcLocation::InsideInstance { .. })
        )),
        "{:?}",
        report
            .repairs
            .iter()
            .map(|repair| &repair.object)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        report
            .repairs
            .iter()
            .filter(|repair| matches!(
                &repair.reveal,
                RevealTarget::Violation(DrcLocation::Terminal { component_id, .. })
                    if *component_id == resistor
            ))
            .count(),
        2,
        "both open legs of the view's own resistor are offered"
    );
}

/// A repair is judged by the design check the report states. On a root
/// whose configuration runs `amp/fixed` in place of the placed
/// `amp/schematic`, the error drawn in `amp/schematic` is in neither the
/// design nor the repaired design, so it must not refuse the repair.
#[test]
fn a_repair_candidate_is_checked_as_the_configuration_runs_it() {
    let mut state = AppState::default();
    let mut work = crate::state::Library::new("work");
    let mut amp = crate::state::Cell::new("amp");
    for view in ["schematic", "fixed"] {
        amp.add_view(crate::state::View::new(
            view,
            crate::state::ViewType::Schematic,
        ));
    }
    work.add_cell(amp);
    state.model.library_manager.add_library(work);

    // Two sources across one pair of nodes: a loop, which is an Error.
    let mut drawn = SchematicState::default();
    let sources = [
        drawn.add_component(
            crate::state::ComponentType::VoltageSource,
            Point::new(30, 0),
        ),
        drawn.add_component(
            crate::state::ComponentType::VoltageSource,
            Point::new(90, 0),
        ),
    ];
    let terminal = |master: &SchematicState, id: u64, pin: &str| {
        master
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .and_then(|component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .find_map(|(name, point)| (name == pin).then_some(point))
            })
            .expect("a voltage source has both terminals")
    };
    for pin in ["+", "-"] {
        let from = terminal(&drawn, sources[0], pin);
        let to = terminal(&drawn, sources[1], pin);
        drawn
            .add_wire(vec![from, to])
            .expect("the two terminals are joined");
    }
    state
        .model
        .workspace
        .insert_schematic_editor("work/amp/schematic".to_owned(), drawn);
    state
        .model
        .workspace
        .insert_schematic_editor("work/amp/fixed".to_owned(), SchematicState::default());

    let mut binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[]);
    let placed = state
        .model
        .schematic
        .add_library_cell_component(Point::new(400, 400), binding);
    let dut = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == placed)
        .map(|component| format!("/{}", component.name))
        .expect("the instance is placed");
    // An alias the manager offers to repair.
    for (id, x, name) in [(10, 0, "VDD!"), (11, 1, "VDD")] {
        state
            .model
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(id, Point::new(x, 0), name));
    }
    state.sync_active_schematic_to_workspace();
    let root = state.model.workspace.content.active_view.clone();
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Fixed amplifier".to_owned(),
            root,
            dut_path: dut,
            executable_view_policy: vec!["fixed".to_owned(), "schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "connectivity manager test".to_owned(),
        })
        .expect("the fixture configuration is well formed");

    let policy = state.model.workspace.content.connectivity.policy.clone();
    let report = build_report(&state, &policy).expect("the configuration resolves");
    assert!(
        report
            .drc
            .violations()
            .iter()
            .all(|finding| finding.violation_type != DrcViolationType::SourceToSource),
        "the configured design runs amp/fixed: {:?}",
        report.drc.violations()
    );
    let repair = report
        .repairs
        .iter()
        .find(|repair| repair.action.is_some())
        .expect("the alias is offered as a repair");
    let mut candidate = state.model.schematic.clone();
    apply_repair_to_candidate(&mut candidate, repair).expect("the repair applies");
    candidate.recalculate_runtime_state();

    // The candidate as its own buffer follows the placed binding into the
    // master the configuration replaced, which is the reading that
    // refused every repair here.
    let projection = preview_projection(&state, &policy).expect("a design projection");
    let hierarchy =
        HierarchySource::from_design_projection(state.model.library_manager.catalog(), &projection);
    let drawn = rspice_design::drc::run_check_with_hierarchy(
        &candidate,
        &hierarchy,
        state.design_check_config(&state.model.workspace.content.active_view),
    );
    assert!(
        drawn
            .errors()
            .iter()
            .any(|finding| finding.violation_type == DrcViolationType::SourceToSource),
        "{:?}",
        drawn.violations()
    );

    validate_repair_candidate(&state, &candidate, &policy)
        .expect("the repair adds no finding to the design the configuration runs");
}

/// All three panes are built from one projection, so the global-net pane
/// cannot name a document the connectivity pane never resolved.
#[test]
fn every_pane_reads_one_projection_of_the_design() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(10, Point::origin(), "VDD!"));
    state.sync_active_schematic_to_workspace();
    let policy = ConnectivityPolicy::default();

    let projection = preview_projection(&state, &policy).expect("a design projection");
    let globals = build_global_rows(&state, &policy, &projection);

    assert!(!globals.is_empty(), "the authored global declares itself");
    for occurrence in globals.iter().flat_map(|row| &row.declarations) {
        assert!(
            projection
                .schematic_buffers()
                .contains_key(&occurrence.view_key),
            "{} is not a document of the projection",
            occurrence.view_key
        );
    }
}

/// Sheet 1 draws `DATA[7:0]` from (0, 0) to (200, 0). Sheet 2 draws
/// `ADDR[7:0]` down from (100, 0), a T at the same coordinates, and
/// `DATA<7:0>` well clear of both.
fn buses_on_two_sheets() -> AppState {
    use crate::state::{Bus, SheetDefinition, SheetPortPolicy, SheetTemplate};
    let declared = |text| Some(BusDeclaration::parse(text).unwrap());
    let mut state = AppState::default();
    state.model.schematic.document_mut_for_test().buses = vec![
        Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(200, 0),
            declared("DATA[7:0]"),
        )
        .unwrap(),
        Bus::segment(
            2,
            Point::new(100, 0),
            Point::new(100, 100),
            declared("ADDR[7:0]"),
        )
        .unwrap(),
        Bus::segment(
            3,
            Point::new(400, 0),
            Point::new(500, 0),
            declared("DATA<7:0>"),
        )
        .unwrap(),
    ];
    state.sync_active_schematic_to_workspace();
    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let management = &mut state.model.workspace.content.design_management;
    let first = management
        .bootstrap_for_cell_view(&key, "Sheet 1", vec![1, 2, 3])
        .expect("first sheet");
    let catalog = management.sheet_catalog_mut(&key).expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Sheet 2".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    catalog
        .assign_objects(catalog.revision(), second, [2, 3])
        .expect("Sheet 2 assignments");
    catalog.set_active(first).expect("active sheet");
    state
}

/// A cell view's sheets share one coordinate space, so a bus drawn on one
/// sheet where another sheet's bus runs touches nothing: neither row reports
/// a conflict between them. A stem declared in two notations is one deck
/// name across sheets, and is still reported.
#[test]
fn the_bus_report_pairs_touching_buses_only_on_one_sheet() {
    let state = buses_on_two_sheets();

    let report = build_report(&state, &ConnectivityPolicy::default()).expect("a resolved design");

    let status = |index: usize| report.buses[index].status.as_str();
    assert_eq!(status(0), "valid");
    assert_eq!(
        status(1),
        "valid",
        "Sheet 2's bus touches nothing on Sheet 1"
    );
    assert!(status(2).contains("two notations"), "{}", status(2));
    assert!(
        !report
            .repairs
            .iter()
            .any(|repair| repair.before.contains("Touching buses")),
        "no repair is offered for a conflict between sheets"
    );

    // The finding is located where Sheet 2 draws the bus, not where the
    // projection lays Sheet 2.
    let projection = preview_projection(&state, &ConnectivityPolicy::default()).unwrap();
    let analysis = canvas_bus_analysis(&state, projected_open_view(&state, &projection).unwrap());
    let located: Vec<_> = analysis
        .diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.bus_id, diagnostic.point))
        .collect();
    assert_eq!(located, vec![(Some(3), Point::new(400, 0))]);
}

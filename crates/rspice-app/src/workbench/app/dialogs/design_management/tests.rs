//! Design management tests.

use super::operations::variant_connectivity_difference_count;
use super::*;
use crate::state::{
    ComponentType, CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDirection,
    CrossSheetPortEndpoint, CrossSheetSignalType, Point,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::project::commands::ProjectRequest;

/// Design management's parts over `app`, as the project feature's context
/// lends them.
fn parts(app: &mut RSpiceApp) -> DesignManagementParts<'_> {
    let state = &mut app.state;
    DesignManagementParts {
        dialog: &mut state.project.design_management,
        project_page: &mut state.project.project_page,
        configuration_tab: &mut state.project.configuration.tab,
        model: &mut state.model,
        platform: &state.platform,
        wb: &mut state.workbench,
        runs: &mut state.runs,
        run_controller: &mut app.simulation_controller,
    }
}

#[test]
fn subflow_edit_and_escape_preserve_fresh_changes_and_reset_discard_confirmation() {
    for confirmed_before_edit in [false, true] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let catalog = app.state.model.workspace.content.design_management.clone();
        open_design_management_dialog(
            &mut app.state.project.design_management,
            &mut app.state.model,
            &app.state.platform,
            &mut app.state.workbench,
        );
        assert!(app.state.project.design_management.open);
        app.state.project.design_management.reset_inputs_for_page(
            DesignManagementPage::NewSheet,
            None,
            None,
        );
        let raw_input = |events| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 850.0),
            )),
            events,
            ..Default::default()
        };
        let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
            render_design_management_dialog(&mut parts(&mut app), ctx);
        });
        app.state.project.design_management.discard_confirmation = confirmed_before_edit;
        let key = |key, modifiers| egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        };
        let _ = ctx.run_ui(
            raw_input(vec![
                key(
                    egui::Key::A,
                    egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
                ),
                egui::Event::Paste("Newly edited sheet".into()),
                key(egui::Key::Escape, egui::Modifiers::NONE),
                key(egui::Key::Enter, egui::Modifiers::NONE),
            ]),
            |ctx| render_design_management_dialog(&mut parts(&mut app), ctx),
        );
        let draft = &app.state.project.design_management;
        assert_eq!(draft.page, DesignManagementPage::NewSheet);
        assert_eq!(draft.inputs.sheet_name, "Newly edited sheet");
        assert!(draft.discard_confirmation);
        assert!(draft.subflow_dirty());

        let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
            render_design_management_dialog(&mut parts(&mut app), ctx);
        });
        assert_eq!(
            app.state.project.design_management.page,
            DesignManagementPage::Manager
        );
        assert!(!app.state.project.design_management.discard_confirmation);
        assert_eq!(app.state.model.workspace.content.design_management, catalog);
    }
}

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

/// Each tab's settings are selects on one grid: one left edge, one width.
/// The manager is as wide as its subflows, so the two never resize the
/// dialog between them, and its counts read in the singular for one.
#[test]
fn the_manager_tabs_stand_their_settings_on_one_grid_at_the_subflow_width() {
    let tabs: [(DesignManagementTab, &[&str]); 4] = [
        (
            DesignManagementTab::Sheets,
            &[
                "Off-sheet connector policy",
                "Page numbering",
                "Delete behavior",
            ],
        ),
        (
            DesignManagementTab::Variants,
            &["Inheritance", "Missing replacement", "Result compatibility"],
        ),
        (
            DesignManagementTab::Annotation,
            &[
                "Reference designators",
                "Annotation scope",
                "Collision policy",
                "Backannotation",
            ],
        ),
        (
            DesignManagementTab::Hierarchy,
            &["Missing view", "Black-box policy", "Cycle detection"],
        ),
    ];
    for (tab, labels) in tabs {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        open_design_management_dialog(
            &mut app.state.project.design_management,
            &mut app.state.model,
            &app.state.platform,
            &mut app.state.workbench,
        );
        app.state.project.design_management.active_tab = tab;
        // Access updates are incremental: keep the latest of every node.
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..4 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| render_design_management_dialog(&mut parts(&mut app), ctx),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let bounds = |label: &str, role: egui::accesskit::Role| {
            nodes
                .values()
                .find(|node| node.role() == role && node.label() == Some(label))
                .and_then(egui::accesskit::Node::bounds)
                .unwrap_or_else(|| panic!("{tab:?}: {label} has bounds"))
        };
        // The surface is centred, and its primary ends 12 pt inside the
        // frame's 1 pt border.
        let primary = bounds(MANAGER_PRIMARY, egui::accesskit::Role::Button);
        let width = 2.0 * (primary.x1 + 13.0) - 1440.0;
        assert!(
            (width - 980.0).abs() < 1.0,
            "{tab:?}: the manager is {width} pt wide"
        );
        let selects = labels
            .iter()
            .map(|label| bounds(label, egui::accesskit::Role::ComboBox))
            .collect::<Vec<_>>();
        for pair in selects.windows(2) {
            assert!(
                (pair[0].x0 - pair[1].x0).abs() < 0.5 && (pair[0].x1 - pair[1].x1).abs() < 0.5,
                "{tab:?}: {selects:?} do not share one column"
            );
        }
    }
    assert_eq!(widgets::counted(1, "sheet"), "1 sheet");
    assert_eq!(widgets::counted(2, "view"), "2 views");
}

#[test]
fn manager_and_subflow_splits_match_the_mockup_contract() {
    assert_eq!(main_split_widths(760.0), (380.0, 380.0));
    assert_eq!(subflow_split_widths(980.0), (646.0, 334.0));
}

#[test]
fn connectivity_only_variant_comparison_detects_dnp_topology_change() {
    let mut state = AppState::default();
    let component = state.model.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::origin(),
    );
    let owner = state.model.workspace.content.active_view.key();
    state
        .model
        .workspace
        .insert_schematic_editor(owner.clone(), state.model.schematic.clone());
    let reference = state
        .model
        .workspace
        .content
        .design_management
        .variants_mut()
        .create(AssemblyVariantDraft {
            name: "Populated".to_owned(),
            parent_id: None,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::new(),
        })
        .expect("reference variant");
    let object = SchematicObjectKey::new(&owner, component).expect("scoped component");
    let comparison = state
        .model
        .workspace
        .content
        .design_management
        .variants_mut()
        .create(AssemblyVariantDraft {
            name: "DNP".to_owned(),
            parent_id: None,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::from([(
                object,
                crate::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "review-1".to_owned(),
                },
            )]),
        })
        .expect("comparison variant");

    assert!(
        variant_connectivity_difference_count(&state.model, reference, comparison)
            .expect("connectivity comparison")
            > 0
    );
}

/// A menu row for Variants or Annotation lands on that tab, not on Sheets.
#[test]
fn design_management_opens_on_the_tab_a_row_asks_for() {
    for tab in [
        DesignManagementTab::Variants,
        DesignManagementTab::Annotation,
    ] {
        let mut app = RSpiceApp::test_instance();
        open_design_management_on(
            &mut app.state.project.design_management,
            &mut app.state.model,
            &app.state.platform,
            &mut app.state.workbench,
            tab,
        );
        assert!(app.state.project.design_management.open);
        assert_eq!(app.state.project.design_management.active_tab, tab);
    }
    let mut app = RSpiceApp::test_instance();
    open_design_management_dialog(
        &mut app.state.project.design_management,
        &mut app.state.model,
        &app.state.platform,
        &mut app.state.workbench,
    );
    assert_eq!(
        app.state.project.design_management.active_tab,
        DesignManagementTab::Sheets
    );
}

/// Create hierarchy, which a move into a new child cell hands on, is opened
/// by the shell where Design management once opened it in place: over the
/// same complete instance-only selection, before the next frame draws.
#[test]
fn a_create_hierarchy_handed_to_the_shell_opens_where_it_opened_in_place() {
    for handed in [false, true] {
        let mut app = RSpiceApp::test_instance();
        let schematic = &mut app.state.model.schematic;
        let r1 = schematic.add_component(ComponentType::Resistor, Point::origin());
        let r2 = schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        schematic.add_component(ComponentType::Resistor, Point::new(160, 0));
        schematic.add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
        schematic.add_wire(vec![Point::new(100, 0), Point::new(140, 0)]);
        schematic.session.editor.selection.select_component(r1);
        schematic.session.editor.selection.select_component(r2);
        assert!(app.state.model.create_hierarchy_available());

        if handed {
            app.state
                .workbench
                .outbox
                .push(ProjectRequest::OpenCreateHierarchy);
            app.after_contribution();
        } else {
            crate::workbench::app::open_create_hierarchy_dialog(&mut app.state);
        }

        let dialog = &app.state.design.create_hierarchy;
        assert!(dialog.open, "handed {handed}");
        assert_eq!(app.state.prompt_text(), None, "handed {handed}");
    }
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

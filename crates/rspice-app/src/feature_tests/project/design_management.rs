//! Design management through the whole application: the manager's tabs on
//! one grid, a subflow's edits kept across escape, variant comparison, and
//! a create handed to the shell against one opened in place.

use egui::Context;
use std::collections::BTreeMap;

use crate::state::{
    AssemblyVariantDraft, ComponentType, Point, SchematicObjectKey, VariantInheritance,
    VariantQualificationPlan,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::design_management::operations::variant_connectivity_difference_count;
use crate::workbench::app::dialogs::design_management::render_design_management_dialog;
use crate::workbench::app::dialogs::design_management::widgets;
use crate::workbench::app::dialogs::design_management::{
    DesignManagementPage, DesignManagementParts, DesignManagementTab, MANAGER_PRIMARY,
    open_design_management_dialog, open_design_management_on,
};
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
        // The settings are the kit's 28 point selects, not the inspector's
        // 22 point drop-downs.
        for select in &selects {
            assert!(
                (select.y1 - select.y0 - 28.0).abs() < 0.5,
                "{tab:?}: a setting is {} pt tall",
                select.y1 - select.y0
            );
        }
        // The left pane's rows start where the title and the footer do: as
        // far in from the surface's left edge as the primary ends from its
        // right. They stood 10 points further in, inside a bordered box.
        let heading = match tab {
            DesignManagementTab::Sheets => "Multi-sheet design",
            DesignManagementTab::Variants => "Assembly variants",
            DesignManagementTab::Annotation => "Reference-designator ownership",
            DesignManagementTab::Hierarchy => "Hierarchy and view resolution",
        };
        let header = nodes
            .values()
            .filter(|node| node.role() != egui::accesskit::Role::Dialog)
            .find(|node| node.label().is_some_and(|label| label.starts_with(heading)))
            .and_then(egui::accesskit::Node::bounds)
            .unwrap_or_else(|| panic!("{tab:?}: {heading} has bounds"));
        let surface_left = primary.x1 + 13.0 - width;
        let text_left = header.x0 + f64::from(crate::workbench::design_system::PANEL_INSET);
        assert!(
            (text_left - (surface_left + 13.0)).abs() < 0.5,
            "{tab:?}: the left pane's rows start at {text_left}, the title at {}",
            surface_left + 13.0
        );
    }
    assert_eq!(widgets::counted(1, "sheet"), "1 sheet");
    assert_eq!(widgets::counted(2, "view"), "2 views");
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
            crate::workbench::app::open_create_hierarchy_dialog(
                &mut app.state.design.create_hierarchy,
                &app.state.model,
                &mut app.state.workbench,
            );
        }

        let dialog = &app.state.design.create_hierarchy;
        assert!(dialog.open, "handed {handed}");
        assert_eq!(app.state.prompt_text(), None, "handed {handed}");
    }
}

/// A manager with nothing pending is at rest: Apply waits and the footer
/// says what for, with no failure strip; and a design without variants
/// states that it has none rather than drawing an empty list.
#[test]
fn a_manager_at_rest_states_facts_not_failures() {
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
    app.state.project.design_management.active_tab = DesignManagementTab::Variants;
    // The test project has no sheet catalog yet, so opening bootstraps one
    // into the draft, which is a change to apply. A design that already has
    // one opens with nothing pending, as this one now does.
    let dialog = &mut app.state.project.design_management;
    dialog.original = dialog.draft.clone();
    assert!(!dialog.dirty(), "nothing is pending");
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
    let says = |text: &str| {
        nodes.values().any(|node| {
            node.label().is_some_and(|label| label.contains(text))
                || node.value().is_some_and(|value| value.contains(text))
        })
    };
    assert!(
        says("No reviewed design changes are pending."),
        "the footer does not say what Apply waits for"
    );
    assert!(
        !says("cannot continue"),
        "a manager at rest is shown failing"
    );
    assert!(
        nodes.values().any(|node| {
            node.label() == Some("Active assembly variant") && node.value() == Some("None")
        }),
        "the design's lack of variants is not stated"
    );
    assert!(
        !nodes.values().any(|node| {
            node.role() == egui::accesskit::Role::ComboBox
                && node.label() == Some("Active assembly variant")
        }),
        "an empty variant list is offered"
    );
}

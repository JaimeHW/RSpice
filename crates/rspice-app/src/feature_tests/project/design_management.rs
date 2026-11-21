//! Design management through the whole application: the manager's tabs on
//! one grid, a subflow's edits kept across escape, variant comparison, and
//! a create handed to the shell against one opened in place.

use egui::Context;
use std::collections::BTreeMap;

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::app::dialogs::design_management::operations::variant_connectivity_difference_count;
use rspice_app_project::workbench::app::dialogs::design_management::render_design_management_dialog;
use rspice_app_project::workbench::app::dialogs::design_management::{
    DesignManagementPage, DesignManagementParts, DesignManagementTab,
    open_design_management_dialog, open_design_management_on,
};
use rspice_app_project::workbench::project::commands::ProjectRequest;
use rspice_app_state::state::{
    AssemblyVariantDraft, ComponentType, Point, SchematicObjectKey, VariantInheritance,
    VariantQualificationPlan,
};

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

/// A hierarchy that expands past a million instances audits through the
/// dialog with every instance a subject: each lists the instances it places
/// in one pass over the design, not one pass per instance.
#[test]
fn a_hierarchy_past_a_million_instances_audits_every_instance() {
    let mut app = super::configuration_page::nested_app(4);
    let expanded = 1 + 32 + 32 * 32 + 32 * 32 * 32 + 32 * 32 * 32 * 32;
    let ctx = super::recovery_frames::offscreen();
    open_design_management_dialog(
        &mut app.state.project.design_management,
        &mut app.state.model,
        &app.state.platform,
        &mut app.state.workbench,
    );
    app.state.project.design_management.reset_inputs_for_page(
        DesignManagementPage::HierarchyAudit,
        None,
        None,
    );
    let frame = |app: &mut RSpiceApp, events| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_100.0, 850.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| render_design_management_dialog(&mut parts(app), ctx),
        )
    };
    frame(&mut app, Vec::new());
    let shown = frame(&mut app, Vec::new());
    frame(
        &mut app,
        super::recovery_frames::click_on(&shown, "Run audit"),
    );

    let dialog = &app.state.project.design_management;
    assert_eq!(dialog.page, DesignManagementPage::Manager);
    let receipt = dialog
        .draft
        .as_ref()
        .and_then(|draft| draft.hierarchy_audits().last())
        .expect("the audit is recorded");
    assert_eq!(receipt.resolved_subjects(), expanded);
    assert!(receipt.passed(), "{:?}", receipt.findings().first());
    // The manager is laid out on the frame after the press and painted on
    // the one after that.
    frame(&mut app, Vec::new());
    let text = super::recovery_frames::painted(&frame(&mut app, Vec::new()));
    assert!(
        text.contains(&format!("{expanded} subjects audited \u{b7} 0 findings")),
        "{text}"
    );
}

#[test]
fn subflow_edit_and_escape_preserve_fresh_changes_and_reset_discard_confirmation() {
    for confirmed_before_edit in [false, true] {
        let ctx = Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
fn connectivity_only_variant_comparison_detects_dnp_topology_change() {
    let mut state = AppState::default();
    let component = state.model.schematic.add_component(
        rspice_app_state::state::ComponentType::Resistor,
        rspice_app_state::state::Point::origin(),
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
                rspice_app_state::state::VariantObjectOverride::DoNotPopulate {
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
            rspice_app_design::workbench::app::dialogs::hierarchy::create::open_create_hierarchy_dialog(
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
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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

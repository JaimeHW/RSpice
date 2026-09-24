//! Design management tests.

use super::operations::variant_connectivity_difference_count;
use super::*;

#[test]
fn subflow_edit_and_escape_preserve_fresh_changes_and_reset_discard_confirmation() {
    for confirmed_before_edit in [false, true] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let catalog = app.state.workspace.design_management.clone();
        open_design_management_dialog(&mut app.state);
        assert!(app.state.dialogs.design_management.open);
        app.state.dialogs.design_management.reset_inputs_for_page(
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
            app.render_design_management_dialog(ctx);
        });
        app.state.dialogs.design_management.discard_confirmation = confirmed_before_edit;
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
            |ctx| app.render_design_management_dialog(ctx),
        );
        let draft = &app.state.dialogs.design_management;
        assert_eq!(draft.page, DesignManagementPage::NewSheet);
        assert_eq!(draft.inputs.sheet_name, "Newly edited sheet");
        assert!(draft.discard_confirmation);
        assert!(draft.subflow_dirty());

        let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
            app.render_design_management_dialog(ctx);
        });
        assert_eq!(
            app.state.dialogs.design_management.page,
            DesignManagementPage::Manager
        );
        assert!(!app.state.dialogs.design_management.discard_confirmation);
        assert_eq!(app.state.workspace.design_management, catalog);
    }
}

#[test]
fn probe_ids_participate_in_sheet_governance_and_selected_authority() {
    let mut schematic = crate::state::SchematicState::default();
    schematic.probes.push(
        crate::state::SchematicProbe::new(
            81,
            crate::state::Point::new(10, 20),
            "V(out)",
            Some("V(out)".to_owned()),
        )
        .unwrap(),
    );
    schematic.selection.select_only_probe(81);

    assert_eq!(all_stable_object_ids(&schematic), vec![81]);
    assert_eq!(selected_stable_object_ids(&schematic), vec![81]);
}

#[test]
fn manager_and_subflow_splits_match_the_mockup_contract() {
    assert_eq!(main_split_widths(760.0), (380.0, 380.0));
    assert_eq!(subflow_split_widths(980.0), (646.0, 334.0));
}

#[test]
fn connectivity_only_variant_comparison_detects_dnp_topology_change() {
    let mut state = AppState::default();
    let component = state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::origin(),
    );
    let owner = state.workspace.active_view.key();
    state
        .workspace
        .schematic_buffers
        .insert(owner.clone(), state.schematic.clone());
    let reference = state
        .workspace
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
        .workspace
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
        variant_connectivity_difference_count(&state, reference, comparison)
            .expect("connectivity comparison")
            > 0
    );
}

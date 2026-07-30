//! Design management tests.

use super::operations::variant_connectivity_difference_count;
use super::widgets::reorder_sheet_ids;
use super::*;

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
fn arrow_order_requires_each_sheet_once() {
    let mut catalog = DesignManagementCatalog::default();
    catalog
        .bootstrap_for_cell_view("work/top/schematic", "A", [1])
        .unwrap();
    let sheets = catalog.sheet_catalog_mut("work/top/schematic").unwrap();
    sheets
        .create_sheet(
            SheetDefinition {
                name: "B".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            sheets.active_sheet_id(),
        )
        .unwrap();
    let mut dialog = DesignManagementDialogState {
        owner_key: "work/top/schematic".to_owned(),
        draft: Some(catalog),
        ..DesignManagementDialogState::default()
    };
    dialog.inputs.reorder_order_text = "B → A".to_owned();
    assert_eq!(reorder_sheet_ids(&dialog).unwrap().len(), 2);
    dialog.inputs.reorder_order_text = "A → A".to_owned();
    assert!(reorder_sheet_ids(&dialog).is_err());
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

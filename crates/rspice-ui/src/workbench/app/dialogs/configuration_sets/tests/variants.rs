//! Configuration receipts follow materialized variant content and diagnostics.

use super::*;
use crate::workbench::examples::hierarchy_reference::{
    activate_variant_override, build, omit_missing_instance,
};

#[test]
fn an_omitted_missing_master_does_not_block_the_configuration_receipt() {
    let mut fixture = build();
    let receipt = |state: &AppState| {
        configuration_receipt(
            &state.workspace,
            &state.library_manager,
            &state.model_library_manager,
            &state.schematic,
            fixture.configuration,
            None,
        )
    };
    let baseline = receipt(&fixture.state);
    assert_ne!(
        baseline.netlist_digest, "generation blocked",
        "{baseline:?}"
    );
    omit_missing_instance(&mut fixture.state);
    let result = receipt(&fixture.state);
    assert_eq!(result.resolved, baseline.resolved);
    assert_ne!(result.netlist_digest, "generation blocked", "{result:?}");
    assert!(
        !result
            .bindings
            .iter()
            .any(|binding| binding.reference.cell == "omitted_master")
    );
    assert!(result.diagnostic.is_none(), "{result:?}");
}

#[test]
fn a_retained_receipt_refreshes_when_a_variant_removes_its_dut() {
    let mut fixture = build();
    let mut dialog = ConfigurationSetsDialogState {
        selected_id: Some(fixture.configuration),
        ..Default::default()
    };
    let receipt = |dialog: &mut ConfigurationSetsDialogState, state: &AppState| {
        selected_configuration_receipt(
            dialog,
            &state.workspace,
            &state.library_manager,
            &state.model_library_manager,
            &state.schematic,
        )
        .unwrap()
    };
    let baseline = receipt(&mut dialog, &fixture.state);
    assert_ne!(
        baseline.netlist_digest, "generation blocked",
        "{baseline:?}"
    );
    let dut = fixture
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.name == "X1")
        .unwrap()
        .id;
    let topology = fixture.state.schematic.topology_version();
    let object = crate::state::SchematicObjectKey::new(&fixture.top.key(), dut).unwrap();
    activate_variant_override(
        &mut fixture.state,
        object,
        crate::state::VariantObjectOverride::DoNotPopulate {
            approval_reference: "reviewed omission".to_owned(),
        },
    );
    assert_eq!(fixture.state.schematic.topology_version(), topology);
    let result = receipt(&mut dialog, &fixture.state);
    assert_eq!(result.netlist_digest, "generation blocked", "{result:?}");
    assert!(result.status.contains("unresolved"), "{result:?}");
    assert!(
        result
            .diagnostic
            .as_deref()
            .is_some_and(|text| text.contains("configured DUT path")),
        "{result:?}"
    );
    assert!(
        !result.bindings.is_empty(),
        "the unresolved circuit retains actionable rows"
    );
    assert_ne!(result.resolved, "Unavailable");
}

#[test]
fn a_retained_receipt_tracks_live_values_without_topology_counter_changes() {
    let mut fixture = build();
    let mut dialog = ConfigurationSetsDialogState {
        selected_id: Some(fixture.configuration),
        ..Default::default()
    };
    let receipt = |dialog: &mut ConfigurationSetsDialogState, state: &AppState| {
        selected_configuration_receipt(
            dialog,
            &state.workspace,
            &state.library_manager,
            &state.model_library_manager,
            &state.schematic,
        )
        .unwrap()
    };
    let baseline = receipt(&mut dialog, &fixture.state);
    assert_ne!(
        baseline.netlist_digest, "generation blocked",
        "{baseline:?}"
    );
    let topology = fixture.state.schematic.topology_version();
    fixture
        .state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.kind == crate::state::ComponentType::VoltageSource)
        .unwrap()
        .value = "2.5".to_owned();
    assert_eq!(fixture.state.schematic.topology_version(), topology);
    let changed = receipt(&mut dialog, &fixture.state);
    assert_ne!(
        changed.netlist_digest, baseline.netlist_digest,
        "live properties change the generated source"
    );
    assert_ne!(changed.netlist_digest, "generation blocked", "{changed:?}");
}

#[test]
fn a_failed_projection_has_no_invented_resolution_count() {
    let fixture = build();
    let mut invalid = fixture
        .state
        .workspace
        .configuration_sets
        .find(fixture.configuration)
        .unwrap()
        .definition()
        .clone();
    invalid.dut_path.clear();
    let receipt = configuration_receipt(
        &fixture.state.workspace,
        &fixture.state.library_manager,
        &fixture.state.model_library_manager,
        &fixture.state.schematic,
        fixture.configuration,
        Some(&invalid),
    );
    assert_eq!(receipt.resolved, "Unavailable");
    assert_eq!(receipt.status, "generation blocked");
    assert!(receipt.diagnostic.is_some());
    assert!(receipt.bindings.is_empty());
}

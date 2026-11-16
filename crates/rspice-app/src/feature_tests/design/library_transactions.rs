//! The library and cell transactions run through the whole application: copying,
//! renaming and deleting cells, libraries and views, and what each leaves outside the
//! model (the code slot's Verilog-A evidence among it).

use super::{state_with_populated_user_library, state_with_work_cell};
use crate::workbench::app_state::AppState;
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, Point, SchematicState, View,
    ViewType,
};

fn add_cell_veriloga_source(
    state: &mut AppState,
    cell: &str,
) -> rspice_app_state::state::ProjectSourceId {
    state
        .model
        .library_manager
        .get_library_mut("work")
        .and_then(|library| library.get_cell_mut(cell))
        .expect("fixture cell")
        .add_view(View::new("behavior", ViewType::VerilogA));
    let bundle = rspice_app_state::state::ProjectSourceBundle::try_new(
        rspice_app_state::state::ProjectSourceOwner::cell_view(CellViewRef::new(
            "work", cell, "behavior",
        )),
        rspice_app_state::state::ProjectSourceLanguage::VerilogA,
        "behavior.va",
        "module behavior(p, n); inout p, n; endmodule",
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("valid source bundle");
    let id = bundle.id();
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("unique source owner");
    id
}

// The bulk-findings case, and the fixture only it uses, live next to the
// check itself in `symbol_document/tests.rs`.

#[test]
fn copy_cell_flushes_live_active_schematic_before_copying_buffers() {
    let mut state = state_with_work_cell("amp");
    let active_key = CellViewRef::new("work", "amp", "schematic").key();
    state
        .model
        .workspace
        .insert_schematic_editor(active_key, SchematicState::default());
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let project_revision_before = state.model.workspace.content.project.revision().get();

    let copied = state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("copy succeeds");

    let copy = state
        .model
        .workspace
        .content
        .schematic_buffers
        .get("work/amp_copy/schematic")
        .expect("copy buffer exists");
    assert_eq!(copied, 1);
    assert_eq!(copy.document().components.len(), 1);
    assert_eq!(copy.document().components[0].kind, ComponentType::Resistor);
    assert_eq!(
        state.model.workspace.content.project.revision().get(),
        project_revision_before + 1
    );
    assert!(state.model.workspace.content.project_metadata_dirty);
}

#[test]
fn copy_and_rename_cell_keep_veriloga_source_ownership_exact() {
    let mut state = state_with_work_cell("amp");
    let original_id = add_cell_veriloga_source(&mut state, "amp");
    let configuration_id = state
        .model
        .workspace
        .content
        .configuration_sets
        .create(rspice_app_state::state::ConfigurationSetDefinition {
            name: "Release".to_owned(),
            root: CellViewRef::new("work", "amp", "schematic"),
            dut_path: "/".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: rspice_app_state::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                rspice_app_state::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: rspice_app_state::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Lifecycle test".to_owned(),
        })
        .expect("configuration root");
    let inactive_configuration_id = state
        .model
        .workspace
        .content
        .configuration_sets
        .clone_configuration(configuration_id, 1, "Characterization")
        .expect("inactive configuration root");
    let project_revision_before_copy = state.model.workspace.content.project.revision().get();

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("copy succeeds");
    let project = &state.model.workspace.content;
    assert_eq!(
        project.project.revision().get(),
        project_revision_before_copy + 1
    );
    assert!(matches!(
        project.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(rspice_app_state::state::ProjectLibraryMutation::CopyCell {
            source_library,
            source_cell,
            target_library,
            target_cell,
        }) if source_library == "work"
            && source_cell == "amp"
            && target_library == "work"
            && target_cell == "amp_copy"
    ));
    let copied_owner = rspice_app_state::state::ProjectSourceOwner::cell_view(CellViewRef::new(
        "work", "amp_copy", "behavior",
    ));
    let copied = project
        .project_sources
        .bundle_for_owner(&copied_owner)
        .expect("copied source exists");
    assert_ne!(copied.id(), original_id);
    assert_eq!(
        copied.root().content(),
        project
            .project_sources
            .get_bundle(original_id)
            .expect("original source")
            .root()
            .content()
    );

    let revision = project
        .project_sources
        .get_bundle(original_id)
        .expect("original source")
        .revision();
    let project_revision_before_rename = project.project.revision().get();
    state
        .rename_cell("work", "amp", "amp_renamed")
        .expect("rename succeeds");
    let project = &state.model.workspace.content;
    assert_eq!(
        project.project.revision().get(),
        project_revision_before_rename + 1
    );
    assert_eq!(project.project.library_mutation_audit().len(), 2);
    assert!(matches!(
        project.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(rspice_app_state::state::ProjectLibraryMutation::RenameCell {
            library,
            from_cell,
            to_cell,
        }) if library == "work" && from_cell == "amp" && to_cell == "amp_renamed"
    ));
    let renamed_owner = rspice_app_state::state::ProjectSourceOwner::cell_view(CellViewRef::new(
        "work",
        "amp_renamed",
        "behavior",
    ));
    let renamed = project
        .project_sources
        .bundle_for_owner(&renamed_owner)
        .expect("renamed source exists");
    assert_eq!(renamed.id(), original_id);
    assert!(renamed.revision() > revision);
    assert!(
        project
            .project_sources
            .bundle_for_owner(&rspice_app_state::state::ProjectSourceOwner::cell_view(
                CellViewRef::new("work", "amp", "behavior")
            ))
            .is_none()
    );
    assert!(project.project_sources_dirty);
    for id in [configuration_id, inactive_configuration_id] {
        let configuration = project
            .configuration_sets
            .find(id)
            .expect("configuration remains");
        assert_eq!(configuration.root().cell, "amp_renamed");
        assert_eq!(configuration.revision(), 2);
    }
    assert!(project.project_metadata_dirty);
}

#[test]
fn canonical_cell_collisions_reject_copy_and_rename_without_partial_mutation() {
    let mut state = state_with_work_cell("\u{c9}tage");
    let original_id = add_cell_veriloga_source(&mut state, "\u{c9}tage");
    let persisted_key = CellViewRef::new("work", "\u{c9}tage", "schematic").key();
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let before_sources = state.model.workspace.content.project_sources.clone();

    let copy_error = state
        .copy_cell("work", "\u{c9}tage", "work", "\u{e9}TAGE")
        .expect_err("accented case aliases cannot create a second cell identity");
    assert!(copy_error.contains("conflicts with an existing cell"));
    assert_eq!(
        state.model.workspace.content.project_sources,
        before_sources
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&persisted_key)
            .expect("source buffer remains")
            .document()
            .components
            .len(),
        0,
        "a rejected copy must not flush or partially copy live document state"
    );
    assert_eq!(
        state
            .model
            .library_manager
            .get_library("work")
            .expect("work library")
            .cell_count(),
        1
    );

    state
        .model
        .library_manager
        .get_library_mut("work")
        .expect("work library")
        .add_cell(Cell::new("Cible"));
    let before_sources = state.model.workspace.content.project_sources.clone();
    let before_revision = state
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(original_id)
        .expect("owned source")
        .revision();
    let rename_error = state
        .rename_cell("work", "\u{c9}tage", "cIBLE")
        .expect_err("rename cannot alias an existing canonical identity");
    assert!(rename_error.contains("conflicts with an existing cell"));
    assert_eq!(
        state.model.workspace.content.project_sources,
        before_sources
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(original_id)
            .expect("owned source remains")
            .revision(),
        before_revision
    );
    let library = state
        .model
        .library_manager
        .get_library("work")
        .expect("work library");
    assert!(library.get_cell("\u{c9}tage").is_some());
    assert!(library.get_cell("Cible").is_some());
    assert!(library.get_cell("cIBLE").is_none());
}

#[test]
fn copy_cell_regenerates_design_management_sheet_and_port_ids_without_cloning_variants() {
    let mut state = state_with_work_cell("amp");
    let main_component = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    let boundary_component = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 0));
    let moved_component = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(80, 0));
    let owner = CellViewRef::new("work", "amp", "schematic").key();
    let main_sheet = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(
            &owner,
            "Main",
            [main_component, boundary_component, moved_component],
        )
        .expect("source sheet catalog");
    let source_catalog = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&owner)
        .expect("source catalog");
    let auxiliary_sheet = source_catalog
        .create_sheet(
            rspice_app_state::state::SheetDefinition {
                name: "Auxiliary".to_owned(),
                port_policy: rspice_app_state::state::SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(main_sheet),
        )
        .expect("auxiliary sheet");
    source_catalog
        .move_selection(rspice_app_state::state::MoveSelectionRequest {
            expected_catalog_revision: source_catalog.revision(),
            object_ids: vec![moved_component],
            destination_sheet_id: auxiliary_sheet,
            boundary_resolution: rspice_app_state::state::MoveBoundaryResolution::ExplicitPorts {
                ports: vec![rspice_app_state::state::CrossSheetPortDefinition {
                    net_name: "BIAS".to_owned(),
                    first: rspice_app_state::state::CrossSheetPortEndpoint {
                        sheet_id: main_sheet,
                        anchor: rspice_app_state::state::CrossSheetPortAnchor::ComponentTerminal {
                            component_id: boundary_component,
                            terminal_name: "BIAS_OUT".to_owned(),
                        },
                    },
                    second: rspice_app_state::state::CrossSheetPortEndpoint {
                        sheet_id: auxiliary_sheet,
                        anchor: rspice_app_state::state::CrossSheetPortAnchor::ComponentTerminal {
                            component_id: moved_component,
                            terminal_name: "BIAS_IN".to_owned(),
                        },
                    },
                    direction: rspice_app_state::state::CrossSheetPortDirection::Output,
                    signal_type: rspice_app_state::state::CrossSheetSignalType::Analog,
                    discipline: rspice_app_state::state::CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .expect("reviewed cross-sheet move");
    let variant_id = state
        .model
        .workspace
        .content
        .design_management
        .variants_mut()
        .create(rspice_app_state::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: rspice_app_state::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan:
                rspice_app_state::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                rspice_app_state::state::SchematicObjectKey::new(&owner, main_component)
                    .expect("scoped source object"),
                rspice_app_state::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-42".to_owned(),
                },
            )]),
        })
        .expect("source variant");

    let source_sheet_ids = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog(&owner)
        .expect("source catalog")
        .sheets()
        .iter()
        .map(rspice_app_state::state::DesignSheet::id)
        .collect::<std::collections::BTreeSet<_>>();
    let source_port_ids = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog(&owner)
        .expect("source catalog")
        .cross_sheet_ports()
        .iter()
        .map(rspice_app_state::state::CrossSheetPortContract::id)
        .collect::<std::collections::BTreeSet<_>>();

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("cell copy succeeds");

    let copied_owner = CellViewRef::new("work", "amp_copy", "schematic").key();
    let copied_catalog = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog(&copied_owner)
        .expect("copied sheet catalog");
    let copied_sheet_ids = copied_catalog
        .sheets()
        .iter()
        .map(rspice_app_state::state::DesignSheet::id)
        .collect::<std::collections::BTreeSet<_>>();
    let copied_port_ids = copied_catalog
        .cross_sheet_ports()
        .iter()
        .map(rspice_app_state::state::CrossSheetPortContract::id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(copied_sheet_ids.len(), source_sheet_ids.len());
    assert!(copied_sheet_ids.is_disjoint(&source_sheet_ids));
    assert_eq!(copied_port_ids.len(), source_port_ids.len());
    assert!(copied_port_ids.is_disjoint(&source_port_ids));

    let resolved = state
        .model
        .workspace
        .content
        .design_management
        .variants()
        .resolve(variant_id)
        .expect("source variant remains resolvable");
    assert!(
        resolved
            .override_for(&owner, main_component)
            .expect("source key is valid")
            .is_some()
    );
    assert!(
        resolved
            .override_for(&copied_owner, main_component)
            .expect("copied key is valid")
            .is_none(),
        "copying a cell must not silently clone project assembly-variant ownership"
    );
}

#[test]
fn rename_cell_remaps_design_management_scoped_variant_and_annotation_ownership() {
    let mut state = state_with_work_cell("amp");
    let object_id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let old_owner = CellViewRef::new("work", "amp", "schematic").key();
    let sheet_id = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&old_owner, "Main", [object_id])
        .expect("source sheet catalog");
    let old_object = rspice_app_state::state::SchematicObjectKey::new(&old_owner, object_id)
        .expect("source scoped object");
    let variant_id = state
        .model
        .workspace
        .content
        .design_management
        .variants_mut()
        .create(rspice_app_state::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: rspice_app_state::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan:
                rspice_app_state::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                old_object.clone(),
                rspice_app_state::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-17".to_owned(),
                },
            )]),
        })
        .expect("source variant");
    let renumber_request = rspice_app_state::state::RenumberRequest {
        scope: rspice_app_state::state::RenumberScope::WholeProject,
        order: rspice_app_state::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references:
            rspice_app_state::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![rspice_app_state::state::AnnotationObject {
            object: old_object.clone(),
            current_reference: "R9".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: Some(sheet_id),
            hierarchy_path: "/top".to_owned(),
            position: rspice_app_state::state::AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = state
        .model
        .workspace
        .content
        .design_management
        .annotation()
        .preview_renumbering(&renumber_request)
        .expect("renumber preview");
    state
        .model
        .workspace
        .content
        .design_management
        .annotation_mut()
        .commit_renumbering(&preview, &renumber_request)
        .expect("reviewed annotation");

    state
        .rename_cell("work", "amp", "amp_rev_b")
        .expect("cell rename succeeds");

    let new_owner = CellViewRef::new("work", "amp_rev_b", "schematic").key();
    assert!(
        state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog(&old_owner)
            .is_none()
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog(&new_owner)
            .expect("renamed sheet catalog")
            .active_sheet_id(),
        Some(sheet_id),
        "renaming ownership must preserve stable sheet identity"
    );
    let resolved = state
        .model
        .workspace
        .content
        .design_management
        .variants()
        .resolve(variant_id)
        .expect("renamed variant remains resolvable");
    assert!(
        resolved
            .override_for(&old_owner, object_id)
            .expect("old key is valid")
            .is_none()
    );
    assert!(
        resolved
            .override_for(&new_owner, object_id)
            .expect("new key is valid")
            .is_some()
    );
    assert!(
        state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .effective_mapping_for(&old_owner, object_id)
            .expect("old annotation lookup")
            .is_none()
    );
    assert!(
        state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .effective_mapping_for(&new_owner, object_id)
            .expect("renamed annotation lookup")
            .is_some()
    );
}

#[test]
fn rename_cell_publishes_scoped_design_management_remaps_without_a_sheet_catalog() {
    let mut state = state_with_work_cell("amp");
    let object_id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let old_owner = CellViewRef::new("work", "amp", "schematic").key();
    let old_object = rspice_app_state::state::SchematicObjectKey::new(&old_owner, object_id)
        .expect("source scoped object");
    let designs = &mut state.model.workspace.content.design_management;
    let variant_id = designs
        .variants_mut()
        .create(rspice_app_state::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: rspice_app_state::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan:
                rspice_app_state::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                old_object.clone(),
                rspice_app_state::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-18".to_owned(),
                },
            )]),
        })
        .expect("source variant");
    let renumber_request = rspice_app_state::state::RenumberRequest {
        scope: rspice_app_state::state::RenumberScope::WholeProject,
        order: rspice_app_state::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references:
            rspice_app_state::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![rspice_app_state::state::AnnotationObject {
            object: old_object,
            current_reference: "R19".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: rspice_app_state::state::AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = designs
        .annotation()
        .preview_renumbering(&renumber_request)
        .expect("renumber preview");
    designs
        .annotation_mut()
        .commit_renumbering(&preview, &renumber_request)
        .expect("reviewed annotation");

    state
        .rename_cell("work", "amp", "amp_rev_c")
        .expect("cell rename succeeds");

    let new_owner = CellViewRef::new("work", "amp_rev_c", "schematic").key();
    let resolved = state
        .model
        .workspace
        .content
        .design_management
        .variants()
        .resolve(variant_id)
        .expect("renamed variant remains resolvable");
    assert!(
        resolved
            .override_for(&old_owner, object_id)
            .expect("old key is valid")
            .is_none()
    );
    assert!(
        resolved
            .override_for(&new_owner, object_id)
            .expect("new key is valid")
            .is_some()
    );
    assert!(
        state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .effective_mapping_for(&new_owner, object_id)
            .expect("renamed annotation lookup")
            .is_some()
    );
}

#[test]
fn rename_top_cell_then_serialize_succeeds() {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    assert_eq!(state.model.workspace.content.project.root_library, "user");
    assert_eq!(state.model.workspace.content.project.top_cell, "top");

    state
        .rename_cell("user", "top", "system")
        .expect("the project root cell is writable");

    assert_eq!(state.model.workspace.content.project.top_cell, "system");
    rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("a renamed project root cell must still serialize and validate");
}

/// A rename moves the master a document was reached *through*, not only the
/// document itself. The occurrence has to follow, or it addresses a cell the
/// libraries no longer hold — while the instance that reaches it keeps its own
/// authored name.
#[test]
fn renaming_a_cell_rewrites_the_masters_inside_every_occurrence() {
    let mut state = state_with_populated_user_library();
    let amp = CellViewRef::new("user", "amp", "schematic");
    state
        .navigation()
        .descend_into_instance(Some("XAMP".to_owned()), amp.clone());
    let path = state.model.workspace.content.occurrence_path();
    assert_eq!(path.to_string(), "/XAMP");

    state
        .rename_cell("user", "top", "tb")
        .expect("a writable cell renames");

    let workspace = &state.model.workspace.content;
    let occurrence = workspace
        .active_occurrence()
        .expect("the descended document is still open");
    assert_eq!(occurrence.root, CellViewRef::new("user", "tb", "schematic"));
    assert_eq!(occurrence.terminal_master(), &amp);
    assert_eq!(
        workspace.occurrence_path().to_string(),
        "/XAMP",
        "renaming a master never renames the instance that reaches it"
    );
}

#[test]
fn rename_library_propagation_matrix() {
    let mut state = state_with_populated_user_library();
    let project = &state.model.workspace.content;
    let source_id = project
        .project_sources
        .iter_bundles()
        .find(|bundle| {
            matches!(
                bundle.owner(),
                rspice_app_state::state::ProjectSourceOwner::CellView { .. }
            )
        })
        .expect("fixture cell-view source bundle")
        .id();

    let remapped = state
        .rename_library("user", "project_lib")
        .expect("a writable library renames");

    let project = &state.model.workspace.content;
    assert_eq!(remapped, 1, "the bound instance follows the library");
    assert!(state.model.library_manager.get_library("user").is_none());
    assert!(
        state
            .model
            .library_manager
            .get_library("project_lib")
            .is_some()
    );
    assert!(
        project
            .schematic_buffers
            .keys()
            .all(|key| key.starts_with("project_lib/")),
        "every buffer is re-keyed under the renamed library"
    );
    assert_eq!(project.active_view.library, "project_lib");
    assert!(
        project
            .open_views
            .iter()
            .all(|open| open.reference.library == "project_lib")
    );
    assert!(
        project
            .hierarchy_stack
            .iter()
            .all(|reference| reference.library == "project_lib")
    );
    assert_eq!(
        state.model.schematic.document().components[0]
            .library_cell
            .as_ref()
            .expect("instance binding")
            .library,
        "project_lib"
    );
    assert_eq!(
        project
            .project_sources
            .get_bundle(source_id)
            .expect("bundle identity is stable across a rename")
            .owner(),
        &rspice_app_state::state::ProjectSourceOwner::cell_view(CellViewRef::new(
            "project_lib",
            "amp",
            "behavior"
        ))
    );
    assert!(
        project
            .design_management
            .sheet_catalog(&CellViewRef::new("project_lib", "amp", "schematic").key())
            .is_some(),
        "sheet-catalog ownership moves with the library"
    );
    assert!(
        project
            .design_management
            .sheet_catalog(&CellViewRef::new("user", "amp", "schematic").key())
            .is_none()
    );
    assert_eq!(
        project.configuration_sets.configurations()[0]
            .root()
            .library,
        "project_lib",
        "configuration roots follow the renamed library"
    );
    assert_eq!(project.project.root_library, "project_lib");
    assert!(matches!(
        project.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(rspice_app_state::state::ProjectLibraryMutation::RenameLibrary {
            from_library,
            to_library,
        }) if from_library == "user" && to_library == "project_lib"
    ));
}

#[test]
fn delete_library_is_blocked_by_root_config_root_and_referenced_master() {
    let mut state = state_with_populated_user_library();

    let root_error = state
        .delete_library("user")
        .expect_err("the project root library cannot be deleted");
    assert!(
        root_error.contains("holds the project root cell"),
        "{root_error}"
    );

    state.model.workspace.content.project.root_library = "spare".to_owned();
    let mut spare = Library::new("spare");
    let mut consumer = Cell::new("consumer");
    consumer.add_view(View::new("schematic", ViewType::Schematic));
    spare.add_cell(consumer);
    state.model.library_manager.add_library(spare);
    let mut consumer_schematic = SchematicState::default();
    consumer_schematic.add_library_cell_component(
        Point::new(20, 20),
        LibraryCellInstance::new("user", "amp", "schematic"),
    );
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("spare", "consumer", "schematic").key(),
        consumer_schematic,
    );

    let configuration_error = state
        .delete_library("user")
        .expect_err("a configuration root cannot be left dangling");
    assert!(
        configuration_error.contains("Configuration set roots still reference"),
        "{configuration_error}"
    );

    state.model.workspace.content.configuration_sets = Default::default();
    let reference_error = state
        .delete_library("user")
        .expect_err("a loaded instance master cannot be deleted out from under it");
    assert!(
        reference_error.contains("outside library 'user' still reference a master in it"),
        "{reference_error}"
    );
    assert!(state.model.library_manager.get_library("user").is_some());
}

#[test]
fn delete_library_removes_its_cells_and_restores_valid_focus() {
    let mut state = state_with_populated_user_library();
    state.model.workspace.content.project.root_library = "spare".to_owned();
    state.model.workspace.content.project.top_cell = "keep".to_owned();
    let mut spare = Library::new("spare");
    let mut keep = Cell::new("keep");
    keep.add_view(View::new("schematic", ViewType::Schematic));
    spare.add_cell(keep);
    state.model.library_manager.add_library(spare);
    let survivor = CellViewRef::new("spare", "keep", "schematic");
    state
        .model
        .workspace
        .insert_schematic_editor(survivor.key(), SchematicState::default());
    state
        .model
        .workspace
        .content
        .open_views
        .push(rspice_app_state::state::OpenCellView::new(
            survivor.clone(),
            ViewType::Schematic,
        ));
    state.model.workspace.content.project_sources = Default::default();
    state.model.workspace.content.configuration_sets = Default::default();
    if let Some(library) = state.model.library_manager.get_library_mut("user") {
        library
            .get_cell_mut("amp")
            .expect("fixture amp cell")
            .remove_view("behavior");
    }

    let cells = state
        .delete_library("user")
        .expect("an unreferenced library is deletable");

    let project = &state.model.workspace.content;
    assert_eq!(cells, 2);
    assert!(state.model.library_manager.get_library("user").is_none());
    assert!(
        project
            .schematic_buffers
            .keys()
            .all(|key| !key.starts_with("user/"))
    );
    assert!(
        project
            .open_views
            .iter()
            .all(|open| open.reference.library != "user")
    );
    assert_eq!(project.active_view, survivor);
    assert!(state.model.workspace.active_context_schematic().is_some());
    assert!(matches!(
        project.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(rspice_app_state::state::ProjectLibraryMutation::DeleteLibrary { library }) if library == "user"
    ));
}

#[test]
fn rename_view_moves_the_buffer_source_and_view_exact_bindings() {
    let mut state = state_with_populated_user_library();

    let remapped = state
        .rename_view("user", "top", "schematic", "netlist_view")
        .expect("the configuration root view renames");
    assert_eq!(remapped, 0, "no instance binds the renamed root view");
    assert_eq!(
        state
            .model
            .workspace
            .content
            .configuration_sets
            .configurations()[0]
            .root()
            .view,
        "netlist_view",
        "configuration roots follow the renamed view"
    );

    let remapped = state
        .rename_view("user", "amp", "schematic", "netlist_view")
        .expect("a writable view renames");

    let project = &state.model.workspace.content;
    assert_eq!(
        remapped, 1,
        "only the instances bound to that exact view move"
    );
    assert!(
        state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("netlist_view"))
            .is_some()
    );
    assert!(
        project
            .schematic_buffers
            .contains_key(&CellViewRef::new("user", "amp", "netlist_view").key())
    );
    assert!(
        !project
            .schematic_buffers
            .contains_key(&CellViewRef::new("user", "amp", "schematic").key())
    );
    assert_eq!(
        state.model.schematic.document().components[0]
            .library_cell
            .as_ref()
            .expect("instance binding")
            .view,
        "netlist_view"
    );
    assert!(
        project
            .design_management
            .sheet_catalog(&CellViewRef::new("user", "amp", "netlist_view").key())
            .is_some()
    );
    assert!(matches!(
        project.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(rspice_app_state::state::ProjectLibraryMutation::RenameView {
            library,
            cell,
            from_view,
            to_view,
        }) if library == "user" && cell == "amp" && from_view == "schematic"
            && to_view == "netlist_view"
    ));

    let error = state
        .rename_view("user", "amp", "behavior", "NETLIST_VIEW")
        .expect_err("a case-folded duplicate is the same view identity");
    assert!(error.contains("conflicts with an existing view"), "{error}");
}

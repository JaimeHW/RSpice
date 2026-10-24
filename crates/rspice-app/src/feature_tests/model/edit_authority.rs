//! [`SchematicEditAuthority`] against a whole session: the authority is
//! captured from and validated against the model an `AppState` holds.

use crate::state::write_policy::LocalSafeModeOptions;
use crate::state::{ComponentType, Point};
use crate::workbench::app::{SchematicEditAuthority, schematic_editor_request_source};
use crate::workbench::app_state::AppState;

#[test]
fn authority_rejects_another_occurrence_or_sheet_of_the_same_master() {
    use crate::state::{CellViewRef, SheetDefinition, SheetPortPolicy, SheetTemplate, ViewType};
    for occurrence_change in [true, false] {
        let mut state = AppState::default();
        let parent = state.model.workspace.content.active_view.clone();
        let master = CellViewRef::new(&parent.library, "authority_child", "schematic");
        state
            .model
            .workspace
            .insert_schematic_editor(master.key(), state.model.schematic.clone());
        state
            .model
            .workspace
            .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
        let first = state
            .model
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        let catalog = state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&master.key())
            .unwrap();
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
            .unwrap();
        catalog.set_active(first).unwrap();
        let authority = SchematicEditAuthority::capture(&state);
        assert!(authority.validate(&state, "Create array").is_ok());

        if occurrence_change {
            state.model.workspace.ascend_one().unwrap();
            state
                .model
                .workspace
                .descend_into("X2".to_owned(), master, ViewType::Schematic);
        } else {
            state
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog_mut(&master.key())
                .unwrap()
                .set_active(second)
                .unwrap();
        }
        let current = schematic_editor_request_source(&state);
        assert_eq!(authority.source.document, current.document);
        assert_eq!(authority.source.design_epoch, current.design_epoch);
        assert_eq!(authority.source.document_epoch, current.document_epoch);
        assert_eq!(authority.source.topology_version, current.topology_version);
        assert!(
            authority
                .snapshot
                .is_equal_document(state.model.schematic.document())
        );
        assert!(authority.validate(&state, "Create array").is_err());
        assert!(
            authority
                .validate_presentation(&state, "Inspect selection")
                .is_err()
        );
    }
}

#[test]
fn authority_rejects_geometry_selection_and_view_drift() {
    let mut state = AppState::default();
    let id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    let authority = SchematicEditAuthority::capture(&state);
    assert!(authority.validate(&state, "Move selection").is_ok());

    state.model.schematic.document_mut_for_test().components[0].value = "2k".to_owned();
    assert!(authority.validate(&state, "Move selection").is_err());
    state.model.schematic.document_mut_for_test().components[0].value = "1k".to_owned();
    state.model.schematic.session.editor.selection.clear();
    assert!(authority.validate(&state, "Move selection").is_err());
}

#[test]
fn authority_revalidates_safe_mode_at_commit_time() {
    let mut state = AppState::default();
    let authority = SchematicEditAuthority::capture(&state);
    state.model.safe_mode.activate(
        LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );

    assert!(authority.validate(&state, "Edit schematic").is_err());
    assert!(
        authority
            .validate_presentation(&state, "Inspect schematic")
            .is_ok(),
        "presentation-only transactions remain available in safe mode"
    );
}

//! What the synchronization boundary owes the sheet catalog.
//!
//! Sheet membership and cross-sheet contracts are durable design facts that
//! live outside the drawing, so a schematic edit can destroy them without the
//! drawing remembering anything. These cases pin what comes back.

use crate::state::{
    CellViewRef, ComponentType, CrossSheetDiscipline, CrossSheetPortAnchor,
    CrossSheetPortDefinition, CrossSheetPortDirection, CrossSheetPortEndpoint,
    CrossSheetSignalType, MoveBoundaryResolution, MoveSelectionRequest, Point, SheetDefinition,
    SheetId, SheetPortPolicy, SheetTemplate,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

/// A governed two-sheet cell whose one cross-sheet contract is anchored to a
/// terminal of the component that sits on the second sheet, plus a spare part
/// on that same sheet that no contract names.
struct GovernedCrossing {
    owner: CellViewRef,
    main: SheetId,
    auxiliary: SheetId,
    stationary: u64,
    moved: u64,
    spare: u64,
}

fn governed_crossing(state: &mut AppState) -> GovernedCrossing {
    state.project_lifecycle.project_open = true;
    let owner = state.workspace.active_schematic_reference();
    let key = owner.key();
    let stationary = state
        .schematic
        .add_component(ComponentType::Diode, Point::new(0, 0));
    let moved = state
        .schematic
        .add_component(ComponentType::Diode, Point::new(200, 0));
    let spare = state
        .schematic
        .add_component(ComponentType::Diode, Point::new(400, 0));
    let stationary_wire = state
        .schematic
        .add_wire(vec![Point::new(20, 0), Point::new(60, 0)])
        .expect("a conductor reaching the stationary cathode");
    let moved_wire = state
        .schematic
        .add_wire(vec![Point::new(140, 0), Point::new(180, 0)])
        .expect("a conductor reaching the moved anode");

    let main = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(
            &key,
            "Main",
            [stationary, moved, spare, stationary_wire, moved_wire],
        )
        .expect("governed sheet catalog");
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("the catalog just bootstrapped");
    let auxiliary = catalog
        .create_sheet(
            SheetDefinition {
                name: "Auxiliary".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(main),
        )
        .expect("second sheet");
    catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![moved, spare, moved_wire],
            destination_sheet_id: auxiliary,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: "BIAS".to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: main,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: stationary,
                            terminal_name: "K".to_owned(),
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: auxiliary,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: moved,
                            terminal_name: "A".to_owned(),
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .expect("reviewed cross-sheet move");
    state.sync_active_schematic_to_workspace();
    GovernedCrossing {
        owner,
        main,
        auxiliary,
        stationary,
        moved,
        spare,
    }
}

/// Every crossing the owner declares, as the durable endpoint identity rather
/// than a count: which sheets it joins, and which terminal of which object it
/// is anchored to on each of them.
fn cross_sheet_port_definitions(
    state: &AppState,
    owner: &CellViewRef,
) -> Vec<CrossSheetPortDefinition> {
    state
        .workspace
        .design_management
        .sheet_catalog(&owner.key())
        .expect("the cell view is governed")
        .cross_sheet_ports()
        .iter()
        .map(|contract| contract.definition().clone())
        .collect()
}

fn cross_sheet_port_count(state: &AppState, owner: &CellViewRef) -> usize {
    state
        .workspace
        .design_management
        .sheet_catalog(&owner.key())
        .expect("the cell view is governed")
        .cross_sheet_ports()
        .len()
}

fn sheet_of(state: &AppState, owner: &CellViewRef, object_id: u64) -> Option<SheetId> {
    state
        .workspace
        .design_management
        .sheet_catalog(&owner.key())
        .expect("the cell view is governed")
        .sheet_for_object(object_id)
}

fn delete_component(state: &mut AppState, object_id: u64) {
    assert!(state.schematic.with_undo("delete selection", |schematic| {
        schematic
            .components
            .retain(|component| component.id != object_id);
    }));
    state.sync_active_schematic_to_workspace();
}

/// Closes C10. Deleting the object a crossing is anchored to retires the
/// contract, and nothing in the drawing remembers it — so the retirement is a
/// project transaction, and one Undo brings back the connection as well as
/// the object it named. A count cannot say that: a differently anchored
/// contract of the same net would satisfy it while joining two other points,
/// so the restored contract is compared endpoint for endpoint against the one
/// the delete retired.
#[test]
fn deleting_a_port_anchor_then_undo_restores_the_connection() {
    let mut app = RSpiceApp::test_instance();
    let crossing = governed_crossing(&mut app.state);
    let retired = cross_sheet_port_definitions(&app.state, &crossing.owner);
    assert_eq!(retired.len(), 1);

    delete_component(&mut app.state, crossing.moved);
    assert_eq!(
        cross_sheet_port_count(&app.state, &crossing.owner),
        0,
        "a contract whose anchor is gone cannot survive reconciliation"
    );

    app.action_edit_undo();

    let restored = cross_sheet_port_definitions(&app.state, &crossing.owner);
    assert_eq!(
        restored, retired,
        "undo restores the crossing that was retired — same net, same sheets, \
         same anchors — not merely a crossing"
    );
    let definition = restored.first().expect("the restored crossing");
    assert_eq!(
        definition.first.sheet_id, crossing.main,
        "the surviving end still starts on the sheet it was drawn on"
    );
    assert_eq!(
        definition.first.anchor,
        CrossSheetPortAnchor::ComponentTerminal {
            component_id: crossing.stationary,
            terminal_name: "K".to_owned(),
        },
        "and is still anchored to the terminal that names it"
    );
    assert_eq!(
        definition.second.sheet_id, crossing.auxiliary,
        "the restored end lands on the sheet the deleted object lived on"
    );
    assert_eq!(
        definition.second.anchor,
        CrossSheetPortAnchor::ComponentTerminal {
            component_id: crossing.moved,
            terminal_name: "A".to_owned(),
        },
        "anchored to the terminal of the object undo brought back"
    );
    assert!(
        app.state
            .schematic
            .components
            .iter()
            .any(|component| component.id == crossing.moved),
        "the anchor the contract names is back in the drawing"
    );
    assert_eq!(
        sheet_of(&app.state, &crossing.owner, crossing.moved),
        Some(crossing.auxiliary),
        "the restored anchor is back on its own sheet"
    );
}

/// Membership travels with the schematic step that restores an object. A part
/// undone back into existence returns to the sheet it was drawn on rather than
/// to whichever sheet the canvas happens to be showing.
#[test]
fn undoing_a_delete_returns_an_object_to_its_recorded_sheet() {
    let mut app = RSpiceApp::test_instance();
    let crossing = governed_crossing(&mut app.state);
    assert_eq!(
        sheet_of(&app.state, &crossing.owner, crossing.spare),
        Some(crossing.auxiliary)
    );

    delete_component(&mut app.state, crossing.spare);
    assert_eq!(
        sheet_of(&app.state, &crossing.owner, crossing.spare),
        None,
        "a deleted object owns no sheet"
    );
    assert_eq!(
        cross_sheet_port_count(&app.state, &crossing.owner),
        1,
        "deleting a part no contract names retires no contract"
    );

    app.action_edit_undo();

    assert_eq!(
        sheet_of(&app.state, &crossing.owner, crossing.spare),
        Some(crossing.auxiliary),
        "the restored part is back on the sheet it was drawn on"
    );
    assert_ne!(
        crossing.auxiliary, crossing.main,
        "and the active sheet was a different one"
    );
}

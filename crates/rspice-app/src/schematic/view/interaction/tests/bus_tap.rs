//! Bus-tap batch context, live geometry validation and atomic undo.

use super::*;
use crate::schematic::view::requests;

fn armed_bus_tap_state(sheets: bool) -> AppState {
    let mut state = AppState::default();
    let master = crate::state::CellViewRef::new("work", "tap_parent", "schematic");
    state
        .workspace
        .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
    let bus_id = state
        .schematic
        .add_bus(vec![Point::new(0, 0), Point::new(120, 0)], None)
        .unwrap();
    let wire_id = state
        .schematic
        .add_wire(vec![Point::new(0, 20), Point::new(120, 20)])
        .unwrap();
    state.schematic.clear_undo_history();
    if sheets {
        state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [bus_id, wire_id])
            .unwrap();
    }
    state.sync_active_schematic_to_workspace();
    let configuration = crate::state::PendingBusTap::new(
        BusDeclaration::parse("DATA[7:0]").unwrap(),
        BusSlice::parse("DATA[3]").unwrap(),
        BusTapOrientation::Automatic,
    )
    .unwrap();
    state.schematic.session.editor.pending_bus_tap = Some(
        rspice_schematic_editor::session::bus::PendingBusTapPlacement::new(
            configuration,
            requests::editor_request_source(&state),
        ),
    );
    state.schematic.arm_tool(Tool::BusTap);
    state
}

#[test]
fn bus_tap_batch_revalidates_live_geometry_and_preserves_atomic_undo() {
    for sheets in [false, true] {
        let mut state = armed_bus_tap_state(sheets);
        let source = requests::editor_request_source(&state);
        for x in [20, 60] {
            with_test_ui(|ui| handle_bus_tap_click(ui, &mut state, Point::new(x, 0), 6));
            let tap = state.schematic.document().bus_taps.last().unwrap();
            assert_eq!(tap.bus_point, Point::new(x, 0));
            assert_eq!(tap.connection_point, Point::new(x, 20));
            assert_eq!(tap.orientation, BusTapOrientation::Down);
            assert_eq!(tap.slice.to_string(), "DATA[3]");
            state.sync_active_schematic_to_workspace();
        }
        assert_eq!(state.schematic.document().bus_taps.len(), 2);
        let current = requests::editor_request_source(&state);
        assert_ne!(source.content_version, current.content_version);
        assert_ne!(source.topology_version, current.topology_version);
        if sheets {
            assert_ne!(source.sheet, current.sheet);
        }
        assert!(
            state
                .schematic
                .session
                .editor
                .pending_bus_tap
                .as_ref()
                .unwrap()
                .authority
                .matches(&current)
        );
        assert!(state.schematic.undo());
        state.sync_active_schematic_to_workspace();
        assert_eq!(state.schematic.document().bus_taps.len(), 1);
        // A live declaration conflict refuses the click without adding history.
        let bus = state.schematic.document().buses[0].clone();
        state
            .schematic
            .edit_bus_properties(&bus, Some(BusDeclaration::parse("ADDR[7:0]").unwrap()))
            .unwrap();
        let version = state.schematic.content_version();
        with_test_ui(|ui| handle_bus_tap_click(ui, &mut state, Point::new(100, 0), 6));
        assert_eq!(state.schematic.document().bus_taps.len(), 1);
        assert_eq!(state.schematic.content_version(), version);
        assert_eq!(state.schematic.session.editor.tool, Tool::BusTap);
        assert!(state.schematic.undo());
        state.sync_active_schematic_to_workspace();
        with_test_ui(|ui| handle_bus_tap_click(ui, &mut state, Point::new(100, 0), 6));
        assert_eq!(state.schematic.document().bus_taps.len(), 2);
        assert!(state.schematic.undo());
        assert!(state.schematic.undo());
        assert!(state.schematic.document().bus_taps.is_empty());
        assert!(state.schematic.document().buses[0].declaration.is_none());
        assert!(!state.schematic.can_undo());
    }
}

#[test]
fn bus_tap_click_cannot_commit_after_its_context_or_permission_changes() {
    for change in [
        "design",
        "buffer",
        "occurrence",
        "sheet",
        "read-only",
        "safe-mode",
        "missing",
        "tool",
    ] {
        let mut state = armed_bus_tap_state(true);
        match change {
            "design" => state.design_execution_epoch += 1,
            "buffer" => state.active_schematic_epoch += 1,
            "occurrence" => {
                let master = state.workspace.content.active_schematic_reference();
                state.workspace.ascend_one().unwrap();
                state
                    .workspace
                    .descend_into("X2".to_owned(), master, ViewType::Schematic);
            }
            "sheet" => {
                let master = state.workspace.content.active_schematic_reference();
                let catalog = state
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
                        catalog.active_sheet_id(),
                    )
                    .unwrap();
                catalog.set_active(second).unwrap();
            }
            "read-only" => state.schematic.session.read_only = true,
            "safe-mode" => state.workbench.safe_mode.activate(
                crate::workbench::state::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "bus-tap test".to_owned(),
            ),
            "missing" => state.schematic.session.editor.pending_bus_tap = None,
            "tool" => state.schematic.session.editor.tool = Tool::Wire,
            _ => unreachable!(),
        }
        let content = state.schematic.content_version();
        let topology = state.schematic.topology_version();
        with_test_ui(|ui| handle_bus_tap_click(ui, &mut state, Point::new(20, 0), 6));
        assert!(state.schematic.document().bus_taps.is_empty(), "{change}");
        assert!(
            state.schematic.document().buses[0].declaration.is_none(),
            "{change}"
        );
        assert_eq!(state.schematic.content_version(), content, "{change}");
        assert_eq!(state.schematic.topology_version(), topology, "{change}");
        assert!(!state.schematic.can_undo(), "{change}");
        if change != "missing" {
            assert_eq!(
                state.schematic.session.editor.tool,
                Tool::Select,
                "{change}"
            );
            assert!(
                state.schematic.session.editor.pending_bus_tap.is_none(),
                "{change}"
            );
        }
    }
}

//! Create array through the whole application: when it is offered, what it
//! names the copies, and what it reports.

use crate::workbench::app::RSpiceApp;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::{ComponentType, Point, SchematicProbe};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;

/// A selection holding a probe greys Create array and says why, rather than
/// opening a form whose first canvas click is refused for that reason.
#[test]
fn create_array_is_greyed_with_its_reason_while_a_probe_is_selected() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let schematic = &mut app.state.model.schematic;
    let part = schematic.add_component(ComponentType::Resistor, Point::origin());
    schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(900, Point::new(0, 40), "P1", None).expect("a probe is well formed"),
    );
    let selection = &mut schematic.session.editor.selection;
    selection.select_only_component(part);
    selection.select_probe(900);

    assert_eq!(
        app.command_availability(DesignCommand::ArraySelection.key()),
        CommandAvailability::Disabled(
            "probes are not copied into an array; leave them out of the selection"
        )
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(part);
    assert_eq!(
        app.command_availability(DesignCommand::ArraySelection.key()),
        CommandAvailability::Available
    );
}

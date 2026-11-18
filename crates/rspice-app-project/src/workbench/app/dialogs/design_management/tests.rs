//! Design management tests.

use super::*;
use crate::state::{
    CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDirection, CrossSheetPortEndpoint,
    CrossSheetSignalType, Point,
};

#[test]
fn probe_ids_participate_in_sheet_governance_and_selected_authority() {
    let mut schematic = crate::state::SchematicState::default();
    schematic.document_mut_for_test().probes.push(
        crate::state::SchematicProbe::new(
            81,
            crate::state::Point::new(10, 20),
            "V(out)",
            Some("V(out)".to_owned()),
        )
        .unwrap(),
    );
    schematic.session.editor.selection.select_only_probe(81);

    assert_eq!(all_stable_object_ids(&schematic), vec![81]);
    assert_eq!(selected_stable_object_ids(&schematic), vec![81]);
}

/// The Sheets table counts each sheet's ports and the nets leaving through
/// them separately: a net reaching two other sheets is two ports but one
/// net, and a second net through a third port is a second net.
#[test]
fn the_sheets_table_counts_ports_and_off_sheet_nets_apart() {
    let [bias, afe, output] = [SheetId::new(), SheetId::new(), SheetId::new()];
    let port = |net: &str, wire: u64, from: SheetId, to: SheetId| CrossSheetPortDefinition {
        net_name: net.to_owned(),
        first: CrossSheetPortEndpoint {
            sheet_id: from,
            anchor: CrossSheetPortAnchor::WirePoint {
                wire_id: wire,
                point: Point::origin(),
            },
        },
        second: CrossSheetPortEndpoint {
            sheet_id: to,
            anchor: CrossSheetPortAnchor::WirePoint {
                wire_id: wire + 100,
                point: Point::origin(),
            },
        },
        direction: CrossSheetPortDirection::Output,
        signal_type: CrossSheetSignalType::Analog,
        discipline: CrossSheetDiscipline::Electrical,
    };
    let ports = [
        port("VREF", 1, bias, afe),
        port("vref", 2, bias, output),
        port("BIAS", 3, bias, afe),
    ];

    assert_eq!(super::manager::sheet_crossings(&ports, bias), (3, 2));
    assert_eq!(super::manager::sheet_crossings(&ports, afe), (2, 2));
    assert_eq!(super::manager::sheet_crossings(&ports, output), (1, 1));
    assert_eq!(
        super::manager::sheet_crossings(&ports, SheetId::new()),
        (0, 0)
    );
}

/// Each audited occurrence lists the instances it places directly: not
/// itself, not the instances they place, and not an instance under another
/// whose name it begins.
#[test]
fn each_audited_occurrence_lists_the_instances_it_places() {
    let subject = |path: &str| HierarchyAuditSubject {
        instance_path: path.to_owned(),
        cell_name: "amp".to_owned(),
        design_view: "schematic".to_owned(),
        declared_fallbacks: Vec::new(),
        resolved_simulation_view: Some("schematic".to_owned()),
        fallback_used: None,
        child_instance_paths: Vec::new(),
        protected_boundary_id: None,
    };
    let mut subjects = [
        "/",
        "/X1",
        "/X1/X1",
        "/X10",
        "/X1/X2",
        "/X10/X1",
        "/X1/X1/R1",
    ]
    .map(subject)
    .to_vec();
    super::operations::list_children(&mut subjects);
    let children = |path: &str| {
        subjects
            .iter()
            .find(|subject| subject.instance_path == path)
            .map(|subject| subject.child_instance_paths.clone())
            .unwrap()
    };
    assert_eq!(children("/"), ["/X1", "/X10"]);
    assert_eq!(children("/X1"), ["/X1/X1", "/X1/X2"]);
    assert_eq!(children("/X10"), ["/X10/X1"]);
    assert_eq!(children("/X1/X1"), ["/X1/X1/R1"]);
    assert!(children("/X1/X2").is_empty());
    assert!(children("/X1/X1/R1").is_empty());

    let mut below = ["/X1/X1", "/X1/X1/R1"].map(subject).to_vec();
    super::operations::list_children(&mut below);
    assert_eq!(below[0].child_instance_paths, ["/X1/X1/R1"]);
}

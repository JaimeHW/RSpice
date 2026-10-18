//! The net inspector's account of off-sheet connectors: where a crossing
//! lands, and which names it counts as one net.

use super::*;

/// The canvas can draw that a name crosses sheets but not where it lands, so
/// the inspector is the only surface that can answer "which page". A net no
/// connector names must not grow the rows at all.
#[test]
fn the_net_inspector_states_the_sheets_a_connector_partners_with() {
    use crate::state::{
        CrossSheetPortDirection, NetLabel, NetLabelKind, SheetDefinition, SheetPortPolicy,
        SheetTemplate,
    };

    let mut state = AppState::default();
    let key = state.workspace.content.active_schematic_reference().key();
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "LOCAL"));
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::off_sheet(
            2,
            Point::new(20, 0),
            "BIAS",
            CrossSheetPortDirection::Output,
        ));
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::off_sheet(
            3,
            Point::new(40, 0),
            "BIAS",
            CrossSheetPortDirection::Input,
        ));

    // A plain label declares no crossing, so the panel stays as it was.
    assert!(off_sheet_declaration(&state, "LOCAL").is_none());
    assert!(off_sheet_declaration(&state, "never_drawn").is_none());

    // Before any sheet catalog exists the crossing has nowhere to land.
    let lone = off_sheet_declaration(&state, "BIAS").expect("a connector declares itself");
    assert_eq!(lone.direction, "input, output");
    assert_eq!(lone.partners, "none \u{2014} no partner on another sheet");

    let first = state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [1, 2])
        .expect("sheet ownership");
    let catalog = state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Output".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    catalog
        .assign_objects(catalog.revision(), second, [3])
        .expect("the partner connector moves to the second sheet");
    catalog
        .set_active(first)
        .expect("the first sheet is active");

    let paired = off_sheet_declaration(&state, "BIAS").expect("a connector declares itself");
    assert_eq!(paired.direction, "input, output");
    assert_eq!(paired.partners, "p2", "the partner's page, not its own");

    // Read from the far side the answer is the other page, never both.
    state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog")
        .set_active(second)
        .expect("the second sheet is active");
    assert_eq!(
        off_sheet_declaration(&state, "BIAS")
            .expect("a connector declares itself")
            .partners,
        "p1"
    );

    assert_eq!(
        NetLabelKind::direction_label(CrossSheetPortDirection::InOut),
        "bidirectional",
        "the inspector and the placement dialog share one direction vocabulary"
    );
    assert_eq!(
        NetLabelKind::default(),
        NetLabelKind::Local,
        "a label makes no crossing claim until one is authored"
    );
}

/// Connectivity joins `Bias` and `BIAS` under either naming setting, so the
/// inspector counts their connectors as one net's under both.
#[test]
fn the_net_inspector_folds_case_the_way_connectivity_does() {
    use crate::state::{CrossSheetPortDirection, NetLabel, NetNamingPolicy};

    for policy in [
        NetNamingPolicy::StrictCaseSensitive,
        NetNamingPolicy::SpiceCompatibleRelaxed,
    ] {
        let mut state = AppState::default();
        let document = state.schematic.document_mut_for_test();
        document.document_policy.net_naming = policy;
        for (id, x, name, direction) in [
            (1, 20, "BIAS", CrossSheetPortDirection::Output),
            (2, 40, "bias", CrossSheetPortDirection::Input),
        ] {
            document
                .net_labels
                .push(NetLabel::off_sheet(id, Point::new(x, 0), name, direction));
        }
        let declared = off_sheet_declaration(&state, "Bias").expect("both connectors declare");
        assert_eq!(declared.direction, "input, output", "{policy:?}");
        assert_eq!(normalized_net_name("Out"), normalized_net_name("out"));
    }
}

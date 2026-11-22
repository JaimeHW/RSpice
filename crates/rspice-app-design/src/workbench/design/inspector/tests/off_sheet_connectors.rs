//! The net inspector's account of off-sheet connectors: where a crossing
//! lands, and which names it counts as one net.

use super::*;
use crate::workbench::design::test_host::DesignTestHost;

/// The canvas can draw that a name crosses sheets but not where it lands, so
/// the inspector is the only surface that can answer "which page". A net no
/// connector names must not grow the rows at all.
#[test]
fn the_net_inspector_states_the_sheets_a_connector_partners_with() {
    use crate::state::{
        CrossSheetPortDirection, NetLabel, NetLabelKind, SheetDefinition, SheetPortPolicy,
    };

    let mut state = DesignTestHost::default();
    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "LOCAL"));
    state
        .model
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
        .model
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
    assert!(off_sheet_declaration(&state.model, "LOCAL").is_none());
    assert!(off_sheet_declaration(&state.model, "never_drawn").is_none());

    // Before any sheet catalog exists the crossing has nowhere to land.
    let lone = off_sheet_declaration(&state.model, "BIAS").expect("a connector declares itself");
    assert_eq!(lone.direction, "input, output");
    assert_eq!(lone.partners, "none \u{2014} no partner on another sheet");

    let first = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [1, 2])
        .expect("sheet ownership");
    let catalog = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Output".to_owned(),
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

    let paired = off_sheet_declaration(&state.model, "BIAS").expect("a connector declares itself");
    assert_eq!(paired.direction, "input, output");
    assert_eq!(paired.partners, "p2", "the partner's page, not its own");

    // Read from the far side the answer is the other page, never both.
    state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog")
        .set_active(second)
        .expect("the second sheet is active");
    assert_eq!(
        off_sheet_declaration(&state.model, "BIAS")
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
        let mut state = DesignTestHost::default();
        let document = state.model.schematic.document_mut_for_test();
        document.document_policy.net_naming = policy;
        for (id, x, name, direction) in [
            (1, 20, "BIAS", CrossSheetPortDirection::Output),
            (2, 40, "bias", CrossSheetPortDirection::Input),
        ] {
            document
                .net_labels
                .push(NetLabel::off_sheet(id, Point::new(x, 0), name, direction));
        }
        let declared =
            off_sheet_declaration(&state.model, "Bias").expect("both connectors declare");
        assert_eq!(declared.direction, "input, output", "{policy:?}");
        assert_eq!(normalized_net_name("Out"), normalized_net_name("out"));
    }
}

/// The Scope row names how far each net's name reaches: a supply symbol's
/// rail everywhere, an off-sheet connector's net and a plain name across
/// the cell view's sheets, and only an unnamed net, or a name on a view of
/// one sheet, its own sheet.
#[test]
fn the_net_inspector_names_each_nets_real_scope() {
    use crate::state::{CrossSheetPortDirection, NetLabel, SheetDefinition, SheetPortPolicy, Wire};
    use rspice_design::schematic::net_label::SupplyGlyph;

    let mut state = DesignTestHost::default();
    let document = state.model.schematic.document_mut_for_test();
    for (id, y) in [(1, 0), (2, 100), (3, 200), (4, 300), (5, 400)] {
        document
            .wires
            .push(Wire::new(id, vec![Point::new(0, y), Point::new(200, y)]));
    }
    document.net_labels.extend([
        NetLabel::supply(11, Point::new(100, 0), "VDD", SupplyGlyph::Bar),
        NetLabel::off_sheet(
            12,
            Point::new(100, 100),
            "BIAS",
            CrossSheetPortDirection::Output,
        ),
        NetLabel::new(13, Point::new(100, 200), "sig"),
        NetLabel::new(14, Point::new(100, 300), "clk!"),
    ]);
    let scopes = |state: &DesignTestHost| {
        let nets =
            rspice_design::connectivity::summary::design_nets(state.model.schematic.document());
        let mut scopes = nets
            .iter()
            .map(|net| {
                let off_sheet = off_sheet_declaration(&state.model, &net.name).is_some();
                let name = if net.authored_name {
                    net.name.to_ascii_lowercase()
                } else {
                    "unnamed".to_owned()
                };
                (name, net_scope(&state.model, net, off_sheet))
            })
            .collect::<Vec<_>>();
        scopes.sort();
        scopes
    };
    let owned = |pairs: &[(&str, &str)]| {
        pairs
            .iter()
            .map(|(name, scope)| ((*name).to_owned(), (*scope).to_owned()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        scopes(&state),
        owned(&[
            ("bias", "cell view (off-sheet)"),
            ("clk!", "global"),
            ("sig", "sheet-local"),
            ("unnamed", "sheet-local"),
            ("vdd", "global (supply rail)"),
        ])
    );

    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [1, 2, 3, 4, 5, 11, 12, 13, 14])
        .expect("sheet ownership");
    state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog")
        .create_sheet(
            SheetDefinition {
                name: "Output".to_owned(),
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    let named = scopes(&state)
        .into_iter()
        .find(|(name, _)| name == "sig")
        .expect("the named net");
    assert_eq!(named.1, "cell view", "a name joins its name on every sheet");
}

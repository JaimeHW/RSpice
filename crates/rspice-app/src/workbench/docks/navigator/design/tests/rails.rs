//! What the whole-design rails claim about the objects a run will reach.
//!
//! These rails answer about the design rather than the sheet: the interface the
//! deck declares, the ports an S-parameter matrix indexes, and every occurrence
//! a run reaches a source through. Their shared claim is that document order is
//! no order at all — a pin is numbered by `netlist_order`, a port by its
//! `port=`, and a source is named by the path that reaches it — and that a name
//! or number claimed twice is a hazard stated on both rows, in the warning tone
//! rather than the faint one the counts use.
//!
//! The fixtures and frame-reading helpers live in the parent, because the
//! hierarchy and keyboard cases share them.

use super::*;

/// The ports rail lists the interface in the order the deck has it, and numbers
/// every pin with the position it occupies there.
///
/// Document order is the order the pins happened to be drawn in, which is no
/// order at all: the `.SUBCKT` line and the node order of every instance of
/// this cell come from `netlist_order`. A rail numbering rows by their place in
/// the file would call a pin `#1` that the deck calls third.
#[test]
fn the_ports_rail_lists_the_interface_in_the_order_the_deck_has_it() {
    let app = interface_design();

    let rows = port_rows(&app.state, SheetScope::AllSheets, "");

    assert_eq!(
        rows.iter()
            .map(|row| (row.spec.name.as_str(), row.meta()))
            .collect::<Vec<_>>(),
        [
            ("ALPHA", "#1 \u{00b7} in".to_owned()),
            ("BETA", "#2 \u{00b7} in".to_owned()),
            ("GAMMA", "#3 \u{00b7} out".to_owned()),
        ]
    );
}

/// A pin whose name declares a vector states the conductors it carries.
///
/// The name is the declaration, so `DATA[7:0]` is one row standing for eight
/// wires. A rail that listed it as one wire would under-count the interface by
/// seven every time it was read.
#[test]
fn a_vector_pin_states_the_conductors_its_name_declares() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(port(
            404,
            "DATA[7:0]",
            Some(1),
            PortDirection::In,
            "the sampled word",
        ));
    app.state.sync_active_schematic_to_workspace();

    let rows = port_rows(&app.state, SheetScope::AllSheets, "");

    assert_eq!(rows[0].meta(), "#1 \u{00b7} in \u{00b7} [8]");
    let tooltip = rows[0].tooltip();
    assert!(
        tooltip.contains("8 conductors") && tooltip.contains("the sampled word"),
        "the tooltip carries the whole contract: {tooltip}"
    );
}

/// A name declared twice is marked on every row that declares it, in the tone a
/// hazard is stated in rather than the tone a count is.
///
/// The interface takes the first declaration and the rest add no pin, so the
/// rail is the only place the reader is told that the sheet shows two ports and
/// the deck has one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_port_name_declared_twice_is_marked_on_both_rows() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    // Folded for case, because the netlist folds it: `SENSE` and `sense` are
    // one node, declared twice.
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(port(405, "SENSE", Some(1), PortDirection::In, ""));
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(port(406, "sense", Some(2), PortDirection::In, ""));
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(port(407, "CLEAN", Some(3), PortDirection::Out, ""));
    app.state.sync_active_schematic_to_workspace();

    let rows = port_rows(&app.state, SheetScope::AllSheets, "");
    assert_eq!(
        rows.iter()
            .map(|row| (row.spec.name.as_str(), row.duplicated))
            .collect::<Vec<_>>(),
        [("SENSE", true), ("sense", true), ("CLEAN", false)],
        "the repetition is a fact about both declarations, not about the second"
    );
    assert!(
        rows[0].tooltip().contains("declared more than once"),
        "{}",
        rows[0].tooltip()
    );

    // And the marking reaches the screen: a warning painted in the faint tone
    // every other meta column uses is not a warning.
    let panel = NavigatorPanel::open(app);
    let warn = Tokens::get(&panel.ctx).color.warn;
    let tone = |meta: &str| {
        panel
            .runs
            .iter()
            .find(|(run, _, _)| run.as_str() == meta)
            .map(|(_, _, color)| *color)
    };
    assert_eq!(tone("#1 \u{00b7} in"), Some(warn));
    assert_eq!(tone("#2 \u{00b7} in"), Some(warn));
    assert_ne!(
        tone("#3 \u{00b7} out"),
        Some(warn),
        "the pin declared once states no hazard"
    );
}

/// An excitation row stands for the instance it names, which is what the shared
/// object menu acts on.
///
/// This was the one rail whose rows answered to the pointer alone: they could
/// be selected and nothing else, while every other rail carried open, rename
/// and find on the same gesture.
#[test]
fn an_excitation_row_stands_for_the_instance_the_object_menu_acts_on() {
    let mut app = RSpiceApp::test_instance();
    let mut source = crate::state::Component::new(
        501,
        ComponentType::VoltageSource,
        crate::state::Point::new(60, 80),
    );
    source.name = "VIN".to_owned();
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(source);
    app.state.sync_active_schematic_to_workspace();

    let sources = crate::simulation::placed_sources::placed_sources(
        &app.state.schematic,
        &app.state.workspace.content.stimulus_library,
        app.state.sim_setup.analysis_plan.as_ref(),
    );
    let placed = sources
        .iter()
        .find(|source| source.reference == "VIN")
        .expect("the fixture source is placed");

    assert_eq!(
        placed_object(&app.state, placed.component_id, &placed.reference),
        Some(NavigatorObject::Component {
            id: 501,
            label: "VIN".to_owned(),
            position: crate::state::Point::new(60, 80),
        })
    );

    // A source the sheet holds no instance for is offered no menu rather than a
    // menu of commands that would act on nothing.
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert_eq!(
        placed_object(&app.state, placed.component_id, &placed.reference),
        None
    );
}

// ------------------------------------------------------------------ RF ports

/// One RF port, as the palette places it: a reference and the parameter string
/// the property sheet writes.
fn rf_port(id: u64, name: &str, params: &str) -> crate::state::Component {
    let mut component = crate::state::Component::new(
        id,
        ComponentType::RfPort,
        crate::state::Point::new(80, 40 + id as i32),
    );
    component.name = name.to_owned();
    component.params = params.to_owned();
    component
}

/// A design whose ports carry `params`, with the navigator active over it.
fn rf_bench(ports: Vec<crate::state::Component>) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .extend(ports);
    app.state.sync_active_schematic_to_workspace();
    app
}

/// The ports the rail lists, resolved the way the section resolves them.
fn listed_rf_ports(app: &RSpiceApp) -> Vec<crate::simulation::placed_sources::PlacedRfPort> {
    crate::simulation::placed_sources::placed_rf_ports(
        &app.state.schematic,
        app.state.sim_setup.analysis_plan.as_ref(),
    )
}

/// A plan holding one analysis of `kind`, enabled.
fn plan_with(
    kind: crate::simulation::plan::AnalysisKind,
) -> crate::simulation::plan::SimulationPlan {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (instance, _) = plan.insert(kind).expect("the fixture analysis inserts");
    plan.set_enabled(instance, true)
        .expect("the fixture analysis takes its enabled flag");
    plan
}

/// The rail exists only above a design that places a port.
///
/// Every other band in this navigator answers about something every design
/// has. A permanent empty "RF ports" band would spend a row of a narrow rail —
/// on every sheet that is not an RF testbench, forever — to report that a
/// device the design never reached for is still unused, which is the density
/// bar this product is held to rather than a matter of taste.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rf_port_rail_is_drawn_only_above_a_design_that_places_one() {
    let announces = |panel: &NavigatorPanel| panel.band_if_present("RF ports").is_some();

    // The ordinary design: an interface, conductors, no port.
    assert!(
        !announces(&NavigatorPanel::open(interface_design())),
        "a design that places no RF port carries no RF band"
    );

    // One port is enough, and the band states what it holds.
    let panel = NavigatorPanel::open(rf_bench(vec![rf_port(601, "P1", "port=1 z0=50")]));
    assert!(announces(&panel));
    assert_eq!(panel.stated_count("RF ports").as_deref(), Some("1"));
    assert!(
        panel.rows_under("RF ports").contains(&"P1".to_owned()),
        "the band paints its row: {:?}",
        panel.rows_under("RF ports")
    );
}

/// The rail lists ports in the order an S-parameter matrix indexes them, and
/// every row states the number it is indexed by, the impedance it presents and
/// what it does behind that impedance.
///
/// Document order is the order the ports happened to be drawn in, which is no
/// order at all: an `.sp` run addresses a port by its `port=` number, so a rail
/// in drawing order would put the row a run calls port 1 second.
#[test]
fn the_rf_port_rail_indexes_its_rows_the_way_an_s_parameter_run_does() {
    let app = rf_bench(vec![
        rf_port(602, "PLOAD", "port=3 z0=75"),
        rf_port(603, "PIN", "port=1 z0=50 pwr=-10"),
        rf_port(604, "POUT", "port=2 z0=5e1"),
    ]);

    let rows = listed_rf_ports(&app);

    assert_eq!(
        rows.iter()
            .map(|port| (port.reference.as_str(), rf_port_meta(port)))
            .collect::<Vec<_>>(),
        [
            (
                "PIN",
                "#1 \u{00b7} Z0 50 \u{00b7} drive \u{00b7} unused".to_owned()
            ),
            // `5e1` and `50` are one impedance, and the rail prints them as one.
            (
                "POUT",
                "#2 \u{00b7} Z0 50 \u{00b7} term \u{00b7} unused".to_owned()
            ),
            (
                "PLOAD",
                "#3 \u{00b7} Z0 75 \u{00b7} term \u{00b7} unused".to_owned()
            ),
        ]
    );
}

/// The meta column names what reads the port, and says the plan reads none of
/// them in the words the studio's Excitations page uses.
///
/// The distinction is the point: a source nothing reads is a finding, and a
/// port nothing reads is a termination still loading the design. Borrowing the
/// excitation rail's `no reader` here would call every port of a time-domain
/// testbench a defect.
#[test]
fn an_rf_port_row_names_the_s_parameter_run_that_reads_it() {
    let mut app = rf_bench(vec![rf_port(605, "P1", "port=1 z0=50")]);

    let unread = rf_port_meta(&listed_rf_ports(&app)[0]);
    assert!(
        unread.ends_with("unused"),
        "a plan with no `.sp` run leaves the port unused: {unread}"
    );

    // A transient reaches the port as the two-terminal element the netlist
    // already carries, which is not readership: it would make every plan read
    // every port.
    app.state.sim_setup.analysis_plan =
        Some(plan_with(crate::simulation::plan::AnalysisKind::Transient));
    let loaded = rf_port_meta(&listed_rf_ports(&app)[0]);
    assert!(
        loaded.ends_with("unused"),
        "only an `.sp` run indexes a port: {loaded}"
    );

    app.state.sim_setup.analysis_plan =
        Some(plan_with(crate::simulation::plan::AnalysisKind::SParameter));
    let read = rf_port_meta(&listed_rf_ports(&app)[0]);
    assert!(
        read.ends_with("S-parameter port"),
        "the row names the part the run reads it as: {read}"
    );
    let tooltip = rf_port_tooltip(&listed_rf_ports(&app)[0], false, None);
    assert!(
        tooltip.contains("S-parameter port") && !tooltip.contains("disabled"),
        "{tooltip}"
    );
}

/// Two ports claiming one number are marked on both rows, in the tone a hazard
/// is stated in rather than the tone a count is, and the tooltip names the
/// number they are fighting over.
///
/// An S-parameter run addresses a port by its number, so the ports sharing one
/// cannot both be the port that run measures. The meta column has room to paint
/// the row as a hazard but not to say what the hazard is, which is why the
/// sentence is in the tooltip.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn two_rf_ports_claiming_one_number_are_marked_on_both_rows() {
    let app = rf_bench(vec![
        rf_port(606, "PA", "port=1 z0=50"),
        rf_port(607, "PB", "port=1 z0=50"),
        rf_port(608, "PC", "port=2 z0=50"),
    ]);

    let rows = listed_rf_ports(&app);
    let collisions = crate::simulation::placed_sources::duplicate_port_numbers(&rows);
    assert_eq!(collisions, vec![1]);

    let tooltip = rf_port_tooltip(&rows[0], true, None);
    assert!(
        tooltip.contains("Port number 1 is claimed by more than one placed port"),
        "the collision names its own number: {tooltip}"
    );
    assert!(
        !rf_port_tooltip(&rows[2], false, None).contains("claimed by more than one"),
        "the port claiming a number alone states no hazard"
    );

    // The marking reaches the screen in the warning tone, not the faint one. The
    // meta elides beside the port's name in this rail: it is found by its opening.
    let panel = NavigatorPanel::open(app);
    let warn = Tokens::get(&panel.ctx).color.warn;
    let tone = |meta: &str| {
        panel
            .runs
            .iter()
            .filter(|(run, _, _)| !run.is_empty())
            .find(|(run, _, _)| meta.starts_with(run.trim_end_matches('\u{2026}')))
            .map(|(_, _, color)| *color)
    };
    assert_eq!(tone(&rf_port_meta(&rows[0])), Some(warn));
    assert_eq!(tone(&rf_port_meta(&rows[1])), Some(warn));
    assert_ne!(
        tone(&rf_port_meta(&rows[2])),
        Some(warn),
        "the port claiming its number alone states no hazard"
    );
}

/// The tooltip carries what the row has no room for: the terminals the port
/// sits across, and the sentence that a plan holding no `.sp` run is not a
/// plan that broke the port.
#[test]
fn an_rf_port_tooltip_reads_the_port_the_row_could_only_summarise() {
    let app = rf_bench(vec![rf_port(609, "P1", "port=1 z0=50")]);
    let rows = listed_rf_ports(&app);

    let tooltip = rf_port_tooltip(&rows[0], false, None);
    assert!(
        tooltip.starts_with("P1 \u{00b7} term \u{00b7} Z0 50"),
        "{tooltip}"
    );
    assert!(
        tooltip.contains("No S-parameter analysis in this plan reads this port"),
        "{tooltip}"
    );
}

// -------------------------------------------- the whole design's excitations

/// One independent source, as the palette places it.
fn source(id: u64, kind: ComponentType, name: &str, params: &str) -> crate::state::Component {
    let mut component =
        crate::state::Component::new(id, kind, crate::state::Point::new(120, 40 + id as i32));
    component.name = name.to_owned();
    component.params = params.to_owned();
    component
}

/// The design this half of the rail exists for: the root places `VDD` and two
/// instances of one master, and that master places a source and an RF port.
///
/// Two instances rather than one, because the multiplicity is the claim — a run
/// flattens the hierarchy, so the master's `V1` is two cards in the deck and the
/// rail owes the reader a row for each.
fn hierarchical_excitations() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(source(701, ComponentType::VoltageSource, "VDD", "dc=5"));
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(placed(702, "XA", "work", "afe"));
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(placed(703, "XB", "work", "afe"));
    app.state.sync_active_schematic_to_workspace();

    let mut child = SchematicState::default();
    child.document_mut_for_test().components.push(source(
        711,
        ComponentType::VoltageSourceSin,
        "V1",
        "freq=1k",
    ));
    child
        .document_mut_for_test()
        .components
        .push(rf_port(712, "P1", "port=1 z0=50"));
    add_master(&mut app.state, "work", "afe", child);
    app
}

/// The whole design, at every occurrence the run reaches a source through.
///
/// The rail read the editor's buffer, so the root of every hierarchical design
/// reported that it places nothing but its own supplies — while the run drove
/// every source drawn below it. Each row of another occurrence states the path
/// in front of the reference, because `V1` names nothing until the path is read
/// and one drawn source becomes two rows here.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_excitation_rail_lists_every_occurrence_the_run_reaches_a_source_through() {
    let panel = NavigatorPanel::open(hierarchical_excitations());

    assert_eq!(
        panel.announced_rows(|label| label.ends_with("VDD") || label.ends_with("V1")),
        ["VDD", "/XA/V1", "/XB/V1"],
        "the root's own source leads, and each occurrence of the master states \
         where the run reaches its source"
    );
    assert_eq!(
        panel.stated_count("Excitations").as_deref(),
        Some("3"),
        "the band counts what the run drives, not what the sheet holds"
    );
}

/// The RF rail joins the same two readings, and keeps the one order an
/// S-parameter run has.
///
/// A number claimed by two occurrences is the finding: the run indexes the
/// flattened design, so one port drawn once and reached twice claims one index
/// of one matrix.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rf_rail_marks_the_number_two_occurrences_of_one_master_both_claim() {
    let panel = NavigatorPanel::open(hierarchical_excitations());

    assert_eq!(
        panel.announced_rows(|label| label.ends_with("P1")),
        ["/XA/P1", "/XB/P1"]
    );
    assert_eq!(panel.stated_count("RF ports").as_deref(), Some("2"));

    let warn = Tokens::get(&panel.ctx).color.warn;
    let meta = panel
        .runs
        .iter()
        .filter(|(run, _, _)| run.starts_with("#1 \u{00b7} Z0 50"))
        .map(|(_, _, color)| *color)
        .collect::<Vec<_>>();
    assert_eq!(
        meta,
        vec![warn, warn],
        "both rows carry the hazard, because the collision is a fact about the pair"
    );
}

/// Open the clicked occurrence before selecting its component; component IDs
/// belong to their schematic and can repeat across documents.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_excitation_of_another_occurrence_opens_it_rather_than_selecting_here() {
    let mut panel = NavigatorPanel::open(hierarchical_excitations());
    let occurrence = InstancePath::root()
        .child("XB")
        .expect("the fixture instance is nameable");

    panel.click("/XB/V1");
    let state = &panel.app.state;

    assert_eq!(
        state.workspace.content.occurrence_path(),
        occurrence,
        "the click lands the session on the occurrence the row names"
    );
    assert_eq!(
        state.workspace.content.active_view,
        CellViewRef::new("work", "afe", "schematic")
    );
    assert!(
        state.schematic.session.editor.selection.has_component(711),
        "and selects the instance there, once the session is standing on it"
    );
}

/// Sheet scope narrows the active cell view, and a row of another occurrence is
/// on no sheet of it.
///
/// Narrowing those rows away would make a control that says "this sheet"
/// silently answer a question about the hierarchy that it does not ask — and
/// would hide, at the position most readers leave it in, every source the run
/// drives below the root. The narrowing itself is unchanged and still reaches
/// the rows of the sheet on screen; see
/// [`the_sheet_scope_narrows_the_object_rails_to_the_active_sheet`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sheet_scope_leaves_the_rows_of_another_occurrence_alone() {
    let mut app = two_sheet_named_signals();
    let elsewhere = InstancePath::root()
        .child("XA")
        .expect("the fixture instance is nameable");
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(placed(304, "XA", "work", "afe"));
    app.state.sync_active_schematic_to_workspace();

    for scope in SheetScope::OPTIONS {
        assert!(
            derived_row_is_in_scope(&app.state, scope, Some(&elsewhere), 302),
            "{scope:?} has no authority over a row drawn in another occurrence"
        );
    }
    // 302 is the source the catalog assigned to the sheet the session is not
    // on, which is exactly the row the scope control does narrow.
    assert!(
        !derived_row_is_in_scope(&app.state, SheetScope::ActiveSheet, None, 302),
        "a row of the sheet on screen is still the scope control's to narrow"
    );
}

/// The occurrence being edited is listed once, from the live buffer.
///
/// Both readings can reach it — the projection binds it and the editor holds
/// it — and the row that has to survive is the editor's, because that is the one
/// carrying uncommitted edits and answering the object menu.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_occurrence_in_front_of_the_reader_is_listed_once_and_from_its_buffer() {
    let mut app = hierarchical_excitations();
    let occurrence = InstancePath::root()
        .child("XA")
        .expect("the fixture instance is nameable");
    hierarchy_tree::open_occurrence(&mut app.state, &occurrence);

    let rows = whole_design_sources(&app);

    assert_eq!(
        rows.iter()
            .map(|source| (source.occurrence_label(), source.reference.as_str()))
            .collect::<Vec<_>>(),
        [
            ("/".to_owned(), "V1"),
            ("/".to_owned(), "VDD"),
            ("/XB".to_owned(), "V1"),
        ],
        "the sheet on screen is read as its own root and the second instance \
         keeps the row that reaches it"
    );
    assert_eq!(
        row_is_elsewhere(&app.state, rows[0].occurrence.as_ref()),
        None,
        "the row of the occurrence being edited carries no path in front of it"
    );

    // The design root is its own separator: read from inside a child master,
    // the root's supply is `/VDD` and not `//VDD`.
    let panel = NavigatorPanel::open(app);
    assert!(
        panel.controls.iter().any(|(label, _, _)| label == "/VDD"),
        "the rail announces: {:#?}",
        panel
            .controls
            .iter()
            .map(|(label, _, _)| label.as_str())
            .collect::<Vec<_>>()
    );
}

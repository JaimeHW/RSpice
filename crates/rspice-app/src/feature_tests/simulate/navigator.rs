//! The navigator's Simulate rail drawn over the whole application: the run
//! set card keeps each value off its label and states a quantity axis's
//! unit, the excitations rail counts the design the page it opens lists, the
//! rail derives what it states once a frame, and its filter reaches past its
//! page rows.

use crate::workbench::RSpiceApp;
use crate::workbench::docks::navigator::simulate::{
    NAV_PROPERTY_COLUMN_GAP, NAV_PROPERTY_PADDING_X,
};
use crate::workbench::state::Workspace;
use rspice_app_workbench::workbench::docks::navigator;
use rspice_simulation_contract::run_set::RunSetDimensionKind;

/// Every painted run of text in the navigator, with where it landed.
fn navigator_painted_lines(width: f32, mut app: RSpiceApp) -> Vec<(String, egui::Rect)> {
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, 1400.0),
        )),
        ..Default::default()
    };
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(width);
                navigator::show(ui, &mut app);
            });
    });
    let mut lines = Vec::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
}

/// No Run set row paints its value on top of its label.
///
/// The card is a two-column form grid in the mockup and the dock is 228 px
/// wide below 1261 px, which is where the single-row layout failed: the value
/// was floated right and grew leftwards with nothing to stop it, so
/// "Supply voltage" ran straight into its values with no gutter, no ellipsis
/// and no way to read where one ended. Measured columns cannot do that, and
/// this is the width that proves it.
#[test]
fn run_set_card_rows_keep_their_value_off_their_label() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    // Only an axis the run sweeps has a row.
    for dimension in &mut app.state.model.sim_setup.run_set.dimensions {
        dimension.enabled = true;
    }
    let names = app
        .state
        .model
        .sim_setup
        .run_set
        .dimensions
        .iter()
        .map(|dimension| dimension.name.clone())
        .collect::<Vec<_>>();
    assert!(!names.is_empty(), "the fixture run set declares axes");
    let lines = navigator_painted_lines(228.0, app);

    let find = |text: &str| -> egui::Rect {
        lines
            .iter()
            .find(|(painted, _)| painted == text)
            .unwrap_or_else(|| {
                panic!(
                    "the Run set card paints {text:?}; it painted {:?}",
                    lines.iter().map(|(text, _)| text).collect::<Vec<_>>()
                )
            })
            .1
    };
    for label in &names {
        let label = label.as_str();
        let label_rect = find(label);
        let (value, value_rect) = lines
            .iter()
            .find(|(_, rect)| {
                rect.left() > label_rect.right()
                    && (rect.center().y - label_rect.center().y).abs() < 2.0
            })
            .map(|(text, rect)| (text.as_str(), *rect))
            .unwrap_or_else(|| panic!("the {label:?} row paints no value"));
        assert!(
            value_rect.left() - label_rect.right() >= NAV_PROPERTY_COLUMN_GAP - 0.5,
            "{label:?} ends at {} and {value:?} starts at {}, closer than the \
             {NAV_PROPERTY_COLUMN_GAP} px column gap",
            label_rect.right(),
            value_rect.left()
        );
        // A value pushed off the card's trailing inset would satisfy the gap
        // above and still be unreadable, so the row has to stay inside the
        // dock as well as clear of its own label.
        assert!(
            value_rect.right() <= 228.0 - NAV_PROPERTY_PADDING_X + 0.5,
            "{value:?} ends at {}, past the card's {NAV_PROPERTY_PADDING_X} px \
             trailing inset in a 228 px dock",
            value_rect.right()
        );
    }
}

/// A quantity axis states its unit; a reference axis has none to state.
///
/// The run set stores temperature as bare lexical numbers, so the rail read
/// "-40 25 125" and the unit lived only in the reader's head. The axis knows
/// its own unit — this is that unit reaching the row.
#[test]
fn run_set_card_states_the_unit_of_a_quantity_axis() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    // A disabled axis states its size, not its values, so the unit has nothing
    // to attach to. Enable the one this asks about.
    let temperature = app
        .state
        .model
        .sim_setup
        .run_set
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Temperature)
        .expect("the fixture run set declares a temperature axis");
    temperature.enabled = true;
    assert_eq!(temperature.unit(), Some("\u{b0}C"));
    let expected = format!(
        "{} \u{b0}C",
        temperature
            .values
            .iter()
            .map(|value| value.lexical.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let painted = navigator_painted_lines(228.0, app)
        .into_iter()
        .map(|(text, _)| text)
        .collect::<Vec<_>>();
    assert!(
        painted.contains(&expected),
        "the temperature row states its unit ({expected:?}); it painted {painted:?}"
    );
}

/// A design the run reaches below the sheet in front of the reader: a root
/// placing one supply, and two instances of a master placing a sine source.
/// The open sheet places one source; the design the run drives places three.
///
/// Built and returned rather than seeded into an app handed in, so the fixture
/// costs the whole-application-access ratchet nothing.
fn hierarchical_design() -> RSpiceApp {
    use crate::state::{
        CellViewRef, Component, ComponentType, Library, LibraryCellInstance, Point, SchematicState,
        View, ViewType,
    };

    fn named(id: u64, kind: ComponentType, name: &str, params: &str) -> Component {
        let mut component = Component::new(id, kind, Point::new(60, 60));
        component.name = name.to_owned();
        component.params = params.to_owned();
        component
    }

    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(named(801, ComponentType::VoltageSource, "VDD", "dc=5"));
    for (id, name) in [(802, "XA"), (803, "XB")] {
        let mut instance = Component::new(id, ComponentType::CellInstance, Point::new(80, 80))
            .with_library_cell(LibraryCellInstance::new("work", "afe", "schematic"));
        instance.name = name.to_owned();
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(instance);
    }
    app.state.sync_active_schematic_to_workspace();

    let mut child = SchematicState::default();
    child.document_mut_for_test().components.push(named(
        811,
        ComponentType::VoltageSourceSin,
        "V1",
        "freq=1k",
    ));

    if app
        .state
        .model
        .library_manager
        .get_library("work")
        .is_none()
    {
        app.state
            .model
            .library_manager
            .add_library(Library::new("work"));
    }
    let owner = app
        .state
        .model
        .library_manager
        .get_library_mut("work")
        .expect("the fixture library exists");
    let target = owner.get_or_create_cell("afe");
    if target.get_view("schematic").is_none() {
        target.add_view(View::new("schematic", ViewType::Schematic));
    }
    app.state
        .model
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "afe", "schematic").key(), child);
    app
}

/// The Excitations row counts the design the run drives, and it is the number
/// the page that row opens states above its rows.
///
/// The rail counted the editor's own buffer, so the root of a hierarchical
/// design was offered `1` on the way into a heading reading `DESIGN · 3
/// sources` — two answers to one question, a click apart, with nothing to tell
/// a reader which one the run has. Both are now
/// [`whole_design_source_count`](crate::simulation::placed_sources::whole_design_source_count),
/// and the page lists exactly what that count is the length of
/// (`workbench/surfaces/simulate/pages.rs`, whose own pin asserts the heading
/// text against this design shape).
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_excitations_rail_counts_the_design_that_the_page_it_opens_lists() {
    let mut app = hierarchical_design();
    app.state.workbench.activate(Workspace::Simulate);

    assert_eq!(
        crate::simulation::placed_sources::placed_source_count(&app.state.model.schematic),
        1,
        "the sheet in front of the reader places one supply and two instances"
    );
    let listed = crate::simulation::placed_sources::whole_design_excitations(
        &app.state.model.library_manager,
        &app.state.model.workspace,
        &app.state.model.schematic,
        app.state.model.sim_setup.analysis_plan.as_ref(),
    )
    .0
    .len();
    assert_eq!(
        listed, 3,
        "the run drives the root's supply and one sine source at each occurrence"
    );

    let painted = navigator_painted_lines(228.0, app);
    let row = painted
        .iter()
        .find(|(text, _)| text == "Excitations")
        .map(|(_, rect)| rect.center().y)
        .expect("the rail draws the Excitations route");
    let meta = painted
        .iter()
        .filter(|(text, rect)| text != "Excitations" && (rect.center().y - row).abs() < 4.0)
        .map(|(text, _)| text.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        meta,
        [listed.to_string()],
        "the row states the design's count, which is the page's own"
    );
}

/// The simulation rail derives what it states once per frame.
///
/// The Run set row states the resolved point count, and the Run set card below
/// it used to state it again, each asking the run set for it — two validations
/// of one declaration to print one number twice. The Excitations row states how many
/// sources the whole design places, and resolving that walks the configured
/// hierarchy, resolves each master's nets, parses every source's parameters and
/// sorts the result — nine rows each asking would pay for it nine times.
///
/// Neither is visible in anything the dock paints, which is why it is counted.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_simulation_rail_derives_what_it_states_once_a_frame() {
    use crate::simulation::cost_probe::{Derivation, count, reset};

    let mut app = RSpiceApp::test_instance();
    rspice_app_state::workbench::examples::load_example(
        "Voltage Divider",
        &mut app.state.model.schematic,
    );
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Simulate);

    reset();
    let painted = navigator_painted_lines(228.0, app);
    assert!(
        painted.iter().any(|(text, _)| text == "Setup"),
        "the frame drew the simulation rail"
    );

    assert_eq!(
        count(Derivation::RunSetValidation),
        1,
        "the rail validates its declaration once and reads both numbers off it"
    );
    assert_eq!(
        count(Derivation::PlacedSources),
        1,
        "and resolves the design's excitations once for all nine rows"
    );
}

/// The simulation rail's filter reaches every line it paints.
///
/// It used to reach the nine route rows and stop there. Typing a corner name
/// hid all nine and left the Run set axes, the Variation row and the standing
/// capability notice sitting under a header for a section whose rows had all
/// gone — a rail that answered a search with everything the search was not
/// about.
///
/// Section headers are claims about what is under them, so one that retains no
/// row goes with its rows. The creating action at the foot of the tree is not
/// a datum and stays: it is how an empty plan is filled, including the plan a
/// filter has emptied on screen.
#[test]
fn the_simulate_rail_filter_reaches_past_its_page_rows() {
    fn rail(query: &str) -> Vec<String> {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Simulate);
        // Only an axis the run sweeps has a row.
        for dimension in &mut app.state.model.sim_setup.run_set.dimensions {
            dimension.enabled = true;
        }
        app.state.workbench.set_navigator_filter(query);
        // The rail, not the dock's own header band above it: that names the
        // panel and is no more a filtered row than the title bar is.
        navigator_painted_lines(228.0, app)
            .into_iter()
            .filter(|(_, rect)| rect.top() >= rspice_ui_kit::panels::PANEL_HEADER_H)
            .map(|(text, _)| text)
            .collect()
    }

    let unfiltered = rail("");
    for line in ["Setup", "Analyses", "Run set", "Supply voltage"] {
        assert!(
            unfiltered.iter().any(|painted| painted.as_str() == line),
            "an unfiltered rail still paints {line:?}; it painted {unfiltered:?}"
        );
    }

    // Matches one axis by name and nothing else the rail holds.
    let filtered = rail("supply");
    assert!(
        filtered
            .iter()
            .any(|painted| painted.as_str() == "Supply voltage"),
        "the axis whose name matches survives; the rail painted {filtered:?}"
    );
    assert!(
        filtered.iter().any(|painted| painted.as_str() == "Run set"),
        "and the section that kept a row keeps its header; it painted {filtered:?}"
    );
    for dropped in ["Analyses", "Setup"] {
        assert!(
            !filtered.iter().any(|painted| painted.as_str() == dropped),
            "{dropped:?} matches nothing the reader typed; the rail painted {filtered:?}"
        );
    }
    assert!(
        filtered
            .iter()
            .any(|painted| painted.starts_with("Add analysis")),
        "the panel's own creating action is not a datum the filter can hide; \
         the rail painted {filtered:?}"
    );
}

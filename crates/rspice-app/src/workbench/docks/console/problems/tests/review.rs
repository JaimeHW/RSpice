//! Renders of the Problems page in its dock, for a person to look at: the
//! states a reader meets, at the sizes a dock is actually given.

use super::*;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::waivers::WaiverAction;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::ConsolePage;

/// The dock sizes reviewed: a full-width dock dragged down to three rows, an
/// ordinary laptop dock, and a narrow window's.
const SIZES: [(f32, f32); 3] = [(1886.0, 146.0), (1280.0, 320.0), (760.0, 240.0)];

/// The three findings of a drawing with one stray label and no ground: a
/// label on no wire, an off-sheet connector with no partner, and the missing
/// ground.
fn three_findings(state: &mut AppState) {
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 40));
    state.sync_active_schematic_to_workspace();
    let label = DrcLocation::NetLabel {
        name: "test".to_owned(),
    };
    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        1,
        DrcViolationType::OrphanNetLabel,
        "Net label 'test' at (80, 120) is not on a wire or terminal",
        label.clone(),
    ));
    result.add_violation(DrcViolation::new(
        2,
        DrcViolationType::OffSheetConnectorWithoutPartner,
        "Off-sheet connector 'test' has no partner on another sheet",
        label,
    ));
    result.add_violation(DrcViolation::new(
        3,
        DrcViolationType::MissingGround,
        "Circuit has no ground reference (node 0 or GND)",
        DrcLocation::Global,
    ));
    result.completed = true;
    state
        .publish_active_design_check_result(result)
        .expect("the fixture publishes a current receipt");
}

/// The listed finding of `code`.
fn finding(state: &AppState, code: &str) -> uuid::Uuid {
    let listed = listing(state);
    (0..listed.len())
        .filter_map(|index| listed.facet(index))
        .find(|facet| facet.code == code)
        .map(|facet| facet.id)
        .unwrap_or_else(|| panic!("{code} is listed"))
}

/// A symbol's pin finding whose symbol has since gone from the library, so
/// whether it still stands cannot be evaluated.
fn unreadable_symbol_finding(state: &mut AppState) {
    use crate::state::{
        Cell, CellViewRef, Library, PortDirection, SymbolDocument, SymbolPin, View, ViewType,
    };
    let schematic = state.workspace.content.active_view.clone();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);
    let symbol = CellViewRef::new("work", "amp", "symbol");
    state.open_workspace_view(symbol.clone());
    state
        .store_active_symbol_document(&SymbolDocument {
            pins: vec![SymbolPin::new("IN", PortDirection::In, None)],
            ..SymbolDocument::default()
        })
        .expect("the fixture stores its symbol");
    let message = "Pin 'IN' has no port of that name in work/amp/schematic";
    let record = DiagnosticRecord::new(
        rspice_app_types::diagnostics::CanonicalDiagnosticMetadata::current(
            "rspice.design.symbol.pins",
            "SYM-PIN-WITHOUT-PORT",
            "work/amp/symbol",
            None,
            0,
            "validation",
            message,
        ),
        DiagnosticLevel::Warning,
        DiagnosticSubject::new(state.workspace.content.project.id(), "Pin 'IN'"),
        message,
    );
    state
        .publish_diagnostics(
            DiagnosticInputs::SymbolPins { subject: symbol },
            vec![record],
        )
        .expect("the symbol is readable when its pins are checked");
    state.open_workspace_view(schematic);
    let _ = state.library_manager.remove_library("work");
}

fn dock_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.console_visible = true;
    app.state.workbench.console_page = ConsolePage::Problems;
    app
}

/// Every reviewed state, each a session ready to draw.
fn states() -> Vec<(&'static str, RSpiceApp)> {
    let mut states = Vec::new();

    let mut three = dock_app();
    three_findings(&mut three.state);
    states.push(("three", three));

    let mut selected = dock_app();
    three_findings(&mut selected.state);
    let info = finding(&selected.state, "DRC-OFF-SHEET-CONNECTOR-WITHOUT-PARTNER");
    selected.state.diagnostics.set_current_finding(Some(info));
    states.push(("selected", selected));

    let mut grouped = dock_app();
    three_findings(&mut grouped.state);
    grouped.state.problems_view.grouping = ProblemGrouping::Code;
    states.push(("grouped", grouped));

    let mut waived = dock_app();
    three_findings(&mut waived.state);
    let label = finding(&waived.state, "DRC-ORPHAN-NET-LABEL");
    let target = {
        let listed = listing(&waived.state);
        (0..listed.len())
            .filter_map(|index| listed.row(index))
            .find(|row| row.id == label)
            .and_then(|row| match row.waiver_action {
                Some(WaiverAction::Waive(target)) => Some(target.clone()),
                _ => None,
            })
            .expect("a stray label may be waived")
    };
    waived
        .state
        .waive_finding(&target, "probe point, left open on purpose")
        .expect("the waiver is recorded");
    states.push(("waived", waived));

    let mut unavailable = dock_app();
    unreadable_symbol_finding(&mut unavailable.state);
    three_findings(&mut unavailable.state);
    states.push(("unavailable", unavailable));

    states.push(("empty", dock_app()));
    states
}

/// `RSPICE_REVIEW_DIR` says where the renders go.
#[test]
#[ignore = "writes review renders for a person to look at"]
fn problems_page_review_renders() {
    let directory = std::env::var_os("RSPICE_REVIEW_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    for (name, mut app) in states() {
        for (width, height) in SIZES {
            app.state.workbench.console_height = height;
            let layout = LayoutSpec::resolve(width, 900.0, &app.state.workbench);
            let size = egui::vec2(width, height);
            // As the shell draws the dock: its panel memory is reconciled
            // with the stored height before the panel is built.
            let canvas = crate::ui::raster::render(size, |ui, _| {
                crate::workbench::docks::synchronize_panel_memory(ui.ctx(), &app, layout);
                crate::workbench::docks::show_console(ui, &mut app, layout);
            });
            let path = directory.join(format!("{name}-{width}x{height}.png"));
            std::fs::write(&path, canvas.png(size.y as usize))
                .expect("the review render is written");
        }
    }
}

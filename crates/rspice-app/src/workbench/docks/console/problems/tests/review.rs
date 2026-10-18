//! Renders of the Problems page in its dock, for a person to look at: the
//! states a reader meets, at the sizes a dock is actually given.

use super::*;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::waivers::WaiverAction;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::ConsolePage;

/// The dock sizes reviewed: a full-width dock dragged down to three rows, an
/// ordinary laptop dock, a narrow window's, and the two heights at which the
/// selected finding's last line used to be cut at the dock's bottom edge.
const SIZES: [(f32, f32); 5] = [
    (1886.0, 146.0),
    (1280.0, 320.0),
    (760.0, 240.0),
    (1886.0, 180.0),
    (1280.0, 200.0),
];

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

    // The missing ground: a message and a remedy line under it.
    let mut ground = dock_app();
    three_findings(&mut ground.state);
    let missing = finding(&ground.state, "DRC-MISSING-GROUND");
    ground.state.diagnostics.set_current_finding(Some(missing));
    states.push(("ground", ground));

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

/// A retained AC run with three measurements, made the active result, and
/// a line in the session log: what the dock's other pages list.
fn measured_run(state: &mut AppState) {
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![
        rspice_core::MeasureResult::success("gain", 42.0),
        rspice_core::MeasureResult::success("phase_margin", 61.5),
        rspice_core::MeasureResult::success("bandwidth", 1.25e6),
    ]);
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.simulation.retained.runs.push(run);
    state.simulation.view.active_run_idx = Some(0);
    state
        .log_buffer
        .warning(crate::diagnostics::LogSource::Run, "a session warning");
}

/// Every page of the dock, for the toolbar and table they share: each drawn
/// over the same session, so moving between them is all that changes.
fn pages() -> Vec<(&'static str, RSpiceApp)> {
    [
        ("page-console", ConsolePage::Console),
        ("page-problems", ConsolePage::Problems),
        ("page-measurements", ConsolePage::Measurements),
        ("page-tasklog", ConsolePage::TaskLog),
    ]
    .into_iter()
    .map(|(name, page)| {
        let mut app = dock_app();
        measured_run(&mut app.state);
        three_findings(&mut app.state);
        app.state.workbench.console_page = page;
        (name, app)
    })
    .collect()
}

/// At every dock height from the least a reader can drag it to, nothing the
/// page draws is cut in half by an edge: a line of text or a control shows
/// whole or not at all, and nothing is drawn into the dock's bottom inset.
/// The missing-ground finding is selected, so the region under the table
/// holds a message with its remedy under it — the line the dock's bottom
/// edge used to cut.
#[test]
fn nothing_in_the_dock_is_cut_at_any_height() {
    fn walk(
        shape: &egui::epaint::Shape,
        clip: egui::Rect,
        out: &mut Vec<(String, egui::Rect, egui::Rect)>,
    ) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
                clip,
            )),
            egui::epaint::Shape::Rect(rect) if rect.stroke.width > 0.0 => {
                out.push(("a control's frame".to_owned(), rect.rect, clip));
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, out);
                }
            }
            _ => {}
        }
    }

    let mut app = dock_app();
    measured_run(&mut app.state);
    three_findings(&mut app.state);
    let missing = finding(&app.state, "DRC-MISSING-GROUND");
    app.state.diagnostics.set_current_finding(Some(missing));
    let width = 1280.0;
    let default = crate::workbench::state::WorkbenchState::default().console_height;
    let pages = [
        ConsolePage::Problems,
        ConsolePage::Console,
        ConsolePage::Measurements,
        ConsolePage::TaskLog,
    ];
    // Every height for this page; a sample of them for the others, which
    // share its toolbar and its bottom inset.
    for (page, height) in pages.into_iter().flat_map(|page| {
        let step = if page == ConsolePage::Problems { 1 } else { 5 };
        (90_u16..=260)
            .step_by(step)
            .map(move |height| (page, f32::from(height)))
    }) {
        app.state.workbench.console_page = page;
        app.state.workbench.console_height = height;
        let layout = LayoutSpec::resolve(width, 900.0, &app.state.workbench);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let size = egui::vec2(width, height);
        let mut output = None;
        for _ in 0..3 {
            output = Some(
                ctx.run_ui(crate::ui::raster::display_input(size, 1.0), |ui| {
                    crate::workbench::docks::synchronize_panel_memory(ui.ctx(), &app, layout);
                    crate::workbench::docks::show_console(ui, &mut app, layout);
                }),
            );
        }
        let mut drawn = Vec::new();
        for clipped in &output.expect("the dock was drawn").shapes {
            walk(&clipped.shape, clipped.clip_rect, &mut drawn);
        }
        let inset = height - super::super::super::CONSOLE_BODY_PADDING_BOTTOM;
        let mut remedy_shown = false;
        let mut whole: Vec<&str> = Vec::new();
        for (what, rect, clip) in &drawn {
            if rect.width() > width * 0.5 {
                continue;
            }
            let shown = rect.intersect(*clip);
            if shown.height() <= 0.5 || shown.width() <= 0.0 {
                continue;
            }
            assert!(
                rect.top() >= clip.top() - 0.5 && rect.bottom() <= clip.bottom() + 0.5,
                "{page:?} at {height} px: {what:?} at {rect:?} is cut by its clip {clip:?}",
            );
            assert!(
                rect.bottom() <= inset + 0.5,
                "{page:?} at {height} px: {what:?} at {rect:?} runs into the bottom inset at {inset}",
            );
            remedy_shown |= what == "Add a ground symbol (GND) to the circuit";
            whole.push(what);
        }
        if page == ConsolePage::Problems && height >= 200.0 {
            assert!(
                remedy_shown,
                "at {height} px the remedy shows under the message"
            );
        }
        // The dock opens tall enough for the toolbar, the header, three whole
        // rows and the selected finding's message and remedy in the region
        // under them, and no taller: a point less does not hold them.
        let rows = [
            "Circuit has no ground reference (node 0 or GND)",
            "Net label 'test' at (80, 120) is not on a wire or terminal",
            "Off-sheet connector 'test' has no partner on another sheet",
        ]
        .iter()
        .filter(|message| whole.contains(message))
        .count();
        let detail = whole
            .iter()
            .filter(|text| **text == "Circuit has no ground reference (node 0 or GND)")
            .count()
            == 2;
        let holds = rows == 3 && detail && remedy_shown;
        if page == ConsolePage::Problems && height == default {
            assert!(
                holds,
                "at the default {default} px the dock shows {rows} whole rows, the selected \
                 finding's message: {detail}, its remedy: {remedy_shown}",
            );
        }
        if page == ConsolePage::Problems && height == default - 1.0 {
            assert!(
                !holds,
                "{height} px already holds them: the default can be lower"
            );
        }
    }
}

/// `RSPICE_REVIEW_DIR` says where the renders go.
#[test]
#[ignore = "writes review renders for a person to look at"]
fn problems_page_review_renders() {
    let directory = std::env::var_os("RSPICE_REVIEW_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let default = crate::workbench::state::WorkbenchState::default().console_height;
    let renders = states()
        .into_iter()
        .map(|(name, app)| (name, app, SIZES.to_vec()))
        .chain(
            pages()
                .into_iter()
                .map(|(name, app)| (name, app, vec![(1280.0, 320.0), (1280.0, default)])),
        );
    for (name, mut app, sizes) in renders {
        for (width, height) in sizes {
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

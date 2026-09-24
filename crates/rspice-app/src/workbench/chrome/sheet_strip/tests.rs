//! What the strip promises: one chip per sheet, hidden when there is only one
//! drawing, and painted directly beneath the document row.
//!
//! The renders below allocate the document bar and then the strip, which is
//! the order `frame::show` allocates them in. egui subtracts each top panel's
//! edge from the remaining rect in call order, so that sequence *is* the
//! vertical arrangement.

#![cfg(not(target_arch = "wasm32"))]

use egui::{Context, Id, RawInput, Rect, Vec2, containers::panel::PanelState};

use super::*;
use crate::state::{
    Component, ComponentType, Point, SheetDefinition, SheetPortPolicy, SheetTemplate,
};
use crate::workbench::state::Workspace;

const VIEWPORT: Vec2 = Vec2::new(1_440.0, 900.0);

fn app_with_sheets(count: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.components = vec![Component::new(
        10,
        ComponentType::Resistor,
        Point::new(20, 20),
    )];
    let key = app.state.workspace.active_schematic_reference().key();
    let first = app
        .state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Input stage", [10])
        .expect("first sheet");
    let catalog = app
        .state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    for ordinal in 2..=count {
        catalog
            .create_sheet(
                SheetDefinition {
                    name: format!("Sheet {ordinal}"),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(ordinal as u32),
                },
                None,
            )
            .expect("further sheet");
    }
    catalog.set_active(first).expect("active sheet");
    app
}

fn chrome_rows(ui: &mut Ui, app: &mut RSpiceApp) {
    let layout = LayoutSpec::resolve(VIEWPORT.x, VIEWPORT.y, &app.state.workbench);
    super::super::document_bar::show(ui, app, layout);
    show(ui, app, layout);
}

fn frame_input() -> RawInput {
    RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, VIEWPORT)),
        ..Default::default()
    }
}

#[test]
fn one_sheet_is_a_drawing_rather_than_a_set_of_them() {
    assert!(!is_visible(&app_with_sheets(1).state));
    assert!(is_visible(&app_with_sheets(2).state));
}

#[test]
fn every_sheet_gets_one_tab_inside_the_strip_tab_list() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = app_with_sheets(3);

    let output = ctx.run_ui(frame_input(), |ui| chrome_rows(ui, &mut app));
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("accessibility tree")
        .nodes;

    let tab_list = nodes
        .iter()
        .find(|(_, node)| node.label() == Some("Sheets in this drawing"))
        .expect("the strip exposes one tab list");
    assert_eq!(
        tab_list.1.role(),
        egui::accesskit::Role::TabList,
        "sheets are navigation, so the strip is a tab list"
    );
    let chips = nodes
        .iter()
        .filter(|(_, node)| {
            node.description()
                .is_some_and(|description| description.starts_with("Sheet "))
        })
        .collect::<Vec<_>>();
    assert_eq!(chips.len(), 3, "one chip per governed sheet");
    assert!(
        chips
            .iter()
            .all(|(_, node)| node.role() == egui::accesskit::Role::Tab)
    );
    assert_eq!(
        chips
            .iter()
            .filter(|(_, node)| node.is_selected() == Some(true))
            .count(),
        1,
        "exactly one chip carries the active sheet"
    );
}

#[test]
fn a_single_sheet_drawing_paints_no_strip_at_all() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = app_with_sheets(1);

    let output = ctx.run_ui(frame_input(), |ui| chrome_rows(ui, &mut app));
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("accessibility tree")
        .nodes;

    assert!(
        !nodes
            .iter()
            .any(|(_, node)| node.label() == Some("Sheets in this drawing")),
        "a drawing with one sheet has nothing to navigate between"
    );
}

#[test]
fn the_strip_hangs_directly_from_the_document_row() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = app_with_sheets(2);

    // Panel rectangles are stored at the end of the pass that allocates them,
    // so the first pass is what makes them readable.
    for _ in 0..2 {
        let _ = ctx.run_ui(frame_input(), |ui| chrome_rows(ui, &mut app));
    }

    let document = PanelState::load(&ctx, Id::new("workbench.document_bar"))
        .expect("the document bar is allocated")
        .outer_rect;
    let strip = PanelState::load(&ctx, Id::new("workbench.sheet_strip"))
        .expect("the sheet strip is allocated")
        .outer_rect;

    assert!(
        (strip.top() - document.bottom()).abs() < 0.5,
        "the strip starts where the document row ends: {strip:?} under {document:?}"
    );
    assert!(
        strip.height() > 0.0 && strip.height() < document.height(),
        "the strip is the shorter secondary row: {strip:?}"
    );
}

#[test]
fn the_active_chip_carries_the_accent_edge_the_document_row_uses() {
    let mut app = app_with_sheets(2);
    let mut band = Rect::NOTHING;
    let canvas = crate::ui::raster::render(VIEWPORT, |ui, _| {
        chrome_rows(ui, &mut app);
        if let Some(state) = PanelState::load(ui.ctx(), Id::new("workbench.sheet_strip")) {
            band = state.outer_rect;
        }
    });
    assert!(band.height() > 0.0, "the strip was never allocated");

    let accent = {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        crate::ui::tokens::Tokens::get(&ctx).color.accent
    };
    let edge = Rect::from_min_max(
        band.left_top(),
        egui::pos2(band.right(), band.top() + CHIP_ACTIVE_EDGE),
    );
    let mut edge_pixels = canvas.pixels_in(edge).peekable();
    assert!(edge_pixels.peek().is_some(), "the edge band is off canvas");
    assert!(
        edge_pixels.any(|pixel| pixel == accent),
        "the active chip paints the document row's accent edge"
    );
    assert!(
        canvas
            .pixels_in(band)
            .any(|pixel| pixel != canvas.background()),
        "the strip painted nothing at all"
    );
}

#[test]
fn a_strip_action_routes_straight_to_the_sheet_actions() {
    let mut app = app_with_sheets(2);
    let entries = sheet_actions::sheet_entries(&app.state);
    app.state.schematic.selection.select_component(10);

    apply(&mut app, StripAction::Activate(entries[1].id), &entries);

    assert_eq!(
        sheet_actions::active_sheet_id(&app.state),
        Some(entries[1].id)
    );
    assert!(app.state.schematic.selection.is_empty());

    apply(
        &mut app,
        StripAction::Reorder {
            sheet: entries[1].id,
            to_index: 0,
        },
        &entries,
    );
    assert_eq!(
        sheet_actions::sheet_entries(&app.state)[0].id,
        entries[1].id,
        "the strip's order follows the published catalog order"
    );
}

//! Renders of the placement forms, for a human to look at.
//!
//! Each of these forms was a 760 pt two-pane mockup frame with one to four
//! real fields in the corner of it. The renders are how the replacement is
//! checked: a card the size of its questions, nothing clipped, and no band
//! left empty where a deleted section used to be.

use crate::state::{Bus, BusDeclaration, DesignNoteKind, Point, Tool};
use crate::ui::Theme;
use crate::ui::tokens::Mode;
use crate::workbench::RSpiceApp;
use crate::workbench::app::schematic_editor_request_source;
use crate::workbench::app_state::AppState;

/// The viewports every form is rendered at: a desktop, the split breakpoint,
/// and a portrait phone.
const VIEWPORTS: [(&str, egui::Vec2); 3] = [
    ("1024x640", egui::vec2(1024.0, 640.0)),
    ("760x900", egui::vec2(760.0, 900.0)),
    ("390x844", egui::vec2(390.0, 844.0)),
];

fn raster_directory() -> std::path::PathBuf {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    directory
}

/// Draw one declared bus, so the bus-tap form has something to prefill from.
fn declare_bus(state: &mut AppState, declaration: &str) {
    let id = state.schematic.next_id();
    let declaration = BusDeclaration::parse(declaration).expect("declaration");
    state.schematic.document_mut_for_test().buses.push(
        Bus::segment(id, Point::new(0, 0), Point::new(40, 0), Some(declaration))
            .expect("bus segment"),
    );
}

/// Every placement form, in each of its reviewable states, at every viewport,
/// in both themes.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_placement_forms() {
    use std::io::Write as _;

    type Open = fn(&mut RSpiceApp);
    type Draw = fn(&mut RSpiceApp, &egui::Context);
    let forms: [(&str, Open, Draw); 7] = [
        (
            "bus-tap-empty",
            |app| super::bus_tap::open_bus_tap(&mut app.state),
            |app, ctx| app.render_bus_tap_dialog(ctx),
        ),
        (
            "bus-tap-resolved",
            |app| {
                declare_bus(&mut app.state, "DATA[15:0]");
                super::bus_tap::open_bus_tap(&mut app.state);
                app.state.dialogs.bus_tap.fields.slice = "DATA[7:0]".to_owned();
            },
            |app, ctx| app.render_bus_tap_dialog(ctx),
        ),
        (
            "bus-tap-refused",
            |app| {
                super::bus_tap::open_bus_tap(&mut app.state);
                app.state.dialogs.bus_tap.fields.bus = "DATA[15:0]".to_owned();
                app.state.dialogs.bus_tap.fields.slice = "ADDR[7:0]".to_owned();
            },
            |app, ctx| app.render_bus_tap_dialog(ctx),
        ),
        (
            "design-note",
            |app| {
                let source = schematic_editor_request_source(&app.state);
                app.state.dialogs.design_note.open(source);
                app.state.dialogs.design_note.kind = DesignNoteKind::ReviewNote;
                app.state.dialogs.design_note.text =
                    "Check the start-up path before tape-out".to_owned();
            },
            |app, ctx| app.render_design_note_dialog(ctx),
        ),
        (
            "design-note-empty",
            |app| {
                let source = schematic_editor_request_source(&app.state);
                app.state.dialogs.design_note.open(source);
            },
            |app, ctx| app.render_design_note_dialog(ctx),
        ),
        (
            "documentation-shape",
            |app| {
                let source = schematic_editor_request_source(&app.state);
                let shapes = app.state.schematic.document().documentation_shapes.clone();
                app.state.dialogs.documentation_shape.open(source, shapes);
            },
            |app, ctx| app.render_documentation_shape_dialog(ctx),
        ),
        (
            "builtin-xspice",
            |app| {
                let descriptor = crate::state::engine_only_xspice_devices()
                    .iter()
                    .find(|descriptor| descriptor.model_type == "d_lut")
                    .expect("lookup-table descriptor");
                let ports =
                    crate::state::builtin_xspice_vector_ports(descriptor).expect("vector ports");
                let source = schematic_editor_request_source(&app.state);
                app.state.dialogs.builtin_xspice_placement.open(
                    descriptor.stable_id,
                    descriptor.display_name,
                    ports,
                    source,
                );
            },
            |app, ctx| app.render_builtin_xspice_placement_dialog(ctx),
        ),
    ];

    let directory = raster_directory();
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    for (stem, open, draw) in forms {
        for (size_stem, size) in VIEWPORTS {
            for (mode_stem, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
                let mut app = RSpiceApp::test_instance();
                open(&mut app);
                let canvas = crate::ui::raster::render_themed(
                    Theme {
                        mode,
                        ..Theme::default()
                    },
                    size,
                    |ui, _background| draw(&mut app, ui.ctx()),
                );
                let height = canvas.content_height().max(1);
                let path = directory.join(format!("{stem}-{size_stem}-{mode_stem}.png"));
                std::fs::write(&path, canvas.png(height)).expect("write form render");
                writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                    .expect("write raster qualification report");
            }
        }
    }
}

/// The net-label and off-sheet entries, where the click put them.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_net_label_entry() {
    use std::io::Write as _;

    let directory = raster_directory();
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    let cases: [(&str, fn(&mut RSpiceApp)); 3] = [
        ("net-label-empty", |app| {
            app.state.schematic.session.editor.tool = Tool::Label;
            crate::workbench::app::open_net_label_placement(&mut app.state, Point::new(30, 50));
        }),
        ("net-label-refused", |app| {
            app.state.schematic.session.editor.tool = Tool::Label;
            crate::workbench::app::open_net_label_placement(&mut app.state, Point::new(30, 50));
            app.state.dialogs.net_label_placement.name = "two nodes".to_owned();
        }),
        ("off-sheet-connector", |app| {
            app.state.schematic.session.editor.tool = Tool::OffSheetConnector;
            crate::workbench::app::open_net_label_placement(&mut app.state, Point::new(30, 50));
            app.state.dialogs.net_label_placement.name = "BIAS".to_owned();
        }),
    ];
    for (stem, open) in cases {
        for (size_stem, size) in VIEWPORTS {
            for (mode_stem, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
                let mut app = RSpiceApp::test_instance();
                open(&mut app);
                // Where the click landed: the entry takes its place from the
                // pointer, so the render has to have one.
                let pointer = egui::pos2(size.x * 0.4, size.y * 0.35);
                let canvas = crate::ui::raster::render_with_pointer_themed(
                    Theme {
                        mode,
                        ..Theme::default()
                    },
                    size,
                    pointer,
                    |ui, _background| app.render_net_label_dialog(ui.ctx()),
                );
                let height = canvas.content_height().max(1);
                let path = directory.join(format!("{stem}-{size_stem}-{mode_stem}.png"));
                std::fs::write(&path, canvas.png(height)).expect("write entry render");
                writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                    .expect("write raster qualification report");
            }
        }
    }
}

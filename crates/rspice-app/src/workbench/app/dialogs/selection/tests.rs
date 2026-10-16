//! What the selection commands and the object-properties form may not say.
//!
//! These scan the shipped half of this lane's own files. They are cheap to
//! keep and they catch the one failure mode a reviewer cannot see: a pattern
//! this sweep removed coming back a surface at a time.

/// Every shipped file this lane owns, as its name and its source.
fn slice() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "selection/array.rs",
            crate::source_guard::production_source(include_str!("array.rs")),
        ),
        (
            "selection/bulk_edit.rs",
            crate::source_guard::production_source(include_str!("bulk_edit.rs")),
        ),
        (
            "selection/commands.rs",
            crate::source_guard::production_source(include_str!("commands.rs")),
        ),
        (
            "selection/move_selection.rs",
            crate::source_guard::production_source(include_str!("move_selection.rs")),
        ),
        (
            "selection/rename.rs",
            crate::source_guard::production_source(include_str!("rename.rs")),
        ),
        (
            "selection/stretch.rs",
            crate::source_guard::production_source(include_str!("stretch.rs")),
        ),
        (
            "object_properties.rs",
            crate::source_guard::production_source(include_str!("../object_properties.rs")),
        ),
        (
            "surfaces/design/tool_options.rs",
            crate::source_guard::production_source(include_str!(
                "../../../surfaces/design/tool_options.rs"
            )),
        ),
    ]
}

/// Colour on these surfaces is reserved for a problem's own mark. Status has
/// no tone of its own, and nothing on them is green.
#[test]
fn no_selection_surface_paints_status_in_colour() {
    for (name, source) in slice() {
        for pattern in [
            "colored_label",
            "status_dot(",
            "StatusMark::Success",
            "StatusMark::Neutral",
            ".color(t.color.ok",
            ".color(t.color.info",
            ".color(t.color.warn",
            "color.ok)",
        ] {
            assert!(
                !source.contains(pattern),
                "{name} paints status with {pattern}"
            );
        }
    }
}

/// Headers and labels are sentence case; nothing here shouts.
#[test]
fn no_selection_surface_upper_cases_a_label() {
    for (name, source) in slice() {
        for pattern in ["to_uppercase", "to_ascii_uppercase"] {
            assert!(!source.contains(pattern), "{name} still up-cases a label");
        }
    }
}

/// A plan or spec ID is a note to the people building the product. None of
/// them reaches the product, and neither does the internal ID of an object
/// the reader selected by clicking on it.
#[test]
fn no_selection_surface_shows_an_internal_identifier() {
    for (name, source) in slice() {
        for pattern in [
            "SEL-06",
            "stable ID",
            "COMPONENT-{",
            "BUS-{",
            "LABEL-{",
            "NOTE-{",
            "SHAPE-{",
            "TAP-{",
            "naming authorit",
        ] {
            assert!(
                !source.contains(pattern),
                "{name} shows the internal identifier {pattern}"
            );
        }
    }
}

/// Success is the absence of a problem. None of these surfaces reports that
/// what it just did worked, and none of them describes its own machinery.
#[test]
fn no_selection_surface_reports_its_own_success_or_internals() {
    for (name, source) in slice() {
        for pattern in [
            "toasts.success",
            "legal preview",
            "scope resolved",
            "one semantic undo record",
            "undoable transaction",
            "Undo boundary",
            "governed",
        ] {
            assert!(!source.contains(pattern), "{name} still says {pattern:?}");
        }
    }
}

/// Move and Stretch act on the canvas. Nothing in this lane opens a modal in
/// front of them again.
#[test]
fn move_and_stretch_open_no_dialog() {
    for (name, source) in slice() {
        for pattern in [
            "render_move_selection_dialog",
            "render_stretch_selection_dialog",
            "selection_command_workflow",
            "schematic_command_workflow",
        ] {
            assert!(!source.contains(pattern), "{name} still builds {pattern}");
        }
    }
}

// ============================================================================
// Renders for a human to look at
// ============================================================================

/// Every form this lane still owns, at three viewports, in both themes.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_selection_forms() {
    use crate::state::{Bus, BusDeclaration, ComponentType, Point};
    use crate::ui::tokens::Mode;
    use crate::workbench::app::RSpiceApp;
    use crate::workbench::app_state::AppState;
    use std::io::Write as _;

    fn with_a_selected_resistor(state: &mut AppState) {
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
    }

    fn with_a_selected_bus(state: &mut AppState) {
        state.schematic.document_mut_for_test().buses.push(
            Bus::segment(
                1,
                Point::new(0, 0),
                Point::new(40, 0),
                Some(BusDeclaration::parse("DATA[15:0]").unwrap()),
            )
            .unwrap(),
        );
        state.schematic.session.editor.selection.select_only_bus(1);
    }

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    for stem in [
        "create-array",
        "rename",
        "bulk-edit",
        "object-properties-bus",
    ] {
        for (size_stem, size) in [
            ("1024x640", egui::vec2(1024.0, 640.0)),
            ("760x900", egui::vec2(760.0, 900.0)),
            ("390x844", egui::vec2(390.0, 844.0)),
        ] {
            for (mode_stem, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
                let mut app = RSpiceApp::test_instance();
                match stem {
                    "create-array" => {
                        with_a_selected_resistor(&mut app.state);
                        super::array::open_array_selection_dialog(&mut app.state);
                    }
                    "rename" => {
                        with_a_selected_resistor(&mut app.state);
                        super::rename::open_selected_object_rename(&mut app.state);
                    }
                    "bulk-edit" => {
                        with_a_selected_resistor(&mut app.state);
                        super::bulk_edit::open_selection_bulk_edit_dialog(&mut app.state);
                    }
                    _ => {
                        with_a_selected_bus(&mut app.state);
                        crate::workbench::app::open_selected_object_properties(&mut app.state);
                    }
                }
                let canvas = crate::ui::raster::render_themed(
                    crate::ui::Theme {
                        mode,
                        ..crate::ui::Theme::default()
                    },
                    size,
                    |ui, _background| match stem {
                        "create-array" => app.render_array_selection_dialog(ui.ctx()),
                        "rename" => app.render_rename_selection_dialog(ui.ctx()),
                        "bulk-edit" => app.render_selection_bulk_edit_dialog(ui.ctx()),
                        _ => app.render_object_properties_dialog(ui.ctx()),
                    },
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

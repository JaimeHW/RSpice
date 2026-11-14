//! The engineering table drawn through a whole application.

use crate::state::EngineeringTableView;
use crate::state::EngineeringViewScope;
use crate::state::engineering_table::active_schematic_dataset;
use crate::state::{ComponentType, Point};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::engineering_table::tests::table;
use crate::workbench::app::dialogs::engineering_table::*;

/// The pages that ask before they act are command forms: a card of the
/// form's width, every label on one edge and every control on one grid.
/// They were raw combo boxes with their labels after them, labels above
/// fields, and a row of facts in a second grid.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_pages_that_ask_first_are_command_forms() {
    for (page, labels, width) in [
        (
            EngineeringTableDialogPage::SaveView,
            &["Name", "Scope", "Default"][..],
            420.0,
        ),
        (
            EngineeringTableDialogPage::Export,
            &["Grid", "Scope", "Format", "Include"][..],
            480.0,
        ),
        (
            EngineeringTableDialogPage::RowDetails,
            &["Source", "Source revision", "Field"][..],
            480.0,
        ),
    ] {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        assert!(open_engineering_table_dialog(
            &mut app.state.design.engineering_table,
            &app.state.design.engineering_table_views,
            &app.state.model
        ));
        let dataset = active_schematic_dataset(&app.state.model.schematic);
        let dialog = &mut app.state.design.engineering_table;
        dialog.page = page;
        dialog.save_name = "Review".to_owned();
        dialog.row_details_id = Some(dataset.rows[0].stable_id.clone());
        let painted = crate::ui::widgets::painted_runs::painted_runs(
            egui::vec2(1440.0, 900.0),
            crate::ui::tokens::Mode::Light,
            3,
            |ctx| table(&mut app).render_engineering_table_dialog(ctx),
        );
        assert_eq!(painted.surface.width(), width, "{page:?}");
        let left = |text: &str| {
            painted
                .rect_of(text)
                .unwrap_or_else(|| panic!("{page:?}: {text} is painted"))
                .left()
        };
        for label in labels {
            assert!(
                (left(label) - left(labels[0])).abs() < 0.5,
                "{page:?}: {label} starts at {}, {} at {}",
                left(label),
                labels[0],
                left(labels[0])
            );
        }
    }
}

#[test]
fn project_scope_is_versioned_while_personal_scope_is_not() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let dataset = active_schematic_dataset(&app.state.model.schematic);
    app.state.design.engineering_table.open(
        EngineeringTableView::for_dataset(&dataset),
        dataset.source_revision,
    );
    app.state.design.engineering_table.save_name = "Personal review".to_owned();
    app.state.design.engineering_table.save_scope = EngineeringViewScope::Personal;
    table(&mut app).save_engineering_table_view(&dataset);
    assert!(!app.state.model.workspace.content.project_metadata_dirty);

    app.state.design.engineering_table.save_name = "Project review".to_owned();
    app.state.design.engineering_table.save_scope = EngineeringViewScope::Project;
    table(&mut app).save_engineering_table_view(&dataset);
    assert!(app.state.model.workspace.content.project_metadata_dirty);
}

/// The manager offers what a reader configures, and its table fits it:
/// no grid id, render window, pixel widths, pin, renderer policies or
/// keyboard line, and every column under its heading inside the
/// dialog. The table's columns ran past the dialog's edge behind a
/// sideways scroll, each value centred in a button under a heading
/// laid out apart from it.
#[test]
fn the_manager_offers_what_a_reader_configures_and_its_table_fits() {
    let mut app = RSpiceApp::test_instance();
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    assert!(open_engineering_table_dialog(
        &mut app.state.design.engineering_table,
        &app.state.design.engineering_table_views,
        &app.state.model
    ));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut shapes = Vec::new();
    for _ in 0..4 {
        shapes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| table(&mut app).render_engineering_table_dialog(ui.ctx()),
            )
            .shapes;
    }
    let texts = shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let at = |wanted: &str| {
        texts
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{wanted:?} was not painted: {texts:?}"))
    };
    for wanted in ["Columns", "Sort", "Filter", "Saved view"] {
        let _ = at(wanted);
    }
    for (text, _) in &texts {
        for gone in [
            "px",
            "Active grid",
            "logical rows",
            "pinned",
            "Large-table rendering",
            "Overscan",
            "Frozen identifiers",
            "Keyboard",
        ] {
            assert!(!text.contains(gone), "{text:?} is still painted");
        }
    }
    let title = at("Engineering table view");
    // The footer's last button ends its padding past its label: the
    // dialog's content edge, which no cell may cross.
    let edge = at("Apply table view").right() + 12.0;
    // Every column is as wide as what it holds: no value is cut while
    // the table has room, and the last column stops short of the rows'
    // scroll bar. Owner read "Active schema…" on every row under it.
    for (text, rect) in &texts {
        // A row that asks for more says so with an ellipsis; nothing
        // else ends in one.
        let asks = ["Save view\u{2026}", "Saved views\u{2026}", "Export\u{2026}"];
        assert!(
            !text.ends_with('\u{2026}') || asks.contains(&text.as_str()),
            "{text:?} is cut"
        );
        if text == "Active schematic" {
            assert!(
                rect.right() <= edge - 13.0,
                "an owner ends at {}, under the rows' scroll bar",
                rect.right()
            );
        }
    }
    let _ = at("Active schematic");
    for (text, rect) in &texts {
        assert!(
            rect.right() <= edge,
            "{text:?} ends at {}, past the dialog's content edge {edge}",
            rect.right(),
        );
    }
    // The table's heading, under the columns list's own "Identifier".
    let heading = texts
        .iter()
        .filter(|(text, _)| text == "Identifier")
        .map(|(_, rect)| *rect)
        .max_by(|a, b| a.top().total_cmp(&b.top()))
        .expect("the table is headed");
    let value = at("VIN");
    assert!(
        (heading.left() - value.left()).abs() < 0.5,
        "the heading starts at {} and its values at {}",
        heading.left(),
        value.left()
    );
    assert!(
        (at("Saved view").left() - title.left()).abs() < 0.5,
        "the first row starts at {}, the title at {}",
        at("Saved view").left(),
        title.left()
    );
}

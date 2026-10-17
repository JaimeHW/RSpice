//! While Find and replace is open, the Navigator lists every text it would
//! change, each as the text now and the text it becomes, so the whole change
//! is read before Replace all makes it. A row in the cell view on the canvas
//! selects its object.

use egui::Ui;
use rspice_design::schematic::find_replace::{FindField, FindMatch};

use super::nav_row_indented_mono_response;
use crate::workbench::app::find_replace_matches;
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::WorkbenchIcon;

/// More rows than this are summed up in one, so a pattern that matches
/// everything cannot bury the rails under it.
const MAX_ROWS: usize = 200;

pub(super) fn find_matches_section(ui: &mut Ui, state: &mut AppState) {
    if !state.dialogs.find_replace.open {
        return;
    }
    // A query that cannot run says why in the form, beside the field.
    let Ok(found) = find_replace_matches(state) else {
        return;
    };
    let active = state.workspace.content.active_schematic_reference().key();
    let several = found.len() > 1;
    let total: usize = found.iter().map(|document| document.matches.len()).sum();
    let mut shown = 0;
    let mut picked = None;
    'documents: for document in &found {
        if several {
            let count = document.matches.len().to_string();
            nav_row_indented_mono_response(
                ui,
                WorkbenchIcon::Design,
                &document.path,
                false,
                Some(&count),
                0,
            );
        }
        for found in &document.matches {
            if shown == MAX_ROWS {
                let rest = format!("{} more", total - shown);
                nav_row_indented_mono_response(ui, WorkbenchIcon::Search, &rest, false, None, 1);
                break 'documents;
            }
            shown += 1;
            let label = format!("{} \u{2192} {}", found.before, found.after);
            let meta = meta(found);
            let response = nav_row_indented_mono_response(
                ui,
                icon(&found.field),
                &label,
                false,
                Some(&meta),
                usize::from(several),
            );
            if response.clicked() && document.key == active {
                picked = Some(found.clone());
            }
        }
    }
    if let Some(found) = picked {
        let selection = &mut state.schematic.session.editor.selection;
        match found.field {
            FindField::NetLabel => selection.select_only_net_label(found.object_id),
            _ => selection.select_only_component(found.object_id),
        }
    }
}

fn icon(field: &FindField) -> WorkbenchIcon {
    match field {
        FindField::InstanceName => WorkbenchIcon::Component,
        FindField::NetLabel => WorkbenchIcon::Label,
        FindField::Value | FindField::Parameter(_) => WorkbenchIcon::Sliders,
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, NetLabel, Point};

    /// The rows the Navigator lists while the form is open.
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_find_matches() {
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        let mut app = crate::workbench::RSpiceApp::test_instance();
        let document = app.state.schematic.document_mut_for_test();
        document.components = (1..=3)
            .map(|id| {
                let mut part = Component::new(id, ComponentType::Resistor, Point::origin());
                part.name = format!("R{id}");
                part.value = "10k".to_owned();
                part
            })
            .collect();
        document.net_labels = vec![NetLabel::new(9, Point::origin(), "out_r1")];
        crate::workbench::app::open_find_replace(&mut app.state);
        let form = &mut app.state.dialogs.find_replace;
        form.find = "r1".to_owned();
        form.replace = "RA".to_owned();
        let canvas = crate::ui::raster::render(egui::vec2(300.0, 160.0), |ui, _background| {
            find_matches_section(ui, &mut app.state);
        });
        let height = canvas.content_height().max(1);
        std::fs::write(
            directory.join("find-replace-navigator.png"),
            canvas.png(height),
        )
        .expect("write navigator render");
    }
}

/// What the row's text is: an instance's own name, or which of its fields.
fn meta(found: &FindMatch) -> String {
    match found.field {
        FindField::InstanceName | FindField::NetLabel => found.field.label(),
        _ => format!("{} {}", found.owner, found.field.label()),
    }
}

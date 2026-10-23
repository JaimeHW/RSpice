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
    if !state.design.find_replace.open {
        return;
    }
    // A query that cannot run says why in the form, beside the field.
    let Ok(found) = find_replace_matches(state) else {
        return;
    };
    let active = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
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
        let selection = &mut state.model.schematic.session.editor.selection;
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

/// What the row's text is: an instance's own name, or which of its fields.
fn meta(found: &FindMatch) -> String {
    match found.field {
        FindField::InstanceName | FindField::NetLabel => found.field.label(),
        _ => format!("{} {}", found.owner, found.field.label()),
    }
}

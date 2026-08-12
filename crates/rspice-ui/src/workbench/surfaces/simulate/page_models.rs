//! Models & sections.
//!
//! The model closure this plan binds against. The page reads the loaded
//! libraries and states, per library, which corner section a run will resolve
//! to and whether its source is pinned well enough to be reproducible.
//!
//! Authoring belongs to Models & PDKs; this page is the plan's read of that
//! closure plus the one binding decision the plan owns — the corner.

use egui::Ui;

use crate::state::model_library::ModelSourceAuthority;
use crate::ui::widgets::select;
use crate::workbench::RSpiceApp;

use super::page_kit::{
    Tone, card, card_body, card_note, card_row, cell_ui, ledger_head, ledger_row, ledger_row_cells,
    paint_text, rule_row,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

const CLOSURE_COLUMNS: [f32; 5] = [0.24, 0.16, 0.16, 0.20, 0.24];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    closure(ui, app);
    card_row(
        ui,
        app,
        |ui, app| binding_policy(ui, app),
        |ui, app| reproducibility(ui, app),
    );
}

fn closure(ui: &mut Ui, app: &mut RSpiceApp) {
    let libraries: Vec<(
        String,
        String,
        usize,
        usize,
        Option<String>,
        ModelSourceAuthority,
    )> = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            (
                library.name.clone(),
                library.technology_node.clone(),
                library.models.len(),
                library.source_closure.len(),
                library.selected_corner.clone(),
                library.source_authority,
            )
        })
        .collect();
    let unpinned = libraries
        .iter()
        .filter(|(_, _, _, closure, _, authority)| {
            *closure == 0 && matches!(authority, ModelSourceAuthority::External)
        })
        .count();
    let status = if unpinned == 0 {
        format!("{} libraries · every source pinned", libraries.len())
    } else {
        format!("{unpinned} external libraries are not pinned")
    };
    let tone = if unpinned == 0 { Tone::Ok } else { Tone::Error };
    let mut requested: Option<(String, String)> = None;
    card(ui, "Model closure", Some((status.as_str(), tone)), |ui| {
        ledger_head(
            ui,
            &CLOSURE_COLUMNS,
            &["Library", "Node", "Models", "Source", "Corner section"],
        );
        if libraries.is_empty() {
            ledger_row(
                ui,
                &CLOSURE_COLUMNS,
                &[
                    ("No libraries loaded", Tone::Neutral),
                    ("—", Tone::Neutral),
                    ("—", Tone::Neutral),
                    ("—", Tone::Neutral),
                    ("nothing to bind against", Tone::Warn),
                ],
                false,
            );
        }
        for (name, node, models, closure_len, corner, authority) in &libraries {
            let corners = corner_options(app, name);
            let (rect, cells) = ledger_row_cells(ui, &CLOSURE_COLUMNS);
            let t = Tokens::get(ui.ctx());
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            let font = theme::mono(tokens::FS_0, FontWeight::Regular);
            paint_text(
                ui,
                cells[0].shrink2(egui::vec2(8.0, 0.0)),
                name,
                font.clone(),
                t.color.accent,
            );
            paint_text(
                ui,
                cells[1].shrink2(egui::vec2(8.0, 0.0)),
                if node.is_empty() {
                    "—"
                } else {
                    node.as_str()
                },
                font.clone(),
                t.color.text_dim,
            );
            paint_text(
                ui,
                cells[2].shrink2(egui::vec2(8.0, 0.0)),
                &models.to_string(),
                font.clone(),
                t.color.text_dim,
            );
            let (source_text, source_color) = match (authority, closure_len) {
                (ModelSourceAuthority::External, 0) => {
                    ("external · unpinned".to_owned(), t.color.err)
                }
                (ModelSourceAuthority::External, len) => {
                    (format!("external · {len} pinned"), t.color.text_dim)
                }
                (_, 0) => ("project-owned".to_owned(), t.color.text_dim),
                (_, len) => (format!("project · {len} pinned"), t.color.text_dim),
            };
            paint_text(
                ui,
                cells[3].shrink2(egui::vec2(8.0, 0.0)),
                &source_text,
                font,
                source_color,
            );
            if corners.is_empty() {
                paint_text(
                    ui,
                    cells[4].shrink2(egui::vec2(8.0, 0.0)),
                    "no corner sections declared",
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_faint,
                );
            } else {
                let selected = corner.clone().unwrap_or_else(|| corners[0].clone());
                let cell_rect = cells[4].shrink2(egui::vec2(6.0, 4.0));
                let mut cell = cell_ui(ui, cell_rect);
                if let Some(index) = select(
                    &mut cell,
                    &format!("simulation.models.corner.{name}"),
                    "Corner section",
                    &selected,
                    &corners,
                    cell_rect.width(),
                ) {
                    requested = Some((name.clone(), corners[index].clone()));
                }
            }
        }
        card_note(
            ui,
            "A library's corner section decides which model cards a run resolves. An external \
             library with no pinned source closure cannot be reproduced from the project alone, \
             so preflight refuses to dispatch against it.",
        );
    });

    if let Some((library, corner)) = requested {
        set_corner(app, &library, &corner);
    }
}

fn corner_options(app: &RSpiceApp, library: &str) -> Vec<String> {
    app.state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|candidate| candidate.name == library)
        .map(|candidate| {
            let mut names: Vec<String> = candidate.corners.keys().cloned().collect();
            names.sort();
            names
        })
        .unwrap_or_default()
}

fn set_corner(app: &mut RSpiceApp, library: &str, corner: &str) {
    let Some(entry) = app.state.model_library_manager.get_library_mut(library) else {
        app.state
            .workbench
            .analysis_lifecycle_status
            .record_refusal(format!("Library {library} is no longer loaded."));
        return;
    };
    entry.selected_corner = Some(corner.to_owned());
    app.invalidate_simulation_preflight();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt(format!("{library} now resolves its {corner} section."));
}

fn binding_policy(ui: &mut Ui, app: &RSpiceApp) {
    let libraries = app.state.model_library_manager.libraries_sorted();
    let with_corner = libraries
        .iter()
        .filter(|library| library.selected_corner.is_some())
        .count();
    let subcircuits: usize = libraries
        .iter()
        .map(|library| library.subcircuits.len())
        .sum();
    card(
        ui,
        "Binding policy",
        Some(("resolved at elaboration", Tone::Ok)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(
                    ui,
                    "Libraries with a corner bound",
                    &format!("{with_corner} of {}", libraries.len()),
                );
                rule_row(ui, "Addressable subcircuits", &subcircuits.to_string());
                rule_row(
                    ui,
                    "Unresolved instance",
                    "refused at preflight · the instance and the name it wanted are reported",
                );
                rule_row(
                    ui,
                    "Duplicate definition",
                    "the first library in load order wins, and the shadowed one is reported",
                );
            });
            card_note(
                ui,
                "Resolution order is the library load order, which the model workspace owns. This \
                 page sets which section of each library a run reads; it never reorders the \
                 closure.",
            );
        },
    );
}

fn reproducibility(ui: &mut Ui, app: &RSpiceApp) {
    let libraries = app.state.model_library_manager.libraries_sorted();
    // Only an external library needs a pinned closure: a project-owned one
    // already carries its bytes in the project file. Counting every library
    // as needing a pin made this card contradict the closure card above it,
    // which reports the same fact.
    let external: Vec<_> = libraries
        .iter()
        .filter(|library| matches!(library.source_authority, ModelSourceAuthority::External))
        .collect();
    let pinned = external
        .iter()
        .filter(|library| !library.source_closure.is_empty())
        .count();
    let records: usize = libraries
        .iter()
        .map(|library| library.source_contents.len())
        .sum();
    let complete = pinned == external.len();
    card(
        ui,
        "Reproducibility",
        Some((
            if complete {
                "every external source pinned"
            } else {
                "incomplete"
            },
            if complete { Tone::Ok } else { Tone::Error },
        )),
        |ui| {
            card_body(ui, |ui| {
                rule_row(
                    ui,
                    "External libraries needing a pin",
                    &format!("{pinned} of {} pinned", external.len()),
                );
                rule_row(
                    ui,
                    "Project-owned libraries",
                    &format!(
                        "{} · bytes already retained in the project",
                        libraries.len() - external.len()
                    ),
                );
                rule_row(ui, "Retained source records", &records.to_string());
                rule_row(
                    ui,
                    "Digest",
                    "each retained source carries its own content digest into the run manifest",
                );
            });
            card_note(
                ui,
                "A pinned closure is what makes a result reproducible from the project alone: the \
                 exact bytes that were read, not the path they were read from. A library resolved \
                 from a live path that has since changed would otherwise produce a different \
                 answer under the same manifest.",
            );
        },
    );
}

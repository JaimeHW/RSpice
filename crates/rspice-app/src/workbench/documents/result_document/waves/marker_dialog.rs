//! The marker purpose dialog: label, kind, and the anchoring facts.
//!
//! Editing is transactional — Apply is the only mutation path, so a
//! half-typed label can always be abandoned without touching the marker, and
//! reclassifying one is a decision rather than a side effect of clicking its
//! kind. The kind choices are exactly the geometries the plot renderer draws;
//! the dialog never offers a marker the canvas could not produce.

use egui::RichText;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::AppState;

use super::{MarkerEditDraft, MarkerKind, MarkerSelector, cached_models};

/// Open the dialog for one marker, seeding the draft from its current state.
pub(in super::super) fn open(state: &mut AppState, selector: MarkerSelector) {
    let seed = match selector {
        MarkerSelector::Quick(id) => state
            .ui
            .results
            .markers
            .iter()
            .find(|marker| marker.id == id)
            .map(|marker| (marker.note.clone(), marker.kind)),
        MarkerSelector::Document { marker_id, .. } => state
            .ui
            .results
            .document_marker(marker_id)
            .map(|marker| (marker.note.clone(), marker.kind)),
    };
    let Some((note, kind)) = seed else {
        return;
    };
    state.ui.results.marker_edit = Some(MarkerEditDraft {
        selector,
        note,
        kind,
    });
}

/// What the dialog reports about the marker it is editing.
struct MarkerFacts {
    display_id: String,
    analysis: super::AnalysisPresentationKey,
    trace_name: String,
    x: f64,
    persistence: &'static str,
}

fn marker_facts(state: &AppState, selector: MarkerSelector) -> Option<MarkerFacts> {
    match selector {
        MarkerSelector::Quick(id) => {
            let marker = state.ui.results.markers.iter().find(|m| m.id == id)?;
            Some(MarkerFacts {
                display_id: format!("M{id}"),
                analysis: marker.analysis,
                trace_name: marker.trace_name.clone(),
                x: marker.x,
                persistence: "Saved with the project · survives zoom, pan and reload",
            })
        }
        MarkerSelector::Document { marker_id, .. } => {
            let marker = state.ui.results.document_marker(marker_id)?;
            Some(MarkerFacts {
                display_id: format!("D{}", marker_id.get()),
                analysis: marker.analysis,
                trace_name: marker.trace_name.clone(),
                x: marker.x,
                persistence: "Retained by this result document · travels with the document",
            })
        }
    }
}

/// Render the dialog while a draft is open.
pub(in super::super) fn show(ctx: &egui::Context, state: &mut AppState) {
    let Some(mut draft) = state.ui.results.marker_edit.clone() else {
        return;
    };
    let Some(marker) = marker_facts(state, draft.selector) else {
        // The marker vanished under the dialog (dataset change, or a document
        // that stopped retaining it): the draft has nothing to apply to and
        // must not linger.
        state.ui.results.marker_edit = None;
        return;
    };

    let t = Tokens::get(ctx);
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let strip = models
        .iter()
        .find(|model| model.analysis_key == marker.analysis);
    let anchor_text = strip.map_or_else(
        || format!("{} · {}", marker.x, marker.trace_name),
        |model| {
            format!(
                "{} · {}",
                model.format_x(marker.x, significant_digits, quantity_policy),
                marker.trace_name
            )
        },
    );
    let sheet_text = strip.map_or_else(
        || "retained analysis".to_owned(),
        |model| model.table_label(),
    );

    let mut window_open = true;
    let mut apply = false;
    let mut cancel = false;
    egui::Window::new(format!("Marker {}", marker.display_id))
        .id(egui::Id::new("rspice.results.marker-edit"))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(false)
        .default_width(400.0)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label(
                RichText::new("RESULTS · MARKER PURPOSE")
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.add_space(6.0);

            ui.label(RichText::new("Label").strong());
            ui.add(
                egui::TextEdit::singleline(&mut draft.note)
                    .desired_width(f32::INFINITY)
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .hint_text("What this marker calls out\u{2026}"),
            );
            ui.add_space(8.0);

            ui.label(RichText::new("Kind").strong());
            for kind in MarkerKind::ALL {
                if ui.radio(draft.kind == kind, kind.dialog_label()).clicked() {
                    draft.kind = kind;
                }
            }
            ui.add_space(10.0);

            egui::Grid::new("rspice.results.marker-edit.facts")
                .num_columns(2)
                .spacing(egui::vec2(14.0, 4.0))
                .show(ui, |ui| {
                    for (label, value) in [
                        ("Anchor", anchor_text.as_str()),
                        ("Sheet", sheet_text.as_str()),
                        ("Persistence", marker.persistence),
                    ] {
                        ui.label(RichText::new(label).color(t.color.text_faint));
                        ui.label(
                            RichText::new(value)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.end_row();
                    }
                });
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                if ui.button("Apply").clicked() {
                    apply = true;
                }
                if ui.button("Cancel").clicked() {
                    cancel = true;
                }
            });
        });

    if apply {
        // The selector names the store, so Apply reaches exactly the marker
        // the dialog opened on and nothing that merely shares its number.
        if let Err(error) =
            super::super::commit_marker_edit(state, draft.selector, &draft.note, draft.kind)
        {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Could not retain marker edit: {error}"
            )));
            state.ui.results.marker_edit = Some(draft);
            return;
        }
        state.ui.results.marker_edit = None;
    } else if cancel || !window_open {
        state.ui.results.marker_edit = None;
    } else {
        state.ui.results.marker_edit = Some(draft);
    }
}

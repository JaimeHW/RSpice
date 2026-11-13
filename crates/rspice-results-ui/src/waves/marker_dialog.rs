//! The marker purpose dialog: label, kind, and the anchoring facts.
//!
//! Editing is transactional — Apply is the only mutation path, so a
//! half-typed label can always be abandoned without touching the marker, and
//! reclassifying one is a decision rather than a side effect of clicking its
//! kind. The kind choices are exactly the geometries the plot renderer draws;
//! the dialog never offers a marker the canvas could not produce.

use egui::RichText;

use crate::session::{MarkerEditDraft, MarkerSelector, ResultViewerState};
use rspice_results::result_presentation::{
    AnalysisPresentationKey, CrossingLevel, MarkerKind, WaveformPresentationKey,
};
use rspice_ui_kit::plot::sample::sample_at_with_shape;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use super::{ReadoutPolicy, StripModel, anchor_key, cursor_interpolation};

/// Open the dialog for one marker, seeding the draft from its current state.
pub fn open(state: &mut ResultViewerState, selector: MarkerSelector) {
    let seed = match selector {
        MarkerSelector::Quick(id) => state
            .markers
            .iter()
            .find(|marker| marker.id == id)
            .map(|marker| (marker.note.clone(), marker.kind)),
        MarkerSelector::Document { marker_id, .. } => state
            .document_marker(marker_id)
            .map(|marker| (marker.note.clone(), marker.kind)),
    };
    let Some((note, kind)) = seed else {
        return;
    };
    state.marker_edit = Some(MarkerEditDraft {
        selector,
        note,
        kind,
    });
}

/// What the dialog reports about the marker it is editing.
struct MarkerFacts {
    display_id: String,
    analysis: AnalysisPresentationKey,
    anchor: WaveformPresentationKey,
    trace_name: String,
    /// A result document keeps notes, peaks and specs; a valley or crossing
    /// is a search only the project's quick markers carry.
    document: bool,
    x: f64,
    persistence: &'static str,
}

fn marker_facts(state: &ResultViewerState, selector: MarkerSelector) -> Option<MarkerFacts> {
    match selector {
        MarkerSelector::Quick(id) => {
            let marker = state.markers.iter().find(|m| m.id == id)?;
            Some(MarkerFacts {
                display_id: format!("M{id}"),
                analysis: marker.analysis,
                anchor: marker.anchor.clone(),
                trace_name: marker.trace_name.clone(),
                document: false,
                x: marker.x,
                persistence: "Saved with the project · survives zoom, pan and reload",
            })
        }
        MarkerSelector::Document { marker_id, .. } => {
            let marker = state.document_marker(marker_id)?;
            Some(MarkerFacts {
                display_id: format!("D{}", marker_id.get()),
                analysis: marker.analysis,
                anchor: marker.anchor.clone(),
                trace_name: marker.trace_name.clone(),
                document: true,
                x: marker.x,
                persistence: "Retained by this result document · travels with the document",
            })
        }
    }
}

/// A draft and its anchoring facts captured before the host refreshes projections.
pub struct MarkerDialog {
    draft: MarkerEditDraft,
    facts: MarkerFacts,
}

pub enum MarkerDialogResponse {
    Editing(MarkerEditDraft),
    Apply(MarkerEditDraft),
    Cancel,
}

/// Drop a vanished marker's draft; otherwise capture its current facts.
pub fn prepare(state: &mut ResultViewerState) -> Option<MarkerDialog> {
    let draft = state.marker_edit.clone()?;
    let Some(marker) = marker_facts(state, draft.selector) else {
        // The marker vanished under the dialog (dataset change, or a document
        // that stopped retaining it): the draft has nothing to apply to and
        // must not linger.
        state.marker_edit = None;
        return None;
    };

    Some(MarkerDialog {
        draft,
        facts: marker,
    })
}

/// Paint the draft and return a source-bound outcome for the host to commit.
pub fn show(
    ctx: &egui::Context,
    dialog: MarkerDialog,
    models: &[StripModel],
    presentation: ReadoutPolicy,
    quantity_policy: QuantityPresentationPolicy,
) -> MarkerDialogResponse {
    let MarkerDialog {
        mut draft,
        facts: marker,
    } = dialog;
    let t = Tokens::get(ctx);
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
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
                    .font(theme::field_sans())
                    .hint_text("What this marker calls out\u{2026}"),
            );
            ui.add_space(8.0);

            ui.label(RichText::new("Kind").strong());
            for kind in MarkerKind::ALL {
                if marker.document && kind.search().is_some() && kind != MarkerKind::Peak {
                    continue;
                }
                if ui
                    .radio(draft.kind.same_choice(kind), kind.dialog_label())
                    .clicked()
                    && !draft.kind.same_choice(kind)
                {
                    draft.kind = match kind {
                        // A crossing marker finds the level the curve has
                        // where it was put.
                        MarkerKind::Crossing(_) => MarkerKind::Crossing(CrossingLevel(
                            level_at(strip, &marker, presentation).unwrap_or(0.0),
                        )),
                        kind => kind,
                    };
                }
            }
            ui.add_space(10.0);

            egui::Grid::new("rspice.results.marker-edit.facts")
                .num_columns(2)
                .spacing(egui::vec2(14.0, 4.0))
                .show(ui, |ui| {
                    let level = match draft.kind {
                        MarkerKind::Crossing(level) => strip
                            .and_then(|model| {
                                let trace = model.traces.iter().find(|trace| {
                                    !trace.overlay && anchor_key(model, trace) == marker.anchor
                                })?;
                                Some(model.format_trace_value(
                                    trace,
                                    level.0,
                                    significant_digits,
                                    quantity_policy,
                                ))
                            })
                            .or_else(|| Some(level.0.to_string())),
                        _ => None,
                    };
                    let rows = [
                        Some(("Anchor", anchor_text.as_str())),
                        level.as_deref().map(|level| ("Level", level)),
                        Some(("Sheet", sheet_text.as_str())),
                        Some(("Persistence", marker.persistence)),
                    ];
                    for (label, value) in rows.into_iter().flatten() {
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
        MarkerDialogResponse::Apply(draft)
    } else if cancel || !window_open {
        MarkerDialogResponse::Cancel
    } else {
        MarkerDialogResponse::Editing(draft)
    }
}

/// The value the marker's trace takes where the marker is, read the way the
/// cursor readout reads it.
fn level_at(
    strip: Option<&StripModel>,
    marker: &MarkerFacts,
    presentation: ReadoutPolicy,
) -> Option<f64> {
    let model = strip?;
    let trace = model
        .traces
        .iter()
        .find(|trace| !trace.overlay && anchor_key(model, trace) == marker.anchor)?;
    let value = sample_at_with_shape(
        &trace.x,
        &trace.y,
        &trace.shape,
        marker.x,
        cursor_interpolation(presentation.cursor_interpolation()),
    );
    value.is_finite().then_some(value)
}

//! The marker dialog: a marker's label and kind, with the level a crossing
//! finds or the marker and amount an offset is read from, each typed in its
//! trace's unit, and where the marker stands with the kind as drafted.
//!
//! Editing is transactional: Apply is the only mutation path, so a
//! half-typed label can always be abandoned without touching the marker, and
//! reclassifying one is a decision rather than a side effect of picking its
//! kind. The kind choices are exactly the geometries the plot renderer draws;
//! the dialog never offers a marker the canvas could not produce.

use crate::session::{MarkerEditDraft, MarkerSelector, ResultViewerState, TypedEntry};
use rspice_results::result_presentation::{
    AnalysisPresentationKey, CrossingLevel, FeatureSize, MarkerKind, MarkerOffset,
    WaveformPresentationKey,
};
use rspice_ui_kit::plot::{SampleInterpolation, fmt_si_significant};
use rspice_ui_kit::widgets::{CommandForm, DialogChoice, FormRows};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use super::cursor_table::parse_typed_x;
use super::cursor_values::{marker_position, offset_position};
use super::{ReadoutPolicy, StripModel, StripTrace, anchor_key, cursor_interpolation};

/// Open the dialog for one marker, seeding the draft from its current state.
pub fn open(state: &mut ResultViewerState, selector: MarkerSelector) {
    let seed = state
        .document_marker(selector.marker_id)
        .map(|marker| (marker.note.clone(), marker.kind));
    let Some((note, kind)) = seed else {
        return;
    };
    state.marker_edit = Some(MarkerEditDraft {
        selector,
        note,
        kind,
        level: None,
        by: None,
        size: None,
    });
}

/// Another marker on the strip an offset can be read from.
struct Reference {
    id: u64,
    name: String,
    anchor: WaveformPresentationKey,
    x: f64,
    kind: MarkerKind,
}

/// What the dialog reports about the marker it is editing.
struct MarkerFacts {
    display_id: String,
    analysis: AnalysisPresentationKey,
    anchor: WaveformPresentationKey,
    trace_name: String,
    x: f64,
    /// The strip's other markers that are not offsets themselves.
    references: Vec<Reference>,
}

fn marker_facts(state: &ResultViewerState, selector: MarkerSelector) -> Option<MarkerFacts> {
    let marker = state.document_marker(selector.marker_id)?;
    let references = state
        .strip_markers(marker.analysis)
        .filter(|other| {
            other.0.retained_id != selector.marker_id
                && other.0.document_id == selector.document_id
                && !matches!(other.kind(), MarkerKind::Offset(_))
        })
        .map(|other| Reference {
            id: other.0.retained_id.get(),
            name: if other.kind().rides_a_trace() {
                format!(
                    "{} \u{b7} {} on {}",
                    other.display_id(),
                    other.kind().label(),
                    other.trace_name()
                )
            } else {
                format!("{} \u{b7} {}", other.display_id(), other.kind().label())
            },
            anchor: other.anchor().clone(),
            x: other.x(),
            kind: other.kind(),
        })
        .collect();
    Some(MarkerFacts {
        display_id: format!("M{}", selector.marker_id.get()),
        analysis: marker.analysis,
        anchor: marker.anchor.clone(),
        trace_name: marker.trace_name.clone(),
        x: marker.x,
        references,
    })
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

/// The kinds the dialog offers: every kind, and an offset while the strip
/// holds a marker to read it from.
fn offered(facts: &MarkerFacts) -> Vec<MarkerKind> {
    let mut kinds = MarkerKind::ALL.to_vec();
    if let Some(first) = facts.references.first() {
        kinds.push(MarkerKind::Offset(MarkerOffset {
            from: first.id,
            by: CrossingLevel(0.0),
        }));
    }
    kinds
}

/// A kind's name in the Kind list: its label, capitalised.
fn kind_name(kind: MarkerKind) -> String {
    let label = kind.label();
    let mut chars = label.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// A typed entry's value: the value it was seeded from while its text is
/// still the seed, so a level shown at four digits is not rounded to them
/// by an Apply that never touched it; `None` when the text does not read.
fn typed_value(entry: &TypedEntry, seeded_from: f64, unit: &str) -> Option<f64> {
    if entry.text.trim() == entry.seed.trim() {
        Some(seeded_from)
    } else {
        parse_typed_x(&entry.text, unit)
    }
}

/// A typed entry seeded with `value` as the table prints it, its unit left
/// to the field's end.
fn entry(value: f64, digits: usize) -> TypedEntry {
    let seed = fmt_si_significant(value, "", digits);
    TypedEntry {
        text: seed.clone(),
        seed,
    }
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
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let strip = models
        .iter()
        .find(|model| model.analysis_key == marker.analysis);
    let trace = strip.and_then(|model| {
        model
            .traces
            .iter()
            .find(|trace| !trace.overlay && anchor_key(model, trace) == marker.anchor)
    });
    let unit = strip
        .zip(trace)
        .map_or("", |(model, trace)| model.trace_unit(trace))
        .to_owned();

    // Each typed entry is seeded, in the trace's unit, the first frame its
    // kind is shown.
    if let MarkerKind::Crossing(level) = draft.kind
        && draft.level.is_none()
    {
        draft.level = Some(entry(level.0, digits));
    }
    if let MarkerKind::Offset(offset) = draft.kind
        && draft.by.is_none()
    {
        draft.by = Some(entry(offset.by.0, digits));
    }
    if let MarkerKind::Peak(size) | MarkerKind::Valley(size) = draft.kind
        && draft.size.is_none()
    {
        // The default size reads as a blank field.
        draft.size = Some(if size.0 > 0.0 {
            entry(size.0, digits)
        } else {
            TypedEntry::default()
        });
    }

    // The kind as drafted, with what was typed read into it.
    let resolved: Result<MarkerKind, String> = match draft.kind {
        MarkerKind::Crossing(level) => draft
            .level
            .as_ref()
            .and_then(|typed| typed_value(typed, level.0, &unit))
            .map(|level| MarkerKind::Crossing(CrossingLevel(level)))
            .ok_or_else(|| format!("Type the level as a number{}.", in_unit(&unit))),
        MarkerKind::Offset(offset) => draft
            .by
            .as_ref()
            .and_then(|typed| typed_value(typed, offset.by.0, &unit))
            .map(|by| {
                MarkerKind::Offset(MarkerOffset {
                    from: offset.from,
                    by: CrossingLevel(by),
                })
            })
            .ok_or_else(|| format!("Type the amount as a number{}.", in_unit(&unit))),
        MarkerKind::Peak(size) | MarkerKind::Valley(size) => draft
            .size
            .as_ref()
            .and_then(|typed| {
                if typed.text.trim().is_empty() {
                    Some(0.0)
                } else {
                    typed_value(typed, size.0, &unit).filter(|size| *size > 0.0)
                }
            })
            .map(|size| match draft.kind {
                MarkerKind::Peak(_) => MarkerKind::Peak(FeatureSize(size)),
                _ => MarkerKind::Valley(FeatureSize(size)),
            })
            .ok_or_else(|| {
                format!(
                    "Type the minimum size as a number above zero{}, or leave it blank.",
                    in_unit(&unit)
                )
            }),
        kind => Ok(kind),
    };
    let stands = resolved.as_ref().ok().map(|&kind| {
        stands_text(
            (strip, trace),
            &marker,
            kind,
            (digits, quantity_policy),
            presentation,
        )
    });

    let title = format!("Marker {}", marker.display_id);
    let offered = offered(&marker);
    let names: Vec<String> = offered.iter().map(|kind| kind_name(*kind)).collect();
    let references: Vec<String> = marker
        .references
        .iter()
        .map(|reference| reference.name.clone())
        .collect();
    let label_id = egui::Id::new("rspice.results.marker-edit.label");
    let choice = CommandForm::new(&title, "Apply")
        .first_field(label_id)
        .primary_enabled(resolved.is_ok())
        .status(match &resolved {
            Ok(_) => Ok(None),
            Err(why) => Err(why.as_str()),
        })
        .show(ctx, |rows: &mut FormRows<'_>| {
            rows.text_sans(
                "Label",
                label_id,
                &mut draft.note,
                "What this marker calls out\u{2026}",
            );
            let current = offered
                .iter()
                .position(|kind| kind.same_choice(draft.kind))
                .map_or_else(|| kind_name(draft.kind), |index| names[index].clone());
            if let Some(index) =
                rows.select("Kind", "rspice.results.marker-edit.kind", &current, &names)
                && !draft.kind.same_choice(offered[index])
            {
                draft.kind = match offered[index] {
                    // A crossing finds the level the curve has where the
                    // marker was put; an offset starts 3 below its marker.
                    MarkerKind::Crossing(_) => MarkerKind::Crossing(CrossingLevel(
                        level_at(strip, &marker.anchor, marker.x, presentation).unwrap_or(0.0),
                    )),
                    MarkerKind::Offset(offset) => MarkerKind::Offset(MarkerOffset {
                        by: CrossingLevel(-3.0),
                        ..offset
                    }),
                    kind => kind,
                };
                draft.level = None;
                draft.by = None;
                draft.size = None;
            }
            if let Some((_, why)) = draft.kind.dialog_label().split_once(" \u{2014} ") {
                rows.derived(why);
            }
            if draft.kind.rides_a_trace() {
                rows.value("Trace", &marker.trace_name);
            }
            match draft.kind {
                MarkerKind::Crossing(_) => {
                    if let Some(level) = draft.level.as_mut() {
                        rows.quantity(
                            "Level",
                            egui::Id::new("rspice.results.marker-edit.level"),
                            &mut level.text,
                            "",
                            &unit,
                            "",
                        );
                    }
                }
                MarkerKind::Peak(_) | MarkerKind::Valley(_) => {
                    if let Some(size) = draft.size.as_mut() {
                        rows.quantity(
                            "Minimum size",
                            egui::Id::new("rspice.results.marker-edit.size"),
                            &mut size.text,
                            "",
                            &unit,
                            "blank: any size",
                        );
                    }
                }
                MarkerKind::Offset(offset) => {
                    let at = marker
                        .references
                        .iter()
                        .position(|reference| reference.id == offset.from);
                    let shown = at.map_or("(gone)", |at| references[at].as_str());
                    if let Some(index) = rows.select(
                        "From",
                        "rspice.results.marker-edit.from",
                        shown,
                        &references,
                    ) {
                        draft.kind = MarkerKind::Offset(MarkerOffset {
                            from: marker.references[index].id,
                            ..offset
                        });
                    }
                    if let Some(by) = draft.by.as_mut() {
                        rows.quantity(
                            "By",
                            egui::Id::new("rspice.results.marker-edit.by"),
                            &mut by.text,
                            "",
                            &unit,
                            "added to its level",
                        );
                    }
                }
                _ => {}
            }
            if let Some(stands) = &stands {
                rows.value_mono("Stands at", stands);
            }
            Some(label_id)
        });

    match choice {
        DialogChoice::Primary => match resolved {
            Ok(kind) => {
                draft.kind = kind;
                MarkerDialogResponse::Apply(draft)
            }
            Err(_) => MarkerDialogResponse::Editing(draft),
        },
        DialogChoice::Cancelled | DialogChoice::Ghost => MarkerDialogResponse::Cancel,
        DialogChoice::None | DialogChoice::Secondary => MarkerDialogResponse::Editing(draft),
    }
}

/// " in dB", or nothing for a trace with no unit.
fn in_unit(unit: &str) -> String {
    if unit.is_empty() {
        String::new()
    } else {
        format!(" in {unit}")
    }
}

/// Where the marker stands with `kind`, its X and the Y of the trace it
/// rides, or why it stands nowhere.
fn stands_text(
    (strip, trace): (Option<&StripModel>, Option<&StripTrace>),
    marker: &MarkerFacts,
    kind: MarkerKind,
    (digits, quantity_policy): (usize, QuantityPresentationPolicy),
    presentation: ReadoutPolicy,
) -> String {
    let Some(model) = strip else {
        return fmt_si_significant(marker.x, "", digits);
    };
    let found = match kind {
        MarkerKind::Offset(offset) => marker
            .references
            .iter()
            .find(|reference| reference.id == offset.from)
            .map_or(Err(marker.x), |reference| {
                offset_position(
                    model,
                    (&reference.anchor, reference.x, reference.kind),
                    (&marker.anchor, marker.x),
                    offset.by.0,
                )
            }),
        kind => marker_position(model, &marker.anchor, marker.x, kind),
    };
    let Ok(x) = found else {
        return match kind {
            MarkerKind::Offset(_) => "Its level is not crossed on this trace".to_owned(),
            kind => format!("No {} on this trace", kind.label()),
        };
    };
    let x_text = model.format_x(x, digits, quantity_policy);
    match trace.filter(|_| kind.rides_a_trace()) {
        Some(trace) => {
            let interpolation = match kind {
                // A crossing or an offset stands where the drawn line meets
                // its level.
                MarkerKind::Crossing(_) | MarkerKind::Offset(_) => SampleInterpolation::Linear,
                _ => cursor_interpolation(presentation.cursor_interpolation()),
            };
            let y = model.trace_value(trace, x, interpolation);
            format!(
                "{x_text} \u{b7} {}",
                model.format_trace_value(trace, y, digits, quantity_policy)
            )
        }
        None => x_text,
    }
}

/// The value the trace `anchor` takes at the marker's `x`, read the way the
/// cursor readout reads it: a stored trace reads only the rows around `x`.
pub(super) fn level_at(
    strip: Option<&StripModel>,
    anchor: &WaveformPresentationKey,
    x: f64,
    presentation: ReadoutPolicy,
) -> Option<f64> {
    let model = strip?;
    let trace = model
        .traces
        .iter()
        .find(|trace| !trace.overlay && anchor_key(model, trace) == *anchor)?;
    let value = model.trace_value(
        trace,
        x,
        cursor_interpolation(presentation.cursor_interpolation()),
    );
    value.is_finite().then_some(value)
}

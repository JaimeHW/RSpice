//! Publishing one placed source's card as a library definition.
//!
//! The left pane is the only part of this transaction a reader authors: a name
//! the netlist reader will accept, and a sentence saying what the stimulus is
//! for. Everything under it is what saving will record, stated before it
//! happens rather than reported after.
//!
//! The right pane is the definition itself. A definition is a shape, not a run,
//! so the strip here is captioned with the window it was actually drawn over
//! and the facts beside it are read off those same samples — never off the
//! parameter string, which is how a readout and the curve above it come to
//! disagree about a source the engine has already substituted a field into.

use egui::{Rect, Sense, Stroke, Ui, vec2};

use crate::properties::source_preview;
use crate::simulation::placed_sources;
use crate::simulation::stimulus_realize::WaveformTrace;
use crate::state::Component;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::app::dialogs::review_primitives::{input_field, text_edit_field};
use crate::workbench::app_state::AppState;

use super::shell::{self, Fact};
use super::{StimulusLinkDialogState, extract_refusal, family_line};

/// The authoring pane. Narrower than the adopt list, because it holds two
/// fields and four facts rather than a library.
const FORM_WIDTH: f32 = 280.0;
/// The one strip. Taller than either adopt strip: there is nothing to compare
/// it against, so the whole pane is this shape.
const STRIP_HEIGHT: f32 = 84.0;
/// Below this the two panes stop being panes.
const MIN_BODY: f32 = 268.0;

/// What the modal is for, for a reader who arrives through the accessibility
/// tree rather than through the schematic.
const PURPOSE: &str =
    "Publish this instance's card as a project stimulus definition and adopt it back";

/// Render the save transaction.
pub(super) fn render(
    ctx: &egui::Context,
    state: &mut AppState,
    session: &StimulusLinkDialogState,
    component: &Component,
) {
    let refusal = extract_refusal(state, component, &session.name);
    let title = format!("Save {} as a definition", session.reference);
    let dialog = Dialog::new("STIMULUS LIBRARY", title.clone(), "Save definition")
        .description(PURPOSE)
        .size(DialogSize::Transaction)
        .without_header()
        .flush_body()
        .manual_body_scroll()
        .ghost("Cancel")
        .primary_enabled(refusal.is_none())
        .hint(format!(
            "{} keeps its card. The library gains a reusable copy.",
            session.reference
        ))
        .initial_focus(DialogInitialFocus::BodyControl);

    let mut name = session.name.clone();
    let mut purpose = session.purpose.clone();
    let mut closed = false;
    let select_name = session.select_name;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        closed = shell::header(
            ui,
            &shell::Identity {
                eyebrow: "STIMULUS LIBRARY",
                title: &title,
                card: &session.card,
                chip: &session.chip,
                provenance: session.provenance,
            },
        );
        let curve = source_curve(component, session);
        let body_height = body_height(ui);
        let mut focus = None;
        shell::panes(
            ui,
            FORM_WIDTH,
            body_height,
            |ui| {
                focus = Some(form_pane(
                    ui,
                    session,
                    component,
                    &mut name,
                    &mut purpose,
                    refusal.as_deref(),
                    select_name,
                ));
            },
            |ui| definition_pane(ui, session, component, &curve),
        );
        focus
    });

    state.dialogs.stimulus_link.name = name;
    state.dialogs.stimulus_link.purpose = purpose;
    state.dialogs.stimulus_link.select_name = false;
    if closed {
        state.dialogs.stimulus_link.close();
        return;
    }
    match choice {
        DialogChoice::Primary => match super::commit_extraction(state, session.component_id) {
            Ok(line) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::info(line));
                state.dialogs.stimulus_link.close();
            }
            Err(refusal) => state.dialogs.stimulus_link.error = Some(refusal),
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => state.dialogs.stimulus_link.close(),
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

/// The shape this definition publishes, over the window a stored shape is drawn
/// against.
fn source_curve(
    component: &Component,
    session: &StimulusLinkDialogState,
) -> Result<WaveformTrace, String> {
    let (window, _) = source_preview::shape_window(session.timing);
    source_preview::source_curve(component, window)
}

/// How tall the body is: what the right pane needs, never less than the form.
fn body_height(ui: &Ui) -> f32 {
    let form = 14.0
        + 2.0 * (16.0 + 5.0 + ui.spacing().interact_size.y)
        + 4.0
        + 18.0
        + 14.0
        + 16.0
        + 4.0 * shell::FACT_PITCH
        + 16.0;
    let pane = 16.0
        + source_preview::instrument_height(1, STRIP_HEIGHT)
        + 12.0
        + shell::card_block_height(1)
        + 12.0
        + 2.0 * shell::FACT_PITCH
        + 16.0;
    form.max(pane).max(MIN_BODY)
}

/// The left pane: the two authored fields, then what saving records.
fn form_pane(
    ui: &mut Ui,
    session: &StimulusLinkDialogState,
    component: &Component,
    name: &mut String,
    purpose: &mut String,
    refusal: Option<&str>,
    select_name: bool,
) -> egui::Id {
    let width = ui.available_width();
    let name_id = ui.id().with("stimulus-save-name");
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: shell::PANE_INSET,
            right: shell::PANE_INSET,
            top: 14,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.set_width(width - 2.0 * f32::from(shell::PANE_INSET));
            ui.spacing_mut().item_spacing.y = 0.0;
            text_edit_field(
                ui,
                "Name",
                egui::TextEdit::singleline(name).id(name_id),
                refusal,
                "The name this definition is listed and placed under",
            );
            if select_name {
                select_whole_name(ui, name_id, name);
            }
            ui.add_space(4.0);
            validation_line(ui, refusal);
            ui.add_space(14.0);
            input_field(
                ui,
                "Purpose",
                purpose,
                "Gate drive for the buck stage",
                None,
                "Shown beside the definition wherever it is offered",
            );
            ui.add_space(16.0);
            let becomes = format!("{} becomes", session.reference);
            shell::facts(
                ui,
                84.0,
                &[
                    ("Family", Fact::Plain(&family_line(component))),
                    ("Revision", Fact::Plain("r1")),
                    ("Stored in", Fact::Plain("Project document")),
                    // Where the instance stands now is in the header, two lines
                    // above this row; saying it again here only costs the
                    // narrowest column in the dialog the words it needs for
                    // where the instance ends up.
                    (becomes.as_str(), Fact::Plain("adopted \u{00b7} r1")),
                ],
            );
        });
    name_id
}

/// Put the whole generated name under the first keystroke.
///
/// The name is a suggestion assembled from the instance and its family, and a
/// reader who has one of their own should be able to type it without first
/// clearing eight characters they did not write.
fn select_whole_name(ui: &Ui, id: egui::Id, name: &str) {
    let mut state = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
    let end = egui::text::CCursor::new(name.chars().count());
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(0),
            end,
        )));
    state.store(ui.ctx(), id);
}

/// One line under the name field: what the netlist reader makes of it.
///
/// The refusal is the model's sentence unrewritten. The accepting case is a
/// mark and four words, because a field that says nothing while it is valid
/// leaves a reader unsure whether it was checked at all.
fn validation_line(ui: &mut Ui, refusal: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 18.0), Sense::hover());
    let (ink, message) = match refusal {
        Some(refusal) => (t.color.err, refusal),
        None => (t.color.ok, "Unique SPICE identifier"),
    };
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, message));
    let mut text_left = rect.left();
    if refusal.is_none() {
        check_mark(
            ui,
            Rect::from_center_size(
                egui::pos2(rect.left() + 5.0, rect.center().y),
                vec2(10.0, 10.0),
            ),
            ink,
        );
        text_left += 16.0;
    }
    let track = Rect::from_min_max(egui::pos2(text_left, rect.top()), rect.max);
    if shell::painted_run(
        ui,
        track,
        false,
        message,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        ink,
    ) {
        response.on_hover_text(message);
    }
}

/// The accepting mark, painted rather than typeset: the bundled Plex faces
/// carry no check, so a character here would rasterize as a tofu box.
fn check_mark(ui: &Ui, rect: Rect, ink: egui::Color32) {
    let stroke = Stroke::new(1.4, ink);
    let center = rect.center();
    ui.painter().line_segment(
        [
            egui::pos2(center.x - 4.0, center.y),
            egui::pos2(center.x - 1.0, center.y + 3.2),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(center.x - 1.0, center.y + 3.2),
            egui::pos2(center.x + 4.2, center.y - 3.4),
        ],
        stroke,
    );
}

/// The right pane: the published shape, the card, and what the samples say
/// about it.
fn definition_pane(
    ui: &mut Ui,
    session: &StimulusLinkDialogState,
    component: &Component,
    curve: &Result<WaveformTrace, String>,
) {
    let width = ui.available_width();
    let unit = placed_sources::source_unit(component);
    let (window, caption) = source_preview::shape_window(session.timing);
    egui::Frame::NONE
        .inner_margin(egui::Margin::same(shell::PANE_INSET))
        .show(ui, |ui| {
            ui.set_width(width - 2.0 * f32::from(shell::PANE_INSET));
            ui.spacing_mut().item_spacing.y = 0.0;
            source_preview::paint_instrument(
                ui,
                &source_preview::Instrument {
                    strips: &[source_preview::Strip {
                        curve,
                        unit,
                        accent: true,
                        caption: "Published card",
                        label: "the waveform this definition publishes",
                    }],
                    row_height: STRIP_HEIGHT,
                    stop: window.tstop,
                    window: Some(&caption),
                },
            );
            ui.add_space(12.0);
            shell::card_block(ui, &session.card, None);
            ui.add_space(12.0);
            let figure =
                placed_sources::source_key_figure_spelled(component, shell::display_spelling);
            let levels = curve
                .as_ref()
                .ok()
                .and_then(WaveformTrace::readouts)
                .map(|readouts| source_preview::level_span_label(readouts, unit));
            let mut rows: Vec<(&str, Fact<'_>)> = Vec::new();
            if !figure.is_empty() {
                rows.push(("Identified by", Fact::Plain(&figure)));
            }
            if let Some(levels) = levels.as_deref() {
                rows.push(("Levels", Fact::Plain(levels)));
            }
            shell::facts(ui, 84.0, &rows);
        });
}

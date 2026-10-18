//! Component evidence cards over live display records and the retained draft.

use super::{
    ComponentEditorContext, ComponentPropertyDialogResult, ComponentPropertyDraft,
    ComponentPropertyServices, section_band,
};
use egui::{Align, Layout, Margin, Ui};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{Problem, problem_line};

pub(super) fn evidence_pane(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    context: &ComponentEditorContext,
    show_source_preview: bool,
    services: &mut impl ComponentPropertyServices,
    action: &mut ComponentPropertyDialogResult,
) {
    // Read before the scroll area, which is the last place the pane's own
    // width is legible: inside it every rect has the scrollbar gutter taken
    // off, and the section bands head the pane rather than the content.
    let pane = ui.max_rect();
    // No fill of its own: the dialog body already painted one, and a second
    // coat that stopped where this pane's content stopped is what put a seam
    // down the middle of one surface.
    egui::ScrollArea::vertical()
        .id_salt("component-editor-evidence")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            evidence_contents(
                ui,
                draft,
                context,
                show_source_preview,
                services,
                action,
                pane,
            )
        });
}

fn evidence_contents(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    context: &ComponentEditorContext,
    show_source_preview: bool,
    services: &mut impl ComponentPropertyServices,
    action: &mut ComponentPropertyDialogResult,
    pane: egui::Rect,
) {
    stimulus_section(
        ui,
        draft,
        context,
        show_source_preview,
        services,
        action,
        pane,
    );
}

/// The colour a provenance chip takes.
///
/// Only the two states a reader has to act on are coloured: the library having
/// moved past the copy, or the definition having gone away, is a finding; a
/// local edit is deliberate and marked rather than flagged. This is the same
/// rule the Studio's Definition column follows, because it is the same fact.
///
/// Shared with the stimulus link dialog, whose header carries the same chip
/// over the same instance: two tables of this rule would let one surface call a
/// state a finding while the other beside it called it routine.
pub fn provenance_colour(
    ui: &Ui,
    provenance: rspice_design::stimulus_library::provenance::ProvenanceState,
) -> egui::Color32 {
    use rspice_design::stimulus_library::provenance::ProvenanceState;

    let c = Tokens::get(ui.ctx()).color;
    match provenance {
        ProvenanceState::Behind { .. }
        | ProvenanceState::ModifiedBehind { .. }
        | ProvenanceState::Removed { .. } => c.warn,
        ProvenanceState::Modified { .. } => c.accent,
        ProvenanceState::FromSchematic | ProvenanceState::Adopted { .. } => c.text_faint,
    }
}

/// What waveform this source carries, where it came from, and the verbs that
/// change that.
///
/// One section rather than two. The curve and the library record are one
/// subject — the curve *is* what the adopted card evaluates to — and showing
/// them as two banded blocks made one thing look like two, with the engine
/// named three times between them: once in a band title, once in its status,
/// and once in the plot's own caption.
///
/// The band's status is the definition this instance copied, which is the one
/// fact the two rows under the old card spent four lines on. The lifecycle
/// word is not repeated here: the dialog header already carries it, in the
/// same colours, two inches up.
///
/// Drawn whenever the dialog gave this pane its half, which it does for either
/// half of the section, and each half is drawn on what it needs alone. Both
/// used to wait on the library record, so a source whose record the host
/// could not resolve lost its waveform as well, and the editor showed an empty
/// pane beside the parameters.
fn stimulus_section(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    context: &ComponentEditorContext,
    show_source_preview: bool,
    services: &mut impl ComponentPropertyServices,
    action: &mut ComponentPropertyDialogResult,
    pane: egui::Rect,
) {
    let stimulus = context.stimulus.as_ref();
    if !show_source_preview && stimulus.is_none() {
        return;
    }
    section_block(ui, "Stimulus", pane, |ui| {
        if show_source_preview {
            services.preview_source(ui, draft);
        }
        if let Some(stimulus) = stimulus {
            stimulus_body(ui, draft, stimulus, action);
        }
    });
}

/// The note this source's provenance needs, and the verbs that move it.
///
/// Split from the section so the gate below can read the verbs without a
/// waveform evaluator behind it: what the verbs offer is decided by the draft
/// and the library, and neither needs a curve to be drawn.
///
/// Only two of the verbs are unconditional. Opening a definition and
/// re-adopting one are offers about a record the library may not hold and a
/// revision that may not exist, so they are absent rather than disabled: a
/// control that is here works, and the band above already states what there is
/// to open.
///
/// Three of the four act on the card the *instance* carries, not on the draft
/// in front of the reader, and all three leave this editor. While the draft
/// holds unapplied edits they are therefore disabled, with the reason on them:
/// a reader who retuned a frequency and pressed Save would otherwise publish
/// the card they had just edited away from, and the editor would close over
/// the edit without a word. This is the rule the model binding already keeps —
/// `Open model detail…` goes unavailable while the draft names a model the
/// instance has not been given — applied to the whole card, because adopting
/// and extracting read the whole card.
///
/// Opening the definition is the one verb that reads nothing of the draft: it
/// shows a library record the draft cannot change, exactly as `Open model
/// detail…` stays available while the draft's model is the committed one.
fn stimulus_body(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    stimulus: &super::StimulusEditorContext,
    action: &mut ComponentPropertyDialogResult,
) {
    let unapplied = draft
        .has_modifications()
        .then_some("Apply or cancel this editor's edits first");
    let held = stimulus
        .definition
        .as_deref()
        .zip(stimulus.library_revision);
    // Which definition this card came from, when it came from one. A source
    // drawn on the sheet adopted nothing, which is the ordinary state of a
    // source and not a row.
    match (held, stimulus.definition.as_deref()) {
        (Some((definition, revision)), _) => {
            evidence_row(
                ui,
                "Definition",
                &format!("{definition} \u{b7} r{revision}"),
            );
        }
        (None, Some(definition)) => {
            evidence_row(ui, "Definition", definition);
            problem_line(
                ui,
                Problem::Warning,
                "Definition deleted from the library; this source keeps its copy.",
            );
        }
        (None, None) => {}
    }
    let readopt = held
        .filter(|_| stimulus.state.offers_readoption())
        .map(|(_, revision)| format!("Re-adopt r{revision}"));
    let mut verbs = Vec::new();
    if !stimulus.library_is_empty {
        verbs.push(Verb {
            label: "Adopt definition…",
            unavailable: unapplied,
            hover: "Copy a library definition's card onto this instance and record which revision \
                    it came from",
            result: ComponentPropertyDialogResult::AdoptStimulus,
        });
    }
    verbs.push(Verb {
        label: "Save as library definition…",
        unavailable: unapplied,
        hover: "Publish this instance's card as a project stimulus definition",
        result: ComponentPropertyDialogResult::ExtractStimulus,
    });
    if held.is_some() {
        verbs.push(Verb {
            label: "Open in Stimulus Library",
            unavailable: None,
            hover: "Show this definition in the Stimulus Library workspace",
            result: ComponentPropertyDialogResult::OpenStimulusDefinition,
        });
    }
    if let Some(label) = readopt.as_deref() {
        verbs.push(Verb {
            label,
            unavailable: unapplied,
            hover: "Copy the library's revision onto this instance and reload this editor from it",
            result: ComponentPropertyDialogResult::ReadoptStimulus,
        });
    }
    ui.add_space(10.0);
    verb_grid(ui, &verbs, action);
}

/// One verb a section offers.
struct Verb<'a> {
    label: &'a str,
    /// Why it cannot be taken right now, if it cannot.
    unavailable: Option<&'a str>,
    hover: &'a str,
    result: ComponentPropertyDialogResult,
}

/// Columns a section's verbs are laid out in.
const VERB_COLUMNS: usize = 2;
const VERB_GAP: f32 = 8.0;
/// The narrowest a verb cell is drawn before the grid gives up on columns.
const VERB_MINIMUM: f32 = 150.0;

/// A section's verbs, on a grid of equal cells.
///
/// Equal cells rather than a wrapped row of shrink-to-fit buttons. Wrapping
/// sized every button to its own label and broke the line wherever the next
/// one stopped fitting, so four verbs came out as a long one beside a short
/// one over two more of different widths again — four sizes and two ragged
/// right edges for one set of equally weighted choices.
///
/// A verb that cannot be taken right now is drawn disabled and announces why,
/// rather than vanishing: the reason is a state the reader clears in one act,
/// and a control that disappeared would leave them looking for it. A verb that
/// is not on offer at all is absent, which is the caller's decision.
fn verb_grid(ui: &mut Ui, verbs: &[Verb<'_>], action: &mut ComponentPropertyDialogResult) {
    if verbs.is_empty() {
        return;
    }
    let track = ui.available_width();
    let columns = if track >= VERB_MINIMUM * VERB_COLUMNS as f32 + VERB_GAP {
        VERB_COLUMNS
    } else {
        1
    };
    let cell =
        ((track - VERB_GAP * (columns - 1) as f32) / columns as f32).max(VERB_MINIMUM.min(track));
    for row in verbs.chunks(columns) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = VERB_GAP;
            for verb in row {
                let response = rspice_ui_kit::widgets::Button::new(verb.label)
                    .enabled(verb.unavailable.is_none())
                    .min_width(cell)
                    .show(ui);
                match verb.unavailable {
                    Some(reason) => {
                        response.on_disabled_hover_text(reason);
                    }
                    None => {
                        if response.on_hover_text(verb.hover).clicked() {
                            *action = verb.result.clone();
                        }
                    }
                }
            }
        });
        ui.add_space(VERB_GAP);
    }
}

/// One banded section: its head, and its content inset under it.
///
/// The content carries no fill and no closing rule. It used to have both, and
/// a section is not the last thing in the pane it sits in: the fill ended where
/// the content did and the pane's own fill carried on past it, so the last
/// section closed with a rule across the width and a panel of a slightly
/// different grey beneath — a blank box the section appeared to have drawn on
/// purpose. The band is what separates one section from the next, and it is
/// enough on its own.
fn section_block(ui: &mut Ui, title: &str, pane: egui::Rect, body: impl FnOnce(&mut Ui)) {
    section_band(ui, title, pane);
    egui::Frame::NONE
        .inner_margin(Margin {
            left: 16,
            right: 16,
            top: 8,
            bottom: 12,
        })
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            body(ui);
        });
}

fn evidence_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.set_min_height(19.0);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                )
                .truncate(),
            );
        });
    });
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rspice_app_types::property::PropertyValue;

    /// The Stimulus section's verbs, as a screen reader receives them.
    fn stimulus_verbs(
        draft: &ComponentPropertyDraft,
        context: &ComponentEditorContext,
    ) -> Vec<(String, bool)> {
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut action = ComponentPropertyDialogResult::None;
        let stimulus = context.stimulus.clone().expect("a source context");
        ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                stimulus_body(ui, draft, &stimulus, &mut action);
            });
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| {
            node.label()
                .map(|label| (label.to_owned(), node.is_disabled()))
        })
        .collect()
    }

    /// One source that has adopted `sensor_drive`, with the library one
    /// revision past it so every verb is offered at once.
    fn adopted_context() -> ComponentEditorContext {
        use rspice_design::stimulus_library::provenance::ProvenanceState;

        ComponentEditorContext {
            stimulus: Some(super::super::StimulusEditorContext {
                state: ProvenanceState::Behind {
                    adopted: 1,
                    library: 2,
                },
                definition: Some("sensor_drive".to_owned()),
                library_revision: Some(2),
                library_is_empty: false,
            }),
            ..ComponentEditorContext::default()
        }
    }

    /// A preview service that draws a stand-in for the curve.
    struct Waveform;

    impl ComponentPropertyServices for Waveform {
        fn browse(
            &mut self,
            _: super::super::PropertyBrowseRequest<'_>,
            _: &mut ComponentPropertyDraft,
        ) {
        }
        fn preview_source(&mut self, ui: &mut Ui, _: &ComponentPropertyDraft) {
            ui.label("waveform");
        }
    }

    /// Whatever made the dialog give this pane its half is drawn in it.
    ///
    /// The dialog splits for a preview or for a library record, either one,
    /// and the section used to wait on the record for both of its halves: a
    /// source whose record the host could not resolve got a pane beside its
    /// parameters with nothing in it at all.
    #[test]
    fn the_stimulus_pane_draws_whichever_half_it_was_given() {
        let painted = |context: &ComponentEditorContext, show_source_preview: bool| {
            let ctx = egui::Context::default();
            rspice_ui_kit::Theme::default().apply(&ctx);
            let draft = ComponentPropertyDraft::default();
            let mut action = ComponentPropertyDialogResult::None;
            ctx.run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    evidence_pane(
                        ui,
                        &draft,
                        context,
                        show_source_preview,
                        &mut Waveform,
                        &mut action,
                    );
                });
            })
            .shapes
            .into_iter()
            .filter_map(|clipped| match clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect::<Vec<_>>()
        };
        let has = |texts: &[String], wanted: &str| texts.iter().any(|text| text == wanted);

        let unresolved = painted(&ComponentEditorContext::default(), true);
        assert!(
            has(&unresolved, "Stimulus") && has(&unresolved, "waveform"),
            "a source with no library record still shows its curve: {unresolved:?}"
        );
        let record_only = painted(&adopted_context(), false);
        assert!(
            has(&record_only, "Stimulus") && has(&record_only, "Save as library definition…"),
            "a library record shows its verbs without a curve: {record_only:?}"
        );
    }

    /// A verb that acts on the instance's card is withheld while the editor
    /// holds edits that card does not have yet.
    ///
    /// Adopting, saving and re-adopting all read or replace the whole card and
    /// all three leave this editor; a reader who retuned a frequency and
    /// pressed Save would otherwise publish the card they had just edited away
    /// from, with the edit discarded and nothing said. Opening the definition
    /// reads nothing of the draft, so it stays available — which is the rule
    /// `Open model detail…` already keeps for the model binding.
    #[test]
    fn the_card_verbs_are_withheld_while_the_editor_holds_unapplied_edits() {
        let mut draft = ComponentPropertyDraft::default();
        let context = adopted_context();

        let clean = stimulus_verbs(&draft, &context);
        for verb in [
            "Adopt definition…",
            "Save as library definition…",
            "Open in Stimulus Library",
            "Re-adopt r2",
        ] {
            assert!(
                clean
                    .iter()
                    .any(|(label, disabled)| label == verb && !disabled),
                "{verb} is offered on a clean editor: {clean:?}"
            );
        }

        draft.set_value("freq", PropertyValue::string("2k"));
        assert!(draft.has_modifications());
        let dirty = stimulus_verbs(&draft, &context);
        for verb in [
            "Adopt definition…",
            "Save as library definition…",
            "Re-adopt r2",
        ] {
            assert!(
                dirty
                    .iter()
                    .any(|(label, disabled)| label == verb && *disabled),
                "{verb} must not act on a card the editor has edited away from: {dirty:?}"
            );
        }
        assert!(
            dirty
                .iter()
                .any(|(label, disabled)| label == "Open in Stimulus Library" && !disabled),
            "opening the definition reads nothing of the draft: {dirty:?}"
        );
    }
}

//! Component evidence cards over live display records and the retained draft.

use super::{
    ComponentEditorContext, ComponentPropertyDialogResult, ComponentPropertyDraft,
    ComponentPropertyServices, section_band,
};
use egui::{Align, Layout, Margin, Stroke, Ui};
use rspice_app_types::property::PropertyValue;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

pub(super) fn evidence_pane(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    context: &ComponentEditorContext,
    show_source_preview: bool,
    services: &mut impl ComponentPropertyServices,
    action: &mut ComponentPropertyDialogResult,
) {
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("component-editor-evidence")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    evidence_contents(ui, draft, context, show_source_preview, services, action)
                });
        });
}

fn evidence_contents(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    context: &ComponentEditorContext,
    show_source_preview: bool,
    services: &mut impl ComponentPropertyServices,
    action: &mut ComponentPropertyDialogResult,
) {
    model_binding_card(ui, draft, context, action);
    operating_point_card(ui, context, action);
    if show_source_preview {
        services.preview_source(ui, draft);
        stimulus_library_card(ui, draft, context, action);
    }
    terminals_card(ui, context);
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

/// Where this source stands with the project's stimulus library, and the four
/// verbs that move it.
///
/// Only two of the verbs are unconditional. Opening a definition and
/// re-adopting one are offers about a record the library may not hold and a
/// revision that may not exist, so they are absent rather than disabled: a
/// control that is here works, and the status line above already states why
/// there is nothing to open.
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
fn stimulus_library_card(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    context: &ComponentEditorContext,
    action: &mut ComponentPropertyDialogResult,
) {
    let Some(stimulus) = context.stimulus.as_ref() else {
        return;
    };
    let unapplied = draft
        .has_modifications()
        .then_some("Apply or cancel this editor's edits first");
    let held = stimulus
        .definition
        .as_deref()
        .zip(stimulus.library_revision);
    section_block(ui, "Stimulus library", &stimulus.state.label(), |ui| {
        let t = Tokens::get(ui.ctx());
        match held {
            Some((definition, revision)) => {
                evidence_row(ui, "Definition", definition);
                evidence_row(ui, "Library holds", &format!("r{revision}"));
            }
            None if stimulus.definition.is_some() => {
                evidence_row(
                    ui,
                    "Definition",
                    stimulus.definition.as_deref().unwrap_or_default(),
                );
                ui.label(
                    egui::RichText::new(
                        "The library no longer holds it. This instance keeps the card it copied.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.warn),
                );
            }
            None => {
                ui.label(
                    egui::RichText::new(if stimulus.library_is_empty {
                        "This project has authored no stimulus definitions yet."
                    } else {
                        "This source was drawn on the sheet and adopted no definition."
                    })
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            }
        }
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            if !stimulus.library_is_empty
                && stimulus_verb(
                    ui,
                    "Adopt definition…",
                    unapplied,
                    "Copy a library definition's card onto this instance and record which \
                     revision it came from",
                )
            {
                *action = ComponentPropertyDialogResult::AdoptStimulus;
            }
            if stimulus_verb(
                ui,
                "Save as library definition…",
                unapplied,
                "Publish this instance's card as a project stimulus definition",
            ) {
                *action = ComponentPropertyDialogResult::ExtractStimulus;
            }
            if held.is_some()
                && stimulus_verb(
                    ui,
                    "Open in Stimulus Library",
                    None,
                    "Show this definition in the Stimulus Library workspace",
                )
            {
                *action = ComponentPropertyDialogResult::OpenStimulusDefinition;
            }
            let readopt = held
                .filter(|_| stimulus.state.offers_readoption())
                .map(|(_, revision)| format!("Re-adopt r{revision}"));
            if let Some(label) = readopt.as_deref()
                && stimulus_verb(
                    ui,
                    label,
                    unapplied,
                    "Copy the library's revision onto this instance and reload this editor from \
                     it",
                )
            {
                *action = ComponentPropertyDialogResult::ReadoptStimulus;
            }
        });
    });
}

/// One verb in the Stimulus library block.
///
/// A verb that cannot be taken right now is drawn disabled and announces why,
/// rather than vanishing: the reason is a state the reader clears in one act,
/// and a control that disappeared would leave them looking for it.
fn stimulus_verb(ui: &mut Ui, label: &str, unavailable: Option<&str>, hover: &str) -> bool {
    let response = rspice_ui_kit::widgets::Button::new(label)
        .enabled(unavailable.is_none())
        .show(ui);
    match unavailable {
        Some(reason) => {
            response.on_disabled_hover_text(reason);
            false
        }
        None => response.on_hover_text(hover).clicked(),
    }
}

fn section_block(ui: &mut Ui, title: &str, status: &str, body: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    section_band(ui, title, status);
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin {
            left: 16,
            right: 16,
            top: 4,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            body(ui);
        });
    let y = ui.cursor().top();
    ui.painter()
        .hline(ui.max_rect().x_range(), y, Stroke::new(1.0, t.color.border));
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

fn model_binding_card(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    context: &ComponentEditorContext,
    action: &mut ComponentPropertyDialogResult,
) {
    let draft_model = draft
        .get_value("model")
        .map(PropertyValue::display_string)
        .filter(|model| !model.trim().is_empty());
    let draft_library = draft
        .get_value("model_library")
        .map(PropertyValue::display_string)
        .filter(|library| !library.trim().is_empty());
    let pending_model = context.model.as_ref().and_then(|model| {
        let identity_changed = draft_model
            .as_deref()
            .is_some_and(|draft| !draft.eq_ignore_ascii_case(&model.name))
            || draft_library.as_deref().is_some_and(|library| {
                model
                    .library
                    .as_deref()
                    .is_none_or(|resolved| !library.eq_ignore_ascii_case(resolved))
            });
        identity_changed.then(|| draft_model.as_deref().unwrap_or(&model.name))
    });
    let status = if pending_model.is_some() {
        "pending"
    } else {
        context
            .model
            .as_ref()
            .map(|model| {
                if model.status.contains("resolved") {
                    "qualified"
                } else if model.status.contains("inline") || model.status.contains("exact") {
                    "exact"
                } else {
                    "unverified"
                }
            })
            .unwrap_or("not bound")
    };

    section_block(ui, "Model binding", status, |ui| {
        if let Some(model) = &context.model {
            let t = Tokens::get(ui.ctx());
            if let Some(pending_model) = pending_model {
                evidence_row(ui, "Model", pending_model);
                evidence_row(
                    ui,
                    "Source",
                    draft_library.as_deref().unwrap_or("Pending validation"),
                );
                evidence_row(ui, "Section", &model.section);
                ui.label(
                    egui::RichText::new("Apply to resolve the new model binding.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.warn),
                );
            } else {
                evidence_row(ui, "Model", &model.name);
                evidence_row(ui, "Source", &model.source);
                evidence_row(ui, "Section", &model.section);
                ui.label(
                    egui::RichText::new(&model.status)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            }
            if model.can_open || model.can_qualify {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if rspice_ui_kit::widgets::Button::new("Open model detail…")
                        .enabled(model.can_open && pending_model.is_none())
                        .show(ui)
                        .clicked()
                    {
                        *action = ComponentPropertyDialogResult::OpenModel;
                    }
                    if rspice_ui_kit::widgets::Button::new("Qualification…")
                        .enabled(model.can_qualify && pending_model.is_none())
                        .show(ui)
                        .clicked()
                    {
                        *action = ComponentPropertyDialogResult::OpenQualification;
                    }
                });
            }
        } else if let Some(model) = draft_model.as_deref() {
            evidence_row(ui, "Model", model);
            evidence_row(ui, "Source", "No catalog source resolved");
            evidence_row(ui, "Section", "default");
        } else {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new("This component has no model binding.")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        }
    });
}

fn operating_point_card(
    ui: &mut Ui,
    context: &ComponentEditorContext,
    action: &mut ComponentPropertyDialogResult,
) {
    let status = context
        .operating_point
        .as_ref()
        .map(|operating_point| {
            format!(
                "Run {} · {}{}",
                operating_point.run_id,
                operating_point.analysis,
                if operating_point.current {
                    ""
                } else {
                    " · stale"
                }
            )
        })
        .unwrap_or_else(|| "no retained run".to_owned());
    section_block(ui, "Evaluated at operating point", &status, |ui| {
        if let Some(operating_point) = &context.operating_point {
            for (label, value) in &operating_point.rows {
                evidence_row(ui, label, value);
            }
            ui.add_space(8.0);
            if rspice_ui_kit::widgets::Button::new("Cross-probe in results…")
                .show(ui)
                .clicked()
            {
                *action = ComponentPropertyDialogResult::CrossProbe;
            }
        } else {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new("No retained device operating point")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.label(
                egui::RichText::new("Run a DC operating-point analysis to populate this card.")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        }
    });
}

fn terminals_card(ui: &mut Ui, context: &ComponentEditorContext) {
    let open_count = context
        .terminals
        .iter()
        .filter(|terminal| terminal.net.is_none())
        .count();
    let status = if context.terminals.is_empty() {
        "none declared".to_owned()
    } else if open_count == 0 {
        "all bound".to_owned()
    } else {
        format!("{open_count} open")
    };
    section_band(ui, "Terminals", &status);
    if context.terminals.is_empty() {
        let t = Tokens::get(ui.ctx());
        egui::Frame::NONE
            .fill(t.color.bg_panel)
            .inner_margin(Margin::symmetric(16, 10))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("This component has no declared terminals.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        return;
    }

    terminal_table_row(ui, "PIN", "DIRECTION", "NET", true, false);
    for terminal in &context.terminals {
        terminal_table_row(
            ui,
            &terminal.pin,
            &terminal.direction,
            terminal.net.as_deref().unwrap_or("open"),
            false,
            terminal.net.is_none(),
        );
    }
}

fn terminal_table_row(
    ui: &mut Ui,
    pin: &str,
    direction: &str,
    net: &str,
    heading: bool,
    open: bool,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    // The row must span the whole evidence pane: a shrink-to-fit frame would
    // stop its fill and its bottom rule at the widest cell, leaving the table
    // narrower than the band above it.
    let row_width = ui.available_width();
    let frame = egui::Frame::NONE
        .fill(if heading { c.bg_panel_2 } else { c.bg_panel })
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_width(row_width - 20.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                let font = if heading {
                    theme::mono(tokens::FS_0, FontWeight::Medium)
                } else {
                    theme::mono(tokens::FS_0, FontWeight::Regular)
                };
                let color = if heading { c.text_faint } else { c.text };
                ui.add_sized(
                    [62.0, 16.0],
                    egui::Label::new(egui::RichText::new(pin).font(font.clone()).color(color)),
                );
                ui.add_sized(
                    [88.0, 16.0],
                    egui::Label::new(
                        egui::RichText::new(direction)
                            .font(if heading {
                                font.clone()
                            } else {
                                theme::sans(tokens::FS_0, FontWeight::Regular)
                            })
                            .color(if heading { c.text_faint } else { c.text_dim }),
                    ),
                );
                ui.add(
                    egui::Label::new(egui::RichText::new(net).font(font).color(if open {
                        c.warn
                    } else {
                        color
                    }))
                    .truncate(),
                );
            });
        });
    ui.painter().hline(
        frame.response.rect.x_range(),
        frame.response.rect.bottom(),
        Stroke::new(1.0, c.border),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    /// The stimulus library block, as a screen reader receives it.
    #[cfg(not(target_arch = "wasm32"))]
    fn stimulus_verbs(
        draft: &ComponentPropertyDraft,
        context: &ComponentEditorContext,
    ) -> Vec<(String, bool)> {
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut action = ComponentPropertyDialogResult::None;
        ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                stimulus_library_card(ui, draft, context, &mut action);
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
    #[cfg(not(target_arch = "wasm32"))]
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

    /// A verb that acts on the instance's card is withheld while the editor
    /// holds edits that card does not have yet.
    ///
    /// Adopting, saving and re-adopting all read or replace the whole card and
    /// all three leave this editor; a reader who retuned a frequency and
    /// pressed Save would otherwise publish the card they had just edited away
    /// from, with the edit discarded and nothing said. Opening the definition
    /// reads nothing of the draft, so it stays available — which is the rule
    /// `Open model detail…` already keeps for the model binding.
    #[cfg(not(target_arch = "wasm32"))]
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

//! Component modal presentation with host-owned services and publication.

use super::evidence::{evidence_pane, provenance_colour};
use super::{
    ComponentEditorContext, ComponentPropertyDialogResult, ComponentPropertyDraft,
    PropertyBrowseRequest, render_component_parameters,
};
use egui::{Align, Layout, Margin, Sense, Stroke, Ui, vec2};
use rspice_app_types::property::{PropertySheet, PropertyValue};
use rspice_app_types::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use rspice_design::properties::SourceContractFinding;
use rspice_design::schematic::document_policy::PropertyCommitPolicy;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};

/// Live host inputs for one component dialog frame.
pub struct ComponentPropertyDialogView<'a> {
    pub component_name: Option<&'a str>,
    pub context: &'a ComponentEditorContext,
    pub sheet: &'a PropertySheet,
    pub advisories: &'a [SourceContractFinding],
    pub session_error: Option<String>,
    pub model_browser_open: bool,
    pub show_source_preview: bool,
    pub quantity_policy: QuantityPresentationPolicy,
    pub number_locale: UiNumberLocale,
    pub commit_policy: PropertyCommitPolicy,
}

/// Services called at their original position within the component dialog.
pub trait ComponentPropertyServices {
    /// Resolve Browse after field edits and before painting its new hint.
    fn browse(&mut self, request: PropertyBrowseRequest<'_>, draft: &mut ComponentPropertyDraft);
    /// Paint the host's evaluated source after this frame's parameter edits.
    fn preview_source(&mut self, ui: &mut Ui, draft: &ComponentPropertyDraft);
}

const DIALOG_SIZE: DialogSize = DialogSize::ComponentEditor;
const EYEBROW: &str = "EDIT · TYPED PARAMETERS";
const DESCRIPTION: &str = "Edit identity, model, parameters, orientation, connectivity, display, constraints, and review metadata.";

/// Render one active component dialog and return a typed host action.
/// The host closes cancelled dialogs and publishes prepared edits after return.
pub fn render_component_property_dialog(
    ctx: &egui::Context,
    draft: &mut ComponentPropertyDraft,
    view: ComponentPropertyDialogView<'_>,
    services: &mut impl ComponentPropertyServices,
) -> ComponentPropertyDialogResult {
    let ComponentPropertyDialogView {
        component_name,
        context,
        sheet,
        advisories,
        session_error,
        model_browser_open,
        show_source_preview,
        quantity_policy,
        number_locale,
        commit_policy,
    } = view;
    let mut result = ComponentPropertyDialogResult::None;
    draft.sync_pwl_validation_error();
    let dirty = draft.has_modifications();
    let footer_hint = session_error
        .clone()
        .or_else(|| dirty.then(|| "Unapplied changes".to_owned()));

    let mut dialog = Dialog::new(EYEBROW, "Edit instance properties", "OK")
        .description(DESCRIPTION)
        .size(DIALOG_SIZE)
        .fixed_height(680.0)
        .without_header()
        .flush_body()
        .manual_body_scroll()
        .ghost("Cancel")
        .secondary("Apply")
        .secondary_enabled(dirty && draft.can_apply(commit_policy) && session_error.is_none())
        .primary_enabled(session_error.is_none() && (!dirty || draft.can_apply(commit_policy)))
        .interaction_enabled(!model_browser_open)
        .initial_focus(DialogInitialFocus::BodyControl);
    if let Some(hint) = footer_hint.as_deref() {
        dialog = dialog.hint(hint);
    }

    let mut side_action = ComponentPropertyDialogResult::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        component_identity_header(ui, draft, component_name, context);
        let body_height = ui.available_height().max(1.0);
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            Tokens::get(ui.ctx()).color.bg_panel,
        );
        let wide = ctx.content_rect().width() > 760.0;
        let mut first_parameter = None;
        if wide {
            let gap = 1.0;
            let left_width = ((ui.available_width() - gap) * (1.1 / 2.1)).max(300.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(left_width, body_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        first_parameter = render_component_parameters(
                            ui,
                            draft,
                            sheet,
                            advisories,
                            quantity_policy,
                            number_locale,
                            &mut |request, draft| services.browse(request, draft),
                        );
                    },
                );
                let (divider, _) = ui.allocate_exact_size(vec2(gap, body_height), Sense::hover());
                ui.painter()
                    .rect_filled(divider, 0.0, Tokens::get(ui.ctx()).color.border);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), body_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        evidence_pane(
                            ui,
                            draft,
                            context,
                            show_source_preview,
                            services,
                            &mut side_action,
                        )
                    },
                );
            });
        } else {
            let gap = 1.0;
            let parameters_height = ((body_height - gap) * 0.56).max(1.0);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), parameters_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        first_parameter = render_component_parameters(
                            ui,
                            draft,
                            sheet,
                            advisories,
                            quantity_policy,
                            number_locale,
                            &mut |request, draft| services.browse(request, draft),
                        );
                    },
                );
                let (divider, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), gap), Sense::hover());
                ui.painter()
                    .rect_filled(divider, 0.0, Tokens::get(ui.ctx()).color.border);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), ui.available_height()),
                    Layout::top_down(Align::Min),
                    |ui| {
                        evidence_pane(
                            ui,
                            draft,
                            context,
                            show_source_preview,
                            services,
                            &mut side_action,
                        )
                    },
                );
            });
        }
        first_parameter
    });

    let dirty_after_render = draft.has_modifications();
    if side_action != ComponentPropertyDialogResult::None {
        result = side_action;
    } else {
        match choice {
            DialogChoice::Primary => {
                if !dirty_after_render {
                    result = ComponentPropertyDialogResult::Cancelled;
                } else if draft.prepare_commit(sheet, commit_policy) {
                    result = if draft.validation_errors.is_empty() {
                        ComponentPropertyDialogResult::AppliedAndClose
                    } else {
                        ComponentPropertyDialogResult::Applied
                    };
                }
            }
            DialogChoice::Secondary => {
                if dirty_after_render && draft.prepare_commit(sheet, commit_policy) {
                    result = ComponentPropertyDialogResult::Applied;
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                result = ComponentPropertyDialogResult::Cancelled;
            }
            DialogChoice::None => {}
        }
    }

    result
}

fn component_identity_header(
    ui: &mut Ui,
    draft: &mut ComponentPropertyDraft,
    component_name: Option<&str>,
    context: &ComponentEditorContext,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let frame = egui::Frame::NONE
        .fill(c.bg_panel_2)
        .inner_margin(Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.set_width(width - 32.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;
                let (glyph, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
                ui.painter().rect_filled(glyph, 5.0, c.bg_panel);
                ui.painter().rect_stroke(
                    glyph,
                    5.0,
                    Stroke::new(1.0, c.border),
                    egui::StrokeKind::Inside,
                );
                ui.painter().text(
                    glyph.center(),
                    egui::Align2::CENTER_CENTER,
                    &context.glyph,
                    theme::mono(tokens::FS_2, FontWeight::SemiBold),
                    c.accent,
                );

                // A source carrying a provenance chip needs room for the chip
                // *and* the family beside it; everything else keeps the track
                // the family alone has always had.
                let status_width = if context.stimulus.is_some() {
                    (ui.available_width() * 0.44).clamp(200.0, 330.0)
                } else {
                    (ui.available_width() * 0.28).clamp(110.0, 190.0)
                };
                let identity_width =
                    (ui.available_width() - status_width - ui.spacing().item_spacing.x).max(180.0);
                ui.allocate_ui_with_layout(
                    vec2(identity_width, 34.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        // Claim the whole track. `allocate_ui_with_layout`
                        // advances the cursor by the content it ends up with,
                        // not by the size it was asked for, so a short instance
                        // path would otherwise drag the family badge in off the
                        // right edge instead of leaving it flush.
                        ui.set_min_width(identity_width);
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 7.0;
                            ui.label(
                                egui::RichText::new("Instance")
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_dim),
                            );
                            let mut name = draft
                                .get_value("name")
                                .map(PropertyValue::display_string)
                                .or_else(|| component_name.map(str::to_owned))
                                .unwrap_or_default();
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut name)
                                    .font(theme::mono(tokens::FS_1, FontWeight::SemiBold))
                                    .desired_width(88.0),
                            );
                            if response.changed() {
                                draft.set_value("name", PropertyValue::String(name));
                            }
                            ui.label(
                                egui::RichText::new(&context.library_cell)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_faint),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            ui.label(
                                egui::RichText::new(&context.subtitle)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_dim),
                            );
                            ui.label(
                                egui::RichText::new("·")
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_faint),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&context.instance_path)
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(c.text_dim),
                                )
                                .truncate(),
                            );
                        });
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(status_width, 34.0),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.set_min_width(status_width);
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&context.family)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_dim),
                            )
                            .truncate(),
                        );
                        if let Some(stimulus) = &context.stimulus {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(stimulus.state.label())
                                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                        .color(provenance_colour(ui, stimulus.state)),
                                )
                                .truncate(),
                            );
                        }
                    },
                );
            });
        });
    ui.painter().hline(
        frame.response.rect.x_range(),
        frame.response.rect.bottom(),
        Stroke::new(1.0, c.border),
    );
}

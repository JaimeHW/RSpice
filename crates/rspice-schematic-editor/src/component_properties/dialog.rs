//! Component modal presentation with host-owned services and publication.

use super::evidence::evidence_pane;
use super::{
    ComponentEditorContext, ComponentPropertyDialogResult, ComponentPropertyDraft,
    PropertyBrowseRequest, render_component_parameters,
};
use egui::{Align, Layout, Margin, Stroke, Ui};
use rspice_app_types::property::{PropertySheet, PropertyValue};
use rspice_app_types::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use rspice_design::properties::SourceContractFinding;
use rspice_design::schematic::document_policy::PropertyCommitPolicy;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{
    Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize,
};

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

/// What a screen reader is told this dialog is for.
///
/// It is a parameter form: it edits no orientation, no display option, no
/// constraint and no review metadata, all of which the description used to
/// promise.
fn accessible_description(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Edit {name}'s parameters."),
        None => "Edit this instance's parameters.".to_owned(),
    }
}

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
    let (footer_hint, footer_tone) = match session_error.clone() {
        Some(error) => (Some(error), DialogHintTone::Error),
        None => (
            dirty.then(|| "Unapplied changes".to_owned()),
            DialogHintTone::Neutral,
        ),
    };

    let described = accessible_description(
        draft
            .get_value("name")
            .map(PropertyValue::display_string)
            .filter(|name| !name.trim().is_empty())
            .as_deref()
            .or(component_name),
    );
    let evidence = has_evidence(context, show_source_preview);
    let mut dialog = Dialog::new("Edit instance properties", "OK")
        .description(&described)
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
        dialog = dialog.hint(hint).hint_tone(footer_tone);
    }
    if !evidence {
        dialog = dialog.width_from_content(PARAMETERS_ONLY_WIDTH);
    }

    let mut side_action = ComponentPropertyDialogResult::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        component_identity_header(ui, draft, component_name, context);
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            Tokens::get(ui.ctx()).color.bg_panel,
        );
        let wide = ctx.content_rect().width() > 760.0;
        let first_parameter;
        // With nothing to show beside them, the parameters are the whole body,
        // on a surface narrowed to them rather than one with a pane that holds
        // a divider and no content.
        if !evidence {
            clip_to_pane(ui);
            first_parameter = render_component_parameters(
                ui,
                draft,
                sheet,
                advisories,
                quantity_policy,
                number_locale,
                &mut |request, draft| services.browse(request, draft),
            );
        } else if wide {
            let (left, divider, right) = split_across(ui.available_rect_before_wrap());
            let mut pane = ui.new_child(pane_builder(left));
            clip_to_pane(&mut pane);
            first_parameter = render_component_parameters(
                &mut pane,
                draft,
                sheet,
                advisories,
                quantity_policy,
                number_locale,
                &mut |request, draft| services.browse(request, draft),
            );
            ui.painter()
                .rect_filled(divider, 0.0, Tokens::get(ui.ctx()).color.border);
            let mut pane = ui.new_child(pane_builder(right));
            clip_to_pane(&mut pane);
            evidence_pane(
                &mut pane,
                draft,
                context,
                show_source_preview,
                services,
                &mut side_action,
            );
            ui.advance_cursor_after_rect(ui.available_rect_before_wrap());
        } else {
            let (top, divider, bottom) = split_down(ui.available_rect_before_wrap());
            let mut pane = ui.new_child(pane_builder(top));
            clip_to_pane(&mut pane);
            first_parameter = render_component_parameters(
                &mut pane,
                draft,
                sheet,
                advisories,
                quantity_policy,
                number_locale,
                &mut |request, draft| services.browse(request, draft),
            );
            ui.painter()
                .rect_filled(divider, 0.0, Tokens::get(ui.ctx()).color.border);
            let mut pane = ui.new_child(pane_builder(bottom));
            clip_to_pane(&mut pane);
            evidence_pane(
                &mut pane,
                draft,
                context,
                show_source_preview,
                services,
                &mut side_action,
            );
            ui.advance_cursor_after_rect(ui.available_rect_before_wrap());
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

/// The rule between two panes, and the only space between them.
const PANE_RULE: f32 = 1.0;

/// How much of a side-by-side body the parameters take.
const PARAMETERS_SHARE: f32 = 1.1 / 2.1;

/// The narrowest the parameters pane is allowed to get beside the evidence.
const PARAMETERS_MINIMUM: f32 = 300.0;

/// How much of a stacked body the parameters take.
const STACKED_PARAMETERS_SHARE: f32 = 0.56;

/// The surface width of an editor that holds its parameters alone.
///
/// The parameters pane of the two-pane surface and the surface's border, so a
/// field is as wide in a resistor's editor as it is beside a source's
/// waveform. A form on the whole two-pane surface stretched its grid into
/// wells over 400 points wide, for values such as `1k`.
const PARAMETERS_ONLY_WIDTH: f32 = 461.0;

/// A body split into two panes and the rule between them.
///
/// Worked out from the body rather than laid out one pane after the other,
/// because a pane is a scroll area and a scroll area does not occupy the rect
/// it is given: with its bar visible it reports an outer rect of the content
/// *plus* the bar, so the pane after it starts that much further along. The
/// amount converges over the first frames as the bar settles, so the rule and
/// the second pane moved on every one of them, and the first pane's section
/// band — which spans the rect the pane was given — stopped 17 points short of
/// a rule that had been pushed away from it.
fn split_across(body: egui::Rect) -> (egui::Rect, egui::Rect, egui::Rect) {
    let width = ((body.width() - PANE_RULE) * PARAMETERS_SHARE).max(PARAMETERS_MINIMUM);
    let split = (body.left() + width).round();
    (
        egui::Rect::from_x_y_ranges(body.left()..=split, body.y_range()),
        egui::Rect::from_x_y_ranges(split..=split + PANE_RULE, body.y_range()),
        egui::Rect::from_x_y_ranges(split + PANE_RULE..=body.right(), body.y_range()),
    )
}

/// [`split_across`] for a body too narrow to hold the panes side by side.
fn split_down(body: egui::Rect) -> (egui::Rect, egui::Rect, egui::Rect) {
    let height = ((body.height() - PANE_RULE) * STACKED_PARAMETERS_SHARE).max(1.0);
    let split = (body.top() + height).round();
    (
        egui::Rect::from_x_y_ranges(body.x_range(), body.top()..=split),
        egui::Rect::from_x_y_ranges(body.x_range(), split..=split + PANE_RULE),
        egui::Rect::from_x_y_ranges(body.x_range(), split + PANE_RULE..=body.bottom()),
    )
}

/// A pane laid out at the rect the split gave it.
fn pane_builder(rect: egui::Rect) -> egui::UiBuilder {
    egui::UiBuilder::new()
        .max_rect(rect)
        .layout(Layout::top_down(Align::Min))
}

/// Confine a pane's painting to the rectangle it was allocated.
///
/// A pane's own rect is the only honest answer to "how wide is this surface",
/// and the section bands inside it need that answer: without it the band has
/// to guess how much of the scroll area's reserved gutter to reach into, and a
/// guess is either a notch beside every title or a bar painted out past the
/// dialog's border. The rasterizer hides the second case, because it clips a
/// dialog to the canvas; the running editor does not.
fn clip_to_pane(ui: &mut Ui) {
    ui.set_clip_rect(ui.clip_rect().intersect(ui.max_rect()));
}

/// Whether this component has anything to show beside its parameters.
///
/// Only a source does: its waveform, and the library verbs that act on the
/// card it carries. Everything else is a parameter form and nothing else, as
/// Virtuoso's Edit Object Properties and ADS's component dialog are. What the
/// pane used to hold beside them — the model binding, the operating point and
/// the terminal table — is the design inspector's, which this modal covers
/// while it is open.
///
/// `show_source_preview` is part of the question and not a duplicate of the
/// stimulus context, even though the host derives both from the same kind: the
/// evidence pane is what calls the preview service, so a source whose
/// provenance failed to resolve would otherwise lose its waveform rather than
/// its provenance line.
fn has_evidence(context: &ComponentEditorContext, show_source_preview: bool) -> bool {
    show_source_preview || context.stimulus.is_some()
}

/// Which instance this form is editing, and what it is.
///
/// Two lines and nothing else. The header used to carry a glyph box, the
/// instance's path, the device family and a provenance chip beside a subtitle
/// that named the device a third time — `R1  R / Resistor` over
/// `Resistor · SPICE primitive · lib/amp/schematic/R1`, with `Resistor ·
/// primitive` flush right. The name is editable, the master is named for an
/// instance that has one, and what the part is takes one word.
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
            ui.vertical(|ui| {
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
                    // Empty for a primitive, whose "library" is the engine.
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&context.library_cell)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.text_faint),
                        )
                        .truncate(),
                    );
                });
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&context.subtitle)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_dim),
                    )
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rspice_design::properties::{PropertyCatalog, component::collect_properties_with_sheet};
    use rspice_design::schematic::component::Component;
    use rspice_design::schematic::component_type::ComponentType;
    use rspice_design_model::Point;

    struct NoServices;

    impl ComponentPropertyServices for NoServices {
        fn browse(&mut self, _: PropertyBrowseRequest<'_>, _: &mut ComponentPropertyDraft) {}
        fn preview_source(&mut self, _: &mut Ui, _: &ComponentPropertyDraft) {}
    }

    /// How far apart the two columns of a freshly opened `kind`'s editor
    /// start, read off the captions `left` and `right` on its first row.
    fn column_pitch(
        kind: ComponentType,
        show_source_preview: bool,
        left: &str,
        right: &str,
    ) -> f32 {
        let catalog = PropertyCatalog::new();
        let sheet = catalog.get(kind).expect("catalog sheet").clone();
        let component = Component::new(1, kind, Point::origin());
        let mut draft = ComponentPropertyDraft::default();
        draft.reset(
            kind,
            &sheet,
            collect_properties_with_sheet(&component, Some(&sheet)),
        );
        let context = ComponentEditorContext::default();
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let mut frame = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    render_component_property_dialog(
                        ui.ctx(),
                        &mut draft,
                        ComponentPropertyDialogView {
                            component_name: Some("X1"),
                            context: &context,
                            sheet: &sheet,
                            advisories: &[],
                            session_error: None,
                            model_browser_open: false,
                            show_source_preview,
                            quantity_policy: QuantityPresentationPolicy::default(),
                            number_locale: UiNumberLocale::default(),
                            commit_policy: PropertyCommitPolicy::default(),
                        },
                        &mut NoServices,
                    );
                },
            )
        };
        drop(frame());
        drop(frame());
        let shapes = frame().shapes;
        let x = |wanted: &str| {
            shapes
                .iter()
                .find_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) if text.galley.text() == wanted => Some(text.pos.x),
                    _ => None,
                })
                .unwrap_or_else(|| panic!("{wanted:?} was not painted"))
        };
        x(right) - x(left)
    }

    /// A field is as wide in an editor that holds parameters alone as it is
    /// beside a source's waveform.
    ///
    /// A resistor's editor used to take the whole two-pane surface for its
    /// one pane, which made each of its columns twice as wide as a source's.
    #[test]
    fn a_form_alone_has_the_columns_it_has_beside_a_source() {
        let source = column_pitch(
            ComponentType::VoltageSourceSin,
            true,
            "DC Offset (VO) *",
            "Amplitude (VA) *",
        );
        let alone = column_pitch(ComponentType::Resistor, false, "Resistance *", "Multiplier");
        // A pane that overflows gives its scroll bar a gutter and one that
        // fits does not, and the two columns share whatever that leaves.
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let bar = ctx.global_style().spacing.scroll;
        let gutter = bar.bar_width + bar.bar_inner_margin + bar.bar_outer_margin;
        assert!(
            (alone - source).abs() <= gutter * 0.5 + 0.5,
            "a resistor's columns start {alone} apart, a source's {source}"
        );
    }
}

//! The schematic instance editor on the modal primitive.
//!
//! Registry categories become sections in the independently scrolling
//! Parameters pane, while the evidence pane is rebuilt from live document
//! state on every frame. Each property is a stable-height field block in the
//! mockup's two-column grid, with validation replacing its reserved hint
//! track instead of reflowing neighboring controls. PWL sources gain a
//! structured point editor, every
//! model-bound family gains model browsing, and source families gain a live
//! preview. The shell follows the mockup's Cancel, Apply, and OK lifecycle.

use egui::{Align, Id, Layout, Margin, Sense, Stroke, Ui, pos2, vec2};

use crate::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertyValue,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};

use super::editors::render_value_editor;
use super::state::{
    ComponentEditorContext, TabbedDialogResult, TabbedPropertyDialogState,
    unit_is_part_of_value_text,
};

const DIALOG_SIZE: DialogSize = DialogSize::ComponentEditor;
const EYEBROW: &str = "EDIT · TYPED PARAMETERS";
const DESCRIPTION: &str = "Edit identity, model, parameters, orientation, connectivity, display, constraints, and review metadata.";

/// Render the dedicated schematic instance editor.
///
/// Every component family is driven by its registered typed property sheet;
/// the shell and evidence pane follow the latest component-editor mockup.
pub fn render_tabbed_property_dialog(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    registry: &PropertyRegistry,
    model_library_manager: &crate::state::ModelLibraryManager,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    commit_policy: crate::state::PropertyCommitPolicy,
) -> TabbedDialogResult {
    let mut result = TabbedDialogResult::None;
    if !state.open {
        return result;
    }
    let Some(component_type) = state.component_type else {
        state.close();
        return TabbedDialogResult::Cancelled;
    };
    let Some(sheet) = registry.get(component_type) else {
        state.close();
        return TabbedDialogResult::Cancelled;
    };

    state.sync_pwl_validation_error();
    let session_error = state.session_error.clone();
    let dirty = state.has_modifications();
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
        .secondary_enabled(dirty && state.can_apply(commit_policy) && session_error.is_none())
        .primary_enabled(session_error.is_none() && (!dirty || state.can_apply(commit_policy)))
        .interaction_enabled(!state.model_browser.open)
        .initial_focus(DialogInitialFocus::BodyControl);
    if let Some(hint) = footer_hint.as_deref() {
        dialog = dialog.hint(hint);
    }

    let mut side_action = TabbedDialogResult::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        component_identity_header(ui, state, context);
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
                        first_parameter = parameters_pane(
                            ui,
                            state,
                            sheet,
                            component_type,
                            quantity_policy,
                            number_locale,
                        );
                    },
                );
                let (divider, _) = ui.allocate_exact_size(vec2(gap, body_height), Sense::hover());
                ui.painter()
                    .rect_filled(divider, 0.0, Tokens::get(ui.ctx()).color.border);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), body_height),
                    Layout::top_down(Align::Min),
                    |ui| evidence_pane(ui, state, context, component_type, &mut side_action),
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
                        first_parameter = parameters_pane(
                            ui,
                            state,
                            sheet,
                            component_type,
                            quantity_policy,
                            number_locale,
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
                    |ui| evidence_pane(ui, state, context, component_type, &mut side_action),
                );
            });
        }
        first_parameter
    });

    let dirty_after_render = state.has_modifications();
    if side_action != TabbedDialogResult::None {
        result = side_action;
    } else {
        match choice {
            DialogChoice::Primary => {
                if !dirty_after_render {
                    state.close();
                    result = TabbedDialogResult::Cancelled;
                } else if state.prepare_commit(sheet, commit_policy) {
                    result = if state.validation_errors.is_empty() {
                        TabbedDialogResult::AppliedAndClose
                    } else {
                        TabbedDialogResult::Applied
                    };
                }
            }
            DialogChoice::Secondary => {
                if dirty_after_render && state.prepare_commit(sheet, commit_policy) {
                    result = TabbedDialogResult::Applied;
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                state.close();
                result = TabbedDialogResult::Cancelled;
            }
            DialogChoice::None => {}
        }
    }

    render_model_browser(ctx, state, model_library_manager);
    result
}

fn component_identity_header(
    ui: &mut Ui,
    state: &mut TabbedPropertyDialogState,
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

                let status_width = (ui.available_width() * 0.28).clamp(110.0, 190.0);
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
                            let mut name = state
                                .get_value("name")
                                .map(PropertyValue::display_string)
                                .or_else(|| state.component_name.clone())
                                .unwrap_or_default();
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut name)
                                    .font(theme::mono(tokens::FS_1, FontWeight::SemiBold))
                                    .desired_width(88.0),
                            );
                            if response.changed() {
                                state.set_value("name", PropertyValue::String(name));
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
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&context.family)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_dim),
                            )
                            .truncate(),
                        );
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

fn parameters_pane(
    ui: &mut Ui,
    state: &mut TabbedPropertyDialogState,
    sheet: &crate::state::property_types::PropertySheet,
    component_type: crate::state::ComponentType,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<Id> {
    let mut first_control = None;
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("component-editor-parameters")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    first_control = parameters_contents(
                        ui,
                        state,
                        sheet,
                        component_type,
                        quantity_policy,
                        number_locale,
                    );
                });
        });
    first_control
}

/// Narrowest cell that still fits a caption, a value input, and a readable
/// hint. Below twice this the grid drops to a single column.
const MIN_CELL_WIDTH: f32 = 190.0;
/// Horizontal gap between grid columns.
const CELL_GAP: f32 = 14.0;
/// Vertical gap between grid rows.
const ROW_GAP: f32 = 10.0;
/// Caption track height (label, required marker, modified dot, unit).
const CAPTION_H: f32 = 15.0;
/// Gap between a field block's caption, control, and hint tracks.
const TRACK_GAP: f32 = 3.0;
/// Reserved hint track for a field whose micro-copy is a single line.
const HINT_LINE_H: f32 = 13.0;
/// Longest hint or validation message rendered before elision.
const HINT_MAX_ROWS: usize = 2;

fn parameters_contents(
    ui: &mut Ui,
    state: &mut TabbedPropertyDialogState,
    sheet: &crate::state::property_types::PropertySheet,
    component_type: crate::state::ComponentType,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<Id> {
    state.present_numeric_drafts(sheet, quantity_policy, number_locale);
    section_band(ui, "Parameters", "typed · unit-checked");
    let properties = sheet
        .iter()
        .filter(|definition| definition.name != "name")
        .filter(|definition| !(component_type.is_pwl_source() && definition.name == "pwl_data"))
        .filter(|definition| match definition.display_mode {
            DisplayMode::Hidden => false,
            DisplayMode::Advanced if !state.show_advanced => false,
            _ => true,
        })
        .filter(|definition| property_is_visible(definition, state))
        .cloned()
        .collect::<Vec<_>>();
    let groups = group_by_category(&properties);

    let mut first_control = None;
    egui::Frame::NONE
        .inner_margin(Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            // A single-category sheet is already titled by the section band
            // above; repeating it would be pure decoration.
            let show_headings = groups.len() > 1;
            for (index, (category, definitions)) in groups.iter().enumerate() {
                if show_headings {
                    if index > 0 {
                        ui.add_space(12.0);
                    }
                    property_group_heading(ui, category);
                }
                let control = property_grid(ui, definitions, state, quantity_policy, number_locale);
                first_control = first_control.or(control);
            }

            if component_type.is_pwl_source() {
                ui.add_space(12.0);
                property_group_heading(ui, "Piecewise-linear waveform");
                let pwl_result = crate::properties::pwl_editor::render_pwl_editor(
                    ui,
                    &mut state.pwl_editor,
                    quantity_policy,
                    number_locale,
                );
                if pwl_result == crate::properties::pwl_editor::PwlEditorResult::Modified {
                    state.pwl_editor.is_modified = true;
                    state.set_value(
                        "pwl_data",
                        PropertyValue::String(state.pwl_editor.to_string()),
                    );
                }
                state.sync_pwl_validation_error();
            }

            let has_advanced = sheet
                .iter()
                .any(|definition| definition.display_mode == DisplayMode::Advanced);
            if has_advanced {
                ui.add_space(8.0);
                crate::ui::widgets::switch_row(
                    ui,
                    "Show advanced properties",
                    &mut state.show_advanced,
                );
            }

            let t = Tokens::get(ui.ctx());
            let message = state
                .commit_error
                .as_deref()
                .or(state.global_error.as_deref());
            ui.add_space(6.0);
            let (rect, _) =
                ui.allocate_exact_size(vec2(ui.available_width(), 16.0), Sense::hover());
            // A failure owns the line when there is one. Otherwise the count of
            // engine advisories takes it, in the muted colour: they are things
            // to know before running, not things to fix before applying, and
            // painting them in the error colour would say the opposite.
            if let Some(message) = message {
                ui.painter().text(
                    pos2(rect.left(), rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    message,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.err,
                );
            } else if !state.source_advisories.is_empty() {
                let count = state.source_advisories.len();
                let summary = if count == 1 {
                    "1 engine advisory — the run differs from what a field states".to_owned()
                } else {
                    format!(
                        "{count} engine advisories — the run differs from what these fields state"
                    )
                };
                ui.painter().text(
                    pos2(rect.left(), rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    summary,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_dim,
                );
            }
        });
    first_control
}

/// Partition the visible sheet into its authored categories, keeping both the
/// categories and their members in schema order.
fn group_by_category(properties: &[PropertyDefinition]) -> Vec<(String, Vec<PropertyDefinition>)> {
    let mut groups: Vec<(String, Vec<PropertyDefinition>)> = Vec::new();
    for definition in properties {
        match groups
            .iter_mut()
            .find(|(category, _)| category == &definition.category)
        {
            Some((_, members)) => members.push(definition.clone()),
            None => groups.push((definition.category.clone(), vec![definition.clone()])),
        }
    }
    groups
}

/// Lay one category out on the two-column field grid.
///
/// Rows are packed first, then measured, so every field block on a row shares
/// one height and the columns below it stay aligned no matter how long an
/// individual description or validation message runs.
fn property_grid(
    ui: &mut Ui,
    definitions: &[PropertyDefinition],
    state: &mut TabbedPropertyDialogState,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<Id> {
    let available = ui.available_width();
    let columns = if available >= MIN_CELL_WIDTH * 2.0 + CELL_GAP {
        2
    } else {
        1
    };
    let single_width = available.max(MIN_CELL_WIDTH);
    let column_width = ((available - CELL_GAP) * 0.5).max(MIN_CELL_WIDTH);

    let mut first_control = None;
    for row in pack_rows(definitions, columns) {
        let widths = row
            .iter()
            .map(|definition| {
                if columns == 1 || property_span(definition) >= columns {
                    single_width
                } else {
                    column_width
                }
            })
            .collect::<Vec<_>>();
        let height = row
            .iter()
            .zip(&widths)
            .map(|(definition, width)| field_block_height(ui, definition, state, *width))
            .fold(0.0_f32, f32::max);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = CELL_GAP;
            for (definition, width) in row.iter().zip(&widths) {
                let control = ui
                    .push_id(("component-editor-property", &definition.name), |ui| {
                        ui.allocate_ui_with_layout(
                            vec2(*width, height),
                            Layout::top_down(Align::Min),
                            |ui| {
                                render_property_field(
                                    ui,
                                    definition,
                                    state,
                                    quantity_policy,
                                    number_locale,
                                )
                            },
                        )
                        .inner
                    })
                    .inner;
                first_control = first_control.or(control);
            }
        });
        ui.add_space(ROW_GAP);
    }
    first_control
}

/// Pack definitions into grid rows, honoring each field's column span.
fn pack_rows(definitions: &[PropertyDefinition], columns: usize) -> Vec<Vec<PropertyDefinition>> {
    let mut rows: Vec<Vec<PropertyDefinition>> = Vec::new();
    let mut used = columns;
    for definition in definitions {
        let span = property_span(definition).min(columns);
        if used + span > columns {
            rows.push(Vec::new());
            used = 0;
        }
        used += span;
        rows.last_mut()
            .expect("a row was just opened")
            .push(definition.clone());
    }
    rows
}

/// Columns one field occupies.
///
/// Composite values — a model binding with its Browse action, or a vector
/// coefficient list — are unreadable in a half-width well, so they take the
/// full grid width. The span is derived from the schema rather than the live
/// draft so typing can never reflow the grid under the cursor.
fn property_span(definition: &PropertyDefinition) -> usize {
    if definition.name == "model" {
        return 2;
    }
    let composite_default = matches!(
        &definition.default_value,
        PropertyValue::String(text) | PropertyValue::Expression(text)
            if text.starts_with('[') || text.starts_with('<')
    );
    if composite_default { 2 } else { 1 }
}

/// Total height of one field block at `width`, including however many hint
/// rows its longest current message needs.
fn field_block_height(
    ui: &Ui,
    definition: &PropertyDefinition,
    state: &TabbedPropertyDialogState,
    width: f32,
) -> f32 {
    let control_h = Tokens::get(ui.ctx()).metrics.ctl_h;
    let hint = field_hint(definition, state);
    let hint_h = if hint.is_empty() {
        HINT_LINE_H
    } else {
        ui.fonts_mut(|fonts| fonts.layout_job(hint_layout_job(&hint, width)))
            .size()
            .y
            .max(HINT_LINE_H)
    };
    CAPTION_H + TRACK_GAP + control_h + TRACK_GAP + hint_h
}

/// The micro-copy under one field: its validation error when it has one, then
/// what the engine will do with the value it currently holds, and its schema
/// description otherwise.
///
/// An advisory outranks the description because the description states what the
/// field is for, which the reader can already see from its label, while the
/// advisory states what will actually happen to the value in front of them.
fn field_hint(definition: &PropertyDefinition, state: &TabbedPropertyDialogState) -> String {
    if let Some(error) = state.validation_errors.get(&definition.name) {
        return error.clone();
    }
    let advisories = state
        .source_advisories
        .iter()
        .filter(|finding| finding.field == definition.name)
        .map(|finding| finding.message.as_str())
        .collect::<Vec<_>>();
    if advisories.is_empty() {
        definition.description.clone()
    } else {
        advisories.join(" · ")
    }
}

/// Wrapped, row-capped layout for a hint track. The cap keeps one verbose
/// message from pushing the rest of the sheet off screen.
fn hint_layout_job(hint: &str, width: f32) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::single_section(
        hint.to_owned(),
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            ..Default::default()
        },
    );
    job.wrap = egui::text::TextWrapping {
        max_width: width,
        max_rows: HINT_MAX_ROWS,
        overflow_character: Some('…'),
        ..Default::default()
    };
    job
}

fn property_is_visible(definition: &PropertyDefinition, state: &TabbedPropertyDialogState) -> bool {
    use crate::state::property_types::VisibilityCondition;
    match &definition.visibility_condition {
        VisibilityCondition::Always => true,
        VisibilityCondition::WhenNonDefault => state
            .get_value(&definition.name)
            .is_some_and(|value| value != &definition.default_value),
        VisibilityCondition::WhenPropertyEquals { property, value } => state
            .get_value(property)
            .is_some_and(|current| current.display_string().eq_ignore_ascii_case(value)),
        VisibilityCondition::WhenPropertySet(property) => state
            .get_value(property)
            .is_some_and(|current| !current.display_string().trim().is_empty()),
    }
}

fn evidence_pane(
    ui: &mut Ui,
    state: &TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    component_type: crate::state::ComponentType,
    action: &mut TabbedDialogResult,
) {
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("component-editor-evidence")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    evidence_contents(ui, state, context, component_type, action)
                });
        });
}

fn evidence_contents(
    ui: &mut Ui,
    state: &TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    component_type: crate::state::ComponentType,
    action: &mut TabbedDialogResult,
) {
    model_binding_card(ui, state, context, action);
    operating_point_card(ui, context, action);
    if supports_source_preview(component_type) {
        source_preview_card(ui, state, component_type);
    }
    terminals_card(ui, context);
}

fn supports_source_preview(kind: crate::state::ComponentType) -> bool {
    use crate::state::ComponentType;
    matches!(
        kind,
        ComponentType::VoltageSource
            | ComponentType::CurrentSource
            | ComponentType::VoltageSourceAc
            | ComponentType::CurrentSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::CurrentSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::CurrentSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::CurrentSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::CurrentSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::CurrentSourceSffm
            | ComponentType::VoltageSourceAm
            | ComponentType::CurrentSourceAm
            | ComponentType::VoltageSourcePat
            | ComponentType::CurrentSourcePat
            | ComponentType::VoltageSourceNoise
            | ComponentType::CurrentSourceNoise
    )
}

fn section_band(ui: &mut Ui, title: &str, status: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin {
            left: 16,
            right: 16,
            top: 8,
            bottom: 4,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(title.to_uppercase())
                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(status.to_uppercase())
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                });
            });
        });
    let y = ui.cursor().top();
    ui.painter()
        .hline(ui.max_rect().x_range(), y, Stroke::new(1.0, t.color.border));
}

/// A category rule: the group name followed by a hairline running to the
/// right edge, so the eye can find where one parameter group ends.
fn property_group_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, 16.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let galley = ui.fonts_mut(|fonts| {
            fonts.layout_no_wrap(
                label.to_uppercase(),
                theme::mono(tokens::FS_0, FontWeight::Medium),
                t.color.text_faint,
            )
        });
        let text_width = galley.size().x;
        ui.painter().galley(
            pos2(rect.left(), rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_faint,
        );
        let rule_start = rect.left() + text_width + 8.0;
        if rule_start < rect.right() {
            ui.painter().hline(
                rule_start..=rect.right(),
                rect.center().y,
                Stroke::new(1.0, t.color.border),
            );
        }
    }
    ui.add_space(5.0);
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
    state: &TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    action: &mut TabbedDialogResult,
) {
    let draft_model = state
        .get_value("model")
        .map(PropertyValue::display_string)
        .filter(|model| !model.trim().is_empty());
    let draft_library = state
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
                    if crate::ui::widgets::Button::new("Open model detail…")
                        .enabled(model.can_open && pending_model.is_none())
                        .show(ui)
                        .clicked()
                    {
                        *action = TabbedDialogResult::OpenModel;
                    }
                    if crate::ui::widgets::Button::new("Qualification…")
                        .enabled(model.can_qualify && pending_model.is_none())
                        .show(ui)
                        .clicked()
                    {
                        *action = TabbedDialogResult::OpenQualification;
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
    action: &mut TabbedDialogResult,
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
            if crate::ui::widgets::Button::new("Cross-probe in results…")
                .show(ui)
                .clicked()
            {
                *action = TabbedDialogResult::CrossProbe;
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

fn source_preview_card(
    ui: &mut Ui,
    state: &TabbedPropertyDialogState,
    kind: crate::state::ComponentType,
) {
    let status = if matches!(
        kind,
        crate::state::ComponentType::CurrentSourceNoise
            | crate::state::ComponentType::VoltageSourceNoise
    ) {
        "representative realization"
    } else {
        "exact expression"
    };
    section_band(ui, "Transient stimulus preview", status);
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), 132.0), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Image,
            true,
            "Preview of the configured stimulus waveform",
        )
    });
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    let plot = egui::Rect::from_min_max(rect.min + vec2(46.0, 8.0), rect.max - vec2(12.0, 20.0));
    ui.painter().line_segment(
        [plot.left_bottom(), plot.left_top()],
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().line_segment(
        [plot.left_bottom(), plot.right_bottom()],
        Stroke::new(1.0, t.color.border),
    );
    for fraction in [0.0_f32, 0.5, 1.0] {
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter()
            .hline(plot.x_range(), y, Stroke::new(0.5, t.color.border));
    }
    let samples = source_preview_samples(state, kind, 96);
    if samples.len() < 2 {
        ui.painter().text(
            plot.center(),
            egui::Align2::CENTER_CENTER,
            "Preview unavailable until values are valid",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    }
    let (minimum, maximum) = samples.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(minimum, maximum), value| (minimum.min(*value), maximum.max(*value)),
    );
    let raw_span = (maximum - minimum).abs();
    let span = raw_span.max(1e-12);
    for (fraction, label) in [
        (1.0_f32, format!("{maximum:.2e}")),
        (0.5, format!("{:.2e}", (maximum + minimum) * 0.5)),
        (0.0, format!("{minimum:.2e}")),
    ] {
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter().text(
            pos2(plot.left() - 5.0, y),
            egui::Align2::RIGHT_CENTER,
            label,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    for (fraction, label) in [(0.0_f32, "0"), (0.5, "50%"), (1.0, "100%")] {
        let x = egui::lerp(plot.left()..=plot.right(), fraction);
        ui.painter().text(
            pos2(x, plot.bottom() + 5.0),
            egui::Align2::CENTER_TOP,
            label,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    let points = samples
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let x = egui::lerp(
                plot.left()..=plot.right(),
                index as f32 / (samples.len() - 1) as f32,
            );
            let normalized = if raw_span <= 1e-12 {
                0.5
            } else {
                ((*value - minimum) / span) as f32
            };
            let y = egui::lerp(plot.bottom()..=plot.top(), normalized);
            pos2(x, y)
        })
        .collect::<Vec<_>>();
    ui.painter()
        .add(egui::Shape::line(points, Stroke::new(1.5, t.color.accent)));
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn source_preview_samples(
    state: &TabbedPropertyDialogState,
    kind: crate::state::ComponentType,
    count: usize,
) -> Vec<f64> {
    use crate::state::ComponentType;
    if count < 2 {
        return Vec::new();
    }
    let value = |key: &str, fallback: f64| {
        state
            .get_value(key)
            .and_then(PropertyValue::as_number)
            .or_else(|| {
                state.get_value(key).and_then(|value| {
                    crate::quantity::parse_engineering_value(&value.display_string()).ok()
                })
            })
            .unwrap_or(fallback)
    };
    let enabled = |key: &str, fallback: bool| {
        state
            .get_value(key)
            .map(|value| match value {
                PropertyValue::Boolean(value) => *value,
                _ => matches!(
                    value.display_string().trim().to_ascii_lowercase().as_str(),
                    "true" | "yes" | "on" | "1"
                ),
            })
            .unwrap_or(fallback)
    };
    let sample_times = |duration: f64| {
        let duration = duration.max(f64::EPSILON);
        (0..count)
            .map(|index| duration * index as f64 / (count - 1) as f64)
            .collect::<Vec<_>>()
    };
    match kind {
        ComponentType::VoltageSource | ComponentType::CurrentSource => {
            vec![value("dc", 0.0); count]
        }
        ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => {
            let amplitude = value("ac", 1.0);
            let phase_offset = value("acphase", 0.0).to_radians();
            (0..count)
                .map(|index| {
                    let phase = index as f64 / (count - 1) as f64 * std::f64::consts::TAU;
                    amplitude * (phase + phase_offset).sin()
                })
                .collect()
        }
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => {
            let low = value(
                if kind == ComponentType::VoltageSourcePulse {
                    "v1"
                } else {
                    "i1"
                },
                0.0,
            );
            let high = value(
                if kind == ComponentType::VoltageSourcePulse {
                    "v2"
                } else {
                    "i2"
                },
                1.0,
            );
            let delay = value("td", 0.0).max(0.0);
            let rise = value("tr", 1e-9).max(0.0);
            let fall = value("tf", 1e-9).max(0.0);
            let width = value("pw", 1e-6).max(0.0);
            let period = value("per", 2e-6).max(f64::EPSILON);
            sample_times(delay + 2.0 * period)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        return low;
                    }
                    let phase = (time - delay) % period;
                    if rise > 0.0 && phase < rise {
                        low + (high - low) * phase / rise
                    } else if phase < rise + width {
                        high
                    } else if fall > 0.0 && phase < rise + width + fall {
                        high - (high - low) * (phase - rise - width) / fall
                    } else {
                        low
                    }
                })
                .collect()
        }
        ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => {
            let offset = value(
                if kind == ComponentType::VoltageSourceSin {
                    "vo"
                } else {
                    "io"
                },
                0.0,
            );
            let amplitude = value(
                if kind == ComponentType::VoltageSourceSin {
                    "va"
                } else {
                    "ia"
                },
                1.0,
            );
            let frequency = value("freq", 1e6).abs().max(f64::EPSILON);
            let delay = value("td", 0.0).max(0.0);
            let damping = value("theta", 0.0).max(0.0);
            let phase = value("phase", 0.0).to_radians();
            sample_times(delay + 2.0 / frequency)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        offset + amplitude * phase.sin()
                    } else {
                        let elapsed = time - delay;
                        offset
                            + amplitude
                                * (-damping * elapsed).exp()
                                * (std::f64::consts::TAU * frequency * elapsed + phase).sin()
                    }
                })
                .collect()
        }
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
            let points = state.pwl_editor.data.points();
            if points.len() < 2 {
                return Vec::new();
            }
            let delay = value("td", 0.0).max(0.0);
            let end = points.last().map_or(1.0, |point| point.time).max(1e-18);
            let repeat = enabled("repeat", false);
            let duration = delay + if repeat { 2.0 * end } else { end };
            sample_times(duration)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        return points[0].value;
                    }
                    let mut time = time - delay;
                    if repeat {
                        time %= end;
                    } else {
                        time = time.min(end);
                    }
                    let right = points
                        .iter()
                        .position(|point| point.time >= time)
                        .unwrap_or(points.len() - 1);
                    if right == 0 {
                        return points[0].value;
                    }
                    let left = right - 1;
                    let span = (points[right].time - points[left].time).max(1e-18);
                    let ratio = (time - points[left].time) / span;
                    points[left].value + ratio * (points[right].value - points[left].value)
                })
                .collect()
        }
        ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => {
            let low = value(
                if kind == ComponentType::VoltageSourceExp {
                    "v1"
                } else {
                    "i1"
                },
                0.0,
            );
            let high = value(
                if kind == ComponentType::VoltageSourceExp {
                    "v2"
                } else {
                    "i2"
                },
                1.0,
            );
            let first_delay = value("td1", 0.0).max(0.0);
            let first_tau = value("tau1", 1e-6).max(f64::EPSILON);
            let second_delay = value("td2", 5e-6).max(first_delay);
            let second_tau = value("tau2", 1e-6).max(f64::EPSILON);
            let duration = second_delay + 5.0 * second_tau;
            sample_times(duration)
                .into_iter()
                .map(|time| {
                    let rise = if time < first_delay {
                        0.0
                    } else {
                        1.0 - (-(time - first_delay) / first_tau).exp()
                    };
                    let fall = if time < second_delay {
                        0.0
                    } else {
                        1.0 - (-(time - second_delay) / second_tau).exp()
                    };
                    low + (high - low) * (rise - fall)
                })
                .collect()
        }
        ComponentType::VoltageSourceSffm | ComponentType::CurrentSourceSffm => {
            let offset = value("vo", 0.0);
            let amplitude = value("va", 1.0);
            let carrier_frequency = value("fc", 1e6).abs().max(f64::EPSILON);
            let modulation = value("mdi", 1.0);
            let signal_frequency = value("fs", 1e3).abs();
            let delay = value("td", 0.0).max(0.0);
            let phase_modulation = value("phasem", 0.0).to_radians();
            let phase_carrier = value("phasec", 0.0).to_radians();
            // ngspice holds the source at exactly 0 before TD rather than at
            // the offset, so the preview has to show the same step.
            sample_times(delay + 3.0 / carrier_frequency)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        return 0.0;
                    }
                    let time = time - delay;
                    offset
                        + amplitude
                            * (std::f64::consts::TAU * carrier_frequency * time
                                + phase_carrier
                                + modulation
                                    * (std::f64::consts::TAU * signal_frequency * time
                                        + phase_modulation)
                                        .sin())
                            .sin()
                })
                .collect()
        }
        ComponentType::VoltageSourceAm | ComponentType::CurrentSourceAm => {
            let offset = value("vo", 0.0);
            let modulation_offset = value("vmo", 0.0);
            let modulation_amplitude = value("vma", 1.0);
            let modulating_frequency = value("fm", 1e3).abs().max(f64::EPSILON);
            let carrier_frequency = value("fc", 1e6).abs().max(f64::EPSILON);
            let delay = value("td", 0.0).max(0.0);
            let phase_modulation = value("phasem", 0.0).to_radians();
            let phase_carrier = value("phasec", 0.0).to_radians();
            // Two modulation periods make the envelope legible; a carrier-based
            // window would draw a solid band at any realistic FC/FM ratio.
            sample_times(delay + 2.0 / modulating_frequency)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        return 0.0;
                    }
                    let time = time - delay;
                    let envelope = modulation_offset
                        + modulation_amplitude
                            * (std::f64::consts::TAU * modulating_frequency * time
                                + phase_modulation)
                                .sin();
                    offset
                        + envelope
                            * (std::f64::consts::TAU * carrier_frequency * time + phase_carrier)
                                .sin()
                })
                .collect()
        }
        ComponentType::VoltageSourcePat | ComponentType::CurrentSourcePat => {
            let high = value("vhi", 1.0);
            let low = value("vlo", 0.0);
            let delay = value("td", 0.0);
            let rise = value("tr", 1e-9).max(f64::EPSILON);
            let fall = value("tf", 1e-9).max(f64::EPSILON);
            let interval = value("tsample", 1e-6).max(f64::EPSILON);
            let bits = state
                .get_value("data")
                .map(|value| value.display_string())
                .unwrap_or_default();
            let bits = bits
                .trim()
                .trim_start_matches(['b', 'B'])
                .chars()
                .filter(|bit| matches!(bit, '0' | '1'))
                .map(|bit| bit == '1')
                .collect::<Vec<_>>();
            if bits.is_empty() {
                return vec![low; count];
            }
            let duration = delay + interval * bits.len() as f64;
            sample_times(duration)
                .into_iter()
                .map(|time| {
                    let time = time - delay;
                    if time <= 0.0 {
                        return low;
                    }
                    let index = ((time / interval).floor() as usize).min(bits.len() - 1);
                    let target = if bits[index] { high } else { low };
                    let previous = if index == 0 {
                        low
                    } else if bits[index - 1] {
                        high
                    } else {
                        low
                    };
                    if previous == target {
                        return target;
                    }
                    // Xyce ramps across TR/TF at the start of the bit slot.
                    let edge = if target > previous { rise } else { fall };
                    let into_bit = time - index as f64 * interval;
                    if into_bit >= edge {
                        target
                    } else {
                        previous + (target - previous) * (into_bit / edge)
                    }
                })
                .collect()
        }
        ComponentType::VoltageSourceNoise | ComponentType::CurrentSourceNoise => {
            let dc = value("dc", 0.0);
            if !enabled("isnoisy", true) {
                return vec![dc; count];
            }
            let white_amplitude = value("na", 1e-9).abs();
            let interval = value("nt", 1e-6).max(f64::EPSILON);
            let flicker_exponent = value("nalpha", 0.0).clamp(0.0, 2.0);
            let flicker_amplitude = value("namp", 0.0).abs();
            let duration = interval * 16.0;
            sample_times(duration)
                .into_iter()
                .map(|time| {
                    // Deterministic, sample-and-hold white component plus a
                    // bounded multi-octave 1/f surrogate. This is explicitly
                    // labeled a representative realization in the UI.
                    let sample = (time / interval).floor();
                    let x = (sample + 1.0) * 12.9898;
                    let white = white_amplitude * (x.sin() * 43_758.545_3).sin();
                    let flicker = if flicker_amplitude == 0.0 {
                        0.0
                    } else {
                        let weighted = (1..=5)
                            .map(|octave| {
                                let frequency = 2_f64.powi(octave - 1) / duration;
                                let weight =
                                    frequency.powf(-0.5 * flicker_exponent.max(f64::EPSILON));
                                weight
                                    * (std::f64::consts::TAU * frequency * time
                                        + octave as f64 * 0.731)
                                        .sin()
                            })
                            .sum::<f64>();
                        let normalization = (1..=5)
                            .map(|octave| {
                                let frequency = 2_f64.powi(octave - 1) / duration;
                                frequency.powf(-0.5 * flicker_exponent.max(f64::EPSILON))
                            })
                            .sum::<f64>()
                            .max(f64::EPSILON);
                        flicker_amplitude * weighted / normalization
                    };
                    dc + white + flicker
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Folder inside a project that attached waveform data is copied into.
const PROJECT_DATA_DIR: &str = "data";

/// Ask for a waveform data file and return the reference to store for it.
///
/// A saved project takes a copy: the file is brought inside the project folder
/// and referenced relative to it, so the design keeps working when the folder is
/// moved, zipped, or handed to someone else — the same bargain every EDA tool
/// strikes, paying one duplicated file for a reference that cannot dangle. An
/// unsaved project has nowhere to copy to, so its reference stays absolute and
/// becomes relative the first time the project is saved somewhere.
///
/// `Ok(None)` means the picker was dismissed.
#[cfg(not(target_arch = "wasm32"))]
fn attach_data_file(state: &TabbedPropertyDialogState) -> Result<Option<String>, String> {
    let Some(source) = rfd::FileDialog::new()
        .add_filter("Waveform data", &["csv", "wav"])
        .add_filter("All files", &["*"])
        .pick_file()
    else {
        return Ok(None);
    };

    let Some(root) = state.data_root.as_deref() else {
        return Ok(Some(source.to_string_lossy().into_owned()));
    };
    // Already inside the project: reference it where it lies rather than
    // making a second copy of a file the project already owns.
    if let Ok(relative) = source.strip_prefix(root) {
        return Ok(Some(relative.to_string_lossy().replace('\\', "/")));
    }

    let name = source
        .file_name()
        .ok_or_else(|| "The selected path has no file name".to_owned())?;
    let directory = root.join(PROJECT_DATA_DIR);
    std::fs::create_dir_all(&directory)
        .map_err(|error| format!("Cannot create '{}': {error}", directory.display()))?;

    let destination = data_file_destination(&directory, &source, std::path::Path::new(name))?;
    if !destination.exists() {
        std::fs::copy(&source, &destination)
            .map_err(|error| format!("Cannot copy '{}': {error}", source.display()))?;
    }
    Ok(Some(format!(
        "{PROJECT_DATA_DIR}/{}",
        destination.file_name().unwrap_or(name).to_string_lossy()
    )))
}

/// Where a copy of `source` belongs in `directory`.
///
/// A slot already holding the identical file is returned as-is, so attaching
/// the same waveform to a second source does not carry in a second copy of it.
/// A slot holding a *different* file of the same name yields to the first free
/// numbered variant, so two unrelated `wave.csv` both survive.
#[cfg(not(target_arch = "wasm32"))]
fn data_file_destination(
    directory: &std::path::Path,
    source: &std::path::Path,
    name: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    let stem = name.file_stem().unwrap_or(name.as_os_str());
    let extension = name.extension();
    for attempt in 0..1000 {
        let mut candidate = if attempt == 0 {
            std::path::PathBuf::from(stem)
        } else {
            std::path::PathBuf::from(format!("{}-{}", stem.to_string_lossy(), attempt + 1))
        };
        if let Some(extension) = extension {
            candidate.set_extension(extension);
        }
        let candidate = directory.join(candidate);
        if !candidate.exists() || files_have_equal_contents(source, &candidate) {
            return Ok(candidate);
        }
    }
    Err(format!(
        "'{}' already holds too many files named like '{}'",
        directory.display(),
        name.display()
    ))
}

/// Whether two files hold the same bytes. An unreadable file is reported as
/// different, which costs a redundant copy rather than a silently wrong reuse.
#[cfg(not(target_arch = "wasm32"))]
fn files_have_equal_contents(left: &std::path::Path, right: &std::path::Path) -> bool {
    let (Ok(left_meta), Ok(right_meta)) = (std::fs::metadata(left), std::fs::metadata(right))
    else {
        return false;
    };
    if left_meta.len() != right_meta.len() {
        return false;
    }
    match (std::fs::read(left), std::fs::read(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

#[cfg(target_arch = "wasm32")]
fn attach_data_file(_state: &TabbedPropertyDialogState) -> Result<Option<String>, String> {
    Err("Waveform data files are only available in the desktop application".to_owned())
}

fn render_model_browser(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    model_library_manager: &crate::state::ModelLibraryManager,
) {
    if state.model_browser.open {
        use crate::properties::model_browser::{ModelBrowserResult, render_model_browser};

        match render_model_browser(ctx, &mut state.model_browser, model_library_manager) {
            ModelBrowserResult::Selected {
                library,
                model,
                corner: _,
            } => {
                state.set_value("model", PropertyValue::String(model));
                state.set_value("model_library", PropertyValue::String(library));
                // Process corner is owned by the simulation plan. Clear any
                // legacy per-instance hint rather than presenting it as an
                // executable component property.
                state.set_value("model_corner", PropertyValue::String(String::new()));
                state.model_browser.open = false;
            }
            ModelBrowserResult::Cancelled => {
                state.model_browser.open = false;
            }
            ModelBrowserResult::None => {}
        }
    }
}

/// One field block: caption, fixed control track, and a micro-copy track that
/// validation can replace without moving its neighbors.
///
/// The caller has already sized this block for the tallest field on its row,
/// so each track is allocated explicitly rather than left to flow.
fn render_property_field(
    ui: &mut Ui,
    def: &PropertyDefinition,
    state: &mut TabbedPropertyDialogState,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.spacing_mut().item_spacing.y = TRACK_GAP;
    let width = ui.available_width();
    let is_modified = state.is_modified(&def.name);
    let error = state.validation_errors.get(&def.name).cloned();
    let current_value = state
        .get_value(&def.name)
        .cloned()
        .unwrap_or_else(|| def.default_value.clone());
    let numeric_text_draft = state.numeric_text_draft(&def.name).map(str::to_owned);
    let picks_data_file = def.name == "file"
        && state
            .component_type
            .is_some_and(|kind| kind.is_pwl_file_source());
    let browse = def.name == "model" || picks_data_file;

    ui.set_width(width);
    field_caption(ui, def, is_modified, error.is_some(), width);

    let mut changed_value = None;
    let mut numeric_text = None;
    let mut numeric_parse_error = None;
    let mut browse_clicked = false;
    let mut editor_id = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let editor_width = if browse {
            (ui.available_width() - 58.0).max(60.0)
        } else {
            ui.available_width().max(60.0)
        };
        let editor = render_value_editor(
            ui,
            def,
            &current_value,
            numeric_text_draft.as_deref(),
            editor_width,
            quantity_policy,
            number_locale,
        );
        changed_value = editor.changed;
        numeric_text = editor.numeric_text;
        numeric_parse_error = editor.parse_error;
        editor_id = editor.control_id;
        if browse && crate::ui::widgets::Button::new("Browse").show(ui).clicked() {
            browse_clicked = true;
        }
    });

    if let Some(control_id) = editor_id {
        ui.ctx().accesskit_node_builder(control_id, |node| {
            node.set_label(def.display_name.clone());
            if !def.description.is_empty() {
                node.set_description(def.description.clone());
            }
            if error.is_some() {
                node.set_invalid(egui::accesskit::Invalid::True);
            } else {
                node.clear_invalid();
            }
        });
        if error.is_some()
            && let Some(response) = ui.ctx().read_response(control_id)
        {
            ui.painter().rect_stroke(
                response.rect,
                3.0,
                Stroke::new(1.0, c.err),
                egui::StrokeKind::Inside,
            );
        }
    }
    if let Some(text) = numeric_text {
        state.update_numeric_text_draft(&def.name, text, numeric_parse_error);
    }
    if let Some(value) = changed_value {
        if def.name == "model" {
            state.set_value(&def.name, value);
            // A manually typed name has no proven catalog identity. Clear the
            // prior exact binding so a duplicate name cannot silently retain
            // the wrong library or process corner.
            state.set_value("model_library", PropertyValue::String(String::new()));
            state.set_value("model_corner", PropertyValue::String(String::new()));
        } else {
            state.set_value(&def.name, value);
        }
    }
    if browse_clicked && picks_data_file {
        match attach_data_file(state) {
            Ok(Some(reference)) => {
                state.set_value(&def.name, PropertyValue::String(reference));
                state.session_error = None;
            }
            Ok(None) => {}
            Err(error) => state.session_error = Some(error),
        }
    } else if browse_clicked {
        state.model_browser.type_filter = state.component_type.and_then(model_type_for_component);
        state.model_browser.allow_corner_selection = false;
        state.model_browser.selected_library = state
            .get_value("model_library")
            .map(PropertyValue::display_string)
            .filter(|value| !value.trim().is_empty());
        state.model_browser.selected_model = state
            .get_value("model")
            .map(PropertyValue::display_string)
            .filter(|value| !value.trim().is_empty());
        state.model_browser.selected_corner = None;
        state.model_browser.open = true;
    }

    // The unit is presented once, in the caption; repeating it here would be
    // the only duplicated token on the block.
    let hint = field_hint(def, state);
    let hint_height = (ui.available_height() - TRACK_GAP).max(HINT_LINE_H);
    let (rect, response) = ui.allocate_exact_size(vec2(width, hint_height), Sense::hover());
    if !hint.is_empty() {
        if ui.is_rect_visible(rect) {
            let galley = ui.fonts_mut(|fonts| fonts.layout_job(hint_layout_job(&hint, width)));
            ui.painter().galley(
                rect.left_top(),
                galley,
                if error.is_some() { c.err } else { c.text_faint },
            );
        }
        response.on_hover_text(hint);
    }
    editor_id
}

/// The caption track: label, required marker, unsaved-edit dot, and the
/// schema unit pinned to the right so units align down the column.
fn field_caption(
    ui: &mut Ui,
    def: &PropertyDefinition,
    is_modified: bool,
    invalid: bool,
    width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) = ui.allocate_exact_size(vec2(width, CAPTION_H), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }

    let unit_width = def
        .unit
        .as_deref()
        .filter(|_| !unit_is_part_of_value_text(def))
        .map(|unit| {
            let galley = ui.fonts_mut(|fonts| {
                fonts.layout_no_wrap(
                    unit.to_owned(),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                )
            });
            ui.painter().galley(
                pos2(
                    rect.right() - galley.size().x,
                    rect.center().y - galley.size().y * 0.5,
                ),
                galley.clone(),
                c.text_faint,
            );
            galley.size().x + 8.0
        })
        .unwrap_or(0.0);

    let dot_width = if is_modified { 9.0 } else { 0.0 };
    let label_width = (rect.width() - unit_width - dot_width).max(0.0);
    let label = if def.required {
        format!("{} *", def.display_name)
    } else {
        def.display_name.clone()
    };
    let mut job = egui::text::LayoutJob::single_section(
        label,
        egui::TextFormat {
            font_id: label_font(def),
            color: if invalid { c.err } else { c.text_dim },
            ..Default::default()
        },
    );
    job.wrap = egui::text::TextWrapping {
        max_width: label_width,
        max_rows: 1,
        overflow_character: Some('…'),
        ..Default::default()
    };
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let label_end = rect.left() + galley.size().x;
    ui.painter().galley(
        pos2(rect.left(), rect.center().y - galley.size().y * 0.5),
        galley,
        if invalid { c.err } else { c.text_dim },
    );
    if is_modified {
        ui.painter()
            .circle_filled(pos2(label_end + 5.0, rect.center().y), 2.5, c.accent);
    }
}

/// A sheet whose labels are the device's exact parameter keys renders them in
/// the mono face, matching the deck the user will read back.
fn label_font(def: &PropertyDefinition) -> egui::FontId {
    if def.display_name == def.name {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    }
}

/// Which model type the browser opens filtered to, for a placement of `kind`.
///
/// The filter narrows what a reader is shown; it is not the binding contract,
/// which `validate_component_model_compatibility` owns and applies afterwards.
/// So a device answers the *type its cards carry*: an SOI MOSFET's cards are
/// MOSFET cards told apart by their level, and share this filter with bulk
/// ones, while a VDMOS card carries a type of its own and does not.
fn model_type_for_component(
    kind: crate::state::ComponentType,
) -> Option<crate::state::model_library::ModelType> {
    use crate::state::ComponentType;
    use crate::state::model_library::ModelType;
    Some(match kind {
        ComponentType::Nmos | ComponentType::NmosSoi => ModelType::Nmos,
        ComponentType::Pmos | ComponentType::PmosSoi => ModelType::Pmos,
        ComponentType::NVdmos => ModelType::NVdmos,
        ComponentType::PVdmos => ModelType::PVdmos,
        ComponentType::NpnBjt | ComponentType::NpnBjt4 | ComponentType::NpnBjt5 => ModelType::Npn,
        ComponentType::PnpBjt | ComponentType::PnpBjt4 | ComponentType::PnpBjt5 => ModelType::Pnp,
        ComponentType::Njfet => ModelType::Njfet,
        ComponentType::Pjfet => ModelType::Pjfet,
        ComponentType::Nmesfet => ModelType::Nmesfet,
        ComponentType::Pmesfet => ModelType::Pmesfet,
        ComponentType::Diode => ModelType::Diode,
        ComponentType::Resistor => ModelType::Resistor,
        ComponentType::Capacitor => ModelType::Capacitor,
        ComponentType::Inductor | ComponentType::SaturableInductor => ModelType::Inductor,
        ComponentType::LossyTransmissionLine
        | ComponentType::CoupledTransmissionLine
        | ComponentType::RfPort => ModelType::Rf,
        // Switches, memristors and cell instances are bound to cards this
        // vocabulary has no family for, so the unclassified type is where
        // their cards genuinely are rather than a shrug.
        ComponentType::Memristor
        | ComponentType::VSwitch
        | ComponentType::ISwitch
        | ComponentType::CellInstance => ModelType::Other,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;

    #[test]
    fn sine_preview_preserves_phase_during_the_configured_delay() {
        let mut state = TabbedPropertyDialogState::default();
        state.set_value("vo", PropertyValue::number(1.0));
        state.set_value("va", PropertyValue::number(2.0));
        state.set_value("freq", PropertyValue::number(1.0));
        state.set_value("td", PropertyValue::number(10.0));
        state.set_value("phase", PropertyValue::number(90.0));

        let samples = source_preview_samples(&state, ComponentType::VoltageSourceSin, 8);

        assert!((samples[0] - 3.0).abs() < 1e-12);
        assert!((samples[1] - 3.0).abs() < 1e-12);
    }

    #[test]
    fn disabled_noise_preview_is_exactly_the_dc_bias() {
        let mut state = TabbedPropertyDialogState::default();
        state.set_value("dc", PropertyValue::number(2e-3));
        state.set_value("na", PropertyValue::number(1e-3));
        state.set_value("namp", PropertyValue::number(1e-3));
        state.set_value("isnoisy", PropertyValue::Boolean(false));

        let samples = source_preview_samples(&state, ComponentType::CurrentSourceNoise, 16);

        assert_eq!(samples, vec![2e-3; 16]);
    }

    /// The preview re-reads a field the dialog holds as text. A period
    /// authored `1ms` used to fail that read and fall through to the 2 µs
    /// default, so the drawn pulse train disagreed with the field above it.
    #[test]
    fn the_preview_reads_a_period_authored_with_its_unit() {
        let mut typed = TabbedPropertyDialogState::default();
        typed.set_value("per", PropertyValue::String("1ms".to_owned()));
        let mut numeric = TabbedPropertyDialogState::default();
        numeric.set_value("per", PropertyValue::number(1e-3));

        assert_eq!(
            source_preview_samples(&typed, ComponentType::VoltageSourcePulse, 16),
            source_preview_samples(&numeric, ComponentType::VoltageSourcePulse, 16)
        );
    }
}

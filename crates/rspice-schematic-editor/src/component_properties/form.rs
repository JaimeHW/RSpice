//! Schema-driven component parameters and typed host browse requests.

use egui::{Align, Id, Layout, Margin, Sense, Stroke, Ui, pos2, vec2};
use rspice_app_types::property::{DisplayMode, PropertyDefinition, PropertySheet, PropertyValue};
use rspice_app_types::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use rspice_design::properties::SourceContractFinding;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

use super::ComponentPropertyDraft;
use crate::property_values::{render_value_editor, unit_is_part_of_value_text};

/// A Browse interaction that the host handles before the field's hint is painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyBrowseRequest<'a> {
    /// Open the model browser using the current draft binding.
    Model,
    /// Attach data and write its reference to the named draft property.
    DataFile(&'a str),
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

/// Render the scrolling parameter form and return its first keyboard-focus target.
///
/// The host handles browse requests synchronously so file selection and model
/// routing retain their position between draft edits and validation-hint painting.
pub fn render_component_parameters(
    ui: &mut Ui,
    draft: &mut ComponentPropertyDraft,
    sheet: &PropertySheet,
    advisories: &[SourceContractFinding],
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    browse: &mut impl FnMut(PropertyBrowseRequest<'_>, &mut ComponentPropertyDraft),
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
                        draft,
                        sheet,
                        advisories,
                        quantity_policy,
                        number_locale,
                        browse,
                    );
                });
        });
    first_control
}

fn parameters_contents(
    ui: &mut Ui,
    draft: &mut ComponentPropertyDraft,
    sheet: &PropertySheet,
    advisories: &[SourceContractFinding],
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    browse: &mut impl FnMut(PropertyBrowseRequest<'_>, &mut ComponentPropertyDraft),
) -> Option<Id> {
    let component_type = draft.component_type?;
    draft.present_numeric_drafts(sheet, quantity_policy, number_locale);
    section_band(ui, "Parameters", "typed · unit-checked");
    let properties = sheet
        .iter()
        .filter(|definition| definition.name != "name")
        .filter(|definition| !(component_type.is_pwl_source() && definition.name == "pwl_data"))
        .filter(|definition| match definition.display_mode {
            DisplayMode::Hidden => false,
            DisplayMode::Advanced if !draft.show_advanced => false,
            _ => true,
        })
        .filter(|definition| property_is_visible(definition, draft))
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
                let control = property_grid(
                    ui,
                    definitions,
                    draft,
                    advisories,
                    quantity_policy,
                    number_locale,
                    browse,
                );
                first_control = first_control.or(control);
            }

            if component_type.is_pwl_source() {
                ui.add_space(12.0);
                property_group_heading(ui, "Piecewise-linear waveform");
                let pwl_result = crate::pwl_editor::render_pwl_editor(
                    ui,
                    &mut draft.pwl_editor,
                    quantity_policy,
                    number_locale,
                );
                if pwl_result == crate::pwl_editor::PwlEditorResult::Modified {
                    draft.pwl_editor.is_modified = true;
                    draft.set_value(
                        "pwl_data",
                        PropertyValue::String(draft.pwl_editor.to_string()),
                    );
                }
                draft.sync_pwl_validation_error();
            }

            let has_advanced = sheet
                .iter()
                .any(|definition| definition.display_mode == DisplayMode::Advanced);
            if has_advanced {
                ui.add_space(8.0);
                rspice_ui_kit::widgets::switch_row(
                    ui,
                    "Show advanced properties",
                    &mut draft.show_advanced,
                );
            }

            let t = Tokens::get(ui.ctx());
            let message = draft
                .commit_error
                .as_deref()
                .or(draft.global_error.as_deref());
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
            } else if !advisories.is_empty() {
                let count = advisories.len();
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
    draft: &mut ComponentPropertyDraft,
    advisories: &[SourceContractFinding],
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    browse: &mut impl FnMut(PropertyBrowseRequest<'_>, &mut ComponentPropertyDraft),
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
            .map(|(definition, width)| {
                field_block_height(ui, definition, draft, advisories, *width)
            })
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
                                    draft,
                                    advisories,
                                    quantity_policy,
                                    number_locale,
                                    browse,
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
    draft: &ComponentPropertyDraft,
    advisories: &[SourceContractFinding],
    width: f32,
) -> f32 {
    let control_h = Tokens::get(ui.ctx()).metrics.ctl_h;
    let hint = field_hint(definition, draft, advisories);
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
fn field_hint(
    definition: &PropertyDefinition,
    draft: &ComponentPropertyDraft,
    advisories: &[SourceContractFinding],
) -> String {
    if let Some(error) = draft.validation_errors.get(&definition.name) {
        return error.clone();
    }
    let advisories = advisories
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

fn property_is_visible(definition: &PropertyDefinition, draft: &ComponentPropertyDraft) -> bool {
    use rspice_app_types::property::VisibilityCondition;
    match &definition.visibility_condition {
        VisibilityCondition::Always => true,
        VisibilityCondition::WhenNonDefault => draft
            .get_value(&definition.name)
            .is_some_and(|value| value != &definition.default_value),
        VisibilityCondition::WhenPropertyEquals { property, value } => draft
            .get_value(property)
            .is_some_and(|current| current.display_string().eq_ignore_ascii_case(value)),
        VisibilityCondition::WhenPropertySet(property) => draft
            .get_value(property)
            .is_some_and(|current| !current.display_string().trim().is_empty()),
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
    draft: &mut ComponentPropertyDraft,
    advisories: &[SourceContractFinding],
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    browse: &mut impl FnMut(PropertyBrowseRequest<'_>, &mut ComponentPropertyDraft),
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.spacing_mut().item_spacing.y = TRACK_GAP;
    let width = ui.available_width();
    let is_modified = draft.is_modified(&def.name);
    let error = draft.validation_errors.get(&def.name).cloned();
    let current_value = draft
        .get_value(&def.name)
        .cloned()
        .unwrap_or_else(|| def.default_value.clone());
    let numeric_text_draft = draft.numeric_text_draft(&def.name).map(str::to_owned);
    let picks_data_file = def.name == "file"
        && draft
            .component_type
            .is_some_and(|kind| kind.is_pwl_file_source());
    let has_browser = def.name == "model" || picks_data_file;

    ui.set_width(width);
    field_caption(ui, def, is_modified, error.is_some(), width);

    let mut changed_value = None;
    let mut numeric_text = None;
    let mut numeric_parse_error = None;
    let mut browse_clicked = false;
    let mut editor_id = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let editor_width = if has_browser {
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
        if has_browser
            && rspice_ui_kit::widgets::Button::new("Browse")
                .show(ui)
                .clicked()
        {
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
        draft.update_numeric_text_draft(&def.name, text, numeric_parse_error);
    }
    if let Some(value) = changed_value {
        if def.name == "model" {
            draft.set_value(&def.name, value);
            // A manually typed name has no proven catalog identity. Clear the
            // prior exact binding so a duplicate name cannot silently retain
            // the wrong library or process corner.
            draft.set_value("model_library", PropertyValue::String(String::new()));
            draft.set_value("model_corner", PropertyValue::String(String::new()));
        } else {
            draft.set_value(&def.name, value);
        }
    }
    if browse_clicked {
        let request = if picks_data_file {
            PropertyBrowseRequest::DataFile(&def.name)
        } else {
            PropertyBrowseRequest::Model
        };
        browse(request, draft);
    }

    // The unit is presented once, in the caption; repeating it here would be
    // the only duplicated token on the block.
    let hint = field_hint(def, draft, advisories);
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

/// Paint a section heading shared by the parameter and evidence panes.
pub fn section_band(ui: &mut Ui, title: &str, status: &str) {
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

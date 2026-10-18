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
    // Read before the scroll area, which is the last place the pane's own
    // width is legible: inside it every rect has the scrollbar gutter taken
    // off, and the section bands head the pane rather than the content.
    let pane = ui.max_rect();
    // No fill of its own. The dialog body paints the editor's one surface, and
    // a pane that painted a second coat of the same token came out a shade off
    // the pane beside it, with the base colour showing between them — a seam
    // down the middle of what is one editor.
    egui::ScrollArea::vertical()
        .id_salt("component-editor-parameters")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // Here rather than inside the contents, which is where the pane's
            // width would have to be an eighth argument: the band belongs to
            // the scroll area, so that it scrolls, and to nothing narrower.
            if draft.component_type.is_some() {
                section_band(ui, "Parameters", pane);
            }
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
    // The advanced fields are judged by the same rules as the rest before the
    // two are told apart, so the section is offered only when opening it
    // would show something. A resistor's advanced fields all wait on a model,
    // and judging the schema alone offered a switch that showed nothing.
    let (advanced, inline): (Vec<_>, Vec<_>) = sheet
        .iter()
        .filter(|definition| definition.name != "name")
        .filter(|definition| !(component_type.is_pwl_source() && definition.name == "pwl_data"))
        .filter(|definition| definition.display_mode != DisplayMode::Hidden)
        .filter(|definition| property_is_visible(definition, draft))
        .cloned()
        .partition(|definition| definition.display_mode == DisplayMode::Advanced);
    let groups = group_by_category(&inline);

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
                // No heading here: the point table heads itself, and two
                // stacked headings over one table read as two tables.
                ui.add_space(12.0);
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

            // After everything else and opening downward, so showing the
            // advanced fields moves nothing already on screen. They used to
            // join their categories in schema order, which reflowed the grid
            // and put whole groups between the switch and what it sat under.
            if !advanced.is_empty() {
                ui.add_space(12.0);
                advanced_disclosure(ui, advanced.len(), &mut draft.show_advanced);
                if draft.show_advanced {
                    for (index, (category, definitions)) in
                        group_by_category(&advanced).into_iter().enumerate()
                    {
                        // The disclosure row carries its own air; the groups
                        // after the first are spaced as the ones above it are.
                        ui.add_space(if index == 0 { 4.0 } else { 12.0 });
                        property_group_heading(ui, &category);
                        let control = property_grid(
                            ui,
                            &definitions,
                            draft,
                            advisories,
                            quantity_policy,
                            number_locale,
                            browse,
                        );
                        first_control = first_control.or(control);
                    }
                }
            }

            // The whole-sheet failure, and only that. What the engine will do
            // with one field is stated on that field, by `field_hint`, which
            // reads the field each rule names; a count of those along the
            // bottom was a second rendering of facts already on screen, and it
            // named no field a reader could go and look at.
            if let Some(message) = draft
                .commit_error
                .as_deref()
                .or(draft.global_error.as_deref())
            {
                ui.add_space(6.0);
                let (rect, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 16.0), Sense::hover());
                ui.painter().text(
                    pos2(rect.left(), rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    message,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    Tokens::get(ui.ctx()).color.err,
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
    // A field with nothing to report takes no hint track at all. Reserving
    // one kept a blank line under every field for the few that ever use it.
    if hint.is_empty() {
        return CAPTION_H + TRACK_GAP + control_h;
    }
    let hint_h = ui
        .fonts_mut(|fonts| fonts.layout_job(hint_layout_job(&hint, width)))
        .size()
        .y
        .max(HINT_LINE_H);
    CAPTION_H + TRACK_GAP + control_h + TRACK_GAP + hint_h
}

/// The micro-copy under one field: its validation error when it has one, and
/// otherwise what the engine will do with the value it currently holds.
///
/// The schema description is not among them. It states what the field is for,
/// which does not change and which the label already carries, and a permanent
/// faint line under every field in a two-column grid was most of this
/// dialog's height. It is the field's tooltip and its accessible description
/// instead, so the track under the control belongs to the two things that do
/// change: a value the sheet rejects, and a value the run will not use as
/// written.
fn field_hint(
    definition: &PropertyDefinition,
    draft: &ComponentPropertyDraft,
    advisories: &[SourceContractFinding],
) -> String {
    if let Some(error) = draft.validation_errors.get(&definition.name) {
        return error.clone();
    }
    advisories
        .iter()
        .filter(|finding| finding.field == definition.name)
        .map(|finding| finding.message.as_str())
        .collect::<Vec<_>>()
        .join(" \u{b7} ")
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
        let browse_button = rspice_ui_kit::widgets::Button::new("Browse");
        // The button as it will be drawn, measured. A 58-point allowance for
        // it was eight short, so the row ran past the column's edge and every
        // grid after it was laid out that much wider than the ones above.
        let editor_width = if has_browser {
            (ui.available_width() - browse_button.measured_width(ui) - ui.spacing().item_spacing.x)
                .max(60.0)
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
        if has_browser && browse_button.show(ui).clicked() {
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
        // What the field is for, on the field, for the reader who wants it.
        if !def.description.is_empty()
            && let Some(response) = ui.ctx().read_response(control_id)
        {
            response.on_hover_text(def.description.clone());
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
    if !hint.is_empty() {
        let hint_height = (ui.available_height() - TRACK_GAP).max(HINT_LINE_H);
        let (rect, response) = ui.allocate_exact_size(vec2(width, hint_height), Sense::hover());
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
/// The height of a section's head.
///
/// Sized to the one line it holds. The band used to be laid out by a frame
/// around a row, which came to 38 points for a 12-point word because the row
/// took the style's own interact height before its margins were added.
const SECTION_BAND_HEIGHT: f32 = 26.0;
/// Where a band's title starts, matching the inset of the content under it.
const SECTION_BAND_INSET: f32 = 16.0;

/// One section's head: the title, and nothing else.
///
/// Sentence case, as written. It used to upper-case the title and carry a
/// right-aligned caption beside it, and the caption was never worth its line:
/// `typed · unit-checked` states a property of the editor rather than of this
/// instance, and the model binding's word repeated the status its own card
/// prints underneath. What one of them said and nothing else did — which
/// definition a source copied — is a row in that section now, where an
/// identifier is set in mono and not folded to `SENSOR_DRIVE · R2`.
pub fn section_band(ui: &mut Ui, title: &str, pane: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    let (row, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), SECTION_BAND_HEIGHT),
        Sense::hover(),
    );
    // The band spans the pane, which is handed in because nothing inside a
    // scroll area can work it out. A vertical `ScrollArea` sets its content's
    // clip rect to `clip_rect().max.x - current_bar_use.x`, deliberately, so
    // every rect in reach here — `max_rect`, `clip_rect`, `available_width` —
    // already has the bar's gutter taken off it.
    //
    // Two expressions written from inside were tried and both left the gutter
    // bare: the content width, and the content width plus a constant standing
    // in for the gutter, which the narrowed clip rect then cut back anyway.
    // The notch they left is small on one band and obvious between two panes,
    // where the two of them read as a gap chopped out of the header bar.
    let band = egui::Rect::from_x_y_ranges(pane.left()..=pane.right(), row.y_range());
    // The pane's own clip, not the content's: the fill has to cross the gutter
    // that the content's clip rect stops at. `with_clip_rect` cannot do this —
    // it *intersects* with the clip already in force, so asking it for the
    // pane gave back the content's rect and painted exactly the notch this is
    // here to remove.
    let mut painter = ui.painter().clone();
    painter.set_clip_rect(pane);
    painter.rect_filled(band, 0.0, t.color.bg_panel_2);
    let mut inner = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(egui::Rect::from_min_max(
                pos2(band.left() + SECTION_BAND_INSET, band.top()),
                band.max,
            ))
            .layout(Layout::left_to_right(Align::Center)),
    );
    inner.label(
        egui::RichText::new(title)
            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    painter.hline(
        band.x_range(),
        band.bottom(),
        Stroke::new(1.0, t.color.border),
    );
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
                label.to_owned(),
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

/// The head of the advanced section: a category rule that opens and closes
/// the fields under it, with how many it holds.
///
/// It was a switch row reading "Show advanced properties" at the foot of the
/// form, which turned on fields scattered through the categories above it.
fn advanced_disclosure(ui: &mut Ui, count: usize, open: &mut bool) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, mut response) =
        ui.allocate_exact_size(vec2(ui.available_width(), t.metrics.row_h), Sense::click());
    if response.clicked() {
        *open = !*open;
        response.mark_changed();
        if *open {
            // The fields open below the row, and a row near the foot of the
            // pane would open them out of sight.
            response.scroll_to_me(Some(Align::Min));
        }
    }
    let label = if count == 1 {
        "Advanced, 1 parameter".to_owned()
    } else {
        format!("Advanced, {count} parameters")
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::CollapsingHeader, ui.is_enabled(), &label)
    });
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_expanded(*open));
    let focus_visible = theme::focus_is_visible(ui.ctx(), &response);
    if !ui.is_rect_visible(rect) {
        return;
    }

    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);
    let painter = ui.painter();
    if hover > 0.0 {
        painter.rect_filled(
            rect,
            t.radius,
            theme::mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }
    // Painted: the bundled faces have no disclosure triangles.
    let cy = rect.center().y;
    let center = pos2(rect.left() + 4.0, cy);
    let r = 3.2;
    let points = if *open {
        vec![
            center + vec2(-r, -r * 0.5),
            center + vec2(r, -r * 0.5),
            center + vec2(0.0, r * 0.9),
        ]
    } else {
        vec![
            center + vec2(-r * 0.5, -r),
            center + vec2(r * 0.9, 0.0),
            center + vec2(-r * 0.5, r),
        ]
    };
    let ink = theme::mix(c.text_dim, c.text, hover);
    painter.add(egui::Shape::convex_polygon(points, ink, Stroke::NONE));

    let title = ui.fonts_mut(|fonts| {
        fonts.layout_no_wrap(
            "Advanced".to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            ink,
        )
    });
    let title_left = rect.left() + 14.0;
    let title_right = title_left + title.size().x;
    painter.galley(pos2(title_left, cy - title.size().y * 0.5), title, ink);
    let tally = ui.fonts_mut(|fonts| {
        fonts.layout_no_wrap(
            count.to_string(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        )
    });
    let tally_left = title_right + 6.0;
    let tally_right = tally_left + tally.size().x;
    painter.galley(
        pos2(tally_left, cy - tally.size().y * 0.5),
        tally,
        c.text_faint,
    );
    let rule_start = tally_right + 8.0;
    if rule_start < rect.right() {
        painter.hline(rule_start..=rect.right(), cy, Stroke::new(1.0, c.border));
    }
    if focus_visible {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand);
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rspice_design::properties::{PropertyCatalog, component::collect_properties_with_sheet};
    use rspice_design::schematic::component::Component;
    use rspice_design::schematic::component_type::ComponentType;
    use rspice_design_model::Point;

    /// Every text the parameter form paints for a freshly opened `kind`, and
    /// where, after `prepare` has had the draft. The pane is tall enough that
    /// nothing scrolls out of it, so a field missing from the list was not
    /// drawn at all rather than drawn out of sight.
    fn painted_texts(
        kind: ComponentType,
        prepare: impl FnOnce(&mut ComponentPropertyDraft),
    ) -> Vec<(String, egui::Rect)> {
        let (sheet, mut draft) = opened(kind);
        prepare(&mut draft);
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        // Fonts on the first pass, measured rows on the second.
        for _ in 0..2 {
            drop(form_frame(&ctx, &sheet, &mut draft, true, Vec::new()));
        }
        texts(form_frame(&ctx, &sheet, &mut draft, true, Vec::new()))
    }

    /// A draft freshly opened on the catalog's sheet for `kind`.
    fn opened(kind: ComponentType) -> (PropertySheet, ComponentPropertyDraft) {
        let sheet = PropertyCatalog::new()
            .get(kind)
            .expect("catalog sheet")
            .clone();
        let component = Component::new(1, kind, Point::origin());
        let mut draft = ComponentPropertyDraft::default();
        draft.reset(
            kind,
            &sheet,
            collect_properties_with_sheet(&component, Some(&sheet)),
        );
        (sheet, draft)
    }

    /// One frame of the form, in a `Ui` that is `enabled` or not.
    fn form_frame(
        ctx: &egui::Context,
        sheet: &PropertySheet,
        draft: &mut ComponentPropertyDraft,
        enabled: bool,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                vec2(460.0, 3000.0),
            )),
            events,
            ..Default::default()
        };
        ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    render_component_parameters(
                        ui,
                        draft,
                        sheet,
                        &[],
                        QuantityPresentationPolicy::default(),
                        UiNumberLocale::default(),
                        &mut |_, _| {},
                    );
                });
            });
        })
    }

    fn texts(output: egui::FullOutput) -> Vec<(String, egui::Rect)> {
        output
            .shapes
            .into_iter()
            .filter_map(|clipped| match clipped.shape {
                egui::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                )),
                _ => None,
            })
            .collect()
    }

    /// Pressed inside a disabled `Ui`, the head leaves the section as it was.
    /// egui withholds the click from a disabled response; the head relies on
    /// that rather than reading the enabled bit itself, and this holds it to it.
    #[test]
    fn a_disabled_advanced_head_does_not_open_when_pressed() {
        fn press(enabled: bool) -> bool {
            let (sheet, mut draft) = opened(ComponentType::VoltageSourceSin);
            let ctx = egui::Context::default();
            rspice_ui_kit::Theme::default().apply(&ctx);
            drop(form_frame(&ctx, &sheet, &mut draft, enabled, Vec::new()));
            let painted = texts(form_frame(&ctx, &sheet, &mut draft, enabled, Vec::new()));
            let at = painted
                .iter()
                .find(|(text, _)| text == "Advanced")
                .map(|(_, rect)| rect.center())
                .expect("a source offers its advanced fields");
            let button = |pressed| egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            drop(form_frame(
                &ctx,
                &sheet,
                &mut draft,
                enabled,
                vec![egui::Event::PointerMoved(at), button(true), button(false)],
            ));
            draft.show_advanced
        }

        assert!(press(true), "an enabled head opens on the press that asked");
        assert!(!press(false), "a disabled one does not");
    }

    fn position_of(texts: &[(String, egui::Rect)], wanted: &str) -> Option<egui::Pos2> {
        texts
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, rect)| rect.min)
    }

    /// Every grid on the form is as wide as the first.
    ///
    /// The model binding's row reserved 58 points for its Browse button, which
    /// draws about eight wider, so the row ran past its column and widened the
    /// form under it: the Temperature fields ended right of the Electrical
    /// ones above them. A unit is set flush with its column's right edge, so
    /// the right column's units say where that edge is in each group.
    #[test]
    fn the_model_row_keeps_to_the_columns_above_it() {
        let texts = painted_texts(ComponentType::Resistor, |_| {});
        let right_edge = |unit: &str| {
            texts
                .iter()
                .filter(|(text, _)| text == unit)
                .map(|(_, rect)| rect.right())
                .fold(f32::NEG_INFINITY, f32::max)
        };
        // AC Resistance, above the model row; Temp Coeff 1, below it.
        let above = right_edge("Ω");
        let below = right_edge("1/°C");
        assert!(
            above.is_finite() && below.is_finite(),
            "both units painted: {above} {below}"
        );
        assert!(
            (above - below).abs() < 0.5,
            "the right column ends at {above} above the model row and {below} below it"
        );
    }

    /// A resistor's advanced fields all wait on a model. With none bound the
    /// section is not offered, because opening it would show nothing; the
    /// switch it replaced was offered anyway and did nothing when pressed.
    #[test]
    fn the_advanced_section_is_offered_only_when_it_has_a_field_to_show() {
        let unbound = painted_texts(ComponentType::Resistor, |_| {});
        assert!(
            position_of(&unbound, "Advanced").is_none(),
            "an unbound resistor offers an empty advanced section"
        );

        let bound = painted_texts(ComponentType::Resistor, |draft| {
            draft.set_value("model", PropertyValue::String("rpoly".to_owned()));
        });
        assert!(
            position_of(&bound, "Advanced").is_some(),
            "a resistor bound to a model has self-heating fields to offer"
        );
    }

    /// Opening the advanced section moves nothing that was already on screen,
    /// and everything it shows lands below its head.
    ///
    /// The fields used to join their own categories in schema order, so a
    /// source's Advanced AC and Distortion groups landed between AC and
    /// Parasitics, and every field under them moved down.
    #[test]
    fn opening_the_advanced_section_moves_nothing_above_it() {
        let closed = painted_texts(ComponentType::VoltageSourceSin, |_| {});
        let open = painted_texts(ComponentType::VoltageSourceSin, |draft| {
            draft.show_advanced = true;
        });
        let head = position_of(&closed, "Advanced").expect("a source offers its advanced fields");
        assert_eq!(position_of(&open, "Advanced"), Some(head));

        for (text, rect) in &closed {
            assert!(
                open.iter()
                    .any(|(other, at)| other == text && (at.min - rect.min).length() < 0.01),
                "{text:?} at {rect:?} moved when the section opened"
            );
        }
        let revealed = open
            .iter()
            .filter(|(text, _)| !closed.iter().any(|(other, _)| other == text))
            .collect::<Vec<_>>();
        assert!(
            revealed.iter().any(|(text, _)| text == "PAC Magnitude"),
            "the section shows the source's periodic AC fields: {revealed:?}"
        );
        assert!(
            revealed.iter().all(|(_, rect)| rect.min.y > head.y),
            "every revealed field sits below the section's head: {revealed:?}"
        );
    }

    /// A band spans the pane it heads, gutter included, through a real scroll
    /// area rather than a stand-in for one.
    ///
    /// The pane is laid out exactly as the editor's is — a fixed rect holding
    /// a vertical `ScrollArea` with content long enough to need its bar — so
    /// the gutter the band has to cross is the one egui actually reserves, not
    /// a number written here. Three expressions failed this before it was
    /// handed the pane: the content width, which leaves the gutter bare; the
    /// content width plus a constant for the gutter, which the scroll area's
    /// own clip rect cut back to the same place; and the clip rect itself,
    /// which egui narrows by `current_bar_use` on purpose.
    #[test]
    fn a_section_band_spans_the_pane_it_heads() {
        const PANE: f32 = 240.0;
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        // A bar that takes its own column, so there is a gutter to cross. A
        // floating bar is drawn over the content and reserves nothing, which
        // makes this pass without proving anything.
        ctx.all_styles_mut(|style| style.spacing.scroll = egui::style::ScrollStyle::solid());
        let mut pane = egui::Rect::NOTHING;
        let mut content_right = f32::NAN;
        let mut frame = || {
            ctx.run_ui(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    pane = egui::Rect::from_min_size(ui.max_rect().min, vec2(PANE, 120.0));
                    let mut pane_ui = ui.new_child(egui::UiBuilder::new().max_rect(pane));
                    pane_ui.set_clip_rect(pane);
                    let rect = pane_ui.max_rect();
                    egui::ScrollArea::vertical()
                        .id_salt("band-test")
                        .auto_shrink([false, false])
                        .show(&mut pane_ui, |ui| {
                            section_band(ui, "Parameters", rect);
                            // Long enough to put the bar on screen, so the
                            // gutter this is about is actually reserved.
                            for row in 0..40 {
                                ui.label(format!("row {row}"));
                            }
                            content_right = ui.clip_rect().right();
                        });
                });
            })
        };
        // The first frame does not know the content overflows, so no bar is
        // shown and no gutter is reserved. The second one has the state.
        drop(frame());
        let output = frame();
        assert!(
            content_right < pane.right(),
            "the scroll area must reserve a gutter for this to prove \
             anything: content clipped at {content_right}, pane at {}",
            pane.right()
        );
        // The band's own fill, not every shape the host panel drew: a
        // `CentralPanel` paints its background over an effectively unbounded
        // rect, which says nothing about this widget.
        let fill = Tokens::get(&ctx).color.bg_panel_2;
        let bands = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Rect(rect) if rect.fill == fill => Some((rect.rect, shape.clip_rect)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(bands.len(), 1, "one band was painted: {bands:?}");
        let (painted, clip) = bands[0];
        assert!(
            (painted.left() - pane.left()).abs() <= 0.5
                && (painted.right() - pane.right()).abs() <= 0.5,
            "the band spans its pane: {painted:?} against {pane:?}, \
             with the content clipped at {content_right}"
        );
        // The shape carries the rect it asked for; what reaches the screen is
        // that rect intersected with the clip it was issued under. Asserting
        // only the first would pass against the very bug this is here for,
        // where the band was the right size and the scroll area's clip rect
        // took the gutter back off it.
        assert!(
            clip.right() >= pane.right() - 0.5 && clip.left() <= pane.left() + 0.5,
            "the band is painted under a clip that admits all of it: \
             {clip:?} against {pane:?}"
        );
    }
}

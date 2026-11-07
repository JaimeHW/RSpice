//! Schema-driven component parameters and typed host browse requests.

use egui::{Align, Id, Layout, Margin, Sense, Stroke, Ui, pos2, vec2};
use rspice_design::properties::SourceContractFinding;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::property::{
    DisplayMode, PropertyDefinition, PropertySheet, PropertyType, PropertyValue,
};
use rspice_vocabulary::quantity::{QuantityPresentationPolicy, UiNumberLocale};

use super::ComponentPropertyDraft;
use crate::property_values::render_value_editor;

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
/// The side of a check's box, drawn at the start of its target.
const CHECK_BOX_SIDE: f32 = 14.0;
/// Gap between a field block's caption, control, and hint tracks.
const TRACK_GAP: f32 = 3.0;
/// Reserved hint track for a field whose micro-copy is a single line.
const HINT_LINE_H: f32 = 13.0;
/// Longest hint or validation message rendered before elision.
const HINT_MAX_ROWS: usize = 2;

/// Which instance a form edits, for the group that heads it.
#[derive(Debug, Clone, Copy)]
pub struct InstanceIdentity<'a> {
    /// The designator the instance had when the editor opened, for a draft
    /// that holds no name of its own.
    pub name: Option<&'a str>,
    /// What the part is, in one word: the device for a primitive.
    pub device: &'a str,
    /// The `library/cell/view` an instance of a master stands for; empty for
    /// a primitive, which the engine implements and no library holds.
    pub master: &'a str,
}

/// How the parameters sit in their dialog.
#[derive(Debug, Clone, Copy)]
pub struct ParametersPane<'a> {
    /// The instance the form edits.
    pub identity: InstanceIdentity<'a>,
    /// Whether the parameters share the body with another pane. A shared
    /// pane is headed with a section band and scrolls on its own; a form
    /// alone is the whole body, under the dialog's own title, and scrolls
    /// with it.
    pub beside_another: bool,
}

/// Render the parameter form and return its first keyboard-focus target.
///
/// The host handles browse requests synchronously so file selection and model
/// routing retain their position between draft edits and validation-hint painting.
#[expect(
    clippy::too_many_arguments,
    reason = "the form's inputs, its two presentation policies, its pane and the host's browse handler"
)]
pub fn render_component_parameters(
    ui: &mut Ui,
    draft: &mut ComponentPropertyDraft,
    sheet: &PropertySheet,
    advisories: &[SourceContractFinding],
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    pane: ParametersPane<'_>,
    browse: &mut impl FnMut(PropertyBrowseRequest<'_>, &mut ComponentPropertyDraft),
) -> Option<Id> {
    if !pane.beside_another {
        return parameters_contents(
            ui,
            draft,
            sheet,
            advisories,
            quantity_policy,
            number_locale,
            pane,
            browse,
        );
    }
    let mut first_control = None;
    // Read before the scroll area, which is the last place the pane's own
    // width is legible: inside it every rect has the scrollbar gutter taken
    // off, and the section bands head the pane rather than the content.
    let pane_rect = ui.max_rect();
    // No fill of its own. The dialog body paints the editor's one surface, and
    // a pane that painted a second coat of the same token came out a shade off
    // the pane beside it, with the base colour showing between them — a seam
    // down the middle of what is one editor.
    egui::ScrollArea::vertical()
        .id_salt("component-editor-parameters")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // Here rather than inside the contents, which is where the pane's
            // width would have to be another argument: the band belongs to
            // the scroll area, so that it scrolls, and to nothing narrower.
            if draft.component_type.is_some() {
                section_band(ui, "Parameters", pane_rect);
            }
            first_control = parameters_contents(
                ui,
                draft,
                sheet,
                advisories,
                quantity_policy,
                number_locale,
                pane,
                browse,
            );
        });
    first_control
}

#[expect(
    clippy::too_many_arguments,
    reason = "the form's inputs, its two presentation policies, the instance and the host's browse handler"
)]
fn parameters_contents(
    ui: &mut Ui,
    draft: &mut ComponentPropertyDraft,
    sheet: &PropertySheet,
    advisories: &[SourceContractFinding],
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    pane: ParametersPane<'_>,
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
    // What of the last row's height lies below what it draws: a row of checks
    // alone is a control-high target with a smaller box in its middle.
    let mut trailing_slack = 0.0_f32;
    let inset = if pane.beside_another {
        PANE_INSET
    } else {
        FORM_INSET
    };
    egui::Frame::NONE
        .inner_margin(Margin {
            left: inset,
            right: inset,
            top: FORM_TOP_AIR,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            // The instance first and unheaded, as the dialog's title names
            // it; every group of parameters after it is headed, so a single
            // group is never left for the title alone to stand over.
            identity_row(ui, draft, pane.identity);
            for (category, definitions) in &groups {
                ui.add_space(GROUP_GAP);
                property_group_heading(ui, category);
                let (control, slack) = property_grid(
                    ui,
                    definitions,
                    draft,
                    advisories,
                    quantity_policy,
                    number_locale,
                    browse,
                );
                first_control = first_control.or(control);
                trailing_slack = slack;
            }

            if component_type.is_pwl_source() {
                trailing_slack = 0.0;
                // No heading here: the point table heads itself, and two
                // stacked headings over one table read as two tables.
                ui.add_space(GROUP_GAP);
                let pwl_result = crate::pwl_editor::render_pwl_editor(
                    ui,
                    &mut draft.pwl_editor,
                    quantity_policy,
                    number_locale,
                );
                if pwl_result == crate::pwl_editor::PwlEditorResult::Modified {
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
                ui.add_space(GROUP_GAP);
                advanced_disclosure(ui, advanced.len(), &mut draft.show_advanced);
                trailing_slack = 0.0;
                if draft.show_advanced {
                    for (index, (category, definitions)) in
                        group_by_category(&advanced).into_iter().enumerate()
                    {
                        // The disclosure row carries its own air; the groups
                        // after the first are spaced as the ones above it are.
                        ui.add_space(if index == 0 { 4.0 } else { GROUP_GAP });
                        property_group_heading(ui, &category);
                        let (control, slack) = property_grid(
                            ui,
                            &definitions,
                            draft,
                            advisories,
                            quantity_policy,
                            number_locale,
                            browse,
                        );
                        first_control = first_control.or(control);
                        trailing_slack = slack;
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
                trailing_slack = 0.0;
                ui.add_space(ROW_GAP);
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
            // As much air under the last thing drawn as over the first
            // caption's ink. A trailing row of checks already has the lower
            // half of its target under its boxes; that counts towards it.
            ui.add_space((FORM_BOTTOM_AIR - trailing_slack).max(0.0));
        });
    first_control
}

/// The form's inset above its first row. The first row is captions, whose
/// ink starts [`CAPTION_INK_TOP`] below their line's top.
const FORM_TOP_AIR: i8 = 10;
/// How far below its line's top a caption's ink starts.
const CAPTION_INK_TOP: f32 = 3.0;
/// The air under the form's last row: the same as over its first caption's
/// ink, so the body is padded alike above and below. It was 13 above and 25
/// under a touch-high row of checks.
const FORM_BOTTOM_AIR: f32 = FORM_TOP_AIR as f32 + CAPTION_INK_TOP;

/// Air above a group's heading, between it and the last row of the group
/// before.
const GROUP_GAP: f32 = 22.0;
/// The form's side inset inside a pane, on its section band's inset.
const PANE_INSET: i8 = 16;
/// The form's side inset alone in its dialog, on the dialog title's inset, so
/// the title, every field and the footer's buttons share one left edge.
const FORM_INSET: i8 = 12;

/// The instance's own row, on the grid the parameters use: the name,
/// which the form edits, and what the part is.
///
/// It used to be a band of its own above the form, `Instance [R1]` over a
/// `Resistor` line, standing where the dialog's title belongs and setting the
/// name in a well a third of a field wide.
fn identity_row(ui: &mut Ui, draft: &mut ComponentPropertyDraft, identity: InstanceIdentity<'_>) {
    let available = ui.available_width();
    let two_columns = available >= MIN_CELL_WIDTH * 2.0 + CELL_GAP;
    let width = if two_columns {
        ((available - CELL_GAP) * 0.5).max(MIN_CELL_WIDTH)
    } else {
        available.max(MIN_CELL_WIDTH)
    };
    let height = CAPTION_H + TRACK_GAP + Tokens::get(ui.ctx()).metrics.ctl_h;
    let name_cell = |ui: &mut Ui, draft: &mut ComponentPropertyDraft| {
        ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            ui.spacing_mut().item_spacing.y = TRACK_GAP;
            caption(
                ui,
                "Name",
                theme::sans(tokens::FS_0, FontWeight::Regular),
                draft.is_modified("name"),
                false,
                width,
            );
            let mut name = draft
                .get_value("name")
                .map(PropertyValue::display_string)
                .or_else(|| identity.name.map(str::to_owned))
                .unwrap_or_default();
            if rspice_ui_kit::widgets::mono_input(ui, "Name", &mut name, width).changed() {
                draft.set_value("name", PropertyValue::String(name));
            }
        });
    };
    let kind_cell = |ui: &mut Ui| {
        let (label, value, font) = if identity.master.is_empty() {
            (
                "Device",
                identity.device,
                theme::sans(tokens::FS_1, FontWeight::Regular),
            )
        } else {
            (
                "Master",
                identity.master,
                theme::mono(tokens::FS_1, FontWeight::Regular),
            )
        };
        ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            ui.spacing_mut().item_spacing.y = TRACK_GAP;
            caption(
                ui,
                label,
                theme::sans(tokens::FS_0, FontWeight::Regular),
                false,
                false,
                width,
            );
            fact_value(ui, label, value, font, width);
        });
    };
    if two_columns {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = CELL_GAP;
            name_cell(ui, draft);
            kind_cell(ui);
        });
    } else {
        name_cell(ui, draft);
        ui.add_space(ROW_GAP);
        kind_cell(ui);
    }
}

/// A value the form states and does not edit, on a field's control track: at
/// the caption's left edge rather than a well's text inset, since no well
/// surrounds it.
fn fact_value(ui: &mut Ui, label: &str, value: &str, font: egui::FontId, width: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::hover());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, value));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_value(value);
    });
    if !ui.is_rect_visible(rect) {
        return;
    }
    let mut job = egui::text::LayoutJob::single_section(
        value.to_owned(),
        egui::TextFormat {
            font_id: font,
            color: t.color.text,
            ..Default::default()
        },
    );
    job.wrap = egui::text::TextWrapping {
        max_width: width,
        max_rows: 1,
        overflow_character: Some('…'),
        ..Default::default()
    };
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    ui.painter().galley(
        pos2(rect.left(), rect.center().y - galley.size().y * 0.5),
        galley,
        t.color.text,
    );
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

/// Lay one category out on the two-column field grid, from its first row's
/// top to its last row's bottom. Returns the first control, and how much of
/// the last row's height lies under what it draws: half a control's height
/// less half a box, for a row of checks alone, and nothing otherwise.
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
) -> (Option<Id>, f32) {
    let control_h = Tokens::get(ui.ctx()).metrics.ctl_h;
    let mut slack = 0.0;
    let available = ui.available_width();
    let columns = if available >= MIN_CELL_WIDTH * 2.0 + CELL_GAP {
        2
    } else {
        1
    };
    let single_width = available.max(MIN_CELL_WIDTH);
    let column_width = ((available - CELL_GAP) * 0.5).max(MIN_CELL_WIDTH);

    let mut first_control = None;
    for (index, row) in pack_rows(definitions, columns).into_iter().enumerate() {
        // Between rows, and not after the last: the space after a group is
        // its caller's, so the form ends on its last field.
        if index > 0 {
            ui.add_space(ROW_GAP);
        }
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
        slack = if row.iter().all(is_check) && height <= control_h {
            (control_h - CHECK_BOX_SIDE) * 0.5
        } else {
            0.0
        };

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
    }
    (first_control, slack)
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
/// full grid width, as does a part's free-text description. The span is derived from the schema rather than the live
/// draft so typing can never reflow the grid under the cursor.
fn property_span(definition: &PropertyDefinition) -> usize {
    if definition.name == "model"
        || definition.name == rspice_design::schematic::bom::BomField::Description.key()
    {
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
    // A check carries its name beside its box, so it has no caption track.
    let caption_h = if is_check(definition) {
        0.0
    } else {
        CAPTION_H + TRACK_GAP
    };
    let hint = field_hint(definition, draft, advisories);
    // A field with nothing to report takes no hint track at all. Reserving
    // one kept a blank line under every field for the few that ever use it.
    if hint.is_empty() {
        return caption_h + control_h;
    }
    let hint_h = ui
        .fonts_mut(|fonts| fonts.layout_job(hint_layout_job(&hint, width)))
        .size()
        .y
        .max(HINT_LINE_H);
    caption_h + control_h + TRACK_GAP + hint_h
}

/// Whether `definition` is drawn as a check: a box with its name beside it,
/// not a caption over a lone box.
fn is_check(definition: &PropertyDefinition) -> bool {
    definition.prop_type == PropertyType::Boolean
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

/// Whether `def` is the value of a part drawn from a preferred-number series.
fn takes_standard_value(draft: &ComponentPropertyDraft, def: &PropertyDefinition) -> bool {
    use rspice_design::schematic::component_type::ComponentType;
    draft.component_type.is_some_and(|kind| {
        matches!(
            kind,
            ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor
        ) && def.name == rspice_design::properties::component::get_primary_property_name(kind)
    })
}

fn property_is_visible(definition: &PropertyDefinition, draft: &ComponentPropertyDraft) -> bool {
    use rspice_vocabulary::property::VisibilityCondition;
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
    let check = is_check(def);
    if !check {
        field_caption(ui, def, is_modified, error.is_some(), width);
    }

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
        let editor_width = if check {
            // A check's target is a control-high square, its box at the start.
            Tokens::get(ui.ctx()).metrics.ctl_h
        } else if has_browser {
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
        if check {
            // The name follows the box, not the rest of its square target.
            ui.add_space(CHECK_BOX_SIDE - Tokens::get(ui.ctx()).metrics.ctl_h);
            let rest = ui.available_width();
            field_caption(ui, def, is_modified, error.is_some(), rest);
        }
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
    // A resistor, capacitor or inductor's value steps along the project's
    // series from the keyboard and the wheel, as a typed edit would.
    if let (Some(control_id), Some(text)) = (editor_id, numeric_text.as_mut())
        && takes_standard_value(draft, def)
        && crate::property_values::step_standard_value(ui, control_id, draft.standard_series, text)
    {
        match rspice_design::properties::value::parse_expression_source(
            def,
            text,
            quantity_policy,
            number_locale,
        ) {
            Ok(value) => {
                changed_value = Some(value);
                numeric_parse_error = None;
            }
            Err(error) => numeric_parse_error = Some(error),
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
            let filled = def.preset_values(&value).to_vec();
            draft.set_value(&def.name, value);
            draft.fill_preset(&filled);
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

    // The unit is presented once, inside the well after the value.
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

/// The caption track: label, required marker and unsaved-edit dot.
///
/// The unit is not here. It used to be pinned to the right of the label's
/// line, where it read as part of the label and sat a field's height away from
/// the value it qualifies; the well states it after the value now, as the
/// inspector's rows do.
fn field_caption(
    ui: &mut Ui,
    def: &PropertyDefinition,
    is_modified: bool,
    invalid: bool,
    width: f32,
) {
    let label = if def.required {
        format!("{} *", def.display_name)
    } else {
        def.display_name.clone()
    };
    caption(ui, &label, label_font(def), is_modified, invalid, width);
}

/// One caption line: `label` in `font`, and the unsaved-edit dot after it.
fn caption(
    ui: &mut Ui,
    label: &str,
    font: egui::FontId,
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

    let dot_width = if is_modified { 9.0 } else { 0.0 };
    let label_width = (rect.width() - dot_width).max(0.0);
    let mut job = egui::text::LayoutJob::single_section(
        label.to_owned(),
        egui::TextFormat {
            font_id: font,
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

/// A group of fields inside the form: its name in the kit's subsection face
/// (the section title's face, in the faint tone) with no band and no rule.
///
/// It was a monospace caption with a hairline run out to the right edge, a
/// second header style beside the section band above it; monospace is for
/// values and identifiers, not for headings.
fn property_group_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(width, GROUP_HEADING_H), Sense::hover());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(label);
        node.set_level(4);
    });
    if ui.is_rect_visible(rect) {
        let galley = ui.fonts_mut(|fonts| {
            fonts.layout_no_wrap(
                label.to_owned(),
                theme::sans(tokens::FS_0, FontWeight::SemiBold),
                t.color.text_faint,
            )
        });
        ui.painter().galley(
            pos2(rect.left(), rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_faint,
        );
    }
    ui.add_space(GROUP_HEADING_GAP);
}

/// The height of a group's heading line.
const GROUP_HEADING_H: f32 = 16.0;
/// Air between a group's heading and its first row of fields.
const GROUP_HEADING_GAP: f32 = 5.0;

/// The head of the advanced section: a group heading that opens and closes
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
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            ink,
        )
    });
    let title_left = rect.left() + 14.0;
    let title_right = title_left + title.size().x;
    painter.galley(pos2(title_left, cy - title.size().y * 0.5), title, ink);
    let tally = ui.fonts_mut(|fonts| {
        fonts.layout_no_wrap(
            count.to_string(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        )
    });
    let tally_left = title_right + 6.0;
    painter.galley(
        pos2(tally_left, cy - tally.size().y * 0.5),
        tally,
        c.text_faint,
    );
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
                        ParametersPane {
                            identity: InstanceIdentity {
                                name: Some("X1"),
                                device: "Device",
                                master: "",
                            },
                            beside_another: true,
                        },
                        &mut |_, _| {},
                    );
                });
            });
        })
    }

    /// The form is padded alike above and below: as much air under its
    /// last row of checks as over its first caption's ink. Under the Noisy
    /// check there was the lower half of its control-high target and then
    /// the form's inset again, 25 points on a touch screen against 13 above.
    #[test]
    fn the_form_is_padded_alike_above_its_first_caption_and_under_its_checks() {
        for target in [None, Some(44.0)] {
            let (full, mut draft) = opened(ComponentType::Resistor);
            // The resistor's sheet as far as its Noise checks; the
            // purchasing rows after them are text fields.
            let mut sheet = PropertySheet::new();
            for definition in full
                .iter()
                .filter(|definition| definition.category != "Bill of materials")
            {
                sheet.add(definition.clone());
            }
            let ctx = egui::Context::default();
            rspice_ui_kit::Theme::default().apply(&ctx);
            rspice_ui_kit::Theme::default().apply_responsive_metrics_with_target(&ctx, target);
            let mut form = egui::Rect::NOTHING;
            let mut painted = Vec::new();
            for _ in 0..3 {
                let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        form = ui
                            .scope(|ui| {
                                render_component_parameters(
                                    ui,
                                    &mut draft,
                                    &sheet,
                                    &[],
                                    QuantityPresentationPolicy::default(),
                                    UiNumberLocale::default(),
                                    ParametersPane {
                                        identity: InstanceIdentity {
                                            name: Some("R1"),
                                            device: "Resistor",
                                            master: "",
                                        },
                                        beside_another: false,
                                    },
                                    &mut |_, _| {},
                                );
                            })
                            .response
                            .rect;
                    });
                });
                painted = texts(output);
            }
            let rect_of = |wanted: &str| {
                painted
                    .iter()
                    .find(|(text, _)| text == wanted)
                    .map(|(_, rect)| *rect)
                    .unwrap_or_else(|| panic!("{wanted} is not painted"))
            };
            let above = rect_of("Name").top() - form.top() + CAPTION_INK_TOP;
            let box_bottom = rect_of("Noisy").center().y + CHECK_BOX_SIDE * 0.5;
            let below = form.bottom() - box_bottom;
            let half_target = (Tokens::get(&ctx).metrics.ctl_h - CHECK_BOX_SIDE) * 0.5;
            assert!(
                (below - above.max(half_target)).abs() < 1.0,
                "touch target {target:?}: {above} above the first caption, {below} under the checks"
            );
        }
    }

    /// Up in a focused resistance field steps to the next value of the
    /// project's series, and the wheel over it steps back down, as typed
    /// edits of the draft.
    #[test]
    fn a_focused_value_field_steps_along_the_project_series() {
        let (sheet, mut draft) = opened(ComponentType::Resistor);
        draft.standard_series = rspice_vocabulary::quantity::ESeries::E12;
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let output = form_frame(&ctx, &sheet, &mut draft, true, Vec::new());
        let well = texts(output)
            .into_iter()
            .find(|(text, _)| text == "1k")
            .expect("the resistance well shows its default")
            .1
            .center();
        let modifiers = egui::Modifiers::NONE;
        let press = |pressed| egui::Event::PointerButton {
            pos: well,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers,
        };
        form_frame(
            &ctx,
            &sheet,
            &mut draft,
            true,
            vec![egui::Event::PointerMoved(well), press(true), press(false)],
        );
        // egui takes a focus lock from the second frame a widget holds the
        // keyboard; a key in the very next frame would still navigate.
        form_frame(&ctx, &sheet, &mut draft, true, Vec::new());
        let resistance = |draft: &ComponentPropertyDraft| match draft.get_value("r") {
            Some(PropertyValue::Number { value, .. }) => *value,
            other => panic!("resistance is {other:?}"),
        };

        let up = egui::Event::Key {
            key: egui::Key::ArrowUp,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        };
        form_frame(&ctx, &sheet, &mut draft, true, vec![up]);
        assert!((resistance(&draft) - 1200.0).abs() < 1e-9);

        let wheel_down = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: vec2(0.0, -1.0),
            phase: egui::TouchPhase::Move,
            modifiers,
        };
        form_frame(&ctx, &sheet, &mut draft, true, vec![wheel_down]);
        assert!((resistance(&draft) - 1000.0).abs() < 1e-9);
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
    /// A check states its name beside its box, as a check does: the Noisy
    /// caption stands right of the column edge where every other caption
    /// starts, not over a lone box.
    #[test]
    fn a_check_names_itself_beside_its_box() {
        let texts = painted_texts(ComponentType::Resistor, |_| {});
        let left = |wanted: &str| {
            texts
                .iter()
                .find(|(text, _)| text == wanted)
                .map(|(_, rect)| rect.left())
                .unwrap_or_else(|| panic!("{wanted} is not painted: {texts:?}"))
        };
        let (noisy, rise) = (left("Noisy"), left("Temp rise"));
        assert!(
            noisy > rise + CHECK_BOX_SIDE,
            "Noisy at {noisy}, the column's captions at {rise}"
        );
    }

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
        // AC resistance, above the model row; Temp coeff 1, below it.
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
            revealed.iter().any(|(text, _)| text == "PAC magnitude"),
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

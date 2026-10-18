//! Mockup-owned rendering for the custom sheet-size workflows.

use std::collections::BTreeMap;

use egui::{ComboBox, Grid, RichText, ScrollArea, TextEdit, Ui, vec2};
use egui_extras::{Column, TableBuilder};

use crate::state::{AuthoredDrawingSheetSize, DrawingSheetPreset, DrawingSheetPresetScope};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::model::{
    ImportCandidate, ImportResolution, PresetEditorDraft, PresetEditorMode, PresetEditorUnit,
    PresetPackageFormat, PresetTransferState, StartingFrame, TransferMode, format_dimension_um,
    unavailable, unsigned_exportable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PresetKey {
    pub(super) id: String,
    pub(super) scope: DrawingSheetPresetScope,
}

impl From<&DrawingSheetPreset> for PresetKey {
    fn from(value: &DrawingSheetPreset) -> Self {
        Self {
            id: value.id.clone(),
            scope: value.scope,
        }
    }
}

pub(super) fn usage_key(scope: DrawingSheetPresetScope, id: &str) -> String {
    format!("{}:{}", scope_label(scope), id.to_lowercase())
}

#[derive(Debug, Clone)]
pub(super) enum LibraryBodyAction {
    New,
    Import,
    Export,
    Use(PresetKey),
    Duplicate(PresetKey),
    Rename(PresetKey),
    Delete(PresetKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EditorBodyAction {
    RefreshPreview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TransferBodyAction {
    ChooseImportFile,
    SelectAllExport(bool),
}

pub(super) fn library_body(
    ui: &mut Ui,
    search: &mut String,
    presets: &[DrawingSheetPreset],
    usage: &BTreeMap<String, usize>,
    project_writable: bool,
    personal_fresh: bool,
    authority_notice: Option<&str>,
) -> Option<LibraryBodyAction> {
    let t = Tokens::get(ui.ctx());
    let mut action = None;

    // Toolbar and table are one continuous page: each section owns its own
    // surface and they meet on their edges, rather than floating as separate
    // boxes on a band of background.
    ui.spacing_mut().item_spacing.y = 0.0;
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(PAGE_INSET, 6))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            // The filter sits at the trailing edge, where the toolbar ends.
            // Packed against the buttons it left the bar's right half empty
            // however wide the dialog was.
            let split = ui.available_width() >= TOOLBAR_SPLIT_WIDTH;
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                if Button::new("New custom size\u{2026}")
                    .accent()
                    .show(ui)
                    .clicked()
                {
                    action = Some(LibraryBodyAction::New);
                }
                if Button::new("Import\u{2026}").show(ui).clicked() {
                    action = Some(LibraryBodyAction::Import);
                }
                let exportable = presets.iter().any(|preset| !unavailable(preset));
                if Button::new("Export\u{2026}")
                    .enabled(exportable)
                    .show(ui)
                    .clicked()
                {
                    action = Some(LibraryBodyAction::Export);
                }
                let mut filter = |ui: &mut Ui, width: f32| {
                    ui.add_sized(
                        vec2(width, t.metrics.ctl_h),
                        TextEdit::singleline(search).hint_text("Filter custom sizes\u{2026}"),
                    );
                };
                if split {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        filter(ui, FILTER_WIDTH)
                    });
                } else {
                    // Wrapped onto its own line, the field takes that line.
                    let width = ui.available_width().max(180.0);
                    filter(ui, width);
                }
            });
        });
    if let Some(notice) = authority_notice {
        warning_notice(ui, notice);
    }
    let query = search.trim().to_lowercase();
    let filtered = presets
        .iter()
        .filter(|preset| {
            query.is_empty()
                || preset.name.to_lowercase().contains(&query)
                || scope_label(preset.scope).contains(&query)
                || preset.id.to_lowercase().contains(&query)
        })
        .collect::<Vec<_>>();

    if filtered.is_empty() {
        empty_state(
            ui,
            if presets.is_empty() {
                "No custom sheet sizes"
            } else {
                "No custom sizes match this filter"
            },
            if presets.is_empty() {
                "Create a project or personal preset, or import a reviewed schema-1 package."
            } else {
                "Change the filter to return to the library."
            },
        );
    } else if ui.available_width() < LIBRARY_TABLE_MIN_WIDTH {
        // Narrower than the table's own floor, the columns would scroll
        // sideways and take every row's actions off the surface with them.
        for preset in &filtered {
            library_entry(
                ui,
                &t,
                preset,
                usage,
                project_writable,
                personal_fresh,
                &mut action,
            );
        }
    } else {
        library_table(
            ui,
            &t,
            &filtered,
            usage,
            project_writable,
            personal_fresh,
            &mut action,
        );
    }
    action
}

fn library_notes_stack(available_width: f32) -> bool {
    available_width < 620.0
}

/// Below this the toolbar wraps and the filter follows the buttons; above it
/// the filter takes the trailing edge.
const TOOLBAR_SPLIT_WIDTH: f32 = 620.0;

/// What the filter field is given once the toolbar is wide enough to split.
const FILTER_WIDTH: f32 = 300.0;

/// The table's headings, in order. Every column is exactly as wide as the
/// widest thing in it, heading included, and the surface is exactly as wide as
/// the columns: an authored width either leaves a band no column reaches, or
/// is too narrow and scrolls.
const LIBRARY_COLUMNS: [&str; 6] = [
    "Preset",
    "Portrait dimensions",
    "Scope",
    "Used by",
    "State",
    "Actions",
];

/// The width below which the library drops the table for its list form.
const LIBRARY_TABLE_MIN_WIDTH: f32 = 700.0;

/// How wide the library's surface has to be to state its table and its toolbar
/// without a band that no column reaches and without scrolling either.
///
/// Asked before the dialog is laid out, so it is measured through the same
/// arithmetic the table and the toolbar then lay themselves out with.
pub(super) fn library_width(
    ctx: &egui::Context,
    presets: &[DrawingSheetPreset],
    usage: &BTreeMap<String, usize>,
) -> f32 {
    // A `Ui` of its own, for the font cache alone: nothing is allocated in it
    // and nothing is painted from it.
    let mut probe = Ui::new(
        ctx.clone(),
        egui::Id::new("custom-sheet-size-library-measure"),
        egui::UiBuilder::new()
            .max_rect(ctx.content_rect())
            .invisible(),
    );
    let visible = presets.iter().collect::<Vec<_>>();
    let columns: f32 = library_column_widths(&mut probe, &visible, usage)
        .iter()
        .sum();
    // The surface's 1 pt border on either side: a dialog's width is its outer
    // edge, and its body is that less the border. Without it the table ran
    // 2 pt past the body and took a horizontal scrollbar for them.
    columns.max(library_toolbar_width(&mut probe)).ceil() + 2.0 * SURFACE_BORDER
}

/// The width of the dialog surface's border, on each side.
const SURFACE_BORDER: f32 = 1.0;

const LIBRARY_HEADER_HEIGHT: f32 = 27.0;

/// Every preset in the library, one row each, each column exactly as wide as
/// the widest thing in it.
fn library_table(
    ui: &mut Ui,
    t: &Tokens,
    presets: &[&DrawingSheetPreset],
    usage: &BTreeMap<String, usize>,
    project_writable: bool,
    personal_fresh: bool,
    action: &mut Option<LibraryBodyAction>,
) {
    let widths = library_column_widths(ui, presets, usage);
    ScrollArea::horizontal()
        .id_salt("custom-sheet-size-library")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            // Cells butt against each other and carry their own gutter. The
            // dialog's item spacing would gutter the columns apart and push
            // the heading band past the page inset.
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let mut table = TableBuilder::new(ui)
                .id_salt("custom-sheet-size-library-table")
                .vscroll(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
            for width in widths {
                table = table.column(Column::exact(width));
            }
            table
                .header(LIBRARY_HEADER_HEIGHT, |mut header| {
                    for heading in LIBRARY_COLUMNS {
                        header.col(|ui| {
                            // The band is what separates the headings from the
                            // first row: six labels alone above the rows read
                            // as a seventh row of data.
                            library_band(ui, t);
                            ui.label(
                                RichText::new(heading.to_uppercase())
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            );
                            // The heading's own trailing gutter, so the next
                            // column's label never abuts this one's.
                            ui.add_space(PAGE_INSET.into());
                        });
                    }
                })
                .body(|mut body| {
                    for preset in presets {
                        let key = PresetKey::from(*preset);
                        let unresolved = unavailable(preset);
                        let used_by = usage
                            .get(&usage_key(preset.scope, &preset.id))
                            .copied()
                            .unwrap_or(0);
                        body.row(t.metrics.row_h, |mut cells| {
                            cells.col(|ui| {
                                library_cell(ui, t);
                                ui.label(
                                    RichText::new(&preset.name)
                                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                        .color(t.color.text),
                                );
                            });
                            cells.col(|ui| {
                                library_cell(ui, t);
                                ui.label(portrait_dimensions(preset));
                            });
                            cells.col(|ui| {
                                library_cell(ui, t);
                                scope_badge(ui, preset.scope);
                            });
                            cells.col(|ui| {
                                library_cell(ui, t);
                                ui.label(if used_by == 0 {
                                    "unused".to_owned()
                                } else {
                                    format!(
                                        "{used_by} reference{}",
                                        if used_by == 1 { "" } else { "s" }
                                    )
                                });
                            });
                            cells.col(|ui| {
                                library_cell(ui, t);
                                state_badge(ui, unresolved);
                            });
                            cells.col(|ui| {
                                // The verbs are centred in the row less its
                                // rule and less its two insets. Centred in the
                                // whole cell instead, the rule took a point
                                // off the gap under them and left the control
                                // visibly higher in its row than it sat low.
                                let row = ui.max_rect();
                                ui.painter().hline(
                                    row.x_range(),
                                    row.bottom() - 0.5,
                                    egui::Stroke::new(1.0, t.color.border),
                                );
                                let inset = f32::from(PAGE_INSET);
                                let content = egui::Rect::from_min_max(
                                    egui::pos2(row.left() + inset, row.top()),
                                    egui::pos2(row.right() - inset, row.bottom() - 1.0),
                                );
                                ui.scope_builder(
                                    egui::UiBuilder::new()
                                        .max_rect(content)
                                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                                    |ui| {
                                        library_verbs(
                                            ui,
                                            preset,
                                            &key,
                                            used_by,
                                            unresolved,
                                            project_writable,
                                            personal_fresh,
                                            action,
                                        );
                                    },
                                );
                            });
                        });
                    }
                });
        });
}

/// How wide each column has to be to state the widest thing in it: its own
/// heading, or one of the rows, plus the inset it opens on and the gutter it
/// closes with.
///
/// The arithmetic lives here rather than in `Column::auto` because the surface
/// is sized from it too, and a width egui settles over a frame or two cannot
/// be handed to the layout that decides how wide the surface is this frame.
fn library_column_widths(
    ui: &mut Ui,
    presets: &[&DrawingSheetPreset],
    usage: &BTreeMap<String, usize>,
) -> [f32; 6] {
    let gutters = f32::from(PAGE_INSET) * 2.0;
    let heading = theme::mono(tokens::FS_0, FontWeight::Regular);
    let name = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let body = egui::TextStyle::Body.resolve(ui.style());
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let text = |ui: &mut Ui, font: &egui::FontId, value: &str| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(value.to_owned(), font.clone(), egui::Color32::PLACEHOLDER)
                .size()
                .x
        })
    };
    let mut widths = [0.0_f32; 6];
    for (index, label) in LIBRARY_COLUMNS.iter().enumerate() {
        widths[index] = text(ui, &heading, &label.to_uppercase());
    }
    for preset in presets {
        let used_by = usage
            .get(&usage_key(preset.scope, &preset.id))
            .copied()
            .unwrap_or(0);
        let references = if used_by == 0 {
            "unused".to_owned()
        } else {
            format!("{used_by} reference{}", if used_by == 1 { "" } else { "s" })
        };
        let state = if unavailable(preset) {
            "unavailable"
        } else {
            "resolved"
        };
        let cells = [
            text(ui, &name, &preset.name),
            text(ui, &body, &portrait_dimensions(preset)),
            text(ui, &mono, &scope_label(preset.scope).to_uppercase()),
            text(ui, &body, &references),
            text(ui, &mono, state),
            0.0,
        ];
        for (column, cell) in widths.iter_mut().zip(cells) {
            *column = column.max(cell);
        }
    }
    widths[5] = widths[5].max(library_verb_width(ui));
    for column in &mut widths {
        *column = (*column + gutters).ceil();
    }
    widths
}

/// What one row's four verbs take, gaps included. Measured through the control
/// itself, so a relabelled or re-padded button cannot drift from its column.
fn library_verb_width(ui: &mut Ui) -> f32 {
    const GAP: f32 = 6.0;
    let labels = ["Use\u{2026}", "Duplicate", "Rename", "Delete"];
    // `measured_width` takes a mutable `Ui` only to lay the label out through
    // the font cache; this child allocates nothing and paints nothing.
    let mut probe = ui.new_child(egui::UiBuilder::new().max_rect(ui.max_rect()).invisible());
    let labelled: f32 = labels
        .iter()
        .map(|label| Button::new(label).compact().measured_width(&mut probe))
        .sum();
    labelled + GAP * (labels.len() - 1) as f32
}

/// The width the library's own toolbar states, which is what the surface takes
/// when the table is narrower than it.
fn library_toolbar_width(ui: &mut Ui) -> f32 {
    const GAP: f32 = 6.0;
    let labels = [
        "New custom size\u{2026}",
        "Import\u{2026}",
        "Export\u{2026}",
    ];
    let buttons: f32 = labels
        .iter()
        .map(|label| Button::new(label).measured_width(ui))
        .sum();
    buttons + GAP * labels.len() as f32 + FILTER_WIDTH + f32::from(PAGE_INSET) * 2.0
}

/// The heading band behind one column, closed by the rule that separates it
/// from the first row, and opened on the page inset.
fn library_band(ui: &mut Ui, t: &Tokens) {
    let band = ui.max_rect();
    ui.painter().rect_filled(band, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        band.x_range(),
        band.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(PAGE_INSET.into());
}

/// A row's cell: it closes itself with the row rule and opens on the page
/// inset, so every column's content starts where its heading does.
fn library_cell(ui: &mut Ui, t: &Tokens) {
    let rect = ui.max_rect();
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(PAGE_INSET.into());
}

/// One preset, stated down the page instead of across it. Below the table's
/// own floor there is no width for six columns, and a table that scrolls
/// sideways puts a row's own verbs off the surface.
fn library_entry(
    ui: &mut Ui,
    t: &Tokens,
    preset: &DrawingSheetPreset,
    usage: &BTreeMap<String, usize>,
    project_writable: bool,
    personal_fresh: bool,
    action: &mut Option<LibraryBodyAction>,
) {
    let key = PresetKey::from(preset);
    let unresolved = unavailable(preset);
    let used_by = usage
        .get(&usage_key(preset.scope, &preset.id))
        .copied()
        .unwrap_or(0);
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(PAGE_INSET, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(&preset.name)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    state_badge(ui, unresolved);
                });
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(portrait_dimensions(preset));
                scope_badge(ui, preset.scope);
                ui.weak(if used_by == 0 {
                    "unused".to_owned()
                } else {
                    format!("{used_by} reference{}", if used_by == 1 { "" } else { "s" })
                });
            });
            ui.add_space(2.0);
            ui.horizontal_wrapped(|ui| {
                library_verbs(
                    ui,
                    preset,
                    &key,
                    used_by,
                    unresolved,
                    project_writable,
                    personal_fresh,
                    action,
                );
            });
        });
    let rule = ui.cursor().top() - 0.5;
    ui.painter().hline(
        ui.max_rect().x_range(),
        rule,
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// What a reader can do to one preset, in the order they would reach for it.
fn library_verbs(
    ui: &mut Ui,
    preset: &DrawingSheetPreset,
    key: &PresetKey,
    used_by: usize,
    unresolved: bool,
    project_writable: bool,
    personal_fresh: bool,
    action: &mut Option<LibraryBodyAction>,
) {
    ui.spacing_mut().item_spacing.x = 6.0;
    if Button::new("Use\u{2026}")
        .compact()
        .enabled(!unresolved)
        .show(ui)
        .clicked()
    {
        *action = Some(LibraryBodyAction::Use(key.clone()));
    }
    if Button::new("Duplicate")
        .compact()
        .enabled(!unresolved && project_writable)
        .show(ui)
        .clicked()
    {
        *action = Some(LibraryBodyAction::Duplicate(key.clone()));
    }
    let owner_writable = match preset.scope {
        DrawingSheetPresetScope::Project => project_writable,
        DrawingSheetPresetScope::User => personal_fresh,
        DrawingSheetPresetScope::Organization => false,
    };
    if Button::new("Rename")
        .compact()
        .enabled(owner_writable)
        .show(ui)
        .clicked()
    {
        *action = Some(LibraryBodyAction::Rename(key.clone()));
    }
    let delete = Button::new("Delete")
        .compact()
        .enabled(owner_writable && used_by == 0)
        .show(ui);
    let delete = if used_by > 0 {
        delete.on_disabled_hover_text(
            "In use by a sheet or drawing-sheet default. Change every reference first \u{2014} \
             deleting a depended-on preset would leave that reference unresolvable.",
        )
    } else {
        delete
    };
    if delete.clicked() {
        *action = Some(LibraryBodyAction::Delete(key.clone()));
    }
}

pub(super) fn editor_body(
    ui: &mut Ui,
    draft: &mut PresetEditorDraft,
    validation_error: Option<&str>,
    project_error: Option<&str>,
    personal_stale: bool,
) -> Option<EditorBodyAction> {
    let t = Tokens::get(ui.ctx());
    let editing_name_only = draft.mode == PresetEditorMode::Edit;
    let available = ui.available_width();
    let mut changed = false;

    // Form and preview are one continuous page, the way the library's sections
    // are: section bands run to the page's own edges and the two columns meet
    // on a single rule, rather than floating as bordered cards over a band of
    // background. The page's fill and that rule are reserved here and painted
    // once the columns have reported how tall the taller of them is, so the
    // shorter column is backed to the same floor.
    ui.spacing_mut().item_spacing.y = 0.0;
    let page = ui.painter().add(egui::Shape::Noop);
    let seam = ui.painter().add(egui::Shape::Noop);
    let stacked = custom_editor_stacks(available);
    let form_width = (available * 0.58).max(360.0).round();
    let row = if stacked {
        ui.vertical(|ui| {
            changed |= preset_editor_form(
                ui,
                draft,
                validation_error,
                project_error,
                personal_stale,
                editing_name_only,
            );
            // Stacked, the preview's own band is what separates it from the
            // form; a hairline directly above a band is noise.
            preset_editor_preview(ui, draft);
        })
        .response
        .rect
    } else {
        let preview_width = (available - form_width - COLUMN_RULE).max(280.0);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(
                vec2(form_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    changed |= preset_editor_form(
                        ui,
                        draft,
                        validation_error,
                        project_error,
                        personal_stale,
                        editing_name_only,
                    );
                },
            );
            ui.add_space(COLUMN_RULE);
            ui.allocate_ui_with_layout(
                vec2(preview_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| preset_editor_preview(ui, draft),
            );
        })
        .response
        .rect
    };

    let page_rect = egui::Rect::from_min_max(
        egui::pos2(ui.max_rect().left(), row.top()),
        egui::pos2(ui.max_rect().right(), row.bottom()),
    );
    ui.painter().set(
        page,
        egui::Shape::rect_filled(page_rect, 0.0, t.color.bg_panel),
    );
    if !stacked {
        let x = (page_rect.left() + form_width).round() + 0.5;
        ui.painter().set(
            seam,
            egui::Shape::line_segment(
                [
                    egui::pos2(x, page_rect.top()),
                    egui::pos2(x, page_rect.bottom()),
                ],
                egui::Stroke::new(COLUMN_RULE, t.color.border),
            ),
        );
    }

    changed.then_some(EditorBodyAction::RefreshPreview)
}

/// The hairline the editor's two columns meet on.
const COLUMN_RULE: f32 = 1.0;

/// The one inset a flush page takes. Every band label and every control on the
/// page starts on it, so nothing reads as indented from its own heading.
const PAGE_INSET: i8 = 12;

/// A section band on a flush page: its fill runs to the page's own edges while
/// its label starts on the page inset, level with the controls beneath it.
fn page_band(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(egui::Margin::symmetric(PAGE_INSET, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
    ui.add_space(6.0);
}

/// A section of the editor page: its heading runs to the column's own edges,
/// and the controls below it sit on one inset.
fn editor_section(ui: &mut Ui, heading: &str, content: impl FnOnce(&mut Ui)) {
    page_band(ui, heading);
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: PAGE_INSET,
            right: PAGE_INSET,
            top: 0,
            bottom: 14,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 6.0;
            content(ui);
        });
}

fn custom_editor_stacks(available_width: f32) -> bool {
    available_width < 720.0
}

fn preset_editor_form(
    ui: &mut Ui,
    draft: &mut PresetEditorDraft,
    validation_error: Option<&str>,
    project_error: Option<&str>,
    personal_stale: bool,
    editing_name_only: bool,
) -> bool {
    let mut changed = false;
    editor_section(
        ui,
        if editing_name_only {
            "Preset identity"
        } else {
            "Identity and scope"
        },
        |form| {
            field_label(form, "Name");
            changed |= form
                .add(
                    TextEdit::singleline(&mut draft.name)
                        .char_limit(crate::state::MAX_DRAWING_SHEET_PRESET_NAME_CHARS)
                        .desired_width(f32::INFINITY),
                )
                .changed();

            form.add_space(4.0);
            field_label(form, "Scope");
            form.add_enabled_ui(!editing_name_only, |ui| {
                ComboBox::from_id_salt("custom-sheet-editor-scope")
                    .width(ui.available_width())
                    .selected_text(scope_label(draft.scope))
                    .show_ui(ui, |ui| {
                        changed |= ui
                            .selectable_value(
                                &mut draft.scope,
                                DrawingSheetPresetScope::Project,
                                "Project",
                            )
                            .changed();
                        changed |= ui
                            .selectable_value(
                                &mut draft.scope,
                                DrawingSheetPresetScope::User,
                                "Personal",
                            )
                            .changed();
                    });
            });
            form.weak(match draft.scope {
                DrawingSheetPresetScope::Project => {
                    "Saved with this project and available to collaborators."
                }
                DrawingSheetPresetScope::User => {
                    "Saved in personal Preferences. A project saves its own copy before using it."
                }
                DrawingSheetPresetScope::Organization => {
                    "Organization definitions are managed and read-only."
                }
            });
        },
    );

    if !editing_name_only {
        editor_section(ui, "Physical size", |form| {
            form.columns(2, |columns| {
                field_label(&mut columns[0], "Portrait width");
                changed |= columns[0]
                    .add(TextEdit::singleline(&mut draft.width).desired_width(f32::INFINITY))
                    .changed();
                field_label(&mut columns[1], "Portrait height");
                changed |= columns[1]
                    .add(TextEdit::singleline(&mut draft.height).desired_width(f32::INFINITY))
                    .changed();
            });
            form.add_space(4.0);
            field_label(form, "Units");
            ComboBox::from_id_salt("custom-sheet-editor-unit")
                .width(form.available_width())
                .selected_text(unit_label(draft.unit))
                .show_ui(form, |ui| {
                    for unit in [
                        PresetEditorUnit::Millimetres,
                        PresetEditorUnit::Centimetres,
                        PresetEditorUnit::Inches,
                    ] {
                        changed |= ui
                            .selectable_value(&mut draft.unit, unit, unit_label(unit))
                            .changed();
                    }
                });
            form.weak(format!(
                "50.8 to 2540 mm per edge \u{00b7} maximum {}:1 \u{00b7} stored to 1 \u{00b5}m; {} is only the editor unit.",
                crate::state::DRAWING_SHEET_MAX_ASPECT_RATIO,
                draft.unit.suffix(),
            ));
        });

        editor_section(ui, "Starting frame", |form| {
            field_label(form, "Margins and frame");
            ComboBox::from_id_salt("custom-sheet-editor-frame")
                .width(form.available_width())
                .selected_text(frame_label(draft.frame))
                .show_ui(form, |ui| {
                    for frame in [
                        StartingFrame::IsoA,
                        StartingFrame::AnsiA,
                        StartingFrame::Plain,
                        StartingFrame::None,
                    ] {
                        changed |= ui
                            .selectable_value(&mut draft.frame, frame, frame_label(frame))
                            .changed();
                    }
                });
        });
    } else {
        editor_section(ui, "Stored definition", |form| {
            key_value(
                form,
                "Portrait size",
                &portrait_dimensions_from_format(&draft.last_valid_preview),
            );
            key_value(
                form,
                "Stable identity",
                draft.source_id.as_deref().unwrap_or("\u{2014}"),
            );
            form.weak(
                "Renaming keeps the identity, physical dimensions, border, zones, title block, and every copy saved in sheets.",
            );
        });
    }

    // The notices keep a reserved band whether or not one is showing, so that
    // typing a refused dimension never resizes the dialog under the pointer.
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: PAGE_INSET,
            right: PAGE_INSET,
            top: 0,
            bottom: 12,
        })
        .show(ui, |form| {
            form.allocate_ui_with_layout(
                vec2(form.available_width(), 96.0),
                egui::Layout::top_down(egui::Align::Min),
                |form| {
                    if let Some(error) = validation_error {
                        error_notice(form, error);
                    } else if draft.unavailable {
                        warning_notice(
                            form,
                            "This definition retains an unavailable managed dependency and cannot be edited or used.",
                        );
                    } else if draft.scope == DrawingSheetPresetScope::Project {
                        if let Some(error) = project_error {
                            warning_notice(form, error);
                        }
                    } else if draft.scope == DrawingSheetPresetScope::User && personal_stale {
                        warning_notice(
                            form,
                            "Personal drawing-sheet preferences changed. Close and reopen this library.",
                        );
                    }
                    if let Some(error) = draft.error.as_deref() {
                        form.add_space(8.0);
                        error_notice(form, error);
                    }
                },
            );
        });
    changed
}

fn preset_editor_preview(ui: &mut Ui, draft: &PresetEditorDraft) {
    // Both columns open on a band at the same line, so the page reads as two
    // columns of one sheet rather than as two stacks that happen to adjoin.
    editor_section(ui, "Preview", |preview| {
        let preview_caption = custom_preview_caption(draft);
        super::super::drawing_sheet_preview::drawing_sheet_preview(
            preview,
            &draft.last_valid_preview,
            230.0,
            &preview_caption,
        );
    });
    editor_section(ui, "Preset contract", |preview| {
        key_value(preview, "Current sheet", "unchanged");
        key_value(preview, "Identity", "stable preset ID");
        key_value(
            preview,
            "Portability",
            "dimensions copied onto every using sheet",
        );
        key_value(
            preview,
            "Undo",
            "preset creation remains reversible until project save",
        );
    });
}

pub(super) fn transfer_body(
    ui: &mut Ui,
    transfer: &mut PresetTransferState,
    visible: &[DrawingSheetPreset],
) -> Option<TransferBodyAction> {
    match transfer.mode {
        TransferMode::Import => import_body(ui, transfer),
        TransferMode::Export => export_body(ui, transfer, visible),
    }
}

fn import_body(ui: &mut Ui, transfer: &mut PresetTransferState) -> Option<TransferBodyAction> {
    let mut action = None;
    let ready = transfer
        .import_candidates
        .iter()
        .filter(|candidate| candidate.selected)
        .count();
    let conflicts = transfer
        .import_candidates
        .iter()
        .filter(|candidate| candidate.selected && candidate.existing_id.is_some())
        .count();
    let banner_body = if transfer.import_candidates.is_empty() {
        "RSpice checks the file format, digest, identities, dimensions, and template dependencies before any candidate can be selected for the project."
            .to_owned()
    } else {
        format!(
            "{ready} preset{} selected; {conflicts} require identity resolution. Import creates project-owned candidates and never replaces a preset silently.",
            if ready == 1 { " is" } else { "s are" }
        )
    };
    concept_banner(
        ui,
        if transfer.import_candidates.is_empty() {
            "Choose a preset package for reviewed import."
        } else {
            "The source package remains untouched."
        },
        &banner_body,
    );
    ui.add_space(10.0);
    field_label(ui, "Preset package");
    ui.horizontal(|ui| {
        if Button::new("Choose package\u{2026}").show(ui).clicked() {
            action = Some(TransferBodyAction::ChooseImportFile);
        }
        ui.label(if transfer.package_name.is_empty() {
            "No file selected"
        } else {
            transfer.package_name.as_str()
        });
    });
    if let Some(digest) = transfer.reviewed_digest.as_deref() {
        ui.label(
            RichText::new(format!("Reviewed \u{00b7} {digest}"))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(Tokens::get(ui.ctx()).color.ok),
        );
    }
    ui.add_space(12.0);
    section_heading(ui, "Import review");
    if transfer.import_candidates.is_empty() {
        empty_state(
            ui,
            "No reviewed definitions",
            "Review a package before choosing identity and dependency resolutions.",
        );
    } else {
        ScrollArea::vertical()
            .id_salt("sheet-preset-import-candidates")
            .auto_shrink([false, true])
            .max_height(260.0)
            .show(ui, |ui| {
                Grid::new("sheet-preset-import-grid")
                    .num_columns(5)
                    .striped(true)
                    .spacing([14.0, 8.0])
                    .show(ui, |ui| {
                        table_header(ui, "Import");
                        table_header(ui, "Candidate");
                        table_header(ui, "Dimensions");
                        table_header(ui, "Dependencies");
                        table_header(ui, "Resolution");
                        ui.end_row();
                        for candidate in &mut transfer.import_candidates {
                            import_candidate_row(ui, candidate);
                        }
                    });
            });
    }
    if let Some(error) = transfer.error.as_deref() {
        ui.add_space(8.0);
        error_notice(ui, error);
    }
    ui.add_space(10.0);
    let notes = [
        (
            "Units",
            "Every candidate shows normalized dimensions; exact micrometre values survive conversion.",
        ),
        (
            "Missing templates",
            "Managed dependencies require an explicit replacement or a retained unavailable dependency.",
        ),
        (
            "Receipt",
            "Import records the source digest, schema, selected candidates, mappings, conflicts, and skips.",
        ),
    ];
    if library_notes_stack(ui.available_width()) {
        for (title, detail) in notes {
            workflow_note(ui, title, detail);
        }
    } else {
        ui.columns(3, |columns| {
            for (column, (title, detail)) in columns.iter_mut().zip(notes) {
                workflow_note(column, title, detail);
            }
        });
    }
    action
}

fn import_candidate_row(ui: &mut Ui, candidate: &mut ImportCandidate) {
    ui.checkbox(&mut candidate.selected, "");
    ui.label(&candidate.portable.name);
    ui.label(format!(
        "{} \u{00d7} {} mm",
        format_mm(candidate.portable.portrait_width_um),
        format_mm(candidate.portable.portrait_height_um)
    ));
    if candidate.missing_managed_dependency {
        ui.colored_label(
            Tokens::get(ui.ctx()).color.warn,
            "organization block \u{00b7} unavailable",
        );
    } else {
        ui.label(match candidate.portable.format.border {
            crate::state::DrawingSheetBorderTemplate::Standard => "standard border",
            crate::state::DrawingSheetBorderTemplate::Plain => "plain border",
            crate::state::DrawingSheetBorderTemplate::None => "no border",
            crate::state::DrawingSheetBorderTemplate::OrganizationManaged => "organization border",
        });
    }
    ui.add_enabled_ui(candidate.selected, |ui| {
        ComboBox::from_id_salt((
            "sheet-preset-import-resolution",
            &candidate.portable.stable_id,
        ))
        .selected_text(resolution_label(candidate.resolution))
        .show_ui(ui, |ui| {
            if candidate.existing_id.is_none() && !candidate.missing_managed_dependency {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::NewIdentity,
                    resolution_label(ImportResolution::NewIdentity),
                );
            }
            if candidate.existing_id.is_some() {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::MapExisting,
                    resolution_label(ImportResolution::MapExisting),
                );
                if !candidate.missing_managed_dependency {
                    ui.selectable_value(
                        &mut candidate.resolution,
                        ImportResolution::KeepBothRename,
                        resolution_label(ImportResolution::KeepBothRename),
                    );
                }
            }
            if candidate.resolution == ImportResolution::MatchesByDigest {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::MatchesByDigest,
                    resolution_label(ImportResolution::MatchesByDigest),
                );
            }
            if candidate.missing_managed_dependency {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::ReplaceManagedDependencies,
                    resolution_label(ImportResolution::ReplaceManagedDependencies),
                );
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::RetainUnavailableDependency,
                    resolution_label(ImportResolution::RetainUnavailableDependency),
                );
            }
            ui.selectable_value(
                &mut candidate.resolution,
                ImportResolution::Skip,
                resolution_label(ImportResolution::Skip),
            );
        });
    });
    ui.end_row();
}

fn export_body(
    ui: &mut Ui,
    transfer: &mut PresetTransferState,
    visible: &[DrawingSheetPreset],
) -> Option<TransferBodyAction> {
    let mut action = None;
    concept_banner(
        ui,
        "The package contains format definitions only.",
        "No schematic geometry, title-block field values, project identity, user paths, credentials, or results are included.",
    );
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        if Button::new("Select all").show(ui).clicked() {
            action = Some(TransferBodyAction::SelectAllExport(true));
        }
        if Button::new("Clear").show(ui).clicked() {
            action = Some(TransferBodyAction::SelectAllExport(false));
        }
        let references = if transfer.include_builtin_frame_references {
            "built-in references"
        } else {
            "no reference manifest"
        };
        let metadata = if transfer.include_source_metadata {
            "source metadata"
        } else {
            "no source metadata"
        };
        ui.weak(format!(
            "Schema 1 \u{00b7} exact micrometres \u{00b7} stable ids \u{00b7} {references} \u{00b7} {metadata} \u{00b7} SHA-256 digest"
        ));
    });
    ui.add_space(10.0);
    section_heading(ui, "Definitions");
    ScrollArea::vertical()
        .id_salt("sheet-preset-export-definitions")
        .auto_shrink([false, true])
        .max_height(360.0)
        .show(ui, |ui| {
            Grid::new("sheet-preset-export-grid")
                .num_columns(5)
                .striped(true)
                .spacing([14.0, 8.0])
                .show(ui, |ui| {
                    table_header(ui, "Export");
                    table_header(ui, "Preset");
                    table_header(ui, "Scope");
                    table_header(ui, "Portrait dimensions");
                    table_header(ui, "Dependencies");
                    ui.end_row();
                    for preset in visible {
                        let identity = super::transfer_identity(preset);
                        let mut selected = transfer.export_ids.contains(&identity);
                        let available = unsigned_exportable(preset);
                        let response =
                            ui.add_enabled(available, egui::Checkbox::new(&mut selected, ""));
                        if response.changed() {
                            if selected {
                                transfer.export_ids.insert(identity);
                            } else {
                                transfer.export_ids.remove(&identity);
                            }
                        }
                        ui.vertical(|ui| {
                            ui.label(&preset.name);
                            if !available {
                                ui.colored_label(
                                    Tokens::get(ui.ctx()).color.warn,
                                    if unavailable(preset) {
                                        "Unavailable dependency"
                                    } else {
                                        "Publisher-signed distribution only"
                                    },
                                );
                            }
                        });
                        ui.label(scope_label(preset.scope));
                        ui.label(portrait_dimensions(preset));
                        ui.label("margins \u{00b7} frame contract");
                        ui.end_row();
                    }
                });
        });
    ui.add_space(10.0);
    field_label(ui, "Package format");
    ComboBox::from_id_salt("sheet-preset-export-format")
        .width(ui.available_width())
        .selected_text(transfer.package_format.label())
        .show_ui(ui, |ui| {
            for format in [
                PresetPackageFormat::CanonicalSchema1,
                PresetPackageFormat::HumanReviewJson,
            ] {
                ui.selectable_value(&mut transfer.package_format, format, format.label());
            }
        });
    ui.checkbox(
        &mut transfer.include_builtin_frame_references,
        "Include compatible built-in border and title-block references",
    );
    ui.checkbox(
        &mut transfer.include_source_metadata,
        "Include digest and source metadata",
    );
    if let Some(error) = transfer.error.as_deref() {
        ui.add_space(8.0);
        error_notice(ui, error);
    }
    action
}

pub(super) fn delete_body(ui: &mut Ui, preset: Option<&DrawingSheetPreset>, used_by: usize) {
    if let Some(preset) = preset {
        key_value(ui, "Preset", &preset.name);
        key_value(ui, "Stable identity", &preset.id);
        key_value(ui, "Authority", scope_label(preset.scope));
        key_value(ui, "Portrait size", &portrait_dimensions(preset));
        key_value(
            ui,
            "Used by",
            &format!(
                "{used_by} drawing-sheet reference{}",
                if used_by == 1 { "" } else { "s" }
            ),
        );
    } else {
        error_notice(ui, "The selected definition is no longer available.");
    }
}

pub(super) fn error_notice(ui: &mut Ui, message: &str) {
    notice(ui, message, NoticeTone::Error);
}

fn warning_notice(ui: &mut Ui, message: &str) {
    notice(ui, message, NoticeTone::Warning);
}

/// The page's opening statement: a full-bleed inset band, not a bordered card.
fn concept_banner(ui: &mut Ui, title: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(title)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.weak(body);
        });
}

fn workflow_note(ui: &mut Ui, title: &str, body: &str) {
    egui::Frame::new()
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(title).strong());
            ui.weak(body);
        });
}

#[derive(Debug, Clone, Copy)]
enum NoticeTone {
    Warning,
    Error,
}

fn notice(ui: &mut Ui, message: &str, tone: NoticeTone) {
    let t = Tokens::get(ui.ctx());
    let color = match tone {
        NoticeTone::Warning => t.color.warn,
        NoticeTone::Error => t.color.err,
    };
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .stroke(egui::Stroke::new(1.0, color))
        .inner_margin(egui::Margin::same(9))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(message).color(color));
        });
}

fn empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.canvas_bg)
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(title)
                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.weak(detail);
            });
        });
}

fn section_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(egui::Margin::symmetric(9, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
    ui.add_space(6.0);
}

fn table_header(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn field_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_dim),
    );
}

fn key_value(ui: &mut Ui, key: &str, value: &str) {
    Grid::new(("custom-sheet-key-value", key))
        .num_columns(2)
        .spacing([16.0, 6.0])
        .show(ui, |ui| {
            ui.weak(key);
            ui.label(
                RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text),
            );
            ui.end_row();
        });
}

fn scope_badge(ui: &mut Ui, scope: DrawingSheetPresetScope) {
    let t = Tokens::get(ui.ctx());
    let color = match scope {
        DrawingSheetPresetScope::Project => t.color.accent,
        DrawingSheetPresetScope::User => t.color.ok,
        DrawingSheetPresetScope::Organization => t.color.text_dim,
    };
    ui.label(
        RichText::new(scope_label(scope).to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn state_badge(ui: &mut Ui, unavailable: bool) {
    let t = Tokens::get(ui.ctx());
    let (label, color) = if unavailable {
        ("unavailable", t.color.warn)
    } else {
        ("resolved", t.color.ok)
    };
    ui.label(
        RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn custom_preview_caption(draft: &PresetEditorDraft) -> String {
    let (width_um, height_um) = draft.last_valid_preview.portrait_dimensions_um();
    format!(
        "{} \u{00d7} {} {} portrait",
        format_dimension_um(width_um, draft.unit),
        format_dimension_um(height_um, draft.unit),
        draft.unit.suffix(),
    )
}

fn scope_label(scope: DrawingSheetPresetScope) -> &'static str {
    match scope {
        DrawingSheetPresetScope::Project => "Project",
        DrawingSheetPresetScope::User => "Personal",
        DrawingSheetPresetScope::Organization => "Organization",
    }
}

fn unit_label(unit: PresetEditorUnit) -> &'static str {
    match unit {
        PresetEditorUnit::Millimetres => "Millimetres (mm)",
        PresetEditorUnit::Centimetres => "Centimetres (cm)",
        PresetEditorUnit::Inches => "Inches (in)",
    }
}

fn frame_label(frame: StartingFrame) -> &'static str {
    match frame {
        StartingFrame::IsoA => "Copy ISO A defaults",
        StartingFrame::AnsiA => "Copy ANSI A defaults",
        StartingFrame::Plain => "Plain border",
        StartingFrame::None => "No border or title block",
    }
}

fn resolution_label(resolution: ImportResolution) -> &'static str {
    match resolution {
        ImportResolution::NewIdentity => "Import as project preset",
        ImportResolution::MatchesByDigest => "Map to identical existing definition",
        ImportResolution::KeepBothRename => "Keep both and rename import",
        ImportResolution::MapExisting => "Map to existing definition",
        ImportResolution::ReplaceManagedDependencies => "Replace managed dependencies",
        ImportResolution::RetainUnavailableDependency => "Retain as unavailable",
        ImportResolution::Skip => "Skip",
    }
}

fn portrait_dimensions(preset: &DrawingSheetPreset) -> String {
    portrait_dimensions_from_format(&preset.format)
}

fn portrait_dimensions_from_format(format: &crate::state::SchematicSheetFormat) -> String {
    let AuthoredDrawingSheetSize::Custom { snapshot } = &format.authored_size else {
        return "Not a custom size".to_owned();
    };
    format!(
        "{} \u{00d7} {} mm",
        format_mm(snapshot.portrait_width_um),
        format_mm(snapshot.portrait_height_um)
    )
}

fn format_mm(value_um: u64) -> String {
    let whole = value_um / 1_000;
    let fraction = value_um % 1_000;
    if fraction == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{fraction:03}")
            .trim_end_matches('0')
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        LIBRARY_COLUMNS, PAGE_INSET, custom_editor_stacks, library_column_widths,
        library_notes_stack, library_verb_width,
    };
    use crate::state::{DrawingSheetPreset, DrawingSheetPresetScope};

    fn measuring_ui() -> egui::Ui {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
        egui::Ui::new(
            ctx.clone(),
            egui::Id::new("custom-sheet-size-column-measure-test"),
            egui::UiBuilder::new()
                .max_rect(ctx.content_rect())
                .invisible(),
        )
    }

    fn preset(name: &str) -> DrawingSheetPreset {
        DrawingSheetPreset {
            id: name.to_lowercase().replace(' ', "-"),
            name: name.to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: super::super::model::custom_format(
                name,
                250_000,
                400_000,
                super::StartingFrame::IsoA,
            )
            .expect("a valid custom format"),
        }
    }

    /// Every column states the widest thing in it, and no more: its own
    /// heading or one of the rows, on the page inset at either end. The
    /// surface is sized from these, so a column that overshot would leave a
    /// band of surface no column reaches — the dead space the authored widths
    /// left — and one that undershot would clip a value.
    #[test]
    fn a_column_is_as_wide_as_its_widest_cell_and_its_two_insets() {
        let mut ui = measuring_ui();
        let short = [preset("A4")];
        let long = [preset("Lab documentation panel, landscape")];
        let usage = BTreeMap::new();
        let narrow = library_column_widths(&mut ui, &short.iter().collect::<Vec<_>>(), &usage);
        let wide = library_column_widths(&mut ui, &long.iter().collect::<Vec<_>>(), &usage);

        assert!(
            wide[0] > narrow[0],
            "the name column answers to the longest name: {} then {}",
            narrow[0],
            wide[0]
        );
        for (index, heading) in LIBRARY_COLUMNS.iter().enumerate().skip(1) {
            assert_eq!(
                narrow[index], wide[index],
                "{heading} holds the same values in both, so it is the same width"
            );
        }
        // "A4" is shorter than the heading over it, so that column is its
        // heading's width: the floor every column is measured against.
        let inset = f32::from(PAGE_INSET) * 2.0;
        assert!(
            narrow[0] > inset,
            "a column is its content plus its two insets, not the insets alone"
        );
        let verbs = library_verb_width(&mut ui);
        assert!(
            narrow[5] >= verbs + inset && narrow[5] < verbs + inset + 2.0,
            "the actions column is its four verbs plus its two insets: {} against {verbs}",
            narrow[5]
        );
    }

    #[test]
    fn library_notes_stack_at_phone_width() {
        assert!(library_notes_stack(390.0));
        assert!(library_notes_stack(619.0));
        assert!(!library_notes_stack(620.0));
        assert!(!library_notes_stack(834.0));
    }

    #[test]
    fn custom_editor_stacks_before_its_preview_can_leave_the_viewport() {
        assert!(custom_editor_stacks(366.0));
        assert!(custom_editor_stacks(719.0));
        assert!(!custom_editor_stacks(720.0));
        assert!(!custom_editor_stacks(980.0));
    }
}

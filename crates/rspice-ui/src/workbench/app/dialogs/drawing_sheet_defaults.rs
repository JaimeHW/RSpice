//! Project and personal defaults for future drawing sheets.
//!
//! This surface edits isolated candidates. Project changes publish through the
//! design-management revision/history authority; personal changes publish
//! through `UserPreferences`. Existing authored sheet records are never
//! rewritten by this workflow.
//!
//! The surface is one fused drafting catalog: two seed-record bands and the
//! new-sheet behavior band across the top, a chip catalog of every format the
//! edited record may start from, one live preview on the desk surface, and a
//! single frame row for the presentation properties. Regions meet on 1 px
//! seams inside one border — no floating cards, no gutters.

use egui::{
    Align, Align2, Color32, ComboBox, Context, Frame, Layout, Rect, RichText, Sense, Stroke,
    StrokeKind, Ui, UiBuilder,
    text::{LayoutJob, TextFormat},
    pos2, vec2,
};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    AuthoredDrawingSheetSize, DrawingSheetBorderTemplate, DrawingSheetDisplayUnit,
    DrawingSheetInheritance, DrawingSheetNewSheetPolicy, DrawingSheetPreset,
    DrawingSheetPresetScope, DrawingSheetProjectSettings, DrawingSheetStandard,
    DrawingSheetStandardSeries, DrawingSheetTitleBlock, DrawingSheetTitleBlockTemplate,
    SchematicPageOrientation, SchematicSheetFormat,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::DrawingSheetPersonalPreferences;
use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};

use super::drawing_sheet_preview::{drawing_sheet_preview, sheet_desk_color};

const EYEBROW: &str = "NEW PROJECTS \u{00b7} NEW SHEETS \u{00b7} INHERITANCE";
const TITLE: &str = "Drawing-sheet defaults";
const DESCRIPTION: &str = "Choose what future projects and sheets start with. A default seeds new work; changing one never rewrites an authored sheet.";

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DrawingSheetDefaultsTarget {
    #[default]
    Project,
    Personal,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct DrawingSheetDefaultsDialogState {
    pub(crate) open: bool,
    edit: Option<SchematicEditAuthority>,
    catalog_revision: u64,
    baseline_project: DrawingSheetProjectSettings,
    draft_project: DrawingSheetProjectSettings,
    baseline_personal: DrawingSheetPersonalPreferences,
    draft_personal: DrawingSheetPersonalPreferences,
    target: DrawingSheetDefaultsTarget,
    error: Option<String>,
}

pub(crate) fn open_drawing_sheet_defaults(state: &mut AppState) -> bool {
    if state.dialogs.drawing_sheet_defaults.open {
        return false;
    }
    let project = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .clone();
    let personal = state.ui.preferences.drawing_sheet_personal_preferences();
    state.dialogs.drawing_sheet_defaults = DrawingSheetDefaultsDialogState {
        open: true,
        edit: Some(SchematicEditAuthority::capture(state)),
        catalog_revision: state.workspace.design_management.revision(),
        baseline_project: project.clone(),
        draft_project: project,
        baseline_personal: personal.clone(),
        draft_personal: personal,
        target: DrawingSheetDefaultsTarget::Project,
        error: None,
    };
    true
}

/// Adopt outside changes to either saved record while its draft is untouched.
///
/// The catalog below the dialog can gain a preset (the "New custom size…"
/// chip stacks the preset library above this dialog) or another surface can
/// save settings. An untouched draft silently follows so the catalog stays
/// current; a touched draft keeps its baseline and the authority check
/// reports the conflict instead.
fn refresh_untouched_baselines(state: &mut AppState) {
    let catalog_revision = state.workspace.design_management.revision();
    if state.dialogs.drawing_sheet_defaults.catalog_revision != catalog_revision
        && state.dialogs.drawing_sheet_defaults.draft_project
            == state.dialogs.drawing_sheet_defaults.baseline_project
    {
        let project = state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .clone();
        let authority = SchematicEditAuthority::capture(state);
        let dialog = &mut state.dialogs.drawing_sheet_defaults;
        dialog.baseline_project = project.clone();
        dialog.draft_project = project;
        dialog.catalog_revision = catalog_revision;
        dialog.edit = Some(authority);
    }
    let personal_now = state.ui.preferences.drawing_sheet_personal_preferences();
    let dialog = &mut state.dialogs.drawing_sheet_defaults;
    if dialog.draft_personal == dialog.baseline_personal && personal_now != dialog.baseline_personal
    {
        dialog.baseline_personal = personal_now.clone();
        dialog.draft_personal = personal_now;
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_defaults_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.drawing_sheet_defaults.open {
            return;
        }
        refresh_untouched_baselines(&mut self.state);
        let following_sheets = {
            let key = self.state.workspace.active_key();
            self.state
                .workspace
                .design_management
                .sheet_catalog(&key)
                .map(|catalog| {
                    let sheets = catalog.sheets();
                    let following = sheets
                        .iter()
                        .filter(|sheet| {
                            sheet.page_format().inheritance
                                == DrawingSheetInheritance::ProjectDefault
                        })
                        .count();
                    (following, sheets.len())
                })
                .filter(|(_, total)| *total > 0)
        };
        let transaction = &self.state.dialogs.drawing_sheet_defaults;
        let project_changed = transaction.draft_project != transaction.baseline_project;
        let personal_changed = transaction.draft_personal != transaction.baseline_personal;
        let project_authority = project_changed
            .then(|| validate_project_authority(&self.state, transaction).err())
            .flatten();
        let project_editable = validate_project_authority(&self.state, transaction).is_ok();
        let valid = transaction.draft_project.validate().is_ok()
            && transaction.draft_personal.validate().is_ok();
        let enabled = valid && project_authority.is_none() && (project_changed || personal_changed);

        let mut open_presets = false;
        let choice = Dialog::new(EYEBROW, TITLE, "Save sheet defaults")
            .description(DESCRIPTION)
            .size(DialogSize::DrawingSheetWorkflow)
            .ghost("Cancel")
            .primary_enabled(enabled)
            .show(ctx, |ui| {
                drawing_sheet_defaults_body(
                    ui,
                    &mut self.state.dialogs.drawing_sheet_defaults,
                    project_authority.as_deref(),
                    project_editable,
                    following_sheets,
                    &mut open_presets,
                );
            });
        if open_presets {
            super::drawing_sheet_presets::open_custom_sheet_size_library(&mut self.state);
        }
        match choice {
            DialogChoice::Primary => match apply_drawing_sheet_defaults(self) {
                Ok(message) => {
                    self.state.push_user_message(ConsoleMessage::info(message));
                    self.state.dialogs.drawing_sheet_defaults = Default::default();
                }
                Err(error) => self.state.dialogs.drawing_sheet_defaults.error = Some(error),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_defaults = Default::default();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

// ── Fused-shell geometry ────────────────────────────────────────────────────

const SHELL_ROW1_H: f32 = 84.0;
const CHIP_H: f32 = 26.0;
const CHIP_GAP: f32 = 5.0;
const SERIES_LABEL_H: f32 = 15.0;
const SERIES_GAP: f32 = 6.0;
const CATALOG_PAD_X: f32 = 11.0;
const CATALOG_PAD_TOP: f32 = 8.0;
const CATALOG_PAD_BOTTOM: f32 = 10.0;
const PREVIEW_W_WIDE: f32 = 324.0;
const PREVIEW_W_NARROW: f32 = 252.0;
const STACK_BREAKPOINT: f32 = 900.0;

#[derive(Clone, Copy, PartialEq)]
enum ChipAction {
    Standard(DrawingSheetStandard),
    Preset(usize),
    NewPreset,
}

struct ChipSpec {
    strong: String,
    dims: String,
    action: ChipAction,
    selected: bool,
    natural_w: f32,
}

struct ChipSeries {
    label: &'static str,
    note: String,
    chips: Vec<ChipSpec>,
    rows: Vec<Vec<usize>>,
}

enum RecordEdit {
    Format(ChipAction),
    Orientation(SchematicPageOrientation),
    Unit(DrawingSheetDisplayUnit),
    Border(DrawingSheetBorderTemplate),
    TitleBlock(DrawingSheetTitleBlockTemplate),
    Reset,
}

fn drawing_sheet_defaults_body(
    ui: &mut Ui,
    state: &mut DrawingSheetDefaultsDialogState,
    project_authority_error: Option<&str>,
    project_editable: bool,
    following_sheets: Option<(usize, usize)>,
    open_presets: &mut bool,
) {
    let t = Tokens::get(ui.ctx());
    let target = state.target;
    let record_enabled = match target {
        DrawingSheetDefaultsTarget::Project => project_editable,
        DrawingSheetDefaultsTarget::Personal => true,
    };
    let record = match target {
        DrawingSheetDefaultsTarget::Project => state.draft_project.default_format.clone(),
        DrawingSheetDefaultsTarget::Personal => state.draft_personal.default_format.clone(),
    };
    let authority = match target {
        DrawingSheetDefaultsTarget::Project => state.baseline_project.default_format.clone(),
        DrawingSheetDefaultsTarget::Personal => state.baseline_personal.default_format.clone(),
    };
    let presets: Vec<DrawingSheetPreset> = match target {
        DrawingSheetDefaultsTarget::Project => state
            .baseline_project
            .presets
            .iter()
            .filter(|preset| preset.scope == DrawingSheetPresetScope::Project)
            .cloned()
            .collect(),
        DrawingSheetDefaultsTarget::Personal => state
            .baseline_personal
            .presets
            .iter()
            .filter(|preset| preset.scope == DrawingSheetPresetScope::User)
            .cloned()
            .collect(),
    };

    let avail = ui.available_width();
    let stacked = avail < STACK_BREAKPOINT;
    let preview_w = if avail < 1_100.0 {
        PREVIEW_W_NARROW
    } else {
        PREVIEW_W_WIDE
    };
    let inner_w = avail - 2.0;
    let left_w = if stacked {
        inner_w
    } else {
        (inner_w - 1.0 - preview_w).max(320.0)
    };

    let series_list = build_chip_series(ui, target, &record, &presets, left_w);
    let catalog_h = catalog_height(&series_list);
    let frame_h = frame_row_height(&t, stacked);

    let mut target_click: Option<DrawingSheetDefaultsTarget> = None;
    let mut edits: Vec<RecordEdit> = Vec::new();

    if stacked {
        let band_h = SHELL_ROW1_H;
        let preview_h = 250.0;
        let total_h =
            2.0 + band_h * 3.0 + 2.0 + 1.0 + catalog_h + 1.0 + preview_h + 1.0 + frame_h;
        let (shell, _) = ui.allocate_exact_size(vec2(avail, total_h), Sense::hover());
        ui.painter().rect_filled(shell, 3.0, t.color.border);
        let x0 = shell.left() + 1.0;
        let mut y = shell.top() + 1.0;
        for (band_target, salt) in [
            (DrawingSheetDefaultsTarget::Project, "target-project"),
            (DrawingSheetDefaultsTarget::Personal, "target-personal"),
        ] {
            let rect = Rect::from_min_size(pos2(x0, y), vec2(inner_w, band_h));
            if target_band(ui, rect, state, band_target, salt) {
                target_click = Some(band_target);
            }
            y += band_h + 1.0;
        }
        let rect = Rect::from_min_size(pos2(x0, y), vec2(inner_w, band_h));
        behavior_band(ui, rect, state, project_editable);
        y += band_h + 1.0;
        let rect = Rect::from_min_size(pos2(x0, y), vec2(inner_w, catalog_h));
        if let Some(action) = catalog_region(ui, rect, &series_list, record_enabled) {
            match action {
                ChipAction::NewPreset => *open_presets = true,
                action => edits.push(RecordEdit::Format(action)),
            }
        }
        y += catalog_h + 1.0;
        let rect = Rect::from_min_size(pos2(x0, y), vec2(inner_w, preview_h));
        preview_region(ui, rect, &record, target, following_sheets);
        y += preview_h + 1.0;
        let rect = Rect::from_min_size(pos2(x0, y), vec2(inner_w, frame_h));
        frame_row(ui, rect, &record, &authority, record_enabled, true, &mut edits);
    } else {
        let band_unit = (left_w - 2.0) / 2.9;
        let band1_w = band_unit.floor();
        let band3_w = left_w - 2.0 - band1_w * 2.0;
        let total_h = 2.0 + SHELL_ROW1_H + 1.0 + catalog_h + 1.0 + frame_h;
        let (shell, _) = ui.allocate_exact_size(vec2(avail, total_h), Sense::hover());
        ui.painter().rect_filled(shell, 3.0, t.color.border);
        let x0 = shell.left() + 1.0;
        let y0 = shell.top() + 1.0;

        let mut x = x0;
        for (band_target, salt, width) in [
            (DrawingSheetDefaultsTarget::Project, "target-project", band1_w),
            (
                DrawingSheetDefaultsTarget::Personal,
                "target-personal",
                band1_w,
            ),
        ] {
            let rect = Rect::from_min_size(pos2(x, y0), vec2(width, SHELL_ROW1_H));
            if target_band(ui, rect, state, band_target, salt) {
                target_click = Some(band_target);
            }
            x += width + 1.0;
        }
        let rect = Rect::from_min_size(pos2(x, y0), vec2(band3_w, SHELL_ROW1_H));
        behavior_band(ui, rect, state, project_editable);

        let catalog_rect = Rect::from_min_size(
            pos2(x0, y0 + SHELL_ROW1_H + 1.0),
            vec2(left_w, catalog_h),
        );
        if let Some(action) = catalog_region(ui, catalog_rect, &series_list, record_enabled) {
            match action {
                ChipAction::NewPreset => *open_presets = true,
                action => edits.push(RecordEdit::Format(action)),
            }
        }

        let preview_rect = Rect::from_min_size(
            pos2(x0 + left_w + 1.0, y0),
            vec2(preview_w, SHELL_ROW1_H + 1.0 + catalog_h),
        );
        preview_region(ui, preview_rect, &record, target, following_sheets);

        let frame_rect = Rect::from_min_size(
            pos2(x0, y0 + SHELL_ROW1_H + 1.0 + catalog_h + 1.0),
            vec2(inner_w, frame_h),
        );
        frame_row(
            ui,
            frame_rect,
            &record,
            &authority,
            record_enabled,
            false,
            &mut edits,
        );
    }

    if let Some(next) = target_click {
        state.target = next;
    }
    if record_enabled {
        apply_record_edits(state, target, &authority, &presets, edits);
    }

    // The strip exists only while there is something to say — an empty
    // reserved slot would read as a designed-in gap. The dialog height is
    // content-fit, so growing for an error is the intended behavior.
    if let Some(error) = project_authority_error.or(state.error.as_deref()) {
        ui.add_space(8.0);
        let t = Tokens::get(ui.ctx());
        Frame::NONE
            .fill(t.color.bg_panel)
            .stroke(egui::Stroke::new(1.0, t.color.err.gamma_multiply(0.5)))
            .inner_margin(egui::Margin::symmetric(10, 7))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.colored_label(t.color.err, error);
            });
    }
}

fn apply_record_edits(
    state: &mut DrawingSheetDefaultsDialogState,
    target: DrawingSheetDefaultsTarget,
    authority: &SchematicSheetFormat,
    presets: &[DrawingSheetPreset],
    edits: Vec<RecordEdit>,
) {
    if edits.is_empty() {
        return;
    }
    let record = match target {
        DrawingSheetDefaultsTarget::Project => &mut state.draft_project.default_format,
        DrawingSheetDefaultsTarget::Personal => &mut state.draft_personal.default_format,
    };
    for edit in edits {
        match edit {
            RecordEdit::Format(ChipAction::Standard(standard)) => {
                *record = retarget_default_to_standard(record, standard);
            }
            RecordEdit::Format(ChipAction::Preset(index)) => {
                if let Some(preset) = presets.get(index) {
                    *record = retarget_default(record, preset.format.clone());
                }
            }
            RecordEdit::Format(ChipAction::NewPreset) => {}
            RecordEdit::Orientation(orientation) => {
                if let Ok(updated) = record.try_update(|draft| draft.orientation = orientation) {
                    *record = updated;
                }
            }
            RecordEdit::Unit(unit) => {
                if let Ok(updated) = record.try_update(|draft| draft.display_unit = unit) {
                    *record = updated;
                }
            }
            RecordEdit::Border(border) => {
                if let Ok(updated) = record.try_update(|draft| draft.apply_border_template(border))
                {
                    *record = updated;
                }
            }
            RecordEdit::TitleBlock(template) => {
                if let Ok(updated) = record.try_update(|draft| {
                    draft.title_block.template = template;
                    if template == DrawingSheetTitleBlockTemplate::None {
                        draft.title_block.offset_x_um = 0;
                        draft.title_block.offset_y_um = 0;
                        draft.title_block.rotation =
                            crate::state::DrawingSheetTitleBlockRotation::Upright;
                    }
                }) {
                    *record = updated;
                }
            }
            RecordEdit::Reset => {
                *record = match target {
                    DrawingSheetDefaultsTarget::Project => {
                        DrawingSheetProjectSettings::default().default_format
                    }
                    DrawingSheetDefaultsTarget::Personal => {
                        DrawingSheetPersonalPreferences::default().default_format
                    }
                };
            }
        }
        enforce_managed_default_authority(record, authority);
    }
}

// ── Row 1: seed-record target bands ─────────────────────────────────────────

fn target_band(
    ui: &mut Ui,
    rect: Rect,
    state: &DrawingSheetDefaultsDialogState,
    band_target: DrawingSheetDefaultsTarget,
    salt: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let active = state.target == band_target;
    let format = match band_target {
        DrawingSheetDefaultsTarget::Project => &state.draft_project.default_format,
        DrawingSheetDefaultsTarget::Personal => &state.draft_personal.default_format,
    };
    let (kicker, seeds) = match band_target {
        DrawingSheetDefaultsTarget::Project => ("PROJECT DEFAULT", "seeds new sheets here"),
        DrawingSheetDefaultsTarget::Personal => ("PERSONAL DEFAULT", "seeds new projects"),
    };
    let response = ui.interact(rect, ui.id().with(("sheet-defaults", salt)), Sense::click());
    let painter = ui.painter();
    let fill = if active {
        t.color.accent.gamma_multiply(0.11)
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_panel_2
    };
    painter.rect_filled(rect, 0.0, fill);
    if active {
        painter.rect_filled(
            Rect::from_min_max(pos2(rect.left(), rect.bottom() - 2.0), rect.max),
            0.0,
            t.color.accent.gamma_multiply(0.43),
        );
    }

    let strong = default_record_summary(format);
    let small = format!(
        "{} \u{00b7} {} \u{2014} {}",
        border_template_label(format.border),
        title_block_template_label(format.title_block.template),
        seeds
    );
    let wrap = rect.width() - 22.0;
    let small_galley = painter.layout(
        small,
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_dim,
        wrap,
    );
    let content_h = 13.0 + 3.0 + 14.0 + 3.0 + small_galley.size().y;
    let x = rect.left() + 11.0;
    let mut y = rect.top() + ((rect.height() - content_h) / 2.0).max(8.0);
    painter.text(
        pos2(x, y),
        Align2::LEFT_TOP,
        kicker,
        theme::sans(tokens::FS_MICRO, FontWeight::Medium),
        if active {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    y += 16.0;
    painter.text(
        pos2(x, y),
        Align2::LEFT_TOP,
        strong.clone(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    y += 17.0;
    painter.galley(pos2(x, y), small_galley, t.color.text_dim);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            true,
            active,
            format!("{kicker}: {strong}"),
        )
    });
    response.clicked()
}

// ── Row 1: new-sheet behavior band ──────────────────────────────────────────

fn behavior_band(
    ui: &mut Ui,
    rect: Rect,
    state: &mut DrawingSheetDefaultsDialogState,
    project_editable: bool,
) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    let inner = Rect::from_min_max(
        pos2(rect.left() + 11.0, rect.top() + 8.0),
        pos2(rect.right() - 11.0, rect.bottom() - 6.0),
    );
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
        |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.label(
                RichText::new("NEW SHEETS")
                    .font(theme::sans(tokens::FS_MICRO, FontWeight::Medium))
                    .color(t.color.text_faint),
            );
            ui.add_enabled_ui(project_editable, |ui| {
                let selected = state.draft_project.new_sheet_policy;
                ComboBox::from_id_salt("drawing-sheet-new-policy")
                    .width(ui.available_width())
                    .selected_text(new_sheet_policy_label(selected))
                    .show_ui(ui, |ui| {
                        for value in [
                            DrawingSheetNewSheetPolicy::ProjectDefault,
                            DrawingSheetNewSheetPolicy::Ask,
                            DrawingSheetNewSheetPolicy::MatchCurrent,
                        ] {
                            ui.selectable_value(
                                &mut state.draft_project.new_sheet_policy,
                                value,
                                new_sheet_policy_label(value),
                            )
                            .on_hover_text(new_sheet_policy_note(value));
                        }
                    })
                    .response
                    .on_hover_text(new_sheet_policy_note(selected));
                ui.checkbox(
                    &mut state.draft_project.remember_last_explicit_format,
                    RichText::new("Remember the last explicit choice")
                        .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        },
    );
}

// ── Row 2: the format chip catalog ──────────────────────────────────────────

fn build_chip_series(
    ui: &Ui,
    target: DrawingSheetDefaultsTarget,
    record: &SchematicSheetFormat,
    presets: &[DrawingSheetPreset],
    region_w: f32,
) -> Vec<ChipSeries> {
    let strong_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let dims_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let measure = |font: &egui::FontId, text: &str| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
                .size()
                .x
        })
    };
    let selected_standard = match record.authored_size {
        AuthoredDrawingSheetSize::Standard { standard } => Some(standard),
        AuthoredDrawingSheetSize::Custom { .. } => None,
    };
    let selected_preset_id = match &record.authored_size {
        AuthoredDrawingSheetSize::Custom { snapshot } => snapshot.preset_id.clone(),
        AuthoredDrawingSheetSize::Standard { .. } => None,
    };

    let chip = |strong: String, dims: String, action: ChipAction, selected: bool| {
        let natural_w =
            10.0 + measure(&strong_font, &strong) + 6.0 + measure(&dims_font, &dims) + 10.0;
        ChipSpec {
            strong,
            dims,
            action,
            selected,
            natural_w,
        }
    };

    let mut series_list: Vec<ChipSeries> = [
        (
            DrawingSheetStandardSeries::Iso,
            "ISO A",
            "ISO 216 \u{00b7} ISO 5457 borders and zones",
        ),
        (
            DrawingSheetStandardSeries::Ansi,
            "ANSI",
            "ASME Y14.1 \u{00b7} inch drawing sizes",
        ),
        (
            DrawingSheetStandardSeries::Architectural,
            "ARCH",
            "Architectural inch series",
        ),
        (
            DrawingSheetStandardSeries::Jis,
            "JIS B",
            "JIS P 0138 \u{00b7} Japanese industrial B",
        ),
    ]
    .into_iter()
    .map(|(series, label, note)| ChipSeries {
        label,
        note: note.to_owned(),
        chips: DrawingSheetStandard::ALL
            .into_iter()
            .filter(|standard| standard.series() == series)
            .map(|standard| {
                chip(
                    standard_short_label(standard),
                    oriented_standard_size(standard, record),
                    ChipAction::Standard(standard),
                    selected_standard == Some(standard),
                )
            })
            .collect(),
        rows: Vec::new(),
    })
    .collect();

    let custom_note = match target {
        DrawingSheetDefaultsTarget::Project => "project presets \u{00b7} travel with the project",
        DrawingSheetDefaultsTarget::Personal => {
            "personal presets \u{00b7} copied into a project on first use"
        }
    };
    let mut custom_chips: Vec<ChipSpec> = presets
        .iter()
        .enumerate()
        .map(|(index, preset)| {
            chip(
                preset.name.clone(),
                oriented_preset_size(preset, record),
                ChipAction::Preset(index),
                selected_preset_id.as_deref() == Some(preset.id.as_str()),
            )
        })
        .collect();
    let add_label = "New custom size\u{2026}".to_owned();
    let strong_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    custom_chips.push(ChipSpec {
        natural_w: 10.0 + 13.0 + 6.0 + measure(&strong_font, &add_label) + 10.0,
        strong: add_label,
        dims: String::new(),
        action: ChipAction::NewPreset,
        selected: false,
    });
    series_list.push(ChipSeries {
        label: "CUSTOM",
        note: custom_note.to_owned(),
        chips: custom_chips,
        rows: Vec::new(),
    });

    let row_w = region_w - 2.0 * CATALOG_PAD_X;
    for series in &mut series_list {
        series.rows = pack_chip_rows(&series.chips, row_w);
    }
    series_list
}

fn pack_chip_rows(chips: &[ChipSpec], avail: f32) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut row: Vec<usize> = Vec::new();
    let mut used = 0.0;
    for (index, spec) in chips.iter().enumerate() {
        let width = spec.natural_w.min(avail);
        if !row.is_empty() && used + CHIP_GAP + width > avail {
            rows.push(std::mem::take(&mut row));
            used = 0.0;
        }
        used += if row.is_empty() {
            width
        } else {
            CHIP_GAP + width
        };
        row.push(index);
    }
    if !row.is_empty() {
        rows.push(row);
    }
    rows
}

fn catalog_height(series_list: &[ChipSeries]) -> f32 {
    let mut height = CATALOG_PAD_TOP + CATALOG_PAD_BOTTOM;
    for (index, series) in series_list.iter().enumerate() {
        if index > 0 {
            height += SERIES_GAP;
        }
        let rows = series.rows.len().max(1) as f32;
        height += SERIES_LABEL_H + 3.0 + rows * CHIP_H + (rows - 1.0) * CHIP_GAP;
    }
    height
}

fn catalog_region(
    ui: &mut Ui,
    rect: Rect,
    series_list: &[ChipSeries],
    enabled: bool,
) -> Option<ChipAction> {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    let alpha = if enabled { 1.0 } else { 0.72 };
    let row_w = rect.width() - 2.0 * CATALOG_PAD_X;
    let mut clicked: Option<ChipAction> = None;
    let mut y = rect.top() + CATALOG_PAD_TOP;
    for (series_index, series) in series_list.iter().enumerate() {
        if series_index > 0 {
            y += SERIES_GAP;
        }
        let label_pos = pos2(rect.left() + CATALOG_PAD_X, y);
        let label_rect = ui.painter().text(
            label_pos,
            Align2::LEFT_TOP,
            series.label,
            theme::sans(tokens::FS_MICRO, FontWeight::SemiBold),
            t.color.text_dim.gamma_multiply(alpha),
        );
        ui.painter().text(
            pos2(label_rect.right() + 8.0, y),
            Align2::LEFT_TOP,
            &series.note,
            theme::sans(tokens::FS_MICRO, FontWeight::Regular),
            t.color.text_faint.gamma_multiply(alpha),
        );
        y += SERIES_LABEL_H + 3.0;
        for row in &series.rows {
            let natural: f32 = row
                .iter()
                .map(|&index| series.chips[index].natural_w.min(row_w))
                .sum::<f32>()
                + CHIP_GAP * (row.len().saturating_sub(1)) as f32;
            let extra = ((row_w - natural) / row.len() as f32).max(0.0);
            let mut x = rect.left() + CATALOG_PAD_X;
            for &index in row {
                let spec = &series.chips[index];
                let width = spec.natural_w.min(row_w) + extra;
                let chip_rect = Rect::from_min_size(pos2(x, y), vec2(width, CHIP_H));
                if chip_button(ui, chip_rect, spec, (series_index, index), enabled, alpha) {
                    clicked = Some(spec.action);
                }
                x += width + CHIP_GAP;
            }
            y += CHIP_H + CHIP_GAP;
        }
        y -= CHIP_GAP;
    }
    clicked
}

fn chip_button(
    ui: &mut Ui,
    rect: Rect,
    spec: &ChipSpec,
    id_salt: (usize, usize),
    enabled: bool,
    alpha: f32,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let response = ui.interact(rect, ui.id().with(("sheet-format-chip", id_salt)), sense);
    let painter = ui.painter();
    let dashed = spec.action == ChipAction::NewPreset;
    let highlight = spec.selected || (enabled && response.hovered());
    let fill = if dashed {
        if enabled && response.hovered() {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        }
    } else if highlight {
        t.color.accent.gamma_multiply(0.11)
    } else {
        t.color.bg_inset
    };
    painter.rect_filled(rect, 3.0, fill);
    let border_color = if highlight && !dashed {
        t.color.accent.gamma_multiply(0.43)
    } else {
        t.color.border
    };
    if dashed {
        dashed_rect(painter, rect.shrink(0.5), Stroke::new(1.0, border_color));
    } else {
        painter.rect_stroke(
            rect,
            3.0,
            Stroke::new(1.0, border_color),
            StrokeKind::Inside,
        );
    }

    let strong_font = if dashed {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::SemiBold)
    };
    let dims_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let measure = |font: &egui::FontId, text: &str| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
                .size()
                .x
        })
    };
    let strong_color = if dashed {
        t.color.text_dim.gamma_multiply(alpha)
    } else {
        t.color.text.gamma_multiply(alpha)
    };
    if dashed {
        let glyph_w = 9.0;
        let text_w = measure(&strong_font, &spec.strong);
        let start = rect.center().x - (glyph_w + 6.0 + text_w) / 2.0;
        let glyph_center = pos2(start + glyph_w / 2.0, rect.center().y);
        let stroke = Stroke::new(1.2, t.color.text_faint.gamma_multiply(alpha));
        painter.line_segment(
            [
                pos2(glyph_center.x - 4.0, glyph_center.y),
                pos2(glyph_center.x + 4.0, glyph_center.y),
            ],
            stroke,
        );
        painter.line_segment(
            [
                pos2(glyph_center.x, glyph_center.y - 4.0),
                pos2(glyph_center.x, glyph_center.y + 4.0),
            ],
            stroke,
        );
        painter.text(
            pos2(start + glyph_w + 6.0, rect.center().y),
            Align2::LEFT_CENTER,
            &spec.strong,
            strong_font,
            strong_color,
        );
    } else {
        let strong_w = measure(&strong_font, &spec.strong);
        let dims_w = measure(&dims_font, &spec.dims);
        let content_w = strong_w + if spec.dims.is_empty() { 0.0 } else { 6.0 + dims_w };
        let start = (rect.center().x - content_w / 2.0).max(rect.left() + 6.0);
        painter.text(
            pos2(start, rect.center().y),
            Align2::LEFT_CENTER,
            &spec.strong,
            strong_font,
            strong_color,
        );
        if !spec.dims.is_empty() {
            painter.text(
                pos2(start + strong_w + 6.0, rect.center().y),
                Align2::LEFT_CENTER,
                &spec.dims,
                dims_font,
                t.color.text_dim.gamma_multiply(alpha),
            );
        }
    }

    if enabled && response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.widget_info(|| {
        if spec.action == ChipAction::NewPreset {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, spec.strong.clone())
        } else {
            egui::WidgetInfo::selected(
                egui::WidgetType::RadioButton,
                enabled,
                spec.selected,
                format!("{} \u{00b7} {}", spec.strong, spec.dims),
            )
        }
    });
    enabled && response.clicked()
}

fn dashed_rect(painter: &egui::Painter, rect: Rect, stroke: Stroke) {
    let corners = [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
        rect.left_top(),
    ];
    for pair in corners.windows(2) {
        painter.extend(egui::Shape::dashed_line(pair, stroke, 4.0, 3.0));
    }
}

// ── Preview column ──────────────────────────────────────────────────────────

fn preview_region(
    ui: &mut Ui,
    rect: Rect,
    record: &SchematicSheetFormat,
    target: DrawingSheetDefaultsTarget,
    following_sheets: Option<(usize, usize)>,
) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, sheet_desk_color(ui.ctx()));

    let facts = preview_facts_galley(ui, rect, record, target, following_sheets);
    let warning = record.title_block_substituted().then(|| {
        let (width, _) = record.oriented_dimensions_um();
        let text = format!(
            "{} needs a wider page than {} {}; new sheets draw the compact block until the format grows.",
            title_block_template_label(record.title_block.template),
            record.display_unit.format_um(width),
            record.display_unit.suffix(),
        );
        let mut job = LayoutJob::simple(
            text,
            theme::sans(tokens::FS_MICRO, FontWeight::Regular),
            t.color.warn,
            rect.width() - 26.0,
        );
        job.halign = Align::Center;
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    });

    let preview_h = 190.0;
    let mut content_h = preview_h + 7.0 + facts.size().y;
    if let Some(warning) = &warning {
        content_h += 3.0 + warning.size().y;
    }
    let top = rect.top() + ((rect.height() - content_h) / 2.0).max(11.0);

    let preview_rect = Rect::from_min_size(
        pos2(rect.left() + 13.0, top),
        vec2(rect.width() - 26.0, preview_h),
    );
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(preview_rect)
            .layout(Layout::top_down(Align::Min)),
        |ui| {
            drawing_sheet_preview(ui, record, preview_h, &default_record_summary(record));
        },
    );

    let mut y = top + preview_h + 7.0;
    ui.painter()
        .galley(pos2(rect.center().x, y), facts.clone(), t.color.text_faint);
    y += facts.size().y + 3.0;
    if let Some(warning) = warning {
        ui.painter()
            .galley(pos2(rect.center().x, y), warning, t.color.warn);
    }
}

fn preview_facts_galley(
    ui: &Ui,
    rect: Rect,
    record: &SchematicSheetFormat,
    target: DrawingSheetDefaultsTarget,
    following_sheets: Option<(usize, usize)>,
) -> std::sync::Arc<egui::Galley>{
    let t = Tokens::get(ui.ctx());
    let faint = TextFormat {
        font_id: theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        color: t.color.text_faint,
        ..Default::default()
    };
    let strong = TextFormat {
        font_id: theme::sans(tokens::FS_MICRO, FontWeight::Medium),
        color: t.color.text_dim,
        ..Default::default()
    };
    let mut job = LayoutJob {
        halign: Align::Center,
        ..Default::default()
    };
    job.wrap.max_width = rect.width() - 26.0;
    match record.geometry() {
        Ok(geometry) => {
            job.append("drawing area ", 0.0, faint.clone());
            job.append(
                &record
                    .display_unit
                    .format_size_um(geometry.drawing_area.width_um, geometry.drawing_area.height_um),
                0.0,
                strong.clone(),
            );
            job.append(" \u{00b7} zones ", 0.0, faint.clone());
            job.append(
                &geometry.zones.map_or_else(
                    || "none".to_owned(),
                    |zones| format!("{} \u{00d7} {}", zones.columns, zones.rows),
                ),
                0.0,
                strong.clone(),
            );
            if target == DrawingSheetDefaultsTarget::Project
                && let Some((following, total)) = following_sheets
            {
                job.append(" \u{00b7} followed by ", 0.0, faint);
                job.append(&format!("{following} of {total} sheets"), 0.0, strong);
            }
        }
        Err(_) => job.append("Invalid physical sheet", 0.0, faint),
    }
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

// ── Row 3: the frame row ────────────────────────────────────────────────────

fn frame_row_height(t: &Tokens, stacked: bool) -> f32 {
    let block = 13.0 + 5.0 + t.metrics.ctl_h;
    if stacked {
        7.0 + 5.0 * (block + 8.0) + 9.0
    } else {
        7.0 + block + 9.0
    }
}

fn frame_row(
    ui: &mut Ui,
    rect: Rect,
    record: &SchematicSheetFormat,
    authority: &SchematicSheetFormat,
    enabled: bool,
    stacked: bool,
    edits: &mut Vec<RecordEdit>,
) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    let inner = Rect::from_min_max(
        pos2(rect.left() + 11.0, rect.top() + 7.0),
        pos2(rect.right() - 11.0, rect.bottom() - 9.0),
    );

    const ORIENTATION_W: f32 = 216.0;
    const RESET_W: f32 = 118.0;
    const GAP: f32 = 14.0;

    ui.scope_builder(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
        |ui| {
            if stacked {
                ui.spacing_mut().item_spacing.y = 8.0;
                let width = ui.available_width();
                orientation_frame_block(ui, width, record, enabled, edits);
                unit_frame_block(ui, width, record, enabled, edits);
                border_frame_block(ui, width, record, authority, enabled, edits);
                title_frame_block(ui, width, record, authority, enabled, edits);
                reset_frame_block(ui, RESET_W, enabled, edits);
            } else {
                let select_w =
                    ((inner.width() - ORIENTATION_W - RESET_W - 4.0 * GAP) / 3.0).max(120.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = GAP;
                    orientation_frame_block(ui, ORIENTATION_W, record, enabled, edits);
                    unit_frame_block(ui, select_w, record, enabled, edits);
                    border_frame_block(ui, select_w, record, authority, enabled, edits);
                    title_frame_block(ui, select_w, record, authority, enabled, edits);
                    reset_frame_block(ui, RESET_W, enabled, edits);
                });
            }
        },
    );
}

fn frame_block(ui: &mut Ui, width: f32, label: &str, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(
        vec2(width, 13.0 + 5.0 + t.metrics.ctl_h),
        Layout::top_down(Align::Min),
        |ui| {
            ui.spacing_mut().item_spacing.y = 5.0;
            ui.label(
                RichText::new(label)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            content(ui);
        },
    );
}

fn orientation_frame_block(
    ui: &mut Ui,
    width: f32,
    record: &SchematicSheetFormat,
    enabled: bool,
    edits: &mut Vec<RecordEdit>,
) {
    frame_block(ui, width, "Orientation", |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let half = (ui.available_width() - 4.0) / 2.0;
                for (value, label) in [
                    (SchematicPageOrientation::Landscape, "Landscape"),
                    (SchematicPageOrientation::Portrait, "Portrait"),
                ] {
                    if orientation_segment(ui, record.orientation == value, value, label, half) {
                        edits.push(RecordEdit::Orientation(value));
                    }
                }
            });
        });
    });
}

fn unit_frame_block(
    ui: &mut Ui,
    width: f32,
    record: &SchematicSheetFormat,
    enabled: bool,
    edits: &mut Vec<RecordEdit>,
) {
    frame_block(ui, width, "Display units", |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            let mut selected = record.display_unit;
            enum_combo(
                ui,
                "sheet-defaults-unit",
                &mut selected,
                &[
                    (DrawingSheetDisplayUnit::Millimetres, "mm"),
                    (DrawingSheetDisplayUnit::Centimetres, "cm"),
                    (DrawingSheetDisplayUnit::Inches, "in"),
                ],
            );
            if selected != record.display_unit {
                edits.push(RecordEdit::Unit(selected));
            }
        });
    });
}

fn border_frame_block(
    ui: &mut Ui,
    width: f32,
    record: &SchematicSheetFormat,
    authority: &SchematicSheetFormat,
    enabled: bool,
    edits: &mut Vec<RecordEdit>,
) {
    let border_locked = authority.border == DrawingSheetBorderTemplate::OrganizationManaged;
    frame_block(ui, width, "Border", |ui| {
        ui.add_enabled_ui(enabled && !border_locked, |ui| {
            let mut selected = record.border;
            if border_locked {
                enum_combo(
                    ui,
                    "sheet-defaults-border",
                    &mut selected,
                    &[(
                        DrawingSheetBorderTemplate::OrganizationManaged,
                        "Organization border \u{00b7} managed",
                    )],
                );
            } else {
                enum_combo(
                    ui,
                    "sheet-defaults-border",
                    &mut selected,
                    &[
                        (
                            DrawingSheetBorderTemplate::Standard,
                            "Standard border with zones",
                        ),
                        (DrawingSheetBorderTemplate::Plain, "Plain border"),
                        (DrawingSheetBorderTemplate::None, "No border"),
                    ],
                );
            }
            if selected != record.border {
                edits.push(RecordEdit::Border(selected));
            }
        });
    });
}

fn title_frame_block(
    ui: &mut Ui,
    width: f32,
    record: &SchematicSheetFormat,
    authority: &SchematicSheetFormat,
    enabled: bool,
    edits: &mut Vec<RecordEdit>,
) {
    let title_locked =
        authority.title_block.template == DrawingSheetTitleBlockTemplate::OrganizationManaged;
    frame_block(ui, width, "Title block", |ui| {
        ui.add_enabled_ui(enabled && !title_locked, |ui| {
            let mut selected = record.title_block.template;
            if title_locked {
                enum_combo(
                    ui,
                    "sheet-defaults-title-block",
                    &mut selected,
                    &[(
                        DrawingSheetTitleBlockTemplate::OrganizationManaged,
                        "Organization block \u{00b7} managed",
                    )],
                );
            } else {
                enum_combo(
                    ui,
                    "sheet-defaults-title-block",
                    &mut selected,
                    &[
                        (DrawingSheetTitleBlockTemplate::Compact, "RSpice compact"),
                        (DrawingSheetTitleBlockTemplate::Standard, "RSpice standard"),
                        (DrawingSheetTitleBlockTemplate::Wide, "RSpice wide"),
                        (DrawingSheetTitleBlockTemplate::None, "No title block"),
                    ],
                );
            }
            if selected != record.title_block.template {
                edits.push(RecordEdit::TitleBlock(selected));
            }
        });
    });
}

fn reset_frame_block(ui: &mut Ui, width: f32, enabled: bool, edits: &mut Vec<RecordEdit>) {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(
        vec2(width, 13.0 + 5.0 + t.metrics.ctl_h),
        Layout::bottom_up(Align::Min),
        |ui| {
            if Button::new("Reset to built-in")
                .ghost()
                .enabled(enabled)
                .show(ui)
                .on_hover_text(
                    "Restore the built-in default: A4 landscape, standard border with zones, RSpice compact title block.",
                )
                .clicked()
            {
                edits.push(RecordEdit::Reset);
            }
        },
    );
}

fn orientation_segment(
    ui: &mut Ui,
    selected: bool,
    value: SchematicPageOrientation,
    label: &str,
    width: f32,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::click());
    let painter = ui.painter();
    let fill = if selected {
        t.color.accent.gamma_multiply(0.11)
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_panel_2
    };
    painter.rect_filled(rect, 3.0, fill);
    painter.rect_stroke(
        rect,
        3.0,
        Stroke::new(
            1.0,
            if selected {
                t.color.accent.gamma_multiply(0.43)
            } else {
                t.color.border
            },
        ),
        StrokeKind::Inside,
    );
    let glyph_size = match value {
        SchematicPageOrientation::Landscape => vec2(14.0, 9.0),
        SchematicPageOrientation::Portrait => vec2(9.0, 14.0),
    };
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let label_w = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(label.to_owned(), label_font.clone(), Color32::WHITE)
            .size()
            .x
    });
    let content_w = glyph_size.x + 8.0 + label_w;
    let start = rect.center().x - content_w / 2.0;
    let glyph = Rect::from_center_size(
        pos2(start + glyph_size.x / 2.0, rect.center().y),
        glyph_size,
    );
    painter.rect_stroke(
        glyph,
        0.5,
        Stroke::new(
            1.0,
            if selected {
                t.color.accent
            } else {
                t.color.text_faint
            },
        ),
        StrokeKind::Inside,
    );
    painter.text(
        pos2(start + glyph_size.x + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        label_font,
        if selected { t.color.text } else { t.color.text_dim },
    );
    if response.hovered() && ui.is_enabled() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::RadioButton,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    response.clicked() && ui.is_enabled() && !selected
}

// ── Shared labels ───────────────────────────────────────────────────────────

fn standard_short_label(standard: DrawingSheetStandard) -> String {
    let label = standard.label();
    match standard.series() {
        DrawingSheetStandardSeries::Iso => label.trim_start_matches("ISO ").to_owned(),
        DrawingSheetStandardSeries::Ansi => label.trim_start_matches("ANSI ").to_owned(),
        DrawingSheetStandardSeries::Jis => label.trim_start_matches("JIS ").to_owned(),
        DrawingSheetStandardSeries::Architectural => label.to_owned(),
    }
}

fn oriented_standard_size(standard: DrawingSheetStandard, record: &SchematicSheetFormat) -> String {
    let (short, long) = standard.portrait_dimensions_um();
    oriented_size_text(short, long, record)
}

fn oriented_preset_size(preset: &DrawingSheetPreset, record: &SchematicSheetFormat) -> String {
    match &preset.format.authored_size {
        AuthoredDrawingSheetSize::Custom { snapshot } => {
            oriented_size_text(snapshot.portrait_width_um, snapshot.portrait_height_um, record)
        }
        AuthoredDrawingSheetSize::Standard { standard } => oriented_standard_size(*standard, record),
    }
}

fn oriented_size_text(portrait_w: u64, portrait_h: u64, record: &SchematicSheetFormat) -> String {
    let (width, height) = match record.orientation {
        SchematicPageOrientation::Landscape => (portrait_h, portrait_w),
        SchematicPageOrientation::Portrait => (portrait_w, portrait_h),
    };
    record.display_unit.format_size_um(width, height)
}

fn orientation_word(orientation: SchematicPageOrientation) -> &'static str {
    match orientation {
        SchematicPageOrientation::Landscape => "landscape",
        SchematicPageOrientation::Portrait => "portrait",
    }
}

fn border_template_label(border: DrawingSheetBorderTemplate) -> &'static str {
    match border {
        DrawingSheetBorderTemplate::Standard => "Standard border with zones",
        DrawingSheetBorderTemplate::Plain => "Plain border",
        DrawingSheetBorderTemplate::None => "No border",
        DrawingSheetBorderTemplate::OrganizationManaged => "Organization border",
    }
}

fn title_block_template_label(template: DrawingSheetTitleBlockTemplate) -> &'static str {
    match template {
        DrawingSheetTitleBlockTemplate::Compact => "RSpice compact",
        DrawingSheetTitleBlockTemplate::Standard => "RSpice standard",
        DrawingSheetTitleBlockTemplate::Wide => "RSpice wide",
        DrawingSheetTitleBlockTemplate::None => "No title block",
        DrawingSheetTitleBlockTemplate::OrganizationManaged => "Organization block",
    }
}

fn default_record_summary(format: &SchematicSheetFormat) -> String {
    let (width, height) = format.oriented_dimensions_um();
    format!(
        "{} \u{00b7} {} \u{00b7} {}",
        format.authored_size.label(),
        orientation_word(format.orientation),
        format.display_unit.format_size_um(width, height)
    )
}

fn new_sheet_policy_label(value: DrawingSheetNewSheetPolicy) -> &'static str {
    match value {
        DrawingSheetNewSheetPolicy::ProjectDefault => "Follow the project default",
        DrawingSheetNewSheetPolicy::Ask => "Ask in the New Sheet dialog",
        DrawingSheetNewSheetPolicy::MatchCurrent => "Copy the current sheet's format",
    }
}

fn new_sheet_policy_note(value: DrawingSheetNewSheetPolicy) -> &'static str {
    match value {
        DrawingSheetNewSheetPolicy::ProjectDefault => {
            "New sheets take the project default silently. Page Setup can still write an explicit override afterwards."
        }
        DrawingSheetNewSheetPolicy::Ask => {
            "Sheet creation opens on a format step with the project default preselected."
        }
        DrawingSheetNewSheetPolicy::MatchCurrent => {
            "A new sheet copies the open sheet's effective format as an explicit override and stops following the default."
        }
    }
}

/// Move a reusable default to another base format without trusting placement
/// offsets that were authored for a differently sized sheet.
///
/// Keep the requested presentation whenever it still fits. If only the
/// retained offsets make the new base invalid, normalize those offsets to the
/// template anchor. The final fallback is the base's own valid title block;
/// format selection must never panic or leave a corrupt draft.
fn retarget_default_to_standard(
    source: &SchematicSheetFormat,
    standard: DrawingSheetStandard,
) -> SchematicSheetFormat {
    retarget_default(
        source,
        SchematicSheetFormat::from_standard(standard, source.orientation),
    )
}

fn retarget_default(
    source: &SchematicSheetFormat,
    base: SchematicSheetFormat,
) -> SchematicSheetFormat {
    let rebuild = |title_block: DrawingSheetTitleBlock| {
        base.try_update(|draft| {
            draft.orientation = source.orientation;
            draft.display_unit = source.display_unit;
            draft.apply_border_template(source.border);
            draft.title_block = title_block;
            draft.inheritance = source.inheritance;
        })
    };

    if let Ok(candidate) = rebuild(source.title_block.clone()) {
        return candidate;
    }

    let mut anchored = source.title_block.clone();
    anchored.offset_x_um = 0;
    anchored.offset_y_um = 0;
    if let Ok(candidate) = rebuild(anchored) {
        return candidate;
    }

    rebuild(base.title_block.clone()).unwrap_or(base)
}

fn enum_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    value: &mut T,
    options: &[(T, &str)],
) {
    let selected = options
        .iter()
        .find_map(|(candidate, label)| (*candidate == *value).then_some(*label))
        .unwrap_or("Unavailable");
    ComboBox::from_id_salt(id)
        .width(ui.available_width())
        .selected_text(selected)
        .show_ui(ui, |ui| {
            for (candidate, label) in options {
                ui.selectable_value(value, *candidate, *label);
            }
        });
}

fn validate_project_authority(
    app: &AppState,
    state: &DrawingSheetDefaultsDialogState,
) -> Result<(), String> {
    state
        .edit
        .as_ref()
        .ok_or_else(|| "Drawing-sheet Defaults has no project edit authority.".to_owned())?
        .validate(app, "Drawing-sheet Defaults")?;
    if app.workspace.design_management.revision() != state.catalog_revision {
        return Err(
            "Project drawing-sheet settings changed. Close and reopen Defaults.".to_owned(),
        );
    }
    Ok(())
}

fn apply_drawing_sheet_defaults(app: &mut RSpiceApp) -> Result<String, String> {
    let mut transaction = app.state.dialogs.drawing_sheet_defaults.clone();
    enforce_managed_default_authority(
        &mut transaction.draft_project.default_format,
        &transaction.baseline_project.default_format,
    );
    enforce_managed_default_authority(
        &mut transaction.draft_personal.default_format,
        &transaction.baseline_personal.default_format,
    );
    transaction.draft_project.default_format = transaction
        .draft_project
        .default_format
        .as_drawing_sheet_default();
    transaction.draft_project.last_explicit_format = transaction
        .draft_project
        .last_explicit_format
        .as_ref()
        .map(SchematicSheetFormat::as_drawing_sheet_default);
    transaction.draft_personal.default_format = transaction
        .draft_personal
        .default_format
        .as_drawing_sheet_default();
    if !transaction.draft_project.remember_last_explicit_format {
        transaction.draft_project.last_explicit_format = None;
    }
    transaction
        .draft_project
        .validate()
        .map_err(|error| error.to_string())?;
    transaction.draft_personal.validate()?;
    if app
        .state
        .ui
        .preferences
        .drawing_sheet_personal_preferences()
        != transaction.baseline_personal
    {
        return Err(
            "Personal drawing-sheet preferences changed. Close and reopen Defaults.".to_owned(),
        );
    }
    let project_changed = transaction.draft_project != transaction.baseline_project;
    let personal_changed = transaction.draft_personal != transaction.baseline_personal;
    if !project_changed && !personal_changed {
        return Ok("Drawing-sheet defaults already matched the saved values.".to_owned());
    }

    let mut personal_candidate = app.state.ui.preferences.clone();
    personal_candidate
        .set_drawing_sheet_personal_preferences(transaction.draft_personal.clone())?;

    if project_changed {
        validate_project_authority(&app.state, &transaction)?;
        let before = app.state.workspace.design_management.clone();
        let mut candidate = before.clone();
        candidate
            .update_drawing_sheet_settings(candidate.revision(), transaction.draft_project.clone())
            .map_err(|error| error.to_string())?;
        commit_project_candidate(app, "Drawing-sheet defaults", before, candidate)?;
    }
    app.state.ui.preferences = personal_candidate;

    Ok(match (project_changed, personal_changed) {
        (true, true) => {
            "Project and personal drawing-sheet defaults saved through their owning authorities."
                .to_owned()
        }
        (true, false) => "Project drawing-sheet defaults saved.".to_owned(),
        (false, true) => "Personal drawing-sheet defaults saved in Preferences.".to_owned(),
        (false, false) => unreachable!(),
    })
}

fn enforce_managed_default_authority(
    candidate: &mut SchematicSheetFormat,
    authority: &SchematicSheetFormat,
) {
    let updated = candidate.clone().try_update(|draft| {
        if authority.border == DrawingSheetBorderTemplate::OrganizationManaged {
            draft.border = authority.border;
            draft.zones.mode = authority.zones.mode;
            draft.zones.custom_columns = authority.zones.custom_columns;
            draft.zones.custom_rows = authority.zones.custom_rows;
        }
        if authority.title_block.template == DrawingSheetTitleBlockTemplate::OrganizationManaged {
            draft.title_block.template = authority.title_block.template;
        }
    });
    if let Ok(updated) = updated {
        *candidate = updated;
    }
}

pub(super) fn commit_project_candidate(
    app: &mut RSpiceApp,
    description: &str,
    before: crate::state::DesignManagementCatalog,
    candidate: crate::state::DesignManagementCatalog,
) -> Result<(), String> {
    candidate.validate().map_err(|error| error.to_string())?;
    let schematic_tx = app
        .state
        .prepare_design_management_schematic_transaction(&candidate)?;
    let owner = app.state.workspace.active_schematic_reference();
    let committed_revision = app
        .state
        .workspace
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    app.state
        .apply_design_management_schematic_transaction(&schematic_tx);
    let after = app.state.workspace.design_management.clone();
    app.state
        .record_design_management_transaction(DesignManagementHistoryEntry {
            description: description.to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            committed_revision,
        });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_defaults_are_a_real_no_op() {
        let mut app = RSpiceApp::test_instance();
        open_drawing_sheet_defaults(&mut app.state);
        let revision = app.state.workspace.design_management.revision();

        assert_eq!(
            apply_drawing_sheet_defaults(&mut app).unwrap(),
            "Drawing-sheet defaults already matched the saved values."
        );
        assert_eq!(app.state.workspace.design_management.revision(), revision);
    }

    #[test]
    fn project_default_is_one_history_transaction_and_personal_uses_preferences() {
        let mut app = RSpiceApp::test_instance();
        open_drawing_sheet_defaults(&mut app.state);
        app.state
            .dialogs
            .drawing_sheet_defaults
            .draft_project
            .default_format = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::AnsiA,
            SchematicPageOrientation::Landscape,
        )
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
        })
        .unwrap();
        app.state
            .dialogs
            .drawing_sheet_defaults
            .draft_personal
            .default_format = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::JisB4,
            SchematicPageOrientation::Portrait,
        )
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::UserDefault;
        })
        .unwrap();

        let before = app.state.workspace.design_management.revision();
        apply_drawing_sheet_defaults(&mut app).unwrap();

        assert_eq!(app.state.workspace.design_management.revision(), before + 1);
        assert!(app.state.can_undo_project_design());
        assert_eq!(
            app.state
                .ui
                .preferences
                .drawing_sheet_personal_preferences()
                .default_format
                .authored_size
                .label(),
            "JIS B4"
        );
    }

    #[test]
    fn managed_default_properties_are_reapplied_at_commit_without_locking_orientation() {
        let mut app = RSpiceApp::test_instance();
        let mut settings = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .clone();
        settings.default_format = settings
            .default_format
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::OrganizationManaged);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            })
            .unwrap();
        let revision = app.state.workspace.design_management.revision();
        app.state
            .workspace
            .design_management
            .update_drawing_sheet_settings(revision, settings)
            .unwrap();
        open_drawing_sheet_defaults(&mut app.state);
        let draft = &mut app
            .state
            .dialogs
            .drawing_sheet_defaults
            .draft_project
            .default_format;
        *draft = draft
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::None);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                draft.orientation = SchematicPageOrientation::Portrait;
            })
            .unwrap();

        apply_drawing_sheet_defaults(&mut app).unwrap();

        let saved = &app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .default_format;
        assert_eq!(
            saved.border,
            DrawingSheetBorderTemplate::OrganizationManaged
        );
        assert_eq!(
            saved.title_block.template,
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        );
        assert_eq!(saved.orientation, SchematicPageOrientation::Portrait);
    }

    #[test]
    fn managed_default_locks_are_independent_per_property() {
        let border_authority = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::OrganizationManaged);
            })
            .unwrap();
        let mut border_candidate = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::None);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::Wide;
            })
            .unwrap();
        enforce_managed_default_authority(&mut border_candidate, &border_authority);
        assert_eq!(
            border_candidate.border,
            DrawingSheetBorderTemplate::OrganizationManaged
        );
        assert_eq!(
            border_candidate.title_block.template,
            DrawingSheetTitleBlockTemplate::Wide
        );

        let title_authority = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            })
            .unwrap();
        let mut title_candidate = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::Plain);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
            })
            .unwrap();
        enforce_managed_default_authority(&mut title_candidate, &title_authority);
        assert_eq!(title_candidate.border, DrawingSheetBorderTemplate::Plain);
        assert_eq!(
            title_candidate.title_block.template,
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        );
    }

    #[test]
    fn changing_default_standard_normalizes_an_offset_that_no_longer_fits() {
        let source = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::IsoA0,
            SchematicPageOrientation::Landscape,
        )
        .try_update(|draft| {
            draft.title_block.offset_x_um = -300_000;
        })
        .unwrap();

        let retargeted = retarget_default_to_standard(&source, DrawingSheetStandard::IsoA5);

        assert_eq!(
            retargeted.authored_size,
            AuthoredDrawingSheetSize::Standard {
                standard: DrawingSheetStandard::IsoA5,
            }
        );
        assert_eq!(retargeted.title_block.template, source.title_block.template);
        assert_eq!(retargeted.title_block.offset_x_um, 0);
        assert_eq!(retargeted.title_block.offset_y_um, 0);
        retargeted.validate().unwrap();
    }

    #[test]
    fn retargeting_to_a_preset_keeps_the_records_presentation() {
        let source = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::IsoA4,
            SchematicPageOrientation::Portrait,
        )
        .try_update(|draft| {
            draft.display_unit = DrawingSheetDisplayUnit::Inches;
            draft.apply_border_template(DrawingSheetBorderTemplate::Plain);
        })
        .unwrap();
        let base = SchematicSheetFormat::try_custom(
            "Lab panel",
            250_000,
            400_000,
            SchematicPageOrientation::Portrait,
        )
        .unwrap();

        let retargeted = retarget_default(&source, base);

        assert!(matches!(
            retargeted.authored_size,
            AuthoredDrawingSheetSize::Custom { .. }
        ));
        assert_eq!(retargeted.orientation, SchematicPageOrientation::Portrait);
        assert_eq!(retargeted.display_unit, DrawingSheetDisplayUnit::Inches);
        assert_eq!(retargeted.border, DrawingSheetBorderTemplate::Plain);
        retargeted.validate().unwrap();
    }

    #[test]
    fn chip_rows_pack_within_the_row_and_stretch_never_drops_a_chip() {
        let chips: Vec<ChipSpec> = (0..7)
            .map(|index| ChipSpec {
                strong: format!("chip {index}"),
                dims: String::new(),
                action: ChipAction::NewPreset,
                selected: false,
                natural_w: 120.0,
            })
            .collect();
        let rows = pack_chip_rows(&chips, 400.0);
        assert_eq!(rows.iter().map(Vec::len).sum::<usize>(), chips.len());
        assert!(rows.iter().all(|row| {
            let natural: f32 = row.len() as f32 * 120.0 + (row.len() - 1) as f32 * CHIP_GAP;
            natural <= 400.0
        }));
        // A chip wider than the whole row still lands on its own row.
        let wide = vec![ChipSpec {
            strong: "wide".to_owned(),
            dims: String::new(),
            action: ChipAction::NewPreset,
            selected: false,
            natural_w: 900.0,
        }];
        assert_eq!(pack_chip_rows(&wide, 400.0).len(), 1);
    }
}

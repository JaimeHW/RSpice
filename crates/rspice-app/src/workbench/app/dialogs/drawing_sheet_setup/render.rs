//! Mockup-authored three-column drawing-sheet setup studio.

mod preview;

use preview::*;

use egui::{
    Align, Align2, Color32, Context, Frame, Layout, Margin, Rect, ScrollArea, Sense, Stroke,
    TextEdit, Ui, Vec2, pos2, vec2,
};

use crate::state::{
    AuthoredDrawingSheetSize, DrawingSheetFrameConvention, DrawingSheetPresetScope,
    DrawingSheetStandard, DrawingSheetStandardSeries, SchematicPageOrientation,
    SchematicSheetFormat,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RSpiceApp;

use super::super::drawing_sheet_preview::DrawingSheetPreviewContent;
use super::commit::{
    apply_drawing_sheet_setup, resume_drawing_sheet_setup_after_support,
    validate_drawing_sheet_authority,
};
use super::state::{
    BorderTemplateChoice, DrawingSheetDraft, DrawingSheetSection, DrawingSheetSetupState,
    DrawingSheetSupportRequest, FieldProblem, PageSetupScope, SheetDisplayUnit, SheetSizeChoice,
    TitleBlockAnchorChoice, TitleBlockRotationChoice, TitleBlockTemplateChoice, ZoneModeChoice,
    display_unit_from_model, format_um,
};

const TITLE: &str = "Page setup";
const DESCRIPTION: &str = "Author the permanent physical drawing sheet independently from printer paper and export media.";
const PRIMARY: &str = "Apply sheet setup";
const NAV_WIDTH: f32 = 230.0;
const COLUMN_GAP: f32 = 0.0;
/// Gutter between the paired fields of a form row, and between an Apply-summary
/// label and its value. The body zeroes item spacing so the three studio
/// columns meet on their shared rules, so every gutter inside a panel has to be
/// asked for explicitly.
const FIELD_GUTTER: f32 = 14.0;
const FACT_ROW_GAP: f32 = 12.0;
const MARGIN_MAP_HEIGHT: f32 = 200.0;
const MARGIN_MAP_GUTTER: f32 = 12.0;
const MARGIN_MAP_CAPTION_HEIGHT: f32 = 26.0;
/// The inset the section panels carry. A figure that bleeds to the editor
/// column's edges has to paint back across it.
const PANEL_INSET_X: f32 = 18.0;
const PANEL_INSET_Y: f32 = 16.0;
const PHONE_FIELD_STACK_BREAKPOINT: f32 = 480.0;

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_setup_dialog(&mut self, ctx: &Context) {
        if self.state.dialogs.drawing_sheet_setup.support_suspended {
            let nested_open = self.state.dialogs.drawing_sheet_support.any_open()
                || self.state.dialogs.drawing_sheet_defaults.open
                || self.state.dialogs.drawing_sheet_presets.any_open();
            if nested_open {
                return;
            }
            if let Err(error) = resume_drawing_sheet_setup_after_support(&mut self.state) {
                self.state.dialogs.drawing_sheet_setup.support_suspended = false;
                self.state.dialogs.drawing_sheet_setup.open = true;
                self.state.dialogs.drawing_sheet_setup.authority_error = Some(error);
            }
        }
        if !self.state.dialogs.drawing_sheet_setup.open {
            return;
        }

        let authority_error = self
            .state
            .dialogs
            .drawing_sheet_setup
            .authority
            .as_ref()
            .ok_or_else(|| "Page Setup has no active sheet authority.".to_owned())
            .and_then(|authority| validate_drawing_sheet_authority(&self.state, authority))
            .err();
        self.state.dialogs.drawing_sheet_setup.authority_error = authority_error.clone();
        let problems = self
            .state
            .dialogs
            .drawing_sheet_setup
            .draft
            .validate()
            .err()
            .unwrap_or_default();
        let dirty = self.state.dialogs.drawing_sheet_setup.is_dirty();
        let read_only = authority_error.is_some();
        let bootstrap_required = self
            .state
            .dialogs
            .drawing_sheet_setup
            .authority
            .as_ref()
            .is_some_and(|authority| authority.governed.is_none());
        let primary_enabled =
            page_setup_primary_enabled(read_only, problems.is_empty(), dirty, bootstrap_required);
        let prospective_format = geometry_preview_format(
            &self.state.dialogs.drawing_sheet_setup.draft,
            &self.state.dialogs.drawing_sheet_setup.last_valid_format,
        );
        let prospective_overflow =
            prospective_overflow_count(&self.state, prospective_format.clone());
        let preview_content = DrawingSheetPreviewContent::from_state(&self.state);

        let mut dialog = Dialog::new(TITLE, if read_only { "Close" } else { PRIMARY })
            .description(DESCRIPTION)
            .size(DialogSize::SchematicPageSetup)
            .fixed_height(760.0)
            .primary_enabled(primary_enabled)
            .flush_body()
            .manual_body_scroll();
        // The surface carries no transaction banner, so the footer is the only
        // place a disabled primary can say what is holding it. From another
        // section, naming which one holds it is the whole of the answer.
        if let Some(reason) = blocked_reason(&problems) {
            dialog = dialog.hint(reason);
        }
        if !read_only {
            dialog = dialog.ghost(if dirty { "Discard changes" } else { "Cancel" });
        }
        let choice = dialog.show(ctx, |ui| {
            drawing_sheet_setup_body(
                ui,
                &mut self.state.dialogs.drawing_sheet_setup,
                &problems,
                prospective_overflow,
                &preview_content,
            );
        });

        if let Some(request) = self
            .state
            .dialogs
            .drawing_sheet_setup
            .pending_support
            .take()
        {
            let result = match request {
                DrawingSheetSupportRequest::SheetFormatManager => {
                    crate::workbench::app::open_sheet_format_manager(&mut self.state)
                }
                DrawingSheetSupportRequest::TitleBlockFields => {
                    crate::workbench::app::open_title_block_fields(&mut self.state)
                }
                DrawingSheetSupportRequest::Defaults => {
                    if crate::workbench::app::open_drawing_sheet_defaults(&mut self.state) {
                        Ok(())
                    } else {
                        Err("Drawing-sheet Defaults is already open.".to_owned())
                    }
                }
                DrawingSheetSupportRequest::CustomSizes => {
                    if crate::workbench::app::open_custom_sheet_size_library(&mut self.state) {
                        Ok(())
                    } else {
                        Err("Custom sheet sizes is already open.".to_owned())
                    }
                }
            };
            match result {
                Ok(()) => {
                    let setup = &mut self.state.dialogs.drawing_sheet_setup;
                    setup.open = false;
                    setup.support_suspended = true;
                }
                Err(error) => {
                    // Answered on the status bar: the surface has no banner,
                    // and a refused request must not fail silently.
                    self.state.prompt(error);
                }
            }
            return;
        }

        match choice {
            DialogChoice::Primary if read_only => {
                self.state.dialogs.drawing_sheet_setup.close();
            }
            DialogChoice::Primary => match apply_drawing_sheet_setup(&mut self.state) {
                Ok(message) | Err(message) => self.state.prompt(message),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_setup.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn drawing_sheet_setup_body(
    ui: &mut Ui,
    state: &mut DrawingSheetSetupState,
    problems: &[FieldProblem],
    prospective_overflow: Option<usize>,
    preview_content: &DrawingSheetPreviewContent,
) {
    if let Some(format) = geometry_preview_format(&state.draft, &state.last_valid_format) {
        state.last_valid_format = format;
    }
    ui.spacing_mut().item_spacing = vec2(COLUMN_GAP, 0.0);
    if let Some(reason) = state.authority_error.as_deref() {
        read_only_banner(ui, reason);
    }
    let available = ui.available_size();
    // No transaction banner. Every field problem is already reported under the
    // field it belongs to, and a commit failure is raised in the console, so a
    // band across the foot of the surface only repeated what the form said.
    let body_height = available.y.max(1.0);
    if available.x < 980.0 {
        stacked_body(
            ui,
            state,
            problems,
            prospective_overflow,
            preview_content,
            body_height,
        );
        return;
    }

    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_app)
        .stroke(Stroke::new(1.0, t.color.border))
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                vec2(available.x, body_height),
                Layout::left_to_right(Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing.x = COLUMN_GAP;
                    let nav = ui.allocate_ui_with_layout(
                        vec2(NAV_WIDTH, body_height),
                        Layout::top_down(Align::Min),
                        |ui| section_navigation(ui, state, problems, body_height),
                    );
                    ui.painter().vline(
                        nav.response.rect.right(),
                        nav.response.rect.y_range(),
                        Stroke::new(1.0, t.color.border),
                    );
                    let remaining = (available.x - NAV_WIDTH).max(720.0);
                    let preview_width = (remaining * (0.82 / 1.82))
                        .max(330.0)
                        .min((remaining - 390.0).max(330.0));
                    let editor_width = (remaining - preview_width).max(390.0);
                    let editor = ui.allocate_ui_with_layout(
                        vec2(editor_width, body_height),
                        Layout::top_down(Align::Min),
                        |ui| editor_column(ui, state, problems, body_height),
                    );
                    ui.painter().vline(
                        editor.response.rect.right(),
                        editor.response.rect.y_range(),
                        Stroke::new(1.0, t.color.border),
                    );
                    ui.allocate_ui_with_layout(
                        vec2(preview_width, body_height),
                        Layout::top_down(Align::Min),
                        |ui| {
                            preview_column(
                                ui,
                                state,
                                prospective_overflow,
                                preview_content,
                                body_height,
                            )
                        },
                    );
                },
            );
        });
}

fn read_only_banner(ui: &mut Ui, reason: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(14, 10))
        .show(ui, |ui| {
            let rect = ui.max_rect();
            ui.painter().vline(
                rect.left() + 1.5,
                rect.y_range(),
                Stroke::new(3.0, t.color.warn),
            );
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Drawing-sheet editing is unavailable.")
                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(reason)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
            });
        });
}

fn stacked_body(
    ui: &mut Ui,
    state: &mut DrawingSheetSetupState,
    problems: &[FieldProblem],
    prospective_overflow: Option<usize>,
    preview_content: &DrawingSheetPreviewContent,
    height: f32,
) {
    ScrollArea::vertical()
        .id_salt("drawing-sheet-setup-stacked")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            Frame::NONE
                .fill(Tokens::get(ui.ctx()).color.bg_panel)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for section in DrawingSheetSection::ALL {
                            // A chip carries its mark in its own text: it has
                            // no room beside it for a glyph, and the text is
                            // what a screen reader is given either way.
                            let label = format!(
                                "{}  {}",
                                section.number(),
                                section_entry_name(
                                    section,
                                    section_problem(problems, section).is_some()
                                )
                            );
                            if ui
                                .selectable_label(state.section == section, label)
                                .clicked()
                            {
                                state.section = section;
                            }
                        }
                    });
                });
            editor_column(ui, state, problems, height.min(560.0));
            preview_column(ui, state, prospective_overflow, preview_content, 460.0);
        });
}

fn section_navigation(
    ui: &mut Ui,
    state: &mut DrawingSheetSetupState,
    problems: &[FieldProblem],
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_width(NAV_WIDTH);
        ui.spacing_mut().item_spacing.y = 0.0;
        for section in DrawingSheetSection::ALL {
            let selected = state.section == section;
            let problem = section_problem(problems, section);
            let detail = section_detail(state, section);
            let response = Frame::NONE
                .fill(if selected {
                    t.color.bg_active
                } else {
                    t.color.bg_panel
                })
                .inner_margin(Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    let width = ui.available_width();
                    ui.allocate_ui_with_layout(
                        vec2(width, 34.0),
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.set_min_width(width);
                            let number_color = if selected {
                                t.color.accent
                            } else {
                                t.color.text_faint
                            };
                            ui.label(
                                egui::RichText::new(section.number())
                                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                                    .color(number_color),
                            );
                            paint_section_icon(ui, section, selected);
                            // The marker's track comes off the text before the
                            // text is laid out. Taken afterwards, a summary
                            // wide enough to fill the row would have pushed the
                            // marker off the rail.
                            let text_width = (ui.available_width()
                                - if problem.is_some() {
                                    PROBLEM_MARKER_TRACK
                                } else {
                                    0.0
                                })
                            .max(1.0);
                            ui.allocate_ui_with_layout(
                                vec2(text_width, ui.available_height()),
                                Layout::top_down(Align::Min),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(section.label())
                                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                            .color(t.color.text),
                                    );
                                    // Truncated, never wrapped: a two-line
                                    // summary would push the entry past the
                                    // height the rail divided its column by.
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(detail)
                                                .font(theme::sans(
                                                    tokens::FS_0,
                                                    FontWeight::Regular,
                                                ))
                                                .color(t.color.text_dim),
                                        )
                                        .truncate(),
                                    );
                                },
                            );
                        },
                    )
                });
            let rect = response.response.rect;
            if problem.is_some() {
                paint_problem_marker(ui, rect);
            }
            if selected {
                ui.painter().rect_filled(
                    Rect::from_min_max(rect.min, egui::pos2(rect.left() + 3.0, rect.bottom())),
                    0.0,
                    t.color.accent,
                );
            }
            let section_response = response.response.interact(Sense::click());
            let name = section_entry_name(section, problem.is_some());
            section_response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    name.as_str(),
                )
            });
            crate::ui::theme::paint_focus_ring(ui, &section_response, rect);
            let section_response =
                section_response.on_hover_cursor(egui::CursorIcon::PointingHand);
            let section_response = match problem {
                Some(message) => section_response.on_hover_text(message),
                None => section_response,
            };
            if section_response.clicked() {
                state.section = section;
            }
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                Stroke::new(1.0, t.color.border),
            );
        }
        // Dock the note on the rail's bottom edge by giving it the leftover
        // space bottom-up. Reserving a guessed height for it instead left a
        // ragged band of panel below the note, and any change of density,
        // font, or wording moved the guess again.
        let remaining = (height - ui.min_rect().height()).max(12.0);
        // The rows plus `remaining` consume the rail's exact assigned height.
        // Applying the full height again after this allocation expands the
        // dialog body, pushes its footer off-screen, and leaves a dead band
        // below the editor and preview columns.
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), remaining),
            Layout::bottom_up(Align::Min),
            |ui| {
                Frame::NONE
                    .fill(t.color.bg_inset)
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        // The frame inherits the bottom-up layout, so the
                        // note's two lines are added in reverse.
                        ui.set_min_width(ui.available_width());
                        ui.label(
                            egui::RichText::new(
                                "Print and Export may fit, tile or clip this sheet without changing it.",
                            )
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new("AUTHORED SHEET ≠ OUTPUT MEDIA")
                                .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                                .color(t.color.accent),
                        );
                });
            },
        );
    });
}

fn paint_section_icon(ui: &mut Ui, section: DrawingSheetSection, selected: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
    let color = if selected {
        t.color.accent
    } else {
        t.color.text_dim
    };
    let stroke = Stroke::new(1.0, color);
    let page = Rect::from_center_size(rect.center(), vec2(13.0, 15.0));
    match section {
        DrawingSheetSection::Format => {
            ui.painter()
                .rect_stroke(page, 0.5, stroke, egui::StrokeKind::Inside);
        }
        DrawingSheetSection::Margins => {
            ui.painter()
                .rect_stroke(page, 0.5, stroke, egui::StrokeKind::Inside);
            ui.painter().rect_stroke(
                page.shrink(3.0),
                0.0,
                Stroke::new(0.75, color),
                egui::StrokeKind::Inside,
            );
        }
        DrawingSheetSection::Frame => {
            ui.painter()
                .rect_stroke(page, 0.5, stroke, egui::StrokeKind::Inside);
            ui.painter()
                .vline(page.center().x, page.y_range(), Stroke::new(0.6, color));
            ui.painter()
                .hline(page.x_range(), page.center().y, Stroke::new(0.6, color));
        }
        DrawingSheetSection::TitleBlock => {
            ui.painter()
                .rect_stroke(page, 0.5, stroke, egui::StrokeKind::Inside);
            let title = Rect::from_min_max(page.center(), page.right_bottom());
            ui.painter().rect_stroke(
                title,
                0.0,
                Stroke::new(0.8, color),
                egui::StrokeKind::Inside,
            );
            ui.painter()
                .hline(title.x_range(), title.center().y, Stroke::new(0.5, color));
        }
        DrawingSheetSection::Scope => {
            let rear = page.translate(vec2(-2.0, -1.5));
            let front = page.translate(vec2(2.0, 1.5));
            ui.painter()
                .rect_stroke(rear, 0.5, Stroke::new(0.7, color), egui::StrokeKind::Inside);
            ui.painter()
                .rect_stroke(front, 0.5, stroke, egui::StrokeKind::Inside);
        }
    }
}

/// The section that owns a validated field.
///
/// The table is the panels' own division: a field is repaired where its section
/// draws it, so marking any other entry would send the operator to a form that
/// does not contain it. A field no section claims is left unattributed rather
/// than guessed at, and the guard below refuses one appearing silently.
fn field_section(field: &str) -> Option<DrawingSheetSection> {
    match field {
        "width" | "height" | "custom-name" => Some(DrawingSheetSection::Format),
        "bleed" => Some(DrawingSheetSection::Margins),
        "title-offset-x" | "title-offset-y" | "title-fields" | "scale" => {
            Some(DrawingSheetSection::TitleBlock)
        }
        _ => None,
    }
}

/// The first problem a section holds, which is what its rail entry reports.
fn section_problem(problems: &[FieldProblem], section: DrawingSheetSection) -> Option<&str> {
    problems
        .iter()
        .find(|problem| field_section(problem.field) == Some(section))
        .map(|problem| problem.message.as_str())
}

/// What a rail entry or a section chip is called once its section holds a field
/// the sheet contract refuses.
///
/// The suffix is the marker assistive technology reads: a painted glyph and a
/// colour are nothing to a screen reader, and a rail that only says which
/// section is wrong in red says it to some of the people using it.
fn section_entry_name(section: DrawingSheetSection, blocked: bool) -> String {
    if blocked {
        format!("{} · blocked", section.label())
    } else {
        section.label().to_owned()
    }
}

/// Why the primary is disabled, in the footer's measure: how much is unresolved
/// and where it lives. Sections are named in rail order so the phrase reads as
/// a route through the surface.
fn blocked_reason(problems: &[FieldProblem]) -> Option<String> {
    let sections = DrawingSheetSection::ALL
        .into_iter()
        .filter(|section| section_problem(problems, *section).is_some())
        .map(DrawingSheetSection::label)
        .collect::<Vec<_>>();
    if sections.is_empty() {
        return None;
    }
    Some(format!(
        "{} unresolved · {}",
        problems.len(),
        sections.join(", ")
    ))
}

/// The width a marked entry gives up from its summary: the glyph, and the
/// gutter that keeps it off the text.
const PROBLEM_MARKER_TRACK: f32 = 22.0;
/// The rail entry's own horizontal inset, which the mark shares so it lines up
/// with the column of text above and below it.
const PROBLEM_MARKER_INSET: f32 = 12.0;

/// A blocked section's mark: a filled warning triangle at the trailing edge of
/// `entry`. It is painted rather than set in text because the icon vocabulary
/// has no warning glyph and the shipped faces have no dependable one either,
/// and painted into the entry's own rectangle rather than allocated inside it
/// because a row that grows by a marker's height stops fitting its column.
fn paint_problem_marker(ui: &Ui, entry: Rect) {
    let t = Tokens::get(ui.ctx());
    let rect = Rect::from_center_size(
        pos2(entry.right() - PROBLEM_MARKER_INSET - 7.0, entry.center().y),
        vec2(14.0, 14.0),
    );
    let triangle = Rect::from_center_size(rect.center(), vec2(13.0, 11.5));
    let painter = ui.painter();
    painter.add(egui::Shape::convex_polygon(
        vec![
            pos2(triangle.center().x, triangle.top()),
            triangle.right_bottom(),
            triangle.left_bottom(),
        ],
        t.color.err,
        Stroke::NONE,
    ));
    // The exclamation is cut out of the fill in the panel's own colour, so the
    // mark reads at rail density instead of turning into a solid lozenge.
    painter.vline(
        triangle.center().x,
        egui::Rangef::new(triangle.top() + 4.0, triangle.bottom() - 4.0),
        Stroke::new(1.5, t.color.bg_panel),
    );
    painter.rect_filled(
        Rect::from_center_size(
            pos2(triangle.center().x, triangle.bottom() - 2.5),
            vec2(1.5, 1.5),
        ),
        0.0,
        t.color.bg_panel,
    );
}

fn section_detail(state: &DrawingSheetSetupState, section: DrawingSheetSection) -> String {
    match section {
        DrawingSheetSection::Format => {
            let orientation = state.draft.orientation.label();
            format!("{} · {orientation}", state.draft.size.label())
        }
        DrawingSheetSection::Margins => {
            let margins = state.draft.margins();
            let unit = state.draft.display_unit;
            let length = |value_um| format_um(value_um, unit);
            if state.draft.border == BorderTemplateChoice::None {
                "no margins".to_owned()
            } else if state.draft.binding_margin {
                format!("{} {} binding", length(margins.left_um), unit.suffix())
            } else if margins.left_um == margins.top_um {
                format!("{} {}", length(margins.top_um), unit.suffix())
            } else {
                format!(
                    "{} · {} {}",
                    length(margins.left_um),
                    length(margins.top_um),
                    unit.suffix()
                )
            }
        }
        DrawingSheetSection::Frame => match state.draft.zone_mode {
            ZoneModeChoice::None => "border only".to_owned(),
            _ => format!(
                "{} × {} zones",
                state.draft.zone_columns, state.draft.zone_rows
            ),
        },
        DrawingSheetSection::TitleBlock => state.draft.title_block.label().to_owned(),
        DrawingSheetSection::Scope => match state.draft.scope {
            PageSetupScope::CurrentSheet => "this sheet".to_owned(),
            PageSetupScope::Document => format!("{} sheets", state.sheet_count),
            PageSetupScope::CurrentSheetAndDefault => "sheet + project default".to_owned(),
        },
    }
}

fn editor_column(
    ui: &mut Ui,
    state: &mut DrawingSheetSetupState,
    problems: &[FieldProblem],
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_height(height);
        ui.allocate_ui(vec2(ui.available_width(), height.max(120.0)), |ui| {
            ScrollArea::vertical()
                .id_salt(("drawing-sheet-editor", state.section))
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    active_panel_header(ui, state.section);
                    ui.add_enabled_ui(state.authority_error.is_none(), |ui| {
                        Frame::NONE
                            .inner_margin(Margin::symmetric(
                                PANEL_INSET_X as i8,
                                PANEL_INSET_Y as i8,
                            ))
                            .show(ui, |ui| match state.section {
                                DrawingSheetSection::Format => format_panel(ui, state, problems),
                                DrawingSheetSection::Margins => margins_panel(ui, state, problems),
                                DrawingSheetSection::Frame => frame_panel(ui, state),
                                DrawingSheetSection::TitleBlock => title_panel(ui, state, problems),
                                DrawingSheetSection::Scope => scope_panel(ui, state),
                            });
                    });
                });
        });
    });
}

fn active_panel_header(ui: &mut Ui, section: DrawingSheetSection) {
    match section {
        DrawingSheetSection::Format => panel_header(
            ui,
            "PHYSICAL PAGE",
            "Size and orientation",
            "Define the authored drawing sheet. Output media is chosen later in Print or Export.",
        ),
        DrawingSheetSection::Margins => panel_header(
            ui,
            "PAPER INSETS",
            "Margins and bleed",
            "The sheet's standard sets its margins. Bleed belongs to export artwork and never enlarges the sheet.",
        ),
        DrawingSheetSection::Frame => panel_header(
            ui,
            "ENGINEERING FRAME",
            "Border, zones and marks",
            "The sheet's standard draws the frame. Choose whether it carries zones and marks.",
        ),
        DrawingSheetSection::TitleBlock => panel_header(
            ui,
            "SHEET IDENTITY",
            "Title block",
            "Select the printed identity form, its reading direction and location on the authored sheet.",
        ),
        DrawingSheetSection::Scope => panel_header(
            ui,
            "COMMIT BOUNDARY",
            "Scope and inheritance",
            "Choose exactly which saved drawing-sheet records Apply will change.",
        ),
    }
}

fn panel_header(ui: &mut Ui, eyebrow: &str, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    let response = Frame::NONE
        .fill(t.color.bg_elevated)
        .inner_margin(Margin::symmetric(18, 14))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(
                egui::RichText::new(eyebrow)
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.accent),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(title)
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(description)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn format_panel(ui: &mut Ui, state: &mut DrawingSheetSetupState, problems: &[FieldProblem]) {
    field_caption(ui, "Drawing-sheet format");
    let previous_size = state.draft.size.clone();
    egui::ComboBox::from_id_salt("drawing-sheet-format")
        .width(ui.available_width())
        .selected_text(selected_size_label(state))
        .show_ui(ui, |ui| {
            let mut previous_series = None;
            for standard in DrawingSheetStandard::ALL {
                let series = standard.series();
                if previous_series != Some(series) {
                    if previous_series.is_some() {
                        ui.separator();
                    }
                    ui.weak(standard_series_heading(series));
                    previous_series = Some(series);
                }
                let choice = SheetSizeChoice::Standard(standard);
                ui.selectable_value(
                    &mut state.draft.size,
                    choice,
                    format_choice_label(
                        standard.label(),
                        standard.portrait_dimensions_um(),
                        state.draft.orientation,
                        state.draft.display_unit,
                    ),
                );
            }
            ui.separator();
            ui.weak("Custom sizes");
            for preset in &state.available_presets {
                let available = !matches!(
                    &preset.format.authored_size,
                    crate::state::AuthoredDrawingSheetSize::Custom { snapshot }
                        if snapshot.source_preset_unavailable
                );
                let choice = SheetSizeChoice::CapturedPreset {
                    id: preset.id.clone(),
                    name: preset.name.clone(),
                    scope: preset.scope,
                };
                ui.add_enabled_ui(available, |ui| {
                    ui.selectable_value(
                        &mut state.draft.size,
                        choice,
                        format!(
                            "{} · {}{}",
                            format_choice_label(
                                &preset.name,
                                preset.format.portrait_dimensions_um(),
                                state.draft.orientation,
                                state.draft.display_unit,
                            ),
                            match preset.scope {
                                crate::state::DrawingSheetPresetScope::Project => "project",
                                crate::state::DrawingSheetPresetScope::User => "personal",
                                crate::state::DrawingSheetPresetScope::Organization =>
                                    "organization",
                            },
                            if available { "" } else { " · unavailable" }
                        ),
                    );
                });
            }
            ui.selectable_value(
                &mut state.draft.size,
                SheetSizeChoice::Custom,
                "One-off custom size…",
            );
        });
    if previous_size != state.draft.size {
        let selected = state.draft.size.clone();
        if let SheetSizeChoice::CapturedPreset { id, scope, .. } = &selected {
            if let Some(preset) = state
                .available_presets
                .iter()
                .find(|preset| preset.id.eq_ignore_ascii_case(id) && preset.scope == *scope)
            {
                let scope = state.draft.scope;
                let title_fields = state.draft.title_fields.clone();
                let managed_authority = state.baseline.clone();
                state.draft = DrawingSheetDraft::from_format(&preset.format);
                state.draft.scope = scope;
                state.draft.title_fields = title_fields;
                state.draft.enforce_managed_authority(&managed_authority);
                if let crate::state::AuthoredDrawingSheetSize::Custom { snapshot } =
                    &preset.format.authored_size
                {
                    state.draft.size = SheetSizeChoice::CapturedPreset {
                        id: preset.id.clone(),
                        name: snapshot.name.clone(),
                        scope: preset.scope,
                    };
                }
            }
        } else {
            state.draft.apply_size_choice(selected);
        }
    }
    ui.add_space(12.0);

    if state.draft.size == SheetSizeChoice::Custom {
        let t = Tokens::get(ui.ctx());
        Frame::NONE
            .fill(t.color.bg_active)
            .stroke(Stroke::new(1.0, t.color.accent))
            .corner_radius(t.radius)
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                let unit = state.draft.display_unit.suffix();
                two_column(ui, |left, right| {
                    length_field(
                        left,
                        "Width",
                        &mut state.draft.width,
                        "297",
                        field_error(problems, "width"),
                        unit,
                    );
                    length_field(
                        right,
                        "Height",
                        &mut state.draft.height,
                        "210",
                        field_error(problems, "height"),
                        unit,
                    );
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = FIELD_GUTTER;
                    if Button::new("Swap edges").show(ui).clicked() {
                        let next = match state.draft.orientation {
                            SchematicPageOrientation::Landscape => {
                                SchematicPageOrientation::Portrait
                            }
                            SchematicPageOrientation::Portrait => {
                                SchematicPageOrientation::Landscape
                            }
                        };
                        state.draft.set_orientation(next);
                    }
                    ui.checkbox(&mut state.draft.save_custom_preset, "Save to custom sizes");
                });
                if state.draft.save_custom_preset {
                    ui.add_space(8.0);
                    two_column(ui, |left, right| {
                        text_field(
                            left,
                            "Custom-size name",
                            &mut state.draft.custom_name,
                            "Lab panel",
                            field_error(problems, "custom-name"),
                        );
                        field_caption(right, "Owner");
                        egui::ComboBox::from_id_salt("drawing-sheet-custom-preset-owner")
                            .width(right.available_width())
                            .selected_text(match state.draft.custom_preset_scope {
                                DrawingSheetPresetScope::Project => "Project",
                                DrawingSheetPresetScope::User => "Personal",
                                DrawingSheetPresetScope::Organization => "Organization",
                            })
                            .show_ui(right, |ui| {
                                ui.selectable_value(
                                    &mut state.draft.custom_preset_scope,
                                    DrawingSheetPresetScope::Project,
                                    "Project",
                                );
                                ui.selectable_value(
                                    &mut state.draft.custom_preset_scope,
                                    DrawingSheetPresetScope::User,
                                    "Personal",
                                );
                            });
                    });
                    ui.weak(match state.draft.custom_preset_scope {
                        DrawingSheetPresetScope::Project => {
                            "Travels with this project and is available to collaborators."
                        }
                        DrawingSheetPresetScope::User => {
                            "Saved in personal preferences; using it saves a copy in the project."
                        }
                        DrawingSheetPresetScope::Organization => {
                            "Organization presets are managed and cannot be created here."
                        }
                    });
                }
            });
    }
    ui.add_space(14.0);

    if format_controls_stack(ui.available_width()) {
        orientation_field(ui, state);
        ui.add_space(11.0);
        display_units_field(ui, state);
    } else {
        two_column(ui, |left, right| {
            orientation_field(left, state);
            display_units_field(right, state);
        });
    }
    ui.add_space(15.0);
    measurement_row(ui, state, &state.last_valid_format);
}

fn format_controls_stack(available_width: f32) -> bool {
    available_width < PHONE_FIELD_STACK_BREAKPOINT
}

fn orientation_field(ui: &mut Ui, state: &mut DrawingSheetSetupState) {
    field_caption(ui, "Orientation");
    ui.horizontal(|ui| {
        let mut selected = state.draft.orientation;
        let segment_width = ((ui.available_width() - ui.spacing().item_spacing.x) * 0.5).max(72.0);
        orientation_choice(
            ui,
            &mut selected,
            SchematicPageOrientation::Landscape,
            "Landscape",
            segment_width,
        );
        orientation_choice(
            ui,
            &mut selected,
            SchematicPageOrientation::Portrait,
            "Portrait",
            segment_width,
        );
        if selected != state.draft.orientation {
            state.draft.set_orientation(selected);
        }
    });
    if let Some(advisory) = state.draft.orientation_advisory() {
        control_note(ui, advisory);
    }
}

fn display_units_field(ui: &mut Ui, state: &mut DrawingSheetSetupState) {
    field_caption(ui, "Display units");
    let previous = state.draft.display_unit;
    egui::ComboBox::from_id_salt("drawing-sheet-unit")
        .width(ui.available_width())
        .selected_text(state.draft.display_unit.label())
        .show_ui(ui, |ui| {
            for unit in SheetDisplayUnit::ALL {
                ui.selectable_value(&mut state.draft.display_unit, unit, unit.label());
            }
        });
    if previous != state.draft.display_unit {
        let selected = state.draft.display_unit;
        state.draft.display_unit = previous;
        // Refused only while a length field does not parse; that field is
        // already marked where it is, and the unit stays as it was.
        let _ = state.draft.set_display_unit(selected);
    }
}

fn margins_panel(ui: &mut Ui, state: &mut DrawingSheetSetupState, problems: &[FieldProblem]) {
    margin_map(ui, &state.draft);
    ui.add_space(15.0);
    let unit = state.draft.display_unit.suffix();
    let bordered = state.draft.border != BorderTemplateChoice::None;
    two_column(ui, |left, right| {
        length_field(
            left,
            "Export bleed",
            &mut state.draft.bleed,
            "0",
            field_error(problems, "bleed"),
            unit,
        );
        right.add_space(19.0);
        // ASME Y14.1 4.1.3 lets a bound drawing widen its margins.
        if bordered && state.draft.frame_convention() == DrawingSheetFrameConvention::AsmeY141 {
            right.checkbox(
                &mut state.draft.binding_margin,
                "Binding margin · 0.75 in left",
            );
        }
    });
    ui.add_space(14.0);
    muted(
        ui,
        match (bordered, state.draft.frame_convention()) {
            (false, _) => "A sheet with no border has no margins.",
            (true, DrawingSheetFrameConvention::Iso5457) => {
                "ISO 5457 sets the margins: 20 mm on the filing side and 10 mm elsewhere."
            }
            (true, DrawingSheetFrameConvention::AsmeY141) => {
                "ASME Y14.1 sets the margins: 0.50 in on every side."
            }
        },
    );
    muted(
        ui,
        "Stored to 1 µm; display-unit changes do not alter physical size.",
    );
}

fn frame_panel(ui: &mut Ui, state: &mut DrawingSheetSetupState) {
    field_caption(ui, "Border template");
    let previous_border = state.draft.border;
    egui::ComboBox::from_id_salt("drawing-sheet-border")
        .width(ui.available_width())
        .selected_text(state.draft.border.label())
        .show_ui(ui, |ui| {
            for choice in BorderTemplateChoice::ALL {
                ui.selectable_value(&mut state.draft.border, choice, choice.label());
            }
        });
    if previous_border != state.draft.border {
        state.draft.set_border_template(state.draft.border);
    }
    control_note(ui, state.draft.border.summary());
    ui.add_space(14.0);
    let zones_available = state.draft.border != BorderTemplateChoice::None;
    let (marks_title, marks_detail) = state.draft.centring_marks_wording();
    let marked = state.draft.standard_frame();
    two_column(ui, |left, right| {
        left.add_enabled_ui(zones_available, |left| {
            field_caption(left, "Zone divisions");
            egui::ComboBox::from_id_salt("drawing-sheet-zone-mode")
                .width(left.available_width())
                .selected_text(state.draft.zone_mode.label())
                .show_ui(left, |ui| {
                    for choice in ZoneModeChoice::ALL {
                        ui.selectable_value(&mut state.draft.zone_mode, choice, choice.label());
                    }
                });
            control_note(
                left,
                &match state.draft.zone_mode {
                    ZoneModeChoice::Automatic => format!(
                        "{} — {} × {} on this sheet{}.",
                        match state.draft.frame_convention() {
                            DrawingSheetFrameConvention::Iso5457 => {
                                "ISO 5457 50 mm fields from the centre"
                            }
                            DrawingSheetFrameConvention::AsmeY141 => {
                                "ASME Y14.1 Table 4-1 zones from the lower right"
                            }
                        },
                        state.draft.zone_columns,
                        state.draft.zone_rows,
                        if state.draft.size.standard() == Some(DrawingSheetStandard::IsoA4) {
                            ", lettered at the top and right"
                        } else {
                            ""
                        }
                    ),
                    ZoneModeChoice::None => {
                        "Reviews quote sheet coordinates instead of zone references.".to_owned()
                    }
                },
            );
        });
        right.add_space(19.0);
        right.add_enabled_ui(marked, |ui| {
            toggle_card(
                ui,
                marks_title,
                marks_detail,
                &mut state.draft.centring_marks,
            );
        });
    });
}

fn title_panel(ui: &mut Ui, state: &mut DrawingSheetSetupState, problems: &[FieldProblem]) {
    field_caption(ui, "Template");
    let previous_template = state.draft.title_block;
    ui.add_enabled_ui(
        state.draft.title_block != TitleBlockTemplateChoice::Organization,
        |ui| {
            egui::ComboBox::from_id_salt("drawing-sheet-title-template")
                .width(ui.available_width())
                .selected_text(state.draft.title_block.label())
                .show_ui(ui, |ui| {
                    // The compact block is offered only to a sheet already on
                    // it, beside the ISO 7200 form that replaces it.
                    let on_compact =
                        state.baseline.title_block == TitleBlockTemplateChoice::Compact;
                    for choice in TitleBlockTemplateChoice::ALL.into_iter().filter(|choice| {
                        *choice != TitleBlockTemplateChoice::Organization
                            && (*choice != TitleBlockTemplateChoice::Compact || on_compact)
                    }) {
                        ui.selectable_value(&mut state.draft.title_block, choice, choice.label());
                    }
                });
        },
    );
    if previous_template != state.draft.title_block {
        state
            .draft
            .set_title_block_template(state.draft.title_block);
    }
    control_note(ui, state.draft.title_block.summary());
    // The substitution warning belongs to the choice that triggers it. It used
    // to be raised in a banner across the foot of the surface, several hundred
    // pixels from the control it is about.
    if let Ok(validated) = state.draft.validate()
        && validated.page_format.title_block_substituted()
    {
        let t = Tokens::get(ui.ctx());
        let warning = match validated.page_format.effective_title_block_template() {
            crate::state::DrawingSheetTitleBlockTemplate::None => {
                "Does not fit this drawing area, so the sheet carries no title block. The \
                 requested choice stays saved."
                    .to_owned()
            }
            printed => format!(
                "Does not fit this drawing area — {} will be drawn. The requested choice stays saved.",
                printed.label(),
            ),
        };
        ui.add_space(6.0);
        ui.add(
            egui::Label::new(
                egui::RichText::new(warning)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.warn),
            )
            .wrap(),
        );
    }
    ui.add_space(14.0);
    let enabled = state.draft.title_block != TitleBlockTemplateChoice::None;
    ui.add_enabled_ui(enabled, |ui| {
        two_column(ui, |left, right| {
            field_caption(left, "Anchor");
            let previous_anchor = state.draft.title_block_anchor;
            egui::ComboBox::from_id_salt("drawing-sheet-title-anchor")
                .width(left.available_width())
                .selected_text(state.draft.title_block_anchor.label())
                .show_ui(left, |ui| {
                    for choice in TitleBlockAnchorChoice::ALL {
                        ui.selectable_value(
                            &mut state.draft.title_block_anchor,
                            choice,
                            choice.label(),
                        );
                    }
                });
            if previous_anchor != state.draft.title_block_anchor {
                state
                    .draft
                    .set_title_block_anchor(state.draft.title_block_anchor);
            }
            field_caption(right, "Reading direction");
            egui::ComboBox::from_id_salt("drawing-sheet-title-rotation")
                .width(right.available_width())
                .selected_text(state.draft.title_block_rotation.label())
                .show_ui(right, |ui| {
                    for choice in TitleBlockRotationChoice::ALL {
                        ui.selectable_value(
                            &mut state.draft.title_block_rotation,
                            choice,
                            choice.label(),
                        );
                    }
                });
            control_note(right, state.draft.title_block_rotation.note());
        });
        ui.add_space(11.0);
        let unit = state.draft.display_unit.suffix();
        two_column(ui, |left, right| {
            length_field(
                left,
                "Horizontal offset",
                &mut state.draft.title_block_offset_x,
                "0",
                field_error(problems, "title-offset-x"),
                unit,
            );
            length_field(
                right,
                "Vertical offset",
                &mut state.draft.title_block_offset_y,
                "0",
                field_error(problems, "title-offset-y"),
                unit,
            );
        });
    });
    ui.add_space(14.0);
    if resource_action(
        ui,
        ResourceGlyph::TitleBlock,
        "Edit title-block fields",
        "Values, visibility, provenance and managed ownership",
        state.authority.is_some(),
    ) {
        state.pending_support = Some(DrawingSheetSupportRequest::TitleBlockFields);
    }
    field_error_slot(ui, field_error(problems, "title-fields"));
}

fn scope_panel(ui: &mut Ui, state: &mut DrawingSheetSetupState) {
    let t = Tokens::get(ui.ctx());
    let comes_from = match state.draft.inheritance {
        crate::state::DrawingSheetInheritance::Explicit => "this sheet",
        crate::state::DrawingSheetInheritance::ProjectDefault => "project default",
        crate::state::DrawingSheetInheritance::UserDefault => "personal default",
    };
    let sheet_count = if state.draft.scope == PageSetupScope::Document {
        format!(
            "{} writable of {}",
            state.writable_sheet_count, state.sheet_count
        )
    } else {
        state.sheet_count.to_string()
    };
    let authored = format_summary(&state.baseline);
    measurement_strip(
        ui,
        &[
            ("Authored format", authored.as_str()),
            ("Comes from", comes_from),
            ("Sheets", sheet_count.as_str()),
        ],
    );
    ui.add_space(15.0);
    for scope in PageSetupScope::ALL {
        Frame::NONE
            .fill(if state.draft.scope == scope {
                Tokens::get(ui.ctx()).color.bg_active
            } else {
                Tokens::get(ui.ctx()).color.bg_inset
            })
            .stroke(Stroke::new(1.0, Tokens::get(ui.ctx()).color.border))
            .inner_margin(Margin::symmetric(10, 9))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.radio_value(&mut state.draft.scope, scope, "");
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(scope.label())
                                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold)),
                        );
                        muted(ui, scope.detail());
                    });
                });
            });
        ui.add_space(7.0);
    }
    ui.add_space(14.0);
    ui.painter().hline(
        ui.available_rect_before_wrap().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
    ui.add_space(10.0);
    ui.label(
        egui::RichText::new("DOCUMENT RESOURCES")
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(Tokens::get(ui.ctx()).color.text_dim),
    );
    ui.add_space(7.0);
    let governed = state
        .authority
        .as_ref()
        .is_some_and(|authority| authority.governed.is_some());
    if resource_action(
        ui,
        ResourceGlyph::Scope,
        &format!("Review all {} sheets", state.sheet_count),
        "Compare formats and apply to a selected set",
        governed,
    ) {
        state.pending_support = Some(DrawingSheetSupportRequest::SheetFormatManager);
    }
    if resource_action(
        ui,
        ResourceGlyph::Sliders,
        "Defaults for future sheets",
        "Project and personal starting formats",
        true,
    ) {
        state.pending_support = Some(DrawingSheetSupportRequest::Defaults);
    }
    if resource_action(
        ui,
        ResourceGlyph::Page,
        "Custom sheet sizes",
        "Project and personal named formats",
        true,
    ) {
        state.pending_support = Some(DrawingSheetSupportRequest::CustomSizes);
    }
    ui.add_space(2.0);
    ui.horizontal_wrapped(|ui| {
        if Button::new("Restore values on open").show(ui).clicked() {
            state.restore_opened_values();
        }
        if state.draft.inheritance != crate::state::DrawingSheetInheritance::ProjectDefault
            && Button::new("Follow project default").show(ui).clicked()
        {
            let scope = state.draft.scope;
            let title_fields = state.draft.title_fields.clone();
            let authority = state.baseline.clone();
            let mut replacement = DrawingSheetDraft::from_format(&state.project_default);
            replacement.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
            replacement.scope = scope;
            replacement.title_fields = title_fields;
            replacement.enforce_managed_authority(&authority);
            state.draft = replacement;
        }
    });
}

/// The reference's fused measurement readout: one bordered strip whose cells
/// meet on 1 px seams, each an uppercase micro label over a mono value.
fn measurement_strip(ui: &mut Ui, cells: &[(&str, &str)]) {
    let t = Tokens::get(ui.ctx());
    if cells.is_empty() {
        return;
    }
    let width = ui.available_width().max(1.0);
    let cell_h = 40.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, cell_h + 2.0), Sense::hover());
    ui.painter().rect_filled(rect, t.radius, t.color.border);
    let count = cells.len() as f32;
    let seams = count - 1.0;
    let cell_w = ((width - 2.0 - seams) / count).max(1.0);
    let value_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut x = rect.left() + 1.0;
    for (index, (label, value)) in cells.iter().enumerate() {
        let w = if index + 1 == cells.len() {
            (rect.right() - 1.0 - x).max(1.0)
        } else {
            cell_w
        };
        let cell = Rect::from_min_size(pos2(x, rect.top() + 1.0), vec2(w, cell_h));
        ui.painter().rect_filled(cell, 0.0, t.color.bg_panel_2);
        ui.painter().text(
            pos2(cell.left() + 10.0, cell.top() + 8.0),
            Align2::LEFT_TOP,
            label.to_uppercase(),
            theme::sans(tokens::FS_MICRO, FontWeight::Regular),
            t.color.text_faint,
        );
        let text = elide_to_width(value, w - 20.0, |text| {
            ui.painter()
                .layout_no_wrap(text.to_owned(), value_font.clone(), Color32::WHITE)
                .size()
                .x
        });
        if text != **value {
            ui.interact(
                cell,
                ui.id().with(("measurement-strip", index)),
                Sense::hover(),
            )
            .on_hover_text(*value);
        }
        ui.painter().text(
            pos2(cell.left() + 10.0, cell.top() + 22.0),
            Align2::LEFT_TOP,
            text,
            value_font.clone(),
            t.color.text,
        );
        x += w + 1.0;
    }
}

/// Shorten `value` until `width_of` reports it within `max_width`, marking the
/// cut with an ellipsis.
///
/// Every step removes one character from the *content* and rebuilds the
/// candidate from scratch. Shrinking a string that already carries the
/// ellipsis by popping its last character removes the ellipsis instead of any
/// content, so re-appending it reproduces the same string: that is a fixed
/// point, and a loop that reaches it never terminates.
fn elide_to_width(value: &str, max_width: f32, width_of: impl Fn(&str) -> f32) -> String {
    if width_of(value) <= max_width {
        return value.to_owned();
    }
    let mut kept = value.to_owned();
    while kept.pop().is_some() {
        let trimmed = kept.trim_end();
        if trimmed.is_empty() {
            break;
        }
        let candidate = format!("{trimmed}\u{2026}");
        if width_of(&candidate) <= max_width {
            return candidate;
        }
    }
    "\u{2026}".to_owned()
}

/// The authored sheet's identity for a measurement cell. Dimensions are the
/// Size & orientation section's readout and are deliberately left out here:
/// a third of the editor column cannot hold them without eliding the part
/// that names the format.
fn format_summary(draft: &DrawingSheetDraft) -> String {
    format!("{} · {}", draft.size.label(), draft.orientation.label())
}

fn selected_size_label(state: &DrawingSheetSetupState) -> String {
    match &state.draft.size {
        SheetSizeChoice::Standard(standard) => format_choice_label(
            standard.label(),
            standard.portrait_dimensions_um(),
            state.draft.orientation,
            state.draft.display_unit,
        ),
        SheetSizeChoice::CapturedPreset { id, name, scope } => state
            .available_presets
            .iter()
            .find(|preset| preset.id.eq_ignore_ascii_case(id) && preset.scope == *scope)
            .map_or_else(
                || name.clone(),
                |preset| {
                    format_choice_label(
                        name,
                        preset.format.portrait_dimensions_um(),
                        state.draft.orientation,
                        state.draft.display_unit,
                    )
                },
            ),
        SheetSizeChoice::Custom => "One-off custom size…".to_owned(),
    }
}

/// The Format menu's label for `standard` on the sheet `format` describes:
/// its designation and oriented size in that sheet's unit. A surface offering
/// the same choice names it exactly as this dialog does.
pub(crate) fn standard_size_choice_label(
    standard: DrawingSheetStandard,
    format: &SchematicSheetFormat,
) -> String {
    format_choice_label(
        standard.label(),
        standard.portrait_dimensions_um(),
        format.orientation,
        display_unit_from_model(format.display_unit),
    )
}

/// The Format menu's label for the size `format` already has, custom sizes
/// included.
pub(crate) fn current_size_choice_label(format: &SchematicSheetFormat) -> String {
    match &format.authored_size {
        AuthoredDrawingSheetSize::Standard { standard } => {
            standard_size_choice_label(*standard, format)
        }
        AuthoredDrawingSheetSize::Custom { snapshot } => format_choice_label(
            &snapshot.name,
            format.portrait_dimensions_um(),
            format.orientation,
            display_unit_from_model(format.display_unit),
        ),
    }
}

fn format_choice_label(
    name: &str,
    portrait: (u64, u64),
    orientation: SchematicPageOrientation,
    unit: SheetDisplayUnit,
) -> String {
    let (width, height) = match orientation {
        SchematicPageOrientation::Landscape => (portrait.1, portrait.0),
        SchematicPageOrientation::Portrait => portrait,
    };
    format!(
        "{name} · {} × {} {}",
        format_um(width, unit),
        format_um(height, unit),
        unit.suffix()
    )
}

const fn standard_series_heading(series: DrawingSheetStandardSeries) -> &'static str {
    match series {
        DrawingSheetStandardSeries::Iso => "ISO A series — international engineering",
        DrawingSheetStandardSeries::Ansi => "ANSI series — North American engineering",
        DrawingSheetStandardSeries::Architectural => {
            "Architectural series — North American large format, ASME Y14.1 format"
        }
        DrawingSheetStandardSeries::Jis => "JIS B series — Japanese engineering",
    }
}

#[derive(Clone, Copy)]
enum ResourceGlyph {
    Page,
    TitleBlock,
    Scope,
    Sliders,
}

fn resource_action(
    ui: &mut Ui,
    glyph: ResourceGlyph,
    title: &str,
    detail: &str,
    enabled: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let response = ui
        .add_enabled_ui(enabled, |ui| {
            Frame::NONE
                .fill(t.color.bg_inset)
                .stroke(Stroke::new(1.0, t.color.border))
                .inner_margin(Margin::symmetric(10, 9))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.horizontal(|ui| {
                        paint_resource_glyph(ui, glyph);
                        ui.vertical(|ui| {
                            ui.set_min_width((ui.available_width() - 20.0).max(80.0));
                            ui.label(
                                egui::RichText::new(title)
                                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                    .color(t.color.text),
                            );
                            muted(ui, detail);
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            paint_resource_chevron(ui);
                        });
                    });
                })
                .response
                .interact(Sense::click())
        })
        .inner;
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, title));
    crate::ui::theme::paint_focus_ring(ui, &response, response.rect);
    ui.add_space(6.0);
    response.clicked()
}

fn paint_resource_glyph(ui: &mut Ui, glyph: ResourceGlyph) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
    let stroke = Stroke::new(1.0, t.color.text_dim);
    match glyph {
        ResourceGlyph::Page | ResourceGlyph::TitleBlock | ResourceGlyph::Scope => {
            let page = rect.shrink2(vec2(3.0, 2.0));
            ui.painter()
                .rect_stroke(page, 0.5, stroke, egui::StrokeKind::Inside);
            match glyph {
                ResourceGlyph::TitleBlock => {
                    let block = Rect::from_min_max(
                        page.center(),
                        egui::pos2(page.right() - 1.5, page.bottom() - 1.5),
                    );
                    ui.painter()
                        .rect_stroke(block, 0.0, stroke, egui::StrokeKind::Inside);
                }
                ResourceGlyph::Scope => {
                    ui.painter().line_segment(
                        [
                            egui::pos2(page.left() + 3.0, page.center().y),
                            egui::pos2(page.right() - 3.0, page.center().y),
                        ],
                        stroke,
                    );
                }
                ResourceGlyph::Page | ResourceGlyph::Sliders => {}
            }
        }
        ResourceGlyph::Sliders => {
            for (index, fraction) in [0.7, 0.35, 0.58].into_iter().enumerate() {
                let y = rect.top() + 4.0 + index as f32 * 5.0;
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.left() + 2.0, y),
                        egui::pos2(rect.right() - 2.0, y),
                    ],
                    stroke,
                );
                ui.painter().circle_filled(
                    egui::pos2(rect.left() + rect.width() * fraction, y),
                    1.75,
                    t.color.text_dim,
                );
            }
        }
    }
}

fn paint_resource_chevron(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(12.0, 18.0), Sense::hover());
    let center = rect.center();
    ui.painter().line_segment(
        [center + vec2(-2.0, -3.5), center + vec2(1.5, 0.0)],
        Stroke::new(1.0, t.color.text_dim),
    );
    ui.painter().line_segment(
        [center + vec2(1.5, 0.0), center + vec2(-2.0, 3.5)],
        Stroke::new(1.0, t.color.text_dim),
    );
}

fn toggle_card(ui: &mut Ui, title: &str, detail: &str, checked: &mut bool) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(10, 9))
        .show(ui, |ui| {
            // Without this the card hugs its own label and the paired cards end
            // up two different widths, neither aligned with the fields above.
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.checkbox(checked, "");
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(title)
                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    muted(ui, detail);
                });
            });
        });
}

/// `Ui::columns` derives the gutter from the parent's item spacing before the
/// column rects exist, so the gutter has to be set here rather than on the
/// columns themselves.
fn two_column(ui: &mut Ui, content: impl FnOnce(&mut Ui, &mut Ui)) {
    let restore = ui.spacing().item_spacing.x;
    ui.spacing_mut().item_spacing.x = FIELD_GUTTER;
    ui.columns(2, |columns| {
        columns[0].spacing_mut().item_spacing.x = 12.0;
        columns[1].spacing_mut().item_spacing.x = 12.0;
        let (left, right) = columns.split_at_mut(1);
        content(&mut left[0], &mut right[0]);
    });
    ui.spacing_mut().item_spacing.x = restore;
}

fn orientation_choice(
    ui: &mut Ui,
    selected: &mut SchematicPageOrientation,
    value: SchematicPageOrientation,
    label: &str,
    width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        vec2(width, t.metrics.ctl_h),
        egui::Button::new(format!("     {label}")).selected(*selected == value),
    );
    let page_size = match value {
        SchematicPageOrientation::Landscape => vec2(14.0, 9.0),
        SchematicPageOrientation::Portrait => vec2(9.0, 14.0),
    };
    let page = Rect::from_center_size(
        egui::pos2(response.rect.left() + 12.0, response.rect.center().y),
        page_size,
    );
    ui.painter().rect_stroke(
        page,
        0.5,
        Stroke::new(
            1.0,
            if *selected == value {
                t.color.accent
            } else {
                t.color.text_dim
            },
        ),
        egui::StrokeKind::Inside,
    );
    if response.clicked() {
        *selected = value;
    }
}

fn field_caption(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(5.0);
}

fn text_field(ui: &mut Ui, label: &str, value: &mut String, hint: &str, error: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    field_caption(ui, label);
    let response = ui.add_sized(
        Vec2::new(ui.available_width(), t.metrics.ctl_h),
        TextEdit::singleline(value)
            .font(egui::TextStyle::Monospace)
            .hint_text(hint)
            .margin(Margin::symmetric(8, 4)),
    );
    if error.is_some() {
        ui.painter().rect_stroke(
            response.rect,
            t.radius,
            Stroke::new(1.0, t.color.err),
            egui::StrokeKind::Inside,
        );
    }
    field_error_slot(ui, error);
}

fn length_field(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    error: Option<&str>,
    unit: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    field_caption(ui, label);
    let width = ui.available_width();
    let suffix_width = 28.0;
    let response = ui
        .allocate_ui_with_layout(
            Vec2::new(width, t.metrics.ctl_h),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let response = ui.add_sized(
                    Vec2::new((width - suffix_width - 6.0).max(36.0), t.metrics.ctl_h),
                    TextEdit::singleline(value)
                        .font(egui::TextStyle::Monospace)
                        .hint_text(hint)
                        .margin(Margin::symmetric(8, 4)),
                );
                ui.allocate_ui_with_layout(
                    Vec2::new(suffix_width, t.metrics.ctl_h),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.monospace(unit);
                    },
                );
                response
            },
        )
        .inner;
    if error.is_some() {
        ui.painter().rect_stroke(
            response.rect,
            t.radius,
            Stroke::new(1.0, t.color.err),
            egui::StrokeKind::Inside,
        );
    }
    let changed = response.changed();
    field_error_slot(ui, error);
    changed
}

fn field_error_slot(ui: &mut Ui, error: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 15.0),
        Layout::top_down(Align::Min),
        |ui| {
            if let Some(error) = error {
                ui.label(
                    egui::RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.err),
                )
                .on_hover_text(error);
            }
        },
    );
}

fn field_error<'a>(problems: &'a [FieldProblem], field: &str) -> Option<&'a str> {
    problems
        .iter()
        .find(|problem| problem.field == field)
        .map(|problem| problem.message.as_str())
}

fn section_caption(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
}

fn fact_row_widths(width: f32) -> (f32, f32) {
    let label_width = (width * 0.36).clamp(64.0, 150.0);
    let value_width = (width - label_width - FACT_ROW_GAP).max(1.0);
    (label_width, value_width)
}

fn fact_row_height(ui: &Ui, label: &str, value: &str, width: f32) -> f32 {
    let t = Tokens::get(ui.ctx());
    let (label_width, value_width) = fact_row_widths(width);
    let label = ui.painter().layout(
        label.to_uppercase(),
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
        label_width,
    );
    let value = ui.painter().layout(
        value.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        value_width,
    );
    label.size().y.max(value.size().y).max(13.0)
}

fn effect_summary_height(ui: &Ui, width: f32, facts: &[(&str, &str)]) -> f32 {
    let caption_height = ui
        .painter()
        .layout_no_wrap(
            "WHAT APPLY DOES".to_owned(),
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            Color32::WHITE,
        )
        .size()
        .y;
    let inner_width = (width - 24.0).max(1.0);
    let rows = facts
        .iter()
        .map(|(label, value)| fact_row_height(ui, label, value, inner_width))
        .sum::<f32>();
    22.0 + caption_height + 8.0 + rows + 6.0 * facts.len().saturating_sub(1) as f32
}

/// One measured `label — value` line of the Apply summary.
///
/// Painting pre-wrapped galleys into an explicitly allocated row keeps the
/// two columns stable under every parent layout and gives the docked summary
/// an exact height before the preview consumes the remaining track.
fn fact_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (label_width, value_width) = fact_row_widths(width);
    let label_galley = ui.painter().layout(
        label.to_uppercase(),
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
        label_width,
    );
    let value_galley = ui.painter().layout(
        value.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        value_width,
    );
    let row_height = label_galley.size().y.max(value_galley.size().y).max(13.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, row_height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{label}: {value}"),
        )
    });
    if !ui.is_rect_visible(rect) {
        return;
    }
    let label_rect = Rect::from_min_size(rect.min, vec2(label_width, row_height));
    let value_rect = Rect::from_min_size(
        pos2(label_rect.right() + FACT_ROW_GAP, rect.top()),
        vec2(value_width, row_height),
    );
    ui.painter().with_clip_rect(label_rect).galley(
        label_rect.min,
        label_galley,
        t.color.text_faint,
    );
    ui.painter()
        .with_clip_rect(value_rect)
        .galley(value_rect.min, value_galley, t.color.text_dim);
}

/// A muted note that explains the control directly above it. The studio body
/// stacks with zero item spacing so the three columns meet on their shared
/// rules, which leaves a bare note sitting flush against its control.
fn control_note(ui: &mut Ui, text: &str) {
    ui.add_space(6.0);
    muted(ui, text);
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

const fn page_setup_primary_enabled(
    read_only: bool,
    valid: bool,
    dirty: bool,
    bootstrap_required: bool,
) -> bool {
    read_only || (valid && (dirty || bootstrap_required))
}

#[cfg(test)]
mod tests;

/// Where the rail's problem markers are measured. It is a module of its own
/// because these cases render the whole dialog, which the narrow policy tests
/// beside them deliberately do not.
#[cfg(test)]
mod blocked_sections {
    use super::*;

    /// Every control the dialog laid out this pass, at the rect it was laid out
    /// at, with the text assistive technology is given for it.
    fn dialog_controls(output: &egui::FullOutput) -> Vec<(String, Rect)> {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("the probe enables AccessKit, so every pass carries a tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.role() != egui::accesskit::Role::Dialog)
            .filter_map(|(_, node)| {
                let bounds = node.bounds()?;
                let rect = Rect::from_min_max(
                    pos2(bounds.x0 as f32, bounds.y0 as f32),
                    pos2(bounds.x1 as f32, bounds.y1 as f32),
                );
                let text = node.label().or_else(|| node.value()).unwrap_or_default();
                rect.is_finite().then(|| (text.to_owned(), rect))
            })
            .collect()
    }

    fn control_rect(controls: &[(String, Rect)], label: &str) -> Option<Rect> {
        controls
            .iter()
            .filter(|(found, _)| found == label)
            .map(|(_, rect)| *rect)
            .reduce(|held, rect| {
                if rect.height() > held.height() {
                    rect
                } else {
                    held
                }
            })
    }

    /// Every error-filled triangle the pass painted. The marker is a path, not
    /// a rectangle, and its colour is the only thing separating it from the
    /// section icons drawn beside it.
    fn marker_rects(output: &egui::FullOutput, err: Color32) -> Vec<Rect> {
        fn walk(shape: &egui::Shape, err: Color32, into: &mut Vec<Rect>) {
            match shape {
                egui::Shape::Path(path) if path.fill == err => {
                    into.push(path.visual_bounding_rect());
                }
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        walk(shape, err, into);
                    }
                }
                _ => {}
            }
        }
        let mut rects = Vec::new();
        for clipped in &output.shapes {
            walk(&clipped.shape, err, &mut rects);
        }
        rects
    }

    fn pass(ctx: &Context, app: &mut RSpiceApp, time: f64) -> egui::FullOutput {
        ctx.run_ui(
            egui::RawInput {
                time: Some(time),
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0))),
                ..Default::default()
            },
            |ui| app.render_drawing_sheet_setup_dialog(ui),
        )
    }

    /// The surface fades in when it opens, and a fade tints every shape it
    /// paints toward the backdrop. A probe that reads a colour has to let the
    /// animation finish or it measures the tint instead of the mark.
    fn settled(ctx: &Context, app: &mut RSpiceApp, start: f64) -> egui::FullOutput {
        let mut output = pass(ctx, app, start);
        for step in 1..=12 {
            output = pass(ctx, app, f64::from(step).mul_add(0.1, start));
        }
        output
    }

    /// The rail is the only thing on screen that can say which section holds a
    /// refused field while another section is open, and the footer is the only
    /// thing that can say why the primary will not move.
    #[test]
    fn a_refused_field_is_named_on_the_rail_and_in_the_footer() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let err = Tokens::get(&ctx).color.err;
        let mut app = RSpiceApp::test_instance();
        crate::workbench::app::open_drawing_sheet_setup(&mut app);
        app.state.dialogs.drawing_sheet_setup.section = DrawingSheetSection::Scope;

        let clean = settled(&ctx, &mut app, 0.0);
        let clean_controls = dialog_controls(&clean);
        let clean_row = control_rect(&clean_controls, DrawingSheetSection::Format.label())
            .expect("the rail names every section");
        assert!(
            marker_rects(&clean, err)
                .iter()
                .all(|marker| !clean_row.contains_rect(*marker)),
            "a valid draft marks nothing"
        );

        app.state.dialogs.drawing_sheet_setup.draft.width = "twelve".to_owned();
        let blocked = settled(&ctx, &mut app, 2.0);
        let controls = dialog_controls(&blocked);

        let marked = control_rect(&controls, "Size & orientation · blocked")
            .expect("the rail entry names the section holding the refused field");
        // The entry sets its title as text inside itself as well. What has to
        // carry the mark is the row a keyboard or screen reader lands on, which
        // is the one that contains the other.
        let title = control_rect(&controls, DrawingSheetSection::Format.label())
            .expect("the entry still draws its title");
        assert!(
            marked.contains_rect(title) && marked.height() > title.height(),
            "the mark is on a label inside the entry rather than on the entry"
        );
        for section in [
            DrawingSheetSection::Margins,
            DrawingSheetSection::Frame,
            DrawingSheetSection::TitleBlock,
        ] {
            assert!(
                control_rect(&controls, &format!("{} · blocked", section.label())).is_none(),
                "{section:?} holds nothing invalid and must not be marked"
            );
        }
        assert!(
            (marked.height() - clean_row.height()).abs() < 0.5,
            "the marker grew the rail row from {:.0} pt to {:.0} pt",
            clean_row.height(),
            marked.height()
        );
        let painted = marker_rects(&blocked, err);
        let markers: Vec<Rect> = painted
            .iter()
            .copied()
            .filter(|marker| marked.contains_rect(*marker))
            .collect();
        assert_eq!(
            markers.len(),
            1,
            "the marked entry {marked:?} carries exactly one painted mark, among {painted:?}"
        );
        assert!(
            marked.right() - markers[0].right() < PROBLEM_MARKER_TRACK,
            "the mark sits at the entry's trailing edge"
        );
        assert!(
            controls
                .iter()
                .any(|(label, _)| label.contains("unresolved · Size & orientation")),
            "the footer says why the primary is disabled: {:?}",
            controls
                .iter()
                .map(|(label, _)| label.as_str())
                .filter(|label| label.contains("unresolved"))
                .collect::<Vec<_>>()
        );
    }

    /// A problem the rail cannot place is a problem an operator cannot find. A
    /// field added to a panel without an owner here would disable the primary
    /// from a section nothing marks, which is the defect this wave removed.
    #[test]
    fn every_field_a_panel_reports_has_a_section() {
        let source = crate::source_guard::production_source(include_str!("render.rs"));
        const CALL: &str = "field_error(problems, \"";
        let mut seen = 0;
        for (index, _) in source.match_indices(CALL) {
            let rest = &source[index + CALL.len()..];
            let field = &rest[..rest.find('"').expect("a closing quote")];
            assert!(
                field_section(field).is_some(),
                "the panels report {field:?}, and no section owns it"
            );
            seen += 1;
        }
        assert!(
            seen >= 5,
            "the guard found only {seen} reported fields, so it is scanning the wrong region"
        );
    }

    /// The footer names what is unresolved and where, and says nothing at all
    /// when nothing is.
    #[test]
    fn the_footer_reason_names_every_section_in_rail_order() {
        assert_eq!(blocked_reason(&[]), None);
        let problems = vec![
            FieldProblem {
                field: "scale",
                message: "bad scale".to_owned(),
            },
            FieldProblem {
                field: "width",
                message: "bad width".to_owned(),
            },
        ];
        assert_eq!(
            blocked_reason(&problems).as_deref(),
            Some("2 unresolved · Size & orientation, Title block")
        );
    }
}

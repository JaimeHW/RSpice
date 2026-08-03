//! Mockup-authored three-column drawing-sheet setup studio.

use egui::{
    Align, Align2, Color32, Context, Frame, Layout, Margin, Rect, ScrollArea, Sense, Stroke,
    TextEdit, Ui, Vec2, pos2, vec2,
};

use crate::state::{
    DrawingSheetPresetScope, DrawingSheetStandard, DrawingSheetStandardSeries,
    SchematicPageOrientation,
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
    DrawingSheetSupportRequest, FieldProblem, MarginEdge, PageSetupScope, SheetDisplayUnit,
    SheetSizeChoice, TitleBlockAnchorChoice, TitleBlockRotationChoice, TitleBlockTemplateChoice,
    ZoneEdgesChoice, ZoneLabelsChoice, ZoneModeChoice, format_um,
};

const TITLE: &str = "Page setup";
const DESCRIPTION: &str = "Author the permanent physical drawing sheet independently from printer paper and export media.";
const PRIMARY: &str = "Apply sheet setup";
const NAV_WIDTH: f32 = 230.0;
const COLUMN_GAP: f32 = 0.0;

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
        let eyebrow = self.state.dialogs.drawing_sheet_setup.eyebrow.clone();
        let prospective_format = geometry_preview_format(
            &self.state.dialogs.drawing_sheet_setup.draft,
            &self.state.dialogs.drawing_sheet_setup.last_valid_format,
        );
        let prospective_overflow =
            prospective_overflow_count(&self.state, prospective_format.clone());
        let preview_content = DrawingSheetPreviewContent::from_state(&self.state);

        let mut dialog = Dialog::new(&eyebrow, TITLE, if read_only { "Close" } else { PRIMARY })
            .description(DESCRIPTION)
            .size(DialogSize::SchematicPageSetup)
            .fixed_height(760.0)
            .primary_enabled(primary_enabled)
            .flush_body()
            .manual_body_scroll();
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
                    setup.commit_error = None;
                }
                Err(error) => self.state.dialogs.drawing_sheet_setup.commit_error = Some(error),
            }
            return;
        }

        match choice {
            DialogChoice::Primary if read_only => {
                self.state.dialogs.drawing_sheet_setup.close();
            }
            DialogChoice::Primary => match apply_drawing_sheet_setup(self) {
                Ok(message) => self
                    .state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(message)),
                Err(error) => {
                    self.state.dialogs.drawing_sheet_setup.commit_error = Some(error);
                }
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
    // Validation belongs to the transaction, not to the editor column. The
    // strip exists only while there is something to report — the reference
    // dialog hides its transaction strip entirely between reports, and a
    // permanently reserved empty band reads as a designed-in gap.
    let validation_height = if validation_notice_visible(state, problems) {
        48.0
    } else {
        0.0
    };
    let body_height = (available.y - validation_height).max(1.0);
    if available.x < 980.0 {
        stacked_body(
            ui,
            state,
            problems,
            prospective_overflow,
            preview_content,
            body_height,
        );
        drawing_sheet_validation_strip(ui, state, problems, validation_height);
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
                        |ui| section_navigation(ui, state, body_height),
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
    drawing_sheet_validation_strip(ui, state, problems, validation_height);
}

fn drawing_sheet_validation_strip(
    ui: &mut Ui,
    state: &DrawingSheetSetupState,
    problems: &[FieldProblem],
    height: f32,
) {
    if validation_notice_visible(state, problems) {
        validation_slot(ui, state, problems, height);
    }
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
        .show(ui, |ui| {
            Frame::NONE
                .fill(Tokens::get(ui.ctx()).color.bg_panel)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for section in DrawingSheetSection::ALL {
                            let label = format!("{}  {}", section.number(), section.label());
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

fn section_navigation(ui: &mut Ui, state: &mut DrawingSheetSetupState, height: f32) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_width(NAV_WIDTH);
        ui.set_min_height(height);
        ui.spacing_mut().item_spacing.y = 0.0;
        for section in DrawingSheetSection::ALL {
            let selected = state.section == section;
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
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(section.label())
                                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                        .color(t.color.text),
                                );
                                ui.label(
                                    egui::RichText::new(detail)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                );
                            });
                        },
                    )
                });
            let rect = response.response.rect;
            if selected {
                ui.painter().rect_filled(
                    Rect::from_min_max(rect.min, egui::pos2(rect.left() + 3.0, rect.bottom())),
                    0.0,
                    t.color.accent,
                );
            }
            let section_response = response.response.interact(Sense::click());
            section_response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    section.label(),
                )
            });
            crate::ui::theme::paint_focus_ring(ui, &section_response, rect);
            if section_response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                state.section = section;
                state.commit_error = None;
            }
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                Stroke::new(1.0, t.color.border),
            );
        }
        let used = DrawingSheetSection::ALL.len() as f32 * 54.0;
        ui.add_space((height - used - 112.0).max(12.0));
        Frame::NONE
            .fill(t.color.bg_inset)
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("AUTHORED SHEET ≠ OUTPUT MEDIA")
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.accent),
                );
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new(
                        "Print and Export may fit, tile or clip this sheet without changing it.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
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

fn section_detail(state: &DrawingSheetSetupState, section: DrawingSheetSection) -> String {
    match section {
        DrawingSheetSection::Format => {
            let orientation = state.draft.orientation.label();
            format!("{} · {orientation}", state.draft.size.label())
        }
        DrawingSheetSection::Margins => format!(
            "{} {} binding",
            state.draft.margin_left,
            state.draft.display_unit.suffix()
        ),
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
                            .inner_margin(Margin::symmetric(18, 16))
                            .show(ui, |ui| match state.section {
                                DrawingSheetSection::Format => format_panel(ui, state, problems),
                                DrawingSheetSection::Margins => margins_panel(ui, state, problems),
                                DrawingSheetSection::Frame => frame_panel(ui, state, problems),
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
            "Margins define the printable boundary. Bleed belongs to export artwork and never enlarges the sheet.",
        ),
        DrawingSheetSection::Frame => panel_header(
            ui,
            "ENGINEERING FRAME",
            "Border, zones and marks",
            "Configure drawing references independently from the paper and printable boundary.",
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
        if selected == SheetSizeChoice::Custom {
            state.capture_restore_margins_from_draft();
        }
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
                state.restore_margins = preset.format.margins;
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
            if let Some(standard) = state.draft.size.standard() {
                state.restore_margins = standard.default_margins();
            }
        }
        state.commit_error = None;
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
                    "Saved in personal preferences; using it captures an exact project copy."
                }
                DrawingSheetPresetScope::Organization => {
                    "Organization presets are managed and cannot be created here."
                }
            });
                }
            });
    }
    ui.add_space(14.0);

    two_column(ui, |left, right| {
        field_caption(left, "Orientation");
        left.horizontal(|ui| {
            let mut selected = state.draft.orientation;
            let segment_width =
                ((ui.available_width() - ui.spacing().item_spacing.x) * 0.5).max(72.0);
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
        field_caption(right, "Display units");
        let previous = state.draft.display_unit;
        egui::ComboBox::from_id_salt("drawing-sheet-unit")
            .width(right.available_width())
            .selected_text(state.draft.display_unit.label())
            .show_ui(right, |ui| {
                for unit in SheetDisplayUnit::ALL {
                    ui.selectable_value(&mut state.draft.display_unit, unit, unit.label());
                }
            });
        if previous != state.draft.display_unit {
            let selected = state.draft.display_unit;
            state.draft.display_unit = previous;
            if let Err(error) = state.draft.set_display_unit(selected) {
                state.commit_error = Some(error);
            }
        }
    });
    ui.add_space(15.0);
    measurement_row(ui, state, &state.last_valid_format);
}

fn margins_panel(ui: &mut Ui, state: &mut DrawingSheetSetupState, problems: &[FieldProblem]) {
    margin_map(ui, &state.draft);
    ui.add_space(15.0);
    let unit = state.draft.display_unit.suffix();
    let mut changed_edge = None;
    ui.columns(4, |columns| {
        if length_field(
            &mut columns[0],
            "Top",
            &mut state.draft.margin_top,
            "10",
            field_error(problems, "margin-top").or_else(|| field_error(problems, "margins")),
            unit,
        ) {
            changed_edge = Some(MarginEdge::Top);
        }
        if length_field(
            &mut columns[1],
            "Right",
            &mut state.draft.margin_right,
            "10",
            field_error(problems, "margin-right").or_else(|| field_error(problems, "margins")),
            unit,
        ) {
            changed_edge = Some(MarginEdge::Right);
        }
        if length_field(
            &mut columns[2],
            "Bottom",
            &mut state.draft.margin_bottom,
            "10",
            field_error(problems, "margin-bottom").or_else(|| field_error(problems, "margins")),
            unit,
        ) {
            changed_edge = Some(MarginEdge::Bottom);
        }
        if length_field(
            &mut columns[3],
            "Left · binding",
            &mut state.draft.margin_left,
            "20",
            field_error(problems, "margin-left").or_else(|| field_error(problems, "margins")),
            unit,
        ) {
            changed_edge = Some(MarginEdge::Left);
        }
    });
    if let Some(edge) = changed_edge {
        state.draft.link_margins_from(edge);
    }
    ui.add_space(12.0);
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
        let linked = right.checkbox(&mut state.draft.margins_linked, "Link all four margins");
        if linked.changed() && state.draft.margins_linked {
            state.draft.link_margins_from(MarginEdge::Top);
        }
    });
    ui.add_space(14.0);
    ui.horizontal_wrapped(|ui| {
        let restore_label = state
            .draft
            .size
            .standard()
            .map(|standard| match standard.series() {
                DrawingSheetStandardSeries::Iso => "Restore ISO A margins",
                DrawingSheetStandardSeries::Ansi => "Restore ANSI margins",
                DrawingSheetStandardSeries::Architectural => "Restore ARCH margins",
                DrawingSheetStandardSeries::Jis => "Restore JIS B margins",
            })
            .unwrap_or("Restore custom-format margins");
        if Button::new(restore_label).show(ui).clicked() {
            state.restore_selected_margins();
        }
        muted(
            ui,
            "Stored to 1 µm; display-unit changes do not alter physical size.",
        );
    });
}

fn frame_panel(ui: &mut Ui, state: &mut DrawingSheetSetupState, problems: &[FieldProblem]) {
    field_caption(ui, "Border template");
    let previous_border = state.draft.border;
    ui.add_enabled_ui(
        state.draft.border != BorderTemplateChoice::Organization,
        |ui| {
            egui::ComboBox::from_id_salt("drawing-sheet-border")
                .width(ui.available_width())
                .selected_text(state.draft.border.label())
                .show_ui(ui, |ui| {
                    for choice in BorderTemplateChoice::ALL
                        .into_iter()
                        .filter(|choice| *choice != BorderTemplateChoice::Organization)
                    {
                        ui.selectable_value(&mut state.draft.border, choice, choice.label());
                    }
                });
        },
    );
    if previous_border != state.draft.border {
        state.draft.set_border_template(state.draft.border);
    }
    muted(ui, state.draft.border.summary());
    ui.add_space(14.0);
    let zones_available = !matches!(
        state.draft.border,
        BorderTemplateChoice::None | BorderTemplateChoice::Plain
    );
    ui.add_enabled_ui(zones_available, |ui| {
        two_column(ui, |left, right| {
            field_caption(left, "Zone divisions");
            left.add_enabled_ui(
                state.draft.border != BorderTemplateChoice::Organization,
                |ui| {
                    egui::ComboBox::from_id_salt("drawing-sheet-zone-mode")
                        .width(ui.available_width())
                        .selected_text(state.draft.zone_mode.label())
                        .show_ui(ui, |ui| {
                            for choice in ZoneModeChoice::ALL {
                                ui.selectable_value(
                                    &mut state.draft.zone_mode,
                                    choice,
                                    choice.label(),
                                );
                            }
                        });
                },
            );
            muted(
                left,
                &match state.draft.zone_mode {
                    ZoneModeChoice::Automatic => format!(
                        "Even counts nearest a 140 mm pitch — {} × {} on this sheet.",
                        state.draft.zone_columns, state.draft.zone_rows
                    ),
                    ZoneModeChoice::Custom => {
                        "Fixed counts, kept through size and orientation changes.".to_owned()
                    }
                    ZoneModeChoice::None => {
                        "Reviews quote sheet coordinates instead of zone references.".to_owned()
                    }
                },
            );
            field_caption(right, "Reference edges");
            right.add_enabled_ui(state.draft.zone_mode != ZoneModeChoice::None, |ui| {
                egui::ComboBox::from_id_salt("drawing-sheet-zone-edges")
                    .width(ui.available_width())
                    .selected_text(state.draft.zone_edges.label())
                    .show_ui(ui, |ui| {
                        for choice in ZoneEdgesChoice::ALL {
                            ui.selectable_value(
                                &mut state.draft.zone_edges,
                                choice,
                                choice.label(),
                            );
                        }
                    });
            });
        });
        if state.draft.zone_mode == ZoneModeChoice::Custom {
            ui.add_space(11.0);
            two_column(ui, |left, right| {
                numeric_u8_field(
                    left,
                    "Columns",
                    &mut state.draft.zone_columns,
                    2..=24,
                    field_error(problems, "zones"),
                );
                numeric_u8_field(
                    right,
                    "Rows",
                    &mut state.draft.zone_rows,
                    2..=24,
                    field_error(problems, "zones"),
                );
            });
        }
        ui.add_space(11.0);
        field_caption(ui, "Zone labels");
        ui.add_enabled_ui(state.draft.zone_mode != ZoneModeChoice::None, |ui| {
            egui::ComboBox::from_id_salt("drawing-sheet-zone-labels")
                .width(ui.available_width())
                .selected_text(state.draft.zone_labels.label())
                .show_ui(ui, |ui| {
                    for choice in ZoneLabelsChoice::ALL {
                        ui.selectable_value(&mut state.draft.zone_labels, choice, choice.label());
                    }
                });
        });
    });
    ui.add_space(14.0);
    two_column(ui, |left, right| {
        toggle_card(
            left,
            "Registration marks",
            "Paper-alignment targets",
            &mut state.draft.registration_marks,
        );
        toggle_card(
            right,
            "Folding marks",
            "Format-defined fold pitch",
            &mut state.draft.folding_marks,
        );
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
                    for choice in TitleBlockTemplateChoice::ALL
                        .into_iter()
                        .filter(|choice| *choice != TitleBlockTemplateChoice::Organization)
                    {
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
    muted(ui, state.draft.title_block.summary());
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
            muted(right, state.draft.title_block_rotation.note());
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
        ui.add_space(11.0);
        text_field(
            ui,
            "Declared drawing scale",
            &mut state.draft.declared_scale,
            "1:1",
            field_error(problems, "scale"),
        );
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
        let mut text = (*value).to_owned();
        let max_width = w - 20.0;
        while ui
            .painter()
            .layout_no_wrap(text.clone(), value_font.clone(), Color32::WHITE)
            .size()
            .x
            > max_width
            && text.chars().count() > 1
        {
            text.pop();
            while !text.is_char_boundary(text.len()) {
                text.pop();
            }
            text = format!("{}\u{2026}", text.trim_end_matches('\u{2026}').trim_end());
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

fn validation_slot(
    ui: &mut Ui,
    state: &DrawingSheetSetupState,
    problems: &[FieldProblem],
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let substitution = state
        .draft
        .validate()
        .ok()
        .filter(|validated| validated.page_format.title_block_substituted());
    let problem_detail = (!problems.is_empty()).then(|| {
        problems
            .iter()
            .map(|problem| problem.message.as_str())
            .collect::<Vec<_>>()
            .join("  •  ")
    });
    let Some((color, title, detail)) = (if let Some(error) = state.commit_error.as_deref() {
        Some((t.color.err, "Sheet setup was not applied", error))
    } else if let Some(detail) = problem_detail.as_deref() {
        Some((t.color.err, "Review the highlighted sheet values", detail))
    } else if substitution.is_some() {
        Some((
            t.color.warn,
            "Title block will use the compact form",
            "The requested title block does not fit this drawing area. RSpice compact will be drawn; the requested choice remains saved.",
        ))
    } else {
        None
    }) else {
        return;
    };
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.set_min_height(height - 12.0);
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(8.0, 8.0), Sense::hover());
                ui.painter().circle_filled(rect.center(), 3.0, color);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(title)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .on_hover_text(detail);
                });
            });
        });
}

fn validation_notice_visible(state: &DrawingSheetSetupState, problems: &[FieldProblem]) -> bool {
    state.commit_error.is_some()
        || !problems.is_empty()
        || state
            .draft
            .validate()
            .ok()
            .is_some_and(|validated| validated.page_format.title_block_substituted())
}

fn preview_column(
    ui: &mut Ui,
    state: &DrawingSheetSetupState,
    prospective_overflow: Option<usize>,
    preview_content: &DrawingSheetPreviewContent,
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_height(height);
        ScrollArea::vertical()
            .id_salt("drawing-sheet-preview-column")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_min_height(height);
                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    Frame::NONE
                .fill(t.color.bg_inset)
                .inner_margin(Margin::symmetric(12, 11))
                .show(ui, |ui| {
                    // The column stacks bottom-up to dock this summary at the
                    // bottom edge, and the frame inherits that layout: the
                    // first row added lands at the bottom. Add the reference's
                    // reading order in reverse so it reads top-down on screen.
                    let sheets_written = match state.draft.scope {
                        PageSetupScope::CurrentSheet => {
                            format!("1 · {}", state.sheet_name)
                        }
                        PageSetupScope::Document => {
                            if state.managed_sheet_names.is_empty() {
                                format!(
                                    "{} writable · {}",
                                    state.writable_sheet_count, state.document_name
                                )
                            } else {
                                format!(
                                    "{} writable of {} · managed sheets skipped: {}",
                                    state.writable_sheet_count,
                                    state.sheet_count,
                                    state.managed_sheet_names.join(", ")
                                )
                            }
                        }
                        PageSetupScope::CurrentSheetAndDefault => {
                            format!("1 · {} plus project default", state.sheet_name)
                        }
                    };
                    fact_row(
                        ui,
                        "Not affected",
                        "input revision · generated netlist · schematic checks · result datasets · release evidence",
                    );
                    fact_row(
                        ui,
                        "Undo",
                        "one entry, “Sheet format”; the previous format is restored exactly",
                    );
                    fact_row(
                        ui,
                        "Content outside the page",
                        prospective_overflow
                            .map_or("unavailable".to_owned(), |count| {
                                if count == 0 {
                                    "none · everything stays inside".to_owned()
                                } else {
                                    format!(
                                        "{count} {} would be outside the drawing area",
                                        if count == 1 { "object" } else { "objects" }
                                    )
                                }
                            })
                            .as_str(),
                    );
                    fact_row(ui, "Changes", &page_setup_change_summary(state));
                    fact_row(ui, "Sheets written", &sheets_written);
                    ui.add_space(8.0);
                    section_caption(ui, "WHAT APPLY DOES");
                });
                    let preview_height = (ui.available_height() - 8.0).max(140.0);
                    Frame::NONE
                        .inner_margin(Margin::symmetric(12, 12))
                        .show(ui, |ui| {
                            draw_sheet_preview(
                                ui,
                                &state.draft,
                                &state.last_valid_format,
                                preview_content,
                                preview_height - 24.0,
                            );
                        });
                });
            });
    });
}

fn page_setup_change_summary(state: &DrawingSheetSetupState) -> String {
    let before = &state.baseline;
    let after = &state.draft;
    let mut changes = Vec::new();
    if before.size != after.size || before.width != after.width || before.height != after.height {
        changes.push(format!(
            "format {} → {}",
            before.size.label(),
            after.size.label()
        ));
    }
    if before.orientation != after.orientation {
        changes.push(format!(
            "orientation {} → {}",
            before.orientation.label().to_lowercase(),
            after.orientation.label().to_lowercase()
        ));
    }
    if before.border != after.border {
        changes.push(format!(
            "border {} → {}",
            before.border.label().to_lowercase(),
            after.border.label().to_lowercase()
        ));
    }
    if before.title_block != after.title_block {
        changes.push(format!(
            "title block {} → {}",
            before.title_block.label(),
            after.title_block.label()
        ));
    }
    if before.margin_top != after.margin_top
        || before.margin_right != after.margin_right
        || before.margin_bottom != after.margin_bottom
        || before.margin_left != after.margin_left
    {
        changes.push("margins".to_owned());
    }
    if changes.is_empty() {
        "none — the draft matches the authored sheet".to_owned()
    } else {
        changes.join(" · ")
    }
}

fn prospective_overflow_count(
    state: &crate::workbench::app_state::AppState,
    format: Option<crate::state::SchematicSheetFormat>,
) -> Option<usize> {
    let format = format?;
    let current = crate::schematic::view::drawing_sheet::ActiveDrawingSheet::resolve(state);
    let prospective = crate::schematic::view::drawing_sheet::ActiveDrawingSheet {
        geometry: crate::schematic::view::drawing_sheet::DrawingSheetGeometry::from_format(&format),
        format,
        sheet_name: current.sheet_name,
        page_label: current.page_label,
        revision: current.revision,
    };
    let symbols = crate::schematic::view::SchematicSymbolContext::from_state(state);
    Some(
        crate::schematic::view::drawing_sheet::drawing_sheet_overflow_summary(
            state,
            &symbols,
            &prospective,
        )
        .finding_count(),
    )
}

fn draw_sheet_preview(
    ui: &mut Ui,
    draft: &DrawingSheetDraft,
    last_valid_format: &crate::state::SchematicSheetFormat,
    preview_content: &DrawingSheetPreviewContent,
    height: f32,
) {
    let current = geometry_preview_format(draft, last_valid_format);
    let format = current.as_ref().unwrap_or(last_valid_format);
    let invalid = current.is_none();
    let label = if invalid {
        format!("Last valid page · {}", format.display())
    } else {
        format.display()
    };
    super::super::drawing_sheet_preview::drawing_sheet_preview_with_content(
        ui,
        format,
        height,
        &label,
        preview_content,
        invalid,
    );
}

/// Resolve the page geometry independently from non-geometric transaction
/// fields. An invalid reusable-preset name or declared drawing scale must
/// disable Apply and receive validation feedback, but it must not make a
/// physically valid page jump back to an earlier preview.
fn geometry_preview_format(
    draft: &DrawingSheetDraft,
    fallback: &crate::state::SchematicSheetFormat,
) -> Option<crate::state::SchematicSheetFormat> {
    let mut preview = draft.clone();
    let fallback_draft = DrawingSheetDraft::from_format(fallback);
    preview.save_custom_preset = false;
    preview.declared_scale = fallback_draft.declared_scale;
    let sheet_title = preview
        .title_fields
        .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle);
    if sheet_title
        .as_ref()
        .is_none_or(|field| !field.visible || field.value.trim().is_empty())
    {
        let fallback_title = fallback_draft
            .title_fields
            .get(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
            .cloned()
            .unwrap_or_else(|| crate::state::DrawingSheetTitleFieldState {
                visible: true,
                value: "Preview".to_owned(),
            });
        match sheet_title {
            Some(field) => *field = fallback_title,
            None => {
                preview.title_fields.insert(
                    crate::state::DrawingSheetTitleFieldId::SheetTitle,
                    fallback_title,
                );
            }
        }
    }
    preview
        .validate()
        .ok()
        .map(|validated| validated.page_format)
}

fn margin_map(ui: &mut Ui, draft: &DrawingSheetDraft) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 160.0), Sense::hover());
    ui.painter().rect_filled(rect, 2.0, t.color.canvas_bg);
    let fallback = crate::state::SchematicSheetFormat::default();
    let Some(format) = geometry_preview_format(draft, &fallback) else {
        return;
    };
    let Ok(geometry) = format.geometry() else {
        return;
    };
    let paper_geometry = geometry.paper;
    let scale = ((rect.width() - 84.0) / paper_geometry.width_um as f32)
        .min((rect.height() - 24.0) / paper_geometry.height_um as f32)
        .max(0.001);
    let page = Rect::from_center_size(
        rect.center(),
        vec2(
            paper_geometry.width_um as f32 * scale,
            paper_geometry.height_um as f32 * scale,
        ),
    );
    let printable = project_sheet_rect(geometry.printable, paper_geometry, page, scale);
    ui.painter().rect_filled(page, 0.0, t.color.bg_elevated);
    for band in [
        Rect::from_min_max(page.min, egui::pos2(page.right(), printable.top())),
        Rect::from_min_max(egui::pos2(page.left(), printable.bottom()), page.max),
        Rect::from_min_max(
            egui::pos2(page.left(), printable.top()),
            egui::pos2(printable.left(), printable.bottom()),
        ),
        Rect::from_min_max(
            egui::pos2(printable.right(), printable.top()),
            egui::pos2(page.right(), printable.bottom()),
        ),
    ] {
        if band.is_positive() {
            ui.painter().rect_filled(band, 0.0, t.color.bg_active);
        }
    }
    ui.painter().rect_stroke(
        page,
        0.0,
        Stroke::new(1.0, t.color.text_dim),
        egui::StrokeKind::Inside,
    );
    paint_dashed_rect(ui, printable, Stroke::new(1.0, t.color.accent), 4.0, 3.0);
    for (position, align, value) in [
        (
            egui::pos2(page.center().x, (page.top() + printable.top()) * 0.5),
            Align2::CENTER_CENTER,
            format!("top {}", draft.margin_top),
        ),
        (
            egui::pos2((printable.right() + page.right()) * 0.5, page.center().y),
            Align2::CENTER_CENTER,
            format!("right {}", draft.margin_right),
        ),
        (
            egui::pos2(page.center().x, (printable.bottom() + page.bottom()) * 0.5),
            Align2::CENTER_CENTER,
            format!("bottom {}", draft.margin_bottom),
        ),
        (
            egui::pos2((page.left() + printable.left()) * 0.5, page.center().y),
            Align2::CENTER_CENTER,
            format!("binding {}", draft.margin_left),
        ),
    ] {
        ui.painter().text(
            position,
            align,
            &value,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text,
        );
    }
    // Margin labels are painter text rather than layout widgets. Keep a fixed
    // caption track below their glyph bounds so high DPI and fractional
    // browser zoom cannot make the bottom label and figcaption collide.
    ui.add_space(28.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 20.0),
        Layout::centered_and_justified(egui::Direction::LeftToRight),
        |ui| {
            ui.label(
                egui::RichText::new(
                    "Shaded bands are margin; the dashed rule is the printable boundary.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        },
    );
}

fn paint_dashed_rect(ui: &Ui, rect: Rect, stroke: Stroke, dash: f32, gap: f32) {
    for (start, end) in [
        (rect.left_top(), rect.right_top()),
        (rect.right_top(), rect.right_bottom()),
        (rect.right_bottom(), rect.left_bottom()),
        (rect.left_bottom(), rect.left_top()),
    ] {
        let delta = end - start;
        let length = delta.length();
        if length <= f32::EPSILON {
            continue;
        }
        let direction = delta / length;
        let mut offset = 0.0;
        while offset < length {
            let segment_end = (offset + dash).min(length);
            ui.painter().line_segment(
                [start + direction * offset, start + direction * segment_end],
                stroke,
            );
            offset += dash + gap;
        }
    }
}

fn project_sheet_rect(
    rect: crate::state::DrawingSheetRect,
    extent: crate::state::DrawingSheetRect,
    extent_screen: Rect,
    scale: f32,
) -> Rect {
    let x = extent_screen.left() + (rect.x_um - extent.x_um) as f32 * scale;
    let y = extent_screen.top() + (rect.y_um - extent.y_um) as f32 * scale;
    Rect::from_min_size(
        egui::pos2(x, y),
        vec2(rect.width_um as f32 * scale, rect.height_um as f32 * scale),
    )
}

fn measurement_row(
    ui: &mut Ui,
    state: &DrawingSheetSetupState,
    fallback: &crate::state::SchematicSheetFormat,
) {
    let validated = geometry_preview_format(&state.draft, fallback);
    let (paper, printable, drawing) = validated.map_or_else(
        || {
            (
                "Invalid dimensions".to_owned(),
                "—".to_owned(),
                "—".to_owned(),
            )
        },
        |value| {
            let geometry = value.geometry().ok();
            let printable = geometry.map(|geometry| geometry.printable);
            let drawing = geometry.map(|geometry| geometry.drawing_area);
            let dimensions = |rect: Option<crate::state::DrawingSheetRect>| {
                rect.map_or_else(
                    || "—".to_owned(),
                    |rect| {
                        format!(
                            "{} × {} {}",
                            format_um(rect.width_um, state.draft.display_unit),
                            format_um(rect.height_um, state.draft.display_unit),
                            state.draft.display_unit.suffix()
                        )
                    },
                )
            };
            (
                format!(
                    "{} × {} {}",
                    format_um(value.oriented_dimensions_um().0, state.draft.display_unit,),
                    format_um(value.oriented_dimensions_um().1, state.draft.display_unit,),
                    state.draft.display_unit.suffix()
                ),
                dimensions(printable),
                dimensions(drawing),
            )
        },
    );
    measurement_strip(
        ui,
        &[
            ("Paper", paper.as_str()),
            ("Printable", printable.as_str()),
            ("Drawing area", drawing.as_str()),
        ],
    );
}

fn format_summary(draft: &DrawingSheetDraft) -> String {
    format!(
        "{} · {} · {} × {} {}",
        draft.size.label(),
        draft.orientation.label(),
        draft.width,
        draft.height,
        draft.display_unit.suffix()
    )
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
            "Architectural series — North American large format"
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

fn two_column(ui: &mut Ui, content: impl FnOnce(&mut Ui, &mut Ui)) {
    ui.columns(2, |columns| {
        columns[0].spacing_mut().item_spacing.x = 12.0;
        columns[1].spacing_mut().item_spacing.x = 12.0;
        let (left, right) = columns.split_at_mut(1);
        content(&mut left[0], &mut right[0]);
    });
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

fn numeric_u8_field(
    ui: &mut Ui,
    label: &str,
    value: &mut u8,
    range: std::ops::RangeInclusive<u8>,
    error: Option<&str>,
) {
    field_caption(ui, label);
    ui.add_sized(
        vec2(ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h),
        egui::DragValue::new(value).range(range),
    );
    field_error_slot(ui, error);
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

fn fact_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.set_min_height(25.0);
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
                .wrap(),
            );
        });
    });
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
mod tests {
    use super::page_setup_primary_enabled;

    #[test]
    fn initial_governed_sheet_can_be_created_without_artificial_edits() {
        assert!(page_setup_primary_enabled(false, true, false, true));
        assert!(!page_setup_primary_enabled(false, true, false, false));
        assert!(!page_setup_primary_enabled(false, false, false, true));
    }
}

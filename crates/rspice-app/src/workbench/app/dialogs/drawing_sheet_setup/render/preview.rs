//! The Page Setup dialog's preview column and the page geometry it draws.
//!
//! The preview, the margin map, and the measurement strip all show the page
//! the draft would author, resolved independently of the draft's
//! non-geometric fields, so an invalid preset name or scale never makes a
//! physically valid page jump back to an earlier one.

use super::*;

pub(super) fn preview_column(
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
                let sheets_written = match state.draft.scope {
                    PageSetupScope::CurrentSheet => format!("1 · {}", state.sheet_name),
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
                let overflow = prospective_overflow.map_or("unavailable".to_owned(), |count| {
                    if count == 0 {
                        "none · everything stays inside".to_owned()
                    } else {
                        format!(
                            "{count} {} would be outside the drawing area",
                            if count == 1 { "object" } else { "objects" }
                        )
                    }
                });
                let changes = page_setup_change_summary(state);
                let facts = [
                    ("Sheets written", sheets_written.as_str()),
                    ("Changes", changes.as_str()),
                    ("Content outside the page", overflow.as_str()),
                    (
                        "Undo",
                        "one entry, “Sheet format”; the previous format is restored exactly",
                    ),
                    (
                        "Not affected",
                        "input revision · generated netlist · schematic checks · result datasets · release evidence",
                    ),
                ];
                let summary_height = effect_summary_height(ui, ui.available_width(), &facts);
                let preview_height = (height - summary_height).max(140.0);

                // The preview and effect summary are ordinary top-down rows.
                // A previous bottom-up layout leaked into the summary's nested
                // rows; zero-height child allocations then shared one baseline
                // and painted every fact on top of the others.
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), preview_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        // No inset: the preview paints its own desk across the
                        // whole region it is given, including the format
                        // caption. An inset here framed that desk in dialog
                        // background and left the caption sitting outside it.
                        draw_sheet_preview(
                            ui,
                            &state.draft,
                            &state.last_valid_format,
                            preview_content,
                            preview_height,
                        );
                    },
                );
                Frame::NONE
                    .fill(t.color.bg_inset)
                    .inner_margin(Margin::symmetric(12, 11))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        section_caption(ui, "WHAT APPLY DOES");
                        ui.add_space(8.0);
                        for (index, (label, value)) in facts.iter().enumerate() {
                            fact_row(ui, label, value);
                            if index + 1 < facts.len() {
                                ui.add_space(6.0);
                            }
                        }
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

pub(super) fn prospective_overflow_count(
    state: &crate::workbench::app_state::AppState,
    format: Option<crate::state::SchematicSheetFormat>,
) -> Option<usize> {
    let format = format?;
    let current = crate::schematic::view::drawing_sheet::resolve_active_drawing_sheet(state);
    let prospective = crate::schematic::view::drawing_sheet::ActiveDrawingSheet {
        geometry: crate::schematic::view::drawing_sheet::DrawingSheetGeometry::from_format(&format),
        format,
        sheet_name: current.sheet_name,
        page_label: current.page_label,
    };
    let symbols = crate::schematic::view::schematic_symbol_context(state);
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
    super::super::super::drawing_sheet_preview::drawing_sheet_preview_with_content(
        ui,
        format,
        height,
        &label,
        preview_content,
        invalid,
    );
}

/// Resolve the page geometry independently from non-geometric transaction
/// fields. An invalid reusable-preset name must disable Apply and receive
/// validation feedback, but it must not make a physically valid page jump
/// back to an earlier preview.
pub(super) fn geometry_preview_format(
    draft: &DrawingSheetDraft,
    fallback: &crate::state::SchematicSheetFormat,
) -> Option<crate::state::SchematicSheetFormat> {
    let mut preview = draft.clone();
    let fallback_draft = DrawingSheetDraft::from_format(fallback);
    preview.save_custom_preset = false;
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

pub(super) fn margin_map(ui: &mut Ui, draft: &DrawingSheetDraft) {
    let t = Tokens::get(ui.ctx());
    // The map is a figure, not a field: it bleeds back through the panel's own
    // inset on three sides, so it meets the section header above it and the
    // column edges beside it, and its caption sits on the same surface rather
    // than on the panel behind it.
    let (allocated, _) = ui.allocate_exact_size(
        vec2(
            ui.available_width(),
            MARGIN_MAP_HEIGHT + MARGIN_MAP_CAPTION_HEIGHT - PANEL_INSET_Y,
        ),
        Sense::hover(),
    );
    let surface = Rect::from_min_max(
        pos2(
            allocated.left() - PANEL_INSET_X,
            allocated.top() - PANEL_INSET_Y,
        ),
        pos2(allocated.right() + PANEL_INSET_X, allocated.bottom()),
    );
    ui.painter().rect_filled(surface, 0.0, t.color.canvas_bg);
    let rect = Rect::from_min_max(
        surface.min,
        pos2(
            surface.right(),
            surface.bottom() - MARGIN_MAP_CAPTION_HEIGHT,
        ),
    );
    let fallback = crate::state::SchematicSheetFormat::default();
    let Some(format) = geometry_preview_format(draft, &fallback) else {
        return;
    };
    let Ok(geometry) = format.geometry() else {
        return;
    };
    let paper_geometry = geometry.paper;
    // Fit the sheet inside the map with a gutter on every edge. A floor on the
    // scale would defeat the fit rather than guard it — every ISO sheet lands
    // well below a 0.001 px/µm floor, and clamping there drew the page larger
    // than the box it was allocated, spilling the diagram over the section
    // header. Guarding the inputs keeps the ratio positive without that.
    let scale = ((rect.width() - 2.0 * MARGIN_MAP_GUTTER).max(1.0)
        / paper_geometry.width_um.max(1) as f32)
        .min(
            (rect.height() - 2.0 * MARGIN_MAP_GUTTER).max(1.0)
                / paper_geometry.height_um.max(1) as f32,
        );
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
    // A margin band is routinely thinner than a line of type — a 10 mm ISO
    // margin is a handful of pixels at map scale — so a value centred in its
    // band crosses the paper edge and the printable rule, and the left and
    // right values run off the sheet entirely. Dimension each margin the way a
    // drawing does instead: a witness line spanning the band, and the value set
    // clear of it inside the drawing area, which is always the largest region
    // on the map.
    let callout = Stroke::new(1.0, t.color.text_dim);
    let label_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let inset = 7.0;
    let binding_value = format!("binding {}", draft.margin_left);
    let right_value = format!("right {}", draft.margin_right);
    let text_width = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), label_font.clone(), Color32::WHITE)
            .size()
            .x
    };
    // A portrait sheet is narrow enough that the two side values would meet in
    // the middle of the drawing area. Split them onto their own rules when the
    // centre line cannot seat both.
    let side_shift = if text_width(&binding_value) + text_width(&right_value) + 2.0 * inset + 8.0
        > printable.width()
    {
        9.0
    } else {
        0.0
    };
    let binding_y = page.center().y - side_shift;
    let right_y = page.center().y + side_shift;
    for (span, anchor, align, value) in [
        (
            [
                egui::pos2(page.center().x, page.top()),
                egui::pos2(page.center().x, printable.top()),
            ],
            egui::pos2(page.center().x, printable.top() + inset),
            Align2::CENTER_TOP,
            format!("top {}", draft.margin_top),
        ),
        (
            [
                egui::pos2(printable.right(), right_y),
                egui::pos2(page.right(), right_y),
            ],
            egui::pos2(printable.right() - inset, right_y),
            Align2::RIGHT_CENTER,
            right_value.clone(),
        ),
        (
            [
                egui::pos2(page.center().x, printable.bottom()),
                egui::pos2(page.center().x, page.bottom()),
            ],
            egui::pos2(page.center().x, printable.bottom() - inset),
            Align2::CENTER_BOTTOM,
            format!("bottom {}", draft.margin_bottom),
        ),
        (
            [
                egui::pos2(page.left(), binding_y),
                egui::pos2(printable.left(), binding_y),
            ],
            egui::pos2(printable.left() + inset, binding_y),
            Align2::LEFT_CENTER,
            binding_value.clone(),
        ),
    ] {
        paint_dimension_span(ui, span, callout);
        ui.painter()
            .text(anchor, align, &value, label_font.clone(), t.color.text);
    }
    // The caption is painted on the map's own surface, in the track reserved
    // for it below the sheet, rather than laid out on the panel behind it.
    ui.painter().text(
        pos2(
            surface.center().x,
            surface.bottom() - MARGIN_MAP_CAPTION_HEIGHT * 0.5,
        ),
        Align2::CENTER_CENTER,
        "Shaded bands are margin; the dashed rule is the printable boundary.",
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

/// A drafting dimension across one margin band: the measured span plus a
/// terminator at each end, so the band being quoted stays legible even when it
/// is only a few pixels deep.
fn paint_dimension_span(ui: &Ui, span: [egui::Pos2; 2], stroke: Stroke) {
    let [start, end] = span;
    let delta = end - start;
    if delta.length() <= f32::EPSILON {
        return;
    }
    let tick = vec2(-delta.y, delta.x).normalized() * 3.0;
    ui.painter().line_segment(span, stroke);
    ui.painter()
        .line_segment([start - tick, start + tick], stroke);
    ui.painter().line_segment([end - tick, end + tick], stroke);
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

pub(super) fn measurement_row(
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

//! Choosing an analysis to add: the catalog window and its search.
//!
//! The catalog lists every analysis kind the plan could take, and each row
//! states its disposition against the current stack rather than being silently
//! hidden — an engineer looking for an analysis that is already configured, or
//! not applicable here, gets told which, not an empty list.

use super::*;

pub(super) fn analysis_catalog_window(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    rows: &[AnalysisStackRow],
    action: &mut Option<StackAction>,
) {
    if !app.state.sim_setup.palette_open {
        return;
    }

    let mut query = app.state.sim_setup.palette_query.clone();
    let mut active = app.state.sim_setup.palette_active;
    let mut chosen = None;
    let mut request_close = false;
    let scroll_to_active = app.state.sim_setup.palette_scroll_to_active;
    let catalog_columns = analysis_catalog_column_count(ctx.content_rect().width());
    let choice = Dialog::new("Simulation Studio", "Add analysis or workflow", "Close")
        .description(
            "Search and add an explicitly classified solver, run-set controller, measurement, check, or optimization workflow.",
        )
        .size(DialogSize::AnalysisCatalog)
        .initial_focus(DialogInitialFocus::BodyControl)
        .primary_on_enter(false)
        .flush_body()
        .manual_body_scroll()
        .hint(
            "Solvers, run-set controllers, measurements, checks, and optimization workflows are classified explicitly.",
        )
        .note_only_footer()
        .show_with_initial_body_focus(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            let mut search_id = None;
            egui::Frame::NONE
                .fill(t.color.bg_inset)
                .show(ui, |ui| {
                    let width = ui.available_width();
                    ui.allocate_ui_with_layout(
                        vec2(width, 48.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = 9.0;
                            ui.add_space(12.0);
                            let (icon_rect, _) =
                                ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                            WorkbenchIcon::Search.paint(
                                ui.painter(),
                                icon_rect,
                                t.color.text_dim,
                            );
                            let keycap_font = theme::mono(tokens::FS_0, FontWeight::Regular);
                            let keycap_width = (ui
                                .painter()
                                .layout_no_wrap(
                                    "Esc".to_owned(),
                                    keycap_font.clone(),
                                    t.color.text_dim,
                                )
                                .size()
                                .x
                                + 10.0)
                                .max(19.0);
                            let input_width =
                                (ui.available_width() - keycap_width - 21.0).max(1.0);
                            let search = ui.add_sized(
                                vec2(input_width, 48.0),
                                analysis_catalog_search_field(&mut query),
                            );
                            // The hint text is published as a placeholder, not
                            // as a name: the first control of the overlay a
                            // keystroke opens announced nothing at all.
                            crate::ui::widgets::name_control(
                                ui,
                                &search,
                                ANALYSIS_CATALOG_SEARCH_LABEL,
                            );
                            search_id = Some(search.id);
                            if search.changed() {
                                active = 0;
                            }
                            let (keycap_rect, _) = ui.allocate_exact_size(
                                vec2(keycap_width, 18.0),
                                Sense::hover(),
                            );
                            ui.painter().rect_filled(
                                keycap_rect,
                                3.0,
                                t.color.bg_panel_2,
                            );
                            ui.painter().rect_stroke(
                                keycap_rect,
                                3.0,
                                Stroke::new(1.0, t.color.border_strong),
                                egui::StrokeKind::Inside,
                            );
                            ui.painter().text(
                                keycap_rect.center(),
                                Align2::CENTER_CENTER,
                                "Esc",
                                keycap_font,
                                t.color.text_dim,
                            );
                            ui.add_space(12.0);
                        },
                    );
                });
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, t.color.border),
            );

            let filtered = filtered_catalog_kinds(&query);
            if filtered.is_empty() {
                active = 0;
            } else {
                active = active.min(filtered.len() - 1);
                if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
                    active = (active + 1).min(filtered.len() - 1);
                }
                if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
                    active = active.saturating_sub(1);
                }
                if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                    chosen = filtered
                        .get(active)
                        .copied()
                        .filter(|kind| kind.execution_blocker().is_none());
                }
            }

            let results_height = ui.available_height().max(1.0);
            egui::Frame::NONE.fill(t.color.bg_app).show(ui, |ui| {
                ui.set_min_height(results_height);
                ScrollArea::vertical()
                    .id_salt("workbench.simulate.analysis_catalog.rows")
                    .auto_shrink([false, false])
                    .max_height(results_height)
                    .min_scrolled_height(results_height)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        if filtered.is_empty() {
                            ui.add_space(12.0);
                            super::page_kit::note_line(
                                ui,
                                "No analysis matches this search.",
                                super::page_kit::Tone::Dim,
                            );
                            return;
                        }
                        let mut rendered_groups = 0;
                        for group in ANALYSIS_CATEGORY_ORDER {
                            let members = filtered
                                .iter()
                                .copied()
                                .enumerate()
                                .filter(|(_, kind)| analysis_catalog_group(*kind) == group)
                                .collect::<Vec<_>>();
                            if members.is_empty() {
                                continue;
                            }
                            analysis_catalog_group_header(
                                ui,
                                group,
                                members.len(),
                                rendered_groups > 0,
                            );
                            rendered_groups += 1;
                            if let Some(kind) = analysis_catalog_group_rows(
                                ui,
                                &members,
                                rows,
                                active,
                                scroll_to_active,
                                catalog_columns,
                            ) {
                                chosen = Some(kind);
                            }
                        }
                    });
            });
            search_id
        });

    if choice == DialogChoice::Cancelled {
        request_close = true;
    }

    if let Some(kind) = chosen {
        *action = Some(StackAction::Insert(kind));
        request_close = true;
    }
    if request_close {
        app.state.sim_setup.palette_open = false;
    }
    app.state.sim_setup.palette_query = query;
    app.state.sim_setup.palette_active = active;
    app.state.sim_setup.palette_scroll_to_active = false;
}

/// What the catalogue's search field is called.
///
/// Separate from its hint text because the two are different things: the hint
/// says what may be typed, the name says which control the reader is on. egui
/// publishes `hint_text` as a placeholder, so a field with only a hint
/// announces no name at all.
pub(super) const ANALYSIS_CATALOG_SEARCH_LABEL: &str = "Search the analysis catalogue";

pub(super) fn analysis_catalog_search_field(query: &mut String) -> egui::TextEdit<'_> {
    egui::TextEdit::singleline(query)
        .id_source("workbench.simulate.analysis_catalog.search")
        .font(theme::sans(tokens::FS_2, FontWeight::Regular))
        .hint_text("Search solvers, sweeps, measurements, checks…")
        .vertical_align(Align::Center)
        .frame(egui::Frame::NONE)
}

pub(super) const fn analysis_catalog_column_count(viewport_width: f32) -> usize {
    if viewport_width >= 1_200.0 { 2 } else { 1 }
}

pub(super) fn analysis_catalog_group_rows(
    ui: &mut Ui,
    members: &[(usize, AnalysisKind)],
    rows: &[AnalysisStackRow],
    active: usize,
    scroll_to_active: bool,
    columns: usize,
) -> Option<AnalysisKind> {
    let mut chosen = None;
    let chunk_count = members.len().div_ceil(columns);
    for (chunk_index, chunk) in members.chunks(columns).enumerate() {
        let draw_bottom_border = chunk_index + 1 < chunk_count;
        if columns == 1 {
            let (index, kind) = chunk[0];
            let disposition = analysis_catalog_disposition(rows, kind);
            let row_height =
                analysis_catalog_row_height(ui, kind, &disposition, ui.available_width());
            if analysis_catalog_row(
                ui,
                kind,
                &disposition,
                index == active,
                scroll_to_active,
                AnalysisCatalogRowLayout {
                    height: row_height,
                    draw_bottom_border,
                    draw_right_border: false,
                },
            ) {
                chosen = Some(kind);
            }
            continue;
        }

        let gap = 0.0;
        let column_width = ((ui.available_width() - gap) / 2.0).max(1.0);
        let row_height = chunk
            .iter()
            .map(|&(_, kind)| {
                let disposition = analysis_catalog_disposition(rows, kind);
                analysis_catalog_row_height(ui, kind, &disposition, column_width)
            })
            .fold(ANALYSIS_CATALOG_ROW_HEIGHT, f32::max);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (column, &(index, kind)) in chunk.iter().enumerate() {
                let disposition = analysis_catalog_disposition(rows, kind);
                ui.allocate_ui_with_layout(
                    vec2(column_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_width(column_width);
                        if analysis_catalog_row(
                            ui,
                            kind,
                            &disposition,
                            index == active,
                            scroll_to_active,
                            AnalysisCatalogRowLayout {
                                height: row_height,
                                draw_bottom_border,
                                draw_right_border: column == 0,
                            },
                        ) {
                            chosen = Some(kind);
                        }
                    },
                );
            }
            if chunk.len() == 1 {
                ui.allocate_exact_size(vec2(column_width, row_height), Sense::hover());
            }
        });
    }
    chosen
}

pub(super) fn analysis_catalog_disposition(
    rows: &[AnalysisStackRow],
    kind: AnalysisKind,
) -> String {
    if kind.execution_blocker().is_some() {
        return "Unavailable".to_owned();
    }
    // Naming the one instance already in the plan is worth more than counting
    // it: "Add another · \"Startup transient\" in plan" says what adding a
    // second one would sit beside. A count only says there is something there.
    // Past one, the names would not fit and the count is the useful fact.
    let configured = rows
        .iter()
        .filter(|row| row.kind == kind)
        .collect::<Vec<_>>();
    match configured.as_slice() {
        [] => "Add instance".to_owned(),
        [only] => format!("Add another · \"{}\" in plan", only.name),
        rows => format!("Add another · {} in plan", rows.len()),
    }
}

pub(super) fn analysis_catalog_group_header(
    ui: &mut Ui,
    group: &str,
    count: usize,
    has_predecessor: bool,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), ANALYSIS_CATALOG_GROUP_HEIGHT),
        Sense::hover(),
    );
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.rect_filled(rect, 0.0, t.color.bg_panel_2);
    if has_predecessor {
        painter.hline(
            rect.x_range(),
            rect.top(),
            Stroke::new(1.0, t.color.border_strong),
        );
    }
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    painter.text(
        rect.left_center() + vec2(12.0, 0.0),
        Align2::LEFT_CENTER,
        group.to_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    painter.text(
        rect.right_center() - vec2(12.0, 0.0),
        Align2::RIGHT_CENTER,
        count,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
}

pub(super) const fn analysis_catalog_kind_label(kind: AnalysisKind) -> &'static str {
    match kind {
        AnalysisKind::MonteCarlo
        | AnalysisKind::Temperature
        | AnalysisKind::Corner
        | AnalysisKind::DcMismatch => "Run-set controller",
        AnalysisKind::Fourier | AnalysisKind::Disto => "Derived measurement",
        AnalysisKind::Reliability | AnalysisKind::Soa => "Verification workspace",
        AnalysisKind::Optimization => "Optimization workspace",
        _ => "Numerical solver",
    }
}

pub(super) fn analysis_catalog_detail_galley(
    ui: &Ui,
    kind: AnalysisKind,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = width.max(1.0);
    job.wrap.max_rows = 2;
    job.append(
        &analysis_catalog_kind_label(kind).to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: t.color.text_faint,
            extra_letter_spacing: 0.04 * tokens::FS_0,
            ..Default::default()
        },
    );
    job.append(
        "  ·  ",
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: t.color.text_faint,
            ..Default::default()
        },
    );
    job.append(
        kind.detail(),
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            color: t.color.text_dim,
            ..Default::default()
        },
    );
    ui.painter().layout_job(job)
}

pub(super) fn analysis_catalog_row_height(
    ui: &Ui,
    kind: AnalysisKind,
    disposition: &str,
    row_width: f32,
) -> f32 {
    let t = Tokens::get(ui.ctx());
    let compact = row_width <= TITLE_ACTION_STACK_BREAKPOINT;
    let code_right = if compact { 54.0 } else { 70.0 };
    let copy_left = code_right + if compact { 9.0 } else { 12.0 };
    let (copy_width, readiness_width) = if compact {
        (
            (row_width - 10.0 - copy_left).max(1.0),
            (row_width - 10.0 - copy_left).max(1.0),
        )
    } else {
        let readiness_left =
            (row_width - 12.0 - ANALYSIS_CATALOG_READINESS_WIDTH).max(copy_left + 96.0);
        (
            (readiness_left - 12.0 - copy_left).max(1.0),
            (row_width - 12.0 - readiness_left - 24.0).max(1.0),
        )
    };
    let title = ui.painter().layout(
        kind.label().to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        copy_width,
    );
    let detail = analysis_catalog_detail_galley(ui, kind, copy_width);
    let copy_height = title.size().y + 3.0 + detail.size().y;
    let action = ui.painter().layout(
        disposition.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        readiness_width,
    );
    let readiness_detail = analysis_catalog_readiness(kind).map(|detail| {
        ui.painter().layout(
            detail.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
            readiness_width,
        )
    });
    let readiness_height = action.size().y
        + readiness_detail
            .as_ref()
            .map_or(0.0, |detail| 2.0 + detail.size().y);
    if compact {
        (16.0 + copy_height + 5.0 + readiness_height)
            .max(ANALYSIS_CATALOG_ROW_HEIGHT)
            .ceil()
    } else {
        (14.0 + copy_height.max(readiness_height))
            .max(ANALYSIS_CATALOG_ROW_HEIGHT)
            .ceil()
    }
}

pub(super) fn analysis_catalog_row(
    ui: &mut Ui,
    kind: AnalysisKind,
    disposition: &str,
    selected: bool,
    scroll_to_active: bool,
    layout: AnalysisCatalogRowLayout,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let compact = ui.available_width() <= TITLE_ACTION_STACK_BREAKPOINT;
    let blocker = kind.execution_blocker();
    let enabled = ui.is_enabled() && blocker.is_none();
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), layout.height), sense);
    if selected && scroll_to_active {
        let reveal = Rect::from_min_max(
            egui::pos2(rect.left(), rect.top() - ANALYSIS_CATALOG_GROUP_HEIGHT),
            rect.max,
        );
        ui.scroll_to_rect(reveal, Some(Align::Min));
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            enabled,
            selected,
            if let Some(reason) = blocker {
                format!("{} unavailable: {reason}", kind.label())
            } else {
                format!("Add {} analysis instance", kind.label())
            },
        )
    });
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.rect_filled(
        rect,
        0.0,
        if selected {
            t.color.bg_panel
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_app
        },
    );
    if layout.draw_bottom_border {
        painter.hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border),
        );
    }
    if layout.draw_right_border {
        painter.vline(
            rect.right() - 0.5,
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    if selected {
        painter.vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    }

    let code_right = rect.left() + if compact { 54.0 } else { 70.0 };
    painter.vline(
        code_right,
        rect.y_range().shrink(7.0),
        Stroke::new(1.0, t.color.border),
    );
    painter.text(
        rect.left_center() + vec2(12.0, 0.0),
        Align2::LEFT_CENTER,
        kind.code(),
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.accent,
    );

    let copy_left = code_right + if compact { 9.0 } else { 12.0 };
    let (copy_right, readiness_left) = if compact {
        (rect.right() - 10.0, copy_left)
    } else {
        let readiness_left =
            (rect.right() - 12.0 - ANALYSIS_CATALOG_READINESS_WIDTH).max(copy_left + 96.0);
        (readiness_left - 12.0, readiness_left)
    };
    let copy_width = (copy_right - copy_left).max(1.0);
    let title_galley = ui.painter().layout(
        kind.label().to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        copy_width,
    );
    let detail_galley = analysis_catalog_detail_galley(ui, kind, copy_width);
    let copy_top = rect.top() + if compact { 8.0 } else { 7.0 };
    let copy_clip = Rect::from_min_max(
        egui::pos2(copy_left, copy_top),
        egui::pos2(copy_right, rect.bottom() - if compact { 8.0 } else { 7.0 }),
    );
    let copy_painter = painter.with_clip_rect(copy_clip);
    copy_painter.galley(
        egui::pos2(copy_left, copy_top),
        title_galley.clone(),
        t.color.text,
    );
    copy_painter.galley(
        egui::pos2(copy_left, copy_top + title_galley.size().y + 3.0),
        detail_galley.clone(),
        t.color.text_dim,
    );

    if !compact {
        painter.vline(
            readiness_left,
            rect.y_range().shrink(7.0),
            Stroke::new(1.0, t.color.border),
        );
    }
    let readiness = analysis_catalog_readiness(kind);
    let copy_height = title_galley.size().y + 3.0 + detail_galley.size().y;
    let readiness_top = if compact {
        copy_top + copy_height + 5.0
    } else {
        rect.top() + 8.0
    };
    let readiness_content_left = if compact {
        readiness_left
    } else {
        readiness_left + 12.0
    };
    painter.circle_filled(
        egui::pos2(readiness_content_left + 2.5, readiness_top + 5.0),
        2.5,
        if blocker.is_some() {
            t.color.err
        } else if availability_label(kind) == "Production" {
            t.color.ok
        } else {
            t.color.warn
        },
    );
    let readiness_text_left = readiness_content_left + 12.0;
    let readiness_text_width = (rect.right() - 12.0 - readiness_text_left).max(1.0);
    let action_galley = ui.painter().layout(
        disposition.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if enabled {
            t.color.text
        } else {
            t.color.text_dim
        },
        readiness_text_width,
    );
    let action_height = action_galley.size().y;
    painter.galley(
        egui::pos2(readiness_text_left, readiness_top),
        action_galley,
        if enabled {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if let Some(readiness) = readiness {
        let detail_galley = ui.painter().layout(
            readiness.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if blocker.is_some() {
                t.color.err
            } else {
                t.color.text_faint
            },
            readiness_text_width,
        );
        painter.galley(
            egui::pos2(readiness_text_left, readiness_top + action_height + 2.0),
            detail_galley,
            if blocker.is_some() {
                t.color.err
            } else {
                t.color.text_faint
            },
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    if let Some(reason) = blocker {
        response.on_hover_text(reason).clicked() && enabled
    } else {
        response
            .on_hover_text(format!("Add {} analysis instance", kind.label()))
            .clicked()
            && enabled
    }
}

pub(super) const fn analysis_catalog_readiness(kind: AnalysisKind) -> Option<&'static str> {
    if let Some(reason) = kind.execution_blocker() {
        Some(reason)
    } else {
        match kind.availability() {
            crate::simulation::plan::AnalysisAvailability::Production => None,
            crate::simulation::plan::AnalysisAvailability::Preview => {
                Some("Preview engine · non-sign-off")
            }
        }
    }
}

pub(super) fn filtered_catalog_kinds(query: &str) -> Vec<AnalysisKind> {
    let query = query.trim().to_ascii_lowercase();
    AnalysisKind::MANIFEST_ORDER
        .into_iter()
        .filter(|kind| {
            query.is_empty()
                || format!(
                    "{} {} {} {} {} {} {} {} {}",
                    kind.stable_id(),
                    kind.code(),
                    kind.glyph(),
                    kind.label(),
                    kind.detail(),
                    analysis_catalog_group(*kind),
                    kind.category().detail,
                    availability_label(*kind),
                    kind.execution_blocker().unwrap_or_default(),
                )
                .to_ascii_lowercase()
                .contains(&query)
        })
        .collect()
}

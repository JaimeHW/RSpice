//! Choosing an analysis to add: the catalog window and its search.
//!
//! The catalog lists every analysis kind the plan could take, and each row
//! states its disposition against the current stack rather than being silently
//! hidden — an engineer looking for an analysis that is already configured, or
//! not applicable here, gets told which, not an empty list.

use super::*;

/// Draw the catalogue and report the kind it chose, if any.
///
/// The choice comes back as a value rather than as a stack action written
/// through an out-parameter: adding an instance is the only thing this window
/// can ask for, and the rail that used to collect its action is no longer the
/// caller. The frame is, on whichever workspace the reader is standing on.
///
/// Whether it is open at all is the host's question, not this function's — the
/// host has to resolve the plan before it can lend the rows, and paying for
/// that on every frame of every workspace to then draw nothing is exactly the
/// cost the guard exists to avoid.
///
/// It takes the setup state rather than the application because the four
/// palette fields are the whole of what it owns. What is done with the kind it
/// returns is the host's business, and the host is the only thing here that
/// needs to reach the plan.
pub(super) fn analysis_catalog_window(
    ctx: &egui::Context,
    setup: &mut SimSetupState,
    rows: &[AnalysisStackRow],
) -> Option<AnalysisKind> {
    let mut query = setup.palette_query.clone();
    let mut active = setup.palette_active;
    let mut chosen = None;
    let mut request_close = false;
    let scroll_to_active = setup.palette_scroll_to_active;
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
            // How many columns there are is the body's question, not the
            // window's. The catalogue's surface is inset from the viewport and
            // its rows give up more to the scrollbar, so a count taken from
            // the viewport is taken from a width nothing is drawn in: at 1199
            // it read one column and left the room for a second one standing
            // empty. It is asked once, here, because the travel below and the
            // rows it moves through have to agree about the shape of the grid
            // within the same frame.
            let catalog_columns = analysis_catalog_column_count(analysis_catalog_row_space(ui));
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
                // Travel is over the grid the rows are drawn in, not over the
                // flat list behind it. At two columns a step of one is a step
                // sideways, so Down used to leave the reader one cell along
                // the same row and Up used to undo it; nothing moved the
                // selection across a column at all.
                for (key, step) in [
                    (egui::Key::ArrowUp, GridStep::Up),
                    (egui::Key::ArrowDown, GridStep::Down),
                    (egui::Key::ArrowLeft, GridStep::Left),
                    (egui::Key::ArrowRight, GridStep::Right),
                ] {
                    if ui.input(|input| input.key_pressed(key)) {
                        active = catalog_grid_step(&filtered, catalog_columns, active, step);
                    }
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

    if choice == DialogChoice::Cancelled || chosen.is_some() {
        request_close = true;
    }
    if request_close {
        setup.palette_open = false;
    }
    setup.palette_query = query;
    setup.palette_active = active;
    setup.palette_scroll_to_active = false;
    chosen
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

/// The width the catalogue's rows are laid out in, inside the body `ui`.
///
/// The rows sit in a vertical scroll area, and this theme's scrollbar is a
/// solid one: it takes its width out of the space the rows get rather than
/// floating over them. Subtracting it on every frame — including the ones
/// where the bar has not appeared yet, or is animating in — is what keeps the
/// count still. Measured against a width the bar is halfway through taking
/// away, the layout would flip columns mid-animation.
fn analysis_catalog_row_space(ui: &Ui) -> f32 {
    (ui.available_width() - ui.spacing().scroll.allocated_width()).max(0.0)
}

/// The narrowest one catalogue column may be.
///
/// Two floors bear on it, and the wider one decides.
///
/// The row's own blocks are the first. A wide row is laid out as `code gutter
/// (70) | 12 | copy | 12 | readiness ([`ANALYSIS_CATALOG_READINESS_WIDTH`],
/// 142) | 12`, and [`analysis_catalog_row`] refuses to push the readiness
/// block left of `copy_left + 96`, which leaves the copy 84 points of text.
/// Those terms are 70 + 12 + 84 + 12 + 142 + 12 = 332 points of row.
///
/// The width at which the row abandons that layout is the second, and it is
/// the wider: at or below [`TITLE_ACTION_STACK_BREAKPOINT`] both
/// [`analysis_catalog_row_height`] and [`analysis_catalog_row`] stack the
/// readiness block under the copy instead. Two columns that each draw a
/// stacked row spend the width the split was meant to buy and hand back a
/// taller list, so the second column is worth taking only when both columns
/// clear that breakpoint.
pub(super) const ANALYSIS_CATALOG_COLUMN_MIN_WIDTH: f32 = TITLE_ACTION_STACK_BREAKPOINT;

/// How many columns of rows fit in `row_space` points.
///
/// The argument is the width the rows themselves are given — see
/// [`analysis_catalog_row_space`] — and not the viewport's, which is wider
/// than that by the dialog's inset, its border and the scrollbar: 39 points
/// of a window whose catalogue turns over at 1160 of them.
///
/// Strictly wider than twice the minimum, because the row's own stacking test
/// is `row_width <= TITLE_ACTION_STACK_BREAKPOINT`: a column drawn at exactly
/// the minimum is already the stacked layout.
pub(super) const fn analysis_catalog_column_count(row_space: f32) -> usize {
    if row_space > 2.0 * ANALYSIS_CATALOG_COLUMN_MIN_WIDTH {
        2
    } else {
        1
    }
}

/// Which way a keystroke moves the catalogue's selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GridStep {
    Up,
    Down,
    Left,
    Right,
}

/// The cells the catalogue draws, row by row, as indices into `filtered`.
///
/// [`analysis_catalog_group_rows`] chunks each category's members into rows of
/// `columns`, so a category with an odd count leaves a short row at its end
/// and the next category starts a fresh one. Keyboard travel has to be shaped
/// against the same chunking, or a keystroke lands somewhere the reader is not
/// looking.
fn analysis_catalog_grid(filtered: &[AnalysisKind], columns: usize) -> Vec<Vec<usize>> {
    let columns = columns.max(1);
    let mut grid = Vec::new();
    for group in ANALYSIS_CATEGORY_ORDER {
        let members = filtered
            .iter()
            .enumerate()
            .filter(|(_, kind)| analysis_catalog_group(**kind) == group)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        for chunk in members.chunks(columns) {
            grid.push(chunk.to_vec());
        }
    }
    grid
}

/// Where `direction` takes the selection from `active`.
///
/// Up and Down hold the column and change the row, crossing group boundaries
/// into whichever cell of the next row is nearest: a short row — the tail of a
/// category with an odd count — is entered at its last cell rather than
/// skipped. Left and Right move within the row and do not wrap, so the ends of
/// a row are ends rather than a way into the neighbouring category. At one
/// column every row holds a single cell, which makes Left and Right no-ops and
/// Down the flat step it has always been.
pub(super) fn catalog_grid_step(
    filtered: &[AnalysisKind],
    columns: usize,
    active: usize,
    direction: GridStep,
) -> usize {
    if filtered.is_empty() {
        return 0;
    }
    let active = active.min(filtered.len() - 1);
    let grid = analysis_catalog_grid(filtered, columns);
    let Some((row, column)) = grid.iter().enumerate().find_map(|(row, cells)| {
        cells
            .iter()
            .position(|cell| *cell == active)
            .map(|column| (row, column))
    }) else {
        return active;
    };
    let vertical = |target: Option<usize>| {
        target
            .and_then(|target| grid.get(target))
            .and_then(|cells| cells.get(column).or_else(|| cells.last()))
            .copied()
            .unwrap_or(active)
    };
    match direction {
        GridStep::Up => vertical(row.checked_sub(1)),
        GridStep::Down => vertical(Some(row + 1)),
        GridStep::Left => column
            .checked_sub(1)
            .and_then(|column| grid[row].get(column))
            .copied()
            .unwrap_or(active),
        GridStep::Right => grid[row].get(column + 1).copied().unwrap_or(active),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A five-member category ahead of a three-member one.
    ///
    /// At two columns that is every shape the catalogue's chunking can make:
    /// two full rows, a short row at the end of a group, the group boundary
    /// after it, and a short row at the end of the list.
    const TWO_GROUPS: [AnalysisKind; 8] = [
        AnalysisKind::OperatingPoint,
        AnalysisKind::Transient,
        AnalysisKind::Ac,
        AnalysisKind::DcSweep,
        AnalysisKind::Noise,
        AnalysisKind::Qpac,
        AnalysisKind::Qpnoise,
        AnalysisKind::Qpxf,
    ];

    /// The fixture is only worth its assertions if it still has that shape.
    ///
    /// Both halves are whole categories, so a kind moved between categories
    /// upstream would quietly turn the five-and-three into something else and
    /// the travel table below would be measuring a grid nobody draws.
    #[test]
    fn the_catalogue_travel_fixture_is_a_group_of_five_before_a_group_of_three() {
        let groups = TWO_GROUPS.map(analysis_catalog_group);
        assert_eq!(groups[..5], ["Core analyses"; 5]);
        assert_eq!(groups[5..], ["Quasi-periodic small-signal"; 3]);
        assert_eq!(
            analysis_catalog_grid(&TWO_GROUPS, 2),
            vec![vec![0, 1], vec![2, 3], vec![4], vec![5, 6], vec![7]],
            "the grid is chunked per category, not across the flat list"
        );
    }

    /// An arrow key moves the selection by a row, not by a cell.
    ///
    /// Stepping the flat index by one is a step sideways in a two-column
    /// layout: Down left the reader on the same row and Up put them back, and
    /// nothing crossed a column at all. Every cell of the fixture is stepped
    /// in every direction here, because the interesting cases are the ones at
    /// the edges — the short row `[4]` that a step from either column of the
    /// row above has to land in, the group boundary under it, and the two ends
    /// of a row, which are ends rather than a way into the next category.
    #[test]
    fn catalogue_arrow_travel_moves_by_a_row_and_clamps_into_short_rows() {
        /// `(cell, [up, down, left, right])`.
        const TRAVEL: [(usize, [usize; 4]); 8] = [
            (0, [0, 2, 0, 1]),
            (1, [1, 3, 0, 1]),
            (2, [0, 4, 2, 3]),
            (3, [1, 4, 2, 3]),
            (4, [2, 5, 4, 4]),
            (5, [4, 7, 5, 6]),
            (6, [4, 7, 5, 6]),
            (7, [5, 7, 7, 7]),
        ];

        for (cell, expected) in TRAVEL {
            for (direction, expected) in [
                (GridStep::Up, expected[0]),
                (GridStep::Down, expected[1]),
                (GridStep::Left, expected[2]),
                (GridStep::Right, expected[3]),
            ] {
                assert_eq!(
                    catalog_grid_step(&TWO_GROUPS, 2, cell, direction),
                    expected,
                    "{direction:?} from cell {cell}"
                );
            }
        }
    }

    /// At one column the grid is the list, and travel is the flat step.
    ///
    /// Which is a claim about the manifest as much as about the helper: the
    /// rows are drawn category by category, so a Down that is `+1` everywhere
    /// — across group boundaries included — is only true while
    /// `MANIFEST_ORDER` runs contiguously in `ANALYSIS_CATEGORY_ORDER`. Left
    /// and Right have nowhere to go in a single-cell row and say so.
    #[test]
    fn single_column_catalogue_travel_is_the_flat_order() {
        let all = filtered_catalog_kinds("");
        assert_eq!(all.len(), AnalysisKind::MANIFEST_ORDER.len());
        let last = all.len() - 1;
        for cell in 0..all.len() {
            assert_eq!(
                catalog_grid_step(&all, 1, cell, GridStep::Down),
                (cell + 1).min(last),
                "down from {cell}"
            );
            assert_eq!(
                catalog_grid_step(&all, 1, cell, GridStep::Up),
                cell.saturating_sub(1),
                "up from {cell}"
            );
            for direction in [GridStep::Left, GridStep::Right] {
                assert_eq!(
                    catalog_grid_step(&all, 1, cell, direction),
                    cell,
                    "{direction:?} from {cell}"
                );
            }
        }
    }

    /// Nothing to step through, and nothing to step to.
    #[test]
    fn catalogue_travel_over_an_empty_search_result_stays_at_the_top() {
        for direction in [
            GridStep::Up,
            GridStep::Down,
            GridStep::Left,
            GridStep::Right,
        ] {
            assert_eq!(catalog_grid_step(&[], 2, 7, direction), 0);
        }
    }

    /// The second column is taken on the width the rows are given.
    ///
    /// Either side of `2 × ANALYSIS_CATALOG_COLUMN_MIN_WIDTH`, and at the
    /// bound itself, where a column drawn at exactly the minimum is already
    /// the stacked row the split was meant to avoid.
    #[test]
    fn the_catalogue_takes_a_second_column_only_when_both_columns_clear_the_row_minimum() {
        let bound = 2.0 * ANALYSIS_CATALOG_COLUMN_MIN_WIDTH;
        assert_eq!(ANALYSIS_CATALOG_COLUMN_MIN_WIDTH, 560.0);
        assert_eq!(analysis_catalog_column_count(bound - 1.0), 1);
        assert_eq!(analysis_catalog_column_count(bound), 1);
        assert_eq!(analysis_catalog_column_count(bound + 1.0), 2);
    }
}

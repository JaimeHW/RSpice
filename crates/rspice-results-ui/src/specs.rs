//! Specification result table and inspector over retained runs and frozen requirements.

use crate::presentation::well_hint;
use egui::Ui;
use rspice_app_types::product::{self, DatasetId};
use rspice_results::{
    analysis_result::AnalysisResult,
    run::{SimulationRun, SimulationRunLifecycle},
    run_receipt::PreparedRunReceipt,
    specification::{
        SpecEntry,
        report::{
            SpecResultRow, SpecResultStatus, resolved_specifications, result_rows, summarize_rows,
        },
    },
    waveform::RetainedWaveform,
};
use rspice_ui_kit::{
    plot::fmt_si,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{StatusMark, measurement_table, paint_status_mark, section_header},
};

/// The width each column wants, and the floor it may not be squeezed below.
///
/// A pair per column rather than one number, because the two were the same
/// number and the table was therefore a fixed 1042 points wide however narrow
/// the pane was. Below that width the last column — STATUS, which is the
/// verdict the whole table exists to state — was simply cut off by the pane
/// edge, at 1600 as well as at 1000, and a horizontal scroll bar under a table
/// is not where a reader looks for a pass/fail.
///
/// The floors are what each column can still be read at: a measurement name
/// and an expression elide legibly, a value and a margin are right-aligned
/// numbers, and the status word does not elide at all — so it keeps its width
/// and the prose columns give theirs up.
const SPEC_COLUMNS: [(f32, f32); 7] = [
    (160.0, 104.0),
    (200.0, 112.0),
    (116.0, 84.0),
    (150.0, 96.0),
    (140.0, 88.0),
    (168.0, 96.0),
    (92.0, 92.0),
];
const SPEC_TABLE_GUTTER: f32 = 16.0;

fn lifecycle_label(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy authority",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "streaming",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed => "immutable",
        SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "aborted",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

/// The narrowest the table can be laid out at without cutting a column off.
fn table_minimum_width() -> f32 {
    SPEC_COLUMNS.iter().map(|(_, floor)| floor).sum::<f32>() + SPEC_TABLE_GUTTER
}

/// The width the table lays itself out at when nothing squeezes it.
fn table_width() -> f32 {
    SPEC_COLUMNS.iter().map(|(want, _)| want).sum::<f32>() + SPEC_TABLE_GUTTER
}

/// What each column gets in a table `content_width` wide.
///
/// Above the preferred total every column has what it wants. Below it, the
/// shortfall is taken from the columns in proportion to how much each has to
/// give — its preferred width less its floor — so the columns that elide
/// legibly give up the room and the verdict column keeps all of it. Below the
/// floors nothing more can be taken and the caller's horizontal scroll is what
/// is left.
fn spec_columns(content_width: f32) -> [f32; 7] {
    let preferred = table_width();
    let mut widths = SPEC_COLUMNS.map(|(want, _)| want);
    if content_width >= preferred {
        return widths;
    }
    let shortfall = preferred - content_width.max(table_minimum_width());
    let give: f32 = SPEC_COLUMNS.iter().map(|(want, floor)| want - floor).sum();
    if shortfall <= 0.0 || give <= 0.0 {
        return widths;
    }
    for (width, (want, floor)) in widths.iter_mut().zip(SPEC_COLUMNS) {
        *width = want - shortfall * (want - floor) / give;
    }
    widths
}

fn column_rect(row: egui::Rect, widths: &[f32; 7], index: usize) -> egui::Rect {
    let left = row.left() + widths[..index].iter().sum::<f32>();
    egui::Rect::from_min_size(
        egui::pos2(left, row.top()),
        egui::vec2(widths[index], row.height()),
    )
}

fn spec_table_row_height(control_height: f32) -> f32 {
    control_height.max(28.0)
}

fn paint_clipped_table_text(
    ui: &Ui,
    clip_rect: egui::Rect,
    align: egui::Align2,
    text: impl ToString,
    font: egui::FontId,
    color: egui::Color32,
) {
    if clip_rect.width() <= 0.0 || clip_rect.height() <= 0.0 {
        return;
    }
    let position = if align == egui::Align2::RIGHT_CENTER {
        clip_rect.right_center()
    } else {
        clip_rect.left_center()
    };
    ui.painter()
        .with_clip_rect(clip_rect)
        .text(position, align, text, font, color);
}

fn value_text(row: &SpecResultRow) -> String {
    row.value
        .map_or_else(|| "—".to_owned(), |value| fmt_si(value, &row.unit, 4))
}

fn margin_text(row: &SpecResultRow) -> String {
    row.margin.map_or_else(
        || {
            if row.status == SpecResultStatus::Unbound {
                "unbound".to_owned()
            } else {
                "—".to_owned()
            }
        },
        |margin| fmt_si(margin, &row.unit, 4),
    )
}

fn row_accessibility_label(row: &SpecResultRow) -> String {
    format!(
        "Measurement {}; expression {}; value {}; limit {}; margin {}; worst corner {}; status {}; {}",
        row.measurement,
        if row.expression.is_empty() {
            "not retained"
        } else {
            &row.expression
        },
        value_text(row),
        row.limit,
        margin_text(row),
        row.worst_corner.as_deref().unwrap_or("not available"),
        row.status.label(),
        row.detail
    )
}

fn paint_table_header(ui: &mut Ui, content_width: f32, row_height: f32) {
    let widths = spec_columns(content_width);
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (header, response) =
        ui.allocate_exact_size(egui::vec2(content_width, row_height), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            "Specification result columns: measurement, expression, value, spec, margin, worst corner, status",
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label("Specification result column headers");
    });
    ui.painter().rect_filled(header, 0.0, c.bg_elevated);
    ui.painter().hline(
        header.x_range(),
        header.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border_strong),
    );
    for (index, label) in [
        "MEASUREMENT",
        "EXPRESSION",
        "VALUE",
        "SPEC",
        "MARGIN",
        "WORST CORNER",
        "STATUS",
    ]
    .iter()
    .enumerate()
    {
        let cell = column_rect(header, &widths, index);
        let cell_response = ui.interact(cell, response.id.with(index), egui::Sense::hover());
        cell_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), *label)
        });
        ui.ctx().accesskit_node_builder(cell_response.id, |node| {
            node.set_role(egui::accesskit::Role::ColumnHeader);
            node.set_label(*label);
        });
        paint_clipped_table_text(
            ui,
            cell.shrink2(egui::vec2(9.0, 0.0)),
            if matches!(index, 2 | 4) {
                egui::Align2::RIGHT_CENTER
            } else {
                egui::Align2::LEFT_CENTER
            },
            label,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }
}

/// The row a hop last scrolled this table to, and the dataset it was in.
///
/// The dataset is half the key. The memo held the measurement name alone, so
/// selecting a different run while the same limit stayed carried left the
/// memo satisfied and the scroll request suppressed: the marked row sat below
/// the fold of a table the reader had no reason to think had scrolled at all.
#[derive(Clone, PartialEq, Eq)]
struct CarriedRowArrival {
    dataset: Option<DatasetId>,
    measurement: String,
}

fn paint_result_row(
    ui: &mut Ui,
    row: &SpecResultRow,
    row_index: usize,
    row_size: egui::Vec2,
    max_margin: f64,
    carried: bool,
    scroll_into_view: bool,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let widths = spec_columns(row_size.x);
    let (rect, response) = ui.allocate_exact_size(row_size, egui::Sense::hover());
    let accessible_label = row_accessibility_label(row);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(accessible_label);
        node.set_selected(carried);
    });
    if carried {
        // A hop from the studio names one limit; the table has to show which
        // one arrived, or the reader is left to find it by eye in a table the
        // navigation already resolved.
        ui.painter().rect_filled(rect, 0.0, c.bg_active);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            c.accent,
        );
        // Scrolled to once per arrival, not every frame: the carried selection
        // outlives the hop, and a row that re-centres itself forever would
        // take the scroll bar away from the reader.
        if scroll_into_view {
            response.scroll_to_me(Some(egui::Align::Center));
        }
    } else if row_index % 2 == 1 {
        ui.painter()
            .rect_filled(rect, 0.0, c.bg_panel.gamma_multiply(0.35));
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border.gamma_multiply(0.65)),
    );

    let expression = if row.expression.is_empty() {
        "—"
    } else {
        &row.expression
    };
    let value = value_text(row);
    let margin = margin_text(row);
    let cell_values = [
        row.measurement.as_str(),
        expression,
        value.as_str(),
        row.limit.as_str(),
        margin.as_str(),
        row.worst_corner.as_deref().unwrap_or("—"),
        row.status.label(),
    ];
    for (index, label) in cell_values.iter().enumerate() {
        if index == 5 && row.source_analysis_index.is_some() {
            continue;
        }
        let cell = column_rect(rect, &widths, index);
        let cell_response = ui.interact(
            cell,
            response.id.with(("cell", index)),
            egui::Sense::hover(),
        );
        cell_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), *label)
        });
        ui.ctx().accesskit_node_builder(cell_response.id, |node| {
            node.set_role(egui::accesskit::Role::Cell);
            node.set_label(*label);
        });
    }

    paint_clipped_table_text(
        ui,
        column_rect(rect, &widths, 0).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        &row.measurement,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        c.text,
    );
    paint_clipped_table_text(
        ui,
        column_rect(rect, &widths, 1).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        expression,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if row.expression.is_empty() {
            c.text_faint
        } else {
            c.text_dim
        },
    );
    paint_clipped_table_text(
        ui,
        column_rect(rect, &widths, 2).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::RIGHT_CENTER,
        &value,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        if row.status == SpecResultStatus::Fail {
            c.err
        } else {
            c.text
        },
    );
    paint_clipped_table_text(
        ui,
        column_rect(rect, &widths, 3).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        &row.limit,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_dim,
    );

    let margin_cell = column_rect(rect, &widths, 4).shrink2(egui::vec2(9.0, 5.0));
    if let Some(value) = row.margin {
        let fraction = (value.abs() / max_margin).clamp(0.04, 1.0) as f32;
        let color = if value >= 0.0 { c.ok } else { c.err };
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(margin_cell.left(), margin_cell.bottom() - 3.0),
                egui::vec2(margin_cell.width() * fraction, 3.0),
            ),
            0.0,
            color,
        );
        paint_clipped_table_text(
            ui,
            margin_cell,
            egui::Align2::RIGHT_CENTER,
            &margin,
            theme::mono(tokens::FS_1, FontWeight::Regular),
            color,
        );
    } else {
        paint_clipped_table_text(
            ui,
            margin_cell,
            egui::Align2::RIGHT_CENTER,
            &margin,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    let corner_cell = column_rect(rect, &widths, 5).shrink2(egui::vec2(6.0, 2.0));
    let focused = if let Some(analysis_index) = row.source_analysis_index {
        let label = row.worst_corner.as_deref().unwrap_or("Open source");
        let button = ui.put(
            corner_cell,
            egui::Button::new(
                egui::RichText::new(label).font(theme::mono(tokens::FS_0, FontWeight::Regular)),
            )
            .frame(false),
        );
        button.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!(
                    "Open retained source analysis for {}{}",
                    row.measurement,
                    row.worst_corner
                        .as_ref()
                        .map_or_else(String::new, |corner| format!(" at {corner}"))
                ),
            )
        });
        button.clicked().then_some(analysis_index)
    } else {
        paint_clipped_table_text(
            ui,
            corner_cell,
            egui::Align2::LEFT_CENTER,
            "—",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
        None
    };

    let status_cell = column_rect(rect, &widths, 6).shrink2(egui::vec2(9.0, 0.0));
    let (status_color, mark) = match row.status {
        SpecResultStatus::Pass => (c.ok, StatusMark::Success),
        SpecResultStatus::Fail | SpecResultStatus::Invalid => (c.err, StatusMark::Failure),
        SpecResultStatus::Unbound | SpecResultStatus::Missing => {
            (c.text_faint, StatusMark::Neutral)
        }
    };
    paint_status_mark(
        &ui.painter().with_clip_rect(status_cell),
        egui::Rect::from_center_size(
            egui::pos2(status_cell.left() + 5.0, status_cell.center().y),
            egui::Vec2::splat(8.0),
        ),
        mark,
        status_color,
    );
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(status_cell.left() + 14.0, status_cell.top()),
            status_cell.right_bottom(),
        ),
        egui::Align2::LEFT_CENTER,
        row.status.label(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        status_color,
    );
    focused
}

fn show_table_shell(
    ui: &mut Ui,
    rows: &[SpecResultRow],
    dataset: Option<DatasetId>,
    salt: impl std::hash::Hash + Copy + std::fmt::Debug,
    empty_message: Option<&str>,
    carried: Option<&str>,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let row_height = spec_table_row_height(t.metrics.ctl_h);
    // Which row the table has to bring into view, once per arrival: a limit
    // carried into a dataset it has not yet been shown in.
    let arrival = carried
        .filter(|measurement| {
            rows.iter()
                .any(|row| row.measurement.eq_ignore_ascii_case(measurement))
        })
        .map(|measurement| CarriedRowArrival {
            dataset,
            measurement: measurement.to_ascii_lowercase(),
        });
    let memo = egui::Id::new("rspice.results.specification-table.scrolled-to");
    let is_new_arrival = arrival.as_ref().is_some_and(|arrival| {
        ui.ctx()
            .data(|data| data.get_temp::<CarriedRowArrival>(memo))
            .as_ref()
            != Some(arrival)
    });
    if is_new_arrival && let Some(arrival) = arrival {
        ui.ctx().data_mut(|data| data.insert_temp(memo, arrival));
    }
    // The pane's width, floored at what the columns can still be read in
    // rather than at what they would prefer. Floored at the preferred width,
    // the table was a fixed 1042 points and the pane simply cut the last
    // column — the verdict — off its right edge at every width below that.
    let content_width = ui.available_width().max(table_minimum_width());
    let max_margin = rows
        .iter()
        .filter_map(|row| row.margin)
        .map(f64::abs)
        .fold(0.0_f64, f64::max)
        .max(f64::EPSILON);
    let mut focused = None;
    let table = egui::Frame::NONE.show(ui, |ui| {
        egui::ScrollArea::horizontal()
            .id_salt(("rspice.results.specification-table-x", salt))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_min_width(content_width);
                paint_table_header(ui, content_width, row_height);
                egui::ScrollArea::vertical()
                    .id_salt(("rspice.results.specification-table-y", salt))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if rows.is_empty() {
                            let height = ui.available_height().max(120.0);
                            ui.allocate_ui_with_layout(
                                egui::vec2(content_width, height),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.set_min_height(height);
                                    well_hint(
                                        ui,
                                        empty_message.unwrap_or("No specification evidence"),
                                    );
                                },
                            );
                        } else {
                            for (index, row) in rows.iter().enumerate() {
                                // Matched case-insensitively for the same
                                // reason the studio's own selection is: a
                                // measurement name is one identity however it
                                // was typed.
                                let selected = carried.is_some_and(|measurement| {
                                    measurement.eq_ignore_ascii_case(&row.measurement)
                                });
                                focused = paint_result_row(
                                    ui,
                                    row,
                                    index,
                                    egui::vec2(content_width, row_height),
                                    max_margin,
                                    selected,
                                    selected && is_new_arrival,
                                )
                                .or(focused);
                            }
                        }
                    });
            });
    });
    table.response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            "Active dataset specification results",
        )
    });
    ui.ctx().accesskit_node_builder(table.response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Active dataset specification results");
    });
    focused
}

fn paint_summary(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    accessible_label: String,
    detail_color: egui::Color32,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 40.0), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border_strong),
    );
    // The band reads left to right, title then verdict, and whatever will not
    // fit is lost from the end. The detail used to be right-aligned into the
    // room the title left, which is an arrangement that elides the *head*: in
    // the 721-point pane the results workspace gives this document at the
    // 1000-point gate, "0 / 2 pass · 2 unavailable · immutable · dataset
    // <uuid>" arrived as "/ 2 pass · …" — the pass count, which is the one
    // thing the band exists to state, cut off by its own left edge. Making the
    // detail shorter is not the fix on its own; a longer verdict would put it
    // straight back, so the layout is what changed.
    let title_font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let title_width = ui
        .painter()
        .layout_no_wrap(title.to_owned(), title_font.clone(), c.text)
        .size()
        .x;
    let split = (rect.left() + 10.0 + title_width + 16.0).min(rect.right() - 10.0);
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(rect.left() + 10.0, rect.top()),
            egui::pos2(split, rect.bottom()),
        ),
        egui::Align2::LEFT_CENTER,
        title,
        title_font,
        c.text,
    );
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(split, rect.top()),
            egui::pos2(rect.right() - 10.0, rect.bottom()),
        ),
        egui::Align2::LEFT_CENTER,
        detail,
        summary_detail_font(),
        detail_color,
    );
}

/// The face the band's verdict is set in.
///
/// Named because a pin that asks whether the line fits has to measure the line
/// that is actually painted; a font spelled twice is a pin that can pass over a
/// band that clips.
fn summary_detail_font() -> egui::FontId {
    theme::mono(tokens::FS_0, FontWeight::Regular)
}

/// The verdict the band states for one retained run.
///
/// The dataset identity is elided to its first eight characters, the way every
/// other surface that has to fit one beside prose elides it. That is not on its
/// own the fix for the band clipping — the layout above is — but it is the
/// difference between a reader seeing the whole verdict and seeing most of it.
fn summary_detail(
    passing: usize,
    bounded: usize,
    unavailable: usize,
    lifecycle: SimulationRunLifecycle,
    dataset: DatasetId,
) -> String {
    let unavailable_text = if unavailable > 0 {
        format!(" \u{b7} {unavailable} unavailable")
    } else {
        String::new()
    };
    format!(
        "{passing} / {bounded} pass{unavailable_text} \u{b7} {} \u{b7} dataset {}",
        lifecycle_label(lifecycle),
        product::short_identity(dataset),
    )
}

/// Draw the active retained result table and return a requested source analysis.
pub fn show<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    ui: &mut Ui,
    run: Option<&SimulationRun<A>>,
    workspace_specs: &[SpecEntry],
    carried: Option<&str>,
) -> Option<usize> {
    let c = Tokens::get(ui.ctx()).color;
    let Some(run) = run else {
        let bounded = workspace_specs
            .iter()
            .filter(|spec| spec.min.is_some() || spec.max.is_some())
            .count();
        paint_summary(
            ui,
            "Specifications",
            &format!("0 / {bounded} pass · no active dataset"),
            format!(
                "Specifications: zero of {bounded} bounded measurements pass; no active dataset"
            ),
            c.warn,
        );
        show_table_shell(
            ui,
            &[],
            None,
            "no-active-dataset",
            Some(
                "No active dataset — select a retained run or run the simulation to evaluate specifications",
            ),
            None,
        );
        return None;
    };

    let run_id = run.id;
    let dataset_id = run.dataset_id;
    let lifecycle = run.lifecycle;
    let specs = resolved_specifications(run, workspace_specs);
    let rows = result_rows(run, &specs);
    let summary = summarize_rows(&rows);
    let passing = summary.passing;
    let bounded = summary.bounded;
    let unavailable = summary.unavailable;
    paint_summary(
        ui,
        &format!("Specifications · Run #{run_id}"),
        &summary_detail(passing, bounded, unavailable, lifecycle, dataset_id),
        format!(
            "Specifications for run {run_id}, dataset {dataset_id}, {}: {passing} of {bounded} bounded measurements pass; {unavailable} unavailable",
            lifecycle_label(lifecycle)
        ),
        if lifecycle == SimulationRunLifecycle::Completed && unavailable == 0 {
            c.text_dim
        } else {
            c.warn
        },
    );
    // The studio's Requirements page and this table read one fact — which
    // limit is being looked at — rather than each keeping a selection of its
    // own, so a hop from either side arrives on the row the other named.
    show_table_shell(
        ui,
        &rows,
        Some(dataset_id),
        dataset_id,
        Some(
            "No measurements — add .MEAS statements, run the simulation, then bind requirement limits",
        ),
        carried,
    )
}

/// Inspect the same frozen requirements and retained evidence as the table.
pub fn right_panel<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    ui: &mut Ui,
    run: Option<&SimulationRun<A>>,
    workspace_specs: &[SpecEntry],
) {
    let Some(run) = run else {
        let specs = workspace_specs;
        if specs.is_empty() {
            section_header(ui, "Specs", None);
            measurement_table(ui, &[("Bounds", "none defined")]);
            return;
        }
        section_header(ui, "Specs · active dataset", None);
        let bounded = specs
            .iter()
            .filter(|spec| spec.min.is_some() || spec.max.is_some())
            .count()
            .to_string();
        let specs_n = specs.len().to_string();
        measurement_table(
            ui,
            &[
                ("Dataset", "none selected"),
                ("Specs", specs_n.as_str()),
                ("Bounded", bounded.as_str()),
                ("Unavailable", bounded.as_str()),
            ],
        );
        return;
    };

    // Results are immutable evidence. The side inspector must therefore use
    // the same frozen receipt requirements as the document, even after the
    // active plan is edited or switched.
    let specs = resolved_specifications(run, workspace_specs);
    if specs.is_empty() {
        section_header(ui, "Specs · active dataset", None);
        measurement_table(
            ui,
            &[("Bounds", "none were frozen into this dataset's run receipt")],
        );
        return;
    }
    let rows = result_rows(run, &specs);
    let summary = summarize_rows(&rows);
    let bounded = summary.bounded;
    let passing = summary.passing;
    let failures = summary.failures;
    let unavailable = summary.unavailable;
    let worst = rows
        .iter()
        .filter(|row| row.is_bounded && row.status == SpecResultStatus::Fail)
        .filter_map(|row| row.margin.map(|margin| (row, margin)))
        .min_by(|(_, left), (_, right)| left.total_cmp(right));

    section_header(ui, "Specs · active dataset", None);
    let specs_n = specs.len().to_string();
    // The results panel is where a spec table gets read into a sign-off
    // package, so it is where an unqualified model has to be visible. It
    // annotates the dataset and changes nothing else: the rows, the margins and
    // the worst violation are all still exactly what the run measured.
    // The stamp names its cause. It printed the verdict alone, which is what a
    // surface says when it knows the answer and not the reason — and the
    // reason is on the receipt, one call away, in the same words Verify and the
    // requirements page use.
    let run_n = run
        .prepared_receipt()
        .and_then(PreparedRunReceipt::sign_off_blocker)
        .map_or_else(
            || format!("run #{}", run.id),
            |blocker| format!("run #{} · NOT SIGN-OFF · {blocker}", run.id),
        );
    let bounded_s = bounded.to_string();
    let passing_s = passing.to_string();
    let fails_s = failures.to_string();
    let unavailable_s = unavailable.to_string();
    measurement_table(
        ui,
        &[
            ("Dataset", run_n.as_str()),
            ("Specs", specs_n.as_str()),
            ("Bounded", bounded_s.as_str()),
            ("Passing", passing_s.as_str()),
            ("Failures", fails_s.as_str()),
            ("Unavailable", unavailable_s.as_str()),
        ],
    );

    if let Some((row, _)) = worst {
        ui.add_space(8.0);
        section_header(ui, "Worst violation", None);
        let value_s = value_text(row);
        let margin_s = margin_text(row);
        let source_s = row.worst_corner.as_deref().unwrap_or("source unavailable");
        measurement_table(
            ui,
            &[
                (row.measurement.as_str(), value_s.as_str()),
                ("Margin", margin_s.as_str()),
                ("Source", source_s),
            ],
        );
    }
}

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    //! Painted text observations shared by specification integration tests.
    pub fn painted_spans(output: &egui::FullOutput) -> Vec<(String, egui::Rect, egui::Rect)> {
        fn walk(
            shape: &egui::epaint::Shape,
            clip: egui::Rect,
            out: &mut Vec<(String, egui::Rect, egui::Rect)>,
        ) {
            match shape {
                egui::epaint::Shape::Text(text) => out.push((
                    text.galley.job.text.clone(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                    clip,
                )),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        walk(shape, clip, out);
                    }
                }
                _ => {}
            }
        }
        let mut spans = Vec::new();
        for shape in &output.shapes {
            walk(&shape.shape, shape.clip_rect, &mut spans);
        }
        spans
    }
}

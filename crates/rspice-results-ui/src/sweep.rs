//! The Sweep sheet: a measured value of every point of a sweep, corner run
//! or temperature step, drawn against a swept quantity.
//!
//! It is ADE's Plot Across Design Points and ADS's value-against-sweep plot:
//! one point per member, the swept quantity on X, the value on Y, and a
//! curve per value of every other swept quantity, told apart by colour,
//! then by dash. The X caption's menu swaps which swept quantity stands on
//! X, as ViVA's Swap Sweep Var does. The points' table stands at the foot,
//! one row per point, exactly its rows tall.

use crate::presentation::{PlotView, table_row_menu, well_hint, with_table_state};
use crate::strip::{LegendChip, LegendGlyph, StripHeader};
use egui::Ui;
use rspice_results::sweep_table::{OutputKind, SweepCurve, SweepTable};
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si_significant};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{Align, Cell, Column, DataTable, LineHeights, SortKey};

/// What the sheet says when the shown analysis is no sweep.
pub const ABSENT_STATE: &str = "No sweep in this analysis — run a .STEP, corner or temperature sweep, \
     then pick one of its points in the data browser";

/// What the sheet says when no swept quantity is a number to stand on X.
const NO_NUMERIC_AXIS: &str =
    "Every swept quantity here is a name, not a number: the table lists each point's value";

/// The reader's choices on the sheet, by name, never by position: a re-run
/// that measures one value fewer must not move the reader onto another.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SweepSheetState {
    /// The value drawn on Y: a `.MEAS` name, or a signal.
    pub output: Option<String>,
    /// The swept quantity on X.
    pub x: Option<String>,
    /// Where a signal is read on each member's own X axis; its record's end
    /// when `None`.
    pub at: Option<f64>,
}

/// The value and the quantity the sheet draws, as positions in the table:
/// the reader's choice where the table still has it, else the first
/// measured value (a `.MEAS` result before a sample, a sample before a
/// signal) against the first numeric quantity.
#[must_use]
pub fn resolve(table: &SweepTable, state: &SweepSheetState) -> (Option<usize>, Option<usize>) {
    let output = state
        .output
        .as_deref()
        .and_then(|name| table.outputs.iter().position(|output| output.name == name))
        .or_else(|| {
            [
                OutputKind::Measurement,
                OutputKind::Sample,
                OutputKind::Signal,
            ]
            .into_iter()
            .find_map(|kind| table.outputs.iter().position(|output| output.kind == kind))
        });
    let x = state
        .x
        .as_deref()
        .and_then(|name| {
            table
                .axes
                .iter()
                .position(|axis| axis.name == name && axis.is_numeric())
        })
        .or_else(|| table.axes.iter().position(|axis| axis.is_numeric()));
    (output, x)
}

/// What a value is called in a select and a column head: its name and,
/// for a signal read at a point, where.
fn output_head(table: &SweepTable, output: usize, at: Option<f64>) -> String {
    let Some(column) = table.outputs.get(output) else {
        return String::new();
    };
    match (column.kind, &table.signal_axis) {
        (OutputKind::Signal, Some((_, unit))) => match at {
            Some(at) => format!("{} at {}", column.name, fmt_si_significant(at, unit, 4)),
            None => format!("{} at end", column.name),
        },
        _ => column.name.clone(),
    }
}

/// Width of the value select: a `.MEAS` name or a signal's.
const OUTPUT_SELECT_WIDTH: f32 = 128.0;
/// Width of the quantity select.
const AXIS_SELECT_WIDTH: f32 = 88.0;
/// Width of the field a signal is read at: `100.0µ` and its unit.
const AT_FIELD_WIDTH: f32 = 72.0;

/// The sheet's controls in its bar: the value drawn, the quantity it is
/// drawn against, and where a signal is read. `false` when the shown
/// analysis is no sweep, so the bar states the sheet's purpose instead.
pub fn domain_bar(ui: &mut Ui, table: Option<&SweepTable>, state: &mut SweepSheetState) -> bool {
    let Some(table) = table else {
        return false;
    };
    let (output, x) = resolve(table, state);
    crate::chrome::bars::bar_label(ui, "Plot");
    let names: Vec<String> = table
        .outputs
        .iter()
        .map(|output| output.name.clone())
        .collect();
    let current = output.map_or("", |output| table.outputs[output].name.as_str());
    if let Some(picked) = rspice_ui_kit::widgets::select(
        ui,
        "rspice.results.sweep-output",
        "Value plotted",
        current,
        &names,
        OUTPUT_SELECT_WIDTH,
    ) {
        state.output = Some(names[picked].clone());
    }
    let numeric: Vec<String> = table.numeric_axes().map(|axis| axis.name.clone()).collect();
    if !numeric.is_empty() {
        ui.add_space(8.0);
        crate::chrome::bars::bar_label(ui, "against");
        let current = x.map_or("", |x| table.axes[x].name.as_str());
        if let Some(picked) = rspice_ui_kit::widgets::select(
            ui,
            "rspice.results.sweep-x",
            "Swept quantity on X",
            current,
            &numeric,
            AXIS_SELECT_WIDTH,
        ) {
            state.x = Some(numeric[picked].clone());
        }
    }
    if let Some(output) = output
        && table.outputs[output].kind == OutputKind::Signal
        && let Some((quantity, unit)) = &table.signal_axis
    {
        ui.add_space(8.0);
        crate::chrome::bars::bar_label(ui, "at");
        let shown = state
            .at
            .map_or_else(String::new, |at| fmt_si_significant(at, unit, 4));
        at_field(
            ui,
            shown,
            &format!(
                "Where on each point's own {quantity} axis the signal is read; its end when empty"
            ),
            &mut |text| {
                let text = text.trim().trim_end_matches(unit.as_str()).trim();
                if text.is_empty() {
                    state.at = None;
                } else if let Ok(value) = rspice_vocabulary::quantity::parse_engineering_value(text)
                    && value.is_finite()
                {
                    state.at = Some(value);
                }
            },
        );
    }
    true
}

/// A field that commits what the reader typed when they leave it.
fn at_field(ui: &mut Ui, shown: String, tooltip: &str, commit: &mut dyn FnMut(&str)) {
    let id = egui::Id::new("rspice.results.sweep-at");
    let draft = ui.data(|data| data.get_temp::<String>(id));
    let drafting = draft.is_some();
    let mut text = draft.unwrap_or(shown);
    let field = ui
        .add(
            egui::TextEdit::singleline(&mut text)
                .id(id.with("field"))
                .desired_width(AT_FIELD_WIDTH)
                .font(rspice_ui_kit::theme::field_mono())
                .hint_text("end"),
        )
        .on_hover_text(tooltip);
    if field.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
    } else if field.lost_focus() || drafting {
        commit(&text);
        ui.data_mut(|data| data.remove::<String>(id));
    }
}

/// What the host resolved for the sheet to draw.
pub struct SweepView<'a> {
    pub table: &'a SweepTable,
    /// Each row's value of the drawn output, read where the reader asked.
    pub values: &'a [Option<f64>],
    pub state: &'a SweepSheetState,
    pub plot_view: PlotView,
    /// Distinguishes this table's traces in the decimation cache.
    pub cache_key: u64,
    /// The reader's displayed significant digits.
    pub significant_digits: usize,
}

/// What the reader did on the sheet this frame.
#[derive(Default)]
pub struct SweepResponse {
    pub view: plot::ViewChange,
    pub fit: bool,
    /// The swept quantity the reader chose to stand on X.
    pub x: Option<String>,
    /// The plot as drawn, for the host's axis fields.
    pub plot: Option<plot::PlotResponse>,
}

/// Share of the sheet the points' table may take before it scrolls.
const TABLE_SHARE: f32 = 0.4;
/// Rows the table keeps however short the sheet.
const TABLE_LEAST_ROWS: f32 = 3.0;

/// Draw the value against the swept quantity, and the points' table at the
/// foot.
pub fn show(
    ui: &mut Ui,
    view: Option<SweepView<'_>>,
    cache: &mut plot::DecimationCache,
) -> SweepResponse {
    let mut response = SweepResponse::default();
    let Some(view) = view else {
        well_hint(ui, ABSENT_STATE);
        return response;
    };
    let table = view.table;
    let (output, x) = resolve(table, view.state);
    let area = ui.available_rect_before_wrap();
    let lines = LineHeights::grid(&Tokens::get(ui.ctx()));
    let rows = table.rows.len().max(1) as f32;
    let cap = ((area.height() * TABLE_SHARE - lines.head) / lines.row)
        .floor()
        .max(TABLE_LEAST_ROWS);
    let table_height = (lines.head + rows.min(cap) * lines.row).min(area.height());
    let drawn = output.zip(x);
    let table_area = if drawn.is_some() {
        egui::Rect::from_min_max(
            egui::pos2(area.left(), (area.bottom() - table_height).max(area.top())),
            area.max,
        )
    } else {
        area
    };
    if let Some((output, x)) = drawn {
        let plot_area =
            egui::Rect::from_min_max(area.min, egui::pos2(area.right(), table_area.top()));
        let mut plot_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("rspice.results.sweep-plot")
                .max_rect(plot_area)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        draw_plot(&mut plot_ui, &view, output, x, cache, &mut response);
    }
    let mut table_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("rspice.results.sweep-points")
            .max_rect(table_area)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    if x.is_none() {
        well_hint(&mut table_ui, NO_NUMERIC_AXIS);
    }
    points_table(&mut table_ui, &view, output, table_area.height());
    ui.advance_cursor_after_rect(area);
    response
}

/// One curve's points in ascending X, the rows with no value left out.
fn curve_points(
    table: &SweepTable,
    values: &[Option<f64>],
    x: usize,
    curve: &SweepCurve,
) -> (Vec<f64>, Vec<f64>) {
    curve
        .rows
        .iter()
        .filter_map(|&row| Some((table.axes[x].number(row)?, (*values.get(row)?)?)))
        .unzip()
}

/// The colour of the curve that holds `level` of the first other quantity.
fn curve_color(ui: &Ui, level: usize) -> egui::Color32 {
    let traces = Tokens::get(ui.ctx()).color.traces;
    traces[level % traces.len()]
}

fn draw_plot(
    ui: &mut Ui,
    view: &SweepView<'_>,
    output: usize,
    x: usize,
    cache: &mut plot::DecimationCache,
    response: &mut SweepResponse,
) {
    let table = view.table;
    let digits = view.significant_digits;
    let axis = &table.axes[x];
    let column = &table.outputs[output];
    let curves = table.curves(&axis.name);
    let points: Vec<(Vec<f64>, Vec<f64>)> = curves
        .iter()
        .map(|curve| curve_points(table, view.values, x, curve))
        .collect();
    let legend: Vec<LegendChip<'_>> = if curves.len() > 1 {
        curves
            .iter()
            .map(|curve| LegendChip {
                name: &curve.label,
                color: curve_color(ui, curve.levels.first().copied().unwrap_or(0)),
                on: true,
                glyph: LegendGlyph::Line,
            })
            .collect()
    } else {
        Vec::new()
    };
    let head = output_head(table, output, view.state.at);
    let count = table.rows.len();
    let subtitle = format!(
        "{head} against {} · {count} point{}",
        axis.name,
        if count == 1 { "" } else { "s" }
    );
    let header = StripHeader::new("SWEEP", &subtitle, &legend)
        .zoomed(view.plot_view.is_zoomed())
        .show(ui);
    response.fit = header.fit_clicked;

    let xs: Vec<f64> = points
        .iter()
        .flat_map(|(xs, _)| xs.iter().copied())
        .collect();
    let ys: Vec<f64> = points
        .iter()
        .flat_map(|(_, ys)| ys.iter().copied())
        .collect();
    if ys.is_empty() {
        well_hint(ui, &format!("No point measured {}", column.name));
        return;
    }
    let extent = |values: &[f64]| {
        values
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), value| {
                (lo.min(*value), hi.max(*value))
            })
    };
    let (x_lo, x_hi) = extent(&xs);
    let (y_lo, y_hi) = extent(&ys);
    // A sweep across decades is read on a logarithmic axis, as a decade
    // `.STEP` is declared.
    let log = x_lo > 0.0 && x_hi / x_lo >= 100.0;
    let x_axis = match view.plot_view.x {
        Some((lo, hi)) if log => Axis::log_decades(lo, hi, &axis.unit),
        Some((lo, hi)) => Axis::linear_with(lo, hi, &axis.unit, 7),
        None if log => Axis::log_decades(x_lo, x_hi, &axis.unit),
        None => {
            let (lo, hi) = plot::padded_fit((x_lo, x_hi));
            Axis::linear_with(lo, hi, &axis.unit, 7)
        }
    }
    .with_label(&axis.name);
    let (y0, y1) = view
        .plot_view
        .y
        .unwrap_or_else(|| plot::padded_fit((y_lo, y_hi)));
    let y_axis = Axis::linear_with(y0, y1, &column.unit, 7).with_label(&column.name);
    let mut spec = PlotSpec::new(
        x_axis,
        if log { XScale::Log10 } else { XScale::Linear },
        y_axis,
    )
    .with_significant_digits(digits)
    .accessible_name("Sweep")
    .accessible_detail(
        "Each point's value against the swept quantity, a curve per value of the others.",
    );
    for (index, (curve, (xs, ys))) in curves.iter().zip(&points).enumerate() {
        let color = curve_color(ui, curve.levels.first().copied().unwrap_or(0));
        let mut trace = Trace::new(xs, ys, color)
            .show_single_point()
            .cache_key(plot::trace_cache_key(view.cache_key, 2 * index));
        if let Some(&dash) = curve.levels.get(1) {
            trace = trace.dash_style(dash);
        }
        if let Some(&marker) = curve.levels.get(2) {
            trace = trace.marker_style(marker);
        }
        spec.traces.push(trace);
        // Each point is a dot where it was measured; the line between two
        // is only the eye's guide.
        spec.traces.push(
            Trace::new(xs, ys, color)
                .draw(plot::DrawMode::Points)
                .width(4.0)
                .cache_key(plot::trace_cache_key(view.cache_key, 2 * index + 1)),
        );
    }
    let readout = |at: f64| {
        curves
            .iter()
            .zip(&points)
            .filter_map(|(curve, (xs, ys))| {
                let nearest = xs
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| (**a - at).abs().total_cmp(&(**b - at).abs()))?
                    .0;
                let name = if curve.label.is_empty() {
                    format!(
                        "{} = {}",
                        axis.name,
                        fmt_si_significant(xs[nearest], &axis.unit, digits)
                    )
                } else {
                    curve.label.clone()
                };
                Some((name, fmt_si_significant(ys[nearest], &column.unit, digits)))
            })
            .collect::<Vec<_>>()
    };
    let drawn = plot::show(ui, &spec, cache, None, Some(&readout));
    response.view = drawn.view;
    if let Some(caption) = drawn.x_label_rect {
        response.x = swap_menu(ui, caption, table, x);
    }
    response.plot = Some(drawn);
}

/// The X caption's menu: which swept quantity the value is drawn against,
/// the others becoming the family's curves.
fn swap_menu(ui: &mut Ui, caption: egui::Rect, table: &SweepTable, x: usize) -> Option<String> {
    let numeric: Vec<&str> = table
        .numeric_axes()
        .map(|axis| axis.name.as_str())
        .collect();
    if numeric.len() < 2 {
        return None;
    }
    let label = ui
        .interact(
            caption.expand(2.0),
            ui.id().with("rspice.results.sweep-x-caption"),
            egui::Sense::click(),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text("Swap sweep variable: choose which swept quantity stands on X");
    label.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("Swap sweep variable, {} on X", table.axes[x].name),
        )
    });
    rspice_ui_kit::theme::paint_focus_ring(ui, &label, caption.expand(2.0));
    if label.hovered()
        || egui::Popup::is_id_open(ui.ctx(), egui::Popup::default_response_id(&label))
    {
        ui.painter().hline(
            caption.x_range(),
            caption.bottom(),
            egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.text_dim),
        );
    }
    egui::Popup::menu(&label)
        .show(|ui| {
            ui.label(egui::RichText::new("Swap sweep variable").weak());
            let mut chosen = None;
            for name in &numeric {
                if ui.radio(table.axes[x].name == *name, *name).clicked() {
                    chosen = Some((*name).to_owned());
                    ui.close();
                }
            }
            chosen
        })
        .and_then(|shown| shown.inner)
}

/// The points' table: a column per swept quantity, as the netlist states
/// it, then a column per measured value, and the drawn signal's value when
/// a signal is drawn.
fn points_table(ui: &mut Ui, view: &SweepView<'_>, output: Option<usize>, height: f32) {
    let table = view.table;
    let digits = view.significant_digits;
    let drawn_signal = output.filter(|&output| table.outputs[output].kind == OutputKind::Signal);
    let listed: Vec<usize> = (0..table.outputs.len())
        .filter(|&index| table.outputs[index].kind != OutputKind::Signal)
        .collect();
    let mut columns: Vec<Column> = table
        .axes
        .iter()
        .map(|axis| Column::name(axis.name.clone(), 56.0))
        .collect();
    // A head names the quantity; its cells carry the value and its unit.
    let value_head = |index: usize| table.outputs[index].name.clone();
    columns.extend(
        listed
            .iter()
            .map(|&index| Column::name(value_head(index), 72.0).align(Align::Right)),
    );
    if let Some(signal) = drawn_signal {
        columns.push(
            Column::name(output_head(table, signal, view.state.at), 96.0).align(Align::Right),
        );
    }
    let number = |value: Option<f64>, unit: &str| match value {
        Some(value) => {
            Cell::number(fmt_si_significant(value, unit, digits)).sort_by(SortKey::Number(value))
        }
        None => Cell::absent(),
    };
    let revision = egui::Id::new((
        "sweep",
        view.cache_key,
        drawn_signal,
        view.state.at.map(f64::to_bits),
        digits,
    ))
    .value();
    let asked = with_table_state(ui, "sweep", |ui, state| {
        DataTable::new("rspice.results.sweep", &columns, table.rows.len())
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(height)
            .show(ui, |row| {
                let mut cells: Vec<Cell> = table
                    .axes
                    .iter()
                    .map(|axis| {
                        let cell = Cell::text(axis.spelled(row));
                        match axis.number(row) {
                            Some(value) => cell.sort_by(SortKey::Number(value)),
                            None => cell,
                        }
                    })
                    .collect();
                cells.extend(listed.iter().map(|&index| {
                    let column = &table.outputs[index];
                    number(column.values.get(row).copied().flatten(), &column.unit)
                }));
                if let Some(signal) = drawn_signal {
                    cells.push(number(
                        view.values.get(row).copied().flatten(),
                        &table.outputs[signal].unit,
                    ));
                }
                cells
            })
            .context_menu
    });
    table_row_menu(ui, "sweep", asked, |ui, row| {
        if ui.button("Copy exact row").clicked() {
            let mut fields: Vec<String> = table.axes.iter().map(|axis| axis.spelled(row)).collect();
            fields.extend(listed.iter().map(|&index| {
                table.outputs[index]
                    .values
                    .get(row)
                    .copied()
                    .flatten()
                    .map_or_else(String::new, |value| format!("{value:.17e}"))
            }));
            if drawn_signal.is_some() {
                fields.push(
                    view.values
                        .get(row)
                        .copied()
                        .flatten()
                        .map_or_else(String::new, |value| format!("{value:.17e}")),
                );
            }
            ui.ctx().copy_text(fields.join("\t"));
            ui.close();
        }
    });
}

#[cfg(test)]
mod tests;

//! The dock's Measurements page: what the shown view keeps and what the
//! netlist measured, in one table with a column per run shown, the active
//! run's first, then each run drawn behind it, as EZwave's measurement tool
//! and ADE's results list one value per history.
//!
//! A kept measurement is its definition: it is read on each run shown, over
//! its window (the whole record, the visible range or between the cursors,
//! as EZwave's measurement tool offers), once per revision of the view, the
//! data, the zoom and the cursors, and a run it cannot be read on says why.
//! The netlist's own `.MEAS` results are rows of the same table, under their
//! own group.

use egui::Ui;
use rspice_app_workbench::workbench::docks::console;
use rspice_results::visualization_document::{MeasurementId, MeasurementWindow};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{
    CELL_PAD, Cell, CellEditor, Column, DataTable, EditOutcome, Group, TableState,
};
use rspice_ui_kit::widgets::{chip_width, compact_chip};

use super::*;

/// One reading of one row on one run.
#[derive(Debug, Clone, PartialEq)]
enum Reading {
    Value(String),
    /// Why there is no value: the reason a kept measurement cannot be read
    /// on the run, or the netlist's error.
    Failed(String),
    /// The run did not measure it.
    Absent,
}

/// One row of the table.
#[derive(Debug, Clone, PartialEq)]
struct Row {
    name: String,
    /// The kept measurement the row reads; `None` for the netlist's own.
    kept: Option<MeasurementId>,
    /// What a kept one measures, and over what part of the record.
    definition: Option<(String, MeasurementWindow)>,
    /// Whether a kept one reports a family's statistic.
    family: bool,
    readings: Vec<Reading>,
}

/// The page's rows, read once per revision of what they read.
#[derive(Debug, Clone, PartialEq)]
struct Page {
    /// Each run's column head.
    runs: Vec<String>,
    rows: Vec<Row>,
    /// The view whose measurements the kept rows are.
    view: Option<ResultDocumentId>,
}

impl Page {
    /// The table as tab-separated text, with where each row comes from.
    fn tsv(&self) -> String {
        let mut lines = vec![
            ["Measurement", "Expression", "Window"]
                .into_iter()
                .map(str::to_owned)
                .chain(self.runs.iter().cloned())
                .chain(std::iter::once("From".to_owned()))
                .collect::<Vec<_>>()
                .join("\t"),
        ];
        lines.extend(self.rows.iter().map(|row| row_text(row).join("\t")));
        lines.join("\n")
    }

    /// How many rows are kept with the view; the netlist's follow.
    fn kept(&self) -> usize {
        self.rows
            .iter()
            .take_while(|row| row.kept.is_some())
            .count()
    }
}

/// One row as text: name, expression, window, a reading per run, and where
/// it comes from.
fn row_text(row: &Row) -> Vec<String> {
    let (expression, window) = row.definition.as_ref().map_or_else(
        || (String::new(), String::new()),
        |(expression, window)| (expression.clone(), window.label().to_owned()),
    );
    [row.name.clone(), expression, window]
        .into_iter()
        .chain(row.readings.iter().map(|reading| match reading {
            Reading::Value(value) => value.clone(),
            Reading::Failed(_) | Reading::Absent => String::new(),
        }))
        .chain(std::iter::once(from(row).to_owned()))
        .collect()
}

fn from(row: &Row) -> &'static str {
    if row.kept.is_some() {
        "view"
    } else {
        "netlist"
    }
}

/// The runs the page reads: the shown run, then each drawn behind it.
fn shown_runs(simulation: &crate::state::SimulationState) -> Vec<&SimulationRun> {
    let Some(shown) = simulation.active_run() else {
        return Vec::new();
    };
    std::iter::once(shown)
        .chain(
            simulation
                .overlaid_dataset_ids()
                .into_iter()
                .filter_map(|dataset| {
                    simulation
                        .retained
                        .runs
                        .iter()
                        .find(|run| run.dataset_id == dataset && run.dataset_id != shown.dataset_id)
                }),
        )
        .collect()
}

/// What changes the page's rows: the view's revision, the data, the runs
/// shown and the digits they are printed at.
fn revision(state: ResultsReads<'_>) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let view = shown_view(state);
    view.hash(&mut hasher);
    view.and_then(|view| state.model.workspace.content.visualization_document(view))
        .map(|document| format!("{:?}", document.revision()))
        .hash(&mut hasher);
    state.model.simulation.view.data_version.hash(&mut hasher);
    super::measurement_reading::window_version(state).hash(&mut hasher);
    for run in shown_runs(&state.model.simulation) {
        run.dataset_id.hash(&mut hasher);
    }
    digits(state).hash(&mut hasher);
    hasher.finish()
}

fn digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .model
            .preferences
            .result_presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}

fn read_page(state: ResultsReads<'_>) -> Page {
    let runs = shown_runs(&state.model.simulation);
    let digits = digits(state);
    let view = shown_view(state);
    let mut rows = Vec::new();
    if let Some(document) =
        view.and_then(|view| state.model.workspace.content.visualization_document(view))
    {
        for measurement in document.measurements_in_order() {
            let Some(expression) = measurement.expression.as_deref() else {
                continue;
            };
            let readings = runs
                .iter()
                .map(|run| {
                    match super::measurement_reading::read_kept(state, document, measurement, run) {
                        Ok(read) => {
                            Reading::Value(rspice_results::number_format::fmt_si_significant(
                                read.value, &read.unit, digits,
                            ))
                        }
                        Err(why) => Reading::Failed(why),
                    }
                })
                .collect();
            let label = measurement.label.trim();
            rows.push(Row {
                name: if label.is_empty() {
                    expression.to_owned()
                } else {
                    label.to_owned()
                },
                kept: Some(measurement.id),
                definition: Some((expression.to_owned(), measurement.window)),
                family: measurement.family.is_some(),
                readings,
            });
        }
    }
    let mut names: Vec<&str> = Vec::new();
    for run in &runs {
        for analysis in &run.analyses {
            for measured in &analysis.measurements {
                if !names.contains(&measured.name.as_str()) {
                    names.push(&measured.name);
                }
            }
        }
    }
    for name in names {
        let readings = runs
            .iter()
            .map(|run| {
                let Some(measured) = run
                    .analyses
                    .iter()
                    .flat_map(|analysis| &analysis.measurements)
                    .find(|measured| measured.name == name)
                else {
                    return Reading::Absent;
                };
                match (&measured.error, measured.value) {
                    (Some(error), _) => Reading::Failed(error.clone()),
                    (None, Some(value)) => {
                        let unit = measured
                            .units
                            .as_ref()
                            .and_then(|units| units.value.symbol())
                            .unwrap_or_default();
                        Reading::Value(rspice_results::number_format::fmt_si_significant(
                            value, unit, digits,
                        ))
                    }
                    (None, None) => Reading::Failed("The netlist measured no value.".to_owned()),
                }
            })
            .collect();
        rows.push(Row {
            name: name.to_owned(),
            kept: None,
            definition: None,
            family: false,
            readings,
        });
    }
    Page {
        runs: runs.iter().map(|run| format!("Run {}", run.id)).collect(),
        rows,
        view,
    }
}

/// The page's rows, read again only when what they read changed.
fn page(ui: &Ui, state: ResultsReads<'_>) -> Page {
    let id = egui::Id::new("rspice.results.measurements.page");
    let revision = revision(state);
    if let Some((held, page)) = ui.data(|data| data.get_temp::<(u64, Page)>(id))
        && held == revision
    {
        return page;
    }
    let page = read_page(state);
    ui.data_mut(|data| data.insert_temp(id, (revision, page.clone())));
    page
}

/// A row's name or expression as the reader edits it in place.
#[derive(Clone)]
struct CellDraft {
    row: usize,
    column: usize,
    text: String,
    focus: bool,
}

/// What the reader asked of the page this frame.
enum Asked {
    Rename(MeasurementId, String),
    Expression(MeasurementId, String),
    Window(MeasurementId, MeasurementWindow),
    /// Move the first to where the second stands.
    Move(MeasurementId, MeasurementId),
    Duplicate(MeasurementId),
    Delete(MeasurementId),
}

/// How many rows the page draws over the shown result: the view's kept
/// measurements and the netlist's, for the tab's badge. `None` when no run
/// is shown.
pub fn measurements_count(state: ResultsReads<'_>) -> Option<usize> {
    let runs = shown_runs(&state.model.simulation);
    if runs.is_empty() {
        return None;
    }
    let kept = shown_view(state)
        .and_then(|view| state.model.workspace.content.visualization_document(view))
        .map_or(0, |document| {
            document
                .measurements()
                .iter()
                .filter(|measurement| measurement.expression.is_some())
                .count()
        });
    let mut names: Vec<&str> = Vec::new();
    for measured in runs
        .iter()
        .flat_map(|run| &run.analyses)
        .flat_map(|analysis| &analysis.measurements)
    {
        if !names.contains(&measured.name.as_str()) {
            names.push(&measured.name);
        }
    }
    Some(kept + names.len())
}

/// Draw the Measurements page over the shown result. `false` when no run is
/// shown, for the console's own page to stand instead.
pub fn measurements_page(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    if state.model.simulation.active_run().is_none() {
        return false;
    }
    let page = page(ui, state.results_reads());
    let (add, copy) = console::toolbar(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_height(Tokens::get(ui.ctx()).metrics.ctl_h);
            ui.spacing_mut().item_spacing.x = 6.0;
            let label = "Add measurement\u{2026}";
            let add = compact_chip(ui, label, false, chip_width(ui, label))
                .on_hover_text("Keep a measurement with this view, read on every run it shows")
                .clicked();
            let copy = ui
                .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    !page.rows.is_empty()
                        && compact_chip(ui, "Copy", false, chip_width(ui, "Copy"))
                            .on_hover_text("Copy the table as tab-separated text")
                            .clicked()
                })
                .inner;
            (add, copy)
        })
        .inner
    });
    if copy {
        ui.ctx().copy_text(page.tsv());
        state.prompt("Copied the measurements");
    }
    if add {
        super::measurement::open_measurement(state);
    }
    if page.rows.is_empty() {
        ui.add_space(console::CONSOLE_BODY_PADDING_TOP);
        if console::prose_fits(ui) {
            console::padded(ui, |ui| {
                console::muted(
                    ui,
                    "No measurements. Add one, or run a netlist with .MEAS statements.",
                );
            });
        }
        return true;
    }
    if let Some(asked) = table(ui, &page)
        && let Some(view) = page.view
    {
        let edits = edits_for(state.results_reads(), view, asked);
        if !edits.is_empty()
            && let Err(error) = transact(state, view, edits)
        {
            state.prompt(format!("Could not change the measurement: {error}"));
        }
    }
    true
}

/// The view's edits for what the reader asked.
fn edits_for(state: ResultsReads<'_>, view: ResultDocumentId, asked: Asked) -> Vec<DocumentEdit> {
    let held = |id: MeasurementId| {
        state
            .model
            .workspace
            .content
            .visualization_document(view)
            .and_then(|document| document.measurements().iter().find(|held| held.id == id))
    };
    let set = |id: MeasurementId, expression: Option<String>, window: Option<MeasurementWindow>| {
        held(id).and_then(|measurement| {
            Some(DocumentEdit::SetMeasurement {
                measurement_id: id,
                expression: expression.or_else(|| measurement.expression.clone())?,
                window: window.unwrap_or(measurement.window),
            })
        })
    };
    match asked {
        Asked::Rename(id, name) => vec![DocumentEdit::Rename {
            entity: EntityRef::Measurement(id),
            value: name,
        }],
        Asked::Expression(id, expression) => set(id, Some(expression), None).into_iter().collect(),
        Asked::Window(id, window) => set(id, None, Some(window)).into_iter().collect(),
        Asked::Move(id, neighbour) => state
            .model
            .workspace
            .content
            .visualization_document(view)
            .and_then(|document| {
                document
                    .measurements_in_order()
                    .iter()
                    .position(|held| held.id == neighbour)
            })
            .map(|index| DocumentEdit::MoveMeasurement {
                measurement_id: id,
                index,
            })
            .into_iter()
            .collect(),
        Asked::Delete(id) => vec![DocumentEdit::Remove(EntityRef::Measurement(id))],
        Asked::Duplicate(id) => duplicate(state, view, id),
    }
}

/// A copy of the kept measurement `id`, read as it reads.
fn duplicate(
    state: ResultsReads<'_>,
    view: ResultDocumentId,
    id: MeasurementId,
) -> Vec<DocumentEdit> {
    let Some(measurement) = state
        .model
        .workspace
        .content
        .visualization_document(view)
        .and_then(|document| document.measurements().iter().find(|held| held.id == id))
    else {
        return Vec::new();
    };
    let Some(expression) = measurement.expression.clone() else {
        return Vec::new();
    };
    vec![match measurement.family {
        Some(family) => DocumentEdit::AddFamilyMeasurement {
            pane_id: measurement.pane_id,
            expression,
            window: measurement.window,
            family,
        },
        None => DocumentEdit::AddScalarMeasurement {
            pane_id: measurement.pane_id,
            expression,
            window: measurement.window,
        },
    }]
}

fn text_width(ui: &Ui, text: &str) -> f32 {
    ui.painter()
        .layout_no_wrap(
            text.to_owned(),
            rspice_ui_kit::theme::mono(
                rspice_ui_kit::tokens::FS_1,
                rspice_ui_kit::theme::FontWeight::Regular,
            ),
            egui::Color32::WHITE,
        )
        .size()
        .x
}

/// The table's fixed columns: name, expression, window; the runs follow.
const NAME: usize = 0;
const EXPRESSION: usize = 1;
const WINDOW: usize = 2;

fn table(ui: &mut Ui, page: &Page) -> Option<Asked> {
    // A measurement is named by what it reads until the reader names it,
    // as ADE's outputs are, so only a name of its own fills the column.
    let named = |row: &Row| -> Option<String> {
        match &row.definition {
            Some((expression, _)) if row.name == *expression => None,
            _ => Some(row.name.clone()),
        }
    };
    let name_width = page
        .rows
        .iter()
        .filter_map(named)
        .map(|name| text_width(ui, &name))
        .fold(text_width(ui, "Name"), f32::max);
    let window_width = MeasurementWindow::ALL
        .into_iter()
        .map(|window| text_width(ui, window.label()))
        .fold(0.0, f32::max);
    let mut columns = vec![
        Column::words("Name", (name_width + CELL_PAD * 2.0).clamp(96.0, 220.0))
            .unsortable()
            .tooltip("The name a kept measurement was given; click to name it"),
        Column::name("Expression", 160.0)
            .unsortable()
            .tooltip("What a kept measurement reads; click to edit it"),
        Column::words("Window", window_width + CELL_PAD * 2.0)
            .unsortable()
            .tooltip(
                "The part of the record it reads: all of it, the visible range, or between the cursors",
            ),
    ];
    for (index, run) in page.runs.iter().enumerate() {
        let width = page
            .rows
            .iter()
            .filter_map(|row| match row.readings.get(index) {
                Some(Reading::Value(value)) => Some(text_width(ui, value)),
                _ => None,
            })
            .fold(text_width(ui, run), f32::max);
        columns.push(
            Column::number(run, (width + CELL_PAD * 2.0).max(88.0))
                .unsortable()
                .tooltip(if index == 0 {
                    "The run shown".to_owned()
                } else {
                    "A run drawn behind the one shown".to_owned()
                }),
        );
    }
    let rows: Vec<Vec<Cell>> = page
        .rows
        .iter()
        .map(|row| {
            let mut cells = match &row.definition {
                Some((expression, window)) => vec![
                    Cell::text(named(row).unwrap_or_default())
                        .editable()
                        .hint("Click to name it; right-click for its menu"),
                    Cell::text(expression)
                        .editable()
                        .hint("Click to edit what it reads"),
                    if row.family {
                        Cell::text(window.label())
                            .hint("A family measurement reads each member's whole record")
                    } else {
                        Cell::text(window.label())
                            .editable()
                            .hint("Click to read it over another part of the record")
                    },
                ],
                None => vec![
                    Cell::text(&row.name)
                        .hint("Measured in the run, by a .MEAS statement or by its analysis"),
                    Cell::absent(),
                    Cell::absent(),
                ],
            };
            cells.extend(row.readings.iter().map(|reading| match reading {
                Reading::Value(value) => Cell::number(value),
                Reading::Failed(why) => Cell::absent().hint(why),
                Reading::Absent => Cell::absent().hint("This run did not measure it"),
            }));
            cells
        })
        .collect();
    let kept = page.kept();
    let mut groups = Vec::new();
    if kept > 0 {
        groups.push(Group::new("Kept with this view", 0));
    }
    if kept < page.rows.len() {
        groups.push(Group::new("From the netlist", kept));
    }

    let id = "rspice.results.measurements";
    let state_id = egui::Id::new(id);
    let draft_id = state_id.with("draft");
    let window_menu = state_id.with("window menu");
    let mut table_state = ui
        .data(|data| data.get_temp::<TableState>(state_id))
        .unwrap_or_default();
    let mut draft = ui.data(|data| data.get_temp::<CellDraft>(draft_id));
    let editor = draft.as_mut().map(|draft| CellEditor {
        row: draft.row,
        column: draft.column,
        text: &mut draft.text,
        focus: std::mem::take(&mut draft.focus),
    });
    let response = DataTable::new(id, &columns, rows.len())
        .state(&mut table_state)
        .groups(&groups)
        .editing(editor)
        .show(ui, |row| rows[row].clone());
    ui.data_mut(|data| data.insert_temp(state_id, table_state));

    let ctx = ui.ctx().clone();
    let mut asked = None;
    if let Some((row, column)) = response.edit
        && let Some(held) = page.rows.get(row)
        && let Some((expression, _)) = &held.definition
    {
        match column {
            NAME => {
                draft = Some(CellDraft {
                    row,
                    column,
                    text: held.name.clone(),
                    focus: true,
                })
            }
            EXPRESSION => {
                draft = Some(CellDraft {
                    row,
                    column,
                    text: expression.clone(),
                    focus: true,
                });
            }
            WINDOW => {
                let position = ctx.pointer_latest_pos().unwrap_or_default();
                ctx.data_mut(|data| data.insert_temp(window_menu, (row, position)));
                egui::Popup::open_id(&ctx, window_menu);
            }
            _ => {}
        }
    }
    match response.edited {
        Some(EditOutcome::Cancel) => draft = None,
        Some(EditOutcome::Commit) => {
            if let Some(open) = draft.take()
                && let Some(row) = page.rows.get(open.row)
                && let (Some(id), Some((expression, _))) = (row.kept, &row.definition)
            {
                let text = open.text.trim().to_owned();
                asked = match open.column {
                    NAME if !text.is_empty() && text != row.name => Some(Asked::Rename(id, text)),
                    EXPRESSION if !text.is_empty() && text != *expression => {
                        Some(Asked::Expression(id, text))
                    }
                    _ => None,
                };
            }
        }
        None => {}
    }
    ui.data_mut(|data| match draft.clone() {
        Some(draft) => {
            data.insert_temp(draft_id, draft);
        }
        None => data.remove::<CellDraft>(draft_id),
    });

    let popup = |menu: egui::Id, position: egui::Pos2, body: &mut dyn FnMut(&mut Ui)| {
        egui::Popup::new(
            menu,
            ctx.clone(),
            egui::PopupAnchor::Position(position),
            egui::LayerId::new(egui::Order::Foreground, menu),
        )
        .kind(egui::PopupKind::Menu)
        .open_memory(None)
        .show(|ui| body(ui));
    };
    let window_items = |ui: &mut Ui, id: MeasurementId, current: MeasurementWindow| {
        let mut chosen = None;
        for window in MeasurementWindow::ALL {
            if ui.radio(current == window, window.label()).clicked() {
                chosen = Some(Asked::Window(id, window));
                ui.close();
            }
        }
        chosen
    };
    if egui::Popup::is_id_open(&ctx, window_menu)
        && let Some((row, position)) =
            ctx.data(|data| data.get_temp::<(usize, egui::Pos2)>(window_menu))
        && let Some(held) = page.rows.get(row)
        && let (Some(id), Some((_, current))) = (held.kept, &held.definition)
    {
        let current = *current;
        popup(window_menu, position, &mut |ui| {
            if let Some(chosen) = window_items(ui, id, current) {
                asked = Some(chosen);
            }
        });
    }

    let menu = state_id.with("menu");
    if let Some(request) = response.context_menu {
        ctx.data_mut(|data| data.insert_temp(menu, (request.cell.row, request.position)));
        egui::Popup::open_id(&ctx, menu);
    }
    if egui::Popup::is_id_open(&ctx, menu)
        && let Some((row, position)) = ctx.data(|data| data.get_temp::<(usize, egui::Pos2)>(menu))
        && let Some(held) = page.rows.get(row)
    {
        popup(menu, position, &mut |ui| {
            if let (Some(id), Some((expression, window))) = (held.kept, &held.definition) {
                let edit = |column: usize, text: &str| {
                    ctx.data_mut(|data| {
                        data.insert_temp(
                            draft_id,
                            CellDraft {
                                row,
                                column,
                                text: text.to_owned(),
                                focus: true,
                            },
                        );
                    });
                };
                if ui.button("Rename").clicked() {
                    edit(NAME, &held.name);
                    ui.close();
                }
                if ui.button("Edit expression").clicked() {
                    edit(EXPRESSION, expression);
                    ui.close();
                }
                if !held.family {
                    ui.menu_button("Read over", |ui| {
                        if let Some(chosen) = window_items(ui, id, *window) {
                            asked = Some(chosen);
                        }
                    });
                }
                ui.separator();
                let above = row.checked_sub(1).and_then(|above| page.rows[above].kept);
                let below = page.rows.get(row + 1).and_then(|below| below.kept);
                if ui
                    .add_enabled(above.is_some(), egui::Button::new("Move up"))
                    .clicked()
                    && let Some(above) = above
                {
                    asked = Some(Asked::Move(id, above));
                    ui.close();
                }
                if ui
                    .add_enabled(below.is_some(), egui::Button::new("Move down"))
                    .clicked()
                    && let Some(below) = below
                {
                    asked = Some(Asked::Move(id, below));
                    ui.close();
                }
                if ui.button("Duplicate").clicked() {
                    asked = Some(Asked::Duplicate(id));
                    ui.close();
                }
                if ui.button("Delete").clicked() {
                    asked = Some(Asked::Delete(id));
                    ui.close();
                }
                ui.separator();
            }
            if ui.button("Copy row").clicked() {
                ui.ctx().copy_text(row_text(held).join("\t"));
                ui.close();
            }
        });
    }
    asked
}

/// Each row as the page draws it: its name, a reading per run and where
/// it comes from.
#[cfg(test)]
type RowText = (String, Vec<String>, &'static str);

/// The page's rows as it would draw them now, under the runs it reads.
#[cfg(test)]
pub(crate) fn page_rows(state: ResultsReads<'_>) -> (Vec<String>, Vec<RowText>) {
    let page = read_page(state);
    let rows = page
        .rows
        .iter()
        .map(|row| {
            (
                row.name.clone(),
                row.readings
                    .iter()
                    .map(|reading| match reading {
                        Reading::Value(value) => value.clone(),
                        Reading::Failed(why) => format!("failed: {why}"),
                        Reading::Absent => "absent".to_owned(),
                    })
                    .collect(),
                from(row),
            )
        })
        .collect();
    (page.runs, rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
    use crate::state::{AnalysisResultProvenance, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::test_fixtures::{
        completed_prepared_run, digest,
    };
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;

    /// A finished run of `plan` whose transient `analysis` ramps V(out) to
    /// `peak` volts, and whose netlist measured `vmax` at `peak`.
    fn plan_run(plan: SimulationPlanId, analysis: AnalysisInstanceId, peak: f64) -> SimulationRun {
        use rspice_core::analysis::{MeasurementUnit, MeasurementUnits};
        let mut run = completed_prepared_run(plan, ObjectRevision::INITIAL, digest(0x50));
        let mut measured = rspice_core::MeasureResult::success("vmax", peak);
        measured.units = Some(MeasurementUnits {
            value: MeasurementUnit::Known("V".to_owned()),
            raw_value: MeasurementUnit::Known("V".to_owned()),
            axis: MeasurementUnit::Known("s".to_owned()),
        });
        let mut result = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![
                RetainedTrace::new("V(out)", vec![0.0, 1.0e-3], vec![0.0, peak], "#ffbd2e")
                    .with_unit("V"),
            ])
            .with_provenance(
                AnalysisResultProvenance::new(
                    analysis,
                    ObjectRevision::INITIAL,
                    digest(0x51),
                    Vec::new(),
                )
                .expect("analysis provenance"),
            );
        result.measurements.push(measured);
        run.add_analysis(result);
        run
    }

    /// A measurement kept with the view is read on every run shown, a
    /// column each, and follows the view to a new run; the netlist's own
    /// result is a row of the same table; a run that cannot be read says
    /// why; rename and delete each change the view.
    #[test]
    fn a_kept_measurement_reads_on_every_run_shown_beside_the_netlists() {
        let (plan, analysis) = (SimulationPlanId::new(), AnalysisInstanceId::new());
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![plan_run(plan, analysis, 2.0)].into();
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        let view = shown_view(state.results_reads()).expect("the run's working view");
        let pane_id = state.working_view().expect("the view").panes()[0].id;
        transact(
            &mut state.results_parts(),
            view,
            vec![DocumentEdit::AddScalarMeasurement {
                pane_id,
                expression: "max(V(out))".to_owned(),
                window: MeasurementWindow::WholeRecord,
            }],
        )
        .expect("the view keeps the measurement");
        let (runs, rows) = page_rows(state.results_reads());
        assert_eq!(runs.len(), 1);
        assert_eq!(rows[0].0, "max(V(out))");
        assert_eq!(rows[0].2, "view");
        assert_eq!(rows[0].1, ["2 V"], "{rows:?}");
        assert_eq!(rows[1].0, "vmax");
        assert_eq!(rows[1].2, "netlist");
        assert_eq!(rows[1].1, ["2 V"], "the netlist's result reads in its unit");

        // The plan runs again, higher, with the first run drawn behind it.
        state
            .model
            .simulation
            .retained
            .runs
            .insert(0, plan_run(plan, analysis, 3.0));
        assert!(state.model.simulation.select_run(0));
        state.model.simulation.view.overlay_previous = 1;
        state.quick_frame();
        let (runs, rows) = page_rows(state.results_reads());
        assert_eq!(runs.len(), 2, "{runs:?}");
        assert_eq!(rows[0].1, ["3 V", "2 V"], "a value per run shown: {rows:?}");
        assert_eq!(rows[1].1, ["3 V", "2 V"], "{rows:?}");

        // A measurement whose signal the run lacks says why.
        let measurement = state.working_view().expect("the view").measurements()[0].id;
        transact(
            &mut state.results_parts(),
            view,
            vec![
                DocumentEdit::AddScalarMeasurement {
                    pane_id,
                    expression: "max(V(gone))".to_owned(),
                    window: MeasurementWindow::WholeRecord,
                },
                DocumentEdit::Rename {
                    entity: EntityRef::Measurement(measurement),
                    value: "Peak".to_owned(),
                },
            ],
        )
        .expect("the view takes both");
        let (_, rows) = page_rows(state.results_reads());
        assert_eq!(rows[0].0, "Peak");
        assert!(rows[1].1[0].starts_with("failed: "), "{rows:?}");

        transact(
            &mut state.results_parts(),
            view,
            vec![DocumentEdit::Remove(EntityRef::Measurement(measurement))],
        )
        .expect("the view lets it go");
        let (_, rows) = page_rows(state.results_reads());
        assert_eq!(rows[0].0, "max(V(gone))");
    }

    /// A measurement over the visible range reads what the strip shows and
    /// follows its zoom while one over the whole record does not; one
    /// between the cursors reads from A to B and says why without them; and
    /// Undo puts back a measurement deleted over the quick view.
    #[test]
    fn a_measurement_reads_over_its_window_and_a_delete_undoes() {
        use crate::workbench::documents::result_document::AnalysisPresentationKey;
        use crate::workbench::documents::result_document::view_edits;
        let (plan, analysis) = (SimulationPlanId::new(), AnalysisInstanceId::new());
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![plan_run(plan, analysis, 2.0)].into();
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        let view = shown_view(state.results_reads()).expect("the run's working view");
        let pane_id = state.working_view().expect("the view").panes()[0].id;
        let edits = MeasurementWindow::ALL
            .into_iter()
            .map(|window| DocumentEdit::AddScalarMeasurement {
                pane_id,
                expression: "max(V(out))".to_owned(),
                window,
            })
            .collect();
        transact(&mut state.results_parts(), view, edits).expect("the view keeps all three");
        let readings = |state: &ResultsTestHost| -> Vec<Vec<String>> {
            page_rows(state.results_reads())
                .1
                .into_iter()
                .filter(|(_, _, from)| *from == "view")
                .map(|(_, readings, _)| readings)
                .collect()
        };
        // V(out) ramps from 0 to 2 V over the millisecond.
        let unzoomed = readings(&state);
        assert_eq!(
            unzoomed[..2],
            [["2 V"], ["2 V"]],
            "unzoomed, the strip shows it all"
        );
        assert!(
            unzoomed[2][0].starts_with("failed: Place cursors A and B"),
            "{unzoomed:?}"
        );

        let key = {
            let run = state.model.simulation.active_run().unwrap();
            AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
        };
        rspice_results_ui::waves::stack::set_shared_x_view(
            &mut state.results.results.session,
            key,
            Some((0.0, 0.5e-3)),
        );
        let session = &mut state.results.results.session;
        session.cursors.a = Some(0.75e-3);
        session.cursors.b = Some(0.25e-3);
        session.cursor_strip = Some(0);
        assert_eq!(
            readings(&state),
            [["2 V"], ["1 V"], ["1.5 V"]],
            "the whole record, the zoomed half, and A to B"
        );

        let first = state.working_view().expect("the view").measurements()[0].id;
        transact(
            &mut state.results_parts(),
            view,
            vec![DocumentEdit::Remove(EntityRef::Measurement(first))],
        )
        .expect("the view lets it go");
        assert_eq!(readings(&state).len(), 2);
        view_edits::step_shown_view(&mut state.results_parts(), view_edits::Step::Undo);
        assert_eq!(
            readings(&state).len(),
            3,
            "Undo over the quick view puts it back"
        );
    }
}

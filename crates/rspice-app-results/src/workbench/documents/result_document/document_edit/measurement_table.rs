//! The dock's Measurements page: what the shown view keeps and what the
//! netlist measured, in one table with a column per run shown, the active
//! run's first, then each run drawn behind it, as EZwave's measurement tool
//! and ADE's results list one value per history.
//!
//! A kept measurement is its definition: it is evaluated on each run shown,
//! once per revision of the view and of the data, and a run it cannot be
//! read on says why. The netlist's own `.MEAS` results are rows of the same
//! table, marked as the netlist's.

use egui::Ui;
use rspice_app_workbench::workbench::docks::console;
use rspice_results::visualization_document::MeasurementId;
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{
    CELL_PAD, Cell, CellEditor, Column, DataTable, EditOutcome, TableState,
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
    /// What it measures, as its hover says.
    definition: String,
    readings: Vec<Reading>,
}

/// The page's rows, read once per revision of the view and the data.
#[derive(Debug, Clone, PartialEq)]
struct Page {
    /// Each run's column head.
    runs: Vec<String>,
    rows: Vec<Row>,
    /// The view whose measurements the kept rows are.
    view: Option<ResultDocumentId>,
}

impl Page {
    fn tsv(&self) -> String {
        let mut lines = vec![
            std::iter::once("Measurement".to_owned())
                .chain(self.runs.iter().cloned())
                .chain(std::iter::once("From".to_owned()))
                .collect::<Vec<_>>()
                .join("\t"),
        ];
        for row in &self.rows {
            lines.push(
                std::iter::once(row.name.clone())
                    .chain(row.readings.iter().map(|reading| match reading {
                        Reading::Value(value) => value.clone(),
                        Reading::Failed(_) | Reading::Absent => String::new(),
                    }))
                    .chain(std::iter::once(from(row).to_owned()))
                    .collect::<Vec<_>>()
                    .join("\t"),
            );
        }
        lines.join("\n")
    }
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

/// The view whose measurements the page lists: the saved document shown,
/// else the shown run's working view.
fn shown_view(state: ResultsReads<'_>) -> Option<ResultDocumentId> {
    active_document_id(state).or_else(|| {
        let run = state.model.simulation.active_run()?;
        super::super::working_view::view_of_run(state, run.dataset_id)
    })
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

/// The analysis of `run` that `document`'s pane `pane_id` reads.
fn pane_analysis<'a>(
    document: &VisualizationDocument,
    pane_id: PaneId,
    run: &'a SimulationRun,
) -> Option<&'a AnalysisResult> {
    run.analyses.iter().find(|analysis| {
        document
            .pane_of_analysis(super::super::create_document::source_analysis_identity(
                run, analysis,
            ))
            .is_some_and(|pane| pane.id == pane_id)
    })
}

fn read_page(state: ResultsReads<'_>) -> Page {
    let runs = shown_runs(&state.model.simulation);
    let digits = digits(state);
    let view = shown_view(state);
    let mut rows = Vec::new();
    if let Some(document) =
        view.and_then(|view| state.model.workspace.content.visualization_document(view))
    {
        for measurement in document.measurements() {
            let Some(expression) = measurement.expression.as_deref() else {
                continue;
            };
            let readings = runs
                .iter()
                .map(|run| {
                    let Some(analysis) = pane_analysis(document, measurement.pane_id, run) else {
                        return Reading::Failed("This run has no such analysis.".to_owned());
                    };
                    match super::measurement::evaluate(analysis, expression) {
                        Ok(value) => {
                            let unit = super::super::waves::measurement_unit(
                                expression,
                                analysis,
                                analysis.analysis_type.axis_info().1,
                            )
                            .unwrap_or_default();
                            Reading::Value(rspice_results::number_format::fmt_si_significant(
                                value, &unit, digits,
                            ))
                        }
                        Err(error) => Reading::Failed(error),
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
                definition: expression.to_owned(),
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
            definition: format!("{name}, the netlist's .MEAS"),
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

/// A row's name as the reader renames it in place.
#[derive(Clone)]
struct NameDraft {
    row: usize,
    text: String,
    focus: bool,
}

/// What the reader asked of the page this frame.
enum Asked {
    Rename(MeasurementId, String),
    Duplicate(MeasurementId),
    Delete(MeasurementId),
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
        let edits = match asked {
            Asked::Rename(id, name) => vec![DocumentEdit::Rename {
                entity: EntityRef::Measurement(id),
                value: name,
            }],
            Asked::Delete(id) => vec![DocumentEdit::Remove(EntityRef::Measurement(id))],
            Asked::Duplicate(id) => duplicate(state.results_reads(), view, id),
        };
        if !edits.is_empty()
            && let Err(error) = transact(state, view, edits)
        {
            state.prompt(format!("Could not change the measurement: {error}"));
        }
    }
    true
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
    vec![DocumentEdit::AddScalarMeasurement {
        pane_id: measurement.pane_id,
        expression,
        value: measurement.value.unwrap_or(0.0),
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

fn table(ui: &mut Ui, page: &Page) -> Option<Asked> {
    let mut columns = vec![Column::name("Measurement", 160.0).unsortable()];
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
    columns.push(
        Column::words("From", 72.0)
            .unsortable()
            .tooltip("Kept with this view, or measured by the netlist's .MEAS"),
    );
    let rows: Vec<Vec<Cell>> = page
        .rows
        .iter()
        .map(|row| {
            let mut cells = vec![match row.kept {
                Some(_) => Cell::text(&row.name).editable().hint(format!(
                    "{}. Click to rename it; right-click for its menu",
                    row.definition
                )),
                None => Cell::text(&row.name).hint(&row.definition),
            }];
            cells.extend(row.readings.iter().map(|reading| match reading {
                Reading::Value(value) => Cell::number(value),
                Reading::Failed(why) => Cell::absent().hint(why),
                Reading::Absent => Cell::absent().hint("This run did not measure it"),
            }));
            cells.push(Cell::text(from(row)));
            cells
        })
        .collect();

    let id = "rspice.results.measurements";
    let state_id = egui::Id::new(id);
    let draft_id = state_id.with("draft");
    let mut table_state = ui
        .data(|data| data.get_temp::<TableState>(state_id))
        .unwrap_or_default();
    let mut draft = ui.data(|data| data.get_temp::<NameDraft>(draft_id));
    let editor = draft.as_mut().map(|draft| CellEditor {
        row: draft.row,
        column: 0,
        text: &mut draft.text,
        focus: std::mem::take(&mut draft.focus),
    });
    let response = DataTable::new(id, &columns, rows.len())
        .state(&mut table_state)
        .editing(editor)
        .show(ui, |row| rows[row].clone());
    ui.data_mut(|data| data.insert_temp(state_id, table_state));

    let mut asked = None;
    if let Some((row, 0)) = response.edit
        && page.rows.get(row).is_some_and(|row| row.kept.is_some())
    {
        draft = Some(NameDraft {
            row,
            text: page.rows[row].name.clone(),
            focus: true,
        });
    }
    match response.edited {
        Some(EditOutcome::Cancel) => draft = None,
        Some(EditOutcome::Commit) => {
            if let Some(open) = draft.take()
                && let Some(row) = page.rows.get(open.row)
                && let Some(id) = row.kept
                && open.text.trim() != row.name
            {
                asked = Some(Asked::Rename(id, open.text.trim().to_owned()));
            }
        }
        None => {}
    }
    ui.data_mut(|data| match draft.clone() {
        Some(draft) => {
            data.insert_temp(draft_id, draft);
        }
        None => data.remove::<NameDraft>(draft_id),
    });

    let ctx = ui.ctx().clone();
    let menu = state_id.with("menu");
    if let Some(request) = response.context_menu {
        ctx.data_mut(|data| data.insert_temp(menu, (request.cell.row, request.position)));
        egui::Popup::open_id(&ctx, menu);
    }
    if egui::Popup::is_id_open(&ctx, menu)
        && let Some((row, position)) = ctx.data(|data| data.get_temp::<(usize, egui::Pos2)>(menu))
        && let Some(held) = page.rows.get(row)
    {
        egui::Popup::new(
            menu,
            ctx.clone(),
            egui::PopupAnchor::Position(position),
            egui::LayerId::new(egui::Order::Foreground, menu),
        )
        .kind(egui::PopupKind::Menu)
        .open_memory(None)
        .show(|ui| {
            if let Some(id) = held.kept {
                if ui.button("Rename").clicked() {
                    ui.data_mut(|data| {
                        data.insert_temp(
                            draft_id,
                            NameDraft {
                                row,
                                text: held.name.clone(),
                                focus: true,
                            },
                        );
                    });
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
                let mut line = vec![held.name.clone()];
                line.extend(held.readings.iter().map(|reading| match reading {
                    Reading::Value(value) => value.clone(),
                    Reading::Failed(_) | Reading::Absent => String::new(),
                }));
                ui.ctx().copy_text(line.join("\t"));
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
                value: 0.0,
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
                    value: 0.0,
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
}

//! Process corners: the ADE Assembler detail view of one corner run.
//!
//! Rows are outputs, columns are corners, and a cell is the value that output
//! took at that corner — coloured only when it breaks its limit or sits within
//! [`NEAR_LIMIT_FRACTION`] of it. Every value is read from the run itself: the
//! measurements a corner family's members retained, the measurements of the
//! point results the run solved for each corner, and the corner waveforms whose
//! samples line up one-for-one with the corners. Nothing is interpolated and no
//! row is invented; a corner a row has no value for is an em dash.
//!
//! The nominal column is the run's own reference corner, taken out of the
//! corner columns rather than repeated beside them. A run that solved no corner
//! at the reference point has no nominal column at all.

use super::table::{Cell, Column, DataTable, Verdict};
use super::toolbar::{PageToolbar, Summary, ToolbarAction, empty_line};
use super::*;

/// A value within this fraction of its limit is near it: of the limit's own
/// magnitude for a one-sided limit, of the span for a range.
pub(super) const NEAR_LIMIT_FRACTION: f64 = 0.05;
/// Least width of a corner or nominal column.
const CORNER_COLUMN_MIN_W: f32 = 88.0;

/// One corner's process, supply and temperature, read back from its label.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct CornerPoint {
    pub(super) process: String,
    pub(super) voltage: Option<f64>,
    pub(super) temperature: f64,
}

/// Read a corner label of the form `TT_1.80V_27C` back into its point.
///
/// The executor writes `{process}_{volts}V_{celsius}C` with as many decimals as
/// it likes; a label in any other shape is not a point this can name, and the
/// column keeps the label as written.
pub(super) fn parse_corner_label(label: &str) -> Option<CornerPoint> {
    let mut parts = label.rsplitn(3, '_');
    let temperature = parts.next()?.strip_suffix('C')?.parse::<f64>().ok()?;
    let voltage = parts.next()?.strip_suffix('V')?.parse::<f64>().ok()?;
    let process = parts.next()?.trim();
    (!process.is_empty() && temperature.is_finite() && voltage.is_finite()).then(|| CornerPoint {
        process: process.to_owned(),
        voltage: Some(voltage),
        temperature,
    })
}

/// One corner column: its short head, and the full point for its tooltip.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct CornerColumn {
    pub(super) head: String,
    pub(super) detail: String,
    point: Option<CornerPoint>,
}

/// One output across every corner.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct CornerRow {
    pub(super) output: String,
    pub(super) unit: String,
    pub(super) spec: Option<SpecEntry>,
    pub(super) values: Vec<Option<f64>>,
}

impl CornerRow {
    /// The verdict on one value, when the row has a limit to judge it by.
    pub(super) fn verdict_of(&self, value: f64) -> Option<Verdict> {
        self.spec.as_ref().map(|spec| judge(spec, value))
    }

    /// The row's verdict across every corner it has a value for.
    pub(super) fn verdict(&self) -> Option<Verdict> {
        self.values
            .iter()
            .flatten()
            .filter_map(|value| self.verdict_of(*value))
            .reduce(Verdict::worst)
    }
}

/// A value against its limit: a failure, a value near the limit, or a pass.
pub(super) fn judge(spec: &SpecEntry, value: f64) -> Verdict {
    if !spec.passes(value) {
        Verdict::Fail
    } else if normalized_margin(spec, value).is_some_and(|margin| margin < NEAR_LIMIT_FRACTION) {
        Verdict::Near
    } else {
        Verdict::Pass
    }
}

/// The whole matrix one corner run answers.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct CornerMatrix {
    pub(super) columns: Vec<CornerColumn>,
    /// The column that is the run's reference corner.
    pub(super) nominal: Option<usize>,
    pub(super) rows: Vec<CornerRow>,
}

impl CornerMatrix {
    /// The corner columns other than the nominal one, in run order.
    pub(super) fn corner_indices(&self) -> Vec<usize> {
        (0..self.columns.len())
            .filter(|index| Some(*index) != self.nominal)
            .collect()
    }

    /// How many limited rows pass and how many have a limit, and the corner
    /// whose worst failure is worst.
    fn tally(&self) -> (usize, usize, usize, Option<usize>) {
        let limited = self
            .rows
            .iter()
            .filter(|row| row.verdict().is_some())
            .count();
        let failing = self
            .rows
            .iter()
            .filter(|row| row.verdict() == Some(Verdict::Fail))
            .count();
        let near = self
            .rows
            .iter()
            .filter(|row| row.verdict() == Some(Verdict::Near))
            .count();
        let worst = self
            .rows
            .iter()
            .filter_map(|row| row.spec.as_ref().map(|spec| (row, spec)))
            .flat_map(|(row, spec)| {
                row.values
                    .iter()
                    .enumerate()
                    .filter_map(move |(column, value)| {
                        let value = (*value)?;
                        (!spec.passes(value)).then(|| {
                            (
                                column,
                                normalized_margin(spec, value).unwrap_or(f64::NEG_INFINITY),
                            )
                        })
                    })
            })
            .min_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(column, _)| column);
        (limited - failing, limited, near, worst)
    }

    /// The toolbar's one line about this matrix.
    pub(super) fn summary(&self) -> Summary {
        let corners = self.columns.len();
        let corner_word = if corners == 1 { "corner" } else { "corners" };
        let (passing, limited, near, worst) = self.tally();
        if limited == 0 {
            let outputs = self.rows.len();
            return Summary::plain(format!(
                "{outputs} output{} across {corners} {corner_word}",
                if outputs == 1 { "" } else { "s" }
            ));
        }
        let mut text = format!("{passing} / {limited} outputs pass across {corners} {corner_word}");
        if let Some(worst) = worst.and_then(|column| self.columns.get(column)) {
            text.push_str(&format!(" \u{00b7} worst {}", worst.head));
            return Summary::failing(text);
        }
        if near > 0 {
            text.push_str(&format!(" \u{00b7} {near} near limit"));
        }
        Summary::plain(text)
    }
}

/// Where the reference corner sits: the plan's reference process at its
/// reference temperature.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct ReferenceCorner {
    pub(super) process: String,
    pub(super) temperature: f64,
}

fn same_quantity(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1.0e-9 * left.abs().max(right.abs()).max(1.0)
}

fn point_matches(
    column: &CornerPoint,
    solved: &rspice_results::provenance::AnalysisResultPvtPoint,
) -> bool {
    column.process.eq_ignore_ascii_case(solved.process())
        && same_quantity(column.temperature, solved.temperature_celsius())
        && match (column.voltage, solved.supply_voltage()) {
            (Some(left), Some(right)) => same_quantity(left, right),
            _ => true,
        }
}

/// The compact number a corner head spells: as few digits as the value needs.
fn head_number(value: f64) -> String {
    let rounded = (value * 1.0e6).round() / 1.0e6;
    format!("{rounded}").replace('-', "\u{2212}")
}

fn corner_columns(labels: &[String]) -> Vec<CornerColumn> {
    let points = labels
        .iter()
        .map(|label| parse_corner_label(label))
        .collect::<Vec<_>>();
    let all_parsed = points.iter().all(Option::is_some);
    let distinct = |key: &dyn Fn(&CornerPoint) -> String| {
        let mut seen = points.iter().flatten().map(key).collect::<Vec<_>>();
        seen.sort();
        seen.dedup();
        seen.len()
    };
    let vary_process = distinct(&|point| point.process.to_ascii_uppercase()) > 1;
    let vary_voltage = distinct(&|point| point.voltage.map_or_else(String::new, head_number)) > 1;
    let vary_temperature = distinct(&|point| head_number(point.temperature)) > 1;
    labels
        .iter()
        .zip(points)
        .map(|(label, point)| {
            let Some(point) = point.filter(|_| all_parsed) else {
                return CornerColumn {
                    head: label.clone(),
                    detail: label.clone(),
                    point: None,
                };
            };
            let voltage = point
                .voltage
                .map(|volts| format!("{} V", head_number(volts)));
            let temperature = format!("{} \u{00b0}C", head_number(point.temperature));
            let mut head = Vec::new();
            if vary_process || !(vary_voltage || vary_temperature) {
                head.push(point.process.clone());
            }
            if vary_voltage && let Some(voltage) = &voltage {
                head.push(voltage.clone());
            }
            if vary_temperature {
                head.push(temperature.clone());
            }
            let detail = [Some(point.process.clone()), voltage, Some(temperature)]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join(" \u{00b7} ");
            CornerColumn {
                head: head.join(" "),
                detail,
                point: Some(point),
            }
        })
        .collect()
}

/// The unit a corner waveform's samples are in: what the producer stated, or
/// what its name says it is.
fn waveform_unit(waveform: &crate::state::WaveformData) -> String {
    waveform.unit.clone().unwrap_or_else(|| {
        let name = waveform.name.trim_start_matches('|');
        if name.starts_with("V(") {
            "V".to_owned()
        } else if name.starts_with("I(") {
            "A".to_owned()
        } else {
            String::new()
        }
    })
}

/// Build the matrix one corner family and the run that holds it answer.
///
/// `plan_labels` names the corners when the family carries no labels of its
/// own; `specs` are the limits the rows are judged by.
pub(super) fn corner_matrix(
    run: &crate::state::SimulationRun,
    family: &crate::state::AnalysisResult,
    plan_labels: Option<Vec<String>>,
    specs: &[SpecEntry],
    reference: Option<&ReferenceCorner>,
) -> CornerMatrix {
    let axis = family
        .waveforms
        .first()
        .map_or(0, |waveform| waveform.x.len());
    let labels = match &family.family_metadata {
        Some(crate::state::AnalysisResultFamilyMetadata::Corner { corner_labels, .. })
            if !corner_labels.is_empty() =>
        {
            corner_labels.clone()
        }
        _ => plan_labels
            .filter(|labels| axis == 0 || labels.len() == axis)
            .unwrap_or_else(|| (1..=axis).map(|index| format!("Point {index}")).collect()),
    };
    let columns = corner_columns(&labels);
    let width = columns.len();
    let spec_for = |name: &str| {
        specs
            .iter()
            .find(|spec| spec.measurement.eq_ignore_ascii_case(name))
    };

    // Measured outputs, in the order they are first met.
    let mut measured: Vec<CornerRow> = Vec::new();
    let mut record = |name: &str, unit: Option<&str>, column: usize, value: f64| {
        let spec = spec_for(name);
        let index = measured
            .iter()
            .position(|row| row.output.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| {
                measured.push(CornerRow {
                    output: name.to_owned(),
                    unit: spec
                        .map(|spec| spec.unit.clone())
                        .or_else(|| unit.map(str::to_owned))
                        .unwrap_or_default(),
                    spec: spec.cloned(),
                    values: vec![None; width],
                });
                measured.len() - 1
            });
        let slot = &mut measured[index].values[column];
        // Two answers for one corner keep the worse against the limit, the
        // way every other specification join reduces them.
        *slot = Some(match (*slot, spec) {
            (Some(held), Some(spec))
                if verification_margin(spec, held) < verification_margin(spec, value) =>
            {
                held
            }
            (Some(held), None) => held,
            _ => value,
        });
    };

    if let Some(metadata) = &family.family_metadata {
        for member in metadata.member_measurements() {
            let crate::state::FamilyMemberId::Corner { index, label } = &member.member else {
                continue;
            };
            let Some(column) = labels
                .iter()
                .position(|candidate| candidate == label)
                .or(Some(*index))
                .filter(|column| *column < width)
            else {
                continue;
            };
            for evidence in member
                .measurements
                .iter()
                .filter(|evidence| evidence.is_measured())
            {
                let unit = spec_for(&evidence.name).map_or("", |spec| spec.unit.as_str());
                if let Ok(Some(value)) = evidence.value_in_unit(unit)
                    && value.is_finite()
                {
                    let native = evidence.unit.as_ref().and_then(|unit| unit.symbol());
                    record(&evidence.name, native, column, value);
                }
            }
        }
    }

    let mut nominal = None;
    for analysis in run
        .analyses
        .iter()
        .filter(|analysis| verified_analysis(analysis))
    {
        let Some(solved) = analysis
            .provenance()
            .and_then(|provenance| provenance.pvt_point())
        else {
            continue;
        };
        let Some(column) = columns.iter().position(|column| {
            column
                .point
                .as_ref()
                .is_some_and(|point| point_matches(point, solved))
        }) else {
            continue;
        };
        if solved.is_nominal() {
            nominal = Some(column);
        }
        for measurement in analysis
            .measurements
            .iter()
            .filter(|measurement| measurement.passed && measurement.error.is_none())
        {
            let unit = spec_for(&measurement.name).map_or("", |spec| spec.unit.as_str());
            if let Ok(Some(value)) = measurement.value_in_unit(unit)
                && value.is_finite()
            {
                let native = measurement
                    .units
                    .as_ref()
                    .and_then(|units| units.value.symbol());
                record(&measurement.name, native, column, value);
            }
        }
    }

    let waveforms = family
        .waveforms
        .iter()
        .filter(|waveform| width > 0 && waveform.y.len() == width && waveform.x.len() == width)
        .map(|waveform| CornerRow {
            output: waveform.name.clone(),
            unit: spec_for(&waveform.name)
                .map_or_else(|| waveform_unit(waveform), |spec| spec.unit.clone()),
            spec: spec_for(&waveform.name).cloned(),
            values: waveform
                .y
                .iter()
                .map(|value| value.is_finite().then_some(*value))
                .collect(),
        });

    // Limited rows first, in the order the specifications are authored, then
    // the other measurements, then the corner waveforms.
    let mut rows = measured.into_iter().chain(waveforms).collect::<Vec<_>>();
    rows.sort_by_key(|row| {
        row.spec.as_ref().map_or(usize::MAX, |spec| {
            specs
                .iter()
                .position(|candidate| candidate == spec)
                .unwrap_or(usize::MAX - 1)
        })
    });

    let nominal = nominal.or_else(|| {
        let reference = reference?;
        let candidates = columns
            .iter()
            .enumerate()
            .filter(|(_, column)| {
                column.point.as_ref().is_some_and(|point| {
                    point.process.eq_ignore_ascii_case(&reference.process)
                        && same_quantity(point.temperature, reference.temperature)
                })
            })
            .collect::<Vec<_>>();
        match candidates.as_slice() {
            [(index, _)] => Some(*index),
            several => several
                .iter()
                .find(|(_, column)| {
                    column
                        .point
                        .as_ref()
                        .and_then(|point| point.voltage)
                        .is_some_and(|volts| same_quantity(volts, 1.0))
                })
                .map(|(index, _)| *index),
        }
    });

    CornerMatrix {
        columns,
        nominal,
        rows,
    }
}

/// What the corners toolbar can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CornerAction {
    CompareNominal,
    Export,
}

pub(super) fn corners(ui: &mut Ui, app: &mut RSpiceApp) {
    let runs = run_choice(app, |run| {
        run.analyses.iter().any(|analysis| {
            analysis.analysis_type == crate::state::AnalysisType::Corner
                && verified_analysis(analysis)
        })
    });
    let run_name = verification_run(app).map(|run| format!("Run {}", run.id));
    let reference = ReferenceCorner {
        process: app
            .state
            .sim_setup
            .reference_pvt
            .process
            .short_name()
            .to_owned(),
        temperature: app.state.sim_setup.reference_pvt.temperature_celsius,
    };
    let matrix = latest_analysis(app, crate::state::AnalysisType::Corner)
        .zip(verification_run(app))
        .map(|(family, run)| {
            corner_matrix(
                run,
                family,
                corner_point_labels(app, family),
                &verification_specs(app),
                Some(&reference),
            )
        });
    let compare = app.state.workbench.verification.corner_compare_nominal;

    let mut actions = Vec::new();
    let mut toolbar_actions = Vec::new();
    if let Some(matrix) = matrix.as_ref().filter(|matrix| !matrix.rows.is_empty()) {
        if matrix.nominal.is_some() {
            actions.push(CornerAction::CompareNominal);
            toolbar_actions.push(ToolbarAction::new("Compare nominal").toggled(compare));
        }
        actions.push(CornerAction::Export);
        toolbar_actions.push(ToolbarAction::new("Export CSV"));
    }
    let summary = matrix
        .as_ref()
        .filter(|matrix| !matrix.rows.is_empty())
        .map(|matrix| {
            let summary = matrix.summary();
            if runs.offers_choice() {
                summary
            } else {
                append_run(summary, run_name.as_deref())
            }
        });
    let output = PageToolbar {
        title: "Process corners",
        runs: &runs,
        summary,
        actions: &toolbar_actions,
    }
    .show(ui);
    match output.action.and_then(|index| actions.get(index)) {
        Some(CornerAction::CompareNominal) => {
            app.state.workbench.verification.corner_compare_nominal = !compare;
        }
        Some(CornerAction::Export) => export_active_corner_matrix(app),
        None => {}
    }
    if let Some(index) = output.run {
        select_verification_run(&mut app.state.simulation, index);
    }
    action_problem_line(ui, app);

    let Some(matrix) = matrix else {
        let text = run_name.map_or_else(
            || "No corner run. Enable corners in the run set, then run the plan.".to_owned(),
            |run| {
                format!("No corner run in {run}. Enable corners in the run set, then run the plan.")
            },
        );
        if empty_line(ui, &text, Some("Open run set")) {
            Command::SimulationPage(crate::workbench::state::SimulationPage::RunSet).execute(app);
        }
        return;
    };
    if matrix.rows.is_empty() {
        if empty_line(
            ui,
            "This corner run kept no outputs. Add measurements to the plan's outputs, then run it again.",
            Some("Open outputs"),
        ) {
            Command::SimulationPage(crate::workbench::state::SimulationPage::Outputs).execute(app);
        }
        return;
    }
    corner_table(ui, &matrix, compare && matrix.nominal.is_some());
}

fn corner_table(ui: &mut Ui, matrix: &CornerMatrix, compare: bool) {
    let t = Tokens::get(ui.ctx());
    let head_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let column_width = |head: &str| {
        let measured = ui
            .painter()
            .layout_no_wrap(head.to_owned(), head_font.clone(), t.color.text)
            .size()
            .x;
        (measured + 20.0).max(CORNER_COLUMN_MIN_W)
    };
    let limited = matrix.rows.iter().any(|row| row.spec.is_some());
    let corners = matrix.corner_indices();
    let mut columns = vec![Column::name("Output", 150.0)];
    if limited {
        columns.push(Column::words("Spec", 104.0));
    }
    if let Some(nominal) = matrix.nominal {
        let column = &matrix.columns[nominal];
        columns.push(
            Column::number("Nominal", column_width("Nominal"))
                .tooltip(format!("Nominal: {}", column.detail)),
        );
    }
    for index in &corners {
        let column = &matrix.columns[*index];
        columns
            .push(Column::number(&column.head, column_width(&column.head)).tooltip(&column.detail));
    }
    if limited {
        columns.push(Column::words("Pass/fail", 70.0));
    }
    let height = ui.available_height();
    DataTable::new("verify.corners.matrix", &columns, matrix.rows.len())
        .max_height(height)
        .show(ui, |index| {
            let row = &matrix.rows[index];
            let nominal = matrix.nominal.and_then(|column| row.values[column]);
            let value_cell = |value: Option<f64>, relative: bool| {
                let Some(value) = value else {
                    return Cell::absent();
                };
                let text = match nominal.filter(|_| relative) {
                    Some(reference) => {
                        let delta = value - reference;
                        let sign = if delta > 0.0 { "+" } else { "" };
                        format!("{sign}{}", format_value(delta, &row.unit))
                    }
                    None => format_value(value, &row.unit),
                };
                let cell = Cell::number(text);
                match row.verdict_of(value) {
                    Some(verdict) => cell.judged(verdict),
                    None => cell,
                }
            };
            let mut cells = vec![Cell::text(&row.output)];
            if limited {
                cells.push(
                    row.spec
                        .as_ref()
                        .map_or_else(Cell::empty, |spec| Cell::number(spec.limit_text())),
                );
            }
            if matrix.nominal.is_some() {
                cells.push(value_cell(nominal, false));
            }
            cells.extend(
                corners
                    .iter()
                    .map(|column| value_cell(row.values[*column], compare)),
            );
            if limited {
                cells.push(row.verdict().map_or_else(Cell::empty, Cell::verdict));
            }
            cells
        });
}

pub(super) fn export_active_corner_matrix(app: &mut RSpiceApp) {
    let result_id =
        latest_analysis(app, crate::state::AnalysisType::Corner).map(|result| result.id);
    let analysis_index = result_id.and_then(|result_id| {
        app.state.simulation.active_run().and_then(|run| {
            run.analyses.iter().position(|analysis| {
                analysis.id == result_id
                    && analysis.analysis_type == crate::state::AnalysisType::Corner
                    && verified_analysis(analysis)
            })
        })
    });
    if let Some(index) = analysis_index {
        app.state.simulation.select_analysis(index);
        Command::ExportWaveformsCsv.execute(app);
        app.state.workbench.verification.action_problem = None;
    } else {
        app.state.workbench.verification.action_problem =
            Some("Nothing to export: this run holds no completed corner result.".to_owned());
    }
}

pub(super) fn corner_point_labels(
    app: &RSpiceApp,
    result: &crate::state::AnalysisResult,
) -> Option<Vec<String>> {
    use crate::simulation::plan::AnalysisDraft;

    let provenance = result.provenance.as_ref()?;
    let plan = app.state.sim_setup.stable_analysis_plan().ok()?;
    let instance = plan
        .instances()
        .iter()
        .find(|instance| instance.id() == provenance.authored_source_instance_id())?;
    if instance.modified_revision() != provenance.source_revision() {
        return None;
    }
    let AnalysisDraft::Corner(state) = instance.draft() else {
        return None;
    };
    let labels = state
        .to_config(
            &app.state.sim_setup.run_set,
            app.state.sim_setup.reference_pvt,
        )
        .ok()?
        .corner_names();
    let points = result.waveforms.first()?.x.len();
    (labels.len() == points).then_some(labels)
}

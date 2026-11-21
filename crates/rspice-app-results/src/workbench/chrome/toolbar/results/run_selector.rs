//! The run selector: which run of its plan a view shows. "Latest" follows
//! the plan's newest run; any other row keeps the view on that run until
//! Latest is chosen or the plan runs again, as ADS picks a display's dataset
//! from a toolbar list and Assembler's History tab lists a test's runs and
//! its Results tab turns to the history a new run makes.

use egui::Sense;

use crate::product::DatasetId;
use crate::results::visualization_document::{ResultDocumentTrackingMode, WorkingViewSource};
use crate::state::{SimulationRun, SimulationRunLifecycle};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark};
use crate::workbench::chrome::document_bar::results::{
    activate_result_dataset, active_result_document, show_result_dataset,
};
use crate::workbench::design_system::{WorkbenchIcon, labeled_icon_button_sized};
use crate::workbench::docks::navigator::results_rows::result_browser_row_height;
use crate::workbench::documents::result_document::working_view::{self, source_of};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::WorkspaceDocumentId;

use rspice_app_workbench::workbench::chrome::toolbar::{explicit_label_width, take_tool_slot};

/// The list's popup, open while the reader picks a run.
pub fn list_id() -> egui::Id {
    egui::Id::new("workbench.results.run-selector")
}

/// The most rows the list shows before it scrolls.
pub const VISIBLE_ROWS: usize = 14;
/// The inset of a row's text from the row's edges.
const ROW_PAD: f32 = 8.0;
/// The room between a row's name and the column of start times.
const NAME_GAP: f32 = 24.0;
/// The room between the start time and the state after it.
const STATE_GAP: f32 = 12.0;
/// The gutter the pinned mark takes, while any run is pinned.
const PIN_GUTTER: f32 = 12.0;

/// What the selector offers for the run Results shows: the plan's view, its
/// runs newest first, and whether the view follows the newest.
pub struct RunChoice<'a> {
    pub source: WorkingViewSource,
    pub shown: &'a SimulationRun,
    pub runs: Vec<&'a SimulationRun>,
    pub following: bool,
}

/// The choice the selector offers while Results shows a run of a plan.
/// `None` for a saved document, which has its own Latest/Pinned control, and
/// for an imported run, whose view reads that run alone.
pub fn run_choice(state: ResultsReads<'_>) -> Option<RunChoice<'_>> {
    let Some(WorkspaceDocumentId::ResultDataset(dataset_id)) =
        active_result_document(state.model, state.workbench)
    else {
        return None;
    };
    let retained = &state.model.simulation.retained.runs;
    let shown = retained.iter().find(|run| run.dataset_id == dataset_id)?;
    let source = source_of(shown);
    let WorkingViewSource::Plan { .. } = source else {
        return None;
    };
    let mut runs: Vec<&SimulationRun> = retained
        .iter()
        .filter(|run| source_of(run) == source)
        .collect();
    runs.sort_by_key(|run| std::cmp::Reverse(run.id));
    let following = working_view::view_of(state, source)
        .and_then(|view| state.model.workspace.content.visualization_document(view))
        .is_none_or(|view| view.tracking().mode == ResultDocumentTrackingMode::Latest);
    Some(RunChoice {
        source,
        shown,
        runs,
        following,
    })
}

/// Show `run` through its plan's view, or follow the plan's newest run when
/// `run` is `None`, and make the view's tab name it.
pub fn choose(state: &mut ResultsParts<'_>, run: Option<DatasetId>) -> Result<(), String> {
    let choice = run_choice(state.results_reads()).ok_or("Results shows no run of a plan")?;
    let (source, shown) = (choice.source, choice.shown.dataset_id);
    choose_in(state, source, run, shown)
}

/// Show the run of `dataset`, as choosing its row does: a plan's run
/// through the plan's view, kept until Latest is chosen or the plan runs
/// again; an imported run through its own.
pub fn show_run(state: &mut ResultsParts<'_>, dataset: DatasetId) -> Result<(), String> {
    let run = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .ok_or("the run is no longer in the run history")?;
    match source_of(run) {
        source @ WorkingViewSource::Plan { .. } => choose_in(state, source, Some(dataset), dataset),
        WorkingViewSource::Imported { .. } => {
            if show_result_dataset(state, &dataset) {
                Ok(())
            } else {
                Err("Results is not the workspace showing".to_owned())
            }
        }
    }
}

/// Choose `run` for the plan view of `source`, making the view from `any`,
/// a run of the plan, when the plan has none yet.
fn choose_in(
    state: &mut ResultsParts<'_>,
    source: WorkingViewSource,
    run: Option<DatasetId>,
    any: DatasetId,
) -> Result<(), String> {
    let view = match working_view::view_of(state.results_reads(), source) {
        Some(view) => view,
        None => {
            activate_result_dataset(state, &any);
            working_view::sync(state);
            working_view::view_of(state.results_reads(), source)
                .ok_or("this plan's runs have no view to show them through")?
        }
    };
    working_view::choose_run(state, view, run)?;
    if let Some(shown) = state
        .model
        .simulation
        .active_run()
        .map(|run| run.dataset_id)
    {
        crate::workbench::chrome::document_bar::record_active_document(
            state.workbench,
            &WorkspaceDocumentId::ResultDataset(shown),
        );
    }
    Ok(())
}

/// What the list says of a run's state, and the problem mark in front of
/// it: nothing for a run that completed and kept its results, which is what
/// a run of the list is unless it says otherwise.
pub fn run_state(run: &SimulationRun) -> Option<(&'static str, Option<Problem>)> {
    match run.lifecycle {
        SimulationRunLifecycle::Completed if run.success => run
            .results_storage_stop()
            .is_some_and(|stop| stop.kept_nothing())
            .then_some(("Not kept", Some(Problem::Warning))),
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::Failed => {
            Some(("Failed", Some(Problem::Failure)))
        }
        SimulationRunLifecycle::Preparing => Some(("Preparing", None)),
        SimulationRunLifecycle::Running => Some(("Running", None)),
        SimulationRunLifecycle::Cancelling => Some(("Stopping", None)),
        SimulationRunLifecycle::Aborted => Some(("Stopped", None)),
        SimulationRunLifecycle::Interrupted => Some(("Interrupted", Some(Problem::Warning))),
        SimulationRunLifecycle::LegacyUnknown => None,
    }
}

/// When `run` started on this host's clock, as the list reads it beside the
/// run's name: `08:50:32` for a run started today, `20 Nov 08:50:32` for one
/// started earlier this year and `20 Nov 2025 08:50:32` before that. Runs
/// that share a second are told apart by their names, so no finer unit is
/// shown.
fn start_clock(run: &SimulationRun, now: std::time::Duration) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    use rspice_app_state::time_compat::local_date_time;
    if !(run.timestamp.is_finite() && run.timestamp >= 0.0) {
        return String::new();
    }
    let Some(started) = local_date_time((run.timestamp * 1_000.0) as u64) else {
        return String::new();
    };
    let clock = format!(
        "{:02}:{:02}:{:02}",
        started.hour(),
        started.minute(),
        started.second()
    );
    let today = local_date_time(now.as_millis() as u64).map(|now| (now.year(), now.date()));
    let month = MONTHS[usize::from(u8::from(started.month())) - 1];
    match today {
        Some((_, date)) if date == started.date() => clock,
        Some((year, _)) if year == started.year() => {
            format!("{} {month} {clock}", started.day())
        }
        _ => format!("{} {month} {} {clock}", started.day(), started.year()),
    }
}

/// The selector on the Results toolbar, while Results shows a run of a plan:
/// "Latest" or the run's name, and the list of the plan's runs below it.
/// Whether it drew anything, for the rule after it.
pub fn show(ui: &mut egui::Ui, state: &mut ResultsParts<'_>, layout: LayoutSpec) -> bool {
    let Some(choice) = run_choice(state.results_reads()) else {
        return false;
    };
    let label = if choice.following {
        "Latest".to_owned()
    } else {
        choice.shown.name().to_owned()
    };
    let hover = if choice.following {
        format!(
            "Showing {}, the newest run of this plan. Choose a run to keep it shown until the plan runs again.",
            choice.shown.label
        )
    } else {
        format!(
            "Showing {} until the plan runs again. Choose Latest to show the newest run now.",
            choice.shown.label
        )
    };
    let width = explicit_label_width(ui, &label, !choice.following);
    if !take_tool_slot(ui, layout, width) {
        return true;
    }
    let button = labeled_icon_button_sized(
        ui,
        WorkbenchIcon::History,
        &label,
        !choice.following,
        width,
        layout.toolbar_control_height,
    )
    .on_hover_text(hover);
    let shown = choice.shown.dataset_id;
    let following = choice.following;
    let now = rspice_app_state::time_compat::unix_epoch();
    let latest = RunRow::latest(choice.runs.first().map_or("", |run| run.name()));
    let runs = choice.runs;
    let mut picked: Option<Option<DatasetId>> = None;
    egui::Popup::menu(&button).id(list_id()).show(|ui| {
        let t = Tokens::get(ui.ctx());
        let row_height = result_browser_row_height(&t);
        let rows: Vec<RunRow> = runs.iter().map(|run| RunRow::of(run, now)).collect();
        // The list is as wide as its widest row, and never narrower than the
        // selector it drops from.
        let columns = Columns::measure(ui, &latest, &rows);
        let width = columns.width().max(button.rect.width());
        ui.set_width(width);
        ui.spacing_mut().item_spacing.y = 0.0;
        if run_row(ui, &latest, &columns, width, row_height, following).clicked() {
            picked = Some(None);
        }
        let (rule, _) = ui.allocate_exact_size(egui::vec2(width, 9.0), Sense::hover());
        ui.painter().hline(
            rule.x_range(),
            rule.center().y,
            egui::Stroke::new(1.0, t.color.border),
        );
        // Only the rows in view are laid out, so a history of hundreds of
        // runs costs a frame no more than a page of them.
        egui::ScrollArea::vertical()
            .id_salt(list_id().with("rows"))
            .max_height(row_height * VISIBLE_ROWS as f32)
            .show_rows(ui, row_height, rows.len(), |ui, range| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for row in &rows[range] {
                    let selected = !following && row.dataset == Some(shown);
                    if run_row(ui, row, &columns, width, row_height, selected).clicked() {
                        picked = Some(row.dataset);
                    }
                }
            });
    });
    if let Some(run) = picked {
        ui.close();
        if let Err(error) = choose(state, run) {
            state
                .workbench
                .prompt(format!("Could not show that run: {error}."));
        }
    }
    true
}

/// One row of the list, taken for each run while the list is open.
struct RunRow {
    dataset: Option<DatasetId>,
    name: String,
    /// What follows the name in faint ink: the run Latest stands for.
    follows: String,
    started: String,
    stamp: String,
    state: Option<(&'static str, Option<Problem>)>,
    pinned: bool,
}

impl RunRow {
    fn latest(newest: &str) -> Self {
        Self {
            dataset: None,
            name: "Latest".to_owned(),
            follows: if newest.is_empty() {
                String::new()
            } else {
                format!(" \u{b7} {newest}")
            },
            started: String::new(),
            stamp: String::new(),
            state: None,
            pinned: false,
        }
    }

    fn of(run: &SimulationRun, now: std::time::Duration) -> Self {
        Self {
            dataset: Some(run.dataset_id),
            name: run.name().to_owned(),
            follows: String::new(),
            started: start_clock(run, now),
            stamp: run.start_stamp().unwrap_or_default(),
            state: run_state(run),
            pinned: run.retention().is_pinned(),
        }
    }
}

/// The widths of the list's columns, from the rows it holds.
struct Columns {
    pin: f32,
    name: f32,
    started: f32,
    /// The problem marks' column, while any state wears one, so every
    /// state's words start together.
    mark: f32,
    state: f32,
    /// The Latest row, which spans the columns with its one phrase.
    latest: f32,
}

impl Columns {
    fn measure(ui: &egui::Ui, latest: &RunRow, rows: &[RunRow]) -> Self {
        let painter = ui.painter();
        let width = |text: &str, font: egui::FontId| {
            if text.is_empty() {
                0.0
            } else {
                painter
                    .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
                    .size()
                    .x
            }
        };
        let sans = theme::sans(tokens::FS_0, FontWeight::Regular);
        let mono = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
        let widest =
            |measure: &dyn Fn(&RunRow) -> f32| rows.iter().map(measure).fold(0.0_f32, f32::max);
        Self {
            pin: if rows.iter().any(|row| row.pinned) {
                PIN_GUTTER
            } else {
                0.0
            },
            name: widest(&|row| width(&row.name, sans.clone())),
            started: widest(&|row| width(&row.started, mono.clone())),
            mark: if rows
                .iter()
                .any(|row| row.state.is_some_and(|(_, problem)| problem.is_some()))
            {
                PROBLEM_MARK_ADVANCE
            } else {
                0.0
            },
            state: widest(&|row| {
                row.state
                    .map_or(0.0, |(words, _)| width(words, sans.clone()))
            }),
            latest: width(&latest.name, sans.clone()) + width(&latest.follows, sans.clone()),
        }
    }

    /// The list's width: its widest row between the rows' insets.
    fn width(&self) -> f32 {
        let started = if self.started > 0.0 {
            NAME_GAP + self.started
        } else {
            0.0
        };
        let state = if self.state > 0.0 {
            STATE_GAP + self.mark + self.state
        } else {
            0.0
        };
        let runs = self.pin + self.name + started + state;
        ROW_PAD * 2.0 + runs.max(self.pin + self.latest)
    }
}

/// A row of the list: the pinned mark, the name, the start time right
/// aligned in its column and any state that is not a plain completion, the
/// chosen row in the selection colour.
fn run_row(
    ui: &mut egui::Ui,
    row: &RunRow,
    columns: &Columns,
    width: f32,
    height: f32,
    selected: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &row.name,
        )
    });
    let painter = ui.painter_at(rect);
    if selected || response.hovered() {
        painter.rect_filled(
            rect,
            t.radius,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    let sans = theme::sans(tokens::FS_0, FontWeight::Regular);
    let mono = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let y = rect.center().y;
    let left = rect.left() + ROW_PAD;
    if row.pinned {
        painter.circle_filled(egui::pos2(left + 3.0, y), 2.5, t.color.accent);
    }
    let name_left = left + columns.pin;
    let name_end = painter
        .text(
            egui::pos2(name_left, y),
            egui::Align2::LEFT_CENTER,
            &row.name,
            sans.clone(),
            t.color.text,
        )
        .right();
    if !row.follows.is_empty() {
        painter.text(
            egui::pos2(name_end, y),
            egui::Align2::LEFT_CENTER,
            &row.follows,
            sans.clone(),
            t.color.text_faint,
        );
    }
    let started_right = name_left + columns.name + NAME_GAP + columns.started;
    if !row.started.is_empty() {
        painter.text(
            egui::pos2(started_right, y),
            egui::Align2::RIGHT_CENTER,
            &row.started,
            mono,
            t.color.text_dim,
        );
    }
    if let Some((words, problem)) = row.state {
        let mark_left = started_right + STATE_GAP;
        if let Some(problem) = problem {
            paint_problem_mark(
                &painter,
                egui::Rect::from_center_size(
                    egui::pos2(mark_left + PROBLEM_MARK_SIDE * 0.5, y),
                    egui::Vec2::splat(PROBLEM_MARK_SIDE),
                ),
                problem,
            );
        }
        painter.text(
            egui::pos2(mark_left + columns.mark, y),
            egui::Align2::LEFT_CENTER,
            words,
            sans,
            t.color.text_dim,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let hover = match (row.stamp.is_empty(), row.pinned) {
        (true, false) => return response,
        (false, false) => format!("Started {}", row.stamp),
        (true, true) => "Pinned: retention never discards this run".to_owned(),
        (false, true) => format!(
            "Started {}. Pinned: retention never discards this run",
            row.stamp
        ),
    };
    response.on_hover_text(hover)
}

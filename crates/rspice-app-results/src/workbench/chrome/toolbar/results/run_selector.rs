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
use crate::workbench::chrome::document_bar::results::active_result_document;
use crate::workbench::design_system::{WorkbenchIcon, labeled_icon_button_sized};
use crate::workbench::documents::result_document::working_view::{self, source_of};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::WorkspaceDocumentId;

use rspice_app_workbench::workbench::chrome::toolbar::{explicit_label_width, take_tool_slot};

/// The list's popup, open while the reader picks a run.
pub fn list_id() -> egui::Id {
    egui::Id::new("workbench.results.run-selector")
}

/// The widest the list grows before its columns stop growing.
const LIST_WIDTH: f32 = 420.0;
/// The most rows the list shows before it scrolls.
pub const VISIBLE_ROWS: usize = 14;

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
    let source = choice.source;
    let view = match working_view::view_of(state.results_reads(), source) {
        Some(view) => view,
        None => {
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

/// The words a run's state reads in the list.
pub fn status_words(run: &SimulationRun) -> &'static str {
    match run.lifecycle {
        SimulationRunLifecycle::Completed if run.success => "Completed",
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::Failed => "Failed",
        SimulationRunLifecycle::Preparing => "Preparing",
        SimulationRunLifecycle::Running => "Running",
        SimulationRunLifecycle::Cancelling => "Cancelling",
        SimulationRunLifecycle::Aborted => "Cancelled",
        SimulationRunLifecycle::Interrupted => "Interrupted",
        SimulationRunLifecycle::LegacyUnknown => "Unknown",
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
    let runs = choice.runs;
    let mut picked: Option<Option<DatasetId>> = None;
    egui::Popup::menu(&button).id(list_id()).show(|ui| {
        ui.set_min_width(LIST_WIDTH);
        if run_row(ui, &RunRow::latest(), following).clicked() {
            picked = Some(None);
        }
        ui.separator();
        let t = Tokens::get(ui.ctx());
        // Only the rows in view are laid out, so a history of hundreds of
        // runs costs a frame no more than a page of them.
        egui::ScrollArea::vertical()
            .id_salt(list_id().with("rows"))
            .max_height(t.metrics.row_h * VISIBLE_ROWS as f32)
            .show_rows(ui, t.metrics.row_h, runs.len(), |ui, range| {
                for run in &runs[range] {
                    let row = RunRow::of(run);
                    let selected = !following && row.dataset == Some(shown);
                    if run_row(ui, &row, selected).clicked() {
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

/// One row of the list, taken for each row in view while the list is open.
struct RunRow {
    dataset: Option<DatasetId>,
    name: String,
    started: String,
    status: &'static str,
    pinned: bool,
}

impl RunRow {
    fn latest() -> Self {
        Self {
            dataset: None,
            name: "Latest".to_owned(),
            started: String::new(),
            status: "",
            pinned: false,
        }
    }

    fn of(run: &SimulationRun) -> Self {
        Self {
            dataset: Some(run.dataset_id),
            name: run.name().to_owned(),
            started: run.start_stamp().unwrap_or_default(),
            status: status_words(run),
            pinned: run.retention().is_pinned(),
        }
    }
}

/// A row of the list: the pin, the name, the start and the state in their
/// columns, the chosen row marked.
fn run_row(ui: &mut egui::Ui, row: &RunRow, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().max(LIST_WIDTH), t.metrics.row_h),
        Sense::click(),
    );
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
            0.0,
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
    if row.pinned {
        painter.text(
            egui::pos2(rect.left() + 12.0, y),
            egui::Align2::CENTER_CENTER,
            "\u{25cf}",
            mono.clone(),
            t.color.accent,
        );
    }
    let name_right = rect.left() + rect.width() * 0.42;
    painter
        .with_clip_rect(egui::Rect::from_min_max(
            rect.left_top(),
            egui::pos2(name_right - 6.0, rect.bottom()),
        ))
        .text(
            egui::pos2(rect.left() + 24.0, y),
            egui::Align2::LEFT_CENTER,
            &row.name,
            sans,
            t.color.text,
        );
    painter.text(
        egui::pos2(name_right, y),
        egui::Align2::LEFT_CENTER,
        &row.started,
        mono.clone(),
        t.color.text_dim,
    );
    painter.text(
        egui::pos2(rect.right() - 8.0, y),
        egui::Align2::RIGHT_CENTER,
        row.status,
        mono,
        if row.status == "Failed" {
            t.color.warn
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    if row.pinned {
        response.on_hover_text("Pinned: retention never discards this run")
    } else {
        response
    }
}

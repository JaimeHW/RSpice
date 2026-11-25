//! The toolbar's run chip: what Run runs, and where the reader picks it.
//!
//! The chip names what a press of Run will do: the analysis and its main
//! number, over the plan it belongs to. A press opens a popover that picks
//! what Run runs where the reader is, without a trip to the Simulate
//! workspace: which of the project's plans is active, and which of its
//! analyses take part. It never changes what an analysis is: a row opens the
//! analysis on the Analyses page, where it is edited, and the analysis
//! catalogue and the plan manager stand under a rule at its foot.
//!
//! The chip is measured, painted, and announced from one derivation of its
//! copy, because its width, its two clipped lines, and its accessibility label
//! all describe the same plan. Naming that plan from a literal was how the chip
//! kept announcing a fixture plan after a rename, while the live summary beside
//! it made the stale name read as authoritative.

use egui::{Rect, Ui, Vec2, pos2, vec2};
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::plan_model::AnalysisInstance;
use rspice_simulation_contract::run_set::RunSetDimensionKind;

use super::pvt_temperature_label;
use crate::product::{AnalysisInstanceId, ProcessCorner, SimulationPlanId};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, LineHeights, RowKey, TABLE_EDGE_INSET, TableState,
    fitted_width,
};
use crate::ui::widgets::{Button, SegmentedWidth, compact_height, name_control, segmented};
use crate::workbench::app_state::Model;
use crate::workbench::design_system::{WorkbenchIcon, elide_middle, elide_text};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::layout::RUN_CONFIG_CHIP_MAX_WIDTH;

/// What the reader asked of the run chip this frame. None of it changes what
/// an analysis is or whether the plan holds it.
#[derive(Debug, Clone)]
pub(in crate::workbench) enum RunChipRequest {
    /// Open the plan manager.
    ManagePlans,
    /// Open the analysis catalogue, the Analyses page's add flow.
    AddAnalysis,
    /// Make this plan the one Run runs.
    ActivatePlan(SimulationPlanId),
    /// Run includes this analysis (true) or leaves it out (false).
    Include(AnalysisInstanceId, bool),
    /// Open this analysis on Simulate › Analyses.
    Open(AnalysisInstanceId),
    /// Make (process, temperature in °C) the reference a plain Run runs at.
    Reference(ProcessCorner, f64),
    /// What the reader typed for the reference temperature is not one: why.
    Unreadable(String),
}

/// The popover's heads and its words.
const PLAN_HEAD: &str = "Plan";
const ANALYSIS_HEAD: &str = "Analysis";
const REFERENCE_HEAD: &str = "Reference";
const TEMPERATURE: &str = "Temperature";
const PROCESS: &str = "Process";
const CELSIUS: &str = "\u{00b0}C";
const NO_ANALYSIS: &str = "This plan runs no analysis.";
const ADD_ANALYSIS: &str = "Add analysis\u{2026}";
const MANAGE_PLANS: &str = "Manage plans\u{2026}";
/// Air between the footer's two buttons, and over and under them.
const FOOTER_GAP: f32 = 8.0;
const FOOTER_PAD: f32 = 6.0;
/// Air between a Reference row's label and its control, and between a typed
/// temperature and its unit.
const LABEL_GAP: f32 = 8.0;
const UNIT_GAP: f32 = 4.0;
/// A field's text inset from its ends.
const FIELD_PAD: f32 = 8.0;
/// What the popover keeps clear of the screen's edges.
const SCREEN_MARGIN: f32 = 8.0;
/// The processes a reference can name, fast to slow, as a corner table
/// orders them.
const PROCESSES: [ProcessCorner; 5] = [
    ProcessCorner::FF,
    ProcessCorner::FS,
    ProcessCorner::TT,
    ProcessCorner::SF,
    ProcessCorner::SS,
];

/// Every string the run chip measures, paints, and announces.
struct RunConfigChipCopy {
    title: String,
    summary: String,
    accessibility: String,
}

fn run_config_chip_copy(model: &Model) -> RunConfigChipCopy {
    let plan = model.sim_setup.active_plan_name().as_str().to_owned();
    let title = analysis_title(model);
    let pvt = pvt_summary(model);
    RunConfigChipCopy {
        accessibility: format!(
            "Analysis {title}, plan {plan}, {pvt}; choose the plan, the analyses and \
             the reference PVT Run runs"
        ),
        summary: format!("{plan} \u{00b7} {pvt}"),
        title,
    }
}

/// What Run runs at, as the chip's second line says it: the reference,
/// `TT · 27 °C`, where a plain Run runs at that one point, by its temperature
/// alone where no model library defines a process to tell it by; how many
/// points otherwise.
fn pvt_summary(model: &Model) -> String {
    let run_set = &model.sim_setup.run_set;
    let points = configured_pvt_count(model);
    let at_reference = points == 1
        && run_set
            .enabled_dimension_of(RunSetDimensionKind::ProcessSection)
            .is_none()
        && run_set
            .enabled_dimension_of(RunSetDimensionKind::Temperature)
            .is_none();
    if !at_reference {
        return format!("{points} PVT");
    }
    let reference = model.sim_setup.reference_pvt;
    let temperature = pvt_temperature_label(
        reference.temperature_celsius,
        model.preferences.quantity_presentation_policy(),
    );
    if library_processes(model, reference.process).is_empty() {
        temperature
    } else {
        format!("{} \u{00b7} {temperature}", reference.process.short_name())
    }
}

/// `TT · 27 °C`: a reference as the prompt that confirms it names it.
pub(in crate::workbench) fn reference_label(
    model: &Model,
    process: ProcessCorner,
    celsius: f64,
) -> String {
    format!(
        "{} \u{00b7} {}",
        process.short_name(),
        pvt_temperature_label(celsius, model.preferences.quantity_presentation_policy())
    )
}

/// The processes a reference can name, fast to slow: each one a model
/// library of the project declares, as the run looks a process up, and the
/// current one, which stays offered whatever the libraries say. Empty where
/// no library defines a corner, so there is nothing to choose among.
///
/// A corner whose name is none of the five is not offered: the reference
/// names one of them.
fn library_processes(model: &Model, current: ProcessCorner) -> Vec<ProcessCorner> {
    let catalog = model.model_library_manager.catalog();
    let declared = |process: ProcessCorner| {
        catalog
            .libraries()
            .any(|library| library.declares_process(process.short_name()))
    };
    let defines = catalog
        .libraries()
        .any(|library| !library.corners.is_empty())
        || PROCESSES.into_iter().any(declared);
    if !defines {
        return Vec::new();
    }
    PROCESSES
        .into_iter()
        .filter(|process| *process == current || declared(*process))
        .collect()
}

/// The analyses Run is for: the plan's enabled analyses that no other
/// enabled analysis runs as its prerequisite, in plan order. An AC sweep's
/// operating point runs because the sweep needs it, so the sweep is what the
/// chip names.
fn leading_analyses(model: &Model) -> Vec<&AnalysisInstance> {
    let enabled: Vec<&AnalysisInstance> = model.sim_setup.enabled_analysis_instances().collect();
    let prerequisite = |candidate: &AnalysisInstance| {
        enabled.iter().any(|other| {
            other
                .dependencies()
                .iter()
                .any(|dependency| dependency.target() == candidate.id())
        })
    };
    let leading: Vec<&AnalysisInstance> = enabled
        .iter()
        .copied()
        .filter(|candidate| !prerequisite(candidate))
        .collect();
    if leading.is_empty() { enabled } else { leading }
}

/// The analysis Run runs, as the chip's first line names it: the lead
/// analysis and its main number, and how many more run beside it.
fn analysis_title(model: &Model) -> String {
    let leading = leading_analyses(model);
    let Some(lead) = leading.first() else {
        return "No analysis".to_owned();
    };
    let name = analysis_name(lead.draft());
    match leading.len() - 1 {
        0 => name,
        more => format!("{name} +{more}"),
    }
}

/// `Transient · 1 ms`, `AC · 1 Hz to 1 GHz`, `Operating point`: a kind with
/// main numbers by them, any other kind by its name.
fn analysis_name(draft: &AnalysisDraft) -> String {
    match draft {
        AnalysisDraft::OperatingPoint(_) => "Operating point".to_owned(),
        AnalysisDraft::Transient(setup) => {
            format!("Transient \u{00b7} {}", spelled(&setup.stop, "s"))
        }
        AnalysisDraft::Ac(setup) => format!(
            "AC \u{00b7} {} to {}",
            spelled(&setup.fstart, "Hz"),
            spelled(&setup.fstop, "Hz")
        ),
        other => other.kind().label().to_owned(),
    }
}

/// What a popover row says after [`analysis_name`]: a transient's step, an
/// AC sweep's points. `None` for a kind its name says whole, or a number the
/// plan leaves blank.
fn analysis_detail(draft: &AnalysisDraft) -> Option<String> {
    let given = |text: &str| Some(text.trim().to_owned()).filter(|text| !text.is_empty());
    match draft {
        AnalysisDraft::Transient(setup) => {
            given(&setup.step).map(|step| format!("step {}", spelled(&step, "s")))
        }
        AnalysisDraft::Ac(setup) => given(&setup.points).map(|points| match setup.sweep {
            0 => format!("{points} points/decade"),
            1 => format!("{points} points/octave"),
            _ => format!("{points} points"),
        }),
        _ => None,
    }
}

/// `text` as a reader writes the quantity, `1 ms`, or as typed when it is not
/// a number, such as a design variable.
fn spelled(text: &str, unit: &str) -> String {
    rspice_app_state::quantity::parse_engineering_value(text).map_or_else(
        |_| text.trim().to_owned(),
        |value| rspice_results::verification::format_bound(value, unit),
    )
}

/// Chip box for measured copy: the icon gutter, the wider of the two lines,
/// and the chevron gutter, capped at what the toolbar reserves for it.
fn run_config_chip_width(title_width: f32, summary_width: f32) -> f32 {
    (31.0 + title_width.max(summary_width) + 22.0).min(RUN_CONFIG_CHIP_MAX_WIDTH)
}

/// Draw the run chip over `model`, enabled as `manage`, the dispatcher's
/// answer for Manage plans, says. A press opens the popover that picks the
/// plan and the analyses Run runs, whose Add analysis… is enabled as `add`
/// says; returns what the reader asked of it.
pub(in crate::workbench) fn run_config_selector(
    ui: &mut egui::Ui,
    model: &Model,
    manage: CommandAvailability,
    add: CommandAvailability,
    height: f32,
) -> Option<RunChipRequest> {
    let t = Tokens::get(ui.ctx());
    let copy = run_config_chip_copy(model);
    let availability = manage;
    let enabled = availability == CommandAvailability::Available && ui.is_enabled();
    let title_width = ui
        .painter()
        .layout_no_wrap(
            copy.title.clone(),
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text,
        )
        .size()
        .x;
    let summary_width = ui
        .painter()
        .layout_no_wrap(
            copy.summary.clone(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .size()
        .x;
    let width = run_config_chip_width(title_width, summary_width);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, &copy.accessibility)
    });
    let popover = ui.make_persistent_id("workbench.run_chip.popover");
    let open = egui::Popup::is_id_open(ui.ctx(), popover);

    let hovered = enabled && (response.hovered() || open);
    ui.painter().rect(
        rect,
        t.radius,
        if hovered {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
        egui::Stroke::new(
            1.0,
            if hovered {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Sliders.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    // At its cap the chip shortens its lines with an ellipsis rather than
    // cutting a glyph at the chevron's gutter.
    let room = rect.width() - 31.0 - 22.0;
    let title_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let summary_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y - 6.5),
        egui::Align2::LEFT_CENTER,
        elide_text(ui, &copy.title, &title_font, room),
        title_font,
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y + 7.0),
        egui::Align2::LEFT_CENTER,
        elide_text(ui, &copy.summary, &summary_font, room),
        summary_font,
        t.color.text_faint,
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.right() - 10.0, rect.center().y),
            Vec2::splat(11.0),
        ),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    if enabled && response.clicked() {
        egui::Popup::toggle_id(ui.ctx(), popover);
    }
    let mut request = None;
    if egui::Popup::is_id_open(ui.ctx(), popover) {
        let rows = PopoverRows::of(model);
        // As wide as its rows, within the room left of the chip's right edge,
        // which the popover's right edge stands under.
        let room = rect.right() - ui.ctx().content_rect().left() - SCREEN_MARGIN;
        let width = rows.width(ui).min(room).max(1.0);
        let commands = Commands {
            add: add == CommandAvailability::Available,
            manage: enabled,
            opening: !open,
        };
        egui::Popup::from_response(&response)
            .id(popover)
            .open_memory(None)
            .align(egui::RectAlign::BOTTOM_END)
            .width(width)
            .frame(egui::Frame::popup(ui.style()).inner_margin(0.0))
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.set_width(width);
                request = popover_body(ui, &rows, commands);
                if matches!(
                    request,
                    Some(
                        RunChipRequest::ManagePlans
                            | RunChipRequest::AddAnalysis
                            | RunChipRequest::Open(_)
                    )
                ) {
                    ui.close();
                }
            });
        // Escape hands the keyboard back to the chip it came from.
        if !egui::Popup::is_id_open(ui.ctx(), popover)
            && ui.input(|input| input.key_pressed(egui::Key::Escape))
        {
            response.request_focus();
        }
    }
    if enabled {
        response.on_hover_text(&copy.accessibility);
    } else if let CommandAvailability::Disabled(reason) = availability {
        response.on_hover_text(reason);
    }
    request
}

/// What the popover's foot can do, and whether it opened this frame.
#[derive(Clone, Copy)]
struct Commands {
    add: bool,
    manage: bool,
    /// The popover opened this frame: the keyboard goes into it, and its
    /// tables start from the active plan.
    opening: bool,
}

/// A plan the popover offers.
struct PlanRow {
    id: SimulationPlanId,
    name: String,
    active: bool,
}

/// One analysis of the active plan, in the popover.
struct AnalysisRow {
    id: AnalysisInstanceId,
    name: String,
    detail: Option<String>,
    included: bool,
}

/// What a plain Run runs at, in the popover.
struct ReferenceRow {
    process: ProcessCorner,
    celsius: f64,
    /// The processes the reader can pick among, fast to slow; empty where
    /// no model library defines a corner, and the popover shows no process.
    processes: Vec<ProcessCorner>,
}

/// What the popover lists: the plans Run can run, the active plan's
/// analyses in plan order, a prerequisite among them as the Analyses page
/// lists it, and the reference a plain Run runs at.
struct PopoverRows {
    plans: Vec<PlanRow>,
    analyses: Vec<AnalysisRow>,
    reference: ReferenceRow,
}

impl PopoverRows {
    /// The active plan and every plan the manager would open in its place;
    /// an archived plan is opened from the manager, so it is not offered.
    /// By name, so picking a plan leaves every row where it was.
    fn of(model: &Model) -> Self {
        let setup = &model.sim_setup;
        let plan = setup.stable_analysis_plan().ok();
        let mut plans: Vec<PlanRow> = plan
            .map(|plan| PlanRow {
                id: plan.id(),
                name: setup.active_plan_name().to_string(),
                active: true,
            })
            .into_iter()
            .chain(
                setup
                    .inactive_plans()
                    .iter()
                    .filter(|stored| !stored.archived())
                    .map(|stored| PlanRow {
                        id: stored.id(),
                        name: stored.name().to_string(),
                        active: false,
                    }),
            )
            .collect();
        plans.sort_by_cached_key(|row| row.name.to_lowercase());
        let analyses = plan.map_or_else(Vec::new, |plan| {
            plan.instances()
                .iter()
                .map(|instance| AnalysisRow {
                    id: instance.id(),
                    name: analysis_name(instance.draft()),
                    detail: analysis_detail(instance.draft()),
                    included: instance.enabled(),
                })
                .collect()
        });
        let reference = setup.reference_pvt;
        Self {
            plans,
            analyses,
            reference: ReferenceRow {
                process: reference.process,
                celsius: reference.temperature_celsius,
                processes: library_processes(model, reference.process),
            },
        }
    }

    /// A plan's row: its name, and which plan is active where there is more
    /// than one to tell it from.
    fn plan_cell(&self, row: usize) -> Cell {
        let plan = &self.plans[row];
        let cell = Cell::text(plan.name.as_str());
        if plan.active && self.plans.len() > 1 {
            cell.state("Active", None)
        } else {
            cell
        }
    }

    /// An analysis's row: whether Run includes it, its kind and main numbers.
    fn analysis_cell(&self, row: usize) -> Cell {
        let analysis = &self.analyses[row];
        let cell = Cell::text(analysis.name.as_str()).check(analysis.included);
        match &analysis.detail {
            Some(detail) => cell.suffix(detail.as_str()),
            None => cell,
        }
    }

    /// The width every row, head, line and button shows whole in.
    fn width(&self, ui: &mut Ui) -> f32 {
        let t = Tokens::get(ui.ctx());
        // A table's only column has the table's edge inset on both sides of
        // its words where a column has its pads.
        let edges = 2.0 * (TABLE_EDGE_INSET - CELL_PAD);
        let plans: Vec<Cell> = (0..self.plans.len())
            .map(|row| self.plan_cell(row))
            .collect();
        let analyses: Vec<Cell> = (0..self.analyses.len())
            .map(|row| self.analysis_cell(row))
            .collect();
        let rows =
            fitted_width(ui, PLAN_HEAD, &plans).max(fitted_width(ui, ANALYSIS_HEAD, &analyses))
                + edges;
        let empty = ui
            .painter()
            .layout_no_wrap(
                NO_ANALYSIS.to_owned(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
            + 2.0 * TABLE_EDGE_INSET;
        let button = Button::new(ADD_ANALYSIS)
            .compact()
            .measured_width(ui)
            .max(Button::new(MANAGE_PLANS).compact().measured_width(ui));
        let footer = 2.0 * button + FOOTER_GAP + 2.0 * TABLE_EDGE_INSET;
        rows.max(empty)
            .max(footer)
            .max(self.reference_width(ui))
            .ceil()
    }

    /// The Reference rows' labels, whole.
    fn reference_labels(&self) -> &'static [&'static str] {
        if self.reference.processes.is_empty() {
            &[TEMPERATURE]
        } else {
            &[TEMPERATURE, PROCESS]
        }
    }

    /// The width the Reference rows' labels show whole in.
    fn reference_label_width(&self, ui: &Ui) -> f32 {
        let t = Tokens::get(ui.ctx());
        self.reference_labels()
            .iter()
            .map(|label| {
                ui.painter()
                    .layout_no_wrap(
                        (*label).to_owned(),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_dim,
                    )
                    .size()
                    .x
            })
            .fold(0.0, f32::max)
            .ceil()
    }

    /// The width the Reference section shows whole in: its labels, then the
    /// coldest temperature the reference takes and its unit, or every
    /// process with its padding, whichever is wider.
    fn reference_width(&self, ui: &Ui) -> f32 {
        let t = Tokens::get(ui.ctx());
        let measured = |text: &str, font: egui::FontId| {
            ui.painter()
                .layout_no_wrap(text.to_owned(), font, t.color.text)
                .size()
                .x
        };
        let field = 2.0 * FIELD_PAD
            + measured("\u{2212}273.15", theme::field_mono())
            + UNIT_GAP
            + measured(CELSIUS, theme::field_mono());
        let processes: f32 = self
            .reference
            .processes
            .iter()
            .map(|process| {
                2.0 * FIELD_PAD
                    + measured(
                        process.short_name(),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                    )
            })
            .sum();
        let head = fitted_width(ui, REFERENCE_HEAD, &[]) + 2.0 * (TABLE_EDGE_INSET - CELL_PAD);
        (2.0 * TABLE_EDGE_INSET + self.reference_label_width(ui) + LABEL_GAP + field.max(processes))
            .max(head)
    }
}

/// The popover's tables and foot. Returns what the reader asked of them.
fn popover_body(ui: &mut Ui, rows: &PopoverRows, commands: Commands) -> Option<RunChipRequest> {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    let footer_h = compact_height(&t.metrics) + 2.0 * FOOTER_PAD;
    let reference_h = LineHeights::grid(&t).head
        + t.metrics.row_h * rows.reference_labels().len() as f32;
    // Below the chip the tables have the rest of the screen, less the
    // Reference rows and the foot: the plans up to half of it, the analyses
    // what the plans leave.
    let room = (ui.ctx().content_rect().bottom()
        - ui.cursor().top()
        - reference_h
        - footer_h
        - SCREEN_MARGIN)
        .max(LineHeights::grid(&t).row * 4.0);
    let top = ui.cursor().top();
    let mut request = None;

    let plan_columns = [Column::name(PLAN_HEAD, 0.0).unsortable()];
    let plans = DataTable::new("workbench.run_chip.plans", &plan_columns, rows.plans.len())
        .max_height(room * 0.5);
    if rows.plans.len() > 1 {
        // A plan is picked by a click, or by Enter on the row the arrows reached.
        let keys: Vec<RowKey> = rows
            .plans
            .iter()
            .map(|plan| RowKey::from(format!("{:?}", plan.id)))
            .collect();
        let memory = ui.id().with("workbench.run_chip.plans");
        let active = rows.plans.iter().position(|plan| plan.active);
        let mut state = table_state(ui, memory, commands.opening, active, &keys);
        let response = plans
            .state(&mut state)
            .keys(&keys)
            .focus(commands.opening)
            .show(ui, |row| vec![rows.plan_cell(row)]);
        ui.data_mut(|data| data.insert_temp(memory, state));
        if let Some(plan) = response
            .activate
            .or(response.clicked)
            .and_then(|row| rows.plans.get(row))
            .filter(|plan| !plan.active)
        {
            request = Some(RunChipRequest::ActivatePlan(plan.id));
        }
    } else {
        // One plan: its name, and nothing to pick.
        let _ = plans.show(ui, |row| vec![rows.plan_cell(row)]);
    }

    let analysis_columns = [Column::name(ANALYSIS_HEAD, 0.0).unsortable()];
    let keys: Vec<RowKey> = rows
        .analyses
        .iter()
        .map(|analysis| RowKey::from(analysis.id.to_string()))
        .collect();
    let memory = ui.id().with("workbench.run_chip.analyses");
    let mut state = table_state(ui, memory, commands.opening, None, &keys);
    let response = DataTable::new(
        "workbench.run_chip.analyses",
        &analysis_columns,
        rows.analyses.len(),
    )
    .state(&mut state)
    .keys(&keys)
    .focus(commands.opening && rows.plans.len() <= 1 && !rows.analyses.is_empty())
    .max_height((room - (ui.cursor().top() - top)).max(LineHeights::grid(&t).row * 2.0))
    .show(ui, |row| vec![rows.analysis_cell(row)]);
    ui.data_mut(|data| data.insert_temp(memory, state));
    // A row's check box is whether Run includes it; the rest of the row opens
    // it where it is edited.
    if let Some((row, included)) = response.checked
        && let Some(analysis) = rows.analyses.get(row)
    {
        request = Some(RunChipRequest::Include(analysis.id, included));
    } else if let Some(analysis) = response
        .activate
        .or(response.clicked)
        .and_then(|row| rows.analyses.get(row))
    {
        request = Some(RunChipRequest::Open(analysis.id));
    }
    if rows.analyses.is_empty() {
        empty_line(ui, NO_ANALYSIS);
    }
    if let Some(asked) = reference_section(ui, rows) {
        request = Some(asked);
    }
    let focus_add = commands.opening && rows.plans.len() <= 1 && rows.analyses.is_empty();
    footer(ui, commands, focus_add).or(request)
}

/// The Reference section, what a plain Run runs at: its temperature, and
/// its process where a model library defines corners to pick among. Returns
/// the reference the reader asked for, or why what they typed is not one.
fn reference_section(ui: &mut Ui, rows: &PopoverRows) -> Option<RunChipRequest> {
    let reference = &rows.reference;
    // A table of no rows: the section's head band is the tables' own.
    let columns = [Column::name(REFERENCE_HEAD, 0.0).unsortable()];
    let _ = DataTable::new("workbench.run_chip.reference", &columns, 0).show(ui, |_| Vec::new());
    let label_width = rows.reference_label_width(ui);
    let field = reference_row(ui, TEMPERATURE, label_width);
    let typed = temperature_field(ui, field, reference.celsius);
    let picked = if reference.processes.is_empty() {
        None
    } else {
        let field = reference_row(ui, PROCESS, label_width);
        process_choice(ui, field, reference)
    };
    match (typed, picked) {
        (Some(Err(words)), _) => Some(RunChipRequest::Unreadable(words)),
        (None, None) => None,
        (typed, picked) => Some(RunChipRequest::Reference(
            picked.unwrap_or(reference.process),
            typed.and_then(Result::ok).unwrap_or(reference.celsius),
        )),
    }
}

/// One Reference row, a row tall: `label` in the label column `label_width`
/// wide, ruled at its foot as a table's row is. Returns where its control
/// goes: the rest of the row, at a compact control's height.
fn reference_row(ui: &mut Ui, label: &str, label_width: f32) -> Rect {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), t.metrics.row_h),
        egui::Sense::hover(),
    );
    let height = compact_height(&t.metrics);
    // The row's last point is its rule; the control takes the rest, with
    // equal air over and under it.
    let top = rect.top() + ((rect.height() - 1.0 - height) * 0.5).max(0.0);
    let field = Rect::from_min_max(
        pos2(
            rect.left() + TABLE_EDGE_INSET + label_width + LABEL_GAP,
            top,
        ),
        pos2(rect.right() - TABLE_EDGE_INSET, top + height),
    );
    ui.painter().text(
        pos2(rect.left() + TABLE_EDGE_INSET, field.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    field
}

/// The reference temperature's field, in °C, filling `rect`. It holds what
/// the reader types while they type; Enter, Tab or a click away asks for it,
/// and Escape leaves the reference as it was. Returns the temperature asked
/// for, or why the words are not one.
fn temperature_field(ui: &mut Ui, rect: Rect, celsius: f64) -> Option<Result<f64, String>> {
    let t = Tokens::get(ui.ctx());
    let id = ui.id().with("workbench.run_chip.temperature");
    let draft = id.with("typed");
    let shown = celsius_text(celsius);
    let mut text = if ui.memory(|memory| memory.has_focus(id)) {
        ui.data(|data| data.get_temp::<String>(draft))
            .unwrap_or_else(|| shown.clone())
    } else {
        ui.data_mut(|data| data.remove::<String>(draft));
        shown.clone()
    };
    let font = theme::field_mono();
    let unit = ui
        .painter()
        .layout_no_wrap(CELSIUS.to_owned(), font.clone(), t.color.text_dim);
    let trailing = (FIELD_PAD + unit.size().x + UNIT_GAP)
        .ceil()
        .min(f32::from(i8::MAX)) as i8;
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let response = field.add_sized(
        rect.size(),
        egui::TextEdit::singleline(&mut text)
            .id(id)
            .font(font)
            .text_color(t.color.text)
            .vertical_align(egui::Align::Center)
            .margin(egui::Margin {
                left: FIELD_PAD as i8,
                right: trailing,
                top: 0,
                bottom: 0,
            }),
    );
    let unit_size = unit.size();
    ui.painter().galley(
        pos2(
            response.rect.right() - FIELD_PAD - unit_size.x,
            response.rect.center().y - unit_size.y * 0.5,
        ),
        unit,
        t.color.text_dim,
    );
    name_control(ui, &response, "Reference temperature in \u{00b0}C");
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(draft, text.clone()));
    } else if !response.lost_focus() {
        // Reached by Tab or a click, the field's words are selected, so the
        // first key typed replaces them.
        let mut state = egui::TextEdit::load_state(ui.ctx(), id).unwrap_or_default();
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(text.chars().count()),
            )));
        egui::TextEdit::store_state(ui.ctx(), id, state);
    }
    if !response.lost_focus() {
        return None;
    }
    ui.data_mut(|data| data.remove::<String>(draft));
    if ui.input(|input| input.key_pressed(egui::Key::Escape)) || text.trim() == shown {
        return None;
    }
    Some(read_celsius(&text))
}

/// A reference temperature as the field shows it: plain digits, `27`,
/// `-40`, `27.5`.
fn celsius_text(celsius: f64) -> String {
    format!("{celsius}")
}

/// The temperature in °C that `text` states, a number with or without the
/// unit after it, or why it states none.
fn read_celsius(text: &str) -> Result<f64, String> {
    let trimmed = text.trim();
    let number = ["\u{00b0}C", "\u{2103}", "C", "c", "\u{00b0}"]
        .iter()
        .find_map(|unit| trimmed.strip_suffix(unit))
        .unwrap_or(trimmed)
        .trim()
        .replace('\u{2212}', "-");
    rspice_app_state::quantity::parse_engineering_value(&number)
        .map_err(|_| format!("\u{201c}{trimmed}\u{201d} is not a temperature in \u{00b0}C"))
}

/// The reference process as one segmented group filling `rect`. Returns the
/// process the reader picked, when it is another.
fn process_choice(ui: &mut Ui, rect: Rect, reference: &ReferenceRow) -> Option<ProcessCorner> {
    let labels: Vec<&str> = reference
        .processes
        .iter()
        .map(|process| process.short_name())
        .collect();
    let mut chosen = reference
        .processes
        .iter()
        .position(|process| *process == reference.process)?;
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    // The group stands at the row's control height, as the field above it.
    field.spacing_mut().interact_size.y = rect.height();
    segmented(
        &mut field,
        "workbench.run_chip.process",
        &labels,
        &mut chosen,
        SegmentedWidth::Equal,
    )
    .then(|| reference.processes[chosen])
    .filter(|process| *process != reference.process)
}

/// A popover table's memory while the popover stays open. Each opening
/// starts afresh, its selection on row `selected`.
fn table_state(
    ui: &Ui,
    memory: egui::Id,
    opening: bool,
    selected: Option<usize>,
    keys: &[RowKey],
) -> TableState {
    if opening {
        let mut state = TableState::default();
        if let Some(row) = selected {
            state.select(row, keys.get(row).cloned());
        }
        state
    } else {
        ui.data(|data| data.get_temp::<TableState>(memory))
            .unwrap_or_default()
    }
}

/// One faint line where a table has no rows, a row tall and ruled as a row.
fn empty_line(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), LineHeights::grid(&t).row),
        egui::Sense::hover(),
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    ui.painter().text(
        pos2(rect.left() + TABLE_EDGE_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// The foot under the tables' last rule: Add analysis… and Manage plans…,
/// equal halves of the popover's width. Returns which was pressed.
fn footer(ui: &mut Ui, commands: Commands, focus_add: bool) -> Option<RunChipRequest> {
    let t = Tokens::get(ui.ctx());
    let height = compact_height(&t.metrics);
    let (band, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), height + 2.0 * FOOTER_PAD),
        egui::Sense::hover(),
    );
    let width = ((band.width() - 2.0 * TABLE_EDGE_INSET - FOOTER_GAP) * 0.5).max(1.0);
    let first = Rect::from_min_size(
        pos2(band.left() + TABLE_EDGE_INSET, band.top() + FOOTER_PAD),
        vec2(width, height),
    );
    let second = first.translate(vec2(width + FOOTER_GAP, 0.0));
    let button = |ui: &mut Ui, rect: Rect, label: &str, enabled: bool| {
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            Button::new(label)
                .compact()
                .min_width(rect.width())
                .enabled(enabled)
                .show(ui)
        })
        .inner
    };
    let add = button(ui, first, ADD_ANALYSIS, commands.add);
    let manage = button(ui, second, MANAGE_PLANS, commands.manage);
    if focus_add {
        add.request_focus();
    }
    if add.clicked() {
        Some(RunChipRequest::AddAnalysis)
    } else if manage.clicked() {
        Some(RunChipRequest::ManagePlans)
    } else {
        None
    }
}

/// Draw the chip that stands in for the plan chip where Run runs a deck:
/// the deck's file name over "Deck", enabled as `show`, the dispatcher's
/// answer for Show deck, says. Returns whether the reader pressed it.
pub(in crate::workbench) fn deck_chip(
    ui: &mut egui::Ui,
    deck: &str,
    show: CommandAvailability,
    height: f32,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let enabled = show == CommandAvailability::Available && ui.is_enabled();
    let accessibility = format!("Deck {deck}; show the deck Run runs");
    let summary = "Deck";
    let measure = |text: &str, font: egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font, t.color.text)
            .size()
            .x
    };
    let width = run_config_chip_width(
        measure(deck, theme::sans(tokens::FS_0, FontWeight::Medium)),
        measure(summary, theme::mono(tokens::FS_0, FontWeight::Regular)),
    );
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, &accessibility)
    });
    let hovered = enabled && response.hovered();
    ui.painter().rect(
        rect,
        t.radius,
        if hovered {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
        egui::Stroke::new(
            1.0,
            if hovered {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Netlist.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        t.color.text_dim,
    );
    // A long file name keeps its start and its extension, shortened in the
    // middle, and leaves the right inset the chip's width allowed for; hover
    // names it whole.
    let name_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y - 6.5),
        egui::Align2::LEFT_CENTER,
        elide_middle(ui, deck, &name_font, rect.width() - 31.0 - 22.0),
        name_font,
        t.color.text,
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y + 7.0),
        egui::Align2::LEFT_CENTER,
        summary,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    let pressed = response.clicked() && enabled;
    response.on_hover_text(&accessibility);
    pressed
}

fn configured_pvt_count(model: &Model) -> usize {
    // The plan-wide Run Set expands every enabled analysis. Legacy Corner and
    // Temperature analysis drafts no longer own the workspace's PVT count and
    // must not be added a second time in the chrome summary.
    model.sim_setup.run_set.point_count().max(1)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::state::pdk_config::PdkConfig;

    use rspice_app_state::workbench::workspace::restore_model_library_manager;

    /// The chip is the product's global entry to the plan manager, so the plan
    /// it names has to be the plan that is active — not the name the fixture
    /// project happened to ship with.
    #[test]
    fn plan_chip_copy_follows_a_rename_of_the_active_plan() {
        let mut model =
            Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
        let plan_id = model
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .id();

        model
            .sim_setup
            .rename_plan(plan_id, "Automotive temperature sweep")
            .expect("rename the active plan");
        let copy = run_config_chip_copy(&model);

        assert_eq!(
            copy.summary,
            "Automotive temperature sweep \u{00b7} 27 \u{00b0}C"
        );
        assert_eq!(
            copy.accessibility,
            "Analysis Transient \u{00b7} 1 ms, plan Automotive temperature sweep, \
             27 \u{00b0}C; choose the plan, the analyses and the reference PVT Run runs"
        );
        assert!(!copy.summary.contains("Default plan"));
        assert!(!copy.accessibility.contains("Default plan"));
    }

    /// The chip names what Run runs: the first enabled analysis by its kind
    /// and main numbers, and how many more run after it.
    #[test]
    fn the_chip_names_the_analysis_run_runs() {
        use rspice_simulation_contract::analysis_kind::AnalysisKind;
        let mut model =
            Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
        assert_eq!(
            run_config_chip_copy(&model).title,
            "Transient \u{00b7} 1 ms"
        );

        let plan = model.sim_setup.stable_analysis_plan_mut().unwrap();
        let transient = plan.instances()[0].id();
        plan.edit(transient, |draft| {
            if let AnalysisDraft::Transient(setup) = draft {
                "20u".clone_into(&mut setup.stop);
            }
        })
        .unwrap();
        assert_eq!(
            run_config_chip_copy(&model).title,
            "Transient \u{00b7} 20 \u{b5}s"
        );

        // The sweep's own operating point is part of the sweep; another
        // analysis beside it is counted.
        let plan = model.sim_setup.stable_analysis_plan_mut().unwrap();
        let (ac, _) = plan.insert_at(AnalysisKind::Ac, 0).unwrap();
        plan.repair_dependencies(ac).unwrap();
        assert_eq!(
            run_config_chip_copy(&model).title,
            "AC \u{00b7} 1 Hz to 1 GHz +1"
        );
    }

    #[test]
    fn toolbar_pvt_summary_uses_the_global_run_set_exactly_once() {
        use crate::product::ProcessCorner;
        use rspice_simulation_contract::corner_config::{CornerBaseAnalysis, CornerConfig};

        let mut model =
            Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
        model.sim_setup.run_set =
            crate::simulation::controller::test_execution::pvt::from_corner_config(&CornerConfig {
                process_corners: vec![ProcessCorner::TT, ProcessCorner::SS],
                voltages: vec![1.0],
                supply_source_names: Vec::new(),
                temperatures: vec![-40.0, 27.0, 125.0],
                full_matrix: true,
                points: Vec::new(),
                base_analysis: CornerBaseAnalysis::Op,
            });

        assert_eq!(configured_pvt_count(&model), 6);
    }
}

//! Specifications: every specification of the plan, judged against the run.
//!
//! One row per specification, in plan order — under a head naming the
//! analysis that produces it when the plan names more than one — with its
//! limit, the value at the nominal point, the worst value and where it was,
//! the margin the worst value leaves, its yield when trials answered it, and
//! its status. Every value is the verification sheet's; this view only lays it
//! out. A run that cannot answer the plan's limits keeps the same rows, with
//! dashes where a verdict would be, and the summary says why.
//!
//! The name and the limit are typed into their cells; everything else about a
//! specification is the inspector's. Every edit is one plan change through
//! [`super::specification_edit`], and the run is judged again the frame it
//! lands.

use egui::{Key, Modifiers, Ui};
use rspice_results::family_measurements::FamilyMemberId;
use rspice_results::number_format::fmt_si_bound;
use rspice_results::specification::SpecificationComparison;
use rspice_results::verification::{LimitsSinceRun, PointSource, Row, RowStatus, Sheet, Worst};

use crate::io::file_exchange::{self, FileKind};
use crate::state::{SpecEntry, SpecPointScope, SpecificationDefinition, VerificationPage};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Problem;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::elide_text;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::state::{
    SpecificationCell, SpecificationCellDraft, SpecificationsMenu, SpecificationsMenuKind,
};

use super::specification_edit::{
    definition_index, edit_specifications, limit_spelling, limit_text, parse_limit, set_limit,
};
use super::table::{
    Cell, CellEditor, Column, DataTable, EditOutcome, Group, RowKey, SortKey, TABLE_EDGE_INSET,
    Verdict,
};
use super::toolbar::{PageToolbar, RunChoice, Summary, ToolbarAction, empty_line};
use super::{format_value, record_action, verification_plan_id};

const TABLE_ID: &str = "verify.specifications";
const MENU_W: f32 = 240.0;
const MENU_ROW_H: f32 = 24.0;

const CSV: FileKind = FileKind {
    label: "Comma-separated values",
    extensions: &["csv"],
    subject: "The specification sheet",
    fallback_name: "specifications.csv",
};

fn export_id() -> egui::Id {
    egui::Id::new("workbench.verify.specifications.export")
}

fn menu_id() -> egui::Id {
    egui::Id::new("workbench.verify.specifications.menu")
}

/// Whether the table held the keyboard when the last frame ended. Escape
/// takes the keyboard from a widget before the frame is drawn, so the table
/// asks what it held then.
fn focus_mark() -> egui::Id {
    egui::Id::new("workbench.verify.specifications.focused")
}

/// A column the view can show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Col {
    Specification,
    Limit,
    Nominal,
    Worst,
    At,
    Margin,
    Yield,
    Status,
}

impl Col {
    fn column(self) -> Column {
        match self {
            Self::Specification => Column::name("Specification", 150.0),
            Self::Limit => Column::number("Limit", 120.0).tooltip(
                "Select a row, then click its limit or press F2: \u{2265} 40 dB, \u{2264} 1.2 mA, \
                 10M..20M, 50 \u{b1} 1.",
            ),
            Self::Nominal => Column::number("Nominal", 96.0),
            Self::Worst => Column::number("Worst", 96.0),
            Self::At => Column::words("At", 110.0),
            Self::Margin => Column::number("Margin", 96.0).tooltip(
                "Room the worst value leaves to the limit, in its unit; negative once broken.",
            ),
            Self::Yield => Column::number("Yield", 76.0),
            Self::Status => Column::words("Status", 96.0),
        }
    }
}

/// Which value of a row the reader asked to see in Results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Nominal,
    Worst,
}

/// One row as the table shows it.
struct RowView {
    measurement: String,
    name: String,
    unit: String,
    comparison: SpecificationComparison,
    cells: Vec<Cell>,
}

/// The sheet laid out for one frame: rows in the order they are shown, with
/// their keys and group heads, and the columns the run calls for.
struct SheetView {
    /// The plan can take an edit.
    editable: bool,
    columns: Vec<Col>,
    rows: Vec<RowView>,
    keys: Vec<RowKey>,
    groups: Vec<Group>,
    summary: Option<Summary>,
}

impl SheetView {
    fn column(&self, col: Col) -> Option<usize> {
        self.columns.iter().position(|shown| *shown == col)
    }

    fn row(&self, measurement: &str) -> Option<usize> {
        self.rows
            .iter()
            .position(|row| row.measurement.eq_ignore_ascii_case(measurement))
    }
}

pub(super) fn specifications(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    {
        let model = &mut *cx.model;
        model
            .simulation
            .refresh_verification(&model.sim_setup, &model.workspace.content);
    }
    poll_export(ui.ctx(), cx);
    let selected = cx.state.selected_specification.clone();
    let view = sheet_view(cx.model, selected.as_deref());
    // Escape closes an open menu, and only that.
    let menu_open = cx.state.verification.specifications_menu.is_some();
    menus(ui, cx, &view);

    let toolbar_top = ui.cursor().top();
    let actions = [
        ToolbarAction::new("Add specification")
            .enabled(view.editable)
            .disabled_reason("The simulation plan is not ready."),
        ToolbarAction::new("Export CSV")
            .enabled(!view.rows.is_empty())
            .disabled_reason("There are no specifications to export."),
    ];
    let output = PageToolbar {
        title: VerificationPage::Specifications.label(),
        runs: &RunChoice::default(),
        summary: view.summary.clone(),
        actions: &actions,
    }
    .show(ui);
    let toolbar_bottom = ui.cursor().top().max(toolbar_top);
    match output.action {
        Some(0) => {
            let x = ui
                .ctx()
                .input(|input| input.pointer.interact_pos())
                .map_or(ui.max_rect().right() - MENU_W, |pointer| pointer.x);
            cx.state.verification.specifications_menu = Some(SpecificationsMenu {
                kind: SpecificationsMenuKind::Add,
                position: egui::pos2(x, toolbar_bottom),
            });
            ui.ctx().request_repaint();
        }
        Some(1) => export_csv(ui.ctx(), cx, &view),
        _ => {}
    }
    if let Some(problem) = &cx.state.verification.action_problem {
        let _ = crate::ui::widgets::problem_line(ui, Problem::Failure, problem);
    }

    if view.rows.is_empty() {
        cx.state.verification.specification_cell_draft = None;
        let text = if view.editable {
            "No specifications in this plan. Add one from the run's measurements."
        } else {
            "The simulation plan is not ready, so it holds no specifications."
        };
        let _ = empty_line(ui, text, None);
        return;
    }

    table(ui, cx, &view, menu_open);
}

/// The table, and what the reader did in it.
fn table(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>, view: &SheetView, menu_open: bool) {
    let table_id = ui.id().with((TABLE_ID, "table"));
    let focused = ui.memory(|memory| memory.has_focus(table_id));
    let had_focus = ui
        .ctx()
        .data(|data| data.get_temp::<bool>(focus_mark()))
        .unwrap_or(false);

    // The table selects what every surface has selected.
    let session = &mut cx.state.verification;
    let selected_row = cx
        .state
        .selected_specification
        .as_deref()
        .and_then(|measurement| view.row(measurement));
    match selected_row {
        Some(row) if session.specifications_table.selected_row() != Some(row) => {
            session
                .specifications_table
                .select(row, Some(view.keys[row].clone()));
        }
        None => session.specifications_table.selection = None,
        Some(_) => {}
    }
    session.specifications_table.filter = cx.wb.navigator_filter().to_owned();

    let mut draft = session
        .specification_cell_draft
        .take()
        .filter(|draft| view.editable && view.row(&draft.measurement).is_some());
    let delete = draft.is_none()
        && focused
        && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Delete));
    let deselect = draft.is_none()
        && had_focus
        && !focused
        && !menu_open
        && ui.input(|input| input.key_pressed(Key::Escape));

    let columns = view
        .columns
        .iter()
        .map(|col| col.column())
        .collect::<Vec<_>>();
    let editor = draft.as_mut().and_then(|draft| {
        Some(CellEditor {
            row: view.row(&draft.measurement)?,
            column: view.column(match draft.cell {
                SpecificationCell::Name => Col::Specification,
                SpecificationCell::Limit => Col::Limit,
            })?,
            text: &mut draft.text,
            focus: std::mem::take(&mut draft.focus),
        })
    });
    let response = DataTable::new(TABLE_ID, &columns, view.rows.len())
        .state(&mut cx.state.verification.specifications_table)
        .keys(&view.keys)
        .groups(&view.groups)
        .freeze(1)
        .cell_selection()
        .max_height(ui.available_height())
        .editing(editor)
        .show(ui, |index| view.rows[index].cells.clone());
    let holds_focus = ui.memory(|memory| memory.has_focus(table_id));
    ui.ctx()
        .data_mut(|data| data.insert_temp(focus_mark(), holds_focus));

    if response.selection_changed {
        cx.state.selected_specification = cx
            .state
            .verification
            .specifications_table
            .selected_row()
            .map(|row| view.rows[row].measurement.clone());
    }
    if deselect {
        cx.state.verification.specifications_table.selection = None;
        cx.state.selected_specification = None;
    }

    match response.edited {
        Some(EditOutcome::Cancel) => draft = None,
        Some(EditOutcome::Commit) => {
            if let Some(open) = draft.take()
                && open.text.trim() != open.seed.trim()
                && let Some(index) = view.row(&open.measurement)
            {
                if let Err(problem) = apply_cell(cx, &view.rows[index], &open) {
                    record_action(&mut cx.state.verification, Err(problem));
                    draft = Some(SpecificationCellDraft {
                        focus: true,
                        ..open
                    });
                } else {
                    record_action(&mut cx.state.verification, Ok(()));
                    ui.ctx().request_repaint();
                }
            }
        }
        None => {}
    }
    if let Some((index, column)) = response.edit {
        let row = &view.rows[index];
        let cell = if Some(column) == view.column(Col::Limit) {
            SpecificationCell::Limit
        } else {
            SpecificationCell::Name
        };
        let seed = match cell {
            SpecificationCell::Name => row.name.clone(),
            SpecificationCell::Limit => limit_spelling(
                &row.comparison,
                &row.unit,
                cx.model.preferences.quantity_presentation_policy(),
                cx.wb.number_locale,
            ),
        };
        draft = Some(SpecificationCellDraft {
            measurement: row.measurement.clone(),
            cell,
            text: seed.clone(),
            seed,
            focus: true,
        });
    }
    cx.state.verification.specification_cell_draft = draft;

    if let Some(cell) = response.double_clicked {
        let target = match cell.column.and_then(|column| view.columns.get(column)) {
            Some(Col::Nominal) => Some(Target::Nominal),
            Some(Col::Worst | Col::At | Col::Margin | Col::Yield) => Some(Target::Worst),
            _ => None,
        };
        if let Some(target) = target {
            let measurement = view.rows[cell.row].measurement.clone();
            let opened = open_in_results(cx, &measurement, target);
            record_action(&mut cx.state.verification, opened);
        }
    }
    if let Some(request) = response.context_menu {
        cx.state.verification.specifications_menu = Some(SpecificationsMenu {
            kind: SpecificationsMenuKind::Row(view.rows[request.cell.row].measurement.clone()),
            position: request.position,
        });
        ui.ctx().request_repaint();
    }
    if delete && let Some(row) = cx.state.verification.specifications_table.selected_row() {
        let measurement = view.rows[row].measurement.clone();
        delete_specification(cx, &measurement);
    }
}

/// Apply what the reader typed into one cell: a new name, or a new limit.
fn apply_cell(
    cx: &mut Cx<'_, SimulateFeature>,
    row: &RowView,
    open: &SpecificationCellDraft,
) -> Result<(), String> {
    let measurement = open.measurement.clone();
    match open.cell {
        SpecificationCell::Name => {
            let name = open.text.trim().to_owned();
            if name.is_empty() {
                return Err(format!(
                    "{} keeps its name: a name cannot be empty.",
                    row.name
                ));
            }
            edit_specifications(cx, |definitions| {
                let index = definition_index(definitions, None, &measurement)
                    .ok_or_else(|| format!("{measurement} is no longer in the plan"))?;
                let definition = &mut definitions[index];
                let detail = format!(
                    "Renamed specification {} to {name}.",
                    definition.requirement_name
                );
                definition.requirement_name = name;
                Ok(detail)
            })
            .map(|_| ())
            .map_err(|problem| format!("{} was not renamed: {problem}.", row.name))
        }
        SpecificationCell::Limit => {
            let comparison = parse_limit(
                &open.text,
                &row.unit,
                &row.comparison,
                cx.model.preferences.quantity_presentation_policy(),
                cx.wb.number_locale,
            )
            .map_err(|problem| format!("The limit of {} was not set: {problem}.", row.name))?;
            edit_specifications(cx, |definitions| {
                let index = definition_index(definitions, None, &measurement)
                    .ok_or_else(|| format!("{measurement} is no longer in the plan"))?;
                Ok(set_limit(&mut definitions[index], comparison))
            })
            .map(|_| ())
            .map_err(|problem| format!("The limit of {} was not set: {problem}.", row.name))
        }
    }
}

/// Take `measurement`'s specification out of the plan, as one plan change,
/// and say on the status line what went.
fn delete_specification(cx: &mut Cx<'_, SimulateFeature>, measurement: &str) {
    let deleted = edit_specifications(cx, |definitions| {
        let index = definition_index(definitions, None, measurement)
            .ok_or_else(|| format!("{measurement} is no longer in the plan"))?;
        let definition = definitions.remove(index);
        Ok(format!(
            "Deleted specification {}.",
            definition.requirement_name
        ))
    });
    match deleted {
        Ok(line) => {
            cx.state.verification.specification_cell_draft = None;
            cx.state.verification.specification_draft = None;
            cx.state.selected_specification = None;
            record_action(&mut cx.state.verification, Ok(()));
            cx.wb.prompt(line);
        }
        Err(problem) => record_action(
            &mut cx.state.verification,
            Err(format!("{measurement} was not deleted: {problem}.")),
        ),
    }
}

/// Add a specification of `measurement`, tracked until a limit is set, and
/// select it so its limit and the inspector are the next thing to edit.
fn add_specification(cx: &mut Cx<'_, SimulateFeature>, measurement: &str, unit: &str) {
    let definition = new_definition(measurement, unit);
    let added = edit_specifications(cx, |definitions| {
        if definition_index(definitions, None, &definition.measurement).is_some() {
            return Err(format!("the plan already names {}", definition.measurement));
        }
        let detail = format!("Added specification {}.", definition.requirement_name);
        definitions.push(definition);
        Ok(detail)
    });
    match added {
        Ok(_) => {
            cx.state.selected_specification = Some(measurement.to_owned());
            cx.state.verification.specification_draft = None;
            record_action(&mut cx.state.verification, Ok(()));
        }
        Err(problem) => record_action(
            &mut cx.state.verification,
            Err(format!("{measurement} was not added: {problem}.")),
        ),
    }
}

fn new_definition(measurement: &str, unit: &str) -> SpecificationDefinition {
    SpecificationDefinition::new_from_projection(&SpecEntry {
        measurement: measurement.to_owned(),
        expression: String::new(),
        min: None,
        max: None,
        unit: unit.to_owned(),
        scope: SpecPointScope::AllPoints,
    })
}

/// The measurements the run holds that no specification names, with the unit
/// the run measured each in, in name order. Only names a specification can
/// take are offered.
fn unnamed_measurements(model: &Model) -> Vec<(String, String)> {
    let Some(run) = model.simulation.active_run() else {
        return Vec::new();
    };
    let named = verification_plan_id(model).map_or_else(
        || model.workspace.content.specs.clone(),
        |plan_id| model.workspace.content.active_specs(plan_id).to_vec(),
    );
    let mut found: Vec<(String, String)> = Vec::new();
    let mut offer = |name: &str, unit: Option<&str>| {
        if named
            .iter()
            .any(|spec| spec.measurement.eq_ignore_ascii_case(name))
            || found
                .iter()
                .any(|(known, _)| known.eq_ignore_ascii_case(name))
        {
            return;
        }
        let unit = unit.unwrap_or_default().to_owned();
        if new_definition(name, &unit).validate().is_ok() {
            found.push((name.to_owned(), unit));
        }
    };
    for analysis in run.analyses.iter().filter(|analysis| analysis.success) {
        for name in analysis.scalar_evidence_names() {
            let unit = analysis
                .scalar_evidence(&name)
                .into_iter()
                .find_map(|candidate| {
                    candidate
                        .unit
                        .and_then(|unit| unit.symbol().map(str::to_owned))
                });
            offer(&name, unit.as_deref());
        }
        if let Some(family) = &analysis.family_metadata {
            for member in family.member_measurements() {
                for measurement in &member.measurements {
                    let unit = measurement.unit.as_ref().and_then(|unit| unit.symbol());
                    offer(&measurement.name, unit);
                }
            }
        }
    }
    found.sort_by(|left, right| left.0.to_lowercase().cmp(&right.0.to_lowercase()));
    found
}

/// The first `measN` no specification of the plan names.
fn new_measurement_name(model: &Model) -> String {
    let named = verification_plan_id(model).map_or_else(
        || model.workspace.content.specs.clone(),
        |plan_id| model.workspace.content.active_specs(plan_id).to_vec(),
    );
    (1..)
        .map(|index| format!("meas{index}"))
        .find(|name| {
            !named
                .iter()
                .any(|spec| spec.measurement.eq_ignore_ascii_case(name))
        })
        .unwrap_or_default()
}

/// What a menu row does.
#[derive(Debug, Clone, PartialEq)]
enum MenuChoice {
    Add { measurement: String, unit: String },
    NewMeasurement,
    OpenInResults(String),
    Delete(String),
}

/// The menu the view has open: the measurements to add, or one row's actions.
fn menus(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>, view: &SheetView) {
    let Some(menu) = cx.state.verification.specifications_menu.clone() else {
        return;
    };
    let mut items: Vec<(String, Option<MenuChoice>)> = Vec::new();
    match &menu.kind {
        SpecificationsMenuKind::Add => {
            let unnamed = unnamed_measurements(cx.model);
            if unnamed.is_empty() {
                let note = if cx.model.simulation.active_run().is_some() {
                    "Every measurement of this run is named"
                } else {
                    "No run to read measurements from"
                };
                items.push((note.to_owned(), None));
            }
            items.extend(unnamed.into_iter().map(|(measurement, unit)| {
                (
                    measurement.clone(),
                    Some(MenuChoice::Add { measurement, unit }),
                )
            }));
            items.push((
                "New measurement\u{2026}".to_owned(),
                Some(MenuChoice::NewMeasurement),
            ));
        }
        SpecificationsMenuKind::Row(measurement) => {
            if view.row(measurement).is_none() {
                cx.state.verification.specifications_menu = None;
                return;
            }
            items.push((
                "Open in Results".to_owned(),
                Some(MenuChoice::OpenInResults(measurement.clone())),
            ));
            items.push((
                "Delete".to_owned(),
                view.editable
                    .then(|| MenuChoice::Delete(measurement.clone())),
            ));
        }
    }
    let mut open = true;
    let mut chosen = None;
    let id = menu_id();
    let shown = egui::Popup::new(
        id,
        ui.ctx().clone(),
        egui::PopupAnchor::Position(menu.position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    .open_bool(&mut open)
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .align(egui::RectAlign::BOTTOM_START)
    .align_alternatives(&[
        egui::RectAlign::TOP_START,
        egui::RectAlign::BOTTOM_END,
        egui::RectAlign::TOP_END,
    ])
    .width(MENU_W)
    .show(|ui| {
        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
        for (label, choice) in &items {
            if menu_row(ui, label, choice.is_some()).clicked() {
                chosen.clone_from(choice);
                ui.close();
            }
        }
    });
    #[cfg(any(test, feature = "test-support"))]
    if let Some(shown) = &shown {
        super::raster::probe(ui, "menu", shown.response.rect);
    }
    let _ = shown;
    if !open || chosen.is_some() {
        cx.state.verification.specifications_menu = None;
    }
    match chosen {
        Some(MenuChoice::Add { measurement, unit }) => add_specification(cx, &measurement, &unit),
        Some(MenuChoice::NewMeasurement) => {
            let name = new_measurement_name(cx.model);
            add_specification(cx, &name, "");
        }
        Some(MenuChoice::OpenInResults(measurement)) => {
            let opened = open_in_results(cx, &measurement, Target::Worst);
            record_action(&mut cx.state.verification, opened);
        }
        Some(MenuChoice::Delete(measurement)) => delete_specification(cx, &measurement),
        None => {}
    }
}

/// One menu row: its label, faint when it does nothing.
fn menu_row(ui: &mut Ui, label: &str, enabled: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let height = if t.metrics.is_touch() {
        t.metrics.touch_target()
    } else {
        MENU_ROW_H
    };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    if enabled && (response.hovered() || response.is_pointer_button_down_on()) {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let words = elide_text(
        ui,
        label,
        &font,
        (rect.width() - 2.0 * TABLE_EDGE_INSET).max(0.0),
    );
    ui.painter().text(
        egui::pos2(rect.left() + TABLE_EDGE_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        words,
        font,
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

/// Write the sheet as the table shows it: its columns, and the rows the
/// filter keeps in the order they are shown.
fn export_csv(ctx: &egui::Context, cx: &mut Cx<'_, SimulateFeature>, view: &SheetView) {
    let text = sheet_csv(view, &cx.state.verification.specifications_table.filter);
    let saved = file_exchange::save_file(
        ctx,
        export_id(),
        CSV,
        CSV.fallback_name.to_owned(),
        text.into_bytes(),
    );
    record_action(&mut cx.state.verification, saved);
}

fn sheet_csv(view: &SheetView, filter: &str) -> String {
    let needle = filter.trim().to_lowercase();
    let field = |text: &str| {
        let text = if text == "\u{2014}" { "" } else { text };
        if text.contains([',', '"', '\n', '\r']) {
            format!("\"{}\"", text.replace('"', "\"\""))
        } else {
            text.to_owned()
        }
    };
    let mut lines = vec![
        view.columns
            .iter()
            .map(|col| field(&col.column().head))
            .collect::<Vec<_>>()
            .join(","),
    ];
    for row in &view.rows {
        if !needle.is_empty()
            && !row
                .cells
                .iter()
                .any(|cell| cell.text.to_lowercase().contains(&needle))
        {
            continue;
        }
        lines.push(
            row.cells
                .iter()
                .map(|cell| field(&cell.text))
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    let mut text = lines.join("\r\n");
    text.push_str("\r\n");
    text
}

/// Say how the last export went, once it has.
fn poll_export(ctx: &egui::Context, cx: &mut Cx<'_, SimulateFeature>) {
    match file_exchange::take_saved(ctx, export_id()) {
        Some(Ok(Some(file))) => cx.wb.prompt(format!("Saved {}.", file.name)),
        Some(Err(problem)) => record_action(&mut cx.state.verification, Err(problem)),
        Some(Ok(None)) | None => {}
    }
}

/// Open the result behind one of `measurement`'s values in Results: the
/// analysis that produced it, narrowed to the family member it came from when
/// it came from one.
fn open_in_results(
    cx: &mut Cx<'_, SimulateFeature>,
    measurement: &str,
    target: Target,
) -> Result<(), String> {
    let (analysis, query) = {
        let sheet = cx
            .model
            .simulation
            .verification_sheet()
            .ok_or_else(|| "nothing has been judged yet".to_owned())?;
        let row = sheet
            .rows
            .iter()
            .find(|row| row.measurement.eq_ignore_ascii_case(measurement))
            .ok_or_else(|| format!("{measurement} is no longer in the plan"))?;
        let run = cx
            .model
            .simulation
            .active_run()
            .ok_or_else(|| "no run is selected".to_owned())?;
        let (point, member) = match target {
            Target::Nominal => (sheet.points.iter().position(|point| point.nominal), None),
            Target::Worst => row
                .worst
                .as_ref()
                .map_or((None, None), |worst| (worst.point, worst.member.clone())),
        };
        let point = point.and_then(|point| sheet.points.get(point));
        let member = member.or_else(|| match point.map(|point| &point.source) {
            Some(PointSource::Member(member)) => Some(member.clone()),
            _ => None,
        });
        result_location(run, row, point.map(|point| &point.source), member.as_ref())
            .ok_or_else(|| format!("this run holds no result of {} to open", row.name))?
    };
    if !cx.model.simulation.select_analysis(analysis) {
        return Err("the analysis that holds it could not be selected".to_owned());
    }
    match query {
        Some(query) => {
            cx.wb.activate(crate::workbench::state::Workspace::Results);
            cx.wb
                .outbox
                .push(crate::workbench::framework::requests::SetFamilyFilter(
                    query,
                ));
            cx.execute(ids::FAMILY_SLICING);
        }
        None => cx.execute(ids::RESULTS_WORKSPACE),
    }
    Ok(())
}

/// The analysis of `run` that produced `row`'s value at `point` — the member
/// `member` of a family when the value came from one, with the family filter
/// that narrows Results to it.
fn result_location(
    run: &crate::state::SimulationRun,
    row: &Row,
    point: Option<&PointSource>,
    member: Option<&FamilyMemberId>,
) -> Option<(usize, Option<String>)> {
    let produced = |analysis: &crate::state::AnalysisResult| {
        analysis.success
            && analysis.provenance.as_ref().is_some_and(|provenance| {
                row.producing_analysis
                    .is_none_or(|id| provenance.authored_source_instance_id() == id)
            })
    };
    if let Some(member) = member {
        for (index, analysis) in run.analyses.iter().enumerate() {
            if !produced(analysis) {
                continue;
            }
            let Some(family) = &analysis.family_metadata else {
                continue;
            };
            let Some(position) = family.member_measurements().iter().position(|evidence| {
                &evidence.member == member
                    && evidence
                        .measurements
                        .iter()
                        .any(|measured| measured.name.eq_ignore_ascii_case(&row.measurement))
            }) else {
                continue;
            };
            let query = format!("sample = {}", position + 1);
            let filters = rspice_results::family_projection::FamilyManifest::from_metadata(
                analysis.analysis_type,
                Some(family),
            )
            .ok()
            .flatten()
            .is_some_and(|manifest| {
                manifest
                    .matching_source_indices(&query)
                    .is_ok_and(|indices| !indices.is_empty())
            });
            return Some((index, filters.then_some(query)));
        }
    }
    let attributed = match point {
        Some(PointSource::Attributed(record)) => Some(record),
        _ => None,
    };
    run.analyses
        .iter()
        .position(|analysis| {
            produced(analysis)
                && analysis
                    .scalar_evidence_names()
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&row.measurement))
                && analysis
                    .provenance
                    .as_ref()
                    .and_then(|provenance| provenance.pvt_point())
                    == attributed
        })
        .map(|index| (index, None))
}

/// The sheet's roll-up in words: how many specifications, and how many of
/// them fail, err or went unmeasured. The navigator, the toolbar and the table
/// all count from the same sheet, so they cannot disagree.
pub fn counts_line(sheet: &Sheet) -> String {
    let total = sheet.rows.len();
    if total == 0 {
        return "no specifications".to_owned();
    }
    let mut parts = vec![format!(
        "{total} specification{}",
        if total == 1 { "" } else { "s" }
    )];
    if sheet.is_judged() {
        let counts = sheet.summary.counts;
        if counts.fail > 0 {
            parts.push(format!(
                "{} {}",
                counts.fail,
                if counts.fail == 1 { "fails" } else { "fail" }
            ));
        }
        if counts.error > 0 {
            parts.push(format!(
                "{} error{}",
                counts.error,
                if counts.error == 1 { "" } else { "s" }
            ));
        }
        if counts.not_measured > 0 {
            parts.push(format!("{} not measured", counts.not_measured));
        }
    }
    parts.join(" \u{b7} ")
}

/// The problem the sheet's roll-up leads with, if it has one.
pub fn counts_problem(sheet: &Sheet) -> Option<Problem> {
    let counts = sheet.summary.counts;
    if !sheet.is_judged() {
        None
    } else if counts.fail + counts.error > 0 {
        Some(Problem::Failure)
    } else if counts.not_measured > 0 {
        Some(Problem::Warning)
    } else {
        None
    }
}

/// The toolbar's summary: the roll-up, whether acceptance is blocked, and
/// whether the limits changed since the run, or why nothing is judged.
fn summary_line(sheet: &Sheet, run: Option<u64>) -> String {
    let mut parts = vec![counts_line(sheet)];
    match sheet.refusal() {
        Some(refusal) => parts.push(format!("not judged: {refusal}")),
        None => {
            if sheet
                .summary
                .acceptance
                .as_ref()
                .is_some_and(|acceptance| acceptance.blocked)
            {
                parts.push("acceptance blocked".to_owned());
            }
            if sheet.summary.limits == LimitsSinceRun::Edited {
                parts.push(run.map_or_else(
                    || "limits edited since the run".to_owned(),
                    |run| format!("limits edited since Run {run}"),
                ));
            }
        }
    }
    parts.join(" \u{b7} ")
}

/// Where a row's worst value was: the trial, or the point.
pub fn worst_at(sheet: &Sheet, worst: &Worst) -> Option<String> {
    let point = worst
        .point
        .and_then(|point| sheet.points.get(point))
        .map(|point| point.label.clone());
    match &worst.member {
        Some(
            FamilyMemberId::MonteCarloTrial { index, .. }
            | FamilyMemberId::MonteCarloSequenceTrial { index, .. },
        ) => Some(match point.filter(|_| sheet.points.len() > 1) {
            Some(point) => format!("trial {index} \u{b7} {point}"),
            None => format!("trial {index}"),
        }),
        _ => point,
    }
}

/// A row's status in words.
pub fn status_word(status: RowStatus) -> &'static str {
    match status {
        RowStatus::Pass => "Pass",
        RowStatus::Near => "Near",
        RowStatus::Fail => "Fail",
        RowStatus::Error => "Error",
        RowStatus::NotMeasured => "Not measured",
        RowStatus::Waived => "Waived",
        RowStatus::Tracked => "Tracked",
    }
}

const fn status_severity(status: RowStatus) -> u8 {
    match status {
        RowStatus::Error => 5,
        RowStatus::Fail => 4,
        RowStatus::NotMeasured => 3,
        RowStatus::Near => 2,
        RowStatus::Waived => 1,
        RowStatus::Pass | RowStatus::Tracked => 0,
    }
}

/// A difference in the unit, signed either way.
fn signed_value(value: f64, unit: &str) -> String {
    let sign = if value > 0.0 { "+" } else { "" };
    format!("{sign}{}", format_value(value, unit))
}

/// Lay the sheet out for this frame, with the row `selected` names open to
/// editing.
fn sheet_view(model: &Model, selected: Option<&str>) -> SheetView {
    let editable = verification_plan_id(model).is_some();
    let Some(sheet) = model.simulation.verification_sheet() else {
        return SheetView {
            editable,
            columns: Vec::new(),
            rows: Vec::new(),
            keys: Vec::new(),
            groups: Vec::new(),
            summary: None,
        };
    };
    let trials = sheet.rows.iter().any(|row| {
        row.worst.as_ref().is_some_and(|worst| {
            matches!(
                worst.member,
                Some(
                    FamilyMemberId::MonteCarloTrial { .. }
                        | FamilyMemberId::MonteCarloSequenceTrial { .. }
                )
            )
        })
    });
    let statistics = sheet.rows.iter().any(|row| row.statistics.is_some());
    let mut columns = vec![Col::Specification, Col::Limit, Col::Nominal, Col::Worst];
    if sheet.points.len() > 1 || trials {
        columns.push(Col::At);
    }
    columns.push(Col::Margin);
    if statistics {
        columns.push(Col::Yield);
    }
    columns.push(Col::Status);

    let (order, groups) = grouped_order(model, sheet);
    let rows = order
        .iter()
        .map(|index| {
            let row = &sheet.rows[*index];
            let open = editable
                && selected.is_some_and(|selected| selected.eq_ignore_ascii_case(&row.measurement));
            RowView {
                measurement: row.measurement.clone(),
                name: row.name.clone(),
                unit: row.unit.clone(),
                comparison: row.comparison.clone(),
                cells: columns
                    .iter()
                    .map(|col| cell(sheet, row, *col, open))
                    .collect(),
            }
        })
        .collect::<Vec<_>>();
    let keys = rows
        .iter()
        .map(|row| RowKey::Text(row.measurement.clone()))
        .collect();
    let run = model.simulation.active_run().map(|run| run.id);
    SheetView {
        editable,
        columns,
        rows,
        keys,
        groups,
        summary: (!sheet.rows.is_empty()).then(|| Summary::plain(summary_line(sheet, run))),
    }
}

/// The rows in the order shown, and the heads over them: rows that name no
/// producing analysis first, then each named analysis's rows under its name,
/// in the order the plan first names it. With fewer than two analyses named,
/// plan order and no heads.
fn grouped_order(model: &Model, sheet: &Sheet) -> (Vec<usize>, Vec<Group>) {
    let mut producers = Vec::new();
    for row in &sheet.rows {
        if let Some(id) = row.producing_analysis
            && !producers.contains(&id)
        {
            producers.push(id);
        }
    }
    let plain = (0..sheet.rows.len()).collect::<Vec<_>>();
    if producers.len() < 2 {
        return (plain, Vec::new());
    }
    let plan = model.sim_setup.stable_analysis_plan().ok();
    let label = |id| {
        plan.and_then(|plan| {
            plan.instances()
                .iter()
                .position(|instance| instance.id() == id)
                .and_then(|index| plan.instance_list_label(index))
        })
        .unwrap_or_else(|| "An analysis no longer in the plan".to_owned())
    };
    let mut order = plain
        .iter()
        .copied()
        .filter(|index| sheet.rows[*index].producing_analysis.is_none())
        .collect::<Vec<_>>();
    let mut groups = Vec::new();
    for id in producers {
        groups.push(Group::new(label(id), order.len()));
        order.extend(
            plain
                .iter()
                .copied()
                .filter(|index| sheet.rows[*index].producing_analysis == Some(id)),
        );
    }
    (order, groups)
}

/// One cell of `row` in `col`; the name and the limit open for entry when the
/// row is selected and the plan can take an edit.
fn cell(sheet: &Sheet, row: &Row, col: Col, open: bool) -> Cell {
    let unit = row.unit.as_str();
    let value = |value: Option<f64>| {
        value.map_or_else(Cell::absent, |value| {
            Cell::number(format_value(value, unit)).sort_by(SortKey::Number(value))
        })
    };
    let editable = |cell: Cell| if open { cell.editable() } else { cell };
    match col {
        Col::Specification => {
            let cell = Cell::text(&row.name);
            let cell = if row.name.eq_ignore_ascii_case(&row.measurement) {
                cell
            } else {
                cell.hint(format!("Measurement {}", row.measurement))
            };
            editable(cell)
        }
        Col::Limit => {
            let cell = limit_text(&row.comparison, unit)
                .map_or_else(Cell::absent, Cell::number)
                .sort_by(SortKey::Text(String::new()));
            let cell = if row.guard_band > 0.0 {
                cell.hint(format!("Guard band {}", fmt_si_bound(row.guard_band, unit)))
            } else {
                cell
            };
            editable(cell)
        }
        Col::Nominal => value(row.nominal_value),
        Col::Worst => {
            let cell = value(row.worst.as_ref().and_then(|worst| worst.value));
            match row.status {
                Some(RowStatus::Fail | RowStatus::Error) => cell.judged(Verdict::Fail),
                Some(RowStatus::Near) => cell.judged(Verdict::Near),
                _ => cell,
            }
        }
        Col::At => row
            .worst
            .as_ref()
            .and_then(|worst| worst_at(sheet, worst))
            .map_or_else(Cell::absent, Cell::text),
        Col::Margin => row
            .worst
            .as_ref()
            .and_then(|worst| worst.signed_margin)
            .map_or_else(Cell::absent, |margin| {
                Cell::number(signed_value(margin, unit)).sort_by(SortKey::Number(margin))
            }),
        Col::Yield => row
            .statistics
            .as_ref()
            .and_then(|statistics| statistics.yield_count)
            .map_or_else(Cell::absent, |count| {
                let percent = count.percent();
                let cell =
                    Cell::number(format!("{percent:.1} %")).sort_by(SortKey::Number(percent));
                match count.interval {
                    Some((low, high)) => cell.hint(format!(
                        "{} of {} trials hold \u{b7} 95 % interval {low:.1} % \u{2026} {high:.1} %",
                        count.passing, count.total
                    )),
                    None => cell,
                }
            }),
        Col::Status => row.status.map_or_else(Cell::absent, |status| {
            let cell = Cell::text(status_word(status));
            let cell = match status {
                RowStatus::Fail | RowStatus::Error => cell.judged(Verdict::Fail),
                RowStatus::Near => cell.judged(Verdict::Near),
                RowStatus::Waived => match &row.waiver {
                    Some(waiver) => cell.hint(format!("Waived under {}", waiver.reference)),
                    None => cell,
                },
                _ => cell,
            };
            cell.sort_by(SortKey::Status(status_severity(status)))
        }),
    }
}

#[cfg(test)]
mod tests;

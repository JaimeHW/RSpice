//! Project > Configuration: RSpice's hierarchy editor.
//!
//! The page shows one configuration at a time, or the built-in Default a
//! project netlists with when none is used for runs: its global bindings
//! (top cell, DUT instance, view list, stop list, unresolved cells) and what
//! every instance of the design resolves to under them. Edits stay on the
//! page until Apply commits them to the project, and the tables always show
//! what the edited configuration resolves to, never a demonstration.

mod cells;
mod grid;
mod instances;
mod menu;

use super::*;

use crate::workbench::app::dialogs::project::configuration::ConfigurationTransaction;
use crate::workbench::app::dialogs::project::configuration_dialogs::{
    ConfigurationDialogRequest, configuration_actions_menu, open_configuration_dialog,
};
use crate::workbench::project::configuration_state::{
    ConfigurationResolution, ConfigurationSelection, used_for_runs,
};

/// What the toolbar band asked for this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToolbarIntent {
    UseForRuns(ConfigurationSelection),
    Apply,
    Revert,
    Open(ConfigurationDialogRequest),
}

pub fn configuration(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    // The page is one screen tall and its table takes what the bands leave
    // of it; where that is less than the table's least, as on a phone, the
    // page scrolls rather than running the table under the dock. The screen
    // is measured once, outside the scroll, so the table's height never
    // follows the scroll offset.
    let screen = ui.available_height();
    ScrollArea::vertical()
        .id_salt("workbench.project.configuration.page")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let size = vec2(ui.available_width(), screen);
            ui.allocate_ui_with_layout(size, Layout::top_down(Align::Min), |ui| {
                page(ui, cx);
            });
        });
}

fn page(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let resolution = cx.state.configuration.resolution(
        &cx.model.workspace,
        &cx.model.library_manager,
        &cx.model.workspace.content.active_view,
        &cx.model.schematic,
    );
    let selection = cx
        .state
        .configuration
        .selection(&cx.model.workspace.content.configuration_sets);
    match toolbar(ui, cx, selection, &resolution) {
        Some(ToolbarIntent::UseForRuns(target)) => {
            ProjectFeature::commit_configuration(cx, ConfigurationTransaction::UseForRuns(target))
        }
        Some(ToolbarIntent::Apply) => {
            ProjectFeature::commit_configuration(cx, ConfigurationTransaction::Apply)
        }
        Some(ToolbarIntent::Revert) => {
            if let ConfigurationSelection::Configuration(id) = selection {
                open_configuration_dialog(
                    &mut cx.state.configuration_dialog,
                    cx.model,
                    ConfigurationDialogRequest::Revert(id),
                );
            }
        }
        Some(ToolbarIntent::Open(request)) => {
            open_configuration_dialog(&mut cx.state.configuration_dialog, cx.model, request)
        }
        None => {}
    }
    if let Some(refusal) = cx.state.configuration.refusal.as_deref() {
        refusal_strip(ui, refusal);
    }
    grid::global_bindings(ui, cx, selection, &resolution);
    instances::tables(ui, cx, selection, &resolution);
}

/// What the band says and offers this frame.
struct Toolbar {
    selection: ConfigurationSelection,
    name: String,
    used_for_runs: bool,
    write_allowed: bool,
    dirty: bool,
    checking: bool,
    out_of_date: bool,
    edits: Option<(&'static str, Color32, Option<&'static str>)>,
    status: BindingStatus,
}

impl Toolbar {
    const fn custom(&self) -> bool {
        !matches!(self.selection, ConfigurationSelection::Default)
    }

    const fn new_label(&self) -> &'static str {
        if self.custom() {
            "New\u{2026}"
        } else {
            "New configuration\u{2026}"
        }
    }
}

/// The least of a configuration's name the band shows before it moves its
/// actions to a second row.
const MIN_NAME_WIDTH: f32 = 120.0;
/// The gap between the band's parts.
const TOOLBAR_GAP: f32 = 8.0;

/// Where the band puts the binding status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusPlace {
    /// At the right end of the name's row.
    NameRow,
    /// At the right end of the actions' row.
    ActionRow,
    /// On a row of its own under them.
    OwnRow,
}

/// How the band lays its parts out at the width it has: one row while the
/// name, its state, the actions and the status fit, the name shortened
/// first; then the actions on a row of their own. Nothing is drawn over
/// anything else, and the status is always whole.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ToolbarLayout {
    two_rows: bool,
    name_width: f32,
    status: StatusPlace,
}

fn toolbar_layout(width: f32, name: f32, state: f32, actions: f32, status: f32) -> ToolbarLayout {
    let gap = TOOLBAR_GAP;
    let one_row = width - state - actions - status - 3.0 * gap;
    if one_row >= name.min(MIN_NAME_WIDTH) {
        return ToolbarLayout {
            two_rows: false,
            name_width: name.min(one_row),
            status: StatusPlace::NameRow,
        };
    }
    let name_row = width - state - gap;
    let beside_name = name_row - status - gap;
    if beside_name >= name.min(MIN_NAME_WIDTH) {
        return ToolbarLayout {
            two_rows: true,
            name_width: name.min(beside_name),
            status: StatusPlace::NameRow,
        };
    }
    ToolbarLayout {
        two_rows: true,
        name_width: name.min(name_row.max(1.0)),
        status: if actions + gap + status <= width {
            StatusPlace::ActionRow
        } else {
            StatusPlace::OwnRow
        },
    }
}

/// The page's band: the configuration in front of the reader, whether runs
/// use it, its edits, its actions and how much of the design it binds.
fn toolbar(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
) -> Option<ToolbarIntent> {
    let t = Tokens::get(ui.ctx());
    let catalog = &cx.model.workspace.content.configuration_sets;
    let dirty = resolution.tables.dirty;
    // While an Apply or a Use for runs is checked, neither is offered again.
    let checking = cx.state.configuration.is_checking();
    // Edits the project changed under, in a part they change too: said at
    // once, not when Apply is refused.
    let out_of_date = cx.state.configuration.is_out_of_date(catalog);
    // Runs use what was applied; the tables show the edits.
    let edits = if checking {
        Some(("Checking\u{2026}", t.color.text_dim, None))
    } else if out_of_date {
        Some((
            "Edits out of date",
            t.color.warn,
            Some(
                "The configuration changed in a part these edits change too. Revert, then \
                 edit again.",
            ),
        ))
    } else {
        dirty.then_some(("Edits not applied", t.color.text_dim, None))
    };
    let bar = Toolbar {
        selection,
        name: selection_name(cx, selection),
        used_for_runs: used_for_runs(catalog, selection),
        write_allowed: !cx.model.safe_mode.project_read_only(),
        dirty,
        checking,
        out_of_date,
        edits,
        status: binding_status(resolution),
    };
    let mut intent = None;
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 0))
        .show(ui, |ui| {
            let width = ui.available_width().max(1.0);
            ui.set_width(width);
            let layout = toolbar_layout(
                width,
                text_width(ui, &bar.name, &name_font()),
                state_width(ui, &bar),
                actions_width(ui, &bar),
                status_width(ui, &bar.status),
            );
            let row_height = toolbar_row_height(&t);
            let row = |ui: &mut Ui, body: &mut dyn FnMut(&mut Ui)| {
                ui.allocate_ui_with_layout(
                    vec2(width, row_height),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = TOOLBAR_GAP;
                        body(ui);
                    },
                );
            };
            ui.spacing_mut().item_spacing.y = 0.0;
            row(ui, &mut |ui| {
                name_and_state(ui, &bar, layout.name_width, &mut intent);
                if !layout.two_rows {
                    actions(ui, &bar, &mut intent);
                }
                if layout.status == StatusPlace::NameRow {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        status_text(ui, &bar.status);
                    });
                }
            });
            if layout.two_rows {
                row(ui, &mut |ui| {
                    actions(ui, &bar, &mut intent);
                    if layout.status == StatusPlace::ActionRow {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            status_text(ui, &bar.status);
                        });
                    }
                });
            }
            if layout.status == StatusPlace::OwnRow {
                row(ui, &mut |ui| status_text(ui, &bar.status));
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    intent
}

/// A band row: a control and the room around it, from the metrics, so it
/// holds its controls under a finger.
fn toolbar_row_height(t: &Tokens) -> f32 {
    t.metrics.ctl_h + grid::ROW_PAD
}

fn name_font() -> egui::FontId {
    theme::sans(tokens::FS_1, FontWeight::SemiBold)
}

fn words_font() -> egui::FontId {
    theme::sans(tokens::FS_0, FontWeight::Regular)
}

fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
}

/// What the name's state takes: whether runs use it, and its edits.
fn state_width(ui: &mut Ui, bar: &Toolbar) -> f32 {
    let runs = if bar.used_for_runs {
        text_width(ui, "Used for runs", &words_font())
    } else {
        Button::new("Use for runs").measured_width(ui)
    };
    runs + bar.edits.map_or(0.0, |(edits, ..)| {
        TOOLBAR_GAP + text_width(ui, edits, &words_font())
    })
}

/// What the actions take, the gaps between them counted.
fn actions_width(ui: &mut Ui, bar: &Toolbar) -> f32 {
    let new = Button::new(bar.new_label()).measured_width(ui);
    if !bar.custom() {
        return new;
    }
    let side = Tokens::get(ui.ctx()).metrics.ctl_h;
    Button::new("Apply").measured_width(ui)
        + Button::new("Revert").measured_width(ui)
        + new
        + side
        + 3.0 * TOOLBAR_GAP
}

/// The configuration's name, shortened to `name_width`, and its state.
fn name_and_state(ui: &mut Ui, bar: &Toolbar, name_width: f32, intent: &mut Option<ToolbarIntent>) {
    let t = Tokens::get(ui.ctx());
    let shown = elide_text(ui, &bar.name, &name_font(), name_width);
    let said = ui.label(
        egui::RichText::new(&shown)
            .font(name_font())
            .color(t.color.text),
    );
    if shown != bar.name {
        said.on_hover_text(&bar.name);
    }
    if bar.used_for_runs {
        ui.label(
            egui::RichText::new("Used for runs")
                .font(words_font())
                .color(t.color.text_dim),
        );
    } else if Button::new("Use for runs")
        .enabled(bar.write_allowed && !bar.checking)
        .show(ui)
        .clicked()
    {
        *intent = Some(ToolbarIntent::UseForRuns(bar.selection));
    }
    if let Some((edits, color, why)) = bar.edits {
        let said = ui.label(egui::RichText::new(edits).font(words_font()).color(color));
        if let Some(why) = why {
            said.on_hover_text(why);
        }
    }
}

/// Apply, Revert, New and the configuration's other actions.
fn actions(ui: &mut Ui, bar: &Toolbar, intent: &mut Option<ToolbarIntent>) {
    if bar.custom() {
        if Button::new("Apply")
            .enabled(bar.write_allowed && bar.dirty && !bar.checking && !bar.out_of_date)
            .show(ui)
            .clicked()
        {
            *intent = Some(ToolbarIntent::Apply);
        }
        if Button::new("Revert").enabled(bar.dirty).show(ui).clicked() {
            *intent = Some(ToolbarIntent::Revert);
        }
    }
    if Button::new(bar.new_label())
        .enabled(bar.write_allowed)
        .show(ui)
        .clicked()
    {
        *intent = Some(ToolbarIntent::Open(ConfigurationDialogRequest::New));
    }
    if bar.custom() {
        let side = Tokens::get(ui.ctx()).metrics.ctl_h;
        let more = crate::ui::panels::icon_button(
            ui,
            WorkbenchIcon::More,
            "More configuration actions",
            false,
            vec2(side, side),
        );
        egui::Popup::menu(&more).show(|ui| {
            if let Some(request) = configuration_actions_menu(ui, bar.selection, bar.write_allowed)
            {
                *intent = Some(ToolbarIntent::Open(request));
                ui.close();
            }
        });
    }
}

/// The name the band and the prompts give `selection`.
fn selection_name(cx: &Cx<'_, ProjectFeature>, selection: ConfigurationSelection) -> String {
    match selection {
        ConfigurationSelection::Default => "Default".to_owned(),
        ConfigurationSelection::Configuration(id) => cx
            .model
            .workspace
            .content
            .configuration_sets
            .find(id)
            .map_or_else(String::new, |configuration| configuration.name().to_owned()),
    }
}

/// How much of the design the shown configuration binds, as the band states
/// it, and whether that is a problem.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BindingStatus {
    text: String,
    problem: Option<Problem>,
}

fn binding_status(resolution: &ConfigurationResolution) -> BindingStatus {
    if resolution.hierarchy.is_err() {
        return BindingStatus {
            text: "Hierarchy does not resolve".to_owned(),
            problem: Some(Problem::Failure),
        };
    }
    let tables = &resolution.tables;
    let total = tables.total_instances;
    let unbound = tables.unbound_instances;
    let mut text = if unbound == 0 {
        format!("{total} of {total} instance{} bound", plural_suffix(total))
    } else {
        format!(
            "{unbound} of {total} instance{} unbound",
            plural_suffix(total)
        )
    };
    let fallback = tables.fallback_instances;
    if fallback > 0 {
        text.push_str(&format!(" \u{00b7} {fallback} bound by fallback"));
    }
    BindingStatus {
        text,
        problem: (unbound > 0).then_some(Problem::Failure),
    }
}

/// The status, right-aligned: neutral words, with the kit's mark in front of
/// them when instances are unbound.
fn status_text(ui: &mut Ui, status: &BindingStatus) {
    let t = Tokens::get(ui.ctx());
    let galley = ui
        .painter()
        .layout_no_wrap(status.text.clone(), status_font(), t.color.text_dim);
    let (rect, response) = ui.allocate_exact_size(
        vec2(status_width(ui, status), toolbar_row_height(&t)),
        Sense::hover(),
    );
    let advance = status_advance(status);
    let announcement = status.problem.map_or_else(
        || status.text.clone(),
        |problem| problem.announce(&status.text),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    if let Some(problem) = status.problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(rect.left() + MARK_SIDE * 0.5, rect.center().y),
                vec2(MARK_SIDE, MARK_SIDE),
            ),
            problem,
        );
    }
    ui.painter().galley(
        pos2(
            rect.left() + advance,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        t.color.text_dim,
    );
}

fn status_font() -> egui::FontId {
    theme::mono(tokens::FS_0, FontWeight::Regular)
}

/// The room in front of the status's words: the mark's, when it has one.
fn status_advance(status: &BindingStatus) -> f32 {
    if status.problem.is_some() {
        MARK_ADVANCE
    } else {
        0.0
    }
}

/// What the status takes, its mark counted.
fn status_width(ui: &Ui, status: &BindingStatus) -> f32 {
    status_advance(status) + text_width(ui, &status.text, &status_font())
}

/// The refusal the last transaction on this page stated, under the band.
fn refusal_strip(ui: &mut Ui, refusal: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            problem_mark_line(ui, Problem::Failure, refusal);
        });
}

/// Side of a problem mark set in front of a line on this page, and the
/// advance from its left edge to the text after it: read from the kit.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};

/// A mark and then neutral words, laid out inside whatever frame the caller
/// has already opened.
///
/// The kit's `problem_line` carries a panel body's inset of its own, which
/// lands a second indent inside a framed strip; this one starts where its
/// caller's margin does.
fn problem_mark_line(ui: &mut Ui, problem: Problem, message: &str) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let width = ui.available_width().max(1.0);
    let galley = ui.painter().layout(
        message.to_owned(),
        font,
        t.color.text,
        (width - MARK_ADVANCE).max(1.0),
    );
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, galley.size().y.max(MARK_SIDE)), Sense::hover());
    let announcement = problem.announce(message);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    let first_line = galley
        .rows
        .first()
        .map_or(galley.size().y * 0.5, |row| row.rect().center().y);
    paint_problem_mark(
        ui.painter(),
        Rect::from_center_size(
            pos2(rect.left() + MARK_SIDE * 0.5, rect.top() + first_line),
            vec2(MARK_SIDE, MARK_SIDE),
        ),
        problem,
    );
    ui.painter().galley(
        pos2(rect.left() + MARK_ADVANCE, rect.top()),
        galley,
        t.color.text,
    );
}

/// A band over one of the page's sections. Its meta is a fact about the
/// section, never a verdict painted in a verdict's colour.
fn section_header(ui: &mut Ui, title: &str, meta: &str) {
    let t = Tokens::get(ui.ctx());
    // Words only, never touched: a list row tall at every density, and no
    // taller under a finger.
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width().max(1.0), t.metrics.row_h + 2.0),
        Sense::hover(),
    );
    let announcement = if meta.is_empty() {
        title.to_owned()
    } else {
        format!("{title}: {meta}")
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().text(
        rect.right_center() - vec2(10.0, 0.0),
        Align2::RIGHT_CENTER,
        meta,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

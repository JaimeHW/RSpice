//! Project > Configuration: RSpice's hierarchy editor.
//!
//! The page shows one configuration at a time, or the built-in Default a
//! project netlists with when none is used for runs: its global bindings
//! (top cell, DUT instance, view list, stop list, unresolved cells) and what
//! every instance of the design resolves to under them. Edits stay on the
//! page until Apply commits them to the project, and the tables always show
//! what the edited configuration resolves to, never a demonstration.
//!
//! One bar picks the configuration and acts on it and on the table, the
//! global bindings stand in one band under it, and the table takes the rest.

mod bar;
mod cells;
mod grid;
mod instances;
mod menu;

use super::*;

use crate::workbench::app::dialogs::project::configuration::ConfigurationTransaction;
use crate::workbench::app::dialogs::project::configuration_dialogs::{
    ConfigurationDialogRequest, open_configuration_dialog,
};
use crate::workbench::project::configuration_state::{
    ConfigurationResolution, ConfigurationSelection,
};

use bar::BarIntent;

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
    // The bar, the band and the table meet on their rules, with no gap.
    ui.spacing_mut().item_spacing.y = 0.0;
    let mut table = None;
    match bar::configuration_bar(ui, cx, selection, &resolution) {
        Some(BarIntent::Select(target)) => cx
            .state
            .configuration
            .request_selection(&cx.model.workspace.content.configuration_sets, target),
        Some(BarIntent::UseForRuns(target)) => {
            ProjectFeature::commit_configuration(cx, ConfigurationTransaction::UseForRuns(target))
        }
        Some(BarIntent::Apply) => {
            ProjectFeature::commit_configuration(cx, ConfigurationTransaction::Apply)
        }
        Some(BarIntent::Revert) => {
            if let ConfigurationSelection::Configuration(id) = selection {
                open_configuration_dialog(
                    &mut cx.state.configuration_dialog,
                    cx.model,
                    ConfigurationDialogRequest::Revert(id),
                );
            }
        }
        Some(BarIntent::Open(request)) => {
            open_configuration_dialog(&mut cx.state.configuration_dialog, cx.model, request)
        }
        Some(BarIntent::Table(action)) => table = Some(action),
        None => {}
    }
    if let Some(refusal) = cx.state.configuration.refusal.as_deref() {
        refusal_strip(ui, refusal);
    }
    grid::global_bindings(ui, cx, selection, &resolution);
    instances::tables(ui, cx, selection, &resolution, table);
}

/// The name the bar and the prompts give `selection`.
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

/// The refusal the last transaction on this page stated, under the bar.
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

//! Project > Configuration: RSpice's hierarchy editor.
//!
//! The page shows one configuration at a time, or the built-in Default a
//! project netlists with when none is used for runs: its global bindings
//! (top cell, DUT instance, view list, stop list, unresolved cells) and what
//! every instance of the design resolves to under them. Edits stay on the
//! page until Apply commits them to the project, and the tables always show
//! what the edited configuration resolves to, never a demonstration.

mod grid;
mod instances;
#[cfg(test)]
mod tests;

use super::*;

use crate::workbench::app::dialogs::project::configuration::{
    ConfigurationTransaction, commit_configuration_transaction,
};
use crate::workbench::app::dialogs::project::configuration_dialogs::{
    ConfigurationDialogRequest, configuration_actions_menu, open_configuration_dialog,
};
use crate::workbench::app_state::AppState;
use crate::workbench::project::configuration_state::{
    ConfigurationResolution, ConfigurationSelection, used_for_runs,
};

const TOOLBAR_HEIGHT: f32 = 32.0;
const SECTION_HEADER_HEIGHT: f32 = 30.0;

/// What the toolbar band asked for this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToolbarIntent {
    UseForRuns(ConfigurationSelection),
    Apply,
    Revert,
    Open(ConfigurationDialogRequest),
}

pub(super) fn configuration(ui: &mut Ui, app: &mut RSpiceApp) {
    let state = &mut app.state;
    let resolution = state.project.configuration.resolution(
        &state.model.workspace,
        &state.model.library_manager,
        &state.model.workspace.content.active_view,
        &state.model.schematic,
    );
    let selection = state
        .project
        .configuration
        .selection(&state.model.workspace.content.configuration_sets);
    match toolbar(ui, &app.state, selection, &resolution) {
        Some(ToolbarIntent::UseForRuns(target)) => {
            let refusal =
                commit_configuration_transaction(app, ConfigurationTransaction::UseForRuns(target))
                    .err();
            app.state.project.configuration.refusal = refusal;
        }
        Some(ToolbarIntent::Apply) => {
            let refusal =
                commit_configuration_transaction(app, ConfigurationTransaction::Apply).err();
            app.state.project.configuration.refusal = refusal;
        }
        Some(ToolbarIntent::Revert) => app.state.project.configuration.revert(),
        Some(ToolbarIntent::Open(request)) => open_configuration_dialog(&mut app.state, request),
        None => {}
    }
    if let Some(refusal) = app.state.project.configuration.refusal.as_deref() {
        refusal_strip(ui, refusal);
    }
    grid::global_bindings(ui, &mut app.state, selection, &resolution);
    instances::instances(ui, &mut app.state, selection, &resolution);
}

/// The page's one-row band: the configuration in front of the reader,
/// whether runs use it, its edits, and how much of the design it binds.
fn toolbar(
    ui: &mut Ui,
    state: &AppState,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
) -> Option<ToolbarIntent> {
    let t = Tokens::get(ui.ctx());
    let catalog = &state.model.workspace.content.configuration_sets;
    let page = &state.project.configuration;
    let name = selection_name(state, selection);
    let write_allowed = !state.model.safe_mode.project_read_only();
    let dirty = page.is_dirty(catalog);
    let status = binding_status(resolution);
    let mut intent = None;
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), TOOLBAR_HEIGHT),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    ui.label(
                        egui::RichText::new(&name)
                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    if used_for_runs(catalog, selection) {
                        ui.label(
                            egui::RichText::new("Used for runs")
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                    } else if Button::new("Use for runs")
                        .enabled(write_allowed)
                        .show(ui)
                        .clicked()
                    {
                        intent = Some(ToolbarIntent::UseForRuns(selection));
                    }
                    if selection != ConfigurationSelection::Default {
                        if Button::new("Apply")
                            .accent()
                            .enabled(write_allowed && dirty)
                            .show(ui)
                            .clicked()
                        {
                            intent = Some(ToolbarIntent::Apply);
                        }
                        if Button::new("Revert").enabled(dirty).show(ui).clicked() {
                            intent = Some(ToolbarIntent::Revert);
                        }
                    }
                    let new_label = if selection == ConfigurationSelection::Default {
                        "New configuration\u{2026}"
                    } else {
                        "New\u{2026}"
                    };
                    if Button::new(new_label)
                        .enabled(write_allowed)
                        .show(ui)
                        .clicked()
                    {
                        intent = Some(ToolbarIntent::Open(ConfigurationDialogRequest::New));
                    }
                    if selection != ConfigurationSelection::Default {
                        let more = crate::ui::panels::icon_button(
                            ui,
                            WorkbenchIcon::More,
                            "More configuration actions",
                            false,
                            vec2(28.0, 26.0),
                        );
                        egui::Popup::menu(&more).show(|ui| {
                            if let Some(request) =
                                configuration_actions_menu(ui, selection, write_allowed)
                            {
                                intent = Some(ToolbarIntent::Open(request));
                                ui.close();
                            }
                        });
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        status_text(ui, &status);
                    });
                },
            );
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    intent
}

/// The name the band and the prompts give `selection`.
fn selection_name(state: &AppState, selection: ConfigurationSelection) -> String {
    match selection {
        ConfigurationSelection::Default => "Default".to_owned(),
        ConfigurationSelection::Configuration(id) => state
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
    let hierarchy = match &resolution.hierarchy {
        Ok(hierarchy) => hierarchy,
        Err(_) => {
            return BindingStatus {
                text: "Hierarchy does not resolve".to_owned(),
                problem: Some(Problem::Failure),
            };
        }
    };
    let total = hierarchy.total_instances;
    let unbound = hierarchy.unresolved_instances();
    let mut text = if unbound == 0 {
        format!("{total} of {total} instance{} bound", plural_suffix(total))
    } else {
        format!(
            "{unbound} of {total} instance{} unbound",
            plural_suffix(total)
        )
    };
    let fallback = hierarchy
        .bindings
        .iter()
        .filter(|binding| binding.used_review_fallback)
        .map(|binding| binding.instance_count)
        .sum::<usize>();
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
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let galley = ui
        .painter()
        .layout_no_wrap(status.text.clone(), font, t.color.text_dim);
    let advance = if status.problem.is_some() {
        MARK_ADVANCE
    } else {
        0.0
    };
    let (rect, response) = ui.allocate_exact_size(
        vec2(galley.size().x + advance, TOOLBAR_HEIGHT),
        Sense::hover(),
    );
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
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width().max(1.0), SECTION_HEADER_HEIGHT),
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

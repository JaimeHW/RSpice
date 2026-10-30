//! Project configuration workspace.
//!
//! This surface is a read/write projection of the project-owned
//! [`ConfigurationSetCatalog`](crate::state::ConfigurationSetCatalog) and of
//! the built-in Default order a project netlists with when no configuration
//! is used for runs. It never substitutes demonstration bindings: every list,
//! binding and count comes from the selected configuration and the live
//! design.

use super::*;

use crate::state::{ConfigurationSet, ConfigurationSetOverride};
use crate::workbench::app::dialogs::project::configuration::{
    ConfigurationTransaction, commit_configuration_transaction,
};
use crate::workbench::project::configuration_state::{
    ConfigurationSelection, built_in, used_for_runs,
};

const TOOLBAR_HEIGHT: f32 = 32.0;
const SECTION_HEADER_HEIGHT: f32 = 30.0;
const PRECEDENCE_ROW_HEIGHT: f32 = 34.0;
const PROPERTY_ROW_HEIGHT: f32 = 28.0;
const TABLE_HEADER_HEIGHT: f32 = 28.0;
const TABLE_ROW_HEIGHT: f32 = 32.0;
const TABLE_MIN_WIDTH: f32 = 850.0;
const SPLIT_BREAKPOINT: f32 = 640.0;
const LEFT_PANE_SHARE: f32 = 0.38;
const FILTER_STATE_ID: &str = "workbench.project.configuration.override-filter";

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectionFacts {
    root: String,
    binding_count: usize,
    behavioral_binding_count: usize,
    configuration_revision: u64,
    configuration_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectionState {
    Executable(ProjectionFacts),
    Blocked(String),
}

/// What the toolbar band asked for this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToolbarIntent {
    UseForRuns(ConfigurationSelection),
    New,
}

pub(super) fn configuration(ui: &mut Ui, app: &mut RSpiceApp) {
    let selection = app
        .state
        .project
        .configuration
        .selection(&app.state.model.workspace.content.configuration_sets);
    match toolbar(ui, app, selection) {
        Some(ToolbarIntent::UseForRuns(target)) => {
            let refusal =
                commit_configuration_transaction(app, ConfigurationTransaction::UseForRuns(target))
                    .err();
            app.state.project.configuration.refusal = refusal;
        }
        Some(ToolbarIntent::New) => Command::ConfigurationSets.execute(app),
        None => {}
    }
    if let Some(refusal) = app.state.project.configuration.refusal.as_deref() {
        refusal_strip(ui, refusal);
    }

    let ConfigurationSelection::Configuration(id) = selection else {
        default_bindings(ui, app);
        return;
    };
    let Some(configuration) = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .find(id)
        .cloned()
    else {
        return;
    };

    let inspection = app.state.model.workspace.inspect_design_projection(
        &app.state.model.library_manager,
        &app.state.model.workspace.content.active_view,
        &app.state.model.schematic,
    );
    let resolution = inspection
        .as_ref()
        .ok()
        .map(|projection| projection.hierarchy_resolution());
    let projection = inspection
        .clone()
        .and_then(|projection| projection.into_execution());
    let projection_state = projection_facts(projection.as_ref());
    let definition = configuration.definition();

    let width = visible_workspace_width(ui);
    if width >= SPLIT_BREAKPOINT {
        let left_width = (width * LEFT_PANE_SHARE).floor().max(320.0);
        let right_width = (width - left_width - 1.0).max(280.0);
        let shown = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(vec2(left_width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.set_width(left_width);
                precedence_panel(ui, &configuration);
                execution_contract(ui, &configuration, app);
                connectivity_contract(ui, app);
            });
            ui.allocate_ui_with_layout(
                vec2(right_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_width(right_width);
                    overrides_panel(
                        ui,
                        definition.overrides.as_slice(),
                        projection.as_ref().ok().map(|value| value.plan()),
                    );
                    validation_panel(ui, &configuration, &projection_state, resolution);
                },
            );
        });
        ui.painter().vline(
            shown.response.rect.left() + left_width + 0.5,
            shown.response.rect.y_range(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
        );
    } else {
        precedence_panel(ui, &configuration);
        execution_contract(ui, &configuration, app);
        connectivity_contract(ui, app);
        overrides_panel(
            ui,
            definition.overrides.as_slice(),
            projection.as_ref().ok().map(|value| value.plan()),
        );
        validation_panel(ui, &configuration, &projection_state, resolution);
    }
}

/// The page's one-row band: the configuration in front of the reader, and
/// whether runs use it.
fn toolbar(
    ui: &mut Ui,
    app: &RSpiceApp,
    selection: ConfigurationSelection,
) -> Option<ToolbarIntent> {
    let t = Tokens::get(ui.ctx());
    let catalog = &app.state.model.workspace.content.configuration_sets;
    let name = match selection {
        ConfigurationSelection::Default => "Default".to_owned(),
        ConfigurationSelection::Configuration(id) => catalog
            .find(id)
            .map_or_else(String::new, |configuration| configuration.name().to_owned()),
    };
    let write_allowed = !app.state.model.safe_mode.project_read_only();
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
                        intent = Some(ToolbarIntent::New);
                    }
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

/// The built-in order, stated: what a project netlists with when no
/// configuration is used for runs. It is read-only; a different order is a
/// new configuration.
fn default_bindings(ui: &mut Ui, app: &RSpiceApp) {
    let project = &app.state.model.workspace.content.project;
    section_header(ui, "Global bindings", "built in", None);
    dense_property_row(
        ui,
        "Top cell",
        &format!("{}/{}/schematic", project.root_library, project.top_cell),
        true,
    );
    dense_property_row(ui, "DUT instance", "\u{2014}", true);
    dense_property_row(ui, "View list", &default_view_list_text(), false);
    dense_property_row(ui, "Stop list", &default_stop_list_text(), true);
    dense_property_row(ui, "Unresolved cells", "Stop the netlist", false);
}

/// The built-in view list in words: its first entry is whatever view the
/// instance itself asks for.
fn default_view_list_text() -> String {
    let order = built_in::default_view_search_order("schematic", false);
    let mut words = vec!["the instance's own view".to_owned()];
    words.extend(order.into_iter().skip(1));
    words.join(", then ")
}

fn default_stop_list_text() -> String {
    built_in::DEFAULT_STOP_VIEW_TYPES
        .iter()
        .map(|view_type| view_type.display_name())
        .collect::<Vec<_>>()
        .join(", ")
}

fn precedence_panel(ui: &mut Ui, configuration: &ConfigurationSet) {
    let definition = configuration.definition();
    section_header(
        ui,
        "Global view precedence",
        &format!(
            "{} ordered view{}",
            definition.executable_view_policy.len(),
            plural_suffix(definition.executable_view_policy.len())
        ),
        None,
    );
    for (index, view) in definition.executable_view_policy.iter().enumerate() {
        let is_stop = definition
            .stop_views
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(view));
        precedence_row(ui, index + 1, view, is_stop);
    }
}

fn precedence_row(ui: &mut Ui, ordinal: usize, view: &str, stop_view: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width().max(1.0), PRECEDENCE_ROW_HEIGHT),
        Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "View precedence {ordinal}: {view}{}",
                if stop_view { ", stop view" } else { "" }
            ),
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        format!("{ordinal:02}"),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
    ui.painter().text(
        rect.left_center() + vec2(44.0, 0.0),
        Align2::LEFT_CENTER,
        view,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        t.color.text,
    );
    ui.painter().text(
        rect.right_center() - vec2(10.0, 0.0),
        Align2::RIGHT_CENTER,
        if stop_view {
            "Stop boundary"
        } else {
            "Descend"
        },
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if stop_view {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
}

fn execution_contract(ui: &mut Ui, configuration: &ConfigurationSet, app: &RSpiceApp) {
    let definition = configuration.definition();
    let technology = app
        .state
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .map(crate::state::ProjectTechnologyBinding::display_label)
        .or_else(|| app.state.model.workspace.content.project.technology.clone())
        .unwrap_or_else(|| "Not attached".to_owned());
    section_header(
        ui,
        "Execution contract",
        &format!("configuration r{}", configuration.revision()),
        None,
    );
    dense_property_row(ui, "Root", &definition.root.display_path(), true);
    dense_property_row(ui, "DUT occurrence", &definition.dut_path, true);
    dense_property_row(ui, "Owner", &definition.owner, false);
    dense_property_row(
        ui,
        "Unresolved binding",
        definition.unresolved_policy.label(),
        false,
    );
    dense_property_row(
        ui,
        "Black-box boundary",
        definition.black_box_policy.label(),
        false,
    );
    dense_property_row(ui, "Model profile", definition.model_profile.label(), false);
    dense_property_row(ui, "Project technology", &technology, false);
    dense_property_row(
        ui,
        "Semantic digest",
        &short_digest(&configuration.semantic_digest().to_string()),
        true,
    );
}

fn connectivity_contract(ui: &mut Ui, app: &RSpiceApp) {
    let connectivity = &app.state.model.workspace.content.connectivity;
    let validation = connectivity.validate();
    // A contract that holds is the ordinary case and says nothing; the error
    // below the rows is what a blocked one has to say.
    section_header(
        ui,
        "Connectivity contract",
        if validation.is_ok() { "" } else { "blocked" },
        validation.is_err().then_some(Problem::Failure),
    );
    dense_property_row(
        ui,
        "Width mismatch",
        connectivity.policy.width_mismatch.label(),
        false,
    );
    dense_property_row(
        ui,
        "Global promotion",
        connectivity.policy.global_promotion.label(),
        false,
    );
    dense_property_row(
        ui,
        "Alias comparison",
        connectivity.policy.alias_comparison.label(),
        false,
    );
    if let Err(error) = validation {
        inline_diagnostic(ui, &error);
    }
}

fn dense_property_row(ui: &mut Ui, label: &str, value: &str, mono: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width().max(1.0), PROPERTY_ROW_HEIGHT),
        Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{label}: {value}"),
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter()
        .with_clip_rect(Rect::from_min_max(
            pos2(rect.left() + rect.width() * 0.43, rect.top()),
            rect.right_bottom(),
        ))
        .text(
            rect.right_center() - vec2(10.0, 0.0),
            Align2::RIGHT_CENTER,
            value,
            if mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Medium)
            },
            t.color.text,
        );
}

fn overrides_panel(
    ui: &mut Ui,
    overrides: &[ConfigurationSetOverride],
    plan: Option<&crate::state::workspace::ConfigurationExecutionPlan>,
) {
    let mut query = ui.ctx().data(|data| {
        data.get_temp::<String>(egui::Id::new(FILTER_STATE_ID))
            .unwrap_or_default()
    });
    let visible = filtered_overrides(overrides, &query);
    let meta = format!("{} / {} shown", visible.len(), overrides.len());
    section_header_with_filter(ui, "Instance overrides", &meta, &mut query);
    ui.ctx().data_mut(|data| {
        data.insert_temp(egui::Id::new(FILTER_STATE_ID), query);
    });

    let visible_width = ui.available_width().max(1.0);
    ScrollArea::horizontal()
        .id_salt("workbench.project.configuration.overrides")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = visible_width.max(TABLE_MIN_WIDTH);
            ui.set_min_width(width);
            override_table_row(
                ui,
                width,
                [
                    "Instance",
                    "Ordered views",
                    "Resolved view",
                    "Stop",
                    "Model section",
                    "Platform / status",
                ],
                true,
                None,
            );
            if visible.is_empty() {
                let message = if overrides.is_empty() {
                    "No instance overrides are defined; the global precedence applies."
                } else {
                    "No instance override matches the current filter."
                };
                empty_table_row(ui, width, message);
            }
            for override_ in visible {
                // A pattern names no single instance, so only an exact-path
                // override has a binding to show.
                let binding = crate::state::InstancePath::parse_legacy(&override_.instance_path)
                    .ok()
                    .and_then(|path| plan.and_then(|plan| plan.binding(&path)));
                let ordered_views = override_.executable_views.join(" \u{2192} ");
                let resolved = binding
                    .map(|binding| binding.resolved_reference().display_path())
                    .unwrap_or_else(|| "\u{2014}".to_owned());
                let stop = override_.stop_view.as_deref().unwrap_or("\u{2014}");
                let model_section = override_.model_section.as_deref().unwrap_or("\u{2014}");
                let platforms = override_
                    .eligible_platforms
                    .iter()
                    .map(|platform| platform.label())
                    .collect::<Vec<_>>()
                    .join(" \u{00b7} ");
                let status = if binding.is_some() {
                    format!("{platforms} / resolved")
                } else {
                    format!("{platforms} / unresolved")
                };
                override_table_row(
                    ui,
                    width,
                    [
                        &override_.instance_path,
                        &ordered_views,
                        &resolved,
                        stop,
                        model_section,
                        &status,
                    ],
                    false,
                    binding.is_none().then_some(Problem::Warning),
                );
            }
        });
}

fn section_header_with_filter(ui: &mut Ui, title: &str, meta: &str, query: &mut String) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width().max(1.0), SECTION_HEADER_HEIGHT + 4.0),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(8.0, 3.0)))
            .layout(Layout::left_to_right(Align::Center)),
        |ui| {
            ui.label(
                egui::RichText::new(title)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(meta)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.add_sized(
                    [210.0_f32.min(ui.available_width().max(90.0)), 24.0],
                    egui::TextEdit::singleline(query)
                        .hint_text("Filter path, view, section\u{2026}")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
                );
            });
        },
    );
}

fn override_table_row(
    ui: &mut Ui,
    width: f32,
    cells: [&str; 6],
    header: bool,
    status_problem: Option<Problem>,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header {
        TABLE_HEADER_HEIGHT
    } else {
        TABLE_ROW_HEIGHT
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            if header {
                format!("Instance override columns: {}", cells.join(", "))
            } else {
                format!(
                    "Instance override {}: views {}; resolved {}; stop {}; model section {}; {}",
                    cells[0], cells[1], cells[2], cells[3], cells[4], cells[5]
                )
            },
        )
    });
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let fractions = [0.19_f32, 0.19, 0.20, 0.10, 0.13, 0.19];
    let mut left = rect.left();
    for (index, (value, fraction)) in cells.iter().zip(fractions).enumerate() {
        let right = if index + 1 == cells.len() {
            rect.right()
        } else {
            left + rect.width() * fraction
        };
        let cell = Rect::from_min_max(pos2(left, rect.top()), pos2(right, rect.bottom()));
        if index > 0 {
            painter.vline(left, rect.y_range(), Stroke::new(1.0, t.color.border));
        }
        // Only the status column can carry a mark, and only an override the
        // configuration could not bind has one.
        let mark = (index + 1 == cells.len())
            .then_some(status_problem)
            .flatten();
        if let Some(problem) = mark {
            paint_problem_mark(
                &painter,
                Rect::from_center_size(
                    pos2(cell.left() + 7.0 + MARK_SIDE * 0.5, cell.center().y),
                    vec2(MARK_SIDE, MARK_SIDE),
                ),
                problem,
            );
        }
        let text_left = 7.0 + if mark.is_some() { MARK_ADVANCE } else { 0.0 };
        let font = if header {
            theme::sans(tokens::FS_0, FontWeight::Medium)
        } else {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        };
        // A cell that simply clips cuts its value mid-glyph at the pane edge,
        // which is what the platform column did: elide it so the reader can
        // see that there is more rather than read half a word.
        let text = elide_text(ui, value, &font, (cell.width() - text_left - 7.0).max(1.0));
        painter
            .with_clip_rect(cell.shrink2(vec2(text_left, 0.0)))
            .text(
                cell.left_center() + vec2(text_left, 0.0),
                Align2::LEFT_CENTER,
                text,
                font,
                if header {
                    t.color.text_faint
                } else {
                    t.color.text_dim
                },
            );
        left = right;
    }
}

fn empty_table_row(ui: &mut Ui, width: f32, message: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, TABLE_ROW_HEIGHT), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), message)
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        message,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

fn validation_panel(
    ui: &mut Ui,
    configuration: &ConfigurationSet,
    projection: &ProjectionState,
    resolution: Option<&crate::state::workspace::HierarchyResolution>,
) {
    let valid = matches!(projection, ProjectionState::Executable(_))
        && resolution.is_some_and(crate::state::workspace::HierarchyResolution::is_valid);
    // "Executable" is what the rows below already read as; only a blocked
    // configuration is worth a word in the band.
    section_header(
        ui,
        "Execution validation",
        if valid { "" } else { "blocked" },
        (!valid).then_some(Problem::Failure),
    );
    dense_property_row(
        ui,
        "Hierarchy",
        &resolution.map_or_else(
            || "Unavailable".to_owned(),
            |resolution| {
                format!(
                    "{} / {} instances resolved",
                    resolution.resolved_instances, resolution.total_instances
                )
            },
        ),
        true,
    );
    dense_property_row(
        ui,
        "Reviewed fallbacks",
        &resolution.map_or_else(
            || "Unavailable".to_owned(),
            |resolution| {
                resolution
                    .bindings
                    .iter()
                    .filter(|binding| binding.used_review_fallback)
                    .count()
                    .to_string()
            },
        ),
        true,
    );
    dense_property_row(
        ui,
        "Overrides",
        &configuration.overrides().len().to_string(),
        true,
    );
    match projection {
        ProjectionState::Executable(facts) => {
            dense_property_row(ui, "Execution root", &facts.root, true);
            dense_property_row(ui, "Bindings", &facts.binding_count.to_string(), true);
            dense_property_row(
                ui,
                "Behavioral bindings",
                &facts.behavioral_binding_count.to_string(),
                true,
            );
            dense_property_row(
                ui,
                "Configuration revision",
                &facts.configuration_revision.to_string(),
                true,
            );
            dense_property_row(ui, "Execution digest", &facts.configuration_digest, true);
        }
        ProjectionState::Blocked(error) => inline_diagnostic(ui, error),
    }
}

fn projection_facts(
    projection: Result<
        &crate::state::workspace::ConfigurationExecutionProjection,
        &crate::state::workspace::ConfigurationExecutionPlanError,
    >,
) -> ProjectionState {
    let projection = match projection {
        Ok(projection) => projection,
        Err(error) => return ProjectionState::Blocked(error.to_string()),
    };
    if let Err(error) = projection.connectivity().validate() {
        return ProjectionState::Blocked(format!(
            "Project connectivity contract is invalid: {error}"
        ));
    }
    let plan = projection.plan();
    ProjectionState::Executable(ProjectionFacts {
        root: plan.root().display_path(),
        binding_count: plan.bindings().len(),
        behavioral_binding_count: plan
            .bindings()
            .filter(|binding| binding.project_veriloga().is_some())
            .count(),
        configuration_revision: plan.configuration_revision(),
        configuration_digest: short_digest(&plan.configuration_digest().to_string()),
    })
}

/// A band over one of the page's property lists. Its meta is a fact about
/// the list, never a verdict painted in a verdict's colour: a section that is
/// in order says nothing, and one that is not carries a mark.
fn section_header(ui: &mut Ui, title: &str, meta: &str, problem: Option<Problem>) {
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
    let announcement = problem.map_or(announcement.clone(), |problem| {
        problem.announce(&announcement)
    });
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
    let meta_right = ui
        .painter()
        .text(
            rect.right_center() - vec2(10.0, 0.0),
            Align2::RIGHT_CENTER,
            meta,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .left();
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(meta_right - 6.0 - MARK_SIDE * 0.5, rect.center().y),
                vec2(MARK_SIDE, MARK_SIDE),
            ),
            problem,
        );
    }
}

fn inline_diagnostic(ui: &mut Ui, message: &str) {
    egui::Frame::new()
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            problem_mark_line(ui, Problem::Failure, message);
        });
}

fn filtered_overrides<'a>(
    overrides: &'a [ConfigurationSetOverride],
    query: &str,
) -> Vec<&'a ConfigurationSetOverride> {
    let query = query.trim().to_ascii_lowercase();
    overrides
        .iter()
        .filter(|override_| override_matches_query(override_, &query))
        .collect()
}

fn override_matches_query(override_: &ConfigurationSetOverride, normalized_query: &str) -> bool {
    normalized_query.is_empty()
        || override_
            .instance_path
            .to_ascii_lowercase()
            .contains(normalized_query)
        || override_
            .executable_views
            .iter()
            .any(|value| value.to_ascii_lowercase().contains(normalized_query))
        || override_
            .stop_view
            .as_deref()
            .is_some_and(|value| value.to_ascii_lowercase().contains(normalized_query))
        || override_
            .model_section
            .as_deref()
            .is_some_and(|value| value.to_ascii_lowercase().contains(normalized_query))
        || override_.eligible_platforms.iter().any(|platform| {
            platform
                .label()
                .to_ascii_lowercase()
                .contains(normalized_query)
        })
}

fn short_digest(value: &str) -> String {
    if value.chars().count() <= 16 {
        return value.to_owned();
    }
    let head = value.chars().take(8).collect::<String>();
    let tail = value
        .chars()
        .rev()
        .take(8)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("{head}\u{2026}{tail}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ConfigurationPlatform;

    #[test]
    fn the_configuration_surface_counts_only_the_populated_variant() {
        for width in [640.0, 1280.0] {
            let mut app = RSpiceApp::test_instance();
            app.state = crate::workbench::examples::hierarchy_reference::build().state;
            crate::workbench::examples::hierarchy_reference::omit_missing_instance(&mut app.state);
            let projection = app
                .state
                .model
                .workspace
                .design_projection(
                    &app.state.model.library_manager,
                    &app.state.model.workspace.content.active_view,
                    &app.state.model.schematic,
                )
                .unwrap();
            let expected = projection.hierarchy_resolution();
            let authored = app.state.model.workspace.resolve_hierarchy_with_active(
                &app.state.model.library_manager,
                &app.state.model.workspace.content.active_view,
                &app.state.model.schematic,
            );
            assert_ne!(authored.total_instances, expected.total_instances);
            let label = format!(
                "Hierarchy: {} / {} instances resolved",
                expected.resolved_instances, expected.total_instances
            );
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 2400.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    egui::CentralPanel::default().show(ui, |ui| configuration(ui, &mut app));
                },
            );
            let nodes = output.platform_output.accesskit_update.unwrap().nodes;
            assert!(
                nodes
                    .iter()
                    .any(|(_, node)| node.value() == Some(label.as_str())),
                "the {width}px surface must state {label}"
            );
        }
    }

    #[test]
    fn unavailable_hierarchy_metrics_are_not_rendered_as_zero_counts() {
        let fixture = crate::workbench::examples::hierarchy_reference::build();
        let configuration = fixture
            .state
            .model
            .workspace
            .content
            .configuration_sets
            .active()
            .unwrap();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(480.0, 800.0),
                )),
                ..Default::default()
            },
            |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    validation_panel(
                        ui,
                        configuration,
                        &ProjectionState::Blocked("Invalid annotation".to_owned()),
                        None,
                    )
                });
            },
        );
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        assert!(
            nodes
                .iter()
                .any(|(_, node)| node.value() == Some("Hierarchy: Unavailable"))
        );
        assert!(
            nodes
                .iter()
                .any(|(_, node)| node.value() == Some("Reviewed fallbacks: Unavailable"))
        );
        assert!(
            !nodes
                .iter()
                .any(|(_, node)| node.value().is_some_and(|label| label.contains("0 / 0")))
        );
    }

    fn override_row(path: &str, view: &str, section: Option<&str>) -> ConfigurationSetOverride {
        ConfigurationSetOverride {
            instance_path: path.to_owned(),
            executable_views: vec![view.to_owned()],
            stop_view: Some(view.to_owned()),
            model_section: section.map(str::to_owned),
            eligible_platforms: vec![ConfigurationPlatform::Desktop],
        }
    }

    #[test]
    fn override_filter_searches_only_retained_configuration_fields() {
        let overrides = vec![
            override_row("/XAFE/X1", "schematic", Some("TT")),
            override_row("/XADC/X2", "veriloga", None),
        ];

        assert_eq!(filtered_overrides(&overrides, "xafe").len(), 1);
        assert_eq!(filtered_overrides(&overrides, "VERILOGA").len(), 1);
        assert_eq!(filtered_overrides(&overrides, "tt").len(), 1);
        assert_eq!(filtered_overrides(&overrides, "desktop").len(), 2);
        assert!(filtered_overrides(&overrides, "unretained reason").is_empty());
    }

    #[test]
    fn digest_copy_is_stable_without_claiming_a_different_identity() {
        assert_eq!(short_digest("short"), "short");
        assert_eq!(
            short_digest("0123456789abcdef0123456789abcdef"),
            "01234567\u{2026}89abcdef"
        );
    }

    /// Every string one pass of `body` painted, with the cell colours it used.
    fn painted(mut body: impl FnMut(&mut Ui)) -> Vec<(String, Color32)> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(560.0, 200.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| body(ui));
            },
        );
        fn collect(shape: &egui::epaint::Shape, out: &mut Vec<(String, Color32)>) {
            match shape {
                egui::epaint::Shape::Text(text) => out.push((
                    text.galley.job.text.clone(),
                    text.galley
                        .job
                        .sections
                        .first()
                        .map_or(text.fallback_color, |section| section.format.color),
                )),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, out);
                    }
                }
                _ => {}
            }
        }
        let mut out = Vec::new();
        for shape in &output.shapes {
            collect(&shape.shape, &mut out);
        }
        out
    }

    /// An override row's platform column used to run off the pane: the cell
    /// clipped its value mid-glyph at the table's right edge, so the reader
    /// saw half a word with nothing to say there was more. Every cell elides.
    #[test]
    fn an_override_cell_too_wide_for_its_column_elides_rather_than_clipping() {
        let platforms = "Desktop \u{00b7} Browser \u{00b7} Managed runtime / unresolved";
        let painted = painted(|ui| {
            override_table_row(
                ui,
                TABLE_MIN_WIDTH,
                [
                    "/XAFE/X1",
                    "schematic",
                    "\u{2014}",
                    "\u{2014}",
                    "TT",
                    platforms,
                ],
                false,
                Some(Problem::Warning),
            );
        });
        let status = painted
            .last()
            .expect("the row paints its status cell last")
            .clone();
        assert!(
            status.0.ends_with('\u{2026}')
                && platforms.starts_with(status.0.trim_end_matches('\u{2026}')),
            "the status cell was clipped rather than elided: {status:?}"
        );
    }

    /// The status column is the one cell that can carry a mark, and its words
    /// are never the colour of the verdict.
    #[test]
    fn an_unresolved_override_is_marked_rather_than_painted_orange() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let tokens = Tokens::get(&ctx);
        for problem in [None, Some(Problem::Warning)] {
            for (text, color) in painted(|ui| {
                override_table_row(
                    ui,
                    TABLE_MIN_WIDTH,
                    ["/XAFE/X1", "schematic", "x", "y", "TT", "Desktop"],
                    false,
                    problem,
                )
            }) {
                assert!(
                    color != tokens.color.warn
                        && color != tokens.color.err
                        && color != tokens.color.ok,
                    "{text:?} is painted in a severity tone"
                );
            }
        }
    }
}

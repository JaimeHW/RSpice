//! Unified console, problems, measurements, and task history.

use egui::{Align, Grid, Layout, ScrollArea, Sense, Ui, Vec2};

use crate::diagnostics::LogSeverity;
use crate::simulation::automation::CommandOutput;
use crate::state::SimulationState;
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::panels::ConsoleHistoryItem;

use crate::workbench::commands::CommandAvailability;
use crate::workbench::commands::vocabulary::{Command, command_catalog};
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::ConsolePage;

const CONSOLE_HEADER_HEIGHT: f32 = 31.0;
const CONSOLE_TOUCH_HEADER_HEIGHT: f32 = 44.0;
const CONSOLE_ACTION_SIZE: f32 = 27.0;
const CONSOLE_ACTION_MARGIN_RIGHT: f32 = 3.0;
const CONSOLE_BODY_PADDING_TOP: f32 = 7.0;
const CONSOLE_BODY_PADDING_BOTTOM: f32 = 7.0;
const CONSOLE_BODY_PADDING_X: f32 = 10.0;
const CONSOLE_ROW_MIN_HEIGHT: f32 = 16.0;
const CONSOLE_TIME_WIDTH: f32 = 58.0;
const CONSOLE_SOURCE_WIDTH: f32 = 62.0;
const CONSOLE_COLUMN_GAP: f32 = 9.0;
const CONSOLE_FONT_SIZE: f32 = tokens::FS_1;
const EMPTY_HINT_PADDING_X: i8 = 12;
const EMPTY_HINT_PADDING_Y: i8 = 20;
const INTERACTIVE_INPUT_GAP: f32 = 6.0;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    ui.spacing_mut().item_spacing.y = 0.0;
    header(ui, app, layout);
    if !layout.show_console_body {
        return;
    }
    let page = app.state.workbench.console_page;
    let body_rect = ui.available_rect_before_wrap();
    ui.painter()
        .rect_filled(body_rect, 0.0, Tokens::get(ui.ctx()).color.bg_inset);
    let content_rect = egui::Rect::from_min_max(
        egui::pos2(
            body_rect.left() + CONSOLE_BODY_PADDING_X,
            body_rect.top() + CONSOLE_BODY_PADDING_TOP,
        ),
        egui::pos2(
            body_rect.right() - CONSOLE_BODY_PADDING_X,
            body_rect.bottom() - CONSOLE_BODY_PADDING_BOTTOM,
        ),
    );
    let body = ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(Layout::top_down(Align::Min)),
        |ui| match page {
            ConsolePage::Console => console(ui, app),
            ConsolePage::Interactive => interactive(ui, app),
            ConsolePage::Problems => problems(ui, app),
            ConsolePage::Measurements => measurements(ui, app),
            ConsolePage::TaskLog => task_log(ui, app),
        },
    );
    ui.ctx().accesskit_node_builder(body.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabPanel);
        node.set_label(page.label());
    });
}

fn header(ui: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ui.ctx());
    let problems = app
        .state
        .dialogs
        .drc_results
        .as_ref()
        .map_or(0, |result| result.total_count())
        + app.state.log_buffer.count_by_severity(LogSeverity::Error)
        + app.state.log_buffer.count_by_severity(LogSeverity::Warning);
    let touch_targets = Tokens::get(ui.ctx()).metrics.ctl_h >= 44.0;
    let header_height = if touch_targets {
        CONSOLE_TOUCH_HEADER_HEIGHT
    } else {
        CONSOLE_HEADER_HEIGHT
    };
    let header = ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), header_height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let action_width = console_trailing_actions_width(
                layout,
                app.state.workbench.console_page,
                touch_targets,
            );
            egui::ScrollArea::horizontal()
                .id_salt("workbench.console.tabs.scroll")
                .max_width((ui.available_width() - action_width).max(header_height))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| console_tabs(ui, app, problems, header_height));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let control_size = if touch_targets {
                    CONSOLE_TOUCH_HEADER_HEIGHT
                } else {
                    CONSOLE_ACTION_SIZE
                };
                ui.add_space(CONSOLE_ACTION_MARGIN_RIGHT);
                if console_action(
                    ui,
                    ConsoleActionIcon::Workbench(WorkbenchIcon::ChevronDown),
                    if layout.show_console_body {
                        "Collapse console"
                    } else {
                        "Expand console"
                    },
                    false,
                    control_size,
                )
                .clicked()
                {
                    if layout.show_console_body {
                        app.state.workbench.console_maximized = false;
                    }
                    Command::ToggleConsole.execute(app);
                }
                if !layout.compact_shell {
                    if !touch_targets {
                        ui.add_space(CONSOLE_ACTION_MARGIN_RIGHT);
                        if console_action(
                            ui,
                            ConsoleActionIcon::Workbench(WorkbenchIcon::Focus),
                            if app.state.workbench.console_maximized {
                                "Restore console"
                            } else {
                                "Maximize console"
                            },
                            app.state.workbench.console_maximized,
                            control_size,
                        )
                        .clicked()
                        {
                            Command::ToggleConsoleMaximized.execute(app);
                        }
                    }
                    let page = app.state.workbench.console_page;
                    let clear = console_clear_action(
                        page,
                        match page {
                            ConsolePage::Console => !app.state.log_buffer.is_empty(),
                            ConsolePage::Interactive => {
                                !app.state.script_console.history.is_empty()
                            }
                            ConsolePage::Problems
                            | ConsolePage::Measurements
                            | ConsolePage::TaskLog => false,
                        },
                    );
                    ui.add_space(CONSOLE_ACTION_MARGIN_RIGHT);
                    let response = ui
                        .add_enabled_ui(clear.enabled, |ui| {
                            console_action(
                                ui,
                                ConsoleActionIcon::Clear,
                                clear.label,
                                false,
                                control_size,
                            )
                        })
                        .inner;
                    if response.clicked() {
                        match page {
                            ConsolePage::Console => app.state.clear_primary_log(),
                            ConsolePage::Interactive => {
                                app.state.script_console.history.clear();
                            }
                            ConsolePage::Problems
                            | ConsolePage::Measurements
                            | ConsolePage::TaskLog => {}
                        }
                    }
                }
                if !layout.compact_shell {
                    console_context(ui, app);
                }
            });
        },
    );
    if layout.show_console_body {
        ui.painter().hline(
            header.response.rect.x_range(),
            header.response.rect.bottom(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
}

fn console_trailing_actions_width(
    layout: LayoutSpec,
    _page: ConsolePage,
    touch_targets: bool,
) -> f32 {
    let control_size = if touch_targets {
        CONSOLE_TOUCH_HEADER_HEIGHT
    } else {
        CONSOLE_ACTION_SIZE
    };
    let mut actions = 1_usize;
    if !layout.compact_shell {
        actions += usize::from(!touch_targets);
        // Clear remains present for every page so switching tabs never moves
        // Maximize or Collapse. Pages backed by immutable/derived data expose
        // the same affordance disabled with a precise explanation.
        actions += 1;
    }
    actions as f32 * (control_size + CONSOLE_ACTION_MARGIN_RIGHT)
}

fn console_tabs(ui: &mut Ui, app: &mut RSpiceApp, problems: usize, height: f32) {
    let mut requested = None;
    let mut focus = None;
    let mut ids = Vec::with_capacity(ConsolePage::ALL.len());
    let tabs = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        for (index, page) in ConsolePage::ALL.into_iter().enumerate() {
            let count = match page {
                ConsolePage::Interactive => app.state.script_console.history.len(),
                ConsolePage::Problems => problems,
                ConsolePage::Measurements => active_measurement_count(&app.state.simulation),
                ConsolePage::TaskLog => app.state.simulation.runs.len(),
                ConsolePage::Console => 0,
            };
            let count_tone = match page {
                ConsolePage::Problems => ConsoleCountTone::Warning,
                ConsolePage::TaskLog => ConsoleCountTone::Success,
                ConsolePage::Console | ConsolePage::Interactive | ConsolePage::Measurements => {
                    ConsoleCountTone::Neutral
                }
            };
            let response = console_tab(
                ui,
                page,
                app.state.workbench.console_page == page,
                count,
                count_tone,
                height,
            );
            ids.push(response.id);
            if response.clicked() {
                requested = Some(page);
                focus = Some(index);
            }
            if response.has_focus() {
                let key = ui.input_mut(|input| {
                    if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                        Some(-1_i32)
                    } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                        Some(1_i32)
                    } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                        Some(i32::MIN)
                    } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                        Some(i32::MAX)
                    } else {
                        None
                    }
                });
                focus = key.map(|direction| {
                    if direction == i32::MIN {
                        0
                    } else if direction == i32::MAX {
                        ConsolePage::ALL.len() - 1
                    } else {
                        (index as i32 + direction).rem_euclid(ConsolePage::ALL.len() as i32)
                            as usize
                    }
                });
            }
        }
    });
    ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabList);
        node.set_label("Console pages");
    });
    if let Some(index) = focus {
        requested = Some(ConsolePage::ALL[index]);
        ui.memory_mut(|memory| memory.request_focus(ids[index]));
    }
    if let Some(page) = requested {
        app.state.workbench.console_page = page;
    }
}

#[derive(Clone, Copy)]
enum ConsoleCountTone {
    Neutral,
    Warning,
    Success,
}

fn console_tab(
    ui: &mut Ui,
    page: ConsolePage,
    selected: bool,
    count: usize,
    count_tone: ConsoleCountTone,
    height: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(CONSOLE_FONT_SIZE, FontWeight::Regular);
    let galley = ui
        .painter()
        .layout_no_wrap(page.label().to_owned(), font, t.color.text_dim);
    let count_width = if count > 0 {
        let digits = count.to_string().len() as f32;
        (digits * 6.5 + 8.0).max(15.0) + 4.0
    } else {
        0.0
    };
    let width = 20.0 + galley.size().x + count_width;
    let mut sense = Sense::click();
    if !selected {
        sense = sense.difference(Sense::focusable_noninteractive());
    }
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            page.label(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
        if count > 0 {
            node.set_label(format!("{}, {count}", page.label()));
        }
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let text_color = if selected {
        t.color.text
    } else {
        t.color.text_dim
    };
    ui.painter().galley(
        egui::pos2(rect.left() + 10.0, rect.center().y - galley.size().y * 0.5),
        galley,
        text_color,
    );
    if count > 0 {
        let digits = count.to_string();
        let badge_width = (digits.len() as f32 * 6.5 + 8.0).max(15.0);
        let badge = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 10.0 - badge_width * 0.5, rect.center().y),
            Vec2::new(badge_width, 15.0),
        );
        let (ink, fill) = match count_tone {
            ConsoleCountTone::Warning => (
                t.color.warn,
                theme::mix(t.color.bg_panel, t.color.warn, 0.11),
            ),
            ConsoleCountTone::Success => {
                (t.color.ok, theme::mix(t.color.bg_panel, t.color.ok, 0.11))
            }
            ConsoleCountTone::Neutral => (t.color.text_dim, t.color.bg_active),
        };
        ui.painter().rect_filled(badge, 7.0, fill);
        ui.painter().text(
            badge.center(),
            egui::Align2::CENTER_CENTER,
            digits,
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::Medium),
            ink,
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(rect.left() + 8.0, rect.bottom() - 2.0),
                egui::pos2(rect.right() - 8.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

enum ConsoleActionIcon {
    Clear,
    Workbench(WorkbenchIcon),
}

fn console_action(
    ui: &mut Ui,
    icon: ConsoleActionIcon,
    label: &str,
    selected: bool,
    size: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), selected, label)
    });
    if selected {
        ui.painter().rect_filled(rect, t.radius, t.color.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let icon_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(16.0));
    let icon_color = if ui.is_enabled() {
        t.color.text_dim
    } else {
        t.color.text_faint
    };
    match icon {
        ConsoleActionIcon::Workbench(icon) => {
            icon.paint(ui.painter(), icon_rect, icon_color);
        }
        ConsoleActionIcon::Clear => paint_trash(ui.painter(), icon_rect, icon_color),
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(label)
}

fn paint_trash(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let stroke = egui::Stroke::new(1.2, color);
    let body = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 4.0, rect.top() + 5.0),
        egui::pos2(rect.right() - 4.0, rect.bottom() - 2.0),
    );
    painter.rect_stroke(body, 1.0, stroke, egui::StrokeKind::Inside);
    painter.hline(
        egui::Rangef::new(rect.left() + 2.0, rect.right() - 2.0),
        rect.top() + 4.0,
        stroke,
    );
    painter.hline(
        egui::Rangef::new(rect.left() + 6.0, rect.right() - 6.0),
        rect.top() + 2.0,
        stroke,
    );
}

fn console_context(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let text = if app.state.simulation.is_running {
        format!(
            "Active job · {}% · execution in progress",
            simulation_progress_percent(app.state.simulation.progress)
        )
    } else if let Some(index) = app.state.simulation.active_run_idx {
        app.state.simulation.runs.get(index).map_or_else(
            || "No retained result selected".to_owned(),
            |run| {
                format!(
                    "Latest result · {} · completed in {:.2} s",
                    run.label, run.elapsed_time
                )
            },
        )
    } else {
        "No retained result selected".to_owned()
    };
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
                .color(t.color.text_faint),
        )
        .truncate(),
    );
}

fn console(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.console.body")
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for entry in app.state.log_buffer.entries() {
                log_row(ui, entry);
            }
            if app.state.log_buffer.is_empty() {
                muted(ui, "Engine messages will appear here.");
            }
        });
}

fn interactive(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut submit = false;
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = INTERACTIVE_INPUT_GAP;
            let command_ready = !app.state.script_console.input_buffer.trim().is_empty();
            let run = ui
                .add_enabled(command_ready, egui::Button::new("Run"))
                .on_hover_text("Execute command");
            run.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    "Execute interactive command",
                )
            });
            submit |= run.clicked();

            let input = ui.add_sized(
                [ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h],
                egui::TextEdit::singleline(&mut app.state.script_console.input_buffer)
                    .hint_text("Typed project expression or command…"),
            );
            input.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::TextEdit,
                    ui.is_enabled(),
                    "Interactive command",
                )
            });
            submit |= input.has_focus()
                && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
        });
        ui.add_space(INTERACTIVE_INPUT_GAP);
        ScrollArea::vertical()
            .id_salt("workbench.interactive.history")
            .stick_to_bottom(true)
            .show(ui, |ui| {
                if app.state.script_console.history.is_empty() {
                    muted(
                        ui,
                        "Queries: project.runs · plan.analyses.enabled · project.results[\"Run 1\"].measure(\"gain\") · help",
                    );
                }
                for item in &app.state.script_console.history {
                    automation_row(ui, item);
                }
            });
    });
    if submit {
        submit_interactive_command(app);
    }
}

fn submit_interactive_command(app: &mut RSpiceApp) -> bool {
    let command = app.state.script_console.input_buffer.trim().to_owned();
    if command.is_empty() {
        return false;
    }
    app.state.script_console.input_buffer.clear();
    let output = execute_interactive_command(&command, app);
    app.state
        .script_console
        .history
        .push(ConsoleHistoryItem { command, output });
    true
}

fn execute_interactive_command(input: &str, app: &mut RSpiceApp) -> CommandOutput {
    let input = input.trim();
    if input.eq_ignore_ascii_case("help") {
        return CommandOutput {
            success: true,
            message: "Read project and retained-result state with typed expressions, or dispatch a governed command by stable ID.".to_owned(),
            data: Some(
                "Queries: project.runs · project.revision · project.specifications · plan.analyses.enabled · project.results[\"Run 1\"].measure(\"gain\")\nCommands: commands [filter] · command <stable-id>".to_owned(),
            ),
        };
    }
    if let Some(output) = execute_project_query(input, app) {
        return output;
    }

    if let Some(filter) = input
        .strip_prefix("commands")
        .filter(|suffix| suffix.is_empty() || suffix.starts_with(char::is_whitespace))
    {
        let filter = filter.trim().to_ascii_lowercase();
        let rows = command_catalog()
            .filter(|command| {
                filter.is_empty()
                    || command.stable_id().to_ascii_lowercase().contains(&filter)
                    || command.spec().label.to_ascii_lowercase().contains(&filter)
            })
            .map(|command| {
                let availability = match command.availability(app) {
                    CommandAvailability::Available => "available",
                    CommandAvailability::Disabled(_) => "disabled",
                    CommandAvailability::Hidden => "hidden",
                };
                format!(
                    "{} · {} · {availability}",
                    command.stable_id(),
                    command.spec().label.trim_end_matches('…')
                )
            })
            .collect::<Vec<_>>();
        return if rows.is_empty() {
            CommandOutput {
                success: false,
                message: if filter.is_empty() {
                    "The canonical command catalog is empty.".to_owned()
                } else {
                    format!("No command matches `{filter}`.")
                },
                data: None,
            }
        } else {
            CommandOutput {
                success: true,
                message: format!(
                    "{} governed command{}",
                    rows.len(),
                    if rows.len() == 1 { "" } else { "s" }
                ),
                data: Some(rows.join("\n")),
            }
        };
    }

    let Some(stable_id) = input.strip_prefix("command ").map(str::trim) else {
        return CommandOutput {
            success: false,
            message:
                "Unknown expression. Use `help` for project queries and governed command syntax."
                    .to_owned(),
            data: None,
        };
    };
    if stable_id.is_empty() || stable_id.chars().any(char::is_whitespace) {
        return CommandOutput {
            success: false,
            message: "A command invocation requires exactly one canonical stable ID.".to_owned(),
            data: None,
        };
    }
    let Some(command) = Command::from_stable_id(stable_id) else {
        return CommandOutput {
            success: false,
            message: format!("Unknown command ID `{stable_id}`. Use `commands` to inspect IDs."),
            data: None,
        };
    };
    if !command.palette_visible() {
        return CommandOutput {
            success: false,
            message: format!("Command `{stable_id}` is private to application chrome."),
            data: None,
        };
    }
    if let Some(output) = unavailable_command_output(stable_id, command.availability(app)) {
        return output;
    }
    let label = command.spec().label.trim_end_matches('…').to_owned();
    command.execute(app);
    CommandOutput {
        success: true,
        message: format!("Dispatched `{stable_id}` · {label}"),
        data: Some(
            "The governed dispatcher accepted this command. Its owning workflow reports asynchronous completion separately.".to_owned(),
        ),
    }
}

fn execute_project_query(input: &str, app: &RSpiceApp) -> Option<CommandOutput> {
    match input {
        "project.revision" => Some(CommandOutput {
            success: true,
            message: "Current mutable project revision".to_owned(),
            data: Some(app.state.workspace.project.revision().get().to_string()),
        }),
        "project.runs" | "help(project.runs)" => {
            let runs = app
                .state
                .simulation
                .runs
                .iter()
                .map(|run| {
                    format!(
                        "Run {} · {} · {} · {} analyses · {:.3} s",
                        run.id,
                        run.label,
                        run_lifecycle_label(run.lifecycle),
                        run.analyses.len(),
                        run.elapsed_time
                    )
                })
                .collect::<Vec<_>>();
            Some(CommandOutput {
                success: true,
                message: format!(
                    "{} retained immutable run{}",
                    runs.len(),
                    if runs.len() == 1 { "" } else { "s" }
                ),
                data: Some(if runs.is_empty() {
                    "No retained runs.".to_owned()
                } else {
                    runs.join("\n")
                }),
            })
        }
        "project.specifications" => {
            let rows = active_specifications(app)
                .into_iter()
                .map(|spec| {
                    format!(
                        "{} · {}{}",
                        spec.measurement,
                        specification_text(&spec),
                        if spec.unit.trim().is_empty() {
                            String::new()
                        } else {
                            format!(" {}", spec.unit.trim())
                        }
                    )
                })
                .collect::<Vec<_>>();
            Some(CommandOutput {
                success: true,
                message: format!(
                    "{} active specification{}",
                    rows.len(),
                    if rows.len() == 1 { "" } else { "s" }
                ),
                data: Some(if rows.is_empty() {
                    "No active specifications.".to_owned()
                } else {
                    rows.join("\n")
                }),
            })
        }
        "plan.analyses.enabled" => {
            let Some(plan) = app.state.sim_setup.analysis_plan.as_ref() else {
                return Some(CommandOutput {
                    success: false,
                    message: "The active simulation plan has no stable analysis graph.".to_owned(),
                    data: None,
                });
            };
            let enabled = plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
                .map(|instance| instance.kind().stable_id())
                .collect::<Vec<_>>();
            Some(CommandOutput {
                success: true,
                message: format!(
                    "{} enabled analysis instance{} · plan revision {}",
                    enabled.len(),
                    if enabled.len() == 1 { "" } else { "s" },
                    plan.revision().get()
                ),
                data: Some(format!("[{}]", enabled.join(", "))),
            })
        }
        _ => parse_measurement_query(input)
            .map(|(run_selector, measurement)| measurement_query(app, run_selector, measurement)),
    }
}

fn parse_measurement_query(input: &str) -> Option<(&str, &str)> {
    let rest = input.strip_prefix("project.results[\"")?;
    let (run, rest) = rest.split_once("\"].measure(\"")?;
    let measurement = rest.strip_suffix("\")")?;
    (!run.trim().is_empty() && !measurement.trim().is_empty()).then_some((run, measurement))
}

fn measurement_query(app: &RSpiceApp, run_selector: &str, measurement: &str) -> CommandOutput {
    let run_number = run_selector
        .strip_prefix("Run ")
        .and_then(|value| value.parse::<u64>().ok());
    let run = app.state.simulation.runs.iter().find(|run| {
        run.label.eq_ignore_ascii_case(run_selector)
            || run_number.is_some_and(|number| run.id == number)
    });
    let Some(run) = run else {
        return CommandOutput {
            success: false,
            message: format!("No retained run matches `{run_selector}`."),
            data: None,
        };
    };
    let matches = run
        .analyses
        .iter()
        .flat_map(|analysis| {
            analysis
                .measurements
                .iter()
                .filter(|candidate| candidate.name.eq_ignore_ascii_case(measurement))
                .map(move |candidate| (analysis.label.as_str(), candidate))
        })
        .collect::<Vec<_>>();
    if matches.is_empty() {
        return CommandOutput {
            success: false,
            message: format!("Run {} has no measurement named `{measurement}`.", run.id),
            data: None,
        };
    }
    let rows = matches
        .iter()
        .map(|(analysis, measurement)| {
            let value = measurement
                .value
                .map(format_measure_value)
                .unwrap_or_else(|| "unavailable".to_owned());
            format!(
                "{} · {} · {}",
                analysis,
                value,
                if measurement.passed { "pass" } else { "fail" }
            )
        })
        .collect::<Vec<_>>();
    CommandOutput {
        success: matches
            .iter()
            .all(|(_, measurement)| measurement.value.is_some()),
        message: format!(
            "{} exact f64 result{} from immutable Run {}",
            rows.len(),
            if rows.len() == 1 { "" } else { "s" },
            run.id
        ),
        data: Some(rows.join("\n")),
    }
}

fn unavailable_command_output(
    stable_id: &str,
    availability: CommandAvailability,
) -> Option<CommandOutput> {
    match availability {
        CommandAvailability::Available => None,
        CommandAvailability::Disabled(reason) => Some(CommandOutput {
            success: false,
            message: format!("Command `{stable_id}` is unavailable: {reason}."),
            data: None,
        }),
        CommandAvailability::Hidden => Some(CommandOutput {
            success: false,
            message: format!("Command `{stable_id}` is hidden in the current context."),
            data: None,
        }),
    }
}

fn active_measurement_count(simulation: &SimulationState) -> usize {
    simulation.active_run().map_or(0, |run| {
        run.analyses
            .iter()
            .map(|analysis| analysis.measurements.len())
            .sum()
    })
}

fn problems(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.problems")
        .show(ui, |ui| {
            let mut any = false;
            if let Some(result) = &app.state.dialogs.drc_results {
                for violation in result.violations() {
                    any = true;
                    issue_row(
                        ui,
                        violation.severity.display_name(),
                        &violation.location.display(),
                        &violation.message,
                        drc_tone(violation.severity),
                    );
                }
            }
            for entry in
                app.state.log_buffer.entries().filter(|entry| {
                    matches!(entry.severity, LogSeverity::Error | LogSeverity::Warning)
                })
            {
                any = true;
                issue_row(
                    ui,
                    entry.severity.name(),
                    entry.context.as_deref().unwrap_or(entry.source.name()),
                    &entry.message,
                    log_tone(entry.severity),
                );
            }
            if !any {
                muted(ui, "No current errors or advisories.");
            }
        });
}

fn measurements(ui: &mut Ui, app: &mut RSpiceApp) {
    let specifications = active_specifications(app);
    let rows = active_measurement_rows(app, &specifications);
    let run_label = app
        .state
        .simulation
        .active_run()
        .map(|run| format!("Run {} · immutable dataset", run.id));
    let mut add_measurement = false;
    ScrollArea::vertical()
        .id_salt("workbench.measurements")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(label) = run_label.as_deref() {
                    ui.label(
                        egui::RichText::new(label)
                            .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
                            .color(Tokens::get(ui.ctx()).color.text_dim),
                    );
                } else {
                    muted(ui, "No retained result selected");
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    add_measurement = ui
                        .add_enabled(
                            app.state.simulation.active_analysis().is_some(),
                            egui::Button::new("+ Add measurement…"),
                        )
                        .on_disabled_hover_text(
                            "Select a retained result analysis before creating a measurement",
                        )
                        .clicked();
                });
            });
            ui.separator();
            if rows.is_empty() {
                if app.state.simulation.active_run().is_some() {
                    muted(ui, "This dataset has no computed measurements.");
                } else {
                    muted(
                        ui,
                        "Select a retained result dataset to inspect measurements.",
                    );
                }
                return;
            }
            Grid::new("workbench.measurements.table")
                .num_columns(7)
                .striped(true)
                .spacing(Vec2::new(16.0, 4.0))
                .show(ui, |ui| {
                    measurement_header(ui, "Measurement");
                    measurement_header(ui, "Expression");
                    measurement_header(ui, "Value");
                    measurement_header(ui, "Spec");
                    measurement_header(ui, "Margin");
                    measurement_header(ui, "Worst point");
                    measurement_header(ui, "Status");
                    ui.end_row();
                    for row in &rows {
                        measurement_cell(ui, &row.name, false);
                        measurement_cell(ui, &row.expression, true);
                        measurement_cell(ui, &row.value, true);
                        measurement_cell(ui, &row.specification, true);
                        measurement_cell_tone(ui, &row.margin, row.tone);
                        measurement_cell(ui, &row.worst_point, false);
                        measurement_cell_tone(ui, row.status, row.tone);
                        ui.end_row();
                    }
                });
        });
    if add_measurement {
        crate::workbench::documents::visualization_studio::open_measurement_editor(app);
    }
}

fn task_log(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.task_log")
        .show(ui, |ui| {
            if app.state.simulation.runs.is_empty() {
                muted(ui, "Queued, active, cancelled, and completed simulation tasks will appear here.");
            }
            for run in &app.state.simulation.runs {
                let active = app.state.simulation.active_execution
                    == run.execution_identity()
                    && !run.lifecycle.is_terminal();
                let (status, tone) = run_lifecycle_presentation(run.lifecycle);
                let progress = if active {
                    format!(
                        " · {}%",
                        simulation_progress_percent(app.state.simulation.progress)
                    )
                } else {
                    String::new()
                };
                let revision = run.prepared_receipt().map_or_else(
                    || "revision unavailable".to_owned(),
                    |receipt| format!("revision {}", receipt.project_revision().get()),
                );
                issue_row(
                    ui,
                    status,
                    &run.label,
                    &format!(
                        "{} analyses · {:.3} s{progress} · {revision}",
                        run.analyses.len(),
                        run.elapsed_time
                    ),
                    tone,
                );
            }
            if !app.state.simulation.is_running {
                issue_row(
                    ui,
                    "IDLE",
                    "queue",
                    "No queued execution. The next run will snapshot the active plan and project revision.",
                    SemanticTone::Info,
                );
            }
        });
}

#[derive(Debug, Clone)]
struct MeasurementTableRow {
    name: String,
    expression: String,
    value: String,
    specification: String,
    margin: String,
    worst_point: String,
    status: &'static str,
    tone: SemanticTone,
}

fn active_specifications(app: &RSpiceApp) -> Vec<crate::state::SpecEntry> {
    let active = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| app.state.workspace.active_plan_data(plan.id()))
        .map(|payload| payload.specs.as_slice())
        .unwrap_or(app.state.workspace.specs.as_slice());
    active.to_vec()
}

fn active_measurement_rows(
    app: &RSpiceApp,
    specifications: &[crate::state::SpecEntry],
) -> Vec<MeasurementTableRow> {
    let Some(run) = app.state.simulation.active_run() else {
        return Vec::new();
    };
    run.analyses
        .iter()
        .flat_map(|analysis| {
            analysis.measurements.iter().map(move |measurement| {
                let spec = specifications
                    .iter()
                    .find(|spec| spec.measurement.eq_ignore_ascii_case(&measurement.name));
                measurement_table_row(measurement, &analysis.label, spec)
            })
        })
        .collect()
}

fn measurement_table_row(
    measurement: &rspice_core::MeasureResult,
    analysis_label: &str,
    specification: Option<&crate::state::SpecEntry>,
) -> MeasurementTableRow {
    let value = measurement.value;
    let specification_text = specification.map_or_else(
        || {
            measurement.expected.map_or_else(
                || "—".to_owned(),
                |expected| {
                    measurement.tolerance.map_or_else(
                        || format!("= {}", format_measure_value(expected)),
                        |tolerance| {
                            format!(
                                "{} ± {}",
                                format_measure_value(expected),
                                format_measure_value(tolerance)
                            )
                        },
                    )
                },
            )
        },
        specification_text,
    );
    let (status, tone, margin) = if let Some(error) = measurement
        .error
        .as_deref()
        .filter(|error| !error.trim().is_empty())
    {
        ("ERROR", SemanticTone::Error, error.to_owned())
    } else if let Some(specification) = specification {
        value.map_or(
            ("NO DATA", SemanticTone::Warning, "—".to_owned()),
            |value| {
                let passing = specification.passes(value);
                let signed_margin = if passing {
                    let lower = specification.min.map(|minimum| value - minimum);
                    let upper = specification.max.map(|maximum| maximum - value);
                    lower
                        .into_iter()
                        .chain(upper)
                        .reduce(f64::min)
                        .unwrap_or(f64::INFINITY)
                } else {
                    -specification.violation(value)
                };
                (
                    if passing { "PASS" } else { "FAIL" },
                    if passing {
                        SemanticTone::Success
                    } else {
                        SemanticTone::Error
                    },
                    if signed_margin.is_finite() {
                        format_signed_measure_value(signed_margin)
                    } else {
                        "unbounded".to_owned()
                    },
                )
            },
        )
    } else if measurement.expected.is_some() {
        let margin = measurement
            .value
            .zip(measurement.expected)
            .zip(measurement.tolerance)
            .map_or_else(
                || "—".to_owned(),
                |((value, expected), tolerance)| {
                    format_signed_measure_value(tolerance - (value - expected).abs())
                },
            );
        (
            if measurement.passed { "PASS" } else { "FAIL" },
            if measurement.passed {
                SemanticTone::Success
            } else {
                SemanticTone::Error
            },
            margin,
        )
    } else {
        ("NO SPEC", SemanticTone::Info, "—".to_owned())
    };
    MeasurementTableRow {
        name: measurement.name.clone(),
        expression: "—".to_owned(),
        value: value
            .map(format_measure_value)
            .unwrap_or_else(|| "—".to_owned()),
        specification: specification_text,
        margin,
        worst_point: analysis_label.to_owned(),
        status,
        tone,
    }
}

fn specification_text(specification: &crate::state::SpecEntry) -> String {
    match (specification.min, specification.max) {
        (Some(minimum), Some(maximum)) => format!(
            "{} … {}",
            format_measure_value(minimum),
            format_measure_value(maximum)
        ),
        (Some(minimum), None) => format!("≥ {}", format_measure_value(minimum)),
        (None, Some(maximum)) => format!("≤ {}", format_measure_value(maximum)),
        (None, None) => "tracked".to_owned(),
    }
}

fn measurement_header(ui: &mut Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(CONSOLE_FONT_SIZE, FontWeight::SemiBold))
            .color(Tokens::get(ui.ctx()).color.text_dim),
    );
}

fn measurement_cell(ui: &mut Ui, text: &str, mono: bool) {
    let font = if mono {
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular)
    } else {
        theme::sans(CONSOLE_FONT_SIZE, FontWeight::Regular)
    };
    ui.label(
        egui::RichText::new(text)
            .font(font)
            .color(Tokens::get(ui.ctx()).color.text),
    );
}

fn measurement_cell_tone(ui: &mut Ui, text: &str, tone: SemanticTone) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
            .color(tone_color(&tokens, tone)),
    );
}

fn run_lifecycle_label(lifecycle: crate::state::SimulationRunLifecycle) -> &'static str {
    use crate::state::SimulationRunLifecycle as Lifecycle;
    match lifecycle {
        Lifecycle::LegacyUnknown => "legacy status unknown",
        Lifecycle::Preparing => "queued",
        Lifecycle::Running => "running",
        Lifecycle::Cancelling => "cancelling",
        Lifecycle::Completed => "completed",
        Lifecycle::Failed => "failed",
        Lifecycle::Aborted => "cancelled",
        Lifecycle::Interrupted => "interrupted",
    }
}

fn run_lifecycle_presentation(
    lifecycle: crate::state::SimulationRunLifecycle,
) -> (&'static str, SemanticTone) {
    use crate::state::SimulationRunLifecycle as Lifecycle;
    match lifecycle {
        Lifecycle::LegacyUnknown => ("UNKNOWN", SemanticTone::Warning),
        Lifecycle::Preparing => ("QUEUED", SemanticTone::Info),
        Lifecycle::Running => ("RUNNING", SemanticTone::Info),
        Lifecycle::Cancelling => ("CANCELLING", SemanticTone::Warning),
        Lifecycle::Completed => ("DONE", SemanticTone::Success),
        Lifecycle::Failed => ("FAIL", SemanticTone::Error),
        Lifecycle::Aborted => ("CANCELLED", SemanticTone::Warning),
        Lifecycle::Interrupted => ("INTERRUPTED", SemanticTone::Warning),
    }
}

fn log_row(ui: &mut Ui, entry: &crate::diagnostics::LogEntry) {
    let t = Tokens::get(ui.ctx());
    let tone = log_tone(entry.severity);
    let source_color = tone_color(&t, tone);
    let message_color = match tone {
        SemanticTone::Error | SemanticTone::Warning | SemanticTone::Success => source_color,
        SemanticTone::Info | SemanticTone::Debug => t.color.text_dim,
        SemanticTone::Trace => t.color.text_faint,
    };
    let message = entry.context.as_ref().map_or_else(
        || entry.message.clone(),
        |context| format!("{} · {context}", entry.message),
    );
    row(
        ui,
        &entry.format_timestamp(),
        entry.source.name(),
        &message,
        source_color,
        message_color,
    );
}

fn automation_row(ui: &mut Ui, item: &ConsoleHistoryItem) {
    let t = Tokens::get(ui.ctx());
    row(
        ui,
        ">",
        "AUTO",
        &item.command,
        t.color.info,
        t.color.text_dim,
    );
    for line in item.output.message.lines() {
        row(
            ui,
            "",
            "",
            line,
            if item.output.success {
                t.color.ok
            } else {
                t.color.err
            },
            if item.output.success {
                t.color.ok
            } else {
                t.color.err
            },
        );
    }
    if let Some(data) = item.output.data.as_deref() {
        for line in data.lines() {
            row(ui, "", "DATA", line, t.color.info, t.color.text_dim);
        }
    }
}

fn issue_row(ui: &mut Ui, status: &str, source: &str, message: &str, tone: SemanticTone) {
    let t = Tokens::get(ui.ctx());
    let color = tone_color(&t, tone);
    row(ui, status, source, message, color, color);
}

/// Semantic row tones are kept independent of the active palette so severity
/// cannot accidentally collapse into a success/error boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SemanticTone {
    Error,
    Warning,
    Success,
    Info,
    Debug,
    Trace,
}

fn tone_color(tokens: &Tokens, tone: SemanticTone) -> egui::Color32 {
    match tone {
        SemanticTone::Error => tokens.color.err,
        SemanticTone::Warning => tokens.color.warn,
        SemanticTone::Success => tokens.color.ok,
        SemanticTone::Info => tokens.color.info,
        SemanticTone::Debug => tokens.color.text_dim,
        SemanticTone::Trace => tokens.color.text_faint,
    }
}

fn log_tone(severity: LogSeverity) -> SemanticTone {
    match severity {
        LogSeverity::Error => SemanticTone::Error,
        LogSeverity::Warning => SemanticTone::Warning,
        LogSeverity::Info => SemanticTone::Info,
        LogSeverity::Debug => SemanticTone::Debug,
        LogSeverity::Trace => SemanticTone::Trace,
    }
}

fn drc_tone(severity: crate::services::drc::DrcSeverity) -> SemanticTone {
    match severity {
        crate::services::drc::DrcSeverity::Critical | crate::services::drc::DrcSeverity::Error => {
            SemanticTone::Error
        }
        crate::services::drc::DrcSeverity::Warning => SemanticTone::Warning,
        crate::services::drc::DrcSeverity::Info => SemanticTone::Info,
    }
}

fn format_measure_value(value: f64) -> String {
    fmt_si(value, "", 6).trim().to_owned()
}

fn format_signed_measure_value(value: f64) -> String {
    let value = format_measure_value(value);
    if value.starts_with('-') {
        value
    } else {
        format!("+{value}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConsoleClearAction {
    enabled: bool,
    label: &'static str,
}

fn console_clear_action(page: ConsolePage, console_has_output: bool) -> ConsoleClearAction {
    match page {
        ConsolePage::Console => ConsoleClearAction {
            enabled: console_has_output,
            label: if console_has_output {
                "Clear console output"
            } else {
                "Console output is already empty"
            },
        },
        ConsolePage::Interactive => ConsoleClearAction {
            enabled: console_has_output,
            label: if console_has_output {
                "Clear interactive command history"
            } else {
                "Interactive command history is already empty"
            },
        },
        ConsolePage::Problems => ConsoleClearAction {
            enabled: false,
            label: "Problems reflect current diagnostics and cannot be cleared",
        },
        ConsolePage::Measurements => ConsoleClearAction {
            enabled: false,
            label: "Measurements belong to the selected immutable dataset",
        },
        ConsolePage::TaskLog => ConsoleClearAction {
            enabled: false,
            label: "Task history is retained and cannot be cleared here",
        },
    }
}

fn row(
    ui: &mut Ui,
    time: &str,
    source: &str,
    message: &str,
    source_color: egui::Color32,
    message_color: egui::Color32,
) {
    let t = Tokens::get(ui.ctx());
    let message_x_offset =
        CONSOLE_TIME_WIDTH + CONSOLE_COLUMN_GAP + CONSOLE_SOURCE_WIDTH + CONSOLE_COLUMN_GAP;
    let message_width = (ui.available_width() - message_x_offset).max(1.0);
    let message_galley = ui.painter().layout(
        message.to_owned(),
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        message_color,
        message_width,
    );
    let row_height = message_galley.size().y.max(CONSOLE_ROW_MIN_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    let time_x = rect.left();
    let source_x = time_x + CONSOLE_TIME_WIDTH + CONSOLE_COLUMN_GAP;
    let message_x = source_x + CONSOLE_SOURCE_WIDTH + CONSOLE_COLUMN_GAP;
    let time_clip = egui::Rect::from_min_max(
        egui::pos2(time_x, rect.top()),
        egui::pos2(time_x + CONSOLE_TIME_WIDTH, rect.bottom()),
    );
    ui.painter().with_clip_rect(time_clip).text(
        egui::pos2(time_x, rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5),
        egui::Align2::LEFT_CENTER,
        time,
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        t.color.text_faint,
    );
    let source_clip = egui::Rect::from_min_max(
        egui::pos2(source_x, rect.top()),
        egui::pos2(source_x + CONSOLE_SOURCE_WIDTH, rect.bottom()),
    );
    ui.painter().with_clip_rect(source_clip).text(
        egui::pos2(source_x, rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5),
        egui::Align2::LEFT_CENTER,
        source,
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Medium),
        source_color,
    );
    let message_clip = egui::Rect::from_min_max(
        egui::pos2(message_x, rect.top()),
        egui::pos2(rect.right(), rect.bottom()),
    );
    let message_y = if message_galley.size().y <= CONSOLE_ROW_MIN_HEIGHT {
        rect.center().y - message_galley.size().y * 0.5
    } else {
        rect.top()
    };
    ui.painter().with_clip_rect(message_clip).galley(
        egui::pos2(message_x, message_y),
        message_galley,
        message_color,
    );
}

fn simulation_progress_percent(progress: f64) -> u8 {
    (progress.clamp(0.0, 1.0) * 100.0).round() as u8
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: EMPTY_HINT_PADDING_X,
            right: EMPTY_HINT_PADDING_X,
            top: EMPTY_HINT_PADDING_Y,
            bottom: EMPTY_HINT_PADDING_Y,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
                        .color(t.color.text_faint),
                )
                .wrap()
                .halign(Align::Center),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::LogSource;
    use crate::services::drc::{DrcResult, DrcSeverity};
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    use crate::workbench::AppState;
    use crate::workbench::state::WorkbenchState;

    #[test]
    fn console_empty_hints_match_mockup_spacing_and_type_scale() {
        assert_eq!(EMPTY_HINT_PADDING_X, 12);
        assert_eq!(EMPTY_HINT_PADDING_Y, 20);
        assert_eq!(CONSOLE_FONT_SIZE, 12.0);
        assert_eq!(CONSOLE_FONT_SIZE, tokens::FS_1);
    }

    #[test]
    fn severity_tones_preserve_warning_info_and_diagnostic_meaning() {
        assert_eq!(log_tone(LogSeverity::Error), SemanticTone::Error);
        assert_eq!(log_tone(LogSeverity::Warning), SemanticTone::Warning);
        assert_eq!(log_tone(LogSeverity::Info), SemanticTone::Info);
        assert_eq!(log_tone(LogSeverity::Debug), SemanticTone::Debug);
        assert_eq!(log_tone(LogSeverity::Trace), SemanticTone::Trace);

        assert_eq!(drc_tone(DrcSeverity::Critical), SemanticTone::Error);
        assert_eq!(drc_tone(DrcSeverity::Error), SemanticTone::Error);
        assert_eq!(drc_tone(DrcSeverity::Warning), SemanticTone::Warning);
        assert_eq!(drc_tone(DrcSeverity::Info), SemanticTone::Info);
    }

    #[test]
    fn clear_affordance_is_truthful_for_every_console_page() {
        let console = console_clear_action(ConsolePage::Console, true);
        assert!(console.enabled);
        assert_eq!(console.label, "Clear console output");
        assert!(!console_clear_action(ConsolePage::Console, false).enabled);

        let interactive = console_clear_action(ConsolePage::Interactive, true);
        assert!(interactive.enabled);
        assert_eq!(interactive.label, "Clear interactive command history");
        assert!(!console_clear_action(ConsolePage::Interactive, false).enabled);

        for page in [
            ConsolePage::Problems,
            ConsolePage::Measurements,
            ConsolePage::TaskLog,
        ] {
            let action = console_clear_action(page, true);
            assert!(!action.enabled);
            assert_ne!(action.label, "Clear console output");
        }
    }

    #[test]
    fn clearing_console_preserves_diagnostics_measurements_and_run_history() {
        let mut state = AppState::default();
        state
            .log_buffer
            .warning(LogSource::Simulation, "visible console warning");
        state.script_console.input_buffer = "pending command".to_owned();
        state.script_console.history.push(ConsoleHistoryItem {
            command: "help".to_owned(),
            output: Default::default(),
        });
        state.dialogs.drc_results = Some(DrcResult::new());

        let measurement = rspice_core::MeasureResult::success("gain", 42.0);
        let analysis =
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![measurement]);
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs.push(run);

        state.clear_primary_log();
        state.script_console.history.clear();

        assert!(state.log_buffer.is_empty());
        assert!(state.script_console.history.is_empty());
        assert_eq!(state.script_console.input_buffer, "pending command");
        assert!(state.dialogs.drc_results.is_some());
        assert_eq!(state.simulation.runs.len(), 1);
        assert_eq!(state.simulation.runs[0].analyses[0].measurements.len(), 1);
    }

    #[test]
    fn interactive_console_uses_the_governed_command_dispatcher() {
        let mut app = RSpiceApp::test_instance();
        let navigator_was_visible = app.state.workbench.navigator_visible;
        app.state.script_console.input_buffer = "command toggle-navigator".to_owned();

        assert!(submit_interactive_command(&mut app));
        assert!(app.state.script_console.input_buffer.is_empty());
        assert_eq!(app.state.script_console.history.len(), 1);
        assert_eq!(
            app.state.script_console.history[0].command,
            "command toggle-navigator"
        );
        assert!(app.state.script_console.history[0].output.success);
        assert_ne!(
            app.state.workbench.navigator_visible, navigator_was_visible,
            "the typed command must execute through the real workbench dispatcher"
        );

        app.state.script_console.input_buffer = "   ".to_owned();
        assert!(!submit_interactive_command(&mut app));
        assert_eq!(app.state.script_console.history.len(), 1);
    }

    #[test]
    fn interactive_console_rejects_unknown_private_and_unavailable_commands() {
        let mut app = RSpiceApp::test_instance();

        let unknown = execute_interactive_command("command no-such-command", &mut app);
        assert!(!unknown.success);
        assert!(unknown.message.contains("Unknown command ID"));

        let fuzzy = execute_interactive_command("command Toggle Navigator", &mut app);
        assert!(!fuzzy.success);
        assert!(
            fuzzy
                .message
                .contains("requires exactly one canonical stable ID")
        );

        let private = execute_interactive_command("command console-clear", &mut app);
        assert!(!private.success);
        assert!(private.message.contains("private to application chrome"));

        let unavailable = execute_interactive_command("command stop-run", &mut app);
        assert!(!unavailable.success);
        assert!(unavailable.message.contains("unavailable"));

        let catalog = execute_interactive_command("commands navigator", &mut app);
        assert!(catalog.success);
        assert!(
            catalog
                .data
                .as_deref()
                .is_some_and(|data| data.contains("toggle-navigator"))
        );

        let hidden =
            unavailable_command_output("future-context-command", CommandAvailability::Hidden)
                .expect("hidden commands are rejected");
        assert!(!hidden.success);
        assert!(hidden.message.contains("hidden in the current context"));
    }

    #[test]
    fn interactive_console_reads_exact_retained_measurement_and_plan_state() {
        let mut app = RSpiceApp::test_instance();
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain_dc", 42.0)]);
        let mut run = SimulationRun::new(7);
        run.add_analysis(analysis);
        app.state.simulation.runs.push(run);

        let measurement = execute_interactive_command(
            "project.results[\"Run 7\"].measure(\"gain_dc\")",
            &mut app,
        );
        assert!(measurement.success);
        assert!(measurement.message.contains("immutable Run 7"));
        assert!(
            measurement
                .data
                .as_deref()
                .is_some_and(|data| data.contains("42.000000"))
        );

        let plan = execute_interactive_command("plan.analyses.enabled", &mut app);
        assert!(plan.success);
        assert!(
            plan.data
                .as_deref()
                .is_some_and(|data| data.contains("tran"))
        );
    }

    #[test]
    fn measurement_table_uses_project_specification_for_verdict_and_margin() {
        let measurement = rspice_core::MeasureResult::success("gain", 9.5);
        let spec = crate::state::SpecEntry {
            measurement: "GAIN".to_owned(),
            min: Some(9.0),
            max: Some(10.0),
            unit: "dB".to_owned(),
        };
        let row = measurement_table_row(&measurement, "AC", Some(&spec));
        assert_eq!(row.status, "PASS");
        assert_eq!(row.tone, SemanticTone::Success);
        assert_eq!(row.specification, "9.000000 … 10.000000");
        assert_eq!(row.margin, "+500.000000 m");
        assert_eq!(row.worst_point, "AC");

        let failed = rspice_core::MeasureResult::success("gain", 10.25);
        let row = measurement_table_row(&failed, "AC", Some(&spec));
        assert_eq!(row.status, "FAIL");
        assert_eq!(row.tone, SemanticTone::Error);
        assert_eq!(row.margin, "-250.000000 m");
    }

    #[test]
    fn task_lifecycle_copy_distinguishes_cancelled_failed_and_interrupted() {
        assert_eq!(
            run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Aborted),
            ("CANCELLED", SemanticTone::Warning)
        );
        assert_eq!(
            run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Failed),
            ("FAIL", SemanticTone::Error)
        );
        assert_eq!(
            run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Interrupted),
            ("INTERRUPTED", SemanticTone::Warning)
        );
    }

    #[test]
    fn measurement_badge_counts_only_the_active_immutable_dataset() {
        let mut simulation = SimulationState::default();
        let first = AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![
            rspice_core::MeasureResult::success("gain", 42.0),
            rspice_core::MeasureResult::success("phase", 60.0),
        ]);
        let second = AnalysisResult::new(2, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![rspice_core::MeasureResult::success("rise", 1.0e-9)]);
        let mut retained = SimulationRun::new(1);
        retained.add_analysis(first);
        retained.add_analysis(second);
        simulation.runs.push(retained);

        assert_eq!(active_measurement_count(&simulation), 0);
        simulation.active_run_idx = Some(0);
        assert_eq!(active_measurement_count(&simulation), 3);
        simulation.active_run_idx = Some(9);
        assert_eq!(active_measurement_count(&simulation), 0);
    }

    #[test]
    fn fractional_engine_progress_is_rendered_as_a_percentage() {
        assert_eq!(simulation_progress_percent(0.0), 0);
        assert_eq!(simulation_progress_percent(0.375), 38);
        assert_eq!(simulation_progress_percent(1.0), 100);
    }

    #[test]
    fn console_chrome_and_grid_geometry_match_the_mockup() {
        assert_eq!(CONSOLE_HEADER_HEIGHT, 31.0);
        assert_eq!(CONSOLE_TOUCH_HEADER_HEIGHT, 44.0);
        assert_eq!(CONSOLE_ACTION_SIZE, 27.0);
        assert_eq!(CONSOLE_ACTION_MARGIN_RIGHT, 3.0);
        assert_eq!(CONSOLE_BODY_PADDING_TOP, 7.0);
        assert_eq!(CONSOLE_BODY_PADDING_BOTTOM, 7.0);
        assert_eq!(CONSOLE_BODY_PADDING_X, 10.0);
        assert_eq!(CONSOLE_TIME_WIDTH, 58.0);
        assert_eq!(CONSOLE_SOURCE_WIDTH, 62.0);
        assert_eq!(CONSOLE_COLUMN_GAP, 9.0);
        assert_eq!(CONSOLE_ROW_MIN_HEIGHT, 16.0);
        assert_eq!(CONSOLE_FONT_SIZE, 12.0);
        assert_eq!(CONSOLE_FONT_SIZE, tokens::FS_1);
    }

    #[test]
    fn console_tab_lane_reserves_every_visible_trailing_action() {
        let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
        for page in ConsolePage::ALL {
            assert_eq!(console_trailing_actions_width(desktop, page, false), 90.0);
        }

        let phone =
            LayoutSpec::resolve_with_pointer(390.0, 844.0, true, &WorkbenchState::default());
        for page in ConsolePage::ALL {
            assert_eq!(console_trailing_actions_width(phone, page, true), 47.0);
        }

        let tablet =
            LayoutSpec::resolve_with_pointer(1_024.0, 768.0, true, &WorkbenchState::default());
        for page in ConsolePage::ALL {
            assert_eq!(console_trailing_actions_width(tablet, page, true), 94.0);
        }
    }
}

//! Unified console, problems, measurements, and task history.

use egui::{Align, Grid, Layout, ScrollArea, Sense, Ui, Vec2};

use crate::diagnostics::LogSeverity;
use crate::simulation::automation::CommandOutput;
use crate::state::SimulationState;
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::session::script_console::ConsoleHistoryItem;
use crate::workbench::{AppState, RSpiceApp};

use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::ConsolePage;
use crate::workbench::commands::CommandAvailability;
use crate::workbench::commands::vocabulary::{Command, command_catalog};

const CONSOLE_HEADER_HEIGHT: f32 = 31.0;
const CONSOLE_TAB_HEIGHT: f32 = 26.0;
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
            ConsolePage::Console => console(ui, &mut app.state),
            ConsolePage::Interactive => interactive(ui, app),
            ConsolePage::Problems => problems(ui, &mut app.state),
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
    let problems = active_problem_count(&app.state);
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
                .show(ui, |ui| {
                    console_tabs(
                        ui,
                        app,
                        problems,
                        if touch_targets {
                            header_height
                        } else {
                            CONSOLE_TAB_HEIGHT
                        },
                    );
                });
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
                    // The command decides what the visible page has to clear
                    // and does the clearing; this control contributes only the
                    // wording for the page in front of the user.
                    let clear = console_clear_action(
                        app.state.workbench.console_page,
                        Command::ClearConsole.is_enabled(app),
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
                        Command::ClearConsole.execute(app);
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
    let text = if app.state.simulation.has_active_execution() {
        let activity = if app.state.simulation.cancellation_is_pending() {
            "cancellation in progress"
        } else {
            "execution in progress"
        };
        format!(
            "Active job · {}% · {activity}",
            simulation_progress_percent(app.state.simulation.progress),
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

fn console(ui: &mut Ui, state: &mut AppState) {
    let filter = state.workbench.console_producer_filter.clone();
    let scroll_to_newest = state
        .workbench
        .console_producer_filter
        .as_mut()
        .is_some_and(|filter| std::mem::take(&mut filter.scroll_to_newest));
    if let Some(filter) = filter.as_ref() {
        let matched = state
            .log_buffer
            .entries()
            .filter(|entry| filter.matches(entry))
            .count();
        if producer_filter_strip(ui, filter, matched, state.log_buffer.len()) {
            state.workbench.console_producer_filter = None;
            return;
        }
    }
    let requested = ScrollArea::vertical()
        .id_salt("workbench.console.body")
        .stick_to_bottom(true)
        .show(ui, |ui| {
            console_rows(ui, state, filter.as_ref(), scroll_to_newest)
        })
        .inner;
    if let Some(anchor) = requested {
        state.jump_to_log_anchor(anchor);
    }
}

/// Simulation entries carry no producer identity yet, so a producer whose
/// entries never name its own quantity has none this filter can find. That is
/// a fact about the log rather than about the run, and the empty state says
/// which — an unexplained empty console would read as a session that never ran.
const UNTAGGED_LOG_HINT: &str =
    "Simulation entries are not yet tagged with the producer that emitted them.";

fn console_rows(
    ui: &mut Ui,
    state: &AppState,
    filter: Option<&crate::workbench::state::ConsoleProducerFilter>,
    scroll_to_newest: bool,
) -> Option<crate::diagnostics::LogAnchor> {
    let mut any = false;
    let mut requested = None;
    for entry in state.log_buffer.entries() {
        if filter.is_some_and(|filter| !filter.matches(entry)) {
            continue;
        }
        any = true;
        // Last click wins, and there is at most one per frame: a pointer
        // press lands on exactly one row.
        requested = log_row(ui, state, entry).or(requested);
    }
    if scroll_to_newest {
        // The reader asked to be shown this producer's newest entry. The
        // request rides the cursor rather than a raw scroll offset so it lands
        // on the end of the *filtered* content, whose height this frame is the
        // first to know.
        ui.scroll_to_cursor(Some(Align::BOTTOM));
    }
    if any {
        return requested;
    }
    match filter {
        Some(filter) => muted(
            ui,
            &format!(
                "No console entry names {}. {UNTAGGED_LOG_HINT}",
                filter.label()
            ),
        ),
        None => muted(ui, "Engine messages will appear here."),
    }
    None
}

/// The strip above a narrowed console: which producer, how much of the log it
/// keeps, and the one control that puts the whole log back. Returns whether
/// the reader cleared the filter.
fn producer_filter_strip(
    ui: &mut Ui,
    filter: &crate::workbench::state::ConsoleProducerFilter,
    matched: usize,
    total: usize,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut cleared = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.label(
            egui::RichText::new(format!(
                "PRODUCER · {} · {matched} of {total} entries",
                filter.label()
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_dim),
        )
        .on_hover_text(&filter.producer);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let clear = ui.button("Show all entries");
            clear.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    "Clear the console producer filter",
                )
            });
            cleared = clear.clicked();
        });
    });
    ui.add_space(4.0);
    cleared
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

fn netlist_diagnostics_own_problems(state: &AppState) -> bool {
    state.is_netlist_first_without_schematic()
        || (state.workbench.workspace == crate::workbench::state::Workspace::Results
            && state.active_result_uses_manual_deck())
        || (state.workbench.workspace == crate::workbench::state::Workspace::Netlist
            && state.ui.code_workspace.page
                == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist)
}

fn active_problem_count(state: &AppState) -> usize {
    if netlist_diagnostics_own_problems(state) {
        return state.ui.netlist.diagnostics.summary().total();
    }
    state
        .dialogs
        .drc_results
        .as_ref()
        .map_or(0, |result| result.total_count())
        + state.log_buffer.count_by_severity(LogSeverity::Error)
        + state.log_buffer.count_by_severity(LogSeverity::Warning)
}

fn problems(ui: &mut Ui, state: &mut AppState) {
    if netlist_diagnostics_own_problems(state) {
        netlist_problems(ui, state);
        return;
    }
    ScrollArea::vertical()
        .id_salt("workbench.problems")
        .show(ui, |ui| {
            let mut any = false;
            if let Some(result) = &state.dialogs.drc_results {
                for violation in result.violations() {
                    any = true;
                    issue_row(
                        ui,
                        violation.severity.display_name(),
                        &violation.location.display(),
                        &violation.message,
                        drc_tone(violation.severity),
                        false,
                    );
                }
            }
            for entry in state
                .log_buffer
                .entries()
                .filter(|entry| matches!(entry.severity, LogSeverity::Error | LogSeverity::Warning))
            {
                any = true;
                issue_row(
                    ui,
                    entry.severity.name(),
                    entry.context.as_deref().unwrap_or(entry.source.name()),
                    &entry.message,
                    log_tone(entry.severity),
                    false,
                );
            }
            if !any {
                muted(ui, "No current errors or advisories.");
            }
        });
}

fn netlist_problems(ui: &mut Ui, state: &mut AppState) {
    let diagnostics = std::sync::Arc::clone(&state.ui.netlist.diagnostics);
    if diagnostics.is_empty() {
        muted(ui, "No current netlist diagnostics.");
        return;
    }
    let mut requested = None;
    ScrollArea::vertical()
        .id_salt("workbench.problems.netlist")
        .show_rows(ui, CONSOLE_ROW_MIN_HEIGHT, diagnostics.len(), |ui, rows| {
            for index in rows {
                let Some(diagnostic) = diagnostics.get(index) else {
                    continue;
                };
                let current = diagnostic.is_current();
                let status = if current {
                    netlist_diagnostic_severity_name(diagnostic.severity).to_owned()
                } else {
                    format!(
                        "STALE {}",
                        netlist_diagnostic_severity_name(diagnostic.severity)
                    )
                };
                let source = netlist_diagnostic_location(diagnostic);
                let tone = if current {
                    netlist_diagnostic_tone(diagnostic.severity)
                } else {
                    SemanticTone::Trace
                };
                let response =
                    netlist_problem_row(ui, &status, &source, &diagnostic.message, tone, current);
                if response.clicked() {
                    requested = Some(diagnostic.canonical.diagnostic_id);
                }
                if !diagnostic.details.is_empty() {
                    response.on_hover_text(&diagnostic.details);
                }
            }
        });
    if let Some(diagnostic_id) = requested
        && let Err(error) = crate::workbench::documents::netlist_document::open_diagnostic_location(
            state,
            diagnostic_id,
        )
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
    }
}

fn netlist_problem_row(
    ui: &mut Ui,
    status: &str,
    source: &str,
    message: &str,
    tone: SemanticTone,
    interactive: bool,
) -> egui::Response {
    let tokens = Tokens::get(ui.ctx());
    let color = tone_color(&tokens, tone);
    let sense = if interactive {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), CONSOLE_ROW_MIN_HEIGHT),
        sense,
    );
    // The row is painted, not built from widgets, so nothing else declares
    // what it is: an interactive row is a keyboard target that opens the
    // diagnostic, and a stale one is still text a screen reader should read.
    let label = row_announcement(&[status, source, message]);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if interactive {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            label.clone(),
        )
    });
    if response.hovered() && interactive {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    let source_x = rect.left() + CONSOLE_TIME_WIDTH + CONSOLE_COLUMN_GAP;
    let message_x = source_x + CONSOLE_SOURCE_WIDTH + CONSOLE_COLUMN_GAP;
    for (text, x, width, font, text_color) in [
        (
            status,
            rect.left(),
            CONSOLE_TIME_WIDTH,
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
            color,
        ),
        (
            source,
            source_x,
            CONSOLE_SOURCE_WIDTH,
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::Medium),
            color,
        ),
        (
            message,
            message_x,
            (rect.right() - message_x).max(1.0),
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
            color,
        ),
    ] {
        let clip = egui::Rect::from_min_max(
            egui::pos2(x, rect.top()),
            egui::pos2((x + width).min(rect.right()), rect.bottom()),
        );
        ui.painter().with_clip_rect(clip).text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            text,
            font,
            text_color,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(format!("{status} · {source}\n{message}"))
}

/// Join the columns a painted console row shows into the one string a screen
/// reader announces, skipping the columns a continuation row leaves blank.
fn row_announcement(columns: &[&str]) -> String {
    columns
        .iter()
        .filter(|column| !column.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" · ")
}

fn netlist_diagnostic_location(
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) -> String {
    let line = diagnostic.line.or(diagnostic.source_line);
    let location = match (line, diagnostic.column) {
        (Some(line), Some(column)) => format!("line {} · column {}", line + 1, column + 1),
        (Some(line), None) => format!("line {}", line + 1),
        (None, _) => "document scope".to_owned(),
    };
    diagnostic
        .source_path
        .as_deref()
        .map_or(location.clone(), |path| {
            format!("{} · {location}", path.display())
        })
}

fn netlist_diagnostic_severity_name(
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> &'static str {
    use crate::workbench::documents::netlist_document::DiagnosticSeverity;
    match severity {
        DiagnosticSeverity::Error => "ERROR",
        DiagnosticSeverity::Warning => "WARNING",
        DiagnosticSeverity::Info => "INFO",
        DiagnosticSeverity::Hint => "HINT",
    }
}

fn netlist_diagnostic_tone(
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> SemanticTone {
    use crate::workbench::documents::netlist_document::DiagnosticSeverity;
    match severity {
        DiagnosticSeverity::Error => SemanticTone::Error,
        DiagnosticSeverity::Warning => SemanticTone::Warning,
        DiagnosticSeverity::Info => SemanticTone::Info,
        DiagnosticSeverity::Hint => SemanticTone::Debug,
    }
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
                    false,
                );
            }
            if !app.state.simulation.has_active_execution() {
                issue_row(
                    ui,
                    "IDLE",
                    "queue",
                    "No queued execution. The next run will snapshot the active plan and project revision.",
                    SemanticTone::Info,
                    false,
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
    if let Some(receipt) = app
        .state
        .simulation
        .active_run()
        .and_then(crate::state::SimulationRun::prepared_receipt)
    {
        return receipt
            .specifications()
            .iter()
            .map(|specification| specification.entry().clone())
            .collect();
    }
    let active = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| app.state.workspace.plan_data(plan.id()))
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
    let mut rows: Vec<MeasurementTableRow> = run
        .analyses
        .iter()
        .flat_map(|analysis| {
            analysis.measurements.iter().map(move |measurement| {
                let spec = specifications
                    .iter()
                    .find(|spec| spec.measurement.eq_ignore_ascii_case(&measurement.name));
                measurement_table_row(measurement, &analysis.label, spec)
            })
        })
        .collect();
    // Derived AC stability margins join the table as first-class rows: a
    // `phase_margin` specification binds to them exactly like a .measure
    // result, and the numbers are the stability inspector card's own. A
    // deck-declared measurement of the same name keeps authority.
    if let Some(summary) =
        crate::state::ac_bode_summary_for_selection(run, app.state.simulation.active_analysis_idx)
    {
        let label = run
            .analyses
            .get(summary.analysis_index)
            .map_or("AC", |analysis| analysis.label.as_str());
        let metrics = summary.metrics;
        for (name, value, event_axis) in [
            ("phase_margin", metrics.pm_deg, metrics.ugf),
            ("gain_margin", metrics.gm_db, metrics.f180),
            ("unity_gain_freq", metrics.ugf, metrics.ugf),
            ("bandwidth_3db", metrics.f3db, metrics.f3db),
        ] {
            let Some(value) = value else { continue };
            if rows.iter().any(|row| row.name.eq_ignore_ascii_case(name)) {
                continue;
            }
            let spec = specifications
                .iter()
                .find(|spec| spec.measurement.eq_ignore_ascii_case(name));
            let derived = rspice_core::MeasureResult {
                name: name.to_owned(),
                value: Some(value),
                error: None,
                passed: true,
                expected: None,
                tolerance: None,
                event_axis,
            };
            rows.push(measurement_table_row(&derived, label, spec));
        }
    }
    rows
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

/// One console line, plus the jump the reader asked it to follow.
///
/// The row cannot follow the anchor itself. The console paints its rows out
/// of the log buffer, so the session is borrowed immutably for the whole
/// pass, and navigating mutates it. The request rides back out to [`console`],
/// which holds the mutable borrow — the same shape [`netlist_problems`] uses
/// for the same reason. That also keeps the row off `RSpiceApp`: everything a
/// jump needs is on `AppState`.
fn log_row(
    ui: &mut Ui,
    state: &AppState,
    entry: &crate::diagnostics::LogEntry,
) -> Option<crate::diagnostics::LogAnchor> {
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
    let Some(anchor) = entry.anchor.as_ref() else {
        row(
            ui,
            &entry.format_timestamp(),
            entry.source.name(),
            &message,
            source_color,
            message_color,
        );
        return None;
    };
    // Asked before the row is sensed, not after it is clicked: an anchor
    // whose objects the drawing no longer carries must not look like a jump.
    let refusal = state.log_anchor_refusal(anchor);
    let response = row_with_sense(
        ui,
        &entry.format_timestamp(),
        entry.source.name(),
        &message,
        source_color,
        message_color,
        if refusal.is_some() {
            Sense::hover()
        } else {
            Sense::click()
        },
    );
    if let Some(refusal) = refusal {
        response.on_hover_text(refusal);
        return None;
    }
    let response = response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(log_anchor_hint(anchor));
    response.clicked().then(|| anchor.clone())
}

/// What following this row's anchor will do, in the words of the surface it
/// lands on.
fn log_anchor_hint(anchor: &crate::diagnostics::LogAnchor) -> String {
    use crate::diagnostics::LogAnchor;
    match anchor {
        LogAnchor::Schematic { .. } => "Show on the schematic".to_owned(),
        LogAnchor::Symbol { pin_name, .. } => format!("Show pin {pin_name} in the symbol view"),
        LogAnchor::Simulation { nets, devices } => {
            let count = nets.len() + devices.len();
            let objects = if count == 1 { "object" } else { "objects" };
            format!("Mark the {count} {objects} this run named on the schematic")
        }
        LogAnchor::ResultRun { run_sequence } => {
            format!("Open run {run_sequence} in Results")
        }
    }
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

fn issue_row(
    ui: &mut Ui,
    status: &str,
    source: &str,
    message: &str,
    tone: SemanticTone,
    interactive: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let color = tone_color(&t, tone);
    row_with_sense(
        ui,
        status,
        source,
        message,
        color,
        color,
        if interactive {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    )
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
) -> egui::Response {
    row_with_sense(
        ui,
        time,
        source,
        message,
        source_color,
        message_color,
        egui::Sense::hover(),
    )
}

fn row_with_sense(
    ui: &mut Ui,
    time: &str,
    source: &str,
    message: &str,
    source_color: egui::Color32,
    message_color: egui::Color32,
    sense: egui::Sense,
) -> egui::Response {
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
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), row_height), sense);
    // Every console line comes through here, and all three columns are
    // painted galleys. Without this the log is invisible to a screen reader,
    // and an issue row — which a clickable sense turns into a tab stop —
    // would take focus with nothing announced and nothing drawn.
    let label = row_announcement(&[time, source, message]);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if sense.senses_click() {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            label.clone(),
        )
    });
    // A row that can be clicked says so under the pointer. Painted before the
    // columns, so the fill sits behind its own text rather than over it.
    if sense.senses_click() && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
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
    theme::paint_focus_ring(ui, &response, rect);
    response
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
    fn netlist_problem_badge_is_owned_only_by_the_canonical_collection() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Netlist);
        app.state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        app.state
            .log_buffer
            .warning(LogSource::Simulation, "unrelated retained log warning");
        app.state.dialogs.drc_results = Some(DrcResult::new());
        app.state.ui.netlist.diagnostics = std::sync::Arc::new(
            crate::workbench::documents::netlist_document::NetlistDiagnosticCollection::try_new(
                vec![
                    crate::workbench::documents::netlist_document::Diagnostic::error(
                        "bad netlist card",
                    ),
                ],
                "",
            )
            .unwrap(),
        );

        assert_eq!(active_problem_count(&app.state), 1);
    }

    #[test]
    fn netlist_first_results_keep_the_canonical_problem_badge() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        let provenance = crate::state::AnalysisResultProvenance::new_with_source_domain(
            crate::state::AnalysisResultSourceDomain::ManualDeck,
            crate::product::AnalysisInstanceId::new(),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([0x52; 32]),
            Vec::new(),
        )
        .unwrap();
        app.state.simulation.start_run().add_analysis(
            crate::state::AnalysisResult::new(
                1,
                crate::state::AnalysisType::Transient,
                "manual transient",
            )
            .with_provenance(provenance),
        );
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        app.state
            .log_buffer
            .warning(LogSource::Simulation, "unrelated retained log warning");
        app.state.dialogs.drc_results = Some(DrcResult::new());

        assert!(!app.state.is_netlist_first_without_schematic());
        assert!(app.state.active_result_uses_manual_deck());
        assert!(netlist_diagnostics_own_problems(&app.state));
        assert_eq!(active_problem_count(&app.state), 0);
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
    fn the_clear_control_clears_the_visible_page_through_its_command() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .log_buffer
            .warning(LogSource::Simulation, "visible console warning");
        app.state.script_console.history.push(ConsoleHistoryItem {
            command: "help".to_owned(),
            output: Default::default(),
        });

        app.state.workbench.console_page = ConsolePage::Interactive;
        assert!(
            console_clear_action(
                ConsolePage::Interactive,
                Command::ClearConsole.is_enabled(&app)
            )
            .enabled,
            "the painted control and the command must agree on what is clearable"
        );
        Command::ClearConsole.execute(&mut app);
        assert!(app.state.script_console.history.is_empty());
        assert!(
            !app.state.log_buffer.is_empty(),
            "clearing the interactive page must not reach output the user cannot see"
        );

        app.state.workbench.console_page = ConsolePage::Problems;
        assert!(!Command::ClearConsole.is_enabled(&app));
        Command::ClearConsole.execute(&mut app);
        assert!(!app.state.log_buffer.is_empty());

        app.state.workbench.console_page = ConsolePage::Console;
        assert!(Command::ClearConsole.is_enabled(&app));
        Command::ClearConsole.execute(&mut app);
        assert!(app.state.log_buffer.is_empty());
        assert!(!Command::ClearConsole.is_enabled(&app));
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
            expression: String::new(),
            min: Some(9.0),
            max: Some(10.0),
            unit: "dB".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
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
        assert_eq!(CONSOLE_TAB_HEIGHT, 26.0);
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

    /// Render the Console page and collect the text it painted.
    fn painted_console(state: &mut AppState) -> String {
        fn collect(shape: &egui::epaint::Shape, rendered: &mut String) {
            match shape {
                egui::epaint::Shape::Text(text) => {
                    rendered.push_str(&text.galley.job.text);
                    rendered.push('\n');
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, rendered);
                    }
                }
                _ => {}
            }
        }

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
            },
        );
        let mut rendered = String::new();
        for clipped in &output.shapes {
            collect(&clipped.shape, &mut rendered);
        }
        rendered
    }

    /// Where the console painted the row whose message contains `needle`.
    ///
    /// Rows are painted galleys rather than widgets, so there is no widget id
    /// to look a rect up by. The text the row drew is the only handle a test
    /// has on it, which is the same handle a reader has.
    fn console_row_position(state: &mut AppState, needle: &str) -> egui::Pos2 {
        fn scan(shape: &egui::epaint::Shape, needle: &str, found: &mut Option<egui::Pos2>) {
            match shape {
                egui::epaint::Shape::Text(text) if found.is_none() => {
                    if text.galley.job.text.contains(needle) {
                        *found = Some(text.pos + text.galley.size() * 0.5);
                    }
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        scan(shape, needle, found);
                    }
                }
                _ => {}
            }
        }

        let ctx = console_context_for_tests();
        let output = ctx.run_ui(console_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        });
        let mut found = None;
        for clipped in &output.shapes {
            scan(&clipped.shape, needle, &mut found);
        }
        found.unwrap_or_else(|| panic!("the console painted no row containing {needle:?}"))
    }

    fn console_context_for_tests() -> egui::Context {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx
    }

    fn console_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(900.0, 600.0),
            )),
            events,
            ..Default::default()
        }
    }

    /// Click the console row whose message contains `needle`.
    ///
    /// Two frames, because egui resolves a press against the widget rects the
    /// previous frame registered: the first lays the rows out, the second
    /// delivers the press to the row that is now known to be there.
    fn click_console_row(state: &mut AppState, needle: &str) {
        let position = console_row_position(state, needle);
        let ctx = console_context_for_tests();
        let _ = ctx.run_ui(console_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        });
        let events = vec![
            egui::Event::PointerMoved(position),
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ];
        let _ = ctx.run_ui(console_input(events), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        });
    }

    /// A sheet drawing the conductor `OUT`, with a cross-probe map captured at
    /// the topology it is currently drawn at.
    fn state_with_probed_conductor() -> AppState {
        use crate::state::{Point, Wire};
        use std::collections::HashMap;

        let mut state = AppState::default();
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        state.schematic.wires.push(Wire::new(91, vec![a, b]));
        state.simulation.cross_probe.update(
            state.workspace.active_view.clone(),
            HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            HashMap::from([("OUT".to_owned(), vec![a, b])]),
            HashMap::new(),
            state.schematic.topology_version(),
        );
        state.log_buffer.clear();
        state
    }

    /// The whole point of an anchored row: the objects a failed run named are
    /// one click from the sentence that named them.
    #[test]
    fn clicking_an_anchored_console_row_marks_what_the_run_named() {
        let mut state = state_with_probed_conductor();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Error,
            LogSource::Simulation,
            "Analysis failed: no DC path at OUT",
            None,
            Some(crate::diagnostics::LogAnchor::Simulation {
                nets: vec!["OUT".to_owned()],
                devices: Vec::new(),
            }),
        );

        click_console_row(&mut state, "no DC path at OUT");

        assert!(
            state.schematic.selection.wires.contains(&91),
            "the row's anchor must mark the conductor the run named"
        );
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Design,
            "and take the reader to the drawing it marked it on"
        );
    }

    /// A name the drawing no longer carries must not look like a jump. The
    /// row stays put, says why on hover, and navigates nowhere when pressed.
    #[test]
    fn an_unresolvable_anchor_renders_inert_and_says_why() {
        let mut state = state_with_probed_conductor();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        let anchor = crate::diagnostics::LogAnchor::Simulation {
            nets: vec!["DELETED".to_owned()],
            devices: Vec::new(),
        };
        state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Error,
            LogSource::Simulation,
            "Analysis failed: no DC path at DELETED",
            None,
            Some(anchor.clone()),
        );

        let refusal = state
            .log_anchor_refusal(&anchor)
            .expect("a name this sheet does not draw cannot be jumped to");
        assert!(
            refusal.contains("DELETED"),
            "the tooltip must name what it could not find: {refusal}"
        );

        click_console_row(&mut state, "no DC path at DELETED");

        assert!(
            state.schematic.selection.wires.is_empty(),
            "an inert row must mark nothing"
        );
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Results,
            "and must not navigate away from what the reader was looking at"
        );
        assert!(
            state
                .log_buffer
                .entries()
                .all(|entry| !entry.message.contains("Marked")),
            "nor report a marking it did not perform"
        );
    }

    /// A run anchor opens the dataset it names, and refuses once that run is
    /// no longer retained rather than opening whatever is selected.
    #[test]
    fn a_run_anchor_opens_its_dataset_and_refuses_once_the_run_is_gone() {
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.runs.push(SimulationRun::new(7));
        let anchor = crate::diagnostics::LogAnchor::ResultRun { run_sequence: 7 };
        assert!(
            app.state.log_anchor_refusal(&anchor).is_none(),
            "a retained run is reachable"
        );

        app.state.jump_to_log_anchor(anchor.clone());
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );

        app.state.simulation.runs.clear();
        let refusal = app
            .state
            .log_anchor_refusal(&anchor)
            .expect("a run this session dropped cannot be opened");
        assert!(refusal.contains("Run 7"), "{refusal}");
    }

    /// A narrowed console shows only the producer's entries, says how much of
    /// the log that is, and offers the one control that puts the rest back.
    #[test]
    fn a_producer_filter_narrows_the_console_and_states_what_it_hid() {
        let mut app = RSpiceApp::test_instance();
        app.state.log_buffer.clear();
        app.state.log_buffer.log(
            crate::diagnostics::LogSeverity::Info,
            LogSource::Simulation,
            "  gain = 1.234000e1",
            None,
        );
        app.state.log_buffer.log(
            crate::diagnostics::LogSeverity::Info,
            LogSource::Simulation,
            "Transient: 512 points, 3 waveforms",
            None,
        );
        app.state.workbench.console_producer_filter =
            Some(crate::workbench::state::ConsoleProducerFilter::new(
                "dataset/7/analysis/3/artifact/gain",
                "gain",
            ));

        let rendered = painted_console(&mut app.state);
        assert!(
            rendered.contains("PRODUCER · gain · 1 of 2 entries"),
            "the strip must state the producer and how much of the log it keeps:\n{rendered}"
        );
        assert!(rendered.contains("gain = 1.234000e1"), "{rendered}");
        assert!(
            !rendered.contains("Transient: 512 points"),
            "an entry that is not this producer's must be filtered out:\n{rendered}"
        );
        assert!(rendered.contains("Show all entries"), "{rendered}");
        assert!(
            !app.state
                .workbench
                .console_producer_filter
                .as_ref()
                .expect("the filter survives the frame")
                .scroll_to_newest,
            "the one-shot scroll request is consumed by the frame that honours it"
        );
    }

    /// Nothing matching is a fact about the log, and the console says which
    /// fact rather than looking like an empty session.
    #[test]
    fn an_unmatched_producer_says_why_the_console_looks_empty() {
        let mut app = RSpiceApp::test_instance();
        app.state.log_buffer.clear();
        app.state.log_buffer.log(
            crate::diagnostics::LogSeverity::Info,
            LogSource::Simulation,
            "Transient: 512 points, 3 waveforms",
            None,
        );
        app.state.workbench.console_producer_filter =
            Some(crate::workbench::state::ConsoleProducerFilter::new(
                "dataset/7/analysis/3/quantity/V(out)",
                "V(out)",
            ));

        let rendered = painted_console(&mut app.state);
        assert!(
            rendered.contains("No console entry names V(out)"),
            "{rendered}"
        );
        assert!(
            rendered.contains("not yet tagged with the producer"),
            "the empty state names the reason, not just the absence:\n{rendered}"
        );
    }
}

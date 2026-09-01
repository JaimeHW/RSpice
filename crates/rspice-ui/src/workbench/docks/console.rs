//! Unified console, problems, measurements, and task history.

mod log_anchor;
#[cfg(test)]
mod measurement_rows_tests;

use log_anchor::log_row;

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

/// Simulation entries carry no producer identity, so a producer whose entries
/// never name its own quantity has none this filter can find. That is a fact
/// about the log rather than about the run, and the empty state says which — an
/// unexplained empty console would read as a session that never ran.
///
/// Present tense on purpose. "Not yet tagged" promised a tagging nobody has
/// scheduled, which is a claim about the roadmap rather than about the log the
/// reader is looking at.
const UNTAGGED_LOG_HINT: &str = "Simulation entries carry no producer tag, so this filter matches only the \
     entries that name the quantity themselves.";

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
    //
    // Fail closed exactly as the Bode sheet does. A failed solve's retained
    // vectors are whatever the engine emitted before it gave up, and a margin
    // read off them is not a measurement — which matters more here than on
    // the sheet, because a specification binds to these rows.
    if let Some(summary) =
        crate::state::ac_bode_summary_for_selection(run, app.state.simulation.active_analysis_idx)
        && let Some(analysis) = run.analyses.get(summary.analysis_index)
        && analysis.success
    {
        let label = analysis.label.as_str();
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
                raw_value: Some(value),
                error: None,
                passed: true,
                expected: None,
                tolerance: None,
                failure_limit: None,
                failure_limit_exceeded: false,
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
    let specification_text = measurement_contract_text(measurement, specification);
    let (status, tone, margin) = if let Some(error) = measurement
        .error
        .as_deref()
        .filter(|_| measurement.value.is_none())
        .filter(|error| !error.trim().is_empty())
    {
        ("ERROR", SemanticTone::Error, error.to_owned())
    } else if measurement_has_contract(measurement, specification) {
        measurement_contract_verdict(measurement, specification)
    } else if let Some(error) = measurement
        .error
        .as_deref()
        .filter(|error| !error.trim().is_empty())
    {
        ("ERROR", SemanticTone::Error, error.to_owned())
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

fn measurement_has_contract(
    measurement: &rspice_core::MeasureResult,
    specification: Option<&crate::state::SpecEntry>,
) -> bool {
    specification.is_some() || measurement.expected.is_some() || measurement.failure_limit.is_some()
}

fn measurement_contract_text(
    measurement: &rspice_core::MeasureResult,
    specification: Option<&crate::state::SpecEntry>,
) -> String {
    let mut contracts = Vec::with_capacity(3);
    if let Some(specification) = specification {
        contracts.push(("PROJECT", specification_text(specification)));
    }
    if let Some(expected) = measurement.expected {
        let text = measurement.tolerance.map_or_else(
            || format!("= {}", format_measure_value(expected)),
            |tolerance| {
                format!(
                    "{} ± {}",
                    format_measure_value(expected),
                    format_measure_value(tolerance)
                )
            },
        );
        contracts.push(("GOAL", text));
    }
    if let Some(limit) = measurement.failure_limit {
        contracts.push((
            "FAILVALUE",
            format!("|raw| < {}", format_measure_value(limit)),
        ));
    }

    match contracts.as_slice() {
        [] => "—".to_owned(),
        [(_, text)] => text.clone(),
        _ => contracts
            .into_iter()
            .map(|(label, text)| format!("{label} {text}"))
            .collect::<Vec<_>>()
            .join(" · "),
    }
}

fn measurement_contract_verdict(
    measurement: &rspice_core::MeasureResult,
    specification: Option<&crate::state::SpecEntry>,
) -> (&'static str, SemanticTone, String) {
    let Some(value) = measurement.value else {
        return ("NO DATA", SemanticTone::Warning, "—".to_owned());
    };
    if measurement.failure_limit.is_some() && measurement.raw_value.is_none() {
        return (
            "NO DATA",
            SemanticTone::Warning,
            "raw value unavailable".to_owned(),
        );
    }

    let project_passed = specification.is_none_or(|specification| specification.passes(value));
    let passed = measurement.passed && project_passed;
    let mut margins = Vec::with_capacity(4);
    let has_numeric_bound = measurement.expected.zip(measurement.tolerance).is_some()
        || measurement.failure_limit.is_some()
        || specification.is_some_and(|specification| {
            specification.min.is_some() || specification.max.is_some()
        });

    if let (Some(expected), Some(tolerance)) = (measurement.expected, measurement.tolerance) {
        margins.push(tolerance - (value - expected).abs());
    }
    if let (Some(raw_value), Some(limit)) = (measurement.raw_value, measurement.failure_limit) {
        let margin = limit - raw_value.abs();
        // FAILVALUE is inclusive: equality is a failure. Preserve that fact in
        // the signed presentation even though the mathematical clearance is
        // exactly zero.
        margins.push(if measurement.failure_limit_exceeded && margin == 0.0 {
            -0.0
        } else {
            margin
        });
    }
    if let Some(specification) = specification {
        margins.extend(
            specification
                .min
                .map(|minimum| value - minimum)
                .into_iter()
                .chain(specification.max.map(|maximum| maximum - value)),
        );
    }

    let limiting_margin = margins
        .into_iter()
        .filter(|margin| margin.is_finite())
        .reduce(f64::min);
    let margin = if passed {
        limiting_margin.map_or_else(
            || {
                if has_numeric_bound {
                    "—".to_owned()
                } else {
                    "unbounded".to_owned()
                }
            },
            format_signed_measure_value,
        )
    } else if let Some(margin) =
        limiting_margin.filter(|margin| *margin < 0.0 || margin.is_sign_negative())
    {
        format_signed_measure_value(margin)
    } else {
        measurement
            .error
            .as_deref()
            .filter(|error| !error.trim().is_empty())
            .unwrap_or("contract failed")
            .to_owned()
    };
    (
        if passed { "PASS" } else { "FAIL" },
        if passed {
            SemanticTone::Success
        } else {
            SemanticTone::Error
        },
        margin,
    )
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
mod tests;

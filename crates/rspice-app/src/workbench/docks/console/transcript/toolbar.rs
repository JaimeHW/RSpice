//! The Console page's controls, set in the dock's header band: the channel,
//! the three severity toggles and the find field.

use egui::{Key, Modifiers, Rect, Sense, Ui, Vec2, pos2};

use super::lines::Rows;
use super::{body_id, find_id, source};
use crate::state::SimulationRun;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    PROBLEM_MARK_SIDE, Problem, StatusMark, paint_problem_mark, paint_status_mark,
    select_mono_with_response,
};
use crate::workbench::AppState;
use crate::workbench::app_state::session::shortcuts::report_escape_owner;
use crate::workbench::state::{ConsoleChannel, ConsoleSeverity, ConsoleView};

/// What the channel selector calls the session's log.
const SESSION: &str = "Session";

/// A select's frame around its text: the inset before it and the chevron's
/// well after it.
const SELECT_CHROME: f32 = 32.0;

/// The find field's width, in characters of its face, and the least it
/// shrinks to.
const FIND_CHARACTERS: f32 = 24.0;
const FIND_MIN_CHARACTERS: f32 = 10.0;

/// A severity toggle's padding either side, and the gap between its mark and
/// its count.
const TOGGLE_PAD: f32 = 6.0;
const TOGGLE_GAP: f32 = 4.0;

/// Draw the controls, editing the reading position they are drawn from.
pub(in crate::workbench::docks) fn toolbar(ui: &mut Ui, state: &mut AppState) {
    let mut view = std::mem::take(&mut state.workbench.console_view);
    let counts = {
        let source = source(state, &mut view);
        let mut rows = Rows::take(ui.ctx());
        rows.sync(source, &view);
        let counts = ConsoleSeverity::ALL.map(|severity| rows.count(severity));
        rows.store(ui.ctx());
        counts
    };
    let advance = character_width(ui);
    ui.spacing_mut().item_spacing.x = 6.0;
    channel(ui, &state.simulation.retained.runs, &mut view, advance);
    for severity in ConsoleSeverity::ALL {
        if severity_toggle(ui, severity, counts[severity.index()], view.shows(severity)).clicked() {
            view.toggle(severity);
        }
    }
    find(ui, &mut view, advance);
    state.workbench.console_view = view;
}

/// The channel selector: the session's log first, then each run the session
/// holds, newest first, by the run's own label.
fn channel(ui: &mut Ui, runs: &[SimulationRun], view: &mut ConsoleView, advance: f32) {
    let mut options = Vec::with_capacity(runs.len() + 1);
    options.push(SESSION.to_owned());
    options.extend(runs.iter().map(|run| run.label.clone()));
    let selected = match view.channel {
        ConsoleChannel::Session => SESSION.to_owned(),
        ConsoleChannel::Run(sequence) => runs
            .iter()
            .find(|run| run.id == sequence)
            .map_or_else(|| SESSION.to_owned(), |run| run.label.clone()),
    };
    // Sized to the longest choice, so picking one never moves the controls
    // beside it.
    let widest = options
        .iter()
        .map(|option| option.chars().count())
        .max()
        .unwrap_or(0);
    let picked = select_mono_with_response(
        ui,
        "workbench.console.channel",
        "Console channel",
        &selected,
        &options,
        advance * widest as f32 + SELECT_CHROME,
    )
    .picked;
    if let Some(picked) = picked {
        let channel = picked
            .checked_sub(1)
            .and_then(|index| runs.get(index))
            .map_or(ConsoleChannel::Session, |run| ConsoleChannel::Run(run.id));
        if channel != view.channel {
            view.open(channel);
        }
    }
}

/// One severity toggle: the severity's mark and the channel's count of it.
/// Pressed while the severity is shown, as a toolbar toggle is.
fn severity_toggle(
    ui: &mut Ui,
    severity: ConsoleSeverity,
    count: usize,
    shown: bool,
) -> egui::Response {
    let tokens = Tokens::get(ui.ctx());
    let ink = if shown {
        tokens.color.text_dim
    } else {
        tokens.color.text_faint
    };
    let digits = ui.painter().layout_no_wrap(
        count.to_string(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        ink,
    );
    let size = Vec2::new(
        TOGGLE_PAD + PROBLEM_MARK_SIDE + TOGGLE_GAP + digits.size().x + TOGGLE_PAD,
        tokens.metrics.ctl_h,
    );
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let label = format!("{}, {count}", severity.label());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            shown,
            label.clone(),
        )
    });
    if shown {
        ui.painter()
            .rect_filled(rect, tokens.radius, tokens.color.bg_active);
    } else if response.hovered() {
        ui.painter()
            .rect_filled(rect, tokens.radius, tokens.color.bg_hover);
    }
    let mark = Rect::from_center_size(
        pos2(
            rect.left() + TOGGLE_PAD + PROBLEM_MARK_SIDE * 0.5,
            rect.center().y,
        ),
        Vec2::splat(PROBLEM_MARK_SIDE),
    );
    // A hidden severity's mark loses its colour with its rows.
    match (severity, shown) {
        (ConsoleSeverity::Errors, true) => paint_problem_mark(ui.painter(), mark, Problem::Failure),
        (ConsoleSeverity::Warnings, true) => {
            paint_problem_mark(ui.painter(), mark, Problem::Warning);
        }
        (ConsoleSeverity::Errors, false) => {
            paint_status_mark(ui.painter(), mark, Problem::Failure.mark(), ink);
        }
        (ConsoleSeverity::Warnings, false) => {
            paint_status_mark(ui.painter(), mark, Problem::Warning.mark(), ink);
        }
        // A note's mark carries no colour, because a note is not a problem.
        (ConsoleSeverity::Info, _) => {
            paint_status_mark(ui.painter(), mark, StatusMark::Info, ink);
        }
    }
    ui.painter().galley(
        pos2(
            mark.right() + TOGGLE_GAP,
            rect.center().y - digits.size().y * 0.5,
        ),
        digits,
        ink,
    );
    // Asked every frame, so a press takes away a ring Tab gave.
    if theme::focus_is_visible(ui.ctx(), &response) {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response.on_hover_text(format!("{}: {count}", severity.label()))
}

/// The find field. Escape clears it and gives the keyboard back to the log.
fn find(ui: &mut Ui, view: &mut ConsoleView, advance: f32) {
    let width = ui
        .available_width()
        .clamp(advance * FIND_MIN_CHARACTERS, advance * FIND_CHARACTERS);
    let response = ui.add_sized(
        [width, Tokens::get(ui.ctx()).metrics.ctl_h],
        egui::TextEdit::singleline(&mut view.find)
            .id(find_id())
            .hint_text("Find")
            // Centred in its well, as the selector's text beside it is.
            .vertical_align(egui::Align::Center)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Find in the log");
    });
    report_escape_owner(&response);
    // Consumed, so the log the keyboard goes back to does not read the same
    // Escape as a request to let go of it.
    if response.lost_focus()
        && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape))
    {
        view.find.clear();
        ui.memory_mut(|memory| memory.request_focus(body_id()));
    }
}

/// One character of the face the controls are set in, which is what they are
/// sized in rather than in pixels somebody typed.
fn character_width(ui: &Ui) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                "M".to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                egui::Color32::PLACEHOLDER,
            )
            .size()
            .x
    })
    .max(1.0)
}

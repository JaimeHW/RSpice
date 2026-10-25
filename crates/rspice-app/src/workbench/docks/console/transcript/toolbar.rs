//! The Console page's controls, on the page's toolbar band: the three
//! severity toggles, the channel and the find field — the controls the
//! Problems page sets for its own list, in the same order and the same kit.

use egui::{Key, Modifiers, Ui};

use super::super::CONSOLE_SELECT_CHROME;
use super::lines::Rows;
use super::{body_id, find_id, source};
use crate::state::SimulationRun;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, select_mono_with_response};
use crate::workbench::app_state::session::shortcuts::report_escape_owner;
use crate::workbench::framework::workbench_host::ConsoleHost;
use crate::workbench::state::{ConsoleChannel, ConsoleSeverity, ConsoleView};

/// What the channel selector calls the session's log.
const SESSION: &str = "Session";

/// The find field's width, in characters of its face, and the least it
/// shrinks to.
const FIND_CHARACTERS: f32 = 24.0;
const FIND_MIN_CHARACTERS: f32 = 10.0;

/// Draw the controls, editing the reading position they are drawn from.
///
/// Every severity keeps its toggle whatever its count: a log grows while it
/// is read, and a toggle that came and went with its first line would move
/// the controls beside it under the pointer.
pub(crate) fn toolbar(ui: &mut Ui, state: &mut impl ConsoleHost) {
    let mut view = std::mem::take(&mut state.workbench_mut().console_view);
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
    for severity in ConsoleSeverity::ALL {
        let label = format!("{} {}", severity.label(), counts[severity.index()]);
        if chip(ui, &label, view.shows(severity)).clicked() {
            view.toggle(severity);
        }
    }
    channel(
        ui,
        &state.model().simulation.retained.runs,
        &mut view,
        advance,
    );
    find(ui, &mut view, advance);
    state.workbench_mut().console_view = view;
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
        advance * widest as f32 + CONSOLE_SELECT_CHROME,
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

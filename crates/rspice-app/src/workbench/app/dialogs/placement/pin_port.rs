//! Create pins: name a cell's pins, then click the sheet to place each one.
//!
//! The form asks four questions and shows what the answers declare. It does
//! not preview the pin: the canvas already draws the real symbol under the
//! pointer, with the real direction overlay, from the same contract this form
//! arms — a second, hand-drawn pin inside the dialog could only ever disagree
//! with it.
//!
//! Nothing here mutates the document. The primary freezes a sequence of names
//! and arms the canvas tool; each click places the next name, measured against
//! the document as it is at that click.

use egui::Context;

use crate::state::{
    ComponentType, PendingPortSequence, PlacementAuthority, Tool, declared_vector, declared_width,
};
use crate::ui::widgets::{CommandForm, DialogChoice};

use crate::workbench::app::{RSpiceApp, schematic_editor_request_source};
use crate::workbench::app_state::AppState;
use rspice_schematic_editor::vector_preview::deck_bits;

const TITLE: &str = "Create pins";
const PRIMARY: &str = "Place";
const DESCRIPTION: &str = "Name the pins of this cell, then click to place each one.";
const READ_ONLY: &str = "This schematic is read-only.";
const DOCUMENT_CHANGED: &str = "The active schematic changed. Close this form and open it again.";
const TOO_MANY: &str = "A batch holds at most 256 pins.";

/// One batch's ceiling. A cell with more than this many new pins is a cell
/// being generated, not drawn.
const MAX_NAMES: usize = 256;

/// Open Create pins for the active schematic.
///
/// Reopening while a batch is armed offers the names that batch has left,
/// under the contract it was armed with, so the reader can correct a typo in
/// name four without replacing the three already on the sheet.
pub(crate) fn open_create_pins(state: &mut AppState) {
    let authority = PlacementAuthority::new(schematic_editor_request_source(state));
    let armed = state
        .schematic
        .session
        .editor
        .pending_port_sequence
        .clone()
        .filter(|sequence| !sequence.names.is_empty());
    let names = match &armed {
        Some(sequence) => sequence
            .names
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" "),
        // Empty on first use: a suggestion invented before the reader has
        // named anything is sample data, and this form used to open holding
        // one.
        None => match state.dialogs.pin_port.last_name.as_str() {
            "" => String::new(),
            base => state.schematic.suggested_port_name(base),
        },
    };
    let draft = &mut state.dialogs.pin_port;
    if let Some(sequence) = armed {
        draft.fields.direction = sequence.direction;
        draft.fields.signal_type = sequence.signal_type;
        draft.fields.discipline = sequence.discipline;
        draft.fields.discipline_touched = true;
    }
    draft.open(names, authority);
}

/// What the typed draft is, measured against the live document.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Draft {
    /// The form cannot be used at all, and says why.
    Blocked(&'static str),
    /// Nothing typed. The primary is off and the form says nothing.
    Empty,
    /// A refusal the reader has to fix.
    Refused(String),
    /// Names ready to arm, in the order they were typed.
    Ready(Vec<String>),
}

impl Draft {
    fn names(&self) -> &[String] {
        match self {
            Self::Ready(names) => names,
            _ => &[],
        }
    }

    /// Why the primary is refused, if it is. An empty form says nothing: the
    /// reader has not done anything wrong by not having typed yet.
    fn refusal(&self) -> Option<String> {
        match self {
            Self::Blocked(message) => Some((*message).to_owned()),
            Self::Refused(message) => Some(message.clone()),
            Self::Empty | Self::Ready(_) => None,
        }
    }
}

fn draft(state: &AppState) -> Draft {
    let form = &state.dialogs.pin_port;
    if state.schematic_edit_read_only() {
        return Draft::Blocked(READ_ONLY);
    }
    let current = form
        .authority
        .as_ref()
        .is_some_and(|authority| authority.matches(&schematic_editor_request_source(state)));
    if !current {
        return Draft::Blocked(DOCUMENT_CHANGED);
    }
    // A pin name can never contain whitespace under either naming policy, so
    // splitting on it is unambiguous and needs no separator to be chosen.
    let names: Vec<String> = form
        .fields
        .names
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if names.is_empty() {
        return Draft::Empty;
    }
    if names.len() > MAX_NAMES {
        return Draft::Refused(TOO_MANY.to_owned());
    }
    let mut seen = std::collections::HashSet::new();
    for name in &names {
        if !seen.insert(name.to_ascii_lowercase()) {
            return Draft::Refused(format!("{name} is listed twice."));
        }
    }
    let several = names.len() > 1;
    for name in &names {
        if let Err(error) = state.schematic.validate_new_port_name(name) {
            // One name needs no prefix: the field it was typed in is the
            // subject. Several do, or the reader cannot tell which one.
            return Draft::Refused(if several {
                format!("{name}: {error}")
            } else {
                error.to_string()
            });
        }
    }
    Draft::Ready(names)
}

/// What the names declare: the pins, the conductors under them, and where they
/// will sit in the cell's port list.
fn declaration_line(state: &AppState, names: &[String]) -> Option<String> {
    if names.is_empty() {
        return None;
    }
    let conductors: usize = names.iter().map(|name| declared_width(name.as_str())).sum();
    let first = state.schematic.next_interface_order();
    let positions = if names.len() == 1 {
        format!("port-list position {first}")
    } else {
        format!("port-list positions {first} to {}", first + names.len() - 1)
    };
    Some(format!(
        "{} {} \u{00b7} {} {} \u{00b7} {positions}",
        names.len(),
        plural(names.len(), "pin", "pins"),
        conductors,
        plural(conductors, "conductor", "conductors"),
    ))
}

/// The deck formals, for the one case where a reader cannot work them out: a
/// single name that declares a range.
fn deck_bits_line(names: &[String]) -> Option<String> {
    let [only] = names else {
        return None;
    };
    let declaration = declared_vector(only)?;
    Some(deck_bits(
        &declaration.name,
        declaration.members().into_iter().map(|member| member.index),
    ))
}

fn plural(count: usize, one: &'static str, many: &'static str) -> &'static str {
    if count == 1 { one } else { many }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_pin_port_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.pin_port.open {
            return;
        }
        let current = draft(&self.state);
        let declaration = declaration_line(&self.state, current.names());
        let bits = deck_bits_line(current.names());
        let usable = !matches!(current, Draft::Blocked(_));
        let ready = matches!(current, Draft::Ready(_));
        let refusal = current.refusal();
        // The cell whose interface this batch will change. It is the one thing
        // about the form that is not in the form, and the reason the
        // document-changed refusal exists at all.
        let view_path = self.state.workspace.content.active_view.display_path();

        let form = &mut self.state.dialogs.pin_port;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .context(Some(view_path.as_str()))
            .describe(DESCRIPTION)
            .primary_enabled(usable && ready)
            .fields_enabled(usable)
            .status(match refusal.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .status_lines(2)
            .show(ctx, |rows| {
                form.fields
                    .show(rows, declaration.as_deref(), bits.as_deref())
            });

        match choice {
            // Revalidate the post-edit frame: Enter must never arm a batch the
            // same frame's keystroke just made invalid.
            DialogChoice::Primary => {
                if let Draft::Ready(names) = draft(&self.state) {
                    self.arm_create_pins(ctx, names);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.pin_port.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    /// Freeze the batch, arm the canvas tool, and give the canvas the keyboard
    /// so R, M and Esc work before the pointer has moved over it.
    fn arm_create_pins(&mut self, ctx: &Context, names: Vec<String>) {
        let form = &self.state.dialogs.pin_port;
        let Some(authority) = form.authority.clone() else {
            return;
        };
        let sequence = PendingPortSequence::new(
            names.iter().cloned(),
            form.fields.direction,
            form.fields.signal_type,
            form.fields.discipline,
        )
        .with_authority(authority);
        let last_name = names.last().cloned().unwrap_or_default();
        self.state
            .schematic
            .arm_tool(Tool::Place(ComponentType::Port));
        self.state.schematic.session.editor.pending_port_sequence = Some(sequence);
        rspice_schematic_editor::view::canvas::request_focus(ctx);
        let form = &mut self.state.dialogs.pin_port;
        form.last_name = last_name;
        form.close();
    }
}

#[cfg(test)]
mod tests;

//! The traversal grammar the design side panel's rails answer to.
//!
//! Every rail of the Design navigator and of the Component shelf is one
//! vertical list of rows, and a reader without a pointer moves through it the
//! way a reader of any other production tree does: Up and Down step, Home and
//! End reach the ends, Right and Left unfold and fold, Enter and Space
//! activate. egui gives none of that on its own — a stack of `Sense::click()`
//! rows is a Tab ring and nothing more — so the rails publish what they paint
//! and this module answers the keys over it.
//!
//! # Why an index rather than a widget
//!
//! Those rows are laid down by six painters across three modules, and what a
//! traversal needs from each of them is the same four facts: the id the row
//! keeps, where it was laid out, how deep it sits, and whether it discloses
//! children. Each painter publishes those as it paints — in paint order, which
//! is the order the reader meets the rows in — and [`traverse`] answers the
//! keys once the rail is complete.
//!
//! The ids are the rows' own persisted identities: `("navigator-occurrence",
//! fold key)` and its like, never a frame-local index. A `ScrollArea` that
//! re-lays out between two frames would otherwise hand the focus to whatever
//! row happens to occupy the position the old one had.
//!
//! # What consumes a key
//!
//! Nothing here consumes anything unless one of the published rows — or the
//! disclosure control of one — holds the keyboard focus. The canvas owns the
//! arrows the rest of the time: it nudges a selection and traverses objects
//! with them, and a panel that ate them from the side would take the canvas's
//! own keys away from it. The single exception is the filter field's Down,
//! which the field consumes itself and hands here as [`traverse`]'s
//! `enter_rows`, because stepping from the field into the rows is the one move
//! that starts outside the rail.

use egui::{FocusDirection, Id, Key, Modifiers, Rect, Response, Ui};

use crate::workbench::state::NavigatorTreeNode;

/// Where one row's fold position is kept, so a key can move it.
///
/// Both spellings are already in the panel; neither is introduced here. What
/// this enum adds is a way to name one of them without a closure, so the index
/// stays a plain value the frame can carry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum RailFold {
    /// A `bool` under the row's own persisted id — the navigator's section
    /// bands and the shelf's catalog groups both hold their position there.
    Persisted(Id),
    /// A node of this workspace's navigator tree — the masters and the
    /// occurrences.
    Tree(NavigatorTreeNode),
}

/// A row that discloses children: the position it is in, and where that
/// position is kept.
#[derive(Clone, Debug)]
pub(super) struct RailDisclosure {
    pub unfolded: bool,
    pub fold: RailFold,
}

/// One published row.
#[derive(Clone, Debug)]
struct RailRow {
    id: Id,
    /// The disclosure control's own hit target, where the row draws one. It is
    /// in the Tab ring beside the row, so a reader can arrive on it, and the
    /// traversal treats it as the row it belongs to rather than as a gap.
    caret: Option<Id>,
    rect: Rect,
    /// How deep the row sits in its rail. A section band is 0 and its rows are
    /// one deeper than their own indentation, so Left has somewhere to climb
    /// to from the outermost row of every rail.
    level: usize,
    disclosure: Option<RailDisclosure>,
}

impl RailRow {
    /// Whether the keyboard is on this row, wherever in it the Tab ring left.
    fn holds(&self, focus: Id) -> bool {
        self.id == focus || self.caret == Some(focus)
    }
}

/// The rail being published this frame.
#[derive(Clone, Default)]
struct RailIndex {
    /// A panel that traverses opened this index at the top of its frame.
    ///
    /// The flag, rather than the index's presence, is what scopes the
    /// collection: [`row`] is called from painters the rest of the workbench
    /// shares, and a panel that never traverses must not grow a list nothing
    /// ever reads.
    collecting: bool,
    rows: Vec<RailRow>,
}

fn index_id() -> Id {
    Id::new("workbench.navigator.rail")
}

/// Begin publishing a rail. Called once, above the rows it will hold.
pub(super) fn open(ctx: &egui::Context) {
    ctx.data_mut(|data| {
        let index = data.get_temp_mut_or_default::<RailIndex>(index_id());
        index.collecting = true;
        index.rows.clear();
    });
}

/// Publish one painted row.
///
/// A no-op unless a rail is open, and unless the row can take the focus at
/// all: a row painted disabled is not in the Tab ring, and a traversal that
/// stepped onto one would strand the keyboard on a row that answers nothing.
pub(super) fn row(ui: &Ui, response: &Response, level: usize, disclosure: Option<RailDisclosure>) {
    publish(ui, response, None, level, disclosure);
}

/// Publish one painted row whose disclosure control is a second hit target of
/// its own.
pub(super) fn row_with_caret(
    ui: &Ui,
    response: &Response,
    caret: Id,
    level: usize,
    disclosure: Option<RailDisclosure>,
) {
    publish(ui, response, Some(caret), level, disclosure);
}

fn publish(
    ui: &Ui,
    response: &Response,
    caret: Option<Id>,
    level: usize,
    disclosure: Option<RailDisclosure>,
) {
    if !ui.is_enabled() {
        return;
    }
    let row = RailRow {
        id: response.id,
        caret,
        rect: response.rect,
        level,
        disclosure,
    };
    ui.ctx().data_mut(|data| {
        let index = data.get_temp_mut_or_default::<RailIndex>(index_id());
        if index.collecting {
            index.rows.push(row);
        }
    });
}

/// What one press asks of the rail.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RailStep {
    Previous,
    Next,
    First,
    Last,
    /// Unfold the row, or step onto the first child of one already unfolded.
    Into,
    /// Fold the row, or climb to the row that holds it.
    Out,
}

/// Answer this frame's traversal keys over the rail just published, and close
/// it.
///
/// Call this from inside the rail's own `ScrollArea`, below its last row. The
/// scroll target it sets is read by the scroll area as that area closes, so a
/// call made outside it would move the focus and leave the row off screen.
///
/// Returns the tree node whose fold the reader moved, for the caller to apply
/// to the workspace's own tree. A band's persisted position is moved here
/// instead of returned, because this module can reach it and no caller has to.
pub(super) fn traverse(ui: &mut Ui, enter_rows: bool) -> Option<NavigatorTreeNode> {
    let rows = ui.ctx().data_mut(|data| {
        let index = data.get_temp_mut_or_default::<RailIndex>(index_id());
        index.collecting = false;
        std::mem::take(&mut index.rows)
    });
    if rows.is_empty() {
        return None;
    }
    // A row's context menu is raised from these same rows and anchored to the
    // one it was raised from, so while one is open the reader is in the menu
    // and the arrows are the menu's — walking the rail behind it would move
    // the anchor out from under the thing on screen. The canvas guards its own
    // object traversal the same way.
    //
    // Tooltips are not this: egui files them outside the popup registry, so a
    // row merely being hovered goes on answering the keys.
    if egui::Popup::is_any_open(ui.ctx()) {
        return None;
    }
    let last = rows.len() - 1;

    // The filter field consumed the press already; what is left is to land on
    // a row, which only the finished rail knows.
    if enter_rows {
        focus(ui, &rows[0]);
        return None;
    }

    let focused = ui.memory(|memory| memory.focused())?;
    let current = rows.iter().position(|row| row.holds(focused))?;

    let step = ui.input_mut(|input| {
        if input.consume_key(Modifiers::NONE, Key::ArrowDown) {
            Some(RailStep::Next)
        } else if input.consume_key(Modifiers::NONE, Key::ArrowUp) {
            Some(RailStep::Previous)
        } else if input.consume_key(Modifiers::NONE, Key::Home) {
            Some(RailStep::First)
        } else if input.consume_key(Modifiers::NONE, Key::End) {
            Some(RailStep::Last)
        } else if input.consume_key(Modifiers::NONE, Key::ArrowRight) {
            Some(RailStep::Into)
        } else if input.consume_key(Modifiers::NONE, Key::ArrowLeft) {
            Some(RailStep::Out)
        } else {
            None
        }
    })?;

    let destination = match step {
        RailStep::Next => Some((current + 1).min(last)),
        RailStep::Previous => Some(current.saturating_sub(1)),
        RailStep::First => Some(0),
        RailStep::Last => Some(last),
        // The caret's own semantics, reached from the keyboard: a folded row
        // unfolds, an unfolded one is already showing its children and the
        // press steps onto the first of them, and a leaf answers nothing.
        //
        // An unfolded row can still have nothing under it — a rail whose whole
        // contents were filtered away is open and empty — and stepping to the
        // row after it there would be a Down wearing the other key's name.
        RailStep::Into => match &rows[current].disclosure {
            Some(disclosure) if !disclosure.unfolded => {
                return finish(ui, fold(ui, disclosure));
            }
            Some(_) => child_of(&rows, current),
            None => None,
        },
        // And its mirror: an unfolded row folds, and anything else climbs to
        // the row that holds it.
        RailStep::Out => match &rows[current].disclosure {
            Some(disclosure) if disclosure.unfolded => {
                return finish(ui, fold(ui, disclosure));
            }
            _ => parent_of(&rows, current),
        },
    };

    match destination {
        Some(index) => focus(ui, &rows[index]),
        // A leaf asked to unfold: the press is still the rail's, so egui does
        // not get to move the focus off the row with it.
        None => cancel_spatial_move(ui),
    }
    None
}

/// Land the keyboard on one row and bring it into view.
///
/// `None` rather than a fixed alignment: a step to the neighbouring row should
/// scroll by one row, not re-centre the whole rail under the reader.
fn focus(ui: &mut Ui, row: &RailRow) {
    cancel_spatial_move(ui);
    ui.memory_mut(|memory| memory.request_focus(row.id));
    ui.scroll_to_rect(row.rect, None);
}

/// Take the press back from egui's own focus machinery.
///
/// egui reads the arrow keys at the top of the pass, before any of this runs,
/// and moves the focus to the nearest widget in that direction as the pass
/// closes. Consuming the event does not undo that — the direction is already
/// recorded — so a rail that only consumed would move the focus once itself
/// and then have egui move it again from the row it just landed on.
fn cancel_spatial_move(ui: &Ui) {
    ui.memory_mut(|memory| memory.move_focus(FocusDirection::None));
}

/// Move one row's fold position, and hand back the one this module cannot
/// reach.
fn fold(ui: &Ui, disclosure: &RailDisclosure) -> Option<NavigatorTreeNode> {
    match &disclosure.fold {
        RailFold::Persisted(id) => {
            let id = *id;
            let unfolded = !disclosure.unfolded;
            ui.data_mut(|data| data.insert_persisted(id, unfolded));
            None
        }
        RailFold::Tree(node) => Some(node.clone()),
    }
}

fn finish(ui: &Ui, folded: Option<NavigatorTreeNode>) -> Option<NavigatorTreeNode> {
    cancel_spatial_move(ui);
    folded
}

/// The row that holds `current`: the nearest row above it that sits shallower.
fn parent_of(rows: &[RailRow], current: usize) -> Option<usize> {
    let level = rows[current].level;
    rows[..current].iter().rposition(|row| row.level < level)
}

/// The first row `current` holds, which is the row after it exactly when that
/// row sits deeper.
fn child_of(rows: &[RailRow], current: usize) -> Option<usize> {
    let child = current + 1;
    rows.get(child)
        .is_some_and(|row| row.level > rows[current].level)
        .then_some(child)
}

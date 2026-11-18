//! Whether this browser tab may write the open project: it holds the
//! project's open lock, or another tab of this browser has the project open
//! and it opened read-only here, as a desktop window does when another
//! window holds the project. The browser's Web Locks are the lease, and the
//! browser lets go of a tab's lock when the tab closes or reloads.
//!
//! The lock is named for the binding the browser keeps the project's file
//! under, so every tab that reopens it from the recent list asks for the
//! same one. A file opened again through Open project is bound afresh; a
//! save from either tab that meets the other's changes still writes nothing.

use std::cell::RefCell;
use std::collections::VecDeque;

use super::persistence::{BrowserOpenLock, BrowserOpenLockAttempt, start_browser_open_lock};
use super::{LifecycleParts, LifecycleSourceMut, PersistenceBinding};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::write_policy::OpenedReadOnly;

/// This tab's standing with the open project's lock.
enum OpenLock {
    /// Asked for; the answer has not come back.
    Asking,
    /// Held for as long as it is kept here; dropping it lets the lock go.
    Held { _lock: BrowserOpenLock },
    /// Another tab holds it, so the project opened read-only here.
    Elsewhere,
    /// Another tab holds it and the reader took the project for writing all
    /// the same.
    Anyway,
    /// The browser has no locks to tell its tabs apart with.
    Unavailable,
}

/// An answer to an ask, and whether the ask was "Open anyway".
type Answer = (uuid::Uuid, BrowserOpenLockAttempt, bool);

thread_local! {
    /// The binding the open project is kept under, and this tab's standing
    /// with its lock.
    static OPEN_LOCK: RefCell<Option<(uuid::Uuid, OpenLock)>> = const { RefCell::new(None) };
    static ANSWERS: RefCell<VecDeque<Answer>> = const { RefCell::new(VecDeque::new()) };
}

/// Keep the lock with the open project, once a frame after the project's
/// opens and saves have landed: take the answers that came back, let go of
/// the lock of a project no longer open here, and ask for the lock of the
/// one that is.
pub fn follow(state: &mut dyn LifecycleSourceMut) {
    let state = &mut state.lifecycle_parts();
    let open = open_binding(state);
    while let Some((binding_id, attempt, anyway)) =
        ANSWERS.with(|answers| answers.borrow_mut().pop_front())
    {
        answer(state, open, binding_id, attempt, anyway);
    }
    let followed = OPEN_LOCK.with(|lock| lock.borrow().as_ref().map(|(id, _)| *id));
    if followed == open {
        return;
    }
    // The project closed, or another took its place, or a save moved it to
    // a file of its own: what another tab held of the old one is past.
    if matches!(
        state.model.safe_mode.opened_read_only,
        Some(OpenedReadOnly::OpenElsewhere { .. })
    ) {
        state.model.safe_mode.opened_read_only = None;
    }
    OPEN_LOCK.with(|lock| *lock.borrow_mut() = open.map(|id| (id, OpenLock::Asking)));
    if let Some(binding_id) = open {
        ask(binding_id, false);
    }
}

/// A project opened again in this tab is asked for again, as a desktop
/// window asks on every open; a lock this tab holds is kept.
pub(super) fn reopened() {
    OPEN_LOCK.with(|lock| {
        let mut lock = lock.borrow_mut();
        if !matches!(lock.as_ref(), Some((_, OpenLock::Held { .. }))) {
            *lock = None;
        }
    });
}

/// "Open anyway": take for writing the project another tab holds. The lock
/// is taken too if that tab has let it go since; whoever saves second is
/// told the file changed and writes nothing.
pub fn open_anyway(state: &mut dyn LifecycleSourceMut) -> bool {
    let state = &mut state.lifecycle_parts();
    if !matches!(
        state.model.safe_mode.opened_read_only,
        Some(OpenedReadOnly::OpenElsewhere { .. })
    ) {
        return false;
    }
    state.model.safe_mode.opened_read_only = None;
    if let Some(binding_id) = open_binding(state) {
        OPEN_LOCK.with(|lock| *lock.borrow_mut() = Some((binding_id, OpenLock::Asking)));
        ask(binding_id, true);
    }
    true
}

fn answer(
    state: &mut LifecycleParts<'_>,
    open: Option<uuid::Uuid>,
    binding_id: uuid::Uuid,
    attempt: BrowserOpenLockAttempt,
    anyway: bool,
) {
    let current = open == Some(binding_id)
        && OPEN_LOCK.with(|lock| {
            matches!(lock.borrow().as_ref(), Some((id, OpenLock::Asking)) if *id == binding_id)
        });
    if !current {
        // A lock taken for a project no longer open here goes as it drops.
        return;
    }
    let standing = match attempt {
        BrowserOpenLockAttempt::Held(lock) => {
            if anyway {
                state.model.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    "Opened the project for writing; the other tab has closed it.",
                );
            }
            OpenLock::Held { _lock: lock }
        }
        BrowserOpenLockAttempt::HeldElsewhere if anyway => {
            state.model.record(
                LogSeverity::Info,
                LogSource::Studio,
                "Opened the project for writing while another tab of this browser also has it \
                 open. A save that meets changes saved there writes nothing and says so.",
            );
            OpenLock::Anyway
        }
        BrowserOpenLockAttempt::HeldElsewhere => {
            state.model.safe_mode.opened_read_only =
                Some(OpenedReadOnly::OpenElsewhere { holder: None });
            OpenLock::Elsewhere
        }
        BrowserOpenLockAttempt::Unavailable => OpenLock::Unavailable,
    };
    OPEN_LOCK.with(|lock| *lock.borrow_mut() = Some((binding_id, standing)));
}

fn ask(binding_id: uuid::Uuid, anyway: bool) {
    start_browser_open_lock(binding_id, move |attempt| {
        ANSWERS.with(|answers| {
            answers
                .borrow_mut()
                .push_back((binding_id, attempt, anyway));
        });
        crate::workbench::browser::file_import::request_browser_import_repaint();
    });
}

/// The binding the open project is kept under, once the browser keeps it.
fn open_binding(state: &LifecycleParts<'_>) -> Option<uuid::Uuid> {
    if !state.platform.project_lifecycle.is_open() {
        return None;
    }
    state
        .platform
        .project_lifecycle
        .binding()
        .and_then(PersistenceBinding::durable_browser_receipt)
        .map(|receipt| receipt.binding_id)
}

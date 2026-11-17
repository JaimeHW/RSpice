//! Whether this window may write the open project: it holds the project's
//! open lease, or it opened the project read-only and says why.
//!
//! Each answer is worked out once, when the project is opened, restored,
//! saved somewhere new or taken with "Open anyway"; the notice and every
//! write gate read the answer, never the disk.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::{LifecycleParts, LifecycleSourceMut};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::io::durable_file::open_lease::{
    self, OpenLease, OpenLeaseAttempt, OpenLeaseHolder, WriteRefusal,
};
use crate::state::write_policy::{OpenedReadOnly, ProjectHolder};

/// What this window may do with the project at one path.
struct Access {
    lease: Option<Rc<OpenLease>>,
    read_only: Option<OpenedReadOnly>,
    /// Said on the Console when the answer could not be worked out whole.
    warning: Option<String>,
}

/// Take the open lease of the project just opened or restored, or open it
/// read-only and say why. A project with no file holds nothing.
pub(super) fn claim(state: &mut LifecycleParts<'_>) {
    let Some(path) = bound_path(state) else {
        release(state);
        return;
    };
    // A window that opens again the project it holds keeps holding it.
    let held = state
        .platform
        .project_lifecycle
        .open_lease
        .take()
        .filter(|lease| lease.covers(&path));
    let access = access_at(&path, held);
    state.platform.project_lifecycle.open_lease = access.lease;
    state.model.safe_mode.opened_read_only = access.read_only;
    if let Some(warning) = access.warning {
        state
            .model
            .record(LogSeverity::Warning, LogSource::Studio, warning);
    }
}

/// Let go of the lease and the read-only answer: the project closed, or the
/// one open now has no file.
pub(super) fn release(state: &mut LifecycleParts<'_>) {
    state.platform.project_lifecycle.open_lease = None;
    state.model.safe_mode.opened_read_only = None;
}

/// After a save: the file was written, so this window can write it. A save
/// to a new path takes that path's lease, letting the old one go; a save
/// over a file another window has open writes it all the same, as the
/// reader chose.
pub(super) fn follow_save(state: &mut LifecycleParts<'_>) {
    state.model.safe_mode.opened_read_only = None;
    let Some(path) = bound_path(state) else {
        state.platform.project_lifecycle.open_lease = None;
        return;
    };
    let held_here = state
        .platform
        .project_lifecycle
        .open_lease
        .as_ref()
        .is_some_and(|lease| lease.covers(&path));
    if !held_here {
        state.platform.project_lifecycle.open_lease = try_lease(&path).map(Rc::new);
    }
}

/// "Open anyway": take a project another window holds for writing. The
/// lease is taken too if that window has let it go since; whoever saves
/// second is then shown what the other changed.
pub fn open_anyway(state: &mut dyn LifecycleSourceMut) -> bool {
    let state = &mut state.lifecycle_parts();
    let Some(OpenedReadOnly::OpenElsewhere { holder }) =
        state.model.safe_mode.opened_read_only.clone()
    else {
        return false;
    };
    state.model.safe_mode.opened_read_only = None;
    let lease = bound_path(state).and_then(|path| try_lease(&path));
    let taken = lease.is_some();
    state.platform.project_lifecycle.open_lease = lease.map(Rc::new);
    let line = if taken {
        "Opened the project for writing; the other window has closed it.".to_owned()
    } else {
        let who = holder
            .map(|holder| holder.display_name())
            .unwrap_or_else(|| "another RSpice window".to_owned());
        format!(
            "Opened the project for writing while {who} also has it open. A save that meets \
             changes saved there shows what differs before anything is written."
        )
    };
    state
        .model
        .record(LogSeverity::Info, LogSource::Studio, line);
    true
}

/// Who holds the project at `path` open now, read when a save meets a file
/// changed on disk.
pub(super) fn holder_of(path: &Path) -> Option<ProjectHolder> {
    open_lease::open_lease_holder(path)
        .ok()
        .flatten()
        .map(project_holder)
}

fn bound_path(state: &LifecycleParts<'_>) -> Option<PathBuf> {
    state
        .platform
        .project_lifecycle
        .canonical_native_path()
        .map(Path::to_path_buf)
}

fn access_at(path: &Path, held: Option<Rc<OpenLease>>) -> Access {
    let folder = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| path.to_path_buf());
    let mut warning = None;
    match open_lease::write_refusal(path) {
        Ok(Some(WriteRefusal::FolderNotWritable(_))) => {
            return Access {
                lease: None,
                read_only: Some(OpenedReadOnly::FolderNotWritable { folder }),
                warning: None,
            };
        }
        Ok(Some(WriteRefusal::FileReadOnly)) => {
            return Access {
                lease: None,
                read_only: Some(OpenedReadOnly::FileReadOnly),
                warning: None,
            };
        }
        Ok(None) => {}
        Err(error) => {
            warning = Some(format!(
                "Could not tell whether the project's folder can be written: {error}."
            ));
        }
    }
    if let Some(lease) = held {
        return Access {
            lease: Some(lease),
            read_only: None,
            warning,
        };
    }
    match open_lease::acquire_open_lease(path, &OpenLeaseHolder::this_process(now())) {
        Ok(OpenLeaseAttempt::Held(lease)) => Access {
            lease: Some(Rc::new(lease)),
            read_only: None,
            warning,
        },
        Ok(OpenLeaseAttempt::HeldElsewhere(holder)) => Access {
            lease: None,
            read_only: Some(OpenedReadOnly::OpenElsewhere {
                holder: holder.map(project_holder),
            }),
            warning,
        },
        Err(error) if open_lease::is_not_writable(&error) => Access {
            lease: None,
            read_only: Some(OpenedReadOnly::FolderNotWritable { folder }),
            warning: None,
        },
        Err(error) => Access {
            lease: None,
            read_only: None,
            warning: Some(format!(
                "Other RSpice windows will not be told this project is open here: {error}."
            )),
        },
    }
}

fn try_lease(path: &Path) -> Option<OpenLease> {
    match open_lease::acquire_open_lease(path, &OpenLeaseHolder::this_process(now())) {
        Ok(OpenLeaseAttempt::Held(lease)) => Some(lease),
        Ok(OpenLeaseAttempt::HeldElsewhere(_)) | Err(_) => None,
    }
}

fn project_holder(holder: OpenLeaseHolder) -> ProjectHolder {
    ProjectHolder {
        user: holder.user,
        machine: holder.machine,
        since_unix_ms: holder.since_unix_ms,
    }
}

fn now() -> u64 {
    crate::time_compat::checked_unix_time_ms().unwrap_or(0)
}

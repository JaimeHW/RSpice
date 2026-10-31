//! App Dialog State
//!
//! The Help Center's pages, which the shell's dialogs share. Each feature's
//! open dialogs live in its slot in [`crate::workbench::slots`].

mod sessions;

pub(crate) use sessions::HelpCenterPage;

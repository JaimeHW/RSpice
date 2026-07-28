//! Persistent application chrome.
//!
//! The frame around the active surface: the activity rail, the title bar and
//! its menus, the toolbar, the document tab bar, the status bar, and the
//! phone-form navigation that replaces the rail on a narrow viewport. Chrome
//! is present in every workspace; what it shows changes, whether it is drawn
//! does not.

pub(crate) mod activity_rail;
pub(crate) mod document_bar;
pub(crate) mod phone_navigation;
pub(crate) mod status_bar;
pub(crate) mod title_bar;
pub(crate) mod toolbar;

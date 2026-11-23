//! The desktop's deck pickers.

#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::{MessageCatalog, MessageId};

/// Pick a SPICE file to open as a deck; `None` when the picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_open_deck_dialog(messages: MessageCatalog) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter(
            messages.text(MessageId::NetlistDeckFilter),
            super::NETLIST_EXTENSIONS,
        )
        .add_filter(messages.text(MessageId::NetlistAllFilesFilter), &["*"])
        .set_title(messages.text(MessageId::NetlistOpenDeckTitle))
        .pick_file()
}

/// Ask where a new deck's file goes, starting in `folder`; `None` when the
/// picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_new_deck_dialog(
    folder: Option<&std::path::Path>,
    messages: MessageCatalog,
) -> Option<std::path::PathBuf> {
    show_deck_save_dialog(
        &messages.text(MessageId::NetlistNewDeckTitle),
        "deck.cir",
        folder,
        messages,
    )
}

/// Ask where a deck's file goes under `title`, offering `file_name` in
/// `folder`; `None` when the picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_deck_save_dialog(
    title: &str,
    file_name: &str,
    folder: Option<&std::path::Path>,
    messages: MessageCatalog,
) -> Option<std::path::PathBuf> {
    let dialog = rfd::FileDialog::new()
        .add_filter(
            messages.text(MessageId::NetlistDeckFilter),
            super::NETLIST_EXTENSIONS,
        )
        .set_title(title)
        .set_file_name(file_name);
    match folder {
        Some(folder) => dialog.set_directory(folder),
        None => dialog,
    }
    .save_file()
}

/// Ask where a ZIP goes under `title`, offering `file_name` in `folder`;
/// `None` when the picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_zip_save_dialog(
    title: &str,
    file_name: &str,
    folder: Option<&std::path::Path>,
    messages: MessageCatalog,
) -> Option<std::path::PathBuf> {
    let dialog = rfd::FileDialog::new()
        .add_filter(messages.text(MessageId::NetlistZipFilter), &["zip"])
        .set_title(title)
        .set_file_name(file_name);
    match folder {
        Some(folder) => dialog.set_directory(folder),
        None => dialog,
    }
    .save_file()
}

/// Ask for the file a line of a deck names as `requested`, offering its
/// name; `None` when the picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_add_deck_file_dialog(
    requested: &str,
    messages: MessageCatalog,
) -> Option<std::path::PathBuf> {
    let name = requested.rsplit(['/', '\\']).next().unwrap_or(requested);
    rfd::FileDialog::new()
        .add_filter(messages.text(MessageId::NetlistAllFilesFilter), &["*"])
        .set_title(messages.format(MessageId::NetlistAddFileTitle, &[("name", name)]))
        .set_file_name(name)
        .pick_file()
}

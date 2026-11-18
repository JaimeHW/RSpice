//! The desktop's deck pickers.

/// Pick a SPICE file to open as a deck; `None` when the picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_open_deck_dialog() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter(super::NETLIST_FILTER.0, super::NETLIST_FILTER.1)
        .add_filter("All Files", &["*"])
        .set_title("Open Deck")
        .pick_file()
}

/// Ask where a new deck's file goes, starting in `folder`; `None` when the
/// picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_new_deck_dialog(folder: Option<&std::path::Path>) -> Option<std::path::PathBuf> {
    show_deck_save_dialog("New Deck", "deck.cir", folder)
}

/// Ask where a deck's file goes under `title`, offering `file_name` in
/// `folder`; `None` when the picker was closed.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn show_deck_save_dialog(
    title: &str,
    file_name: &str,
    folder: Option<&std::path::Path>,
) -> Option<std::path::PathBuf> {
    let dialog = rfd::FileDialog::new()
        .add_filter(super::NETLIST_FILTER.0, super::NETLIST_FILTER.1)
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
) -> Option<std::path::PathBuf> {
    let dialog = rfd::FileDialog::new()
        .add_filter("ZIP archive", &["zip"])
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
pub fn show_add_deck_file_dialog(requested: &str) -> Option<std::path::PathBuf> {
    let name = requested.rsplit(['/', '\\']).next().unwrap_or(requested);
    rfd::FileDialog::new()
        .add_filter("All Files", &["*"])
        .set_title(format!("Add {name} to the Deck"))
        .set_file_name(name)
        .pick_file()
}

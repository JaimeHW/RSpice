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
    let dialog = rfd::FileDialog::new()
        .add_filter(super::NETLIST_FILTER.0, super::NETLIST_FILTER.1)
        .set_title("New Deck")
        .set_file_name("deck.cir");
    match folder {
        Some(folder) => dialog.set_directory(folder),
        None => dialog,
    }
    .save_file()
}

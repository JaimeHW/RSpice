//! The desktop's deck picker.

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

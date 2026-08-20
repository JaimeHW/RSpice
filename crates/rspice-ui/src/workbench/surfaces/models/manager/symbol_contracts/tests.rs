//! What the symbol-contract cache must notice.
//!
//! A stale row here is not a cosmetic lag: it states which pins a symbol has
//! and whether they match the provider's, so serving one for artwork that has
//! since been replaced is a wrong verdict rather than an old one.

use super::*;
use crate::state::{Cell, Library, LibraryManager, View};

/// A one-symbol design catalog whose symbol declares `family`.
///
/// Both catalogs this file builds take the same two mutations — one
/// `add_library`, one `get_library_mut` — so they carry the same content
/// revision, which is the point.
fn symbol_catalog(family: &str) -> LibraryManager {
    let mut libraries = LibraryManager::new();
    libraries.add_library(Library::new("sym"));
    let library = libraries
        .get_library_mut("sym")
        .expect("the library was just added");
    let mut cell = Cell::new("device");
    cell.metadata
        .insert("model.family".to_owned(), family.to_owned());
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    libraries
}

/// The families one frame's symbol registry reports, painted on `ctx` so that
/// successive calls see the same cache the real page would.
fn painted_families(ctx: &egui::Context, state: &mut AppState) -> Vec<String> {
    let mut families = Vec::new();
    let mut pending_actions = Vec::new();
    let app = ManagerRenderContext {
        state,
        pending_actions: &mut pending_actions,
    };
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            families = symbol_rows(ui, &app)
                .into_iter()
                .map(|row| row.family)
                .collect();
        });
    });
    families
}

#[test]
fn repainting_an_untouched_catalog_reads_no_symbol_twice() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state.library_manager = symbol_catalog("nch");

    crate::state::SYMBOL_VIEW_PARSES.with(|count| count.set(0));
    assert_eq!(painted_families(&ctx, &mut state), vec!["nch".to_owned()]);
    assert_eq!(
        crate::state::SYMBOL_VIEW_PARSES.with(std::cell::Cell::get),
        1,
        "the first frame is the one that reads the corpus"
    );

    crate::state::SYMBOL_VIEW_PARSES.with(|count| count.set(0));
    assert_eq!(painted_families(&ctx, &mut state), vec!["nch".to_owned()]);
    assert_eq!(
        crate::state::SYMBOL_VIEW_PARSES.with(std::cell::Cell::get),
        0,
        "nothing changed, so the second frame must read the cache"
    );
}

#[test]
fn a_catalog_replaced_at_the_same_revision_is_still_re_read() {
    // The hazard the key is content for. Opening a project, accepting a
    // recovery comparison, and restoring a design-history candidate all replace
    // the whole `LibraryManager` with one carrying whatever revision counter it
    // was serialized with — and two catalogs assembled by the same number of
    // mutations carry the same counter. A key made of the counter alone would
    // paint the first catalog's symbol contracts over the second's artwork.
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state.library_manager = symbol_catalog("nch");
    assert_eq!(painted_families(&ctx, &mut state), vec!["nch".to_owned()]);

    let replacement = symbol_catalog("pch");
    assert_eq!(
        replacement.revision(),
        state.library_manager.revision(),
        "the two catalogs must be indistinguishable by revision, or this test \
         proves nothing about the content half of the key"
    );
    state.library_manager = replacement;

    assert_eq!(painted_families(&ctx, &mut state), vec!["pch".to_owned()]);
}

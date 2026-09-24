//! What the registry must still do at the size of a real technology library.
//!
//! The table browses two corpora with a band between them, which is the one
//! shape a uniform-pitch virtualized list cannot draw. The arithmetic that
//! replaced `show_rows` is tested here directly: it decides which entries a
//! frame builds at all, so an off-by-one in it is either a row that cannot be
//! reached or six hundred rows built at once.

use super::*;

/// A layout with `project` project rows, `technology` technology rows, and the
/// band between them.
fn layout(project: usize, technology: usize) -> RegistryLayout {
    RegistryLayout {
        rows: project + technology,
        band: (project > 0 && technology > 0).then(|| (project, "band".to_owned())),
    }
}

#[test]
fn every_entry_is_reachable_and_no_two_overlap() {
    // The offsets are closed-form rather than accumulated, which is what keeps
    // the layout free of per-row state at six hundred rows. The price is that
    // nothing sums them, so this does: every entry must start where the last
    // one ended, and the last must end at the height the scrollbar was given.
    for (project, technology) in [(0, 0), (0, 5), (5, 0), (1, 1), (12, 24), (300, 300)] {
        let layout = layout(project, technology);
        let mut expected = 0.0_f32;
        for entry in 0..layout.entries() {
            assert_eq!(
                layout.top(entry),
                expected,
                "entry {entry} of {project}+{technology} does not start where the \
                 previous one ended"
            );
            expected += if layout.row_of(entry).is_some() {
                SYMBOL_ROW_H
            } else {
                SYMBOL_BAND_H
            };
        }
        assert_eq!(
            layout.total_height(),
            expected,
            "the scrollbar extent of {project}+{technology} does not match the entries \
             under it"
        );
    }
}

#[test]
fn each_entry_maps_to_its_own_row_and_the_band_maps_to_none() {
    let layout = layout(2, 3);
    let mapped = (0..layout.entries())
        .map(|entry| layout.row_of(entry))
        .collect::<Vec<_>>();
    assert_eq!(
        mapped,
        vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
        "the band must occupy an entry of its own without consuming a row"
    );
}

#[test]
fn a_viewport_selects_the_entries_it_actually_covers() {
    // The window a frame builds. A viewport that resolved one entry short at
    // the top would paint a gap where a row is, and one that resolved past the
    // end would index out of bounds — the reason `entry_at` clamps.
    let layout = layout(12, 24);
    assert_eq!(
        layout.entry_at(-40.0),
        0,
        "above the top is the first entry"
    );
    assert_eq!(layout.entry_at(0.0), 0);
    assert_eq!(layout.entry_at(SYMBOL_ROW_H - 0.01), 0);
    assert_eq!(layout.entry_at(SYMBOL_ROW_H), 1);

    let band_top = 12.0 * SYMBOL_ROW_H;
    assert_eq!(layout.entry_at(band_top - 0.01), 11, "the last project row");
    assert_eq!(layout.entry_at(band_top), 12, "the band itself");
    assert_eq!(layout.entry_at(band_top + SYMBOL_BAND_H - 0.01), 12);
    assert_eq!(
        layout.entry_at(band_top + SYMBOL_BAND_H),
        13,
        "the first technology row starts immediately under the band"
    );
    assert_eq!(
        layout.entry_at(layout.total_height() + 1000.0),
        layout.entries(),
        "past the bottom clamps to the end of the entry list rather than \
         indexing past it"
    );
}

#[test]
fn a_screenful_of_a_six_hundred_symbol_registry_is_a_screenful() {
    // The corpus the scale fixture builds. `entry_at` is what stands between a
    // 600-row technology library and 600 painted rows per frame.
    let layout = layout(12, 600);
    let viewport_height = 900.0;
    let first = layout.entry_at(0.0);
    let last = (layout.entry_at(viewport_height) + 1).min(layout.entries());
    assert!(
        last - first <= (viewport_height / SYMBOL_ROW_H).ceil() as usize + 2,
        "a {viewport_height}px viewport selected {} of {} entries",
        last - first,
        layout.entries()
    );
}

#[test]
fn a_layout_with_only_one_corpus_has_no_band_and_uniform_offsets() {
    for (project, technology) in [(6, 0), (0, 6)] {
        let layout = layout(project, technology);
        assert_eq!(layout.entries(), 6);
        assert_eq!(layout.band_height(), 0.0);
        assert_eq!(layout.row_of(3), Some(3));
        assert_eq!(layout.top(3), 3.0 * SYMBOL_ROW_H);
    }
}

/// The three tones the STATUS column reads in.
#[test]
fn a_review_is_coloured_as_a_finding_rather_than_as_a_settled_state() {
    // The defect this replaced: the painter chose the colour by string-matching
    // `pin mismatch`, so `review` — a contract that would not parse — was
    // painted in the same dim grey as `bound`.
    let tokens = crate::ui::tokens::Tokens::new(
        crate::ui::tokens::Direction::Instrument,
        crate::ui::tokens::Mode::Dark,
        crate::ui::tokens::Density::Compact,
    );
    assert_eq!(
        status_color(&tokens, SymbolStatus::PinMismatch),
        tokens.color.err
    );
    assert_eq!(
        status_color(&tokens, SymbolStatus::Review),
        tokens.color.warn
    );
    assert_eq!(
        status_color(&tokens, SymbolStatus::Bound),
        tokens.color.text_dim
    );
    assert_eq!(
        status_color(&tokens, SymbolStatus::ReadOnly),
        tokens.color.text_dim
    );
    assert_ne!(
        status_color(&tokens, SymbolStatus::Review),
        status_color(&tokens, SymbolStatus::Bound),
        "a review is work and a bound symbol is not; the column cannot say \
         both in the same colour"
    );
}

/// RENDER EVIDENCE — writes PNGs of the page with symbols on it.
///
/// The QA harness seeds nothing, so the Symbols page rasterizes as its empty
/// state and none of the tables above appear in it. Two renders instead: one
/// project-only registry holding an aligned symbol, a symbol whose artwork
/// disagrees with its provider, and a legacy symbol with no typed contract —
/// the three states the registry has to tell apart — and one two-corpus
/// registry with the signed technology package bound, which is the only shape
/// that draws the group band.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_populated_symbols_page() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");

    for (name, bind_technology) in [
        ("egui-01b-symbols-populated.png", false),
        ("egui-01c-symbols-two-corpora.png", true),
    ] {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.models_page = crate::workbench::state::ModelsPage::Symbols;
        super::super::symbol_contracts::seed_symbol_registry(&mut app.state);
        if bind_technology {
            app.state
                .provision_test_project_symbol_technology_contract();
        }

        let canvas = crate::ui::raster::render(egui::vec2(1180.0, 900.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    super::super::super::show(ui, &mut app);
                });
        });
        let path = directory.join(name);
        let height = canvas.content_height().max(200);
        std::fs::write(&path, canvas.png(height)).expect("write png");
        writeln!(
            std::io::stderr(),
            "wrote {} ({}x{height})",
            path.display(),
            canvas.width()
        )
        .ok();
    }
}

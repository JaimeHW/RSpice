//! What adopting and saving do to a placed source, and what the dialog that
//! offers them states.
//!
//! The claims that matter are the ones a reader cannot check by eye: a
//! cross-family adoption really is a re-type that keeps both terminals on
//! their nets, one undo puts the old type *and* the old card back, a
//! cross-kind adoption is refused rather than converted, and the list a reader
//! walks never arrives on a row the primary would refuse.

/// The two waveforms are one instrument: the same plot width, the same left
/// and right edges and the same height, so a time on one row is directly above
/// the same time on the other.
#[test]
fn the_two_waveform_rows_are_one_instrument() {
    let frame = egui::Rect::from_min_size(
        egui::pos2(120.0, 40.0),
        egui::vec2(
            400.0,
            crate::properties::source_preview::instrument_height(2, 64.0) - 16.0,
        ),
    );
    let rows = crate::properties::source_preview::strip_rows(frame, 64.0, 2);
    assert_eq!(rows.len(), 2);
    assert!(
        (rows[0].plot.left() - rows[1].plot.left()).abs() <= 0.5
            && (rows[0].plot.right() - rows[1].plot.right()).abs() <= 0.5,
        "the plots do not share their x range: {:?} {:?}",
        rows[0].plot,
        rows[1].plot
    );
    assert!(
        (rows[0].plot.height() - rows[1].plot.height()).abs() <= 0.5,
        "the plots are different heights: {} {}",
        rows[0].plot.height(),
        rows[1].plot.height()
    );
    for row in &rows {
        let gap = row.plot.top() - row.caption.bottom();
        assert!(
            (0.0..=4.0).contains(&gap),
            "a caption sits {gap} points above the plot it names"
        );
        assert_eq!(
            row.caption.left(),
            row.plot.left(),
            "both captions start at the plot's own left edge"
        );
        for other in &rows {
            // Touching edges are not an overlap: a caption's floor is above
            // the plot's ceiling, which is the whole point of giving it a row.
            assert!(
                row.caption.bottom() <= other.plot.top()
                    || row.caption.top() >= other.plot.bottom(),
                "a caption is painted over a plot: {:?} over {:?}",
                row.caption,
                other.plot
            );
        }
    }
    let air = rows[1].plot.top() - rows[0].plot.bottom();
    assert!(
        air >= 30.0,
        "only {air} points separate the two plots: the divider, the caption and both traces \
         are stacked with no air between them"
    );
}

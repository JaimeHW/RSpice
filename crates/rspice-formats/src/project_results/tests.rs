use super::*;

/// The reader's accept set is the engine's own label vocabulary, so nothing the
/// engine can name is refused on write. This is the property that failed: the
/// reader carried a hand-copied subset, and every family that had outgrown it
/// wrote a project the reader would not take back.
#[test]
fn the_project_reader_accepts_every_label_the_engine_can_emit() {
    for label in rspice_core::circuit::OP_LABELS {
        require_static_label(label.as_str(), "device_op label")
            .unwrap_or_else(|error| panic!("{error}"));
    }
}

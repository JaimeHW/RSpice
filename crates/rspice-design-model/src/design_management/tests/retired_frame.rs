//! Sheets, defaults and presets saved with frame options the drawing-format
//! standards do not have still load, onto the standard frame, and are written
//! back without them.

use super::*;

/// A sheet as an earlier RSpice wrote it: today's default sheet with `edit`
/// applied to its wire form.
fn saved_with(edit: impl FnOnce(&mut serde_json::Value)) -> SchematicSheetFormat {
    let mut wire = serde_json::to_value(SchematicSheetFormat::default()).unwrap();
    edit(&mut wire);
    serde_json::from_value(wire).expect("a sheet saved with a retired option loads")
}

#[test]
fn registration_and_folding_flags_load_as_the_centring_marks_option() {
    for (registration, folding) in [(true, true), (true, false), (false, true), (false, false)] {
        let format = saved_with(|wire| {
            wire["marks"] = serde_json::json!({
                "registration": registration,
                "folding": folding,
            });
        });
        assert_eq!(format.marks.centring, registration);
        let written = serde_json::to_value(&format).unwrap();
        assert_eq!(
            written["marks"],
            serde_json::json!({ "centring": registration }),
            "neither retired flag is written back"
        );
    }
}

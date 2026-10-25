//! What a schematic's document policies are called wherever a reader chooses
//! one: Preferences, for new schematics, and the sheet inspector, for the
//! schematic open.

use crate::state::SchematicGridPitch;

/// What one schematic grid pitch measures on the printed sheet: "2.5 mm".
///
/// Derived from the sheet's calibration rather than spelled by hand, so that
/// every place a pitch is named agrees with the drawing it prints on.
pub fn grid_pitch_label(pitch: SchematicGridPitch) -> String {
    let micrometres = u64::from(pitch.canvas_grid_size().unsigned_abs())
        * rspice_schematic_editor::view::drawing_sheet::DRAWING_SHEET_WORLD_UNIT_UM;
    format!("{} mm", micrometres as f64 / 1_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_schematic_editor::view::drawing_sheet::DRAWING_SHEET_UNITS_PER_MM;

    /// A grid label is what the pitch measures on the sheet, so reading the
    /// label back through the sheet scale gives the pitch's own unit count.
    #[test]
    fn grid_labels_measure_the_pitch_on_the_printed_sheet() {
        use crate::state::SchematicGridPitch;
        for (pitch, label) in [
            (SchematicGridPitch::Mil50, "2.5 mm"),
            (SchematicGridPitch::Mil25, "1.25 mm"),
            (SchematicGridPitch::Metric, "1 mm"),
        ] {
            assert_eq!(grid_pitch_label(pitch), label);
            let millimetres = label
                .strip_suffix(" mm")
                .and_then(|value| value.parse::<f64>().ok())
                .expect("a grid label is millimetres");
            assert_eq!(
                millimetres * DRAWING_SHEET_UNITS_PER_MM,
                f64::from(pitch.canvas_grid_size()),
                "{label} disagrees with the sheet scale"
            );
        }
    }
}

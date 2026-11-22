//! What a schematic's document policies are called wherever a reader chooses
//! one: Preferences, for new schematics, and the sheet inspector, for the
//! schematic open.

use crate::state::{DrawingSheetDisplayUnit, SchematicGridPitch};

/// What one schematic grid pitch measures on the printed sheet: "2.5 mm",
/// and on a sheet that shows inches its length in inches as well, "2.5 mm
/// (0.098 in)". The pitch stays metric there: it is the pitch symbol pins
/// stand on, so an inch sheet changes how it reads, not the grid.
///
/// Derived from the sheet's calibration rather than spelled by hand, so that
/// every place a pitch is named agrees with the drawing it prints on.
pub fn grid_pitch_label(pitch: SchematicGridPitch, unit: DrawingSheetDisplayUnit) -> String {
    let micrometres = u64::from(pitch.canvas_grid_size().unsigned_abs())
        * rspice_schematic_editor::view::drawing_sheet::DRAWING_SHEET_WORLD_UNIT_UM;
    let millimetres = micrometres as f64 / 1_000.0;
    match unit {
        DrawingSheetDisplayUnit::Inches => {
            format!("{millimetres} mm ({:.3} in)", millimetres / 25.4)
        }
        DrawingSheetDisplayUnit::Millimetres | DrawingSheetDisplayUnit::Centimetres => {
            format!("{millimetres} mm")
        }
    }
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
            (SchematicGridPitch::Mm2_5, "2.5 mm"),
            (SchematicGridPitch::Mm1_25, "1.25 mm"),
            (SchematicGridPitch::Mm1, "1 mm"),
        ] {
            assert_eq!(
                grid_pitch_label(pitch, DrawingSheetDisplayUnit::Millimetres),
                label
            );
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

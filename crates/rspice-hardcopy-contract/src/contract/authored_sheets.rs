//! The setup an authored drawing sheet prints with.

use super::*;

impl HardcopySetup {
    /// Print authored drawing sheets as drawn: each page is the sheet itself,
    /// at its own size and orientation, at 1:1, with no margin.
    ///
    /// The sheet identifies itself. Its title block (ISO 7200, ASME Y14.1)
    /// names the project, the drawing, the revision and the sheet of the set,
    /// and its frame carries the zones, so the header, the ink legend and the
    /// provenance footer a plot or a report prints would only repeat the
    /// title block and take paper from the drawing. They are off here and
    /// stay on in [`HardcopySetup::default`] for plots, results and reports.
    /// Every one of them, like the frame, title block, zones and grid, stays
    /// a toggle.
    ///
    /// The page is the sheet, so there is nothing outside it to trim or to
    /// outline: crop marks and the paper edge are off.
    pub fn for_authored_sheets(media: Vec<AuthoredSheetMedia>) -> Result<Self, HardcopyError> {
        let orientation = match media.first().map(AuthoredSheetMedia::dimensions) {
            Some((width, height)) if width > height => Orientation::Landscape,
            _ => Orientation::Portrait,
        };
        let defaults = Self::default();
        let sheet = SchematicHardcopySetup::default();
        Self::try_new_with_schematic(
            PhysicalPageSetup::try_new(
                PaperSize::MatchAuthoredSheets(media),
                PageMargins::uniform(Length::ZERO),
                Bleed::None,
                orientation,
            )?,
            ScaleMode::EngineeringOneToOne,
            defaults.tiling(),
            defaults.render().clone(),
            DecorationSetup::try_new(false, false, false, Watermark::None)?,
            SchematicHardcopySetup::new(
                SchematicHardcopyExtent::AuthoredDrawingSheet,
                OutsideSheetContentPolicy::Ask,
                false,
                false,
                sheet.includes_border(),
                sheet.includes_title_block(),
                sheet.includes_zones(),
                sheet.includes_grid(),
            ),
            defaults.print_mapping().clone(),
        )
    }
}

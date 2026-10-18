//! Authoring a custom drawing-sheet format from a starting frame.
//!
//! A custom size is not just a width and a height: the drafting convention it
//! starts from decides margins, border, and title block. Those rules live with
//! the model because both the editor and the package contract's own tests need
//! to produce a format that will survive validation.

use crate::design_management::{
    AuthoredDrawingSheetSize, DRAWING_SHEET_MAX_ASPECT_RATIO, DRAWING_SHEET_MAX_EDGE_UM,
    DRAWING_SHEET_MIN_EDGE_UM, DrawingSheetBorderTemplate, DrawingSheetInheritance,
    DrawingSheetStandardSeries, DrawingSheetTitleBlockTemplate, DrawingSheetZones,
    SchematicSheetFormat,
};
use crate::primitives::SchematicPageOrientation;

/// The frame a custom size starts from: ISO 5457's, as an ISO A sheet has;
/// the ASME Y14.1 format, as an ANSI A sheet has; or none, with no title
/// block either.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StartingFrame {
    #[default]
    IsoA,
    AnsiA,
    None,
}

pub fn validate_dimensions(width_um: u64, height_um: u64) -> Result<(), String> {
    for (field, value) in [("Width", width_um), ("Height", height_um)] {
        if value < DRAWING_SHEET_MIN_EDGE_UM {
            return Err(format!("{field} is below the 50.8 mm minimum."));
        }
        if value > DRAWING_SHEET_MAX_EDGE_UM {
            return Err(format!("{field} exceeds the 2540 mm maximum."));
        }
    }
    let short = width_um.min(height_um);
    let long = width_um.max(height_um);
    if u128::from(long) > u128::from(short) * u128::from(DRAWING_SHEET_MAX_ASPECT_RATIO) {
        return Err(format!(
            "Aspect ratio exceeds the {}:1 limit; a sheet this narrow cannot carry a border or title block.",
            DRAWING_SHEET_MAX_ASPECT_RATIO
        ));
    }
    Ok(())
}

pub fn custom_format(
    name: &str,
    width_um: u64,
    height_um: u64,
    frame: StartingFrame,
) -> Result<SchematicSheetFormat, String> {
    validate_dimensions(width_um, height_um)?;
    let mut format = SchematicSheetFormat::try_custom(
        name,
        width_um,
        height_um,
        SchematicPageOrientation::Portrait,
    )
    .map_err(|error| error.to_string())?;
    format = format
        .try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::Explicit;
            match frame {
                StartingFrame::IsoA => {
                    draft.apply_border_template(DrawingSheetBorderTemplate::Standard);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Standard;
                }
                StartingFrame::AnsiA => {
                    // The size keeps the ASME Y14.1 format it was started on.
                    if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size
                    {
                        snapshot.started_from = Some(DrawingSheetStandardSeries::Ansi);
                    }
                    draft.apply_border_template(DrawingSheetBorderTemplate::Standard);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::AsmeY141;
                }
                StartingFrame::None => {
                    draft.apply_border_template(DrawingSheetBorderTemplate::None);
                    draft.zones = DrawingSheetZones::none();
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::None;
                }
            }
        })
        .map_err(|error| error.to_string())?;
    // A size started from ISO A takes ISO 7200's block and one started from
    // ANSI A takes ASME Y14.1's, whether or not its drawing area holds it: one
    // that cannot carries no block, and the sheet's properties say why.
    Ok(format)
}

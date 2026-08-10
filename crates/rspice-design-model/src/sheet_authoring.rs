//! Authoring a custom drawing-sheet format from a starting frame.
//!
//! A custom size is not just a width and a height: the drafting convention it
//! starts from decides margins, border, and title block. Those rules live with
//! the model because both the editor and the package contract's own tests need
//! to produce a format that will survive validation.

use crate::design_management::{
    DRAWING_SHEET_MAX_ASPECT_RATIO, DRAWING_SHEET_MAX_EDGE_UM, DRAWING_SHEET_MIN_EDGE_UM,
    DrawingSheetBorderTemplate, DrawingSheetInheritance, DrawingSheetMargins,
    DrawingSheetTitleBlockTemplate, DrawingSheetZones, SchematicSheetFormat,
};
use crate::primitives::SchematicPageOrientation;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StartingFrame {
    #[default]
    IsoA,
    AnsiA,
    Plain,
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
                    draft.margins = DrawingSheetMargins {
                        top_um: 10_000,
                        right_um: 10_000,
                        bottom_um: 10_000,
                        left_um: 20_000,
                    };
                    draft.apply_border_template(DrawingSheetBorderTemplate::Standard);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                }
                StartingFrame::AnsiA => {
                    draft.margins = DrawingSheetMargins {
                        top_um: 12_700,
                        right_um: 12_700,
                        bottom_um: 12_700,
                        left_um: 19_050,
                    };
                    draft.apply_border_template(DrawingSheetBorderTemplate::Standard);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                }
                StartingFrame::Plain => {
                    draft.margins = DrawingSheetMargins {
                        top_um: 10_000,
                        right_um: 10_000,
                        bottom_um: 10_000,
                        left_um: 10_000,
                    };
                    draft.apply_border_template(DrawingSheetBorderTemplate::Plain);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                }
                StartingFrame::None => {
                    draft.margins = DrawingSheetMargins::zero();
                    draft.apply_border_template(DrawingSheetBorderTemplate::None);
                    draft.zones = DrawingSheetZones::none();
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::None;
                }
            }
        })
        .map_err(|error| error.to_string())?;
    Ok(format)
}

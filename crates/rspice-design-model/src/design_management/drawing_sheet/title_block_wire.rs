//! A title block as sheets, defaults, presets and packages have written it.
//!
//! The product has retired some of what it once wrote, and every sheet and
//! package that names it still loads: the `compact` block reads as the
//! sheet's standard block, a block placed anywhere but upright in the lower
//! right corner, turned or offset, reads as placed there, and a declared
//! scale reads as none.

use super::*;

/// A title block as it was written, which names the retired `compact` block
/// among the templates, may place the block as RSpice no longer does, and
/// may declare a scale.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DrawingSheetTitleBlockWire {
    template: DrawingSheetTitleBlockTemplateWire,
    #[serde(default)]
    anchor: Option<RetiredTitleBlockAnchor>,
    #[serde(default)]
    rotation: Option<RetiredTitleBlockRotation>,
    #[serde(default)]
    offset_x_um: Option<i64>,
    #[serde(default)]
    offset_y_um: Option<i64>,
    #[serde(default)]
    scale: Option<RetiredDrawingSheetScale>,
    #[serde(default)]
    managed_template: Option<DrawingSheetManagedTemplateSnapshot>,
    fields: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DrawingSheetTitleBlockTemplateWire {
    /// Retired: a 130 mm block that predated ISO 7200's and ASME Y14.1's
    /// forms.
    Compact,
    #[serde(alias = "wide")]
    Standard,
    OrganizationManaged,
    None,
    #[serde(rename = "asme-y14-1")]
    AsmeY141,
}

/// Retired: the corner, or strip along the foot, a sheet once anchored its
/// title block in. Read so the sheets that name one load; every block now
/// stands in the lower right corner.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum RetiredTitleBlockAnchor {
    BottomRight,
    BottomLeft,
    BottomStrip,
    TopRight,
}

/// Retired: the quarter turn a sheet once gave its title block. Read so the
/// sheets that name one load; every block now stands upright.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum RetiredTitleBlockRotation {
    Upright,
    Clockwise90,
    CounterClockwise90,
}

/// Retired: the scale a sheet's title block once declared, before the block
/// stopped lettering one. Read so the sheets that declare one load, and so a
/// package sealed with one is respelled as written (see `sheet_package`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum RetiredDrawingSheetScale {
    NotToScale,
    Ratio {
        drawing_units: u32,
        reality_units: u32,
    },
}

impl DrawingSheetTitleBlockWire {
    /// The title block as the product draws it: a block written as the
    /// retired compact one is `convention`'s standard block, ISO 7200's or
    /// ASME Y14.1's, its field values and choices kept, every block stands
    /// upright in the lower right corner, wherever it was placed, and none
    /// declares a scale.
    pub(super) fn into_title_block(
        self,
        convention: DrawingSheetFrameConvention,
    ) -> DrawingSheetTitleBlock {
        // The retired placement and scale are read only to be let go.
        let _ = (
            self.anchor,
            self.rotation,
            self.offset_x_um,
            self.offset_y_um,
            self.scale,
        );
        let template = match self.template {
            DrawingSheetTitleBlockTemplateWire::Compact => {
                DrawingSheetTitleBlockTemplate::for_convention(convention)
            }
            DrawingSheetTitleBlockTemplateWire::Standard => {
                DrawingSheetTitleBlockTemplate::Standard
            }
            DrawingSheetTitleBlockTemplateWire::OrganizationManaged => {
                DrawingSheetTitleBlockTemplate::OrganizationManaged
            }
            DrawingSheetTitleBlockTemplateWire::None => DrawingSheetTitleBlockTemplate::None,
            DrawingSheetTitleBlockTemplateWire::AsmeY141 => {
                DrawingSheetTitleBlockTemplate::AsmeY141
            }
        };
        DrawingSheetTitleBlock {
            template,
            managed_template: self.managed_template,
            fields: self.fields,
        }
    }
}

impl<'de> Deserialize<'de> for DrawingSheetTitleBlock {
    /// A title block read without the sheet it belongs to: the retired
    /// compact block reads as ISO 7200's. A sheet's own reads as its
    /// format's standard block (see [`SchematicSheetFormat`]).
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        DrawingSheetTitleBlockWire::deserialize(deserializer)
            .map(|wire| wire.into_title_block(DrawingSheetFrameConvention::Iso5457))
    }
}

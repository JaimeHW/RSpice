//! A printed page carries the project's logo in its title block's owner
//! cell, as the PNG file it was imported as, clear of the owner's name; a
//! project without a logo prints none.

use super::title_lettering::title_block_scene;
use super::*;
use crate::sources::{SchematicHardcopySource, resolve_schematic_source};
use rspice_design::schematic::{document::SchematicDocument, owned::Schematic};
use rspice_design_model::design_management::{
    DrawingSheetInheritance, DrawingSheetOwnerLogo, DrawingSheetProjectSettings,
    drawing_sheet_title_layout_with_owner_logo,
};

/// A PNG file of a picture three times as wide as it is high.
fn wide_logo_png() -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, 6, 2);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&[0x40; 6 * 2 * 4]).unwrap();
    writer.finish().unwrap();
    bytes
}

/// The title block a blank schematic on `format` prints, in a project
/// named Acme Instruments whose logo is `logo`.
fn printed(format: &SchematicSheetFormat, logo: Option<&DrawingSheetOwnerLogo>) -> HardcopyScene {
    title_block_scene(&resolved(format, logo))
}

/// A blank schematic on `format` in a project named Acme Instruments whose
/// logo is `logo`, resolved for print.
fn resolved(
    format: &SchematicSheetFormat,
    logo: Option<&DrawingSheetOwnerLogo>,
) -> ResolvedHardcopyDocument {
    let schematic = Schematic::from_document(SchematicDocument::default());
    let project_default = format
        .try_update(|draft| draft.inheritance = DrawingSheetInheritance::ProjectDefault)
        .unwrap();
    let mut values = DrawingSheetProjectSettings::default().title_block_field_values;
    values.insert(
        DrawingSheetTitleFieldId::Organization,
        "Acme Instruments".to_owned(),
    );
    resolve_schematic_source(SchematicHardcopySource {
        identity: HardcopySourceIdentity::try_new(
            "logo-schematic",
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(0x4c4f_474f)).unwrap(),
            ObjectRevision::INITIAL,
            "Logo schematic",
        )
        .unwrap(),
        schematic: &schematic,
        selection: None,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: Some(&project_default),
        project_title_block_field_values: Some(&values),
        project_owner_logo: logo,
        scope: HardcopyScope::ActiveDocument,
        parameter_labels: Default::default(),
    })
    .unwrap()
}

#[test]
fn every_form_prints_the_owner_logo_in_its_owner_cell() {
    let logo = DrawingSheetOwnerLogo::from_png(wide_logo_png(), "Acme Instruments logo").unwrap();
    for (standard, template) in [
        (
            DrawingSheetStandard::IsoA4,
            DrawingSheetTitleBlockTemplate::Compact,
        ),
        (
            DrawingSheetStandard::IsoA4,
            DrawingSheetTitleBlockTemplate::Standard,
        ),
        (
            DrawingSheetStandard::AnsiB,
            DrawingSheetTitleBlockTemplate::AsmeY141,
        ),
    ] {
        let format =
            SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
                .try_update(|draft| draft.title_block.template = template)
                .unwrap();
        let rasters = |scene: &HardcopyScene| {
            scene
                .primitives()
                .iter()
                .filter_map(|primitive| match primitive {
                    ScenePrimitive::RasterImage {
                        rect,
                        png,
                        content_digest,
                        alternative_text,
                    } => Some((
                        *rect,
                        png.clone(),
                        *content_digest,
                        alternative_text.clone(),
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        assert!(rasters(&printed(&format, None)).is_empty(), "{template:?}");

        let scene = printed(&format, Some(&logo));
        let [(rect, png, digest, alternative_text)] = &rasters(&scene)[..] else {
            panic!("{template:?} prints no logo");
        };
        assert_eq!(png.as_slice(), logo.png(), "{template:?}");
        assert_eq!(*digest, logo.digest(), "{template:?}");
        assert_eq!(alternative_text, "Acme Instruments logo", "{template:?}");

        // The owner's cell on the page, which the plot draws from the paper's
        // top-left corner.
        let geometry = format.geometry().unwrap();
        let block = geometry.title_block.unwrap();
        let (layout, _) =
            drawing_sheet_title_layout_with_owner_logo(&format, &geometry, Some(&logo)).unwrap();
        let cell = layout.cell(DrawingSheetTitleFieldId::Organization).unwrap();
        let (left, top) = (block.x_um as u64 + cell.x_um, block.y_um as u64 + cell.y_um);
        let (x, y) = (rect.x.micrometres(), rect.y.micrometres());
        let (width, height) = (rect.width.micrometres(), rect.height.micrometres());
        assert!(
            x > left
                && y > top
                && x + width < left + cell.width_um
                && y + height < top + cell.height_um,
            "{template:?}: {rect:?} outside the owner cell at ({left}, {top})"
        );
        // The owner's name is printed clear of it: to its right, or above.
        let name = scene
            .primitives()
            .iter()
            .find_map(|primitive| match primitive {
                ScenePrimitive::Text { origin, text, .. } if text == "Acme Instruments" => {
                    Some(*origin)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("{template:?} prints no owner"));
        assert!(
            name.x.micrometres() >= x + width || name.y.micrometres() < y,
            "{template:?}: the name at {name:?} overlaps the logo {rect:?}"
        );
    }
}

//! A plotted title block is lettered at the heights the canvas letters it at.

use super::*;
use rspice_design_model::design_management::{
    drawing_sheet_mono_em_um, drawing_sheet_title_block_lettering,
};

/// Em of every caption and every value the title block plots.
fn plotted_title_ems(format: &SchematicSheetFormat) -> (Vec<u64>, Vec<u64>) {
    let source = resolved_blank_schematic(format);
    // The title block alone, so every monospace line is one of its captions.
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        false,
        false,
        false,
        true,
        false,
        false,
    );
    let scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        schematic,
        resolved_metadata(),
    )
    .unwrap();
    let mut captions = Vec::new();
    let mut values = Vec::new();
    for primitive in scene.primitives() {
        let ScenePrimitive::Text { font, size, .. } = primitive else {
            continue;
        };
        match font {
            SceneFont::Monospace => captions.push(size.micrometres()),
            SceneFont::MonospaceMedium => values.push(size.micrometres()),
            _ => {}
        }
    }
    (captions, values)
}

#[test]
fn plotted_title_capitals_stand_at_the_height_the_standard_names() {
    for (standard, template) in [
        (
            DrawingSheetStandard::IsoA4,
            DrawingSheetTitleBlockTemplate::Compact,
        ),
        (
            DrawingSheetStandard::IsoA3,
            DrawingSheetTitleBlockTemplate::Standard,
        ),
        (
            DrawingSheetStandard::IsoA1,
            DrawingSheetTitleBlockTemplate::OrganizationManaged,
        ),
        (
            DrawingSheetStandard::IsoA0,
            DrawingSheetTitleBlockTemplate::Wide,
        ),
    ] {
        let format =
            SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
                .try_update(|draft| draft.title_block.template = template)
                .unwrap();
        let lettering =
            drawing_sheet_title_block_lettering(&format, &format.geometry().unwrap()).unwrap();
        let (captions, values) = plotted_title_ems(&format);
        assert!(
            !values.is_empty() && captions.len() == values.len(),
            "{standard:?} {template:?} must plot one caption per value"
        );
        for (ems, height_um, role) in [
            (&captions, lettering.caption_um, "caption"),
            (&values, lettering.data_um, "value"),
        ] {
            for &em_um in ems {
                // The canvas sets the same em, so the two agree exactly.
                assert_eq!(
                    em_um,
                    drawing_sheet_mono_em_um(height_um),
                    "{standard:?} {template:?} plots a {role} at the wrong em"
                );
                // IBM Plex Mono's capitals are 698/1000 of the em.
                assert!(
                    em_um * 698 / 1_000 >= height_um - 1,
                    "{standard:?} {template:?} plots {role} capitals below {height_um} um"
                );
            }
        }
    }
}

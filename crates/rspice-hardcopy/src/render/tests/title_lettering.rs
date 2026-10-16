//! A plotted title block and its zone references are lettered at the heights
//! the canvas letters them at.

use super::*;
use rspice_design_model::design_management::{
    DrawingSheetBorderTemplate, drawing_sheet_mono_em_um, drawing_sheet_title_block_lettering,
    drawing_sheet_zone_lettering_um,
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

#[test]
fn plotted_zone_references_stand_centred_in_their_band() {
    for border in [
        DrawingSheetBorderTemplate::Standard,
        DrawingSheetBorderTemplate::Plain,
    ] {
        let format = SchematicSheetFormat::default()
            .try_update(|draft| draft.apply_border_template(border))
            .unwrap();
        let geometry = format.geometry().unwrap();
        let band_um = geometry.border_band_um;
        let em_um = drawing_sheet_mono_em_um(drawing_sheet_zone_lettering_um(band_um));
        let source = resolved_blank_schematic(&format);
        // The zone band alone, so every line of text is a reference.
        let schematic = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            false,
            false,
            true,
            false,
        );
        let scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            schematic,
            resolved_metadata(),
        )
        .unwrap();
        // The top band is bounded by its column rules, the highest of the
        // vertical rules one band deep.
        let (band_top_um, band_bottom_um) = scene
            .primitives()
            .iter()
            .filter_map(|primitive| {
                let ScenePrimitive::Line { from, to, .. } = primitive else {
                    return None;
                };
                let (from_y, to_y) = (from.y.micrometres(), to.y.micrometres());
                (from.x == to.x && from_y.abs_diff(to_y) == band_um)
                    .then(|| (from_y.min(to_y), from_y.max(to_y)))
            })
            .min()
            .expect("the top band is ruled");
        let mut top_references = 0;
        for primitive in scene.primitives() {
            let ScenePrimitive::Text {
                origin,
                font,
                size,
                anchor,
                ..
            } = primitive
            else {
                continue;
            };
            assert_eq!(*font, SceneFont::Monospace, "{border:?}");
            assert_eq!(size.micrometres(), em_um, "{border:?} plots at another em");
            assert_eq!(
                *anchor,
                TextAnchor::Middle,
                "{border:?} must centre a reference"
            );
            let baseline_um = origin.y.micrometres();
            if baseline_um > band_bottom_um {
                continue;
            }
            top_references += 1;
            // IBM Plex Mono's capitals are 698/1000 of the em: they must
            // stand inside the band, centred on it within a twentieth of the
            // em, as the canvas centres them.
            let capitals_top_um = baseline_um - em_um * 698 / 1_000;
            assert!(
                capitals_top_um >= band_top_um,
                "{border:?} references rise out of the band"
            );
            let capitals_centre_um = (capitals_top_um + baseline_um) / 2;
            assert!(
                capitals_centre_um.abs_diff((band_top_um + band_bottom_um) / 2) <= em_um / 20,
                "{border:?} references sit off the band's centre line"
            );
        }
        assert!(
            top_references > 0,
            "{border:?} plots references on its top edge"
        );
    }
}

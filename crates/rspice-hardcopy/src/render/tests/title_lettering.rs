//! A plotted title block and its zone references are lettered at the heights
//! the canvas letters them at.

use super::*;
use rspice_design_model::design_management::{
    DRAWING_SHEET_TITLE_BLOCK_LETTERING, DrawingSheetBorderTemplate,
    DrawingSheetTitleFieldValueAuthority, DrawingSheetTitleLettering,
    DrawingSheetTitleLetteringRole, drawing_sheet_mono_em_um, drawing_sheet_title_layout,
    drawing_sheet_zone_lettering_um, resolve_drawing_sheet_title_fields,
};

/// Every line the title block plots, with the face and em it is set in.
fn plotted_title_lines(format: &SchematicSheetFormat) -> Vec<(SceneFont, u64, String)> {
    plotted_lines_of(&resolved_blank_schematic(format))
}

/// A blank schematic on `format`, with `values` for the title fields its
/// project and document control supply.
fn blank_schematic_titled(
    format: &SchematicSheetFormat,
    values: BTreeMap<DrawingSheetTitleFieldId, String>,
) -> ResolvedHardcopyDocument {
    let identity = HardcopySourceIdentity::try_new(
        "test-schematic",
        HardcopyDocumentId::try_from_uuid(Uuid::from_u128(0x5343_4845_4d41)).unwrap(),
        ObjectRevision::INITIAL,
        "Test schematic",
    )
    .unwrap();
    let semantic = SemanticSchematic {
        view_path: identity.source_key.clone(),
        drawing_sheet: Some(format.clone()),
        drawing_sheet_title_values: values,
        drawing_sheet_page_numbering: None,
        grid_pitch_units: 10,
        components: Vec::new(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    };
    let digest = rspice_hardcopy_contract::sources::canonical_digest(
        b"rspice-hardcopy-blank-schematic-sheet-v1",
        &semantic,
    )
    .unwrap();
    resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::CurrentSheet,
        HardcopySemanticDocument::Schematic(Box::new(semantic)),
        drawing_sheet_artwork_bounds(format).unwrap(),
    )
    .unwrap()
}

/// Every line the title block of `source` plots.
fn plotted_lines_of(source: &ResolvedHardcopyDocument) -> Vec<(SceneFont, u64, String)> {
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
        source,
        source.default_print_mapping(),
        schematic,
        resolved_metadata(),
    )
    .unwrap();
    scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Text {
                font: font @ (SceneFont::Monospace | SceneFont::MonospaceMedium),
                size,
                text,
                ..
            } => Some((*font, size.micrometres(), text.clone())),
            _ => None,
        })
        .collect()
}

/// `format` with every authored field it shows filled in.
fn filled_in(format: &SchematicSheetFormat) -> SchematicSheetFormat {
    format
        .try_update(|draft| {
            for (field, state) in &mut draft.title_block.fields {
                if field.policy().value_authority == DrawingSheetTitleFieldValueAuthority::Authored
                    && state.value.is_empty()
                {
                    state.value = "Filled in".to_owned();
                }
            }
        })
        .unwrap()
}

/// Every line the title block of `format` letters by the layout, with the
/// values a blank schematic supplies, as `resolved_blank_schematic`
/// captures them.
fn laid_out_lettering(format: &SchematicSheetFormat) -> Vec<DrawingSheetTitleLettering> {
    let layout = drawing_sheet_title_layout(format, &format.geometry().unwrap()).unwrap();
    let identity = HardcopySourceIdentity::try_new(
        "test-schematic",
        HardcopyDocumentId::try_from_uuid(Uuid::from_u128(0x5343_4845_4d41)).unwrap(),
        ObjectRevision::INITIAL,
        "Test schematic",
    )
    .unwrap();
    let fields = resolve_drawing_sheet_title_fields(
        format,
        &drawing_sheet_title_values(&identity, format, None, None),
    );
    layout.lettering(&fields)
}

/// Every line the title block of `format` letters by the layout, with the
/// em and weight it is set in and its text, in the order it is lettered.
fn laid_out_title_lines(format: &SchematicSheetFormat) -> Vec<(SceneFont, u64, String)> {
    laid_out_lettering(format)
        .into_iter()
        .map(|line| {
            (
                if line.emphasised {
                    SceneFont::MonospaceMedium
                } else {
                    SceneFont::Monospace
                },
                drawing_sheet_mono_em_um(line.lettering_um),
                line.text,
            )
        })
        .collect()
}

#[test]
fn only_the_document_identifier_is_plotted_in_the_heavier_weight() {
    // ISO 7200 emphasises the identification number alone; every other value
    // stands out from its caption by its height.
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::DocumentId)
                .unwrap()
                .value = "PSAFE-SCH-001".to_owned();
        })
        .unwrap();
    let heavier = plotted_title_lines(&format)
        .into_iter()
        .filter(|(font, _, _)| *font == SceneFont::MonospaceMedium)
        .map(|(_, _, text)| text)
        .collect::<Vec<_>>();
    assert_eq!(heavier, ["PSAFE-SCH-001"]);

    // A PDF letters that line from the embedded Medium cut, and a sheet that
    // does not show the identifier embeds no Medium cut at all.
    let embedded_faces = |format: &SchematicSheetFormat| {
        let source = resolved_blank_schematic(format);
        let setup = setup_with_schematic_output(
            OutputFormat::PdfVector,
            SchematicHardcopySetup::new(
                SchematicHardcopyExtent::AuthoredDrawingSheet,
                OutsideSheetContentPolicy::Ask,
                false,
                false,
                false,
                true,
                false,
                false,
            ),
        );
        let plan =
            HardcopyPlan::compile(source.authority().clone(), setup, source.content_extent())
                .unwrap();
        let pdf = HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
        let parsed = ParsedPdf::load_mem(pdf.single_part().unwrap().bytes()).unwrap();
        parsed
            .objects
            .values()
            .filter_map(|object| object.as_dict().ok())
            .filter_map(|dictionary| dictionary.get(b"BaseFont").ok())
            .filter_map(|name| name.as_name().ok())
            .map(|name| String::from_utf8_lossy(name).into_owned())
            .collect::<Vec<_>>()
    };
    let shown = embedded_faces(&format);
    assert!(
        shown
            .iter()
            .any(|face| face.ends_with("IBMPlexMono-Medium")),
        "the identifier must be set in the Medium cut: {shown:?}"
    );
    // ISO 7200's form always carries the identifier, so the sheet that does
    // not show it is one on the compact block.
    let hidden = format
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::DocumentId)
                .unwrap()
                .visible = false;
        })
        .unwrap();
    let without = embedded_faces(&hidden);
    assert!(
        !without.iter().any(|face| face.contains("Medium")),
        "only the identifier is set in the Medium cut: {without:?}"
    );
    assert!(DrawingSheetTitleFieldId::DocumentId.emphasised());
    assert_eq!(
        DrawingSheetTitleFieldId::ALL
            .into_iter()
            .filter(|field| field.emphasised())
            .count(),
        1
    );
}

#[test]
fn a_plotted_caption_letters_the_field_name_and_nothing_else() {
    // ISO 7200 marks no field as filled in by the system, so the caption of
    // a field RSpice fills in reads exactly as an authored field's does, in
    // the standard's own words.
    let format = SchematicSheetFormat::default();
    let caption_em = drawing_sheet_mono_em_um(DRAWING_SHEET_TITLE_BLOCK_LETTERING.caption_um);
    let captions = plotted_title_lines(&format)
        .into_iter()
        .filter(|(_, em_um, _)| *em_um == caption_em)
        .map(|(_, _, text)| text)
        .collect::<Vec<_>>();
    assert_eq!(
        captions,
        [
            "Created by",
            "Approved by",
            "Legal owner",
            "Document type",
            "Document status",
            "Title",
            "Identification number",
            "Rev.",
            "Date of issue",
            "Sheet",
        ]
    );
}

#[test]
fn the_plot_letters_exactly_the_lines_the_layout_letters() {
    for template in [
        DrawingSheetTitleBlockTemplate::Standard,
        DrawingSheetTitleBlockTemplate::Compact,
        DrawingSheetTitleBlockTemplate::OrganizationManaged,
    ] {
        let format = filled_in(
            &SchematicSheetFormat::default()
                .try_update(|draft| draft.title_block.template = template)
                .unwrap(),
        );
        assert_eq!(
            plotted_title_lines(&format),
            laid_out_title_lines(&format),
            "{template:?}"
        );
    }
}

#[test]
fn the_title_block_is_outlined_wide_and_ruled_narrow() {
    // ISO 128-20's wide and narrow lines: a 0.7 mm outline and 0.35 mm rules
    // between the cells, each rule plotted once.
    let format = SchematicSheetFormat::default();
    let source = resolved_blank_schematic(&format);
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
    let outlines = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Rect {
                stroke: Some(stroke),
                ..
            } => Some(stroke.width.micrometres()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(outlines, [700]);
    let rules = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Line { stroke, .. } => Some(stroke.width.micrometres()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let layout = drawing_sheet_title_layout(&format, &format.geometry().unwrap()).unwrap();
    assert_eq!(rules.len(), layout.rules.len());
    assert!(rules.iter().all(|width| *width == 350), "{rules:?}");
}

#[test]
fn an_empty_authored_field_plots_its_caption_alone() {
    // A title block is a form: Created by and Approved by start on a sheet
    // empty, boxes to sign on the print, with no dash in them.
    let lines = plotted_title_lines(&SchematicSheetFormat::default());
    let value_em = drawing_sheet_mono_em_um(DRAWING_SHEET_TITLE_BLOCK_LETTERING.data_um);
    assert!(
        lines.iter().all(|(_, _, text)| !text.trim().is_empty()),
        "no line of the block is blank: {lines:?}"
    );
    let after = |caption: &str| {
        let at = lines
            .iter()
            .position(|(_, _, text)| text == caption)
            .unwrap_or_else(|| panic!("{caption} is plotted: {lines:?}"));
        lines.get(at + 1).cloned()
    };
    for caption in ["Created by", "Approved by"] {
        assert!(
            after(caption).is_none_or(|(_, em_um, _)| em_um != value_em),
            "{caption} plots a value: {:?}",
            after(caption)
        );
    }
    // A date RSpice does not have yet still reads as not yet known.
    assert_eq!(
        after("Date of issue").map(|(_, _, text)| text).as_deref(),
        Some("—")
    );
}

#[test]
fn a_new_sheet_plots_typical_title_values_whole() {
    // Each cell is as wide as what its field typically holds, so a new
    // A4 sheet plots a typical identity, responsibility and status with
    // nothing cut short.
    let authored = [
        (
            DrawingSheetTitleFieldId::SheetTitle,
            "Bandgap reference core",
        ),
        (DrawingSheetTitleFieldId::DrawnBy, "J. Whitfield"),
        (DrawingSheetTitleFieldId::ApprovedBy, "J. Whitfield"),
    ];
    let supplied = [
        (
            DrawingSheetTitleFieldId::CellView,
            "mycircuit/top/schematic",
        ),
        (DrawingSheetTitleFieldId::Page, "12 of 12"),
        (DrawingSheetTitleFieldId::Revision, "B"),
        (DrawingSheetTitleFieldId::Date, "2026-10-18"),
        (DrawingSheetTitleFieldId::DocumentStatus, "Under approval"),
        (DrawingSheetTitleFieldId::DocumentId, "RS-0001-SCH-0042"),
    ];
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            // ISO 7200's form has no project field, so asking for one
            // letters nothing.
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::Project)
                .unwrap()
                .visible = true;
            for (field, value) in authored {
                draft.title_block.fields.get_mut(&field).unwrap().value = value.to_owned();
            }
        })
        .unwrap();
    let source = blank_schematic_titled(
        &format,
        supplied
            .into_iter()
            .chain([
                (DrawingSheetTitleFieldId::Project, "MyCircuit"),
                (
                    DrawingSheetTitleFieldId::Organization,
                    "Whitfield Analog Devices",
                ),
            ])
            .map(|(field, value)| (field, value.to_owned()))
            .collect(),
    );
    let text = plotted_lines_of(&source)
        .into_iter()
        .map(|(_, _, text)| text)
        .collect::<Vec<_>>();
    assert!(
        !text.iter().any(|line| line.contains('…')),
        "the plot cuts a typical value short: {text:?}"
    );
    for (_, value) in authored.into_iter().chain(supplied) {
        assert!(text.iter().any(|line| line == value), "{value:?}: {text:?}");
    }
    assert!(text.iter().any(|line| line == "Circuit diagram"));
    assert!(!text.iter().any(|line| line == "MyCircuit"));
    // The legal owner's cell is large so that its name wraps rather than
    // being cut.
    let owner = text
        .iter()
        .position(|line| line == "Whitfield Analog")
        .expect("the legal owner is plotted");
    assert_eq!(text[owner + 1], "Devices");
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
            DrawingSheetTitleBlockTemplate::Standard,
        ),
    ] {
        let format =
            SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
                .try_update(|draft| draft.title_block.template = template)
                .unwrap();
        // ISO 7200 uses one title block for every paper size, so an A0 block
        // letters what an A4 one does: 1.8 mm captions, 2.5 mm values, and on
        // the form a 3.5 mm identification number.
        let filled = filled_in(&format);
        let laid_out = laid_out_lettering(&filled);
        let plotted = plotted_title_lines(&filled);
        assert_eq!(plotted.len(), laid_out.len(), "{standard:?} {template:?}");
        assert!(
            laid_out
                .iter()
                .any(|line| line.role == DrawingSheetTitleLetteringRole::Value),
            "{standard:?} {template:?} plots values"
        );
        let lettering = DRAWING_SHEET_TITLE_BLOCK_LETTERING;
        for ((_, em_um, text), line) in plotted.iter().zip(&laid_out) {
            let height_um = match line.role {
                DrawingSheetTitleLetteringRole::Caption => lettering.caption_um,
                DrawingSheetTitleLetteringRole::Value
                    if line.emphasised && template == DrawingSheetTitleBlockTemplate::Standard =>
                {
                    3_500
                }
                DrawingSheetTitleLetteringRole::Value => lettering.data_um,
            };
            // The canvas sets the same em, so the two agree exactly.
            assert_eq!(
                *em_um,
                drawing_sheet_mono_em_um(height_um),
                "{standard:?} {template:?} plots {text:?} at the wrong em"
            );
            // IBM Plex Mono's capitals are 698/1000 of the em.
            assert!(
                em_um * 698 / 1_000 >= height_um - 1,
                "{standard:?} {template:?} plots {text:?} capitals below {height_um} um"
            );
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
        let em_um = drawing_sheet_mono_em_um(drawing_sheet_zone_lettering_um(
            format.frame_convention(),
            band_um,
        ));
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

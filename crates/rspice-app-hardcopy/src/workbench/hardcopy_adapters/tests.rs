//! Application source resolution and headless renderer integration fixtures.

use rspice_hardcopy::render::{HardcopyRenderError, HardcopySceneMetadata};
#[cfg(test)]
use rspice_hardcopy::render::{ScenePrimitive, scene_from_resolved};

use rspice_hardcopy::sources::ResolvedHardcopyDocument;

pub fn source_metadata(
    source: &ResolvedHardcopyDocument,
    creator: impl Into<String>,
) -> Result<HardcopySceneMetadata, HardcopyRenderError> {
    let mut metadata = HardcopySceneMetadata::try_new(source.authority().display_name(), creator)?;
    metadata.set_provenance_lines(vec![format!(
        "source {} · document {} · revision {} · digest {}",
        source.source_key(),
        source.authority().document_id(),
        source.authority().revision().get(),
        source.authority().content_digest()
    )])?;
    Ok(metadata)
}

#[cfg(test)]
use super::sources::{
    HardcopySemanticDocument, SchematicHardcopySource, resolve_hardcopy_source_set_with,
    resolve_schematic_source, schematic_sheet_identity, source_set_member_from_resolved,
};
#[cfg(test)]
use crate::product::ObjectRevision;
#[cfg(test)]
use crate::state::{
    DrawingSheetTitleFieldId, SchematicState, SheetCatalog, SheetDefinition, SheetPageNumbering,
    SheetPortPolicy, SheetTemplate,
};
#[cfg(test)]
use rspice_hardcopy_contract::sources::{HardcopySourceIdentity, HardcopySourceSet};
#[cfg(test)]
use rspice_hardcopy_contract::{
    HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope, SchematicHardcopySetup,
};

#[test]
fn named_print_set_projects_per_set_sheet_numbers_without_mutating_child_authority() {
    let sheet = |name: &str| SheetDefinition {
        name: name.to_owned(),
        template: SheetTemplate::AnalogSchematic,
        port_policy: SheetPortPolicy::TypedOffSheetPorts,
        explicit_page_number: None,
    };
    let schematic = SchematicState::default();
    let mut catalog = SheetCatalog::default();
    let first = catalog.create_sheet(sheet("First"), None).unwrap();
    let second = catalog.create_sheet(sheet("Second"), Some(first)).unwrap();
    let third = catalog.create_sheet(sheet("Third"), Some(second)).unwrap();
    let mut settings = catalog.settings().clone();
    settings.page_numbering = SheetPageNumbering::PerPrintSet;
    catalog.set_settings(catalog.revision(), settings).unwrap();
    let project_settings = crate::state::DrawingSheetProjectSettings::default();
    let base_identity = HardcopySourceIdentity::try_new(
        "named-set-schematic",
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Active document",
    )
    .unwrap();
    let mut selected = [second, third]
        .into_iter()
        .map(|sheet_id| {
            resolve_schematic_source(SchematicHardcopySource {
                identity: schematic_sheet_identity(&base_identity, catalog.find(sheet_id).unwrap())
                    .unwrap(),
                schematic: schematic.editor_ref().design,
                selection: None,
                expected_topology_version: schematic.topology_version(),
                symbol_resolver: None,
                sheet_catalog: Some(&catalog),
                sheet_id: Some(sheet_id),
                project_default_drawing_sheet: Some(&project_settings.default_format),
                project_title_block_field_values: Some(&project_settings.title_block_field_values),
                scope: HardcopyScope::CurrentSheet,
            })
            .unwrap()
        })
        .collect::<Vec<_>>();
    for (resolved, expected) in selected.iter().zip(["2 of 3", "3 of 3"]) {
        let HardcopySemanticDocument::Schematic(schematic) = resolved.semantic_document() else {
            panic!("expected governed schematic sheet")
        };
        assert_eq!(
            schematic
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Page)
                .map(String::as_str),
            Some(expected),
            "the retained child keeps its catalog-relative source semantics"
        );
    }
    let members = selected
        .iter()
        .map(source_set_member_from_resolved)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Release subset",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::NamedPrintSet("Release subset".to_owned()),
        members,
    )
    .unwrap();
    let mut selected = selected.drain(..);
    let aggregate = resolve_hardcopy_source_set_with(&source_set, |_| {
        Ok(selected.next().expect("one exact retained set member"))
    })
    .unwrap();
    let HardcopySemanticDocument::Aggregate(semantic) = aggregate.semantic_document() else {
        panic!("expected named aggregate")
    };
    assert_eq!(
        semantic
            .children
            .iter()
            .map(|child| child.publication_page_label.as_deref())
            .collect::<Vec<_>>(),
        [Some("1 of 2"), Some("2 of 2")]
    );

    let scene = scene_from_resolved(
        &aggregate,
        aggregate.default_print_mapping(),
        SchematicHardcopySetup::default(),
        source_metadata(&aggregate, "RSpice test").unwrap(),
    )
    .unwrap();
    let printed_text = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Text { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(printed_text.contains(&"1 of 2"));
    assert!(printed_text.contains(&"2 of 2"));
    assert!(!printed_text.contains(&"2 of 3"));
    assert!(!printed_text.contains(&"3 of 3"));
}

/// The canvas and the plot letter a sheet's title block alike, whatever
/// script it is written in: the canvas lays its lines out by the width it
/// measures, the plot by the width print measures, and the two must cut
/// every value at the same place and set it against the same edge.
#[test]
fn the_canvas_and_the_plot_letter_a_title_in_any_script_alike() {
    use crate::state::{
        DrawingSheetTitleAlign, DrawingSheetTitleFieldValueAuthority,
        DrawingSheetTitleLetteringRole, SchematicSheetFormat, drawing_sheet_title_layout,
        resolve_drawing_sheet_title_fields,
    };
    use rspice_hardcopy::render::TextAnchor;
    use rspice_hardcopy::sources::{
        SemanticSchematic, authored_sheet_bounds, drawing_sheet_title_values,
        resolve_semantic_source,
    };
    use rspice_schematic_editor::view::drawing_sheet::DrawingSheetTitleWidth;

    // Arabic with its vowel marks, Hebrew with a number, a unit and
    // brackets, Thai with its tone marks, Japanese and Latin, each long
    // enough to be cut.
    let mut values = [
        "\u{645}\u{64e}\u{631}\u{652}\u{62d}\u{64e}\u{628}\u{64b}\u{627} \u{628}\u{650}\u{643}\u{64f}\u{645}\u{652} ",
        "\u{5de}\u{5d2}\u{5d1}\u{5e8} R1 = 10 k\u{3a9} (V+) ",
        "\u{e2a}\u{e27}\u{e31}\u{e2a}\u{e14}\u{e35}\u{e04}\u{e23}\u{e31}\u{e1a} ",
        "\u{65e5}\u{672c}\u{8a9e}\u{306e}\u{56de}\u{8def}\u{56f3} ",
        "Whitfield Analog Devices ",
    ]
    .map(|phrase| phrase.repeat(6).trim_end().to_owned())
    .into_iter();
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            for (field, state) in &mut draft.title_block.fields {
                if field.policy().value_authority == DrawingSheetTitleFieldValueAuthority::Authored
                    && !field.is_signature_date()
                    && state.visible
                    && let Some(value) = values.next()
                {
                    state.value = value;
                }
            }
        })
        .unwrap();
    assert_eq!(values.next(), None, "every script has a field");

    // The canvas's lines.
    let identity = HardcopySourceIdentity::try_new(
        "title-scripts",
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Title scripts",
    )
    .unwrap();
    let title_values = drawing_sheet_title_values(&identity, &format, None, None);
    let geometry = format.geometry().unwrap();
    let layout = drawing_sheet_title_layout(&format, &geometry).unwrap();
    let canvas = layout.lettering_measured(
        &resolve_drawing_sheet_title_fields(&format, &title_values),
        &DrawingSheetTitleWidth,
    );
    assert!(canvas.iter().any(|line| line.text.ends_with('…')));
    assert!(
        canvas
            .iter()
            .any(|line| line.align == DrawingSheetTitleAlign::Right)
    );

    // The plot's lines.
    let semantic = SemanticSchematic {
        view_path: identity.source_key.clone(),
        drawing_sheet: Some(format.clone()),
        drawing_sheet_title_values: title_values,
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
    let digest =
        rspice_hardcopy_contract::sources::canonical_digest(b"rspice-title-scripts", &semantic)
            .unwrap();
    let resolved = resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::CurrentSheet,
        HardcopySemanticDocument::Schematic(Box::new(semantic)),
        authored_sheet_bounds(&format).unwrap(),
    )
    .unwrap();
    let scene = scene_from_resolved(
        &resolved,
        resolved.default_print_mapping(),
        SchematicHardcopySetup::default(),
        source_metadata(&resolved, "RSpice test").unwrap(),
    )
    .unwrap();
    // The block's outline, from whose left edge each line is placed.
    let block = geometry.title_block.unwrap();
    let block_x_um = scene
        .primitives()
        .iter()
        .find_map(|primitive| match primitive {
            ScenePrimitive::Rect { rect, .. }
                if rect.width.micrometres() == block.width_um
                    && rect.height.micrometres() == block.height_um =>
            {
                Some(rect.x.micrometres())
            }
            _ => None,
        })
        .expect("the plot outlines the title block");
    let plotted = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Text {
                origin,
                text,
                anchor,
                ..
            } => Some((text.as_str(), origin.x.micrometres(), *anchor)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let first = plotted
        .iter()
        .position(|(text, ..)| *text == canvas[0].text)
        .expect("the plot letters the block");
    let plotted = &plotted[first..first + canvas.len()];

    // The same lines, cut at the same place, each against the same edge.
    for ((text, x_um, anchor), line) in plotted.iter().zip(&canvas) {
        assert_eq!(*text, line.text);
        let (edge_um, expected) = match line.align {
            DrawingSheetTitleAlign::Left => (line.x_um, TextAnchor::Start),
            DrawingSheetTitleAlign::Right => (line.x_um + line.width_um, TextAnchor::End),
        };
        assert_eq!(*anchor, expected, "{text:?}");
        assert!(
            (x_um - block_x_um).abs_diff(edge_um) <= 50,
            "{text:?} is plotted {} um from the block's left edge; the canvas sets it at {edge_um} um",
            x_um - block_x_um
        );
    }
    assert!(
        canvas
            .iter()
            .filter(|line| line.role == DrawingSheetTitleLetteringRole::Value)
            .count()
            >= 5
    );
}

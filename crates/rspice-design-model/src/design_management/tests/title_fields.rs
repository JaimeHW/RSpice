//! The fields ISO 7200 gives a title block, and the sheets and sealed
//! organization templates written before the field set last changed.

use super::*;

#[test]
fn paper_size_names_the_sheet_as_iso_7200_writes_it() {
    let field = DrawingSheetTitleFieldId::PaperSize;
    assert!(DrawingSheetTitleFieldId::ALL.contains(&field));
    assert_eq!(field.display_label(), "Paper size");
    assert!(!field.policy().required_visible);
    assert_eq!(
        field.policy().value_authority,
        DrawingSheetTitleFieldValueAuthority::Automatic
    );
    assert!(
        !field.visible_by_default(),
        "an optional field that restates the sheet in hand waits until a sheet asks for it"
    );

    // ISO 7200 gives the field four characters and writes the A series by
    // its bare designation.
    assert_eq!(DrawingSheetStandard::IsoA4.paper_size(), "A4");
    assert_eq!(DrawingSheetStandard::IsoA0.paper_size(), "A0");
    // The other series keep their prefix, since B4 alone would not say
    // whose B4 it is.
    assert_eq!(DrawingSheetStandard::JisB4.paper_size(), "JIS B4");
    assert_eq!(DrawingSheetStandard::AnsiB.paper_size(), "ANSI B");
    assert_eq!(DrawingSheetStandard::ArchD.paper_size(), "ARCH D");
    for standard in DrawingSheetStandard::ALL {
        assert!(
            standard.paper_size().chars().count() <= 6,
            "{standard:?} is called {:?}",
            standard.paper_size()
        );
    }
    assert_eq!(
        SchematicSheetFormat::default().authored_size.paper_size(),
        "A4"
    );
    let custom = SchematicSheetFormat::try_custom(
        "Review strip",
        210_000,
        594_000,
        SchematicPageOrientation::Portrait,
    )
    .unwrap();
    assert_eq!(custom.authored_size.paper_size(), "Review strip");

    // The field is stored under the name it was saved with before it was
    // withdrawn, so those sheets name it the same way.
    assert_eq!(
        serde_json::to_value(field).unwrap(),
        serde_json::json!("format")
    );
}

#[test]
fn document_type_says_what_kind_of_document_a_sheet_is() {
    let field = DrawingSheetTitleFieldId::DocumentType;
    assert!(DrawingSheetTitleFieldId::ALL.contains(&field));
    assert_eq!(field.display_label(), "Document type");
    assert_eq!(
        field.policy().value_authority,
        DrawingSheetTitleFieldValueAuthority::Authored
    );
    assert!(!field.policy().required_visible);
    assert!(
        field.visible_by_default(),
        "ISO 7200 makes the document type mandatory"
    );

    let value = |format: &SchematicSheetFormat| {
        resolve_drawing_sheet_title_fields(format, &BTreeMap::new())
            .into_iter()
            .find(|resolved| resolved.id == field)
            .map(|resolved| resolved.value)
    };
    let format = SchematicSheetFormat::default();
    assert_eq!(value(&format).as_deref(), Some("Circuit diagram"));
    // A sheet that is some other kind of document says so.
    let wiring = format
        .try_update(|draft| {
            draft.title_block.fields.get_mut(&field).unwrap().value = "Wiring diagram".to_owned();
        })
        .unwrap();
    assert_eq!(value(&wiring).as_deref(), Some("Wiring diagram"));
}

#[test]
fn document_status_holds_the_release_state_revision_and_date_used_to() {
    let field = DrawingSheetTitleFieldId::DocumentStatus;
    assert!(DrawingSheetTitleFieldId::ALL.contains(&field));
    assert_eq!(field.display_label(), "Document status");
    assert_eq!(
        field.policy().value_authority,
        DrawingSheetTitleFieldValueAuthority::Automatic
    );
    assert!(field.visible_by_default());

    // An unreleased document letters its status in its own field, and the
    // index and date fields hold nothing rather than a status word.
    let control = DrawingSheetDocumentControl::default();
    let authority_values = [
        (
            DrawingSheetTitleFieldId::Revision,
            control.revision_index().to_owned(),
        ),
        (
            DrawingSheetTitleFieldId::Date,
            control.revision_date_utc.clone(),
        ),
        (field, control.status.label().to_owned()),
    ]
    .into_iter()
    .collect();
    let resolved =
        resolve_drawing_sheet_title_fields(&SchematicSheetFormat::default(), &authority_values);
    let value = |id| {
        resolved
            .iter()
            .find(|resolved| resolved.id == id)
            .map(|resolved| resolved.value.as_str())
    };
    assert_eq!(value(field), Some("Draft"));
    assert_eq!(value(DrawingSheetTitleFieldId::Revision), Some("—"));
    assert_eq!(value(DrawingSheetTitleFieldId::Date), Some("—"));
}

#[test]
fn a_new_sheet_starts_on_the_180_mm_block_iso_7200_specifies() {
    let format = SchematicSheetFormat::default();
    assert_eq!(
        format.title_block.template,
        DrawingSheetTitleBlockTemplate::Standard
    );
    assert!(!format.title_block_substituted());
    let geometry = format.geometry().unwrap();
    assert_eq!(geometry.title_block.unwrap().width_um, 180_000);

    // ISO 7200 recommends 25 characters for a title and 16 for the document
    // identifier; every value cell of the default block holds the longer.
    let visible = resolve_drawing_sheet_title_fields(&format, &BTreeMap::new()).len();
    let value_chars = drawing_sheet_title_cell_capacity(
        &format,
        &geometry,
        visible,
        format.authored_size.lettering().data_um,
    )
    .unwrap();
    assert!(
        value_chars >= 25,
        "the default block holds {value_chars} characters a value"
    );

    // Paper too narrow for that block starts on the compact one, rather than
    // requesting a block Page Setup would report as substituted.
    for narrow in [
        SchematicSheetFormat::from_standard(
            DrawingSheetStandard::IsoA4,
            SchematicPageOrientation::Portrait,
        ),
        crate::custom_format("Panel", 250_000, 400_000, crate::StartingFrame::IsoA).unwrap(),
    ] {
        assert_eq!(
            narrow.title_block.template,
            DrawingSheetTitleBlockTemplate::Compact
        );
        assert!(!narrow.title_block_substituted());
    }
    let wide_custom =
        crate::custom_format("Panel", 420_000, 297_000, crate::StartingFrame::AnsiA).unwrap();
    assert_eq!(
        wide_custom.title_block.template,
        DrawingSheetTitleBlockTemplate::Standard
    );

    // A sheet saved before title blocks were stored is still drawn with the
    // compact block it always had.
    let mut legacy = serde_json::to_value(&format).unwrap();
    legacy.as_object_mut().unwrap().remove("title_block");
    let restored: SchematicSheetFormat = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        restored.title_block.template,
        DrawingSheetTitleBlockTemplate::Compact
    );
}

#[test]
fn an_organization_template_sealed_while_paper_size_was_withdrawn_still_loads() {
    let managed = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
        })
        .unwrap();
    // The built-in template exactly as the build that withdrew Format sealed
    // it: thirteen fields with no paper size, and a digest over those.
    let sealed_order = [
        "project",
        "cell-view",
        "sheet-title",
        "page",
        "revision",
        "scale",
        "drawn-by",
        "checked-by",
        "approved-by",
        "date",
        "organization",
        "document-id",
        "classification",
    ];
    let sealed_digest = "565da65b96c4e40484340e3fabe9f898b4f3a3392bc291a296853a1730e4ae2b";
    let mut wire = serde_json::to_value(&managed).unwrap();
    wire["title_block"]["fields"]
        .as_object_mut()
        .unwrap()
        .remove("format");
    wire["title_block"]["managed_template"]["field_order"] = serde_json::json!(sealed_order);
    wire["title_block"]["managed_template"]["content_digest"] = serde_json::json!(sealed_digest);

    let restored: SchematicSheetFormat = serde_json::from_value(wire).unwrap();
    assert_eq!(
        restored
            .title_block
            .managed_template
            .as_ref()
            .unwrap()
            .content_digest()
            .to_string(),
        sealed_digest,
        "loading must not re-seal what was released"
    );
    // Every field the seal names keeps its sealed place, and the ones it never
    // named follow them in the product's order.
    let gained = [
        DrawingSheetTitleFieldId::DocumentType,
        DrawingSheetTitleFieldId::PaperSize,
        DrawingSheetTitleFieldId::DocumentStatus,
    ];
    let mut expected = DrawingSheetTitleFieldId::ALL
        .into_iter()
        .filter(|field| !gained.contains(field))
        .collect::<Vec<_>>();
    expected.extend(gained);
    assert_eq!(restored.title_block_field_order(), expected);
    assert!(!restored.title_block.fields[&DrawingSheetTitleFieldId::PaperSize].visible);

    let rewritten = serde_json::to_value(&restored).unwrap();
    assert_eq!(
        rewritten["title_block"]["managed_template"]["field_order"],
        serde_json::json!(sealed_order)
    );
    assert_eq!(
        serde_json::from_value::<SchematicSheetFormat>(rewritten).unwrap(),
        restored
    );

    // A template sealed today names every current field.
    let mut missing = DrawingSheetTitleFieldId::ALL.to_vec();
    missing.retain(|field| *field != DrawingSheetTitleFieldId::PaperSize);
    assert!(
        DrawingSheetManagedTemplateSnapshot::try_new(
            "example.release-title-block",
            "2026.10",
            180_000,
            50_000,
            5,
            missing,
            Vec::new(),
        )
        .is_err()
    );
}

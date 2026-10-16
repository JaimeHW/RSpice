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
    // Every field the seal names keeps its sealed place, and the one it never
    // named follows them.
    let mut expected = DrawingSheetTitleFieldId::ALL
        .into_iter()
        .filter(|field| *field != DrawingSheetTitleFieldId::PaperSize)
        .collect::<Vec<_>>();
    expected.push(DrawingSheetTitleFieldId::PaperSize);
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

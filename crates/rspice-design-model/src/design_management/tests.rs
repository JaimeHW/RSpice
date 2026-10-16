//! Design management tests.

use super::*;

mod annotation_allocation;
mod drawing_sheet_release;
mod manual_annotation;
mod sheet_lifecycle;
mod title_fields;

fn sheet(name: &str, page: u32) -> SheetDefinition {
    SheetDefinition {
        name: name.to_owned(),
        template: SheetTemplate::AnalogSchematic,
        port_policy: SheetPortPolicy::TypedOffSheetPorts,
        explicit_page_number: Some(page),
    }
}

fn authored_format(format: SchematicSheetFormat, title: &str) -> SchematicSheetFormat {
    format
        .try_update(|draft| {
            let field = draft
                .title_block
                .fields
                .entry(DrawingSheetTitleFieldId::SheetTitle)
                .or_default();
            field.visible = true;
            field.value = title.to_owned();
        })
        .unwrap()
}

fn drawing_sheet_preset(
    id: impl Into<String>,
    name: impl Into<String>,
    scope: DrawingSheetPresetScope,
    format: SchematicSheetFormat,
) -> DrawingSheetPreset {
    DrawingSheetPreset {
        id: id.into(),
        name: name.into(),
        scope,
        format,
    }
    .normalized_for_storage()
    .unwrap()
}

#[test]
fn governed_sheet_page_format_is_revisioned_and_legacy_safe() {
    let mut catalog = SheetCatalog::default();
    let id = catalog.create_sheet(sheet("Input", 1), None).unwrap();
    let original_sheet_revision = catalog.find(id).unwrap().revision();
    let original_catalog_revision = catalog.revision();
    let semantic_digest = catalog.find(id).unwrap().semantic_digest();
    let custom = authored_format(
        SchematicSheetFormat::try_custom(
            "Review board",
            457_200,
            304_800,
            SchematicPageOrientation::Landscape,
        )
        .unwrap(),
        "Input",
    );

    let revision = catalog
        .update_sheet_page_format(id, original_sheet_revision, custom.clone())
        .unwrap();

    assert_eq!(revision, original_sheet_revision + 1);
    assert_eq!(catalog.revision(), original_catalog_revision + 1);
    assert_eq!(catalog.find(id).unwrap().page_format(), &custom);
    assert_eq!(
        catalog.find(id).unwrap().semantic_digest(),
        semantic_digest,
        "physical presentation must not rewrite electrical semantics"
    );
    assert_eq!(custom.portrait_dimensions_um(), (304_800, 457_200));
    assert_eq!(custom.oriented_dimensions_um(), (457_200, 304_800));
    assert!(matches!(
        catalog.update_sheet_page_format(id, revision, custom.clone()),
        Err(DesignManagementError::NoChanges("sheet page format"))
    ));
    assert!(
        catalog
            .update_sheet_page_format(id, original_sheet_revision, custom)
            .is_err(),
        "stale sheet revision must fail closed"
    );

    let mut legacy = serde_json::to_value(&catalog).unwrap();
    legacy["sheets"][0]
        .as_object_mut()
        .unwrap()
        .remove("page_format");
    let restored: SheetCatalog = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        restored.find(id).unwrap().page_format(),
        &authored_format(SchematicSheetFormat::default(), "Input")
    );
}

#[test]
fn drawing_sheet_standards_are_exact_valid_and_round_trip() {
    assert_eq!(
        SchematicSheetFormat::default().title_block.scale,
        DrawingSheetScale::Ratio {
            drawing_units: 1,
            reality_units: 1,
        }
    );
    let expected = [
        (DrawingSheetStandard::IsoA5, (148_000, 210_000)),
        (DrawingSheetStandard::IsoA4, (210_000, 297_000)),
        (DrawingSheetStandard::IsoA3, (297_000, 420_000)),
        (DrawingSheetStandard::IsoA2, (420_000, 594_000)),
        (DrawingSheetStandard::IsoA1, (594_000, 841_000)),
        (DrawingSheetStandard::IsoA0, (841_000, 1_189_000)),
        (DrawingSheetStandard::AnsiA, (215_900, 279_400)),
        (DrawingSheetStandard::AnsiB, (279_400, 431_800)),
        (DrawingSheetStandard::AnsiC, (431_800, 558_800)),
        (DrawingSheetStandard::AnsiD, (558_800, 863_600)),
        (DrawingSheetStandard::AnsiE, (863_600, 1_117_600)),
        (DrawingSheetStandard::ArchA, (228_600, 304_800)),
        (DrawingSheetStandard::ArchB, (304_800, 457_200)),
        (DrawingSheetStandard::ArchC, (457_200, 609_600)),
        (DrawingSheetStandard::ArchD, (609_600, 914_400)),
        (DrawingSheetStandard::ArchE, (914_400, 1_219_200)),
        (DrawingSheetStandard::JisB5, (182_000, 257_000)),
        (DrawingSheetStandard::JisB4, (257_000, 364_000)),
        (DrawingSheetStandard::JisB3, (364_000, 515_000)),
        (DrawingSheetStandard::JisB2, (515_000, 728_000)),
    ];
    assert_eq!(DrawingSheetStandard::ALL.len(), expected.len());
    for (standard, dimensions) in expected {
        assert_eq!(standard.portrait_dimensions_um(), dimensions);
        let format =
            SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Portrait);
        format.validate().unwrap();
        assert_eq!(format.portrait_dimensions_um(), dimensions);
        let encoded = serde_json::to_string(&format).unwrap();
        let restored: SchematicSheetFormat = serde_json::from_str(&encoded).unwrap();
        assert_eq!(restored, format);
    }
}

#[test]
fn drawing_sheet_series_margins_and_border_mark_defaults_are_normative() {
    assert_eq!(
        DrawingSheetStandard::IsoA4.default_margins(),
        DrawingSheetMargins {
            top_um: 10_000,
            right_um: 10_000,
            bottom_um: 10_000,
            left_um: 20_000,
        }
    );
    assert_eq!(
        DrawingSheetStandard::AnsiA.default_margins(),
        DrawingSheetMargins {
            top_um: 12_700,
            right_um: 12_700,
            bottom_um: 12_700,
            left_um: 19_050,
        }
    );
    assert_eq!(
        DrawingSheetStandard::ArchA.default_margins(),
        DrawingSheetMargins {
            top_um: 12_700,
            right_um: 12_700,
            bottom_um: 12_700,
            left_um: 25_400,
        }
    );
    assert_eq!(
        DrawingSheetStandard::JisB5.default_margins(),
        DrawingSheetMargins {
            top_um: 10_000,
            right_um: 10_000,
            bottom_um: 10_000,
            left_um: 20_000,
        }
    );
    assert_eq!(
        DrawingSheetBorderTemplate::Standard.default_marks(),
        DrawingSheetMarks {
            registration: true,
            folding: false,
        }
    );
    assert_eq!(
        DrawingSheetBorderTemplate::OrganizationManaged.default_marks(),
        DrawingSheetMarks {
            registration: true,
            folding: true,
        }
    );
    let mut draft = SchematicSheetFormatDraft::from(&SchematicSheetFormat::default());
    draft.apply_border_template(DrawingSheetBorderTemplate::OrganizationManaged);
    assert_eq!(
        draft.marks,
        DrawingSheetMarks {
            registration: true,
            folding: true,
        }
    );
}

#[test]
fn drawing_sheet_title_contract_matches_field_provenance_and_required_visibility() {
    let format = SchematicSheetFormat::default();
    for field in [
        DrawingSheetTitleFieldId::CellView,
        DrawingSheetTitleFieldId::Page,
        DrawingSheetTitleFieldId::Revision,
    ] {
        let policy = field.policy();
        assert!(policy.required_visible);
        assert_eq!(
            policy.value_authority,
            DrawingSheetTitleFieldValueAuthority::Automatic
        );
        assert!(format.title_block.fields[&field].visible);
    }
    // The project name repeats the leading segment of `Cell / view`, so it is
    // shown by default but the sheet may drop it.
    let project = DrawingSheetTitleFieldId::Project.policy();
    assert!(!project.required_visible);
    assert_eq!(
        project.value_authority,
        DrawingSheetTitleFieldValueAuthority::Automatic
    );
    assert!(format.title_block.fields[&DrawingSheetTitleFieldId::Project].visible);
    let sheet_title = DrawingSheetTitleFieldId::SheetTitle.policy();
    assert!(sheet_title.required_visible);
    assert_eq!(
        sheet_title.value_authority,
        DrawingSheetTitleFieldValueAuthority::Authored
    );
    assert_eq!(
        DrawingSheetTitleFieldId::Date.policy().value_authority,
        DrawingSheetTitleFieldValueAuthority::Automatic
    );
    // A schematic is not drawn to scale, and ISO 7200 keeps scale out of the
    // title block, so a sheet carries no Scale field at all.
    assert!(
        !format
            .title_block
            .fields
            .contains_key(&DrawingSheetTitleFieldId::Scale)
    );
    assert_eq!(DrawingSheetDisplayUnit::Inches.format_um(25_400), "1");
    assert_eq!(
        DrawingSheetDisplayUnit::Millimetres.format_size_um(210_000, 297_000),
        "210 × 297 mm"
    );

    // The project name may be dropped now that `Cell / view` carries it.
    let without_project = format
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::Project)
                .unwrap()
                .visible = false;
        })
        .expect("the project name is no longer a required cell");
    assert!(
        !resolve_drawing_sheet_title_fields(&without_project, &BTreeMap::new())
            .iter()
            .any(|field| field.id == DrawingSheetTitleFieldId::Project)
    );

    let hidden = format.try_update(|draft| {
        draft
            .title_block
            .fields
            .get_mut(&DrawingSheetTitleFieldId::CellView)
            .unwrap()
            .visible = false;
    });
    assert!(matches!(
        hidden,
        Err(DesignManagementError::NumericRange(
            "required drawing sheet title field visibility"
        ))
    ));
    let overridden = format.try_update(|draft| {
        draft
            .title_block
            .fields
            .get_mut(&DrawingSheetTitleFieldId::Page)
            .unwrap()
            .value = "17".to_owned();
    });
    assert!(matches!(
        overridden,
        Err(DesignManagementError::InvalidText {
            field: "automatic drawing sheet title field",
            ..
        })
    ));
}

#[test]
fn drawing_sheet_title_resolution_is_one_renderer_independent_authority() {
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = "Power tree".to_owned();
            // An optional field is hidden until a sheet opts into it, and a
            // shown one may be dropped.
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::CheckedBy)
                .unwrap()
                .visible = true;
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::ApprovedBy)
                .unwrap()
                .visible = false;
            // Legacy project-owned storage remains a migration fallback only.
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::Organization)
                .unwrap()
                .value = "Legacy Labs".to_owned();
        })
        .unwrap();
    let authority_values = [
        (DrawingSheetTitleFieldId::Project, "Precision Instruments"),
        (DrawingSheetTitleFieldId::CellView, "analog/top/schematic"),
        (DrawingSheetTitleFieldId::Page, "2 of 4"),
        (DrawingSheetTitleFieldId::Revision, "B"),
        (DrawingSheetTitleFieldId::Scale, "1:1"),
        (DrawingSheetTitleFieldId::Date, "2026-08-04"),
        (DrawingSheetTitleFieldId::Organization, "RSpice Engineering"),
    ]
    .into_iter()
    .map(|(id, value)| (id, value.to_owned()))
    .collect();

    let resolved = resolve_drawing_sheet_title_fields(&format, &authority_values);
    let value = |id| {
        resolved
            .iter()
            .find(|field| field.id == id)
            .map(|field| field.value.as_str())
    };
    assert_eq!(
        value(DrawingSheetTitleFieldId::Project),
        Some("Precision Instruments")
    );
    assert_eq!(
        value(DrawingSheetTitleFieldId::SheetTitle),
        Some("Power tree")
    );
    assert_eq!(value(DrawingSheetTitleFieldId::DrawnBy), Some("—"));
    assert_eq!(value(DrawingSheetTitleFieldId::CheckedBy), Some("—"));
    assert_eq!(
        value(DrawingSheetTitleFieldId::Organization),
        Some("RSpice Engineering")
    );
    assert_eq!(value(DrawingSheetTitleFieldId::ApprovedBy), None);
    assert_eq!(
        value(DrawingSheetTitleFieldId::Scale),
        None,
        "a retired field resolves to nothing even when a value is offered"
    );
    assert_eq!(
        DrawingSheetTitleFieldId::SheetTitle.display_label(),
        "Sheet title"
    );
}

#[test]
fn managed_title_templates_are_versioned_digest_verified_and_layout_authoritative() {
    let managed = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
        })
        .unwrap();
    let built_in = managed
        .title_block
        .managed_template
        .as_ref()
        .expect("managed selection captures the built-in policy snapshot");
    assert_eq!(built_in.template_id(), "rspice.organization-title-block");
    assert_eq!(built_in.revision(), "1");
    built_in.validate().unwrap();
    assert_eq!(
        managed.title_block_dimensions_um(DrawingSheetTitleBlockTemplate::OrganizationManaged),
        Some((180_000, 50_000))
    );
    assert_eq!(
        managed.title_block_rows(DrawingSheetTitleBlockTemplate::OrganizationManaged),
        Some(5)
    );
    assert!(built_in.locks_field(DrawingSheetTitleFieldId::Classification));

    let mut reversed = DrawingSheetTitleFieldId::ALL.to_vec();
    reversed.reverse();
    let logo = DrawingSheetManagedLogo::try_new(
        "Example Labs mark",
        40_000,
        vec![
            DrawingSheetManagedLogoPrimitive::try_new(
                vec![
                    DrawingSheetManagedLogoPoint::try_new(1_000, 9_000).unwrap(),
                    DrawingSheetManagedLogoPoint::try_new(5_000, 1_000).unwrap(),
                    DrawingSheetManagedLogoPoint::try_new(9_000, 9_000).unwrap(),
                ],
                true,
                true,
            )
            .unwrap(),
            DrawingSheetManagedLogoPrimitive::try_new(
                vec![
                    DrawingSheetManagedLogoPoint::try_new(2_500, 7_000).unwrap(),
                    DrawingSheetManagedLogoPoint::try_new(7_500, 7_000).unwrap(),
                ],
                false,
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let snapshot = DrawingSheetManagedTemplateSnapshot::try_new_with_logo(
        "example.release-title-block",
        "2026.08",
        160_000,
        40_000,
        4,
        reversed.clone(),
        vec![DrawingSheetTitleFieldId::ApprovedBy],
        Some(logo),
    )
    .unwrap();
    let custom = managed
        .try_update(|draft| {
            draft.title_block.managed_template = Some(snapshot.clone());
        })
        .unwrap();
    assert_eq!(custom.title_block_field_order(), reversed);
    assert_eq!(
        custom.title_block_dimensions_um(DrawingSheetTitleBlockTemplate::OrganizationManaged),
        Some((160_000, 40_000))
    );
    assert_eq!(
        custom.title_block_rows(DrawingSheetTitleBlockTemplate::OrganizationManaged),
        Some(4)
    );
    assert_eq!(
        custom
            .title_block_logo(DrawingSheetTitleBlockTemplate::OrganizationManaged)
            .map(DrawingSheetManagedLogo::alternative_text),
        Some("Example Labs mark")
    );
    let without_logo_snapshot = DrawingSheetManagedTemplateSnapshot::try_new(
        "example.release-title-block",
        "2026.08",
        160_000,
        40_000,
        4,
        reversed.clone(),
        vec![DrawingSheetTitleFieldId::ApprovedBy],
    )
    .unwrap();
    let without_logo = custom
        .try_update(|draft| {
            draft.title_block.managed_template = Some(without_logo_snapshot);
        })
        .unwrap();
    assert!(
        drawing_sheet_title_cell_capacity(
            &custom,
            &custom.geometry().unwrap(),
            9,
            custom.authored_size.lettering().data_um
        )
        .unwrap()
            < drawing_sheet_title_cell_capacity(
                &without_logo,
                &without_logo.geometry().unwrap(),
                9,
                without_logo.authored_size.lettering().data_um
            )
            .unwrap()
    );
    // A managed template governs the order fields resolve in; sheet-owned
    // visibility decides which of them appear, and the two compose.
    let resolved_order = resolve_drawing_sheet_title_fields(&custom, &BTreeMap::new())
        .into_iter()
        .map(|field| field.id)
        .collect::<Vec<_>>();
    let expected_order = reversed
        .iter()
        .copied()
        .filter(|field| custom.title_block.fields[field].visible)
        .collect::<Vec<_>>();
    assert_eq!(resolved_order, expected_order);
    assert_eq!(
        resolved_order.first(),
        Some(&DrawingSheetTitleFieldId::DocumentId),
        "classification leads the template order but is hidden until released"
    );

    let mut tampered = serde_json::to_value(&custom).unwrap();
    tampered["title_block"]["managed_template"]["content_digest"] =
        serde_json::Value::String("00".repeat(32));
    assert!(serde_json::from_value::<SchematicSheetFormat>(tampered).is_err());

    let mut tampered_logo = serde_json::to_value(&custom).unwrap();
    tampered_logo["title_block"]["managed_template"]["logo"]["primitives"][0]["points"][0]["x"] =
        serde_json::Value::from(2_000);
    assert!(serde_json::from_value::<SchematicSheetFormat>(tampered_logo).is_err());

    let mut legacy = serde_json::to_value(&managed).unwrap();
    legacy["title_block"]
        .as_object_mut()
        .unwrap()
        .remove("managed_template");
    let migrated: SchematicSheetFormat = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        migrated
            .title_block
            .managed_template
            .as_ref()
            .map(DrawingSheetManagedTemplateSnapshot::content_digest),
        Some(DrawingSheetManagedTemplateSnapshot::default().content_digest())
    );
}

#[test]
fn managed_title_logo_rejects_ambiguous_unbounded_and_excessive_geometry() {
    assert!(
        DrawingSheetManagedLogoPoint::try_new(DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS + 1, 0)
            .is_err()
    );
    let concave = vec![
        DrawingSheetManagedLogoPoint::try_new(0, 0).unwrap(),
        DrawingSheetManagedLogoPoint::try_new(10_000, 0).unwrap(),
        DrawingSheetManagedLogoPoint::try_new(5_000, 5_000).unwrap(),
        DrawingSheetManagedLogoPoint::try_new(10_000, 10_000).unwrap(),
        DrawingSheetManagedLogoPoint::try_new(0, 10_000).unwrap(),
    ];
    assert!(DrawingSheetManagedLogoPrimitive::try_new(concave, true, true).is_err());
    assert!(
        DrawingSheetManagedLogoPrimitive::try_new(
            vec![
                DrawingSheetManagedLogoPoint::try_new(0, 0).unwrap(),
                DrawingSheetManagedLogoPoint::try_new(10_000, 10_000).unwrap(),
            ],
            false,
            true,
        )
        .is_err()
    );

    let line = DrawingSheetManagedLogoPrimitive::try_new(
        vec![
            DrawingSheetManagedLogoPoint::try_new(0, 0).unwrap(),
            DrawingSheetManagedLogoPoint::try_new(10_000, 10_000).unwrap(),
        ],
        false,
        false,
    )
    .unwrap();
    assert!(
        DrawingSheetManagedLogo::try_new(
            "Excessive mark",
            20_000,
            vec![line.clone(); MAX_DRAWING_SHEET_MANAGED_LOGO_PRIMITIVES + 1],
        )
        .is_err()
    );
    let oversized_reserve =
        DrawingSheetManagedLogo::try_new("Oversized mark", 120_000, vec![line]).unwrap();
    assert!(
        DrawingSheetManagedTemplateSnapshot::try_new_with_logo(
            "example.release-title-block",
            "2026.08",
            160_000,
            40_000,
            4,
            DrawingSheetTitleFieldId::ALL.to_vec(),
            Vec::new(),
            Some(oversized_reserve),
        )
        .is_err()
    );
}

#[test]
fn applying_a_format_preserves_target_sheet_fields() {
    let source = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.scale = DrawingSheetScale::Ratio {
                drawing_units: 2,
                reality_units: 5,
            };
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::DrawnBy)
                .unwrap()
                .value = "Source owner".to_owned();
        })
        .unwrap();
    let target = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = "Power tree".to_owned();
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::DrawnBy)
                .unwrap()
                .value = "Target owner".to_owned();
        })
        .unwrap();

    let applied = source.with_target_sheet_title_fields(&target);

    assert_eq!(
        applied.title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
        "Power tree"
    );
    assert_eq!(
        applied.title_block.fields[&DrawingSheetTitleFieldId::DrawnBy].value,
        "Target owner"
    );
    assert_eq!(
        applied.title_block.scale,
        DrawingSheetScale::Ratio {
            drawing_units: 2,
            reality_units: 5,
        }
    );
    assert!(applied.validate().is_ok());
}

#[test]
fn drawing_sheet_geometry_is_canonical_and_bounded() {
    let format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Landscape,
    );
    let geometry = format.geometry().unwrap();
    assert_eq!(
        geometry.paper,
        DrawingSheetRect {
            x_um: 0,
            y_um: 0,
            width_um: 297_000,
            height_um: 210_000,
        }
    );
    assert_eq!(
        geometry.printable,
        DrawingSheetRect {
            x_um: 20_000,
            y_um: 10_000,
            width_um: 267_000,
            height_um: 190_000,
        }
    );
    assert_eq!(
        geometry.drawing_area,
        DrawingSheetRect {
            x_um: 25_000,
            y_um: 15_000,
            width_um: 257_000,
            height_um: 180_000,
        }
    );
    assert_eq!(
        geometry.title_block,
        Some(DrawingSheetRect {
            x_um: 102_000,
            y_um: 150_000,
            width_um: 180_000,
            height_um: 45_000,
        })
    );
    assert_eq!(geometry.zones.unwrap().columns, 4);
    assert_eq!(geometry.zones.unwrap().rows, 4);
}

#[test]
fn drawing_sheet_title_block_substitution_is_visible_and_non_destructive() {
    let format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    )
    .try_update(|draft| {
        draft.title_block.template = DrawingSheetTitleBlockTemplate::Standard;
    })
    .unwrap();

    assert_eq!(
        format.title_block.template,
        DrawingSheetTitleBlockTemplate::Standard,
        "the requested template remains authored"
    );
    assert_eq!(
        format.effective_title_block_template(),
        DrawingSheetTitleBlockTemplate::Compact
    );
    assert!(format.title_block_substituted());
    assert_eq!(
        format.geometry().unwrap().effective_title_block_template,
        DrawingSheetTitleBlockTemplate::Compact
    );

    let landscape = format
        .try_update(|draft| {
            draft.orientation = SchematicPageOrientation::Landscape;
        })
        .unwrap();
    assert!(!landscape.title_block_substituted());
    assert_eq!(
        landscape.geometry().unwrap().effective_title_block_template,
        DrawingSheetTitleBlockTemplate::Standard
    );
}

#[test]
fn drawing_sheet_drafts_derive_compatibility_and_reject_invalid_edits_atomically() {
    let original = SchematicSheetFormat::default();
    let arch = original
        .try_update(|draft| {
            draft.authored_size = AuthoredDrawingSheetSize::Standard {
                standard: DrawingSheetStandard::ArchD,
            };
            draft.orientation = SchematicPageOrientation::Portrait;
            draft.margins = DrawingSheetStandard::ArchD.default_margins();
        })
        .unwrap();
    assert_eq!(arch.portrait_dimensions_um(), (609_600, 914_400));
    assert!(matches!(
        &arch.size,
        SchematicSheetSize::Custom {
            name,
            portrait_width_um: 609_600,
            portrait_height_um: 914_400,
        } if name == "ARCH D"
    ));

    let invalid = original.try_update(|draft| {
        draft.margins.left_um = 2_000_000;
    });
    assert!(matches!(
        invalid,
        Err(DesignManagementError::NumericRange(_))
    ));
    assert_eq!(original, SchematicSheetFormat::default());
}

#[test]
fn legacy_sheet_format_receives_contextual_defaults() {
    let legacy = serde_json::json!({
        "size": {"kind": "standard", "size": "us-letter"},
        "orientation": "portrait"
    });
    let format: SchematicSheetFormat = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        format.authored_size,
        AuthoredDrawingSheetSize::Standard {
            standard: DrawingSheetStandard::AnsiA
        }
    );
    assert_eq!(
        format.margins,
        DrawingSheetStandard::AnsiA.default_margins()
    );
    assert_eq!(format.portrait_dimensions_um(), (215_900, 279_400));
    format.validate().unwrap();
}

#[test]
fn project_drawing_sheet_defaults_and_presets_are_revision_guarded() {
    let mut catalog = DesignManagementCatalog::default();
    let initial_revision = catalog.revision();
    let semantic_digest = catalog.semantic_digest().unwrap();
    let project_default = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiC,
        SchematicPageOrientation::Landscape,
    );
    let default_revision = catalog
        .update_drawing_sheet_default(initial_revision, project_default)
        .unwrap();
    assert_eq!(default_revision, initial_revision + 1);
    assert_eq!(
        catalog.semantic_digest().unwrap(),
        semantic_digest,
        "drawing-sheet presentation must not invalidate electrical semantics"
    );
    assert_eq!(
        catalog.drawing_sheet_settings().default_format.inheritance,
        DrawingSheetInheritance::ProjectDefault
    );

    let preset = drawing_sheet_preset(
        "review-board",
        "Review Board",
        DrawingSheetPresetScope::Project,
        SchematicSheetFormat::from_standard(
            DrawingSheetStandard::ArchB,
            SchematicPageOrientation::Landscape,
        ),
    );
    let preset_revision = catalog
        .publish_drawing_sheet_preset(default_revision, preset.clone())
        .unwrap();
    assert_eq!(
        catalog.drawing_sheet_settings().find_preset("REVIEW-BOARD"),
        Some(&preset)
    );
    assert!(matches!(
        catalog.publish_drawing_sheet_preset(default_revision, preset.clone()),
        Err(DesignManagementError::RevisionConflict { .. })
    ));

    let duplicate_name = DrawingSheetPreset {
        id: "another-id".to_owned(),
        name: "review board".to_owned(),
        ..preset
    };
    assert!(matches!(
        catalog.publish_drawing_sheet_preset(preset_revision, duplicate_name),
        Err(DesignManagementError::DuplicateListEntry { .. })
    ));
    let removed_revision = catalog
        .remove_drawing_sheet_preset(preset_revision, "Review-Board")
        .unwrap();
    assert_eq!(removed_revision, preset_revision + 1);
    assert!(
        catalog
            .drawing_sheet_settings()
            .find_preset("review-board")
            .is_none()
    );

    let mut legacy = serde_json::to_value(&catalog).unwrap();
    legacy
        .as_object_mut()
        .unwrap()
        .remove("drawing_sheet_settings");
    let restored: DesignManagementCatalog = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        restored.drawing_sheet_settings(),
        &DrawingSheetProjectSettings::default()
    );
}

#[test]
fn reviewed_publish_accepts_drawing_sheet_only_project_changes() {
    let mut live = DesignManagementCatalog::default();
    let expected_revision = live.revision();
    let electrical_digest = live.semantic_digest().unwrap();
    let preset = drawing_sheet_preset(
        "lab-documentation-panel",
        "Lab documentation panel",
        DrawingSheetPresetScope::Project,
        SchematicSheetFormat::try_custom(
            "Lab documentation panel",
            250_000,
            400_000,
            SchematicPageOrientation::Portrait,
        )
        .unwrap(),
    );
    let mut candidate = live.clone();
    candidate
        .publish_drawing_sheet_preset(candidate.revision(), preset.clone())
        .unwrap();

    let published_revision = live
        .publish_reviewed_candidate(expected_revision, candidate)
        .unwrap();

    assert_eq!(published_revision, expected_revision + 1);
    assert_eq!(
        live.drawing_sheet_settings()
            .find_preset("LAB-DOCUMENTATION-PANEL"),
        Some(&preset)
    );
    assert_eq!(
        live.semantic_digest().unwrap(),
        electrical_digest,
        "publishing drawing-sheet presentation must preserve electrical semantics"
    );
}

#[test]
fn drawing_sheet_preset_rename_updates_every_embedded_reference_without_geometry_changes() {
    let mut catalog = DesignManagementCatalog::default();
    let preset_id = "review-panel";
    let mut format = SchematicSheetFormat::try_custom(
        "Review panel",
        210_000,
        420_000,
        SchematicPageOrientation::Landscape,
    )
    .unwrap();
    format = format
        .try_update(|draft| {
            if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                snapshot.preset_id = Some(preset_id.to_owned());
            }
        })
        .unwrap();
    let geometry = format.geometry().unwrap();
    catalog
        .publish_drawing_sheet_preset(
            catalog.revision(),
            DrawingSheetPreset {
                id: preset_id.to_owned(),
                name: "Review panel".to_owned(),
                scope: DrawingSheetPresetScope::Project,
                format: format.clone(),
            },
        )
        .unwrap();

    let mut settings = catalog.drawing_sheet_settings().clone();
    settings.default_format = format
        .try_update(|draft| draft.inheritance = DrawingSheetInheritance::ProjectDefault)
        .unwrap();
    settings.last_explicit_format = Some(format.clone());
    catalog
        .update_drawing_sheet_settings(catalog.revision(), settings)
        .unwrap();
    let sheet_id = catalog
        .bootstrap_for_cell_view("work/top/schematic", "Main", [])
        .unwrap();
    let sheet_revision = catalog
        .sheet_catalog("work/top/schematic")
        .unwrap()
        .find(sheet_id)
        .unwrap()
        .revision();

    catalog
        .rename_drawing_sheet_preset(
            catalog.revision(),
            preset_id,
            "Qualification panel".to_owned(),
        )
        .unwrap();

    let settings = catalog.drawing_sheet_settings();
    let referenced_names = [
        &settings.find_preset(preset_id).unwrap().format,
        &settings.default_format,
        settings.last_explicit_format.as_ref().unwrap(),
        catalog
            .sheet_catalog("work/top/schematic")
            .unwrap()
            .find(sheet_id)
            .unwrap()
            .page_format(),
    ]
    .map(|format| match &format.authored_size {
        AuthoredDrawingSheetSize::Custom { snapshot } => snapshot.name.as_str(),
        AuthoredDrawingSheetSize::Standard { .. } => panic!("expected a custom format"),
    });
    assert!(
        referenced_names
            .into_iter()
            .all(|name| name == "Qualification panel")
    );
    assert_eq!(
        settings.find_preset(preset_id).unwrap().name,
        "Qualification panel"
    );
    assert_eq!(settings.default_format.geometry().unwrap(), geometry);
    assert_eq!(
        catalog
            .sheet_catalog("work/top/schematic")
            .unwrap()
            .find(sheet_id)
            .unwrap()
            .revision(),
        sheet_revision + 1
    );
    assert!(matches!(
        catalog.remove_drawing_sheet_preset(catalog.revision(), preset_id),
        Err(DesignManagementError::DrawingSheetPresetInUse { count: 3, .. })
    ));
}

#[test]
fn governed_sheet_bootstrap_inherits_the_project_drawing_sheet_default() {
    let mut catalog = DesignManagementCatalog::default();
    let project_default = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiC,
        SchematicPageOrientation::Portrait,
    );
    catalog
        .update_drawing_sheet_default(catalog.revision(), project_default)
        .unwrap();

    let sheet_id = catalog
        .bootstrap_for_cell_view("work/top/schematic", "Main", [])
        .unwrap();
    let sheet = catalog
        .sheet_catalog("work/top/schematic")
        .and_then(|sheets| sheets.find(sheet_id))
        .unwrap();

    assert_eq!(
        sheet.page_format(),
        &authored_format(
            catalog.drawing_sheet_settings().default_format.clone(),
            "Main"
        )
    );
    assert_eq!(
        sheet.page_format().inheritance,
        DrawingSheetInheritance::ProjectDefault
    );
}

#[test]
fn project_default_changes_refresh_inherited_fallbacks_without_authored_sheet_revisions() {
    let mut catalog = DesignManagementCatalog::default();
    let inherited_id = catalog
        .bootstrap_for_cell_view("work/top/schematic", "Main", [])
        .unwrap();
    let explicit_format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiC,
        SchematicPageOrientation::Landscape,
    );
    let (explicit_id, explicit_snapshot, inherited_revision, sheet_catalog_revision) = {
        let sheets = catalog.sheet_catalog_mut("work/top/schematic").unwrap();
        let explicit_id = sheets
            .create_sheet_with_page_format(
                sheet("Detail", 2),
                Some(inherited_id),
                explicit_format.clone(),
            )
            .unwrap();
        (
            explicit_id,
            sheets.find(explicit_id).unwrap().page_format().clone(),
            sheets.find(inherited_id).unwrap().revision(),
            sheets.revision(),
        )
    };

    let project_default = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA3,
        SchematicPageOrientation::Portrait,
    );
    catalog
        .update_drawing_sheet_default(catalog.revision(), project_default.clone())
        .unwrap();

    let sheets = catalog.sheet_catalog("work/top/schematic").unwrap();
    let inherited = sheets.find(inherited_id).unwrap();
    let explicit = sheets.find(explicit_id).unwrap();
    assert_eq!(
        inherited.page_format().authored_size,
        project_default.authored_size
    );
    assert_eq!(
        inherited.page_format().title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
        "Main"
    );
    assert_eq!(inherited.revision(), inherited_revision);
    assert_eq!(sheets.revision(), sheet_catalog_revision);
    assert_eq!(explicit.page_format(), &explicit_snapshot);
    assert_eq!(explicit.revision(), 1);
}

#[test]
fn project_drawing_sheet_presets_are_bounded_and_project_owned() {
    let presets = (0..MAX_DRAWING_SHEET_PROJECT_PRESETS)
        .map(|index| {
            drawing_sheet_preset(
                format!("preset-{index}"),
                format!("Preset {index}"),
                DrawingSheetPresetScope::Project,
                SchematicSheetFormat::default(),
            )
        })
        .collect::<Vec<_>>();
    let settings = DrawingSheetProjectSettings {
        default_format: DrawingSheetProjectSettings::default().default_format,
        presets,
        ..DrawingSheetProjectSettings::default()
    };
    settings.validate().unwrap();

    let mut over_limit = settings.clone();
    over_limit.presets.push(drawing_sheet_preset(
        "over-limit",
        "Over limit",
        DrawingSheetPresetScope::Project,
        SchematicSheetFormat::default(),
    ));
    assert!(matches!(
        over_limit.validate(),
        Err(DesignManagementError::LimitExceeded {
            domain: "drawing sheet project presets",
            actual,
            maximum: MAX_DRAWING_SHEET_PROJECT_PRESETS,
        }) if actual == MAX_DRAWING_SHEET_PROJECT_PRESETS + 1
    ));
    assert!(
        serde_json::from_value::<DrawingSheetProjectSettings>(
            serde_json::to_value(&over_limit).unwrap()
        )
        .is_err(),
        "standalone deserialization must enforce the resource bound"
    );

    let foreign = DrawingSheetProjectSettings {
        default_format: DrawingSheetProjectSettings::default().default_format,
        presets: vec![drawing_sheet_preset(
            "user-owned",
            "User owned",
            DrawingSheetPresetScope::User,
            SchematicSheetFormat::default(),
        )],
        ..DrawingSheetProjectSettings::default()
    };
    assert!(matches!(
        foreign.validate(),
        Err(DesignManagementError::NumericRange(
            "project drawing sheet preset ownership"
        ))
    ));
}

#[test]
fn drawing_sheet_preset_import_receipts_are_bounded_and_serde_compatible() {
    let source = DrawingSheetPresetImportReference {
        preset_id: "portable-review-strip".to_owned(),
        scope: DrawingSheetPresetScope::User,
    };
    let target = DrawingSheetPresetImportReference {
        preset_id: "project-review-strip".to_owned(),
        scope: DrawingSheetPresetScope::Project,
    };
    let receipt = DrawingSheetPresetImportReceipt {
        source_digest_sha256: "a".repeat(64),
        source_schema: "rspice-sheet-formats".to_owned(),
        source_schema_version: 1,
        reviewed_candidate_count: 1,
        selected_candidates: vec![source.clone()],
        mappings: vec![DrawingSheetPresetImportMapping {
            source: source.clone(),
            target,
            kind: DrawingSheetPresetImportMappingKind::CreatedProjectPreset,
        }],
        conflicts: Vec::new(),
        skipped_candidates: Vec::new(),
    };
    receipt.validate().unwrap();

    let mut settings = DrawingSheetProjectSettings::default();
    settings.preset_import_receipts.push(receipt.clone());
    settings.validate().unwrap();
    let restored: DrawingSheetProjectSettings =
        serde_json::from_value(serde_json::to_value(&settings).unwrap()).unwrap();
    assert_eq!(restored.preset_import_receipts, vec![receipt]);

    let mut legacy = serde_json::to_value(DrawingSheetProjectSettings::default()).unwrap();
    legacy
        .as_object_mut()
        .unwrap()
        .remove("preset_import_receipts");
    let restored_legacy: DrawingSheetProjectSettings = serde_json::from_value(legacy).unwrap();
    assert!(restored_legacy.preset_import_receipts.is_empty());

    let over_limit = DrawingSheetProjectSettings {
        preset_import_receipts: vec![
            DrawingSheetPresetImportReceipt {
                source_digest_sha256: "b".repeat(64),
                source_schema: "rspice-sheet-formats".to_owned(),
                source_schema_version: 1,
                reviewed_candidate_count: 0,
                selected_candidates: Vec::new(),
                mappings: Vec::new(),
                conflicts: Vec::new(),
                skipped_candidates: Vec::new(),
            };
            MAX_DRAWING_SHEET_PRESET_IMPORT_RECEIPTS + 1
        ],
        ..DrawingSheetProjectSettings::default()
    };
    assert!(matches!(
        over_limit.validate(),
        Err(DesignManagementError::LimitExceeded {
            domain: "drawing sheet preset import receipts",
            actual,
            maximum: MAX_DRAWING_SHEET_PRESET_IMPORT_RECEIPTS,
        }) if actual == MAX_DRAWING_SHEET_PRESET_IMPORT_RECEIPTS + 1
    ));
}

#[test]
fn drawing_sheet_preset_import_receipt_requires_a_complete_outcome_partition() {
    let source = DrawingSheetPresetImportReference {
        preset_id: "portable-review-strip".to_owned(),
        scope: DrawingSheetPresetScope::User,
    };
    let incomplete = DrawingSheetPresetImportReceipt {
        source_digest_sha256: "c".repeat(64),
        source_schema: "rspice-sheet-formats".to_owned(),
        source_schema_version: 1,
        reviewed_candidate_count: 1,
        selected_candidates: vec![source],
        mappings: Vec::new(),
        conflicts: Vec::new(),
        skipped_candidates: Vec::new(),
    };
    assert!(matches!(
        incomplete.validate(),
        Err(DesignManagementError::NumericRange(
            "drawing sheet preset import selected-candidate outcome"
        ))
    ));
}

#[test]
fn preset_storage_strips_authored_values_and_canonicalizes_custom_identity() {
    let mut format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiB,
        SchematicPageOrientation::Landscape,
    );
    format = format
        .try_update(|draft| {
            draft.title_block.scale = DrawingSheetScale::Ratio {
                drawing_units: 2,
                reality_units: 5,
            };
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = "Confidential power tree".to_owned();
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::DrawnBy)
                .unwrap()
                .value = "Engineer A".to_owned();
        })
        .unwrap();
    let preset = DrawingSheetPreset {
        id: "review-board".to_owned(),
        name: "Review board".to_owned(),
        scope: DrawingSheetPresetScope::Project,
        format,
    }
    .normalized_for_storage()
    .unwrap();

    let AuthoredDrawingSheetSize::Custom { snapshot } = &preset.format.authored_size else {
        panic!("preset authority must materialize a custom snapshot");
    };
    assert_eq!(snapshot.preset_id.as_deref(), Some("review-board"));
    assert_eq!(snapshot.name, "Review board");
    assert!(
        preset
            .format
            .title_block
            .fields
            .values()
            .all(|field| field.value.is_empty())
    );
    assert_eq!(
        preset.format.title_block.scale,
        DrawingSheetScale::Ratio {
            drawing_units: 2,
            reality_units: 5,
        }
    );
    preset.validate().unwrap();
}

#[test]
fn legacy_scale_mirror_and_missing_preset_references_normalize_safely() {
    // Builds that lettered a Scale cell mirrored the scale into its field
    // string; the field is retired now, so the stale copy goes with it.
    let mut legacy_format = serde_json::to_value(SchematicSheetFormat::default()).unwrap();
    legacy_format["title_block"]["fields"]["scale"] =
        serde_json::json!({ "value": "stale 99:1", "visible": true });
    let restored_format: SchematicSheetFormat = serde_json::from_value(legacy_format).unwrap();
    assert!(
        !restored_format
            .title_block
            .fields
            .contains_key(&DrawingSheetTitleFieldId::Scale)
    );
    assert_eq!(
        restored_format.title_block.scale,
        DrawingSheetScale::Ratio {
            drawing_units: 1,
            reality_units: 1,
        }
    );

    let missing = SchematicSheetFormat::try_custom(
        "Missing review strip",
        210_000,
        594_000,
        SchematicPageOrientation::Landscape,
    )
    .unwrap()
    .try_update(|draft| {
        draft.inheritance = DrawingSheetInheritance::ProjectDefault;
        let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size else {
            unreachable!();
        };
        snapshot.preset_id = Some("missing-review-strip".to_owned());
        snapshot.source_preset_unavailable = false;
    })
    .unwrap();
    let settings = DrawingSheetProjectSettings {
        default_format: missing,
        ..DrawingSheetProjectSettings::default()
    };
    let restored: DrawingSheetProjectSettings =
        serde_json::from_value(serde_json::to_value(settings).unwrap()).unwrap();
    let AuthoredDrawingSheetSize::Custom { snapshot } = &restored.default_format.authored_size
    else {
        panic!("missing custom reference must retain its captured geometry");
    };
    assert_eq!(snapshot.preset_id.as_deref(), Some("missing-review-strip"));
    assert!(snapshot.source_preset_unavailable);
}

#[test]
fn durable_sheet_title_rejects_blank_updates_and_migrates_legacy_records() {
    let mut catalog = SheetCatalog::default();
    let id = catalog.create_sheet(sheet("Control", 1), None).unwrap();
    let revision = catalog.find(id).unwrap().revision();
    assert!(matches!(
        catalog.update_sheet_page_format(id, revision, SchematicSheetFormat::default()),
        Err(DesignManagementError::InvalidText {
            field: "authored drawing sheet title",
            ..
        })
    ));

    let mut legacy = serde_json::to_value(&catalog).unwrap();
    legacy["sheets"][0]["page_format"]["title_block"]["fields"]["sheet-title"]["value"] =
        serde_json::Value::String(String::new());
    let restored: SheetCatalog = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        restored.find(id).unwrap().page_format().title_block.fields
            [&DrawingSheetTitleFieldId::SheetTitle]
            .value,
        "Control"
    );
}

#[test]
fn import_conflict_resolution_must_match_its_durable_outcome() {
    let source = DrawingSheetPresetImportReference {
        preset_id: "managed-panel".to_owned(),
        scope: DrawingSheetPresetScope::Organization,
    };
    let target = DrawingSheetPresetImportReference {
        preset_id: "managed-panel-project".to_owned(),
        scope: DrawingSheetPresetScope::Project,
    };
    let mut receipt = DrawingSheetPresetImportReceipt {
        source_digest_sha256: "d".repeat(64),
        source_schema: "rspice-sheet-formats".to_owned(),
        source_schema_version: 1,
        reviewed_candidate_count: 1,
        selected_candidates: vec![source.clone()],
        mappings: vec![DrawingSheetPresetImportMapping {
            source: source.clone(),
            target,
            kind: DrawingSheetPresetImportMappingKind::CreatedProjectPreset,
        }],
        conflicts: vec![DrawingSheetPresetImportConflict {
            source,
            existing: None,
            missing_managed_dependency: true,
            resolution: DrawingSheetPresetImportResolution::ReplaceManagedDependencies,
        }],
        skipped_candidates: Vec::new(),
    };
    receipt.validate().unwrap();
    receipt.conflicts[0].resolution = DrawingSheetPresetImportResolution::MapExisting;
    assert!(matches!(
        receipt.validate(),
        Err(DesignManagementError::NumericRange(
            "drawing sheet preset import existing conflict outcome"
        ))
    ));
}

#[test]
fn project_title_block_values_have_one_canonical_authority_and_migrate_legacy_defaults() {
    let mut settings = DrawingSheetProjectSettings::default();
    settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::Organization,
        "RSpice Engineering".to_owned(),
    );
    settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::DocumentId,
        "PSAFE-SCH-001".to_owned(),
    );
    settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::Classification,
        "Internal".to_owned(),
    );
    settings.validate().unwrap();
    let round_trip: DrawingSheetProjectSettings =
        serde_json::from_value(serde_json::to_value(&settings).unwrap()).unwrap();
    assert_eq!(round_trip, settings);

    let mut invalid = settings.clone();
    invalid
        .title_block_field_values
        .insert(DrawingSheetTitleFieldId::Scale, "2:1".to_owned());
    assert!(matches!(
        invalid.validate(),
        Err(DesignManagementError::NumericRange(
            "project drawing sheet title field set"
        ))
    ));

    let mut legacy = serde_json::to_value(DrawingSheetProjectSettings::default()).unwrap();
    let legacy_object = legacy.as_object_mut().unwrap();
    legacy_object.remove("title_block_field_values");
    legacy_object["default_format"]["title_block"]["fields"]["organization"]["value"] =
        serde_json::Value::String("Legacy organization".to_owned());
    let migrated: DrawingSheetProjectSettings = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        migrated
            .title_block_field_values
            .get(&DrawingSheetTitleFieldId::Organization)
            .map(String::as_str),
        Some("Legacy organization")
    );
    assert_eq!(
        migrated.default_format.title_block.fields[&DrawingSheetTitleFieldId::Organization].value,
        ""
    );
}

fn substitution(cell: &str, qualification: VariantQualificationState) -> ComponentSubstitution {
    ComponentSubstitution {
        library: "project".to_owned(),
        cell: cell.to_owned(),
        view: "schematic".to_owned(),
        value_override: None,
        model_section: None,
        port_equivalence_digest: Some(ContentDigest::from_bytes([7; 32])),
        qualification,
    }
}

fn variant_draft(
    name: &str,
    parent_id: Option<AssemblyVariantId>,
    overrides: BTreeMap<SchematicObjectKey, VariantObjectOverride>,
) -> AssemblyVariantDraft {
    AssemblyVariantDraft {
        name: name.to_owned(),
        parent_id,
        inheritance: VariantInheritance::OverrideChangedObjectsOnly,
        qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
        overrides,
    }
}

fn annotation_object(id: u64, reference: &str, x: i64) -> AnnotationObject {
    AnnotationObject {
        object: object_key(id),
        current_reference: reference.to_owned(),
        device_family: "R".to_owned(),
        sheet_id: None,
        hierarchy_path: "/top".to_owned(),
        position: AnnotationPosition { x, y: 0 },
        connectivity_order: Some(id),
        locked: false,
        external: false,
        imported: false,
    }
}

fn object_key(id: u64) -> SchematicObjectKey {
    SchematicObjectKey::new("work/top/schematic", id).unwrap()
}

#[test]
fn empty_catalog_round_trips_and_rejects_unknown_fields() {
    let catalog = DesignManagementCatalog::default();
    assert!(catalog.is_empty());
    catalog.validate().expect("default catalog validates");
    let encoded = serde_json::to_string(&catalog).expect("serialize catalog");
    let decoded: DesignManagementCatalog =
        serde_json::from_str(&encoded).expect("deserialize catalog");
    assert_eq!(decoded, catalog);
    assert_eq!(
        decoded.semantic_digest().unwrap(),
        catalog.semantic_digest().unwrap()
    );

    let mut value = serde_json::to_value(catalog).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("invented".to_owned(), serde_json::Value::Bool(true));
    assert!(serde_json::from_value::<DesignManagementCatalog>(value).is_err());
}

#[test]
fn sheet_order_changes_presentation_without_changing_stable_identity() {
    let mut catalog = SheetCatalog::default();
    let afe = catalog.create_sheet(sheet("AFE core", 1), None).unwrap();
    let bias = catalog
        .create_sheet(sheet("Bias and reference", 2), Some(afe))
        .unwrap();
    let afe_digest = catalog.find(afe).unwrap().semantic_digest();
    let revision = catalog.revision();

    catalog
        .reorder(
            revision,
            vec![bias, afe],
            ReorderPageNumbering::RetainExplicitPageNumbers,
            ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
        )
        .unwrap();

    assert_eq!(
        catalog
            .sheets()
            .iter()
            .map(DesignSheet::id)
            .collect::<Vec<_>>(),
        vec![bias, afe]
    );
    assert_eq!(catalog.find(afe).unwrap().semantic_digest(), afe_digest);
    assert!(matches!(
        catalog.reorder(
            revision,
            vec![afe, bias],
            ReorderPageNumbering::RetainExplicitPageNumbers,
            ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
        ),
        Err(DesignManagementError::RevisionConflict { .. })
    ));
}

#[test]
fn sheet_move_and_reconciliation_are_atomic_and_remove_dead_ports() {
    let mut catalog = SheetCatalog::default();
    let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
    catalog
        .assign_objects(catalog.revision(), source, [1, 2])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), destination, [3])
        .unwrap();
    let receipt = catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![1],
            destination_sheet_id: destination,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: "VREF".to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: source,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: 2,
                            point: Point::new(14, 9),
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: destination,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: 3,
                            terminal_name: "VREF".to_owned(),
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .unwrap();
    assert_eq!(catalog.sheet_for_object(1), Some(destination));
    assert_eq!(receipt.created_port_ids.len(), 1);
    let retained = catalog.cross_sheet_ports()[0].definition();
    assert_eq!(
        retained.first.anchor,
        CrossSheetPortAnchor::WirePoint {
            wire_id: 2,
            point: Point::new(14, 9),
        }
    );
    assert_eq!(retained.second.object_id(), 3);

    let reconciled = catalog
        .reconcile_object_assignments(catalog.revision(), [1, 4], Some(destination))
        .unwrap();
    assert_eq!(reconciled.added_assignments, 1);
    assert_eq!(reconciled.removed_assignments, 2);
    assert_eq!(reconciled.removed_cross_sheet_ports, 1);
    assert_eq!(catalog.sheet_for_object(4), Some(destination));
    assert!(catalog.cross_sheet_ports().is_empty());
}

#[test]
fn typed_cross_sheet_anchor_rejects_ambiguous_terminal_without_mutation() {
    let mut catalog = SheetCatalog::default();
    let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
    catalog
        .assign_objects(catalog.revision(), source, [1, 2])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), destination, [3])
        .unwrap();
    let before = catalog.clone();
    let result = catalog.move_selection(MoveSelectionRequest {
        expected_catalog_revision: catalog.revision(),
        object_ids: vec![1],
        destination_sheet_id: destination,
        boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
            ports: vec![CrossSheetPortDefinition {
                net_name: "VREF".to_owned(),
                first: CrossSheetPortEndpoint {
                    sheet_id: source,
                    anchor: CrossSheetPortAnchor::ComponentTerminal {
                        component_id: 2,
                        terminal_name: String::new(),
                    },
                },
                second: CrossSheetPortEndpoint {
                    sheet_id: destination,
                    anchor: CrossSheetPortAnchor::WirePoint {
                        wire_id: 3,
                        point: Point::new(0, 0),
                    },
                },
                direction: CrossSheetPortDirection::Output,
                signal_type: CrossSheetSignalType::Analog,
                discipline: CrossSheetDiscipline::Electrical,
            }],
        },
    });
    assert!(matches!(
        result,
        Err(DesignManagementError::InvalidText {
            field: "component terminal",
            ..
        })
    ));
    assert_eq!(catalog, before);
}

#[test]
fn verified_empty_boundary_move_is_distinct_from_empty_explicit_ports() {
    let mut catalog = SheetCatalog::default();
    let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
    catalog
        .assign_objects(catalog.revision(), source, [1])
        .unwrap();
    let receipt = catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![1],
            destination_sheet_id: destination,
            boundary_resolution: MoveBoundaryResolution::VerifiedNoBoundaryNets,
        })
        .unwrap();
    assert!(receipt.created_port_ids.is_empty());
    assert_eq!(catalog.sheet_for_object(1), Some(destination));

    let before = catalog.clone();
    assert!(matches!(
        catalog.move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![1],
            destination_sheet_id: source,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts { ports: Vec::new() },
        }),
        Err(DesignManagementError::EmptyExplicitBoundaryPorts)
    ));
    assert_eq!(catalog, before);
}

#[test]
fn variant_resolution_is_immutable_and_comparison_does_not_mutate() {
    let mut catalog = AssemblyVariantCatalog::default();
    let base = catalog
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                object_key(10),
                VariantObjectOverride::Substitute {
                    replacement: substitution(
                        "resistor_industrial",
                        VariantQualificationState::Current,
                    ),
                },
            )]),
        ))
        .unwrap();
    let child = catalog
        .create(variant_draft(
            "Automotive",
            Some(base),
            BTreeMap::from([(
                object_key(11),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-42".to_owned(),
                },
            )]),
        ))
        .unwrap();
    let before = catalog.clone();
    let resolved = catalog.resolve(child).unwrap();
    assert_eq!(resolved.lineage.len(), 2);
    assert_eq!(resolved.overrides.len(), 2);
    let comparison = catalog.compare(base, child).unwrap();
    assert_eq!(comparison.differences.len(), 1);
    assert_eq!(catalog, before, "comparison must be read-only");

    let base_revision = catalog.find(base).unwrap().revision();
    assert!(matches!(
        catalog.update(
            base,
            base_revision,
            variant_draft("Industrial revised", None, BTreeMap::new()),
        ),
        Err(DesignManagementError::VariantHasDependents(id)) if id == base
    ));
    assert_eq!(catalog, before);
}

#[test]
fn substitution_matrix_enforces_qualification_before_any_commit() {
    let mut catalog = AssemblyVariantCatalog::default();
    let id = catalog
        .create(variant_draft("Industrial", None, BTreeMap::new()))
        .unwrap();
    let revision = catalog.find(id).unwrap().revision();
    let before = catalog.clone();
    let result = catalog.apply_substitution_matrix(
        vec![VariantMatrixEdit {
            variant_id: id,
            expected_revision: revision,
            object: object_key(44),
            replacement: Some(substitution(
                "candidate",
                VariantQualificationState::ReviewRequired,
            )),
        }],
        MissingReplacementPolicy::Block,
        ModelEquivalencePolicy::RequireQualifiedReplacement,
    );
    assert!(matches!(
        result,
        Err(DesignManagementError::UnqualifiedReplacement(object)) if object == object_key(44)
    ));
    assert_eq!(catalog, before);
}

#[test]
fn renumber_preview_is_deterministic_and_commit_retains_immutable_mapping() {
    let mut state = AnnotationState::default();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![
            annotation_object(2, "R20", 20),
            annotation_object(1, "R10", 10),
        ],
    };
    let first = state.preview_renumbering(&request).unwrap();
    let second = state.preview_renumbering(&request).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        first.mappings.get(&object_key(1)).unwrap().new_reference,
        "R1"
    );
    assert_eq!(
        first.mappings.get(&object_key(2)).unwrap().new_reference,
        "R2"
    );

    let id = state.commit_renumbering(&first, &request).unwrap();
    let retained = state.journal().last().unwrap();
    assert_eq!(retained.id(), id);
    assert_eq!(retained.mappings(), &first.mappings);
    let encoded = serde_json::to_string(&state).unwrap();
    let restored: AnnotationState = serde_json::from_str(&encoded).unwrap();
    assert_eq!(restored, state);
}

#[test]
fn the_design_root_scope_encloses_every_occurrence() {
    let state = AnnotationState::default();
    let mut nested = annotation_object(1, "R10", 10);
    nested.hierarchy_path = "/X1".to_owned();
    let mut deeper = annotation_object(2, "R20", 20);
    deeper.hierarchy_path = "/X1/X2".to_owned();
    let request = RenumberRequest {
        scope: RenumberScope::CurrentHierarchy {
            path: "/".to_owned(),
        },
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![nested, deeper],
    };

    let preview = state.preview_renumbering(&request).unwrap();
    assert_eq!(preview.mappings.len(), 2);

    let scoped = RenumberRequest {
        scope: RenumberScope::CurrentHierarchy {
            path: "/X1".to_owned(),
        },
        ..request
    };
    let preview = state.preview_renumbering(&scoped).unwrap();
    assert_eq!(
        preview.mappings.len(),
        2,
        "an occurrence scope encloses itself and everything below it"
    );
}

#[test]
fn invalid_annotation_policy_is_rejected_without_partial_mutation() {
    let mut state = AnnotationState::default();
    let before = state.clone();
    let definition = AnnotationPolicyDefinition {
        reserved_ranges: vec![
            AnnotationReservedRange {
                scope: AnnotationRangeScope::Project,
                prefixes: vec!["R".to_owned()],
                first: 1,
                last: 10,
            },
            AnnotationReservedRange {
                scope: AnnotationRangeScope::Project,
                prefixes: vec!["R".to_owned()],
                first: 5,
                last: 15,
            },
        ],
        ..AnnotationPolicyDefinition::default()
    };
    let revision = state.policy().revision();
    assert!(matches!(
        state.update_policy(revision, definition),
        Err(DesignManagementError::OverlappingAnnotationRanges)
    ));
    assert_eq!(state, before);
}

#[test]
fn effective_annotation_folds_partial_journal_entries_in_sequence_order() {
    let mut state = AnnotationState::default();
    let first_sheet = SheetId::new();
    let second_sheet = SheetId::new();
    let mut first_object = annotation_object(1, "R10", 10);
    first_object.sheet_id = Some(first_sheet);
    let mut second_object = annotation_object(2, "R20", 20);
    second_object.sheet_id = Some(second_sheet);
    let first_request = RenumberRequest {
        scope: RenumberScope::CurrentSheet {
            sheet_id: first_sheet,
        },
        order: RenumberOrder::SheetThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![first_object.clone(), second_object.clone()],
    };
    let first_preview = state.preview_renumbering(&first_request).unwrap();
    state
        .commit_renumbering(&first_preview, &first_request)
        .unwrap();

    first_object.current_reference = first_preview.mappings[&object_key(1)].new_reference.clone();
    let second_request = RenumberRequest {
        scope: RenumberScope::CurrentSheet {
            sheet_id: second_sheet,
        },
        order: RenumberOrder::SheetThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![first_object, second_object],
    };
    let second_preview = state.preview_renumbering(&second_request).unwrap();
    state
        .commit_renumbering(&second_preview, &second_request)
        .unwrap();

    let effective = state.effective_mappings();
    assert_eq!(effective.len(), 2);
    assert_eq!(effective[&object_key(1)].new_reference, "R1");
    assert_eq!(effective[&object_key(2)].new_reference, "R2");
}

#[test]
fn annotation_authority_rejects_cycles_and_unrelated_object_conflation() {
    let mut state = AnnotationState::default();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![
            annotation_object(1, "R10", 10),
            annotation_object(2, "R20", 20),
        ],
    };
    let preview = state.preview_renumbering(&request).unwrap();
    state.commit_renumbering(&preview, &request).unwrap();
    let target = object_key(3);
    state.object_authorities.insert(
        object_key(1),
        AnnotationObjectAuthority::Redirect {
            target: target.clone(),
        },
    );
    state.object_authorities.insert(
        object_key(2),
        AnnotationObjectAuthority::Redirect {
            target: target.clone(),
        },
    );
    assert!(matches!(
        state.validate(),
        Err(DesignManagementError::AnnotationAuthorityConflation {
            target: ref actual,
            ..
        }) if actual == &target
    ));

    let mut cycle = AnnotationState::default();
    cycle.object_authorities.insert(
        object_key(1),
        AnnotationObjectAuthority::Redirect {
            target: object_key(2),
        },
    );
    cycle.object_authorities.insert(
        object_key(2),
        AnnotationObjectAuthority::Redirect {
            target: object_key(1),
        },
    );
    assert!(matches!(
        cycle.validate(),
        Err(DesignManagementError::AnnotationAuthorityCycle(_))
    ));
}

#[test]
fn hierarchy_audit_records_cycle_and_protected_boundary_failures() {
    let mut catalog = DesignManagementCatalog::default();
    let request = HierarchyAuditRequest {
        configuration: HierarchyAuditConfiguration::ActiveProject,
        view_checks: HierarchyViewChecks::AllDeclaredFallbacks,
        protected_boundaries: ProtectedBoundaryChecks::ValidateSignaturesAndPins,
        subjects: vec![
            HierarchyAuditSubject {
                instance_path: "/top".to_owned(),
                cell_name: "top".to_owned(),
                design_view: "schematic".to_owned(),
                declared_fallbacks: vec!["schematic".to_owned()],
                resolved_simulation_view: Some("schematic".to_owned()),
                fallback_used: Some("schematic".to_owned()),
                child_instance_paths: vec!["/top/X1".to_owned()],
                protected_boundary_id: None,
            },
            HierarchyAuditSubject {
                instance_path: "/top/X1".to_owned(),
                cell_name: "vendor".to_owned(),
                design_view: "symbol".to_owned(),
                declared_fallbacks: vec!["protected-spice".to_owned()],
                resolved_simulation_view: Some("protected-spice".to_owned()),
                fallback_used: Some("protected-spice".to_owned()),
                child_instance_paths: vec!["/top".to_owned()],
                protected_boundary_id: Some("vendor-boundary".to_owned()),
            },
        ],
        boundary_evidence: vec![ProtectedBoundaryEvidence {
            boundary_id: "vendor-boundary".to_owned(),
            signature_valid: false,
            pins_match: false,
        }],
    };
    let id = catalog.run_and_record_hierarchy_audit(&request).unwrap();
    let receipt = catalog.hierarchy_audits().last().unwrap();
    assert_eq!(receipt.id(), id);
    assert!(!receipt.passed());
    let kinds = receipt
        .findings()
        .iter()
        .map(|finding| finding.kind)
        .collect::<BTreeSet<_>>();
    assert!(kinds.contains(&HierarchyAuditFindingKind::HierarchyCycle));
    assert!(kinds.contains(&HierarchyAuditFindingKind::InvalidProtectedBoundarySignature));
    assert!(kinds.contains(&HierarchyAuditFindingKind::ProtectedBoundaryPinMismatch));
}

#[test]
fn cell_view_sheet_ownership_and_reviewed_publish_are_deterministic() {
    let mut live = DesignManagementCatalog::default();
    let sheet_id = live
        .bootstrap_for_cell_view(" Project/Top/Schematic ", "Main", [8, 9])
        .unwrap();
    assert_eq!(
        live.sheet_for_object_or_active("project/top/schematic", 8),
        Some(sheet_id)
    );
    assert_eq!(
        live.sheet_for_object_or_active("PROJECT/TOP/SCHEMATIC", 500),
        Some(sheet_id),
        "legacy unassigned objects inherit active/first sheet without mutation"
    );
    let original_revision = live.revision();
    let mut candidate = live.clone();
    candidate
        .sheet_catalog_mut("project/top/schematic")
        .unwrap()
        .create_sheet(sheet("Power", 2), Some(sheet_id))
        .unwrap();
    let new_revision = live
        .publish_reviewed_candidate(original_revision, candidate)
        .unwrap();
    assert_eq!(new_revision, original_revision + 1);
    assert_eq!(
        live.sheet_catalog("project/top/schematic")
            .unwrap()
            .sheets()
            .len(),
        2
    );
}

#[test]
fn cell_rename_and_delete_remap_sheet_catalog_ownership_atomically() {
    let mut catalog = DesignManagementCatalog::default();
    catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    catalog
        .bootstrap_for_cell_view("work/amp/testbench", "Bench", [2])
        .unwrap();
    let original_revision = catalog.revision();
    let renamed = catalog
        .rename_cell_sheet_catalogs("work", "amp", "amp_rev_b")
        .unwrap();
    assert_eq!(renamed.affected_sheet_catalogs, 2);
    assert_eq!(renamed.catalog_revision, original_revision + 1);
    assert!(catalog.sheet_catalog("work/amp/schematic").is_none());
    assert!(catalog.sheet_catalog("work/amp_rev_b/schematic").is_some());

    let removed = catalog
        .remove_sheet_catalog_for_view("work/amp_rev_b/testbench")
        .unwrap();
    assert_eq!(removed.affected_sheet_catalogs, 1);
    let removed = catalog
        .remove_sheet_catalogs_for_cell("work", "amp_rev_b")
        .unwrap();
    assert_eq!(removed.affected_sheet_catalogs, 1);
    assert!(catalog.sheet_catalogs().is_empty());
    let revision = catalog.revision();
    let copy = catalog
        .copy_cell_sheet_catalogs("work", "missing", "work", "copy")
        .unwrap();
    assert_eq!(copy.copied_sheet_catalogs, 0);
    assert_eq!(copy.catalog_revision, revision);
    assert_eq!(catalog.revision(), revision);
}

#[test]
fn deleting_sheet_ownership_blocks_while_annotation_range_references_it() {
    let mut catalog = DesignManagementCatalog::default();
    let sheet_id = catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    let mut policy = catalog.annotation().policy().definition().clone();
    policy.reserved_ranges.push(AnnotationReservedRange {
        scope: AnnotationRangeScope::Sheet { sheet_id },
        prefixes: vec!["R".to_owned()],
        first: 1,
        last: 399,
    });
    let revision = catalog.annotation().policy().revision();
    catalog
        .annotation_mut()
        .update_policy(revision, policy)
        .unwrap();
    let before = catalog.clone();
    assert!(matches!(
        catalog.remove_sheet_catalog_for_view("work/amp/schematic"),
        Err(DesignManagementError::SheetCatalogReferenced(id)) if id == sheet_id
    ));
    assert_eq!(catalog, before);
}

#[test]
fn cell_rename_remaps_live_variant_and_annotation_object_owners() {
    let mut catalog = DesignManagementCatalog::default();
    catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    let old_object = SchematicObjectKey::new("work/amp/schematic", 1).unwrap();
    let new_object = SchematicObjectKey::new("work/amp_rev_b/schematic", 1).unwrap();
    let variant = catalog
        .variants_mut()
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                old_object.clone(),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-9".to_owned(),
                },
            )]),
        ))
        .unwrap();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: old_object.clone(),
            current_reference: "R10".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: catalog
                .sheet_catalog("work/amp/schematic")
                .unwrap()
                .active_sheet_id(),
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = catalog.annotation().preview_renumbering(&request).unwrap();
    catalog
        .annotation_mut()
        .commit_renumbering(&preview, &request)
        .unwrap();

    let receipt = catalog
        .rename_cell_sheet_catalogs("work", "amp", "amp_rev_b")
        .unwrap();
    assert_eq!(receipt.remapped_variant_objects, 1);
    assert_eq!(receipt.remapped_annotation_objects, 1);
    let resolved = catalog.variants().resolve(variant).unwrap();
    assert!(!resolved.overrides.contains_key(&old_object));
    assert!(resolved.overrides.contains_key(&new_object));
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp_rev_b/schematic", 1)
            .unwrap()
            .is_some()
    );
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp/schematic", 1)
            .unwrap()
            .is_none(),
        "renamed ownership must not remain effective under the old key"
    );
    assert_eq!(catalog.annotation().journal().len(), 1);
    assert!(matches!(
        catalog.annotation().object_authorities().get(&old_object),
        Some(AnnotationObjectAuthority::Redirect { target }) if target == &new_object
    ));
    let encoded = serde_json::to_string(&catalog).unwrap();
    let restored: DesignManagementCatalog = serde_json::from_str(&encoded).unwrap();
    assert_eq!(restored, catalog);
}

#[test]
fn cell_delete_blocks_live_variant_then_tombstones_annotation_without_rewriting_history() {
    let mut catalog = DesignManagementCatalog::default();
    catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    let object = SchematicObjectKey::new("work/amp/schematic", 1).unwrap();
    let variant = catalog
        .variants_mut()
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                object.clone(),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-11".to_owned(),
                },
            )]),
        ))
        .unwrap();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: object.clone(),
            current_reference: "R8".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = catalog.annotation().preview_renumbering(&request).unwrap();
    catalog
        .annotation_mut()
        .commit_renumbering(&preview, &request)
        .unwrap();

    let before = catalog.clone();
    assert!(matches!(
        catalog.remove_sheet_catalogs_for_cell("work", "amp"),
        Err(DesignManagementError::LiveVariantObjectReference { variant: id, object: ref key })
            if id == variant && key == &object
    ));
    assert_eq!(catalog, before, "blocked deletion must be atomic");

    let revision = catalog.variants().find(variant).unwrap().revision();
    catalog
        .variants_mut()
        .update(
            variant,
            revision,
            variant_draft("Industrial", None, BTreeMap::new()),
        )
        .unwrap();
    let removed = catalog
        .remove_sheet_catalogs_for_cell("work", "amp")
        .unwrap();
    assert_eq!(removed.affected_sheet_catalogs, 1);
    assert_eq!(removed.remapped_annotation_objects, 1);
    assert_eq!(catalog.annotation().journal().len(), 1);
    assert!(catalog.annotation().effective_mappings().is_empty());
    assert!(matches!(
        catalog.annotation().object_authorities().get(&object),
        Some(AnnotationObjectAuthority::Tombstone)
    ));
    assert!(matches!(
        catalog.annotation().preview_renumbering(&request),
        Err(DesignManagementError::InactiveAnnotationObjectAuthority(ref key)) if key == &object
    ));
}

#[test]
fn cell_copy_regenerates_sheet_port_identity_and_clones_sheet_annotation_policy() {
    let mut catalog = DesignManagementCatalog::default();
    let main = catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1, 2, 3])
        .unwrap();
    let sheets = catalog.sheet_catalog_mut("work/amp/schematic").unwrap();
    let auxiliary = sheets
        .create_sheet(sheet("Auxiliary", 2), Some(main))
        .unwrap();
    sheets
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: sheets.revision(),
            object_ids: vec![3],
            destination_sheet_id: auxiliary,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: "BIAS".to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: main,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: 1,
                            terminal_name: "BIAS_OUT".to_owned(),
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: auxiliary,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: 3,
                            point: Point::new(21, -4),
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .unwrap();
    let source_port = sheets.cross_sheet_ports()[0].id();
    let mut policy = catalog.annotation().policy().definition().clone();
    policy.reserved_ranges.push(AnnotationReservedRange {
        scope: AnnotationRangeScope::Sheet { sheet_id: main },
        prefixes: vec!["R".to_owned()],
        first: 1,
        last: 399,
    });
    let policy_revision = catalog.annotation().policy().revision();
    catalog
        .annotation_mut()
        .update_policy(policy_revision, policy)
        .unwrap();
    let annotation_request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: SchematicObjectKey::new("work/amp/schematic", 1).unwrap(),
            current_reference: "R12".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: Some(main),
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let annotation_preview = catalog
        .annotation()
        .preview_renumbering(&annotation_request)
        .unwrap();
    catalog
        .annotation_mut()
        .commit_renumbering(&annotation_preview, &annotation_request)
        .unwrap();
    let variant = catalog
        .variants_mut()
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                SchematicObjectKey::new("work/amp/schematic", 1).unwrap(),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-1".to_owned(),
                },
            )]),
        ))
        .unwrap();

    let receipt = catalog
        .copy_cell_sheet_catalogs("work", "amp", "work", "amp_copy")
        .unwrap();
    assert_eq!(receipt.copied_sheet_catalogs, 1);
    assert_ne!(receipt.sheet_identity_map[&main], main);
    assert_ne!(receipt.port_identity_map[&source_port], source_port);
    let copied = catalog.sheet_catalog("work/amp_copy/schematic").unwrap();
    assert_eq!(
        copied.sheet_for_object(1),
        Some(receipt.sheet_identity_map[&main])
    );
    assert_eq!(
        copied.cross_sheet_ports()[0].definition().first.sheet_id,
        receipt.sheet_identity_map[&main]
    );
    assert_eq!(
        copied.cross_sheet_ports()[0].definition().first.anchor,
        CrossSheetPortAnchor::ComponentTerminal {
            component_id: 1,
            terminal_name: "BIAS_OUT".to_owned(),
        }
    );
    assert!(
        catalog
            .annotation()
            .policy()
            .definition()
            .reserved_ranges
            .iter()
            .any(|range| range.scope
                == AnnotationRangeScope::Sheet {
                    sheet_id: receipt.sheet_identity_map[&main],
                })
    );
    let resolved = catalog.variants().resolve(variant).unwrap();
    assert!(
        resolved
            .overrides
            .contains_key(&SchematicObjectKey::new("work/amp/schematic", 1).unwrap())
    );
    assert!(
        !resolved
            .overrides
            .contains_key(&SchematicObjectKey::new("work/amp_copy/schematic", 1).unwrap())
    );
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp/schematic", 1)
            .unwrap()
            .is_some()
    );
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp_copy/schematic", 1)
            .unwrap()
            .is_none()
    );
}

#[test]
fn drawing_sheet_transactions_retain_complete_revisioned_project_receipts() {
    let mut catalog = DesignManagementCatalog::default();
    let sheet_id = catalog
        .bootstrap_for_cell_view("work/top/schematic", "Main", [])
        .unwrap();
    let format = catalog
        .sheet_catalog("work/top/schematic")
        .unwrap()
        .find(sheet_id)
        .unwrap()
        .page_format()
        .clone();
    let committed_revision = catalog.revision() + 1;
    let receipt = DrawingSheetTransactionReceipt {
        catalog_revision: committed_revision,
        kind: DrawingSheetTransactionKind::PageSetup,
        owner_cell_view_key: "work/top/schematic".to_owned(),
        source_format_digest: format.content_digest().unwrap(),
        selected_sheet_ids: vec![sheet_id],
        applied_sheet_ids: vec![sheet_id],
        unchanged_sheet_ids: Vec::new(),
        skipped: Vec::new(),
        project_default_changed: false,
        project_preset_saved: false,
        project_settings_changed: false,
    };
    assert_eq!(
        catalog
            .record_drawing_sheet_transaction(catalog.revision(), receipt.clone())
            .unwrap(),
        committed_revision
    );
    assert_eq!(
        catalog
            .drawing_sheet_settings()
            .transaction_receipts
            .as_slice(),
        &[receipt]
    );

    let restored: DesignManagementCatalog =
        serde_json::from_value(serde_json::to_value(&catalog).unwrap()).unwrap();
    assert_eq!(
        restored.drawing_sheet_settings().transaction_receipts.len(),
        1
    );

    let mut incomplete = restored
        .drawing_sheet_settings()
        .transaction_receipts
        .first()
        .unwrap()
        .clone();
    incomplete.applied_sheet_ids.clear();
    assert!(matches!(
        incomplete.validate(),
        Err(DesignManagementError::NumericRange(
            "drawing sheet transaction disposition coverage"
        ))
    ));
}

#[test]
fn drawing_sheet_title_cell_budget_is_physical_and_rotation_stable() {
    let upright = SchematicSheetFormat::default();
    let upright_geometry = upright.geometry().unwrap();
    let upright_capacity = drawing_sheet_title_cell_capacity(
        &upright,
        &upright_geometry,
        14,
        upright.authored_size.lettering().data_um,
    )
    .unwrap();
    let rotated = upright
        .try_update(|draft| {
            draft.title_block.rotation = DrawingSheetTitleBlockRotation::Clockwise90;
        })
        .unwrap();
    let rotated_geometry = rotated.geometry().unwrap();

    assert_eq!(
        drawing_sheet_title_block_rows(DrawingSheetTitleBlockTemplate::Compact),
        Some(3)
    );
    assert_eq!(
        drawing_sheet_title_block_rows(DrawingSheetTitleBlockTemplate::Standard),
        Some(4)
    );
    assert_eq!(
        drawing_sheet_title_block_rows(DrawingSheetTitleBlockTemplate::OrganizationManaged),
        Some(5)
    );
    assert_eq!(
        drawing_sheet_title_block_rows(DrawingSheetTitleBlockTemplate::None),
        None
    );
    assert_eq!(
        drawing_sheet_title_cell_capacity(
            &rotated,
            &rotated_geometry,
            14,
            rotated.authored_size.lettering().data_um
        ),
        Some(upright_capacity),
        "rotating the complete block must not change a field's authored text budget"
    );
    assert!(
        drawing_sheet_title_cell_capacity(
            &upright,
            &upright_geometry,
            14,
            upright.authored_size.lettering().caption_um
        )
        .unwrap()
            > upright_capacity,
        "smaller caption lettering must buy more characters in the same cell"
    );
}

#[test]
fn a_sheet_authored_before_a_field_retired_still_loads_without_it() {
    let format = SchematicSheetFormat::default();
    let mut wire = serde_json::to_value(&format).unwrap();
    // Reintroduce the withdrawn field exactly as an older build wrote it.
    wire["title_block"]["fields"]["scale"] = serde_json::json!({ "value": "", "visible": true });
    let restored: SchematicSheetFormat = serde_json::from_value(wire).unwrap();
    assert!(
        !restored
            .title_block
            .fields
            .contains_key(&DrawingSheetTitleFieldId::Scale),
        "a retired field must be dropped rather than refused or rendered"
    );
    assert_eq!(
        restored.title_block.fields.len(),
        DrawingSheetTitleFieldId::ALL.len()
    );
    assert!(restored.validate().is_ok());
    assert!(
        !resolve_drawing_sheet_title_fields(&restored, &BTreeMap::new())
            .iter()
            .any(|field| field.id == DrawingSheetTitleFieldId::Scale)
    );

    // A sheet missing a field the product has gained is filled in, so the
    // same normalization covers both directions.
    let mut sparse = serde_json::to_value(&format).unwrap();
    sparse["title_block"]["fields"]
        .as_object_mut()
        .unwrap()
        .remove("document-id");
    let filled: SchematicSheetFormat = serde_json::from_value(sparse).unwrap();
    assert!(filled.validate().is_ok());
    assert!(
        filled
            .title_block
            .fields
            .contains_key(&DrawingSheetTitleFieldId::DocumentId)
    );
}

#[test]
fn an_organization_template_sealed_before_a_field_retired_still_loads() {
    let managed = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
        })
        .unwrap();
    // The built-in template exactly as a build that still offered a Scale
    // cell sealed it: fourteen fields, and a digest over all fourteen.
    let sealed_order = [
        "project",
        "cell-view",
        "sheet-title",
        "page",
        "revision",
        "format",
        "scale",
        "drawn-by",
        "checked-by",
        "approved-by",
        "date",
        "organization",
        "document-id",
        "classification",
    ];
    let sealed_digest = "51714aad10ff2f84fb2bb451212d0e39659310c069b00ff49f40afdde92b300d";
    let mut wire = serde_json::to_value(&managed).unwrap();
    wire["title_block"]["fields"]["scale"] = serde_json::json!({ "value": "", "visible": true });
    wire["title_block"]["managed_template"]["field_order"] = serde_json::json!(sealed_order);
    wire["title_block"]["managed_template"]["content_digest"] = serde_json::json!(sealed_digest);

    let restored: SchematicSheetFormat = serde_json::from_value(wire).unwrap();
    let snapshot = restored.title_block.managed_template.as_ref().unwrap();
    assert_eq!(
        snapshot.content_digest().to_string(),
        sealed_digest,
        "the seal is the record of what was released, so loading must not re-seal it"
    );
    // The retired field keeps its place in the seal and nowhere else, and a
    // field the seal predates follows every field it names.
    let gained = [
        DrawingSheetTitleFieldId::DocumentType,
        DrawingSheetTitleFieldId::DocumentStatus,
    ];
    let mut expected = DrawingSheetTitleFieldId::ALL
        .into_iter()
        .filter(|field| !gained.contains(field))
        .collect::<Vec<_>>();
    expected.extend(gained);
    assert_eq!(restored.title_block_field_order(), expected);
    assert!(
        !resolve_drawing_sheet_title_fields(&restored, &BTreeMap::new())
            .iter()
            .any(|field| field.id == DrawingSheetTitleFieldId::Scale)
    );
    let rewritten = serde_json::to_value(&restored).unwrap();
    assert_eq!(
        rewritten["title_block"]["managed_template"]["field_order"],
        serde_json::json!(sealed_order)
    );
    assert_eq!(
        serde_json::from_value::<SchematicSheetFormat>(rewritten).unwrap(),
        restored
    );

    let mut with_retired = DrawingSheetTitleFieldId::ALL.to_vec();
    with_retired.push(DrawingSheetTitleFieldId::Scale);
    assert!(
        DrawingSheetManagedTemplateSnapshot::try_new(
            "example.release-title-block",
            "2026.10",
            180_000,
            50_000,
            5,
            with_retired,
            Vec::new(),
        )
        .is_err(),
        "a template sealed today must not name a field the product withdrew"
    );
    let mut repeated = DrawingSheetTitleFieldId::ALL.to_vec();
    repeated.push(DrawingSheetTitleFieldId::Page);
    assert!(
        DrawingSheetManagedTemplateSnapshot::try_new(
            "example.release-title-block",
            "2026.10",
            180_000,
            50_000,
            5,
            repeated,
            Vec::new(),
        )
        .is_err()
    );
}

#[test]
fn a_new_sheet_shows_the_fields_iso_7200_requires_and_waits_on_the_rest() {
    let format = SchematicSheetFormat::default();
    let visible = resolve_drawing_sheet_title_fields(&format, &BTreeMap::new());
    // ISO 7200 makes the creator and the approval person mandatory, so a
    // sheet names both before anyone has signed it.
    for field in [
        DrawingSheetTitleFieldId::DrawnBy,
        DrawingSheetTitleFieldId::ApprovedBy,
    ] {
        assert!(
            visible
                .iter()
                .any(|resolved| resolved.id == field && resolved.value == "—"),
            "{field:?} is mandatory in ISO 7200 and must start on the sheet"
        );
    }
    // Checked by is not one of the standard's fields, Classification is set
    // by a release policy, and Paper size restates the sheet in hand.
    let hidden = [
        DrawingSheetTitleFieldId::CheckedBy,
        DrawingSheetTitleFieldId::Classification,
        DrawingSheetTitleFieldId::PaperSize,
    ];
    for field in hidden {
        assert!(
            !visible.iter().any(|resolved| resolved.id == field),
            "{field:?} must wait until a sheet asks for it"
        );
        assert!(
            !field.policy().required_visible,
            "{field:?} must remain optional so it can default to hidden"
        );
    }
    assert_eq!(
        visible.len(),
        DrawingSheetTitleFieldId::ALL.len() - hidden.len()
    );

    // A field a sheet does not spend is width the others keep, so more of
    // each value survives.
    let geometry = format.geometry().unwrap();
    let lettering = format.authored_size.lettering();
    let shown =
        drawing_sheet_title_cell_capacity(&format, &geometry, visible.len(), lettering.data_um)
            .unwrap();
    let crowded = drawing_sheet_title_cell_capacity(
        &format,
        &geometry,
        DrawingSheetTitleFieldId::ALL.len() + 1,
        lettering.data_um,
    )
    .unwrap();
    assert!(
        shown > crowded,
        "hiding the optional fields must widen the remaining cells: {shown} vs {crowded}"
    );
}

#[test]
fn drawing_sheet_lettering_is_iso_3098_and_its_budget_is_a_measurement() {
    // Every height a sheet can letter at, for every size RSpice supports,
    // must be a step on the ISO 3098 series, and a caption must never
    // outweigh the value it names.
    const SERIES: [u64; 8] = [1_800, 2_500, 3_500, 5_000, 7_000, 10_000, 14_000, 20_000];
    for standard in DrawingSheetStandard::ALL {
        let (width_um, height_um) = standard.portrait_dimensions_um();
        let lettering = DrawingSheetLettering::for_sheet(width_um, height_um);
        for height in [lettering.caption_um, lettering.data_um] {
            assert!(
                SERIES.contains(&height),
                "{height} um on {standard:?} is not an ISO 3098 lettering height"
            );
        }
        assert!(
            lettering.caption_um < lettering.data_um,
            "{standard:?} letters its captions at or above its values"
        );
    }

    // The recommended minimum rises with the sheet, and A4 takes the row the
    // standard gives it: 1.8 mm notes and 2.5 mm titles.
    let a4 = DrawingSheetLettering::for_sheet(210_000, 297_000);
    assert_eq!((a4.caption_um, a4.data_um), (1_800, 2_500));
    let a0 = DrawingSheetLettering::for_sheet(841_000, 1_189_000);
    assert_eq!((a0.caption_um, a0.data_um), (3_500, 5_000));
    assert_eq!(
        DrawingSheetLettering::for_sheet(297_000, 420_000),
        DrawingSheetLettering::for_sheet(594_000, 841_000),
        "A3 and A1 share the standard's middle row"
    );

    // An ISO 3098 height is the height of the capitals, so the em a renderer
    // is handed must be the larger number. Letter at the height itself and
    // the capitals come out at 0.698 of it, which on A4 puts a caption at
    // 1.26 mm — under the smallest height the standard permits.
    for height_um in [1_800, 2_500, 3_500, 5_000, 7_000] {
        let em_um = drawing_sheet_mono_em_um(height_um);
        assert!(
            em_um > height_um,
            "an em that letters capitals {height_um} um tall is taller than they are"
        );
        assert!(
            em_um * 698 / 1_000 >= height_um - 1,
            "lettering at em {em_um} um must return capitals of {height_um} um"
        );
    }

    // IBM Plex Mono advances 600 and ascends 1025 over 1000-unit em, as the
    // shipped faces report. Both derivations are exact at every height used,
    // and both are taken from the em rather than from the capital height.
    assert_eq!(drawing_sheet_mono_em_um(2_500), 3_581);
    assert_eq!(drawing_sheet_mono_em_um(1_800), 2_578);
    assert_eq!(drawing_sheet_mono_advance_um(2_500), 2_148);
    assert_eq!(drawing_sheet_mono_advance_um(1_800), 1_546);
    assert_eq!(drawing_sheet_mono_baseline_offset_um(2_500), 1_342);
    assert_eq!(drawing_sheet_mono_baseline_offset_um(1_800), 966);

    // Every caption must fit the narrowest cell the default sheet produces,
    // because a caption is fixed wording that no operator can shorten.
    let format = SchematicSheetFormat::default();
    let geometry = format.geometry().unwrap();
    let fields = DrawingSheetTitleFieldId::ALL.len();
    let caption_chars = drawing_sheet_title_cell_capacity(
        &format,
        &geometry,
        fields,
        format.authored_size.lettering().caption_um,
    )
    .unwrap();
    for field in DrawingSheetTitleFieldId::ALL {
        let caption = field.display_label();
        assert!(
            caption.chars().count() <= caption_chars,
            "caption {caption:?} needs {} of {caption_chars} characters in the default block",
            caption.chars().count()
        );
    }
}

#[test]
fn zone_references_letter_at_the_iso_5457_height_their_band_can_hold() {
    // ISO 5457 letters a grid reference 3.5 mm high on every sheet, because
    // the band carrying it is the same width on every sheet. A band too
    // narrow for that steps down the series until the capitals clear both
    // rules by a tenth of their height.
    for border in [
        DrawingSheetBorderTemplate::Standard,
        DrawingSheetBorderTemplate::Plain,
        DrawingSheetBorderTemplate::OrganizationManaged,
    ] {
        let band_um = border.band_width_um();
        let height_um = drawing_sheet_zone_lettering_um(band_um);
        assert!(ISO_3098_HEIGHTS_UM.contains(&height_um));
        assert!(height_um <= 3_500);
        assert!(
            height_um + 2 * (height_um / 10) <= band_um,
            "{border:?} letters {height_um} um references that crowd its {band_um} um band"
        );
    }
    assert_eq!(
        drawing_sheet_zone_lettering_um(DrawingSheetBorderTemplate::Standard.band_width_um()),
        3_500
    );
    assert_eq!(
        drawing_sheet_zone_lettering_um(DrawingSheetBorderTemplate::Plain.band_width_um()),
        1_800
    );
    assert_eq!(
        drawing_sheet_zone_lettering_um(
            DrawingSheetBorderTemplate::OrganizationManaged.band_width_um()
        ),
        3_500,
        "a wider band does not letter its references any larger"
    );
}

#[test]
fn title_block_lettering_steps_down_to_the_row_that_carries_it() {
    // A title block is a fixed form: the block on an A0 sheet has the same
    // rows in the same height as the block on an A4 one. The heights a sheet
    // recommends rise with the paper, so on the largest sheets a value's line
    // is taller than the space under its seat in the row — and the canvas
    // clips a cell to its rules, which makes an unclamped height a field
    // truncated outright rather than a field lettered generously.
    let mut stepped_down = 0;
    for standard in DrawingSheetStandard::ALL {
        for template in [
            DrawingSheetTitleBlockTemplate::Compact,
            DrawingSheetTitleBlockTemplate::Standard,
            DrawingSheetTitleBlockTemplate::Wide,
            DrawingSheetTitleBlockTemplate::OrganizationManaged,
        ] {
            let format = SchematicSheetFormat::default()
                .try_update(|draft| {
                    draft.authored_size = AuthoredDrawingSheetSize::Standard { standard };
                    draft.margins = standard.default_margins();
                    draft.title_block.template = template;
                })
                .expect("every standard sheet size is a valid format");
            let geometry = format.geometry().expect("a standard sheet has geometry");
            let Some(lettering) = drawing_sheet_title_block_lettering(&format, &geometry) else {
                continue;
            };
            let recommended = format.authored_size.lettering();
            assert!(
                lettering.caption_um <= recommended.caption_um
                    && lettering.data_um <= recommended.data_um,
                "{standard:?} {template:?} must not letter above what its sheet recommends"
            );
            if lettering.data_um < recommended.data_um {
                stepped_down += 1;
            }
            for height_um in [lettering.caption_um, lettering.data_um] {
                assert!(
                    ISO_3098_HEIGHTS_UM.contains(&height_um),
                    "{standard:?} {template:?} letters at {height_um} um, off the series"
                );
            }
            assert!(
                lettering.caption_um < lettering.data_um,
                "{standard:?} {template:?} letters a caption level with or above its value"
            );

            // A line seated on a fraction of the row reaches 0.587 of its em
            // below that seat — 3/8 to the baseline, then 212/1000 to the foot
            // of a `g` — and its capitals rise 0.323 of its em above it. The
            // value has to stop above the rule under it, and the caption's
            // descenders a tenth of the value's height above the value.
            let rows = format
                .title_block_rows(geometry.effective_title_block_template)
                .expect("a block has rows") as u64;
            let row_height_um = geometry.title_block.expect("a block has a rect").height_um / rows;
            let seat_um =
                |(numerator, denominator): (u64, u64)| row_height_um * numerator / denominator;
            let value_seat_um = seat_um(DRAWING_SHEET_TITLE_VALUE_POSITION);
            assert!(
                value_seat_um + drawing_sheet_mono_em_um(lettering.data_um) * 587 / 1_000
                    <= row_height_um,
                "{standard:?} {template:?} letters a value whose ink crosses the rule below it"
            );
            let caption_foot_um = seat_um(DRAWING_SHEET_TITLE_CAPTION_POSITION)
                + drawing_sheet_mono_em_um(lettering.caption_um) * 587 / 1_000;
            let value_capitals_um =
                value_seat_um - drawing_sheet_mono_em_um(lettering.data_um) * 323 / 1_000;
            assert!(
                caption_foot_um + lettering.data_um / 10 <= value_capitals_um,
                "{standard:?} {template:?} seats caption descenders on the value's capitals"
            );
        }
    }
    assert!(
        stepped_down > 0,
        "the largest sheets recommend more lettering than a title row can seat"
    );

    // An organization may seal rows too shallow for any step between the
    // two: both then stand on the lowest height, and the caption still never
    // outgrows the value.
    let shallow = DrawingSheetManagedTemplateSnapshot::try_new(
        "example.shallow-title-block",
        "1",
        180_000,
        30_000,
        5,
        DrawingSheetTitleFieldId::ALL.to_vec(),
        Vec::new(),
    )
    .unwrap();
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.authored_size = AuthoredDrawingSheetSize::Standard {
                standard: DrawingSheetStandard::IsoA0,
            };
            draft.margins = DrawingSheetStandard::IsoA0.default_margins();
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            draft.title_block.managed_template = Some(shallow);
        })
        .unwrap();
    let lettering = drawing_sheet_title_block_lettering(&format, &format.geometry().unwrap())
        .expect("a managed block has lettering");
    assert_eq!((lettering.caption_um, lettering.data_um), (1_800, 1_800));
}

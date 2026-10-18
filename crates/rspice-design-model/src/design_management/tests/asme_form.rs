//! ANSI and ARCH sheets carry ASME Y14.1's title block as a designed form:
//! Fig 5-1 on A to C sizes and Fig 5-2 on D and larger, at the figures'
//! inch dimensions, with the figures' headings and ASME Y14.2's lettering.

use super::*;

use DrawingSheetTitleFieldId as Field;

/// A length in hundredths of an inch, as the figures dimension the block.
const fn inch(hundredths: u64) -> u64 {
    hundredths * 254
}

fn sheet(standard: DrawingSheetStandard) -> SchematicSheetFormat {
    SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
}

fn layout_of(format: &SchematicSheetFormat) -> DrawingSheetTitleLayout {
    drawing_sheet_title_layout(format, &format.geometry().unwrap())
        .expect("the sheet carries a title block")
}

/// The cell whose heading is `heading`.
fn cell<'a>(layout: &'a DrawingSheetTitleLayout, heading: &str) -> &'a DrawingSheetTitleCell {
    layout
        .cells
        .iter()
        .find(|cell| cell.caption.is_some_and(|caption| caption.text == heading))
        .unwrap_or_else(|| panic!("no {heading} cell"))
}

/// `(x, y, width, height)` of a cell, in the block's frame.
fn bounds(cell: &DrawingSheetTitleCell) -> (u64, u64, u64, u64) {
    (cell.x_um, cell.y_um, cell.width_um, cell.height_um)
}

fn resolved(
    format: &SchematicSheetFormat,
    values: &[(DrawingSheetTitleFieldId, &str)],
) -> Vec<ResolvedDrawingSheetTitleField> {
    let values = values
        .iter()
        .map(|(field, value)| (*field, (*value).to_owned()))
        .collect();
    resolve_drawing_sheet_title_fields(format, &values)
}

fn lettered(
    layout: &DrawingSheetTitleLayout,
    fields: &[ResolvedDrawingSheetTitleField],
) -> Vec<String> {
    layout
        .lettering(fields)
        .into_iter()
        .map(|line| line.text)
        .collect()
}

#[test]
fn a_new_ansi_or_arch_sheet_starts_on_the_asme_form_and_an_iso_sheet_on_iso_7200() {
    for standard in DrawingSheetStandard::ALL {
        let format = sheet(standard);
        let expected = match standard.series() {
            DrawingSheetStandardSeries::Ansi | DrawingSheetStandardSeries::Architectural => {
                DrawingSheetTitleBlockTemplate::AsmeY141
            }
            DrawingSheetStandardSeries::Iso | DrawingSheetStandardSeries::Jis => {
                DrawingSheetTitleBlockTemplate::Standard
            }
        };
        assert_eq!(format.title_block.template, expected, "{standard:?}");
        if expected == DrawingSheetTitleBlockTemplate::AsmeY141 {
            assert!(!format.title_block_substituted(), "{standard:?}");
        }
    }
}

#[test]
fn the_block_takes_fig_5_1_on_a_to_c_and_fig_5_2_on_d_and_larger() {
    // Fig 5-1: 6.25 x 2.00 in. Fig 5-2: 7.62 x 2.50 in.
    for (standard, expected) in [
        (DrawingSheetStandard::AnsiA, (inch(625), inch(200))),
        (DrawingSheetStandard::AnsiB, (inch(625), inch(200))),
        (DrawingSheetStandard::AnsiC, (inch(625), inch(200))),
        (DrawingSheetStandard::AnsiD, (inch(762), inch(250))),
        (DrawingSheetStandard::AnsiE, (inch(762), inch(250))),
        // ARCH: widened at the left by what `ARCH C` (3 mm) and `ARCH E1`
        // (6 mm) need in SIZE.
        (DrawingSheetStandard::ArchA, (inch(657), inch(200))),
        (DrawingSheetStandard::ArchC, (inch(657), inch(200))),
        (DrawingSheetStandard::ArchD, (inch(885), inch(250))),
        (DrawingSheetStandard::ArchE1, (inch(885), inch(250))),
    ] {
        for orientation in [
            SchematicPageOrientation::Landscape,
            SchematicPageOrientation::Portrait,
        ] {
            let format = SchematicSheetFormat::from_standard(standard, orientation);
            let geometry = format.geometry().unwrap();
            let block = geometry.title_block.expect("the sheet carries a block");
            assert_eq!(
                (block.width_um, block.height_um),
                expected,
                "{standard:?} {orientation:?}"
            );
            // ASME Y14.1 6.1: the lower right corner of the format, upright.
            let area = geometry.drawing_area;
            assert_eq!(
                block.x_um + block.width_um as i64,
                area.x_um + area.width_um as i64
            );
            assert_eq!(
                block.y_um + block.height_um as i64,
                area.y_um + area.height_um as i64
            );
            assert_eq!(
                format.title_block.rotation,
                DrawingSheetTitleBlockRotation::Upright
            );
        }
    }
}

#[test]
fn the_template_is_named_with_the_size_of_the_block_the_sheet_carries() {
    let label = |standard| {
        let format = sheet(standard);
        format.title_block_template_label(format.title_block.template)
    };
    assert_eq!(
        label(DrawingSheetStandard::AnsiB),
        "ASME Y14.1 · 6.25 × 2.00 in"
    );
    assert_eq!(
        label(DrawingSheetStandard::AnsiD),
        "ASME Y14.1 · 7.62 × 2.50 in"
    );
    // ASME Y14.1 does not define the ARCH series, so its block is named for
    // the format rather than claimed as the standard's.
    assert_eq!(
        label(DrawingSheetStandard::ArchB),
        "ASME Y14.1 format · 6.57 × 2.00 in"
    );
    assert_eq!(
        label(DrawingSheetStandard::ArchD),
        "ASME Y14.1 format · 8.85 × 2.50 in"
    );
}

#[test]
fn fig_5_1_cells_stand_at_the_figures_dimensions() {
    let layout = layout_of(&sheet(DrawingSheetStandard::AnsiB));
    assert_eq!((layout.width_um, layout.height_um), (inch(625), inch(200)));
    // Block E 2.00 in; A 0.62, B (TITLE) 0.76, the number row 0.37 and the
    // bottom row 0.25 in high, every division measured from the right edge.
    let x0 = inch(200);
    assert_eq!(
        bounds(cell(&layout, "TITLE")),
        (x0, inch(62), inch(425), inch(76))
    );
    let row3 = inch(62 + 76);
    assert_eq!(
        bounds(cell(&layout, "SIZE")),
        (x0, row3, inch(38), inch(37))
    );
    assert_eq!(
        bounds(cell(&layout, "DAI")),
        (x0 + inch(38), row3, inch(112), inch(37))
    );
    assert_eq!(
        bounds(cell(&layout, "DWG NO")),
        (x0 + inch(150), row3, inch(237), inch(37))
    );
    assert_eq!(
        bounds(cell(&layout, "REV")),
        (x0 + inch(387), row3, inch(38), inch(37))
    );
    let row4 = row3 + inch(37);
    assert_eq!(
        bounds(cell(&layout, "SCALE")),
        (x0, row4, inch(100), inch(25))
    );
    assert_eq!(
        bounds(cell(&layout, "WEIGHT")),
        (x0 + inch(100), row4, inch(150), inch(25))
    );
    assert_eq!(
        bounds(cell(&layout, "SHEET")),
        (x0 + inch(250), row4, inch(175), inch(25))
    );
    // A spans the right part above TITLE and carries no heading.
    let organization = layout.cell(Field::Organization).unwrap();
    assert_eq!(bounds(organization), (x0, 0, inch(425), inch(62)));
    assert!(organization.caption.is_none());
}

#[test]
fn fig_5_2_cells_stand_at_the_figures_dimensions() {
    let layout = layout_of(&sheet(DrawingSheetStandard::AnsiD));
    assert_eq!((layout.width_um, layout.height_um), (inch(762), inch(250)));
    let x0 = inch(250);
    assert_eq!(
        bounds(cell(&layout, "TITLE")),
        (x0, inch(62), inch(512), inch(113))
    );
    let row3 = inch(62 + 113);
    for (heading, x, width) in [
        ("SIZE", 0, 37),
        ("DAI", 37, 125),
        ("DWG NO", 162, 300),
        ("REV", 462, 50),
    ] {
        assert_eq!(
            bounds(cell(&layout, heading)),
            (x0 + inch(x), row3, inch(width), inch(50)),
            "{heading}"
        );
    }
    let row4 = row3 + inch(50);
    for (heading, x, width) in [("SCALE", 0, 137), ("WEIGHT", 137, 200), ("SHEET", 337, 175)] {
        assert_eq!(
            bounds(cell(&layout, heading)),
            (x0 + inch(x), row4, inch(width), inch(25)),
            "{heading}"
        );
    }
}

#[test]
fn an_arch_block_widens_size_and_scale_and_keeps_every_rule_measured_from_the_right() {
    for (arch, ansi) in [
        (DrawingSheetStandard::ArchB, DrawingSheetStandard::AnsiB),
        (DrawingSheetStandard::ArchD, DrawingSheetStandard::AnsiD),
    ] {
        let wide = layout_of(&sheet(arch));
        let figure = layout_of(&sheet(ansi));
        let from_right = |layout: &DrawingSheetTitleLayout, heading: &str| {
            let cell = cell(layout, heading);
            layout.width_um - cell.x_um - cell.width_um
        };
        for heading in ["DAI", "DWG NO", "REV", "WEIGHT", "SHEET"] {
            assert_eq!(
                from_right(&wide, heading),
                from_right(&figure, heading),
                "{arch:?} {heading}"
            );
            assert_eq!(
                cell(&wide, heading).width_um,
                cell(&figure, heading).width_um,
                "{arch:?} {heading}"
            );
        }
        let extra = wide.width_um - figure.width_um;
        for heading in ["SIZE", "SCALE", "TITLE"] {
            assert_eq!(
                cell(&wide, heading).width_um,
                cell(&figure, heading).width_um + extra,
                "{arch:?} {heading}"
            );
        }
        // Block E keeps the figure's width.
        assert_eq!(cell(&wide, "TITLE").x_um, cell(&figure, "TITLE").x_um);
    }
}

#[test]
fn headings_are_the_figures_capitals_and_the_bottom_row_letters_them_inline() {
    let layout = layout_of(&sheet(DrawingSheetStandard::AnsiB));
    for heading in ["TITLE", "SIZE", "DAI", "DWG NO", "REV"] {
        let caption = cell(&layout, heading).caption.unwrap();
        assert_eq!(caption.placement, DrawingSheetTitleCaptionPlacement::Over);
        assert_eq!(caption.lettering_um, 2_500, "{heading}");
    }
    for heading in ["SCALE", "WEIGHT", "SHEET"] {
        let caption = cell(&layout, heading).caption.unwrap();
        assert_eq!(caption.placement, DrawingSheetTitleCaptionPlacement::Inline);
        assert_eq!(caption.lettering_um, 2_500, "{heading}");
    }
    // An inline heading and its value share a baseline, the value after the
    // heading on the same line.
    let sheet_cell = cell(&layout, "SHEET");
    let heading = sheet_cell.caption.unwrap();
    let value = sheet_cell.value(Field::Page).unwrap();
    assert_eq!(
        heading.seat_um + drawing_sheet_mono_baseline_offset_um(heading.lettering_um),
        value.seat_um + drawing_sheet_mono_baseline_offset_um(value.lettering_um)
    );
    assert!(value.x_um >= heading.x_um + heading.width_um);
}

#[test]
fn key_entries_are_lettered_3_mm_on_a_to_c_and_6_mm_on_d_and_larger() {
    for (standard, key_um) in [
        (DrawingSheetStandard::AnsiB, 3_000),
        (DrawingSheetStandard::ArchC, 3_000),
        (DrawingSheetStandard::AnsiD, 6_000),
        (DrawingSheetStandard::ArchE1, 6_000),
    ] {
        let layout = layout_of(&sheet(standard));
        for field in [
            Field::SheetTitle,
            Field::CageCode,
            Field::DocumentId,
            Field::Revision,
        ] {
            assert_eq!(
                layout.value(field).unwrap().lettering_um,
                key_um,
                "{standard:?} {field:?}"
            );
        }
        let size = cell(&layout, "SIZE").values[0];
        assert_eq!(size.lettering_um, key_um, "{standard:?} SIZE");
        for field in [
            Field::Organization,
            Field::Page,
            Field::DrawnBy,
            Field::ApprovedBy,
        ] {
            assert_eq!(
                layout.value(field).unwrap().lettering_um,
                3_000,
                "{field:?}"
            );
        }
    }
}

#[test]
fn size_letters_the_designation_scale_none_and_weight_stays_blank() {
    for (standard, size) in [
        (DrawingSheetStandard::AnsiB, "B"),
        (DrawingSheetStandard::AnsiE, "E"),
        (DrawingSheetStandard::ArchC, "ARCH C"),
        (DrawingSheetStandard::ArchD, "ARCH D"),
        (DrawingSheetStandard::ArchE1, "ARCH E1"),
    ] {
        let format = sheet(standard);
        let layout = layout_of(&format);
        let fields = resolved(&format, &[(Field::Page, "1 of 3")]);
        let size_cell = cell(&layout, "SIZE");
        assert_eq!(
            size_cell.values[0].letter(size),
            [size],
            "{standard:?}: SIZE holds it whole"
        );
        assert_eq!(size_cell.letter_caption(), "SIZE", "{standard:?}");
        let text = lettered(&layout, &fields);
        assert!(
            text.iter().any(|line| line == size),
            "{standard:?}: {text:?}"
        );
        assert!(text.iter().any(|line| line == "NONE"), "{text:?}");
        assert!(text.iter().any(|line| line == "1 OF 3"), "{text:?}");
        assert!(cell(&layout, "WEIGHT").values.is_empty());
    }
}

#[test]
fn dai_letters_the_projects_cage_code_and_is_blank_without_one() {
    let format = sheet(DrawingSheetStandard::AnsiD);
    let layout = layout_of(&format);
    let dai = cell(&layout, "DAI");
    assert_eq!(dai.field(), Some(Field::CageCode));
    // A CAGE code's five characters at 6 mm.
    assert_eq!(dai.values[0].capacity(), 5);
    let blank = lettered(&layout, &resolved(&format, &[]));
    let signed = lettered(&layout, &resolved(&format, &[(Field::CageCode, "1ABC2")]));
    assert!(signed.iter().any(|line| line == "1ABC2"));
    assert_eq!(signed.len(), blank.len() + 1, "{blank:?} {signed:?}");
}

#[test]
fn the_fields_map_onto_the_blocks_the_audit_assigns() {
    let layout = layout_of(&sheet(DrawingSheetStandard::AnsiB));
    assert_eq!(cell(&layout, "TITLE").field(), Some(Field::SheetTitle));
    assert_eq!(cell(&layout, "DWG NO").field(), Some(Field::DocumentId));
    assert_eq!(cell(&layout, "REV").field(), Some(Field::Revision));
    assert_eq!(cell(&layout, "SHEET").field(), Some(Field::Page));
    assert_eq!(cell(&layout, "DRAWN").field(), Some(Field::DrawnBy));
    assert_eq!(cell(&layout, "CHECKED").field(), Some(Field::CheckedBy));
    assert_eq!(cell(&layout, "APPROVED").field(), Some(Field::ApprovedBy));
    // The drawing number is set in the heavier weight.
    assert!(layout.value(Field::DocumentId).unwrap().emphasised);
    // Every field of the form is shown, whatever a sheet's visibility says.
    let hidden = sheet(DrawingSheetStandard::AnsiB)
        .try_update(|draft| {
            for state in draft.title_block.fields.values_mut() {
                state.visible = false;
            }
            draft
                .title_block
                .fields
                .get_mut(&Field::Page)
                .unwrap()
                .visible = true;
            draft
                .title_block
                .fields
                .get_mut(&Field::SheetTitle)
                .unwrap()
                .visible = true;
        })
        .unwrap();
    for field in ASME_Y14_1_FORM_FIELDS {
        assert!(hidden.title_field_shown(field), "{field:?}");
    }
    assert!(!hidden.title_field_shown(Field::DocumentType));
    assert_eq!(layout_of(&hidden), layout);
}

#[test]
fn the_outline_and_main_divisions_are_thick_and_the_subdivisions_thin() {
    let layout = layout_of(&sheet(DrawingSheetStandard::AnsiB));
    assert_eq!(layout.outline_um, 600);
    let x0 = inch(200);
    let thick = layout
        .rules
        .iter()
        .filter(|rule| rule.width_um == 600)
        .map(|rule| (rule.from_x_um, rule.from_y_um, rule.to_x_um, rule.to_y_um))
        .collect::<Vec<_>>();
    let right = inch(625);
    assert_eq!(
        thick,
        [
            (x0, inch(62), right, inch(62)),
            (x0, inch(138), right, inch(138)),
            (x0, inch(175), right, inch(175)),
            (x0, 0, x0, inch(200)),
        ]
    );
    assert!(
        layout
            .rules
            .iter()
            .all(|rule| rule.width_um == 600 || rule.width_um == 300)
    );
}

#[test]
fn every_line_of_the_form_stays_inside_its_cell() {
    for standard in [
        DrawingSheetStandard::AnsiA,
        DrawingSheetStandard::AnsiD,
        DrawingSheetStandard::ArchB,
        DrawingSheetStandard::ArchE1,
    ] {
        let format = sheet(standard);
        let layout = layout_of(&format);
        let long = "WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE";
        let values = ASME_Y14_1_FORM_FIELDS.map(|field| (field, long));
        let fields = resolved(&format, &values);
        for line in layout.lettering(&fields) {
            let cell = &layout.cells[line.cell];
            let em = drawing_sheet_mono_em_um(line.lettering_um);
            let width =
                drawing_sheet_mono_advance_um(line.lettering_um) * line.text.chars().count() as u64;
            assert!(line.x_um >= cell.x_um, "{standard:?} {line:?}");
            assert!(
                line.x_um + width <= cell.x_um + cell.width_um,
                "{standard:?} {line:?}"
            );
            // Capitals clear the rule above and descenders the rule below.
            assert!(
                line.seat_um >= cell.y_um + em * 323 / 1_000,
                "{standard:?} {line:?}"
            );
            assert!(
                line.seat_um + em * 587 / 1_000 <= cell.y_um + cell.height_um,
                "{standard:?} {line:?}"
            );
        }
    }
}

#[test]
fn the_form_is_never_turned_and_steps_down_to_fig_5_1_where_fig_5_2_does_not_fit() {
    // A custom size started from ANSI, long enough to take Fig 5-2's block
    // but too narrow for its 7.62 in: 200 mm less two 0.5 in margins leaves
    // 174.6 mm across, under Fig 5-2's 193.55 mm, over Fig 5-1's 158.75 mm.
    let narrow = |width_um| {
        crate::sheet_authoring::custom_format(
            "Narrow",
            width_um,
            900_000,
            crate::sheet_authoring::StartingFrame::AnsiA,
        )
        .unwrap()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::AsmeY141;
            draft.title_block.rotation = DrawingSheetTitleBlockRotation::Clockwise90;
            draft.title_block.anchor = DrawingSheetTitleBlockAnchor::TopRight;
        })
        .unwrap()
    };
    let narrowed = narrow(200_000);
    assert_eq!(
        narrowed.title_block.rotation,
        DrawingSheetTitleBlockRotation::Upright
    );
    assert_eq!(
        narrowed.title_block.anchor,
        DrawingSheetTitleBlockAnchor::BottomRight
    );
    let form = narrowed.asme_title_block_form().unwrap();
    assert_eq!(form.size, AsmeY141TitleBlockSize::Small);
    assert!(!narrowed.title_block_substituted());
    // Narrower still, 154.6 mm across, neither fits, and the sheet carries
    // no block.
    let too_narrow = narrow(180_000);
    assert_eq!(too_narrow.asme_title_block_form(), None);
    assert_eq!(
        too_narrow.effective_title_block_template(),
        DrawingSheetTitleBlockTemplate::None
    );
}

#[test]
fn a_saved_ansi_sheet_keeps_the_block_it_was_saved_with() {
    for template in [
        DrawingSheetTitleBlockTemplate::Standard,
        DrawingSheetTitleBlockTemplate::Compact,
    ] {
        let saved = sheet(DrawingSheetStandard::AnsiB)
            .try_update(|draft| draft.title_block.template = template)
            .unwrap();
        let text = serde_json::to_string(&saved).unwrap();
        let loaded: SchematicSheetFormat = serde_json::from_str(&text).unwrap();
        assert_eq!(loaded.title_block.template, template);
        assert_eq!(loaded, saved);
    }
    // The ASME form round-trips under its own name.
    let asme = sheet(DrawingSheetStandard::AnsiB);
    let text = serde_json::to_string(&asme).unwrap();
    assert!(text.contains(r#""template":"asme-y14-1""#), "{text}");
    assert_eq!(
        serde_json::from_str::<SchematicSheetFormat>(&text).unwrap(),
        asme
    );
}

#[test]
fn block_e_is_a_name_and_date_grid_under_the_contract_number() {
    for (standard, name_chars) in [
        (DrawingSheetStandard::AnsiB, 8),
        (DrawingSheetStandard::AnsiD, 13),
    ] {
        let layout = layout_of(&sheet(standard));
        let block_e = layout
            .rules
            .iter()
            .find(|rule| rule.width_um == 600 && rule.from_x_um == rule.to_x_um)
            .unwrap()
            .from_x_um;
        // The contract number and the column headings stand as tall as
        // block A beside them.
        let contract = cell(&layout, "CONTRACT NO");
        assert_eq!(bounds(contract), (0, 0, block_e, inch(37)));
        assert_eq!(contract.field(), Some(Field::ContractNumber));
        let (name, date) = (cell(&layout, "NAME"), cell(&layout, "DATE"));
        assert_eq!(name.y_um + name.height_um, inch(62), "{standard:?}");
        assert_eq!(date.x_um + date.width_um, block_e);
        assert_eq!(date.width_um, inch(110));
        let mut bottom = inch(62);
        for (heading, person, signed) in [
            ("DRAWN", Field::DrawnBy, Field::DrawnDate),
            ("CHECKED", Field::CheckedBy, Field::CheckedDate),
            ("ENGR", Field::Engineer, Field::EngineerDate),
            ("APPROVED", Field::ApprovedBy, Field::ApprovedDate),
        ] {
            let row = cell(&layout, heading);
            assert_eq!(row.y_um, bottom, "{standard:?} {heading}");
            bottom = row.y_um + row.height_um;
            assert_eq!(row.field(), Some(person));
            let name_line = row.values[0];
            assert_eq!(name_line.capacity(), name_chars, "{standard:?} {heading}");
            let date_cell = layout.cell(signed).unwrap();
            assert_eq!(date_cell.y_um, row.y_um);
            assert_eq!(date_cell.x_um, name.x_um + name.width_um);
            let date_line = date_cell.value(signed).unwrap();
            // The date stands on the name's line and holds YYYY-MM-DD.
            assert_eq!(date_line.seat_um, name_line.seat_um);
            assert!(!date_line.elides("2026-10-18"));
        }
        assert_eq!(bottom, layout.height_um);
    }
}

#[test]
fn a_signature_date_is_a_calendar_date_like_the_document_date() {
    let format = sheet(DrawingSheetStandard::AnsiB);
    let dated = |field: DrawingSheetTitleFieldId, value: &str| {
        format.try_update(|draft| {
            draft.title_block.fields.get_mut(&field).unwrap().value = value.to_owned();
        })
    };
    for field in [
        Field::DrawnDate,
        Field::CheckedDate,
        Field::EngineerDate,
        Field::ApprovedDate,
    ] {
        assert!(field.is_signature_date());
        assert_eq!(field.entry_hint(), "YYYY-MM-DD");
        assert!(dated(field, "2026-10-18").is_ok(), "{field:?}");
        assert!(dated(field, "").is_ok(), "{field:?}");
        for refused in ["18/10/2026", "2026-02-30", "2026-10-18T00:00"] {
            assert!(
                matches!(
                    dated(field, refused),
                    Err(DesignManagementError::DocumentControl(_))
                ),
                "{field:?} {refused}"
            );
        }
    }
    // The engineer's name is free text, as the other names are.
    assert!(dated(Field::Engineer, "J. Whitfield").is_ok());
}

#[test]
fn the_approval_date_reads_the_date_of_issue_until_the_sheet_gives_one() {
    let format = sheet(DrawingSheetStandard::AnsiB);
    let layout = layout_of(&format);
    let issued = [(Field::ApprovedDate, "2026-10-01")];
    let text = lettered(&layout, &resolved(&format, &issued));
    assert!(text.iter().any(|line| line == "2026-10-01"), "{text:?}");
    let signed = format
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&Field::ApprovedDate)
                .unwrap()
                .value = "2026-10-17".to_owned();
        })
        .unwrap();
    let text = lettered(&layout, &resolved(&signed, &issued));
    assert!(text.iter().any(|line| line == "2026-10-17"), "{text:?}");
    assert!(!text.iter().any(|line| line == "2026-10-01"), "{text:?}");
    // The other dates are the sheet's alone: an empty one is a box to fill.
    let blank = lettered(&layout, &resolved(&format, &[]));
    assert!(
        !blank
            .iter()
            .any(|line| line.contains('-') && line.len() == 10)
    );
}

#[test]
fn a_project_saved_before_the_cage_code_and_contract_number_loads_without_them() {
    let mut settings = serde_json::to_value(DrawingSheetProjectSettings::default()).unwrap();
    let values = settings["title_block_field_values"]
        .as_object_mut()
        .unwrap();
    values.remove("cage-code");
    values.remove("contract-number");
    let restored: DrawingSheetProjectSettings = serde_json::from_value(settings).unwrap();
    for field in [Field::CageCode, Field::ContractNumber] {
        assert!(field.is_project_owned());
        assert_eq!(restored.title_block_field_values[&field], "");
    }
    // A sheet saved before them loads with them, unseen on every block but
    // the ASME form.
    let mut saved = serde_json::to_value(SchematicSheetFormat::default()).unwrap();
    let fields = saved["title_block"]["fields"].as_object_mut().unwrap();
    for name in [
        "cage-code",
        "engineer",
        "drawn-date",
        "checked-date",
        "engineer-date",
        "approved-date",
        "contract-number",
    ] {
        assert!(fields.remove(name).is_some(), "{name}");
    }
    let loaded: SchematicSheetFormat = serde_json::from_value(saved).unwrap();
    assert_eq!(loaded, SchematicSheetFormat::default());
}

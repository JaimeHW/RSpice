//! ANSI and ARCH sheets carry ASME Y14.1's title block as a designed form:
//! Fig 5-1 on A to C sizes and Fig 5-2 on D and larger, at the figures'
//! inch dimensions, with the figures' headings and ASME Y14.2's lettering.

use super::*;

use DrawingSheetTitleFieldId as Field;

/// A length in hundredths of an inch, as the figures dimension the block.
const fn inch(hundredths: u64) -> u64 {
    hundredths * 254
}

/// A new landscape sheet showing none of the fields the supplementary strip
/// carries, so its block is the figure's alone.
fn sheet(standard: DrawingSheetStandard) -> SchematicSheetFormat {
    showing_supplementary(standard, &[])
}

/// A new landscape sheet showing exactly `fields` of the supplementary
/// strip's.
fn showing_supplementary(
    standard: DrawingSheetStandard,
    fields: &[DrawingSheetTitleFieldId],
) -> SchematicSheetFormat {
    SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
        .try_update(|draft| {
            for field in ASME_Y14_1_SUPPLEMENTARY_FIELDS {
                draft.title_block.fields.get_mut(&field).unwrap().visible = fields.contains(&field);
            }
        })
        .unwrap()
}

/// A new sheet held upright, with a binding margin when `bound`, showing
/// every field of the supplementary strip.
fn upright(standard: DrawingSheetStandard, bound: bool) -> SchematicSheetFormat {
    SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Portrait)
        .try_update(|draft| {
            draft.binding_margin = bound;
            for field in ASME_Y14_1_SUPPLEMENTARY_FIELDS {
                draft.title_block.fields.get_mut(&field).unwrap().visible = true;
            }
        })
        .unwrap()
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
    // Fig 5-1, 6.25 x 2.00 in, widened to 7.50 in for its values; Fig 5-2,
    // 7.62 x 2.50 in, widened to 8.87 in. Held upright, an ANSI A sheet's
    // drawing area is 7.50 in across and holds Fig 5-1 exactly.
    for (standard, expected) in [
        (DrawingSheetStandard::AnsiA, (inch(750), inch(200))),
        (DrawingSheetStandard::AnsiB, (inch(750), inch(200))),
        (DrawingSheetStandard::AnsiC, (inch(750), inch(200))),
        (DrawingSheetStandard::AnsiD, (inch(887), inch(250))),
        (DrawingSheetStandard::AnsiE, (inch(887), inch(250))),
        // ARCH: widened at the left by what `ARCH C` (3 mm) and `ARCH E1`
        // (6 mm) need in SIZE.
        (DrawingSheetStandard::ArchA, (inch(782), inch(200))),
        (DrawingSheetStandard::ArchC, (inch(782), inch(200))),
        (DrawingSheetStandard::ArchD, (inch(1010), inch(250))),
        (DrawingSheetStandard::ArchE1, (inch(1010), inch(250))),
    ] {
        for orientation in [
            SchematicPageOrientation::Landscape,
            SchematicPageOrientation::Portrait,
        ] {
            // A new sheet shows Cell / view and Document status, which the
            // strip above the block carries.
            let format = SchematicSheetFormat::from_standard(standard, orientation);
            assert!(
                !format.asme_title_block_form().unwrap().figure_minimum,
                "{standard:?} {orientation:?}"
            );
            let geometry = format.geometry().unwrap();
            let block = geometry.title_block.expect("the sheet carries a block");
            assert_eq!(
                (block.width_um, block.height_um),
                (expected.0, expected.1 + inch(37)),
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
        "ASME Y14.1 · 7.50 × 2.00 in"
    );
    assert_eq!(
        label(DrawingSheetStandard::AnsiD),
        "ASME Y14.1 · 8.87 × 2.50 in"
    );
    // ASME Y14.1 does not define the ARCH series, so its block is named for
    // the format rather than claimed as the standard's.
    assert_eq!(
        label(DrawingSheetStandard::ArchB),
        "ASME Y14.1 format · 7.82 × 2.00 in"
    );
    assert_eq!(
        label(DrawingSheetStandard::ArchD),
        "ASME Y14.1 format · 10.10 × 2.50 in"
    );
    // Every label names the block it reports.
    for size in [AsmeY141TitleBlockSize::Small, AsmeY141TitleBlockSize::Large] {
        for architectural in [false, true] {
            for figure_minimum in [false, true] {
                let form = AsmeY141TitleBlockForm {
                    size,
                    architectural,
                    supplementary: false,
                    figure_minimum,
                };
                let (width, height) = form.dimensions_um();
                let named = format!(
                    "{:.2} × {:.2} in",
                    width as f64 / 25_400.0,
                    height as f64 / 25_400.0
                );
                assert!(form.label().ends_with(&named), "{form:?} {named}");
            }
        }
    }
}

#[test]
fn fig_5_1_cells_right_of_e_stand_at_the_figures_dimensions() {
    let layout = layout_of(&sheet(DrawingSheetStandard::AnsiB));
    assert_eq!((layout.width_um, layout.height_um), (inch(750), inch(200)));
    // Block E 3.25 in, widened from the figure's 2.00 for its names; A 0.62,
    // B (TITLE) 0.76, the number row 0.37 and the bottom row 0.25 in high,
    // every division measured from the right edge.
    let x0 = inch(325);
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
fn fig_5_2_widens_dwg_no_leftward_and_keeps_rev_and_the_bottom_row_in_place() {
    let layout = layout_of(&sheet(DrawingSheetStandard::AnsiD));
    assert_eq!((layout.width_um, layout.height_um), (inch(887), inch(250)));
    // Block E 3.25 in and the part right of it 5.62, the figure's 5.12 and
    // what DWG NO gains.
    let x0 = inch(325);
    assert_eq!(
        bounds(cell(&layout, "TITLE")),
        (x0, inch(62), inch(562), inch(113))
    );
    let row3 = inch(62 + 113);
    for (heading, x, width) in [
        ("SIZE", 0, 37),
        ("DAI", 37, 125),
        ("DWG NO", 162, 350),
        ("REV", 512, 50),
    ] {
        assert_eq!(
            bounds(cell(&layout, heading)),
            (x0 + inch(x), row3, inch(width), inch(50)),
            "{heading}"
        );
    }
    let row4 = row3 + inch(50);
    for (heading, x, width) in [("SCALE", 0, 187), ("WEIGHT", 187, 200), ("SHEET", 387, 175)] {
        assert_eq!(
            bounds(cell(&layout, heading)),
            (x0 + inch(x), row4, inch(width), inch(25)),
            "{heading}"
        );
    }
    // Each rule's distance from the right edge, against the figure's: REV
    // and the bottom row's rules stand where the figure puts them, and the
    // rules left of DWG NO move left by what it gains.
    let from_right = |heading: &str| layout.width_um - cell(&layout, heading).x_um;
    for (heading, rule, figure) in [
        ("REV", 50, 50),
        ("DWG NO", 400, 350),
        ("DAI", 525, 475),
        ("SIZE", 562, 512),
        ("SHEET", 175, 175),
        ("WEIGHT", 375, 375),
        ("SCALE", 562, 512),
    ] {
        assert_eq!(
            from_right(heading),
            inch(rule),
            "{heading}, figure {figure}"
        );
    }
    // DWG NO's rule stands a quarter inch clear of the WEIGHT | SCALE rule
    // below it, as the figure's stands.
    assert_eq!(from_right("DWG NO") - from_right("WEIGHT"), inch(25));
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
    let x0 = inch(325);
    let thick = layout
        .rules
        .iter()
        .filter(|rule| rule.width_um == 600)
        .map(|rule| (rule.from_x_um, rule.from_y_um, rule.to_x_um, rule.to_y_um))
        .collect::<Vec<_>>();
    let right = inch(750);
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
    for (standard, format) in [
        DrawingSheetStandard::AnsiA,
        DrawingSheetStandard::AnsiD,
        DrawingSheetStandard::ArchB,
        DrawingSheetStandard::ArchE1,
    ]
    .map(|standard| {
        (
            standard,
            showing_supplementary(standard, &ASME_Y14_1_SUPPLEMENTARY_FIELDS),
        )
    })
    .into_iter()
    .chain(
        // The figure's own block, on a bound sheet held upright.
        [DrawingSheetStandard::AnsiA, DrawingSheetStandard::ArchA]
            .map(|standard| (standard, upright(standard, true))),
    ) {
        let layout = layout_of(&format);
        let long = "WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE WIDE";
        let values = ASME_Y14_1_FORM_FIELDS
            .into_iter()
            .chain(ASME_Y14_1_SUPPLEMENTARY_FIELDS)
            .map(|field| (field, long))
            .collect::<Vec<_>>();
        let fields = resolved(&format, &values);
        assert_eq!(fields.len(), values.len(), "{standard:?}");
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
fn the_form_steps_down_to_fig_5_1_where_fig_5_2_does_not_fit() {
    // A custom size started from ANSI, long enough to take Fig 5-2's block,
    // narrowed in steps. Less its two 0.5 in margins, each width leaves a
    // drawing area across that holds the next block down and no wider one:
    // Fig 5-2 widened, 225.30 mm; Fig 5-2 at the figure's own dimensions,
    // 193.55 mm; Fig 5-1 widened, 190.50 mm; Fig 5-1 at the figure's own,
    // 158.75 mm.
    use AsmeY141TitleBlockSize::{Large, Small};
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
        })
        .unwrap()
    };
    for (width_um, size, figure_minimum) in [
        (260_000, Large, false),
        (230_000, Large, true),
        (217_000, Small, false),
        (200_000, Small, true),
    ] {
        let narrowed = narrow(width_um);
        let form = narrowed.asme_title_block_form().unwrap();
        assert_eq!(
            (form.size, form.figure_minimum),
            (size, figure_minimum),
            "{width_um}"
        );
        assert!(!narrowed.title_block_substituted());
    }
    // Narrower still, 154.6 mm across, none fits, and the sheet carries no
    // block.
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
        DrawingSheetTitleBlockTemplate::None,
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
    // Twenty characters of a name at 3 mm, the length ISO 7200 recommends.
    for (standard, name_chars) in [
        (DrawingSheetStandard::AnsiB, 20),
        (DrawingSheetStandard::AnsiD, 20),
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

/// A sheet's title values as an ordinary drawing fills them in.
const FILLED: [(DrawingSheetTitleFieldId, &str); 21] = [
    (Field::Project, "MyCircuit"),
    (Field::CellView, "mycircuit/top/schematic"),
    (Field::SheetTitle, "Bandgap reference core"),
    (Field::Page, "1 of 3"),
    (Field::Revision, "B"),
    (Field::DrawnBy, "J. Whitfield"),
    (Field::CheckedBy, "R. Okafor"),
    (Field::ApprovedBy, "A. Moreau"),
    (Field::Engineer, "M. Lindqvist"),
    (Field::DocumentStatus, "Under approval"),
    (Field::Organization, "Whitfield Analog Devices"),
    (Field::DocumentId, "RS-0001-SCH-0042"),
    (Field::Classification, "Company confidential"),
    (Field::Language, "en"),
    (Field::CageCode, "1ABC2"),
    (Field::DrawnDate, "2026-10-12"),
    (Field::CheckedDate, "2026-10-14"),
    (Field::EngineerDate, "2026-10-15"),
    (Field::ApprovedDate, "2026-10-18"),
    (Field::ContractNumber, "N00024-26-C-0001"),
    (Field::Date, "2026-10-18"),
];

#[test]
fn the_form_holds_ordinary_values_whole_at_the_y14_2_heights() {
    // Characters each cell holds at its height: names twenty and the
    // drawing number sixteen, as ISO 7200 recommends, a classification
    // twenty, a CAGE code five, and a cell / view path 24.
    use DrawingSheetStandard::{AnsiB, AnsiD};
    for (standard, held) in [
        (
            AnsiB,
            [
                ("DRAWN", 20),
                ("CHECKED", 20),
                ("ENGR", 20),
                ("APPROVED", 20),
                ("DWG NO", 22),
                ("DAI", 10),
                ("PROJECT", 9),
                ("CELL / VIEW", 24),
                ("STATUS", 14),
                ("CLASSIFICATION", 20),
                ("LANG", 2),
            ],
        ),
        (
            AnsiD,
            [
                ("DRAWN", 20),
                ("CHECKED", 20),
                ("ENGR", 20),
                ("APPROVED", 20),
                ("DWG NO", 16),
                ("DAI", 5),
                ("PROJECT", 11),
                ("CELL / VIEW", 33),
                ("STATUS", 14),
                ("CLASSIFICATION", 20),
                ("LANG", 4),
            ],
        ),
    ] {
        let format = showing_supplementary(standard, &ASME_Y14_1_SUPPLEMENTARY_FIELDS);
        let layout = layout_of(&format);
        for (heading, chars) in held {
            let cell = cell(&layout, heading);
            assert_eq!(cell.values[0].capacity(), chars, "{standard:?} {heading}");
            assert_eq!(cell.letter_caption(), heading, "{standard:?}");
        }
        // Every value of an ordinary drawing is lettered whole: the date of
        // issue stands in no cell of its own on the form.
        let fields = resolved(&format, &FILLED);
        assert!(layout.cut_fields(&fields).is_empty(), "{standard:?}");
        let text = lettered(&layout, &fields);
        for (field, value) in FILLED
            .into_iter()
            .filter(|(field, _)| *field != Field::Date)
        {
            assert!(
                text.iter().any(|line| line.eq_ignore_ascii_case(value)),
                "{standard:?} {field:?}: {text:?}"
            );
        }
    }
}

#[test]
fn fig_5_1_fills_an_upright_ansi_a_drawing_area_and_a_bound_one_keeps_the_figures_block() {
    // Held upright, ANSI A's drawing area is 8.5 in less two 0.5 in margins,
    // 7.50 in across, and holds the widened Fig 5-1 with nothing to spare;
    // ARCH A's 8.00 in holds its 7.82 in block.
    for (standard, area, block) in [
        (DrawingSheetStandard::AnsiA, inch(750), inch(750)),
        (DrawingSheetStandard::ArchA, inch(800), inch(782)),
    ] {
        let format = upright(standard, false);
        let geometry = format.geometry().unwrap();
        assert_eq!(geometry.drawing_area.width_um, area, "{standard:?}");
        assert_eq!(
            geometry.title_block.unwrap().width_um,
            block,
            "{standard:?}"
        );
        assert!(!format.asme_title_block_form().unwrap().figure_minimum);
    }
    // A binding margin leaves 7.25 and 7.75 in, which hold only the
    // figure's own block: still the standard's, as its sizes are minimums,
    // with the figure's eight-character names.
    for (standard, label, block) in [
        (
            DrawingSheetStandard::AnsiA,
            "ASME Y14.1 · 6.25 × 2.00 in",
            inch(625),
        ),
        (
            DrawingSheetStandard::ArchA,
            "ASME Y14.1 format · 6.57 × 2.00 in",
            inch(657),
        ),
    ] {
        let format = upright(standard, true);
        assert!(!format.title_block_substituted(), "{standard:?}");
        assert!(format.asme_title_block_form().unwrap().figure_minimum);
        assert_eq!(
            format.title_block_template_label(format.title_block.template),
            label
        );
        let layout = layout_of(&format);
        assert_eq!(layout.width_um, block, "{standard:?}");
        assert_eq!(cell(&layout, "TITLE").x_um, inch(200), "{standard:?}");
        assert_eq!(cell(&layout, "DRAWN").values[0].capacity(), 8);
        let strip = ["PROJECT", "CELL / VIEW", "STATUS", "CLASSIFICATION", "LANG"]
            .map(|heading| cell(&layout, heading).width_um);
        assert_eq!(
            strip,
            [100, 200 + (block - inch(625)) / 254, 150, 125, 50].map(inch),
            "{standard:?}"
        );
    }
}

#[test]
fn a_cell_view_path_too_long_for_its_cell_keeps_its_cell_and_view() {
    let format = showing_supplementary(
        DrawingSheetStandard::AnsiB,
        &ASME_Y14_1_SUPPLEMENTARY_FIELDS,
    );
    let layout = layout_of(&format);
    let path = layout.value(Field::CellView).unwrap();
    assert_eq!(
        path.letter("mycircuit/top/schematic"),
        ["mycircuit/top/schematic"]
    );
    // Cut at its start, at the separator before the cell.
    assert_eq!(
        path.letter("analog_reference_library/top/schematic"),
        ["…/top/schematic"]
    );
    assert!(path.elides("analog_reference_library/top/schematic"));
    // A cell name too long to keep whole keeps as much of its end as fits.
    assert_eq!(
        path.letter("lib/bandgap_reference_core_trimmed/schematic"),
        ["…_core_trimmed/schematic"]
    );
}

#[test]
fn a_value_the_form_still_cuts_short_is_reported_for_the_sheet_to_say_so() {
    let format = sheet(DrawingSheetStandard::AnsiB);
    let layout = layout_of(&format);
    let long_name = "Maximilian Featherstonehaugh-Smythe";
    let fields = resolved(
        &format,
        &[
            (Field::DrawnBy, long_name),
            (Field::CheckedBy, "R. Okafor"),
            (Field::DocumentId, "RS-0001-SCH-0042"),
        ],
    );
    assert_eq!(layout.cut_fields(&fields), [Field::DrawnBy]);
    assert!(
        lettered(&layout, &fields)
            .iter()
            .any(|line| line.ends_with('…') && long_name.starts_with(line.trim_end_matches('…')))
    );
    assert_eq!(
        DRAWING_SHEET_TITLE_VALUE_CUT_ADVISORY,
        "Longer than its cell in this sheet's title block holds; it prints cut short."
    );
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

#[test]
fn fields_with_no_asme_block_stand_in_a_strip_above_it_never_inside_it() {
    use DrawingSheetStandard::{AnsiB, AnsiD, ArchD};
    let every = ASME_Y14_1_SUPPLEMENTARY_FIELDS;
    for (standard, widths) in [
        (AnsiB, [100, 252, 150, 211, 37]),
        (AnsiD, [125, 351, 150, 211, 50]),
    ] {
        let layout = layout_of(&showing_supplementary(standard, &every));
        let figure = layout_of(&sheet(standard));
        let strip = inch(37);
        assert_eq!(layout.height_um, figure.height_um + strip);
        let mut x = 0;
        for ((heading, width), field) in
            ["PROJECT", "CELL / VIEW", "STATUS", "CLASSIFICATION", "LANG"]
                .into_iter()
                .zip(widths)
                .zip(every)
        {
            let cell = cell(&layout, heading);
            assert_eq!(
                bounds(cell),
                (x, 0, inch(width), strip),
                "{standard:?} {heading}"
            );
            assert_eq!(cell.field(), Some(field));
            assert_eq!(cell.letter_caption(), heading, "the heading fits");
            x += inch(width);
        }
        // Blocks A to L stand under the strip exactly as the figure draws
        // them, and the strip's foot is the block's thick outline.
        for heading in [
            "TITLE", "SIZE", "DAI", "DWG NO", "REV", "SCALE", "SHEET", "DRAWN",
        ] {
            let (x, y, width, height) = bounds(cell(&figure, heading));
            assert_eq!(
                bounds(cell(&layout, heading)),
                (x, y + strip, width, height)
            );
        }
        assert!(layout.rules.iter().any(|rule| {
            (
                rule.from_x_um,
                rule.from_y_um,
                rule.to_x_um,
                rule.to_y_um,
                rule.width_um,
            ) == (0, strip, layout.width_um, strip, 600)
        }));
        assert!(
            layout
                .rules
                .iter()
                .filter(|rule| rule.to_y_um <= strip && rule.from_x_um == rule.to_x_um)
                .all(|rule| rule.width_um == 300)
        );
    }
    // An ARCH block's strip widens CELL / VIEW by what SIZE gained.
    let arch = layout_of(&showing_supplementary(ArchD, &every));
    assert_eq!(cell(&arch, "CELL / VIEW").width_um, inch(351 + 123));
}

#[test]
fn a_hidden_supplementary_field_gives_its_cell_to_its_neighbour() {
    let layout_showing = |fields: &[DrawingSheetTitleFieldId]| {
        layout_of(&showing_supplementary(DrawingSheetStandard::AnsiB, fields))
    };
    // A new sheet shows Cell / view and the status: Project's width goes to
    // Cell / view on its right, Classification's and Language's to the
    // status on their left.
    let new = layout_of(&SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiB,
        SchematicPageOrientation::Landscape,
    ));
    assert_eq!(
        bounds(cell(&new, "CELL / VIEW")),
        (0, 0, inch(352), inch(37))
    );
    assert_eq!(
        bounds(cell(&new, "STATUS")),
        (inch(352), 0, inch(398), inch(37))
    );
    assert_eq!(
        new,
        layout_showing(&[Field::CellView, Field::DocumentStatus])
    );
    let alone = layout_showing(&[Field::Language]);
    assert_eq!(bounds(cell(&alone, "LANG")), (0, 0, inch(750), inch(37)));
    // A sheet showing none of them carries no strip at all.
    let none = layout_showing(&[]);
    assert_eq!(none.height_um, inch(200));
    assert!(
        none.cells
            .iter()
            .all(|cell| cell.caption.is_none_or(|caption| caption.text != "LANG"))
    );
}

#[test]
fn the_asme_form_letters_no_document_type_and_the_status_in_capitals() {
    let format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiB,
        SchematicPageOrientation::Landscape,
    )
    .try_update(|draft| {
        let document_type = draft
            .title_block
            .fields
            .get_mut(&Field::DocumentType)
            .unwrap();
        document_type.visible = true;
        document_type.value = "Schematic diagram".to_owned();
    })
    .unwrap();
    assert!(
        !format
            .title_block_field_order()
            .contains(&Field::DocumentType)
    );
    assert!(!format.title_field_shown(Field::DocumentType));
    let text = lettered(
        &layout_of(&format),
        &resolved(
            &format,
            &[
                (Field::DocumentStatus, "Under approval"),
                (Field::CellView, "lib/top/schematic"),
            ],
        ),
    );
    assert!(text.iter().any(|line| line == "UNDER APPROVAL"), "{text:?}");
    assert!(
        text.iter().any(|line| line == "lib/top/schematic"),
        "{text:?}"
    );
    assert!(!text.iter().any(|line| line.contains("Schematic diagram")));
}

#[test]
fn rev_stays_blank_until_the_document_is_issued_a_revision() {
    // ASME Y14.35 reads a dash in REV as the initial release, so a sheet not
    // yet issued a revision letters nothing there.
    let format = sheet(DrawingSheetStandard::AnsiB);
    let layout = layout_of(&format);
    let rev = |fields: &[ResolvedDrawingSheetTitleField]| {
        let cell = layout.cell(Field::Revision).unwrap();
        layout
            .lettering(fields)
            .into_iter()
            .filter(|line| layout.cells[line.cell] == *cell && line.text != "REV")
            .map(|line| line.text)
            .collect::<Vec<_>>()
    };
    let draft = resolved(&format, &[]);
    assert_eq!(
        draft
            .iter()
            .find(|field| field.id == Field::Revision)
            .unwrap()
            .value,
        ""
    );
    assert!(rev(&draft).is_empty());
    assert_eq!(rev(&resolved(&format, &[(Field::Revision, "B")])), ["B"]);
    // A dash is lettered only when it is the revision entered, for the
    // initial release.
    assert_eq!(rev(&resolved(&format, &[(Field::Revision, "-")])), ["-"]);
    // ISO 7200's form still reads an index not yet issued as not yet known.
    let iso = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA3,
        SchematicPageOrientation::Landscape,
    );
    let iso_revision = resolve_drawing_sheet_title_fields(&iso, &BTreeMap::new())
        .into_iter()
        .find(|field| field.id == Field::Revision)
        .unwrap();
    assert_eq!(iso_revision.value, "\u{2014}");
}

#[test]
fn a_revision_outside_the_y14_35_sequence_is_advised_against_on_the_asme_form() {
    let asme = DrawingSheetTitleBlockTemplate::AsmeY141;
    for issued in ["", "-", "A", "B", "Y", "AA", "AB", "BY", "YY"] {
        assert_eq!(asme.revision_advisory(issued), None, "{issued}");
    }
    for letter in ["I", "O", "Q", "S", "X", "Z", "AI", "ZA"] {
        assert_eq!(
            asme.revision_advisory(letter),
            Some("ASME Y14.35 letters a revision in capitals from A, without I, O, Q, S, X or Z."),
            "{letter}"
        );
    }
    for other in ["b", "1", "A1", "B-2"] {
        assert!(asme.revision_advisory(other).is_some(), "{other}");
    }
    assert_eq!(
        asme.revision_advisory("ABC"),
        Some("ASME Y14.35 gives a revision two letters at most.")
    );
    // Every other block keeps ISO 7200's advice: Q and S are fine there.
    let iso = DrawingSheetTitleBlockTemplate::Standard;
    assert_eq!(iso.revision_advisory("Q"), None);
    assert_eq!(
        iso.revision_advisory("O"),
        drawing_sheet_revision_advisory("O")
    );
}

#[test]
fn sheet_one_of_several_says_all_sheets_are_the_same_revision_near_its_block() {
    for standard in [
        DrawingSheetStandard::AnsiA,
        DrawingSheetStandard::AnsiD,
        DrawingSheetStandard::ArchE1,
    ] {
        let format =
            SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape);
        let geometry = format.geometry().unwrap();
        let block = geometry.title_block.unwrap();
        let note = format
            .same_revision_note(&geometry, "1 of 3")
            .expect("sheet one of three carries the note");
        assert_eq!(note.text, "ALL SHEETS ARE THE SAME REVISION LEVEL");
        assert_eq!(note.lettering_um, 3_000);
        // Directly above the block, ending at its right edge, and inside
        // the drawing area.
        let em = drawing_sheet_mono_em_um(note.lettering_um) as i64;
        let width = (drawing_sheet_mono_advance_um(note.lettering_um) * 38) as i64;
        assert!(note.seat_um + em * 587 / 1_000 < block.y_um, "{standard:?}");
        assert!(note.seat_um > block.y_um - 10_000, "{standard:?}");
        assert!(note.x_um >= block.x_um);
        assert!(note.x_um + width <= block.x_um + block.width_um as i64);
        assert!(note.seat_um - em > geometry.drawing_area.y_um);
        // Only sheet one of a drawing of several.
        for page in ["2 of 3", "1 of 1", "not a page"] {
            assert_eq!(format.same_revision_note(&geometry, page), None, "{page}");
        }
    }
    // Not on ISO 7200's block, nor on an ANSI sheet that chose it.
    for format in [
        SchematicSheetFormat::from_standard(
            DrawingSheetStandard::IsoA3,
            SchematicPageOrientation::Landscape,
        ),
        sheet(DrawingSheetStandard::AnsiB)
            .try_update(|draft| {
                draft.title_block.template = DrawingSheetTitleBlockTemplate::Standard;
            })
            .unwrap(),
    ] {
        let geometry = format.geometry().unwrap();
        assert_eq!(format.same_revision_note(&geometry, "1 of 3"), None);
    }
    assert_eq!(
        DrawingSheetNumber::from_page_label("12 of 12"),
        Some(DrawingSheetNumber {
            number: 12,
            count: 12
        })
    );
}

#[test]
fn sheets_mix_sizes_when_any_two_differ_whatever_each_is_called() {
    let standard = |standard| AuthoredDrawingSheetSize::Standard { standard };
    let b = standard(DrawingSheetStandard::AnsiB);
    let d = standard(DrawingSheetStandard::AnsiD);
    assert!(!drawing_sheet_sizes_mixed([]));
    assert!(!drawing_sheet_sizes_mixed([&b]));
    assert!(!drawing_sheet_sizes_mixed([&b, &b, &b]));
    assert!(drawing_sheet_sizes_mixed([&b, &b, &d]));
    // A custom size the size of ANSI B is the same paper.
    let custom = SchematicSheetFormat::try_custom(
        "Ledger",
        279_400,
        431_800,
        SchematicPageOrientation::Landscape,
    )
    .unwrap()
    .authored_size;
    assert!(!drawing_sheet_sizes_mixed([&b, &custom]));
}

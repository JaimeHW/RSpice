//! A title block is a set of cells every renderer draws: ISO 7200's form as
//! designed, and an organization's block as it always flowed.

use super::*;

use DrawingSheetTitleFieldId as Field;

/// What an engineering sheet typically letters in each field it shows.
const TYPICAL_VALUES: [(DrawingSheetTitleFieldId, &str); 12] = [
    (Field::Project, "MyCircuit"),
    (Field::CellView, "mycircuit/top/schematic"),
    (Field::SheetTitle, "Bandgap reference core"),
    (Field::DocumentType, "Circuit diagram"),
    (Field::Page, "12 of 12"),
    (Field::Revision, "B"),
    (Field::Date, "2026-10-18"),
    (Field::DocumentStatus, "Under approval"),
    (Field::DrawnBy, "J. Whitfield"),
    (Field::ApprovedBy, "J. Whitfield"),
    (Field::Organization, "Whitfield Analog Devices"),
    (Field::DocumentId, "RS-0001-SCH-0042"),
];

fn layout_of(format: &SchematicSheetFormat) -> DrawingSheetTitleLayout {
    drawing_sheet_title_layout(format, &format.geometry().unwrap())
        .expect("the sheet carries a title block")
}

/// `format` showing exactly `fields`, and the fields its block requires.
fn showing(
    format: &SchematicSheetFormat,
    fields: &[DrawingSheetTitleFieldId],
) -> SchematicSheetFormat {
    format
        .try_update(|draft| {
            for (field, state) in &mut draft.title_block.fields {
                state.visible = field.policy().required_visible || fields.contains(field);
            }
        })
        .unwrap()
}

fn on(template: DrawingSheetTitleBlockTemplate) -> SchematicSheetFormat {
    SchematicSheetFormat::default()
        .try_update(|draft| draft.title_block.template = template)
        .unwrap()
}

/// A sheet on an organization's flowed block 130 × 32 mm in three rows: a
/// block small enough to crowd, as the retired compact block was.
fn small_flowed_block() -> SchematicSheetFormat {
    let snapshot = DrawingSheetManagedTemplateSnapshot::try_new(
        "example.small-title-block",
        "1",
        130_000,
        32_000,
        3,
        DrawingSheetTitleFieldId::ALL.to_vec(),
        Vec::new(),
    )
    .unwrap();
    SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            draft.title_block.managed_template = Some(snapshot);
        })
        .unwrap()
}

/// `format` with Project shown as well as whatever it already shows.
fn with_project(format: &SchematicSheetFormat) -> SchematicSheetFormat {
    format
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&Field::Project)
                .unwrap()
                .visible = true;
        })
        .unwrap()
}

/// A flowed grid's rows, top to bottom, each its fields left to right; a
/// blank strip is an empty row.
fn rows_of(layout: &DrawingSheetTitleLayout) -> Vec<Vec<DrawingSheetTitleFieldId>> {
    let mut tops = layout
        .cells
        .iter()
        .map(|cell| cell.y_um)
        .collect::<Vec<_>>();
    tops.sort_unstable();
    tops.dedup();
    tops.iter()
        .map(|top| {
            layout
                .cells
                .iter()
                .filter(|cell| cell.y_um == *top)
                .filter_map(DrawingSheetTitleCell::field)
                .collect()
        })
        .collect()
}

/// Every cell of the ISO form as `(field, x, y, width, height)` in mm/1000.
fn form_cells(layout: &DrawingSheetTitleLayout) -> Vec<(Field, u64, u64, u64, u64)> {
    layout
        .cells
        .iter()
        .map(|cell| {
            (
                cell.field().expect("every form cell is for a field"),
                cell.x_um,
                cell.y_um,
                cell.width_um,
                cell.height_um,
            )
        })
        .collect()
}

/// The optional fields of the ISO form, every combination of which it must
/// lay out.
const ISO_OPTIONAL: [Field; 6] = [
    Field::CheckedBy,
    Field::Classification,
    Field::DocumentStatus,
    Field::CellView,
    Field::Revision,
    Field::Language,
];

fn iso_with_optional(shown: &[Field]) -> SchematicSheetFormat {
    showing(&SchematicSheetFormat::default(), shown)
}

#[test]
fn the_iso_7200_form_is_a_fixed_set_of_cells() {
    let every = iso_with_optional(&ISO_OPTIONAL);
    let layout = layout_of(&every);
    assert_eq!((layout.width_um, layout.height_um), (180_000, 45_000));
    // Four rows of 11.25 mm, after ISO 7200's Figure 2: the people above,
    // the legal owner beside the title, the identification at lower right.
    assert_eq!(
        form_cells(&layout),
        [
            (Field::DrawnBy, 0, 0, 47_500, 11_250),
            (Field::CheckedBy, 47_500, 0, 47_500, 11_250),
            (Field::ApprovedBy, 95_000, 0, 47_500, 11_250),
            (Field::Classification, 142_500, 0, 37_500, 11_250),
            (Field::Organization, 0, 11_250, 47_500, 33_750),
            (Field::DocumentType, 47_500, 11_250, 62_500, 11_250),
            (Field::DocumentStatus, 110_000, 11_250, 70_000, 11_250),
            (Field::SheetTitle, 47_500, 22_500, 62_500, 22_500),
            (Field::DocumentId, 110_000, 22_500, 70_000, 11_250),
            (Field::Revision, 110_000, 33_750, 10_500, 11_250),
            (Field::Date, 120_500, 33_750, 26_000, 11_250),
            (Field::Page, 146_500, 33_750, 21_500, 11_250),
            (Field::Language, 168_000, 33_750, 12_000, 11_250),
        ]
    );
    // What each value line holds at its lettering: (height, characters).
    let held = |field| {
        let value = layout.value(field).unwrap();
        (value.lettering_um, value.capacity(), value.max_lines)
    };
    for (field, expected) in [
        (Field::DrawnBy, (2_500, 20, 1)),
        (Field::CheckedBy, (2_500, 20, 1)),
        (Field::ApprovedBy, (2_500, 20, 1)),
        (Field::Classification, (2_500, 15, 1)),
        (Field::Organization, (2_500, 20, 3)),
        (Field::DocumentType, (2_500, 27, 1)),
        (Field::DocumentStatus, (2_500, 30, 1)),
        (Field::SheetTitle, (2_500, 27, 1)),
        (Field::CellView, (2_500, 27, 1)),
        (Field::DocumentId, (3_500, 21, 1)),
        (Field::Revision, (2_500, 3, 1)),
        (Field::Date, (2_500, 10, 1)),
        (Field::Page, (2_500, 8, 1)),
        (Field::Language, (2_500, 3, 1)),
    ] {
        assert_eq!(held(field), expected, "{field:?}");
    }
    // The supplementary title is the title cell's second line, seated on the
    // value line of the row beside it; only the identifier is emphasised.
    let title = layout.cell(Field::SheetTitle).unwrap();
    assert_eq!(title.values.len(), 2);
    assert_eq!(
        layout.value(Field::CellView).unwrap().seat_um,
        layout.value(Field::Date).unwrap().seat_um
    );
    assert_eq!(
        layout.value(Field::SheetTitle).unwrap().seat_um,
        layout.value(Field::DocumentId).unwrap().seat_um
    );
    for cell in &layout.cells {
        for value in &cell.values {
            assert_eq!(value.emphasised, value.field() == Some(Field::DocumentId));
        }
    }
}

#[test]
fn the_iso_form_letters_iso_7200s_captions() {
    let layout = layout_of(&iso_with_optional(&ISO_OPTIONAL));
    let captions = layout
        .cells
        .iter()
        .map(DrawingSheetTitleCell::letter_caption)
        .collect::<Vec<_>>();
    assert_eq!(
        captions,
        [
            "Created by",
            "Checked by",
            "Approved by",
            "Classification",
            "Legal owner",
            "Document type",
            "Document status",
            "Title",
            "Identification number",
            "Rev.",
            "Date of issue",
            "Sheet",
            "Lang.",
        ]
    );
    for cell in &layout.cells {
        let caption = cell.caption.unwrap();
        assert_eq!(caption.lettering_um, 1_800);
        assert_eq!(caption.placement, DrawingSheetTitleCaptionPlacement::Over);
    }
    // The panels name each field by the same ISO term, in full.
    for (field, name) in [
        (Field::SheetTitle, "Title"),
        (Field::DrawnBy, "Created by"),
        (Field::Organization, "Legal owner"),
        (Field::DocumentId, "Identification number"),
        (Field::Revision, "Revision"),
        (Field::Date, "Date of issue"),
        (Field::Page, "Sheet"),
        (Field::Language, "Language"),
    ] {
        assert_eq!(field.display_label(), name);
    }
}

#[test]
fn the_iso_form_holds_what_iso_7200_recommends_each_field_hold() {
    let layout = layout_of(&iso_with_optional(&ISO_OPTIONAL));
    for field in ISO_7200_FORM_FIELDS {
        let value = layout.value(field).unwrap();
        match (field, iso_7200_typical_chars(field)) {
            // ISO 7200 gives a document type 30 characters; the form holds
            // its terms whole, IEC 61082's "Circuit diagram" among them.
            (Field::DocumentType, _) => {
                assert!(!value.elides(DRAWING_SHEET_DEFAULT_DOCUMENT_TYPE));
                assert!(!value.elides("Wiring diagram"));
            }
            (_, Some(chars)) => assert!(
                value.capacity() >= chars,
                "{field:?} holds {} of ISO 7200's {chars}",
                value.capacity()
            ),
            (_, None) => {}
        }
    }
    // The figures every engineering sheet letters, whole.
    for (field, typical) in TYPICAL_VALUES {
        let Some(value) = layout.value(field) else {
            assert_eq!(field, Field::Project, "the form has no project field");
            continue;
        };
        assert!(!value.elides(typical), "{field:?} cuts {typical:?}");
    }
    for status in DrawingSheetReleaseStatus::ALL {
        assert!(
            !layout
                .value(Field::DocumentStatus)
                .unwrap()
                .elides(status.label())
        );
    }
}

#[test]
fn a_hidden_optional_field_merges_into_its_neighbour() {
    // A new sheet shows neither Checked by, Classification nor the language
    // code: each merges into the cell on its left, so Created by takes
    // Checked by's place, Approved by Classification's and Sheet Lang.'s.
    // No rule moves, and no other cell does.
    let layout = layout_of(&SchematicSheetFormat::default());
    assert_eq!(
        form_cells(&layout),
        [
            (Field::DrawnBy, 0, 0, 95_000, 11_250),
            (Field::ApprovedBy, 95_000, 0, 85_000, 11_250),
            (Field::Organization, 0, 11_250, 47_500, 33_750),
            (Field::DocumentType, 47_500, 11_250, 62_500, 11_250),
            (Field::DocumentStatus, 110_000, 11_250, 70_000, 11_250),
            (Field::SheetTitle, 47_500, 22_500, 62_500, 22_500),
            (Field::DocumentId, 110_000, 22_500, 70_000, 11_250),
            (Field::Revision, 110_000, 33_750, 10_500, 11_250),
            (Field::Date, 120_500, 33_750, 26_000, 11_250),
            (Field::Page, 146_500, 33_750, 33_500, 11_250),
        ]
    );
    // A cell spanning rows is not a neighbour to merge into: the revision
    // starts its row beside the title, so it merges into the date of issue.
    let without_revision = layout_of(&iso_with_optional(&[]));
    let date = without_revision.cell(Field::Date).unwrap();
    assert_eq!((date.x_um, date.width_um), (110_000, 36_500));
    let status_gone = without_revision.cell(Field::DocumentType).unwrap();
    assert_eq!((status_gone.x_um, status_gone.width_um), (47_500, 132_500));
    let title = without_revision.cell(Field::SheetTitle).unwrap();
    assert_eq!((title.x_um, title.width_um), (47_500, 62_500));
    // Hiding the supplementary title takes its line out of the title cell,
    // which keeps its place.
    assert!(without_revision.value(Field::CellView).is_none());
    assert_eq!(title.values.len(), 1);

    // Every combination of optional fields tiles the block exactly.
    for mask in 0..1_u32 << ISO_OPTIONAL.len() {
        let shown = ISO_OPTIONAL
            .iter()
            .enumerate()
            .filter(|(bit, _)| mask & (1 << bit) != 0)
            .map(|(_, field)| *field)
            .collect::<Vec<_>>();
        let layout = layout_of(&iso_with_optional(&shown));
        let area = layout
            .cells
            .iter()
            .map(|cell| cell.width_um * cell.height_um)
            .sum::<u64>();
        assert_eq!(area, 180_000 * 45_000, "{shown:?} leaves a gap or overlap");
        for (index, cell) in layout.cells.iter().enumerate() {
            for other in &layout.cells[index + 1..] {
                let apart = cell.x_um + cell.width_um <= other.x_um
                    || other.x_um + other.width_um <= cell.x_um
                    || cell.y_um + cell.height_um <= other.y_um
                    || other.y_um + other.height_um <= cell.y_um;
                assert!(
                    apart,
                    "{shown:?}: {:?} overlaps {:?}",
                    cell.field(),
                    other.field()
                );
            }
            assert!(cell.caption.is_some() && !cell.values.is_empty());
        }
        let shown_fields = layout
            .cells
            .iter()
            .flat_map(|cell| cell.values.iter().filter_map(|value| value.field()))
            .collect::<Vec<_>>();
        let expected = ISO_7200_FORM_FIELDS
            .into_iter()
            .filter(|field| !ISO_OPTIONAL.contains(field) || shown.contains(field))
            .collect::<Vec<_>>();
        assert_eq!(shown_fields, expected, "{shown:?}");
    }
}

/// Whatever a sheet hides, every rule stands where the form designs one: a
/// vertical rule at one of its column lines, a horizontal one on a row line.
#[test]
fn the_iso_forms_rules_stand_only_where_it_designs_them() {
    const COLUMNS_UM: [u64; 7] = [47_500, 95_000, 110_000, 120_500, 142_500, 146_500, 168_000];
    const ROWS_UM: [u64; 3] = [11_250, 22_500, 33_750];
    for mask in 0..1_u32 << ISO_OPTIONAL.len() {
        let shown = ISO_OPTIONAL
            .iter()
            .enumerate()
            .filter(|(bit, _)| mask & (1 << bit) != 0)
            .map(|(_, field)| *field)
            .collect::<Vec<_>>();
        let layout = layout_of(&iso_with_optional(&shown));
        for rule in &layout.rules {
            if rule.from_x_um == rule.to_x_um {
                assert!(COLUMNS_UM.contains(&rule.from_x_um), "{shown:?}: {rule:?}");
            } else {
                assert!(ROWS_UM.contains(&rule.from_y_um), "{shown:?}: {rule:?}");
            }
        }
        // Every cell edge is a designed line too, or the block's own edge.
        for cell in &layout.cells {
            for x in [cell.x_um, cell.x_um + cell.width_um] {
                assert!(
                    x == 0 || x == 180_000 || COLUMNS_UM.contains(&x),
                    "{shown:?}"
                );
            }
        }
    }
}

/// A vertical rule in one row either continues a rule in the row beside it
/// or stands clear of every rule there, so no two nearly meet.
#[test]
fn the_iso_forms_vertical_rules_stand_on_a_common_grid() {
    const ROW_UM: u64 = 11_250;
    for mask in 0..1_u32 << ISO_OPTIONAL.len() {
        let shown = ISO_OPTIONAL
            .iter()
            .enumerate()
            .filter(|(bit, _)| mask & (1 << bit) != 0)
            .map(|(_, field)| *field)
            .collect::<Vec<_>>();
        let layout = layout_of(&iso_with_optional(&shown));
        let in_row = |row: u64| {
            layout
                .rules
                .iter()
                .filter(|rule| {
                    rule.from_x_um == rule.to_x_um
                        && rule.from_y_um <= row * ROW_UM
                        && (row + 1) * ROW_UM <= rule.to_y_um
                })
                .map(|rule| rule.from_x_um)
                .collect::<Vec<_>>()
        };
        for row in 0..3 {
            for x in in_row(row) {
                for beside in in_row(row + 1) {
                    assert!(
                        x == beside || x.abs_diff(beside) >= 10_000,
                        "{shown:?}: rows {row} and {} rule at {x} and {beside} um",
                        row + 1
                    );
                }
            }
        }
    }
    // The legal owner's rule runs from the top of the block to its foot and
    // the title's from the second row down, each drawn once.
    let layout = layout_of(&iso_with_optional(&ISO_OPTIONAL));
    assert!(layout.rules.contains(&DrawingSheetTitleRule {
        from_x_um: 47_500,
        from_y_um: 0,
        to_x_um: 47_500,
        to_y_um: 45_000,
        width_um: DRAWING_SHEET_TITLE_RULE_UM,
    }));
    assert!(layout.rules.contains(&DrawingSheetTitleRule {
        from_x_um: 110_000,
        from_y_um: 11_250,
        to_x_um: 110_000,
        to_y_um: 45_000,
        width_um: DRAWING_SHEET_TITLE_RULE_UM,
    }));
    let mut rules = layout.rules.clone();
    rules.sort_unstable();
    rules.dedup();
    assert_eq!(rules.len(), layout.rules.len());
    // No rule lies on the outline, and none crosses the title cell between
    // its two lines.
    for rule in &layout.rules {
        assert!(rule.from_y_um != rule.to_y_um || (1..45_000).contains(&rule.from_y_um));
        assert!(rule.from_x_um != rule.to_x_um || (1..180_000).contains(&rule.from_x_um));
        let crosses_title =
            rule.from_y_um == 33_750 && rule.from_x_um < 110_000 && rule.to_x_um > 47_500;
        assert!(!crosses_title, "{rule:?}");
    }
}

#[test]
fn the_iso_forms_mandatory_fields_stay_on_it() {
    // A sheet saved hiding every field still draws the eight ISO 7200 makes
    // mandatory, without its saved choices being rewritten.
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            for (field, state) in &mut draft.title_block.fields {
                state.visible = field.policy().required_visible;
            }
        })
        .unwrap();
    for field in ISO_7200_MANDATORY_FIELDS {
        assert!(
            DrawingSheetTitleBlockTemplate::Standard.requires_field(field),
            "{field:?}"
        );
        assert!(format.title_field_shown(field), "{field:?}");
        assert!(layout_of(&format).value(field).is_some(), "{field:?}");
    }
    assert!(
        !format.title_block.fields[&Field::Organization].visible,
        "the saved choice is kept"
    );
    // The cell / view and the revision are optional, there and everywhere.
    for field in [Field::CellView, Field::Revision] {
        assert!(!field.policy().required_visible);
        assert!(!DrawingSheetTitleBlockTemplate::Standard.requires_field(field));
        assert!(field.visible_by_default());
        assert!(!format.title_field_shown(field));
    }
    // Off the form the mandatory fields are the sheet's to hide again.
    let flowed = format
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
        })
        .unwrap();
    assert!(!flowed.title_field_shown(Field::Organization));
    assert!(
        !DrawingSheetTitleBlockTemplate::OrganizationManaged.requires_field(Field::Organization)
    );
    // Project and Paper size are not on the form, whatever a sheet shows.
    let every = showing(
        &SchematicSheetFormat::default(),
        &DrawingSheetTitleFieldId::ALL,
    );
    for field in [Field::Project, Field::PaperSize] {
        assert!(!every.title_field_shown(field));
        assert!(!every.title_block_field_order().contains(&field));
        assert!(layout_of(&every).value(field).is_none());
        assert!(
            with_project(&flowed)
                .title_block_field_order()
                .contains(&field)
        );
    }
}

#[test]
fn a_long_legal_owner_wraps_in_its_cell() {
    let layout = layout_of(&SchematicSheetFormat::default());
    let owner = layout.value(Field::Organization).unwrap();
    assert_eq!(
        owner.letter("Whitfield Analog Devices"),
        ["Whitfield Analog", "Devices"]
    );
    assert!(!owner.elides("Whitfield Analog Devices"));
    // ISO 3098's baseline spacing for lettering that may carry diacritics.
    assert_eq!(owner.line_pitch_um, 4_750);
    let long = "Whitfield Analog Devices Precision Reference Group of Companies";
    let lines = owner.letter(long);
    assert_eq!(lines.len(), 3);
    assert!(lines.last().unwrap().ends_with('…'));
    assert!(owner.elides(long));
    // A word too long for a line is broken where the line ends.
    assert_eq!(
        owner.letter("Abcdefghijklmnopqrstuvwxyz"),
        ["Abcdefghijklmnopqrst", "uvwxyz"]
    );
    // A single-line value never wraps.
    let title = layout.value(Field::SheetTitle).unwrap();
    assert_eq!(title.letter(long).len(), 1);
}

#[test]
fn a_stretched_iso_block_keeps_its_proportions() {
    let strip = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.anchor = DrawingSheetTitleBlockAnchor::BottomStrip;
        })
        .unwrap();
    let layout = layout_of(&strip);
    assert!(layout.width_um > 180_000);
    let owner = layout.cell(Field::Organization).unwrap();
    assert_eq!(owner.width_um, 47_500 * layout.width_um / 180_000);
    let page = layout.cell(Field::Page).unwrap();
    assert_eq!(page.x_um + page.width_um, layout.width_um);
}

#[test]
fn every_lettered_line_stays_inside_its_cell() {
    // A line's capitals rise 7/20 of its em above its seat and its deepest
    // descender reaches 12/20 below; lines under one another in a cell keep
    // a tenth of the lower line's height between them.
    let filled = |format: &SchematicSheetFormat| {
        let layout = layout_of(format);
        let resolved = layout
            .cells
            .iter()
            .flat_map(|cell| cell.values.iter().filter_map(|value| value.field()))
            .map(|id| ResolvedDrawingSheetTitleField {
                id,
                value: "Gjpqy Whitfield Analog Devices Ltd".to_owned(),
                authority: DrawingSheetTitleFieldValueAuthority::Authored,
            })
            .collect::<Vec<_>>();
        (layout.lettering(&resolved), layout)
    };
    for format in [
        iso_with_optional(&ISO_OPTIONAL),
        on(DrawingSheetTitleBlockTemplate::OrganizationManaged),
    ] {
        let (lines, layout) = filled(&format);
        assert!(!lines.is_empty());
        for (index, line) in lines.iter().enumerate() {
            let cell = &layout.cells[line.cell];
            let em_um = drawing_sheet_mono_em_um(line.lettering_um);
            let (top_um, foot_um) = (
                line.seat_um - em_um * 7 / 20,
                line.seat_um + em_um * 12 / 20,
            );
            assert!(
                cell.y_um < top_um && foot_um <= cell.y_um + cell.height_um,
                "{:?} {:?} leaves its cell",
                format.title_block.template,
                line.text
            );
            assert!(line.x_um >= cell.x_um + DRAWING_SHEET_TITLE_CELL_PADDING_UM);
            if let Some(next) = lines.get(index + 1).filter(|next| next.cell == line.cell) {
                let next_top_um =
                    next.seat_um - drawing_sheet_mono_em_um(next.lettering_um) * 7 / 20;
                assert!(
                    foot_um + next.lettering_um / 10 <= next_top_um,
                    "{:?} crowds {:?}",
                    line.text,
                    next.text
                );
            }
        }
    }
}

#[test]
fn a_new_sheet_letters_typical_values_whole() {
    let new_sheet = SchematicSheetFormat::default();
    assert_eq!(
        new_sheet.geometry().unwrap().effective_title_block_template,
        DrawingSheetTitleBlockTemplate::Standard
    );
    let layout = layout_of(&new_sheet);
    let resolved = TYPICAL_VALUES
        .into_iter()
        .map(|(id, value)| ResolvedDrawingSheetTitleField {
            id,
            value: value.to_owned(),
            authority: DrawingSheetTitleFieldValueAuthority::Authored,
        })
        .collect::<Vec<_>>();
    let lettered = layout
        .lettering(&resolved)
        .into_iter()
        .map(|line| line.text)
        .collect::<Vec<_>>();
    assert!(
        !lettered.iter().any(|line| line.contains('…')),
        "{lettered:?}"
    );
    for (field, value) in TYPICAL_VALUES {
        if field == Field::Project || field == Field::Organization {
            continue;
        }
        assert!(
            lettered.iter().any(|line| line == value),
            "{value:?}: {lettered:?}"
        );
    }
}

#[test]
fn a_small_flowed_block_letters_its_fixed_form_values_whole() {
    let compact = small_flowed_block();
    let mut fixed = vec![
        (Field::Page, "12 of 12"),
        (Field::Revision, "C"),
        (Field::Date, "2026-10-18"),
        (Field::DocumentType, DRAWING_SHEET_DEFAULT_DOCUMENT_TYPE),
    ];
    fixed.extend(
        DrawingSheetReleaseStatus::ALL
            .into_iter()
            .map(|status| (Field::DocumentStatus, status.label())),
    );
    for format in [compact.clone(), with_project(&compact)] {
        let layout = layout_of(&format);
        assert_eq!(layout.width_um, 130_000);
        for (field, value) in &fixed {
            let line = layout.value(*field).expect("a new sheet shows the field");
            assert!(!line.elides(value), "{field:?} cuts {value:?}");
            assert_eq!(
                layout.cell(*field).unwrap().letter_caption(),
                field.grid_caption()
            );
        }
    }
}

/// A sheet that shows every field on a small flowed block — every field but
/// the language code and the ASME form's fields — has more to letter than
/// the block holds. The free text gives way, then the document status; a
/// page number, revision, paper size, date or classification is still
/// lettered whole, and every caption is.
#[test]
fn an_overfull_block_cuts_free_text_and_keeps_fixed_forms_whole() {
    let compact = small_flowed_block();
    let saved = DrawingSheetTitleFieldId::ALL
        .into_iter()
        .filter(|field| {
            !matches!(
                field,
                Field::Language
                    | Field::CageCode
                    | Field::Engineer
                    | Field::DrawnDate
                    | Field::CheckedDate
                    | Field::EngineerDate
                    | Field::ApprovedDate
                    | Field::ContractNumber
            )
        })
        .collect::<Vec<_>>();
    let every = showing(&compact, &saved);
    let layout = layout_of(&every);
    assert_eq!(
        layout
            .cells
            .iter()
            .filter_map(DrawingSheetTitleCell::field)
            .count(),
        saved.len()
    );
    let fixed = [
        (Field::Page, "12 of 12"),
        (Field::Revision, "C"),
        (Field::PaperSize, "ANSI B"),
        (Field::Date, "2026-10-18"),
        (Field::Classification, "Confidential"),
    ];
    for (field, value) in fixed {
        let cell = layout.cell(field).expect("every field is shown");
        assert!(
            !cell.value(field).unwrap().elides(value),
            "{field:?} cuts {value:?}"
        );
        assert!(cell.width_um >= field.title_cell_need_um(), "{field:?}");
    }
    for cell in &layout.cells {
        let field = cell.field().unwrap();
        assert_eq!(cell.letter_caption(), field.grid_caption());
    }
    // The block is over-full, so something gives way, and it is free text or
    // the status.
    let cut = TYPICAL_VALUES
        .iter()
        .filter(|(field, value)| layout.value(*field).is_some_and(|line| line.elides(value)))
        .map(|(field, _)| *field)
        .collect::<Vec<_>>();
    assert!(!cut.is_empty());
    assert!(
        cut.iter()
            .all(|field| !fixed.iter().any(|(fixed, _)| fixed == field))
    );
}

#[test]
fn every_flowed_row_spans_the_field_grid_exactly() {
    let every = DrawingSheetTitleFieldId::ALL;
    let template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
    for block in [small_flowed_block(), on(template)] {
        for rotation in [
            DrawingSheetTitleBlockRotation::Upright,
            DrawingSheetTitleBlockRotation::Clockwise90,
        ] {
            let placed = block
                .try_update(|draft| draft.title_block.rotation = rotation)
                .unwrap();
            for format in [
                placed.clone(),
                showing(&placed, &[]),
                showing(&placed, &every),
            ] {
                let layout = layout_of(&format);
                let resolved = resolve_drawing_sheet_title_fields(&format, &BTreeMap::new())
                    .into_iter()
                    .map(|field| field.id)
                    .collect::<Vec<_>>();
                assert_eq!(
                    layout
                        .cells
                        .iter()
                        .filter_map(DrawingSheetTitleCell::field)
                        .collect::<Vec<_>>(),
                    resolved,
                    "{template:?} lays out the fields it resolves, in order"
                );
                let rows = rows_of(&layout);
                let rows_expected = format.title_block_rows(template).unwrap();
                assert_eq!(rows.len(), rows_expected, "{template:?}");
                let filled = rows.iter().take_while(|row| !row.is_empty()).count();
                assert_eq!(filled, rows_expected.min(resolved.len()), "{template:?}");
                assert!(rows[filled..].iter().all(Vec::is_empty));
                let mut tops = layout
                    .cells
                    .iter()
                    .map(|cell| cell.y_um)
                    .collect::<Vec<_>>();
                tops.sort_unstable();
                tops.dedup();
                for top in tops {
                    let mut x_um = layout.grid_x_um;
                    for cell in layout.cells.iter().filter(|cell| cell.y_um == top) {
                        assert_eq!(cell.x_um, x_um, "{template:?} row at {top} has a gap");
                        assert!(cell.width_um > 0);
                        x_um += cell.width_um;
                    }
                    assert_eq!(x_um, layout.width_um, "{template:?} row misses the edge");
                }
                // Every row is ruled under the one above it.
                for top in 1..rows_expected as u64 {
                    let y_um = layout.height_um * top / rows_expected as u64;
                    assert!(layout.rules.iter().any(|rule| rule.from_y_um == y_um
                        && rule.to_y_um == y_um
                        && rule.from_x_um == layout.grid_x_um
                        && rule.to_x_um == layout.width_um));
                }
            }
        }
    }
}

/// The cells' rules run straight down a flowed block. A block with room for
/// all its fields hold is an even grid; a crowded one stays in columns, and
/// only the column that must hold more is wider.
#[test]
fn flowed_columns_line_up_and_stay_even_while_everything_fits() {
    let column_edges = |layout: &DrawingSheetTitleLayout| {
        let mut edges = layout
            .cells
            .iter()
            .filter(|cell| cell.field().is_some())
            .map(|cell| cell.x_um)
            .collect::<Vec<_>>();
        edges.sort_unstable();
        edges.dedup();
        edges
    };
    let managed = on(DrawingSheetTitleBlockTemplate::OrganizationManaged);
    for format in [managed.clone(), with_project(&managed)] {
        let layout = layout_of(&format);
        let columns = layout.cells.iter().filter(|cell| cell.y_um == 0).count();
        assert_eq!(column_edges(&layout).len(), columns);
        let even_um = layout.grid_width_um() / columns as u64;
        for cell in &layout.cells {
            let Some(field) = cell.field() else {
                continue;
            };
            let ends_row = cell.x_um + cell.width_um == layout.width_um;
            if !ends_row {
                assert_eq!(cell.width_um, even_um, "{field:?}");
            }
            assert!(
                cell.width_um >= field.title_cell_need_um(),
                "{field:?} is {} um wide and needs {}",
                cell.width_um,
                field.title_cell_need_um()
            );
        }
    }

    let compact = layout_of(&small_flowed_block());
    let columns = compact.cells.iter().filter(|cell| cell.y_um == 0).count();
    assert_eq!(column_edges(&compact).len(), columns);
    let document_type = compact.cell(Field::DocumentType).unwrap();
    let cell_view = compact.cell(Field::CellView).unwrap();
    assert!(document_type.width_um > cell_view.width_um);

    // A caption is ASCII, so its byte length is its character count.
    for field in DrawingSheetTitleFieldId::ALL {
        assert!(field.grid_caption().is_ascii());
        assert!(field.display_label().is_ascii());
    }
}

/// Fields fill the rows from the top in the order the sheet lists them, and
/// every row gets at least one.
#[test]
fn flowed_fields_fill_the_rows_from_the_top_in_order() {
    let shown = [
        Field::Project,
        Field::CellView,
        Field::Revision,
        Field::DocumentType,
        Field::DrawnBy,
        Field::ApprovedBy,
        Field::Date,
        Field::DocumentStatus,
        Field::Organization,
        Field::DocumentId,
    ];
    // The small block has three rows, so twelve fields run four to a row.
    let compact = showing(&small_flowed_block(), &shown);
    assert_eq!(
        rows_of(&layout_of(&compact)),
        [
            vec![
                Field::Project,
                Field::CellView,
                Field::SheetTitle,
                Field::DocumentType
            ],
            vec![
                Field::Page,
                Field::Revision,
                Field::DrawnBy,
                Field::ApprovedBy
            ],
            vec![
                Field::Date,
                Field::DocumentStatus,
                Field::Organization,
                Field::DocumentId
            ],
        ]
    );
    // A sheet showing fewer fields than the block has rows gives each field
    // a row of its own, and the rows left over are blank strips.
    let sparse = showing(
        &on(DrawingSheetTitleBlockTemplate::OrganizationManaged),
        &[],
    );
    let layout = layout_of(&sparse);
    assert_eq!(
        rows_of(&layout),
        [
            vec![Field::SheetTitle],
            vec![Field::Page],
            vec![],
            vec![],
            vec![],
        ]
    );
}

#[test]
fn a_value_longer_than_its_cell_is_cut_with_an_ellipsis() {
    let advance_um = drawing_sheet_mono_advance_um(DRAWING_SHEET_TITLE_BLOCK_LETTERING.data_um);
    let value = |chars: u64| DrawingSheetTitleValue {
        text: DrawingSheetTitleText::Field(Field::CellView),
        x_um: 0,
        seat_um: 0,
        width_um: advance_um * chars,
        lettering_um: DRAWING_SHEET_TITLE_BLOCK_LETTERING.data_um,
        emphasised: false,
        max_lines: 1,
        line_pitch_um: 0,
        upper_case: false,
    };
    assert_eq!(value(5).letter("ABCDEFG"), ["ABCD…"]);
    assert!(value(5).elides("ABCDEFG"));
    assert_eq!(value(5).letter("ABCDE"), ["ABCDE"]);
    assert!(!value(5).elides("ABCDE"));
    assert_eq!(value(4).letter("ééééé"), ["ééé…"]);
    assert_eq!(value(4).letter(""), [""]);
    // Lettering never shrinks to fit: a cell too narrow for one character
    // still letters the ellipsis rather than smaller text.
    assert_eq!(value(0).letter("Anything"), ["…"]);
}

//! A title block's cells are sized by what they hold, and every renderer
//! draws the one layout.

use super::*;

use DrawingSheetTitleFieldId as Field;

/// What an engineering sheet typically letters in each field it shows.
const TYPICAL_VALUES: [(DrawingSheetTitleFieldId, &str); 12] = [
    (Field::Project, "MyCircuit"),
    (Field::CellView, "mycircuit/top/schematic"),
    (Field::SheetTitle, "Bandgap reference core"),
    (Field::DocumentType, "Circuit diagram"),
    (Field::Page, "12 of 12"),
    (Field::Revision, "C"),
    (Field::Date, "2026-10-18"),
    (Field::DocumentStatus, "In review"),
    (Field::DrawnBy, "J. Whitfield"),
    (Field::ApprovedBy, "J. Whitfield"),
    (Field::Organization, "Whitfield Analog Devices"),
    (Field::DocumentId, "RS-0001-SCH-0042"),
];

fn layout_of(format: &SchematicSheetFormat) -> DrawingSheetTitleLayout {
    drawing_sheet_title_layout(format, &format.geometry().unwrap())
        .expect("the sheet carries a title block")
}

/// `format` showing exactly `fields`, and the fields a sheet must show.
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

fn rows_of(layout: &DrawingSheetTitleLayout) -> Vec<Vec<DrawingSheetTitleFieldId>> {
    let mut rows = vec![Vec::new(); layout.rows];
    for cell in &layout.cells {
        rows[cell.row].push(cell.id);
    }
    rows
}

#[test]
fn a_new_sheet_letters_typical_values_whole() {
    let new_sheet = SchematicSheetFormat::default();
    assert_eq!(
        new_sheet.geometry().unwrap().effective_title_block_template,
        DrawingSheetTitleBlockTemplate::Standard
    );
    for format in [new_sheet.clone(), with_project(&new_sheet)] {
        let layout = layout_of(&format);
        for (field, value) in TYPICAL_VALUES {
            let Some(cell) = layout.cell(field) else {
                assert_eq!(field, Field::Project, "{field:?} starts on a new sheet");
                continue;
            };
            assert_eq!(cell.letter_value(value), value, "{field:?} cuts {value:?}");
            assert!(!cell.elides(value));
        }
        for cell in &layout.cells {
            assert_eq!(cell.letter_caption(), cell.id.display_label());
        }
    }
}

#[test]
fn the_compact_block_letters_its_fixed_form_values_whole() {
    let compact = SchematicSheetFormat::default()
        .try_update(|draft| draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact)
        .unwrap();
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
            let cell = layout.cell(*field).expect("a new sheet shows the field");
            assert_eq!(cell.letter_value(value), *value, "{field:?} cuts {value:?}");
            assert_eq!(cell.letter_caption(), field.display_label());
        }
    }
}

#[test]
fn every_row_spans_the_field_grid_exactly() {
    let base = SchematicSheetFormat::default();
    let every = DrawingSheetTitleFieldId::ALL;
    for template in [
        DrawingSheetTitleBlockTemplate::Compact,
        DrawingSheetTitleBlockTemplate::Standard,
        DrawingSheetTitleBlockTemplate::OrganizationManaged,
    ] {
        for rotation in [
            DrawingSheetTitleBlockRotation::Upright,
            DrawingSheetTitleBlockRotation::Clockwise90,
        ] {
            let placed = base
                .try_update(|draft| {
                    draft.title_block.template = template;
                    draft.title_block.rotation = rotation;
                })
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
                    layout.cells.iter().map(|cell| cell.id).collect::<Vec<_>>(),
                    resolved,
                    "{template:?} lays out the fields it resolves, in order"
                );
                let rows = rows_of(&layout);
                let filled = rows.iter().take_while(|row| !row.is_empty()).count();
                assert_eq!(filled, layout.rows.min(resolved.len()), "{template:?}");
                assert!(rows[filled..].iter().all(Vec::is_empty));
                for row in 0..filled {
                    let cells = layout
                        .cells
                        .iter()
                        .filter(|cell| cell.row == row)
                        .collect::<Vec<_>>();
                    let mut x_um = layout.grid_x_um;
                    for cell in cells {
                        assert_eq!(cell.x_um, x_um, "{template:?} row {row} has a gap");
                        assert!(cell.width_um > 0);
                        x_um += cell.width_um;
                    }
                    assert_eq!(
                        x_um, layout.width_um,
                        "{template:?} row {row} misses the edge"
                    );
                }
                assert_eq!(layout.row_top_um(0), 0);
                assert_eq!(layout.row_top_um(layout.rows), layout.height_um);
            }
        }
    }
}

#[test]
fn a_cell_is_sized_by_what_it_holds() {
    // The cells that hold a short fixed form are narrower than the cells that
    // hold free text, and while every row fits, no cell is narrower than the
    // width its field needs.
    let layout = layout_of(&with_project(&SchematicSheetFormat::default()));
    let width = |field| layout.cell(field).unwrap().width_um;
    assert!(width(Field::Page) < width(Field::CellView));
    assert!(width(Field::Revision) < width(Field::SheetTitle));
    for cell in &layout.cells {
        assert!(
            cell.width_um >= cell.id.title_cell_need_um(),
            "{:?} is {} um wide and needs {}",
            cell.id,
            cell.width_um,
            cell.id.title_cell_need_um()
        );
    }
    // A caption is ASCII, so its byte length is its character count.
    for field in DrawingSheetTitleFieldId::ALL {
        assert!(field.display_label().is_ascii());
    }
}

#[test]
fn rows_are_cut_so_the_fullest_overruns_least_and_the_rest_share_alike() {
    let shown = [
        Field::Project,
        Field::DocumentType,
        Field::DrawnBy,
        Field::ApprovedBy,
        Field::Date,
        Field::DocumentStatus,
        Field::Organization,
        Field::DocumentId,
    ];
    let standard = showing(&SchematicSheetFormat::default(), &shown);
    assert_eq!(
        rows_of(&layout_of(&standard)),
        [
            vec![Field::Project, Field::CellView, Field::SheetTitle],
            vec![Field::DocumentType, Field::Page, Field::Revision],
            vec![Field::DrawnBy, Field::ApprovedBy, Field::Date],
            vec![
                Field::DocumentStatus,
                Field::Organization,
                Field::DocumentId
            ],
        ]
    );
    // The compact block's needs fill it: every row fits only cut this way.
    let compact = standard
        .try_update(|draft| draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact)
        .unwrap();
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
    // a row of its own.
    let sparse = showing(
        &SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            })
            .unwrap(),
        &[],
    );
    let layout = layout_of(&sparse);
    assert_eq!(layout.rows, 5);
    assert_eq!(
        rows_of(&layout),
        [
            vec![Field::CellView],
            vec![Field::SheetTitle],
            vec![Field::Page],
            vec![Field::Revision],
            vec![],
        ]
    );
}

#[test]
fn a_value_longer_than_its_cell_is_cut_with_an_ellipsis() {
    let advance_um = drawing_sheet_mono_advance_um(DRAWING_SHEET_TITLE_BLOCK_LETTERING.data_um);
    let cell = |chars: u64| DrawingSheetTitleCell {
        id: Field::CellView,
        row: 0,
        x_um: 0,
        width_um: DRAWING_SHEET_TITLE_CELL_PADDING_UM * 2 + advance_um * chars,
    };
    assert_eq!(cell(5).letter_value("ABCDEFG"), "ABCD…");
    assert!(cell(5).elides("ABCDEFG"));
    assert_eq!(cell(5).letter_value("ABCDE"), "ABCDE");
    assert!(!cell(5).elides("ABCDE"));
    assert_eq!(cell(4).letter_value("ééééé"), "ééé…");
    assert_eq!(cell(4).letter_value(""), "");
    // Lettering never shrinks to fit: a cell too narrow for one character
    // still letters the ellipsis rather than smaller text.
    assert_eq!(cell(0).letter_value("Anything"), "…");
}

//! The project's logo: kept in the project file as it was imported, and
//! seated by every form in the cell that names the legal owner, with the
//! owner's lettering clear of it.

use super::*;

use DrawingSheetTitleFieldId as Field;

/// The head of a PNG file of a picture `width` by `height` pixels: the
/// signature and the header chunk, all the form reads of a picture.
fn png_head(width: u32, height: u32) -> Vec<u8> {
    let mut png = vec![0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1a, b'\n'];
    png.extend_from_slice(&13_u32.to_be_bytes());
    png.extend_from_slice(b"IHDR");
    png.extend_from_slice(&width.to_be_bytes());
    png.extend_from_slice(&height.to_be_bytes());
    // Eight bits of red, green and blue; no interlace; and a checksum the
    // form never reads.
    png.extend_from_slice(&[8, 2, 0, 0, 0, 0, 0, 0, 0]);
    png
}

fn logo(width: u32, height: u32) -> DrawingSheetOwnerLogo {
    DrawingSheetOwnerLogo::from_png(png_head(width, height), "Acme Instruments logo").unwrap()
}

fn on(
    standard: DrawingSheetStandard,
    template: DrawingSheetTitleBlockTemplate,
) -> SchematicSheetFormat {
    SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
        .try_update(|draft| draft.title_block.template = template)
        .unwrap()
}

const TEMPLATES: [(DrawingSheetStandard, DrawingSheetTitleBlockTemplate); 3] = [
    (
        DrawingSheetStandard::IsoA4,
        DrawingSheetTitleBlockTemplate::Compact,
    ),
    (
        DrawingSheetStandard::IsoA4,
        DrawingSheetTitleBlockTemplate::Standard,
    ),
    (
        DrawingSheetStandard::AnsiB,
        DrawingSheetTitleBlockTemplate::AsmeY141,
    ),
];

#[test]
fn a_project_keeps_its_logo_in_its_file_as_it_was_imported() {
    let logo = logo(300, 100);
    assert_eq!(logo.pixel_size(), (300, 100));
    assert_eq!(
        logo.digest(),
        ContentDigest::from_bytes(Sha256::digest(png_head(300, 100)).into())
    );
    let settings = DrawingSheetProjectSettings {
        owner_logo: Some(logo.clone()),
        ..DrawingSheetProjectSettings::default()
    };
    let text = serde_json::to_string(&settings).unwrap();
    let loaded: DrawingSheetProjectSettings = serde_json::from_str(&text).unwrap();
    assert_eq!(loaded.owner_logo.as_ref(), Some(&logo));
    assert_eq!(loaded.owner_logo.unwrap().png(), png_head(300, 100));

    // A project with no logo writes none, and one saved before logos
    // existed loads without one.
    let plain = serde_json::to_string(&DrawingSheetProjectSettings::default()).unwrap();
    assert!(!plain.contains("owner_logo"), "{plain}");
    let loaded: DrawingSheetProjectSettings = serde_json::from_str(&plain).unwrap();
    assert_eq!(loaded.owner_logo, None);

    // A file whose picture is not the one its digest names is refused.
    let tampered = text.replace(&logo.digest().to_string(), &"0".repeat(64));
    assert!(serde_json::from_str::<DrawingSheetProjectSettings>(&tampered).is_err());
}

#[test]
fn only_a_png_picture_is_taken_as_a_logo() {
    assert!(matches!(
        DrawingSheetOwnerLogo::from_png(b"GIF89a not a png".to_vec(), "Logo"),
        Err(DesignManagementError::InvalidOwnerLogo(_))
    ));
    assert!(DrawingSheetOwnerLogo::from_png(png_head(0, 10), "Logo").is_err());
    assert!(DrawingSheetOwnerLogo::from_png(png_head(10, 10), "").is_err());
}

#[test]
fn every_form_seats_the_logo_in_its_owner_cell_clear_of_the_owner_lettering() {
    for (standard, template) in TEMPLATES {
        let format = on(standard, template);
        let geometry = format.geometry().unwrap();
        let plain = drawing_sheet_title_layout(&format, &geometry).unwrap();
        let (layout, seat) =
            drawing_sheet_title_layout_with_owner_logo(&format, &geometry, Some(&logo(300, 100)))
                .unwrap();
        let seat = seat.unwrap_or_else(|| panic!("{template:?} seats no logo"));
        let cell = layout.cell(Field::Organization).unwrap();
        assert!(
            seat.x_um > cell.x_um
                && seat.y_um > cell.y_um
                && seat.x_um + seat.width_um < cell.x_um + cell.width_um
                && seat.y_um + seat.height_um < cell.y_um + cell.height_um,
            "{template:?}: {seat:?} is not inside {cell:?}"
        );
        // The logo keeps its proportions.
        assert!(
            (seat.width_um as i64 - 3 * seat.height_um as i64).abs() <= 3,
            "{template:?}: {seat:?}"
        );
        // Every line the owner's cell letters stands clear of the logo:
        // to its right, or above it.
        let fields = resolve_drawing_sheet_title_fields(
            &format,
            &BTreeMap::from([(Field::Organization, "Acme Instruments".to_owned())]),
        );
        let index = layout
            .cells
            .iter()
            .position(|candidate| candidate == cell)
            .unwrap();
        let lines = layout
            .lettering(&fields)
            .into_iter()
            .filter(|line| line.cell == index)
            .collect::<Vec<_>>();
        assert!(
            lines.iter().any(|line| line.text == "Acme Instruments"),
            "{template:?}: {lines:?}"
        );
        for line in &lines {
            let beside = line.x_um >= seat.x_um + seat.width_um;
            let above = line.seat_um + line.lettering_um <= seat.y_um;
            assert!(beside || above, "{template:?}: {line:?} overlaps {seat:?}");
        }
        // No other cell moves.
        for (moved, before) in layout.cells.iter().zip(&plain.cells) {
            if moved.value(Field::Organization).is_none() {
                assert_eq!(moved, before, "{template:?}");
            }
        }
        assert_eq!(layout.rules, plain.rules, "{template:?}");
    }
}

#[test]
fn a_one_line_owner_cell_gives_a_wide_logo_no_more_than_two_fifths_of_it() {
    let format = on(
        DrawingSheetStandard::IsoA4,
        DrawingSheetTitleBlockTemplate::Compact,
    );
    let geometry = format.geometry().unwrap();
    let (layout, seat) =
        drawing_sheet_title_layout_with_owner_logo(&format, &geometry, Some(&logo(4000, 100)))
            .unwrap();
    let seat = seat.unwrap();
    let cell = layout.cell(Field::Organization).unwrap();
    assert!(
        seat.width_um * 5 <= cell.width_um * 2,
        "{seat:?} in {cell:?}"
    );
    let owner = layout.value(Field::Organization).unwrap();
    assert!(owner.x_um > seat.x_um + seat.width_um);
    assert!(owner.capacity() >= 20, "{owner:?}");
}

#[test]
fn a_sheet_without_a_logo_or_a_block_seats_none() {
    let format = on(
        DrawingSheetStandard::IsoA4,
        DrawingSheetTitleBlockTemplate::Compact,
    );
    let geometry = format.geometry().unwrap();
    let (layout, seat) =
        drawing_sheet_title_layout_with_owner_logo(&format, &geometry, None).unwrap();
    assert_eq!(seat, None);
    assert_eq!(
        layout,
        drawing_sheet_title_layout(&format, &geometry).unwrap()
    );
    let bare = on(
        DrawingSheetStandard::IsoA4,
        DrawingSheetTitleBlockTemplate::None,
    );
    assert!(
        drawing_sheet_title_layout_with_owner_logo(
            &bare,
            &bare.geometry().unwrap(),
            Some(&logo(10, 10))
        )
        .is_none()
    );
}

//! A printed page draws the project's logo in its title block's owner cell,
//! as the PNG file it was imported as, clear of the owner's name, in every
//! output: the PNG and TIFF raster, the dialog's preview, SVG and PDF. A
//! project without a logo prints none.

use super::title_lettering::{title_block_scene, title_block_setup};
use super::*;
use crate::sources::{SchematicHardcopySource, resolve_schematic_source};
use base64::Engine as _;
use rspice_design::schematic::{document::SchematicDocument, owned::Schematic};
use rspice_design_model::design_management::{
    DrawingSheetInheritance, DrawingSheetOwnerLogo, DrawingSheetProjectSettings,
    drawing_sheet_title_layout_with_owner_logo,
};

/// The logo's pixel size: three times as wide as it is high.
const LOGO_PIXELS: (u32, u32) = (6, 2);

/// A PNG file of a picture three times as wide as it is high, in a magenta
/// nothing else on a printed page is drawn in.
fn wide_logo_png() -> Vec<u8> {
    let (width, height) = LOGO_PIXELS;
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer
        .write_image_data(&[200, 0, 200, 255].repeat((width * height) as usize))
        .unwrap();
    writer.finish().unwrap();
    bytes
}

/// A blank schematic on `format` in a project named Acme Instruments whose
/// logo is `logo`, resolved for print.
fn resolved(
    format: &SchematicSheetFormat,
    logo: Option<&DrawingSheetOwnerLogo>,
) -> ResolvedHardcopyDocument {
    let schematic = Schematic::from_document(SchematicDocument::default());
    let project_default = format
        .try_update(|draft| draft.inheritance = DrawingSheetInheritance::ProjectDefault)
        .unwrap();
    let mut values = DrawingSheetProjectSettings::default().title_block_field_values;
    values.insert(
        DrawingSheetTitleFieldId::Organization,
        "Acme Instruments".to_owned(),
    );
    resolve_schematic_source(SchematicHardcopySource {
        identity: HardcopySourceIdentity::try_new(
            "logo-schematic",
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(0x4c4f_474f)).unwrap(),
            ObjectRevision::INITIAL,
            "Logo schematic",
        )
        .unwrap(),
        schematic: &schematic,
        selection: None,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: Some(&project_default),
        project_title_block_field_values: Some(&values),
        project_owner_logo: logo,
        scope: HardcopyScope::ActiveDocument,
        parameter_labels: Default::default(),
    })
    .unwrap()
}

/// The forms whose owner cell seats a logo: the compact block's one-line
/// Owner row, ISO 7200's legal owner with room beneath the name, and ASME
/// Y14.1's block A.
fn forms() -> [SchematicSheetFormat; 3] {
    [
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
    ]
    .map(|(standard, template)| {
        SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
            .try_update(|draft| draft.title_block.template = template)
            .unwrap()
    })
}

/// A box as left, top, right and bottom.
type Box4 = [f64; 4];

/// `format`'s owner cell and the logo's seat in it, in the scene's
/// micrometres, which the plot draws from the paper's top-left corner.
fn owner_cell_and_seat(
    format: &SchematicSheetFormat,
    logo: &DrawingSheetOwnerLogo,
) -> (Box4, Box4) {
    let geometry = format.geometry().unwrap();
    let block = geometry.title_block.unwrap();
    let (layout, seat) =
        drawing_sheet_title_layout_with_owner_logo(format, &geometry, Some(logo)).unwrap();
    let seat = seat.unwrap();
    let cell = layout.cell(DrawingSheetTitleFieldId::Organization).unwrap();
    let (x, y) = (block.x_um as f64, block.y_um as f64);
    let cell = [
        x + cell.x_um as f64,
        y + cell.y_um as f64,
        x + (cell.x_um + cell.width_um) as f64,
        y + (cell.y_um + cell.height_um) as f64,
    ];
    let seat = [
        x + seat.x_um as f64,
        y + seat.y_um as f64,
        x + (seat.x_um + seat.width_um) as f64,
        y + (seat.y_um + seat.height_um) as f64,
    ];
    (cell, seat)
}

/// `scene`, a box in the scene's micrometres, where `page` prints it: in
/// micrometres from the paper's top-left corner.
fn on_page(page: &PreviewPage, scene: Box4) -> Box4 {
    let content = page.geometry().content_rect();
    let window = page.scaled_content_window();
    let scale = page.scale().numerator() as f64 / page.scale().denominator() as f64;
    let x =
        |value: f64| content.x.micrometres() as f64 + value * scale - window.x.micrometres() as f64;
    let y =
        |value: f64| content.y.micrometres() as f64 + value * scale - window.y.micrometres() as f64;
    [x(scene[0]), y(scene[1]), x(scene[2]), y(scene[3])]
}

/// Whether `inner` lies inside `outer`, give or take `slack`.
fn within(inner: Box4, outer: Box4, slack: f64) -> bool {
    inner[0] >= outer[0] - slack
        && inner[1] >= outer[1] - slack
        && inner[2] <= outer[2] + slack
        && inner[3] <= outer[3] + slack
}

/// A plan printing `source`'s sheet on one page as `format`.
fn plan_for(source: &ResolvedHardcopyDocument, format: OutputFormat) -> HardcopyPlan {
    let setup = setup_with_schematic_output(format, title_block_setup());
    let extent = source.content_extent_for_setup(setup.schematic()).unwrap();
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(0x4c4f_474f)).unwrap(),
        source.authority().clone(),
        setup,
        extent,
    )
    .unwrap()
}

/// What a raster page draws in the logo's magenta: how many of `seat`'s
/// pixels it covers, and the box round every such pixel on the page, in
/// micrometres from the paper's top-left corner.
fn magenta_ink(rgba: &[u8], width: u32, page: &PreviewPage, seat: Box4) -> (f64, Option<Box4>) {
    let (paper_width, _) = page.geometry().physical_size();
    let pixel_um = paper_width.micrometres() as f64 / f64::from(width);
    let mut inked = None::<Box4>;
    let mut covered = 0_u64;
    for (index, pixel) in rgba.chunks_exact(4).enumerate() {
        let [red, green, blue, _] = [pixel[0], pixel[1], pixel[2], pixel[3]].map(i32::from);
        // Magenta stays clear of grey and black even through the preview's
        // soft proof, which takes a quarter of every colour's chroma.
        if red - green < 96 || blue - green < 96 {
            continue;
        }
        let (column, row) = (index as u32 % width, index as u32 / width);
        let (left, top) = (f64::from(column) * pixel_um, f64::from(row) * pixel_um);
        let (right, bottom) = (left + pixel_um, top + pixel_um);
        inked = Some(match inked {
            None => [left, top, right, bottom],
            Some(seen) => [
                seen[0].min(left),
                seen[1].min(top),
                seen[2].max(right),
                seen[3].max(bottom),
            ],
        });
        if within([left, top, right, bottom], seat, 0.0) {
            covered += 1;
        }
    }
    let seat_pixels = (seat[2] - seat[0]) * (seat[3] - seat[1]) / (pixel_um * pixel_um);
    (covered as f64 / seat_pixels, inked)
}

/// The box an SVG page's one `<image>` stands in, and the PNG file it shows.
fn svg_image(svg: &str) -> (Box4, Vec<u8>) {
    let document = usvg::roxmltree::Document::parse(svg).unwrap();
    let images = document
        .descendants()
        .filter(|node| node.has_tag_name("image"))
        .collect::<Vec<_>>();
    let [image] = images[..] else {
        panic!("the SVG page holds {} images", images.len());
    };
    let number = |name: &str| image.attribute(name).unwrap().parse::<f64>().unwrap();
    let (x, y) = (number("x"), number("y"));
    let png = base64::engine::general_purpose::STANDARD
        .decode(
            image
                .attribute("href")
                .unwrap()
                .strip_prefix("data:image/png;base64,")
                .unwrap(),
        )
        .unwrap();
    ([x, y, x + number("width"), y + number("height")], png)
}

/// The boxes a PDF page draws its images in, in micrometres from the
/// paper's top-left corner, with each image's pixel size.
fn pdf_images(bytes: &[u8], paper_height_um: f64) -> Vec<(Box4, (i64, i64))> {
    const POINT_UM: f64 = 25_400.0 / 72.0;
    let pdf = ParsedPdf::load_mem(bytes).unwrap();
    let page = pdf.get_pages()[&1];
    let resources = pdf
        .get_dict_in_dict(pdf.get_dictionary(page).unwrap(), b"Resources")
        .unwrap();
    let objects = pdf.get_dict_in_dict(resources, b"XObject").ok();
    let float = |object: &lopdf::Object| f64::from(object.as_float().unwrap());
    let mut ctm = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    let mut saved = Vec::new();
    let mut images = Vec::new();
    for operation in pdf.get_and_decode_page_content(page).unwrap().operations {
        let operands = &operation.operands;
        match operation.operator.as_str() {
            "q" => saved.push(ctm),
            "Q" => ctm = saved.pop().unwrap(),
            "cm" => {
                let m: Vec<f64> = operands.iter().map(float).collect();
                ctm = [
                    m[0] * ctm[0] + m[1] * ctm[2],
                    m[0] * ctm[1] + m[1] * ctm[3],
                    m[2] * ctm[0] + m[3] * ctm[2],
                    m[2] * ctm[1] + m[3] * ctm[3],
                    m[4] * ctm[0] + m[5] * ctm[2] + ctm[4],
                    m[4] * ctm[1] + m[5] * ctm[3] + ctm[5],
                ];
            }
            "Do" => {
                let name = operands[0].as_name().unwrap();
                let object = objects.and_then(|objects| objects.get(name).ok()).unwrap();
                let stream = pdf.dereference(object).unwrap().1.as_stream().unwrap();
                if stream.dict.get(b"Subtype").unwrap().as_name().unwrap() != b"Image" {
                    continue;
                }
                // An image fills the unit square of the space it is drawn in;
                // page space is points from the paper's bottom-left corner.
                let corners = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)].map(|(u, v)| {
                    let x = ctm[0] * u + ctm[2] * v + ctm[4];
                    let y = ctm[1] * u + ctm[3] * v + ctm[5];
                    (x * POINT_UM, paper_height_um - y * POINT_UM)
                });
                let xs = corners.map(|(x, _)| x);
                let ys = corners.map(|(_, y)| y);
                let least = |values: [f64; 4]| values.into_iter().fold(f64::INFINITY, f64::min);
                let most = |values: [f64; 4]| values.into_iter().fold(f64::NEG_INFINITY, f64::max);
                let dimension = |key: &[u8]| stream.dict.get(key).unwrap().as_i64().unwrap();
                images.push((
                    [least(xs), least(ys), most(xs), most(ys)],
                    (dimension(b"Width"), dimension(b"Height")),
                ));
            }
            _ => {}
        }
    }
    images
}

#[test]
fn every_output_draws_the_owner_logo_inside_its_owner_cell() {
    let logo = DrawingSheetOwnerLogo::from_png(wide_logo_png(), "Acme Instruments logo").unwrap();
    let (logo_width, logo_height) = LOGO_PIXELS;
    for format in forms() {
        let template = format.title_block.template;
        let source = resolved(&format, Some(&logo));
        let (cell, seat) = owner_cell_and_seat(&format, &logo);

        // The PNG file, the TIFF file and the printer's pages are drawn by
        // one rasterizer; the dialog's preview is drawn by it too, through
        // a soft proof.
        let plan = plan_for(&source, OutputFormat::Png { dpi: 150 });
        let page = &plan.pagination().pages()[0];
        let (cell_on_page, seat_on_page) = (on_page(page, cell), on_page(page, seat));
        let png = HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
        let mut reader = png::Decoder::new(std::io::Cursor::new(
            png.single_part().unwrap().bytes().to_vec(),
        ))
        .read_info()
        .unwrap();
        let mut rgba = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut rgba).unwrap();
        let preview = HardcopyRenderer::render_preview_page_resolved(
            &plan,
            &source,
            resolved_metadata(),
            0,
            150,
        )
        .unwrap();
        for (output, rgba, width) in [
            ("PNG", &rgba[..info.buffer_size()], info.width),
            ("preview", preview.rgba(), preview.width()),
        ] {
            let (coverage, inked) = magenta_ink(rgba, width, page, seat_on_page);
            let inked = inked.unwrap_or_else(|| panic!("{template:?}: the {output} draws no logo"));
            assert!(
                coverage > 0.9,
                "{template:?}: the {output}'s logo covers {coverage:.2} of its seat"
            );
            assert!(
                within(inked, cell_on_page, 0.0),
                "{template:?}: the {output}'s logo at {inked:?} leaves the owner cell {cell_on_page:?}"
            );
        }
        let bare = resolved(&format, None);
        let bare_png = HardcopyRenderer::render_resolved(
            &plan_for(&bare, OutputFormat::Png { dpi: 150 }),
            &bare,
            resolved_metadata(),
        )
        .unwrap();
        let mut reader = png::Decoder::new(std::io::Cursor::new(
            bare_png.single_part().unwrap().bytes().to_vec(),
        ))
        .read_info()
        .unwrap();
        let mut bare_rgba = vec![0; reader.output_buffer_size().unwrap()];
        let bare_info = reader.next_frame(&mut bare_rgba).unwrap();
        assert_eq!(
            magenta_ink(
                &bare_rgba[..bare_info.buffer_size()],
                bare_info.width,
                page,
                seat_on_page
            )
            .1,
            None,
            "{template:?}: a project without a logo prints none"
        );

        let plan = plan_for(&source, OutputFormat::SvgVector);
        let page = &plan.pagination().pages()[0];
        let svg = HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
        let (image, png) =
            svg_image(std::str::from_utf8(svg.single_part().unwrap().bytes()).unwrap());
        assert_eq!(
            png,
            logo.png(),
            "{template:?}: the SVG shows another picture"
        );
        assert!(
            within(image, on_page(page, seat), 1.0),
            "{template:?}: the SVG's logo at {image:?} is not in its seat"
        );
        assert!(within(image, on_page(page, cell), 0.0), "{template:?}");

        let plan = plan_for(&source, OutputFormat::PdfVector);
        let page = &plan.pagination().pages()[0];
        let (_, paper_height) = page.geometry().physical_size();
        let pdf = HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
        let images = pdf_images(
            pdf.single_part().unwrap().bytes(),
            paper_height.micrometres() as f64,
        );
        let [(image, pixels)] = images[..] else {
            panic!("{template:?}: the PDF page draws {} images", images.len());
        };
        assert_eq!(
            pixels,
            (i64::from(logo_width), i64::from(logo_height)),
            "{template:?}"
        );
        assert!(
            within(image, on_page(page, seat), 1.0),
            "{template:?}: the PDF's logo at {image:?} is not in its seat {:?}",
            on_page(page, seat)
        );
        assert!(within(image, on_page(page, cell), 0.0), "{template:?}");
    }
}

#[test]
fn the_owners_name_prints_clear_of_the_logo() {
    let logo = DrawingSheetOwnerLogo::from_png(wide_logo_png(), "Acme Instruments logo").unwrap();
    for format in forms() {
        let template = format.title_block.template;
        assert!(
            !title_block_scene(&resolved(&format, None))
                .primitives()
                .iter()
                .any(|primitive| matches!(primitive, ScenePrimitive::RasterImage { .. })),
            "{template:?}"
        );
        let scene = title_block_scene(&resolved(&format, Some(&logo)));
        let rasters = scene
            .primitives()
            .iter()
            .filter_map(|primitive| match primitive {
                ScenePrimitive::RasterImage {
                    rect,
                    alternative_text,
                    ..
                } => Some((*rect, alternative_text.clone())),
                _ => None,
            })
            .collect::<Vec<_>>();
        let [(rect, alternative_text)] = &rasters[..] else {
            panic!("{template:?} prints {} logos", rasters.len());
        };
        assert_eq!(alternative_text, "Acme Instruments logo", "{template:?}");
        let (x, y) = (rect.x.micrometres(), rect.y.micrometres());
        let width = rect.width.micrometres();
        // The owner's name is printed clear of it: to its right, or above.
        let name = scene
            .primitives()
            .iter()
            .find_map(|primitive| match primitive {
                ScenePrimitive::Text { origin, text, .. } if text == "Acme Instruments" => {
                    Some(*origin)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("{template:?} prints no owner"));
        assert!(
            name.x.micrometres() >= x + width || name.y.micrometres() < y,
            "{template:?}: the name at {name:?} overlaps the logo {rect:?}"
        );
    }
}

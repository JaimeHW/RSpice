//! A glyph its face lacks prints from the next face of its chain, the face
//! the canvas borrows it from, and a glyph no face carries prints as the
//! canvas's replacement: text never refuses a print.

use super::*;

fn text(font: SceneFont, value: &str, y_um: u64) -> ScenePrimitive {
    ScenePrimitive::Text {
        origin: ScenePoint::new(
            Length::from_micrometres(5_000),
            Length::from_micrometres(y_um),
        ),
        text: value.to_owned(),
        font,
        size: Length::from_micrometres(12_000),
        color: SemanticColor::Foreground,
        anchor: TextAnchor::Start,
        rotation: SceneTextRotation::Upright,
    }
}

fn text_scene(content: ContentExtent, primitives: Vec<ScenePrimitive>) -> HardcopyScene {
    // The title is uncovered on purpose: it once refused the whole document.
    let metadata = HardcopySceneMetadata::try_new("10 kΩ 中", "RSpice hardcopy tests").unwrap();
    HardcopyScene::try_new(content, metadata, primitives, Vec::new()).unwrap()
}

/// The page's RGBA pixels at `dpi`.
fn raster(scene: &HardcopyScene, content: ContentExtent) -> Vec<u8> {
    let plan = plan(OutputFormat::Png { dpi: 150 }, content, false);
    let png = HardcopyRenderer::render(&plan, scene).unwrap();
    let decoder = png::Decoder::new(Cursor::new(png.single_part().unwrap().bytes()));
    let mut reader = decoder.read_info().unwrap();
    let mut rgba = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut rgba).unwrap();
    rgba.truncate(info.buffer_size());
    rgba
}

#[test]
fn each_font_letters_engineering_symbols_through_its_chain() {
    let faces = PrintFaces::get().unwrap();
    let fonts = [
        SceneFont::Sans,
        SceneFont::SansSemibold,
        SceneFont::Monospace,
        SceneFont::MonospaceMedium,
    ];
    for font in fonts {
        for symbol in "Ωµμ Δ≥≤±°×·√".chars().filter(|c| !c.is_whitespace()) {
            let (face, lettered) = faces.resolve(font, symbol);
            assert_eq!(lettered, symbol, "{font:?} letters {symbol}");
            assert!(font.fallback_chain().contains(&face), "{font:?} {symbol}");
        }
    }
    // Plex Mono carries no Greek, so mono borrows it from Plex Sans, as the
    // canvas's mono family does.
    for font in [SceneFont::Monospace, SceneFont::MonospaceMedium] {
        assert_eq!(faces.resolve(font, 'Ω'), (PrintFace::SansRegular, 'Ω'));
        assert_eq!(faces.resolve(font, 'R'), (font.fallback_chain()[0], 'R'));
    }
    // Box drawing is Plex Mono's alone, and no face carries an angle sign or
    // CJK: each prints the replacement from the chain's own face.
    assert_eq!(
        faces.resolve(SceneFont::SansSemibold, '─'),
        (PrintFace::SansSemibold, REPLACEMENT_CHARACTER)
    );
    for font in fonts {
        for uncovered in ['∠', '中', '\u{10ffff}'] {
            assert_eq!(
                faces.resolve(font, uncovered),
                (font.fallback_chain()[0], REPLACEMENT_CHARACTER),
                "{font:?} {uncovered}"
            );
        }
    }
}

/// Every renderer is told which face letters each run, so none of them falls
/// back on its own, to whichever face it happens to find first.
#[test]
fn svg_names_the_face_of_every_borrowed_run() {
    let content = extent(150_000, 80_000);
    let scene = text_scene(
        content,
        vec![
            text(SceneFont::Monospace, "R1 10 kΩ α", 20_000),
            text(SceneFont::MonospaceMedium, "DWG Δ", 40_000),
            text(SceneFont::Sans, "Plain 中", 60_000),
        ],
    );
    let plan = plan(OutputFormat::SvgVector, content, false);
    let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
    let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
    let sans = "<tspan font-family=\"RSpice Plex Sans\" font-weight=\"400\">";
    assert!(
        svg.contains(&format!(">R1 10 k{sans}Ω</tspan> {sans}α</tspan></text>")),
        "{svg}"
    );
    assert!(
        svg.contains(&format!(">DWG {sans}Δ</tspan></text>")),
        "{svg}"
    );
    assert!(svg.contains(">Plain ?</text>"), "{svg}");
}

#[test]
fn pdf_letters_a_borrowed_glyph_in_the_face_that_carries_it() {
    let content = extent(150_000, 80_000);
    let scene = text_scene(content, vec![text(SceneFont::Monospace, "10 kΩ", 20_000)]);
    let plan = plan(OutputFormat::PdfVector, content, false);
    let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
    let pdf = ParsedPdf::load_mem(publication.single_part().unwrap().bytes()).unwrap();
    let page = pdf.get_pages()[&1];
    let base_fonts = pdf
        .get_page_fonts(page)
        .unwrap()
        .values()
        .map(|font| {
            String::from_utf8_lossy(font.get(b"BaseFont").unwrap().as_name().unwrap()).into_owned()
        })
        .collect::<Vec<_>>();
    assert!(
        base_fonts.iter().any(|name| name.contains("IBMPlexMono")),
        "{base_fonts:?}"
    );
    assert!(
        base_fonts.iter().any(|name| name.contains("IBMPlexSans")),
        "{base_fonts:?}"
    );
}

/// A borrowed glyph is the lending face's own: the mono Ω prints pixel for
/// pixel as the sans Ω does, and it is not the replacement.
#[test]
fn a_borrowed_glyph_prints_as_its_lending_face_letters_it() {
    let content = extent(60_000, 40_000);
    let page = |font, value: &str| {
        raster(
            &text_scene(content, vec![text(font, value, 25_000)]),
            content,
        )
    };
    let mono = page(SceneFont::Monospace, "Ω");
    assert_eq!(mono, page(SceneFont::Sans, "Ω"));
    assert_ne!(mono, page(SceneFont::Monospace, "?"));
    assert_ne!(
        mono,
        raster(&text_scene(content, Vec::new()), content),
        "the Ω inked nothing"
    );
    // What no face carries prints as the replacement, never refused.
    assert_eq!(page(SceneFont::Sans, "中"), page(SceneFont::Sans, "?"));
}

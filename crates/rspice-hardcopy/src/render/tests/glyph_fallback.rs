//! A glyph its face lacks prints from the next face of its role's chain in
//! `rspice-fonts`, the face the canvas borrows it from, and a glyph no face
//! carries prints as the replacement square the canvas paints for it: text
//! never refuses a print.

use rspice_fonts::{REPLACEMENT, Role};

use super::*;

const FONTS: [SceneFont; 4] = [
    SceneFont::Sans,
    SceneFont::SansSemibold,
    SceneFont::Monospace,
    SceneFont::MonospaceMedium,
];

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

/// Each scene font prints through the role the canvas letters the same text
/// in, so screen and print fall back through one chain.
#[test]
fn each_scene_font_prints_through_its_screen_role() {
    assert_eq!(
        FONTS.map(SceneFont::role),
        [Role::Sans, Role::SansSemibold, Role::Mono, Role::MonoMedium]
    );
    for font in FONTS {
        assert_eq!(font.role().chain().last(), Some(&Face::Symbols), "{font:?}");
    }
}

#[test]
fn each_font_letters_engineering_symbols_through_its_chain() {
    for font in FONTS {
        for symbol in "Ωµμ Δ≥≤±°×·√".chars().filter(|c| !c.is_whitespace()) {
            let (face, lettered) = font.role().resolve(symbol);
            assert_eq!(lettered, symbol, "{font:?} letters {symbol}");
            assert!(font.role().chain().contains(&face), "{font:?} {symbol}");
        }
        // What no Plex face carries prints from RSpice Symbols.
        for symbol in "⌀∅⏚⌭⌖Ⓜ∠".chars() {
            assert_eq!(
                font.role().resolve(symbol),
                (Face::Symbols, symbol),
                "{font:?} {symbol}"
            );
        }
    }
    // Plex Mono carries no Greek, so mono borrows it from Plex Sans at its own
    // weight, as the canvas's mono families do.
    assert_eq!(
        SceneFont::Monospace.role().resolve('Ω'),
        (Face::SansRegular, 'Ω')
    );
    assert_eq!(
        SceneFont::MonospaceMedium.role().resolve('Ω'),
        (Face::SansMedium, 'Ω')
    );
    for font in [SceneFont::Monospace, SceneFont::MonospaceMedium] {
        assert_eq!(font.role().resolve('R'), (font.role().primary(), 'R'));
    }
    // Box drawing is Plex Mono's alone, and no face carries CJK: each prints
    // the replacement square from RSpice Symbols.
    assert_eq!(
        SceneFont::SansSemibold.role().resolve('─'),
        (Face::Symbols, REPLACEMENT)
    );
    for font in FONTS {
        for uncovered in ['中', '\u{10ffff}'] {
            assert_eq!(
                font.role().resolve(uncovered),
                (Face::Symbols, REPLACEMENT),
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
            text(SceneFont::Monospace, "⌀6 ⏚", 70_000),
        ],
    );
    let plan = plan(OutputFormat::SvgVector, content, false);
    let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
    let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
    let sans = "<tspan font-family=\"RSpice Plex Sans\" font-weight=\"400\">";
    let sans_medium = "<tspan font-family=\"RSpice Plex Sans\" font-weight=\"500\">";
    let symbols = "<tspan font-family=\"RSpice Symbols\" font-weight=\"400\">";
    assert!(
        svg.contains(&format!(">R1 10 k{sans}Ω</tspan> {sans}α</tspan></text>")),
        "{svg}"
    );
    assert!(
        svg.contains(&format!(">DWG {sans_medium}Δ</tspan></text>")),
        "{svg}"
    );
    assert!(
        svg.contains(&format!(">Plain {symbols}\u{25fb}</tspan></text>")),
        "{svg}"
    );
    assert!(
        svg.contains(&format!(">{symbols}⌀</tspan>6 {symbols}⏚</tspan></text>")),
        "{svg}"
    );
    // The face the symbols are named in travels with the drawing.
    assert!(
        svg.contains("@font-face{font-family:'RSpice Symbols';font-style:normal;font-weight:400;"),
        "{svg}"
    );
}

#[test]
fn pdf_letters_a_borrowed_glyph_in_the_face_that_carries_it() {
    let content = extent(150_000, 80_000);
    let scene = text_scene(
        content,
        vec![text(SceneFont::Monospace, "10 kΩ ⌀6", 20_000)],
    );
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
    assert!(
        base_fonts.iter().any(|name| name.contains("RSpiceSymbols")),
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
    let blank = raster(&text_scene(content, Vec::new()), content);
    let mono = page(SceneFont::Monospace, "Ω");
    assert_eq!(mono, page(SceneFont::Sans, "Ω"));
    assert_ne!(mono, page(SceneFont::Monospace, "\u{25fb}"));
    assert_ne!(mono, blank, "the Ω inked nothing");
    // A symbol no Plex face has prints from RSpice Symbols, as itself.
    let diameter = page(SceneFont::Monospace, "⌀");
    assert_eq!(diameter, page(SceneFont::Sans, "⌀"));
    assert_ne!(diameter, page(SceneFont::Monospace, "\u{25fb}"));
    assert_ne!(diameter, blank, "the ⌀ inked nothing");
    // What no face carries prints as the replacement square, never refused,
    // and never as a question mark.
    let uncovered = page(SceneFont::Sans, "中");
    assert_eq!(uncovered, page(SceneFont::Sans, "\u{25fb}"));
    assert_ne!(uncovered, page(SceneFont::Sans, "?"));
}

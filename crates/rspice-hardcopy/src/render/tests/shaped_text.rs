//! Print sets text as it is read: shaped in the face that carries each
//! glyph, right-to-left runs laid out from the right, and placed by the
//! advances the shaper gave. A PDF draws no missing glyph, archives under
//! PDF/A-2b, and gives its text back in logical order when it is copied.

use std::collections::BTreeMap;

use lopdf::content::Content;
use lopdf::{Encoding, Object};
use rspice_hardcopy_contract::HardcopyFaceSource;

use super::glyph_fallback::{text, undecorated_plan};
use super::*;

/// One line of `value` in `font`, on a scene with a publication time, so
/// it can be archived.
fn line_scene(content: ContentExtent, font: SceneFont, value: &str) -> HardcopyScene {
    let mut metadata =
        HardcopySceneMetadata::try_new("Lettering", "RSpice hardcopy tests").unwrap();
    metadata.set_publication_timestamp(
        HardcopyPublicationTimestamp::try_new(2026, 7, 22, 12, 30, 0).unwrap(),
    );
    HardcopyScene::try_new(
        content,
        metadata,
        vec![text(font, value, 20_000)],
        Vec::new(),
    )
    .unwrap()
}

/// `value` printed alone on an undecorated page in `format`.
fn printed(format: OutputFormat, font: SceneFont, value: &str) -> RenderedHardcopyPublication {
    let content = extent(150_000, 40_000);
    let plan = undecorated_plan(format, content);
    HardcopyRenderer::render(&plan, &line_scene(content, font, value)).unwrap()
}

/// What a PDF page draws, as a reader copies it and as its fonts code it.
struct PageText {
    /// The text each show string maps to through its font's ToUnicode
    /// map, read backwards inside `/ReversedChars` and replaced by the
    /// span's text inside `/ActualText`.
    copied: String,
    /// The base font of every glyph drawn, with its code.
    glyphs: Vec<(String, u16)>,
}

enum Mark {
    Reversed(usize),
    Actual(usize, String),
    Other,
}

/// The first page's text. krilla writes every font as a two-byte
/// Identity-H CID font, so each glyph is two bytes of a show string.
fn page_text(bytes: &[u8]) -> PageText {
    let pdf = ParsedPdf::load_mem(bytes).unwrap();
    let page = pdf.get_pages()[&1];
    let fonts = pdf.get_page_fonts(page).unwrap();
    let base_fonts: BTreeMap<Vec<u8>, String> = fonts
        .iter()
        .map(|(name, font)| {
            let base = font.get(b"BaseFont").unwrap().as_name().unwrap();
            (name.clone(), String::from_utf8_lossy(base).into_owned())
        })
        .collect();
    let encodings: BTreeMap<Vec<u8>, Encoding> = fonts
        .iter()
        .map(|(name, font)| (name.clone(), font.get_font_encoding(&pdf).unwrap()))
        .collect();
    let content = Content::decode(&pdf.get_page_content(page)).unwrap();
    let mut units: Vec<String> = Vec::new();
    let mut glyphs = Vec::new();
    let mut marks = Vec::new();
    let mut font: Option<&[u8]> = None;
    for operation in &content.operations {
        match operation.operator.as_str() {
            "Tf" => font = Some(operation.operands[0].as_name().unwrap()),
            "Tj" | "TJ" => {
                let font = font.expect("a font is set before text is shown");
                let strings = match &operation.operands[0] {
                    Object::Array(items) => items.clone(),
                    other => vec![other.clone()],
                };
                for item in strings {
                    let Object::String(codes, _) = item else {
                        continue;
                    };
                    for code in codes.chunks(2) {
                        glyphs.push((
                            base_fonts[font].clone(),
                            u16::from_be_bytes([code[0], code[1]]),
                        ));
                        units.push(ParsedPdf::decode_text(&encodings[font], code).unwrap());
                    }
                }
            }
            "BMC" => marks.push(
                if operation.operands[0].as_name().unwrap() == b"ReversedChars" {
                    Mark::Reversed(units.len())
                } else {
                    Mark::Other
                },
            ),
            "BDC" => {
                let actual = operation
                    .operands
                    .get(1)
                    .and_then(|properties| properties.as_dict().ok())
                    .and_then(|properties| properties.get(b"ActualText").ok())
                    .map(|actual| lopdf::decode_text_string(actual).unwrap());
                marks.push(match actual {
                    Some(actual) => Mark::Actual(units.len(), actual),
                    None => Mark::Other,
                });
            }
            "EMC" => match marks.pop() {
                Some(Mark::Reversed(start)) => units[start..].reverse(),
                Some(Mark::Actual(start, actual)) => {
                    units.truncate(start);
                    units.push(actual);
                }
                _ => {}
            },
            _ => {}
        }
    }
    PageText {
        copied: units.concat(),
        glyphs,
    }
}

/// Every script print letters from a bundled face, with the face it must
/// print from.
fn scripts() -> Vec<(&'static str, SceneFont, &'static str)> {
    let mut scripts = vec![
        ("Latin", SceneFont::Sans, "IBMPlexSans"),
        ("Greek ΔΩμ ±3 kΩ", SceneFont::Monospace, "IBMPlexSans"),
        ("⌀10 ⏚ ∅ ⌭ Ⓜ", SceneFont::Sans, "RSpiceSymbols"),
    ];
    #[cfg(not(target_arch = "wasm32"))]
    scripts.extend([
        ("لا إله قيمة", SceneFont::Sans, "IBMPlexSansArabic"),
        ("مَرْحَبًا", SceneFont::SansSemibold, "IBMPlexSansArabic"),
        ("שָׁלוֹם עולם", SceneFont::Sans, "IBMPlexSansHebrew"),
        ("מגבר R1 = 10 kΩ (V+)", SceneFont::Sans, "IBMPlexSansHebrew"),
        ("ภาษาไทย น้ำ", SceneFont::Sans, "IBMPlexSansThai"),
    ]);
    scripts
}

#[test]
fn a_pdf_draws_every_script_from_its_face_and_copies_back_as_typed() {
    for (value, font, face) in scripts() {
        let publication = printed(OutputFormat::PdfVector, font, value);
        let page = page_text(publication.single_part().unwrap().bytes());
        assert_eq!(page.copied, value, "{value:?}");
        assert!(
            page.glyphs.iter().any(|(base, _)| base.contains(face)),
            "{value:?} is not drawn from {face}: {:?}",
            page.glyphs
        );
        assert!(
            page.glyphs.iter().all(|(_, code)| *code != 0),
            "{value:?} draws a missing glyph: {:?}",
            page.glyphs
        );
    }
}

#[test]
fn an_archived_pdf_of_every_script_passes_validation() {
    for (value, font, _) in scripts() {
        let publication = printed(OutputFormat::PdfA, font, value);
        assert_eq!(
            publication.pdf_conformance(),
            Some(PdfConformance::PdfA2bValidated),
            "{value:?}"
        );
        let page = page_text(publication.single_part().unwrap().bytes());
        assert_eq!(page.copied, value, "{value:?}");
    }
}

#[test]
fn a_character_no_face_has_prints_as_the_square_and_copies_as_itself() {
    let value = "R\u{E000}\u{10FFFD} a\u{0363}";
    let publication = printed(OutputFormat::PdfA, SceneFont::Sans, value);
    let page = page_text(publication.single_part().unwrap().bytes());
    assert_eq!(page.copied, value);
    assert!(page.glyphs.iter().all(|(_, code)| *code != 0));
    assert!(
        page.glyphs
            .iter()
            .any(|(base, _)| base.contains("RSpiceSymbols"))
    );
}

/// Where each text run a PDF page draws starts, in points, and the size its
/// font is set at.
fn run_starts(bytes: &[u8]) -> (Vec<f32>, f32) {
    let pdf = ParsedPdf::load_mem(bytes).unwrap();
    let page = pdf.get_pages()[&1];
    let content = Content::decode(&pdf.get_page_content(page)).unwrap();
    let mut size = 0.0;
    let mut starts = Vec::new();
    for operation in &content.operations {
        match operation.operator.as_str() {
            "Tf" => size = operation.operands[1].as_float().unwrap(),
            "Tm" => starts.push(operation.operands[4].as_float().unwrap()),
            _ => {}
        }
    }
    (starts, size)
}

/// The PDF places a line by the advances the shaper gave: kerning closes
/// "AV" and "To", so a line anchored at its end starts where its shaped
/// width puts it, not where the faces' own advances would.
#[test]
fn a_line_is_placed_by_its_shaped_width() {
    let value = "AVATAR To Wa";
    let shaped: i64 = rspice_fonts::print::line(SceneFont::Sans.role(), value)
        .iter()
        .map(rspice_fonts::print::Piece::advance)
        .sum();
    let face = PrintFaces::get().unwrap().face(PrintFace::SansRegular);
    let unshaped: i64 = value
        .chars()
        .filter_map(|character| face.glyph_index(character))
        .filter_map(|glyph| face.glyph_hor_advance(glyph))
        .map(i64::from)
        .sum();
    assert!(shaped < unshaped, "{shaped} vs {unshaped}");
    let content = extent(150_000, 40_000);
    let starts = |anchor: TextAnchor| {
        let mut line = text(SceneFont::Sans, value, 20_000);
        if let ScenePrimitive::Text { anchor: placed, .. } = &mut line {
            *placed = anchor;
        }
        let scene = HardcopyScene::try_new(
            content,
            HardcopySceneMetadata::try_new("Lettering", "RSpice hardcopy tests").unwrap(),
            vec![line],
            Vec::new(),
        )
        .unwrap();
        let plan = undecorated_plan(OutputFormat::PdfVector, content);
        let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
        run_starts(publication.single_part().unwrap().bytes())
    };
    let (from_start, size) = starts(TextAnchor::Start);
    let (from_end, _) = starts(TextAnchor::End);
    assert_eq!((from_start.len(), from_end.len()), (1, 1));
    let em = f32::from(face.units_per_em());
    let width = from_start[0] - from_end[0];
    let expected = shaped as f32 * size / em;
    assert!((width - expected).abs() < 0.01, "{width} vs {expected}");
    assert!((width - unshaped as f32 * size / em).abs() > 0.1);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_receipt_names_the_script_faces_a_page_letters() {
    let publication = printed(OutputFormat::PdfVector, SceneFont::Sans, "قيمة שלום ไทย R1");
    let mut families: Vec<&str> = publication
        .faces_used()
        .iter()
        .map(|face| {
            assert_eq!(face.source(), &HardcopyFaceSource::Bundled);
            face.family()
        })
        .collect();
    families.sort_unstable();
    assert_eq!(
        families,
        [
            "IBM Plex Sans",
            "IBM Plex Sans Arabic",
            "IBM Plex Sans Hebrew",
            "IBM Plex Sans Thai",
        ]
    );
}

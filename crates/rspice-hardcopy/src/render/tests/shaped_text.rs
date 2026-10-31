//! Print sets text as it is read: shaped in the face that carries each
//! glyph, right-to-left runs laid out from the right, and placed by the
//! advances the shaper gave. A PDF draws no missing glyph, archives under
//! PDF/A-2b, and gives its text back in logical order when it is copied.

use std::collections::BTreeMap;
use std::sync::Arc;

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
    let face = PrintFaces::get()
        .unwrap()
        .face(PrintFace::Bundled(Face::SansRegular));
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

/// The render budget allows each page the faces it can embed: a Latin
/// sheet the six faces every build embeds, as before Arabic, Hebrew and
/// Thai could print, and an Arabic sheet the Arabic cut as well.
#[test]
fn the_render_budget_allows_for_the_faces_a_page_letters_in() {
    let content = extent(150_000, 40_000);
    let allowance = |value: &str| {
        let plan = undecorated_plan(OutputFormat::SvgVector, content);
        crate::render::publication::embedded_font_allowance(
            &plan,
            &line_scene(content, SceneFont::Sans, value),
        )
        .unwrap()
    };
    // Plex Sans R, M and SB, Plex Mono R and M, and RSpice Symbols, twice.
    let every_build = 1_777_480;
    assert_eq!(allowance("Latin text, 10 kΩ ⌀6"), every_build);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(
        allowance("قيمة R1"),
        every_build + 2 * PrintFace::Bundled(Face::SansArabicRegular).bytes().len() as u64
    );
}

/// Each glyph a viewer lays `svg`'s text out in, as usvg's own layout
/// places it: its id and x, with the controls that steer the layout left
/// out, and the size the text is lettered at. The viewer letters from the
/// faces the SVG embeds, under the families the SVG names them, as a
/// browser does.
fn laid_out_glyphs(svg: &str) -> (Vec<(u32, f32)>, f32) {
    use base64::Engine as _;

    let mut database = usvg::fontdb::Database::new();
    for rule in svg.split("@font-face{").skip(1) {
        let family = rule
            .split("font-family:'")
            .nth(1)
            .and_then(|rest| rest.split('\'').next())
            .expect("a face names its family");
        let data = rule
            .split("base64,")
            .nth(1)
            .and_then(|rest| rest.split(')').next())
            .expect("a face carries its data");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .unwrap();
        for id in database.load_font_source(usvg::fontdb::Source::Binary(Arc::new(bytes))) {
            let mut face = database.face(id).cloned().unwrap();
            face.families = vec![(
                family.to_owned(),
                usvg::fontdb::Language::English_UnitedStates,
            )];
            database.push_face_info(face);
            database.remove_face(id);
        }
    }
    let options = usvg::Options {
        fontdb: Arc::new(database),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(svg, &options).unwrap();
    let mut glyphs = Vec::new();
    let mut size = 0.0;
    let mut groups = vec![tree.root()];
    while let Some(group) = groups.pop() {
        for node in group.children() {
            match node {
                usvg::Node::Group(nested) => groups.push(nested),
                usvg::Node::Text(text) => {
                    for span in text.layouted() {
                        for glyph in &span.positioned_glyphs {
                            // usvg gives a cluster's text to one of its
                            // glyphs and none to the rest, so a glyph with no
                            // text of its own is still a letter or a mark.
                            if !glyph.text.is_empty()
                                && glyph.text.chars().all(|character| {
                                    matches!(
                                        character,
                                        '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}'
                                    )
                                })
                            {
                                continue;
                            }
                            size = glyph.font_size();
                            glyphs.push((glyph.id.0, glyph.transform().tx));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    (glyphs, size)
}

/// Each glyph of `value` where print sets it, `size` units to the em, from
/// the line's left end.
fn printed_glyphs(font: SceneFont, value: &str, size: f32) -> Vec<(u32, f32)> {
    let mut left = 0.0;
    let mut glyphs = Vec::new();
    for piece in rspice_fonts::print::line(font.role(), value) {
        let scale = size / f32::from(piece.units_per_em);
        for glyph in &piece.glyphs {
            glyphs.push((glyph.id, left + glyph.x_offset as f32 * scale));
            left += glyph.advance as f32 * scale;
        }
    }
    glyphs
}

/// A viewer sets an SVG's text glyph for glyph where print does: each piece
/// at the x the shaped advances give it, in its face, right-to-left pieces
/// from their right end. usvg, which knows only a left-to-right paragraph,
/// lays the mixed line out as the PDF does.
#[test]
fn an_svg_viewer_sets_every_glyph_where_print_does() {
    let mut lines = vec![
        (SceneFont::Sans, "AVATAR To Wa"),
        (SceneFont::Monospace, "R1 10 kΩ α ⌀6 ⏚"),
        (SceneFont::Sans, "Plain \u{0378} \u{E000}"),
    ];
    #[cfg(not(target_arch = "wasm32"))]
    lines.extend([
        (SceneFont::Sans, "מגבר R1 = 10 kΩ (V+)"),
        (SceneFont::Sans, "לא (גד) 12 שלום"),
        (SceneFont::Sans, "لا إله قيمة 123"),
        (SceneFont::SansSemibold, "مَرْحَبًا R2"),
        (SceneFont::Sans, "שָׁלוֹם R3"),
        (SceneFont::Sans, "ภาษาไทย น้ำ"),
    ]);
    for anchor in [TextAnchor::Start, TextAnchor::Middle, TextAnchor::End] {
        for (font, value) in &lines {
            let content = extent(150_000, 40_000);
            let mut line = text(*font, value, 20_000);
            if let ScenePrimitive::Text {
                origin,
                anchor: placed,
                ..
            } = &mut line
            {
                *origin = ScenePoint::new(
                    Length::from_micrometres(75_000),
                    Length::from_micrometres(20_000),
                );
                *placed = anchor;
            }
            let scene = HardcopyScene::try_new(
                content,
                HardcopySceneMetadata::try_new("Lettering", "RSpice hardcopy tests").unwrap(),
                vec![line],
                Vec::new(),
            )
            .unwrap();
            let plan = undecorated_plan(OutputFormat::SvgVector, content);
            let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
            let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
            let (mut laid_out, size) = laid_out_glyphs(svg);
            let mut printed = printed_glyphs(*font, value, size);
            assert_eq!(
                laid_out.len(),
                printed.len(),
                "{value:?}: {laid_out:?} {printed:?}"
            );
            let origin = |glyphs: &[(u32, f32)]| {
                glyphs.iter().map(|(_, x)| *x).fold(f32::INFINITY, f32::min)
            };
            let (from_viewer, from_print) = (origin(&laid_out), origin(&printed));
            for glyphs in [&mut laid_out, &mut printed] {
                glyphs.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
            }
            let tolerance = size * 0.01;
            for ((viewer_id, viewer_x), (print_id, print_x)) in laid_out.iter().zip(&printed) {
                assert_eq!(
                    viewer_id, print_id,
                    "{value:?} {anchor:?}: {laid_out:?} {printed:?}"
                );
                assert!(
                    ((viewer_x - from_viewer) - (print_x - from_print)).abs() < tolerance,
                    "{value:?} {anchor:?}: glyph {viewer_id} at {} where print sets it at {}",
                    viewer_x - from_viewer,
                    print_x - from_print
                );
            }
        }
    }
}

/// A right-to-left piece is written in logical order inside a right-to-left
/// override, and names its face.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_svg_writes_a_right_to_left_piece_as_typed_inside_an_override() {
    let publication = printed(OutputFormat::SvgVector, SceneFont::Sans, "שלום R1");
    let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
    let named = super::glyph_fallback::without_positions(svg);
    assert!(
        named.contains(
            "<tspan font-family=\"RSpice Plex Sans Hebrew\" font-weight=\"400\">\u{202E}שלום\u{200F}\u{202C}</tspan>"
        ),
        "{named}"
    );
    assert!(named.contains("<tspan>R1</tspan>"), "{named}");
}

/// Print letters from an installed face for as long as this holds it, and
/// from none afterwards. Print looks in one place per process, so the tests
/// that move it take turns.
#[cfg(not(target_arch = "wasm32"))]
struct Installed {
    _turn: std::sync::MutexGuard<'static, ()>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Installed {
    fn faces(known: Vec<rspice_fonts::system::KnownFace>) -> Self {
        static TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let turn = TURN
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        rspice_fonts::system::print::print_from(
            rspice_fonts::system::Source::files(known),
            rspice_fonts::HanVariant::Japanese.order(),
        );
        Self { _turn: turn }
    }

    /// IBM Plex Sans JP, read as an installed face so every machine has it.
    fn japanese() -> Self {
        Self::faces(vec![installed_face(fixture_font(
            "IBMPlexSansJP-Regular.ttf",
        ))])
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for Installed {
    fn drop(&mut self) {
        rspice_fonts::system::print::print_from_none();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn fixture_font(file: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fonts")
        .join(file)
}

#[cfg(not(target_arch = "wasm32"))]
fn installed_face(path: std::path::PathBuf) -> rspice_fonts::system::KnownFace {
    rspice_fonts::system::KnownFace::new(
        path,
        "IBM Plex Sans JP",
        rspice_fonts::system::FaceClass::Han(rspice_fonts::HanVariant::Japanese),
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pdf_letters_what_no_embedded_face_has_from_an_installed_face() {
    let _installed = Installed::japanese();
    let value = "出力 R1 中文";
    for format in [OutputFormat::PdfVector, OutputFormat::PdfA] {
        let publication = printed(format, SceneFont::Sans, value);
        let page = page_text(publication.single_part().unwrap().bytes());
        assert_eq!(page.copied, value);
        assert!(
            page.glyphs
                .iter()
                .any(|(base, _)| base.contains("IBMPlexSansJP")),
            "{:?}",
            page.glyphs
        );
        assert!(
            !page
                .glyphs
                .iter()
                .any(|(base, _)| base.contains("RSpiceSymbols")),
            "nothing prints as the square: {:?}",
            page.glyphs
        );
        if format == OutputFormat::PdfA {
            assert_eq!(
                publication.pdf_conformance(),
                Some(PdfConformance::PdfA2bValidated)
            );
        }
    }
}

/// An SVG draws an installed face's glyphs as outlines, each defined once
/// and used where it stands, titled with the text as typed; it carries no
/// copy of the face itself.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_svg_draws_an_installed_face_as_outlines_defined_once() {
    let _installed = Installed::japanese();
    let publication = printed(OutputFormat::SvgVector, SceneFont::Sans, "R1 中文中");
    let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
    assert_eq!(svg.matches("<path id=\"o").count(), 2, "{svg}");
    // Each id names its page: a browser print puts every page in one
    // document.
    assert!(svg.contains("<path id=\"o1-1\""), "{svg}");
    assert_eq!(svg.matches("<use href=\"#o").count(), 3, "{svg}");
    assert!(svg.contains("<title>中文中</title>"), "{svg}");
    assert!(!svg.contains("IBM Plex Sans JP"), "{svg}");
    let text = &svg[svg.find("<text").unwrap()..];
    assert!(
        !text.contains('中'),
        "the outlined text is not also text: {text}"
    );
    // The rasteriser draws the outlines, not the square.
    let raster = |value: &str| {
        printed(OutputFormat::Png { dpi: 150 }, SceneFont::Sans, value)
            .single_part()
            .unwrap()
            .bytes()
            .to_vec()
    };
    assert_ne!(raster("中"), raster("\u{0378}"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_receipt_names_an_installed_face_by_its_file_never_the_home_directory() {
    let _installed = Installed::japanese();
    let publication = printed(OutputFormat::PdfVector, SceneFont::Sans, "R1 中文");
    let installed: Vec<_> = publication
        .faces_used()
        .iter()
        .filter(|face| face.source() != &HardcopyFaceSource::Bundled)
        .collect();
    let [face] = installed.as_slice() else {
        panic!("one installed face: {:?}", publication.faces_used());
    };
    assert_eq!(face.family(), "IBM Plex Sans JP");
    assert_eq!(face.weight(), 400);
    let HardcopyFaceSource::System { path } = face.source() else {
        panic!("{face:?}");
    };
    assert!(path.ends_with("IBMPlexSansJP-Regular.ttf"), "{path}");
    let home = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    if let Some(home) = std::env::var_os(home).filter(|home| !home.is_empty()) {
        assert!(!path.contains(home.to_str().unwrap()), "{path}");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_receipt_writes_a_face_one_user_installed_from_the_home_directory() {
    use std::path::{MAIN_SEPARATOR as SEPARATOR, Path};

    use crate::render::glyph_runs::receipt_path;

    let home = if cfg!(windows) {
        Path::new(r"C:\Users\Ada")
    } else {
        Path::new("/home/ada")
    };
    let per_user = if cfg!(windows) {
        home.join(r"AppData\Local\Microsoft\Windows\Fonts\NotoSansJP-Regular.ttf")
    } else {
        home.join(".local/share/fonts/NotoSansJP-Regular.ttf")
    };
    let within = per_user.strip_prefix(home).unwrap().display().to_string();
    assert_eq!(
        receipt_path(&per_user, Some(home)),
        format!("~{SEPARATOR}{within}")
    );
    let system = if cfg!(windows) {
        Path::new(r"C:\Windows\Fonts\YuGothM.ttc")
    } else {
        Path::new("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc")
    };
    assert_eq!(
        receipt_path(system, Some(home)),
        system.display().to_string()
    );
    assert_eq!(
        receipt_path(&per_user, None),
        per_user.display().to_string()
    );
}

/// A face whose licence keeps it out of documents is passed over: its
/// characters print as the square, and the PDF carries no part of it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn print_never_embeds_a_face_whose_licence_forbids_it() {
    let mut bytes = std::fs::read(fixture_font("IBMPlexSansJP-Regular.ttf")).unwrap();
    let tables = usize::from(u16::from_be_bytes([bytes[4], bytes[5]]));
    let os2 = (0..tables)
        .map(|table| 12 + 16 * table)
        .find(|&record| &bytes[record..record + 4] == b"OS/2")
        .map(|record| u32::from_be_bytes(bytes[record + 8..record + 12].try_into().unwrap()))
        .expect("the face has an OS/2 table") as usize;
    // fsType 2: Restricted License embedding.
    bytes[os2 + 8..os2 + 10].copy_from_slice(&2_u16.to_be_bytes());
    let path = std::env::temp_dir().join(format!(
        "rspice-hardcopy-restricted-{}.ttf",
        std::process::id()
    ));
    std::fs::write(&path, bytes).unwrap();
    let installed = Installed::faces(vec![installed_face(path.clone())]);
    let value = "R1 中文";
    let publication = printed(OutputFormat::PdfA, SceneFont::Sans, value);
    drop(installed);
    let _ = std::fs::remove_file(path);
    let page = page_text(publication.single_part().unwrap().bytes());
    assert_eq!(page.copied, value);
    assert!(
        !page
            .glyphs
            .iter()
            .any(|(base, _)| base.contains("IBMPlexSansJP")),
        "{:?}",
        page.glyphs
    );
    assert!(
        page.glyphs
            .iter()
            .any(|(base, _)| base.contains("RSpiceSymbols"))
    );
    assert_eq!(
        publication.pdf_conformance(),
        Some(PdfConformance::PdfA2bValidated)
    );
}

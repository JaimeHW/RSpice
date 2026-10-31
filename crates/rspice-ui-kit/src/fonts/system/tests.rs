use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Instant;

use egui::{Color32, FontId, Pos2, Rect, RichText, Vec2};

use super::*;

const SIZE: Vec2 = Vec2::new(90.0, 30.0);
const PATIENCE: Duration = Duration::from_secs(30);

/// Fixtures are offered through the same path as an installed face, so a
/// machine without these faces still tests that path.
fn fixture(file: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fonts")
        .join(file)
}

fn korean() -> KnownFace {
    KnownFace::new(
        fixture("IBMPlexSansKR-Regular.ttf"),
        "IBM Plex Sans KR",
        FaceClass::Han(HanVariant::Korean),
    )
}

fn japanese() -> KnownFace {
    KnownFace::new(
        fixture("IBMPlexSansJP-Regular.ttf"),
        "IBM Plex Sans JP",
        FaceClass::Han(HanVariant::Japanese),
    )
}

/// Every script face in assets/fonts is embedded on a desktop, so a face no
/// chain holds is offered as an installed script face in their place.
fn script_stand_in() -> KnownFace {
    KnownFace::new(
        fixture("IBMPlexSansJP-Regular.ttf"),
        "IBM Plex Sans JP",
        FaceClass::Script,
    )
}

fn themed() -> Context {
    let ctx = Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::Theme::default().apply(&ctx);
    ctx
}

fn context(source: Source) -> Context {
    let ctx = themed();
    attach_with(
        &ctx,
        SystemFonts::spawn(source, HanVariant::Simplified.order()),
        Vec::new(),
    );
    ctx
}

fn frame(ctx: &Context, text: &str) -> FullOutput {
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SIZE)),
        ..Default::default()
    };
    ctx.run_ui(input, |ui| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                ui.label(
                    RichText::new(text)
                        .font(FontId::proportional(18.0))
                        .color(Color32::WHITE),
                );
            });
    })
}

fn ink(ctx: &Context, output: &FullOutput) -> Vec<Color32> {
    crate::raster::rasterize(ctx, SIZE, output)
        .pixels_in(Rect::from_min_size(Pos2::ZERO, SIZE))
        .collect()
}

fn plugin<R>(ctx: &Context, read: impl FnOnce(&mut SystemFontsPlugin) -> R) -> R {
    ctx.with_plugin(read).expect("the plugin is attached")
}

/// Paints `text` until the worker has answered every request and the faces
/// it found are lettering, and returns the last frame.
fn settle(ctx: &Context, text: &str) -> FullOutput {
    let deadline = Instant::now() + PATIENCE;
    frame(ctx, text);
    while plugin(ctx, |plugin| plugin.outstanding > 0) {
        assert!(Instant::now() < deadline, "the worker answers");
        std::thread::sleep(Duration::from_millis(5));
        frame(ctx, text);
    }
    // The pass that took the answer installed it; the next one rebuilds the
    // fonts, and the one after letters at the settled size.
    frame(ctx, text);
    frame(ctx, text)
}

#[test]
fn a_script_no_embedded_face_has_paints_from_an_installed_face() {
    let ctx = context(Source::files(vec![korean()]));
    let before = ink(&ctx, &frame(&ctx, "한국"));
    let square = ink(&ctx, &frame(&ctx, "\u{25FB}\u{25FB}"));
    assert_eq!(before, square, "until the face joins, Hangul is the square");

    let after = ink(&ctx, &settle(&ctx, "한국"));
    assert_eq!(joined_families(&ctx), ["IBM Plex Sans KR"]);
    assert_ne!(after, before);
    assert_ne!(after, ink(&ctx, &frame(&ctx, " ")));

    // The same face installed directly letters the same pixels.
    let worker = SystemFonts::spawn(
        Source::files(vec![korean()]),
        HanVariant::Simplified.order(),
    );
    worker.request(vec!['한', '국']);
    let joined = worker.wait_resolution(PATIENCE).expect("answered").joined;
    let reference = themed();
    reference.set_fonts(definitions(&joined));
    frame(&reference, "한국");
    frame(&reference, "한국");
    assert_eq!(after, ink(&reference, &frame(&reference, "한국")));
}

#[test]
fn the_first_frame_does_not_wait_for_the_font_files() {
    let (release, gate) = mpsc::channel::<()>();
    // A plugin that waited on the worker would hang on the first frame; the
    // worker is released after a while so such a failure reports.
    let watchdog = release.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(20));
        let _ = watchdog.send(());
    });
    let ctx = context(Source::files(vec![korean()]).after(gate));

    let started = Instant::now();
    frame(&ctx, "한국");
    let first = started.elapsed();
    assert!(
        first < Duration::from_secs(10),
        "first frame took {first:?}"
    );
    assert_eq!(plugin(&ctx, |plugin| plugin.outstanding), 1);
    for _ in 0..3 {
        frame(&ctx, "한국");
    }
    assert!(joined_families(&ctx).is_empty(), "the worker is still held");

    release.send(()).unwrap();
    settle(&ctx, "한국");
    assert_eq!(joined_families(&ctx), ["IBM Plex Sans KR"]);
}

#[test]
fn faces_found_for_one_frame_join_in_one_rebuild_scripts_before_cjk() {
    let ctx = context(Source::files(vec![korean(), script_stand_in()]));
    settle(&ctx, "한 カナ");
    assert_eq!(plugin(&ctx, |plugin| plugin.rebuilds), 1);
    assert_eq!(
        joined_families(&ctx),
        ["IBM Plex Sans JP", "IBM Plex Sans KR"]
    );
    let keys: Vec<String> = plugin(&ctx, |plugin| {
        plugin.joined.iter().map(SystemFace::key).collect()
    });
    let fonts = plugin(&ctx, |plugin| definitions(&plugin.joined));
    for role in Role::ALL {
        let chain = &fonts.families[&role.egui_family()];
        let embedded: Vec<&str> = role.chain().iter().map(|face| face.key()).collect();
        assert_eq!(chain[..embedded.len()], embedded, "{role:?}");
        assert_eq!(chain[embedded.len()..], keys, "{role:?}");
    }
}

#[test]
fn text_the_embedded_faces_letter_asks_for_nothing() {
    let ctx = context(Source::files(vec![korean()]));
    frame(&ctx, "V(out) = 1.2 V");
    frame(&ctx, "R1 4.7 kΩ ±1 % 25 °C µA ⌀");
    plugin(&ctx, |plugin| {
        assert!(plugin.asked.is_empty());
        assert_eq!(plugin.outstanding, 0);
    });
}

#[test]
fn the_document_language_reorders_the_cjk_faces() {
    // U+3001 IDEOGRAPHIC COMMA is in both faces; Simplified Chinese orders
    // Japanese ahead of Korean.
    let ctx = context(Source::files(vec![korean(), japanese()]));
    settle(&ctx, "、");
    assert_eq!(joined_families(&ctx), ["IBM Plex Sans JP"]);

    set_document_language(&ctx, Some("ko"));
    settle(&ctx, "、");
    assert_eq!(
        joined_families(&ctx),
        ["IBM Plex Sans KR", "IBM Plex Sans JP"]
    );
    let rebuilds = plugin(&ctx, |plugin| plugin.rebuilds);

    set_document_language(&ctx, Some("ko"));
    set_document_language(&ctx, Some("ko-KR"));
    settle(&ctx, "、");
    assert_eq!(plugin(&ctx, |plugin| plugin.rebuilds), rebuilds);
}

#[test]
fn reinstalling_the_fonts_keeps_the_faces_that_joined() {
    let ctx = context(Source::files(vec![korean()]));
    let squares = ink(&ctx, &frame(&ctx, "한국"));
    let joined = ink(&ctx, &settle(&ctx, "한국"));
    assert_ne!(joined, squares);

    // Losing the context's memory loses the theme's once-flag, so the theme
    // installs the fonts again.
    ctx.memory_mut(|memory| *memory = egui::Memory::default());
    crate::Theme::default().apply(&ctx);
    frame(&ctx, "한국");
    frame(&ctx, "한국");
    assert_eq!(ink(&ctx, &frame(&ctx, "한국")), joined);

    // So does an install made directly.
    crate::fonts::install(&ctx);
    frame(&ctx, "한국");
    frame(&ctx, "한국");
    assert_eq!(ink(&ctx, &frame(&ctx, "한국")), joined);
    assert_eq!(joined_families(&ctx), ["IBM Plex Sans KR"]);
}

#[test]
fn coverage_is_settled_by_the_installed_faces() {
    use crate::fonts::coverage::{Coverage, ask, changes, coverage};
    // The embedded faces answer at once, and a joiner draws nothing.
    for character in ['R', '\u{3a9}', '\u{2300}', '\u{5d0}', '\u{200d}'] {
        assert_eq!(coverage(character), Coverage::Lettered, "{character:?}");
    }
    let ctx = context(Source::files(vec![japanese()]));
    // A kana only the installed Japanese face has, once a frame paints it.
    let kana = '\u{3090}';
    assert_eq!(coverage(kana), Coverage::Unsettled);
    let before = changes();
    settle(&ctx, &kana.to_string());
    assert_eq!(coverage(kana), Coverage::Lettered);
    assert!(changes() > before);
    // A code point no face has, asked about without being painted: whichever
    // worker takes the question, no face it reads has it.
    let unassigned = '\u{0378}';
    ask([unassigned]);
    let deadline = Instant::now() + PATIENCE;
    while coverage(unassigned) == Coverage::Unsettled {
        assert!(Instant::now() < deadline, "the question is answered");
        frame(&ctx, "");
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(coverage(unassigned), Coverage::Missing);
}

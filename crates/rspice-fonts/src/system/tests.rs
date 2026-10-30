use std::time::Duration;

use super::*;
use crate::HanVariant::{Japanese, Korean, Simplified};

/// A fixture is read through the same path as an installed face, so a machine
/// without the face still tests that path.
fn fixture(file: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fonts")
        .join(file)
}

fn korean() -> KnownFace {
    KnownFace::new(
        fixture("IBMPlexSansKR-Regular.ttf"),
        "IBM Plex Sans KR",
        FaceClass::Han(Korean),
    )
}

fn japanese() -> KnownFace {
    KnownFace::new(
        fixture("IBMPlexSansJP-Regular.ttf"),
        "IBM Plex Sans JP",
        FaceClass::Han(Japanese),
    )
}

const PATIENCE: Duration = Duration::from_secs(30);

fn answer(fonts: &SystemFonts) -> Resolution {
    fonts
        .wait_resolution(PATIENCE)
        .expect("the worker answers every request")
}

#[test]
fn a_face_joins_for_a_character_only_it_has() {
    let fonts = SystemFonts::spawn(Source::files(vec![korean()]), Simplified.order());
    fonts.request(vec!['한']);
    let resolution = answer(&fonts);
    assert!(resolution.missing.is_empty(), "{resolution:?}");
    let [face] = resolution.joined.as_slice() else {
        panic!("one face joins: {resolution:?}");
    };
    assert_eq!(face.family(), "IBM Plex Sans KR");
    assert_eq!(face.class(), FaceClass::Han(Korean));
    assert!(face.covers('한') && face.covers('국'));
    assert_eq!(
        face.bytes(),
        std::fs::read(fixture("IBMPlexSansKR-Regular.ttf")).unwrap()
    );
}

#[test]
fn a_face_joins_once_and_a_character_none_has_is_missing() {
    let fonts = SystemFonts::spawn(Source::files(vec![korean()]), Simplified.order());
    fonts.request(vec!['한']);
    assert_eq!(answer(&fonts).joined.len(), 1);
    // The Korean face has no Han characters, and the fixture source offers
    // nothing else.
    fonts.request(vec!['국', '一']);
    let resolution = answer(&fonts);
    assert!(resolution.joined.is_empty(), "{resolution:?}");
    assert_eq!(resolution.missing, ['一']);
}

#[test]
fn the_han_order_chooses_the_face_of_a_shared_character() {
    // U+3001 IDEOGRAPHIC COMMA is in both faces.
    for (order, family) in [
        (Japanese.order(), "IBM Plex Sans JP"),
        (Korean.order(), "IBM Plex Sans KR"),
    ] {
        let fonts = SystemFonts::spawn(Source::files(vec![korean(), japanese()]), order);
        fonts.request(vec!['、']);
        let joined = answer(&fonts).joined;
        assert_eq!(
            joined.iter().map(SystemFace::family).collect::<Vec<_>>(),
            [family]
        );
    }
}

#[test]
fn a_new_order_joins_the_face_of_the_form_now_first() {
    let fonts = SystemFonts::spawn(Source::files(vec![korean(), japanese()]), Korean.order());
    fonts.request(vec!['、']);
    assert_eq!(answer(&fonts).joined[0].family(), "IBM Plex Sans KR");
    fonts.reorder(Japanese.order(), vec!['、']);
    let joined = answer(&fonts).joined;
    assert_eq!(
        joined.iter().map(SystemFace::family).collect::<Vec<_>>(),
        ["IBM Plex Sans JP"]
    );
    assert!(
        joined[0].precedence(&Japanese.order()) < (1, 4),
        "Japanese stands first among the CJK faces"
    );
}

#[test]
fn a_gated_worker_reads_nothing_and_answers_nothing_until_released() {
    let (release, gate) = mpsc::channel();
    let fonts = SystemFonts::spawn(
        Source::files(vec![korean()]).after(gate),
        Simplified.order(),
    );
    fonts.request(vec!['한']);
    assert!(fonts.try_resolution().is_none());
    assert!(fonts.wait_resolution(Duration::from_millis(200)).is_none());
    release.send(()).unwrap();
    assert_eq!(answer(&fonts).joined.len(), 1);
}

#[test]
fn a_file_that_no_longer_parses_when_read_joins_nothing() {
    let path =
        std::env::temp_dir().join(format!("rspice-fonts-refused-{}.ttf", std::process::id()));
    std::fs::copy(fixture("IBMPlexSansKR-Regular.ttf"), &path).unwrap();
    let mut catalog = Catalog::probe(&[KnownFace::new(
        &path,
        "IBM Plex Sans KR",
        FaceClass::Han(Korean),
    )]);
    assert_eq!(catalog.candidates.len(), 1);
    // Replaced on disk after the probe found a face in it: neither the
    // coverage check nor the copy read to join may take it for a font.
    let mut damaged = std::fs::read(&path).unwrap();
    damaged[..4].copy_from_slice(b"junk");
    std::fs::write(&path, damaged).unwrap();

    let mut joined = HashSet::new();
    let mut pending = vec!['한'];
    let mut resolution = Resolution::default();
    catalog.resolve(
        &Simplified.order(),
        &mut joined,
        &mut pending,
        &mut resolution,
    );
    let loaded = SystemFace::load(&catalog.candidates[0], &['한']);
    let _ = std::fs::remove_file(&path);

    assert!(resolution.joined.is_empty());
    assert_eq!(pending, ['한']);
    assert!(loaded.is_none(), "the copy read to join is parsed first");
}

#[test]
fn a_known_file_that_is_absent_offers_nothing() {
    let catalog = Catalog::probe(&[
        KnownFace::new(fixture("absent.ttf"), "Absent", FaceClass::Script),
        KnownFace::new(
            fixture("IBMPlexSansKR-Regular.ttf"),
            "Another family",
            FaceClass::Script,
        ),
    ]);
    assert!(catalog.candidates.is_empty());
}

#[test]
fn system_faces_stand_scripts_then_cjk_in_order_then_extras_then_found() {
    let order = Japanese.order();
    let mut classes = [
        (FaceClass::Found, 0),
        (FaceClass::Extra, 0),
        (FaceClass::Han(Simplified), 0),
        (FaceClass::Han(Japanese), 9),
        (FaceClass::Script, 1),
        (FaceClass::Script, 0),
    ];
    classes.sort_by_key(|&(class, position)| precedence(class, position, &order));
    assert_eq!(
        classes,
        [
            (FaceClass::Script, 0),
            (FaceClass::Script, 1),
            (FaceClass::Han(Japanese), 9),
            (FaceClass::Han(Simplified), 0),
            (FaceClass::Extra, 0),
            (FaceClass::Found, 0),
        ]
    );
}

use std::path::{Path, PathBuf};
use std::sync::MutexGuard;

use super::*;
use crate::HanVariant::{Japanese, Korean, Simplified};
use crate::system::FaceClass;
use crate::{PrintFace, Role};

/// Print looks in one place per process, so the tests that move it take
/// turns, and leave it looking nowhere.
fn print_source() -> MutexGuard<'static, ()> {
    static TURN: Mutex<()> = Mutex::new(());
    TURN.lock().unwrap_or_else(PoisonError::into_inner)
}

fn fixture(file: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fonts")
        .join(file)
}

fn japanese() -> KnownFace {
    KnownFace::new(
        fixture("IBMPlexSansJP-Regular.ttf"),
        "IBM Plex Sans JP",
        FaceClass::Han(Japanese),
    )
}

fn korean_at(path: PathBuf) -> KnownFace {
    KnownFace::new(path, "IBM Plex Sans KR", FaceClass::Han(Korean))
}

/// A copy of `file` whose OS/2 table sets `fs_type`, written where the test
/// can read it as an installed face.
fn with_fs_type(file: &str, fs_type: u16) -> PathBuf {
    let mut bytes = std::fs::read(fixture(file)).unwrap();
    let tables = usize::from(u16::from_be_bytes([bytes[4], bytes[5]]));
    let os2 = (0..tables)
        .map(|table| 12 + 16 * table)
        .find(|&record| &bytes[record..record + 4] == b"OS/2")
        .map(|record| {
            u32::from_be_bytes(bytes[record + 8..record + 12].try_into().unwrap()) as usize
        })
        .expect("the face has an OS/2 table");
    bytes[os2 + 8..os2 + 10].copy_from_slice(&fs_type.to_be_bytes());
    let path = std::env::temp_dir().join(format!(
        "rspice-fonts-fs-type-{fs_type:04x}-{}-{file}",
        std::process::id()
    ));
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn print_letters_from_the_installed_faces_it_is_told_of() {
    let _turn = print_source();
    print_from_none();
    assert_eq!(
        face_for("中", &['中']),
        None,
        "print looks nowhere by default"
    );

    print_from(Source::files(vec![japanese()]), Simplified.order());
    let (id, whole) = face_for("中", &['中']).expect("the Japanese face has 中");
    assert!(whole);
    assert_eq!(id.face().family(), "IBM Plex Sans JP");
    assert_eq!(
        id.face().bytes(),
        std::fs::read(fixture("IBMPlexSansJP-Regular.ttf")).unwrap()
    );
    assert_eq!(face_for("中", &['中']), Some((id, true)), "asked once");
    assert_eq!(face_for("\u{0378}", &['\u{0378}']), None);

    let pieces = crate::print::line(Role::Sans, "R1 中文");
    let faces: Vec<PrintFace> = pieces.iter().map(|piece| piece.face).collect();
    assert_eq!(
        faces,
        [
            PrintFace::Bundled(crate::Face::SansRegular),
            PrintFace::System(id)
        ]
    );
    assert!(pieces[1].glyphs.iter().all(|glyph| glyph.id != 0));
    print_from_none();
}

/// A face whose licence keeps it out of documents is never printed from,
/// so its characters print as the square in every format.
#[test]
fn print_passes_over_a_face_it_may_not_embed() {
    let _turn = print_source();
    for (fs_type, embeddable) in [
        (0x0000, true),
        (0x0002, false),
        (0x0004, true),
        (0x0008, true),
        (0x0100, false),
        (0x0200, false),
    ] {
        let path = with_fs_type("IBMPlexSansKR-Regular.ttf", fs_type);
        print_from(Source::files(vec![korean_at(path.clone())]), Korean.order());
        assert_eq!(
            face_for("한", &['한']).is_some(),
            embeddable,
            "fsType {fs_type:#06x}"
        );
        print_from_none();
        let _ = std::fs::remove_file(path);
    }
}

#[test]
fn print_orders_the_cjk_faces_as_the_screen_does() {
    let _turn = print_source();
    let korean = korean_at(fixture("IBMPlexSansKR-Regular.ttf"));
    print_from(Source::files(vec![korean, japanese()]), Korean.order());
    let shared = '\u{3002}';
    let cluster = shared.to_string();
    let first = face_for(&cluster, &[shared]).expect("both faces have the full stop");
    assert_eq!(first.0.face().family(), "IBM Plex Sans KR");
    set_print_order(Japanese.order());
    let then = face_for(&cluster, &[shared]).expect("both faces have the full stop");
    assert_eq!(then.0.face().family(), "IBM Plex Sans JP");
    print_from_none();
}

//! The faces each operating system ships for the scripts the embedded faces
//! lack, by file and family.
//!
//! A face that has only part of a script stands after the face that has all
//! of it, or a word would be lettered in two faces: Segoe UI has a few Thai
//! and Devanagari letters, so Leelawadee UI and Nirmala UI come first. Faces
//! for rarer scripts come after the CJK faces, since several carry the CJK
//! punctuation. A file that is not installed is passed over.

use std::path::{Path, PathBuf};

use super::{FaceClass, KnownFace};
use crate::HanVariant::{HongKong, Japanese, Korean, Simplified, Traditional};

const SCRIPT: FaceClass = FaceClass::Script;
#[cfg(any(windows, target_os = "macos"))]
const EXTRA: FaceClass = FaceClass::Extra;
const SC: FaceClass = FaceClass::Han(Simplified);
const TC: FaceClass = FaceClass::Han(Traditional);
const HK: FaceClass = FaceClass::Han(HongKong);
const JP: FaceClass = FaceClass::Han(Japanese);
const KR: FaceClass = FaceClass::Han(Korean);

/// Windows 10 and 11. Hong Kong has no face of its own there, so it takes
/// Traditional's. The UI cuts are the ones Windows letters its own interface
/// in; Yu Gothic UI's Regular is in the Medium file.
#[cfg(windows)]
const FACES: &[(&str, &str, FaceClass)] = &[
    ("LeelawUI.ttf", "Leelawadee UI", SCRIPT),
    ("Nirmala.ttc", "Nirmala UI", SCRIPT),
    ("Nirmala.ttf", "Nirmala UI", SCRIPT),
    ("segoeui.ttf", "Segoe UI", SCRIPT),
    ("msyh.ttc", "Microsoft YaHei UI", SC),
    ("msjh.ttc", "Microsoft JhengHei UI", TC),
    ("msjh.ttc", "Microsoft JhengHei UI", HK),
    ("YuGothM.ttc", "Yu Gothic UI", JP),
    ("malgun.ttf", "Malgun Gothic", KR),
    ("ebrima.ttf", "Ebrima", EXTRA),
    ("gadugi.ttf", "Gadugi", EXTRA),
    ("mmrtext.ttf", "Myanmar Text", EXTRA),
    ("himalaya.ttf", "Microsoft Himalaya", EXTRA),
    ("monbaiti.ttf", "Mongolian Baiti", EXTRA),
    ("msyi.ttf", "Microsoft Yi Baiti", EXTRA),
    ("sylfaen.ttf", "Sylfaen", EXTRA),
    ("seguihis.ttf", "Segoe UI Historic", EXTRA),
    ("javatext.ttf", "Javanese Text", EXTRA),
    ("seguisym.ttf", "Segoe UI Symbol", EXTRA),
    ("seguiemj.ttf", "Segoe UI Emoji", EXTRA),
];

#[cfg(target_os = "macos")]
const FACES: &[(&str, &str, FaceClass)] = &[
    ("Thonburi.ttc", "Thonburi", SCRIPT),
    ("Kohinoor.ttc", "Kohinoor Devanagari", SCRIPT),
    ("GeezaPro.ttc", "Geeza Pro", SCRIPT),
    ("ArialHB.ttc", "Arial Hebrew", SCRIPT),
    ("PingFang.ttc", "PingFang SC", SC),
    ("PingFang.ttc", "PingFang TC", TC),
    ("PingFang.ttc", "PingFang HK", HK),
    ("ヒラギノ角ゴシック W3.ttc", "Hiragino Sans", JP),
    ("AppleSDGothicNeo.ttc", "Apple SD Gothic Neo", KR),
    ("Apple Symbols.ttf", "Apple Symbols", EXTRA),
];

/// Linux, as Debian, Fedora and Arch package Noto and DejaVu.
#[cfg(not(any(windows, target_os = "macos")))]
const FACES: &[(&str, &str, FaceClass)] = &[
    ("NotoSansThai-Regular.ttf", "Noto Sans Thai", SCRIPT),
    (
        "NotoSansDevanagari-Regular.ttf",
        "Noto Sans Devanagari",
        SCRIPT,
    ),
    ("NotoSansArabic-Regular.ttf", "Noto Sans Arabic", SCRIPT),
    ("NotoSansHebrew-Regular.ttf", "Noto Sans Hebrew", SCRIPT),
    ("DejaVuSans.ttf", "DejaVu Sans", SCRIPT),
    ("NotoSansCJK-Regular.ttc", "Noto Sans CJK SC", SC),
    ("NotoSansCJK-Regular.ttc", "Noto Sans CJK TC", TC),
    ("NotoSansCJK-Regular.ttc", "Noto Sans CJK HK", HK),
    ("NotoSansCJK-Regular.ttc", "Noto Sans CJK JP", JP),
    ("NotoSansCJK-Regular.ttc", "Noto Sans CJK KR", KR),
];

/// The directories the files above are looked for in, in order.
fn directories() -> Vec<PathBuf> {
    #[cfg(windows)]
    {
        let root = std::env::var_os("SystemRoot")
            .or_else(|| std::env::var_os("windir"))
            .map_or_else(|| PathBuf::from(r"C:\Windows"), PathBuf::from);
        vec![root.join("Fonts")]
    }
    #[cfg(target_os = "macos")]
    {
        [
            "/System/Library/Fonts",
            "/System/Library/Fonts/Supplemental",
            "/Library/Fonts",
        ]
        .map(PathBuf::from)
        .to_vec()
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        [
            "/usr/share/fonts/opentype/noto",
            "/usr/share/fonts/truetype/noto",
            "/usr/share/fonts/google-noto-cjk",
            "/usr/share/fonts/google-noto",
            "/usr/share/fonts/noto-cjk",
            "/usr/share/fonts/noto",
            "/usr/share/fonts/truetype/dejavu",
            "/usr/share/fonts/dejavu",
        ]
        .map(PathBuf::from)
        .to_vec()
    }
}

/// The installed faces of [`FACES`], each at the first directory that holds
/// its file.
pub(super) fn faces() -> Vec<KnownFace> {
    let directories = directories();
    FACES
        .iter()
        .filter_map(|&(file, family, class)| {
            let path = directories
                .iter()
                .map(|directory| directory.join(file))
                .find(|path| Path::is_file(path))?;
            Some(KnownFace::new(path, family, class))
        })
        .collect()
}

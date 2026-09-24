//! The real on-disk identity a filesystem fixture must build under.
//!
//! [`std::env::temp_dir`] reports whatever `TMPDIR`/`TMP`/`TEMP` names, and on
//! several hosts that is an alias rather than the directory's own path. macOS
//! hands out `/var/folders/...` for a tree that really lives under
//! `/private/var`, and the GitHub Windows runner image points `%TEMP%` at an
//! 8.3 alias, `C:\Users\RUNNER~1\AppData\Local\Temp`. A developer profile such
//! as `C:\Users\James\AppData\Local\Temp` needs neither substitution, so there
//! the alias and the real path are the same string.
//!
//! The difference stays invisible until one fixture hands a path to code that
//! records it through [`std::fs::canonicalize`] -- which resolves both the
//! symlink and the short name -- and a sibling path to code that compares
//! recorded identities as text. The two then disagree on every host whose
//! temporary directory is aliased and agree on every host whose is not, so the
//! test reports the host it ran on rather than the subject it names.
//!
//! Fixtures build their roots here so that only one identity is ever in play.

use std::path::PathBuf;

/// [`std::env::temp_dir`] resolved to the path the filesystem itself reports.
///
/// The Windows verbatim prefix that [`std::fs::canonicalize`] prepends is
/// dropped. It carries no identity of its own, and keeping it would rewrite
/// every fixture path on the one host where the resolution is otherwise a
/// no-op, which is the host these fixtures are usually run on.
///
/// An unresolvable temporary directory is returned as reported: a fixture that
/// cannot create its root should fail on that, not here.
pub(crate) fn canonical_temp_dir() -> PathBuf {
    let reported = std::env::temp_dir();
    std::fs::canonicalize(&reported).map_or(reported, without_verbatim_prefix)
}

#[cfg(windows)]
fn without_verbatim_prefix(path: PathBuf) -> PathBuf {
    let Some(text) = path.to_str() else {
        return path;
    };
    if let Some(share) = text.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{share}"));
    }
    // Only a drive-letter path is safe to shorten. Device paths reached
    // through `\\?\` have no equivalent spelling without it.
    match text.strip_prefix(r"\\?\") {
        Some(drive)
            if drive
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
                && drive.get(1..3) == Some(":\\") =>
        {
            PathBuf::from(drive)
        }
        _ => path,
    }
}

#[cfg(not(windows))]
fn without_verbatim_prefix(path: PathBuf) -> PathBuf {
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole point: a fixture root and the same path put through
    /// `canonicalize` have to be the one identity, whatever the host calls its
    /// temporary directory.
    #[test]
    fn a_fixture_root_survives_canonicalization_unchanged() {
        let root = canonical_temp_dir().join(format!(
            "rspice-fixture-root-{}-{}",
            std::process::id(),
            line!()
        ));
        std::fs::create_dir_all(&root).expect("create fixture root");
        let recorded = std::fs::canonicalize(&root).expect("record fixture root");

        assert_eq!(without_verbatim_prefix(recorded), root);

        std::fs::remove_dir_all(&root).expect("remove fixture root");
    }

    #[cfg(windows)]
    #[test]
    fn only_a_drive_letter_path_is_shortened() {
        assert_eq!(
            without_verbatim_prefix(PathBuf::from(r"\\?\C:\Users\example")),
            PathBuf::from(r"C:\Users\example")
        );
        assert_eq!(
            without_verbatim_prefix(PathBuf::from(r"\\?\UNC\server\share\example")),
            PathBuf::from(r"\\server\share\example")
        );
        assert_eq!(
            without_verbatim_prefix(PathBuf::from(
                r"\\?\Volume{00000000-0000-0000-0000-000000000000}\example"
            )),
            PathBuf::from(r"\\?\Volume{00000000-0000-0000-0000-000000000000}\example")
        );
    }
}

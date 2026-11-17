//! The recently opened projects and schematics the session remembers.

use std::path::Path;

/// What kind of document a recent-files entry points at. `.json` is a valid
/// extension for both schematics and projects, so the kind is recorded at
/// open/save time instead of being inferred from the path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RecentKind {
    Schematic,
    Project,
}

/// One entry in the File ▸ Open recent list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecentFile {
    pub kind: RecentKind,
    pub path: std::path::PathBuf,
    /// Milliseconds since the Unix epoch when this entry most recently
    /// became active. Zero when the time was never recorded.
    #[serde(default)]
    pub opened_at_unix_ms: u64,
    /// Pinned by the reader: listed under Pinned and never trimmed off the
    /// list by the Recent projects listed count.
    #[serde(default)]
    pub pinned: bool,
    /// An open this launch found the file gone. Runtime only: the drive may
    /// come back, and the next open that finds the file clears it.
    #[serde(skip)]
    pub missing: bool,
}

/// Pin `path`'s entry, or unpin it when it is pinned. Returns whether an
/// entry was found.
pub fn toggle_pin(recent_files: &mut [RecentFile], path: &Path) -> bool {
    let Some(recent) = recent_files.iter_mut().find(|recent| recent.path == path) else {
        return false;
    };
    recent.pinned = !recent.pinned;
    true
}

/// Mark whether an open just found `path`'s file. Returns whether the mark
/// moved.
pub fn mark_missing(recent_files: &mut [RecentFile], path: &Path, missing: bool) -> bool {
    let Some(recent) = recent_files.iter_mut().find(|recent| recent.path == path) else {
        return false;
    };
    let moved = recent.missing != missing;
    recent.missing = missing;
    moved
}

/// Point `from`'s entry at `to`, where the reader found the file: it keeps
/// its place and its pin, and an entry already listed for `to` gives way.
/// Returns the entry as it now stands, or `None` when `from` is not listed.
pub fn relocate(recent_files: &mut Vec<RecentFile>, from: &Path, to: &Path) -> Option<RecentFile> {
    if from != to {
        recent_files.retain(|recent| recent.path != to);
    }
    let recent = recent_files.iter_mut().find(|recent| recent.path == from)?;
    recent.path = to.to_path_buf();
    recent.missing = false;
    Some(recent.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(path: &str) -> RecentFile {
        RecentFile {
            kind: RecentKind::Project,
            path: path.into(),
            opened_at_unix_ms: 0,
            pinned: false,
            missing: false,
        }
    }

    #[test]
    fn a_pin_survives_the_session_and_a_missing_mark_does_not() {
        let mut list = vec![project("C:/Engineering/afe.rspiceproj")];
        assert!(toggle_pin(
            &mut list,
            Path::new("C:/Engineering/afe.rspiceproj")
        ));
        assert!(mark_missing(
            &mut list,
            Path::new("C:/Engineering/afe.rspiceproj"),
            true
        ));

        let stored = serde_json::to_string(&list[0]).expect("serializes");
        let restored: RecentFile = serde_json::from_str(&stored).expect("restores");
        assert!(restored.pinned);
        assert!(!restored.missing);
        assert!(!stored.contains("missing"), "{stored}");
    }

    #[test]
    fn a_located_entry_keeps_its_place_and_pin_and_replaces_a_duplicate() {
        let mut list = vec![
            project("C:/a.rspiceproj"),
            project("E:/moved/b.rspiceproj"),
            project("D:/b.rspiceproj"),
        ];
        list[2].pinned = true;
        list[2].missing = true;

        let located = relocate(
            &mut list,
            Path::new("D:/b.rspiceproj"),
            Path::new("E:/moved/b.rspiceproj"),
        )
        .expect("listed");

        assert_eq!(located.path, Path::new("E:/moved/b.rspiceproj"));
        assert!(located.pinned);
        assert!(!located.missing);
        assert_eq!(
            list.iter()
                .map(|recent| recent.path.clone())
                .collect::<Vec<_>>(),
            [
                std::path::PathBuf::from("C:/a.rspiceproj"),
                std::path::PathBuf::from("E:/moved/b.rspiceproj"),
            ]
        );
    }
}

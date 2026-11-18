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

/// Where a browser keeps a project it can open again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserStore {
    /// A file the reader picked: the browser keeps a handle to it.
    File,
    /// The browser's own storage for this site.
    Storage,
}

impl BrowserStore {
    const fn segment(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Storage => "storage",
        }
    }

    /// What the list's Location column says of a project kept here: the
    /// browser does not say which folder a picked file is in.
    pub const fn location(self) -> &'static str {
        match self {
            Self::File => "File on this device",
            Self::Storage => "This browser's storage",
        }
    }
}

/// A project a browser keeps has no path: the browser keeps its file handle
/// under a binding id. Its recent entry is listed under a path of its own,
/// `browser/<store>/<binding>/<file name>`, which names no file on a desktop,
/// so the list keys, pins and removes it as it does any other entry, and
/// names it by its file name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserRecent {
    pub store: BrowserStore,
    pub binding_id: uuid::Uuid,
    pub file_name: String,
}

impl BrowserRecent {
    const ROOT: &'static str = "browser";

    /// The path the entry is listed under.
    pub fn path(&self) -> std::path::PathBuf {
        [
            Self::ROOT,
            self.store.segment(),
            &self.binding_id.to_string(),
            &self.file_name,
        ]
        .iter()
        .collect()
    }

    /// The browser project `path` lists, if it lists one.
    pub fn of(path: &Path) -> Option<Self> {
        let mut parts = path.components().map(|part| part.as_os_str().to_str());
        if parts.next()? != Some(Self::ROOT) {
            return None;
        }
        let store = match parts.next()?? {
            "file" => BrowserStore::File,
            "storage" => BrowserStore::Storage,
            _ => return None,
        };
        let binding_id = parts.next()??.parse().ok()?;
        let file_name = parts.next()??.to_owned();
        parts.next().is_none().then_some(Self {
            store,
            binding_id,
            file_name,
        })
    }
}

/// Where a recent entry's file is, as its row says it: its path, or, for a
/// project a browser keeps, where the browser keeps it.
pub fn location(recent: &RecentFile) -> String {
    BrowserRecent::of(&recent.path).map_or_else(
        || recent.path.display().to_string(),
        |browser| browser.store.location().to_owned(),
    )
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

    /// A browser project's entry names its file and where the browser keeps
    /// it, and reads back to the binding it reopens through.
    #[test]
    fn a_browser_project_is_listed_by_its_file_name_and_where_the_browser_keeps_it() {
        let binding_id = uuid::Uuid::from_u128(0x5eed);
        for (store, location) in [
            (BrowserStore::File, "File on this device"),
            (BrowserStore::Storage, "This browser's storage"),
        ] {
            let browser = BrowserRecent {
                store,
                binding_id,
                file_name: "amp v2.rspiceproj".to_owned(),
            };
            let recent = project(&browser.path().display().to_string());
            assert_eq!(BrowserRecent::of(&recent.path), Some(browser));
            assert_eq!(super::location(&recent), location);
            assert_eq!(
                recent.path.file_stem().and_then(|stem| stem.to_str()),
                Some("amp v2")
            );
        }
        assert_eq!(
            BrowserRecent::of(Path::new("C:/browser/amp.rspiceproj")),
            None
        );
        assert_eq!(
            BrowserRecent::of(Path::new("browser/file/not-an-id/amp.rspiceproj")),
            None
        );
        assert_eq!(
            super::location(&project("C:/Engineering/afe.rspiceproj")),
            Path::new("C:/Engineering/afe.rspiceproj")
                .display()
                .to_string()
        );
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

//! The recently opened projects and schematics the session remembers.

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
    /// became active. Legacy sessions deserialize to zero and retain their
    /// stored list order.
    #[serde(default)]
    pub opened_at_unix_ms: u64,
    /// User pin state for Project Launcher filtering.
    #[serde(default)]
    pub pinned: bool,
    /// Governed project owner. This is never guessed from the operating-
    /// system account when project metadata does not provide it.
    #[serde(default)]
    pub owner: Option<String>,
    /// Searchable project classifications retained by the launcher.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl RecentFile {
    #[cfg(test)]
    pub(crate) fn is_shared_project(&self) -> bool {
        self.kind == RecentKind::Project && path_is_shared(&self.path)
    }
}

#[cfg(test)]
fn path_is_shared(path: &std::path::Path) -> bool {
    let text = path.as_os_str().to_string_lossy();
    text.starts_with(r"\\") || text.starts_with("//")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_metadata_is_backward_compatible_and_shared_paths_are_detected() {
        let legacy = r#"{"kind":"Project","path":"C:/Engineering/afe.rspiceproj"}"#;
        let recent: RecentFile = serde_json::from_str(legacy).expect("legacy recent entry loads");
        assert_eq!(recent.opened_at_unix_ms, 0);
        assert!(!recent.pinned);
        assert!(recent.owner.is_none());
        assert!(recent.tags.is_empty());
        assert!(!recent.is_shared_project());

        let shared: RecentFile =
            serde_json::from_str(r#"{"kind":"Project","path":"\\\\server\\share\\rf.rspiceproj"}"#)
                .expect("UNC recent entry loads");
        assert!(shared.is_shared_project());
    }
}

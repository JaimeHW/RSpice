//! The project's ordered include search chain.
//!
//! One owner decides two things the rest of the application must not decide
//! twice: how the persisted entries become host directories, and which engine
//! entry point a host-file parse goes through. Every surface that resolves a
//! `.include`, `.inc` or `.lib` against the filesystem — the live editor, the
//! prepared-run expansion that seals a deck's dependencies, and the engine
//! bridge — walks this chain, so a relative name can never resolve one way in
//! the navigator and another way in the run.

use std::path::{Path, PathBuf};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{IncludeProcessor, NetlistParseOptions, ParseWithAbortError};

/// One persisted entry, resolved against the project and checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeSearchEntry {
    authored: PathBuf,
    resolved: PathBuf,
    exists: bool,
}

impl IncludeSearchEntry {
    /// The entry exactly as the project persists it.
    pub fn authored(&self) -> &Path {
        &self.authored
    }

    /// The host directory the entry resolves to.
    pub fn resolved(&self) -> &Path {
        &self.resolved
    }

    /// Whether that directory is present on this host.
    pub const fn exists(&self) -> bool {
        self.exists
    }
}

/// The project's ordered include search chain, resolved to host directories.
///
/// A relative entry is relative to the project's data root, so a moved project
/// folder keeps resolving; an absolute entry is a host decision and is taken as
/// written. A project that has never been saved has no data root, so a relative
/// entry cannot be resolved and is reported as missing rather than guessed at
/// against the process working directory.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IncludeSearchChain {
    entries: Vec<IncludeSearchEntry>,
}

impl IncludeSearchChain {
    /// Resolve the persisted entries against a project data root.
    #[must_use]
    pub fn resolve(authored: &[PathBuf], data_root: Option<&Path>) -> Self {
        Self {
            entries: authored
                .iter()
                .map(|entry| {
                    let placed = entry.is_absolute() || data_root.is_some();
                    let resolved = match data_root {
                        Some(root) if !entry.is_absolute() => root.join(entry),
                        _ => entry.clone(),
                    };
                    let exists = placed && directory_exists(&resolved);
                    IncludeSearchEntry {
                        authored: entry.clone(),
                        resolved,
                        exists,
                    }
                })
                .collect(),
        }
    }

    /// The chain a project's includes resolve through: the project's own
    /// entries in its order, then the library folders this host adds, in
    /// theirs. The project's come first, so where both hold a name the
    /// project's file wins.
    ///
    /// Every surface that resolves a project's includes — a run, a check, a
    /// configuration's seal and the live editor — builds its chain here, so
    /// no two of them can walk different ones.
    ///
    /// A library folder belongs to the host, not the project, so only one
    /// named by a full path is searched. One the chain already holds is not
    /// added again, so the engine numbers its search paths as the chain does.
    #[must_use]
    pub fn for_project(
        project: &rspice_project_contract::ProjectDescriptor,
        library_folders: &[PathBuf],
    ) -> Self {
        let mut chain = Self::resolve(project.include_search_paths(), project.data_root());
        for folder in library_folders {
            if !folder.is_absolute() || chain.entries.iter().any(|entry| entry.resolved == *folder)
            {
                continue;
            }
            chain.entries.push(IncludeSearchEntry {
                authored: folder.clone(),
                resolved: folder.clone(),
                exists: directory_exists(folder),
            });
        }
        chain
    }

    /// Every entry, in the order the resolver walks them.
    #[must_use]
    pub fn entries(&self) -> &[IncludeSearchEntry] {
        &self.entries
    }

    /// Whether this host can say anything about a search directory's presence.
    ///
    /// The browser has no host filesystem, so a row there states the chain's
    /// order and stops rather than calling every entry missing.
    #[must_use]
    pub const fn states_presence() -> bool {
        cfg!(not(target_arch = "wasm32"))
    }

    /// The host directories, in order, as the engine consumes them.
    #[must_use]
    pub fn directories(&self) -> Vec<PathBuf> {
        self.entries
            .iter()
            .map(|entry| entry.resolved.clone())
            .collect()
    }

    /// Seed an include processor with this chain, in order.
    pub fn apply_to(&self, processor: &mut IncludeProcessor) {
        for entry in &self.entries {
            processor.add_lib_path(entry.resolved.clone());
        }
    }

    /// Parse a host-backed deck, choosing the search-path entry point exactly
    /// when the project states a chain.
    ///
    /// This is the only place that choice is made. A parse without a source
    /// path resolves nothing from the host and never reaches the chain.
    pub fn parse_with_abort(
        &self,
        input: &str,
        source_path: Option<&Path>,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<rspice_core::Netlist, ParseWithAbortError> {
        match source_path {
            Some(path) if !self.entries.is_empty() => {
                rspice_core::Netlist::parse_with_search_paths_and_options_and_abort(
                    input,
                    path,
                    &self.directories(),
                    options,
                    abort,
                )
            }
            Some(path) => rspice_core::Netlist::parse_with_path_and_options_and_abort(
                input, path, options, abort,
            ),
            None => rspice_core::Netlist::parse_with_options_and_abort(input, options, abort),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn directory_exists(_path: &Path) -> bool {
    // The browser build has no host filesystem to state anything about.
    false
}

#[cfg(not(target_arch = "wasm32"))]
fn directory_exists(path: &Path) -> bool {
    path.is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_project_with_no_data_root_cannot_place_a_relative_entry() {
        let chain = IncludeSearchChain::resolve(&[PathBuf::from("models")], None);

        assert_eq!(chain.entries()[0].resolved(), Path::new("models"));
        assert!(
            !chain.entries()[0].exists(),
            "an unsaved project must not resolve a relative entry against the process directory"
        );
    }

    #[test]
    fn library_folders_follow_the_project_paths_and_are_searched_once() {
        let root = std::env::temp_dir().join("rspice-include-search-project");
        let mut project = rspice_project_contract::ProjectDescriptor::default();
        project
            .set_include_search_paths(vec![PathBuf::from("models")])
            .expect("a relative project path");
        project.path = Some(root.join("design.rspice"));
        let library = std::env::temp_dir().join("rspice-include-search-library");

        let chain = IncludeSearchChain::for_project(
            &project,
            &[
                library.clone(),
                PathBuf::from("relative"),
                library.clone(),
                root.join("models"),
            ],
        );

        assert_eq!(chain.directories(), vec![root.join("models"), library]);
    }

    #[test]
    fn an_empty_chain_parses_through_the_ordinary_path_entry_point() {
        let chain = IncludeSearchChain::default();
        assert!(chain.entries().is_empty());
        assert!(chain.directories().is_empty());
    }
}

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

    /// Every entry, in the order the resolver walks them.
    #[must_use]
    pub fn entries(&self) -> &[IncludeSearchEntry] {
        &self.entries
    }

    /// Whether the project states no search chain at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
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
    use crate::state::ProjectDescriptor;
    use crate::state::workspace::ProjectDescriptorError;

    #[test]
    fn include_search_paths_round_trip_and_default_to_empty() {
        let mut project = ProjectDescriptor::default();
        assert!(project.include_search_paths().is_empty());

        // Absent from a stored descriptor is the empty chain, not a failure.
        let stored = serde_json::to_value(&project).expect("descriptor serializes");
        assert!(
            stored.get("include_search_paths").is_none(),
            "an empty chain must not be written into the project file"
        );
        let restored: ProjectDescriptor =
            serde_json::from_value(stored).expect("descriptor without a chain restores");
        assert!(restored.include_search_paths().is_empty());

        project
            .set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")])
            .expect("an ordered chain of distinct directories is accepted");
        let wire = serde_json::to_value(&project).expect("descriptor serializes");
        let restored: ProjectDescriptor =
            serde_json::from_value(wire).expect("descriptor with a chain restores");
        assert_eq!(
            restored.include_search_paths(),
            [PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")],
            "order is the setting and must survive the round trip"
        );
        restored.validate().expect("a persisted chain validates");
    }

    #[test]
    fn a_chain_rejects_an_empty_or_repeated_entry() {
        let mut project = ProjectDescriptor::default();
        assert!(matches!(
            project.set_include_search_paths(vec![PathBuf::from("  ")]),
            Err(ProjectDescriptorError::EmptyIncludeSearchPath)
        ));
        assert!(matches!(
            project
                .set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("models")]),
            Err(ProjectDescriptorError::DuplicateIncludeSearchPath(_))
        ));
        assert!(project.include_search_paths().is_empty());
    }

    #[test]
    fn a_project_places_its_relative_entries_against_its_own_folder() {
        let mut project = ProjectDescriptor::default();
        project.path = Some(PathBuf::from("/projects/mixer/mixer.rspiceproj"));
        project
            .set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")])
            .expect("chain is accepted");

        let directories = project.include_search_chain().directories();
        assert_eq!(directories.len(), 2);
        assert_eq!(directories[0], Path::new("/projects/mixer").join("models"));
        assert_eq!(directories[1], PathBuf::from("/opt/pdk/lib"));
    }

    #[test]
    fn relative_entries_resolve_against_the_project_data_root() {
        let root = crate::fixture_root::canonical_temp_dir().join(format!(
            "rspice-include-chain-relative-{}",
            std::process::id()
        ));
        let models = root.join("models");
        std::fs::create_dir_all(&models).expect("create chain fixture");

        let chain = IncludeSearchChain::resolve(
            &[PathBuf::from("models"), PathBuf::from("absent")],
            Some(root.as_path()),
        );

        assert_eq!(chain.entries().len(), 2);
        assert_eq!(chain.entries()[0].resolved(), models.as_path());
        assert!(chain.entries()[0].exists());
        assert_eq!(chain.entries()[1].resolved(), root.join("absent"));
        assert!(!chain.entries()[1].exists());
        std::fs::remove_dir_all(&root).ok();
    }

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
    fn an_empty_chain_parses_through_the_ordinary_path_entry_point() {
        let chain = IncludeSearchChain::default();
        assert!(chain.is_empty());
        assert!(chain.directories().is_empty());
    }

    /// The choice of engine entry point lives here and nowhere else, so it is
    /// asserted by its effect: the same deck refuses to resolve without a
    /// chain and resolves through it with one.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_chain_decides_which_engine_entry_point_a_host_parse_takes() {
        let root = crate::fixture_root::canonical_temp_dir().join(format!(
            "rspice-include-chain-entry-point-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ));
        let deck_dir = root.join("deck");
        let library_dir = root.join("pdk");
        std::fs::create_dir_all(&deck_dir).expect("create deck directory");
        std::fs::create_dir_all(&library_dir).expect("create library directory");
        std::fs::write(
            library_dir.join("device.lib"),
            ".model DCHAIN D(Is=1e-14)\n",
        )
        .expect("write library");
        let deck_path = deck_dir.join("top.cir");
        let source = "chain entry point\n.include device.lib\nD1 out 0 DCHAIN\n.end\n";
        let options = rspice_core::netlist::NetlistParseOptions::default();

        let without = IncludeSearchChain::default().parse_with_abort(
            source,
            Some(&deck_path),
            options,
            &rspice_core::abort_signal::NoAbort,
        );
        assert!(
            without.is_err(),
            "without a chain the include is nowhere the resolver looks"
        );

        let chain = IncludeSearchChain::resolve(&[library_dir.clone()], Some(root.as_path()));
        let with = chain
            .parse_with_abort(
                source,
                Some(&deck_path),
                options,
                &rspice_core::abort_signal::NoAbort,
            )
            .expect("the chain resolves the include");
        assert_eq!(with.source_path.as_deref(), Some(deck_path.as_path()));

        std::fs::remove_dir_all(&root).ok();
    }
}

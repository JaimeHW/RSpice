//! Editor scopes around design-owned sheet topology.
//!
//! All sheets of a cell view live in one document and share its coordinates.
//! An operation that scans every wire -- splitting at a T, maintaining
//! junctions, cleaning up, rubber-banding, stretching -- would reach into the
//! sheets the user cannot see. So each of those operations demands an
//! [`ActiveSheetTopology`], and the only way to hold one is inside
//! [`SchematicState::with_active_wire_topology`], with the other sheets'
//! conductors already out of reach.
use crate::state::SchematicState;

/// Proof that the cell view's other sheets are out of reach.
///
/// Only [`SchematicState::with_active_wire_topology`] makes one, and it lends
/// it to its closure, which cannot keep it.
#[derive(Debug)]
pub struct ActiveSheetTopology(());

impl SchematicState {
    /// Run `operation` with the conductors `hidden` names taken out of the
    /// document, then put them back exactly where they were.
    pub(crate) fn with_active_wire_topology<R>(
        &mut self,
        hidden: impl Fn(u64) -> bool,
        operation: impl FnOnce(&mut SchematicState, &ActiveSheetTopology) -> R,
    ) -> R {
        let hidden = self.design.take_hidden_wire_topology(hidden);
        let result = operation(self, &ActiveSheetTopology(()));
        hidden.restore(&mut self.design);
        result
    }

    /// A document with one sheet hides nothing; tests build those.
    #[cfg(test)]
    pub(crate) fn with_single_sheet_topology<R>(
        &mut self,
        operation: impl FnOnce(&mut SchematicState, &ActiveSheetTopology) -> R,
    ) -> R {
        self.with_active_wire_topology(|_| false, operation)
    }
    pub(crate) fn with_hidden_wire_topology_preserved<R>(
        &mut self,
        hidden: impl Fn(u64) -> bool,
        operation: impl FnOnce(&mut SchematicState) -> R,
    ) -> R {
        let hidden = self.design.capture_hidden_wire_topology(hidden);
        let result = operation(self);
        hidden.restore(&mut self.design);
        result
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    /// Every editor operation that scans the whole document's wires.
    const WHOLE_DOCUMENT_OPERATIONS: &[&str] = &[
        "split_wires_at_t_junction",
        "split_wires_under_pins",
        "merge_overlapping_wires",
        "update_wire_junctions",
        "auto_place_junctions",
        "remove_orphan_junctions",
        "optimize_all_wires",
        "remove_degenerate_segments",
        "cleanup_wire_topology",
        "cleanup_wire_topology_with_junction_policy",
        "move_all_vertices_at",
        "move_junction",
        "move_selection_with_rubber_band",
        "move_selection_with_rubber_band_resolved",
        "move_selection_with_mode",
        "move_selection_with_mode_resolved",
        "move_component_with_wires_resolved",
        "stretch_target",
        "stretch_target_resolved",
        "transform_selection",
    ];

    /// Where a production caller may open a scope: this module; the one
    /// authority on sheet membership; the route tool, which hands that
    /// authority's predicate through; an example circuit, which replaces the
    /// whole document.
    const SCOPE_OPENERS: &[&str] = &[
        "state/schematic/state/editor_ops/sheet_topology.rs",
        "schematic/view/sheet_visibility.rs",
        "state/schematic/state/editor_ops/wire_management.rs",
        "workbench/examples.rs",
    ];

    fn sources(directory: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(directory).expect("read source directory") {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                sources(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }

    /// Each shipped source of this crate, as a path under `src/` with `/`
    /// separators, and its text with test-only items blanked.
    fn shipped_sources() -> Vec<(String, String)> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut paths = Vec::new();
        sources(&root, &mut paths);
        assert!(paths.len() > 500, "the scan reached {} files", paths.len());
        paths
            .into_iter()
            .map(|path| {
                let relative = path
                    .strip_prefix(&root)
                    .expect("under src")
                    .to_string_lossy()
                    .replace('\\', "/");
                let text = std::fs::read_to_string(&path).expect("read source");
                (relative, crate::source_guard::without_test_items(&text))
            })
            .collect()
    }

    /// The parameter list of `fn name`, wherever this source declares it.
    fn signatures<'a>(source: &'a str, name: &str) -> Vec<&'a str> {
        let mut found = Vec::new();
        for pattern in [format!("fn {name}("), format!("fn {name}<")] {
            for (start, _) in source.match_indices(&pattern) {
                let rest = &source[start..];
                let end = rest.find('{').unwrap_or(rest.len());
                found.push(&rest[..end]);
            }
        }
        found
    }

    /// All sheets of a cell view share one document, so an operation over
    /// every wire reaches the sheets nobody is looking at unless the other
    /// sheets are out of reach first. Each of those operations therefore
    /// demands the proof that they are, and only the sheet scope makes it.
    #[test]
    fn whole_document_wire_operations_demand_a_sheet_scope() {
        let shipped = shipped_sources();
        let mut bare = Vec::new();
        for name in WHOLE_DOCUMENT_OPERATIONS {
            let declarations = shipped
                .iter()
                .flat_map(|(path, source)| {
                    signatures(source, name)
                        .into_iter()
                        .map(move |signature| (path, signature))
                })
                .collect::<Vec<_>>();
            assert!(!declarations.is_empty(), "`{name}` is declared nowhere");
            for (path, signature) in declarations {
                if !signature.contains("ActiveSheetTopology") {
                    bare.push(format!("{path}: fn {name} takes no sheet scope"));
                }
            }
        }
        for (path, source) in &shipped {
            if source.contains("ActiveSheetTopology(())")
                && path != "state/schematic/state/editor_ops/sheet_topology.rs"
            {
                bare.push(format!("{path}: mints a sheet scope"));
            }
            if source.contains(".take_hidden_wire_topology(")
                && path != "state/schematic/state/editor_ops/sheet_topology.rs"
            {
                bare.push(format!("{path}: hides other sheets by hand"));
            }
            if source.contains(".with_active_wire_topology(")
                && !SCOPE_OPENERS.contains(&path.as_str())
            {
                bare.push(format!("{path}: opens a sheet scope of its own"));
            }
        }
        assert!(bare.is_empty(), "{}", bare.join("\n"));
    }
}

//! Per-document occurrences and navigation projections.

use super::*;
use rspice_design::occurrence::{DocumentOccurrence, OccurrencePrune};

impl ProjectWorkspace {
    pub fn active_key(&self) -> String {
        self.active_view.key()
    }

    pub fn active_display_path(&self) -> String {
        self.active_view.display_path()
    }

    pub fn active_view_type(&self) -> ViewType {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| open.view_type)
            .unwrap_or(ViewType::Schematic)
    }

    pub fn active_schematic_reference(&self) -> CellViewRef {
        if self.active_view_type() == ViewType::Symbol {
            return CellViewRef::new(
                &self.active_view.library,
                &self.active_view.cell,
                DEFAULT_SCHEMATIC_VIEW,
            );
        }
        self.active_view.clone()
    }

    /// The occurrence the active document is editing.
    pub fn active_occurrence(&self) -> Option<&DocumentOccurrence> {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| &open.occurrence)
    }

    /// The active document's occurrence, or the root occurrence its reference
    /// implies while no document claims it.
    pub fn active_occurrence_or_root(&self) -> DocumentOccurrence {
        self.active_occurrence()
            .cloned()
            .unwrap_or_else(|| DocumentOccurrence::rooted(self.active_view.clone()))
    }

    pub fn set_active_occurrence(&mut self, occurrence: DocumentOccurrence) {
        let active = self.active_view.clone();
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == active)
        {
            occurrence.debug_assert_opens(&open.reference);
            open.occurrence = occurrence;
        }
        self.project_active_occurrence();
    }

    /// Root every document that carries no occurrence at its own reference.
    ///
    /// This is the one repair for a tab record written before documents owned
    /// an occurrence, and it never invents a step: a document restored without
    /// one is a root, not a guessed descent.
    fn root_unrooted_occurrences(&mut self) {
        for open in &mut self.open_views {
            if open.occurrence.is_unrooted() || open.occurrence.terminal_master() != &open.reference
            {
                open.occurrence = DocumentOccurrence::rooted(open.reference.clone());
            }
        }
    }

    /// Refresh the session-global breadcrumb from the active document.
    ///
    /// The two vectors are a read-only projection for surfaces that have not
    /// moved onto the per-document occurrence yet; the occurrence on the open
    /// document is the authority, and this is the only writer.
    pub fn project_active_occurrence(&mut self) {
        let occurrence = self.active_occurrence_or_root();
        self.hierarchy_stack = occurrence.masters().cloned().collect();
        self.hierarchy_instances = occurrence
            .steps
            .iter()
            .map(|step| step.instance_name.clone())
            .collect();
    }

    /// Publish prepared instance-name changes while retaining each tab's
    /// chosen root and master. Breadcrumbs derive from the resulting records.
    pub(crate) fn replace_document_occurrences(
        &mut self,
        occurrences: Vec<(CellViewRef, DocumentOccurrence)>,
    ) {
        if occurrences.is_empty() {
            return;
        }
        for (reference, occurrence) in occurrences {
            if let Some(open) = self
                .open_views
                .iter_mut()
                .find(|open| open.reference == reference)
            {
                open.occurrence = occurrence;
            }
        }
        self.project_active_occurrence();
    }

    /// Display labels for the active occurrence: the root cell, then the
    /// instance descended through at each level.
    #[cfg(any(test, feature = "test-support"))]
    pub fn occurrence_labels(&self) -> Vec<String> {
        self.active_occurrence_or_root().labels()
    }

    /// The occurrence the active document is editing, as an instance path.
    pub fn occurrence_path(&self) -> rspice_vocabulary::hierarchy_path::InstancePath {
        self.active_occurrence_or_root().instance_path()
    }

    /// Levels on the active occurrence, counting the design root.
    pub fn occurrence_depth(&self) -> usize {
        self.active_occurrence_or_root().depth()
    }

    /// Whether the active document was opened as a read-only hierarchy
    /// reference.
    pub fn active_read_only_reference(&self) -> bool {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .is_some_and(|open| open.read_only_reference)
    }

    pub fn set_active_read_only_reference(&mut self, read_only: bool) {
        let active = self.active_view.clone();
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == active)
        {
            open.read_only_reference = read_only;
        }
    }

    /// Re-root the active document's occurrence at the document itself —
    /// what a prune leaves behind once whatever it was reached through is
    /// gone.
    pub fn reroot_active_occurrence(&mut self) {
        let reference = self.active_view.clone();
        self.set_active_occurrence(DocumentOccurrence::rooted(reference));
    }

    /// Prune every open document's occurrence to what still exists.
    ///
    /// A document whose root master is gone closes; one that passes through a
    /// master that is gone keeps the deepest prefix still entirely valid and
    /// re-targets onto that prefix's terminal master, because an occurrence
    /// step is only ever created by descending into a schematic. Nothing is
    /// invented to fill a gap. Returns whether any occurrence changed.
    pub fn retain_valid_occurrences(&mut self, is_valid: impl Fn(&CellViewRef) -> bool) -> bool {
        let mut pruned = false;
        self.open_views.retain_mut(
            |open| match open.occurrence.retain_valid_prefix(&is_valid) {
                OccurrencePrune::Intact => true,
                OccurrencePrune::Truncated => {
                    open.reference = open.occurrence.terminal_master().clone();
                    open.view_type = ViewType::Schematic;
                    pruned = true;
                    true
                }
                OccurrencePrune::Rootless => {
                    pruned = true;
                    false
                }
            },
        );
        // A document re-targeted onto a master another tab already shows is
        // the same document twice; the first one keeps it.
        let mut seen = HashSet::new();
        self.open_views
            .retain(|open| seen.insert(open.reference.key()));
        if !self
            .open_views
            .iter()
            .any(|open| open.reference == self.active_view)
            && let Some(next) = self.open_views.first()
        {
            self.active_view = next.reference.clone();
        }
        self.project_active_occurrence();
        pruned
    }

    /// Rewrite the masters a library, cell, or view rename moved, on every
    /// open document's occurrence. Callers remap `active_view` and each
    /// document's `reference` first, so the terminal-master invariant holds
    /// across the whole transaction.
    pub fn remap_occurrence_masters(&mut self, mut remap: impl FnMut(&mut CellViewRef)) {
        for open in &mut self.open_views {
            for master in open.occurrence.masters_mut() {
                remap(master);
            }
            open.occurrence.debug_assert_opens(&open.reference);
        }
        self.project_active_occurrence();
    }

    /// Root any unrooted document and refresh the breadcrumb projection from
    /// the active document. Restoring a document never re-targets which one
    /// is active.
    pub fn restore_occurrences(&mut self) {
        self.root_unrooted_occurrences();
        self.project_active_occurrence();
    }

    pub fn close_view(&mut self, reference: &CellViewRef) {
        if self.open_views.len() <= 1 {
            return;
        }

        self.open_views.retain(|open| &open.reference != reference);
        if &self.active_view == reference
            && let Some(next) = self.open_views.last().cloned()
        {
            self.active_view = next.reference;
        }
        self.project_active_occurrence();
    }

    pub fn set_active_dirty(&mut self, dirty: bool) {
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == self.active_view)
        {
            open.dirty = dirty;
        }
    }
}

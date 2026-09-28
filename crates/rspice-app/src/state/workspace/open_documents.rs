//! The open workspace: what is active, what is dirty, what the hierarchy
//! resolves to.
//!
//! Navigation and buffer management over [`ProjectWorkspace`] — opening a
//! cell view, keeping its schematic buffer in sync, resolving the design
//! hierarchy against the library, and the per-project hardcopy setup that
//! rides along with it.
//!
//! Separate from the configuration and plan half in the parent, which
//! describes what a run *will* do; this describes what a session currently
//! has open.

use super::*;

impl ProjectWorkspace {
    /// Resolve the exact root schematic while projecting the live editor only
    /// when it is the selected root. A different open tab can never silently
    /// replace the configuration's simulation source.
    pub(crate) fn simulation_root_schematic<'a>(
        &'a self,
        active_reference: &CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> Option<SchematicEditorRef<'a>> {
        let root = self.content.simulation_root_reference();
        if root.key().eq_ignore_ascii_case(&active_reference.key()) {
            Some(active_schematic.editor_ref())
        } else {
            find_schematic(self, &root)
        }
    }

    /// Create a new default project and ensure its editable top cell exists in
    /// the shared library manager.
    pub fn new_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let verilog_a = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
                "sensor_bridge.va",
                "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule",
            [],
            [],
        )
        .expect("the built-in Verilog-A example is valid");
        let automation = crate::state::ProjectSourceBundle::try_new_with_roles(
            crate::state::ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::RSpiceAutomation,
            ),
            ProjectSourceLanguage::RSpiceAutomation,
            crate::state::AutomationStarterFile::PythonEntry.path(),
            crate::state::DEFAULT_AUTOMATION_PYTHON,
            [
                crate::state::ProjectSourceFile::try_new(
                    crate::state::AutomationStarterFile::RunPlan.path(),
                    crate::state::DEFAULT_AUTOMATION_RUN_PLAN,
                )
                .expect("the built-in run plan is valid"),
                crate::state::ProjectSourceFile::try_new(
                    crate::state::AutomationStarterFile::EnvironmentLock.path(),
                    crate::state::DEFAULT_ENVIRONMENT_LOCK,
                )
                .expect("the built-in environment lock is valid"),
                crate::state::ProjectSourceFile::try_new(
                    crate::state::AutomationStarterFile::Permissions.path(),
                    crate::state::DEFAULT_AUTOMATION_PERMISSIONS,
                )
                .expect("the built-in permission manifest is valid"),
            ],
            [
                crate::state::ProjectSourceDependency::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::AutomationStarterFile::RunPlan.path(),
                )
                .expect("the run-plan dependency is valid"),
                crate::state::ProjectSourceDependency::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::AutomationStarterFile::EnvironmentLock.path(),
                )
                .expect("the environment-lock dependency is valid"),
                crate::state::ProjectSourceDependency::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::AutomationStarterFile::Permissions.path(),
                )
                .expect("the permission-manifest dependency is valid"),
            ],
            [
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::ProjectSourceRole::AutomationEntry,
                )
                .expect("the Automation entry role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::RunPlan.path(),
                    crate::state::ProjectSourceRole::AutomationRunPlan,
                )
                .expect("the Automation run-plan role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::EnvironmentLock.path(),
                    crate::state::ProjectSourceRole::AutomationEnvironmentLock,
                )
                .expect("the Automation environment-lock role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::Permissions.path(),
                    crate::state::ProjectSourceRole::AutomationPermissionManifest,
                )
                .expect("the Automation permission role is valid"),
            ],
        )
        .expect("the built-in Automation workspace is valid");
        let mut project_sources = ProjectSourceRegistry::try_from_bundles([verilog_a, automation])
            .expect("the bootstrapped Code source registry is valid");
        // The canonical Verilog-A fixture is compiled during bootstrap. Python
        // Automation is intentionally left unvalidated: only the exact
        // packaged CPython worker may create that receipt.
        project_sources
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .expect("the built-in Verilog-A identity is valid");
        let mut workspace = Self::default();
        workspace.content.project_sources = project_sources;
        workspace.ensure_library_model(libraries);
        workspace
    }

    /// Create a genuinely empty project under the requested identity. The
    /// canonical startup fixture keeps the mockup's example sources, while
    /// File > New must not force an unrelated circuit to compile or execute
    /// demonstration code.
    ///
    /// The identity is adopted before the library model is ensured, so the
    /// seeded design library, top cell, active view, open tab, hierarchy root
    /// and schematic buffer key are all the requested ones — never the default
    /// ones renamed afterwards. The caller owns validating the three names
    /// against the persisted cell/view and project-name contracts; nothing
    /// here can report a rejection to the reader.
    pub fn new_empty_bootstrapped(
        libraries: &mut LibraryManager,
        name: &str,
        root_library: &str,
        top_cell: &str,
    ) -> Self {
        let project = ProjectDescriptor::new(name, root_library, top_cell);
        let active_view = CellViewRef::new(root_library, top_cell, DEFAULT_SCHEMATIC_VIEW);
        let (design, session) = SchematicState::default().into_parts();
        let schematic_buffers = HashMap::from([(active_view.key(), design)]);
        let schematic_sessions = HashMap::from([(active_view.key(), session)]);
        let mut workspace = Self {
            schematic_sessions,
            ..Self::default()
        };
        workspace.content.project = project;
        workspace.content.open_views =
            vec![OpenCellView::new(active_view.clone(), ViewType::Schematic)];
        workspace.content.schematic_buffers = schematic_buffers;
        workspace.content.active_view = active_view;
        workspace.ensure_library_model(libraries);
        workspace
    }

    /// Resolve the complete executable library/cell/view closure rooted at the
    /// project testbench. Open tabs are intentionally irrelevant: this receipt
    /// follows placed hierarchical instances and the same schematic/source
    /// ownership used by netlisting.
    pub fn resolve_hierarchy(&self, libraries: &LibraryManager) -> HierarchyResolution {
        HierarchyResolver::new(self, libraries, None).resolve()
    }

    /// Resolve the hierarchy while projecting the live editor buffer over its
    /// persisted workspace copy. Rendering and validation use this form so a
    /// just-placed instance cannot disappear from the receipt until save or a
    /// view switch.
    pub fn resolve_hierarchy_with_active<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        active_reference: &'a CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> HierarchyResolution {
        HierarchyResolver::new(
            self,
            libraries,
            Some((active_reference, active_schematic.editor_ref())),
        )
        .resolve()
    }

    /// Materialize the exact multi-sheet, active-variant, and annotation
    /// projection consumed by DRC and netlisting. Authored canvas coordinates
    /// stay local to each sheet; the execution clone namespaces them by sheet
    /// order so coincident coordinates on different pages cannot create an
    /// accidental electrical connection. Explicit cross-sheet port contracts
    /// are then materialized as identically named labels at both endpoints.
    #[cfg(test)]
    pub(super) fn materialize_design_management_schematic(
        &self,
        cell_view_key: &str,
        source: &SchematicState,
    ) -> Result<rspice_design::projection::ProjectedSchematic, crate::state::DesignManagementError>
    {
        use rspice_design::projection::ProjectionSource;
        source.materialize(&self.content.design_management, cell_view_key)
    }

    /// Ensure the workspace's top library/cell/view exists in the library tree.
    pub fn ensure_library_model(&mut self, libraries: &mut LibraryManager) {
        ensure_project_library(libraries, &self.content.project.root_library);

        if self.content.active_view.library.is_empty() {
            self.content.active_view.library = self.content.project.root_library.clone();
        }
        if self.content.active_view.cell.is_empty() {
            self.content.active_view.cell = self.content.project.top_cell.clone();
        }
        if self.content.active_view.view.is_empty() {
            self.content.active_view.view = DEFAULT_SCHEMATIC_VIEW.to_string();
        }

        let active_view_type = self
            .content
            .open_views
            .iter()
            .find(|open| open.reference == self.content.active_view)
            .map(|open| open.view_type)
            .or_else(|| library_view_type(libraries, &self.content.active_view))
            .unwrap_or(ViewType::Schematic);

        ensure_cell_view(
            libraries,
            &self.content.active_view.library,
            &self.content.active_view.cell,
            &self.content.active_view.view,
            active_view_type,
        );

        if self.content.open_views.is_empty() {
            self.content.open_views.push(OpenCellView::new(
                self.content.active_view.clone(),
                active_view_type,
            ));
        }
        // Restore paths that never run project migration — session restore —
        // reach the occurrence model only here. On a live workspace this is an
        // identity, because the projection already mirrors the active
        // document.
        self.adopt_breadcrumb_for_active_document();

        if is_schematic_like(active_view_type) {
            self.ensure_active_buffer();
        }
        libraries.select_view(
            &self.content.active_view.library,
            &self.content.active_view.cell,
            &self.content.active_view.view,
        );
    }

    pub fn active_key(&self) -> String {
        self.content.active_view.key()
    }

    pub fn active_display_path(&self) -> String {
        self.content.active_view.display_path()
    }

    pub fn active_view_type(&self) -> ViewType {
        self.content
            .open_views
            .iter()
            .find(|open| open.reference == self.content.active_view)
            .map(|open| open.view_type)
            .unwrap_or(ViewType::Schematic)
    }

    pub fn ensure_active_buffer(&mut self) {
        let key = self.active_key();
        if !self.content.schematic_buffers.contains_key(&key) {
            self.insert_schematic_editor(key, SchematicState::default());
        }
    }

    pub(crate) fn active_schematic(&self) -> Option<SchematicEditorRef<'_>> {
        self.schematic_editor(&self.active_key())
    }

    pub fn active_schematic_reference(&self) -> CellViewRef {
        if self.active_view_type() == ViewType::Symbol {
            return CellViewRef::new(
                &self.content.active_view.library,
                &self.content.active_view.cell,
                DEFAULT_SCHEMATIC_VIEW,
            );
        }
        self.content.active_view.clone()
    }

    pub(crate) fn active_context_schematic(&self) -> Option<SchematicEditorRef<'_>> {
        let reference = self.active_schematic_reference();
        self.schematic_editor(&reference.key())
    }

    pub fn save_active_schematic(&mut self, schematic: &SchematicState) {
        if !is_schematic_like(self.active_view_type()) {
            return;
        }
        let key = self.active_key();
        self.insert_schematic_editor(key, schematic.clone());
        self.set_active_dirty(schematic.session.is_dirty);
    }

    pub fn mark_all_clean(&mut self) {
        self.content.mark_all_clean();
        for session in self.schematic_sessions.values_mut() {
            session.is_dirty = false;
        }
    }

    pub fn any_dirty(&self) -> bool {
        self.content.any_dirty()
            || self.content.schematic_buffers.keys().any(|key| {
                self.schematic_sessions
                    .get(key)
                    .is_some_and(|session| session.is_dirty)
            })
    }

    /// Ensure `reference` has an open document and make it the active one.
    ///
    /// A document that is already open keeps the occurrence and the read-only
    /// marking it was opened with; one that is not opens as a design root,
    /// because nothing was descended through to reach it.
    pub fn open_view(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.content.active_view = reference.clone();
        if !self
            .content
            .open_views
            .iter()
            .any(|open| open.reference == reference)
        {
            self.content
                .open_views
                .push(OpenCellView::new(reference.clone(), view_type));
        }
        if is_schematic_like(view_type)
            && !self
                .content
                .schematic_buffers
                .contains_key(&reference.key())
        {
            self.insert_schematic_editor(reference.key(), SchematicState::default());
        }
        self.project_active_occurrence();
    }

    /// Activate the document `reference` names. This is the tab gesture: it
    /// restores the occurrence that document was opened at rather than
    /// re-rooting the session on the master.
    pub fn activate_view(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference, view_type);
    }

    /// Open `reference` as a design root, discarding whatever occurrence the
    /// document previously carried. Only File/browser entry re-roots a
    /// document; every other gesture reaches one through an instance.
    pub fn open_as_root(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        self.set_active_occurrence(DocumentOccurrence::rooted(reference));
    }

    /// Descend into `instance`, opening its master `reference` on the active
    /// document's occurrence.
    pub fn descend_into(&mut self, instance: String, reference: CellViewRef, view_type: ViewType) {
        let mut occurrence = self.active_occurrence_or_root();
        let already_open = occurrence.terminal_master() == &reference;
        self.open_view(reference.clone(), view_type);
        if already_open {
            return;
        }
        occurrence.descend(instance, reference);
        self.set_active_occurrence(occurrence);
    }

    /// The occurrence the active document is editing.
    pub fn active_occurrence(&self) -> Option<&DocumentOccurrence> {
        self.content
            .open_views
            .iter()
            .find(|open| open.reference == self.content.active_view)
            .map(|open| &open.occurrence)
    }

    /// The active document's occurrence, or the root occurrence its reference
    /// implies while no document claims it.
    fn active_occurrence_or_root(&self) -> DocumentOccurrence {
        self.active_occurrence()
            .cloned()
            .unwrap_or_else(|| DocumentOccurrence::rooted(self.content.active_view.clone()))
    }

    fn set_active_occurrence(&mut self, occurrence: DocumentOccurrence) {
        let active = self.content.active_view.clone();
        if let Some(open) = self
            .content
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
        for open in &mut self.content.open_views {
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
    fn project_active_occurrence(&mut self) {
        let occurrence = self.active_occurrence_or_root();
        self.content.hierarchy_stack = occurrence.masters().cloned().collect();
        self.content.hierarchy_instances = occurrence
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
                .content
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
    pub fn occurrence_labels(&self) -> Vec<String> {
        self.active_occurrence_or_root().labels()
    }

    /// The occurrence the active document is editing, as an instance path.
    pub fn occurrence_path(&self) -> crate::state::InstancePath {
        self.active_occurrence_or_root().instance_path()
    }

    /// Levels on the active occurrence, counting the design root.
    pub fn occurrence_depth(&self) -> usize {
        self.active_occurrence_or_root().depth()
    }

    /// Whether the active document was opened as a read-only hierarchy
    /// reference.
    pub fn active_read_only_reference(&self) -> bool {
        self.content
            .open_views
            .iter()
            .find(|open| open.reference == self.content.active_view)
            .is_some_and(|open| open.read_only_reference)
    }

    pub fn set_active_read_only_reference(&mut self, read_only: bool) {
        let active = self.content.active_view.clone();
        if let Some(open) = self
            .content
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
        let reference = self.content.active_view.clone();
        self.set_active_occurrence(DocumentOccurrence::rooted(reference));
    }

    /// Pop one hierarchy level (the U gesture). Returns the new focus.
    pub fn ascend_one(&mut self) -> Option<CellViewRef> {
        let depth = self.occurrence_depth();
        if depth < 2 {
            return None;
        }
        self.focus_breadcrumb(depth - 2)
    }

    pub fn focus_breadcrumb(&mut self, index: usize) -> Option<CellViewRef> {
        let mut occurrence = self.active_occurrence_or_root();
        if index >= occurrence.depth() {
            return None;
        }

        occurrence.truncate_to(index);
        let reference = occurrence.terminal_master().clone();
        self.open_view(reference.clone(), ViewType::Schematic);
        self.set_active_occurrence(occurrence);
        Some(reference)
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
        self.content.open_views.retain_mut(|open| {
            match open.occurrence.retain_valid_prefix(&is_valid) {
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
            }
        });
        // A document re-targeted onto a master another tab already shows is
        // the same document twice; the first one keeps it.
        let mut seen = HashSet::new();
        self.content
            .open_views
            .retain(|open| seen.insert(open.reference.key()));
        if !self
            .content
            .open_views
            .iter()
            .any(|open| open.reference == self.content.active_view)
            && let Some(next) = self.content.open_views.first()
        {
            self.content.active_view = next.reference.clone();
        }
        self.project_active_occurrence();
        pruned
    }

    /// Rewrite the masters a library, cell, or view rename moved, on every
    /// open document's occurrence. Callers remap `active_view` and each
    /// document's `reference` first, so the terminal-master invariant holds
    /// across the whole transaction.
    pub fn remap_occurrence_masters(&mut self, mut remap: impl FnMut(&mut CellViewRef)) {
        for open in &mut self.content.open_views {
            for master in open.occurrence.masters_mut() {
                remap(master);
            }
            open.occurrence.debug_assert_opens(&open.reference);
        }
        self.project_active_occurrence();
    }

    /// The occurrence a session-global breadcrumb spells, and how many of its
    /// levels it could not name. Zipping stops at the shorter of the two
    /// vectors, because a missing instance name cannot be invented.
    fn breadcrumb_occurrence(&self) -> Option<(DocumentOccurrence, usize)> {
        let root = self.content.hierarchy_stack.first().cloned()?;
        let mut occurrence = DocumentOccurrence::rooted(root);
        for (master, instance) in self
            .content
            .hierarchy_stack
            .iter()
            .skip(1)
            .zip(&self.content.hierarchy_instances)
        {
            occurrence.descend(instance.clone(), master.clone());
        }
        let unnamed = self.content.hierarchy_stack.len() - occurrence.depth();
        Some((occurrence, unnamed))
    }

    /// Adopt a breadcrumb that describes the document already in front.
    ///
    /// Restore paths that never run project migration reach the occurrence
    /// model here, and so does every schematic restore, so this must never
    /// re-target which document is active: the breadcrumb records where a
    /// session had navigated, not which document a caller just opened. A
    /// breadcrumb that ends anywhere else is dropped in favour of the
    /// projection.
    fn adopt_breadcrumb_for_active_document(&mut self) {
        self.root_unrooted_occurrences();
        match self.breadcrumb_occurrence() {
            Some((occurrence, _)) if occurrence.terminal_master() == &self.content.active_view => {
                self.set_active_occurrence(occurrence);
            }
            _ => self.project_active_occurrence(),
        }
    }

    /// Fold a save's session-global breadcrumb onto the document it described.
    ///
    /// Every document is first rooted at its own reference, then the active
    /// one adopts the breadcrumb. A save whose two vectors disagree keeps only
    /// the prefix both spell, and adopts it only if it ends at a document that
    /// is actually open — an occurrence that named a master no tab shows would
    /// address a different instance than the document on screen. Returns the
    /// load warning that repair owes the reader.
    pub fn migrate_document_occurrences(&mut self) -> Option<String> {
        self.root_unrooted_occurrences();
        let Some((occurrence, unnamed)) = self.breadcrumb_occurrence() else {
            self.project_active_occurrence();
            return None;
        };
        let terminal = occurrence.terminal_master().clone();
        let adopted = self
            .content
            .open_views
            .iter()
            .any(|open| open.reference == terminal);

        if !adopted {
            self.project_active_occurrence();
            return Some(format!(
                "This project's saved hierarchy breadcrumb ended at {}, which no open document \
                 shows; the active document was restored at its own root instead.",
                terminal.display_path()
            ));
        }

        self.content.active_view = terminal;
        self.set_active_occurrence(occurrence);
        (unnamed > 0).then(|| {
            format!(
                "This project's saved hierarchy breadcrumb named {unnamed} level(s) it carried no \
                 instance name for; the occurrence was kept at {} rather than guessing them.",
                self.occurrence_path()
            )
        })
    }

    pub fn close_view(&mut self, reference: &CellViewRef) {
        if self.content.open_views.len() <= 1 {
            return;
        }

        self.content
            .open_views
            .retain(|open| &open.reference != reference);
        if &self.content.active_view == reference
            && let Some(next) = self.content.open_views.last().cloned()
        {
            self.content.active_view = next.reference;
        }
        self.project_active_occurrence();
    }

    pub fn set_active_dirty(&mut self, dirty: bool) {
        if let Some(open) = self
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == self.content.active_view)
        {
            open.dirty = dirty;
        }
    }
}

/// Ensure the project's editable design library exists under the name the
/// descriptor claims as its root.
pub fn ensure_project_library(libraries: &mut LibraryManager, name: &str) {
    if libraries.get_library(name).is_none() {
        let mut library = Library::new(name);
        library
            .metadata
            .insert("role".to_string(), "project".to_string());
        library.metadata.insert(
            "description".to_string(),
            "Project design library".to_string(),
        );
        libraries.add_library(library);
    }
}

/// Ensure a cell view exists in the library manager.
pub fn ensure_cell_view(
    libraries: &mut LibraryManager,
    library_name: &str,
    cell_name: &str,
    view_name: &str,
    view_type: ViewType,
) {
    if libraries.get_library(library_name).is_none() {
        libraries.add_library(Library::new(library_name));
    }

    if let Some(mut library) = libraries.edit_library(library_name) {
        library.ensure_cell_view(cell_name, view_name, view_type, "Top-level design cell");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn master(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "schematic")
    }

    /// A session that descended `labels` from the default root, as the
    /// gestures themselves build it.
    fn descended(labels: &[&str]) -> ProjectWorkspace {
        let mut workspace = ProjectWorkspace::default();
        let root = workspace.content.simulation_root_reference();
        workspace.open_as_root(root, ViewType::Schematic);
        for (index, label) in labels.iter().enumerate() {
            workspace.descend_into(
                (*label).to_owned(),
                master(&format!("level_{index}")),
                ViewType::Schematic,
            );
        }
        workspace
    }

    #[test]
    fn the_occurrence_path_names_instances_below_the_implicit_root() {
        assert!(descended(&[]).occurrence_path().is_root());
        assert_eq!(descended(&["X1"]).occurrence_path().to_string(), "/X1");
        assert_eq!(
            descended(&["X1", "XB"]).occurrence_path().to_string(),
            "/X1/XB"
        );
        assert!(
            descended(&["X 1"]).occurrence_path().is_root(),
            "a label the grammar cannot name resolves to the root, not to half a path"
        );
        assert!(descended(&["X1", "X 2"]).occurrence_path().is_root());
    }

    /// The defect this model exists to kill: one session-global breadcrumb
    /// meant the second tab's descent overwrote the first tab's, and coming
    /// back to a tab reported whichever path the last navigation left behind.
    #[test]
    fn two_documents_reached_through_different_parents_keep_their_own_occurrence() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.descend_into("XA".to_owned(), master("afe"), ViewType::Schematic);
        assert_eq!(workspace.occurrence_path().to_string(), "/XA");

        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.descend_into("XB".to_owned(), master("bias"), ViewType::Schematic);
        workspace.descend_into("XR".to_owned(), master("ref"), ViewType::Schematic);
        assert_eq!(workspace.occurrence_path().to_string(), "/XB/XR");

        workspace.activate_view(master("afe"), ViewType::Schematic);
        assert_eq!(
            workspace.occurrence_path().to_string(),
            "/XA",
            "activating a document restores the occurrence it was opened at"
        );
        workspace.activate_view(master("ref"), ViewType::Schematic);
        assert_eq!(workspace.occurrence_path().to_string(), "/XB/XR");
        assert_eq!(
            workspace.content.hierarchy_stack,
            vec![master("tb"), master("bias"), master("ref")],
            "the legacy breadcrumb is a projection of whichever document is active"
        );
    }

    #[test]
    fn every_open_document_ends_its_occurrence_at_the_master_it_shows() {
        let workspace = descended(&["X1", "XB"]);
        for open in &workspace.content.open_views {
            open.occurrence.debug_assert_opens(&open.reference);
            assert_eq!(open.occurrence.terminal_master(), &open.reference);
        }
    }

    #[test]
    fn a_read_only_reference_marking_belongs_to_the_document_it_was_opened_on() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.open_as_root(master("afe"), ViewType::Schematic);
        workspace.set_active_read_only_reference(true);
        assert!(workspace.active_read_only_reference());

        workspace.activate_view(master("tb"), ViewType::Schematic);
        assert!(
            !workspace.active_read_only_reference(),
            "the other document was never opened read-only"
        );
        workspace.activate_view(master("afe"), ViewType::Schematic);
        assert!(
            workspace.active_read_only_reference(),
            "returning to the reference document still refuses writes"
        );
    }

    #[test]
    fn pruning_truncates_to_what_survives_and_closes_a_rootless_document() {
        let mut workspace = descended(&["X1", "XB"]);
        let deepest = workspace.content.active_view.clone();
        assert!(workspace.retain_valid_occurrences(|reference| reference.cell != "level_0"));
        assert!(
            workspace
                .content
                .open_views
                .iter()
                .all(|open| open.reference != deepest),
            "the document below a master that is gone folds onto the surviving prefix"
        );
        assert!(workspace.occurrence_path().is_root());

        let mut rootless = ProjectWorkspace::default();
        rootless.open_as_root(master("keep"), ViewType::Schematic);
        rootless.open_as_root(master("gone"), ViewType::Schematic);
        rootless.descend_into("X1".to_owned(), master("child"), ViewType::Schematic);
        assert!(rootless.retain_valid_occurrences(|reference| reference.cell != "gone"));
        assert!(
            rootless
                .content
                .open_views
                .iter()
                .any(|open| open.reference == master("keep")),
            "an unrelated document is untouched"
        );
        assert!(
            rootless
                .content
                .open_views
                .iter()
                .all(|open| open.reference.cell != "gone" && open.reference.cell != "child"),
            "a document whose root is gone has no occurrence left, so it closes"
        );
    }

    #[test]
    fn a_legacy_breadcrumb_migrates_onto_the_document_it_described() {
        let mut workspace = ProjectWorkspace::default();
        let root = workspace.content.active_view.clone();
        workspace.open_view(master("afe"), ViewType::Schematic);
        workspace.content.hierarchy_stack = vec![root.clone(), master("afe")];
        workspace.content.hierarchy_instances = vec!["XAFE".to_owned()];

        assert!(workspace.migrate_document_occurrences().is_none());
        assert_eq!(workspace.occurrence_path().to_string(), "/XAFE");
        assert_eq!(workspace.content.active_view, master("afe"));
        assert_eq!(
            workspace
                .active_occurrence()
                .map(|occurrence| &occurrence.root),
            Some(&root)
        );
    }

    #[test]
    fn disagreeing_legacy_arrays_keep_the_shorter_prefix_and_warn() {
        let mut workspace = ProjectWorkspace::default();
        let root = workspace.content.active_view.clone();
        workspace.open_view(master("afe"), ViewType::Schematic);
        workspace.open_view(master("bias"), ViewType::Schematic);
        workspace.content.hierarchy_stack = vec![root, master("afe"), master("bias")];
        workspace.content.hierarchy_instances = vec!["XAFE".to_owned()];

        let warning = workspace
            .migrate_document_occurrences()
            .expect("a breadcrumb that cannot be spelled owes the reader a warning");
        assert!(warning.contains("1 level"), "{warning}");
        assert_eq!(
            workspace.occurrence_path().to_string(),
            "/XAFE",
            "the level with no instance name is dropped, never invented"
        );
        assert_eq!(workspace.content.active_view, master("afe"));
    }

    /// A crossing contract and a hand-placed connector must produce the same
    /// object, or the canvas would draw a materialized crossing as an ordinary
    /// local name and checks would never ask it for a partner.
    #[test]
    fn a_materialized_crossing_is_a_pair_of_off_sheet_connectors() {
        use crate::state::{
            CellViewRef, CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition,
            CrossSheetPortDirection, CrossSheetPortEndpoint, CrossSheetSignalType,
            MoveBoundaryResolution, MoveSelectionRequest, Point, SheetDefinition, SheetPortPolicy,
            SheetTemplate,
        };

        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let first = schematic
            .add_wire(vec![Point::origin(), Point::new(10, 0)])
            .expect("first wire");
        let second = schematic
            .add_wire(vec![Point::origin(), Point::new(0, 10)])
            .expect("second wire");

        let source_sheet = workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Input", [first, second])
            .expect("bootstrap sheet ownership");
        let catalog = workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("sheet catalog");
        let destination_sheet = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Output".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(source_sheet),
            )
            .expect("second sheet");
        catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![second],
                destination_sheet_id: destination_sheet,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "BIAS".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: source_sheet,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: first,
                                point: Point::origin(),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: destination_sheet,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: second,
                                point: Point::origin(),
                            },
                        },
                        direction: CrossSheetPortDirection::Supply,
                        signal_type: CrossSheetSignalType::Power,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .expect("move with explicit boundary contract");

        assert!(
            schematic.document().net_labels.is_empty(),
            "the crossing is a sheet contract, not an authored label"
        );
        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("materialize governed design");
        let crossing: Vec<_> = projected
            .document()
            .net_labels
            .iter()
            .filter(|label| label.name == "BIAS")
            .collect();

        assert_eq!(crossing.len(), 2, "one connector per side of the contract");
        for label in &crossing {
            assert_eq!(
                label.kind,
                crate::state::NetLabelKind::OffSheet {
                    direction: CrossSheetPortDirection::Supply
                },
                "a materialized crossing carries the contract's own direction"
            );
        }
        assert_ne!(
            crossing[0].pos, crossing[1].pos,
            "the pair lands in the two sheets' separate coordinate namespaces"
        );
    }
    #[test]
    fn design_management_projection_applies_active_variant_and_annotation() {
        use crate::state::{ComponentType, Point, SchematicState};
        use std::collections::BTreeMap;

        use crate::state::{
            AnnotationObject, AnnotationPosition, AssemblyVariantDraft, ComponentSubstitution,
            ProtectedReferencePolicy, RenumberOrder, RenumberRequest, RenumberScope,
            SchematicObjectKey, VariantInheritance, VariantObjectOverride,
            VariantQualificationPlan, VariantQualificationState,
        };

        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let substituted = schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
        let omitted = schematic.add_component(ComponentType::Capacitor, Point::new(20, 10));
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == substituted)
            .unwrap()
            .name = "R42".to_owned();
        let variant = workspace
            .content
            .design_management
            .variants_mut()
            .create(AssemblyVariantDraft {
                name: "Automotive".to_owned(),
                parent_id: None,
                inheritance: VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
                overrides: BTreeMap::from([
                    (
                        SchematicObjectKey::new(&key, substituted)
                            .expect("scoped substituted identity"),
                        VariantObjectOverride::Substitute {
                            replacement: ComponentSubstitution {
                                library: "qualified".to_owned(),
                                cell: "resistor_aecq".to_owned(),
                                view: "schematic".to_owned(),
                                value_override: Some("2 kohm".to_owned()),
                                model_section: Some("automotive".to_owned()),
                                port_equivalence_digest: Some(ContentDigest::from_bytes([9; 32])),
                                qualification: VariantQualificationState::Current,
                            },
                        },
                    ),
                    (
                        SchematicObjectKey::new(&key, omitted).expect("scoped omitted identity"),
                        VariantObjectOverride::DoNotPopulate {
                            approval_reference: "ECO-104".to_owned(),
                        },
                    ),
                ]),
            })
            .expect("create governed variant");
        workspace
            .content
            .design_management
            .variants_mut()
            .set_active(variant)
            .expect("activate variant");

        let request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![AnnotationObject {
                object: SchematicObjectKey::new(&key, substituted)
                    .expect("scoped annotation identity"),
                current_reference: "R42".to_owned(),
                device_family: "R".to_owned(),
                sheet_id: None,
                hierarchy_path: "/top".to_owned(),
                position: AnnotationPosition { x: 10, y: 10 },
                connectivity_order: Some(1),
                locked: false,
                external: false,
                imported: false,
            }],
        };
        let preview = workspace
            .content
            .design_management
            .annotation()
            .preview_renumbering(&request)
            .expect("preview annotation");
        workspace
            .content
            .design_management
            .annotation_mut()
            .commit_renumbering(&preview, &request)
            .expect("commit annotation receipt");

        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("materialize variant and annotation");
        assert!(
            projected
                .document()
                .components
                .iter()
                .all(|component| component.id != omitted)
        );
        assert!(
            projected
                .document()
                .connections
                .iter()
                .all(|connection| connection.component_id != omitted)
        );
        let component = projected
            .document()
            .components
            .iter()
            .find(|component| component.id == substituted)
            .expect("substituted component");
        let binding = component
            .library_cell
            .as_ref()
            .expect("qualified cell binding");
        assert_eq!(binding.library, "qualified");
        assert_eq!(binding.cell, "resistor_aecq");
        assert_eq!(component.value, "2 kohm");
        assert_eq!(binding.model_section.as_deref(), Some("automotive"));
        assert!(!component.params.contains("model_section="));
        assert_eq!(component.name, "R1");

        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == substituted)
            .unwrap()
            .params = "note='unterminated".to_owned();
        let error = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .unwrap_err();
        assert!(matches!(
            error,
            crate::state::DesignManagementError::InvalidReplacementParameters { .. }
        ));
        assert!(error.to_string().contains("unterminated"));
        assert_eq!(
            schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == substituted)
                .unwrap()
                .params,
            "note='unterminated"
        );
    }
}

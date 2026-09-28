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
        let mut workspace = Self::default();
        workspace.session.schematic_sessions = schematic_sessions;
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
        self.project_hierarchy(libraries, None).resolve()
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
        self.project_hierarchy(
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
        self.session
            .ensure_library_model(&mut self.content, libraries);
    }

    pub fn ensure_active_buffer(&mut self) {
        let key = self.content.active_key();
        if !self.content.schematic_buffers.contains_key(&key) {
            self.insert_schematic_editor(key, SchematicState::default());
        }
    }

    pub(crate) fn active_schematic(&self) -> Option<SchematicEditorRef<'_>> {
        self.schematic_editor(&self.content.active_key())
    }

    pub(crate) fn active_context_schematic(&self) -> Option<SchematicEditorRef<'_>> {
        let reference = self.content.active_schematic_reference();
        self.schematic_editor(&reference.key())
    }

    pub fn save_active_schematic(&mut self, schematic: &SchematicState) {
        if !is_schematic_like(self.content.active_view_type()) {
            return;
        }
        let key = self.content.active_key();
        self.insert_schematic_editor(key, schematic.clone());
        self.content.set_active_dirty(schematic.session.is_dirty);
    }

    pub fn mark_all_clean(&mut self) {
        self.content.mark_all_clean();
        for session in self.session.schematic_sessions.values_mut() {
            session.is_dirty = false;
        }
    }

    pub fn any_dirty(&self) -> bool {
        self.content.any_dirty()
            || self.content.schematic_buffers.keys().any(|key| {
                self.session
                    .schematic_sessions
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
        self.content.project_active_occurrence();
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
        self.content
            .set_active_occurrence(DocumentOccurrence::rooted(reference));
    }

    /// Descend into `instance`, opening its master `reference` on the active
    /// document's occurrence.
    pub fn descend_into(&mut self, instance: String, reference: CellViewRef, view_type: ViewType) {
        let mut occurrence = self.content.active_occurrence_or_root();
        let already_open = occurrence.terminal_master() == &reference;
        self.open_view(reference.clone(), view_type);
        if already_open {
            return;
        }
        occurrence.descend(instance, reference);
        self.content.set_active_occurrence(occurrence);
    }

    /// Pop one hierarchy level (the U gesture). Returns the new focus.
    pub fn ascend_one(&mut self) -> Option<CellViewRef> {
        let depth = self.content.occurrence_depth();
        if depth < 2 {
            return None;
        }
        self.focus_breadcrumb(depth - 2)
    }

    pub fn focus_breadcrumb(&mut self, index: usize) -> Option<CellViewRef> {
        let mut occurrence = self.content.active_occurrence_or_root();
        if index >= occurrence.depth() {
            return None;
        }

        occurrence.truncate_to(index);
        let reference = occurrence.terminal_master().clone();
        self.open_view(reference.clone(), ViewType::Schematic);
        self.content.set_active_occurrence(occurrence);
        Some(reference)
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

impl super::WorkspaceSession {
    pub(crate) fn ensure_library_model(
        &mut self,
        content: &mut rspice_project::ProjectWorkspace,
        libraries: &mut LibraryManager,
    ) {
        ensure_project_library(libraries, &content.project.root_library);

        if content.active_view.library.is_empty() {
            content.active_view.library = content.project.root_library.clone();
        }
        if content.active_view.cell.is_empty() {
            content.active_view.cell = content.project.top_cell.clone();
        }
        if content.active_view.view.is_empty() {
            content.active_view.view = DEFAULT_SCHEMATIC_VIEW.to_string();
        }

        let active_view_type = content
            .open_views
            .iter()
            .find(|open| open.reference == content.active_view)
            .map(|open| open.view_type)
            .or_else(|| library_view_type(libraries, &content.active_view))
            .unwrap_or(ViewType::Schematic);

        ensure_cell_view(
            libraries,
            &content.active_view.library,
            &content.active_view.cell,
            &content.active_view.view,
            active_view_type,
        );

        if content.open_views.is_empty() {
            content.open_views.push(OpenCellView::new(
                content.active_view.clone(),
                active_view_type,
            ));
        }
        // Restore paths that never run project migration — session restore —
        // reach the occurrence model only here. On a live workspace this is an
        // identity, because the projection already mirrors the active
        // document.
        content.adopt_breadcrumb_for_active_document();

        if is_schematic_like(active_view_type) {
            let key = content.active_key();
            if !content.schematic_buffers.contains_key(&key) {
                self.insert_schematic_editor(content, key, SchematicState::default());
            }
        }
        libraries.select_view(
            &content.active_view.library,
            &content.active_view.cell,
            &content.active_view.view,
        );
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
        assert!(descended(&[]).content.occurrence_path().is_root());
        assert_eq!(
            descended(&["X1"]).content.occurrence_path().to_string(),
            "/X1"
        );
        assert_eq!(
            descended(&["X1", "XB"])
                .content
                .occurrence_path()
                .to_string(),
            "/X1/XB"
        );
        assert!(
            descended(&["X 1"]).content.occurrence_path().is_root(),
            "a label the grammar cannot name resolves to the root, not to half a path"
        );
        assert!(
            descended(&["X1", "X 2"])
                .content
                .occurrence_path()
                .is_root()
        );
    }

    /// The defect this model exists to kill: one session-global breadcrumb
    /// meant the second tab's descent overwrote the first tab's, and coming
    /// back to a tab reported whichever path the last navigation left behind.
    #[test]
    fn two_documents_reached_through_different_parents_keep_their_own_occurrence() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.descend_into("XA".to_owned(), master("afe"), ViewType::Schematic);
        assert_eq!(workspace.content.occurrence_path().to_string(), "/XA");

        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.descend_into("XB".to_owned(), master("bias"), ViewType::Schematic);
        workspace.descend_into("XR".to_owned(), master("ref"), ViewType::Schematic);
        assert_eq!(workspace.content.occurrence_path().to_string(), "/XB/XR");

        workspace.activate_view(master("afe"), ViewType::Schematic);
        assert_eq!(
            workspace.content.occurrence_path().to_string(),
            "/XA",
            "activating a document restores the occurrence it was opened at"
        );
        workspace.activate_view(master("ref"), ViewType::Schematic);
        assert_eq!(workspace.content.occurrence_path().to_string(), "/XB/XR");
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
        workspace.content.set_active_read_only_reference(true);
        assert!(workspace.content.active_read_only_reference());

        workspace.activate_view(master("tb"), ViewType::Schematic);
        assert!(
            !workspace.content.active_read_only_reference(),
            "the other document was never opened read-only"
        );
        workspace.activate_view(master("afe"), ViewType::Schematic);
        assert!(
            workspace.content.active_read_only_reference(),
            "returning to the reference document still refuses writes"
        );
    }

    #[test]
    fn pruning_truncates_to_what_survives_and_closes_a_rootless_document() {
        let mut workspace = descended(&["X1", "XB"]);
        let deepest = workspace.content.active_view.clone();
        assert!(
            workspace
                .content
                .retain_valid_occurrences(|reference| reference.cell != "level_0")
        );
        assert!(
            workspace
                .content
                .open_views
                .iter()
                .all(|open| open.reference != deepest),
            "the document below a master that is gone folds onto the surviving prefix"
        );
        assert!(workspace.content.occurrence_path().is_root());

        let mut rootless = ProjectWorkspace::default();
        rootless.open_as_root(master("keep"), ViewType::Schematic);
        rootless.open_as_root(master("gone"), ViewType::Schematic);
        rootless.descend_into("X1".to_owned(), master("child"), ViewType::Schematic);
        assert!(
            rootless
                .content
                .retain_valid_occurrences(|reference| reference.cell != "gone")
        );
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

        assert!(workspace.content.migrate_document_occurrences().is_none());
        assert_eq!(workspace.content.occurrence_path().to_string(), "/XAFE");
        assert_eq!(workspace.content.active_view, master("afe"));
        assert_eq!(
            workspace
                .content
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
            .content
            .migrate_document_occurrences()
            .expect("a breadcrumb that cannot be spelled owes the reader a warning");
        assert!(warning.contains("1 level"), "{warning}");
        assert_eq!(
            workspace.content.occurrence_path().to_string(),
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

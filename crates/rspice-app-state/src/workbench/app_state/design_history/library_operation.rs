//! Library operations as steps of the one Undo.
//!
//! Creating, copying, renaming and deleting a library, a cell or a view is
//! one step. The step holds, on each side of the operation, only what the
//! operation changed: the catalog's libraries and cells, the drawings and
//! the placements in them, the open tabs, the configuration roots, the
//! Design Management ownership, the owned sources, the layouts and the
//! project top. Undo makes those what they were and Redo what they became.
//!
//! Each crossing is a library transaction of its own, as the operation was:
//! the project's revision and the catalog's only move forward, the audit
//! records the crossing as the operation it amounts to, and the catalog is
//! issued a new stamp, so nothing derived from the catalog the step left is
//! served again.
//!
//! A step stands while what it changed is as it left it. The Edit menu asks
//! whether it does on every frame, so that answer is what exists; the exact
//! answer, every recorded part compared, is given when the step is taken.
//! The open tabs are where the reader is looking, not what the step owns:
//! the crossing takes away the tabs the operation opened and brings back
//! the ones it closed, and leaves every other tab as the reader left it.

use super::*;
use crate::state::workspace::PreparedPhysicalLayoutCatalog;
use crate::state::{
    ConfigurationSetCatalog, ConfigurationSetDefinition, ConfigurationSetId, Library,
    PhysicalLayoutDocument, PreparedProjectLibraryMutation, ProjectLibraryMutation,
    ProjectSourceRegistry,
};
use rspice_design::schematic::document::SchematicDocument;

/// Everything a library operation can change, as it stood before the
/// operation ran. Compared with the project once the operation is
/// published, it gives the step its two sides.
pub struct LibraryOperationStart {
    operation: ProjectLibraryMutation,
    libraries: LibraryManager,
    drawings: BTreeMap<String, SchematicState>,
    tabs: Tabs,
    top: (String, String),
    configurations: BTreeMap<ConfigurationSetId, ConfigurationSetDefinition>,
    design_management: DesignManagementCatalog,
    sources: ProjectSourceRegistry,
    layouts: BTreeMap<String, PhysicalLayoutDocument>,
}

/// One library operation, as one step.
#[derive(Debug, Clone)]
pub struct LibraryOperationRecord {
    pub(super) description: String,
    operation: ProjectLibraryMutation,
    before: Side,
    after: Side,
    tabs_before: Tabs,
    tabs_after: Tabs,
}

/// What one side of a step holds: each part the operation changed, as it
/// stood on that side. A key with `None` is absent on that side.
#[derive(Debug, Clone, Default)]
struct Side {
    /// Each library whose own record the operation changed, without its
    /// cells, which are held one by one below.
    libraries: BTreeMap<String, Option<Library>>,
    cells: BTreeMap<(String, String), Option<Cell>>,
    selection: [Option<String>; 3],
    drawings: BTreeMap<String, Drawing>,
    configurations: BTreeMap<ConfigurationSetId, ConfigurationSetDefinition>,
    design_management: Option<DesignManagementCatalog>,
    sources: Option<ProjectSourceRegistry>,
    layouts: BTreeMap<String, Option<PhysicalLayoutDocument>>,
    top: Option<(String, String)>,
}

/// One drawing on one side of a step.
#[derive(Debug, Clone)]
enum Drawing {
    /// The operation brought it in or took it away; this side has none.
    Absent,
    /// The operation brought it in or took it away; this side has this
    /// one, its history and its view of it with it.
    Whole(Box<SchematicState>),
    /// The operation changed what it holds, its placements' bindings.
    Content(Box<SchematicSnapshot>),
}

/// The open tabs, each with the occurrence it is reached through, and the
/// one in front.
#[derive(Debug, Clone)]
struct Tabs {
    open: Vec<OpenCellView>,
    active: CellViewRef,
}

/// A crossing made ready: everything that can refuse it has been asked, and
/// nothing has changed.
struct Crossing {
    receipt: PreparedProjectLibraryMutation,
    libraries: LibraryManager,
    configurations: Option<ConfigurationSetCatalog>,
    design_management: Option<DesignManagementCatalog>,
    layouts: Option<PreparedPhysicalLayoutCatalog>,
}

impl Model {
    /// What the library operation `operation` can change, as it stands now,
    /// for the step its publication records. `None` for a restore from a
    /// publication, which replaces the whole project rather than taking a
    /// step in it.
    pub(crate) fn start_library_operation(
        &self,
        operation: &ProjectLibraryMutation,
    ) -> Option<Box<LibraryOperationStart>> {
        reversal(operation)?;
        let content = &self.workspace.content;
        let active = content.active_schematic_reference().key();
        let drawings = content
            .schematic_buffers
            .keys()
            .filter_map(|key| {
                let drawing = if *key == active {
                    self.schematic.clone()
                } else {
                    self.workspace.clone_schematic_editor(key)?
                };
                Some((key.clone(), drawing))
            })
            .collect();
        Some(Box::new(LibraryOperationStart {
            operation: operation.clone(),
            libraries: self.library_manager.clone(),
            drawings,
            tabs: Tabs::capture(self),
            top: project_top(self),
            configurations: configuration_definitions(&content.configuration_sets),
            design_management: (*content.design_management).clone(),
            sources: content.project_sources.clone(),
            layouts: content.physical_layout_documents().clone(),
        }))
    }

    /// Record the library operation `start` began, now published, as one
    /// step holding what it changed. An operation that changed nothing
    /// records nothing.
    pub(crate) fn record_library_operation(&mut self, start: LibraryOperationStart) {
        let (before, after) = sides(self, &start);
        if before.is_empty() {
            return;
        }
        // A drawing the operation edited in place, by taking placements off
        // it, has its sheet membership settled by the next sync. Undo puts
        // those objects back on the sheets they were drawn on, which the
        // project still records until that sync.
        let documents = after
            .drawings
            .iter()
            .filter(|(_, drawing)| matches!(drawing, Drawing::Content(_)))
            .filter_map(|(key, _)| reference_preparation::reference_from_key(key).ok())
            .map(|reference| DocumentCompensation::with_recorded_sheets(self, reference))
            .collect();
        let record = LibraryOperationRecord {
            description: describe(&start.operation),
            operation: start.operation,
            before,
            after,
            tabs_before: start.tabs,
            tabs_after: Tabs::capture(self),
        };
        self.push_project_record(
            RecordHeader::committed(documents, None, None),
            ProjectDesignBody::LibraryOperation(Box::new(record)),
        );
    }
}

impl LibraryOperationRecord {
    /// Whether what the operation made still exists as it made it, by what
    /// exists: cheap enough for every frame.
    pub(super) fn after_design_matches(&self, state: &Model) -> bool {
        self.after.exists_in(state)
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        self.before.exists_in(state)
    }

    pub(super) fn validate_mutation(&self, state: &Model, operation: &str) -> Result<(), String> {
        if !state.project_open.is_open() {
            return Err(format!(
                "The {} step cannot be {operation} without an open project.",
                self.description
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "The {} step cannot be {operation} while the project is read-only.",
                self.description
            ));
        }
        Ok(())
    }

    /// The exact answer: every part the step holds is as the side it leaves
    /// has it, and the crossing can be made.
    pub(super) fn validate_crossing(&self, state: &Model, forward: bool) -> Result<(), String> {
        self.prepare_crossing(state, forward).map(|_| ())
    }

    fn prepare_crossing(&self, state: &Model, forward: bool) -> Result<Crossing, String> {
        // The project stands on the side the crossing leaves and is made
        // into the other.
        let (verb, leaving, making) = if forward {
            ("redone", &self.before, &self.after)
        } else {
            ("undone", &self.after, &self.before)
        };
        self.validate_mutation(state, verb)?;
        if !leaving.matches(state) {
            return Err(format!(
                "The {} step cannot be {verb} because what it changed was edited since.",
                self.description
            ));
        }
        let mutation = if forward {
            self.operation.clone()
        } else {
            reversal(&self.operation).expect("a recorded operation has a reversal")
        };
        Crossing::prepare(state, making, mutation)
            .map_err(|error| format!("The {} step cannot be {verb}: {error}", self.description))
    }

    /// Select in the library browser what the side the crossing made had
    /// selected. Taking up the drawing in front selects that drawing's view,
    /// so this follows it.
    pub(super) fn restore_selection(&self, state: &mut Model, forward: bool) {
        let side = if forward { &self.after } else { &self.before };
        let [library, cell, view] = side.selection.clone();
        let libraries = &mut state.library_manager;
        libraries.selected_library = library;
        libraries.selected_cell = cell;
        libraries.selected_view = view;
    }

    pub(super) fn apply(
        &mut self,
        state: &mut Model,
        forward: bool,
        effects: &mut HistoryEffects,
    ) -> Result<(), String> {
        let crossing = self.prepare_crossing(state, forward)?;
        // What the reader left standing is what the next crossing back
        // returns to: the tabs as they are now, those opened since the
        // step was recorded included, and the library selection.
        let standing = (Tabs::capture(state), selection(&state.library_manager));
        let (making, tabs_leaving, tabs_making) = if forward {
            (&self.after, &self.tabs_before, &self.tabs_after)
        } else {
            (&self.before, &self.tabs_after, &self.tabs_before)
        };
        crossing.commit(state, making, tabs_leaving, tabs_making, effects);
        let (left, tabs_left) = if forward {
            (&mut self.before, &mut self.tabs_before)
        } else {
            (&mut self.after, &mut self.tabs_after)
        };
        (*tabs_left, left.selection) = standing;
        Ok(())
    }
}

impl Crossing {
    /// Ask everything that can refuse the crossing to `side`, changing
    /// nothing: the audit's next receipt, the catalog, the configurations,
    /// Design Management and the layouts as the side has them.
    fn prepare(
        state: &Model,
        side: &Side,
        mutation: ProjectLibraryMutation,
    ) -> Result<Self, String> {
        let receipt = state
            .workspace
            .content
            .project
            .prepare_library_mutation(mutation, state.library_manager.revision())
            .map_err(|error| error.to_string())?;
        let mut libraries = state.library_manager.clone();
        for (key, shell) in &side.libraries {
            let cells = libraries
                .get_library(key)
                .map(|library| library.cells.clone())
                .unwrap_or_default();
            libraries.remove_library(key);
            if let Some(shell) = shell {
                let mut library = shell.clone();
                library.cells = cells;
                libraries.add_library(library);
            }
        }
        for ((library, cell), entry) in &side.cells {
            match (libraries.edit_library(library), entry) {
                (Some(mut edit), Some(entry)) => edit.add_cell(entry.clone()),
                (Some(mut edit), None) => {
                    edit.remove_cell(cell);
                }
                (None, Some(_)) => return Err(format!("library '{library}' is missing")),
                (None, None) => {}
            }
        }
        let [selected_library, selected_cell, selected_view] = side.selection.clone();
        libraries.selected_library = selected_library;
        libraries.selected_cell = selected_cell;
        libraries.selected_view = selected_view;

        let content = &state.workspace.content;
        let configurations = if side.configurations.is_empty() {
            None
        } else {
            let mut catalog = (*content.configuration_sets).clone();
            for (id, definition) in &side.configurations {
                let current = catalog
                    .find(*id)
                    .ok_or_else(|| "a configuration it changed is gone".to_owned())?;
                if current.definition() == definition {
                    continue;
                }
                let revision = current.revision();
                catalog
                    .update(*id, revision, definition.clone())
                    .map_err(|error| error.to_string())?;
            }
            Some(catalog)
        };
        let design_management = match &side.design_management {
            Some(catalog)
                if !design_management_semantics_match(&content.design_management, catalog) =>
            {
                let mut published = (*content.design_management).clone();
                published
                    .publish_reviewed_candidate(published.revision(), catalog.clone())
                    .map_err(|error| error.to_string())?;
                Some(published)
            }
            _ => None,
        };
        let layouts = if side.layouts.is_empty() {
            None
        } else {
            Some(
                content
                    .prepare_replaced_physical_layout_documents(&side.layouts)
                    .map_err(|error| error.to_string())?,
            )
        };
        Ok(Self {
            receipt,
            libraries,
            configurations,
            design_management,
            layouts,
        })
    }

    /// Make the project `side`, with `tabs_making` taking the place of
    /// `tabs_leaving`, and publish the crossing's receipt.
    fn commit(
        self,
        state: &mut Model,
        side: &Side,
        tabs_leaving: &Tabs,
        tabs_making: &Tabs,
        effects: &mut HistoryEffects,
    ) {
        let previous = state.workspace.content.project.revision();
        // The live drawing joins the workspace's copy of it, so every
        // drawing is put back in one place and the editor takes up the one
        // in front afterwards.
        let active = state.workspace.content.active_schematic_reference().key();
        if state
            .workspace
            .content
            .schematic_buffers
            .contains_key(&active)
        {
            state
                .workspace
                .insert_schematic_editor(active.clone(), state.schematic.clone());
        }
        state
            .library_manager
            .replace_catalog_from_snapshot(&self.libraries)
            .expect("the catalog revision was reserved with the receipt");
        // A drawing put back is unsaved: the project file may hold the
        // version the step left.
        for (key, drawing) in &side.drawings {
            match drawing {
                Drawing::Absent => {
                    state.workspace.remove_schematic_editor(key);
                }
                Drawing::Whole(drawing) => {
                    let mut drawing = (**drawing).clone();
                    drawing.session.is_dirty = true;
                    state
                        .workspace
                        .insert_schematic_editor(key.clone(), drawing);
                }
                Drawing::Content(snapshot) => {
                    if let Some(mut stored) = state.workspace.schematic_editor_mut(key) {
                        stored.editor.apply_snapshot(snapshot);
                        stored.editor.session.is_dirty = true;
                    }
                }
            }
        }
        restore_tabs(
            &mut state.workspace.content,
            &state.library_manager,
            tabs_leaving,
            tabs_making,
        );
        for open in &mut state.workspace.content.open_views {
            if side.drawings.contains_key(&open.reference.key()) {
                open.dirty = true;
            }
        }
        if let Some(configurations) = self.configurations {
            *state.workspace.content.configuration_sets = configurations;
        }
        if let Some(design_management) = self.design_management {
            *state.workspace.content.design_management = design_management;
        }
        if let Some(sources) = &side.sources {
            state.workspace.content.project_sources = sources.clone();
            state.workspace.content.project_sources_dirty = true;
            effects.void_veriloga_evidence = true;
        }
        if let Some(layouts) = self.layouts {
            state
                .workspace
                .content
                .commit_prepared_physical_layout_catalog(layouts);
        }
        if let Some((library, cell)) = &side.top {
            let project = &mut state.workspace.content.project;
            project.root_library.clone_from(library);
            project.top_cell.clone_from(cell);
        }
        // The editor takes up the drawing in front only when the crossing
        // put back that drawing or brought another to the front; otherwise
        // what it shows, and how, is left as it is.
        let front = state.workspace.content.active_schematic_reference().key();
        if (front != active || side.drawings.contains_key(&front))
            && let Some(drawing) = state.workspace.clone_schematic_editor(&front)
        {
            state.schematic = drawing;
            effects.restore_active_schematic = true;
        }

        state
            .workspace
            .content
            .project
            .publish_library_mutation(self.receipt, state.library_manager.revision())
            .expect("the crossing's receipt was fully prepared");
        state.workspace.content.project_metadata_dirty = true;
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        state.netlist_artifacts.current_generation_input_digest = None;
        let published = state.workspace.content.project.revision();
        state.carry_project_history_over_revision(previous, published);
    }
}

impl Side {
    fn is_empty(&self) -> bool {
        self.libraries.is_empty()
            && self.cells.is_empty()
            && self.drawings.is_empty()
            && self.configurations.is_empty()
            && self.design_management.is_none()
            && self.sources.is_none()
            && self.layouts.is_empty()
            && self.top.is_none()
    }

    /// Whether everything this side holds exists in the project as it has
    /// it, by presence alone.
    fn exists_in(&self, state: &Model) -> bool {
        let libraries = &state.library_manager;
        self.libraries
            .iter()
            .all(|(key, shell)| libraries.get_library(key).is_some() == shell.is_some())
            && self.cells.iter().all(|((library, cell), entry)| {
                libraries
                    .get_library(library)
                    .and_then(|library| library.get_cell(cell))
                    .is_some()
                    == entry.is_some()
            })
            && self.drawings.iter().all(|(key, drawing)| {
                live_document(state, key).is_some() != matches!(drawing, Drawing::Absent)
            })
    }

    /// Whether every part this side holds is in the project exactly as it
    /// has it.
    fn matches(&self, state: &Model) -> bool {
        let content = &state.workspace.content;
        let libraries = &state.library_manager;
        self.libraries
            .iter()
            .all(|(key, shell)| library_records_match(libraries.get_library(key), shell.as_ref()))
            && self.cells.iter().all(|((library, cell), entry)| {
                cells_match(
                    libraries
                        .get_library(library)
                        .and_then(|library| library.get_cell(cell)),
                    entry.as_ref(),
                )
            })
            && self
                .drawings
                .iter()
                .all(|(key, drawing)| drawing.matches(live_document(state, key)))
            && self.configurations.iter().all(|(id, definition)| {
                content
                    .configuration_sets
                    .find(*id)
                    .is_some_and(|current| current.definition() == definition)
            })
            && self.design_management.as_ref().is_none_or(|catalog| {
                design_management_semantics_match(&content.design_management, catalog)
            })
            && self
                .sources
                .as_ref()
                .is_none_or(|sources| content.project_sources == *sources)
            && self.layouts.iter().all(|(key, document)| {
                content.physical_layout_documents().get(key) == document.as_ref()
            })
            && self
                .top
                .as_ref()
                .is_none_or(|top| project_top(state) == *top)
    }
}

impl Drawing {
    fn matches(&self, live: Option<&SchematicDocument>) -> bool {
        match (self, live) {
            (Self::Absent, None) => true,
            (Self::Whole(drawing), Some(live)) => {
                SchematicSnapshot::capture(drawing.document()).is_equal_document(live)
            }
            (Self::Content(snapshot), Some(live)) => snapshot.is_equal_document(live),
            _ => false,
        }
    }
}

impl Tabs {
    fn capture(state: &Model) -> Self {
        let content = &state.workspace.content;
        Self {
            open: content.open_views.clone(),
            active: content.active_view.clone(),
        }
    }

    fn holds(&self, reference: &CellViewRef) -> Option<&OpenCellView> {
        self.open.iter().find(|open| open.reference == *reference)
    }
}

/// Put the tabs a crossing changes back as `making` has them: the tabs only
/// `leaving` has close, the ones only `making` has open where they stood, a
/// tab both have follows the occurrence the step moved it through, and the
/// tab in front is the one `making` had in front while the reader is still
/// on the one the step left in front. A tab opened after the step was
/// recorded on a view `catalog` no longer holds closes with it.
fn restore_tabs(
    content: &mut rspice_project::ProjectWorkspace,
    catalog: &LibraryManager,
    leaving: &Tabs,
    making: &Tabs,
) {
    let mut open = content.open_views.clone();
    open.retain(|tab| {
        leaving.holds(&tab.reference).is_none() || making.holds(&tab.reference).is_some()
    });
    for tab in &mut open {
        if let (Some(left), Some(made)) =
            (leaving.holds(&tab.reference), making.holds(&tab.reference))
            && tab.occurrence == left.occurrence
        {
            tab.occurrence = made.occurrence.clone();
        }
    }
    for (index, tab) in making.open.iter().enumerate() {
        if leaving.holds(&tab.reference).is_none()
            && !open.iter().any(|open| open.reference == tab.reference)
        {
            open.insert(index.min(open.len()), tab.clone());
        }
    }
    if content.active_view == leaving.active
        || !open.iter().any(|tab| tab.reference == content.active_view)
    {
        content.active_view = making.active.clone();
    }
    content.open_views = open;
    content.retain_valid_occurrences(|reference| {
        catalog
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some()
    });
}

/// What the operation changed, as each side has it.
fn sides(state: &Model, start: &LibraryOperationStart) -> (Side, Side) {
    let mut before = Side {
        selection: selection(&start.libraries),
        ..Side::default()
    };
    let mut after = Side {
        selection: selection(&state.library_manager),
        ..Side::default()
    };

    let earlier = start
        .libraries
        .libraries_by_key()
        .collect::<BTreeMap<_, _>>();
    let later = state
        .library_manager
        .libraries_by_key()
        .collect::<BTreeMap<_, _>>();
    for key in earlier.keys().chain(later.keys()).collect::<BTreeSet<_>>() {
        let (was, is) = (earlier.get(key).copied(), later.get(key).copied());
        if !library_records_match(was, is) {
            before
                .libraries
                .insert((*key).to_owned(), was.map(library_record));
            after
                .libraries
                .insert((*key).to_owned(), is.map(library_record));
        }
        let cells = was
            .into_iter()
            .chain(is)
            .flat_map(|library| library.cells.keys())
            .collect::<BTreeSet<_>>();
        for cell in cells {
            let was = was.and_then(|library| library.get_cell(cell));
            let is = is.and_then(|library| library.get_cell(cell));
            if !cells_match(was, is) {
                let key = ((*key).to_owned(), cell.clone());
                before.cells.insert(key.clone(), was.cloned());
                after.cells.insert(key, is.cloned());
            }
        }
    }

    let content = &state.workspace.content;
    let active = content.active_schematic_reference().key();
    let keys = start
        .drawings
        .keys()
        .chain(content.schematic_buffers.keys())
        .collect::<BTreeSet<_>>();
    for key in keys {
        let live = content.schematic_buffers.get(key).map(|_| {
            if *key == active {
                state.schematic.editor_ref()
            } else {
                state
                    .workspace
                    .schematic_editor(key)
                    .expect("a drawing the workspace holds")
            }
        });
        let sides = match (start.drawings.get(key), live) {
            (None, None) => None,
            (Some(was), None) => Some((Drawing::Whole(Box::new(was.clone())), Drawing::Absent)),
            (None, Some(is)) => {
                Some((Drawing::Absent, Drawing::Whole(Box::new(is.clone_editor()))))
            }
            (Some(was), Some(is))
                if was.editor_ref().design.content_stamp() == is.design.content_stamp() =>
            {
                None
            }
            (Some(was), Some(is)) => {
                let was = SchematicSnapshot::capture(was.document());
                (!was.is_equal_document(is.document())).then(|| {
                    (
                        Drawing::Content(Box::new(was)),
                        Drawing::Content(Box::new(SchematicSnapshot::capture(is.document()))),
                    )
                })
            }
        };
        if let Some((was, is)) = sides {
            before.drawings.insert(key.clone(), was);
            after.drawings.insert(key.clone(), is);
        }
    }

    let configurations = configuration_definitions(&content.configuration_sets);
    for (id, was) in &start.configurations {
        if let Some(is) = configurations.get(id)
            && is != was
        {
            before.configurations.insert(*id, was.clone());
            after.configurations.insert(*id, is.clone());
        }
    }
    if !design_management_semantics_match(&start.design_management, &content.design_management) {
        before.design_management = Some(start.design_management.clone());
        after.design_management = Some((*content.design_management).clone());
    }
    if start.sources != content.project_sources {
        before.sources = Some(start.sources.clone());
        after.sources = Some(content.project_sources.clone());
    }
    let layouts = content.physical_layout_documents();
    for key in start
        .layouts
        .keys()
        .chain(layouts.keys())
        .collect::<BTreeSet<_>>()
    {
        let (was, is) = (start.layouts.get(key), layouts.get(key));
        if was != is {
            before.layouts.insert(key.clone(), was.cloned());
            after.layouts.insert(key.clone(), is.cloned());
        }
    }
    let top = project_top(state);
    if start.top != top {
        before.top = Some(start.top.clone());
        after.top = Some(top);
    }
    (before, after)
}

/// The drawing `key` names as the project holds it: the editor's own for
/// the one in front, the workspace's copy for every other.
fn live_document<'a>(state: &'a Model, key: &str) -> Option<&'a SchematicDocument> {
    let stored = state.workspace.content.schematic_buffers.get(key)?;
    if key == state.workspace.content.active_schematic_reference().key() {
        Some(state.schematic.document())
    } else {
        Some(stored.document())
    }
}

fn project_top(state: &Model) -> (String, String) {
    let project = &state.workspace.content.project;
    (project.root_library.clone(), project.top_cell.clone())
}

fn selection(libraries: &LibraryManager) -> [Option<String>; 3] {
    [
        libraries.selected_library.clone(),
        libraries.selected_cell.clone(),
        libraries.selected_view.clone(),
    ]
}

fn configuration_definitions(
    catalog: &ConfigurationSetCatalog,
) -> BTreeMap<ConfigurationSetId, ConfigurationSetDefinition> {
    catalog
        .configurations()
        .iter()
        .map(|configuration| (configuration.id(), configuration.definition().clone()))
        .collect()
}

/// A library's own record, without its cells.
fn library_record(library: &Library) -> Library {
    Library {
        name: library.name.clone(),
        path: library.path.clone(),
        cells: Default::default(),
        technology: library.technology.clone(),
        read_only: library.read_only,
        metadata: library.metadata.clone(),
    }
}

/// Whether two libraries' own records are the same, their cells aside.
fn library_records_match(left: Option<&Library>, right: Option<&Library>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            left.name == right.name
                && left.path == right.path
                && left.technology == right.technology
                && left.read_only == right.read_only
                && left.metadata == right.metadata
        }
        _ => false,
    }
}

/// Whether two cells hold the same design. Whether a view has unsaved edits
/// is the editor's to say, not the cell's, so it is not compared.
fn cells_match(left: Option<&Cell>, right: Option<&Cell>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            left.name == right.name
                && left.description == right.description
                && left.metadata == right.metadata
                && left.views.len() == right.views.len()
                && left.views.iter().all(|(key, view)| {
                    right.views.get(key).is_some_and(|other| {
                        view.name == other.name
                            && view.view_type == other.view_type
                            && view.file_path == other.file_path
                            && view.metadata == other.metadata
                    })
                })
        }
        _ => false,
    }
}

/// The operation that takes `operation` back, as the audit records it.
/// `None` for a restore from a publication, which is not a step.
fn reversal(operation: &ProjectLibraryMutation) -> Option<ProjectLibraryMutation> {
    use ProjectLibraryMutation as Operation;
    Some(match operation.clone() {
        Operation::CreateLibrary { library } => Operation::DeleteLibrary { library },
        Operation::DeleteLibrary { library } => Operation::CreateLibrary { library },
        Operation::RenameLibrary {
            from_library,
            to_library,
        } => Operation::RenameLibrary {
            from_library: to_library,
            to_library: from_library,
        },
        Operation::CreateCell { library, cell }
        | Operation::CopyCell {
            target_library: library,
            target_cell: cell,
            ..
        } => Operation::DeleteCell { library, cell },
        Operation::DeleteCell { library, cell } => Operation::CreateCell { library, cell },
        Operation::RenameCell {
            library,
            from_cell,
            to_cell,
        } => Operation::RenameCell {
            library,
            from_cell: to_cell,
            to_cell: from_cell,
        },
        Operation::CreateView {
            library,
            cell,
            view,
        } => Operation::DeleteView {
            library,
            cell,
            view,
        },
        Operation::DeleteView {
            library,
            cell,
            view,
        } => Operation::CreateView {
            library,
            cell,
            view,
        },
        Operation::RenameView {
            library,
            cell,
            from_view,
            to_view,
        } => Operation::RenameView {
            library,
            cell,
            from_view: to_view,
            to_view: from_view,
        },
        Operation::RollbackPublication { .. } => return None,
    })
}

/// The step's name, as the Console states what Undo took back.
fn describe(operation: &ProjectLibraryMutation) -> String {
    use ProjectLibraryMutation as Operation;
    match operation {
        Operation::CreateLibrary { library } => format!("create library {library}"),
        Operation::DeleteLibrary { library } => format!("delete library {library}"),
        Operation::RenameLibrary {
            from_library,
            to_library,
        } => format!("rename library {from_library} to {to_library}"),
        Operation::CreateCell { library, cell } => format!("create cell {library}/{cell}"),
        Operation::CopyCell {
            source_library,
            source_cell,
            target_library,
            target_cell,
        } => format!("copy cell {source_library}/{source_cell} to {target_library}/{target_cell}"),
        Operation::DeleteCell { library, cell } => format!("delete cell {library}/{cell}"),
        Operation::RenameCell {
            library,
            from_cell,
            to_cell,
        } => format!("rename cell {library}/{from_cell} to {library}/{to_cell}"),
        Operation::CreateView {
            library,
            cell,
            view,
        } => format!("create view {library}/{cell}/{view}"),
        Operation::DeleteView {
            library,
            cell,
            view,
        } => format!("delete view {library}/{cell}/{view}"),
        Operation::RenameView {
            library,
            cell,
            from_view,
            to_view,
        } => format!("rename view {library}/{cell}/{from_view} to {library}/{cell}/{to_view}"),
        Operation::RollbackPublication {
            publication_label, ..
        } => format!("restore project from publication {publication_label}"),
    }
}

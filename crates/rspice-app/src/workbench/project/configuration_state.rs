//! What the Project > Configuration page keeps between frames: which
//! configuration is in front of the reader, the edits not yet applied to it,
//! the refusal the last transaction on it stated, and the hierarchy those
//! edits resolve to.
//!
//! The project's configuration catalog stays the authority. Nothing here is
//! saved with the project, and nothing here changes what a run executes: an
//! edit reaches the project only through Apply.

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::state::workspace::{DesignProjectionKey, HierarchyResolution};
use crate::state::{
    CellViewRef, ConfigurationCellBinding, ConfigurationPlatform, ConfigurationSetCatalog,
    ConfigurationSetDefinition, ConfigurationSetError, ConfigurationSetId,
    ConfigurationSetOverride, InstancePath, InstancePathPattern, LibraryManager, ProjectWorkspace,
    SchematicState, ViewType,
};

/// The configuration the page shows. `Default` is the built-in search order
/// a project runs with when no configuration is used for runs; it is not a
/// catalog entry and can be neither renamed nor deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ConfigurationSelection {
    Default,
    Configuration(ConfigurationSetId),
}

/// Edits to one configuration that are not applied yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConfigurationDraft {
    pub(crate) id: ConfigurationSetId,
    /// The revision the edits started from. Apply commits against it, so a
    /// change made elsewhere in the meantime is refused, never overwritten.
    pub(crate) base_revision: u64,
    pub(crate) definition: ConfigurationSetDefinition,
}

/// Text typed into one of the page's fields and not committed yet. A field
/// commits on Enter or when it loses focus, so typing never re-resolves the
/// design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldEdit {
    /// Which field: its own stable key.
    pub(crate) field: String,
    pub(crate) text: String,
}

/// One change to an instance or pattern binding, made in the inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BindingChange {
    Views(crate::ui::widgets::OrderedChipsEdit),
    Stop(Option<String>),
    ModelSection(Option<String>),
    RunsOn(Vec<ConfigurationPlatform>),
    /// A pattern binding's pattern.
    Path(String),
    Remove,
}

/// The two tables under the global bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ConfigurationTab {
    #[default]
    Instances,
    Cells,
}

/// The table row whose properties the inspector shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConfigurationRow {
    /// One expanded instance, by canonical path.
    Instance(String),
    /// One pattern binding, by its pattern as written.
    Pattern(String),
    /// One library/cell, bound or not.
    Cell { library: String, cell: String },
}

/// The Configuration page's own state.
#[derive(Debug, Clone, Default)]
pub(crate) struct ConfigurationPageState {
    /// What the reader picked. `None` follows whatever is used for runs, so
    /// a project opens on the configuration it netlists with.
    selection: Option<ConfigurationSelection>,
    draft: Option<ConfigurationDraft>,
    /// Why the last apply, activation or deletion was refused. It stays on
    /// the page until the next transaction or a new selection.
    pub(crate) refusal: Option<String>,
    /// A configuration the reader picked while the one in front had edits
    /// that are not applied; the page asks before it switches.
    pub(crate) pending_selection: Option<ConfigurationSelection>,
    pub(crate) tab: ConfigurationTab,
    pub(crate) row: Option<ConfigurationRow>,
    pub(crate) field_edit: Option<FieldEdit>,
    /// Instances whose children the tree hides, by fold key.
    collapsed: BTreeSet<String>,
    resolution: ResolutionCache,
}

impl ConfigurationPageState {
    /// The configuration in front of the reader. A picked configuration that
    /// no longer exists falls back to the one used for runs.
    pub(crate) fn selection(&self, catalog: &ConfigurationSetCatalog) -> ConfigurationSelection {
        match self.selection {
            Some(ConfigurationSelection::Default) => ConfigurationSelection::Default,
            Some(ConfigurationSelection::Configuration(id)) if catalog.find(id).is_some() => {
                ConfigurationSelection::Configuration(id)
            }
            _ => catalog
                .active_configuration_id()
                .map_or(ConfigurationSelection::Default, |id| {
                    ConfigurationSelection::Configuration(id)
                }),
        }
    }

    /// Put `selection` in front of the reader, dropping any edits to the
    /// configuration it replaces. Callers ask first when those edits are not
    /// applied (see [`Self::request_selection`]).
    pub(crate) fn select(&mut self, selection: ConfigurationSelection) {
        if self.selection != Some(selection) {
            self.refusal = None;
            self.row = None;
        }
        if self
            .draft
            .as_ref()
            .is_some_and(|draft| selection != ConfigurationSelection::Configuration(draft.id))
        {
            self.draft = None;
        }
        self.selection = Some(selection);
        self.pending_selection = None;
    }

    /// The reader picked `selection`. Edits that are not applied are never
    /// dropped silently: with some in front, the switch waits for an answer.
    pub(crate) fn request_selection(
        &mut self,
        catalog: &ConfigurationSetCatalog,
        selection: ConfigurationSelection,
    ) {
        if self.is_dirty(catalog) && self.selection(catalog) != selection {
            self.pending_selection = Some(selection);
        } else {
            self.select(selection);
        }
    }

    /// The unapplied edits, if any.
    pub(crate) fn draft(&self) -> Option<&ConfigurationDraft> {
        self.draft.as_ref()
    }

    /// The name of the configuration whose edits are not applied, if any.
    pub(crate) fn unapplied_edits(&self, catalog: &ConfigurationSetCatalog) -> Option<String> {
        self.draft
            .as_ref()
            .filter(|_| self.is_dirty(catalog))
            .and_then(|draft| catalog.find(draft.id))
            .map(|committed| committed.name().to_owned())
    }

    /// Whether there are edits that Apply would commit.
    pub(crate) fn is_dirty(&self, catalog: &ConfigurationSetCatalog) -> bool {
        self.draft.as_ref().is_some_and(|draft| {
            catalog
                .find(draft.id)
                .is_some_and(|committed| committed.definition() != &draft.definition)
        })
    }

    /// The definition the page shows for `id`: the edits when there are
    /// some, otherwise what the project holds.
    pub(crate) fn definition<'a>(
        &'a self,
        catalog: &'a ConfigurationSetCatalog,
        id: ConfigurationSetId,
    ) -> Option<&'a ConfigurationSetDefinition> {
        self.draft
            .as_ref()
            .filter(|draft| draft.id == id)
            .map(|draft| &draft.definition)
            .or_else(|| catalog.find(id).map(|committed| committed.definition()))
    }

    /// Edit `id`'s definition. The first edit starts the draft from what the
    /// project holds; an edit that puts it back the way it was ends it.
    pub(crate) fn edit(
        &mut self,
        catalog: &ConfigurationSetCatalog,
        id: ConfigurationSetId,
        edit: impl FnOnce(&mut ConfigurationSetDefinition),
    ) {
        let Some(committed) = catalog.find(id) else {
            return;
        };
        let mut draft = match self.draft.take() {
            Some(draft) if draft.id == id => draft,
            _ => ConfigurationDraft {
                id,
                base_revision: committed.revision(),
                definition: committed.definition().clone(),
            },
        };
        edit(&mut draft.definition);
        if draft.definition != *committed.definition() {
            self.draft = Some(draft);
        }
        self.refusal = None;
    }

    /// The configuration `id` was renamed to `name` at `revision` by a
    /// dialog: edits to it keep going from the renamed configuration rather
    /// than becoming edits to a configuration that changed under them.
    pub(crate) fn renamed(&mut self, id: ConfigurationSetId, name: &str, revision: u64) {
        if let Some(draft) = self.draft.as_mut().filter(|draft| draft.id == id) {
            draft.definition.name = name.to_owned();
            draft.base_revision = revision;
        }
    }

    /// Drop the edits; the page shows what the project holds again.
    pub(crate) fn revert(&mut self) {
        self.draft = None;
        self.refusal = None;
    }

    /// Whether `path`'s children are hidden.
    pub(crate) fn is_collapsed(&self, path: &str) -> bool {
        self.collapsed.contains(&fold(path))
    }

    pub(crate) fn toggle_collapsed(&mut self, path: &str) {
        let key = fold(path);
        if !self.collapsed.remove(&key) {
            self.collapsed.insert(key);
        }
        self.resolution.visible = None;
    }

    /// The candidate hierarchy for what the page shows: the selection with
    /// its edits applied. It is rebuilt only when the selection, the edits or
    /// the design change, never because a frame was drawn.
    pub(crate) fn resolution(
        &mut self,
        workspace: &ProjectWorkspace,
        libraries: &LibraryManager,
        active_view: &CellViewRef,
        schematic: &SchematicState,
    ) -> Arc<ConfigurationResolution> {
        let catalog: &ConfigurationSetCatalog = &workspace.content.configuration_sets;
        let selection = self.selection(catalog);
        let draft = match selection {
            ConfigurationSelection::Configuration(id) => self
                .draft
                .as_ref()
                .filter(|draft| draft.id == id)
                .map(|draft| &draft.definition),
            ConfigurationSelection::Default => None,
        };
        let design = workspace.design_projection_key(libraries, active_view, schematic);
        let library_revision = libraries.catalog().revision();
        let project_revision = workspace.content.project.revision().get();
        if let (Some(key), Some(value)) = (&self.resolution.key, &self.resolution.value)
            && key.selection == selection
            && key.draft.as_ref() == draft
            && key.design == design
            && key.library_revision == library_revision
            && key.project_revision == project_revision
        {
            return Arc::clone(value);
        }
        let value = Arc::new(ConfigurationResolution::build(
            workspace,
            libraries,
            active_view,
            schematic,
            selection,
            draft,
        ));
        self.resolution = ResolutionCache {
            key: Some(ResolutionKey {
                selection,
                draft: draft.cloned(),
                design,
                library_revision,
                project_revision,
            }),
            value: Some(Arc::clone(&value)),
            visible: None,
            builds: self.resolution.builds + 1,
        };
        value
    }

    /// The last resolution built, without rebuilding: what the inspector
    /// reads beside the stage.
    pub(crate) fn last_resolution(&self) -> Option<Arc<ConfigurationResolution>> {
        self.resolution.value.clone()
    }

    /// The tree rows not hidden under a collapsed instance, as indices into
    /// [`ConfigurationResolution::instances`]. Kept until the tree or its
    /// collapsed set changes, so a frame pays for the rows it draws.
    pub(crate) fn visible_instances(&mut self) -> Arc<[usize]> {
        if let Some(visible) = &self.resolution.visible {
            return Arc::clone(visible);
        }
        let rows = self
            .resolution
            .value
            .as_ref()
            .map_or(&[][..], |value| value.instances.as_slice());
        let mut visible = Vec::with_capacity(rows.len());
        let mut hidden_below: Option<usize> = None;
        for (index, row) in rows.iter().enumerate() {
            if let Some(depth) = hidden_below {
                if row.depth > depth {
                    continue;
                }
                hidden_below = None;
            }
            visible.push(index);
            if row.children > 0 && self.collapsed.contains(&fold(&row.path)) {
                hidden_below = Some(row.depth);
            }
        }
        let visible: Arc<[usize]> = visible.into();
        self.resolution.visible = Some(Arc::clone(&visible));
        visible
    }

    /// How many times the candidate hierarchy was built.
    #[cfg(test)]
    pub(crate) fn resolution_builds(&self) -> u64 {
        self.resolution.builds
    }
}

#[derive(Debug, Clone, Default)]
struct ResolutionCache {
    key: Option<ResolutionKey>,
    value: Option<Arc<ConfigurationResolution>>,
    visible: Option<Arc<[usize]>>,
    builds: u64,
}

/// Everything the candidate hierarchy depends on. The design key digests the
/// live buffers and the project's authorities, the catalog among them; the
/// two revisions cover a design whose authorities cannot be digested.
#[derive(Debug, Clone, PartialEq)]
struct ResolutionKey {
    selection: ConfigurationSelection,
    draft: Option<ConfigurationSetDefinition>,
    design: Option<DesignProjectionKey>,
    library_revision: u64,
    project_revision: u64,
}

/// What the selection, with its edits, resolves every instance to.
#[derive(Debug)]
pub(crate) struct ConfigurationResolution {
    /// The resolved hierarchy, or why the candidate cannot resolve one.
    pub(crate) hierarchy: Result<HierarchyResolution, String>,
    /// Every expanded instance, parents before their children.
    pub(crate) instances: Vec<InstanceRow>,
    /// Every distinct library/cell the expanded design instantiates, in
    /// library/cell order.
    pub(crate) cells: Vec<CellRow>,
    /// The views a configuration can take as its top cell.
    pub(crate) top_cells: Vec<CellViewRef>,
    /// Every view name the project's libraries hold.
    pub(crate) view_names: Vec<String>,
}

/// One library/cell of the expanded design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CellRow {
    pub(crate) library: String,
    pub(crate) cell: String,
    /// Expanded instances of it.
    pub(crate) instances: usize,
    /// The distinct views its instances resolved to; empty when none did.
    pub(crate) views_found: Vec<String>,
}

/// One expanded instance in the tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InstanceRow {
    /// Canonical instance path; the design root is `/`.
    pub(crate) path: String,
    /// Segments below the root.
    pub(crate) depth: usize,
    /// Index of its grouped binding in the hierarchy's bindings.
    pub(crate) binding: usize,
    /// How many rows sit directly under it.
    pub(crate) children: usize,
}

impl ConfigurationResolution {
    fn build(
        workspace: &ProjectWorkspace,
        libraries: &LibraryManager,
        active_view: &CellViewRef,
        schematic: &SchematicState,
        selection: ConfigurationSelection,
        draft: Option<&ConfigurationSetDefinition>,
    ) -> Self {
        let catalog: &ConfigurationSetCatalog = &workspace.content.configuration_sets;
        let hierarchy = candidate_catalog(catalog, selection, draft).and_then(|candidate| {
            let projection = if candidate == *catalog {
                workspace.inspect_design_projection(libraries, active_view, schematic)
            } else {
                let mut candidate_workspace = workspace.clone();
                *candidate_workspace.content.configuration_sets = candidate;
                candidate_workspace.inspect_design_projection(libraries, active_view, schematic)
            };
            projection
                .map(|projection| projection.hierarchy_resolution().clone())
                .map_err(|error| error.to_string())
        });
        let instances = hierarchy.as_ref().map(instance_rows).unwrap_or_default();
        let cells = hierarchy.as_ref().map(cell_rows).unwrap_or_default();
        Self {
            hierarchy,
            instances,
            cells,
            top_cells: top_cell_choices(workspace, libraries),
            view_names: project_view_names(libraries),
        }
    }

    /// The tree row for `path`.
    pub(crate) fn instance(&self, path: &str) -> Option<&InstanceRow> {
        let key = fold(path);
        self.instances.iter().find(|row| fold(&row.path) == key)
    }
}

/// The catalog the page resolves: Default stops using any configuration,
/// and a configuration is used for runs with its edits applied.
fn candidate_catalog(
    catalog: &ConfigurationSetCatalog,
    selection: ConfigurationSelection,
    draft: Option<&ConfigurationSetDefinition>,
) -> Result<ConfigurationSetCatalog, String> {
    let mut candidate = catalog.clone();
    match selection {
        ConfigurationSelection::Default => candidate
            .deactivate()
            .map_err(|error| plain_refusal(&error))?,
        ConfigurationSelection::Configuration(id) => {
            if let Some(definition) = draft
                && let Some(committed) = catalog.find(id)
            {
                match candidate.update(id, committed.revision(), definition.clone()) {
                    Ok(_) | Err(ConfigurationSetError::NoChanges(_)) => {}
                    Err(error) => return Err(plain_refusal(&error)),
                }
            }
            candidate
                .activate(id)
                .map_err(|error| plain_refusal(&error))?;
        }
    }
    Ok(candidate)
}

/// The grouped bindings expanded to one row per instance, ordered so every
/// instance follows its parent.
fn instance_rows(hierarchy: &HierarchyResolution) -> Vec<InstanceRow> {
    let mut rows = hierarchy
        .bindings
        .iter()
        .enumerate()
        .flat_map(|(binding, row)| {
            row.instance_paths.iter().filter_map(move |path| {
                let parsed = InstancePath::parse_legacy(path).ok()?;
                let order = parsed
                    .segments()
                    .iter()
                    .map(|segment| segment.to_ascii_lowercase())
                    .collect::<Vec<_>>();
                Some((order, parsed.to_string(), binding))
            })
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows.dedup_by(|right, left| right.0 == left.0);
    let mut instances = rows
        .iter()
        .map(|(order, path, binding)| InstanceRow {
            path: path.clone(),
            depth: order.len(),
            binding: *binding,
            children: 0,
        })
        .collect::<Vec<_>>();
    // A parent is the nearest earlier row one level up; the order above puts
    // it there.
    let mut parents: Vec<usize> = Vec::new();
    for index in 0..instances.len() {
        let depth = instances[index].depth;
        while parents
            .last()
            .is_some_and(|parent| instances[*parent].depth >= depth)
        {
            parents.pop();
        }
        if let Some(parent) = parents.last()
            && rows[index].0.starts_with(&rows[*parent].0)
        {
            instances[*parent].children += 1;
        }
        parents.push(index);
    }
    instances
}

/// The grouped bindings regrouped by the library/cell they instantiate.
fn cell_rows(hierarchy: &HierarchyResolution) -> Vec<CellRow> {
    let mut cells: std::collections::BTreeMap<(String, String), CellRow> =
        std::collections::BTreeMap::new();
    for binding in &hierarchy.bindings {
        let reference = &binding.reference;
        let row = cells
            .entry((
                reference.library.to_ascii_lowercase(),
                reference.cell.to_ascii_lowercase(),
            ))
            .or_insert_with(|| CellRow {
                library: reference.library.clone(),
                cell: reference.cell.clone(),
                instances: 0,
                views_found: Vec::new(),
            });
        row.instances += binding.instance_count;
        if binding.status.is_resolved()
            && !row
                .views_found
                .iter()
                .any(|view| view.eq_ignore_ascii_case(&reference.view))
        {
            row.views_found.push(reference.view.clone());
        }
    }
    cells.into_values().collect()
}

/// The cell binding for `library`/`cell`, if the definition has one.
pub(crate) fn cell_binding<'a>(
    definition: &'a ConfigurationSetDefinition,
    library: &str,
    cell: &str,
) -> Option<&'a ConfigurationCellBinding> {
    definition.cell_bindings.iter().find(|binding| {
        binding.library.eq_ignore_ascii_case(library) && binding.cell.eq_ignore_ascii_case(cell)
    })
}

/// Apply `change` to the cell binding for `library`/`cell`. A cell with no
/// binding gets one, starting from `inherited`; Remove returns every
/// instance of it to the view list. Model sections and platforms belong to
/// instance bindings, so those changes leave a cell binding alone.
pub(crate) fn apply_cell_binding_change(
    definition: &mut ConfigurationSetDefinition,
    library: &str,
    cell: &str,
    inherited: &[String],
    change: BindingChange,
) {
    let position = definition.cell_bindings.iter().position(|binding| {
        binding.library.eq_ignore_ascii_case(library) && binding.cell.eq_ignore_ascii_case(cell)
    });
    let index = match (position, &change) {
        (Some(index), _) => index,
        (None, BindingChange::Views(_) | BindingChange::Stop(_)) => {
            definition.cell_bindings.push(ConfigurationCellBinding {
                library: library.to_owned(),
                cell: cell.to_owned(),
                executable_views: inherited.to_vec(),
                stop_view: None,
            });
            definition.cell_bindings.len() - 1
        }
        (None, _) => return,
    };
    let binding = &mut definition.cell_bindings[index];
    match change {
        BindingChange::Views(edit) => {
            edit.apply(&mut binding.executable_views);
            let views = binding.executable_views.clone();
            binding.stop_view = binding
                .stop_view
                .take()
                .filter(|stop| views.iter().any(|view| view.eq_ignore_ascii_case(stop)));
        }
        BindingChange::Stop(stop) => binding.stop_view = stop,
        BindingChange::Remove => {
            definition.cell_bindings.remove(index);
        }
        BindingChange::ModelSection(_) | BindingChange::RunsOn(_) | BindingChange::Path(_) => {}
    }
}

/// Bind every instance of `library`/`cell` to exactly `view`, or with
/// `None` remove the cell binding so they inherit the view list again.
pub(crate) fn set_cell_view(
    definition: &mut ConfigurationSetDefinition,
    library: &str,
    cell: &str,
    view: Option<&str>,
) {
    match view {
        None => apply_cell_binding_change(definition, library, cell, &[], BindingChange::Remove),
        Some(view) => {
            if let Some(binding) = definition.cell_bindings.iter_mut().find(|binding| {
                binding.library.eq_ignore_ascii_case(library)
                    && binding.cell.eq_ignore_ascii_case(cell)
            }) {
                binding.executable_views = vec![view.to_owned()];
                binding.stop_view = binding
                    .stop_view
                    .take()
                    .filter(|stop| stop.eq_ignore_ascii_case(view));
            } else {
                definition.cell_bindings.push(ConfigurationCellBinding {
                    library: library.to_owned(),
                    cell: cell.to_owned(),
                    executable_views: vec![view.to_owned()],
                    stop_view: None,
                });
            }
        }
    }
}

fn fold(path: &str) -> String {
    InstancePath::parse_legacy(path)
        .map_or_else(|_| path.to_ascii_lowercase(), |parsed| parsed.fold_key())
}

/// Whether `selection` is what runs use today.
pub(crate) fn used_for_runs(
    catalog: &ConfigurationSetCatalog,
    selection: ConfigurationSelection,
) -> bool {
    match selection {
        ConfigurationSelection::Default => catalog.active_configuration_id().is_none(),
        ConfigurationSelection::Configuration(id) => catalog.active_configuration_id() == Some(id),
    }
}

/// The exact-path instance binding for `path`, if the definition has one.
pub(crate) fn instance_binding<'a>(
    definition: &'a ConfigurationSetDefinition,
    path: &str,
) -> Option<&'a ConfigurationSetOverride> {
    let key = InstancePath::parse_legacy(path).ok()?.fold_key();
    definition.overrides.iter().find(|binding| {
        InstancePath::parse_legacy(&binding.instance_path)
            .is_ok_and(|exact| exact.fold_key() == key)
    })
}

/// Whether an instance binding names a pattern rather than one instance.
pub(crate) fn is_pattern(binding: &ConfigurationSetOverride) -> bool {
    InstancePath::parse_legacy(&binding.instance_path).is_err()
}

/// The binding that governs `path`: the matching one that pins the most
/// positions by name, as the resolver chooses it.
pub(crate) fn governing_binding<'a>(
    definition: &'a ConfigurationSetDefinition,
    path: &str,
) -> Option<&'a ConfigurationSetOverride> {
    let instance = InstancePath::parse_legacy(path).ok()?;
    definition
        .overrides
        .iter()
        .filter_map(|binding| {
            let pattern = InstancePathPattern::parse_legacy(&binding.instance_path).ok()?;
            pattern
                .matches(&instance)
                .then(|| (binding, pattern.specificity()))
        })
        .max_by_key(|(_, specificity)| *specificity)
        .map(|(binding, _)| binding)
}

/// Bind `path` to exactly `view`, or with `None` remove its instance binding
/// so it inherits the view list again. A binding's other properties survive
/// a change of view; a new binding runs on every platform.
pub(crate) fn set_instance_view(
    definition: &mut ConfigurationSetDefinition,
    path: &str,
    view: Option<&str>,
) {
    let Ok(parsed) = InstancePath::parse_legacy(path) else {
        return;
    };
    let key = parsed.fold_key();
    let existing = definition.overrides.iter().position(|binding| {
        InstancePath::parse_legacy(&binding.instance_path)
            .is_ok_and(|exact| exact.fold_key() == key)
    });
    match (view, existing) {
        (None, Some(index)) => {
            definition.overrides.remove(index);
        }
        (None, None) => {}
        (Some(view), Some(index)) => {
            let binding = &mut definition.overrides[index];
            binding.executable_views = vec![view.to_owned()];
            binding.stop_view = binding
                .stop_view
                .take()
                .filter(|stop| stop.eq_ignore_ascii_case(view));
        }
        (Some(view), None) => definition.overrides.push(ConfigurationSetOverride {
            instance_path: parsed.to_string(),
            executable_views: vec![view.to_owned()],
            stop_view: None,
            model_section: None,
            eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
        }),
    }
}

/// Apply `change` to the binding stored under `path`. An instance with no
/// binding of its own gets one, starting from `inherited`, the view list it
/// inherits, so editing it changes only what the reader changed.
pub(crate) fn apply_binding_change(
    definition: &mut ConfigurationSetDefinition,
    path: &str,
    inherited: &[String],
    change: BindingChange,
) {
    let exact_key = InstancePath::parse_legacy(path)
        .ok()
        .map(|exact| exact.fold_key());
    let position = definition.overrides.iter().position(|binding| {
        binding.instance_path.eq_ignore_ascii_case(path)
            || exact_key.as_ref().is_some_and(|key| {
                InstancePath::parse_legacy(&binding.instance_path)
                    .is_ok_and(|exact| &exact.fold_key() == key)
            })
    });
    let index = match (position, &change) {
        (Some(index), _) => index,
        (None, BindingChange::Remove | BindingChange::Path(_)) => return,
        (None, _) => {
            definition.overrides.push(ConfigurationSetOverride {
                instance_path: exact_key
                    .and_then(|_| InstancePath::parse_legacy(path).ok())
                    .map_or_else(|| path.to_owned(), |exact| exact.to_string()),
                executable_views: inherited.to_vec(),
                stop_view: None,
                model_section: None,
                eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
            });
            definition.overrides.len() - 1
        }
    };
    let binding = &mut definition.overrides[index];
    match change {
        BindingChange::Views(edit) => {
            edit.apply(&mut binding.executable_views);
            let views = binding.executable_views.clone();
            binding.stop_view = binding
                .stop_view
                .take()
                .filter(|stop| views.iter().any(|view| view.eq_ignore_ascii_case(stop)));
        }
        BindingChange::Stop(stop) => binding.stop_view = stop,
        BindingChange::ModelSection(section) => {
            binding.model_section = section
                .map(|section| section.trim().to_owned())
                .filter(|section| !section.is_empty());
        }
        BindingChange::RunsOn(platforms) => binding.eligible_platforms = platforms,
        BindingChange::Path(pattern) => binding.instance_path = pattern.trim().to_owned(),
        BindingChange::Remove => {
            definition.overrides.remove(index);
        }
    }
}

/// Where a binding runs, in the page's words.
pub(crate) fn runs_on_label(platforms: &[ConfigurationPlatform]) -> &'static str {
    let desktop = platforms.contains(&ConfigurationPlatform::Desktop);
    let browser = platforms.contains(&ConfigurationPlatform::Browser);
    match (desktop, browser) {
        (true, false) => "Desktop",
        (false, true) => "Browser",
        _ => "All",
    }
}

/// Every view name the project's libraries hold, once, case-insensitively.
fn project_view_names(libraries: &LibraryManager) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for (_, library) in libraries.libraries_by_key() {
        for cell in library.cells.values() {
            for view in cell.views.values() {
                if !names
                    .iter()
                    .any(|existing| existing.eq_ignore_ascii_case(&view.name))
                {
                    names.push(view.name.clone());
                }
            }
        }
    }
    names.sort_by_key(|name| name.to_lowercase());
    names
}

/// The views `library`/`cell` holds, by name.
pub(crate) fn cell_view_names(
    libraries: &LibraryManager,
    library: &str,
    cell: &str,
) -> Vec<String> {
    let mut names = libraries
        .libraries_by_key()
        .filter(|(key, _)| key.eq_ignore_ascii_case(library))
        .flat_map(|(_, found)| found.cells.iter())
        .filter(|(key, _)| key.eq_ignore_ascii_case(cell))
        .flat_map(|(_, found)| found.views.values())
        .filter(|view| view.view_type != ViewType::Symbol)
        .map(|view| view.name.clone())
        .collect::<Vec<_>>();
    names.sort_by_key(|name| name.to_lowercase());
    names
}

/// The schematic and testbench views a configuration can take as its top
/// cell: those open in the project, so they can be netlisted.
fn top_cell_choices(workspace: &ProjectWorkspace, libraries: &LibraryManager) -> Vec<CellViewRef> {
    let mut roots = Vec::new();
    for (library_key, library) in libraries.libraries_by_key() {
        for (cell_key, cell) in &library.cells {
            for (view_key, view) in &cell.views {
                if !matches!(view.view_type, ViewType::Schematic | ViewType::Testbench) {
                    continue;
                }
                let reference = CellViewRef::new(library_key, cell_key, view_key);
                let open = reference
                    .key()
                    .eq_ignore_ascii_case(&workspace.content.active_view.key())
                    || workspace
                        .content
                        .schematic_buffers
                        .keys()
                        .any(|key| key.eq_ignore_ascii_case(&reference.key()));
                if open {
                    roots.push(reference);
                }
            }
        }
    }
    roots.sort_by_key(|reference| reference.key().to_lowercase());
    roots
}

/// A refusal from the configuration model, in the page's words: what is
/// wrong and what to do about it.
pub(crate) fn plain_refusal(error: &ConfigurationSetError) -> String {
    fn field(field: &str) -> &'static str {
        match field {
            "configuration.name" => "Name",
            "configuration.dut-path" => "DUT instance",
            "configuration.executable-view-policy" => "View list",
            "configuration.stop-views" => "Stop list",
            "configuration.override.path" => "An instance binding's path",
            "configuration.override.executable-views" => "An instance binding's view list",
            "configuration.override.stop-view" => "An instance binding's stop view",
            "configuration.override.model-section" => "Model section",
            "configuration.cell-binding.library" => "A cell binding's library",
            "configuration.cell-binding.cell" => "A cell binding's cell",
            "configuration.cell-binding.executable-views" => "A cell binding's view list",
            "configuration.cell-binding.stop-view" => "A cell binding's stop view",
            _ => "A field",
        }
    }
    match error {
        ConfigurationSetError::RevisionConflict { .. } => {
            "It changed while you were editing. Revert, then make your edits again.".to_owned()
        }
        ConfigurationSetError::DuplicateName(name) => {
            format!("Another configuration is named {name}. Pick another name.")
        }
        ConfigurationSetError::InvalidText {
            field: name,
            maximum_bytes,
        } => format!(
            "{} must not be empty, start or end with a space, or exceed {maximum_bytes} characters.",
            field(name)
        ),
        ConfigurationSetError::InvalidInstancePath {
            field: name, path, ..
        }
        | ConfigurationSetError::InvalidHierarchyPattern {
            field: name,
            pattern: path,
            ..
        } => format!("{} {path} is not an instance path.", field(name)),
        ConfigurationSetError::EngineSpelledInstancePath {
            field: name,
            path,
            display,
        } => format!(
            "{} {path} is the netlist spelling; write {display}.",
            field(name)
        ),
        ConfigurationSetError::InvalidViewPolicyLength {
            field: name,
            actual,
            maximum,
            require_nonempty,
        } => {
            if *require_nonempty && *actual == 0 {
                format!("{} is empty. Add at least one view.", field(name))
            } else {
                format!(
                    "{} holds {actual} views; it can hold {maximum}.",
                    field(name)
                )
            }
        }
        ConfigurationSetError::InvalidViewName {
            field: name,
            view,
            reason,
        } => format!("{}: {view} is not a view name ({reason}).", field(name)),
        ConfigurationSetError::DuplicateView { field: name, view } => {
            format!("{} names {view} twice. Remove one.", field(name))
        }
        ConfigurationSetError::StopViewOutsidePolicy { field: name, view } => format!(
            "{}: {view} is not in its view list. Add it to the view list or remove the stop.",
            field(name)
        ),
        ConfigurationSetError::DuplicateOverridePath(path) => {
            format!("{path} is bound twice. Remove one binding.")
        }
        ConfigurationSetError::AmbiguousOverridePatterns { left, right } => format!(
            "Pattern bindings {left} and {right} match the same instances equally. Make one more specific."
        ),
        ConfigurationSetError::NoEligiblePlatforms(path) => {
            format!("{path} runs on no platform. Set Runs on to All, Desktop or Browser.")
        }
        ConfigurationSetError::InvalidModelSection(section) => {
            format!("{section} is not a model section name.")
        }
        ConfigurationSetError::DuplicateCellBinding { library, cell } => {
            format!("{library}/{cell} has two cell bindings. Remove one.")
        }
        ConfigurationSetError::InvalidCellBindingName {
            field: name,
            name: value,
            reason,
        } => format!("{}: {value} is not a name ({reason}).", field(name)),
        ConfigurationSetError::ActiveConfigurationRemoval(_) => {
            "Runs use this configuration. Use another one (or Default) for runs, then delete it."
                .to_owned()
        }
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests;

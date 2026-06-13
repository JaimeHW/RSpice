use crate::common::app::{AppState, ConsoleMessage, RSpiceApp};
use crate::state::{CellViewRef, ComponentType, SchematicState, View, ViewType};

fn view_type_for_reference(state: &AppState, reference: &CellViewRef) -> ViewType {
    state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
        .unwrap_or(ViewType::Schematic)
}

fn schematic_for_workspace(state: &mut AppState, reference: &CellViewRef) -> SchematicState {
    let mut schematic = state
        .workspace
        .schematic_buffers
        .get(&reference.key())
        .cloned()
        .unwrap_or_default();
    // Workspace buffers round-trip through serde, which skips the runtime
    // ID counter and name counters: without recalculation a freshly placed
    // component reuses an existing ID and selection matches both.
    schematic.recalculate_runtime_state();
    // Views from read-only libraries open for inspection, never for edit —
    // the docbar banner explains and every edit path checks this flag.
    schematic.read_only = state
        .library_manager
        .get_library(&reference.library)
        .is_some_and(|library| library.read_only);
    schematic
}

impl AppState {
    /// Migrate persisted sessions away from the legacy seeded "primitives"
    /// library: any workspace tab or schematic buffer pointing into it is
    /// moved to the user library (preserving drawn content), then the
    /// placeholder library is dropped. Without the workspace scrub,
    /// `ensure_library_model` would faithfully re-create the legacy cell
    /// the active tab points at on every launch.
    pub(crate) fn migrate_legacy_primitives(&mut self) {
        const LEGACY: &str = crate::state::LibraryManager::PRIMITIVES_LIBRARY;
        let user = crate::state::LibraryManager::USER_LIBRARY;
        let legacy_prefix = format!("{LEGACY}/");

        // Move schematic buffers (the actual drawn content) to user keys.
        let legacy_keys: Vec<String> = self
            .workspace
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&legacy_prefix))
            .cloned()
            .collect();
        for key in legacy_keys {
            let Some(buffer) = self.workspace.schematic_buffers.remove(&key) else {
                continue;
            };
            let tail = &key[legacy_prefix.len()..];
            let new_key = format!("{user}/{tail}");
            self.workspace
                .schematic_buffers
                .entry(new_key)
                .or_insert(buffer);

            // Make the migrated cell/view exist in the user library so the
            // browser lists it and tabs resolve.
            if let Some((cell_name, view_name)) = tail.split_once('/')
                && let Some(library) = self.library_manager.get_library_mut(user)
            {
                if library.get_cell(cell_name).is_none() {
                    library.add_cell(crate::state::Cell::new(cell_name));
                }
                if let Some(cell) = library.get_cell_mut(cell_name)
                    && cell.get_view(view_name).is_none()
                {
                    cell.add_view(crate::state::View::new(
                        view_name,
                        crate::state::ViewType::Schematic,
                    ));
                }
            }
        }

        // Repoint tabs, breadcrumbs, and the active view.
        let remap = |reference: &mut CellViewRef| {
            if reference.library == LEGACY {
                reference.library = user.to_string();
            }
        };
        remap(&mut self.workspace.active_view);
        for open in &mut self.workspace.open_views {
            remap(&mut open.reference);
        }
        for entry in &mut self.workspace.hierarchy_stack {
            remap(entry);
        }

        self.library_manager.purge_legacy_primitives();
    }

    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        self.workspace.save_active_schematic(&self.schematic);
        self.sync_generated_symbol_view();
    }

    /// Keep the active cell's generated "symbol" view in step with its
    /// schematic interface — created when ports exist, refreshed when they
    /// change, removed when the last port goes. A hand-authored symbol
    /// view (no `generated` marker) is never touched: the user owns it.
    fn sync_generated_symbol_view(&mut self) {
        const GENERATED_KEY: &str = "generated";
        const PORTS_KEY: &str = "ports";

        let reference = self.workspace.active_view.clone();
        if !reference.view.eq_ignore_ascii_case("schematic") {
            return;
        }
        let ports = self.schematic.interface_ports();
        let Some(cell) = self
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
        else {
            return;
        };

        if ports.is_empty() {
            if cell
                .get_view("symbol")
                .is_some_and(|view| view.metadata.contains_key(GENERATED_KEY))
            {
                cell.remove_view("symbol");
            }
            return;
        }

        let encoded: String = ports
            .iter()
            .map(|port| format!("{}:{}", port.name, port.direction.keyword()))
            .collect::<Vec<_>>()
            .join(" ");
        match cell.get_view_mut("symbol") {
            Some(view) if view.metadata.contains_key(GENERATED_KEY) => {
                view.metadata.insert(PORTS_KEY.to_owned(), encoded);
            }
            Some(_) => {} // hand-authored symbol: leave it alone
            None => {
                let mut view = View::new("symbol", ViewType::Symbol);
                view.metadata
                    .insert(GENERATED_KEY.to_owned(), "ports".to_owned());
                view.metadata.insert(PORTS_KEY.to_owned(), encoded);
                cell.add_view(view);
            }
        }
    }

    pub(crate) fn restore_active_schematic_from_workspace(&mut self) {
        self.workspace
            .ensure_library_model(&mut self.library_manager);
        let reference = self.workspace.active_view.clone();
        self.schematic = schematic_for_workspace(self, &reference);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
    }

    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        self.sync_active_schematic_to_workspace();
        if self.workspace.active_view == reference {
            return;
        }
        let view_type = view_type_for_reference(self, &reference);
        self.workspace.open_as_root(reference.clone(), view_type);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
            self.schematic = schematic_for_workspace(self, &reference);
        }
        self.push_user_message(ConsoleMessage::info(format!(
            "Opened {}",
            reference.display_path()
        )));
    }

    /// Descend into a hierarchical instance: open its master and record
    /// the instance name on the occurrence path. `None` (no instance
    /// context) labels the level with the cell name.
    pub(crate) fn descend_into_instance(
        &mut self,
        instance: Option<String>,
        reference: CellViewRef,
    ) {
        self.sync_active_schematic_to_workspace();
        let view_type = view_type_for_reference(self, &reference);
        match instance {
            Some(name) => {
                self.workspace
                    .descend_into(name, reference.clone(), view_type)
            }
            None => self.workspace.enter_hierarchy(reference.clone(), view_type),
        }
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
            self.schematic = schematic_for_workspace(self, &reference);
        }
        self.push_user_message(ConsoleMessage::info(format!(
            "Entered {}",
            reference.display_path()
        )));
    }

    /// Copy a whole cell — every view and its drawn content — into a
    /// writable library under a new name. Returns the number of views
    /// copied, or the user-facing error.
    pub(crate) fn copy_cell(
        &mut self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let source = self
            .library_manager
            .get_library(src_library)
            .ok_or_else(|| format!("Library '{src_library}' not found"))?;
        let mut copy = source
            .get_cell(cell)
            .ok_or_else(|| format!("Cell '{cell}' not found in library '{src_library}'"))?
            .clone();
        let destination = self
            .library_manager
            .get_library(dst_library)
            .ok_or_else(|| format!("Library '{dst_library}' not found"))?;
        if destination.read_only {
            return Err(format!("Library '{dst_library}' is read-only"));
        }
        if destination.get_cell(new_name).is_some() {
            return Err(format!(
                "Cell '{new_name}' already exists in library '{dst_library}'"
            ));
        }

        copy.name = new_name.to_owned();
        let view_names: Vec<String> = copy.views.keys().cloned().collect();
        let view_count = view_names.len();
        if let Some(destination) = self.library_manager.get_library_mut(dst_library) {
            destination.add_cell(copy);
        }

        // The drawn content lives in the workspace buffers, keyed
        // "library/cell/view" — a copy without it would be a lie.
        for view in view_names {
            let old_key = CellViewRef::new(src_library, cell, view.as_str()).key();
            let new_key = CellViewRef::new(dst_library, new_name, view.as_str()).key();
            if let Some(buffer) = self.workspace.schematic_buffers.get(&old_key).cloned() {
                self.workspace.schematic_buffers.insert(new_key, buffer);
            }
        }

        self.library_manager.select_cell(dst_library, new_name);
        Ok(view_count)
    }

    /// Rename a cell and remap everything that pointed at it: view buffers,
    /// open workspace references, and the Library/Cell binding of every
    /// instance in this project's designs. Returns the number of instance
    /// bindings remapped, or the user-facing error.
    pub(crate) fn rename_cell(
        &mut self,
        library: &str,
        cell: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let lib = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if lib.read_only {
            return Err(format!("Library '{library}' is read-only"));
        }
        if lib.get_cell(cell).is_none() {
            return Err(format!("Cell '{cell}' not found in library '{library}'"));
        }
        if lib.get_cell(new_name).is_some() {
            return Err(format!(
                "Cell '{new_name}' already exists in library '{library}'"
            ));
        }

        if let Some(lib) = self.library_manager.get_library_mut(library) {
            if let Some(mut moved) = lib.cells.remove(cell) {
                moved.name = new_name.to_owned();
                lib.cells.insert(new_name.to_owned(), moved);
            }
        }

        // Buffers move with the cell.
        let old_prefix = format!("{library}/{cell}/");
        let moved_keys: Vec<String> = self
            .workspace
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&old_prefix))
            .cloned()
            .collect();
        for key in moved_keys {
            if let Some(buffer) = self.workspace.schematic_buffers.remove(&key) {
                let tail = &key[old_prefix.len()..];
                self.workspace
                    .schematic_buffers
                    .insert(format!("{library}/{new_name}/{tail}"), buffer);
            }
        }

        // Open references follow.
        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library && reference.cell == cell {
                reference.cell = new_name.to_owned();
            }
        };
        remap_ref(&mut self.workspace.active_view);
        for reference in &mut self.workspace.hierarchy_stack {
            remap_ref(reference);
        }

        // Instance bindings follow — in every buffer and the live sheet.
        let mut remapped = 0usize;
        let mut remap_schematic = |schematic: &mut crate::state::SchematicState| {
            for component in &mut schematic.components {
                if let Some(binding) = component.library_cell.as_mut()
                    && binding.library == library
                    && binding.cell == cell
                {
                    binding.cell = new_name.to_owned();
                    remapped += 1;
                }
            }
        };
        for buffer in self.workspace.schematic_buffers.values_mut() {
            remap_schematic(buffer);
        }
        remap_schematic(&mut self.schematic);

        self.library_manager.select_cell(library, new_name);
        Ok(remapped)
    }

    /// Refuse an edit on a read-only view, with the console line that names
    /// the library. Returns true when the edit must be blocked.
    pub(crate) fn deny_read_only_edit(&mut self) -> bool {
        if !self.schematic.read_only {
            return false;
        }
        let library = self.workspace.active_view.library.clone();
        self.push_user_message(ConsoleMessage::warning(format!(
            "Read-only — '{library}' masters cannot be edited"
        )));
        true
    }

    /// Ascend one hierarchy level (the U gesture / pathbar action).
    pub(crate) fn ascend_workspace_level(&mut self) {
        let len = self.workspace.hierarchy_stack.len();
        if len >= 2 {
            self.focus_workspace_breadcrumb(len - 2);
        }
    }

    pub(crate) fn focus_workspace_breadcrumb(&mut self, index: usize) {
        self.sync_active_schematic_to_workspace();
        if let Some(reference) = self.workspace.focus_breadcrumb(index) {
            self.library_manager
                .select_view(&reference.library, &reference.cell, &reference.view);
            self.schematic = schematic_for_workspace(self, &reference);
        }
    }

    pub(crate) fn open_selected_instance_master(&mut self) {
        let Some(component_id) = self.schematic.selection.single_component() else {
            self.push_user_message(ConsoleMessage::warning(
                "Select one hierarchical instance first",
            ));
            return;
        };

        let Some(component) = self
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)
        else {
            return;
        };

        if component.kind != ComponentType::CellInstance {
            self.push_user_message(ConsoleMessage::warning(
                "Selected component is not a hierarchical instance",
            ));
            return;
        }

        let Some(binding) = component.library_cell.as_ref() else {
            self.push_user_message(ConsoleMessage::warning(
                "Selected instance has no Library/Cell/View binding",
            ));
            return;
        };

        let instance_name = component.name.clone();
        let reference = CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        );
        self.descend_into_instance(Some(instance_name), reference);
    }
}

impl RSpiceApp {
    pub(crate) fn restore_workspace_after_project_load(&mut self) {
        self.state.restore_active_schematic_from_workspace();
        self.state.clear_transient_specialized_viewer_data();
    }
}

#[cfg(test)]
mod tests {
    use crate::common::app::AppState;
    use crate::state::{Cell, ComponentType, Library, Point, View, ViewType};

    /// A persisted session whose active tab points into the legacy seeded
    /// "primitives" library must come back with the drawn content moved to
    /// the user library and the placeholder library gone — otherwise
    /// `ensure_library_model` resurrects the placeholder cell every launch.
    #[test]
    fn legacy_primitives_content_migrates_to_user_library() {
        let mut state = AppState::default();

        // Simulate the legacy session: seeded library + an open tab with
        // a drawn schematic inside it.
        let mut legacy = Library::new("primitives");
        let mut cell = Cell::new("ISource AC");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        legacy.add_cell(cell);
        state.library_manager.add_library(legacy);

        let mut drawn = crate::state::SchematicState::default();
        drawn.add_component(ComponentType::Njfet, Point::new(0, 0));
        state
            .workspace
            .schematic_buffers
            .insert("primitives/ISource AC/schematic".to_owned(), drawn);
        state.workspace.active_view =
            crate::state::CellViewRef::new("primitives", "ISource AC", "schematic");

        state.migrate_legacy_primitives();

        assert!(
            state.library_manager.get_library("primitives").is_none(),
            "legacy library must be gone"
        );
        assert_eq!(state.workspace.active_view.library, "user");
        let migrated = state
            .workspace
            .schematic_buffers
            .get("user/ISource AC/schematic")
            .expect("buffer migrated to the user library");
        assert_eq!(migrated.components.len(), 1, "drawn content preserved");
        assert!(
            state
                .library_manager
                .get_library("user")
                .and_then(|lib| lib.get_cell("ISource AC"))
                .is_some(),
            "migrated cell exists in the user library"
        );
    }
}

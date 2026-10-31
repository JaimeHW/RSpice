//! New View: a command form stating the target library and cell, asking for
//! a view name and the view type.

use super::shared::{DialogActionOutcome, LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use super::{Context, RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::workspace::{
    starter_veriloga_source, verilog_module_name, verilog_port_declaration,
};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::AppState;
use crate::workbench::app_state::Model;
use rspice_design::symbol::generated::{GENERATED_SYMBOL_VIEW, store_generated_symbol};

/// Cell-view kinds backed by a complete, editable production surface.
///
/// Other persisted `ViewType` variants remain valid for project interchange,
/// but must not be offered by the creation workflow until their editor and
/// storage contract are implemented end to end.
const CREATABLE_VIEW_TYPES: [crate::state::ViewType; 5] = [
    crate::state::ViewType::Schematic,
    crate::state::ViewType::Testbench,
    crate::state::ViewType::Symbol,
    crate::state::ViewType::Layout,
    crate::state::ViewType::VerilogA,
];

const fn is_creatable_view_type(view_type: crate::state::ViewType) -> bool {
    matches!(
        view_type,
        crate::state::ViewType::Schematic
            | crate::state::ViewType::Testbench
            | crate::state::ViewType::Symbol
            | crate::state::ViewType::Layout
            | crate::state::ViewType::VerilogA
    )
}

/// The cell New View adds a view to when it is invoked as a command: the cell
/// selected in the library, else the cell of the active document, while its
/// library can be written.
pub(crate) fn new_view_target(model: &Model) -> Option<(String, String)> {
    let selected = model
        .library_manager
        .selected_library
        .as_deref()
        .zip(model.library_manager.selected_cell.as_deref());
    let active = &model.workspace.content.active_view;
    selected
        .into_iter()
        .chain([(active.library.as_str(), active.cell.as_str())])
        .find(|(library, cell)| {
            model
                .library_manager
                .get_library(library)
                .is_some_and(|library| !library.read_only && library.get_cell(cell).is_some())
        })
        .map(|(library, cell)| (library.to_owned(), cell.to_owned()))
}

impl AppState {
    /// Open New View against an exact, currently writable cell identity.
    ///
    /// The target is resolved before any dialog state changes. This keeps a
    /// stale row click from opening an apparently valid transaction against a
    /// different or read-only object.
    pub(crate) fn open_new_view_dialog(&mut self, library: &str, cell: &str) -> Result<(), String> {
        self.open_new_view_dialog_as(library, cell, None)
    }

    /// [`Self::open_new_view_dialog`] preset to `view_type`, so a "Create
    /// symbol from schematic" row opens New view on the active cell's symbol.
    /// `None` picks the first type the cell does not have yet.
    pub(crate) fn open_new_view_dialog_as(
        &mut self,
        library: &str,
        cell: &str,
        view_type: Option<crate::state::ViewType>,
    ) -> Result<(), String> {
        self.project
            .open_new_view_dialog(&self.model, library, cell, view_type)
    }
}

/// The view type New View opens on for the exact cell `library`/`cell`:
/// `view_type` when New View creates that type, else the first type it
/// creates that the cell does not have yet. A cell that no longer exists,
/// or whose library is read-only, is refused.
pub(crate) fn new_view_opening(
    model: &Model,
    library: &str,
    cell: &str,
    view_type: Option<crate::state::ViewType>,
) -> Result<crate::state::ViewType, String> {
    let target = model
        .library_manager
        .get_library(library)
        .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
    if target.read_only {
        return Err(format!(
            "Library '{library}' is read-only; a new view cannot be created."
        ));
    }
    let cell_target = target
        .get_cell(cell)
        .ok_or_else(|| format!("Cell '{library}/{cell}' no longer exists."))?;
    Ok(view_type
        .filter(|view_type| is_creatable_view_type(*view_type))
        .or_else(|| {
            CREATABLE_VIEW_TYPES.into_iter().find(|view_type| {
                !cell_target
                    .views
                    .values()
                    .any(|view| view.view_type == *view_type)
            })
        })
        .unwrap_or(crate::state::ViewType::Schematic))
}

/// Why Create is waiting while the view has no name.
const NEW_VIEW_NAME_MISSING: &str = "Enter a name for the view";

/// What the line under the view name says when Create cannot run on it.
enum NameLine {
    /// Nothing typed yet: what Create waits for, not a refusal.
    Missing,
    /// A name the library would refuse, and why.
    Refused(String),
}

fn new_view_name_line(name: &str) -> Option<NameLine> {
    let name = name.trim();
    if name.is_empty() {
        return Some(NameLine::Missing);
    }
    validate_lcv_name(name, "View name").map(NameLine::Refused)
}

impl RSpiceApp {
    pub(in crate::workbench) fn process_new_view_dialog(&mut self, ctx: &Context) {
        if !self.state.project.new_view_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;
        let mut persist_global_veriloga = false;

        let catalog_current = self.state.model.library_manager.revision()
            == self.state.project.new_view_library_revision;
        if !catalog_current {
            self.state.project.new_view_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut self.state.project;
        let name_refusal = new_view_name_line(&dialogs.new_view_name);
        let can_create = name_refusal.is_none()
            && !dialogs.new_view_library.is_empty()
            && !dialogs.new_view_cell.is_empty()
            && is_creatable_view_type(dialogs.new_view_type)
            && catalog_current;
        let error = dialogs.new_view_error.clone();
        let name_id = egui::Id::new("library.new-view.name");
        let type_names = CREATABLE_VIEW_TYPES.map(|view_type| view_type.display_name());
        let mut type_index = CREATABLE_VIEW_TYPES
            .iter()
            .position(|view_type| *view_type == dialogs.new_view_type)
            .unwrap_or_default();

        let choice = CommandForm::new("New view", "Create")
            .describe("Create a named view of the selected type in this library cell.")
            .primary_enabled(can_create)
            // A refusal of the name stands under it; the slot holds only what
            // belongs to no field, so it reserves nothing while empty.
            .status(error.as_deref().map_or(Ok(None), Err))
            .status_fixed_at_open()
            .show(ctx, |rows| {
                rows.value_mono("Library", &dialogs.new_view_library);
                rows.value_mono("Cell", &dialogs.new_view_cell);
                rows.text("View name", name_id, &mut dialogs.new_view_name, "");
                rows.field_line(
                    Some(name_id),
                    match &name_refusal {
                        None => FieldLine::Blank,
                        Some(NameLine::Missing) => FieldLine::Derived(NEW_VIEW_NAME_MISSING),
                        Some(NameLine::Refused(refusal)) => FieldLine::Error(refusal),
                    },
                );
                if rows.choice("Type", &type_names, &mut type_index) {
                    dialogs.new_view_type = CREATABLE_VIEW_TYPES[type_index];
                }
                Some(name_id)
            });

        match choice {
            DialogChoice::Primary => should_create = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_create {
            let outcome = self.handle_new_view_create_action();
            should_close |= outcome.close;
            persist_global_veriloga |= outcome.persist_global_veriloga;
        }

        if persist_global_veriloga {
            self.persist_global_veriloga_library_with_feedback();
        }

        if should_close {
            self.state.project.new_view_dialog = false;
            self.state.project.new_view_name.clear();
            self.state.project.new_view_error = None;
        }
    }

    pub(super) fn handle_new_view_create_action(&mut self) -> DialogActionOutcome {
        let mut outcome = DialogActionOutcome::default();
        let view_name = self.state.project.new_view_name.trim().to_string();
        let library = self.state.project.new_view_library.clone();
        let cell = self.state.project.new_view_cell.clone();

        if self.state.model.library_manager.revision()
            != self.state.project.new_view_library_revision
        {
            self.state.project.new_view_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
            return outcome;
        }
        if let Some(error) = validate_lcv_name(&view_name, "View name") {
            self.state.project.new_view_error = Some(error);
            return outcome;
        }
        if library.is_empty() {
            self.state.project.new_view_error = Some("Library cannot be empty".to_string());
            return outcome;
        }
        if cell.is_empty() {
            self.state.project.new_view_error = Some("Cell cannot be empty".to_string());
            return outcome;
        }
        let view_type = self.state.project.new_view_type;
        if !is_creatable_view_type(view_type) {
            self.state.project.new_view_error = Some(format!(
                "{} views cannot be created until their production editor and storage workflow are available",
                view_type.display_name()
            ));
            return outcome;
        }

        let Some(lib_ro) = self.state.model.library_manager.get_library(&library) else {
            self.state.project.new_view_error = Some(format!("Library '{}' not found", library));
            return outcome;
        };
        if lib_ro.read_only {
            self.state.project.new_view_error = Some(format!(
                "Library '{library}' is read only; create the view in an editable library"
            ));
            return outcome;
        }
        let Some(cell_ro) = lib_ro.get_cell(&cell) else {
            self.state.project.new_view_error = Some(format!(
                "Cell '{}' not found in library '{}'",
                cell, library
            ));
            return outcome;
        };
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(&library, &cell, &view_name);
        if cell_ro.views.values().any(|view| {
            crate::state::canonical_cell_view_owner_key(&library, &cell, &view.name)
                == requested_identity
        }) {
            self.state.project.new_view_error = Some(format!(
                "View '{}' clashes with an existing view in cell '{}' (view names ignore letter case)",
                view_name, cell
            ));
            return outcome;
        }

        if view_type == crate::state::ViewType::Layout {
            return self.create_layout_cell_view(&library, &cell, &view_name);
        }
        if view_type == crate::state::ViewType::VerilogA {
            return self.create_veriloga_cell_view(&library, &cell, &view_name);
        }

        use crate::state::View;

        let seeded_view = if view_type == crate::state::ViewType::Symbol {
            let ports = cell_schematic_ports(&self.state, &library, &cell);
            match seeded_symbol_view(&view_name, &ports) {
                Ok(view) => view,
                Err(error) => {
                    self.state.project.new_view_error = Some(error);
                    return outcome;
                }
            }
        } else {
            View::new(&view_name, view_type)
        };

        let project_mutation = match self.state.model.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CreateView {
                library: library.clone(),
                cell: cell.clone(),
                view: view_name.clone(),
            },
        ) {
            Ok(prepared) => prepared,
            Err(error) => {
                self.state.project.new_view_error = Some(error);
                return outcome;
            }
        };
        let seeded_schematic = self.state.new_schematic_document();
        if let Some(mut lib) = self.state.model.library_manager.edit_library(&library) {
            if lib.add_view(&cell, seeded_view).is_some() {
                if matches!(
                    view_type,
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
                ) {
                    let reference = crate::state::CellViewRef::new(&library, &cell, &view_name);
                    self.state
                        .model
                        .workspace
                        .insert_schematic_editor(reference.key(), seeded_schematic);
                }
                self.state
                    .model
                    .publish_project_library_mutation(project_mutation);
                self.state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Created view '{view_name}' in cell '{cell}'."),
                );
                if matches!(
                    view_type,
                    crate::state::ViewType::Schematic
                        | crate::state::ViewType::Testbench
                        | crate::state::ViewType::Symbol
                ) {
                    self.state
                        .open_workspace_view(crate::state::CellViewRef::new(
                            library.clone(),
                            cell.clone(),
                            view_name.clone(),
                        ));
                }
                self.state.project.new_view_error = None;
                outcome.close = true;
                outcome.persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
            } else {
                self.state.project.new_view_error = Some(format!(
                    "Cell '{}' not found in library '{}'",
                    cell, library
                ));
            }
        } else {
            self.state.project.new_view_error = Some(format!("Library '{}' not found", library));
        }

        outcome
    }

    fn create_layout_cell_view(
        &mut self,
        library: &str,
        cell: &str,
        view_name: &str,
    ) -> DialogActionOutcome {
        let mut outcome = DialogActionOutcome::default();
        let reference = crate::state::CellViewRef::new(library, cell, view_name);
        let technology = match self.state.exact_project_layout_technology_binding() {
            Ok(technology) => technology,
            Err(error) => {
                self.state.project.new_view_error = Some(format!(
                    "A Layout view requires the project's validated, signed PDK: {error}"
                ));
                return outcome;
            }
        };
        let document =
            match crate::state::PhysicalLayoutDocument::try_new(reference.clone(), technology) {
                Ok(document) => document,
                Err(error) => {
                    self.state.project.new_view_error =
                        Some(format!("The physical-layout document is invalid: {error}"));
                    return outcome;
                }
            };
        let candidate_layouts = match self
            .state
            .model
            .workspace
            .content
            .prepare_insert_physical_layout_document(document)
        {
            Ok(candidate) => candidate,
            Err(error) => {
                self.state.project.new_view_error = Some(format!(
                    "The physical-layout document could not be retained: {error}"
                ));
                return outcome;
            }
        };
        let mut candidate_libraries = self.state.model.library_manager.clone();
        let Some(mut candidate_library) = candidate_libraries.edit_library(library) else {
            self.state.project.new_view_error = Some(format!("Library '{library}' not found"));
            return outcome;
        };
        if candidate_library.library().read_only {
            self.state.project.new_view_error = Some(format!(
                "Library '{library}' became read only before the Layout view could be created"
            ));
            return outcome;
        }
        let Some(candidate_cell) = candidate_library.library().get_cell(cell) else {
            self.state.project.new_view_error =
                Some(format!("Cell '{cell}' not found in library '{library}'"));
            return outcome;
        };
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(library, cell, view_name);
        if candidate_cell.views.values().any(|view| {
            crate::state::canonical_cell_view_owner_key(library, cell, &view.name)
                == requested_identity
        }) {
            self.state.project.new_view_error = Some(format!(
                "View '{view_name}' clashes with an existing view in cell '{cell}' (view names ignore letter case)"
            ));
            return outcome;
        }
        candidate_library.add_view(
            cell,
            crate::state::View::new(view_name, crate::state::ViewType::Layout),
        );
        let project_mutation = match self.state.model.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CreateView {
                library: library.to_owned(),
                cell: cell.to_owned(),
                view: view_name.to_owned(),
            },
        ) {
            Ok(prepared) => prepared,
            Err(error) => {
                self.state.project.new_view_error = Some(error);
                return outcome;
            }
        };

        self.state.model.library_manager = candidate_libraries;
        self.state
            .model
            .workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);
        self.state
            .model
            .publish_project_library_mutation(project_mutation);
        self.state.project.new_view_error = None;
        self.state.open_workspace_view(reference);
        self.state.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!("Created Layout view '{view_name}' for cell '{cell}' under the project PDK."),
        );
        outcome.close = true;
        outcome
    }

    fn create_veriloga_cell_view(
        &mut self,
        library: &str,
        cell: &str,
        view_name: &str,
    ) -> DialogActionOutcome {
        let mut outcome = DialogActionOutcome::default();
        let reference = crate::state::CellViewRef::new(library, cell, view_name);
        let ports = match derive_cell_interface(self, library, cell) {
            Ok(ports) => ports,
            Err(error) => {
                self.state.project.new_view_error = Some(error);
                return outcome;
            }
        };
        let module_name = verilog_module_name(cell);
        if let Some(invalid) = ports
            .iter()
            .find(|port| verilog_port_declaration(&port.name).is_none())
        {
            self.state.project.new_view_error = Some(format!(
                "Interface port '{}' is not a valid Verilog-A identifier; rename the cell port before creating this view",
                invalid.name
            ));
            return outcome;
        }
        let source = starter_veriloga_source(&module_name, &ports);
        let root_path = format!("{module_name}.va");
        let mut bundle = match crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference.clone()),
            crate::state::ProjectSourceLanguage::VerilogA,
            root_path,
            source,
            [],
            [],
        ) {
            Ok(bundle) => bundle,
            Err(error) => {
                self.state.project.new_view_error = Some(format!(
                    "The Verilog-A source bundle could not be created: {error}"
                ));
                return outcome;
            }
        };
        let receipt =
            match crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
                self.state.model.workspace.content.project.id(),
                &bundle,
                Some(&module_name),
            ) {
                Ok(receipt) => receipt,
                Err(diagnostics) => {
                    let detail = diagnostics
                        .first()
                        .map(|diagnostic| format!("{}: {}", diagnostic.message, diagnostic.detail))
                        .unwrap_or_else(|| "the compiler returned no diagnostic".to_owned());
                    self.state.project.new_view_error = Some(format!(
                        "The generated Verilog-A starter did not pass semantic compilation: {detail}"
                    ));
                    return outcome;
                }
            };
        if let Err(error) = bundle.mark_validated() {
            self.state.project.new_view_error = Some(format!(
                "The compiled Verilog-A source could not retain its validation identity: {error}"
            ));
            return outcome;
        }

        // Prepare both durable stores on clones. No live project mutation is
        // visible until the complete view/source transaction has succeeded.
        let mut candidate_sources = self.state.model.workspace.content.project_sources.clone();
        if let Err(error) = candidate_sources.insert_bundle(bundle) {
            self.state.project.new_view_error = Some(format!(
                "The Verilog-A source bundle could not be retained: {error}"
            ));
            return outcome;
        }
        let port_names = ports
            .iter()
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>();
        let encoded_ports = match serde_json::to_string(&port_names) {
            Ok(encoded) => encoded,
            Err(error) => {
                self.state.project.new_view_error = Some(format!(
                    "The Verilog-A interface contract could not be retained: {error}"
                ));
                return outcome;
            }
        };
        let mut view = crate::state::View::new(view_name, crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), module_name.clone());
        view.metadata
            .insert("veriloga.ports".to_owned(), encoded_ports);
        let mut candidate_libraries = self.state.model.library_manager.clone();
        let Some(mut candidate_library) = candidate_libraries.edit_library(library) else {
            self.state.project.new_view_error = Some(format!("Library '{library}' not found"));
            return outcome;
        };
        if candidate_library.library().read_only {
            self.state.project.new_view_error = Some(format!(
                "Library '{library}' became read only before the view could be created"
            ));
            return outcome;
        }
        let Some(candidate_cell) = candidate_library.library().get_cell(cell) else {
            self.state.project.new_view_error =
                Some(format!("Cell '{cell}' not found in library '{library}'"));
            return outcome;
        };
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(library, cell, view_name);
        if candidate_cell.views.values().any(|view| {
            crate::state::canonical_cell_view_owner_key(library, cell, &view.name)
                == requested_identity
        }) {
            self.state.project.new_view_error = Some(format!(
                "View '{view_name}' clashes with an existing view in cell '{cell}' (view names ignore letter case)"
            ));
            return outcome;
        }
        candidate_library.add_view(cell, view);

        let project_mutation = match self.state.model.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CreateView {
                library: library.to_owned(),
                cell: cell.to_owned(),
                view: view_name.to_owned(),
            },
        ) {
            Ok(prepared) => prepared,
            Err(error) => {
                self.state.project.new_view_error = Some(error);
                return outcome;
            }
        };
        self.state.model.workspace.content.project_sources = candidate_sources;
        self.state.model.workspace.content.project_sources_dirty = true;
        self.state.model.library_manager = candidate_libraries;
        self.state
            .model
            .publish_project_library_mutation(project_mutation);
        self.state.project.new_view_error = None;
        // The new view is a cell view, so it opens as a Design document.
        self.state.open_workspace_view(reference);
        self.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        self.state.code.code_workspace.veriloga.receipt = Some(receipt);
        self.state.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!(
                "Created Verilog-A view '{view_name}' for cell '{cell}' and opened its source."
            ),
        );
        outcome.close = true;
        outcome.persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
        outcome
    }
}

/// A symbol view seeded from a cell's interface `ports`.
///
/// [`GENERATED_SYMBOL_VIEW`] additionally carries the `generated` marker, so
/// the schematic keeps the symbol in step as ports come and go until the
/// first hand edit takes ownership by clearing the marker.
pub(super) fn seeded_symbol_view(
    view_name: &str,
    ports: &[crate::state::PortSpec],
) -> Result<crate::state::View, String> {
    let mut view = crate::state::View::new(view_name, crate::state::ViewType::Symbol);
    let stored = if view_name == GENERATED_SYMBOL_VIEW {
        store_generated_symbol(&mut view, ports, |_| None)
    } else {
        crate::state::SymbolDocument::generated_from_ports(ports).store_in_view(&mut view)
    };
    stored.map_err(|error| format!("The generated symbol could not be encoded: {error}"))?;
    Ok(view)
}

/// The interface a cell's schematic declares right now.
///
/// The view named `schematic` is the interface authority — it is the one the
/// generated-symbol synchronization reads — and the working copy of an open
/// document outranks its stored buffer.
fn cell_schematic_ports(
    state: &AppState,
    library: &str,
    cell: &str,
) -> Vec<crate::state::PortSpec> {
    let reference = crate::state::CellViewRef::new(library, cell, "schematic");
    if state.model.workspace.content.active_view == reference {
        return state.model.schematic.document().interface_ports();
    }
    state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&reference.key())
        .map(|schematic| schematic.document().interface_ports())
        .unwrap_or_default()
}

fn derive_cell_interface(
    app: &RSpiceApp,
    library_name: &str,
    cell_name: &str,
) -> Result<Vec<crate::state::PortSpec>, String> {
    let cell = app
        .state
        .model
        .library_manager
        .get_library(library_name)
        .and_then(|library| library.get_cell(cell_name))
        .ok_or_else(|| format!("Cell '{cell_name}' not found in library '{library_name}'"))?;
    let mut views = cell.views.values().collect::<Vec<_>>();
    views.sort_by(|left, right| {
        interface_view_rank(left)
            .cmp(&interface_view_rank(right))
            .then_with(|| left.name.cmp(&right.name))
    });

    let mut contracts: Vec<(String, Vec<crate::state::PortSpec>)> = Vec::new();
    for view in &views {
        if !matches!(
            view.view_type,
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        ) {
            continue;
        }
        let reference = crate::state::CellViewRef::new(library_name, cell_name, &view.name);
        let schematic = if app.state.model.workspace.content.active_view == reference {
            Some(app.state.model.schematic.editor_ref().design)
        } else {
            app.state
                .model
                .workspace
                .content
                .schematic_buffers
                .get(&reference.key())
        };
        if let Some(schematic) = schematic {
            let ports = schematic.document().interface_ports();
            if !ports.is_empty() {
                contracts.push((reference.display_path(), ports));
            }
        }
    }
    for view in views {
        if view.view_type != crate::state::ViewType::Symbol {
            continue;
        }
        let document = crate::state::SymbolDocument::load_from_view(view).map_err(|error| {
            format!(
                "The symbol interface in {library_name}/{cell_name}/{} is invalid: {error}",
                view.name
            )
        })?;
        if document.pins.is_empty() {
            continue;
        }
        let ports = document
            .pins
            .into_iter()
            .map(|pin| crate::state::PortSpec {
                name: pin.name,
                direction: pin.direction,
            })
            .collect::<Vec<_>>();
        contracts.push((format!("{library_name}/{cell_name}/{}", view.name), ports));
    }

    // A cell that declares no interface yet gets a module that declares none:
    // its first compile is what defines the ports. Inventing a `p`/`n` pair
    // here gave every such cell an interface nobody had asked for.
    let Some((authority_name, authority)) = contracts.first() else {
        return Ok(Vec::new());
    };
    validate_unique_ports(authority_name, authority)?;
    for (source_name, candidate) in contracts.iter().skip(1) {
        validate_unique_ports(source_name, candidate)?;
        if candidate != authority {
            return Err(format!(
                "The existing cell interfaces disagree between '{authority_name}' and '{source_name}'. Reconcile their ordered port names and directions before creating a Verilog-A view."
            ));
        }
    }
    Ok(authority.clone())
}

fn interface_view_rank(view: &crate::state::View) -> (u8, u8) {
    let kind = match view.view_type {
        crate::state::ViewType::Schematic => 0,
        crate::state::ViewType::Testbench => 1,
        crate::state::ViewType::Symbol => 2,
        _ => 3,
    };
    let canonical_name = u8::from(!matches!(
        view.name.as_str(),
        "schematic" | "testbench" | "symbol"
    ));
    (kind, canonical_name)
}

fn validate_unique_ports(source: &str, ports: &[crate::state::PortSpec]) -> Result<(), String> {
    let mut names = std::collections::HashSet::new();
    for port in ports {
        let key = port.name.to_ascii_lowercase();
        if !names.insert(key) {
            return Err(format!(
                "The interface in '{source}' declares port '{}' more than once",
                port.name
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, ComponentType, Library, NetNamingPolicy, Point, PortDirection, PortSpec,
        ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner, SchematicState,
        SymbolDocument, SymbolPin, View, ViewType,
    };
    use crate::workbench::ChoicePreference;

    fn schematic_with_ports(names: &[&str]) -> SchematicState {
        let mut schematic = SchematicState::default();
        for (index, name) in names.iter().enumerate() {
            let id = schematic.add_component(ComponentType::Port, Point::new(index as i32 * 20, 0));
            schematic
                .document_mut_for_test()
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .expect("the placed interface port")
                .value = (*name).to_owned();
        }
        schematic
    }

    #[test]
    fn new_view_entry_point_resolves_exact_writable_cell_before_opening() {
        let mut state = AppState::default();
        let mut library = Library::new("entry_point_test");
        let mut cell = Cell::new("filter");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        state.model.library_manager.add_library(library);

        state
            .open_new_view_dialog("entry_point_test", "filter")
            .expect("writable exact target should open");

        assert!(state.project.new_view_dialog);
        assert_eq!(state.project.new_view_library, "entry_point_test");
        assert_eq!(state.project.new_view_cell, "filter");
        assert_eq!(state.project.new_view_type, ViewType::Schematic);
        assert_eq!(state.project.new_view_name, "schematic");
        assert!(state.project.new_view_error.is_none());
        assert_eq!(
            state.project.new_view_library_revision,
            state.model.library_manager.revision()
        );
    }

    /// "Create symbol from schematic" opens New view on the symbol of the
    /// active cell, not on the first type the cell lacks (a testbench).
    #[test]
    fn new_view_opens_preset_to_a_requested_view_type() {
        let mut state = AppState::default();
        let mut library = Library::new("preset_test");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
        state.model.library_manager.add_library(library);

        state
            .open_new_view_dialog("preset_test", "amp")
            .expect("writable target");
        assert_eq!(state.project.new_view_type, ViewType::Testbench);

        state
            .open_new_view_dialog_as("preset_test", "amp", Some(ViewType::Symbol))
            .expect("writable target");
        assert!(state.project.new_view_dialog);
        assert_eq!(state.project.new_view_type, ViewType::Symbol);
        assert_eq!(state.project.new_view_name, "symbol");
    }

    /// New view is a command form: a 420 pt card with Library and Cell on
    /// its label/value grid, not a 760 pt transaction with its two facts
    /// pushed to the far edge; and an empty name says what Create waits for.
    #[test]
    fn new_view_is_a_command_form_that_says_what_create_waits_for() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("form_test");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state
            .open_new_view_dialog("form_test", "amp")
            .expect("writable target");
        app.state.project.new_view_name.clear();

        // Access updates are incremental: keep the latest of every node.
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..4 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| app.process_new_view_dialog(ctx),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        // The modal's own node is the whole screen. The surface is centred
        // on it, and Create ends 12 pt in from the surface's right edge.
        let create = nodes
            .values()
            .find(|node| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some("Create")
            })
            .and_then(egui::accesskit::Node::bounds)
            .expect("Create has bounds");
        let width = 2.0 * (create.x1 + 12.0) - 1440.0;
        assert!(width <= 424.0, "New view is {width} pt wide");
        assert!(nodes.values().any(|node| {
            node.role() == egui::accesskit::Role::TextInput && node.label() == Some("View name")
        }));
        assert!(
            nodes
                .values()
                .any(|node| node.value() == Some(NEW_VIEW_NAME_MISSING)),
            "Create does not say what it waits for"
        );
        assert!(matches!(
            new_view_name_line("bad name"),
            Some(NameLine::Refused(_))
        ));
        assert!(new_view_name_line("symbol").is_none());
    }

    #[test]
    fn new_view_entry_point_rejects_read_only_or_stale_target_without_mutation() {
        let mut state = AppState::default();
        let mut library = Library::new("entry_point_read_only");
        library.read_only = true;
        library.add_cell(Cell::new("filter"));
        state.model.library_manager.add_library(library);
        state.project.new_view_library = "sentinel".to_owned();

        let read_only_error = state
            .open_new_view_dialog("entry_point_read_only", "filter")
            .expect_err("read-only target must fail closed");
        let stale_error = state
            .open_new_view_dialog("missing", "filter")
            .expect_err("stale target must fail closed");

        assert!(read_only_error.contains("read-only"));
        assert!(stale_error.contains("no longer exists"));
        assert!(!state.project.new_view_dialog);
        assert_eq!(state.project.new_view_library, "sentinel");
    }

    #[test]
    fn creating_schematic_view_freezes_current_defaults_into_its_buffer() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("view_policy_test");
        library.add_cell(Cell::new("filter"));
        app.state.model.library_manager.add_library(library);
        app.state
            .model
            .preferences
            .set_choice(ChoicePreference::NetNamingPolicy, 1)
            .unwrap();
        app.state.project.new_view_library = "view_policy_test".to_owned();
        app.state.project.new_view_cell = "filter".to_owned();
        app.state.project.new_view_name = "schematic".to_owned();
        app.state.project.new_view_type = crate::state::ViewType::Schematic;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        let project_revision_before = app.state.model.workspace.content.project.revision().get();

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close);
        assert_eq!(
            app.state.model.workspace.content.project.revision().get(),
            project_revision_before + 1
        );
        assert!(matches!(
            app.state
                .model.workspace
                .content.project
                .library_mutation_audit()
                .last()
                .map(|receipt| receipt.mutation()),
            Some(crate::state::ProjectLibraryMutation::CreateView {
                library,
                cell,
                view,
            }) if library == "view_policy_test" && cell == "filter" && view == "schematic"
        ));
        let key = crate::state::CellViewRef::new("view_policy_test", "filter", "schematic").key();
        let document = app
            .state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&key)
            .expect("new schematic view owns an explicit document buffer");
        assert_eq!(
            document.document().document_policy.net_naming,
            NetNamingPolicy::SpiceCompatibleRelaxed
        );
    }

    #[test]
    fn new_symbol_view_reflects_the_cells_current_schematic_interface() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("interface_test");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state.model.workspace.insert_schematic_editor(
            CellViewRef::new("interface_test", "amp", "schematic").key(),
            schematic_with_ports(&["IN", "OUT"]),
        );
        app.state.project.new_view_library = "interface_test".to_owned();
        app.state.project.new_view_cell = "amp".to_owned();
        app.state.project.new_view_name = "symbol".to_owned();
        app.state.project.new_view_type = ViewType::Symbol;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close, "{:?}", app.state.project.new_view_error);
        let view = app
            .state
            .model
            .library_manager
            .get_library("interface_test")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("the created symbol view");
        assert_eq!(
            view.metadata.get("generated").map(String::as_str),
            Some("ports")
        );
        assert_eq!(
            view.metadata.get("ports").map(String::as_str),
            Some("IN:inout OUT:inout")
        );
        assert_eq!(
            SymbolDocument::load_from_view(view)
                .expect("the seeded symbol document parses")
                .pins
                .iter()
                .map(|pin| pin.name.as_str())
                .collect::<Vec<_>>(),
            ["IN", "OUT"],
            "a symbol created against an existing schematic starts on its ports"
        );
    }

    #[test]
    fn first_symbol_editor_commit_takes_the_view_off_the_generated_marker() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("marker_test");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(
            seeded_symbol_view(
                "symbol",
                &[PortSpec {
                    name: "IN".to_owned(),
                    direction: PortDirection::In,
                }],
            )
            .expect("the generated symbol seeds"),
        );
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state
            .open_workspace_view(CellViewRef::new("marker_test", "amp", "symbol"));
        let document = app
            .state
            .load_active_symbol_document()
            .expect("the generated symbol loads into the editor");

        app.state
            .store_active_symbol_document(&document)
            .expect("the editor commits the symbol");

        let view = app
            .state
            .model
            .library_manager
            .get_library("marker_test")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("the committed symbol view");
        assert!(!view.metadata.contains_key("generated"));
        assert!(!view.metadata.contains_key("ports"));
    }

    #[test]
    fn creation_catalog_contains_only_views_with_production_editors() {
        assert!(CREATABLE_VIEW_TYPES.into_iter().all(is_creatable_view_type));
        assert!(
            crate::state::ViewType::ALL
                .into_iter()
                .filter(|view_type| !is_creatable_view_type(*view_type))
                .all(|view_type| !CREATABLE_VIEW_TYPES.contains(&view_type))
        );
    }

    #[test]
    fn unsupported_view_type_is_rejected_before_project_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("view_policy_test");
        library.add_cell(Cell::new("filter"));
        app.state.model.library_manager.add_library(library);
        app.state.project.new_view_library = "view_policy_test".to_owned();
        app.state.project.new_view_cell = "filter".to_owned();
        app.state.project.new_view_name = "doc".to_owned();
        app.state.project.new_view_type = crate::state::ViewType::Document;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        assert!(
            app.state
                .model
                .library_manager
                .get_library("view_policy_test")
                .and_then(|library| library.get_cell("filter"))
                .is_some_and(|cell| cell.get_view("doc").is_none())
        );
        assert!(
            app.state
                .project
                .new_view_error
                .as_deref()
                .is_some_and(|error| error.contains("production editor"))
        );
    }

    #[test]
    fn creating_layout_view_atomically_retains_exact_signed_pdk_document_and_opens_it() {
        let mut app = RSpiceApp::test_instance();
        app.state.provision_test_project_technology_contract();
        let mut library = Library::new("layout_designs");
        library.add_cell(Cell::new("precision_amp"));
        app.state.model.library_manager.add_library(library);
        app.state.project.new_view_library = "layout_designs".to_owned();
        app.state.project.new_view_cell = "precision_amp".to_owned();
        app.state.project.new_view_name = "layout".to_owned();
        app.state.project.new_view_type = ViewType::Layout;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        let project_revision_before = app.state.model.workspace.content.project.revision().get();

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close, "{:?}", app.state.project.new_view_error);
        assert_eq!(
            app.state.model.workspace.content.project.revision().get(),
            project_revision_before + 1
        );
        let reference = CellViewRef::new("layout_designs", "precision_amp", "layout");
        let view = app
            .state
            .model
            .library_manager
            .get_library("layout_designs")
            .and_then(|library| library.get_cell("precision_amp"))
            .and_then(|cell| cell.get_view("layout"))
            .expect("Layout view commits");
        assert_eq!(view.view_type, ViewType::Layout);
        let document = app
            .state
            .model
            .workspace
            .content
            .physical_layout_document(&reference)
            .expect("authoritative physical-layout document commits atomically");
        assert_eq!(document.owner(), &reference);
        assert_eq!(document.technology().package_id(), "demo180");
        assert_eq!(document.technology().stack_id(), "1P2M");
        assert_eq!(app.state.model.workspace.content.active_view, reference);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design
        );
        crate::workbench::lifecycle::project_lifecycle::snapshot(&app.state)
            .expect("created Layout view and document validate as one project transaction");
    }

    #[test]
    fn creating_layout_view_without_exact_signed_project_pdk_is_transactional() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("layout_designs");
        library.add_cell(Cell::new("precision_amp"));
        app.state.model.library_manager.add_library(library);
        app.state.project.new_view_library = "layout_designs".to_owned();
        app.state.project.new_view_cell = "precision_amp".to_owned();
        app.state.project.new_view_name = "layout".to_owned();
        app.state.project.new_view_type = ViewType::Layout;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        let project_revision_before = app.state.model.workspace.content.project.revision();

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        assert_eq!(
            app.state.model.workspace.content.project.revision(),
            project_revision_before
        );
        let reference = CellViewRef::new("layout_designs", "precision_amp", "layout");
        assert!(
            app.state
                .model
                .library_manager
                .get_library("layout_designs")
                .and_then(|library| library.get_cell("precision_amp"))
                .is_some_and(|cell| cell.get_view("layout").is_none())
        );
        assert!(
            app.state
                .model
                .workspace
                .content
                .physical_layout_document(&reference)
                .is_none()
        );
        assert!(
            app.state
                .project
                .new_view_error
                .as_deref()
                .is_some_and(|error| error.contains("validated, signed PDK"))
        );
    }

    #[test]
    fn new_view_rejects_accented_canonical_collision_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("identity_test");
        let mut cell = Cell::new("filter");
        cell.add_view(View::new("Mod\u{e8}le", ViewType::Symbol));
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state.project.new_view_library = "identity_test".to_owned();
        app.state.project.new_view_cell = "filter".to_owned();
        app.state.project.new_view_name = "MOD\u{c8}LE".to_owned();
        app.state.project.new_view_type = ViewType::Schematic;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        let buffers_before = app.state.model.workspace.content.schematic_buffers.len();

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        let cell = app
            .state
            .model
            .library_manager
            .get_library("identity_test")
            .and_then(|library| library.get_cell("filter"))
            .expect("identity cell remains");
        assert_eq!(cell.view_count(), 1);
        assert!(cell.get_view("Mod\u{e8}le").is_some());
        assert_eq!(
            app.state.model.workspace.content.schematic_buffers.len(),
            buffers_before
        );
        assert!(
            app.state
                .project
                .new_view_error
                .as_deref()
                .is_some_and(|error| error.contains("clashes with an existing view"))
        );
    }

    #[test]
    fn creating_veriloga_view_atomically_retains_contract_and_opens_exact_editor() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("behavioral_models");
        let mut cell = Cell::new("precision_amp");
        let mut symbol = View::new("symbol", ViewType::Symbol);
        let symbol_document = SymbolDocument {
            pins: vec![
                SymbolPin::new("inp", PortDirection::In, None),
                SymbolPin::new("inn", PortDirection::In, None),
                SymbolPin::new("out", PortDirection::Out, None),
                SymbolPin::new("vss", PortDirection::Supply, None),
            ],
            ..SymbolDocument::default()
        };
        symbol_document.store_in_view(&mut symbol).unwrap();
        cell.add_view(symbol);
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state.project.new_view_library = "behavioral_models".to_owned();
        app.state.project.new_view_cell = "precision_amp".to_owned();
        app.state.project.new_view_name = "veriloga".to_owned();
        app.state.project.new_view_type = ViewType::VerilogA;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        let project_revision_before = app.state.model.workspace.content.project.revision().get();

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close, "{:?}", app.state.project.new_view_error);
        assert_eq!(
            app.state.model.workspace.content.project.revision().get(),
            project_revision_before + 1
        );
        let reference =
            crate::state::CellViewRef::new("behavioral_models", "precision_amp", "veriloga");
        let owner = ProjectSourceOwner::cell_view(reference.clone());
        let bundle = app
            .state
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&owner)
            .expect("created view owns a sealed source bundle");
        assert_eq!(bundle.language(), ProjectSourceLanguage::VerilogA);
        assert_eq!(bundle.root().logical_path(), "precision_amp.va");
        assert!(
            bundle
                .root()
                .content()
                .contains("module precision_amp(inp, inn, out, vss);")
        );
        assert!(bundle.root().content().contains("input inp, inn;"));
        assert!(bundle.root().content().contains("output out;"));
        assert!(bundle.root().content().contains("inout vss;"));
        let view = app
            .state
            .model
            .library_manager
            .get_library("behavioral_models")
            .and_then(|library| library.get_cell("precision_amp"))
            .and_then(|cell| cell.get_view("veriloga"))
            .unwrap();
        assert_eq!(
            view.metadata.get("veriloga.module").map(String::as_str),
            Some("precision_amp")
        );
        assert_eq!(
            view.metadata.get("veriloga.ports").map(String::as_str),
            Some(r#"["inp","inn","out","vss"]"#)
        );
        assert_eq!(app.state.model.workspace.content.active_view, reference);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design,
            "a new Verilog-A view opens as a Design document"
        );
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            crate::workbench::SurfaceId::Design,
            "reached through navigation, not by writing the workspace"
        );
        let selected =
            crate::workbench::documents::code_workspace::selected_veriloga_source(&app).unwrap();
        assert_eq!(selected.bundle().id(), bundle.id());
        assert_eq!(selected.selected_module(), Some("precision_amp"));
        assert!(selected.bundle().validation_is_current());
        assert!(app.state.code.code_workspace.veriloga.receipt.is_some());
    }

    /// A cell with no schematic ports and no symbol pins has no interface yet,
    /// and its module declares none. It used to be handed a `p`/`n` pair that
    /// nothing had asked for, which then became the cell's interface.
    #[test]
    fn a_veriloga_view_of_a_cell_with_no_interface_starts_without_ports() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("behavioral_models");
        library.add_cell(Cell::new("blank"));
        app.state.model.library_manager.add_library(library);
        app.state.project.new_view_library = "behavioral_models".to_owned();
        app.state.project.new_view_cell = "blank".to_owned();
        app.state.project.new_view_name = "veriloga".to_owned();
        app.state.project.new_view_type = ViewType::VerilogA;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close, "{:?}", app.state.project.new_view_error);
        let owner = ProjectSourceOwner::cell_view(CellViewRef::new(
            "behavioral_models",
            "blank",
            "veriloga",
        ));
        let source = app
            .state
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&owner)
            .expect("the view owns its source")
            .root()
            .content()
            .to_owned();
        assert!(source.contains("module blank;\nendmodule"), "{source}");
        assert!(!source.contains("electrical"), "{source}");
        let view = app
            .state
            .model
            .library_manager
            .get_library("behavioral_models")
            .and_then(|library| library.get_cell("blank"))
            .and_then(|cell| cell.get_view("veriloga"))
            .expect("the created view");
        assert_eq!(
            view.metadata.get("veriloga.ports").map(String::as_str),
            Some("[]")
        );
    }

    #[test]
    fn duplicate_cell_view_source_owner_rolls_back_view_creation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("behavioral_models");
        library.add_cell(Cell::new("gain_stage"));
        app.state.model.library_manager.add_library(library);
        let reference =
            crate::state::CellViewRef::new("behavioral_models", "gain_stage", "veriloga");
        app.state
            .model
            .workspace
            .content
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(reference.clone()),
                    ProjectSourceLanguage::VerilogA,
                    "existing.va",
                    "module existing; endmodule\n",
                    [],
                    [],
                )
                .unwrap(),
            )
            .unwrap();
        let sources_before = app
            .state
            .model
            .workspace
            .content
            .project_sources
            .iter_bundles()
            .count();
        app.state.project.new_view_library = "behavioral_models".to_owned();
        app.state.project.new_view_cell = "gain_stage".to_owned();
        app.state.project.new_view_name = "veriloga".to_owned();
        app.state.project.new_view_type = ViewType::VerilogA;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        assert_eq!(
            app.state
                .model
                .workspace
                .content
                .project_sources
                .iter_bundles()
                .count(),
            sources_before
        );
        assert!(
            app.state
                .model
                .library_manager
                .get_library("behavioral_models")
                .and_then(|library| library.get_cell("gain_stage"))
                .is_some_and(|cell| cell.get_view("veriloga").is_none())
        );
        assert!(app.state.project.new_view_error.is_some());
    }

    #[test]
    fn new_view_rejects_catalog_change_after_dialog_open_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("stale_test");
        library.add_cell(Cell::new("filter"));
        app.state.model.library_manager.add_library(library);
        app.state.project.new_view_library = "stale_test".to_owned();
        app.state.project.new_view_cell = "filter".to_owned();
        app.state.project.new_view_name = "schematic".to_owned();
        app.state.project.new_view_type = ViewType::Schematic;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        app.state
            .model
            .library_manager
            .add_library(Library::new("intervening_change"));
        let project_revision_before = app.state.model.workspace.content.project.revision();

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        assert!(
            app.state
                .model
                .library_manager
                .get_library("stale_test")
                .and_then(|library| library.get_cell("filter"))
                .is_some_and(|cell| cell.get_view("schematic").is_none())
        );
        assert_eq!(
            app.state.model.workspace.content.project.revision(),
            project_revision_before
        );
        assert_eq!(
            app.state.project.new_view_error.as_deref(),
            Some(LIBRARY_CATALOG_STALE_MESSAGE)
        );
    }
}

//! New View: a command form stating the target library and cell, asking for
//! a view name and the view type.

use egui::Context;

use crate::state::workspace::VERILOGA_LIBRARY_NAME;
use crate::state::workspace::{
    starter_veriloga_source, verilog_module_name, verilog_port_declaration,
};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};
use crate::workbench::slots::ProjectState;
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

impl ProjectState {
    /// Open New View on the exact cell `library`/`cell`, against the library
    /// revision `model` holds now, preset to `view_type`, or for `None` to
    /// the first type the cell does not have yet.
    ///
    /// The target is resolved before any dialog state changes. This keeps a
    /// stale row click from opening an apparently valid transaction against a
    /// different or read-only object.
    pub(crate) fn open_new_view_dialog(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
        view_type: Option<crate::state::ViewType>,
    ) -> Result<(), String> {
        let view_type = new_view_opening(model, library, cell, view_type)?;
        self.new_view_library = library.to_owned();
        self.new_view_cell = cell.to_owned();
        self.new_view_name = view_type.display_name().to_owned();
        self.new_view_type = view_type;
        self.new_view_error = None;
        self.new_view_library_revision = model.library_manager.revision();
        self.new_view_dialog = true;
        Ok(())
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

impl ProjectFeature {
    /// Draw New View. What a created view leaves outside its catalog change
    /// goes to the shell, which carries it out as the dialog returns.
    pub(in crate::workbench) fn process_new_view_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.new_view_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;

        let catalog_current =
            cx.model.library_manager.revision() == cx.state.new_view_library_revision;
        if !catalog_current {
            cx.state.new_view_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut *cx.state;
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
            should_close |= create_new_view(cx);
        }

        if should_close {
            cx.state.new_view_dialog = false;
            cx.state.new_view_name.clear();
            cx.state.new_view_error = None;
        }
    }
}

/// Create the view the dialog names, and whether it was created. A refusal
/// is stated in the dialog and changes nothing.
fn create_new_view(cx: &mut Cx<'_, ProjectFeature>) -> bool {
    let view_name = cx.state.new_view_name.trim().to_string();
    let library = cx.state.new_view_library.clone();
    let cell = cx.state.new_view_cell.clone();

    if cx.model.library_manager.revision() != cx.state.new_view_library_revision {
        cx.state.new_view_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        return false;
    }
    if let Some(error) = validate_lcv_name(&view_name, "View name") {
        cx.state.new_view_error = Some(error);
        return false;
    }
    if library.is_empty() {
        cx.state.new_view_error = Some("Library cannot be empty".to_string());
        return false;
    }
    if cell.is_empty() {
        cx.state.new_view_error = Some("Cell cannot be empty".to_string());
        return false;
    }
    let view_type = cx.state.new_view_type;
    if !is_creatable_view_type(view_type) {
        cx.state.new_view_error = Some(format!(
            "{} views cannot be created until their production editor and storage workflow are available",
            view_type.display_name()
        ));
        return false;
    }

    let Some(lib_ro) = cx.model.library_manager.get_library(&library) else {
        cx.state.new_view_error = Some(format!("Library '{}' not found", library));
        return false;
    };
    if lib_ro.read_only {
        cx.state.new_view_error = Some(format!(
            "Library '{library}' is read only; create the view in an editable library"
        ));
        return false;
    }
    let Some(cell_ro) = lib_ro.get_cell(&cell) else {
        cx.state.new_view_error = Some(format!(
            "Cell '{}' not found in library '{}'",
            cell, library
        ));
        return false;
    };
    let requested_identity =
        crate::state::canonical_cell_view_owner_key(&library, &cell, &view_name);
    if cell_ro.views.values().any(|view| {
        crate::state::canonical_cell_view_owner_key(&library, &cell, &view.name)
            == requested_identity
    }) {
        cx.state.new_view_error = Some(format!(
            "View '{}' clashes with an existing view in cell '{}' (view names ignore letter case)",
            view_name, cell
        ));
        return false;
    }

    if view_type == crate::state::ViewType::Layout {
        return create_layout_cell_view(cx, &library, &cell, &view_name);
    }
    if view_type == crate::state::ViewType::VerilogA {
        return create_veriloga_cell_view(cx, &library, &cell, &view_name);
    }

    use crate::state::View;

    let seeded_view = if view_type == crate::state::ViewType::Symbol {
        let ports = cell_schematic_ports(cx.model, &library, &cell);
        match seeded_symbol_view(&view_name, &ports) {
            Ok(view) => view,
            Err(error) => {
                cx.state.new_view_error = Some(error);
                return false;
            }
        }
    } else {
        View::new(&view_name, view_type)
    };

    let project_mutation = match cx.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::CreateView {
            library: library.clone(),
            cell: cell.clone(),
            view: view_name.clone(),
        },
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            cx.state.new_view_error = Some(error);
            return false;
        }
    };
    let Some(mut lib) = cx.model.library_manager.edit_library(&library) else {
        cx.state.new_view_error = Some(format!("Library '{}' not found", library));
        return false;
    };
    if lib.add_view(&cell, seeded_view).is_none() {
        cx.state.new_view_error = Some(format!(
            "Cell '{}' not found in library '{}'",
            cell, library
        ));
        return false;
    }
    let reference = crate::state::CellViewRef::new(&library, &cell, &view_name);
    let draws_schematic = matches!(
        view_type,
        crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
    );
    let opens = draws_schematic || view_type == crate::state::ViewType::Symbol;
    cx.wb.outbox.push(ProjectRequest::CellViewsCreated {
        schematics: draws_schematic
            .then(|| reference.clone())
            .into_iter()
            .collect(),
        mutation: ReservedLibraryRevision::new(project_mutation),
        notice: format!("Created view '{view_name}' in cell '{cell}'."),
        open: opens.then_some(reference),
        veriloga: None,
        persist_global_veriloga: library == VERILOGA_LIBRARY_NAME,
    });
    cx.state.new_view_error = None;
    true
}

fn create_layout_cell_view(
    cx: &mut Cx<'_, ProjectFeature>,
    library: &str,
    cell: &str,
    view_name: &str,
) -> bool {
    let reference = crate::state::CellViewRef::new(library, cell, view_name);
    let technology = match cx.model.exact_project_layout_technology_binding() {
        Ok(technology) => technology,
        Err(error) => {
            cx.state.new_view_error = Some(format!(
                "A Layout view requires the project's validated, signed PDK: {error}"
            ));
            return false;
        }
    };
    let document =
        match crate::state::PhysicalLayoutDocument::try_new(reference.clone(), technology) {
            Ok(document) => document,
            Err(error) => {
                cx.state.new_view_error =
                    Some(format!("The physical-layout document is invalid: {error}"));
                return false;
            }
        };
    let candidate_layouts = match cx
        .model
        .workspace
        .content
        .prepare_insert_physical_layout_document(document)
    {
        Ok(candidate) => candidate,
        Err(error) => {
            cx.state.new_view_error = Some(format!(
                "The physical-layout document could not be retained: {error}"
            ));
            return false;
        }
    };
    let mut candidate_libraries = cx.model.library_manager.clone();
    let Some(mut candidate_library) = candidate_libraries.edit_library(library) else {
        cx.state.new_view_error = Some(format!("Library '{library}' not found"));
        return false;
    };
    if candidate_library.library().read_only {
        cx.state.new_view_error = Some(format!(
            "Library '{library}' became read only before the Layout view could be created"
        ));
        return false;
    }
    let Some(candidate_cell) = candidate_library.library().get_cell(cell) else {
        cx.state.new_view_error = Some(format!("Cell '{cell}' not found in library '{library}'"));
        return false;
    };
    let requested_identity = crate::state::canonical_cell_view_owner_key(library, cell, view_name);
    if candidate_cell.views.values().any(|view| {
        crate::state::canonical_cell_view_owner_key(library, cell, &view.name) == requested_identity
    }) {
        cx.state.new_view_error = Some(format!(
            "View '{view_name}' clashes with an existing view in cell '{cell}' (view names ignore letter case)"
        ));
        return false;
    }
    candidate_library.add_view(
        cell,
        crate::state::View::new(view_name, crate::state::ViewType::Layout),
    );
    let project_mutation = match cx.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::CreateView {
            library: library.to_owned(),
            cell: cell.to_owned(),
            view: view_name.to_owned(),
        },
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            cx.state.new_view_error = Some(error);
            return false;
        }
    };

    cx.model.library_manager = candidate_libraries;
    cx.model
        .workspace
        .content
        .commit_prepared_physical_layout_catalog(candidate_layouts);
    cx.wb.outbox.push(ProjectRequest::ViewOpened {
        mutation: ReservedLibraryRevision::new(project_mutation),
        view: reference,
        veriloga: None,
        notice: format!(
            "Created Layout view '{view_name}' for cell '{cell}' under the project PDK."
        ),
        persist_global_veriloga: false,
    });
    cx.state.new_view_error = None;
    true
}

fn create_veriloga_cell_view(
    cx: &mut Cx<'_, ProjectFeature>,
    library: &str,
    cell: &str,
    view_name: &str,
) -> bool {
    let reference = crate::state::CellViewRef::new(library, cell, view_name);
    let ports = match derive_cell_interface(cx.model, library, cell) {
        Ok(ports) => ports,
        Err(error) => {
            cx.state.new_view_error = Some(error);
            return false;
        }
    };
    let module_name = verilog_module_name(cell);
    if let Some(invalid) = ports
        .iter()
        .find(|port| verilog_port_declaration(&port.name).is_none())
    {
        cx.state.new_view_error = Some(format!(
            "Interface port '{}' is not a valid Verilog-A identifier; rename the cell port before creating this view",
            invalid.name
        ));
        return false;
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
            cx.state.new_view_error = Some(format!(
                "The Verilog-A source bundle could not be created: {error}"
            ));
            return false;
        }
    };
    let receipt = match crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
        cx.model.workspace.content.project.id(),
        &bundle,
        Some(&module_name),
    ) {
        Ok(receipt) => receipt,
        Err(diagnostics) => {
            let detail = diagnostics
                .first()
                .map(|diagnostic| format!("{}: {}", diagnostic.message, diagnostic.detail))
                .unwrap_or_else(|| "the compiler returned no diagnostic".to_owned());
            cx.state.new_view_error = Some(format!(
                "The generated Verilog-A starter did not pass semantic compilation: {detail}"
            ));
            return false;
        }
    };
    if let Err(error) = bundle.mark_validated() {
        cx.state.new_view_error = Some(format!(
            "The compiled Verilog-A source could not retain its validation identity: {error}"
        ));
        return false;
    }

    // Prepare both durable stores on clones. No live project mutation is
    // visible until the complete view/source transaction has succeeded.
    let mut candidate_sources = cx.model.workspace.content.project_sources.clone();
    if let Err(error) = candidate_sources.insert_bundle(bundle) {
        cx.state.new_view_error = Some(format!(
            "The Verilog-A source bundle could not be retained: {error}"
        ));
        return false;
    }
    let port_names = ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    let encoded_ports = match serde_json::to_string(&port_names) {
        Ok(encoded) => encoded,
        Err(error) => {
            cx.state.new_view_error = Some(format!(
                "The Verilog-A interface contract could not be retained: {error}"
            ));
            return false;
        }
    };
    let mut view = crate::state::View::new(view_name, crate::state::ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), module_name.clone());
    view.metadata
        .insert("veriloga.ports".to_owned(), encoded_ports);
    let mut candidate_libraries = cx.model.library_manager.clone();
    let Some(mut candidate_library) = candidate_libraries.edit_library(library) else {
        cx.state.new_view_error = Some(format!("Library '{library}' not found"));
        return false;
    };
    if candidate_library.library().read_only {
        cx.state.new_view_error = Some(format!(
            "Library '{library}' became read only before the view could be created"
        ));
        return false;
    }
    let Some(candidate_cell) = candidate_library.library().get_cell(cell) else {
        cx.state.new_view_error = Some(format!("Cell '{cell}' not found in library '{library}'"));
        return false;
    };
    let requested_identity = crate::state::canonical_cell_view_owner_key(library, cell, view_name);
    if candidate_cell.views.values().any(|view| {
        crate::state::canonical_cell_view_owner_key(library, cell, &view.name) == requested_identity
    }) {
        cx.state.new_view_error = Some(format!(
            "View '{view_name}' clashes with an existing view in cell '{cell}' (view names ignore letter case)"
        ));
        return false;
    }
    candidate_library.add_view(cell, view);

    let project_mutation = match cx.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::CreateView {
            library: library.to_owned(),
            cell: cell.to_owned(),
            view: view_name.to_owned(),
        },
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            cx.state.new_view_error = Some(error);
            return false;
        }
    };
    cx.model.workspace.content.project_sources = candidate_sources;
    cx.model.workspace.content.project_sources_dirty = true;
    cx.model.library_manager = candidate_libraries;
    // The new view is a cell view, so it opens as a Design document.
    cx.wb.outbox.push(ProjectRequest::ViewOpened {
        mutation: ReservedLibraryRevision::new(project_mutation),
        view: reference,
        veriloga: Some(Box::new(receipt)),
        notice: format!(
            "Created Verilog-A view '{view_name}' for cell '{cell}' and opened its source."
        ),
        persist_global_veriloga: library == VERILOGA_LIBRARY_NAME,
    });
    cx.state.new_view_error = None;
    true
}

/// A symbol view seeded from a cell's interface `ports`.
///
/// [`GENERATED_SYMBOL_VIEW`] additionally carries the `generated` marker, so
/// the schematic keeps the symbol in step as ports come and go until the
/// first hand edit takes ownership by clearing the marker.
pub(in crate::workbench) fn seeded_symbol_view(
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
fn cell_schematic_ports(model: &Model, library: &str, cell: &str) -> Vec<crate::state::PortSpec> {
    let reference = crate::state::CellViewRef::new(library, cell, "schematic");
    if model.workspace.content.active_view == reference {
        return model.schematic.document().interface_ports();
    }
    model
        .workspace
        .content
        .schematic_buffers
        .get(&reference.key())
        .map(|schematic| schematic.document().interface_ports())
        .unwrap_or_default()
}

fn derive_cell_interface(
    model: &Model,
    library_name: &str,
    cell_name: &str,
) -> Result<Vec<crate::state::PortSpec>, String> {
    let cell = model
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
        let schematic = if model.workspace.content.active_view == reference {
            Some(model.schematic.editor_ref().design)
        } else {
            model
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
    use crate::workbench::project::library_dialogs::{in_front, published, shown};
    use crate::workbench::project::tests::with_cx;
    use crate::workbench::{AppState, ChoicePreference, RSpiceApp};

    /// Create the view the dialog names in the feature's context, with
    /// what it raised answered as the dialog step answers it, and whether
    /// it was created.
    fn create(app: &mut RSpiceApp) -> bool {
        let created = with_cx(app, create_new_view);
        app.after_contribution();
        created
    }

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
            .project
            .open_new_view_dialog(&state.model, "entry_point_test", "filter", None)
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
            .project
            .open_new_view_dialog(&state.model, "preset_test", "amp", None)
            .expect("writable target");
        assert_eq!(state.project.new_view_type, ViewType::Testbench);

        state
            .project
            .open_new_view_dialog(&state.model, "preset_test", "amp", Some(ViewType::Symbol))
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
            .project
            .open_new_view_dialog(&app.state.model, "form_test", "amp", None)
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
                |ctx| {
                    with_cx(&mut app, |cx| {
                        ProjectFeature::process_new_view_dialog(cx, ctx)
                    })
                },
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
            .project
            .open_new_view_dialog(&state.model, "entry_point_read_only", "filter", None)
            .expect_err("read-only target must fail closed");
        let stale_error = state
            .project
            .open_new_view_dialog(&state.model, "missing", "filter", None)
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

        let created = create(&mut app);

        assert!(created);
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

        let created = create(&mut app);

        assert!(created, "{:?}", app.state.project.new_view_error);
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

        let created = create(&mut app);

        assert!(!created);
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

        let created = create(&mut app);

        assert!(created, "{:?}", app.state.project.new_view_error);
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

        let created = create(&mut app);

        assert!(!created);
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

        let created = create(&mut app);

        assert!(!created);
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

        let created = create(&mut app);

        assert!(created, "{:?}", app.state.project.new_view_error);
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

        let created = create(&mut app);

        assert!(created, "{:?}", app.state.project.new_view_error);
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

        let created = create(&mut app);

        assert!(!created);
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

        let created = create(&mut app);

        assert!(!created);
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

    /// A cell of its own in an editable library, with New View open on it
    /// to create a `view_type` view named `name`, under the project's signed
    /// PDK when that is a Layout view.
    fn creating(view_type: ViewType, name: &str) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        if view_type == ViewType::Layout {
            app.state.provision_test_project_technology_contract();
        }
        let mut library = Library::new("designs");
        library.add_cell(Cell::new("amp"));
        app.state.model.library_manager.add_library(library);
        let project = &mut app.state.project;
        project.new_view_library = "designs".to_owned();
        project.new_view_cell = "amp".to_owned();
        project.new_view_name = name.to_owned();
        project.new_view_type = view_type;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        app
    }

    /// Carry out what the creation handed over in the order the creation
    /// once did it in place, with the new document it read before it
    /// changed the catalog.
    fn carry_out_in_place(app: &mut RSpiceApp, seeded: SchematicState) {
        let request = app.state.workbench.outbox.take::<ProjectRequest>();
        let state = &mut app.state;
        match request {
            Some(ProjectRequest::CellViewsCreated {
                schematics,
                mutation,
                notice,
                open,
                veriloga,
                persist_global_veriloga,
            }) => {
                assert!(veriloga.is_none(), "New View seeds no Verilog-A view here");
                assert!(
                    !persist_global_veriloga,
                    "the view is not the Verilog-A library's"
                );
                for reference in schematics {
                    state
                        .model
                        .workspace
                        .insert_schematic_editor(reference.key(), seeded.clone());
                }
                state.model.publish_project_library_mutation(
                    mutation
                        .take()
                        .expect("the reserved revision is published once"),
                );
                state.record(
                    crate::diagnostics::LogSeverity::Info,
                    crate::diagnostics::LogSource::Studio,
                    notice,
                );
                if let Some(view) = open {
                    state.open_workspace_view(view);
                }
            }
            Some(ProjectRequest::ViewOpened {
                mutation,
                view,
                veriloga,
                notice,
                persist_global_veriloga,
            }) => {
                assert!(
                    !persist_global_veriloga,
                    "the view is not the Verilog-A library's"
                );
                state.model.publish_project_library_mutation(
                    mutation
                        .take()
                        .expect("the reserved revision is published once"),
                );
                state.open_workspace_view(view);
                if let Some(receipt) = veriloga {
                    state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Design);
                    state.code.code_workspace.veriloga.receipt = Some(*receipt);
                }
                state.record(
                    crate::diagnostics::LogSeverity::Info,
                    crate::diagnostics::LogSource::Studio,
                    notice,
                );
            }
            _ => panic!("the creation hands the shell what it left"),
        }
    }

    /// A view created in the feature's context, with what it left handed to
    /// the shell, ends where the creation carried out in place did. A view
    /// a schematic or symbol editor edits: the new document, the revision,
    /// the creation's line in the log, then the view opened. A Layout or a
    /// Verilog-A view: the revision, the view opened, the Verilog-A receipt
    /// with Design in front, then the creation's line in the log.
    #[test]
    fn a_created_view_ends_where_the_creation_carried_out_in_place_did() {
        for (view_type, name) in [
            (ViewType::Schematic, "schematic"),
            (ViewType::Symbol, "symbol"),
            (ViewType::Layout, "layout"),
            (ViewType::VerilogA, "veriloga"),
        ] {
            let mut in_place = creating(view_type, name);
            let seeded = in_place.state.new_schematic_document();
            assert!(
                with_cx(&mut in_place, create_new_view),
                "{view_type:?}: {:?}",
                in_place.state.project.new_view_error
            );
            carry_out_in_place(&mut in_place, seeded);

            let mut handed = creating(view_type, name);
            assert!(create(&mut handed), "{view_type:?}");

            assert_eq!(shown(&handed), shown(&in_place), "{view_type:?}");
            assert_eq!(published(&handed), published(&in_place), "{view_type:?}");
            assert_eq!(in_front(&handed), in_front(&in_place), "{view_type:?}");
            assert_eq!(in_front(&handed).1, view_type == ViewType::VerilogA);
        }
    }
}

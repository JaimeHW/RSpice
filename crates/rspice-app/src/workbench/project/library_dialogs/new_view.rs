//! New View: a command form stating the target library and cell, asking for
//! a view name and the view type.

use egui::Context;

use crate::state::workspace::VERILOGA_LIBRARY_NAME;
use crate::state::workspace::{
    starter_veriloga_source, verilog_module_name, verilog_port_declaration,
};
use crate::ui::widgets::{CommandForm, DialogChoice};
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
pub(crate) const CREATABLE_VIEW_TYPES: [crate::state::ViewType; 5] = [
    crate::state::ViewType::Schematic,
    crate::state::ViewType::Testbench,
    crate::state::ViewType::Symbol,
    crate::state::ViewType::Layout,
    crate::state::ViewType::VerilogA,
];

/// What a Type segment says: the view type in sentence case. The view keeps
/// its lowercase name in the library and the project; the segments read
/// "schematic testbench symbol layout veriloga" beside capitalised labels.
pub(crate) const fn view_type_label(view_type: crate::state::ViewType) -> &'static str {
    match view_type {
        crate::state::ViewType::Schematic => "Schematic",
        crate::state::ViewType::Testbench => "Testbench",
        crate::state::ViewType::Symbol => "Symbol",
        crate::state::ViewType::Layout => "Layout",
        crate::state::ViewType::VerilogA => "Verilog-A",
        _ => "Other",
    }
}

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
pub(crate) const NEW_VIEW_NAME_MISSING: &str = "Enter a name for the view.";

/// What the footer says about the view name when Create cannot run on it.
pub(crate) enum NameLine {
    /// Nothing typed yet: what Create waits for, not a refusal.
    Missing,
    /// A name the library would refuse, and why.
    Refused(String),
}

pub(crate) fn new_view_name_line(name: &str) -> Option<NameLine> {
    let name = name.trim();
    if name.is_empty() {
        return Some(NameLine::Missing);
    }
    validate_lcv_name(name, "View name").map(NameLine::Refused)
}

impl ProjectFeature {
    /// Draw New View. What a created view leaves outside its catalog change
    /// goes to the shell, which carries it out as the dialog returns.
    pub(crate) fn process_new_view_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
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
        let type_names = CREATABLE_VIEW_TYPES.map(view_type_label);
        let mut type_index = CREATABLE_VIEW_TYPES
            .iter()
            .position(|view_type| *view_type == dialogs.new_view_type)
            .unwrap_or_default();

        // The five types are one row of equal segments, which the 420 pt
        // card would run past its edge.
        let choice = CommandForm::new("New view", "Create")
            .option_columns()
            .describe("Create a named view of the selected type in this library cell.")
            .primary_enabled(can_create)
            // What Create waits for, or why it refuses the name, stands in
            // the footer beside it, as New cell's does, so the rows keep one
            // pitch and the body ends at its last row.
            .status(match (&error, &name_refusal) {
                (Some(error), _) => Err(error.as_str()),
                (None, Some(NameLine::Missing)) => Ok(Some(NEW_VIEW_NAME_MISSING)),
                (None, Some(NameLine::Refused(refusal))) => Err(refusal.as_str()),
                (None, None) => Ok(None),
            })
            .show(ctx, |rows| {
                rows.value_mono("Library", &dialogs.new_view_library);
                rows.value_mono("Cell", &dialogs.new_view_cell);
                rows.text("View name", name_id, &mut dialogs.new_view_name, "");
                if rows.segmented_columns(
                    "Type",
                    "library.new-view.type",
                    &type_names,
                    &mut type_index,
                ) {
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
pub(crate) fn create_new_view(cx: &mut Cx<'_, ProjectFeature>) -> bool {
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
                "A Layout view requires the project's validated PDK: {error}"
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
    let receipt = match crate::workbench::app_state::model::compile_project_bundle_receipt(
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
                "The Verilog-A starter module did not compile: {detail}"
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
pub(crate) fn seeded_symbol_view(
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
    use crate::state::{Cell, Library, View, ViewType};
    use crate::workbench::project::tests::harness;

    #[test]
    fn new_view_entry_point_resolves_exact_writable_cell_before_opening() {
        let mut harness = harness();
        let mut library = Library::new("entry_point_test");
        let mut cell = Cell::new("filter");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        harness.model.library_manager.add_library(library);

        harness
            .state
            .open_new_view_dialog(&harness.model, "entry_point_test", "filter", None)
            .expect("writable exact target should open");

        assert!(harness.state.new_view_dialog);
        assert_eq!(harness.state.new_view_library, "entry_point_test");
        assert_eq!(harness.state.new_view_cell, "filter");
        assert_eq!(harness.state.new_view_type, ViewType::Schematic);
        assert_eq!(harness.state.new_view_name, "schematic");
        assert!(harness.state.new_view_error.is_none());
        assert_eq!(
            harness.state.new_view_library_revision,
            harness.model.library_manager.revision()
        );
    }

    /// "Create symbol from schematic" opens New view on the symbol of the
    /// active cell, not on the first type the cell lacks (a testbench).
    #[test]
    fn new_view_opens_preset_to_a_requested_view_type() {
        let mut harness = harness();
        let mut library = Library::new("preset_test");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
        harness.model.library_manager.add_library(library);

        harness
            .state
            .open_new_view_dialog(&harness.model, "preset_test", "amp", None)
            .expect("writable target");
        assert_eq!(harness.state.new_view_type, ViewType::Testbench);

        harness
            .state
            .open_new_view_dialog(&harness.model, "preset_test", "amp", Some(ViewType::Symbol))
            .expect("writable target");
        assert!(harness.state.new_view_dialog);
        assert_eq!(harness.state.new_view_type, ViewType::Symbol);
        assert_eq!(harness.state.new_view_name, "symbol");
    }

    #[test]
    fn new_view_entry_point_rejects_read_only_or_stale_target_without_mutation() {
        let mut harness = harness();
        let mut library = Library::new("entry_point_read_only");
        library.read_only = true;
        library.add_cell(Cell::new("filter"));
        harness.model.library_manager.add_library(library);
        harness.state.new_view_library = "sentinel".to_owned();

        let read_only_error = harness
            .state
            .open_new_view_dialog(&harness.model, "entry_point_read_only", "filter", None)
            .expect_err("read-only target must fail closed");
        let stale_error = harness
            .state
            .open_new_view_dialog(&harness.model, "missing", "filter", None)
            .expect_err("stale target must fail closed");

        assert!(read_only_error.contains("read-only"));
        assert!(stale_error.contains("no longer exists"));
        assert!(!harness.state.new_view_dialog);
        assert_eq!(harness.state.new_view_library, "sentinel");
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
}

//! New Verilog-A device…: a cell whose module the reader writes next.
//!
//! The form asks three questions — which library, what the cell is called,
//! and which ports the module starts with — and states what the answers make:
//! the module's name, which is the cell's own wherever a module can carry
//! it, and how many ports its generated symbol starts with. The ports are a
//! starting point only. Once the module compiles, the module defines them, so
//! the field is optional, and a module typed with none declares none: nothing
//! is invented in its place.
//!
//! Each field states what it declares, or why it is refused, in the one line
//! under it, which is always there; so nothing moves while the reader types,
//! and the form reserves no status band. The status line is left for a
//! failure that belongs to no field: a library that cannot be written, or a
//! create that the project refused.
//!
//! Nothing is created unless all of it is. The cell, its `veriloga` view, the
//! view's source bundle and the generated `symbol` view are made on copies of
//! the catalog and the source registry and published as one library
//! transaction. The draft is measured against the live catalog every frame,
//! so a catalog that moves while the form is open is judged as it is now
//! rather than refused for having moved.
//!
//! The schematic the form was opened from is remembered. Once the reader's
//! own compile gives the module a port, the compile result offers to place
//! the device there.

use std::collections::HashSet;

use egui::Context;
use rspice_design::symbol::generated::{PortEntryError, parse_port_entry};

use super::new_view_dialog::{
    starter_veriloga_source, verilog_module_name, verilog_port_declaration,
};
use super::{RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::diagnostics::ConsoleMessage;
use crate::state::workspace::VERILOGA_VIEW;
use crate::state::{
    Cell, CellViewRef, LibraryManager, PortDirection, PortSpec, ProjectSourceBundle,
    ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry, View, ViewType,
};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::app::dialogs::state::NewVerilogADeviceFormState;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::{
    VerilogACompileReceipt, VerilogAPlacementHandoff, apply_receipt_interface,
    compile_project_bundle_receipt, writable_libraries,
};
use crate::workbench::{MessageCatalog, MessageId};

/// Open New Verilog-A device… on the library holding the active design,
/// remembering the schematic it was opened from.
pub(crate) fn open_new_veriloga_device_form(state: &mut AppState) {
    let libraries = writable_libraries(&state.library_manager);
    let active = &state.workspace.content.active_view;
    let library = libraries
        .iter()
        .find(|name| **name == active.library)
        .or_else(|| libraries.first())
        .cloned()
        .unwrap_or_default();
    let origin = matches!(
        state.workspace.content.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    )
    .then(|| active.clone());
    state.dialogs.new_veriloga_device = NewVerilogADeviceFormState {
        open: true,
        library,
        origin,
        ..NewVerilogADeviceFormState::default()
    };
}

/// Whether New Verilog-A device… can make anything at all: it needs a
/// library it can write.
pub(crate) fn new_veriloga_device_available(state: &AppState) -> bool {
    !writable_libraries(&state.library_manager).is_empty()
}

/// What the typed draft is, measured against the live catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Draft {
    /// No cell name yet. The primary is off and the form says nothing.
    Empty,
    /// A refusal the reader has to fix.
    Refused(String),
    /// A cell ready to create, with the ports its starter module declares.
    Ready { cell: String, ports: Vec<PortSpec> },
}

fn draft(state: &AppState) -> Draft {
    let form = &state.dialogs.new_veriloga_device;
    let messages = state.ui.messages();
    if !writable_libraries(&state.library_manager).contains(&form.library) {
        return Draft::Refused(messages.text(MessageId::VerilogADeviceNoLibrary));
    }
    match typed_cell(state, &messages) {
        None => Draft::Empty,
        Some(Err(refusal)) => Draft::Refused(refusal),
        Some(Ok(cell)) => match typed_ports(&form.ports, &messages) {
            Ok(ports) => Draft::Ready { cell, ports },
            Err(refusal) => Draft::Refused(refusal),
        },
    }
}

/// The cell the Cell name field names: none yet, one the library can take,
/// or why it cannot. A cell name is letters, digits and underscores, as New
/// Cell's is; the module it is named after is made from it by
/// [`verilog_module_name`].
fn typed_cell(state: &AppState, messages: &MessageCatalog) -> Option<Result<String, String>> {
    let form = &state.dialogs.new_veriloga_device;
    let cell = form.name.trim();
    if cell.is_empty() {
        return None;
    }
    if !cell
        .chars()
        .all(|character| character.is_alphanumeric() || character == '_')
    {
        return Some(Err(messages.text(MessageId::VerilogADeviceCellInvalid)));
    }
    let requested = crate::state::canonical_cell_view_owner_key(&form.library, cell, "");
    let taken = state
        .library_manager
        .get_library(&form.library)
        .is_some_and(|library| {
            library.cells.values().any(|existing| {
                crate::state::canonical_cell_view_owner_key(&form.library, &existing.name, "")
                    == requested
            })
        });
    if taken {
        return Some(Err(messages.format(
            MessageId::VerilogADeviceCellExists,
            &[("library", &form.library), ("cell", cell)],
        )));
    }
    Some(Ok(cell.to_owned()))
}

/// The ports the field declares, in the order typed, read by the one ports
/// codec: `name`, `name:direction`, or a bus `d[7:0]:direction`, with
/// `inout` when no direction is given. Each must be one a Verilog-A module
/// can declare, and none may be listed twice.
fn typed_ports(typed: &str, messages: &MessageCatalog) -> Result<Vec<PortSpec>, String> {
    let mut ports = Vec::new();
    let mut seen = HashSet::new();
    for entry in typed.split_whitespace() {
        let port = parse_port_entry(entry).map_err(|error| match error {
            PortEntryError::UnknownDirection(direction) if direction.is_empty() => messages.format(
                MessageId::VerilogADevicePortNoDirection,
                &[("entry", entry)],
            ),
            PortEntryError::UnknownDirection(direction) => messages.format(
                MessageId::VerilogADevicePortDirection,
                &[("entry", entry), ("direction", &direction)],
            ),
            PortEntryError::MissingName => {
                messages.format(MessageId::VerilogADevicePortName, &[("entry", entry)])
            }
        })?;
        if port.direction == PortDirection::Supply {
            return Err(messages.format(MessageId::VerilogADevicePortSupply, &[("entry", entry)]));
        }
        let Some((base, _)) = verilog_port_declaration(&port.name) else {
            return Err(messages.format(MessageId::VerilogADevicePortName, &[("entry", entry)]));
        };
        if !seen.insert(base.to_ascii_lowercase()) {
            return Err(messages.format(MessageId::VerilogADevicePortTwice, &[("port", base)]));
        }
        ports.push(port);
    }
    Ok(ports)
}

/// What one field's line says, owned for the frame it is drawn in.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
    Blank,
    Derived(String),
    Error(String),
}

impl Line {
    fn field_line(&self) -> FieldLine<'_> {
        match self {
            Self::Blank => FieldLine::Blank,
            Self::Derived(text) => FieldLine::Derived(text),
            Self::Error(text) => FieldLine::Error(text),
        }
    }
}

/// The line under Cell name: the module the cell's view will declare, or why
/// the name is refused.
fn cell_line(state: &AppState, messages: &MessageCatalog) -> Line {
    match typed_cell(state, messages) {
        None => Line::Blank,
        Some(Err(refusal)) => Line::Error(refusal),
        Some(Ok(cell)) => Line::Derived(messages.format(
            MessageId::VerilogADeviceModule,
            &[("module", &verilog_module_name(&cell))],
        )),
    }
}

/// The line under Ports: how many the generated symbol starts with, or the
/// entry that is refused.
fn ports_line(typed: &str, messages: &MessageCatalog) -> Line {
    match typed_ports(typed, messages) {
        Err(refusal) => Line::Error(refusal),
        Ok(ports) => Line::Derived(match ports.len() {
            0 => messages.text(MessageId::VerilogADeviceNoPorts),
            1 => messages.text(MessageId::VerilogADeviceOnePort),
            count => messages.format(
                MessageId::VerilogADevicePortCount,
                &[("count", &count.to_string())],
            ),
        }),
    }
}

/// Give the cell at `reference` a `veriloga` view holding a starter module
/// for `ports`, with the interface and the generated symbol that module's
/// compile defines, in `libraries` and `sources`. Both are copies: the caller
/// publishes them only once everything here has been made.
///
/// New Verilog-A device… and New Cell's Verilog-A view both take this one
/// path, so a cell made either way is the cell its first compile would make.
pub(super) fn seed_veriloga_view(
    state: &AppState,
    libraries: &mut LibraryManager,
    sources: &mut ProjectSourceRegistry,
    reference: &CellViewRef,
    ports: &[PortSpec],
) -> Result<VerilogACompileReceipt, String> {
    let module_name = verilog_module_name(&reference.cell);
    let mut bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        format!("{module_name}.va"),
        starter_veriloga_source(&module_name, ports),
        [],
        [],
    )
    .map_err(|error| format!("The Verilog-A source could not be created: {error}"))?;
    let receipt = compile_project_bundle_receipt(
        state.workspace.content.project.id(),
        &bundle,
        Some(&module_name),
    )
    .map_err(|diagnostics| {
        let detail = diagnostics
            .first()
            .map(|diagnostic| format!("{}: {}", diagnostic.message, diagnostic.detail))
            .unwrap_or_else(|| "the compiler returned no diagnostic".to_owned());
        format!("The starter module did not compile: {detail}")
    })?;
    bundle.mark_validated().map_err(|error| {
        format!("The compiled starter module could not keep its validation identity: {error}")
    })?;

    // The view, then the source it owns, then the interface its compile
    // defines, which writes `veriloga.ports` and the generated symbol.
    let mut view = View::new(&reference.view, ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), module_name);
    libraries
        .edit_library(&reference.library)
        .ok_or_else(|| format!("Library '{}' not found", reference.library))?
        .add_view(&reference.cell, view)
        .ok_or_else(|| {
            format!(
                "Cell '{}' not found in library '{}'",
                reference.cell, reference.library
            )
        })?;
    sources
        .insert_bundle(bundle)
        .map_err(|error| format!("The Verilog-A source could not be retained: {error}"))?;
    apply_receipt_interface(
        libraries,
        &state.workspace.content.schematic_buffers,
        reference,
        receipt,
    )
    .map_err(|reason| format!("The module's ports could not become the cell's: {reason}"))
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_new_veriloga_device_form(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_veriloga_device.open {
            return;
        }
        let messages = self.state.ui.messages();
        let libraries = writable_libraries(&self.state.library_manager);
        let current = draft(&self.state);
        let form = &self.state.dialogs.new_veriloga_device;
        // A refusal about a field stands under that field. The status line
        // holds only what belongs to none: no library to write, or a create
        // the project refused.
        let status = form.error.clone().or_else(|| {
            (!libraries.contains(&form.library))
                .then(|| messages.text(MessageId::VerilogADeviceNoLibrary))
        });
        let cell = cell_line(&self.state, &messages);
        let ports = ports_line(&form.ports, &messages);
        let title = messages.text(MessageId::VerilogADeviceTitle);
        let primary = messages.text(MessageId::VerilogADevicePrimary);
        let description = messages.text(MessageId::VerilogADeviceDescription);
        let library_label = messages.text(MessageId::VerilogADeviceLibrary);
        let cell_label = messages.text(MessageId::VerilogADeviceCell);
        let ports_label = messages.text(MessageId::VerilogADevicePorts);
        let ports_hint = messages.text(MessageId::VerilogADevicePortsHint);
        let name_id = egui::Id::new("workbench.new-veriloga-device.name");
        let ports_id = egui::Id::new("workbench.new-veriloga-device.ports");

        let form = &mut self.state.dialogs.new_veriloga_device;
        let typed_before = (form.name.clone(), form.ports.clone());
        let mut picked = None;
        let choice = CommandForm::new(&title, &primary)
            .describe(&description)
            .primary_enabled(matches!(current, Draft::Ready { .. }) && form.error.is_none())
            .fields_enabled(!libraries.is_empty())
            .status(match status.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .status_lines(2)
            .status_fixed_at_open()
            .show(ctx, |rows| {
                picked = rows.select(
                    &library_label,
                    "workbench.new-veriloga-device.library",
                    &form.library,
                    &libraries,
                );
                rows.text(&cell_label, name_id, &mut form.name, "");
                rows.field_line(Some(name_id), cell.field_line());
                rows.text(&ports_label, ports_id, &mut form.ports, &ports_hint);
                rows.field_line(Some(ports_id), ports.field_line());
                Some(name_id)
            });
        if let Some(index) = picked
            && let Some(library) = libraries.get(index)
        {
            form.library = library.clone();
            form.error = None;
        }
        if (form.name.as_str(), form.ports.as_str())
            != (typed_before.0.as_str(), typed_before.1.as_str())
        {
            form.error = None;
        }
        match choice {
            // Revalidate the post-edit frame: Enter must never create a cell
            // the same frame's keystroke just made invalid. A draft refusal is
            // already under its field; only a failure of the create itself
            // reaches the status line.
            DialogChoice::Primary if matches!(draft(&self.state), Draft::Ready { .. }) => {
                if let Err(error) = self.commit_new_veriloga_device() {
                    self.state.dialogs.new_veriloga_device.error = Some(error);
                }
            }
            DialogChoice::Primary => {}
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.new_veriloga_device = NewVerilogADeviceFormState::default();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    /// Create the drafted cell in one library transaction and open its
    /// module in Design.
    pub(in crate::workbench) fn commit_new_veriloga_device(
        &mut self,
    ) -> Result<CellViewRef, String> {
        let (cell, ports) = match draft(&self.state) {
            Draft::Ready { cell, ports } => (cell, ports),
            Draft::Refused(refusal) => return Err(refusal),
            Draft::Empty => {
                return Err(self
                    .state
                    .ui
                    .messages()
                    .text(MessageId::VerilogADeviceCellInvalid));
            }
        };
        let library = self.state.dialogs.new_veriloga_device.library.clone();
        let reference = CellViewRef::new(&library, &cell, VERILOGA_VIEW);

        let mut libraries = self.state.library_manager.clone();
        libraries
            .edit_library(&library)
            .ok_or_else(|| format!("Library '{library}' not found"))?
            .add_cell(Cell::new(&cell));
        let mut sources = self.state.workspace.content.project_sources.clone();
        let receipt = seed_veriloga_view(
            &self.state,
            &mut libraries,
            &mut sources,
            &reference,
            &ports,
        )?;
        let prepared = self.state.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CreateCell {
                library: library.clone(),
                cell: cell.clone(),
            },
        )?;

        self.state.workspace.content.project_sources = sources;
        self.state.workspace.content.project_sources_dirty = true;
        self.state.library_manager = libraries;
        self.state.publish_project_library_mutation(prepared);
        let form = std::mem::take(&mut self.state.dialogs.new_veriloga_device);
        // The starter compile above is not the reader's: the device is offered
        // for placement after the first compile they start leaves it a port.
        let handoffs = &mut self.state.ui.code_workspace.veriloga.placement_handoffs;
        handoffs.retain(|handoff| handoff.view != reference);
        handoffs.push(VerilogAPlacementHandoff {
            view: reference.clone(),
            schematic: form.origin,
        });
        if library == VERILOGA_LIBRARY_NAME {
            self.persist_global_veriloga_library_with_feedback();
        }
        // The module is a view of the cell, so it opens as a Design document.
        self.state.open_workspace_view(reference.clone());
        self.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        self.state.ui.code_workspace.veriloga.receipt = Some(receipt);
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Created {} with a starter module and the symbol generated from its ports. Write the module and compile it.",
            reference.display_path()
        )));
        Ok(reference)
    }
}

#[cfg(test)]
mod tests;

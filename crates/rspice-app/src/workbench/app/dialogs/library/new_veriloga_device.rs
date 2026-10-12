//! New Verilog-A device…: a cell whose module the reader writes next.
//!
//! The form asks three questions — which library, what the cell is called,
//! and which ports the module starts with — and states what the answers make:
//! the module's name, and how many ports its generated symbol starts with.
//! The ports are a starting point only. Once the module compiles, the module
//! defines them, so the field is optional, and a module typed with none
//! declares none: nothing is invented in its place.
//!
//! Nothing is created unless all of it is. The cell, its `veriloga` view, the
//! view's source bundle and the generated `symbol` view are made on copies of
//! the catalog and the source registry and published as one library
//! transaction. The draft is measured against the live catalog every frame,
//! so a catalog that moves while the form is open is judged as it is now
//! rather than refused for having moved.

use std::collections::HashSet;

use egui::Context;
use rspice_design::symbol::generated::{PortEntryError, parse_port_entry};

use super::new_view_dialog::{
    is_verilog_identifier, starter_veriloga_source, verilog_module_name, verilog_port_declaration,
};
use super::{RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::diagnostics::ConsoleMessage;
use crate::state::workspace::VERILOGA_VIEW;
use crate::state::{
    Cell, CellViewRef, LibraryManager, PortDirection, PortSpec, ProjectSourceBundle,
    ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry, View, ViewType,
};
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::dialogs::state::NewVerilogADeviceFormState;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::{
    VerilogACompileReceipt, apply_receipt_interface, compile_project_bundle_receipt,
    writable_libraries,
};
use crate::workbench::{MessageCatalog, MessageId};

/// Open New Verilog-A device… on the library holding the active design.
pub(crate) fn open_new_veriloga_device_form(state: &mut AppState) {
    let libraries = writable_libraries(&state.library_manager);
    let active = &state.workspace.content.active_view.library;
    let library = libraries
        .iter()
        .find(|name| *name == active)
        .or_else(|| libraries.first())
        .cloned()
        .unwrap_or_default();
    state.dialogs.new_veriloga_device = NewVerilogADeviceFormState {
        open: true,
        library,
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
    let cell = form.name.trim();
    if cell.is_empty() {
        return Draft::Empty;
    }
    if !is_verilog_identifier(cell) {
        return Draft::Refused(messages.text(MessageId::VerilogADeviceCellInvalid));
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
        return Draft::Refused(messages.format(
            MessageId::VerilogADeviceCellExists,
            &[("library", &form.library), ("cell", cell)],
        ));
    }
    match typed_ports(&form.ports, &messages) {
        Ok(ports) => Draft::Ready {
            cell: cell.to_owned(),
            ports,
        },
        Err(refusal) => Draft::Refused(refusal),
    }
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

/// The line under Cell name: the module the cell's view will declare.
fn module_line(name: &str, messages: &MessageCatalog) -> Option<String> {
    let name = name.trim();
    is_verilog_identifier(name).then(|| {
        messages.format(
            MessageId::VerilogADeviceModule,
            &[("module", &verilog_module_name(name))],
        )
    })
}

/// The line under Ports: how many the generated symbol starts with.
fn ports_line(typed: &str, messages: &MessageCatalog) -> Option<String> {
    let ports = typed_ports(typed, messages).ok()?;
    Some(match ports.len() {
        0 => messages.text(MessageId::VerilogADeviceNoPorts),
        1 => messages.text(MessageId::VerilogADeviceOnePort),
        count => messages.format(
            MessageId::VerilogADevicePortCount,
            &[("count", &count.to_string())],
        ),
    })
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
        let refusal = form.error.clone().or(match &current {
            Draft::Refused(message) => Some(message.clone()),
            Draft::Empty | Draft::Ready { .. } => None,
        });
        let module = module_line(&form.name, &messages);
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
            .status(match refusal.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .status_lines(2)
            .show(ctx, |rows| {
                picked = rows.select(
                    &library_label,
                    "workbench.new-veriloga-device.library",
                    &form.library,
                    &libraries,
                );
                rows.text(&cell_label, name_id, &mut form.name, "");
                if let Some(line) = module.as_deref() {
                    rows.derived(line);
                }
                rows.text(&ports_label, ports_id, &mut form.ports, &ports_hint);
                if let Some(line) = ports.as_deref() {
                    rows.derived(line);
                }
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
            // the same frame's keystroke just made invalid.
            DialogChoice::Primary => {
                if let Err(error) = self.commit_new_veriloga_device() {
                    self.state.dialogs.new_veriloga_device.error = Some(error);
                }
            }
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
        self.state.dialogs.new_veriloga_device = NewVerilogADeviceFormState::default();
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

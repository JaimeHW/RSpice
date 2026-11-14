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
//! under it, which is always there; so nothing moves while the reader types.
//! The footer's status says what Create waits for while there is no name, as
//! New cell's does in the same words, and why it is refused for a failure
//! that belongs to no field: a library that cannot be written, or a create
//! that the project refused.
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

use crate::state::workspace::{
    VERILOGA_LIBRARY_NAME, VERILOGA_VIEW, verilog_module_name, verilog_port_declaration,
};
use crate::state::{Cell, CellViewRef, PortDirection, PortSpec, ViewType};
use crate::state::{VerilogACompileReceipt, VerilogAPlacementHandoff};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::seed_veriloga_view;
use crate::workbench::documents::code_workspace::persist_global_veriloga_library;
use crate::workbench::documents::code_workspace::{VerilogAWorkbenchState, writable_libraries};
use crate::workbench::{MessageCatalog, MessageId};

/// Draft for New Verilog-A device….
#[derive(Debug, Clone, Default)]
pub struct NewVerilogADeviceFormState {
    pub open: bool,
    pub library: String,
    pub name: String,
    /// The ports the starter module declares, as typed: `inp:in d[7:0] vss`.
    pub ports: String,
    /// Why the last commit was refused. Any edit clears it.
    pub error: Option<String>,
    /// The schematic the form was opened from: where the device is offered
    /// for placement once its module compiles with a port.
    pub(crate) origin: Option<crate::state::CellViewRef>,
}

/// The shell's `open_new_veriloga_device_form`, over the model and the form alone.
pub fn open_new_veriloga_device_form_in(model: &Model, form: &mut NewVerilogADeviceFormState) {
    let libraries = writable_libraries(&model.library_manager);
    let active = &model.workspace.content.active_view;
    let library = libraries
        .iter()
        .find(|name| **name == active.library)
        .or_else(|| libraries.first())
        .cloned()
        .unwrap_or_default();
    let origin = matches!(
        model.workspace.content.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    )
    .then(|| active.clone());
    *form = NewVerilogADeviceFormState {
        open: true,
        library,
        origin,
        ..NewVerilogADeviceFormState::default()
    };
}

/// Whether New Verilog-A device… can make anything at all: it needs a
/// library it can write.
pub fn new_veriloga_device_available_in(model: &Model) -> bool {
    !writable_libraries(&model.library_manager).is_empty()
}

/// What the typed draft is, measured against the live catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Draft {
    /// No cell name yet. The primary is off and the form says nothing.
    Empty,
    /// A refusal the reader has to fix.
    Refused(String),
    /// A cell ready to create, with the ports its starter module declares.
    Ready { cell: String, ports: Vec<PortSpec> },
}

/// The form's draft, measured against the model's catalog and stated in
/// the reader's messages.
pub fn draft_in(
    form: &NewVerilogADeviceFormState,
    model: &Model,
    messages: &MessageCatalog,
) -> Draft {
    if !writable_libraries(&model.library_manager).contains(&form.library) {
        return Draft::Refused(messages.text(MessageId::VerilogADeviceNoLibrary));
    }
    match typed_cell(form, model, messages) {
        None => Draft::Empty,
        Some(Err(refusal)) => Draft::Refused(refusal),
        Some(Ok(cell)) => match typed_ports(&form.ports, messages) {
            Ok(ports) => Draft::Ready { cell, ports },
            Err(refusal) => Draft::Refused(refusal),
        },
    }
}

/// The cell the Cell name field names: none yet, one the library can take,
/// or why it cannot. A cell name is letters, digits and underscores, as New
/// Cell's is; the module it is named after is made from it by
/// [`verilog_module_name`].
fn typed_cell(
    form: &NewVerilogADeviceFormState,
    model: &Model,
    messages: &MessageCatalog,
) -> Option<Result<String, String>> {
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
    let taken = model
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
pub enum Line {
    /// Nothing declared yet: the slot keeps its height and stays blank.
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

/// The line under Cell name: blank while there is none (the footer says
/// what Create waits for), the module the cell's view will declare, or why
/// the name is refused.
pub fn cell_line(
    form: &NewVerilogADeviceFormState,
    model: &Model,
    messages: &MessageCatalog,
) -> Line {
    match typed_cell(form, model, messages) {
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
pub fn ports_line(typed: &str, messages: &MessageCatalog) -> Line {
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

/// Draw New Verilog-A device… while it is open, over the form, the model and
/// the reader's messages. Returns whether the reader asked to create a draft
/// that is ready on this frame's edits.
pub fn render_new_veriloga_device_form(
    form: &mut NewVerilogADeviceFormState,
    model: &Model,
    messages: &MessageCatalog,
    ctx: &Context,
) -> bool {
    if !form.open {
        return false;
    }
    let libraries = writable_libraries(&model.library_manager);
    let current = draft_in(form, model, messages);
    // A refusal about a field stands under that field. The status line
    // holds what belongs to none: no library to write, or a create the
    // project refused; and, as every form's does, what Create waits for.
    let status = form.error.clone().or_else(|| {
        (!libraries.contains(&form.library))
            .then(|| messages.text(MessageId::VerilogADeviceNoLibrary))
    });
    let waiting = form
        .name
        .trim()
        .is_empty()
        .then(|| messages.text(MessageId::VerilogADeviceCellMissing));
    let cell = cell_line(form, model, messages);
    let ports = ports_line(&form.ports, messages);
    let title = messages.text(MessageId::VerilogADeviceTitle);
    let primary = messages.text(MessageId::VerilogADevicePrimary);
    let description = messages.text(MessageId::VerilogADeviceDescription);
    let library_label = messages.text(MessageId::VerilogADeviceLibrary);
    let cell_label = messages.text(MessageId::VerilogADeviceCell);
    let ports_label = messages.text(MessageId::VerilogADevicePorts);
    let ports_hint = messages.text(MessageId::VerilogADevicePortsHint);
    let name_id = egui::Id::new("workbench.new-veriloga-device.name");
    let ports_id = egui::Id::new("workbench.new-veriloga-device.ports");

    let typed_before = (form.name.clone(), form.ports.clone());
    let mut picked = None;
    let choice = CommandForm::new(&title, &primary)
        .describe(&description)
        .primary_enabled(matches!(current, Draft::Ready { .. }) && form.error.is_none())
        .fields_enabled(!libraries.is_empty())
        .status(match status.as_deref() {
            Some(message) => Err(message),
            None => Ok(waiting.as_deref()),
        })
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
        DialogChoice::Primary => {
            matches!(draft_in(form, model, messages), Draft::Ready { .. })
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            *form = NewVerilogADeviceFormState::default();
            false
        }
        DialogChoice::None | DialogChoice::Secondary => false,
    }
}

/// What New Verilog-A device… made: the module's cell view, which opens in
/// Design, the receipt of its starter compile, and the line the log records.
pub struct CreatedVerilogADevice {
    pub reference: CellViewRef,
    pub receipt: VerilogACompileReceipt,
    pub record: String,
}

/// Create the drafted cell in one library transaction, and remember the
/// schematic the form came from for the placement offer. The shell opens the
/// module in Design (commands/veriloga_commits.rs).
pub fn publish_new_veriloga_device(
    model: &mut Model,
    form: &mut NewVerilogADeviceFormState,
    veriloga: &mut VerilogAWorkbenchState,
    messages: &MessageCatalog,
) -> Result<CreatedVerilogADevice, String> {
    let (cell, ports) = match draft_in(form, model, messages) {
        Draft::Ready { cell, ports } => (cell, ports),
        Draft::Refused(refusal) => return Err(refusal),
        Draft::Empty => {
            return Err(messages.text(MessageId::VerilogADeviceCellInvalid));
        }
    };
    let library = form.library.clone();
    let reference = CellViewRef::new(&library, &cell, VERILOGA_VIEW);

    let mut libraries = model.library_manager.clone();
    libraries
        .edit_library(&library)
        .ok_or_else(|| format!("Library '{library}' not found"))?
        .add_cell(Cell::new(&cell));
    let mut sources = model.workspace.content.project_sources.clone();
    let receipt = seed_veriloga_view(model, &mut libraries, &mut sources, &reference, &ports)?;
    let prepared = model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::CreateCell {
            library: library.clone(),
            cell: cell.clone(),
        },
    )?;

    model.workspace.content.project_sources = sources;
    model.workspace.content.project_sources_dirty = true;
    model.library_manager = libraries;
    model.publish_project_library_mutation(prepared);
    let form = std::mem::take(form);
    // The starter compile above is not the reader's: the device is offered
    // for placement after the first compile they start leaves it a port.
    let handoffs = &mut veriloga.placement_handoffs;
    handoffs.retain(|handoff| handoff.view != reference);
    handoffs.push(VerilogAPlacementHandoff {
        view: reference.clone(),
        schematic: form.origin,
    });
    if library == VERILOGA_LIBRARY_NAME {
        persist_global_veriloga_library(model);
    }
    let record = format!(
        "Created {} with a starter module and the symbol generated from its ports. Write the module and compile it.",
        reference.display_path()
    );
    Ok(CreatedVerilogADevice {
        reference,
        receipt,
        record,
    })
}

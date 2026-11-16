//! New Verilog-A device…: a cell whose module the reader writes next.
//!
//! The form asks which library, what the cell is called, and which ports the
//! module starts with, and states what the answers make: the module's name,
//! which is the cell's own wherever a module can carry it, and how many ports
//! its generated symbol starts with.
//!
//! The ports are a pin list, as a schematic tool takes one: a row of names
//! for each direction, Inputs, Outputs and Inouts, so a name is all anyone
//! types and the row says which way it points. A bus is a name too,
//! `d[7:0]`. The module declares them in row order, inputs first, each row
//! in the order typed, and the generated symbol's pins follow. Every port is
//! electrical. They are a starting point only: once the module compiles, it
//! defines them, so every row is optional, and a module typed with none
//! declares none; nothing is invented in its place.
//!
//! Each field states what it declares, or why it is refused, in the one line
//! under it, which is always there, Library's too; so every row stands on one
//! pitch and nothing moves while the reader types. The footer's status says
//! what Create waits for while there is no name, as New cell's does in the
//! same words, then how many ports the symbol starts with, and why Create is
//! refused for a failure that belongs to no field: a library that cannot be
//! written, or a create that the project refused.
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

use std::collections::HashMap;

use egui::Context;
use rspice_design::symbol::generated::parse_port_entry;

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
    /// The input ports' names, as typed: `inp inn` or `d[7:0], clk`.
    pub inputs: String,
    /// The output ports' names, as typed.
    pub outputs: String,
    /// The inout ports' names, as typed.
    pub inouts: String,
    /// Why the last commit was refused. Any edit clears it.
    pub error: Option<String>,
    /// The schematic the form was opened from: where the device is offered
    /// for placement once its module compiles with a port.
    pub(crate) origin: Option<crate::state::CellViewRef>,
}

impl NewVerilogADeviceFormState {
    /// The names typed in `row`.
    pub fn row(&self, row: PortRow) -> &str {
        match row {
            PortRow::Inputs => &self.inputs,
            PortRow::Outputs => &self.outputs,
            PortRow::Inouts => &self.inouts,
        }
    }

    fn row_mut(&mut self, row: PortRow) -> &mut String {
        match row {
            PortRow::Inputs => &mut self.inputs,
            PortRow::Outputs => &mut self.outputs,
            PortRow::Inouts => &mut self.inouts,
        }
    }
}

/// One of the form's port rows: the direction every name in it has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortRow {
    Inputs,
    Outputs,
    Inouts,
}

impl PortRow {
    /// The rows in module order, which is the order the form shows them.
    pub const ALL: [Self; 3] = [Self::Inputs, Self::Outputs, Self::Inouts];

    pub fn direction(self) -> PortDirection {
        match self {
            Self::Inputs => PortDirection::In,
            Self::Outputs => PortDirection::Out,
            Self::Inouts => PortDirection::InOut,
        }
    }

    fn label(self) -> MessageId {
        match self {
            Self::Inputs => MessageId::VerilogADeviceInputs,
            Self::Outputs => MessageId::VerilogADeviceOutputs,
            Self::Inouts => MessageId::VerilogADeviceInouts,
        }
    }

    fn hint(self) -> MessageId {
        match self {
            Self::Inputs => MessageId::VerilogADeviceInputsHint,
            Self::Outputs => MessageId::VerilogADeviceOutputsHint,
            Self::Inouts => MessageId::VerilogADeviceInoutsHint,
        }
    }

    /// What a name typed again elsewhere is told about its first place.
    fn already(self) -> MessageId {
        match self {
            Self::Inputs => MessageId::VerilogADevicePortIsInput,
            Self::Outputs => MessageId::VerilogADevicePortIsOutput,
            Self::Inouts => MessageId::VerilogADevicePortIsInout,
        }
    }

    fn id(self) -> egui::Id {
        egui::Id::new(("workbench.new-veriloga-device.ports", self as u8))
    }
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
        Some(ViewType::Schematic | ViewType::Testbench)
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
        Some(Ok(cell)) => {
            let typed = typed_ports(form, messages);
            match typed.refusals.into_iter().flatten().next() {
                Some(refusal) => Draft::Refused(refusal),
                None => Draft::Ready {
                    cell,
                    ports: typed.ports,
                },
            }
        }
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

/// What the three port rows declare together.
struct TypedPorts {
    /// Every port accepted: the inputs, then the outputs, then the inouts,
    /// each row's in the order typed. This is the module's port order.
    ports: Vec<PortSpec>,
    /// Each row's first refused entry, by [`PortRow::ALL`].
    refusals: [Option<String>; 3],
}

/// The ports the rows declare. A row is names separated by spaces or
/// commas, each a name or a bus like `d[7:0]` that a Verilog-A module can
/// declare, with the row's direction. An entry that states a direction of
/// its own, `inp:in`, is refused rather than read: the row already says it.
/// A name is declared once; typed again, in any row, the later one is
/// refused, saying where it already is.
fn typed_ports(form: &NewVerilogADeviceFormState, messages: &MessageCatalog) -> TypedPorts {
    let mut typed = TypedPorts {
        ports: Vec::new(),
        refusals: [None, None, None],
    };
    let mut declared: HashMap<String, PortRow> = HashMap::new();
    for (index, row) in PortRow::ALL.into_iter().enumerate() {
        let entries = form
            .row(row)
            .split(|character: char| character.is_whitespace() || character == ',')
            .filter(|entry| !entry.is_empty());
        for entry in entries {
            match typed_port(entry, row, &declared, messages) {
                Ok((base, port)) => {
                    declared.insert(base, row);
                    typed.ports.push(port);
                }
                Err(refusal) => {
                    typed.refusals[index].get_or_insert(refusal);
                }
            }
        }
    }
    typed
}

/// One entry of `row`, with the lower-case name it declares, or why it is
/// refused. `declared` holds the names the entries before it declared.
fn typed_port(
    entry: &str,
    row: PortRow,
    declared: &HashMap<String, PortRow>,
    messages: &MessageCatalog,
) -> Result<(String, PortSpec), String> {
    // The ports codec reads a direction after an entry's last colon outside
    // a bus range. Whatever it reads one from, or fails to, carried one.
    let name = match parse_port_entry(entry) {
        Ok(port) if port.name == entry => port.name,
        _ => {
            return Err(messages.format(MessageId::VerilogADevicePortDirected, &[("entry", entry)]));
        }
    };
    let Some((base, _)) = verilog_port_declaration(&name) else {
        return Err(messages.format(MessageId::VerilogADevicePortName, &[("entry", entry)]));
    };
    let key = base.to_ascii_lowercase();
    if let Some(first) = declared.get(&key) {
        return Err(messages.format(first.already(), &[("port", base)]));
    }
    let port = PortSpec {
        direction: row.direction(),
        name,
    };
    Ok((key, port))
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

/// The lines under the port rows, by [`PortRow::ALL`]: each blank, or why
/// its row's first refused entry is refused.
pub fn port_lines(form: &NewVerilogADeviceFormState, messages: &MessageCatalog) -> [Line; 3] {
    typed_ports(form, messages)
        .refusals
        .map(|refusal| refusal.map_or(Line::Blank, Line::Error))
}

/// The footer's status: why Create is refused for a failure that belongs to
/// no field, else what it waits for while there is no name, else, for a
/// draft that is ready, how many ports the generated symbol starts with.
pub fn status_in(
    form: &NewVerilogADeviceFormState,
    model: &Model,
    messages: &MessageCatalog,
) -> Result<Option<String>, String> {
    if let Some(error) = &form.error {
        return Err(error.clone());
    }
    match draft_in(form, model, messages) {
        // The one refusal that stands under no field.
        Draft::Refused(_)
            if !writable_libraries(&model.library_manager).contains(&form.library) =>
        {
            Err(messages.text(MessageId::VerilogADeviceNoLibrary))
        }
        Draft::Refused(_) => Ok(None),
        Draft::Empty => Ok(Some(messages.text(MessageId::VerilogADeviceCellMissing))),
        Draft::Ready { ports, .. } => Ok(Some(match ports.len() {
            0 => messages.text(MessageId::VerilogADeviceNoPorts),
            1 => messages.text(MessageId::VerilogADeviceOnePort),
            count => messages.format(
                MessageId::VerilogADevicePortCount,
                &[("count", &count.to_string())],
            ),
        })),
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
    // project refused; and, as every form's does, what Create waits for,
    // then what it will make.
    let status = status_in(form, model, messages);
    let cell = cell_line(form, model, messages);
    let port_lines = port_lines(form, messages);
    let title = messages.text(MessageId::VerilogADeviceTitle);
    let primary = messages.text(MessageId::VerilogADevicePrimary);
    let description = messages.text(MessageId::VerilogADeviceDescription);
    let library_label = messages.text(MessageId::VerilogADeviceLibrary);
    let cell_label = messages.text(MessageId::VerilogADeviceCell);
    let port_words =
        PortRow::ALL.map(|row| (messages.text(row.label()), messages.text(row.hint())));
    let name_id = egui::Id::new("workbench.new-veriloga-device.name");

    let typed_before = form.clone();
    let mut picked = None;
    let choice = CommandForm::new(&title, &primary)
        .describe(&description)
        .primary_enabled(matches!(current, Draft::Ready { .. }) && form.error.is_none())
        .fields_enabled(!libraries.is_empty())
        .status(match &status {
            Ok(fact) => Ok(fact.as_deref()),
            Err(refusal) => Err(refusal.as_str()),
        })
        .show(ctx, |rows| {
            picked = rows.select(
                &library_label,
                "workbench.new-veriloga-device.library",
                &form.library,
                &libraries,
            );
            // Library declares nothing, but its slot keeps every row of the
            // form on one pitch.
            rows.field_line(None, FieldLine::Blank);
            rows.text(&cell_label, name_id, &mut form.name, "");
            rows.field_line(Some(name_id), cell.field_line());
            for ((row, (label, hint)), line) in
                PortRow::ALL.into_iter().zip(&port_words).zip(&port_lines)
            {
                rows.text(label, row.id(), form.row_mut(row), hint);
                rows.field_line(Some(row.id()), line.field_line());
            }
            Some(name_id)
        });
    if let Some(index) = picked
        && let Some(library) = libraries.get(index)
    {
        form.library = library.clone();
        form.error = None;
    }
    if form.name != typed_before.name
        || PortRow::ALL
            .into_iter()
            .any(|row| form.row(row) != typed_before.row(row))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::UiTextLocale;

    fn messages() -> MessageCatalog {
        MessageCatalog::new(UiTextLocale::EnglishUnitedStates)
    }

    /// A form with `rows` typed into Inputs, Outputs and Inouts.
    fn form(rows: [&str; 3]) -> NewVerilogADeviceFormState {
        let [inputs, outputs, inouts] = rows.map(str::to_owned);
        NewVerilogADeviceFormState {
            inputs,
            outputs,
            inouts,
            ..NewVerilogADeviceFormState::default()
        }
    }

    fn ports(rows: [&str; 3]) -> Vec<(String, PortDirection)> {
        let typed = typed_ports(&form(rows), &messages());
        assert_eq!(typed.refusals, [None, None, None], "{rows:?}");
        typed
            .ports
            .into_iter()
            .map(|port| (port.name, port.direction))
            .collect()
    }

    fn lines(rows: [&str; 3]) -> [Line; 3] {
        port_lines(&form(rows), &messages())
    }

    /// A name says nothing of its direction: its row does. A bus is a name.
    #[test]
    fn a_port_takes_its_direction_from_its_row() {
        use PortDirection::{In, InOut, Out};
        let typed = ports(["inp, d[7:0]", "out", "vdd,vss"]);
        assert_eq!(
            typed,
            [
                ("inp".to_owned(), In),
                ("d[7:0]".to_owned(), In),
                ("out".to_owned(), Out),
                ("vdd".to_owned(), InOut),
                ("vss".to_owned(), InOut),
            ]
        );
        assert_eq!(
            lines(["inp, d[7:0]", "out", "vdd,vss"]),
            [Line::Blank, Line::Blank, Line::Blank]
        );
        assert!(ports(["", " , ", ""]).is_empty());
    }

    /// The module declares the inputs, then the outputs, then the inouts,
    /// each row's in the order typed, whatever order the names would sort in.
    #[test]
    fn ports_are_declared_inputs_then_outputs_then_inouts_each_as_typed() {
        let names = ports(["zeta alpha", "mid", "beta aardvark"])
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        assert_eq!(names, ["zeta", "alpha", "mid", "beta", "aardvark"]);
    }

    /// A row already sets the direction, so an entry that states one is
    /// refused in that row's line, whatever direction it states; the colon of
    /// a bus range is no direction. Each line holds its own row's first
    /// refused entry.
    #[test]
    fn an_entry_that_states_a_direction_is_refused_in_its_rows_line() {
        let directed =
            |entry: &str| Line::Error(format!("{entry}: the row already sets the direction."));
        assert_eq!(
            lines(["inp:in inn:in", "out:sideways", "vdd d[7:0]:inout"]),
            [
                directed("inp:in"),
                directed("out:sideways"),
                directed("d[7:0]:inout")
            ]
        );
        assert_eq!(lines(["", "out:", ""])[1], directed("out:"));
        assert_eq!(
            lines(["9x a<3:0>", "", ""])[0],
            Line::Error("9x: not a port name or a bus like d[7:0].".to_owned())
        );
    }

    /// A name is declared once. Typed again, in its own row or another, the
    /// later one is refused under its row, saying where the name already is;
    /// the first stands.
    #[test]
    fn a_name_typed_twice_is_refused_where_it_is_typed_again() {
        assert_eq!(
            lines(["inp out", "out", "OUT"]),
            [
                Line::Blank,
                Line::Error("out is already an input.".to_owned()),
                Line::Error("OUT is already an input.".to_owned()),
            ]
        );
        assert_eq!(
            lines(["", "y y", "y"]),
            [
                Line::Blank,
                Line::Error("y is already an output.".to_owned()),
                Line::Error("y is already an output.".to_owned()),
            ]
        );
        // A bus is declared by its base name.
        assert_eq!(
            lines(["", "", "d d[3:0]"])[2],
            Line::Error("d is already an inout.".to_owned())
        );
    }
}

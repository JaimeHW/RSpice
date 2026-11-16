//! Initial conditions and node sets: where the run starts.
//!
//! Two net/value grids side by side, as Virtuoso ADE's Convergence Aids
//! forms give them: a row names a net of the plan's simulation root as the
//! design names it, picked from the design's nets (the one selected on the
//! schematic first) or typed, and its volts: a number, or an expression of
//! the plan's design variables, so a corner or a sweep that moves one moves
//! where the run starts. A net the design does not have, ground, a net that
//! holds no voltage, a net already set or a variable the plan does not
//! define is refused in place: the cell keeps what was typed, ringed, with
//! the reason on its hover. The rows are the plan's options, written into the deck as `.IC`
//! and `.NODESET` cards, so an edit is one step of the setup's Undo.

use egui::{TextEdit, Ui, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{IconButton, editable_select_mono, name_control, text_field};
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, record_plan_refusal};
use rspice_design::connectivity::summary::DesignNet;
use rspice_simulation::netlist_preparation::startup_nets::{startup_net, startup_value};
use rspice_simulation_contract::design_variable::DesignVariable;
use rspice_simulation_contract::options::{NodeVoltage, StartupKind};

use super::super::page_kit::{Tone, card, cell_ui, ledger_head_explained, ledger_row_cells};
use super::{commit_draft, released};

/// Net, volts, remove.
const COLUMNS: [f32; 3] = [0.56, 0.32, 0.12];

/// The two grids side by side, each as tall as the longer, so neither card
/// ends short of the other.
pub(super) fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let nets = cx.model.simulation_root_nets();
    let selected = cx
        .model
        .schematic
        .session
        .editor
        .net_highlight
        .selected_net_name
        .clone();
    let choices = match &nets {
        Ok(nets) => net_choices(nets, selected.as_deref()),
        Err(_) => Vec::new(),
    };
    let draft = &cx.model.sim_setup.session.options_draft;
    let rows = draft.initial_conditions.len().max(draft.nodesets.len()) + 1;
    let nets = nets.as_ref().map(|nets| nets.as_slice());
    super::super::page_kit::card_row(
        ui,
        cx,
        |ui, cx| grid(ui, cx, StartupKind::InitialCondition, nets, &choices, rows),
        |ui, cx| grid(ui, cx, StartupKind::NodeSet, nets, &choices, rows),
    );
}

/// The nets a row can name, as the design names them: the net selected on
/// the schematic first, then every other net that holds a voltage.
fn net_choices(nets: &[DesignNet], selected: Option<&str>) -> Vec<String> {
    let mut choices = nets
        .iter()
        .filter(|net| startup_net(&net.name, nets).is_ok())
        .map(|net| net.name.clone())
        .collect::<Vec<_>>();
    if let Some(selected) = selected
        && let Some(index) = choices
            .iter()
            .position(|name| name.eq_ignore_ascii_case(selected))
    {
        let name = choices.remove(index);
        choices.insert(0, name);
    }
    choices
}

/// What one grid's row is called: the run's own word for it, then its
/// place, so a screen reader and a test name the same cell.
fn row_name(kind: StartupKind, row: Option<usize>) -> String {
    let noun = match kind {
        StartupKind::InitialCondition => "initial condition",
        StartupKind::NodeSet => "node set",
    };
    match row {
        Some(index) => format!("{} {}", capitalized(noun), index + 1),
        None => format!("New {noun}"),
    }
}

fn capitalized(text: &str) -> String {
    let mut characters = text.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

fn rows_of(
    kind: StartupKind,
    draft: &crate::simulation::plan::OptionsDialogState,
) -> &[NodeVoltage] {
    match kind {
        StartupKind::InitialCondition => &draft.initial_conditions,
        StartupKind::NodeSet => &draft.nodesets,
    }
}

fn rows_of_mut(
    kind: StartupKind,
    draft: &mut crate::simulation::plan::OptionsDialogState,
) -> &mut Vec<NodeVoltage> {
    match kind {
        StartupKind::InitialCondition => &mut draft.initial_conditions,
        StartupKind::NodeSet => &mut draft.nodesets,
    }
}

const NET_HOVER: &str = "A net of the simulation root as the design names it: pick one, the net \
                         selected on the schematic first, or type its name.";

fn volts_hover(kind: StartupKind) -> &'static str {
    match kind {
        StartupKind::InitialCondition => {
            "Volts, or an expression of the design variables. Held while the operating point a \
             transient starts from is solved, then released (.IC)."
        }
        StartupKind::NodeSet => {
            "Volts, or an expression of the design variables. Where the operating point's search \
             starts; the solution settles where the circuit takes it (.NODESET)."
        }
    }
}

/// What a press in one grid asked for.
enum Edit {
    Net(Option<usize>, String),
    Volts(Option<usize>, String),
    Remove(usize),
}

fn grid(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    kind: StartupKind,
    nets: Result<&[DesignNet], &String>,
    choices: &[String],
    rows: usize,
) {
    let mut edit = None;
    card(ui, kind.label(), Some((kind.card(), Tone::Neutral)), |ui| {
        ledger_head_explained(
            ui,
            &COLUMNS,
            &["Net", "Volts", ""],
            &[NET_HOVER, volts_hover(kind)],
        );
        let set = rows_of(kind, &cx.model.sim_setup.session.options_draft).to_vec();
        for index in 0..rows {
            let (_, cells) = ledger_row_cells(ui, &COLUMNS);
            if index > set.len() {
                continue;
            }
            let row = set.get(index);
            let place = row.map(|_| index);
            let name = row_name(kind, place);
            let pad = vec2(4.0, 4.0);
            let net_rect = cells[0].shrink2(pad);
            let mut cell = cell_ui(ui, net_rect);
            let shown = row.map_or_else(String::new, |row| row.net.clone());
            if let Some(text) = net_cell(&mut cell, &name, &shown, choices, net_rect.width()) {
                edit = Some(Edit::Net(place, text));
            }
            let volts_rect = cells[1].shrink2(pad);
            let mut cell = cell_ui(ui, volts_rect);
            let shown = row.map_or_else(String::new, |row| row.value.clone());
            if let Some(text) = volts_cell(&mut cell, &name, &shown, volts_rect.width()) {
                edit = Some(Edit::Volts(place, text));
            }
            if row.is_some() {
                let mut cell = cell_ui(ui, cells[2].shrink2(pad));
                let remove = format!("Remove {}", name.to_lowercase());
                if IconButton::new(Icon::Trash)
                    .tooltip(&remove)
                    .show(&mut cell)
                    .clicked()
                {
                    edit = Some(Edit::Remove(index));
                }
            }
        }
    });
    if let Some(edit) = edit {
        apply(ui, cx, kind, nets, edit);
    }
}

/// Apply `edit` to the draft and commit it, or refuse it in place.
fn apply(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    kind: StartupKind,
    nets: Result<&[DesignNet], &String>,
    edit: Edit,
) {
    let variables = match edit {
        Edit::Volts(..) => plan_variables(cx.model),
        _ => Vec::new(),
    };
    let draft = &mut cx.model.sim_setup.session.options_draft;
    let refusal = match edit {
        Edit::Remove(index) => {
            rows_of_mut(kind, draft).remove(index);
            None
        }
        Edit::Net(place, text) => {
            let name = row_name(kind, place);
            match drawn_net(kind, draft, place, &text, nets) {
                Ok(net) => match place {
                    Some(index) => {
                        rows_of_mut(kind, draft)[index].net = net;
                        None
                    }
                    None => {
                        pending_set(ui, &name, Column::Net, net);
                        add_when_complete(ui, kind, draft);
                        None
                    }
                },
                Err(reason) => Some((name, Column::Net, text, reason)),
            }
        }
        Edit::Volts(place, text) => {
            let name = row_name(kind, place);
            match startup_value(&text, &variables) {
                Ok(value) => match place {
                    Some(index) => {
                        rows_of_mut(kind, draft)[index].value = value;
                        None
                    }
                    None => {
                        pending_set(ui, &name, Column::Volts, value);
                        add_when_complete(ui, kind, draft);
                        None
                    }
                },
                Err(reason) => Some((name, Column::Volts, text, reason)),
            }
        }
    };
    match refusal {
        None => commit_draft(cx),
        Some((name, column, text, reason)) => {
            ui.data_mut(|data| {
                data.insert_temp(
                    cell_id(&name, column).with("refused"),
                    Refused {
                        text,
                        reason: reason.clone(),
                    },
                );
            });
            record_plan_refusal(
                cx,
                format!("{} were not changed \u{b7} {name}: {reason}", kind.label()),
            );
        }
    }
}

/// The design's spelling of the net `text` names, or why a row cannot name
/// it.
fn drawn_net(
    kind: StartupKind,
    draft: &crate::simulation::plan::OptionsDialogState,
    place: Option<usize>,
    text: &str,
    nets: Result<&[DesignNet], &String>,
) -> Result<String, String> {
    let nets = nets.map_err(Clone::clone)?;
    let net = startup_net(text, nets)?;
    let taken = rows_of(kind, draft)
        .iter()
        .enumerate()
        .any(|(index, row)| Some(index) != place && row.net.eq_ignore_ascii_case(&net.name));
    if taken {
        return Err(format!("{} is already set", net.name));
    }
    Ok(net.name.clone())
}

/// The design variables a value can name: the plan's own.
fn plan_variables(model: &crate::workbench::app_state::Model) -> Vec<DesignVariable> {
    model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| model.workspace.content.plan_data(plan.id()))
        .map(|payload| payload.design_variables.clone())
        .unwrap_or_default()
}

#[derive(Clone, Copy)]
enum Column {
    Net,
    Volts,
}

impl Column {
    const fn word(self) -> &'static str {
        match self {
            Self::Net => "net",
            Self::Volts => "volts",
        }
    }
}

/// Where a cell keeps what is being typed into it, what it refused, and, on
/// the new row, what it holds until the row is complete.
fn cell_id(name: &str, column: Column) -> egui::Id {
    egui::Id::new(("startup-cell", name.to_owned(), column.word()))
}

fn pending_set(ui: &Ui, name: &str, column: Column, text: String) {
    ui.data_mut(|data| data.insert_temp(cell_id(name, column).with("pending"), text));
}

fn pending(ui: &Ui, name: &str, column: Column) -> Option<String> {
    ui.data(|data| data.get_temp::<String>(cell_id(name, column).with("pending")))
}

/// The new row joins the grid once it names both its net and its volts.
fn add_when_complete(
    ui: &Ui,
    kind: StartupKind,
    draft: &mut crate::simulation::plan::OptionsDialogState,
) {
    let name = row_name(kind, None);
    let (Some(net), Some(value)) = (
        pending(ui, &name, Column::Net),
        pending(ui, &name, Column::Volts),
    ) else {
        return;
    };
    rows_of_mut(kind, draft).push(NodeVoltage { net, value });
    ui.data_mut(|data| {
        for column in [Column::Net, Column::Volts] {
            data.remove::<String>(cell_id(&name, column).with("pending"));
        }
    });
}

/// A cell's typed text the grid refused, and why.
#[derive(Clone)]
struct Refused {
    text: String,
    reason: String,
}

/// The text a cell shows: what is being typed, else what it refused, else
/// what the new row holds, else `shown`.
fn cell_text(ui: &Ui, name: &str, column: Column, shown: &str) -> (String, Option<Refused>) {
    let id = cell_id(name, column);
    let refused = ui.data(|data| data.get_temp::<Refused>(id.with("refused")));
    let text = ui
        .data(|data| data.get_temp::<String>(id))
        .or_else(|| refused.as_ref().map(|refused| refused.text.clone()))
        .or_else(|| pending(ui, name, column))
        .unwrap_or_else(|| shown.to_owned());
    (text, refused)
}

/// Ring a refused cell and give its reason on hover and to a screen reader.
fn ring(ui: &Ui, response: egui::Response, refused: Option<&Refused>) -> egui::Response {
    let Some(refused) = refused else {
        return response;
    };
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_stroke(
        response.rect,
        t.radius,
        egui::Stroke::new(1.0, t.color.err),
        egui::StrokeKind::Inside,
    );
    let reason = refused.reason.clone();
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_description(reason));
    response.on_hover_text(&refused.reason)
}

/// Keep what is typed while the cell has focus; hand it back, and forget
/// what was refused, when the cell is released.
fn settle(ui: &Ui, id: egui::Id, response: &egui::Response, text: &str) -> Option<String> {
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text.to_owned()));
    }
    if !released(response) {
        return None;
    }
    ui.data_mut(|data| {
        data.remove::<String>(id);
        data.remove::<Refused>(id.with("refused"));
    });
    Some(text.to_owned())
}

fn net_cell(
    ui: &mut Ui,
    name: &str,
    shown: &str,
    choices: &[String],
    width: f32,
) -> Option<String> {
    let id = cell_id(name, Column::Net);
    let (mut text, refused) = cell_text(ui, name, Column::Net, shown);
    let label = format!("{name} net");
    let output = editable_select_mono(ui, &label, &label, &mut text, "net", choices, width);
    if let Some(choice) = output.picked.and_then(|index| choices.get(index)) {
        ui.data_mut(|data| {
            data.remove::<String>(id);
            data.remove::<Refused>(id.with("refused"));
        });
        return Some(choice.clone());
    }
    let field = ring(ui, output.field, refused.as_ref());
    settle(ui, id, &field, &text).filter(|typed| typed.trim() != shown)
}

fn volts_cell(ui: &mut Ui, name: &str, shown: &str, width: f32) -> Option<String> {
    let id = cell_id(name, Column::Volts);
    let (mut text, refused) = cell_text(ui, name, Column::Volts, shown);
    let label = format!("{name} volts");
    let response = text_field(
        ui,
        TextEdit::singleline(&mut text)
            .id(id.with("field"))
            .hint_text("V")
            .font(theme::field_mono())
            .margin(egui::Margin::symmetric(5, 4)),
        width,
    );
    name_control(ui, &response, &label);
    let response = ring(ui, response, refused.as_ref());
    settle(ui, id, &response, &text).filter(|typed| typed.trim() != shown)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The new row is named as such, a set row by its place, so each cell
    /// has one name a screen reader reads and a test presses.
    #[test]
    fn each_cell_is_named_by_its_grid_and_place() {
        assert_eq!(
            row_name(StartupKind::InitialCondition, Some(0)),
            "Initial condition 1"
        );
        assert_eq!(row_name(StartupKind::NodeSet, None), "New node set");
    }
}

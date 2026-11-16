//! Connect rules: what converts at the design's analog/digital boundaries.
//!
//! Verilog-AMS converts every boundary with the connect module a
//! `connectrules` block selects (LRM 2.4 section 7.7). The page states which
//! block the run selects (`.OPTIONS CONNECTRULES`), and then that block's
//! statements in one grid: the run's own block is edited there cell by cell,
//! while a shipped set and a block the design or a bound model library
//! declares are shown as written, a project's one opening its source. The
//! run's block is stored with the plan's options and written into the deck
//! as a `.CONNECTRULES` card.
//!
//! The page carries no prose: what a control or a column means is its hover,
//! and a table with no rows says why in one line.
//!
//! Below them, the converters the checked run's decks insert, one row each
//! as the engine reports them (Virtuoso AMS's elaboration report of inserted
//! connect modules), each selecting the net it converts on the schematic.

use egui::{RichText, TextEdit, Ui, vec2};

use crate::simulation::plan::connect_rules::{
    self, ConnectRuleColumn, DEFAULT_BASE, DeclaredConnectRules, DeclaredIn,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, editable_select_mono, name_control, select_with_response, text_field,
};
use crate::workbench::app_state::run_preflight::CheckedConverter;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, record_plan_refusal};
use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticRoute;
use rspice_simulation_contract::options::{
    RUN_CONNECT_MODULES, RunConnectRules, RunConnectStatement, SimulationOptions,
};

use super::page_kit::{
    CARD_PAD_X, RowPress, Tone, card, card_body, card_empty, card_head_row, cell_ui, field_pair,
    ledger_group, ledger_head_explained, ledger_row, ledger_row_cells, switch_cell,
};
use super::page_solver::{commit_draft, released};

/// The CONNECTRULES list's entry for an empty value, and the hint the empty
/// field shows.
const DESIGN_DEFAULT_RULES: &str = "(design default)";

/// The CONNECTRULES field's hover.
const RULES_HOVER: &str = "The connectrules block that converts at every analog/digital \
                           boundary. The design default takes the design's only block; \
                           run_rules is this run's own block; an rspice_ set is RSpice's; any \
                           other name is a block the design or a bound library declares.";

/// The grid's columns: the module, what it converts, then
/// [`ConnectRuleColumn::ALL`].
const GRID: [f32; 9] = [0.11, 0.08, 0.12, 0.11, 0.11, 0.11, 0.11, 0.125, 0.125];

/// The converter table's columns: [`CheckedConverter::cells`].
const REPORT: [f32; 7] = [0.11, 0.13, 0.08, 0.10, 0.13, 0.10, 0.35];

const REPORT_LABELS: [&str; 7] = [
    "Net",
    "Port",
    "Converts",
    "Module",
    "Rule",
    "Supply",
    "Set by statement",
];

const REPORT_HOVERS: [&str; 7] = [
    "The deck net an analog node and a discrete port share. Select a row to show it on the \
     schematic.",
    "The instance and port whose discrete side meets the net.",
    "A\u{2192}D reads the net into a logic input, D\u{2192}A drives it from a logic output, \
     A\u{2194}D both ways; R is a real-valued port.",
    "The connect module inserted on the net.",
    "The connectrules block, or the rule set, that chose the module.",
    "The supply the converter converts against.",
    "The parameters the connect statement sets. Hover a row for every value and where it came \
     from.",
];

pub(super) fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let declared = declared_blocks(ui, cx);
    selection_card(ui, cx, &declared);
    ui.add_space(12.0);
    statements_card(ui, cx, &declared);
    ui.add_space(12.0);
    converters_card(ui, cx);
}

/// The converters the checked run's decks insert, one row each as the engine
/// reports them; selecting one shows the net it converts.
fn converters_card(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let preflight = &cx.runs.preflight;
    let sets = &preflight.converters;
    let checked = preflight.report.is_some();
    let count: usize = sets.iter().map(|set| set.converters.len()).sum();
    let status = checked.then(|| format!("{count} converter{}", if count == 1 { "" } else { "s" }));
    let mut route = None;
    card(
        ui,
        "Inserted converters",
        status.as_deref().map(|status| (status, Tone::Neutral)),
        |ui| {
            if count == 0 {
                let _ = card_empty(
                    ui,
                    if checked {
                        "The checked decks insert no converters."
                    } else {
                        "Check the run to list inserted converters."
                    },
                );
                return;
            }
            ledger_head_explained(ui, &REPORT, &REPORT_LABELS, &REPORT_HOVERS);
            let several = sets.len() > 1;
            for set in sets {
                if several {
                    ledger_group(ui, &set.tasks.join(", "));
                }
                for converter in &set.converters {
                    let cells: Vec<(&str, Tone)> = converter
                        .cells
                        .iter()
                        .map(|cell| (cell.as_str(), Tone::Neutral))
                        .collect();
                    let response = ledger_row(ui, &REPORT, &cells, false, RowPress::Taken)
                        .on_hover_ui(|ui| converter_detail(ui, converter));
                    if response.clicked() {
                        route = Some(converter.net.clone());
                    }
                }
            }
        },
    );
    if let Some(route) = route {
        cx.model
            .diagnostics
            .request_route(DiagnosticRoute::Repair(route));
    }
}

/// A converter's hover: where its rule is written, and every value with
/// where it came from.
fn converter_detail(ui: &mut Ui, converter: &CheckedConverter) {
    if let Some(location) = &converter.location {
        ui.label(cell_text(ui, &format!("statement at {location}"), false));
    }
    for value in &converter.values {
        ui.label(cell_text(ui, value, true));
    }
}

/// The blocks the design and the plan's bound model libraries declare, read
/// once per source revision and binding.
fn declared_blocks(
    ui: &Ui,
    cx: &Cx<'_, SimulateFeature>,
) -> std::sync::Arc<Vec<DeclaredConnectRules>> {
    let sources = &cx.model.workspace.content.project_sources;
    let bindings = &cx.model.sim_setup.model_bindings;
    let id = sources
        .iter_bundles()
        .fold(egui::Id::new("connect-rules-declared"), |id, bundle| {
            id.with((bundle.id(), bundle.revision().get()))
        });
    let id = bindings.iter().fold(id, |id, binding| {
        id.with((&binding.library_name, binding.source_digest))
    });
    if let Some(blocks) = ui.data(|data| data.get_temp(id)) {
        return blocks;
    }
    let blocks = std::sync::Arc::new(connect_rules::declared_blocks(
        sources,
        &cx.model.model_library_manager,
        bindings,
    ));
    ui.data_mut(|data| data.insert_temp(id, std::sync::Arc::clone(&blocks)));
    blocks
}

/// The run's own block's name: the stored block's, else the name a new one
/// takes.
fn run_block_name(cx: &Cx<'_, SimulateFeature>) -> String {
    cx.model
        .sim_setup
        .session
        .options_draft
        .connect_rule_block
        .as_ref()
        .map_or_else(
            || RunConnectRules::DEFAULT_NAME.to_owned(),
            |block| block.name.clone(),
        )
}

fn selection_card(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    declared: &[DeclaredConnectRules],
) {
    card(ui, "Connect rule set", None, |ui| {
        card_body(ui, |ui| {
            let run_block = run_block_name(cx);
            let choices = rule_choices(
                &cx.model.sim_setup.session.options_draft.connect_rules,
                &run_block,
                declared,
            );
            let mut output = None;
            field_pair(
                ui,
                (
                    SimulationOptions::CONNECT_RULES_LABEL,
                    &mut |ui: &mut Ui, width: f32| {
                        // Typed as well as picked, so a block the design
                        // declares in a source that does not compile alone is
                        // entered as written.
                        output = Some(editable_select_mono(
                            ui,
                            "simulation.connect_rules.selection",
                            SimulationOptions::CONNECT_RULES_LABEL,
                            &mut cx.model.sim_setup.session.options_draft.connect_rules,
                            DESIGN_DEFAULT_RULES,
                            &choices,
                            width,
                        ));
                    },
                ),
                None,
            );
            let Some(output) = output else {
                return;
            };
            let field = output.field.on_hover_text(RULES_HOVER);
            if let Some(choice) = output.picked.and_then(|index| choices.get(index)) {
                let choice = if choice == DESIGN_DEFAULT_RULES {
                    String::new()
                } else {
                    choice.clone()
                };
                select_rules(cx, Some(choice));
            } else if released(&field) {
                select_rules(cx, None);
            }
        });
    });
}

/// Select `choice` (or the typed name, when `None`), creating the run's
/// block from the default set the first time it is selected.
fn select_rules(cx: &mut Cx<'_, SimulateFeature>, choice: Option<String>) {
    let draft = &mut cx.model.sim_setup.session.options_draft;
    if let Some(choice) = choice {
        draft.connect_rules = choice;
    }
    let selected = draft.connect_rules.trim().to_owned();
    if selected == RunConnectRules::DEFAULT_NAME && draft.connect_rule_block.is_none() {
        draft.connect_rule_block = Some(connect_rules::started_from(DEFAULT_BASE));
    }
    commit_draft(cx);
}

/// What the CONNECTRULES list offers: the design default, the run's own
/// block, every shipped set, the blocks the design declares, and the current
/// value when it is none of those.
fn rule_choices(current: &str, run_block: &str, declared: &[DeclaredConnectRules]) -> Vec<String> {
    let mut choices = vec![DESIGN_DEFAULT_RULES.to_owned(), run_block.to_owned()];
    choices.extend(connect_rules::shipped_set_names().iter().cloned());
    for block in declared {
        if !choices.contains(&block.name) {
            choices.push(block.name.clone());
        }
    }
    let current = current.trim();
    if !current.is_empty() && !choices.iter().any(|choice| choice == current) {
        choices.push(current.to_owned());
    }
    choices
}

/// What the selection names, as the statement grid shows it.
enum Selected<'a> {
    /// The run's own block: edited here.
    Run(RunConnectRules),
    /// A shipped set: shown as RSpice writes it.
    Shipped(String, Vec<RunConnectStatement>),
    /// Blocks the design declares: the one named, or every one for the
    /// design default.
    Declared(Vec<&'a DeclaredConnectRules>),
    /// A name nothing on this page knows.
    Unknown(String),
}

fn selected<'a>(
    cx: &Cx<'_, SimulateFeature>,
    declared: &'a [DeclaredConnectRules],
) -> Selected<'a> {
    let options = &cx.model.sim_setup.options;
    let name = options.connect_rules.as_deref().unwrap_or_default();
    if let Some(block) = &options.connect_rule_block
        && block.name == name
    {
        return Selected::Run(block.clone());
    }
    if name.is_empty() {
        return Selected::Declared(declared.iter().collect());
    }
    if let Some(statements) = connect_rules::shipped_statements(name) {
        return Selected::Shipped(name.to_owned(), statements);
    }
    let named: Vec<_> = declared.iter().filter(|block| block.name == name).collect();
    if named.is_empty() {
        Selected::Unknown(name.to_owned())
    } else {
        Selected::Declared(named)
    }
}

fn statements_card(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    declared: &[DeclaredConnectRules],
) {
    let selection = selected(cx, declared);
    let title = match &selection {
        Selected::Run(block) => format!("{} \u{b7} this run's block", block.name),
        Selected::Shipped(name, _) => format!("{name} \u{b7} shipped rule set"),
        Selected::Declared(blocks) if blocks.len() == 1 => {
            format!("{} \u{b7} {}", blocks[0].name, declared_place(blocks[0]))
        }
        Selected::Declared(_) => "Declared blocks".to_owned(),
        Selected::Unknown(name) => format!("{name} \u{b7} not declared in the design's sources"),
    };
    let mut rebase = None;
    let mut copy = false;
    super::page_kit::card_with_head(
        ui,
        |ui| {
            card_head_row(ui, &title, None, |ui| match &selection {
                Selected::Run(block) => {
                    let sets = connect_rules::shipped_set_names();
                    let base = block.base.as_deref().unwrap_or(DEFAULT_BASE);
                    let output = select_with_response(
                        ui,
                        "simulation.connect_rules.base",
                        "Start from",
                        &format!("from {base}"),
                        sets,
                        170.0,
                    );
                    let _ = output.response.on_hover_text(
                        "The set the block starts from. A cell left empty takes the set's value, \
                         shown faint; clearing a typed value takes it back.",
                    );
                    rebase = output.picked.and_then(|index| sets.get(index).cloned());
                }
                Selected::Shipped(..) => {
                    copy = Button::new("Edit as this run's block")
                        .show(ui)
                        .on_hover_text("Copy these values into a block this run owns.")
                        .clicked();
                }
                _ => {}
            });
        },
        |ui| {
            let rows = match &selection {
                Selected::Run(_) => true,
                Selected::Shipped(_, statements) => !statements.is_empty(),
                Selected::Declared(blocks) => {
                    blocks.iter().any(|block| !block.statements.is_empty())
                }
                Selected::Unknown(_) => false,
            };
            if rows {
                ledger_head_explained(ui, &GRID, &grid_labels(), &grid_hovers());
            }
            match &selection {
                Selected::Run(block) => run_rows(ui, cx, block),
                Selected::Shipped(_, statements) => {
                    for statement in statements {
                        let _ = read_only_row(ui, statement, RowPress::Ignored);
                    }
                }
                Selected::Declared(blocks) => declared_rows(ui, cx, blocks),
                Selected::Unknown(_) => {}
            }
            if !rows {
                empty_statements(ui, &selection);
            }
        },
    );
    if let Some(base) = rebase {
        let draft = &mut cx.model.sim_setup.session.options_draft;
        if let Some(block) = &draft.connect_rule_block {
            draft.connect_rule_block = Some(connect_rules::rebased(block, &base));
            commit_draft(cx);
        }
    }
    if copy && let Selected::Shipped(name, _) = &selection {
        let draft = &mut cx.model.sim_setup.session.options_draft;
        let block = match &draft.connect_rule_block {
            Some(block) => connect_rules::rebased(block, name),
            None => connect_rules::started_from(name),
        };
        draft.connect_rules.clone_from(&block.name);
        draft.connect_rule_block = Some(block);
        commit_draft(cx);
    }
}

/// Why the grid has no rows, in one line, with the rest on hover.
fn empty_statements(ui: &mut Ui, selection: &Selected<'_>) {
    let (line, hover) = match selection {
        Selected::Unknown(_) => (
            "The engine looks for it in every source the run compiles.",
            "If none declares it, the run is refused, naming the blocks it found.",
        ),
        Selected::Shipped(..) => ("The set states no converter.", ""),
        _ => (
            "Neither the design nor a bound library declares a connectrules block.",
            "An HDL port then converts with the rspice_builtin set and an XSPICE port with \
             ngspice's bridges. Select run_rules or a shipped set to state the rules here.",
        ),
    };
    let response = card_empty(ui, line);
    if !hover.is_empty() {
        let _ = response.on_hover_text(hover);
    }
}

fn grid_labels() -> Vec<&'static str> {
    let mut labels = vec!["Module", "Converts"];
    labels.extend(ConnectRuleColumn::ALL.iter().map(|column| column.label()));
    labels
}

/// What each grid column takes, on its header's hover.
fn grid_hovers() -> Vec<&'static str> {
    let mut hovers = vec![
        "The connect module the statement selects.",
        "What it converts: A is the analog side, D a logic port, R a real one.",
    ];
    hovers.extend(ConnectRuleColumn::ALL.iter().map(|column| match column {
        ConnectRuleColumn::Supply => {
            "Volts; a net the converter tracks while it runs (vdd); or deck for the deck's own."
        }
        ConnectRuleColumn::ThresholdHigh | ConnectRuleColumn::ThresholdLow => {
            "Volts, or a share of the supply written with %, which follows a tracked supply."
        }
        ConnectRuleColumn::DriveHigh | ConnectRuleColumn::DriveLow => {
            "The output resistance driving the level, in ohms."
        }
        ConnectRuleColumn::Rise | ConnectRuleColumn::Fall => "The output's transition time.",
    }));
    hovers
}

/// The cells of one statement, as text.
fn statement_cells(statement: &RunConnectStatement) -> Vec<(String, bool)> {
    let mut cells = vec![
        (statement.module.clone(), false),
        (
            connect_rules::conversion(&statement.module).to_owned(),
            false,
        ),
    ];
    for column in ConnectRuleColumn::ALL {
        cells.push(if column.applies(&statement.module) {
            connect_rules::cell(statement, column).map_or_else(
                || (default_text(column).to_owned(), true),
                |cell| (cell.text, false),
            )
        } else {
            ("\u{2014}".to_owned(), true)
        });
    }
    cells
}

/// What a cell that states nothing takes.
fn default_text(column: ConnectRuleColumn) -> &'static str {
    match column {
        ConnectRuleColumn::Supply => "deck",
        _ => "default",
    }
}

fn read_only_row(ui: &mut Ui, statement: &RunConnectStatement, press: RowPress) -> egui::Response {
    let cells = statement_cells(statement);
    let toned: Vec<(&str, Tone)> = cells
        .iter()
        .map(|(text, _)| (text.as_str(), Tone::Neutral))
        .collect();
    ledger_row(ui, &GRID, &toned, false, press)
}

/// Where a declared block is written, as its caption and the statement
/// grid's title name it: `file:line`, after the library's name for a bound
/// one.
fn declared_place(block: &DeclaredConnectRules) -> String {
    match &block.origin {
        DeclaredIn::Project(_) => format!("{}:{}", block.logical_path, block.line),
        DeclaredIn::Library(library) => {
            format!("{library} \u{203a} {}:{}", block.logical_path, block.line)
        }
    }
}

/// The declared blocks' statements: a project's own opening its statement in
/// the source editor, a bound library's read here only.
fn declared_rows(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>, blocks: &[&DeclaredConnectRules]) {
    let several = blocks.len() > 1;
    for block in blocks {
        if several {
            ledger_group(
                ui,
                &format!("{} \u{b7} {}", block.name, declared_place(block)),
            );
        }
        for (statement, line) in &block.statements {
            let DeclaredIn::Project(bundle) = block.origin else {
                let _ = read_only_row(ui, statement, RowPress::Ignored);
                continue;
            };
            if read_only_row(ui, statement, RowPress::Taken)
                .on_hover_text(format!("Open {}:{line}", block.logical_path))
                .clicked()
            {
                cx.model
                    .diagnostics
                    .request_route(DiagnosticRoute::CodeDocument {
                        bundle,
                        logical_path: block.logical_path.as_str().into(),
                        line: Some(*line),
                        column: None,
                    });
            }
        }
    }
}

/// The run's block, one editable row per shipped module: the logic
/// converters always, the real ones while the block selects them.
fn run_rows(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>, block: &RunConnectRules) {
    let mut edit: Option<(String, Edit)> = None;
    for (index, module) in RUN_CONNECT_MODULES.iter().enumerate() {
        if index == 0 {
            ledger_group(ui, "electrical \u{2194} logic");
        } else if index == 3 {
            ledger_group(ui, "electrical \u{2194} real");
        }
        let statement = block.statement(module);
        let (_, cells) = ledger_row_cells(ui, &GRID);
        let pad = vec2(CARD_PAD_X * 0.8, 0.0);
        {
            let mut cell = cell_ui(ui, cells[0].shrink2(pad));
            if index < 3 {
                cell.label(cell_text(&cell, module, false));
            } else {
                let mut on = statement.is_some();
                if switch_cell(&mut cell, &format!("Select {module}"), &mut on).changed() {
                    edit = Some(((*module).to_owned(), Edit::Select(on)));
                }
                cell.add_space(6.0);
                cell.label(cell_text(&cell, module, false));
            }
        }
        let mut converts = cell_ui(ui, cells[1].shrink2(pad));
        converts.label(cell_text(
            &converts,
            connect_rules::conversion(module),
            true,
        ));
        for (offset, column) in ConnectRuleColumn::ALL.into_iter().enumerate() {
            let rect = cells[offset + 2].shrink2(vec2(3.0, 0.0));
            let mut cell = cell_ui(ui, rect);
            let Some(statement) = statement.filter(|_| column.applies(module)) else {
                cell.label(cell_text(&cell, "\u{2014}", true));
                continue;
            };
            if let Some(text) = edit_cell(&mut cell, statement, column, rect.width()) {
                edit = Some(((*module).to_owned(), Edit::Cell(column, text)));
            }
        }
    }
    let Some((module, edit)) = edit else {
        return;
    };
    let draft = &mut cx.model.sim_setup.session.options_draft;
    let Some(block) = draft.connect_rule_block.as_mut() else {
        return;
    };
    let applied = match &edit {
        Edit::Select(on) => {
            connect_rules::set_selected(block, &module, *on);
            Ok(())
        }
        Edit::Cell(column, text) => connect_rules::set_cell(block, &module, *column, text),
    };
    match (applied, edit) {
        (Ok(()), _) => commit_draft(cx),
        (Err(error), edit) => {
            // Refused in place: the cell keeps what was typed, ringed, with
            // the reason on its hover, until it is typed again.
            if let Edit::Cell(column, text) = edit {
                ui.data_mut(|data| {
                    data.insert_temp(
                        cell_id(&module, column).with("refused"),
                        RefusedCell {
                            text,
                            reason: error.clone(),
                        },
                    );
                });
            }
            record_plan_refusal(cx, format!("Connect rules were not changed \u{b7} {error}"));
        }
    }
}

/// A cell's typed text the block refused, and why.
#[derive(Clone)]
struct RefusedCell {
    text: String,
    reason: String,
}

/// Where a cell keeps what is being typed into it, and what it refused.
fn cell_id(module: &str, column: ConnectRuleColumn) -> egui::Id {
    egui::Id::new(("connect-rule-cell", module, column.label()))
}

/// A cell's words in the grid's mono face: the row's own ink, or faint for
/// what the row only describes.
fn cell_text(ui: &Ui, text: &str, faint: bool) -> RichText {
    let t = Tokens::get(ui.ctx());
    RichText::new(text)
        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
        .color(if faint {
            t.color.text_faint
        } else {
            t.color.text_dim
        })
}

enum Edit {
    Select(bool),
    Cell(ConnectRuleColumn, String),
}

/// One editable cell: the run's own value as text, the set's as the faint
/// hint. Returns the typed text when the field is released.
fn edit_cell(
    ui: &mut Ui,
    statement: &RunConnectStatement,
    column: ConnectRuleColumn,
    width: f32,
) -> Option<String> {
    let id = cell_id(&statement.module, column);
    let shown = connect_rules::cell(statement, column);
    let (own, hint) = match &shown {
        Some(cell) if !cell.from_base => (cell.text.clone(), String::new()),
        Some(cell) => (String::new(), cell.text.clone()),
        None => (String::new(), default_text(column).to_owned()),
    };
    let refused = ui.data(|data| data.get_temp::<RefusedCell>(id.with("refused")));
    let mut text = ui
        .data(|data| data.get_temp::<String>(id))
        .or_else(|| refused.as_ref().map(|refused| refused.text.clone()))
        .unwrap_or(own);
    let hint = cell_text(ui, &hint, true).font(theme::field_mono());
    let mut response = text_field(
        ui,
        TextEdit::singleline(&mut text)
            .id(id.with("field"))
            .hint_text(hint)
            .font(theme::field_mono())
            .margin(egui::Margin::symmetric(5, 4)),
        width,
    );
    name_control(
        ui,
        &response,
        &format!("{} {}", statement.module, column.label()),
    );
    if let Some(refused) = &refused {
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
        response = response.on_hover_text(&refused.reason);
    }
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text.clone()));
    }
    if released(&response) {
        ui.data_mut(|data| {
            data.remove::<String>(id);
            data.remove::<RefusedCell>(id.with("refused"));
        });
        return Some(text);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{DESIGN_DEFAULT_RULES, rule_choices};

    /// The list opens on the design default and the run's block, offers
    /// every shipped set once, and keeps a name the design declares.
    #[test]
    fn the_list_offers_the_run_block_the_shipped_sets_and_a_declared_name() {
        let listed = rule_choices("", "run_rules", &[]);
        assert_eq!(listed[..2], [DESIGN_DEFAULT_RULES, "run_rules"]);
        assert_eq!(listed.len(), 2 + 15, "{listed:?}");
        assert_eq!(rule_choices("rspice_1v8_slow", "run_rules", &[]), listed);
        let declared = rule_choices(" gate_rules ", "run_rules", &[]);
        assert_eq!(declared.last().map(String::as_str), Some("gate_rules"));
    }
}

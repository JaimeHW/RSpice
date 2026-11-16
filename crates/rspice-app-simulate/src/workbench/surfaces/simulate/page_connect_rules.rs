//! Connect rules: what converts at the design's analog/digital boundaries.
//!
//! Verilog-AMS converts every boundary with the connect module a
//! `connectrules` block selects (LRM 2.4 section 7.7). The page states which
//! block the run selects (`.OPTIONS CONNECTRULES`), and then that block's
//! statements in one grid: the run's own block is edited there cell by cell,
//! while a shipped set and a block the design declares are shown as written,
//! a declared one opening its source. The run's block is stored with the
//! plan's options and written into the deck as a `.CONNECTRULES` card.

use egui::{RichText, TextEdit, Ui, vec2};

use crate::simulation::plan::connect_rules::{
    self, ConnectRuleColumn, DEFAULT_BASE, DeclaredConnectRules,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, mono_input, name_control, select, select_mono_with_response, text_field,
};
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, record_plan_refusal};
use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticRoute;
use rspice_simulation_contract::options::{
    RUN_CONNECT_MODULES, RunConnectRules, RunConnectStatement, SimulationOptions,
};

use super::page_kit::{
    CARD_PAD_X, RowPress, Tone, card, card_body, card_head_row, card_note, cell_ui, field_pair,
    ledger_group, ledger_head, ledger_row, ledger_row_cells, switch_cell,
};
use super::page_solver::{commit_draft, released};

/// The CONNECTRULES list's entry for an empty value.
const DESIGN_DEFAULT_RULES: &str = "(design default)";

/// The grid's columns: the module, what it converts, then
/// [`ConnectRuleColumn::ALL`].
const GRID: [f32; 9] = [0.11, 0.08, 0.12, 0.11, 0.11, 0.11, 0.11, 0.125, 0.125];

pub(super) fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let declared = declared_blocks(ui, cx);
    selection_card(ui, cx, &declared);
    ui.add_space(12.0);
    statements_card(ui, cx, &declared);
}

/// The design's own blocks, read from its sources once per source revision.
fn declared_blocks(
    ui: &Ui,
    cx: &Cx<'_, SimulateFeature>,
) -> std::sync::Arc<Vec<DeclaredConnectRules>> {
    let sources = &cx.model.workspace.content.project_sources;
    let id = sources
        .iter_bundles()
        .fold(egui::Id::new("connect-rules-declared"), |id, bundle| {
            id.with((bundle.id(), bundle.revision().get()))
        });
    if let Some(blocks) = ui.data(|data| data.get_temp(id)) {
        return blocks;
    }
    let blocks = std::sync::Arc::new(connect_rules::declared_blocks(sources));
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
            let mut typed = None;
            let mut picked = None;
            field_pair(
                ui,
                (
                    SimulationOptions::CONNECT_RULES_LABEL,
                    &mut |ui: &mut Ui, width: f32| {
                        // The name stays typed text, so a block the design
                        // declares in a source that does not compile alone is
                        // entered as written.
                        let gap = 6.0;
                        let text_width = ((width - gap) * 0.5).max(1.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = gap;
                            let rules = &mut cx.model.sim_setup.session.options_draft.connect_rules;
                            typed = Some(mono_input(
                                ui,
                                SimulationOptions::CONNECT_RULES_LABEL,
                                rules,
                                text_width,
                            ));
                            let shown = if rules.trim().is_empty() {
                                DESIGN_DEFAULT_RULES
                            } else {
                                rules.trim()
                            };
                            picked = select_mono_with_response(
                                ui,
                                "simulation.connect_rules.selection",
                                "Connect rule set",
                                shown,
                                &choices,
                                (width - gap - text_width).max(1.0),
                            )
                            .picked;
                        });
                    },
                ),
                None,
            );
            if let Some(response) = typed
                && released(&response)
            {
                select_rules(cx, None);
            }
            if let Some(choice) = picked.and_then(|index| choices.get(index)) {
                let choice = if choice == DESIGN_DEFAULT_RULES {
                    String::new()
                } else {
                    choice.clone()
                };
                select_rules(cx, Some(choice));
            }
        });
        card_note(
            ui,
            "CONNECTRULES names the connectrules block that converts at every analog/digital \
             boundary: the design default takes the design's only block, run_rules is this run's \
             own block, an rspice_ set is RSpice's, and any other name is a block the design \
             declares.",
        );
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
            format!(
                "{} \u{b7} {}:{}",
                blocks[0].name, blocks[0].logical_path, blocks[0].line
            )
        }
        Selected::Declared(_) => "Blocks the design declares".to_owned(),
        Selected::Unknown(name) => format!("{name} \u{b7} not declared in the project's sources"),
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
                    rebase = select(
                        ui,
                        "simulation.connect_rules.base",
                        "Start from",
                        &format!("from {base}"),
                        sets,
                        170.0,
                    )
                    .and_then(|index| sets.get(index).cloned());
                }
                Selected::Shipped(..) => {
                    copy = Button::new("Edit as this run's block").show(ui).clicked();
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
                ledger_head(ui, &GRID, &grid_labels());
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
            card_note(ui, statements_note(&selection));
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

fn statements_note(selection: &Selected<'_>) -> &'static str {
    match selection {
        Selected::Run(_) => {
            "Faint values come from the set the block starts from; a value typed here replaces \
             one, and clearing it takes the set's back. A supply is volts, a net the converter \
             tracks while it runs (vdd), or deck for the deck's own. A threshold is volts, or a \
             share of the supply written with % that follows a tracked supply."
        }
        Selected::Shipped(..) => {
            "RSpice's own values. Edit as this run's block copies them into a block this run \
             owns, where they can be changed."
        }
        Selected::Declared(blocks) if blocks.is_empty() => {
            "No Verilog-AMS source in the project declares a connectrules block, so an HDL \
             port converts with the rspice_builtin set and an XSPICE port with ngspice's \
             bridges. Select run_rules or a shipped set to state the rules here."
        }
        Selected::Declared(_) => {
            "Declared in the design's Verilog-AMS sources and edited there: select a row to open \
             its statement."
        }
        Selected::Unknown(_) => {
            "No source this page can read declares this block; the engine looks for it in every \
             source the run compiles, and names the blocks it finds if there is none."
        }
    }
}

fn grid_labels() -> Vec<&'static str> {
    let mut labels = vec!["Module", "Converts"];
    labels.extend(ConnectRuleColumn::ALL.iter().map(|column| column.label()));
    labels
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

fn declared_rows(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>, blocks: &[&DeclaredConnectRules]) {
    let several = blocks.len() > 1;
    for block in blocks {
        if several {
            ledger_group(
                ui,
                &format!(
                    "{} \u{b7} {}:{}",
                    block.name, block.logical_path, block.line
                ),
            );
        }
        for (statement, line) in &block.statements {
            if read_only_row(ui, statement, RowPress::Taken).clicked() {
                cx.model
                    .diagnostics
                    .request_route(DiagnosticRoute::CodeDocument {
                        bundle: block.bundle,
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
    let applied = match edit {
        Edit::Select(on) => {
            connect_rules::set_selected(block, &module, on);
            Ok(())
        }
        Edit::Cell(column, text) => connect_rules::set_cell(block, &module, column, &text),
    };
    match applied {
        Ok(()) => commit_draft(cx),
        Err(error) => {
            record_plan_refusal(cx, format!("Connect rules were not changed \u{b7} {error}"))
        }
    }
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
    let id = ui.make_persistent_id(("connect-rule-cell", &statement.module, column.label()));
    let shown = connect_rules::cell(statement, column);
    let (own, hint) = match &shown {
        Some(cell) if !cell.from_base => (cell.text.clone(), String::new()),
        Some(cell) => (String::new(), cell.text.clone()),
        None => (String::new(), default_text(column).to_owned()),
    };
    let mut text = ui.data(|data| data.get_temp::<String>(id)).unwrap_or(own);
    let hint = cell_text(ui, &hint, true).font(theme::field_mono());
    let response = text_field(
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
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text.clone()));
    }
    if released(&response) {
        ui.data_mut(|data| data.remove::<String>(id));
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

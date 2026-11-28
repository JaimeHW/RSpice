//! Importing an LTspice (`.asc`) or QSPICE (`.qsch`) schematic into the open
//! project, or into a new one named after the file when none is open.
//!
//! The drawing becomes a cell of the active library with a schematic view.
//! A symbol RSpice draws no part for becomes a cell of its own, and a block's
//! schematic becomes its cell's schematic view. The directives become project
//! content: the analyses, options and temperature a simulation plan named
//! after the cell, constant parameters its design variables, the drawing's
//! model text and the source program's own cards for the models its parts
//! name a model library of the project, and every file the drawing includes
//! a model library bound in place. Any other card the netlist can carry is a
//! SPICE directive note where its block stood, or, when the block was not
//! drawn, the design's Added lines. A card with no project place stays on the
//! sheet as text. Every notice is written to the log.

mod plan;
mod sheet;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use rspice_design::foreign_schematic::{
    ForeignCard, ForeignCardRole, ForeignLibrary, ForeignNoticeSeverity, ForeignSymbolCell,
    ForeignTool, InstalledForeignLibrary, bind_instances_to_sources, decode_foreign_text,
    foreign_directive_cards, import_foreign_schematic, parse_asc, parse_qsch,
};
use rspice_design::schematic::document::SchematicDocument;
use rspice_simulation_contract::design_variable::{
    DesignVariable, DesignVariableOverridePolicy, DesignVariableScope,
    DesignVariableSweepEligibility,
};
use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;
use rspice_simulation_contract::run_set::{RunSetDimensionKind, RunSetState, value_id};

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::workflows::file_workflow::SchematicLoadOrigin;
use rspice_app_services::workbench::slots::platform::foreign_import_refusal_in;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
use rspice_app_state::state::model_library::SimulationPlanModelBinding;
use rspice_app_state::state::{Cell, CellViewRef, SchematicState, View, ViewType};

use plan::{ForeignPlan, plan_from_cards};
use sheet::{OwnSheet, drawing_centre, place_on_sheet};

/// The file-picker filter and the gesture's name, per source program.
const fn picker(tool: ForeignTool) -> (&'static str, &'static str, &'static str) {
    match tool {
        ForeignTool::Ltspice => ("LTspice Schematic", "asc", "Import LTspice Schematic"),
        ForeignTool::Qspice => ("QSPICE Schematic", "qsch", "Import QSPICE Schematic"),
    }
}

/// Asks for a schematic file and imports it.
pub(crate) fn import_foreign_schematic_with_dialog(app: &mut RSpiceApp, tool: ForeignTool) {
    let (filter, extension, title) = picker(tool);
    let Some(path) = rfd::FileDialog::new()
        .add_filter(filter, &[extension])
        .add_filter("All Files", &["*"])
        .set_title(title)
        .pick_file()
    else {
        return;
    };
    if let Err(error) = import_foreign_schematic_file(app, &path, tool, &mut ask_natively) {
        app.state.record(
            LogSeverity::Error,
            LogSource::Studio,
            format!("{title} failed: {error}"),
        );
        app.state.prompt(error);
    }
}

/// Asks the plan question in a native dialog. Closing it keeps the plan the
/// project runs, which changes nothing the reader has not agreed to.
fn ask_natively(question: &PlanQuestion) -> PlanAnswer {
    let replace = "Replace";
    let answer = rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_title(&question.title)
        .set_description(&question.description)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            replace.to_owned(),
            question.keep.clone(),
        ))
        .show();
    match answer {
        rfd::MessageDialogResult::Ok => PlanAnswer::Replace,
        rfd::MessageDialogResult::Custom(label) if label == replace => PlanAnswer::Replace,
        _ => PlanAnswer::Keep,
    }
}

/// What an import asks before the drawing's analyses replace the plan the
/// project runs: what would be replaced, and the two ways to answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlanQuestion {
    pub(crate) title: String,
    pub(crate) description: String,
    /// The label of the answer that keeps the active plan.
    pub(crate) keep: String,
}

/// The answer to a [`PlanQuestion`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlanAnswer {
    /// The drawing's analyses become the active plan.
    Replace,
    /// The active plan stays; the drawing's analyses are a plan beside it.
    Keep,
}

/// The reason the import cannot run now, if any.
pub(crate) fn foreign_import_refusal(state: &AppState) -> Option<&'static str> {
    foreign_import_refusal_in(&state.model, &state.platform)
}

/// A drawing and everything under it, imported.
struct ImportedTree {
    document: SchematicDocument,
    /// Symbol cells, with the schematic of each block.
    cells: Vec<(ForeignSymbolCell, Option<SchematicDocument>)>,
    cards: Vec<ForeignCard>,
    model_cards: Vec<String>,
    notices: Vec<(ForeignNoticeSeverity, String)>,
    /// The top drawing's directive blocks: the note each is drawn as, where
    /// the block stands, and the text of each of its cards.
    directive_blocks: Vec<(u64, rspice_design_model::Point, Vec<String>)>,
}

/// Imports the schematic at `path` into the open project, or into a new one
/// named after the file when none is open, and opens it,
/// saying on the status line what came across and how much did not; when
/// something did not, the Problems page opens on a row for each. Returns
/// that sentence.
///
/// When the project goes on simulating another design, `ask` is asked
/// whether the drawing's analyses replace the active plan.
pub(crate) fn import_foreign_schematic_file(
    app: &mut RSpiceApp,
    path: &Path,
    tool: ForeignTool,
    ask: &mut dyn FnMut(&PlanQuestion) -> PlanAnswer,
) -> Result<String, String> {
    if let Some(refusal) = foreign_import_refusal(&app.state) {
        return Err(format!("The schematic cannot be imported: {refusal}."));
    }
    let source = path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    if !app.state.platform.project_lifecycle.is_open() {
        // As a netlist opened with no project open is: a project named after
        // the file, made only once the file reads as a drawing, so a file
        // that does not leaves the landing as it was.
        read_drawing(tool, path)?;
        app.state
            .create_imported_schematic_project(&source)
            .map_err(|refusal| format!("The schematic cannot be imported: {refusal}"))?;
    }
    let library = app
        .state
        .model
        .workspace
        .content
        .active_view
        .library
        .clone();
    let access = InstalledForeignLibrary::for_drawing(tool, path);
    let mut seen = HashSet::from([path.to_path_buf()]);
    let mut tree = import_tree(tool, path, &access, &library, &mut seen, true)?;
    rename_taken_cells(&app.state, &library, &mut tree);
    let mut plan = plan_from_cards(
        std::mem::take(&mut tree.cards),
        tool,
        &tree.document,
        |draft| {
            app.state
                .model
                .sim_setup
                .analysis_draft_validation_error(draft)
        },
    );
    plan.options.source_simulator = match tool {
        ForeignTool::Ltspice => rspice_simulation_contract::options::SourceSimulator::Ltspice,
        ForeignTool::Qspice => rspice_simulation_contract::options::SourceSimulator::Qspice,
    };
    let mut notices = std::mem::take(&mut tree.notices);
    notices.extend(
        plan.notices
            .drain(..)
            .map(|notice| (ForeignNoticeSeverity::Notice, notice)),
    );

    let mut includes = resolve_includes(&access, &mut plan, &mut notices);
    bind_model_files(&mut tree, &library, &mut includes);
    draw_directive_notes(&mut tree, &mut plan, &mut notices);
    note_cards_with_no_place(&plan, &mut notices);

    let mut own_sheets = add_symbol_cells(&mut app.state, &library, &mut tree.cells)?;
    let top_sheet = place_on_sheet(&app.state.model, &mut tree.document);
    let parts = tree.document.components.len();
    let centre = drawing_centre(&app.state.model, &tree.document);
    let reference = app.state.import_loaded_schematic_as_cell_view(
        SchematicState::from_document(std::mem::take(&mut tree.document)),
        SchematicLoadOrigin::PersistentPath(path),
    )?;
    own_sheets.extend(top_sheet.map(|sheet| (reference.clone(), sheet)));
    add_added_lines(&mut app.state, &reference, &plan, &mut notices);
    give_own_sheets(&mut app.state, own_sheets, &mut notices);
    let adopted = adopt_as_top_cell(&mut app.state, &reference, &mut notices);
    let model_bindings = add_model_libraries(
        &mut app.state,
        &reference.cell,
        model_cards(&plan, &tree.model_cards),
        &includes,
        &mut notices,
    );
    let asking = (!adopted).then(|| Asking {
        title: format!("Import {} Schematic", tool.label()),
        source: &source,
        ask,
    });
    let plan = commit_plan(
        app,
        &reference.cell,
        &plan,
        model_bindings,
        asking,
        &mut notices,
    );

    let refusals = notices
        .iter()
        .filter(|(severity, _)| *severity == ForeignNoticeSeverity::Refusal)
        .map(|(_, message)| message.clone())
        .collect::<Vec<_>>();
    for (severity, message) in &notices {
        let severity = match severity {
            ForeignNoticeSeverity::Notice => LogSeverity::Info,
            ForeignNoticeSeverity::Refusal => LogSeverity::Warning,
        };
        app.state
            .record(severity, LogSource::Studio, message.clone());
    }
    let facts = app.state.slot_facts();
    app.state
        .model
        .publish_import_refusals(&reference, &source, centre, &refusals, facts);
    let summary = import_summary(
        &format!("{} schematic {source}", tool.label()),
        &reference,
        parts,
        plan.as_deref(),
        refusals.len(),
    );
    app.state
        .record(LogSeverity::Info, LogSource::Studio, summary.clone());
    app.state.prompt(summary.clone());
    if !refusals.is_empty() {
        app.state.workbench.show_problem_findings();
    }
    app.invalidate_simulation_preflight();
    Ok(summary)
}

/// What an import says it did: the drawing and the plan that came across,
/// and how much did not, which the Problems page lists.
fn import_summary(
    source: &str,
    reference: &CellViewRef,
    parts: usize,
    plan: Option<&str>,
    refusals: usize,
) -> String {
    use rspice_ui_kit::accessibility::counted;

    let mut summary = format!(
        "Imported {source} as {}: {}",
        reference.display_path(),
        counted(parts, "part", "parts")
    );
    if let Some(plan) = plan {
        summary.push_str("; ");
        summary.push_str(plan);
    }
    summary.push('.');
    if refusals > 0 {
        summary.push_str(&format!(
            " {} did not come across; Problems lists {}.",
            counted(refusals, "item", "items"),
            if refusals == 1 { "it" } else { "them" }
        ));
    }
    summary
}

fn read_drawing(
    tool: ForeignTool,
    path: &Path,
) -> Result<rspice_design::foreign_schematic::ForeignSchematic, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("{} cannot be read: {error}", path.display()))?;
    match tool {
        ForeignTool::Ltspice => parse_asc(&decode_foreign_text(&bytes)),
        ForeignTool::Qspice => parse_qsch(&bytes),
    }
    .map_err(|error| {
        format!(
            "{} is not a {} schematic: {error}",
            path.display(),
            tool.label()
        )
    })
}

/// Imports a drawing and, depth first, the schematic of every block it
/// places. A block's own model text, parameters and includes come with it;
/// its analyses do not, as the source program ignores them too.
fn import_tree(
    tool: ForeignTool,
    path: &Path,
    access: &InstalledForeignLibrary,
    library: &str,
    seen: &mut HashSet<PathBuf>,
    top: bool,
) -> Result<ImportedTree, String> {
    let drawing = read_drawing(tool, path)?;
    let imported = import_foreign_schematic(&drawing, library, access);
    let mut cards = foreign_directive_cards(&imported.directives);
    if !top {
        cards.retain(|card| {
            matches!(
                card.role,
                ForeignCardRole::ModelText | ForeignCardRole::Parameter | ForeignCardRole::Include
            )
        });
    }
    let directive_blocks = if top {
        imported
            .directives
            .iter()
            .filter_map(|directive| {
                let cards = foreign_directive_cards(std::slice::from_ref(directive));
                directive.note.map(|note| {
                    (
                        note,
                        directive.position,
                        cards.into_iter().map(|card| card.text).collect(),
                    )
                })
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut tree = ImportedTree {
        document: imported.document,
        cells: Vec::new(),
        cards,
        directive_blocks,
        model_cards: imported.model_cards,
        notices: imported
            .notices
            .into_iter()
            .map(|notice| (notice.severity, notice.message))
            .collect(),
    };
    for cell in imported.cells {
        let schematic = match &cell.schematic {
            Some(block) if seen.insert(block.clone()) => {
                let child = import_tree(
                    tool,
                    block,
                    &access.for_another_drawing(block),
                    library,
                    seen,
                    false,
                )?;
                tree.cards.extend(child.cards);
                tree.model_cards.extend(child.model_cards);
                tree.notices.extend(child.notices);
                tree.cells.extend(child.cells);
                let mut document = child.document;
                cell.order_block_ports(&mut document);
                Some(document)
            }
            _ => None,
        };
        tree.cells.push((cell, schematic));
    }
    Ok(tree)
}

/// Gives each imported cell a name its library does not hold yet, and
/// points every placed instance of it at that name.
fn rename_taken_cells(state: &AppState, library: &str, tree: &mut ImportedTree) {
    let Some(existing) = state.model.library_manager.get_library(library) else {
        return;
    };
    let mut taken = existing
        .cells
        .values()
        .map(|cell| cell.name.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    let mut renamed = BTreeMap::new();
    let mut unique = Vec::new();
    for (cell, schematic) in std::mem::take(&mut tree.cells) {
        if unique
            .iter()
            .any(|(kept, _): &(ForeignSymbolCell, _)| kept.cell.eq_ignore_ascii_case(&cell.cell))
        {
            continue;
        }
        let mut name = cell.cell.clone();
        let mut suffix = 2;
        while !taken.insert(name.to_ascii_lowercase()) {
            name = format!("{}_{suffix}", cell.cell);
            suffix += 1;
        }
        if name != cell.cell {
            renamed.insert(cell.cell.clone(), name.clone());
        }
        let mut cell = cell;
        cell.cell = name;
        unique.push((cell, schematic));
    }
    let rebind = |document: &mut SchematicDocument| {
        for component in &mut document.components {
            if let Some(binding) = component.library_cell.as_mut()
                && binding.library == library
                && let Some(name) = renamed.get(&binding.cell)
            {
                binding.cell.clone_from(name);
            }
        }
    };
    rebind(&mut tree.document);
    for (_, schematic) in &mut unique {
        if let Some(document) = schematic.as_mut() {
            rebind(document);
        }
    }
    tree.cells = unique;
}

/// The model files the drawing's `.include` and `.lib` cards name, found
/// where the source program looks. A file not found, or one only its vendor
/// can read, leaves its card on the sheet.
/// Binds each cell whose model an included file defines to that file, so
/// the cell runs through a `spice` view over it, and adds every cell's file
/// to the files the project binds as model libraries. A file is named as
/// the model library that reads it names it, so the two agree.
fn bind_model_files(
    tree: &mut ImportedTree,
    library: &str,
    includes: &mut Vec<(PathBuf, Option<String>)>,
) {
    let files = includes
        .iter()
        .filter_map(|(path, section)| {
            let bytes = std::fs::read(path).ok()?;
            Some((
                path.clone(),
                section.clone(),
                String::from_utf8_lossy(&bytes).into_owned(),
            ))
        })
        .collect::<Vec<_>>();
    for (cell, _) in &mut tree.cells {
        cell.find_source(&files);
        if let Some(source) = cell.source.as_mut() {
            if let Ok(path) = std::fs::canonicalize(&source.path) {
                source.path = path;
            }
            includes.push((source.path.clone(), source.section.clone()));
        }
    }
    let cells = tree
        .cells
        .iter()
        .map(|(cell, _)| cell.clone())
        .collect::<Vec<_>>();
    bind_instances_to_sources(&mut tree.document, library, &cells);
    for document in tree
        .cells
        .iter_mut()
        .filter_map(|(_, document)| document.as_mut())
    {
        bind_instances_to_sources(document, library, &cells);
    }
}

fn resolve_includes(
    access: &InstalledForeignLibrary,
    plan: &mut ForeignPlan,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) -> Vec<(PathBuf, Option<String>)> {
    let mut found = Vec::new();
    for card in std::mem::take(&mut plan.includes) {
        let Some(file) = card.tokens.get(1) else {
            plan.kept.push((card, "it names no file".to_owned()));
            continue;
        };
        match access.model_file(file) {
            Some(model) if model.encrypted => {
                notices.push((
                    ForeignNoticeSeverity::Refusal,
                    format!(
                        "`{}` is encrypted, so only its vendor's simulator can read it; the models it holds are unresolved.",
                        model.path.display()
                    ),
                ));
                plan.kept
                    .push((card, "the file it names is encrypted".to_owned()));
            }
            Some(model) => {
                let section = (card.keyword() == ".lib")
                    .then(|| card.tokens.get(2).cloned())
                    .flatten();
                found.push((model.path, section));
            }
            None => {
                notices.push((
                    ForeignNoticeSeverity::Refusal,
                    format!(
                        "`{file}` was not found beside the schematic or in the source program's library; the models it holds are unresolved."
                    ),
                ));
                plan.kept
                    .push((card, "the file it names was not found".to_owned()));
            }
        }
    }
    found
}

/// Makes each directive block of the drawing a SPICE directive note of the
/// cards the netlist carries that no project place took, so editing the
/// note on the sheet edits what runs, as editing the block did in the source
/// program. Cards the plan or the model libraries took leave the sheet, which
/// would otherwise show a second, inert copy of them. Cards nothing runs stay
/// as plain text, below the directive's lines. A card no note can hold stays
/// in the design's Added lines.
fn draw_directive_notes(
    tree: &mut ImportedTree,
    plan: &mut ForeignPlan,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    use rspice_design::schematic::design_note::{DesignNoteKind, lettered_note_line_advance};

    for (id, position, texts) in &tree.directive_blocks {
        let of_block =
            |card: &ForeignCard| card.position == *position && texts.contains(&card.text);
        let (carried, others): (Vec<_>, Vec<_>) = std::mem::take(&mut plan.added_lines)
            .into_iter()
            .partition(|card| of_block(card));
        plan.added_lines = others;
        let kept = plan
            .kept
            .iter()
            .filter(|(card, _)| of_block(card))
            .map(|(card, _)| card.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let notes = &mut tree.document.design_notes;
        let Some(index) = notes.iter().position(|note| note.id == *id) else {
            plan.added_lines.extend(carried);
            continue;
        };
        if carried.is_empty() && kept.is_empty() {
            notes.remove(index);
            continue;
        }
        if carried.is_empty() {
            let note = &mut notes[index];
            let _ = note.update(DesignNoteKind::PlainText, kept);
            continue;
        }
        let directive = carried
            .iter()
            .map(|card| card.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let note = &mut notes[index];
        if let Err(error) = note.update(DesignNoteKind::SpiceDirective, directive.clone()) {
            notices.push((
                ForeignNoticeSeverity::Notice,
                format!("A directive block could not become a SPICE directive note: {error}."),
            ));
            plan.added_lines.extend(carried);
            continue;
        }
        for card in &carried {
            notices.push((
                ForeignNoticeSeverity::Notice,
                format!(
                    "`{}` runs from the SPICE directive on the sheet.",
                    card.text.lines().next().unwrap_or_default()
                ),
            ));
        }
        if kept.is_empty() {
            continue;
        }
        let mut plain = note.clone();
        let advance = lettered_note_line_advance(plain.text_size_percent);
        let lines = directive.lines().count() as f32;
        plain.pos.y += (advance * lines).round() as i32;
        plain.id = rspice_design::schematic::identity::SchematicIdentity::with_cursor(*id + 1)
            .allocate(&tree.document);
        if plain.update(DesignNoteKind::PlainText, kept).is_ok() {
            tree.document.design_notes.push(plain);
        }
    }
}

/// Says which cards found no project place: the sheet shows them as text,
/// but nothing runs them.
fn note_cards_with_no_place(
    plan: &ForeignPlan,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    for (card, reason) in &plan.kept {
        notices.push((
            ForeignNoticeSeverity::Notice,
            format!(
                "`{}` stays on the sheet as text: {reason}.",
                card.text.lines().next().unwrap_or_default()
            ),
        ));
    }
}

/// Makes the cards no plan field holds the imported design's Added lines,
/// which every run of the design appends to its generated netlist. The
/// design's name is new to the project, so lines an earlier cell of that
/// name left behind are replaced.
fn add_added_lines(
    state: &mut AppState,
    design: &CellViewRef,
    plan: &ForeignPlan,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    let text = plan
        .added_lines
        .iter()
        .map(|card| card.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    rspice_design::added_lines::set_added_lines(
        &mut state.model.workspace.content.added_lines,
        design,
        text,
    );
    for card in &plan.added_lines {
        notices.push((
            ForeignNoticeSeverity::Notice,
            format!(
                "`{}` runs from the Added lines of {}.",
                card.text.lines().next().unwrap_or_default(),
                design.display_path()
            ),
        ));
    }
}

/// Adds the symbol cells, and each block's schematic, to the library, each
/// schematic placed on its sheet. Returns the sheets the schematics need
/// in place of the project's.
fn add_symbol_cells(
    state: &mut AppState,
    library: &str,
    cells: &mut Vec<(ForeignSymbolCell, Option<SchematicDocument>)>,
) -> Result<Vec<(CellViewRef, OwnSheet)>, String> {
    let Some((first, _)) = cells.first() else {
        return Ok(Vec::new());
    };
    let first = first.cell.clone();
    let mut candidate = state.model.library_manager.clone();
    let mut edit = candidate
        .edit_library(library)
        .ok_or_else(|| format!("library `{library}` is not in this project"))?;
    let mut blocks = Vec::new();
    for (cell, schematic) in cells.drain(..) {
        let mut entry = Cell::new(&cell.cell);
        let mut symbol = View::new("symbol", ViewType::Symbol);
        cell.symbol.store_in_view(&mut symbol)?;
        entry.add_view(symbol);
        if let (Some(source), Some(model)) = (&cell.source, &cell.model) {
            entry.add_view(spice_view(&cell, source, model)?);
        }
        if let Some(document) = schematic {
            entry.add_view(View::new("schematic", ViewType::Schematic));
            blocks.push((CellViewRef::new(library, &cell.cell, "schematic"), document));
        }
        edit.add_cell(entry);
    }
    let mutation = state.model.preflight_project_library_mutation(
        rspice_app_state::state::ProjectLibraryMutation::CreateCell {
            library: library.to_owned(),
            cell: first,
        },
    )?;
    state.model.library_manager = candidate;
    let mut own_sheets = Vec::new();
    for (reference, mut document) in blocks {
        if let Some(sheet) = place_on_sheet(&state.model, &mut document) {
            own_sheets.push((reference.clone(), sheet));
        }
        let mut schematic = SchematicState::from_document(document);
        schematic.session.is_dirty = true;
        schematic.session.editor.snap_engine = state.design.schematic_snap.clone();
        schematic.reconcile_grid_pitch_runtime();
        state
            .model
            .workspace
            .insert_schematic_editor(reference.key(), schematic);
    }
    state.model.publish_project_library_mutation(mutation);
    Ok(own_sheets)
}

/// Gives each schematic too large for the project's sheet its own, as one
/// project transaction.
fn give_own_sheets(
    state: &mut AppState,
    sheets: Vec<(CellViewRef, OwnSheet)>,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    if sheets.is_empty() {
        return;
    }
    let before = (*state.model.workspace.content.design_management).clone();
    let mut candidate = before.clone();
    let mut given = Vec::new();
    for (reference, OwnSheet { format, objects }) in sheets {
        let key = reference.key();
        let label = format!(
            "{} {}",
            format.authored_size.label(),
            format.orientation.label()
        );
        let outcome = candidate
            .bootstrap_for_cell_view(&key, "Sheet 1", objects)
            .and_then(|sheet| {
                let catalog = candidate.sheet_catalog_mut(&key).ok_or_else(|| {
                    rspice_design_model::design_management::DesignManagementError::MissingReference {
                        domain: "sheet catalog",
                        identity: key.clone(),
                    }
                })?;
                // The new size keeps the sheet's own title fields.
                let (revision, format) = catalog.find(sheet).map_or((0, format.clone()), |held| {
                    (
                        held.revision(),
                        format.with_target_sheet_title_fields(held.page_format()),
                    )
                });
                catalog.update_sheet_page_format(sheet, revision, format)
            });
        match outcome {
            Ok(_) => given.push(format!(
                "{} is drawn on {label}: the project's sheet does not hold it.",
                reference.display_path()
            )),
            Err(error) => notices.push((
                ForeignNoticeSeverity::Notice,
                format!(
                    "{} keeps the project's sheet, which does not hold it: {error}.",
                    reference.display_path()
                ),
            )),
        }
    }
    match state
        .model
        .commit_sheet_catalog("import drawing sheets", before, candidate)
    {
        Ok(()) => notices.extend(
            given
                .into_iter()
                .map(|message| (ForeignNoticeSeverity::Notice, message)),
        ),
        Err(error) => notices.push((
            ForeignNoticeSeverity::Notice,
            format!(
                "The imported drawings keep the project's sheet, which does not hold them: {error}."
            ),
        )),
    }
}

/// The executable view of a cell whose model a file defines: the file, the
/// model it names, its ports in instance-line order and, for a device letter
/// other than `X`, its instance line.
fn spice_view(
    cell: &ForeignSymbolCell,
    source: &rspice_design::foreign_schematic::ForeignModelSource,
    model: &str,
) -> Result<View, String> {
    let mut view = View::new("spice", ViewType::Spice).with_path(source.path.clone());
    let ports = serde_json::to_string(
        &cell
            .ports
            .iter()
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>(),
    )
    .map_err(|error| format!("the ports of cell {} cannot be stated: {error}", cell.cell))?;
    view.metadata.insert("netlist.ports".to_owned(), ports);
    view.metadata
        .insert("netlist.model".to_owned(), model.to_owned());
    if let Some(template) = &cell.template {
        view.metadata
            .insert("netlist.template".to_owned(), template.clone());
    }
    if let Some(section) = &source.section {
        view.metadata
            .insert("netlist.section".to_owned(), section.clone());
    }
    Ok(view)
}

/// A project whose top cell draws nothing simulates the imported cell; any
/// other keeps its top cell, and the log says how to simulate the import.
/// Says whether the project now simulates the imported cell.
fn adopt_as_top_cell(
    state: &mut AppState,
    reference: &CellViewRef,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) -> bool {
    let content = &state.model.workspace.content;
    let top = content.simulation_root_reference();
    let top_is_empty = state
        .model
        .workspace
        .simulation_root_schematic(&content.active_view, &state.model.schematic)
        .is_none_or(|schematic| schematic.design.document().components.is_empty());
    if content.configuration_sets.active().is_none() && top_is_empty {
        let project = &mut state.model.workspace.content.project;
        project.root_library.clone_from(&reference.library);
        project.top_cell.clone_from(&reference.cell);
        true
    } else {
        notices.push((
            ForeignNoticeSeverity::Notice,
            format!(
                "The project simulates {}; make {} its top cell to simulate the imported schematic.",
                top.display_path(),
                reference.display_path()
            ),
        ));
        false
    }
}

/// The model-library text: the drawing's own model cards and parameter
/// expressions, then the source program's cards for the models its parts
/// name.
fn model_cards(plan: &ForeignPlan, library_cards: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    plan.model_text
        .iter()
        .chain(library_cards)
        .filter(|card| seen.insert(card.to_ascii_lowercase()))
        .cloned()
        .collect()
}

/// The model library whose text holds `card` word for word, on lines of its
/// own.
fn library_holding(state: &AppState, card: &str) -> Option<String> {
    let card = format!("\n{card}\n");
    state
        .model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| {
            library.source_contents.iter().any(|source| {
                format!("\n{}\n", String::from_utf8_lossy(&source.bytes)).contains(&card)
            })
        })
        .map(|library| library.name.clone())
}

/// Adds the drawing's model text as a model library of the project, binds
/// each included file as a model library read in place, and returns what a
/// plan binds to read them: a run reads only the libraries its plan names. A
/// card a library already holds is read from that library, because a second
/// provider of one name refuses every run in the project.
fn add_model_libraries(
    state: &mut AppState,
    cell: &str,
    cards: Vec<String>,
    includes: &[(PathBuf, Option<String>)],
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) -> Vec<SimulationPlanModelBinding> {
    use rspice_app_state::workbench::app_state::design_history::publish_model_library_candidate;

    let mut libraries = Vec::new();
    let mut text = String::new();
    for card in &cards {
        match library_holding(state, card) {
            Some(library) => libraries.push(library),
            None => {
                text.push_str(card);
                text.push('\n');
            }
        }
    }
    if !text.is_empty() {
        let file = format!("{cell}.lib");
        let mut candidate = state.model.model_library_manager.clone();
        let loaded = candidate
            .load_library_bundle_from_root(
                &file,
                &file,
                vec![(file.clone(), text.into_bytes())],
                None,
            )
            .and_then(|name| {
                publish_model_library_candidate(
                    &mut state.model,
                    candidate,
                    &name,
                    format!("import the model text of schematic {cell}"),
                )
                .map(|_| name)
            });
        match loaded {
            Ok(name) => {
                notices.push((
                    ForeignNoticeSeverity::Notice,
                    format!("The schematic's model text is the project's model library `{name}`."),
                ));
                libraries.push(name);
            }
            Err(error) => notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("The schematic's model text could not become a model library: {error}"),
            )),
        }
    }
    for (path, section) in includes {
        let mut candidate = state.model.model_library_manager.clone();
        let loaded = candidate
            .load_library_file(path, section.as_deref())
            .and_then(|name| {
                publish_model_library_candidate(
                    &mut state.model,
                    candidate,
                    &name,
                    format!("bind {} for schematic {cell}", path.display()),
                )
                .map(|_| name)
            });
        match loaded {
            Ok(name) => libraries.push(name),
            Err(error) if error.contains("already exists") => {
                libraries.extend(
                    state
                        .model
                        .model_library_manager
                        .libraries_sorted()
                        .into_iter()
                        .find(|library| library.root_path.as_deref() == Some(path.as_path()))
                        .map(|library| library.name.clone()),
                );
            }
            Err(error) => notices.push((
                ForeignNoticeSeverity::Refusal,
                format!(
                    "{} could not be bound as a model library: {error}",
                    path.display()
                ),
            )),
        }
    }
    let mut seen = HashSet::new();
    libraries
        .into_iter()
        .filter(|name| seen.insert(name.clone()))
        .filter_map(|name| {
            match state
                .model
                .model_library_manager
                .simulation_plan_binding(&name)
            {
                Ok(binding) => Some(binding),
                Err(error) => {
                    notices.push((
                        ForeignNoticeSeverity::Refusal,
                        format!("Model library `{name}` cannot join the simulation plan: {error}"),
                    ));
                    None
                }
            }
        })
        .collect()
}

/// A drawing with no analysis leaves the active plan's analyses as they are,
/// but a run of that plan still has to read the models the drawing's parts
/// name.
fn bind_to_active_plan(
    app: &mut RSpiceApp,
    bindings: Vec<SimulationPlanModelBinding>,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    let mut candidate = app.state.model.sim_setup.clone();
    let before = candidate.model_bindings.len();
    for binding in bindings {
        if !candidate
            .model_bindings
            .iter()
            .any(|existing| existing.library_name == binding.library_name)
        {
            candidate.model_bindings.push(binding);
        }
    }
    if candidate.model_bindings.len() == before {
        return;
    }
    let committed = app
        .state
        .model
        .model_library_manager
        .validate_simulation_plan_bindings(&candidate.model_bindings)
        .and_then(|()| {
            candidate
                .commit_active_plan_configuration_change(
                    "Attached the schematic's model libraries.",
                )
                .map_err(|error| error.to_string())
        });
    match committed {
        Ok(_) => app.state.model.sim_setup = candidate,
        Err(error) => notices.push((
            ForeignNoticeSeverity::Refusal,
            format!("The schematic's model libraries could not join the active plan: {error}"),
        )),
    }
}

/// Whom an import asks before its analyses replace the active plan, and
/// what it names in the question.
struct Asking<'a> {
    /// The gesture's name, the question's title.
    title: String,
    /// The file imported, as the question names it.
    source: &'a str,
    ask: &'a mut dyn FnMut(&PlanQuestion) -> PlanAnswer,
}

/// Activates a simulation plan named after the cell holding the drawing's
/// analyses, options, temperature, design variables, run-set axes and
/// measurements. A drawing with no analysis leaves the active plan as it is.
/// With `asking`, the active plan is replaced only when the answer says so;
/// otherwise the drawing's plan joins the project beside it. Returns what the
/// import's summary says of the plan, if it made one.
fn commit_plan(
    app: &mut RSpiceApp,
    cell: &str,
    plan: &ForeignPlan,
    model_bindings: Vec<SimulationPlanModelBinding>,
    asking: Option<Asking<'_>>,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) -> Option<String> {
    let mut analyses = rspice_simulation_contract::plan_model::SimulationPlan::empty();
    for (card, draft) in &plan.analyses {
        let id = rspice_vocabulary::product::AnalysisInstanceId::new();
        let position = analyses.instances().len();
        if let Err(error) = analyses.insert_draft_with_id(id, draft.clone(), true, position) {
            notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("`{}` could not join the plan: {error}", card.text),
            ));
        }
    }
    bind_implied_operating_point(&mut analyses, notices);
    if analyses.instances().is_empty() {
        notices.push((
            ForeignNoticeSeverity::Notice,
            "The schematic states no analysis RSpice runs; the active simulation plan's analyses are unchanged."
                .to_owned(),
        ));
        bind_to_active_plan(app, model_bindings, notices);
        return None;
    }
    let stated = analyses.instances().len();
    let name = free_plan_name(app, cell);
    let active = app.state.model.sim_setup.active_plan_name().to_string();
    let replace = asking.is_none_or(|asking| {
        let question = plan_question(app, &asking, &analyses, &name);
        (asking.ask)(&question) == PlanAnswer::Replace
    });
    let mut variables = Vec::new();
    for (name, value) in &plan.variables {
        match DesignVariable::new(
            name.clone(),
            value.clone(),
            DesignVariableQuantity::Dimensionless,
            DesignVariableScope::Testbench,
            String::new(),
            None,
            DesignVariableSweepEligibility::NestedSweepAndOptimization,
            DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        ) {
            Ok(variable) => variables.push(variable),
            Err(error) => notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("`.param {name}={value}` is not a design variable: {error}"),
            )),
        }
    }
    let options = plan.options.clone();
    let mut reference_pvt =
        rspice_app_state::workbench::app_state::sim_setup::ReferencePvtPoint::default();
    if let Some(temperature) = plan.temperature {
        reference_pvt.temperature_celsius = temperature;
    }
    let payload = rspice_app_state::state::SimulationPlanPayload {
        design_variables: variables,
        specs: plan
            .measurements
            .iter()
            .map(|(_, definition)| definition.projected_entry())
            .collect(),
        specification_definitions: plan
            .measurements
            .iter()
            .map(|(_, definition)| definition.clone())
            .collect(),
        ..rspice_app_state::state::SimulationPlanPayload::default()
    };
    match commit_plan_document(
        app,
        &name,
        analyses,
        reference_pvt,
        options,
        run_set(plan),
        model_bindings,
        &payload,
        replace,
    ) {
        Ok(()) => {
            let (role, stays) = if replace {
                ("the active plan", String::new())
            } else {
                ("the plan", format!(", and `{active}` stays active"))
            };
            notices.push((
                ForeignNoticeSeverity::Notice,
                format!(
                    "The schematic's analyses are {role} `{name}`, with {} stepped axis(es) in its run set and {} measurement(s){stays}.",
                    plan.sweeps.len(),
                    plan.measurements.len()
                ),
            ));
            Some(format!(
                "its {} {} {role} `{name}`{stays}",
                rspice_ui_kit::accessibility::counted(stated, "analysis", "analyses"),
                if stated == 1 { "is" } else { "are" }
            ))
        }
        Err(error) => {
            notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("The schematic's analyses could not become a simulation plan: {error}"),
            ));
            None
        }
    }
}

/// A deck's `.ac` or `.noise` solves its own operating point; a plan states
/// it, so the analyses that need one get one ahead of them and bind it.
fn bind_implied_operating_point(
    analyses: &mut rspice_simulation_contract::plan_model::SimulationPlan,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    use rspice_app_state::simulation::plan::{AnalysisDraft, AnalysisKind};
    let dependents = analyses
        .instances()
        .iter()
        .map(|instance| instance.id())
        .filter(|id| {
            analyses
                .required_prerequisite_roles(*id)
                .contains(&AnalysisKind::OperatingPoint)
        })
        .collect::<Vec<_>>();
    if dependents.is_empty() {
        return;
    }
    if let Err(error) = analyses.insert_draft_with_id(
        rspice_vocabulary::product::AnalysisInstanceId::new(),
        AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
        true,
        0,
    ) {
        notices.push((
            ForeignNoticeSeverity::Refusal,
            format!("The plan's operating point could not be added: {error}"),
        ));
        return;
    }
    for id in dependents {
        if let Err(error) = analyses.auto_bind_dependencies(id) {
            notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("An imported analysis could not bind its operating point: {error}"),
            ));
        }
    }
}

/// The plan's run set: its reference point alone, crossed with every axis
/// the drawing steps. A stepped temperature takes the run set's own
/// temperature axis.
fn run_set(plan: &ForeignPlan) -> RunSetState {
    let mut run_set = RunSetState::reference_only();
    for (_, sweep) in &plan.sweeps {
        let template = run_set
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.kind == sweep.kind)
            .filter(|_| sweep.kind == RunSetDimensionKind::Temperature);
        match template {
            Some(temperature) => {
                temperature.values = sweep
                    .values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        let mut value = value.clone();
                        value.id = value_id(&temperature.id, index);
                        value
                    })
                    .collect();
                temperature.enabled = true;
            }
            None => run_set.dimensions.push(sweep.clone()),
        }
    }
    run_set
}

/// The name the drawing's plan takes: the cell's, numbered past any plan of
/// the project that has it.
fn free_plan_name(app: &RSpiceApp, cell: &str) -> String {
    let setup = &app.state.model.sim_setup;
    let taken = |name: &str| {
        setup.active_plan_name().as_str().eq_ignore_ascii_case(name)
            || setup
                .inactive_plans()
                .iter()
                .any(|plan| plan.name().as_str().eq_ignore_ascii_case(name))
    };
    std::iter::once(cell.to_owned())
        .chain((2..).map(|suffix| format!("{cell} {suffix}")))
        .find(|name| !taken(name))
        .expect("an unbounded suffix range finds a free name")
}

/// What the import asks before the drawing's `analyses` replace the active
/// plan: both plans' analyses, and where the drawing's go when it is kept.
fn plan_question(
    app: &RSpiceApp,
    asking: &Asking<'_>,
    analyses: &rspice_simulation_contract::plan_model::SimulationPlan,
    name: &str,
) -> PlanQuestion {
    let listed = |plan: &rspice_simulation_contract::plan_model::SimulationPlan| {
        let names = (0..plan.instances().len())
            .filter_map(|index| plan.instance_list_label(index))
            .collect::<Vec<_>>();
        if names.is_empty() {
            "no analyses".to_owned()
        } else {
            names.join(", ")
        }
    };
    let setup = &app.state.model.sim_setup;
    let active = setup.active_plan_name().to_string();
    let theirs = listed(analyses);
    let ours = setup
        .stable_analysis_plan()
        .map_or_else(|_| "its analyses".to_owned(), listed);
    PlanQuestion {
        title: asking.title.clone(),
        description: format!(
            "The analyses of {} ({theirs}) would replace the active plan \u{201c}{active}\u{201d} \
             ({ours}), which the project runs. Replace it, or keep it and add them as the plan \
             \u{201c}{name}\u{201d}?",
            asking.source
        ),
        keep: format!("Keep \u{201c}{active}\u{201d}"),
    }
}

/// Imports the drawing's plan under `name`, making it the active plan when
/// `activate` says so; otherwise it joins the project beside the active
/// plan, which stays.
fn commit_plan_document(
    app: &mut RSpiceApp,
    name: &str,
    analysis_plan: rspice_simulation_contract::plan_model::SimulationPlan,
    reference_pvt: rspice_app_state::workbench::app_state::sim_setup::ReferencePvtPoint,
    options: rspice_simulation_contract::options::SimulationOptions,
    run_set: RunSetState,
    model_bindings: Vec<SimulationPlanModelBinding>,
    payload: &rspice_app_state::state::SimulationPlanPayload,
    activate: bool,
) -> Result<(), String> {
    use rspice_app_state::workbench::app_state::sim_setup::plan_catalog::SimulationPlanName;
    use rspice_simulation_contract::plan_catalog::SimulationPlanImportDocument;

    let document = SimulationPlanImportDocument {
        source_plan_id: analysis_plan.id(),
        source_revision: analysis_plan.revision(),
        name: SimulationPlanName::new(name).map_err(|error| error.to_string())?,
        analysis_plan,
        reference_pvt,
        run_set,
        model_bindings,
        design_configuration: Default::default(),
        // A deck's `.save` and `.probe` cards are not carried over, so the
        // plan keeps every waveform the run produces.
        save_policy: rspice_app_state::workbench::app_state::sim_setup::SimulationSavePolicy {
            output_selection_mode: rspice_app_state::state::OutputSelectionMode::SaveAll,
            ..rspice_app_state::workbench::app_state::sim_setup::SimulationSavePolicy::default()
        },
        options,
    };
    let mut setup = app.state.model.sim_setup.clone();
    let mut workspace = app.state.model.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.content.migrate_active_plan_data(current_id);
    let outcome = setup
        .import_plan(document)
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .import_plan_data(
            outcome.cloned_plan_id,
            payload,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    if !activate {
        // Opened back as the plan manager opens a plan: the imported plan's
        // payload moves out with it, and the kept plan's comes back.
        workspace
            .content
            .migrate_active_plan_data(outcome.cloned_plan_id);
        workspace.content.migrate_inactive_plan_data(current_id);
        setup
            .activate_plan(current_id)
            .map_err(|error| error.to_string())?;
        workspace.content.sync_legacy_specs_projection(current_id);
    }
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    rspice_app_services::io::capture_execution_context(
        &setup,
        &app.state.model.model_library_manager,
    )?;
    let first = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    app.state.model.sim_setup = setup;
    app.state.model.workspace = workspace;
    if activate {
        app.state.simulate.active_analysis_instance = first;
    }
    Ok(())
}

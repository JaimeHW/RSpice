//! Importing an LTspice (`.asc`) or QSPICE (`.qsch`) schematic into the open
//! project.
//!
//! The drawing becomes a cell of the active library with a schematic view.
//! A symbol RSpice draws no part for becomes a cell of its own, and a block's
//! schematic becomes its cell's schematic view. The directives become project
//! content: the analyses, options and temperature a simulation plan named
//! after the cell, constant parameters its design variables, the drawing's
//! model text and the source program's own cards for the models its parts
//! name a model library of the project, and every file the drawing includes
//! a model library bound in place. A card with no project place stays on the
//! sheet as text. Every notice is written to the log.

mod plan;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use rspice_design::foreign_schematic::{
    ForeignCard, ForeignCardRole, ForeignLibrary, ForeignNoticeSeverity, ForeignSymbolCell,
    ForeignTool, InstalledForeignLibrary, decode_foreign_text, foreign_directive_cards,
    import_foreign_schematic, keep_directive_text, parse_asc, parse_qsch,
};
use rspice_design::schematic::document::SchematicDocument;
use rspice_simulation_contract::design_variable::{
    DesignVariable, DesignVariableOverridePolicy, DesignVariableScope,
    DesignVariableSweepEligibility,
};
use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{Cell, CellViewRef, SchematicState, View, ViewType};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::file_workflow::{
    SchematicLoadOrigin, import_loaded_schematic_as_cell_view,
};

use plan::{ForeignPlan, plan_from_cards};

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
    if let Err(error) = import_foreign_schematic_file(app, &path, tool) {
        app.state.record(
            LogSeverity::Error,
            LogSource::Studio,
            format!("{title} failed: {error}"),
        );
        app.state.prompt(error);
    }
}

/// The reason the import cannot run now, if any.
pub(crate) fn foreign_import_refusal(state: &AppState) -> Option<&'static str> {
    if !state.platform.project_lifecycle.is_open() {
        Some("open a project first")
    } else if state.model.safe_mode.project_read_only() {
        Some("the project is open read-only")
    } else if state.model.simulation.execution.has_active_execution() {
        Some("an active simulation execution still owns the project")
    } else {
        None
    }
}

/// A drawing and everything under it, imported.
struct ImportedTree {
    document: SchematicDocument,
    /// Symbol cells, with the schematic of each block.
    cells: Vec<(ForeignSymbolCell, Option<SchematicDocument>)>,
    cards: Vec<ForeignCard>,
    model_cards: Vec<String>,
    notices: Vec<(ForeignNoticeSeverity, String)>,
}

/// Imports the schematic at `path` into the open project and opens it.
/// Returns the sentence the log records.
pub(crate) fn import_foreign_schematic_file(
    app: &mut RSpiceApp,
    path: &Path,
    tool: ForeignTool,
) -> Result<String, String> {
    if let Some(refusal) = foreign_import_refusal(&app.state) {
        return Err(format!("The schematic cannot be imported: {refusal}."));
    }
    let library = app
        .state
        .model
        .workspace
        .content
        .active_view
        .library
        .clone();
    let mut seen = HashSet::from([path.to_path_buf()]);
    let mut tree = import_tree(tool, path, &library, &mut seen, true)?;
    rename_taken_cells(&app.state, &library, &mut tree);
    let mut plan = plan_from_cards(std::mem::take(&mut tree.cards));
    let mut notices = std::mem::take(&mut tree.notices);
    notices.extend(
        plan.notices
            .drain(..)
            .map(|notice| (ForeignNoticeSeverity::Notice, notice)),
    );

    let access = InstalledForeignLibrary::for_drawing(tool, path);
    let includes = resolve_includes(&access, &mut plan, &mut notices);
    keep_on_sheet(&mut tree.document, &plan, &mut notices);

    add_symbol_cells(&mut app.state, &library, &mut tree.cells)?;
    let reference = import_loaded_schematic_as_cell_view(
        &mut app.state,
        SchematicState::from_document(std::mem::take(&mut tree.document)),
        SchematicLoadOrigin::PersistentPath(path),
    )?;
    adopt_as_top_cell(&mut app.state, &reference, &mut notices);
    add_model_libraries(
        &mut app.state,
        &reference.cell,
        model_text(&plan, &tree.model_cards),
        &includes,
        &mut notices,
    );
    commit_plan(app, &reference.cell, &plan, &mut notices);

    let refusals = notices
        .iter()
        .filter(|(severity, _)| *severity == ForeignNoticeSeverity::Refusal)
        .count();
    for (severity, message) in &notices {
        let severity = match severity {
            ForeignNoticeSeverity::Notice => LogSeverity::Info,
            ForeignNoticeSeverity::Refusal => LogSeverity::Warning,
        };
        app.state
            .record(severity, LogSource::Studio, message.clone());
    }
    let summary = format!(
        "Imported {} schematic {} as {}: {} notice(s), {refusals} of them about content that did not come across.",
        tool.label(),
        path.display(),
        reference.display_path(),
        notices.len(),
    );
    app.state
        .record(LogSeverity::Info, LogSource::Studio, summary.clone());
    app.invalidate_simulation_preflight();
    Ok(summary)
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
    library: &str,
    seen: &mut HashSet<PathBuf>,
    top: bool,
) -> Result<ImportedTree, String> {
    let drawing = read_drawing(tool, path)?;
    let access = InstalledForeignLibrary::for_drawing(tool, path);
    let imported = import_foreign_schematic(&drawing, library, &access);
    let mut cards = foreign_directive_cards(&imported.directives);
    if !top {
        cards.retain(|card| {
            matches!(
                card.role,
                ForeignCardRole::ModelText | ForeignCardRole::Parameter | ForeignCardRole::Include
            )
        });
    }
    let mut tree = ImportedTree {
        document: imported.document,
        cells: Vec::new(),
        cards,
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
                let child = import_tree(tool, block, library, seen, false)?;
                tree.cards.extend(child.cards);
                tree.model_cards.extend(child.model_cards);
                tree.notices.extend(child.notices);
                tree.cells.extend(child.cells);
                Some(child.document)
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

/// Writes every card with no project place onto the sheet where its block
/// was drawn, one note per block.
fn keep_on_sheet(
    document: &mut SchematicDocument,
    plan: &ForeignPlan,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    let mut blocks = BTreeMap::<(i32, i32), Vec<&str>>::new();
    for (card, reason) in &plan.kept {
        notices.push((
            ForeignNoticeSeverity::Notice,
            format!(
                "`{}` stays on the sheet as text: {reason}.",
                card.text.lines().next().unwrap_or_default()
            ),
        ));
        blocks
            .entry((card.position.x, card.position.y))
            .or_default()
            .push(card.text.as_str());
    }
    for ((x, y), texts) in blocks {
        if let Err(error) = keep_directive_text(
            document,
            rspice_design_model::Point::new(x, y),
            &texts.join("\n"),
        ) {
            notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("Directive text could not be placed on the sheet: {error}."),
            ));
        }
    }
}

/// Adds the symbol cells, and each block's schematic, to the library.
fn add_symbol_cells(
    state: &mut AppState,
    library: &str,
    cells: &mut Vec<(ForeignSymbolCell, Option<SchematicDocument>)>,
) -> Result<(), String> {
    let Some((first, _)) = cells.first() else {
        return Ok(());
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
        if let Some(document) = schematic {
            entry.add_view(View::new("schematic", ViewType::Schematic));
            blocks.push((CellViewRef::new(library, &cell.cell, "schematic"), document));
        }
        edit.add_cell(entry);
    }
    drop(edit);
    let mutation = state.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::CreateCell {
            library: library.to_owned(),
            cell: first,
        },
    )?;
    state.model.library_manager = candidate;
    for (reference, document) in blocks {
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
    Ok(())
}

/// A project whose top cell draws nothing simulates the imported cell; any
/// other keeps its top cell, and the log says how to simulate the import.
fn adopt_as_top_cell(
    state: &mut AppState,
    reference: &CellViewRef,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
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
    } else {
        notices.push((
            ForeignNoticeSeverity::Notice,
            format!(
                "The project simulates {}; make {} its top cell to simulate the imported schematic.",
                top.display_path(),
                reference.display_path()
            ),
        ));
    }
}

/// The model-library text: the drawing's own model cards and parameter
/// expressions, then the source program's cards for the models its parts
/// name.
fn model_text(plan: &ForeignPlan, library_cards: &[String]) -> String {
    let mut seen = HashSet::new();
    let mut text = String::new();
    for card in plan.model_text.iter().chain(library_cards) {
        if seen.insert(card.to_ascii_lowercase()) {
            text.push_str(card);
            text.push('\n');
        }
    }
    text
}

/// Adds the drawing's model text as a model library of the project, and
/// binds each included file as a model library read in place.
fn add_model_libraries(
    state: &mut AppState,
    cell: &str,
    text: String,
    includes: &[(PathBuf, Option<String>)],
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    use crate::workbench::app_state::design_history::publish_model_library_candidate;

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
            Ok(name) => notices.push((
                ForeignNoticeSeverity::Notice,
                format!("The schematic's model text is the project's model library `{name}`."),
            )),
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
        if let Err(error) = loaded
            && !error.contains("already exists")
        {
            notices.push((
                ForeignNoticeSeverity::Refusal,
                format!(
                    "{} could not be bound as a model library: {error}",
                    path.display()
                ),
            ));
        }
    }
}

/// Activates a simulation plan named after the cell holding the drawing's
/// analyses, options, temperature and design variables. A drawing with no
/// analysis leaves the active plan as it is.
fn commit_plan(
    app: &mut RSpiceApp,
    cell: &str,
    plan: &ForeignPlan,
    notices: &mut Vec<(ForeignNoticeSeverity, String)>,
) {
    let mut analyses = rspice_simulation_contract::plan_model::SimulationPlan::empty();
    for (card, draft) in &plan.analyses {
        if let Some(error) = app
            .state
            .model
            .sim_setup
            .analysis_draft_validation_error(draft)
        {
            notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("`{}` is not a valid plan analysis: {error}", card.text),
            ));
            continue;
        }
        let id = rspice_vocabulary::product::AnalysisInstanceId::new();
        let position = analyses.instances().len();
        if let Err(error) = analyses.insert_draft_with_id(id, draft.clone(), true, position) {
            notices.push((
                ForeignNoticeSeverity::Refusal,
                format!("`{}` could not join the plan: {error}", card.text),
            ));
        }
    }
    if analyses.instances().is_empty() {
        notices.push((
            ForeignNoticeSeverity::Notice,
            "The schematic states no analysis RSpice runs; the active simulation plan is unchanged."
                .to_owned(),
        ));
        return;
    }
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
    let mut options = plan.options.clone();
    let mut reference_pvt = crate::workbench::app_state::ReferencePvtPoint::default();
    if let Some(temperature) = plan.temperature {
        reference_pvt.temperature_celsius = temperature;
        options.temp = temperature;
    }
    match commit_plan_document(app, cell, analyses, reference_pvt, options, variables) {
        Ok(name) => notices.push((
            ForeignNoticeSeverity::Notice,
            format!("The schematic's analyses are the active simulation plan `{name}`."),
        )),
        Err(error) => notices.push((
            ForeignNoticeSeverity::Refusal,
            format!("The schematic's analyses could not become a simulation plan: {error}"),
        )),
    }
}

fn commit_plan_document(
    app: &mut RSpiceApp,
    cell: &str,
    analysis_plan: rspice_simulation_contract::plan_model::SimulationPlan,
    reference_pvt: crate::workbench::app_state::ReferencePvtPoint,
    options: rspice_simulation_contract::options::SimulationOptions,
    design_variables: Vec<DesignVariable>,
) -> Result<String, String> {
    use crate::workbench::app_state::{SimulationPlanImportDocument, SimulationPlanName};

    let taken = |name: &str| {
        let setup = &app.state.model.sim_setup;
        setup.active_plan_name().as_str().eq_ignore_ascii_case(name)
            || setup
                .inactive_plans()
                .iter()
                .any(|plan| plan.name().as_str().eq_ignore_ascii_case(name))
    };
    let name = std::iter::once(cell.to_owned())
        .chain((2..).map(|suffix| format!("{cell} {suffix}")))
        .find(|name| !taken(name))
        .expect("an unbounded suffix range finds a free name");
    let document = SimulationPlanImportDocument {
        source_plan_id: analysis_plan.id(),
        source_revision: analysis_plan.revision(),
        name: SimulationPlanName::new(name).map_err(|error| error.to_string())?,
        analysis_plan,
        reference_pvt,
        run_set: rspice_simulation_contract::run_set::RunSetState::reference_only(),
        model_bindings: Vec::new(),
        save_policy: crate::workbench::app_state::SimulationSavePolicy::default(),
        options,
    };
    let payload = crate::state::SimulationPlanPayload {
        design_variables,
        ..crate::state::SimulationPlanPayload::default()
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
            &payload,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    crate::io::capture_execution_context(&setup, &app.state.model.model_library_manager)?;
    let first = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let name = setup.active_plan_name().to_string();
    app.state.model.sim_setup = setup;
    app.state.model.workspace = workspace;
    app.state.simulate.active_analysis_instance = first;
    Ok(name)
}

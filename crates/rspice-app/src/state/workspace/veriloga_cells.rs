//! Every Verilog-A source is a cell view.
//!
//! A module becomes a cell named after it, with a `veriloga` view that owns
//! the bundle and states the module and its ports. Two paths arrive here: a
//! project that still carries the retired project-wide Verilog-A bundle,
//! which is moved into a cell when it loads, and File > Import, which makes
//! one cell per top module of the file it reads. Both name cells by the same
//! rule, so an imported module and a migrated one land in the same place.

use std::path::{Path, PathBuf};

use rspice_design::schematic::interface_repair::same_terminal_contract;

use crate::state::{
    Cell, CellViewRef, Library, LibraryManager, ProjectSourceBundle, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRegistry, View, ViewType,
};

/// The view every module is placed under.
pub(crate) const VERILOGA_VIEW: &str = "veriloga";

/// The root file of the demonstration module every new project used to be
/// created with. A project whose project-wide bundle still holds exactly this
/// carries nothing its author wrote, so loading it drops the bundle.
const RETIRED_DEMO_ROOT: &str = "sensor_bridge.va";
const RETIRED_DEMO_SOURCE: &str = "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule";

/// One module a source declares, with its ports in declaration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclaredModule {
    pub(crate) name: String,
    pub(crate) ports: Vec<String>,
}

/// What a Verilog-A source declares: its top modules, the ones no other
/// module in the same source instantiates, first-in-the-root-file first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModuleSurvey {
    pub(crate) tops: Vec<DeclaredModule>,
}

/// Read the modules `bundle` declares.
pub(crate) fn survey_veriloga_bundle(bundle: &ProjectSourceBundle) -> Result<ModuleSurvey, String> {
    survey_veriloga_sources(bundle.root().logical_path(), &|path| {
        bundle.file_content(path)
    })
}

/// Read the modules one file declares, as the root of a bundle of its own.
pub(crate) fn survey_veriloga_file(path: &str, contents: &str) -> Result<ModuleSurvey, String> {
    survey_veriloga_sources(path, &|candidate| (candidate == path).then_some(contents))
}

/// Read the modules the source rooted at `root` declares, through the same
/// preprocessor and parser the compiler runs, so an include or a macro is read
/// the way a compile reads it. Refused, with the front end's own words, for a
/// source that does not parse or declares no module.
fn survey_veriloga_sources<'a>(
    root: &str,
    file: &dyn Fn(&str) -> Option<&'a str>,
) -> Result<ModuleSurvey, String> {
    let root_source = file(root).ok_or_else(|| format!("{root} is not in the source"))?;
    let preprocessed = rspice_veriloga::Preprocessor::new()
        .preprocess_provider_root(&SourceFiles(file), Path::new(root))
        .map_err(|error| error.to_string())?;
    let mut sources = rspice_veriloga::SourceMap::new();
    let source_id = sources.add_source_mut(root, preprocessed.as_str());
    let tokens = rspice_veriloga::Lexer::new(&preprocessed, source_id)
        .collect_tokens()
        .map_err(|error| error.to_string())?;
    let file = rspice_veriloga::Parser::new(&tokens)
        .parse()
        .map_err(|error| error.to_string())?;
    let modules = file
        .items
        .iter()
        .filter_map(|item| match item {
            rspice_veriloga::ast::Item::Module(module) => Some(module),
            _ => None,
        })
        .collect::<Vec<_>>();
    if modules.is_empty() {
        return Err(format!("{root} declares no module"));
    }
    let instantiated = modules
        .iter()
        .flat_map(|module| module.instances.iter())
        .map(|instance| instance.module.to_ascii_lowercase())
        .collect::<std::collections::BTreeSet<_>>();
    let root_order = root_file_module_order(root_source);
    let mut declared = modules
        .iter()
        .enumerate()
        .map(|(position, module)| {
            let in_root = root_order
                .iter()
                .position(|name| name.eq_ignore_ascii_case(&module.name))
                .unwrap_or(usize::MAX);
            let top = !instantiated.contains(&module.name.to_ascii_lowercase());
            let module = DeclaredModule {
                name: module.name.to_string(),
                ports: module
                    .ports
                    .iter()
                    .map(|port| port.name.to_string())
                    .collect(),
            };
            ((in_root, position), top, module)
        })
        .collect::<Vec<_>>();
    declared.sort_by_key(|(order, _, _)| *order);
    // A source whose modules all instantiate one another has no top, and then
    // every module is a candidate.
    let any_top = declared.iter().any(|(_, top, _)| *top);
    Ok(ModuleSurvey {
        tops: declared
            .into_iter()
            .filter(|(_, top, _)| *top || !any_top)
            .map(|(_, _, module)| module)
            .collect(),
    })
}

/// The modules the root file itself declares, in the order it declares them.
///
/// The preprocessed text interleaves included files with the root, so it
/// cannot say which module the author wrote first in the root; the root's own
/// tokens can.
fn root_file_module_order(root: &str) -> Vec<String> {
    let mut sources = rspice_veriloga::SourceMap::new();
    let source_id = sources.add_source_mut("root", root);
    let Ok(tokens) = rspice_veriloga::Lexer::new(root, source_id).collect_tokens() else {
        return Vec::new();
    };
    tokens
        .windows(2)
        .filter(|pair| {
            matches!(
                pair[0].kind,
                rspice_veriloga::TokenKind::Module | rspice_veriloga::TokenKind::Macromodule
            ) && pair[1].kind == rspice_veriloga::TokenKind::Identifier
        })
        .filter_map(|pair| pair[1].text.clone())
        .collect()
}

/// A source's files, as the preprocessor's include provider. An include
/// resolves beside the file that names it, then from the source root; one the
/// source does not hold falls through to the preprocessor's standard headers.
struct SourceFiles<'f, 'a>(&'f dyn Fn(&str) -> Option<&'a str>);

impl SourceFiles<'_, '_> {
    fn document(&self, logical_path: &str) -> Option<rspice_veriloga::SourceDocument> {
        (self.0)(logical_path)
            .map(|content| rspice_veriloga::SourceDocument::provided(logical_path, content))
    }
}

impl rspice_veriloga::SourceProvider for SourceFiles<'_, '_> {
    fn load_root(
        &self,
        requested: &Path,
    ) -> Result<rspice_veriloga::SourceDocument, rspice_veriloga::PreprocessorError> {
        let logical = logical_path(requested);
        self.document(&logical).ok_or_else(|| {
            rspice_veriloga::PreprocessorError::new(
                format!("{logical} is not in the source bundle"),
                Some(requested.to_path_buf()),
                0,
            )
        })
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        _include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<rspice_veriloga::SourceDocument>, rspice_veriloga::PreprocessorError> {
        let beside = including_file.map(logical_path).and_then(|including| {
            including
                .rsplit_once('/')
                .map(|(parent, _)| format!("{parent}/{requested}"))
        });
        Ok(beside
            .and_then(|path| self.document(&path))
            .or_else(|| self.document(requested)))
    }
}

fn logical_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Whether `bundle` is the untouched demonstration module: its one file, by
/// the same name, byte for byte.
fn is_retired_demo(bundle: &ProjectSourceBundle) -> bool {
    bundle.root().logical_path() == RETIRED_DEMO_ROOT
        && bundle.root().content() == RETIRED_DEMO_SOURCE
        && bundle.files().is_empty()
}

/// The cell view a module is placed in, and whether that cell is new.
///
/// `<module>` when no cell has that name. An existing cell of that name takes
/// the view when it has no `veriloga` view and declares the module's
/// interface; otherwise the module goes to the first free `<module>_va`,
/// `<module>_va2`, and so on. `reserved` holds names an enclosing transaction
/// has already claimed.
///
/// Names are compared by the canonical cell-view identity New Cell and the
/// source registry use, so a module never lands in a cell that differs from
/// one already there only by Unicode case or composition.
pub(crate) fn veriloga_cell_for_module(
    libraries: &LibraryManager,
    interface_of: &dyn Fn(&Cell, &CellViewRef) -> Vec<String>,
    library: &str,
    module: &DeclaredModule,
    reserved: &[String],
) -> (CellViewRef, bool) {
    let same = |left: &str, right: &str| {
        crate::state::canonical_cell_view_owner_key("", left, "")
            == crate::state::canonical_cell_view_owner_key("", right, "")
    };
    let existing = |name: &str| {
        libraries
            .get_library(library)
            .and_then(|library| library.cells.values().find(|cell| same(&cell.name, name)))
    };
    let claimed =
        |name: &str| existing(name).is_some() || reserved.iter().any(|taken| same(taken, name));
    let base = cell_name_for_module(&module.name);
    if let Some(cell) = existing(&base)
        && !reserved.iter().any(|taken| same(taken, &base))
        && !cell
            .views
            .values()
            .any(|view| same(&view.name, VERILOGA_VIEW))
    {
        let reference = CellViewRef::new(library, &cell.name, VERILOGA_VIEW);
        if same_terminal_contract(&interface_of(cell, &reference), &module.ports) {
            return (reference, false);
        }
    }
    if !claimed(&base) {
        return (CellViewRef::new(library, &base, VERILOGA_VIEW), true);
    }
    let name = (1_u32..)
        .map(|index| {
            if index == 1 {
                format!("{base}_va")
            } else {
                format!("{base}_va{index}")
            }
        })
        .find(|candidate| !claimed(candidate))
        .expect("an unbounded sequence has a free name");
    (CellViewRef::new(library, &name, VERILOGA_VIEW), true)
}

/// A module name as a cell name: every character a cell name cannot hold
/// becomes `_`.
fn cell_name_for_module(module: &str) -> String {
    let name = module
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if name.is_empty() {
        "module".to_owned()
    } else {
        name
    }
}

/// The interface a cell already declares: its schematic's ports when one of
/// its schematic views has a buffer here, else its symbol's pins.
pub(crate) fn declared_cell_interface(
    schematic_ports: &dyn Fn(&CellViewRef) -> Option<Vec<String>>,
    cell: &Cell,
    library: &str,
) -> Vec<String> {
    let mut views = cell.views.values().collect::<Vec<_>>();
    views.sort_by(|left, right| left.name.cmp(&right.name));
    views
        .iter()
        .filter(|view| matches!(view.view_type, ViewType::Schematic | ViewType::Testbench))
        .find_map(|view| {
            schematic_ports(&CellViewRef::new(library, &cell.name, &view.name))
                .filter(|ports| !ports.is_empty())
        })
        .or_else(|| {
            views
                .iter()
                .filter(|view| view.view_type == ViewType::Symbol)
                .find_map(|view| {
                    crate::state::SymbolDocument::load_from_view(view)
                        .ok()
                        .map(|symbol| symbol.pins.into_iter().map(|pin| pin.name).collect())
                        .filter(|pins: &Vec<String>| !pins.is_empty())
                })
        })
        .unwrap_or_default()
}

/// The `veriloga` view stating `module` and its ports, the way New View
/// writes them. A module the source could not be read for states neither.
pub(crate) fn veriloga_view(module: Option<&DeclaredModule>) -> Result<View, String> {
    let mut view = View::new(VERILOGA_VIEW, ViewType::VerilogA);
    if let Some(module) = module {
        view.metadata
            .insert("veriloga.module".to_owned(), module.name.clone());
        view.metadata.insert(
            "veriloga.ports".to_owned(),
            serde_json::to_string(&module.ports).map_err(|error| error.to_string())?,
        );
    }
    Ok(view)
}

/// Put `view` at `reference`: in a new cell, or beside the views of the cell
/// that already holds that name. The library is created if it is missing.
pub(crate) fn place_veriloga_view(
    libraries: &mut LibraryManager,
    reference: &CellViewRef,
    view: View,
) -> Result<(), String> {
    if libraries.get_library(&reference.library).is_none() {
        libraries.add_library(Library::new(&reference.library));
    }
    let mut library = libraries
        .edit_library(&reference.library)
        .ok_or_else(|| format!("Library '{}' not found", reference.library))?;
    if library.library().get_cell(&reference.cell).is_none() {
        library.add_cell(Cell::new(&reference.cell));
    }
    library
        .add_view(&reference.cell, view)
        .ok_or_else(|| format!("Cell '{}' not found", reference.cell))
}

/// Move a project-wide Verilog-A bundle, if the project still carries one,
/// into the cell view its top module names, and say what was created.
///
/// The untouched demonstration module is dropped without a word. Anything
/// else is the author's: it becomes a cell in the library that holds the
/// project's top cell, owning the same bundle under the same identity, so a
/// deck that named it still does. Nothing changes when there is no such
/// bundle, which is what makes a second load of the same project a no-op.
pub(crate) fn retire_code_workspace_veriloga(
    content: &mut rspice_project::ProjectWorkspace,
    libraries: &mut LibraryManager,
) -> Option<String> {
    let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
    let bundle = content.project_sources.bundle_for_owner(&owner)?.clone();
    if is_retired_demo(&bundle) {
        content
            .project_sources
            .remove(ProjectSourceLanguage::VerilogA);
        return None;
    }
    let survey = survey_veriloga_bundle(&bundle);
    let top = survey
        .as_ref()
        .ok()
        .and_then(|survey| survey.tops.first())
        .cloned();
    let fallback = DeclaredModule {
        name: bundle
            .root()
            .logical_path()
            .rsplit('/')
            .next()
            .and_then(|name| name.split('.').next())
            .filter(|stem| !stem.is_empty())
            .unwrap_or("module")
            .to_owned(),
        ports: Vec::new(),
    };
    let library = content.project.root_library.clone();
    let buffers = &content.schematic_buffers;
    let schematic_ports = |reference: &CellViewRef| {
        buffers.get(&reference.key()).map(|schematic| {
            schematic
                .document()
                .interface_ports()
                .into_iter()
                .map(|port| port.name)
                .collect()
        })
    };
    let interface_of = |cell: &Cell, reference: &CellViewRef| {
        declared_cell_interface(&schematic_ports, cell, &reference.library)
    };
    let (reference, _) = veriloga_cell_for_module(
        libraries,
        &interface_of,
        &library,
        top.as_ref().unwrap_or(&fallback),
        &[],
    );

    let mut candidate_libraries = libraries.clone();
    let mut candidate_sources: ProjectSourceRegistry = content.project_sources.clone();
    let moved = veriloga_view(top.as_ref())
        .and_then(|view| place_veriloga_view(&mut candidate_libraries, &reference, view))
        .and_then(|()| {
            candidate_sources
                .move_code_workspace_bundle_to_cell_view(
                    ProjectSourceLanguage::VerilogA,
                    reference.clone(),
                )
                .map_err(|error| error.to_string())
        });
    if let Err(error) = moved {
        return Some(format!(
            "The project's Verilog-A source {} could not be moved into cell view {}: {error}. It is kept unchanged and is not editable until it is.",
            bundle.root().logical_path(),
            reference.display_path()
        ));
    }
    *libraries = candidate_libraries;
    content.project_sources = candidate_sources;

    let mut notice = format!(
        "Verilog-A now lives in cell views: the project's Verilog-A source {} became cell view {}",
        bundle.root().logical_path(),
        reference.display_path()
    );
    match (&top, &survey) {
        (Some(top), Ok(survey)) => {
            notice.push_str(&format!(" for module {}", top.name));
            let others = survey
                .tops
                .iter()
                .skip(1)
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>();
            if !others.is_empty() {
                notice.push_str(&format!(
                    "; its other top modules ({}) stay in the same source",
                    others.join(", ")
                ));
            }
            notice.push('.');
        }
        (_, Err(error)) => notice.push_str(&format!(
            "; its source does not parse yet ({error}), so the view names no module."
        )),
        (None, Ok(_)) => notice.push('.'),
    }
    Some(notice)
}

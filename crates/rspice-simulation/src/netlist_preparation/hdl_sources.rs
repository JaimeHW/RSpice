//! The Verilog-A/AMS sources a deck's own lines name, sealed by content.
//!
//! A prepared run executes sealed sources only, so a deck's `.hdl`, `.va`,
//! `.veriloga`, `.vams`, `.verilog` and `.ahdl_include` lines cannot reach it
//! as written. Each
//! named file is read through the chain `.include` walks, from the deck's
//! directory and then its include search path; compiled with the unified
//! runtime options against the deck's cells, under its min:typ:max selection
//! and `wreal` resolution and with the line's own `define=` macros, as the
//! engine compiles a deck's HDL; sealed under a key that names what was
//! compiled; and its line rewritten to the `.veriloga "<key>" <alias>` form a
//! sealed project runtime takes. Every other line is left as written, so the
//! deck's line numbers hold.
//!
//! The prepared manual deck the application runs seals here, and so does the
//! mixed-signal conformance runner before it hands a deck and its runtimes
//! to the headless route.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use rspice_core::ElaborationErrorKind;
use rspice_core::abort_signal::NoAbort;
use rspice_core::netlist::{
    IncludeProcessor, NetlistParseOptions, ParseError, ParseWithAbortError,
    ResolvedIncludeDependency, VerilogAInclude,
};
use rspice_veriloga::ExternalCell;
use rspice_veriloga::canonical_ir::digital::{DigitalDelaySelect, DigitalRealResolution};
use rspice_vocabulary::product::ContentDigest;
use sha2::{Digest as _, Sha256};

use super::IncludeSearchChain;
use crate::preparation::{PreparationError, PreparationStage};
use crate::veriloga::{
    PreparedVerilogAConnectionLibrary, PreparedVerilogARuntime, PreparedVerilogARuntimeSet,
};

/// A deck whose Verilog-A source lines name sealed runtimes.
#[derive(Debug, Clone, Default)]
pub struct SealedDeckHdl {
    /// The deck with each source line rewritten to the runtime sealed from it.
    /// A file declaring several modules the deck instantiates seals one
    /// runtime for each: its line names the first, and the others are named
    /// before the terminal `.end`, so no line moves.
    pub source: String,
    /// The runtimes and connection libraries the rewritten lines name.
    pub runtimes: PreparedVerilogARuntimeSet,
    /// Every source exactly as it was read and sealed, with the chain that
    /// found it, so a run can ask later whether the host still gives it.
    pub dependencies: Vec<ResolvedIncludeDependency>,
}

/// Why a Verilog-A source a deck names could not be sealed.
///
/// It names the deck line that names the source and the file that line
/// names, so the refusal points at both before anything is solved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckHdlRefusal {
    /// What kind of failure, in the engine's own taxonomy.
    pub kind: ElaborationErrorKind,
    /// The 1-based deck line that names the source.
    pub line: usize,
    /// The file the line names: where it resolved, or as written when it
    /// resolved nowhere.
    pub path: PathBuf,
    /// The module the refusal is about, when one was selected.
    pub module: Option<String>,
    /// What the reader, resolver or compiler said.
    pub detail: String,
}

impl DeckHdlRefusal {
    fn new(
        kind: ElaborationErrorKind,
        line: usize,
        path: &Path,
        module: Option<&str>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            line,
            path: path.to_path_buf(),
            module: module.map(str::to_owned),
            detail: detail.into(),
        }
    }

    /// The file name the refusal is about.
    pub fn file_name(&self) -> String {
        self.path.file_name().map_or_else(
            || self.path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        )
    }

    /// The source location the compiler named, `file:line[:column]`, when
    /// its message leads with one; otherwise the file.
    pub fn location(&self) -> String {
        leading_location(&self.detail).unwrap_or_else(|| self.path.display().to_string())
    }
}

impl std::fmt::Display for DeckHdlRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let subject = match &self.module {
            Some(module) => format!("module '{module}' of '{}'", self.path.display()),
            None => format!("'{}'", self.path.display()),
        };
        let line = self.line;
        let detail = &self.detail;
        match self.kind {
            ElaborationErrorKind::MissingSource => write!(
                formatter,
                "line {line}: the Verilog-A source {subject} cannot be read: {detail}"
            ),
            ElaborationErrorKind::CompileRefusal => write!(
                formatter,
                "line {line}: the Verilog-A compiler refused {subject}: {detail}"
            ),
            _ => write!(
                formatter,
                "line {line}: the Verilog-A source {subject} cannot be sealed: {detail}"
            ),
        }
    }
}

impl std::error::Error for DeckHdlRefusal {}

impl From<DeckHdlRefusal> for PreparationError {
    fn from(refusal: DeckHdlRefusal) -> Self {
        let line = refusal.line;
        PreparationError::new(PreparationStage::SourceChecks, refusal.to_string())
            .at_line(Some(line))
    }
}

/// Seal every Verilog-A source the deck `source` names on a line of its own,
/// reading each through `include_search` from the directory of `origin`, the
/// deck's file.
///
/// A deck that names none is returned as it is, without being read further.
/// A line inside a false `.if` branch, or after the terminal `.end`, names
/// nothing and is left alone. A line that names a sealed runtime's key is
/// already sealed. A source a file the deck includes names is not this
/// deck's line; preparation refuses it as the unsealed source it is.
pub fn seal_deck_hdl_sources(
    source: &str,
    origin: Option<&Path>,
    include_search: &IncludeSearchChain,
) -> Result<SealedDeckHdl, DeckHdlRefusal> {
    let mut sites = deck_hdl_sites(source);
    if sites.is_empty() {
        return Ok(SealedDeckHdl {
            source: source.to_owned(),
            ..SealedDeckHdl::default()
        });
    }
    let deck = DeckContext::read(source, origin, include_search);
    if let Some(active) = &deck.active_lines {
        sites.retain(|(line, _)| active.contains(line));
    }
    let owner = origin.map(|origin| {
        super::dependencies::absolute_source_identity(origin)
            .unwrap_or_else(|_| origin.to_path_buf())
    });
    let mut processor = IncludeProcessor::new(owner.as_deref().unwrap_or(Path::new(".")));
    include_search.apply_to(&mut processor);

    let mut sealer = Sealer {
        deck: &deck,
        prepared: HashMap::new(),
        runtimes: Vec::new(),
        connections: Vec::new(),
        aliases: BTreeMap::new(),
        rewritten: BTreeMap::new(),
        appended: Vec::new(),
    };
    for (line, directive) in &sites {
        let dependency = read_source(&mut processor, owner.as_deref(), *line, directive)?;
        sealer.seal(*line, directive, &dependency)?;
    }
    let last_line = sites.last().map_or(0, |(line, _)| *line);
    let Sealer {
        runtimes,
        connections,
        rewritten,
        appended,
        ..
    } = sealer;
    let runtimes = PreparedVerilogARuntimeSet::try_with_connections(runtimes, connections)
        .map_err(|error| {
            DeckHdlRefusal::new(
                ElaborationErrorKind::CompileRefusal,
                last_line,
                Path::new(""),
                None,
                error,
            )
        })?;
    let mut sealed = String::with_capacity(source.len());
    for (index, physical) in source.split_inclusive('\n').enumerate() {
        match rewritten.get(&(index + 1)) {
            Some(directive) => {
                let ending = &physical[physical.trim_end_matches(['\r', '\n']).len()..];
                sealed.push_str(directive);
                sealed.push_str(ending);
            }
            None => sealed.push_str(physical),
        }
    }
    if !appended.is_empty() {
        sealed = super::splice_before_terminal_end_card(&sealed, &appended.join("\n"));
    }
    Ok(SealedDeckHdl {
        source: sealed,
        runtimes,
        dependencies: processor.resolved_dependencies().to_vec(),
    })
}

/// Each line of the deck that names a Verilog-A source file, with its 1-based
/// number: read by the engine's own tokenizer, past the title and up to the
/// terminal `.end`.
fn deck_hdl_sites(source: &str) -> Vec<(usize, VerilogAInclude)> {
    let mut sites = Vec::new();
    for (index, line) in source.lines().enumerate().skip(1) {
        if rspice_core::netlist::is_spice_end_card(
            line,
            rspice_core::config::ExpressionDialect::Ngspice,
        ) {
            break;
        }
        let Some(directive) = rspice_core::netlist::parse_veriloga_source_directive(line) else {
            continue;
        };
        if names_sealed_runtime(&directive.file_path) {
            continue;
        }
        sites.push((index + 1, directive));
    }
    sites
}

/// The refusal for a Verilog-A source line a file the deck includes holds,
/// named by that file and its own line.
///
/// Only the deck's own lines are sealed, so a source line left in the
/// expanded deck `expanded` came from one of the included files
/// `includes`. Preparation refuses it either way; counted in the expanded
/// text its line number points nowhere the author can look, so this finds
/// the file and line that hold it. `None` when no such line is left, or when
/// no included file holds it as written.
pub(crate) fn included_hdl_source_refusal(
    expanded: &str,
    includes: &[ResolvedIncludeDependency],
) -> Option<PreparationError> {
    super::executable_logical_lines(expanded)
        .into_iter()
        .filter_map(|(_, line)| {
            let directive = rspice_core::netlist::parse_veriloga_source_directive(&line)?;
            (!names_sealed_runtime(&directive.file_path)).then_some((line, directive))
        })
        .find_map(|(line, directive)| {
            includes.iter().find_map(|include| {
                let number = include
                    .source()
                    .lines()
                    .position(|physical| physical.trim() == line)?
                    + 1;
                Some(PreparationError::new(
                    PreparationStage::SourceChecks,
                    format!(
                        "line {number} of '{}': the Verilog-A source '{}' is named inside an \
                         included file, and a run seals only the sources the deck's own lines \
                         name; name it in the deck",
                        include.resolved_path().display(),
                        directive.file_path.display()
                    ),
                ))
            })
        })
}

/// Whether a directive names the virtual key of a sealed runtime rather than
/// a file.
fn names_sealed_runtime(path: &Path) -> bool {
    path.to_string_lossy()
        .split(['/', '\\'])
        .next()
        .is_some_and(|root| {
            [
                "__rspice_project__",
                "__rspice_pdk__",
                "__rspice_model_library__",
            ]
            .iter()
            .any(|namespace| root.eq_ignore_ascii_case(namespace))
        })
}

/// What the deck gives every HDL compile beyond its sources, read as the
/// engine reads it from the deck it builds, and which of its lines name a
/// source the parse keeps.
struct DeckContext {
    /// The deck's `.subckt`s and model cards, which a design may instantiate
    /// as leaves; `None` when the deck does not parse.
    external_cells: Option<Vec<ExternalCell>>,
    delay_select: DigitalDelaySelect,
    wreal_resolution: DigitalRealResolution,
    /// The deck lines whose source directive the parse keeps; `None` when
    /// the deck does not parse, and every line is taken as written.
    active_lines: Option<std::collections::HashSet<usize>>,
    /// The masters the deck's instances and model cards name, upper case.
    masters: std::collections::HashSet<String>,
}

impl DeckContext {
    /// Read the deck as the engine will: with its includes, through the same
    /// chain. A deck that does not parse here is refused by the same parser
    /// when it is prepared; its sources are still sealed, so a source that
    /// cannot be read is refused by name first.
    fn read(source: &str, origin: Option<&Path>, include_search: &IncludeSearchChain) -> Self {
        let parsed = include_search
            .parse_with_abort(source, origin, NetlistParseOptions::default(), &NoAbort)
            .ok();
        let Some(netlist) = parsed else {
            return Self {
                external_cells: None,
                delay_select: DigitalDelaySelect::default(),
                wreal_resolution: DigitalRealResolution::default(),
                active_lines: None,
                masters: Default::default(),
            };
        };
        let root = origin.map(canonical);
        let active_lines = netlist
            .veriloga_includes
            .iter()
            .filter(|include| {
                include
                    .origin
                    .path
                    .as_deref()
                    .is_none_or(|path| Some(canonical(path)) == root)
            })
            .map(|include| include.origin.line)
            .collect();
        let mut masters = std::collections::HashSet::new();
        let mut subcircuits = netlist.subcircuits.iter().collect::<Vec<_>>();
        let mut elements = netlist.elements.iter().collect::<Vec<_>>();
        while let Some(subcircuit) = subcircuits.pop() {
            elements.extend(&subcircuit.elements);
            subcircuits.extend(&subcircuit.nested_subcircuits);
        }
        for element in elements {
            if let rspice_core::netlist::ElementKind::Subcircuit { subckt_name, .. } = &element.kind
            {
                masters.insert(subckt_name.to_ascii_uppercase());
            }
        }
        for model in &netlist.models {
            masters.insert(model.model_type.to_ascii_uppercase());
        }
        Self {
            external_cells: Some(rspice_core::engine::hdl_external_cells(&netlist)),
            delay_select: rspice_core::engine::hdl_delay_select(
                &rspice_core::resolve_simulation_config(
                    &rspice_core::SimulationConfig::default(),
                    Some(&netlist.options),
                    &rspice_core::SimulationConfigOverrides::default(),
                ),
            ),
            wreal_resolution: rspice_core::engine::hdl_wreal_resolution(&netlist),
            active_lines: Some(active_lines),
            masters,
        }
    }
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// Read the source one deck line names, through the chain from the deck's
/// file. A deck with no file can name a source only by its full path.
fn read_source(
    processor: &mut IncludeProcessor,
    owner: Option<&Path>,
    line: usize,
    directive: &VerilogAInclude,
) -> Result<ResolvedIncludeDependency, DeckHdlRefusal> {
    let spelling = directive.file_path.to_string_lossy().into_owned();
    let owner = match owner {
        Some(owner) => owner.to_path_buf(),
        None if directive.file_path.is_absolute() => directive.file_path.clone(),
        None => {
            return Err(DeckHdlRefusal::new(
                ElaborationErrorKind::MissingSource,
                line,
                &directive.file_path,
                None,
                "the deck has not been saved to a file, so a relative path has no directory to \
                 resolve from; save the deck or name the source by its full path",
            ));
        }
    };
    processor
        .read_verilog_source_with_abort(&owner, line, &spelling, &NoAbort)
        .map_err(|error| {
            let detail = match error {
                ParseWithAbortError::Parse(ParseError::Syntax { message, .. }) => message,
                other => other.to_string(),
            };
            DeckHdlRefusal::new(
                ElaborationErrorKind::MissingSource,
                line,
                &directive.file_path,
                None,
                detail,
            )
        })
}

/// One source file, prepared once for every line that names it under the
/// same macros.
struct PreparedSource {
    file_name: String,
    digest: ContentDigest,
    /// What was compiled: the bytes, the line's macros and the deck's `wreal`
    /// resolution. The cells and the min:typ:max selection are part of every
    /// runtime's own identity.
    identity: ContentDigest,
    prepared: rspice_veriloga::PreparedVirtualSource,
}

/// A source file under the `define=` macros of the line that names it.
type SourceUnderMacros = (PathBuf, Vec<(String, Option<String>)>);

struct Sealer<'a> {
    deck: &'a DeckContext,
    prepared: HashMap<SourceUnderMacros, PreparedSource>,
    runtimes: Vec<PreparedVerilogARuntime>,
    connections: Vec<PreparedVerilogAConnectionLibrary>,
    /// Each alias bound so far: its artifact and the line that bound it.
    aliases: BTreeMap<String, (ContentDigest, usize)>,
    /// The sealed directive each rewritten line now carries.
    rewritten: BTreeMap<usize, String>,
    /// Directives for further modules of a line's source.
    appended: Vec<String>,
}

impl Sealer<'_> {
    fn seal(
        &mut self,
        line: usize,
        directive: &VerilogAInclude,
        dependency: &ResolvedIncludeDependency,
    ) -> Result<(), DeckHdlRefusal> {
        let path = dependency.resolved_path().to_path_buf();
        let deck = self.deck;
        let key = (path.clone(), directive.defines.clone());
        if !self.prepared.contains_key(&key) {
            let source = prepare(deck, line, directive, dependency)?;
            self.prepared.insert(key.clone(), source);
        }
        let source = &self.prepared[&key];
        let refusal = |kind, module: Option<&str>, detail: String| {
            DeckHdlRefusal::new(kind, line, &path, module, detail)
        };
        let modules = source
            .prepared
            .module_names()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        // A source of connect rules and connect modules alone contributes
        // rules, not instances, as the engine reads it.
        if modules.is_empty() && directive.selected_module.is_none() {
            let artifact = source.prepared.connection_artifact().ok_or_else(|| {
                refusal(
                    ElaborationErrorKind::UnknownModule,
                    None,
                    "this source declares neither a device module nor a connection library"
                        .to_owned(),
                )
            })?;
            let alias = directive
                .model_name
                .clone()
                .unwrap_or_else(|| connection_alias(&source.file_name));
            let source_key =
                deck_source_key(source.identity, "connections", &alias, &source.file_name);
            let library = PreparedVerilogAConnectionLibrary::try_new(
                source_key.clone(),
                source.digest,
                alias.clone(),
                artifact,
            )
            .map_err(|error| refusal(ElaborationErrorKind::CompileRefusal, None, error))?;
            let artifact = library.binding().artifact_digest();
            self.bind_alias(&alias, artifact, line, &path)?;
            if !self.connections.iter().any(|known| {
                known
                    .binding()
                    .source_key()
                    .eq_ignore_ascii_case(&source_key)
            }) {
                self.connections.push(library);
            }
            self.rewritten
                .insert(line, super::project_veriloga_directive(&source_key, &alias));
            return Ok(());
        }
        let selected = select_modules(directive, &modules, &self.deck.masters)
            .map_err(|detail| refusal(ElaborationErrorKind::ModuleNotSelected, None, detail))?;
        for (index, module) in selected.iter().enumerate() {
            let source = &self.prepared[&key];
            let compilation = source.prepared.compile_runtime(module).map_err(|failure| {
                refusal(
                    ElaborationErrorKind::CompileRefusal,
                    Some(module),
                    failure.to_string(),
                )
            })?;
            let alias = match (&directive.model_name, selected.len()) {
                (Some(alias), 1) => alias.clone(),
                _ => module.clone(),
            };
            let source_key = deck_source_key(source.identity, module, &alias, &source.file_name);
            // Sealed with the deck's cells it was compiled against, which a
            // design with a leaf carries in its identity and install key, and
            // with the selection it was compiled under, which every design
            // does.
            let runtime = PreparedVerilogARuntime::try_from_virtual_compilation_against(
                source_key.clone(),
                source.digest,
                alias.clone(),
                &compilation,
                self.deck.external_cells.as_deref(),
                self.deck.delay_select,
            )
            .map_err(|error| {
                refusal(
                    ElaborationErrorKind::CompileRefusal,
                    Some(module),
                    error.to_string(),
                )
            })?;
            self.bind_alias(&alias, runtime.artifact_digest(), line, &path)?;
            if !self
                .runtimes
                .iter()
                .any(|known| known.source_key().eq_ignore_ascii_case(&source_key))
            {
                self.runtimes.push(runtime);
            }
            let sealed = super::project_veriloga_directive(&source_key, &alias);
            if index == 0 {
                self.rewritten.insert(line, sealed);
            } else {
                self.appended.push(sealed);
            }
        }
        Ok(())
    }

    /// One deck names one master per alias.
    fn bind_alias(
        &mut self,
        alias: &str,
        artifact: ContentDigest,
        line: usize,
        path: &Path,
    ) -> Result<(), DeckHdlRefusal> {
        match self.aliases.get(&alias.to_ascii_uppercase()) {
            Some((bound, _)) if *bound == artifact => Ok(()),
            Some((_, first)) => Err(DeckHdlRefusal::new(
                ElaborationErrorKind::ModuleNotSelected,
                line,
                path,
                None,
                format!(
                    "the name '{alias}' already names a different Verilog-A master, sealed from line {first}"
                ),
            )),
            None => {
                self.aliases
                    .insert(alias.to_ascii_uppercase(), (artifact, line));
                Ok(())
            }
        }
    }
}

/// Prepare one source under the deck's options and its line's macros.
fn prepare(
    deck: &DeckContext,
    line: usize,
    directive: &VerilogAInclude,
    dependency: &ResolvedIncludeDependency,
) -> Result<PreparedSource, DeckHdlRefusal> {
    let path = dependency.resolved_path();
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("source.va")
        .to_owned();
    let text = dependency.source();
    let digest = ContentDigest::from_bytes(Sha256::digest(text.as_bytes()).into());
    let identity = compile_identity(digest, &directive.defines, deck.wreal_resolution);
    let refusal = |detail: String| {
        DeckHdlRefusal::new(
            ElaborationErrorKind::CompileRefusal,
            line,
            path,
            None,
            detail,
        )
    };
    let bundle = rspice_veriloga::VirtualSourceBundle::new(
        file_name.clone(),
        [rspice_veriloga::VirtualSourceFile::new(
            file_name.clone(),
            text,
        )],
    )
    .map_err(|error| refusal(error.to_string()))?;
    let compiler = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
        external_cells: deck.external_cells.clone(),
        delay_select: deck.delay_select,
        wreal_resolution: deck.wreal_resolution,
        defines: directive.defines.clone(),
        ..crate::compilation::unified_runtime_compiler_options()
    });
    let prepared = compiler
        .prepare_virtual_runtime_source(
            &bundle,
            crate::project_veriloga::project_virtual_compile_limits(),
        )
        .map_err(|failure| refusal(failure.to_string()))?;
    Ok(PreparedSource {
        file_name,
        digest,
        identity,
        prepared,
    })
}

/// The modules a line seals from a source declaring `modules`: the one its
/// `module=` names; the one its alias names; the only one; or, as the engine
/// reads a source declaring several, each one the deck instantiates.
fn select_modules(
    directive: &VerilogAInclude,
    modules: &[String],
    masters: &std::collections::HashSet<String>,
) -> Result<Vec<String>, String> {
    if let Some(module) = &directive.selected_module {
        return Ok(vec![module.clone()]);
    }
    if let Some(alias) = &directive.model_name
        && let Some(module) = modules
            .iter()
            .find(|module| module.eq_ignore_ascii_case(alias))
    {
        return Ok(vec![module.clone()]);
    }
    if let [only] = modules {
        return Ok(vec![only.clone()]);
    }
    let instantiated = if directive.model_name.is_none() {
        modules
            .iter()
            .filter(|module| masters.contains(&module.to_ascii_uppercase()))
            .cloned()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    if instantiated.is_empty() {
        return Err(format!(
            "the file declares multiple modules: {}; select one with module=<name> on the line that loads it",
            modules.join(", ")
        ));
    }
    Ok(instantiated)
}

/// The identity of one compile: the source's bytes, the macros its line
/// defines and the deck's `wreal` resolution. The deck's cells and its
/// min:typ:max selection are part of the sealed runtime's own identity.
fn compile_identity(
    digest: ContentDigest,
    defines: &[(String, Option<String>)],
    wreal_resolution: DigitalRealResolution,
) -> ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.deck-hdl-source/v1\0");
    let mut field = |bytes: &[u8]| {
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    };
    field(digest.as_bytes());
    field(wreal_resolution.keyword().as_bytes());
    field(&(defines.len() as u64).to_le_bytes());
    for (name, value) in defines {
        field(name.as_bytes());
        match value {
            Some(value) => {
                field(b"=");
                field(value.as_bytes());
            }
            None => field(b""),
        }
    }
    ContentDigest::from_bytes(hasher.finalize().into())
}

/// The virtual key a deck source's runtime is installed under. The engine
/// keys an HDL source by its file and the module selected from it, so one
/// file named by two lines (`module=tb_sar`, `module=sar8`) is two runtimes
/// with two keys; the alias is part of it too, so one module under two names
/// is two runtimes.
pub fn deck_source_key(
    identity: ContentDigest,
    module: &str,
    alias: &str,
    file_name: &str,
) -> String {
    format!(
        "__rspice_project__/deck/{identity}/{module}/{}/{file_name}",
        alias.to_ascii_lowercase()
    )
}

/// The import alias of a connection library its line names no alias for:
/// the file's stem as a SPICE name.
fn connection_alias(file_name: &str) -> String {
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(file_name);
    let mut alias: String = stem
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect();
    if !alias.starts_with(|first: char| first.is_ascii_alphabetic()) {
        alias.insert(0, '_');
    }
    alias
}

/// `file:line[:column]` at the start of a compiler message, if it leads with
/// one.
fn leading_location(message: &str) -> Option<String> {
    let head = message.split_whitespace().next()?;
    let head = head.trim_end_matches(':');
    let mut parts = head.rsplitn(3, ':');
    let last = parts.next()?;
    last.parse::<usize>().ok()?;
    Some(head.to_owned())
}

#[cfg(test)]
mod tests;

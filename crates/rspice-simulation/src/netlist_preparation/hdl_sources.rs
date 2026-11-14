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
    IncludeProcessor, NetlistParseOptions, NetlistSourceLocation, ParseError, ParseWithAbortError,
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

mod closure;

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
    /// Every source and header exactly as it was read and sealed, with the
    /// chain that found it, so a run can ask later whether the host still
    /// gives it.
    pub dependencies: Vec<ResolvedIncludeDependency>,
    /// The sealed directive for each source line an included file writes,
    /// by that file's canonical path and the line's 1-based number there.
    pub included: BTreeMap<(PathBuf, usize), String>,
}

impl SealedDeckHdl {
    /// The sealed directive for the line `location` names, when it is a
    /// source line of an included file this deck sealed: what an expansion
    /// of the deck's includes puts in its place
    /// (`IncludeProcessor::expand_content_rewriting`).
    pub fn included_line(&self, location: &NetlistSourceLocation) -> Option<String> {
        if self.included.is_empty() {
            return None;
        }
        let path = canonical(location.path.as_deref()?);
        self.included.get(&(path, location.line)).cloned()
    }
}

/// Why a Verilog-A source a deck names could not be sealed.
///
/// It names the deck line that names the source and the file that line
/// names, so the refusal points at both before anything is solved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckHdlRefusal {
    /// What kind of failure, in the engine's own taxonomy.
    pub kind: ElaborationErrorKind,
    /// The 1-based line that names the source, in the deck or in `file`.
    pub line: usize,
    /// The included file that writes the line; `None` for the deck's own.
    pub file: Option<PathBuf>,
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
            file: None,
            path: path.to_path_buf(),
            module: module.map(str::to_owned),
            detail: detail.into(),
        }
    }

    /// The refusal of a line the included file `file` writes.
    fn in_file(mut self, file: Option<&Path>) -> Self {
        self.file = file.map(Path::to_path_buf);
        self
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
        let line = match &self.file {
            Some(file) => format!("{} of '{}'", self.line, file.display()),
            None => self.line.to_string(),
        };
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
        // A line of an included file is no line of the deck; the message
        // names its file.
        let line = refusal.file.is_none().then_some(refusal.line);
        PreparationError::new(PreparationStage::SourceChecks, refusal.to_string()).at_line(line)
    }
}

/// Seal every Verilog-A source the deck `source` names, on its own lines and
/// on the lines of the files it includes or selects from a library, reading
/// each through `include_search` from the file that names it: `origin`, the
/// deck's file, for its own lines.
///
/// The sources compile as the engine compiles a deck's HDL. The files no
/// line gives macros (`define=`), and that are not connect-rule libraries,
/// are one compilation unit, read in the order the deck names them, when
/// there are several (IEEE 1364-2005 clause 19), so a module of one may
/// instantiate a module of another; every other file compiles alone. A
/// source's `` `include `` headers are sealed by content with it, so a host
/// with no file system runs what was compiled here.
///
/// A deck that names none is returned as it is. A line inside a false `.if`
/// branch, in a `.lib` section nothing selects, or after the terminal `.end`
/// names nothing and is left alone. A line that names a sealed runtime's key
/// is already sealed. The deck's own lines are rewritten in
/// [`SealedDeckHdl::source`]; an included file's lines are rewritten where
/// an expansion of the deck's includes puts them
/// ([`SealedDeckHdl::included_line`]).
pub fn seal_deck_hdl_sources(
    source: &str,
    origin: Option<&Path>,
    include_search: &IncludeSearchChain,
) -> Result<SealedDeckHdl, DeckHdlRefusal> {
    let own = deck_hdl_sites(source);
    if own.is_empty() && !includes_name_hdl_sources(source, origin, include_search) {
        return Ok(SealedDeckHdl {
            source: source.to_owned(),
            ..SealedDeckHdl::default()
        });
    }
    let deck = DeckContext::read(source, origin, include_search);
    let sites = match &deck.sites {
        // The parse names every line the run reads, in the engine's order;
        // the deck's own keep the spelling they were written with.
        Some(parsed) => parsed
            .iter()
            .filter_map(|site| match &site.owner {
                Some(_) => Some(site.clone()),
                None => own
                    .iter()
                    .find(|(line, _)| *line == site.line)
                    .map(|(line, directive)| Site {
                        line: *line,
                        owner: None,
                        directive: directive.clone(),
                    }),
            })
            .collect::<Vec<_>>(),
        None => own
            .into_iter()
            .map(|(line, directive)| Site {
                line,
                owner: None,
                directive,
            })
            .collect(),
    };
    if sites.is_empty() {
        return Ok(SealedDeckHdl {
            source: source.to_owned(),
            ..SealedDeckHdl::default()
        });
    }
    let owner = origin.map(|origin| {
        super::dependencies::absolute_source_identity(origin)
            .unwrap_or_else(|_| origin.to_path_buf())
    });
    let mut processor = IncludeProcessor::new(owner.as_deref().unwrap_or(Path::new(".")));
    include_search.apply_to(&mut processor);

    let mut reads = Vec::with_capacity(sites.len());
    for site in &sites {
        let read_from = site.owner.as_deref().or(owner.as_deref());
        reads.push(read_source(&mut processor, read_from, site)?);
    }
    let mut sealer = Sealer {
        deck: &deck,
        sets: Vec::new(),
        placed: HashMap::new(),
        runtimes: Vec::new(),
        connections: Vec::new(),
        aliases: BTreeMap::new(),
        rewritten: BTreeMap::new(),
        included: BTreeMap::new(),
        appended: Vec::new(),
    };
    sealer.compile(&mut processor, &sites, &reads)?;
    for (site, read) in sites.iter().zip(&reads) {
        sealer.seal(site, read)?;
    }
    let last = sites.last().expect("a site was sealed");
    let Sealer {
        runtimes,
        connections,
        rewritten,
        included,
        appended,
        ..
    } = sealer;
    let runtimes = PreparedVerilogARuntimeSet::try_with_connections(runtimes, connections)
        .map_err(|error| {
            DeckHdlRefusal::new(
                ElaborationErrorKind::CompileRefusal,
                last.line,
                Path::new(""),
                None,
                error,
            )
            .in_file(last.owner.as_deref())
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
        included,
    })
}

/// Whether a file the deck includes, or a `.lib` section it selects, writes
/// a line that names a Verilog-A source, as the expansion of its includes
/// reads them. A deck with no include, or no file, has none.
fn includes_name_hdl_sources(
    source: &str,
    origin: Option<&Path>,
    include_search: &IncludeSearchChain,
) -> bool {
    let Some(origin) = origin else {
        return false;
    };
    if !super::contains_external_include_directive(source) {
        return false;
    }
    let origin = super::dependencies::absolute_source_identity(origin)
        .unwrap_or_else(|_| origin.to_path_buf());
    let root = canonical(&origin);
    let mut processor = IncludeProcessor::new(&origin);
    include_search.apply_to(&mut processor);
    let mut found = false;
    // An expansion that fails is refused by the same expansion when the deck
    // is prepared; there is nothing to seal from it here.
    let _ = processor.expand_content_rewriting(source, &origin, &mut |location, text| {
        found |= location
            .path
            .as_deref()
            .is_some_and(|path| canonical(path) != root)
            && rspice_core::netlist::parse_veriloga_source_directive(text)
                .is_some_and(|directive| !names_sealed_runtime(&directive.file_path));
        None
    });
    found
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
/// Every line the run reads is sealed where it stands, so a source line
/// left in the expanded deck `expanded` from one of the included files
/// `includes` is one the seal did not read, such as a line in a false `.if`
/// branch. Preparation refuses it either way; counted in the expanded text
/// its line number points nowhere the author can look, so this finds the
/// file and line that hold it. `None` when no such line is left, or when no
/// included file holds it as written.
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
                        "line {number} of '{}': the Verilog-A source '{}' is named on a line \
                         the run's seal did not read, such as one in a false .if branch, and a \
                         run executes sealed sources only",
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
    /// Every line the parse keeps that names a source, in the engine's
    /// order, the deck's own and its included files'; `None` when the deck
    /// does not parse, and its own lines are taken as written.
    sites: Option<Vec<Site>>,
    /// The deck's `.options hdl_libmap`, which a sealed run cannot carry.
    library_map: Option<String>,
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
                sites: None,
                library_map: None,
                masters: Default::default(),
            };
        };
        let root = origin.map(canonical);
        let sites = netlist
            .veriloga_includes
            .iter()
            .filter(|include| !names_sealed_runtime(&include.file_path))
            .map(|include| {
                let owner = include
                    .origin
                    .path
                    .as_deref()
                    .map(canonical)
                    .filter(|path| Some(path) != root.as_ref());
                Site {
                    line: include.origin.line,
                    owner,
                    directive: include.clone(),
                }
            })
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
            sites: Some(sites),
            library_map: netlist.options.hdl_libmap.clone(),
            masters,
        }
    }
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// Read the source one line names, through the chain from the file that
/// names it, `owner`. A deck with no file can name a source only by its full
/// path.
fn read_source(
    processor: &mut IncludeProcessor,
    owner: Option<&Path>,
    site: &Site,
) -> Result<ResolvedIncludeDependency, DeckHdlRefusal> {
    let directive = &site.directive;
    let spelling = directive.file_path.to_string_lossy().into_owned();
    let owner = match owner {
        Some(owner) => owner.to_path_buf(),
        None if directive.file_path.is_absolute() => directive.file_path.clone(),
        None => {
            return Err(site.refusal(
                ElaborationErrorKind::MissingSource,
                &directive.file_path,
                None,
                "the deck has not been saved to a file, so a relative path has no directory to \
                 resolve from; save the deck or name the source by its full path",
            ));
        }
    };
    processor
        .read_verilog_source_with_abort(&owner, site.line, &spelling, &NoAbort)
        .map_err(|error| {
            let detail = match error {
                ParseWithAbortError::Parse(ParseError::Syntax { message, .. }) => message,
                other => other.to_string(),
            };
            site.refusal(
                ElaborationErrorKind::MissingSource,
                &directive.file_path,
                None,
                detail,
            )
        })
}

/// One line that names a Verilog-A source: the deck's own, or a line of a
/// file the deck includes.
#[derive(Debug, Clone)]
struct Site {
    /// The 1-based line, in the deck or in `owner`.
    line: usize,
    /// The included file that writes the line; `None` for the deck's own.
    owner: Option<PathBuf>,
    directive: VerilogAInclude,
}

impl Site {
    fn refusal(
        &self,
        kind: ElaborationErrorKind,
        path: &Path,
        module: Option<&str>,
        detail: impl Into<String>,
    ) -> DeckHdlRefusal {
        DeckHdlRefusal::new(kind, self.line, path, module, detail).in_file(self.owner.as_deref())
    }
}

/// A source file under the `define=` macros of the line that names it.
type SourceUnderMacros = (PathBuf, Vec<(String, Option<String>)>);

/// The files the engine compiles together, prepared once from their sealed
/// bytes for every line that names one of them.
struct CompileSet {
    prepared: rspice_veriloga::PreparedVirtualSource,
    /// What was compiled: every file's name and bytes, the macros and the
    /// deck's `wreal` resolution. The cells and the min:typ:max selection are
    /// part of every runtime's own identity.
    identity: ContentDigest,
    /// The modules each file of the set declares, with what an `include of
    /// its own declares, in the set's order.
    modules: Vec<Vec<String>>,
    /// Whether the set is a compilation unit of several files.
    unit: bool,
}

struct Sealer<'a> {
    deck: &'a DeckContext,
    sets: Vec<CompileSet>,
    /// The set each source is compiled in, and its place in that set.
    placed: HashMap<SourceUnderMacros, (usize, usize)>,
    runtimes: Vec<PreparedVerilogARuntime>,
    connections: Vec<PreparedVerilogAConnectionLibrary>,
    /// Each alias bound so far: its artifact and the line that bound it.
    aliases: BTreeMap<String, (ContentDigest, usize)>,
    /// The sealed directive each rewritten line of the deck now carries.
    rewritten: BTreeMap<usize, String>,
    /// The sealed directive each rewritten line of an included file carries.
    included: BTreeMap<(PathBuf, usize), String>,
    /// Directives for further modules of a line's source.
    appended: Vec<String>,
}

impl Sealer<'_> {
    /// Prepare the deck's sources as the engine groups them: one compile per
    /// file under the macros of the lines that name it, and the files no line
    /// gives macros, and that hold no connect rules, as one unit when there
    /// are several. Each compile reads its files' headers from the host.
    fn compile(
        &mut self,
        processor: &mut IncludeProcessor,
        sites: &[Site],
        reads: &[ResolvedIncludeDependency],
    ) -> Result<(), DeckHdlRefusal> {
        let mut groups: Vec<(SourceUnderMacros, usize)> = Vec::new();
        for (index, (site, read)) in sites.iter().zip(reads).enumerate() {
            let key = (
                read.resolved_path().to_path_buf(),
                site.directive.defines.clone(),
            );
            if !groups.iter().any(|(known, _)| *known == key) {
                groups.push((key, index));
            }
        }
        let (unit, alone): (Vec<_>, Vec<_>) = groups.into_iter().partition(|((_, defines), at)| {
            defines.is_empty() && !declares_connect_items(reads[*at].source())
        });
        if let (Some(map), Some((_, at))) = (&self.deck.library_map, unit.first()) {
            let site = &sites[*at];
            return Err(site.refusal(
                ElaborationErrorKind::CompileRefusal,
                reads[*at].resolved_path(),
                None,
                format!(
                    "the deck's `.options hdl_libmap={map}` places its Verilog files in \
                     libraries, which a run's sealed sources do not carry; run the deck from the \
                     command line"
                ),
            ));
        }
        let (unit, mut alone) = if unit.len() > 1 {
            (unit, alone)
        } else {
            (Vec::new(), [unit, alone].concat())
        };
        alone.sort_by_key(|(_, at)| *at);
        // The engine prepares the files that compile alone first, then the
        // unit; a refusal names the first that fails in that order.
        for (key, at) in alone {
            self.prepare_set(processor, &[(key, at)], sites, reads)?;
        }
        if !unit.is_empty() {
            self.prepare_set(processor, &unit, sites, reads)?;
        }
        Ok(())
    }

    fn prepare_set(
        &mut self,
        processor: &mut IncludeProcessor,
        members: &[(SourceUnderMacros, usize)],
        sites: &[Site],
        reads: &[ResolvedIncludeDependency],
    ) -> Result<(), DeckHdlRefusal> {
        let (first_key, first) = &members[0];
        let site = &sites[*first];
        let defines = &first_key.1;
        let refusal = |detail: String| {
            site.refusal(
                ElaborationErrorKind::CompileRefusal,
                reads[*first].resolved_path(),
                None,
                detail,
            )
        };
        let roots: Vec<closure::HostSource> = members
            .iter()
            .map(|(_, at)| closure::HostSource {
                path: reads[*at].resolved_path().to_path_buf(),
                text: reads[*at].source().to_owned(),
            })
            .collect();
        let sources = closure::read_closure(processor, &roots, defines).map_err(&refusal)?;
        let (bundle, names) = closure::bundle(&sources).map_err(&refusal)?;
        let compiler = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
            external_cells: self.deck.external_cells.clone(),
            delay_select: self.deck.delay_select,
            wreal_resolution: self.deck.wreal_resolution,
            defines: defines.clone(),
            ..crate::compilation::unified_runtime_compiler_options()
        });
        let limits = crate::project_veriloga::project_virtual_compile_limits();
        let unit = members.len() > 1;
        let prepared = if unit {
            let root_names: Vec<&str> = roots
                .iter()
                .map(|root| {
                    let index = sources
                        .iter()
                        .position(|source| source.path == root.path)
                        .expect("every root is read");
                    names[index].as_str()
                })
                .collect();
            compiler.prepare_virtual_unit_runtime_source(&bundle, &root_names, limits)
        } else {
            compiler.prepare_virtual_runtime_source(&bundle, limits)
        }
        .map_err(|failure| refusal(failure.to_string()))?;
        closure::same_closure(&prepared, &names).map_err(&refusal)?;
        let modules = if unit {
            prepared
                .unit_files()
                .iter()
                .map(|file| {
                    file.modules
                        .iter()
                        .map(|module| module.to_string())
                        .collect()
                })
                .collect()
        } else {
            vec![prepared.module_names().map(str::to_owned).collect()]
        };
        let identity = compile_identity(
            &sources,
            &names,
            roots.len(),
            defines,
            self.deck.wreal_resolution,
        );
        let set = self.sets.len();
        for (position, (key, _)) in members.iter().enumerate() {
            self.placed.insert(key.clone(), (set, position));
        }
        self.sets.push(CompileSet {
            prepared,
            identity,
            modules,
            unit,
        });
        Ok(())
    }

    fn seal(
        &mut self,
        site: &Site,
        read: &ResolvedIncludeDependency,
    ) -> Result<(), DeckHdlRefusal> {
        let path = read.resolved_path().to_path_buf();
        let directive = &site.directive;
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("source.va")
            .to_owned();
        let (set_index, position) = self.placed[&(path.clone(), directive.defines.clone())];
        let set = &self.sets[set_index];
        let identity = set.identity;
        let refusal =
            |kind, module: Option<&str>, detail: String| site.refusal(kind, &path, module, detail);
        let modules = set.modules[position].clone();
        if modules.is_empty() && set.unit {
            // A file of a unit may declare no module: the macros and
            // `timescale` the files after it read. It has no model, and its
            // line stays a line.
            if let Some(module) = &directive.selected_module {
                return Err(refusal(
                    ElaborationErrorKind::UnknownModule,
                    None,
                    format!(
                        "this source declares no device module, so module '{module}' cannot be selected"
                    ),
                ));
            }
            self.rewrite(site, format!("* {}", directive_text(site)));
            return Ok(());
        }
        // A source of connect rules and connect modules alone contributes
        // rules, not instances, as the engine reads it.
        if modules.is_empty() && directive.selected_module.is_none() {
            let artifact = set.prepared.connection_artifact().ok_or_else(|| {
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
                .unwrap_or_else(|| connection_alias(&file_name));
            let source_key = deck_source_key(identity, "connections", &alias, &file_name);
            let library = PreparedVerilogAConnectionLibrary::try_new(
                source_key.clone(),
                identity,
                alias.clone(),
                artifact,
            )
            .map_err(|error| refusal(ElaborationErrorKind::CompileRefusal, None, error))?;
            let artifact = library.binding().artifact_digest();
            self.bind_alias(&alias, artifact, site, &path)?;
            if !self.connections.iter().any(|known| {
                known
                    .binding()
                    .source_key()
                    .eq_ignore_ascii_case(&source_key)
            }) {
                self.connections.push(library);
            }
            self.rewrite(site, super::project_veriloga_directive(&source_key, &alias));
            return Ok(());
        }
        let selected = select_modules(directive, &modules, &self.deck.masters)
            .map_err(|detail| refusal(ElaborationErrorKind::ModuleNotSelected, None, detail))?;
        for (index, module) in selected.iter().enumerate() {
            let set = &self.sets[set_index];
            let compilation = set.prepared.compile_runtime(module).map_err(|failure| {
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
            let source_key = deck_source_key(identity, module, &alias, &file_name);
            // Sealed with the deck's cells it was compiled against, which a
            // design with a leaf carries in its identity and install key, and
            // with the selection it was compiled under, which every design
            // does.
            let runtime = PreparedVerilogARuntime::try_from_virtual_compilation_against(
                source_key.clone(),
                identity,
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
            self.bind_alias(&alias, runtime.artifact_digest(), site, &path)?;
            if !self
                .runtimes
                .iter()
                .any(|known| known.source_key().eq_ignore_ascii_case(&source_key))
            {
                self.runtimes.push(runtime);
            }
            let sealed = super::project_veriloga_directive(&source_key, &alias);
            if index == 0 {
                self.rewrite(site, sealed);
            } else {
                self.appended.push(sealed);
            }
        }
        Ok(())
    }

    /// Put `directive` on the line `site` stands on.
    fn rewrite(&mut self, site: &Site, directive: String) {
        match &site.owner {
            None => {
                self.rewritten.insert(site.line, directive);
            }
            Some(owner) => {
                self.included
                    .insert((canonical(owner), site.line), directive);
            }
        }
    }

    /// One deck names one master per alias.
    fn bind_alias(
        &mut self,
        alias: &str,
        artifact: ContentDigest,
        site: &Site,
        path: &Path,
    ) -> Result<(), DeckHdlRefusal> {
        match self.aliases.get(&alias.to_ascii_uppercase()) {
            Some((bound, _)) if *bound == artifact => Ok(()),
            Some((_, first)) => Err(site.refusal(
                ElaborationErrorKind::ModuleNotSelected,
                path,
                None,
                format!(
                    "the name '{alias}' already names a different Verilog-A master, sealed from line {first}"
                ),
            )),
            None => {
                self.aliases
                    .insert(alias.to_ascii_uppercase(), (artifact, site.line));
                Ok(())
            }
        }
    }
}

/// The line a site was written as, for a line that keeps its place as a
/// comment.
fn directive_text(site: &Site) -> String {
    let directive = &site.directive;
    format!(
        ".hdl \"{}\" (sealed into the deck's Verilog unit)",
        directive.file_path.display()
    )
}

/// Whether a source declares connect rules or connect modules, which make it
/// a connection library the engine compiles on its own, never in a unit. A
/// word test, as the engine's own.
fn declares_connect_items(text: &str) -> bool {
    let identifier =
        |character: char| character.is_ascii_alphanumeric() || "_$".contains(character);
    ["connectmodule", "connectrules"].iter().any(|word| {
        text.match_indices(word).any(|(at, _)| {
            !text[..at].chars().next_back().is_some_and(identifier)
                && !text[at + word.len()..]
                    .chars()
                    .next()
                    .is_some_and(identifier)
        })
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

/// The identity of one compile: every file it read, by its name in the
/// sealed bundle and its bytes, how many of them are the roots it compiled,
/// the macros the roots' lines define and the deck's `wreal` resolution. The
/// deck's cells and its min:typ:max selection are part of the sealed
/// runtime's own identity.
fn compile_identity(
    sources: &[closure::HostSource],
    names: &[String],
    roots: usize,
    defines: &[(String, Option<String>)],
    wreal_resolution: DigitalRealResolution,
) -> ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.deck-hdl-source/v2\0");
    let mut field = |bytes: &[u8]| {
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    };
    field(wreal_resolution.keyword().as_bytes());
    field(&(roots as u64).to_le_bytes());
    field(&(sources.len() as u64).to_le_bytes());
    for (source, name) in sources.iter().zip(names) {
        field(name.as_bytes());
        field(source.text.as_bytes());
    }
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

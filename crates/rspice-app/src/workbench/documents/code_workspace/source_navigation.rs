//! What a source navigator shows for the file in front of the reader.
//!
//! Two answers, each from the authority that owns it. The outline of a
//! Verilog-A file comes from the compiler's own parser, so a port reads as
//! one row stating its direction and discipline rather than once per
//! declaration that names it. The file's includes come from the compiler's
//! own preprocessor, resolved in its order — the cell's files first, then
//! the standard headers built into RSpice — so the panel cannot disagree with
//! a compile about which file an include names.
//!
//! Both are derived once per source closure and kept against its digest: a
//! navigator paints every frame, and parsing or preprocessing does not.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rspice_veriloga::ast::{Item, Module};
use rspice_veriloga::{
    PreprocessorError, SourceDocument, SourceDocumentOrigin, SourceProvider, SourceProviderLimits,
    Span,
};

use crate::product::{ContentDigest, ProjectId};
use crate::state::{
    ProjectSourceBundle, ProjectSourceId, ProjectSourceLanguage, ProjectSourceRole,
};

use super::language_services::{LanguageServiceIndex, SourceSymbolKind};

/// The kinds a Verilog-A outline groups its rows into, in the order the
/// groups are shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SourceOutlineGroup {
    Module,
    Ports,
    Parameters,
    Variables,
    Nets,
    Functions,
}

/// One row of a source file's outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceOutlineRow {
    /// The group a Verilog-A row belongs to. An Automation row has none.
    pub group: Option<SourceOutlineGroup>,
    pub name: String,
    /// What the declaration says about `name`, in the source's own words:
    /// `inout electrical`, `real = 1m from (0:inf)`.
    pub detail: String,
    /// One-based line of the declaration, ready for the editor's go-to-line
    /// request.
    pub line: usize,
}

/// Where one `` `include `` of a file resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceIncludeTarget {
    /// A file of the cell's own bundle, by its logical path.
    Project(String),
    /// A standard header built into RSpice, by its canonical identity
    /// (`@rspice/stdlib/disciplines.vams`).
    BuiltIn(&'static str),
    /// Nothing: neither the bundle nor the built-in headers hold it. The
    /// directive is in `including`, on `line`.
    Missing { including: String, line: usize },
}

/// One include of a file's closure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceInclude {
    /// The name the directive wrote.
    pub requested: String,
    pub target: SourceIncludeTarget,
}

/// The navigator's view of one source file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SourceNavigation {
    pub outline: Vec<SourceOutlineRow>,
    /// Every include the file pulls in, its own and those of the project
    /// files it includes, in the order the preprocessor reaches them.
    pub includes: Vec<SourceInclude>,
}

/// The language index for one bundle and the navigation derived for its
/// files, retained against the exact closure they were built from.
///
/// Keying on the closure digest rather than a revision counter means an edit
/// that produces identical bytes reuses all of it instead of re-parsing the
/// tree.
#[derive(Debug, Clone)]
pub(crate) struct SourceIndexCache {
    bundle_id: ProjectSourceId,
    closure_digest: ContentDigest,
    /// `None` when the closure is past the language index's size bound.
    index: Option<LanguageServiceIndex>,
    /// Navigation per file, by path key, derived the first time it is shown.
    documents: BTreeMap<String, SourceNavigation>,
}

/// The navigation of `logical_path`, deriving it only when it must.
///
/// Nothing falls back to the cache of whichever bundle was shown before: two
/// cell views can share a logical path, and the outline of one is not the
/// outline of the other.
pub(crate) fn document_navigation(
    project_id: ProjectId,
    bundle: &ProjectSourceBundle,
    cache: &mut Option<SourceIndexCache>,
    logical_path: &str,
) -> SourceNavigation {
    let current = cache.as_ref().is_some_and(|cache| {
        cache.bundle_id == bundle.id() && cache.closure_digest == bundle.closure_digest()
    });
    if !current {
        *cache = Some(SourceIndexCache {
            bundle_id: bundle.id(),
            closure_digest: bundle.closure_digest(),
            index: LanguageServiceIndex::build(project_id, bundle).ok(),
            documents: BTreeMap::new(),
        });
    }
    let Some(cache) = cache.as_mut() else {
        return SourceNavigation::default();
    };
    let index = cache.index.as_ref();
    cache
        .documents
        .entry(crate::state::project_source_path_key(logical_path))
        .or_insert_with(|| derive_navigation(bundle, index, logical_path))
        .clone()
}

fn derive_navigation(
    bundle: &ProjectSourceBundle,
    index: Option<&LanguageServiceIndex>,
    logical_path: &str,
) -> SourceNavigation {
    match bundle.language() {
        ProjectSourceLanguage::VerilogA => {
            // The build profile is policy, not Verilog-A: it has no outline
            // and includes nothing.
            if bundle.role_for_path(logical_path) == Some(ProjectSourceRole::VerilogABuildProfile) {
                return SourceNavigation::default();
            }
            let Some(source) = bundle.file_content(logical_path) else {
                return SourceNavigation::default();
            };
            SourceNavigation {
                outline: veriloga_outline(source)
                    .unwrap_or_else(|| indexed_veriloga_outline(index, logical_path)),
                includes: veriloga_includes(bundle, logical_path),
            }
        }
        ProjectSourceLanguage::RSpiceAutomation => SourceNavigation {
            outline: index
                .map(|index| {
                    index
                        .document_symbols(logical_path)
                        .into_iter()
                        .map(|symbol| SourceOutlineRow {
                            group: None,
                            name: symbol.name.clone(),
                            detail: symbol.signature.clone(),
                            line: symbol.line,
                        })
                        .collect()
                })
                .unwrap_or_default(),
            includes: Vec::new(),
        },
    }
}

/// The source text of a canonical built-in header identity, or of any name
/// an include may spell it by.
pub(crate) fn builtin_header(identity: &str) -> Option<(&'static str, &'static str)> {
    rspice_veriloga::stdlib::builtin_include_document(identity)
}

// ---------------------------------------------------------------------------
// Outline
// ---------------------------------------------------------------------------

/// The outline the compiler's parser reads from `source`, or `None` when it
/// cannot read the file.
///
/// The parser reads the file's own text, so every row's line is the file's
/// line and every detail is quoted as the author wrote it. A file mid-edit,
/// or one whose declarations are themselves macros, does not parse; the
/// indexed outline stands in for it then, names without detail, rather than
/// the outline going blank.
fn veriloga_outline(source: &str) -> Option<Vec<SourceOutlineRow>> {
    let source_id = rspice_veriloga::SourceMap::new().add_source("outline", String::new());
    let readable = without_preprocessor_syntax(source);
    let tokens = rspice_veriloga::Lexer::new(&readable, source_id)
        .collect_tokens()
        .ok()?;
    let file = rspice_veriloga::Parser::new(&tokens).parse().ok()?;
    let text = SourceText::new(source);
    let mut rows = Vec::new();
    for item in &file.items {
        if let Item::Module(module) | Item::ConnectModule(module) = item {
            module_outline(module, &text, &mut rows);
        }
    }
    // Groups in their fixed order; rows keep the order they were declared in.
    rows.sort_by_key(|row| row.group);
    Some(rows)
}

fn module_outline(module: &Module, text: &SourceText<'_>, rows: &mut Vec<SourceOutlineRow>) {
    // Names are unique within a module, not across a file's modules.
    let first = rows.len();
    let mut push = |group, name: &str, detail: String, span: Span| {
        if text.authored(span)
            && !rows[first..]
                .iter()
                .any(|row| row.group == Some(group) && row.name == name)
        {
            rows.push(SourceOutlineRow {
                group: Some(group),
                name: name.to_owned(),
                detail,
                line: text.line(span),
            });
        }
    };
    push(
        SourceOutlineGroup::Module,
        &module.name,
        String::new(),
        module.span,
    );

    let is_port = |name: &str| module.ports.iter().any(|port| port.name == name);
    // A port is one row however many declarations name it: `inout p` gives
    // it a direction and `electrical p` a discipline.
    for port in &module.ports {
        let declaration = module
            .port_declarations
            .iter()
            .find(|declaration| declaration.names.contains(&port.name));
        let mut words = Vec::new();
        if let Some(declaration) = declaration {
            words.push(declaration.direction.label().to_owned());
            if let Some(net_type) = declaration.net_type {
                words.push(port_net_type(net_type).to_owned());
            }
        }
        let discipline = declaration
            .and_then(|declaration| declaration.discipline.clone())
            .or_else(|| {
                module
                    .nets
                    .iter()
                    .filter(|net| net.names.contains(&port.name))
                    .find_map(|net| net.discipline.clone())
            });
        words.extend(discipline.map(|discipline| discipline.to_string()));
        if let Some(range) = declaration.and_then(|declaration| declaration.range.as_ref()) {
            words.push(text.slice(range.span).to_owned());
        }
        let span = declaration.map_or(port.span, |declaration| declaration.span);
        push(SourceOutlineGroup::Ports, &port.name, words.join(" "), span);
    }

    let mut parameters = module
        .parameters
        .iter()
        .map(|parameter| (parameter, "parameter"))
        .chain(
            module
                .localparams
                .iter()
                .map(|parameter| (parameter, "localparam")),
        )
        .collect::<Vec<_>>();
    parameters.sort_by_key(|(parameter, _)| parameter.span.start);
    for (parameter, keyword) in parameters {
        let mut words = Vec::new();
        if keyword == "localparam" {
            words.push(keyword.to_owned());
        }
        words.push(
            match parameter.param_type {
                rspice_veriloga::ast::ParamType::Real => "real",
                rspice_veriloga::ast::ParamType::Integer => "integer",
                rspice_veriloga::ast::ParamType::String => "string",
            }
            .to_owned(),
        );
        if let Some(default) = &parameter.default {
            words.push(format!("= {}", text.slice(default.span())));
        }
        if let Some(range) = &parameter.range {
            words.push(text.slice(range.span).to_owned());
        }
        push(
            SourceOutlineGroup::Parameters,
            &parameter.name,
            words.join(" "),
            parameter.span,
        );
    }
    for alias in &module.aliasparams {
        push(
            SourceOutlineGroup::Parameters,
            &alias.alias,
            format!("aliasparam = {}", alias.target),
            alias.span,
        );
    }

    for declaration in &module.variables {
        let kind = match declaration.var_type {
            rspice_veriloga::ast::VarType::Real => "real",
            rspice_veriloga::ast::VarType::Integer => "integer",
            rspice_veriloga::ast::VarType::String => "string",
        };
        for item in declaration.items.iter().filter(|item| !is_port(&item.name)) {
            push(
                SourceOutlineGroup::Variables,
                &item.name,
                kind.to_owned(),
                item.span,
            );
        }
    }
    for declaration in &module.digital_variables {
        let detail = keyword_with_range(text, declaration.span, declaration.range.as_ref());
        for item in declaration.items.iter().filter(|item| !is_port(&item.name)) {
            push(
                SourceOutlineGroup::Variables,
                &item.name,
                detail.clone(),
                declaration.span,
            );
        }
    }

    // A net is one row too: `electrical gnd` and `ground gnd` both describe
    // the same node.
    let mut nets: Vec<(&str, Vec<String>, Span)> = Vec::new();
    for declaration in &module.nets {
        let mut words = Vec::new();
        words.extend(
            declaration
                .discipline
                .as_ref()
                .map(|discipline| discipline.to_string()),
        );
        if declaration.is_ground {
            words.push("ground".to_owned());
        }
        if let Some(range) = &declaration.range {
            words.push(text.slice(range.span).to_owned());
        }
        for name in declaration.names.iter().filter(|name| !is_port(name)) {
            match nets
                .iter_mut()
                .find(|(known, _, _)| *known == name.as_str())
            {
                Some((_, known_words, _)) => {
                    for word in &words {
                        if !known_words.contains(word) {
                            known_words.push(word.clone());
                        }
                    }
                }
                None => nets.push((name, words.clone(), declaration.span)),
            }
        }
    }
    for declaration in &module.digital_nets {
        let detail = keyword_with_range(text, declaration.span, declaration.range.as_ref());
        for item in declaration.items.iter().filter(|item| !is_port(&item.name)) {
            nets.push((&item.name, vec![detail.clone()], declaration.span));
        }
    }
    for branch in &module.branches {
        let terminals = if branch.neg.is_empty() {
            format!("({})", branch.pos)
        } else {
            format!("({}, {})", branch.pos, branch.neg)
        };
        nets.push((&branch.name, vec![terminals], branch.span));
    }
    nets.sort_by_key(|(_, _, span)| span.start);
    for (name, words, span) in nets {
        push(SourceOutlineGroup::Nets, name, words.join(" "), span);
    }

    for function in &module.functions {
        let kind = match function.return_type {
            rspice_veriloga::ast::VarType::Real => "real",
            rspice_veriloga::ast::VarType::Integer => "integer",
            rspice_veriloga::ast::VarType::String => "string",
        };
        let arguments = function
            .params
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        push(
            SourceOutlineGroup::Functions,
            &function.name,
            format!("{kind} ({arguments})"),
            function.span,
        );
    }
}

/// `source` with the preprocessor's syntax made inert, byte for byte, so the
/// parser can read a file the preprocessor has not expanded and every span
/// it reports still points into `source`.
///
/// A directive line, with the lines a `` `define `` continues onto, becomes
/// blank. A macro use becomes an identifier of the same length — `` `P_Q ``
/// reads as `_P_Q`, and `` `MAX(a, b) `` as a call — which is all a
/// declaration's outline needs from an expression: where it starts and ends.
fn without_preprocessor_syntax(source: &str) -> String {
    const DIRECTIVES: [&str; 13] = [
        "include",
        "define",
        "undef",
        "ifdef",
        "ifndef",
        "else",
        "elsif",
        "endif",
        "timescale",
        "resetall",
        "default_nettype",
        "default_transition",
        "default_discipline",
    ];
    let blank = |line: &str, out: &mut String| {
        for character in line.chars() {
            if matches!(character, '\n' | '\r') {
                out.push(character);
            } else {
                out.extend(std::iter::repeat_n(' ', character.len_utf8()));
            }
        }
    };
    let mut out = String::with_capacity(source.len());
    let mut continued = false;
    for line in source.split_inclusive('\n') {
        let directive = line
            .trim_start()
            .strip_prefix('`')
            .and_then(|rest| {
                rest.split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                    .next()
            })
            .is_some_and(|name| DIRECTIVES.contains(&name));
        if continued || directive {
            blank(line, &mut out);
            continued = (continued || line.trim_start().starts_with("`define"))
                && line.trim_end().ends_with('\\');
        } else {
            out.extend(
                line.chars()
                    .map(|character| if character == '`' { '_' } else { character }),
            );
        }
    }
    out
}

const fn port_net_type(net_type: rspice_veriloga::ast::PortNetType) -> &'static str {
    use rspice_veriloga::ast::PortNetType;
    match net_type {
        PortNetType::Wire => "wire",
        PortNetType::Reg => "reg",
        PortNetType::Integer => "integer",
        PortNetType::Wreal(resolution) => {
            rspice_veriloga::ast::DigitalNetKind::Wreal(resolution).keyword()
        }
        PortNetType::Real => "real",
    }
}

/// A discrete declaration's keyword as written, with its vector range.
fn keyword_with_range(
    text: &SourceText<'_>,
    span: Span,
    range: Option<&rspice_veriloga::ast::VectorRange>,
) -> String {
    let keyword = text
        .slice(span)
        .split_whitespace()
        .next()
        .unwrap_or_default();
    range.map_or_else(
        || keyword.to_owned(),
        |range| format!("{keyword} {}", text.slice(range.span)),
    )
}

/// The index's outline of a Verilog-A file the parser could not read: one row
/// per declared name, grouped the same way, without detail.
fn indexed_veriloga_outline(
    index: Option<&LanguageServiceIndex>,
    logical_path: &str,
) -> Vec<SourceOutlineRow> {
    let Some(index) = index else {
        return Vec::new();
    };
    let symbols = index.document_symbols(logical_path);
    let mut rows: Vec<SourceOutlineRow> = Vec::new();
    for symbol in &symbols {
        let group = match symbol.kind {
            SourceSymbolKind::Module => SourceOutlineGroup::Module,
            SourceSymbolKind::Port => SourceOutlineGroup::Ports,
            SourceSymbolKind::Parameter => SourceOutlineGroup::Parameters,
            SourceSymbolKind::Variable => SourceOutlineGroup::Variables,
            SourceSymbolKind::Net
                if symbols.iter().any(|other| {
                    other.kind == SourceSymbolKind::Port && other.name == symbol.name
                }) =>
            {
                continue;
            }
            SourceSymbolKind::Net => SourceOutlineGroup::Nets,
            SourceSymbolKind::Function => SourceOutlineGroup::Functions,
            _ => continue,
        };
        if !rows
            .iter()
            .any(|row| row.group == Some(group) && row.name == symbol.name)
        {
            rows.push(SourceOutlineRow {
                group: Some(group),
                name: symbol.name.clone(),
                detail: String::new(),
                line: symbol.line,
            });
        }
    }
    rows.sort_by_key(|row| row.group);
    rows
}

/// Byte spans to text and lines.
struct SourceText<'a> {
    source: &'a str,
    line_starts: Vec<usize>,
}

impl<'a> SourceText<'a> {
    fn new(source: &'a str) -> Self {
        let line_starts = std::iter::once(0)
            .chain(source.match_indices('\n').map(|(index, _)| index + 1))
            .collect();
        Self {
            source,
            line_starts,
        }
    }

    /// Whether `span` is text the author wrote. The parser synthesizes
    /// declarations — a port's implied net, a generate loop's copies — with
    /// empty spans, and those are not rows of their own.
    fn authored(&self, span: Span) -> bool {
        span.end > span.start && (span.end as usize) <= self.source.len()
    }

    fn slice(&self, span: Span) -> &'a str {
        self.source
            .get(span.start as usize..span.end as usize)
            .unwrap_or_default()
            .trim()
    }

    /// One-based line of the start of `span`.
    fn line(&self, span: Span) -> usize {
        self.line_starts
            .partition_point(|start| *start <= span.start as usize)
            .max(1)
    }
}

// ---------------------------------------------------------------------------
// Includes
// ---------------------------------------------------------------------------

/// Every include `logical_path` pulls in and where each one resolved.
///
/// The preprocessor walks the file as a compile would, under the build
/// profile's include paths and definitions, so a directive a conditional
/// leaves out is not listed. A name neither the bundle nor the built-in
/// headers hold would stop a compile at the first one; here it is recorded
/// and stood in for by an empty document, so every missing include is listed
/// rather than only the first.
fn veriloga_includes(bundle: &ProjectSourceBundle, logical_path: &str) -> Vec<SourceInclude> {
    let profile = super::veriloga_profile::resolve_veriloga_build_profile(bundle)
        .ok()
        .map(|resolved| resolved.profile);
    let provider = BundleIncludeProvider::new(
        bundle,
        profile
            .as_ref()
            .map_or(&[][..], |profile| profile.include_paths.as_slice()),
    );
    let mut preprocessor = rspice_veriloga::Preprocessor::new();
    if let Some(profile) = &profile {
        for name in &profile.preprocessor.undefines {
            preprocessor.undefine(name);
        }
        for (name, value) in &profile.preprocessor.defines {
            preprocessor.define(
                name.clone(),
                rspice_veriloga::preprocessor::MacroDef::simple(value.clone()),
            );
        }
    }
    // A preprocessing error ends the walk where the compile would end it; the
    // includes reached before it are still the file's.
    let _ = preprocessor.preprocess_provider_root(&provider, Path::new(logical_path));
    let missing = provider.missing.into_inner();
    let builtin = preprocessor
        .dependency_documents()
        .iter()
        .filter(|document| document.origin == SourceDocumentOrigin::BuiltIn)
        .map(|document| logical(&document.logical_path))
        .collect::<Vec<_>>();
    let mut includes: Vec<SourceInclude> = Vec::new();
    for edge in preprocessor.take_include_graph() {
        let included = logical(&edge.included_path);
        let target = if let Some(including) = missing.get(&included) {
            SourceIncludeTarget::Missing {
                line: bundle
                    .file_content(including)
                    .and_then(|source| include_directive_line(source, edge.include_index))
                    .unwrap_or(1),
                including: including.clone(),
            }
        } else if builtin.contains(&included) {
            match builtin_header(&included) {
                Some((identity, _)) => SourceIncludeTarget::BuiltIn(identity),
                None => continue,
            }
        } else {
            SourceIncludeTarget::Project(included)
        };
        // A file reached twice is one include of the closure; a missing name
        // is listed at every directive that writes it, because each is a line
        // to fix.
        let repeated = !matches!(target, SourceIncludeTarget::Missing { .. })
            && includes.iter().any(|include| include.target == target);
        if !repeated {
            includes.push(SourceInclude {
                requested: edge.requested_path,
                target,
            });
        }
    }
    includes
}

fn logical(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// The bundle's files as the compile's virtual source provider resolves
/// them: beside the including file, then along the build profile's include
/// paths, then from the bundle root. Paths compare case-insensitively, as
/// there.
struct BundleIncludeProvider<'a> {
    files: BTreeMap<String, (&'a str, &'a str)>,
    include_paths: Vec<String>,
    /// Stand-in path of each missing include, to the file that wrote it.
    missing: RefCell<BTreeMap<String, String>>,
}

impl<'a> BundleIncludeProvider<'a> {
    fn new(bundle: &'a ProjectSourceBundle, include_paths: &[String]) -> Self {
        let files = std::iter::once((bundle.root().logical_path(), bundle.root().content()))
            .chain(
                bundle
                    .files()
                    .iter()
                    // The compile leaves the build profile out of the closure.
                    .filter(|file| {
                        bundle.role_for_path(file.logical_path())
                            != Some(ProjectSourceRole::VerilogABuildProfile)
                    })
                    .map(|file| (file.logical_path(), file.content())),
            )
            .map(|(path, source)| (fold(path), (path, source)))
            .collect();
        Self {
            files,
            include_paths: include_paths.to_vec(),
            missing: RefCell::new(BTreeMap::new()),
        }
    }

    fn document(&self, path: &str) -> Option<SourceDocument> {
        self.files
            .get(&fold(path))
            .map(|(path, source)| SourceDocument::provided(*path, *source))
    }
}

fn fold(path: &str) -> String {
    path.chars().flat_map(char::to_lowercase).collect()
}

/// `requested` resolved against `parent` the way the virtual provider
/// resolves it, or `None` for a name it would refuse.
fn resolve_logical(parent: Option<&str>, requested: &str) -> Option<String> {
    let absolute = requested.starts_with('/')
        || requested.starts_with('\\')
        || requested.as_bytes().get(1) == Some(&b':');
    if requested.is_empty() || requested.contains('\0') || absolute {
        return None;
    }
    let mut components = parent
        .map(|parent| parent.split('/').collect::<Vec<_>>())
        .unwrap_or_default();
    let requested = requested.replace('\\', "/");
    for component in requested.split('/') {
        match component {
            "" => return None,
            "." => {}
            ".." => {
                components.pop()?;
            }
            component => components.push(component),
        }
    }
    (!components.is_empty()).then(|| components.join("/"))
}

impl SourceProvider for BundleIncludeProvider<'_> {
    fn load_root(&self, requested: &Path) -> Result<SourceDocument, PreprocessorError> {
        let requested = logical(requested);
        self.document(&requested).ok_or_else(|| {
            PreprocessorError::new(
                format!("Source not in the bundle: {requested}"),
                Some(PathBuf::from(requested)),
                0,
            )
        })
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        _include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<SourceDocument>, PreprocessorError> {
        let including = including_file.map(logical).unwrap_or_default();
        let parent = including.rsplit_once('/').map(|(parent, _)| parent);
        let found = parent
            .map(|parent| resolve_logical(Some(parent), requested))
            .into_iter()
            .chain(
                self.include_paths
                    .iter()
                    .map(|path| resolve_logical(Some(path), requested)),
            )
            .chain(std::iter::once(resolve_logical(None, requested)))
            .flatten()
            .find_map(|candidate| self.document(&candidate));
        if found.is_some() {
            return Ok(found);
        }
        // Declining hands the name to the built-in headers.
        if builtin_header(requested).is_some() {
            return Ok(None);
        }
        let mut missing = self.missing.borrow_mut();
        let stand_in = format!("\0missing/{}/{requested}", missing.len());
        missing.insert(stand_in.clone(), including);
        Ok(Some(SourceDocument::provided(stand_in, String::new())))
    }

    fn limits(&self) -> SourceProviderLimits {
        let limits = rspice_simulation::project_veriloga::project_virtual_compile_limits();
        SourceProviderLimits {
            max_dependencies: limits.max_files.saturating_mul(2),
            max_total_source_bytes: limits.max_total_source_bytes.saturating_mul(2),
            max_include_depth: limits.max_include_depth,
            max_expanded_bytes: limits.max_expanded_bytes,
        }
    }
}

/// One-based line of the `include_index`-th `` `include `` directive in
/// `source`, counted the way the preprocessor counts it: after comments are
/// removed, every directive line, including those a conditional skips.
fn include_directive_line(source: &str, include_index: usize) -> Option<usize> {
    strip_comments(source)
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            line.trim()
                .strip_prefix('`')
                .and_then(|rest| rest.split(char::is_whitespace).next())
                == Some("include")
        })
        .nth(include_index)
        .map(|(line, _)| line + 1)
}

/// `source` with its comments removed and its lines kept, as the
/// preprocessor removes them before it reads a directive.
fn strip_comments(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut in_string = false;
    while let Some(character) = chars.next() {
        if character == '\n' {
            out.push(character);
            in_string = false;
            continue;
        }
        if in_string {
            out.push(character);
            if character == '\\' {
                if let Some(next) = chars.next() {
                    out.push(next);
                }
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match (character, chars.peek()) {
            ('"', _) => {
                in_string = true;
                out.push(character);
            }
            ('/', Some('/')) => {
                while chars.peek().is_some_and(|next| *next != '\n') {
                    chars.next();
                }
            }
            ('/', Some('*')) => {
                chars.next();
                let mut previous = '\0';
                for next in chars.by_ref() {
                    if next == '\n' {
                        out.push('\n');
                    }
                    if previous == '*' && next == '/' {
                        break;
                    }
                    previous = next;
                }
            }
            _ => out.push(character),
        }
    }
    out
}

#[cfg(test)]
mod tests;

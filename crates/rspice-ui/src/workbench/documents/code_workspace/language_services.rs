//! Revision-bound language index for project-owned Verilog-A and Automation sources.
//!
//! The compiler and managed Python worker remain the syntax and semantic
//! authorities. This index supplies bounded source navigation and conservative
//! refactors: it never edits comments/strings, and rename is refused whenever a
//! symbol has multiple project-owned definitions.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use crate::product::{ContentDigest, ProjectId};
use crate::state::{
    ProjectSourceBundle, ProjectSourceId, ProjectSourceLanguage, ProjectSourceOwner,
};
use crate::workbench::RSpiceApp;

const MAX_INDEX_BYTES: usize = 16 * 1024 * 1024;
const MAX_INDEX_TOKENS: usize = 500_000;
const MAX_RENAME_EDITS: usize = 50_000;
const MAX_COMPLETIONS: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SourceSymbolKind {
    Module,
    Function,
    Class,
    Parameter,
    Port,
    Net,
    Variable,
    Import,
    Configuration,
    Discipline,
    Nature,
}

impl SourceSymbolKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Function => "function",
            Self::Class => "class",
            Self::Parameter => "parameter",
            Self::Port => "port",
            Self::Net => "net",
            Self::Variable => "variable",
            Self::Import => "import",
            Self::Configuration => "configuration",
            Self::Discipline => "discipline",
            Self::Nature => "nature",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceSymbol {
    pub name: String,
    pub kind: SourceSymbolKind,
    pub logical_path: String,
    pub byte_range: Range<usize>,
    pub line: usize,
    pub column: usize,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SymbolOccurrence {
    pub name: String,
    pub logical_path: String,
    pub byte_range: Range<usize>,
    pub line: usize,
    pub column: usize,
    pub definition: bool,
    pub line_preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LanguageCompletion {
    pub label: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceCodeActionKind {
    QuickFix,
    Format,
}

impl SourceCodeActionKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::QuickFix => "quick fix",
            Self::Format => "format",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceCodeAction {
    pub title: String,
    pub kind: SourceCodeActionKind,
    pub logical_path: String,
    replacement: String,
}

#[derive(Debug, Clone)]
struct IndexedToken {
    name: String,
    logical_path: String,
    byte_range: Range<usize>,
    line: usize,
    column: usize,
    definition: bool,
    renamable: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct LanguageServiceIndex {
    pub project_id: ProjectId,
    pub bundle_id: ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: ContentDigest,
    pub language: ProjectSourceLanguage,
    symbols: Vec<SourceSymbol>,
    tokens: Vec<IndexedToken>,
    sources: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum LanguageToolView {
    #[default]
    DocumentSymbols,
    Completion,
    Hover,
    SignatureHelp,
    Definitions,
    Declarations,
    References,
    WorkspaceSymbols,
    CodeActions,
}

impl LanguageToolView {
    /// Every view the index can answer, in the order the tools surface offers
    /// them: what the document declares, what the caret is on, then what the
    /// whole project says about it.
    pub(crate) const ALL: [Self; 9] = [
        Self::DocumentSymbols,
        Self::Completion,
        Self::Hover,
        Self::SignatureHelp,
        Self::Definitions,
        Self::Declarations,
        Self::References,
        Self::WorkspaceSymbols,
        Self::CodeActions,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::DocumentSymbols => "Document symbols",
            Self::Completion => "Completion",
            Self::Hover => "Hover",
            Self::SignatureHelp => "Signature help",
            Self::Definitions => "Definitions",
            Self::Declarations => "Declarations",
            Self::References => "References",
            Self::WorkspaceSymbols => "Workspace symbols",
            Self::CodeActions => "Code actions",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CodeLanguageToolsState {
    pub index: LanguageServiceIndex,
    pub active_path: String,
    pub caret_char_index: usize,
    pub query: String,
    pub replacement: String,
    pub view: LanguageToolView,
    pub confirm_rename: bool,
    pub status: Option<String>,
    pub error: Option<String>,
}

pub(crate) fn open_language_tools(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    active_path: &str,
    caret_char_index: usize,
) -> Result<(), String> {
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(language))
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    let bundle_id = bundle.id();
    open_language_tools_in_bundle(app, language, bundle_id, active_path, caret_char_index)
}

pub(crate) fn open_language_tools_in_bundle(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    active_path: &str,
    caret_char_index: usize,
) -> Result<(), String> {
    if !super::source_bundle_contains_document(app, language, bundle_id, active_path) {
        return Err(format!(
            "The active source file '{active_path}' no longer belongs to this project source bundle."
        ));
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(bundle_id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    let active_path = bundle
        .retained_logical_path(active_path)
        .ok_or_else(|| {
            format!(
                "The active source file '{active_path}' no longer belongs to this project source bundle."
            )
        })?
        .to_owned();
    let index = LanguageServiceIndex::build(app.state.workspace.project.id(), bundle)?;
    let query = index
        .symbol_at(&active_path, caret_char_index)
        .unwrap_or_default()
        .to_owned();
    app.state.ui.code_workspace.language_tools = Some(CodeLanguageToolsState {
        index,
        active_path,
        caret_char_index,
        query,
        replacement: String::new(),
        view: LanguageToolView::DocumentSymbols,
        confirm_rename: false,
        status: None,
        error: None,
    });
    Ok(())
}

pub(crate) fn commit_language_rename(app: &mut RSpiceApp) -> Result<String, String> {
    let state = current_language_transaction(app)?;
    let replacements = state
        .index
        .rename_preview(&state.query, &state.replacement)?;
    if let Some((logical_path, _)) = replacements.iter().find(|(logical_path, _)| {
        !super::source_bundle_document_is_editable(
            app,
            state.index.language,
            state.index.bundle_id,
            logical_path,
        )
    }) {
        return Err(format!(
            "Rename would change governed read-only source '{logical_path}'. No project source was changed."
        ));
    }
    let edit_count = state.index.references(&state.query).len();
    let changed_files = app
        .state
        .workspace
        .replace_project_source_bundle_files_transactionally(state.index.bundle_id, replacements)
        .map_err(|error| error.to_string())?;
    match state.index.language {
        ProjectSourceLanguage::VerilogA => super::veriloga::invalidate_veriloga_evidence(app),
        ProjectSourceLanguage::RSpiceAutomation => {
            super::automation::invalidate_automation_evidence(app);
        }
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(state.index.bundle_id)
        .ok_or_else(|| "The renamed source bundle could not be reloaded.".to_owned())?;
    let index = LanguageServiceIndex::build(app.state.workspace.project.id(), bundle)?;
    let message = format!(
        "Renamed '{}' to '{}' at {} indexed occurrence{} across {} project source file{}.",
        state.query,
        state.replacement,
        edit_count,
        if edit_count == 1 { "" } else { "s" },
        changed_files,
        if changed_files == 1 { "" } else { "s" }
    );
    app.state.ui.code_workspace.language_tools = Some(CodeLanguageToolsState {
        index,
        active_path: state.active_path,
        caret_char_index: state.caret_char_index,
        query: state.replacement,
        replacement: String::new(),
        view: LanguageToolView::References,
        confirm_rename: false,
        status: Some(message.clone()),
        error: None,
    });
    app.state
        .push_user_message(crate::diagnostics::ConsoleMessage::info(message.clone()));
    Ok(message)
}

pub(crate) fn commit_language_completion(
    app: &mut RSpiceApp,
    completion: &str,
) -> Result<String, String> {
    let state = current_language_transaction(app)?;
    if !valid_identifier(completion) {
        return Err("The selected completion is not an insertable identifier.".to_owned());
    }
    let replacement = state.index.completion_replacement(
        &state.active_path,
        state.caret_char_index,
        completion,
    )?;
    replace_one_language_document(app, &state, &state.active_path, replacement)?;
    let message = format!(
        "Inserted completion '{completion}' into '{}'.",
        state.active_path
    );
    app.state
        .push_user_message(crate::diagnostics::ConsoleMessage::info(message.clone()));
    Ok(message)
}

pub(crate) fn commit_language_code_action(
    app: &mut RSpiceApp,
    action: &SourceCodeAction,
) -> Result<String, String> {
    let state = current_language_transaction(app)?;
    if !crate::state::project_source_paths_equal(&action.logical_path, &state.active_path) {
        return Err("The code action does not target the active source document.".to_owned());
    }
    let current = state
        .index
        .source_for_path(&action.logical_path)
        .ok_or_else(|| "The code-action source is no longer indexed.".to_owned())?;
    if current == &action.replacement {
        return Ok("The active source already satisfies this code action.".to_owned());
    }
    replace_one_language_document(
        app,
        &state,
        &action.logical_path,
        action.replacement.clone(),
    )?;
    let message = format!("Applied {}: {}.", action.kind.label(), action.title);
    app.state
        .push_user_message(crate::diagnostics::ConsoleMessage::info(message.clone()));
    Ok(message)
}

fn current_language_transaction(app: &RSpiceApp) -> Result<CodeLanguageToolsState, String> {
    let state = app
        .state
        .ui
        .code_workspace
        .language_tools
        .clone()
        .ok_or_else(|| "No source-language transaction is open.".to_owned())?;
    if let Some(reason) = super::source_file_mutation_block_reason(app, state.index.language) {
        return Err(reason.to_owned());
    }
    if !super::source_bundle_document_is_editable(
        app,
        state.index.language,
        state.index.bundle_id,
        &state.active_path,
    ) {
        return Err(format!(
            "'{}' is governed as a read-only source. Create an editable project copy before applying source changes.",
            state.active_path
        ));
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(state.index.bundle_id)
        .ok_or_else(|| "The indexed source bundle no longer exists.".to_owned())?;
    if !state
        .index
        .is_current(app.state.workspace.project.id(), bundle)
    {
        return Err(
            "The source bundle changed after the language transaction was opened. Reindex the current revision."
                .to_owned(),
        );
    }
    Ok(state)
}

fn replace_one_language_document(
    app: &mut RSpiceApp,
    state: &CodeLanguageToolsState,
    logical_path: &str,
    replacement: String,
) -> Result<(), String> {
    app.state
        .workspace
        .replace_project_source_bundle_file(state.index.bundle_id, logical_path, replacement)
        .map_err(|error| error.to_string())?;
    match state.index.language {
        ProjectSourceLanguage::VerilogA => super::veriloga::invalidate_veriloga_evidence(app),
        ProjectSourceLanguage::RSpiceAutomation => {
            super::automation::invalidate_automation_evidence(app);
        }
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(state.index.bundle_id)
        .ok_or_else(|| "The changed source bundle could not be reloaded.".to_owned())?;
    let index = LanguageServiceIndex::build(app.state.workspace.project.id(), bundle)?;
    if let Some(tools) = app.state.ui.code_workspace.language_tools.as_mut() {
        tools.index = index;
        tools.confirm_rename = false;
        tools.error = None;
    }
    Ok(())
}

impl LanguageServiceIndex {
    pub(crate) fn build(
        project_id: ProjectId,
        bundle: &ProjectSourceBundle,
    ) -> Result<Self, String> {
        let documents = bundle_documents(bundle).collect::<Vec<_>>();
        let total_bytes = documents
            .iter()
            .map(|(_, source)| source.len())
            .sum::<usize>();
        if total_bytes > MAX_INDEX_BYTES {
            return Err(format!(
                "The source closure is {total_bytes} bytes; language navigation is bounded to {MAX_INDEX_BYTES} bytes."
            ));
        }
        let mut symbols = Vec::new();
        let mut tokens = Vec::new();
        let mut sources = BTreeMap::new();
        for (logical_path, source) in documents {
            let syntax = document_syntax(bundle.language(), logical_path);
            let mut document_tokens = lex_identifiers(logical_path, source, syntax);
            let definitions =
                definitions_for_document(logical_path, source, syntax, &document_tokens);
            let definition_ranges = definitions
                .iter()
                .map(|symbol| (symbol.byte_range.start, symbol.byte_range.end))
                .collect::<BTreeSet<_>>();
            for token in &mut document_tokens {
                token.definition =
                    definition_ranges.contains(&(token.byte_range.start, token.byte_range.end));
                token.renamable =
                    matches!(syntax, DocumentSyntax::Python | DocumentSyntax::VerilogA);
            }
            symbols.extend(definitions);
            tokens.extend(document_tokens);
            sources.insert(
                crate::state::project_source_path_key(logical_path),
                source.to_owned(),
            );
            if tokens.len() > MAX_INDEX_TOKENS {
                return Err(format!(
                    "The source closure contains more than {MAX_INDEX_TOKENS} identifier tokens. Narrow the source closure before indexing."
                ));
            }
        }
        symbols.sort_by(|left, right| {
            (&left.logical_path, left.byte_range.start, &left.name).cmp(&(
                &right.logical_path,
                right.byte_range.start,
                &right.name,
            ))
        });
        Ok(Self {
            project_id,
            bundle_id: bundle.id(),
            bundle_revision: bundle.revision().get(),
            closure_digest: bundle.closure_digest(),
            language: bundle.language(),
            symbols,
            tokens,
            sources,
        })
    }

    pub(crate) fn is_current(&self, project_id: ProjectId, bundle: &ProjectSourceBundle) -> bool {
        self.project_id == project_id
            && self.bundle_id == bundle.id()
            && self.bundle_revision == bundle.revision().get()
            && self.closure_digest == bundle.closure_digest()
            && self.language == bundle.language()
    }

    pub(crate) fn symbols(&self) -> &[SourceSymbol] {
        &self.symbols
    }

    pub(crate) fn document_symbols(&self, logical_path: &str) -> Vec<&SourceSymbol> {
        self.symbols
            .iter()
            .filter(|symbol| {
                crate::state::project_source_paths_equal(&symbol.logical_path, logical_path)
            })
            .collect()
    }

    pub(crate) fn symbol_at(&self, logical_path: &str, char_index: usize) -> Option<&str> {
        let source = self.source_for_path(logical_path)?;
        let byte_index = char_to_byte(source, char_index);
        self.tokens
            .iter()
            .find(|token| {
                crate::state::project_source_paths_equal(&token.logical_path, logical_path)
                    && (token.byte_range.contains(&byte_index)
                        || token.byte_range.end == byte_index)
            })
            .map(|token| token.name.as_str())
    }

    pub(crate) fn definitions(&self, name: &str) -> Vec<&SourceSymbol> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.name == name)
            .collect()
    }

    pub(crate) fn references(&self, name: &str) -> Vec<SymbolOccurrence> {
        self.tokens
            .iter()
            .filter(|token| token.name == name)
            .map(|token| SymbolOccurrence {
                name: token.name.clone(),
                logical_path: token.logical_path.clone(),
                byte_range: token.byte_range.clone(),
                line: token.line,
                column: token.column,
                definition: token.definition,
                line_preview: self
                    .source_for_path(&token.logical_path)
                    .map_or_else(String::new, |source| {
                        source_line_preview(source, token.line)
                    }),
            })
            .collect()
    }

    pub(crate) fn declarations(&self, name: &str) -> Vec<&SourceSymbol> {
        // Python and Verilog-A declarations are the authoritative indexed
        // definitions in this project-owned source closure. Keeping this as a
        // separate operation preserves the language-service contract even
        // where the two locations are identical.
        self.definitions(name)
    }

    pub(crate) fn completions(
        &self,
        logical_path: &str,
        caret_char_index: usize,
        query: &str,
    ) -> Vec<LanguageCompletion> {
        let prefix = self
            .identifier_prefix(logical_path, caret_char_index)
            .unwrap_or_default();
        let filter = if query.trim().is_empty() {
            prefix
        } else {
            query.trim()
        }
        .to_ascii_lowercase();
        let mut candidates = BTreeMap::<String, LanguageCompletion>::new();
        for symbol in &self.symbols {
            candidates
                .entry(symbol.name.clone())
                .or_insert_with(|| LanguageCompletion {
                    label: symbol.name.clone(),
                    kind: symbol.kind.label().to_owned(),
                    detail: format!(
                        "{} · {}:{}",
                        symbol.signature, symbol.logical_path, symbol.line
                    ),
                });
        }
        let syntax = document_syntax(self.language, logical_path);
        for (label, kind, detail) in language_completion_catalog(syntax) {
            candidates
                .entry((*label).to_owned())
                .or_insert_with(|| LanguageCompletion {
                    label: (*label).to_owned(),
                    kind: (*kind).to_owned(),
                    detail: (*detail).to_owned(),
                });
        }
        candidates
            .into_values()
            .filter(|candidate| {
                filter.is_empty() || candidate.label.to_ascii_lowercase().starts_with(&filter)
            })
            .take(MAX_COMPLETIONS)
            .collect()
    }

    pub(crate) fn hover(&self, name: &str) -> Option<String> {
        if name.is_empty() {
            return None;
        }
        let definitions = self.definitions(name);
        if !definitions.is_empty() {
            let signatures = definitions
                .iter()
                .map(|definition| {
                    format!(
                        "{} · {}:{}",
                        definition.signature, definition.logical_path, definition.line
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            return Some(format!(
                "{} {} · {} indexed reference{}\n{}",
                definitions[0].kind.label(),
                name,
                self.references(name).len(),
                if self.references(name).len() == 1 {
                    ""
                } else {
                    "s"
                },
                signatures
            ));
        }
        language_completion_catalog_for(self.language)
            .iter()
            .find(|(label, _, _)| *label == name)
            .map(|(_, kind, detail)| format!("{kind} {name}\n{detail}"))
    }

    pub(crate) fn signature_help(
        &self,
        logical_path: &str,
        caret_char_index: usize,
    ) -> Option<String> {
        let source = self.source_for_path(logical_path)?;
        let caret = char_to_byte(source, caret_char_index);
        let function = call_identifier_before(source, caret)?;
        let indexed = self
            .definitions(function)
            .into_iter()
            .filter(|symbol| matches!(symbol.kind, SourceSymbolKind::Function))
            .map(|symbol| symbol.signature.clone())
            .collect::<Vec<_>>();
        if !indexed.is_empty() {
            return Some(indexed.join("\n"));
        }
        language_completion_catalog(document_syntax(self.language, logical_path))
            .iter()
            .find(|(label, kind, _)| *label == function && *kind == "function")
            .map(|(_, _, detail)| (*detail).to_owned())
    }

    pub(crate) fn code_actions(&self, logical_path: &str) -> Result<Vec<SourceCodeAction>, String> {
        let source = self
            .source_for_path(logical_path)
            .ok_or_else(|| format!("The source document '{logical_path}' is not indexed."))?;
        let syntax = document_syntax(self.language, logical_path);
        validate_source_for_formatting(syntax, logical_path, source)?;
        let protected = protected_string_lines(source, syntax);
        let (normalized, removed_whitespace) = normalize_source_whitespace(source, &protected);
        let mut actions = Vec::new();
        if removed_whitespace {
            actions.push(SourceCodeAction {
                title: "Remove trailing whitespace outside string literals".to_owned(),
                kind: SourceCodeActionKind::QuickFix,
                logical_path: logical_path.to_owned(),
                replacement: normalized.clone(),
            });
        }
        if normalized != *source {
            actions.push(SourceCodeAction {
                title: "Format active document".to_owned(),
                kind: SourceCodeActionKind::Format,
                logical_path: logical_path.to_owned(),
                replacement: normalized,
            });
        }
        Ok(actions)
    }

    fn identifier_prefix(&self, logical_path: &str, caret_char_index: usize) -> Option<&str> {
        let source = self.source_for_path(logical_path)?;
        let byte = char_to_byte(source, caret_char_index);
        let start = source[..byte]
            .char_indices()
            .rev()
            .find(|(_, character)| !(*character == '_' || character.is_alphanumeric()))
            .map_or(0, |(index, character)| index + character.len_utf8());
        source.get(start..byte)
    }

    fn completion_replacement(
        &self,
        logical_path: &str,
        caret_char_index: usize,
        completion: &str,
    ) -> Result<String, String> {
        let source = self
            .source_for_path(logical_path)
            .ok_or_else(|| format!("The source document '{logical_path}' is not indexed."))?;
        let caret = char_to_byte(source, caret_char_index);
        let start = source[..caret]
            .char_indices()
            .rev()
            .find(|(_, character)| !(*character == '_' || character.is_alphanumeric()))
            .map_or(0, |(index, character)| index + character.len_utf8());
        let end = source[caret..]
            .char_indices()
            .find(|(_, character)| !(*character == '_' || character.is_alphanumeric()))
            .map_or(source.len(), |(index, _)| caret + index);
        let mut replacement = source.clone();
        replacement.replace_range(start..end, completion);
        Ok(replacement)
    }

    pub(crate) fn rename_preview(
        &self,
        name: &str,
        replacement: &str,
    ) -> Result<Vec<(String, String)>, String> {
        if !valid_identifier(replacement) || language_keyword(self.language, replacement) {
            return Err(format!(
                "'{replacement}' is not a valid non-keyword identifier."
            ));
        }
        let definitions = self.definitions(name);
        if definitions.is_empty() {
            return Err(format!(
                "No project-owned definition of '{name}' is indexed."
            ));
        }
        if definitions.len() != 1 {
            return Err(format!(
                "Rename is ambiguous because '{}' has {} project-owned definitions. Find references and rename a narrower scope instead.",
                name,
                definitions.len()
            ));
        }
        let occurrences = self
            .tokens
            .iter()
            .filter(|token| token.name == name && token.renamable)
            .collect::<Vec<_>>();
        if occurrences.is_empty() {
            return Err(format!("No editable references to '{name}' are indexed."));
        }
        if occurrences.len() > MAX_RENAME_EDITS {
            return Err(format!(
                "Rename would create more than {MAX_RENAME_EDITS} edits. Narrow the source closure first."
            ));
        }
        let mut by_path = BTreeMap::<&str, Vec<&IndexedToken>>::new();
        for occurrence in occurrences {
            by_path
                .entry(&occurrence.logical_path)
                .or_default()
                .push(occurrence);
        }
        let mut replacements = Vec::with_capacity(by_path.len());
        for (path, mut edits) in by_path {
            let mut source = self
                .source_for_path(path)
                .ok_or_else(|| {
                    format!("The indexed source document '{path}' is no longer available.")
                })?
                .clone();
            edits.sort_by_key(|token| std::cmp::Reverse(token.byte_range.start));
            for edit in edits {
                source.replace_range(edit.byte_range.clone(), replacement);
            }
            replacements.push((path.to_owned(), source));
        }
        Ok(replacements)
    }

    fn source_for_path(&self, logical_path: &str) -> Option<&String> {
        self.sources
            .get(&crate::state::project_source_path_key(logical_path))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentSyntax {
    VerilogA,
    Python,
    Yaml,
    Toml,
}

fn document_syntax(language: ProjectSourceLanguage, logical_path: &str) -> DocumentSyntax {
    match language {
        ProjectSourceLanguage::VerilogA => DocumentSyntax::VerilogA,
        ProjectSourceLanguage::RSpiceAutomation => {
            let lower = logical_path.to_ascii_lowercase();
            if lower.ends_with(".yaml") || lower.ends_with(".yml") {
                DocumentSyntax::Yaml
            } else if lower.ends_with(".toml") || lower.ends_with(".lock") {
                DocumentSyntax::Toml
            } else {
                DocumentSyntax::Python
            }
        }
    }
}

fn language_completion_catalog(
    syntax: DocumentSyntax,
) -> &'static [(&'static str, &'static str, &'static str)] {
    match syntax {
        DocumentSyntax::VerilogA => &[
            ("module", "keyword", "module name(ports); ... endmodule"),
            ("analog", "keyword", "analog begin ... end"),
            ("parameter", "keyword", "parameter real name = value"),
            ("electrical", "keyword", "electrical node declarations"),
            ("discipline", "keyword", "discipline declaration"),
            ("nature", "keyword", "nature declaration"),
            (
                "transition",
                "function",
                "transition(expression, delay, rise, fall)",
            ),
            ("slew", "function", "slew(expression, rise, fall)"),
            ("ddt", "function", "ddt(expression)"),
            ("idt", "function", "idt(expression[, initial_condition])"),
            (
                "laplace_nd",
                "function",
                "laplace_nd(expression, numerator, denominator)",
            ),
            (
                "laplace_np",
                "function",
                "laplace_np(expression, numerator, poles)",
            ),
            ("white_noise", "function", "white_noise(power, name)"),
            (
                "flicker_noise",
                "function",
                "flicker_noise(power, exponent, name)",
            ),
            (
                "cross",
                "function",
                "cross(expression[, direction[, time_tol[, expr_tol]]])",
            ),
            (
                "timer",
                "function",
                "timer(start_time[, period[, time_tol]])",
            ),
            ("V", "access function", "V(node) or V(positive, negative)"),
            ("I", "access function", "I(branch) or I(positive, negative)"),
        ],
        DocumentSyntax::Python => &[
            ("def", "keyword", "def name(parameters):"),
            ("class", "keyword", "class Name:"),
            ("async", "keyword", "async def or async with statement"),
            ("await", "keyword", "await expression"),
            ("import", "keyword", "import module"),
            ("from", "keyword", "from module import name"),
            ("Project", "class", "rspice.Project.open(path) -> Project"),
            (
                "Environment",
                "class",
                "rspice.Environment provides capability-scoped environment reads",
            ),
            (
                "ArtifactFormat",
                "enum",
                "RSpice artifact format identifier",
            ),
            ("open", "function", "Project.open(path) -> Project"),
            (
                "get",
                "function",
                "Environment.get(name, default=None) -> str | None",
            ),
            (
                "validate",
                "function",
                "plan.validate(target, fail_closed=True) -> Preview",
            ),
            ("execute", "function", "preview.execute() -> Run"),
            ("evaluate", "function", "run.requirements.evaluate(profile)"),
            (
                "compare",
                "function",
                "run.compare(baseline, waveforms=True)",
            ),
            (
                "export",
                "function",
                "run.export(formats) -> list[Artifact]",
            ),
        ],
        DocumentSyntax::Yaml => &[
            ("schema", "configuration key", "Run-plan schema identity"),
            ("name", "configuration key", "Human-readable run-plan name"),
            (
                "source",
                "configuration key",
                "Project-relative top-deck source",
            ),
            (
                "analyses",
                "configuration key",
                "Ordered analysis selection",
            ),
            ("corners", "configuration key", "Qualified corner selection"),
            (
                "requirements",
                "configuration key",
                "Requirement profile identity",
            ),
            (
                "artifacts",
                "configuration key",
                "Requested governed artifacts",
            ),
        ],
        DocumentSyntax::Toml => &[
            (
                "project_files",
                "capability key",
                "Project-file access disposition",
            ),
            (
                "artifact_directory",
                "capability key",
                "Artifact-directory access disposition",
            ),
            (
                "network",
                "capability key",
                "Network capability disposition",
            ),
            (
                "process_spawn",
                "capability key",
                "Process-spawn capability disposition",
            ),
            (
                "environment",
                "capability key",
                "Explicit environment-variable allowlist",
            ),
            (
                "secret_logging",
                "capability key",
                "Secret redaction policy",
            ),
            ("python", "lock key", "Managed Python version requirement"),
            (
                "rspice_api",
                "lock key",
                "RSpice Automation API version requirement",
            ),
        ],
    }
}

fn language_completion_catalog_for(
    language: ProjectSourceLanguage,
) -> &'static [(&'static str, &'static str, &'static str)] {
    match language {
        ProjectSourceLanguage::VerilogA => language_completion_catalog(DocumentSyntax::VerilogA),
        ProjectSourceLanguage::RSpiceAutomation => {
            language_completion_catalog(DocumentSyntax::Python)
        }
    }
}

fn call_identifier_before(source: &str, caret: usize) -> Option<&str> {
    let bytes = source.as_bytes();
    let mut cursor = caret.min(bytes.len());
    let mut depth = 0_usize;
    let open = loop {
        cursor = cursor.checked_sub(1)?;
        match bytes[cursor] {
            b')' | b']' | b'}' => depth += 1,
            b'(' if depth == 0 => break cursor,
            b'(' | b'[' | b'{' => depth = depth.saturating_sub(1),
            b'\n' if depth == 0 => return None,
            _ => {}
        }
    };
    let mut end = open;
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    let mut start = end;
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    (start < end).then_some(&source[start..end])
}

fn validate_source_for_formatting(
    syntax: DocumentSyntax,
    logical_path: &str,
    source: &str,
) -> Result<(), String> {
    match syntax {
        // Python grammar belongs exclusively to the exact managed CPython or
        // Pyodide runtime. These actions only normalize whitespace outside
        // strings, so a lagging convenience parser must not reject current
        // Python syntax or be mistaken for execution evidence.
        DocumentSyntax::Python => Ok(()),
        DocumentSyntax::VerilogA => {
            let mut source_map = rspice_veriloga::SourceMap::new();
            let source_id = source_map.add_source_mut(logical_path, source);
            let tokens = rspice_veriloga::Lexer::new(source, source_id)
                .collect_tokens()
                .map_err(|error| format!("Formatting is blocked by Verilog-A lexing: {error}"))?;
            rspice_veriloga::Parser::new(&tokens)
                .parse()
                .map(|_| ())
                .map_err(|error| format!("Formatting is blocked by Verilog-A parsing: {error}"))
        }
        DocumentSyntax::Toml => toml::from_str::<toml::Value>(source)
            .map(|_| ())
            .map_err(|error| format!("Formatting is blocked by TOML parsing: {error}")),
        DocumentSyntax::Yaml => validate_yaml_shape(source),
    }
}

fn validate_yaml_shape(source: &str) -> Result<(), String> {
    let mut flow = Vec::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut escaped = false;
    for (offset, character) in source.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' && double_quote {
            escaped = true;
            continue;
        }
        if character == '\'' && !double_quote {
            single_quote = !single_quote;
            continue;
        }
        if character == '"' && !single_quote {
            double_quote = !double_quote;
            continue;
        }
        if single_quote || double_quote {
            continue;
        }
        match character {
            '[' | '{' => flow.push(character),
            ']' if flow.pop() != Some('[') => {
                return Err(format!(
                    "Formatting is blocked by an unmatched ']' at byte {offset}."
                ));
            }
            '}' if flow.pop() != Some('{') => {
                return Err(format!(
                    "Formatting is blocked by an unmatched '}}' at byte {offset}."
                ));
            }
            '\t' => {
                let line_start = source[..offset].rfind('\n').map_or(0, |line| line + 1);
                if source[line_start..offset].chars().all(char::is_whitespace) {
                    return Err(
                        "Formatting is blocked because YAML indentation contains a tab.".to_owned(),
                    );
                }
            }
            _ => {}
        }
    }
    if single_quote || double_quote || !flow.is_empty() {
        return Err(
            "Formatting is blocked because the YAML document has an unterminated quote or flow collection."
                .to_owned(),
        );
    }
    Ok(())
}

fn protected_string_lines(source: &str, syntax: DocumentSyntax) -> BTreeSet<usize> {
    let mut protected = BTreeSet::new();
    let mut quote: Option<char> = None;
    let mut triple = false;
    let mut escaped = false;
    let mut line = 0_usize;
    let mut chars = source.char_indices().peekable();
    while let Some((_, character)) = chars.next() {
        if quote.is_some() {
            protected.insert(line);
            if character == '\n' {
                line += 1;
                continue;
            }
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' {
                escaped = true;
                continue;
            }
            if Some(character) == quote {
                if triple {
                    let mut lookahead = chars.clone();
                    if lookahead.next().is_some_and(|(_, next)| next == character)
                        && lookahead.next().is_some_and(|(_, next)| next == character)
                    {
                        chars.next();
                        chars.next();
                        quote = None;
                        triple = false;
                    }
                } else {
                    quote = None;
                }
            }
            continue;
        }
        if character == '\n' {
            line += 1;
            continue;
        }
        if matches!(character, '\'' | '"') {
            let mut lookahead = chars.clone();
            triple = matches!(syntax, DocumentSyntax::Python)
                && lookahead.next().is_some_and(|(_, next)| next == character)
                && lookahead.next().is_some_and(|(_, next)| next == character);
            if triple {
                chars.next();
                chars.next();
            }
            quote = Some(character);
            protected.insert(line);
        }
    }

    if matches!(syntax, DocumentSyntax::Yaml) {
        let lines = source.lines().collect::<Vec<_>>();
        let mut scalar_indent = None;
        for (index, line_text) in lines.iter().enumerate() {
            let indentation = line_text.len() - line_text.trim_start_matches(' ').len();
            if scalar_indent
                .is_some_and(|indent| !line_text.trim().is_empty() && indentation <= indent)
            {
                scalar_indent = None;
            }
            if scalar_indent.is_some() {
                protected.insert(index);
            }
            let content = line_text.trim_end();
            if content.ends_with('|') || content.ends_with('>') {
                scalar_indent = Some(indentation);
            }
        }
    }
    protected
}

fn normalize_source_whitespace(source: &str, protected_lines: &BTreeSet<usize>) -> (String, bool) {
    let preferred_eol = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut output = String::with_capacity(source.len().saturating_add(preferred_eol.len()));
    let mut removed = false;
    for (line_index, segment) in source.split_inclusive('\n').enumerate() {
        let (body, eol) = segment.strip_suffix("\r\n").map_or_else(
            || {
                segment
                    .strip_suffix('\n')
                    .map_or((segment, ""), |body| (body, "\n"))
            },
            |body| (body, "\r\n"),
        );
        if protected_lines.contains(&line_index) {
            output.push_str(body);
        } else {
            let trimmed = body.trim_end_matches([' ', '\t']);
            removed |= trimmed.len() != body.len();
            output.push_str(trimmed);
        }
        output.push_str(eol);
    }
    if source.is_empty() {
        return (source.to_owned(), false);
    }
    if !source.ends_with('\n') {
        output.push_str(preferred_eol);
    }
    (output, removed)
}

fn bundle_documents(bundle: &ProjectSourceBundle) -> impl Iterator<Item = (&str, &str)> {
    std::iter::once((bundle.root().logical_path(), bundle.root().content())).chain(
        bundle
            .files()
            .iter()
            .map(|file| (file.logical_path(), file.content())),
    )
}

fn lex_identifiers(logical_path: &str, source: &str, syntax: DocumentSyntax) -> Vec<IndexedToken> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut cursor = 0;
    let mut line = 1;
    let mut line_start = 0;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\n' {
            line += 1;
            cursor += 1;
            line_start = cursor;
            continue;
        }
        if starts_comment(bytes, cursor, syntax) {
            if matches!(syntax, DocumentSyntax::VerilogA)
                && bytes.get(cursor..cursor + 2) == Some(b"/*")
            {
                cursor += 2;
                while cursor + 1 < bytes.len() && &bytes[cursor..cursor + 2] != b"*/" {
                    if bytes[cursor] == b'\n' {
                        line += 1;
                        line_start = cursor + 1;
                    }
                    cursor += 1;
                }
                cursor = (cursor + 2).min(bytes.len());
            } else {
                while cursor < bytes.len() && bytes[cursor] != b'\n' {
                    cursor += 1;
                }
            }
            continue;
        }
        if matches!(bytes[cursor], b'\'' | b'"') {
            let quote = bytes[cursor];
            let triple = matches!(syntax, DocumentSyntax::Python)
                && bytes.get(cursor..cursor + 3) == Some(&[quote, quote, quote]);
            cursor += if triple { 3 } else { 1 };
            while cursor < bytes.len() {
                if bytes[cursor] == b'\\' {
                    cursor = (cursor + 2).min(bytes.len());
                    continue;
                }
                if bytes[cursor] == b'\n' {
                    line += 1;
                    line_start = cursor + 1;
                }
                let closed = if triple {
                    bytes.get(cursor..cursor + 3) == Some(&[quote, quote, quote])
                } else {
                    bytes[cursor] == quote
                };
                cursor += 1;
                if closed {
                    cursor += if triple { 2 } else { 0 };
                    break;
                }
            }
            continue;
        }
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        let identifier_start = character == '_' || character.is_alphabetic();
        if identifier_start {
            let start = cursor;
            cursor += character.len_utf8();
            while cursor < bytes.len() {
                let Some(next) = source[cursor..].chars().next() else {
                    break;
                };
                if next == '_' || next.is_alphanumeric() {
                    cursor += next.len_utf8();
                } else {
                    break;
                }
            }
            tokens.push(IndexedToken {
                name: source[start..cursor].to_owned(),
                logical_path: logical_path.to_owned(),
                byte_range: start..cursor,
                line,
                column: source[line_start..start].chars().count() + 1,
                definition: false,
                renamable: false,
            });
        } else {
            cursor += character.len_utf8();
        }
    }
    tokens
}

fn starts_comment(bytes: &[u8], cursor: usize, syntax: DocumentSyntax) -> bool {
    match syntax {
        DocumentSyntax::VerilogA => {
            matches!(bytes.get(cursor..cursor + 2), Some(b"//") | Some(b"/*"))
        }
        DocumentSyntax::Python | DocumentSyntax::Yaml | DocumentSyntax::Toml => {
            bytes[cursor] == b'#'
        }
    }
}

fn definitions_for_document(
    logical_path: &str,
    source: &str,
    syntax: DocumentSyntax,
    tokens: &[IndexedToken],
) -> Vec<SourceSymbol> {
    match syntax {
        DocumentSyntax::VerilogA => verilog_definitions(logical_path, source, tokens),
        DocumentSyntax::Python => python_definitions(logical_path, source, tokens),
        DocumentSyntax::Yaml | DocumentSyntax::Toml => {
            configuration_definitions(logical_path, source, syntax, tokens)
        }
    }
}

fn verilog_definitions(
    logical_path: &str,
    source: &str,
    tokens: &[IndexedToken],
) -> Vec<SourceSymbol> {
    let declarations = [
        ("module", SourceSymbolKind::Module),
        ("function", SourceSymbolKind::Function),
        ("parameter", SourceSymbolKind::Parameter),
        ("localparam", SourceSymbolKind::Parameter),
        ("aliasparam", SourceSymbolKind::Parameter),
        ("input", SourceSymbolKind::Port),
        ("output", SourceSymbolKind::Port),
        ("inout", SourceSymbolKind::Port),
        ("electrical", SourceSymbolKind::Net),
        ("discipline", SourceSymbolKind::Discipline),
        ("nature", SourceSymbolKind::Nature),
    ];
    let mut symbols = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let Some((_, kind)) = declarations
            .iter()
            .find(|(keyword, _)| token.name.eq_ignore_ascii_case(keyword))
        else {
            continue;
        };
        let Some(candidate) = tokens[index + 1..]
            .iter()
            .take_while(|candidate| candidate.line <= token.line + 8)
            .find(|candidate| !verilog_type_keyword(&candidate.name))
        else {
            continue;
        };
        symbols.push(symbol_from_token(logical_path, source, candidate, *kind));
    }
    deduplicate_symbols(symbols)
}

fn python_definitions(
    logical_path: &str,
    source: &str,
    tokens: &[IndexedToken],
) -> Vec<SourceSymbol> {
    let mut symbols = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let (offset, kind) = match token.name.as_str() {
            "def" => (1, SourceSymbolKind::Function),
            "class" => (1, SourceSymbolKind::Class),
            "async" if tokens.get(index + 1).is_some_and(|next| next.name == "def") => {
                (2, SourceSymbolKind::Function)
            }
            "import" | "from" => (1, SourceSymbolKind::Import),
            _ => {
                let remainder = source[token.byte_range.end..]
                    .split_once('\n')
                    .map_or(&source[token.byte_range.end..], |(line, _)| line)
                    .trim_start();
                if first_token_on_line(tokens, index) && assignment_starts(remainder) {
                    symbols.push(symbol_from_token(
                        logical_path,
                        source,
                        token,
                        SourceSymbolKind::Variable,
                    ));
                }
                continue;
            }
        };
        if let Some(candidate) = tokens.get(index + offset) {
            symbols.push(symbol_from_token(logical_path, source, candidate, kind));
        }
    }
    deduplicate_symbols(symbols)
}

fn configuration_definitions(
    logical_path: &str,
    source: &str,
    syntax: DocumentSyntax,
    tokens: &[IndexedToken],
) -> Vec<SourceSymbol> {
    let mut symbols = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if !first_token_on_line(tokens, index) {
            continue;
        }
        let remainder = source[token.byte_range.end..]
            .split_once('\n')
            .map_or(&source[token.byte_range.end..], |(line, _)| line)
            .trim_start();
        let defines_key = match syntax {
            DocumentSyntax::Yaml => remainder.starts_with(':'),
            DocumentSyntax::Toml => remainder.starts_with('=') || remainder.starts_with(']'),
            _ => false,
        };
        if defines_key {
            symbols.push(symbol_from_token(
                logical_path,
                source,
                token,
                SourceSymbolKind::Configuration,
            ));
        }
    }
    deduplicate_symbols(symbols)
}

fn first_token_on_line(tokens: &[IndexedToken], index: usize) -> bool {
    index == 0 || tokens[index - 1].line != tokens[index].line
}

fn assignment_starts(remainder: &str) -> bool {
    remainder.starts_with('=') && !remainder.starts_with("==")
}

fn verilog_type_keyword(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "real" | "integer" | "string" | "wire" | "electrical" | "input" | "output" | "inout"
    )
}

fn symbol_from_token(
    logical_path: &str,
    source: &str,
    token: &IndexedToken,
    kind: SourceSymbolKind,
) -> SourceSymbol {
    SourceSymbol {
        name: token.name.clone(),
        kind,
        logical_path: logical_path.to_owned(),
        byte_range: token.byte_range.clone(),
        line: token.line,
        column: token.column,
        signature: source_line_preview(source, token.line),
    }
}

fn deduplicate_symbols(symbols: Vec<SourceSymbol>) -> Vec<SourceSymbol> {
    let mut seen = BTreeSet::new();
    symbols
        .into_iter()
        .filter(|symbol| {
            seen.insert((
                symbol.logical_path.clone(),
                symbol.byte_range.start,
                symbol.byte_range.end,
                symbol.kind,
            ))
        })
        .collect()
}

fn valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|character| character == '_' || character.is_alphabetic())
        && characters.all(|character| character == '_' || character.is_alphanumeric())
}

fn language_keyword(language: ProjectSourceLanguage, value: &str) -> bool {
    match language {
        ProjectSourceLanguage::VerilogA => matches!(
            value,
            "module"
                | "endmodule"
                | "analog"
                | "parameter"
                | "real"
                | "integer"
                | "input"
                | "output"
                | "inout"
                | "electrical"
                | "begin"
                | "end"
        ),
        ProjectSourceLanguage::RSpiceAutomation => matches!(
            value,
            "def"
                | "class"
                | "async"
                | "await"
                | "import"
                | "from"
                | "return"
                | "if"
                | "else"
                | "for"
                | "while"
                | "try"
                | "except"
                | "with"
                | "match"
                | "case"
                | "True"
                | "False"
                | "None"
        ),
    }
}

fn char_to_byte(source: &str, char_index: usize) -> usize {
    source
        .char_indices()
        .nth(char_index)
        .map_or(source.len(), |(byte, _)| byte)
}

fn source_line_preview(source: &str, one_based_line: usize) -> String {
    source
        .lines()
        .nth(one_based_line.saturating_sub(1))
        .unwrap_or_default()
        .trim()
        .chars()
        .take(240)
        .collect()
}

/// One row of a source file's outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceOutlineRow {
    pub kind: &'static str,
    pub name: String,
    pub detail: String,
    /// One-based line, ready to hand to the editor's go-to-line request.
    pub line: usize,
}

/// The language index for one bundle, retained against the exact closure it
/// was built from.
///
/// A navigator runs every frame and indexing a source closure does not, so the
/// index is rebuilt only when the bundle identity it was built against
/// changes. Keying on the closure digest rather than a revision counter means
/// an edit that produces identical bytes reuses the index instead of
/// re-parsing the tree.
#[derive(Debug, Clone)]
pub(crate) struct SourceIndexCache {
    bundle_id: ProjectSourceId,
    closure_digest: ContentDigest,
    index: LanguageServiceIndex,
}

/// The outline of `logical_path`, indexing the bundle only when it must.
///
/// An index that cannot be built — a closure past the navigation size bound —
/// yields no rows rather than a partial outline, because a short outline and a
/// complete one look identical to the reader.
pub(crate) fn document_outline(
    project_id: ProjectId,
    sources: &crate::state::ProjectSourceRegistry,
    cache: &mut Option<SourceIndexCache>,
    language: ProjectSourceLanguage,
    logical_path: &str,
) -> Vec<SourceOutlineRow> {
    let owner = ProjectSourceOwner::code_workspace(language);
    let refreshed = {
        let Some(bundle) = sources.bundle_for_owner(&owner) else {
            return Vec::new();
        };
        let current = cache.as_ref().is_some_and(|cache| {
            cache.bundle_id == bundle.id() && cache.closure_digest == bundle.closure_digest()
        });
        if current {
            None
        } else {
            // A closure past the navigation size bound yields no outline
            // rather than a partial one: a short outline and a complete one
            // look identical to the reader.
            LanguageServiceIndex::build(project_id, bundle)
                .ok()
                .map(|index| SourceIndexCache {
                    bundle_id: bundle.id(),
                    closure_digest: bundle.closure_digest(),
                    index,
                })
        }
    };
    if let Some(refreshed) = refreshed {
        *cache = Some(refreshed);
    }
    cache
        .as_ref()
        .map(|cache| {
            cache
                .index
                .document_symbols(logical_path)
                .into_iter()
                .map(|symbol| SourceOutlineRow {
                    kind: symbol.kind.label(),
                    name: symbol.name.clone(),
                    detail: symbol.signature.clone(),
                    line: symbol.line.saturating_add(1),
                })
                .collect()
        })
        .unwrap_or_default()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ProjectSourceDependency, ProjectSourceFile, ProjectSourceOwner, ProjectSourceRole,
        ProjectSourceRoleBinding,
    };

    fn bundle(
        language: ProjectSourceLanguage,
        root_path: &str,
        root: &str,
        files: Vec<ProjectSourceFile>,
    ) -> ProjectSourceBundle {
        let dependencies = files
            .iter()
            .map(|file| ProjectSourceDependency::try_new(root_path, file.logical_path()).unwrap())
            .collect::<Vec<_>>();
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(language),
            language,
            root_path,
            root,
            files,
            dependencies,
        )
        .unwrap()
    }

    #[test]
    fn verilog_index_navigates_and_renames_without_touching_comments_or_strings() {
        let bundle = bundle(
            ProjectSourceLanguage::VerilogA,
            "models/resistor.va",
            "module resistor(p, n);\nparameter real resistance = 1k;\nanalog V(p,n) <+ resistance;\n// resistance stays\nendmodule\n",
            vec![
                ProjectSourceFile::try_new(
                    "models/helper.va",
                    "module helper;\nstring label = \"resistance\";\nreal copy;\nanalog copy = resistance;\nendmodule\n",
                )
                .unwrap(),
            ],
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        assert_eq!(index.definitions("resistance").len(), 1);
        assert_eq!(index.references("resistance").len(), 3);
        let changed = index.rename_preview("resistance", "r_value").unwrap();
        assert_eq!(changed.len(), 2);
        let root = changed
            .iter()
            .find(|(path, _)| path == "models/resistor.va")
            .unwrap();
        assert!(root.1.contains("parameter real r_value"));
        assert!(root.1.contains("// resistance stays"));
        let helper = changed
            .iter()
            .find(|(path, _)| path == "models/helper.va")
            .unwrap();
        assert!(helper.1.contains("\"resistance\""));
        assert!(helper.1.contains("copy = r_value"));
    }

    #[test]
    fn python_index_is_comment_string_safe_and_ambiguous_rename_fails_closed() {
        let bundle = bundle(
            ProjectSourceLanguage::RSpiceAutomation,
            "main.py",
            "def evaluate(value):\n    return value\n# evaluate\nlabel = \"evaluate\"\nevaluate(1)\n",
            vec![
                ProjectSourceFile::try_new("helper.py", "def evaluate(other):\n    return other\n")
                    .unwrap(),
            ],
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        assert_eq!(index.definitions("evaluate").len(), 2);
        assert_eq!(index.references("evaluate").len(), 3);
        assert!(index.rename_preview("evaluate", "measure").is_err());
        assert!(index.rename_preview("evaluate", "class").is_err());
    }

    #[test]
    fn language_index_uses_unicode_project_path_identity() {
        let source = "def mesurer(value):  \n    return value\nmesurer(1)\n";
        let bundle = bundle(
            ProjectSourceLanguage::RSpiceAutomation,
            "Flux/été.py",
            source,
            Vec::new(),
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        let alias = "FLUX/ÉTÉ.PY";

        assert_eq!(index.document_symbols(alias).len(), 1);
        assert_eq!(index.symbol_at(alias, 6), Some("mesurer"));
        assert_eq!(index.references("mesurer").len(), 2);
        let actions = index.code_actions(alias).unwrap();
        assert!(actions.iter().any(|action| {
            action.kind == SourceCodeActionKind::QuickFix
                && crate::state::project_source_paths_equal(&action.logical_path, alias)
        }));
        assert_eq!(
            index.identifier_prefix(alias, source.chars().count()),
            index.identifier_prefix(bundle.root().logical_path(), source.chars().count())
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn project_rename_is_atomic_when_an_arbitrary_path_has_a_governed_role() {
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let root_path = "flows/main.py";
        let governed_path = "locks/arbitrary_name.py";
        let root_source =
            "def protected_name(value):\n    return value\n\nresult = protected_name(1)\n";
        let governed_source = "locked_reference = protected_name\n";
        let source_bundle = ProjectSourceBundle::try_new_with_roles(
            ProjectSourceOwner::code_workspace(language),
            language,
            root_path,
            root_source,
            [ProjectSourceFile::try_new(governed_path, governed_source).unwrap()],
            [ProjectSourceDependency::try_new(root_path, governed_path).unwrap()],
            [
                ProjectSourceRoleBinding::try_new(root_path, ProjectSourceRole::AutomationEntry)
                    .unwrap(),
                ProjectSourceRoleBinding::try_new(
                    governed_path,
                    ProjectSourceRole::AutomationEnvironmentLock,
                )
                .unwrap(),
            ],
        )
        .unwrap();
        let bundle_id = source_bundle.id();

        let mut app = crate::workbench::RSpiceApp::test_instance();
        app.state.workspace.remove_project_source(language);
        app.state
            .workspace
            .insert_project_source_bundle(source_bundle)
            .unwrap();
        let index = LanguageServiceIndex::build(
            app.state.workspace.project.id(),
            app.state
                .workspace
                .project_sources
                .get_bundle(bundle_id)
                .unwrap(),
        )
        .unwrap();
        assert_eq!(index.definitions("protected_name").len(), 1);
        assert_eq!(
            index
                .rename_preview("protected_name", "renamed")
                .unwrap()
                .len(),
            2
        );
        let before_revision = index.bundle_revision;
        app.state.ui.code_workspace.language_tools = Some(CodeLanguageToolsState {
            index,
            active_path: root_path.to_owned(),
            caret_char_index: root_source.find("protected_name").unwrap(),
            query: "protected_name".to_owned(),
            replacement: "renamed".to_owned(),
            view: LanguageToolView::References,
            confirm_rename: true,
            status: None,
            error: None,
        });

        let error = commit_language_rename(&mut app).unwrap_err();
        assert!(error.contains(governed_path));
        assert!(error.contains("No project source was changed"));
        let unchanged = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(unchanged.revision().get(), before_revision);
        assert_eq!(unchanged.root().content(), root_source);
        assert_eq!(unchanged.file_content(governed_path), Some(governed_source));
    }

    #[test]
    fn completion_hover_signature_and_declaration_use_the_exact_indexed_closure() {
        let source = "from rspice import Project\n\ndef evaluate(value):\n    return value\n\nresult = evaluate(1)\n";
        let bundle = bundle(
            ProjectSourceLanguage::RSpiceAutomation,
            "flows/main.py",
            source,
            Vec::new(),
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        let caret = source.find("eval").unwrap() + "eval".len();
        let caret_chars = source[..caret].chars().count();
        let completions = index.completions("flows/main.py", caret_chars, "");
        assert!(
            completions
                .iter()
                .any(|completion| completion.label == "evaluate")
        );
        let catalog_completions = index.completions("flows/main.py", 0, "");
        assert!(
            catalog_completions
                .iter()
                .any(|completion| completion.label == "Environment")
        );
        assert!(
            index
                .hover("evaluate")
                .unwrap()
                .contains("indexed reference")
        );
        assert_eq!(index.declarations("evaluate").len(), 1);

        let call_caret = source.rfind("evaluate(1").unwrap() + "evaluate(".len();
        let signature = index
            .signature_help("flows/main.py", source[..call_caret].chars().count())
            .unwrap();
        assert!(signature.contains("def evaluate(value)"));
    }

    #[test]
    fn completion_replaces_only_the_identifier_fragment_at_the_captured_caret() {
        let source = "def evaluate(value):\n    return value\n\neval(1)\n";
        let source_bundle = bundle(
            ProjectSourceLanguage::RSpiceAutomation,
            "main.py",
            source,
            Vec::new(),
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &source_bundle).unwrap();
        let byte_caret = source.rfind("eval(").unwrap() + "eval".len();
        let replacement = index
            .completion_replacement("main.py", source[..byte_caret].chars().count(), "evaluate")
            .unwrap();
        assert!(replacement.ends_with("evaluate(1)\n"));
        assert!(replacement.starts_with("def evaluate(value):"));
    }

    #[test]
    fn formatting_actions_preserve_literal_whitespace_without_claiming_python_authority() {
        let source = "def evaluate(value):   \n    label = \"keep   \"\n    return value   ";
        let source_bundle = bundle(
            ProjectSourceLanguage::RSpiceAutomation,
            "main.py",
            source,
            Vec::new(),
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &source_bundle).unwrap();
        let actions = index.code_actions("main.py").unwrap();
        let format = actions
            .iter()
            .find(|action| action.kind == SourceCodeActionKind::Format)
            .unwrap();
        assert!(format.replacement.contains("label = \"keep   \""));
        assert!(!format.replacement.contains("return value   "));
        assert!(format.replacement.ends_with('\n'));

        let malformed = bundle(
            ProjectSourceLanguage::RSpiceAutomation,
            "broken.py",
            "def broken(:\n",
            Vec::new(),
        );
        let malformed = LanguageServiceIndex::build(ProjectId::new(), &malformed).unwrap();
        assert!(malformed.code_actions("broken.py").unwrap().is_empty());
    }
}

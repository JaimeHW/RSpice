//! Revision-bound language index for project-owned Verilog-A and Automation sources.
//!
//! The compiler and managed Python worker remain the syntax and semantic
//! authorities. This index supplies bounded source navigation and conservative
//! refactors: it never edits comments/strings. A Verilog-A name is resolved
//! through its file's [`VerilogASymbols`], so a rename reaches exactly the
//! uses of the declaration under the caret; elsewhere rename is refused
//! whenever a symbol has multiple project-owned definitions.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use super::CodeWorkspaceParts;
use crate::product::{ContentDigest, ProjectId};
use crate::state::{ProjectSourceBundle, ProjectSourceId, ProjectSourceLanguage};

use super::veriloga_catalog::VerilogACatalog;
use super::veriloga_symbols::{VerilogASymbolKind, VerilogASymbols};

const MAX_INDEX_BYTES: usize = 16 * 1024 * 1024;
const MAX_INDEX_TOKENS: usize = 500_000;
const MAX_RENAME_EDITS: usize = 50_000;

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

/// One occurrence of a name, as Find references lists it: where it is, and
/// the line it sits on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SymbolOccurrence {
    pub logical_path: String,
    pub line: usize,
    pub column: usize,
    pub line_preview: String,
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
pub struct LanguageServiceIndex {
    pub project_id: ProjectId,
    pub bundle_id: ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: ContentDigest,
    pub language: ProjectSourceLanguage,
    symbols: Vec<SourceSymbol>,
    tokens: Vec<IndexedToken>,
    sources: BTreeMap<String, String>,
    /// Each Verilog-A file's own reading, by path key.
    veriloga: BTreeMap<String, VerilogASymbols>,
}

/// An open Rename symbol: the index it was opened against, the name under
/// the caret, and the new name being typed.
#[derive(Debug, Clone)]
pub struct CodeLanguageToolsState {
    pub index: LanguageServiceIndex,
    pub active_path: String,
    /// The character the name was taken from.
    pub caret: usize,
    pub query: String,
    pub replacement: String,
    /// Why the last Rename was refused, until the name is edited.
    pub error: Option<String>,
}

pub fn open_language_tools_in_bundle(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    active_path: &str,
    caret_char_index: usize,
) -> Result<(), String> {
    if !super::source_bundle_contains_document(parts, language, bundle_id, active_path) {
        return Err(format!(
            "The active source file '{active_path}' no longer belongs to this project source bundle."
        ));
    }
    let bundle = parts
        .model
        .workspace
        .content
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
    let index = LanguageServiceIndex::build(parts.model.workspace.content.project.id(), bundle)?;
    let query = index
        .symbol_at(&active_path, caret_char_index)
        .unwrap_or_default()
        .to_owned();
    parts.code.language_tools = Some(CodeLanguageToolsState {
        index,
        active_path,
        caret: caret_char_index,
        query,
        replacement: String::new(),
        error: None,
    });
    Ok(())
}

pub fn commit_language_rename(parts: &mut CodeWorkspaceParts<'_>) -> Result<String, String> {
    let state = current_language_transaction(parts)?;
    let replacements = state.index.rename_preview_at(
        &state.active_path,
        state.caret,
        &state.query,
        &state.replacement,
    )?;
    if let Some((logical_path, _)) = replacements.iter().find(|(logical_path, _)| {
        !super::source_files::source_bundle_document_is_editable_in(
            parts.model,
            parts.code,
            parts.platform,
            state.index.language,
            state.index.bundle_id,
            logical_path,
        )
    }) {
        return Err(format!(
            "Rename would change governed read-only source '{logical_path}'. No project source was changed."
        ));
    }
    let (occurrences, _) =
        state
            .index
            .rename_extent_at(&state.active_path, state.caret, &state.query);
    let changed_files = {
        parts.code.source_edit_history.replace(
            &mut parts.model.workspace.content,
            state.index.bundle_id,
            replacements,
        )?
    };
    match state.index.language {
        ProjectSourceLanguage::VerilogA => {
            super::veriloga::invalidate_veriloga_evidence_in(
                parts.model,
                parts.code,
                state.index.bundle_id,
            );
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            super::automation::invalidate_automation_evidence_in(parts.code, parts.handles);
        }
    }
    let message = format!(
        "Renamed {} to {}: {}.",
        state.query,
        state.replacement,
        extent_words(occurrences, changed_files)
    );
    // The rename is the form's one transaction: done, the form is closed.
    parts.code.language_tools = None;
    parts.wb.prompt(message.clone());
    Ok(message)
}

/// "3 occurrences in 2 files", for the rename form and its receipt.
pub fn extent_words(occurrences: usize, files: usize) -> String {
    format!(
        "{occurrences} occurrence{} in {files} file{}",
        if occurrences == 1 { "" } else { "s" },
        if files == 1 { "" } else { "s" }
    )
}

fn current_language_transaction(
    parts: &CodeWorkspaceParts<'_>,
) -> Result<CodeLanguageToolsState, String> {
    let state = parts
        .code
        .language_tools
        .clone()
        .ok_or_else(|| "No source-language transaction is open.".to_owned())?;
    if let Some(reason) = super::source_file_mutation_block_reason(parts, state.index.language) {
        return Err(reason.to_owned());
    }
    if !super::source_files::source_bundle_document_is_editable_in(
        parts.model,
        parts.code,
        parts.platform,
        state.index.language,
        state.index.bundle_id,
        &state.active_path,
    ) {
        return Err(format!(
            "'{}' is governed as a read-only source. Create an editable project copy before applying source changes.",
            state.active_path
        ));
    }
    let bundle = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(state.index.bundle_id)
        .ok_or_else(|| "The indexed source bundle no longer exists.".to_owned())?;
    if !state
        .index
        .is_current(parts.model.workspace.content.project.id(), bundle)
    {
        return Err(
            "The source bundle changed after the language transaction was opened. Reindex the current revision."
                .to_owned(),
        );
    }
    Ok(state)
}

/// Format document over a project source: the formatter's whole-document
/// edit, applied in place and stated on the status bar.
///
/// The editor's Format document used to open the language-tools dialog on
/// its code actions instead, leaving the user to find and apply the format
/// there. A read-only source, or one the formatter refuses to touch because
/// it does not parse, says why instead of changing anything.
pub fn format_language_document(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    bundle_id: ProjectSourceId,
    logical_path: &str,
) -> Result<String, String> {
    if let Some(reason) = super::source_file_mutation_block_reason(parts, language) {
        return Err(reason.to_owned());
    }
    if !super::source_files::source_bundle_document_is_editable_in(
        parts.model,
        parts.code,
        parts.platform,
        language,
        bundle_id,
        logical_path,
    ) {
        return Err(format!(
            "{logical_path} is read-only, so it was not formatted."
        ));
    }
    let source = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .and_then(|bundle| {
            bundle_documents(bundle)
                .find(|(path, _)| crate::state::project_source_paths_equal(path, logical_path))
                .map(|(_, source)| source.to_owned())
        })
        .ok_or_else(|| format!("{logical_path} is no longer part of this project."))?;
    let formatted = formatted_source(language, logical_path, &source)?;
    let message = if formatted == source {
        format!("{logical_path} is already formatted.")
    } else {
        write_language_document(parts, language, bundle_id, logical_path, formatted)?;
        format!("Formatted {logical_path}.")
    };
    parts.wb.prompt(message.clone());
    Ok(message)
}

/// The formatter's whole-document edit, or why the source is refused. For
/// every language: trailing blanks removed outside string literals and one
/// final line break. Verilog-A is also laid out by its block structure.
fn formatted_source(
    language: ProjectSourceLanguage,
    logical_path: &str,
    source: &str,
) -> Result<String, String> {
    let syntax = document_syntax(language, logical_path);
    validate_source_for_formatting(syntax, logical_path, source)?;
    let protected = protected_string_lines(source, syntax);
    let normalized = normalize_source_whitespace(source, &protected).0;
    Ok(match syntax {
        DocumentSyntax::VerilogA => super::veriloga_format::format_source(&normalized),
        DocumentSyntax::Python | DocumentSyntax::Yaml | DocumentSyntax::Toml => normalized,
    })
}

#[cfg(test)]
pub(super) fn formatted_verilog_a(logical_path: &str, source: &str) -> Result<String, String> {
    formatted_source(ProjectSourceLanguage::VerilogA, logical_path, source)
}

fn write_language_document(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    bundle_id: ProjectSourceId,
    logical_path: &str,
    replacement: String,
) -> Result<(), String> {
    parts.code.source_edit_history.replace(
        &mut parts.model.workspace.content,
        bundle_id,
        vec![(logical_path.to_owned(), replacement)],
    )?;
    match language {
        ProjectSourceLanguage::VerilogA => {
            super::veriloga::invalidate_veriloga_evidence_in(parts.model, parts.code, bundle_id);
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            super::automation::invalidate_automation_evidence_in(parts.code, parts.handles);
        }
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
        let mut veriloga = BTreeMap::new();
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
            if syntax == DocumentSyntax::VerilogA {
                veriloga.insert(
                    crate::state::project_source_path_key(logical_path),
                    VerilogASymbols::read(source),
                );
            }
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
            veriloga,
        })
    }

    pub(crate) fn is_current(&self, project_id: ProjectId, bundle: &ProjectSourceBundle) -> bool {
        self.project_id == project_id
            && self.bundle_id == bundle.id()
            && self.bundle_revision == bundle.revision().get()
            && self.closure_digest == bundle.closure_digest()
            && self.language == bundle.language()
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
                logical_path: token.logical_path.clone(),
                line: token.line,
                column: token.column,
                line_preview: self
                    .source_for_path(&token.logical_path)
                    .map_or_else(String::new, |source| {
                        source_line_preview(source, token.line)
                    }),
            })
            .collect()
    }

    /// What a rename of `name` rewrites: its occurrences outside comments and
    /// strings, and the files they sit in.
    pub fn rename_extent(&self, name: &str) -> (usize, usize) {
        let occurrences = self
            .tokens
            .iter()
            .filter(|token| token.name == name && token.renamable);
        let files = occurrences
            .clone()
            .map(|token| crate::state::project_source_path_key(&token.logical_path))
            .collect::<BTreeSet<_>>();
        (occurrences.count(), files.len())
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

    /// The definition of the name at `char_index` of `logical_path`: the
    /// declaration a Verilog-A name refers to, or the name's only or first
    /// indexed definition.
    pub(crate) fn definition_at(
        &self,
        logical_path: &str,
        char_index: usize,
        name: &str,
    ) -> Option<SymbolPlace> {
        if let Some(target) = self.veriloga_target(logical_path, char_index, name) {
            let source = self.source_for_path(&target.logical_path)?;
            let place = self.veriloga_symbols(&target.logical_path)?;
            let declaration = place.declaration(target.declaration)?;
            let (line, column) = place_of(source, declaration.name_range.start);
            return Some(SymbolPlace {
                logical_path: target.logical_path,
                line,
                column,
            });
        }
        let definitions = self.definitions(name);
        definitions
            .iter()
            .find(|symbol| {
                crate::state::project_source_paths_equal(&symbol.logical_path, logical_path)
            })
            .or_else(|| definitions.first())
            .map(|symbol| SymbolPlace {
                logical_path: symbol.logical_path.clone(),
                line: symbol.line,
                column: symbol.column,
            })
    }

    /// Every use of the name at `char_index` of `logical_path`, in file and
    /// text order. A Verilog-A name that resolves lists the uses of its
    /// declaration alone.
    pub(crate) fn references_at(
        &self,
        logical_path: &str,
        char_index: usize,
        name: &str,
    ) -> Vec<SymbolOccurrence> {
        let Some(target) = self.veriloga_target(logical_path, char_index, name) else {
            return self.references(name);
        };
        self.veriloga_sites(&target, name)
            .into_iter()
            .flat_map(|(path, ranges)| {
                let source = self.source_for_path(&path).cloned().unwrap_or_default();
                ranges.into_iter().map(move |range| {
                    let (line, column) = place_of(&source, range.start);
                    SymbolOccurrence {
                        logical_path: path.clone(),
                        line,
                        column,
                        line_preview: source_line_preview(&source, line),
                    }
                })
            })
            .collect()
    }

    /// What a rename of the name at `char_index` of `logical_path` rewrites:
    /// its uses and the files they sit in.
    pub(crate) fn rename_extent_at(
        &self,
        logical_path: &str,
        char_index: usize,
        name: &str,
    ) -> (usize, usize) {
        match self.veriloga_target(logical_path, char_index, name) {
            Some(target) => {
                let sites = self.veriloga_sites(&target, name);
                (
                    sites.iter().map(|(_, ranges)| ranges.len()).sum(),
                    sites.len(),
                )
            }
            None => self.rename_extent(name),
        }
    }

    /// The new text of every file a rename of the name at `char_index` of
    /// `logical_path` changes, or why it is refused.
    pub(crate) fn rename_preview_at(
        &self,
        logical_path: &str,
        char_index: usize,
        name: &str,
        replacement: &str,
    ) -> Result<Vec<(String, String)>, String> {
        if self.language != ProjectSourceLanguage::VerilogA {
            return self.rename_preview(name, replacement);
        }
        if !valid_identifier(replacement) {
            return Err(format!("'{replacement}' is not a valid name."));
        }
        let catalog = VerilogACatalog::get();
        if rspice_veriloga::language_catalog::is_reserved_word(replacement) {
            return Err(format!(
                "'{replacement}' is a reserved word of Verilog-A, so it cannot be a name."
            ));
        }
        if catalog.reserves(replacement) {
            return Err(format!(
                "'{replacement}' already means something in Verilog-A, so it cannot be a name."
            ));
        }
        if catalog.reserves(name)
            && self
                .veriloga_target(logical_path, char_index, name)
                .is_none()
        {
            return Err(format!(
                "'{name}' is part of Verilog-A and cannot be renamed."
            ));
        }
        let symbols = self.veriloga_symbols(logical_path).ok_or_else(|| {
            format!("The indexed source document '{logical_path}' is no longer available.")
        })?;
        let Some(target) = self.veriloga_target(logical_path, char_index, name) else {
            return self.rename_unresolved(logical_path, symbols, name, replacement);
        };
        self.veriloga_rename_refusal(&target, name, replacement)?;
        let sites = self.veriloga_sites(&target, name);
        if sites.iter().map(|(_, ranges)| ranges.len()).sum::<usize>() > MAX_RENAME_EDITS {
            return Err(format!(
                "Rename would create more than {MAX_RENAME_EDITS} edits. Narrow the source closure first."
            ));
        }
        let mut replacements = Vec::with_capacity(sites.len());
        for (path, ranges) in sites {
            let mut source = self
                .source_for_path(&path)
                .ok_or_else(|| {
                    format!("The indexed source document '{path}' is no longer available.")
                })?
                .clone();
            for range in ranges.into_iter().rev() {
                source.replace_range(range, replacement);
            }
            replacements.push((path, source));
        }
        Ok(replacements)
    }

    /// A rename of a name the file on screen does not declare. A fragment
    /// that is included into a module cannot be read by itself, so the name
    /// is followed by the text alone, and only when the bundle holds one
    /// definition of it. A whole file that does not read is refused: where
    /// its scopes begin and end is unknown.
    fn rename_unresolved(
        &self,
        logical_path: &str,
        symbols: &VerilogASymbols,
        name: &str,
        replacement: &str,
    ) -> Result<Vec<(String, String)>, String> {
        if let Some((line, reason)) = symbols.unreadable()
            && self.has_a_module(logical_path)
        {
            return Err(format!(
                "Rename needs to read the whole file, and line {line} does not read: {reason}"
            ));
        }
        self.rename_preview(name, replacement)
    }

    fn has_a_module(&self, logical_path: &str) -> bool {
        self.tokens.iter().any(|token| {
            token.name == "module"
                && crate::state::project_source_paths_equal(&token.logical_path, logical_path)
        })
    }

    fn veriloga_symbols(&self, logical_path: &str) -> Option<&VerilogASymbols> {
        self.veriloga
            .get(&crate::state::project_source_path_key(logical_path))
    }

    /// The declaration the Verilog-A name at `char_index` refers to.
    fn veriloga_target(
        &self,
        logical_path: &str,
        char_index: usize,
        name: &str,
    ) -> Option<VerilogATarget> {
        let symbols = self.veriloga_symbols(logical_path)?;
        let source = self.source_for_path(logical_path)?;
        let written = symbols
            .name_at(char_to_byte(source, char_index))
            .filter(|written| written.name == name)?;
        Some(VerilogATarget {
            logical_path: logical_path.to_owned(),
            declaration: written.declaration?,
        })
    }

    /// Every place `target` is written, by file, each file's ranges in text
    /// order: its uses in its own file and, for a declaration a whole module
    /// or file can see, the names another file of the bundle leaves
    /// undeclared. Such a file is a fragment included where the name is
    /// declared.
    fn veriloga_sites(
        &self,
        target: &VerilogATarget,
        name: &str,
    ) -> Vec<(String, Vec<Range<usize>>)> {
        let Some(symbols) = self.veriloga_symbols(&target.logical_path) else {
            return Vec::new();
        };
        let mut sites = vec![(
            target.logical_path.clone(),
            symbols
                .references(target.declaration)
                .map(|written| written.range.clone())
                .collect::<Vec<_>>(),
        )];
        let module_wide = symbols
            .declaration(target.declaration)
            .is_some_and(|declaration| symbols.is_module_level(declaration.scope));
        if module_wide {
            for (path, other) in self.veriloga_documents() {
                if crate::state::project_source_paths_equal(path, &target.logical_path) {
                    continue;
                }
                let undeclared = other
                    .names()
                    .iter()
                    .filter(|written| written.name == name && written.declaration.is_none())
                    .map(|written| written.range.clone())
                    .collect::<Vec<_>>();
                if !undeclared.is_empty() {
                    sites.push((path.to_owned(), undeclared));
                }
            }
        }
        sites
    }

    /// Every Verilog-A file's logical path and its reading.
    fn veriloga_documents(&self) -> impl Iterator<Item = (&str, &VerilogASymbols)> {
        let mut paths = BTreeMap::new();
        for token in &self.tokens {
            paths
                .entry(crate::state::project_source_path_key(&token.logical_path))
                .or_insert(token.logical_path.as_str());
        }
        paths
            .into_iter()
            .filter_map(|(key, path)| self.veriloga.get(&key).map(|symbols| (path, symbols)))
    }

    /// Why renaming `target` to `replacement` would change what the source
    /// means, when it would.
    fn veriloga_rename_refusal(
        &self,
        target: &VerilogATarget,
        name: &str,
        replacement: &str,
    ) -> Result<(), String> {
        let Some(symbols) = self.veriloga_symbols(&target.logical_path) else {
            return Ok(());
        };
        let Some(declaration) = symbols.declaration(target.declaration) else {
            return Ok(());
        };
        if symbols
            .declared_in(declaration.scope, replacement)
            .is_some()
        {
            return Err(format!(
                "'{replacement}' is already declared beside '{name}'."
            ));
        }
        // A use of the name that an inner declaration of the new name would
        // take over.
        for written in symbols.references(target.declaration) {
            let shadowed = symbols
                .enclosing_scopes(written.range.start)
                .take_while(|scope| *scope != declaration.scope)
                .any(|scope| symbols.declared_in(scope, replacement).is_some());
            if shadowed {
                return Err(format!(
                    "'{replacement}' is declared in an inner scope that uses '{name}' at line {}, and would hide it there.",
                    written.line
                ));
            }
        }
        // A use of the new name that the renamed declaration would take over.
        for written in symbols
            .names()
            .iter()
            .filter(|written| written.name == replacement)
        {
            let captured = symbols
                .enclosing_scopes(written.range.start)
                .take_while(|scope| symbols.declared_in(*scope, replacement).is_none())
                .any(|scope| scope == declaration.scope);
            if captured {
                return Err(format!(
                    "Line {} already uses '{replacement}', which would then mean the renamed '{name}'.",
                    written.line
                ));
            }
        }
        // A port or parameter written after a `.` in an instance names the
        // declaration from outside, where its scope cannot be told.
        if matches!(
            declaration.kind,
            VerilogASymbolKind::Port | VerilogASymbolKind::Parameter
        ) {
            for (path, other) in self.veriloga_documents() {
                if let Some(member) = other.member_named(name) {
                    return Err(format!(
                        "'{name}' is connected by name at {path}:{}, which Rename cannot follow.",
                        member.line
                    ));
                }
            }
        }
        // A file that leaves the name undeclared means one module's, and
        // more than one module of the bundle may declare it.
        let module_wide = symbols.is_module_level(declaration.scope);
        let leaves_undeclared = self.veriloga_documents().any(|(path, other)| {
            !crate::state::project_source_paths_equal(path, &target.logical_path)
                && other
                    .names()
                    .iter()
                    .any(|written| written.name == name && written.declaration.is_none())
        });
        if module_wide && leaves_undeclared {
            let candidates = self
                .veriloga_documents()
                .flat_map(|(_, other)| {
                    other.declarations().iter().filter(move |candidate| {
                        candidate.name == name && other.is_module_level(candidate.scope)
                    })
                })
                .count();
            if candidates > 1 {
                return Err(format!(
                    "Another file uses '{name}' without declaring it, and {candidates} declarations could be the one it means."
                ));
            }
        }
        Ok(())
    }

    fn source_for_path(&self, logical_path: &str) -> Option<&String> {
        self.sources
            .get(&crate::state::project_source_path_key(logical_path))
    }
}

/// A place a definition is written: one-based line and column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SymbolPlace {
    pub logical_path: String,
    pub line: usize,
    pub column: usize,
}

/// The declaration a Verilog-A name resolves to, in the file it is written.
#[derive(Debug, Clone, PartialEq, Eq)]
struct VerilogATarget {
    logical_path: String,
    declaration: usize,
}

/// The one-based line and character column of byte `offset`.
fn place_of(source: &str, offset: usize) -> (usize, usize) {
    let before = &source[..offset.min(source.len())];
    let line_start = before.rfind('\n').map_or(0, |newline| newline + 1);
    (
        before.matches('\n').count() + 1,
        before[line_start..].chars().count() + 1,
    )
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
            // Layout is read from the tokens and keeps them all, so it needs
            // no more than that they read: a header of macros is no module,
            // and a module's directives are not yet expanded here.
            rspice_veriloga::Lexer::new(source, source_id)
                .collect_tokens()
                .map(|_| ())
                .map_err(|error| format!("Formatting is blocked by Verilog-A lexing: {error}"))
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

pub(super) fn bundle_documents(bundle: &ProjectSourceBundle) -> impl Iterator<Item = (&str, &str)> {
    std::iter::once((bundle.root().logical_path(), bundle.root().content())).chain(
        bundle
            .files()
            .iter()
            .map(|file| (file.logical_path(), file.content())),
    )
}

fn lex_identifiers(logical_path: &str, source: &str, syntax: DocumentSyntax) -> Vec<IndexedToken> {
    if syntax == DocumentSyntax::VerilogA {
        return lex_veriloga_identifiers(logical_path, source);
    }
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
        // Python, YAML and TOML each comment from `#` to the end of the line.
        if bytes[cursor] == b'#' {
            while cursor < bytes.len() && bytes[cursor] != b'\n' {
                cursor += 1;
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

/// The names of a Verilog-A source, read through the compiler's own lexer.
///
/// A number such as `10p`, `1e-3` or the `10ns` of a time scale, a based
/// number such as `4'b1010` and a string each hold no name, so neither a
/// rename nor a reference search reaches into one, and the code after a based
/// number is read as code. Inside every other token a name is a word as the
/// editor reads words, so `$temperature` and a macro's `` `NAME `` index their
/// word. A stretch the lexer refuses, as a half-typed source holds, is stepped
/// over and lexing resumes after it, so the names on either side stay indexed.
fn lex_veriloga_identifiers(logical_path: &str, source: &str) -> Vec<IndexedToken> {
    use rspice_veriloga::TokenKind;

    let mut source_map = rspice_veriloga::SourceMap::new();
    let source_id = source_map.add_source_mut(logical_path, source);
    let line_starts = std::iter::once(0)
        .chain(source.match_indices('\n').map(|(index, _)| index + 1))
        .collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut resume = 0;
    // Where the number just read ends: a word that starts right there is the
    // rest of its text, as `s` is of `10ns`, and no name.
    let mut number_end = None;
    while resume < source.len() {
        let mut lexer = rspice_veriloga::Lexer::new(&source[resume..], source_id);
        let refused = loop {
            let token = match lexer.next_token() {
                Ok(token) if token.kind == TokenKind::Eof => break None,
                Ok(token) => token,
                Err(error) => break Some(error.span),
            };
            let start = resume + token.span.start as usize;
            let glued = number_end.take() == Some(start);
            let end = match token.kind {
                TokenKind::IntegerLiteral | TokenKind::RealLiteral => {
                    number_end = Some(resume + token.span.end as usize);
                    continue;
                }
                TokenKind::FourStateLiteral | TokenKind::StringLiteral => continue,
                TokenKind::Identifier if glued => continue,
                // A directive's span runs on over the operand its line gives
                // it, whose own tokens the lexer still yields after it.
                TokenKind::Directive => start + token.text.as_deref().map_or(0, str::len),
                _ => resume + token.span.end as usize,
            };
            push_identifier_words(&mut tokens, logical_path, source, &line_starts, start..end);
        };
        let Some(span) = refused else {
            break;
        };
        let start = resume + span.start as usize;
        let end = resume + span.end as usize;
        resume = if end > start {
            end
        } else {
            start + source[start..].chars().next().map_or(1, char::len_utf8)
        };
    }
    tokens
}

/// Index each word of `source[range]`: a letter or `_`, then letters, digits
/// and `_`, the shape every syntax's names take here.
fn push_identifier_words(
    tokens: &mut Vec<IndexedToken>,
    logical_path: &str,
    source: &str,
    line_starts: &[usize],
    range: Range<usize>,
) {
    let mut characters = source[range.clone()].char_indices().peekable();
    while let Some((offset, character)) = characters.next() {
        if !(character == '_' || character.is_alphabetic()) {
            continue;
        }
        let mut word_end = offset + character.len_utf8();
        while let Some(&(next_offset, next)) = characters.peek() {
            if !(next == '_' || next.is_alphanumeric()) {
                break;
            }
            word_end = next_offset + next.len_utf8();
            characters.next();
        }
        let start = range.start + offset;
        let end = range.start + word_end;
        let line = line_starts.partition_point(|&line_start| line_start <= start);
        tokens.push(IndexedToken {
            name: source[start..end].to_owned(),
            logical_path: logical_path.to_owned(),
            byte_range: start..end,
            line,
            column: source[line_starts[line - 1]..start].chars().count() + 1,
            definition: false,
            renamable: false,
        });
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
        let following = &tokens[index + 1..];
        let Some(first) = following
            .iter()
            .take_while(|candidate| candidate.line <= token.line + 8)
            .position(|candidate| !verilog_type_keyword(&candidate.name))
        else {
            continue;
        };
        symbols.push(symbol_from_token(
            logical_path,
            source,
            &following[first],
            *kind,
        ));
        // `input a, b;` and `electrical a, b;` declare every name they list.
        if matches!(kind, SourceSymbolKind::Port | SourceSymbolKind::Net) {
            for pair in following[first..].windows(2) {
                if source[pair[0].byte_range.end..pair[1].byte_range.start].trim() != "," {
                    break;
                }
                symbols.push(symbol_from_token(logical_path, source, &pair[1], *kind));
            }
        }
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

pub(super) fn source_line_preview(source: &str, one_based_line: usize) -> String {
    source
        .lines()
        .nth(one_based_line.saturating_sub(1))
        .unwrap_or_default()
        .trim()
        .chars()
        .take(240)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ProjectSourceDependency, ProjectSourceFile, ProjectSourceOwner, ProjectSourceRole,
        ProjectSourceRoleBinding,
    };
    use crate::workbench::code::{CodeFeature, tests::harness};

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
        // What the rename form states: three occurrences in the two files.
        let caret = "module resistor(p, n);\nparameter real ".chars().count();
        assert_eq!(
            index.rename_extent_at("models/resistor.va", caret, "resistance"),
            (3, 2)
        );
        let changed = index
            .rename_preview_at("models/resistor.va", caret, "resistance", "r_value")
            .unwrap();
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

    /// A parameter named like a scale factor or an exponent shares its letter
    /// with number literals; renaming it rewrote `10p` and `1e-3` as well, and
    /// a time scale's `1ns` read as a name.
    #[test]
    fn renaming_a_verilog_a_name_leaves_number_literals_alone() {
        let bundle = bundle(
            ProjectSourceLanguage::VerilogA,
            "models/leaky_cap.va",
            "`timescale 1ns/1ps\nmodule leaky_cap(a, b);\ninout a, b;\nelectrical a, b;\nparameter real p = 10p;\nparameter real e = 2.5e-3;\nanalog I(a, b) <+ p * ddt(V(a, b)) + e * 1e-3 * V(a, b);\nendmodule\n",
            vec![],
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        assert_eq!(index.references("p").len(), 2);
        assert_eq!(index.references("e").len(), 2);
        assert!(index.references("ns").is_empty());
        assert!(index.references("s").is_empty());

        let changed = index.rename_preview("p", "capacitance").unwrap();
        assert_eq!(changed.len(), 1);
        assert!(changed[0].1.contains("parameter real capacitance = 10p;"));
        assert!(changed[0].1.contains("<+ capacitance * ddt"));

        let changed = index.rename_preview("e", "leakage").unwrap();
        assert!(changed[0].1.contains("parameter real leakage = 2.5e-3;"));
        assert!(changed[0].1.contains("+ leakage * 1e-3 *"));
    }

    /// A based number's apostrophe once opened a quoted span that ran to the
    /// next apostrophe, hiding every name after it from references.
    #[test]
    fn names_after_a_based_verilog_a_number_are_indexed() {
        let bundle = bundle(
            ProjectSourceLanguage::VerilogA,
            "models/masked_gain.va",
            "module masked_gain(out);\noutput out;\nelectrical out;\nparameter integer mask = 4'b1010;\nparameter real gain = 2.0;\nanalog V(out) <+ gain * mask;\nendmodule\n",
            vec![],
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        assert_eq!(index.definitions("gain").len(), 1);
        assert_eq!(index.references("gain").len(), 2);
        assert_eq!(index.references("mask").len(), 2);
        let changed = index.rename_preview("gain", "scale").unwrap();
        assert!(changed[0].1.contains("<+ scale * mask;"));
    }

    /// A source the lexer refuses somewhere, as one being typed is, keeps the
    /// names before and after the refused stretch.
    #[test]
    fn a_verilog_a_stretch_the_lexer_refuses_hides_no_names_around_it() {
        let bundle = bundle(
            ProjectSourceLanguage::VerilogA,
            "models/half_typed.va",
            "module half_typed(a);\ninout a;\nelectrical a;\nparameter real before = 1.0;\nreal broken = 3e+;\nparameter real after = 2.0;\nanalog V(a) <+ before + after * \"unterminated",
            vec![],
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        assert_eq!(index.definitions("before").len(), 1);
        assert_eq!(index.definitions("after").len(), 1);
        assert_eq!(index.references("before").len(), 2);
        assert_eq!(index.references("after").len(), 2);
        assert!(index.references("unterminated").is_empty());
    }

    /// A string continued onto the next line with `\` was counted as one line,
    /// so every name after it was placed a line early.
    #[test]
    fn verilog_a_names_after_a_continued_string_keep_their_lines() {
        let bundle = bundle(
            ProjectSourceLanguage::VerilogA,
            "models/continued.va",
            "module continued(a);\ninout a;\nelectrical a;\n`define WARN $strobe(\"first \\\nsecond\")\nparameter real gain = 1.0;\nanalog V(a) <+ gain;\nendmodule\n",
            vec![],
        );
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        let definitions = index.definitions("gain");
        assert_eq!(definitions.len(), 1);
        assert_eq!((definitions[0].line, definitions[0].column), (6, 16));
    }

    const TWO_SCOPES: &str = "`include \"disciplines.vams\"\nmodule one(a, b);\n  inout a, b;\n  electrical a, b;\n  analog V(a, b) <+ 0;\nendmodule\nmodule two(b, c);\n  inout b, c;\n  electrical b, c;\n  real x;\n  analog begin\n    x = V(b, c);\n    I(b, c) <+ x;\n  end\nendmodule\n";

    fn two_scopes() -> LanguageServiceIndex {
        let bundle = bundle(
            ProjectSourceLanguage::VerilogA,
            "two.va",
            TWO_SCOPES,
            Vec::new(),
        );
        LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap()
    }

    fn char_of(source: &str, written: &str) -> usize {
        source[..source.find(written).expect("the text is in the source")]
            .chars()
            .count()
    }

    /// Renaming `b` in `electrical a, b;` of the first module rewrites that
    /// module's `b` and leaves the second module's alone, where the rename
    /// used to be refused as ambiguous, or to rewrite every `b` of the file.
    #[test]
    fn a_verilog_a_rename_touches_only_the_scope_of_the_name_under_the_caret() {
        let index = two_scopes();
        let caret = char_of(TWO_SCOPES, "b;\n  analog V(a");
        let changed = index
            .rename_preview_at("two.va", caret, "b", "bottom")
            .unwrap();
        assert_eq!(changed.len(), 1);
        assert_eq!(
            changed[0].1,
            TWO_SCOPES
                .replace("module one(a, b);", "module one(a, bottom);")
                .replace("inout a, b;", "inout a, bottom;")
                .replace("electrical a, b;", "electrical a, bottom;")
                .replace("V(a, b)", "V(a, bottom)")
        );
        assert_eq!(index.rename_extent_at("two.va", caret, "b"), (4, 1));
        assert_eq!(index.references_at("two.va", caret, "b").len(), 4);
        // The second declarator of a list is a declaration of its own.
        let index = two_scopes();
        let definition = index.definition_at("two.va", caret, "b").unwrap();
        assert_eq!(definition.line, 2);
        assert_eq!(definition.column, 15);
    }

    /// The whole reserved-word list and the language's own names are
    /// refused, not the twelve words the guard used to know.
    #[test]
    fn a_verilog_a_rename_to_a_keyword_or_a_built_in_is_refused_in_plain_words() {
        let index = two_scopes();
        let caret = char_of(TWO_SCOPES, "b;\n  analog V(a");
        let refusal = |replacement: &str| {
            index
                .rename_preview_at("two.va", caret, "b", replacement)
                .unwrap_err()
        };
        assert_eq!(
            refusal("begin"),
            "'begin' is a reserved word of Verilog-A, so it cannot be a name."
        );
        assert_eq!(
            refusal("localparam"),
            "'localparam' is a reserved word of Verilog-A, so it cannot be a name."
        );
        assert_eq!(
            refusal("ddt"),
            "'ddt' already means something in Verilog-A, so it cannot be a name."
        );
        assert_eq!(refusal("$strobe"), "'$strobe' is not a valid name.");
        assert_eq!(refusal("2b"), "'2b' is not a valid name.");
    }

    #[test]
    fn a_verilog_a_rename_that_would_change_the_meaning_is_refused() {
        let source = "module m(p);\n  inout p;\n  electrical p;\n  real a, b;\n  analog function real f;\n    input c;\n    real d;\n    f = a + c + b;\n  endfunction\n  analog begin : body\n    integer k;\n    k = 1;\n    V(p) <+ a * k;\n  end\nendmodule\n";
        let bundle = bundle(ProjectSourceLanguage::VerilogA, "m.va", source, Vec::new());
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        let rename = |written: &str, name: &str, replacement: &str| {
            index.rename_preview_at("m.va", char_of(source, written), name, replacement)
        };
        assert_eq!(
            rename("a, b;", "a", "b").unwrap_err(),
            "'b' is already declared beside 'a'."
        );
        assert_eq!(
            rename("a, b;", "a", "d").unwrap_err(),
            "'d' is declared in an inner scope that uses 'a' at line 8, and would hide it there."
        );
        assert_eq!(
            rename("c;\n", "c", "a").unwrap_err(),
            "Line 8 already uses 'a', which would then mean the renamed 'c'."
        );
        // Fine: nothing inside `body` is called `q`, and `k` is local to it.
        let renamed = rename("k;\n", "k", "q").unwrap();
        assert_eq!(
            renamed[0].1,
            source
                .replace("integer k;", "integer q;")
                .replace("k = 1;", "q = 1;")
                .replace("a * k;", "a * q;")
        );
    }

    /// A file that does not read has no scopes to rename within.
    #[test]
    fn a_verilog_a_rename_in_a_module_that_does_not_read_says_which_line() {
        let source = "module m(p);\n  inout p;\n  electrical p;\n  parameter real g = ;\n  analog V(p) <+ g;\nendmodule\n";
        let bundle = bundle(ProjectSourceLanguage::VerilogA, "m.va", source, Vec::new());
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        let refusal = index
            .rename_preview_at("m.va", char_of(source, "g = ;"), "g", "gain")
            .unwrap_err();
        assert!(
            refusal.starts_with("Rename needs to read the whole file, and line 4"),
            "{refusal}"
        );
    }

    /// A parameter connected by name from an instance cannot be followed.
    #[test]
    fn a_port_or_parameter_connected_by_name_is_not_renamed_behind_it() {
        let source = "module inner(p);\n  inout p;\n  electrical p;\n  parameter real g = 1;\n  analog V(p) <+ g;\nendmodule\nmodule outer(q);\n  inout q;\n  electrical q;\n  inner #(.g(2)) u1 (.p(q));\nendmodule\n";
        let bundle = bundle(ProjectSourceLanguage::VerilogA, "m.va", source, Vec::new());
        let index = LanguageServiceIndex::build(ProjectId::new(), &bundle).unwrap();
        let refusal = index
            .rename_preview_at("m.va", char_of(source, "g = 1"), "g", "gain")
            .unwrap_err();
        assert_eq!(
            refusal,
            "'g' is connected by name at m.va:10, which Rename cannot follow."
        );
    }

    /// References list the uses of the one declaration, with their columns.
    #[test]
    fn verilog_a_references_are_those_of_the_declaration_under_the_caret() {
        let index = two_scopes();
        let caret = char_of(TWO_SCOPES, "x = V(");
        let references = index.references_at("two.va", caret, "x");
        assert_eq!(
            references
                .iter()
                .map(|reference| (
                    reference.line,
                    reference.column,
                    reference.line_preview.as_str()
                ))
                .collect::<Vec<_>>(),
            [
                (10, 8, "real x;"),
                (12, 5, "x = V(b, c);"),
                (13, 16, "I(b, c) <+ x;"),
            ]
        );
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
        assert_eq!(index.rename_extent("mesurer"), (2, 1));
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

        let mut harness = harness();
        harness
            .model
            .workspace
            .content
            .remove_project_source(language);
        harness
            .model
            .workspace
            .content
            .insert_project_source_bundle(source_bundle)
            .unwrap();
        let index = LanguageServiceIndex::build(
            harness.model.workspace.content.project.id(),
            harness
                .model
                .workspace
                .content
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
        harness.state.code_workspace.language_tools = Some(CodeLanguageToolsState {
            index,
            active_path: root_path.to_owned(),
            caret: 4,
            query: "protected_name".to_owned(),
            replacement: "renamed".to_owned(),
            error: None,
        });

        let error = commit_language_rename(&mut CodeFeature::workspace_parts(&mut harness.cx()))
            .unwrap_err();
        assert!(error.contains(governed_path));
        assert!(error.contains("No project source was changed"));
        let unchanged = harness
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(unchanged.revision().get(), before_revision);
        assert_eq!(unchanged.root().content(), root_source);
        assert_eq!(unchanged.file_content(governed_path), Some(governed_source));
    }

    #[test]
    fn formatting_preserves_literal_whitespace_without_claiming_python_authority() {
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let source = "def evaluate(value):   \n    label = \"keep   \"\n    return value   ";
        let formatted = formatted_source(language, "main.py", source).unwrap();
        assert!(formatted.contains("label = \"keep   \""));
        assert!(!formatted.contains("return value   "));
        assert!(formatted.ends_with('\n'));

        // Python's grammar is the managed interpreter's to judge: a source
        // it would refuse is left as it is, not refused here.
        let malformed = "def broken(:\n";
        assert_eq!(
            formatted_source(language, "broken.py", malformed).unwrap(),
            malformed
        );
    }
}

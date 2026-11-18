//! Revision-bound search and atomic replacement across a project source bundle.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use std::borrow::Cow;
use std::sync::Arc;

use sha2::{Digest as _, Sha256};

use super::CodeWorkspaceParts;
use crate::product::ContentDigest;
use crate::state::ProjectSourceLanguage;
use crate::state::{
    BoundedFindMatches, FindDirection, FindOptions, ProjectSourceBundle,
    find_all_in_source_bounded, replace_source_ranges,
};
use crate::ui::accessibility::counted;
use crate::workbench::documents::netlist_document::NetlistDocumentState;
use rspice_app_services::workbench::slots::platform::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;

use super::CodeWorkspaceRuntimeState;
use super::language_services::bundle_documents;
use super::{CodeSourceSearchScope, CodeSourceSearchState};

pub fn source_search_query_id() -> egui::Id {
    egui::Id::new(("rspice.source-find", "query"))
}

pub const SOURCE_SEARCH_RESULT_LIMIT: usize = 500;
pub(crate) const SOURCE_SEARCH_STREAM_LIMIT: usize = 50_000;
pub(crate) const GENERATED_REFERENCE_PATH: &str = "rspice://generated/top-deck";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSourceSearchResult {
    pub logical_path: String,
    pub line: usize,
    pub column: usize,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_preview: String,
    pub editable: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CodeSourceSearchResults {
    pub matches: Vec<CodeSourceSearchResult>,
    pub total_matches: usize,
    pub editable_matches: usize,
    pub display_truncated: bool,
    pub truncated: bool,
    pub read_only_matches: usize,
}

pub fn open_source_search(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    active_path: &str,
) -> Result<(), String> {
    let bundle_id = parts
        .model
        .workspace
        .active_source_bundle(language)
        .map(ProjectSourceBundle::id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    open_source_search_in_bundle(parts, language, bundle_id, active_path)
}

pub fn open_source_search_in_bundle(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    active_path: &str,
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
    let selection_char_range = parts
        .code
        .source_selections
        .get(&(bundle.id(), active_path.clone()))
        .copied()
        .filter(|(start, end)| start < end);
    if parts.code.source_search.as_ref().is_some_and(|search| {
        search.project_id == parts.model.workspace.content.project.id()
            && search.bundle_id == bundle.id()
            && search.bundle_revision == bundle.revision().get()
            && search.closure_digest == bundle.closure_digest()
    }) {
        if let Some(search) = parts.code.source_search.as_mut() {
            search.active_path.clone_from(&active_path);
            search.selection_char_range = selection_char_range;
            if search.scope == CodeSourceSearchScope::Selection
                && search.selection_char_range.is_none()
            {
                search.scope = CodeSourceSearchScope::CurrentDocument;
            }
        }
        return Ok(());
    }
    parts.code.source_search = Some(CodeSourceSearchState {
        project_id: parts.model.workspace.content.project.id(),
        bundle_id: bundle.id(),
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        generated_reference_revision: parts.netlist.revision,
        generated_reference_digest: ContentDigest::from_bytes(
            Sha256::digest(parts.model.netlist_artifacts.generated_source.as_bytes()).into(),
        ),
        language,
        active_path,
        selection_char_range,
        scope: CodeSourceSearchScope::ActiveLanguageProject,
        query: String::new(),
        replacement: String::new(),
        match_case: false,
        whole_symbol: false,
        regular_expression: false,
        include_comments: true,
        include_generated_references: false,
        confirm_replace: false,
        error: None,
    });
    Ok(())
}

pub fn open_active_source_search(parts: &mut CodeWorkspaceParts<'_>) -> Result<(), String> {
    match super::active_source_language(parts.wb.workspace, &parts.model.workspace) {
        None => {
            parts.netlist.find.open = true;
            Ok(())
        }
        Some(ProjectSourceLanguage::VerilogA) => {
            let selected =
                super::veriloga::selected_veriloga_editor_snapshot_in(parts.model, parts.code)?;
            let active_path = selected.active_path().to_owned();
            let bundle_id = selected.bundle_id();
            open_source_search_in_bundle(
                parts,
                ProjectSourceLanguage::VerilogA,
                bundle_id,
                &active_path,
            )
        }
    }
}

/// The matches the search finds in its source bundle, over the model, the
/// Code workspace's runtime state, the netlist editor's state and the
/// platform's.
pub fn source_search_results_in(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
    search: &CodeSourceSearchState,
) -> Result<CodeSourceSearchResults, String> {
    let scan = scan_source_bundle(model, code, netlist, search)?;
    Ok(scan.results(|logical_path| {
        super::source_bundle_document_is_editable_in(
            model,
            code,
            platform,
            search.language,
            search.bundle_id,
            logical_path,
        )
    }))
}

/// [`source_search_results_in`] for a frame to draw. A search is bound to
/// the revisions of the sources it reads, so while it stands what it finds
/// stands too: the sources are scanned once per search, kept in `ctx`, and
/// a frame only checks that the search still stands and who may write to
/// each file.
pub fn kept_source_search_results_in(
    ctx: &egui::Context,
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
    search: &CodeSourceSearchState,
) -> Result<CodeSourceSearchResults, String> {
    let key = ScanKey::of(code, search);
    let kept = ctx
        .data(|data| data.get_temp::<KeptScan>(kept_scan_id()))
        .filter(|kept| kept.key == key);
    let scan = if let Some(kept) = kept {
        current_bundle(model, netlist, search)?;
        kept.scan
    } else {
        let scan = Arc::new(scan_source_bundle(model, code, netlist, search)?);
        let kept = KeptScan {
            key,
            scan: Arc::clone(&scan),
        };
        ctx.data_mut(|data| data.insert_temp(kept_scan_id(), kept));
        scan
    };
    Ok(scan.results(|logical_path| {
        super::source_bundle_document_is_editable_in(
            model,
            code,
            platform,
            search.language,
            search.bundle_id,
            logical_path,
        )
    }))
}

/// Let go of what the last search found: the search closed.
pub fn forget_source_search_results(ctx: &egui::Context) {
    ctx.data_mut(|data| data.remove::<KeptScan>(kept_scan_id()));
}

fn kept_scan_id() -> egui::Id {
    source_search_query_id().with("found")
}

/// What a search found, kept with the search it answers.
#[derive(Clone)]
struct KeptScan {
    key: ScanKey,
    scan: Arc<SourceSearchScan>,
}

/// What a scan depends on: the search, less what only a replace reads, and
/// for a search of the open documents, which ones are open.
#[derive(Clone, PartialEq)]
struct ScanKey {
    search: CodeSourceSearchState,
    open: Vec<String>,
}

impl ScanKey {
    fn of(code: &CodeWorkspaceRuntimeState, search: &CodeSourceSearchState) -> Self {
        let open = if search.scope == CodeSourceSearchScope::OpenDocuments {
            code.source_carets
                .keys()
                .filter(|(bundle_id, _)| *bundle_id == search.bundle_id)
                .map(|(_, path)| path.clone())
                .collect()
        } else {
            Vec::new()
        };
        Self {
            search: CodeSourceSearchState {
                replacement: String::new(),
                confirm_replace: false,
                error: None,
                ..search.clone()
            },
            open,
        }
    }
}

/// What a search found in its source bundle, before anything about who may
/// write to the files.
#[derive(Debug, Default)]
struct SourceSearchScan {
    documents: Vec<ScannedDocument>,
    truncated: bool,
}

#[derive(Debug)]
struct ScannedDocument {
    logical_path: String,
    /// Whether a replace may write to it at all: a project file, not the
    /// generated deck.
    writable: bool,
    /// How many matches it holds.
    found: usize,
    /// The matches the results list shows.
    shown: Vec<ScannedMatch>,
}

#[derive(Debug)]
struct ScannedMatch {
    line: usize,
    column: usize,
    byte_start: usize,
    byte_end: usize,
    line_preview: String,
}

impl SourceSearchScan {
    /// The results, with whether `editable` lets each file be written.
    fn results(&self, mut editable: impl FnMut(&str) -> bool) -> CodeSourceSearchResults {
        let mut output = CodeSourceSearchResults::default();
        for document in &self.documents {
            let editable = document.writable && editable(&document.logical_path);
            output.read_only_matches += document.found * usize::from(!editable);
            output.editable_matches += document.found * usize::from(editable);
            output.total_matches += document.found;
            output
                .matches
                .extend(document.shown.iter().map(|found| CodeSourceSearchResult {
                    logical_path: document.logical_path.clone(),
                    line: found.line,
                    column: found.column,
                    byte_start: found.byte_start,
                    byte_end: found.byte_end,
                    line_preview: found.line_preview.clone(),
                    editable,
                }));
        }
        output.truncated = self.truncated;
        output.display_truncated = output.total_matches > output.matches.len() || output.truncated;
        output
    }
}

fn scan_source_bundle(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    netlist: &NetlistDocumentState,
    search: &CodeSourceSearchState,
) -> Result<SourceSearchScan, String> {
    let bundle = current_bundle(model, netlist, search)?;
    if search.query.is_empty() {
        return Ok(SourceSearchScan::default());
    }
    let options = search_options(search);
    let mut scan = SourceSearchScan::default();
    let mut total = 0_usize;
    let mut shown = 0_usize;
    let documents = search_documents(model, code, bundle, search)?;
    for (index, document) in documents.iter().enumerate() {
        let searchable_source = searchable_document_source(document, search)?;
        let remaining = SOURCE_SEARCH_STREAM_LIMIT.saturating_sub(total);
        if remaining == 0 {
            for later in &documents[index..] {
                let later_source = searchable_document_source(later, search)?;
                if find_all_in_source_bounded(&later_source, &search.query, options, 1)
                    .is_ok_and(|found| !found.matches().is_empty())
                {
                    scan.truncated = true;
                    break;
                }
            }
            break;
        }
        let found: BoundedFindMatches =
            find_all_in_source_bounded(&searchable_source, &search.query, options, remaining)
                .map_err(|error| error.to_string())?;
        total += found.matches().len();
        let mut lines = LineCursor::new(document.complete_source);
        let document_shown = found
            .matches()
            .iter()
            .take(SOURCE_SEARCH_RESULT_LIMIT.saturating_sub(shown))
            .map(|found| {
                let relative_range = found.byte_range();
                let byte_start = document.base_byte + relative_range.start;
                let (line, column, line_preview) = lines.at(byte_start);
                ScannedMatch {
                    line,
                    column,
                    byte_start,
                    byte_end: document.base_byte + relative_range.end,
                    line_preview,
                }
            })
            .collect::<Vec<_>>();
        shown += document_shown.len();
        scan.documents.push(ScannedDocument {
            logical_path: document.logical_path.to_owned(),
            writable: document.editable,
            found: found.matches().len(),
            shown: document_shown,
        });
        if found.truncated() {
            scan.truncated = true;
            break;
        }
    }
    Ok(scan)
}

/// Where a run of byte offsets in one source falls, read forward from the
/// last one: the line, the column and the line's text for each, without
/// reading the source from its start again for every match.
struct LineCursor<'a> {
    source: &'a str,
    /// The byte the cursor has read up to.
    byte: usize,
    /// The one-based line holding `byte`, and where that line starts.
    line: usize,
    line_start: usize,
}

impl<'a> LineCursor<'a> {
    const fn new(source: &'a str) -> Self {
        Self {
            source,
            byte: 0,
            line: 1,
            line_start: 0,
        }
    }

    /// The one-based line and column of `byte`, and its line's preview.
    fn at(&mut self, byte: usize) -> (usize, usize, String) {
        let byte = byte.min(self.source.len());
        if byte < self.byte {
            *self = Self::new(self.source);
        }
        let read = &self.source.as_bytes()[self.byte..byte];
        if let Some(last) = read.iter().rposition(|byte| *byte == b'\n') {
            self.line += read.iter().filter(|byte| **byte == b'\n').count();
            self.line_start = self.byte + last + 1;
        }
        self.byte = byte;
        let column = self.source[self.line_start..byte].chars().count() + 1;
        let line_end = self.source[self.line_start..]
            .find('\n')
            .map_or(self.source.len(), |end| self.line_start + end);
        let preview = self.source[self.line_start..line_end]
            .trim()
            .chars()
            .take(240)
            .collect();
        (self.line, column, preview)
    }
}

pub fn commit_source_search_replace(parts: &mut CodeWorkspaceParts<'_>) -> Result<String, String> {
    let search = parts
        .code
        .source_search
        .clone()
        .ok_or_else(|| "No source search transaction is open.".to_owned())?;
    if let Some(reason) = super::source_file_mutation_block_reason(parts, search.language) {
        return Err(reason.to_owned());
    }
    let results = source_search_results_in(
        parts.model,
        parts.code,
        parts.netlist,
        parts.platform,
        &search,
    )?;
    if results.truncated {
        return Err(format!(
            "More than {SOURCE_SEARCH_STREAM_LIMIT} matches exist. Narrow the search before replacing so the transaction remains bounded."
        ));
    }
    let bundle = current_bundle(parts.model, parts.netlist, &search)?;
    let options = search_options(&search);
    let mut replacements = Vec::new();
    let mut replacement_count = 0_usize;
    for document in search_documents(parts.model, parts.code, bundle, &search)? {
        if !document.editable
            || !super::source_files::source_bundle_document_is_editable_in(
                parts.model,
                parts.code,
                parts.platform,
                search.language,
                search.bundle_id,
                document.logical_path,
            )
        {
            continue;
        }
        let searchable_source = searchable_document_source(&document, &search)?;
        let found = find_all_in_source_bounded(
            &searchable_source,
            &search.query,
            options,
            SOURCE_SEARCH_STREAM_LIMIT.saturating_sub(replacement_count),
        )
        .map_err(|error| error.to_string())?;
        if found.truncated() {
            return Err(format!(
                "More than {SOURCE_SEARCH_STREAM_LIMIT} matches exist. Narrow the search before replacing so the transaction remains bounded."
            ));
        }
        let ranges = found
            .matches()
            .iter()
            .map(|found| found.byte_range())
            .collect::<Vec<_>>();
        let outcome = replace_source_ranges(
            document.searchable_source,
            &search.query,
            &search.replacement,
            options,
            &ranges,
            false,
        )
        .map_err(|error| error.to_string())?;
        if outcome.replacement_count() > 0 {
            replacement_count += outcome.replacement_count();
            let replacement = outcome.into_source();
            let complete = if document.base_byte == 0
                && document.searchable_source.len() == document.complete_source.len()
            {
                replacement
            } else {
                let end = document.base_byte + document.searchable_source.len();
                let mut complete = String::with_capacity(
                    document.complete_source.len() - document.searchable_source.len()
                        + replacement.len(),
                );
                complete.push_str(&document.complete_source[..document.base_byte]);
                complete.push_str(&replacement);
                complete.push_str(&document.complete_source[end..]);
                complete
            };
            replacements.push((document.logical_path.to_owned(), complete));
        }
    }
    if replacement_count == 0 {
        return Err(if results.read_only_matches > 0 {
            "Every match is in a read-only project source; no files were changed.".to_owned()
        } else {
            "The current source bundle contains no matches to replace.".to_owned()
        });
    }
    let changed_files = {
        parts.code.source_edit_history.replace(
            &mut parts.model.workspace.content,
            search.bundle_id,
            replacements,
        )?
    };
    match search.language {
        ProjectSourceLanguage::VerilogA => {
            super::veriloga::invalidate_veriloga_evidence_in(
                parts.model,
                parts.code,
                search.bundle_id,
            );
        }
    }
    parts.code.source_search = None;
    let message = format!(
        "Replaced {} in {}{}.",
        counted(replacement_count, "match", "matches"),
        counted(changed_files, "project source file", "project source files"),
        if results.read_only_matches == 0 {
            String::new()
        } else {
            format!(
                "; left {} unchanged",
                counted(
                    results.read_only_matches,
                    "read-only match",
                    "read-only matches"
                )
            )
        }
    );
    parts.wb.prompt(message.clone());
    Ok(message)
}

fn current_bundle<'a>(
    model: &'a Model,
    netlist: &NetlistDocumentState,
    search: &CodeSourceSearchState,
) -> Result<&'a ProjectSourceBundle, String> {
    if search.project_id != model.workspace.content.project.id() {
        return Err("The active project changed while search results were open.".to_owned());
    }
    let bundle = model
        .workspace
        .content
        .project_sources
        .get_bundle(search.bundle_id)
        .ok_or_else(|| "The searched source bundle no longer exists.".to_owned())?;
    if bundle.language() != search.language
        || bundle.revision().get() != search.bundle_revision
        || bundle.closure_digest() != search.closure_digest
    {
        return Err(
            "The source bundle changed after this search began. Close it and search the current files again."
                .to_owned(),
        );
    }
    if search.include_generated_references
        && (netlist.revision != search.generated_reference_revision
            || ContentDigest::from_bytes(
                Sha256::digest(model.netlist_artifacts.generated_source.as_bytes()).into(),
            ) != search.generated_reference_digest)
    {
        return Err(
            "The generated reference deck changed after this search began. Close it and search the current project again."
                .to_owned(),
        );
    }
    Ok(bundle)
}

#[derive(Debug, Clone, Copy)]
struct SearchDocument<'a> {
    logical_path: &'a str,
    searchable_source: &'a str,
    complete_source: &'a str,
    base_byte: usize,
    editable: bool,
}

fn search_documents<'a>(
    model: &'a Model,
    code: &CodeWorkspaceRuntimeState,
    bundle: &'a ProjectSourceBundle,
    search: &'a CodeSourceSearchState,
) -> Result<Vec<SearchDocument<'a>>, String> {
    let mut documents = bundle_documents(bundle)
        .filter(|(logical_path, _)| match search.scope {
            CodeSourceSearchScope::ActiveLanguageProject => true,
            CodeSourceSearchScope::OpenDocuments => {
                crate::state::project_source_paths_equal(logical_path, &search.active_path)
                    || code.source_carets.keys().any(|(bundle_id, path)| {
                        *bundle_id == search.bundle_id
                            && crate::state::project_source_paths_equal(path, logical_path)
                    })
            }
            CodeSourceSearchScope::Selection | CodeSourceSearchScope::CurrentDocument => {
                crate::state::project_source_paths_equal(logical_path, &search.active_path)
            }
        })
        .map(|(logical_path, source)| SearchDocument {
            logical_path,
            searchable_source: source,
            complete_source: source,
            base_byte: 0,
            editable: true,
        })
        .collect::<Vec<_>>();
    if search.include_generated_references
        && search.scope == CodeSourceSearchScope::ActiveLanguageProject
        && !model.netlist_artifacts.generated_source.is_empty()
    {
        let generated = model.netlist_artifacts.generated_source.as_str();
        documents.push(SearchDocument {
            logical_path: GENERATED_REFERENCE_PATH,
            searchable_source: generated,
            complete_source: generated,
            base_byte: 0,
            editable: false,
        });
    }
    if search.scope != CodeSourceSearchScope::Selection {
        return Ok(documents);
    }
    let document = documents
        .into_iter()
        .next()
        .ok_or_else(|| "The selected source document is no longer available.".to_owned())?;
    let (start, end) = search
        .selection_char_range
        .filter(|(start, end)| start < end)
        .ok_or_else(|| "The captured source selection is empty.".to_owned())?;
    let start = char_to_byte(document.complete_source, start);
    let end = char_to_byte(document.complete_source, end);
    if start >= end {
        return Err("The captured source selection is empty or no longer valid.".to_owned());
    }
    Ok(vec![SearchDocument {
        searchable_source: &document.complete_source[start..end],
        base_byte: start,
        ..document
    }])
}

fn searchable_document_source<'a>(
    document: &SearchDocument<'a>,
    search: &CodeSourceSearchState,
) -> Result<Cow<'a, str>, String> {
    if search.include_comments {
        return Ok(Cow::Borrowed(document.searchable_source));
    }
    let masked = mask_source_comments(
        document.complete_source,
        search.language,
        document.logical_path,
    )?;
    let end = document
        .base_byte
        .checked_add(document.searchable_source.len())
        .ok_or_else(|| "The source-search selection range overflowed.".to_owned())?;
    let selected = masked
        .get(document.base_byte..end)
        .ok_or_else(|| "The source-search selection is no longer a valid text range.".to_owned())?;
    Ok(Cow::Owned(selected.to_owned()))
}

fn mask_source_comments(
    source: &str,
    language: ProjectSourceLanguage,
    logical_path: &str,
) -> Result<String, String> {
    if logical_path == GENERATED_REFERENCE_PATH {
        return mask_spice_comments(source);
    }
    match language {
        ProjectSourceLanguage::VerilogA => mask_verilog_comments(source),
    }
}

fn mask_verilog_comments(source: &str) -> Result<String, String> {
    let bytes = source.as_bytes();
    let mut output = bytes.to_vec();
    let mut in_string = false;
    let mut escaped = false;
    let mut block_comment = false;
    let mut index = 0_usize;
    while index < bytes.len() {
        if block_comment {
            if index + 1 < bytes.len() && bytes[index..index + 2] == *b"*/" {
                output[index] = b' ';
                output[index + 1] = b' ';
                block_comment = false;
                index += 2;
            } else {
                if !matches!(bytes[index], b'\r' | b'\n') {
                    output[index] = b' ';
                }
                index += 1;
            }
            continue;
        }
        if in_string {
            if bytes[index] == b'"' && !escaped {
                in_string = false;
            }
            escaped = !escaped && bytes[index] == b'\\';
            if bytes[index] != b'\\' {
                escaped = false;
            }
            index += 1;
            continue;
        }
        if bytes[index] == b'"' {
            in_string = true;
            index += 1;
        } else if index + 1 < bytes.len() && bytes[index..index + 2] == *b"//" {
            while index < bytes.len() && !matches!(bytes[index], b'\r' | b'\n') {
                output[index] = b' ';
                index += 1;
            }
        } else if index + 1 < bytes.len() && bytes[index..index + 2] == *b"/*" {
            output[index] = b' ';
            output[index + 1] = b' ';
            block_comment = true;
            index += 2;
        } else {
            index += 1;
        }
    }
    String::from_utf8(output)
        .map_err(|_| "Verilog-A source-search masking produced invalid UTF-8.".to_owned())
}

/// Blank every SPICE comment, keeping byte offsets, by the parser's own rule
/// (a `*` card, and core's inline `;`, `$`, `#` and `//` comments outside
/// quotes) so search never matches text the simulator does not read.
fn mask_spice_comments(source: &str) -> Result<String, String> {
    let mut output = source.as_bytes().to_vec();
    let mut offset = 0_usize;
    for line in source.split_inclusive('\n') {
        let body = line.trim_end_matches(['\r', '\n']);
        let comment_start = if body.trim_start().starts_with('*') {
            0
        } else {
            rspice_core::netlist::strip_spice_inline_comment(
                body,
                rspice_core::config::ExpressionDialect::Ngspice,
            )
            .len()
        };
        for byte in &mut output[offset + comment_start..offset + body.len()] {
            *byte = b' ';
        }
        offset += line.len();
    }
    String::from_utf8(output)
        .map_err(|_| "Netlist source-search masking produced invalid UTF-8.".to_owned())
}

fn char_to_byte(source: &str, char_index: usize) -> usize {
    source
        .char_indices()
        .nth(char_index)
        .map_or(source.len(), |(byte, _)| byte)
}

fn search_options(search: &CodeSourceSearchState) -> FindOptions {
    FindOptions {
        direction: FindDirection::Forward,
        match_case: search.match_case,
        whole_word: search.whole_symbol,
        regular_expression: search.regular_expression,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ProjectSourceOwner;
    use crate::workbench::code::{
        CodeFeature,
        tests::{CodeHarness, harness},
    };

    fn veriloga_reference() -> crate::state::CellViewRef {
        crate::state::CellViewRef::new("behavioral", "sensor_bridge", "veriloga")
    }

    fn veriloga_owner() -> ProjectSourceOwner {
        ProjectSourceOwner::cell_view(veriloga_reference())
    }

    /// A test app with a Verilog-A cell view open over a one-file bundle.
    fn veriloga_view_app() -> CodeHarness {
        let mut harness = harness();
        let reference = veriloga_reference();
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "sensor_bridge".to_owned());
        let mut cell = crate::state::Cell::new(&reference.cell);
        cell.add_view(view);
        let mut library = crate::state::Library::new(&reference.library);
        library.add_cell(cell);
        harness.model.library_manager.add_library(library);
        harness.model.workspace
            .content
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    veriloga_owner(),
                    ProjectSourceLanguage::VerilogA,
                    "sensor_bridge.va",
                    "module sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0;\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule\n",
                    [],
                    [],
                )
                .unwrap(),
            )
            .unwrap();
        harness
            .model
            .workspace
            .open_view(reference, crate::state::ViewType::VerilogA);
        harness
    }

    #[test]
    fn comment_masking_preserves_offsets_and_strings() {
        let veriloga = "analog V(out) <+ \"//gain\"; // gain µ\n/* gain */\n";
        let masked = mask_verilog_comments(veriloga).expect("Verilog-A mask");
        assert_eq!(masked.len(), veriloga.len());
        assert!(masked.contains("\"//gain\""));
        assert!(!masked.contains("/* gain */"));

        let spice = "* gain\nR1 a b 1k $ gain\nR2 a b \"$gain\"\n";
        let masked = mask_spice_comments(spice).expect("Netlist mask");
        assert!(masked.contains("R1 a b 1k"));
        assert!(masked.contains("\"$gain\""));
        assert!(!masked.lines().next().unwrap().contains("gain"));

        // The parser's rules, not a copy of them: `;` ends a card anywhere
        // outside quotes, `#` opens a comment card, `$` after a comma opens
        // one, and PSpice's `$G_` node names stay names.
        let spice = "R3 a b 1k;gain\n  # gain\r\nR4 a b 1k,$gain\nR5 $G_DPWR b 'x;gain'\n";
        let masked = mask_spice_comments(spice).expect("Netlist mask");
        assert_eq!(masked.len(), spice.len());
        let lines = masked.lines().collect::<Vec<_>>();
        assert_eq!(lines[0].trim_end(), "R3 a b 1k");
        assert_eq!(lines[1].trim(), "");
        assert!(masked.contains("\r\n"));
        assert_eq!(lines[2].trim_end(), "R4 a b 1k,");
        assert_eq!(lines[3], "R5 $G_DPWR b 'x;gain'");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn selection_scope_reports_absolute_ranges_and_replaces_only_the_capture() {
        let mut harness = veriloga_view_app();
        let (bundle_id, root) = {
            let bundle = harness
                .model
                .workspace
                .content
                .project_sources
                .bundle_for_owner(&veriloga_owner())
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "alpha gain beta gain gamma".to_owned(),
            )
            .unwrap();
        harness
            .state
            .code_workspace
            .source_selections
            .insert((bundle_id, root.clone()), (6, 10));
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        {
            let search = harness.state.code_workspace.source_search.as_mut().unwrap();
            search.scope = CodeSourceSearchScope::Selection;
            search.query = "gain".to_owned();
            search.replacement = "stage".to_owned();
        }
        let search = harness.state.code_workspace.source_search.clone().unwrap();
        let results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &search,
        )
        .unwrap();
        assert_eq!(results.total_matches, 1);
        assert_eq!(results.matches[0].byte_start, 6);
        assert_eq!(results.matches[0].byte_end, 10);
        assert_eq!((results.matches[0].line, results.matches[0].column), (1, 7));

        commit_source_search_replace(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        let bundle = harness
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(
            bundle.file_content(&root),
            Some("alpha stage beta gain gamma")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn unicode_case_alias_is_canonicalized_before_search_state_is_published() {
        let mut harness = veriloga_view_app();
        let (bundle_id, original_path) = {
            let bundle = harness
                .model
                .workspace
                .content
                .project_sources
                .bundle_for_owner(&veriloga_owner())
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        let retained_path = "Models/étage.va";
        harness
            .model
            .workspace
            .content
            .rename_project_source_bundle_file(bundle_id, &original_path, retained_path)
            .unwrap();
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                retained_path,
                "module étage; real gain; endmodule\n".to_owned(),
            )
            .unwrap();

        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            "MODELS/ÉTAGE.VA",
        )
        .unwrap();
        let search = harness.state.code_workspace.source_search.as_mut().unwrap();
        assert_eq!(search.active_path, retained_path);
        search.scope = CodeSourceSearchScope::CurrentDocument;
        search.query = "gain".to_owned();
        let search = search.clone();
        let results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &search,
        )
        .unwrap();

        assert_eq!(results.total_matches, 1);
        assert_eq!(results.matches[0].logical_path, retained_path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn replacement_rejects_stale_results_and_commits_atomically() {
        let mut harness = veriloga_view_app();
        let active_path = harness
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&veriloga_owner())
            .unwrap()
            .root()
            .logical_path()
            .to_owned();
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &active_path,
        )
        .unwrap();
        {
            let search = harness.state.code_workspace.source_search.as_mut().unwrap();
            search.query = "gain".to_owned();
            search.replacement = "stage_gain".to_owned();
        }
        let search = harness.state.code_workspace.source_search.clone().unwrap();
        let results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &search,
        )
        .unwrap();
        assert!(!results.matches.is_empty());
        commit_source_search_replace(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        let bundle = harness
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&veriloga_owner())
            .unwrap();
        assert!(bundle.root().content().contains("stage_gain"));
        assert!(harness.model.workspace.content.project_sources_dirty);
        let root = bundle.root().logical_path().to_owned();

        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &active_path,
        )
        .unwrap();
        let bundle_id = harness
            .state
            .code_workspace
            .source_search
            .as_ref()
            .unwrap()
            .bundle_id;
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module changed; endmodule\n".to_owned(),
            )
            .unwrap();
        let error =
            commit_source_search_replace(&mut CodeFeature::workspace_parts(&mut harness.cx()))
                .unwrap_err();
        assert!(error.contains("changed after this search began"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn replacement_can_exclude_comments_without_rewriting_them() {
        let mut harness = veriloga_view_app();
        let (bundle_id, root) = {
            let bundle = harness
                .model
                .workspace
                .content
                .project_sources
                .bundle_for_owner(&veriloga_owner())
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        let source = "module gain; // gain\nreal gain; /* gain */\nendmodule\n";
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(bundle_id, &root, source.to_owned())
            .unwrap();
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        {
            let search = harness.state.code_workspace.source_search.as_mut().unwrap();
            search.query = "gain".to_owned();
            search.replacement = "stage".to_owned();
            search.include_comments = false;
        }
        let results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            harness.state.code_workspace.source_search.as_ref().unwrap(),
        )
        .unwrap();
        assert_eq!(results.total_matches, 2);
        commit_source_search_replace(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        assert_eq!(
            harness
                .model
                .workspace
                .content
                .project_sources
                .get_bundle(bundle_id)
                .unwrap()
                .file_content(&root),
            Some("module stage; // gain\nreal stage; /* gain */\nendmodule\n")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn generated_references_are_find_only_and_stale_bound() {
        let mut harness = veriloga_view_app();
        harness.model.netlist_artifacts.generated_source =
            "XMODEL generated_reference\n".to_owned();
        harness.state.netlist.revision = harness.state.netlist.revision.wrapping_add(1);
        let active_path = harness
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&veriloga_owner())
            .unwrap()
            .root()
            .logical_path()
            .to_owned();
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &active_path,
        )
        .unwrap();
        {
            let search = harness.state.code_workspace.source_search.as_mut().unwrap();
            search.query = "generated_reference".to_owned();
            search.include_generated_references = true;
        }
        let search = harness.state.code_workspace.source_search.clone().unwrap();
        let results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &search,
        )
        .unwrap();
        assert_eq!(results.total_matches, 1);
        assert_eq!(results.read_only_matches, 1);
        assert_eq!(results.matches[0].logical_path, GENERATED_REFERENCE_PATH);
        assert!(!results.matches[0].editable);

        harness
            .model
            .netlist_artifacts
            .generated_source
            .push_str("* changed\n");
        harness.state.netlist.revision = harness.state.netlist.revision.wrapping_add(1);
        assert!(
            source_search_results_in(
                &harness.model,
                &harness.state.code_workspace,
                &harness.state.netlist,
                &harness.platform,
                &search
            )
            .unwrap_err()
            .contains("generated reference deck changed")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn search_streams_the_contract_limit_but_displays_only_five_hundred_rows() {
        let mut harness = veriloga_view_app();
        let (bundle_id, root) = {
            let bundle = harness
                .model
                .workspace
                .content
                .project_sources
                .bundle_for_owner(&veriloga_owner())
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(bundle_id, &root, "gain ".repeat(600))
            .unwrap();
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        harness
            .state
            .code_workspace
            .source_search
            .as_mut()
            .unwrap()
            .query = "gain".to_owned();
        let search = harness.state.code_workspace.source_search.clone().unwrap();
        let results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &search,
        )
        .unwrap();
        assert_eq!(results.total_matches, 600);
        assert_eq!(results.matches.len(), SOURCE_SEARCH_RESULT_LIMIT);
        assert!(results.display_truncated);
        assert!(!results.truncated);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn search_accepts_exactly_fifty_thousand_and_detects_one_more() {
        let mut harness = veriloga_view_app();
        let (bundle_id, root) = {
            let bundle = harness
                .model
                .workspace
                .content
                .project_sources
                .bundle_for_owner(&veriloga_owner())
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "scale_token ".repeat(SOURCE_SEARCH_STREAM_LIMIT),
            )
            .unwrap();
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        harness
            .state
            .code_workspace
            .source_search
            .as_mut()
            .unwrap()
            .query = "scale_token".to_owned();
        let exact = harness.state.code_workspace.source_search.clone().unwrap();
        let exact_results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &exact,
        )
        .unwrap();
        assert_eq!(exact_results.total_matches, SOURCE_SEARCH_STREAM_LIMIT);
        assert!(!exact_results.truncated);

        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "scale_token ".repeat(SOURCE_SEARCH_STREAM_LIMIT + 1),
            )
            .unwrap();
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        harness
            .state
            .code_workspace
            .source_search
            .as_mut()
            .unwrap()
            .query = "scale_token".to_owned();
        let over = harness.state.code_workspace.source_search.clone().unwrap();
        let over_results = source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &over,
        )
        .unwrap();
        assert_eq!(over_results.total_matches, SOURCE_SEARCH_STREAM_LIMIT);
        assert!(over_results.truncated);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn active_search_command_routes_to_the_visible_code_document_kind() {
        let mut harness = harness();
        open_active_source_search(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        assert!(harness.state.netlist.find.open);
        assert!(harness.state.code_workspace.source_search.is_none());
    }

    /// Reading match positions forward gives each the line, column and
    /// preview that reading the source from its start gives it, over line
    /// breaks of both kinds, blank lines and characters of several bytes.
    #[test]
    fn match_positions_read_forward_agree_with_reading_from_the_start() {
        let source = "  first \u{b5} line\r\nsecond\n\n  third gain  \nlast";
        let mut lines = LineCursor::new(source);
        for byte in (0..=source.len()).filter(|byte| source.is_char_boundary(*byte)) {
            let prefix = &source[..byte];
            let line = prefix.matches('\n').count() + 1;
            let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
            let column = source[line_start..byte].chars().count() + 1;
            let preview = source
                .lines()
                .nth(line - 1)
                .unwrap_or_default()
                .trim()
                .chars()
                .take(240)
                .collect::<String>();
            assert_eq!(lines.at(byte), (line, column, preview), "at byte {byte}");
        }
        assert_eq!(
            lines.at(0),
            (1, 1, "first \u{b5} line".to_owned()),
            "a position before the last one read is read from the start"
        );
    }

    /// The find dialog reads the sources once per search: a frame that draws
    /// the same search, or the same search with a replacement typed, shows
    /// what the first frame found; a new query reads them again; and once a
    /// source changes the search is not answered from what it found.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_search_reads_its_sources_once_until_the_search_or_a_source_changes() {
        let mut harness = veriloga_view_app();
        let (bundle_id, root) = {
            let bundle = harness
                .model
                .workspace
                .content
                .project_sources
                .bundle_for_owner(&veriloga_owner())
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        open_source_search(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        let mut search = harness.state.code_workspace.source_search.clone().unwrap();
        search.query = "gain".to_owned();
        let ctx = egui::Context::default();
        let kept = || {
            ctx.data(|data| data.get_temp::<KeptScan>(kept_scan_id()))
                .map(|kept| kept.scan)
        };
        let found = |harness: &CodeHarness, search: &CodeSourceSearchState| {
            kept_source_search_results_in(
                &ctx,
                &harness.model,
                &harness.state.code_workspace,
                &harness.state.netlist,
                &harness.platform,
                search,
            )
        };
        let read_now = |harness: &CodeHarness, search: &CodeSourceSearchState| {
            source_search_results_in(
                &harness.model,
                &harness.state.code_workspace,
                &harness.state.netlist,
                &harness.platform,
                search,
            )
        };

        let first = found(&harness, &search).unwrap();
        assert_eq!(first, read_now(&harness, &search).unwrap());
        assert_eq!(first.total_matches, 2);
        let scan = kept().expect("the first frame keeps what it found");
        assert_eq!(found(&harness, &search).unwrap(), first);
        assert!(
            Arc::ptr_eq(&kept().unwrap(), &scan),
            "the next frame shows what the first found"
        );
        let mut typed = search.clone();
        typed.replacement = "loss".to_owned();
        assert_eq!(found(&harness, &typed).unwrap(), first);
        assert!(
            Arc::ptr_eq(&kept().unwrap(), &scan),
            "a replacement typed finds nothing new"
        );

        let mut narrower = search.clone();
        narrower.query = "gain *".to_owned();
        let narrowed = found(&harness, &narrower).unwrap();
        assert_eq!(narrowed, read_now(&harness, &narrower).unwrap());
        assert_eq!(narrowed.total_matches, 1);
        assert!(
            !Arc::ptr_eq(&kept().unwrap(), &scan),
            "a new query reads again"
        );

        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module m; endmodule\n".to_owned(),
            )
            .unwrap();
        assert!(
            found(&harness, &narrower).is_err(),
            "a search of a source that changed is not answered from what it found"
        );

        forget_source_search_results(&ctx);
        assert!(
            kept().is_none(),
            "closing the search lets go of what it found"
        );
    }
}

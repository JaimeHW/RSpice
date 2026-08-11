//! Revision-bound search and atomic replacement across a project source bundle.

#[cfg(test)]
use std::borrow::Cow;

use sha2::{Digest as _, Sha256};

#[cfg(test)]
use crate::diagnostics::ConsoleMessage;
use crate::product::ContentDigest;
#[cfg(test)]
use crate::state::{
    BoundedFindMatches, FindDirection, FindOptions, ProjectSourceBundle,
    find_all_in_source_bounded, replace_source_ranges,
};
use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};
use crate::workbench::RSpiceApp;

use super::{CodeSourceSearchScope, CodeSourceSearchState};

#[cfg(test)]
pub(crate) const SOURCE_SEARCH_RESULT_LIMIT: usize = 500;
#[cfg(test)]
pub(crate) const SOURCE_SEARCH_STREAM_LIMIT: usize = 50_000;
#[cfg(test)]
pub(crate) const GENERATED_REFERENCE_PATH: &str = "rspice://generated/top-deck";

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CodeSourceSearchResult {
    pub logical_path: String,
    pub line: usize,
    pub column: usize,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_preview: String,
    pub editable: bool,
}

#[cfg(test)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CodeSourceSearchResults {
    pub matches: Vec<CodeSourceSearchResult>,
    pub total_matches: usize,
    pub editable_matches: usize,
    pub display_truncated: bool,
    pub truncated: bool,
    pub read_only_matches: usize,
}

pub(crate) fn open_source_search(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    active_path: &str,
) -> Result<(), String> {
    let owner = ProjectSourceOwner::code_workspace(language);
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    let bundle_id = bundle.id();
    open_source_search_in_bundle(app, language, bundle_id, active_path)
}

pub(crate) fn open_source_search_in_bundle(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    active_path: &str,
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
    let selection_char_range = app
        .state
        .ui
        .code_workspace
        .source_selections
        .get(&(language, active_path.clone()))
        .copied()
        .filter(|(start, end)| start < end);
    if app
        .state
        .ui
        .code_workspace
        .source_search
        .as_ref()
        .is_some_and(|search| {
            search.project_id == app.state.workspace.project.id()
                && search.bundle_id == bundle.id()
                && search.bundle_revision == bundle.revision().get()
                && search.closure_digest == bundle.closure_digest()
        })
    {
        if let Some(search) = app.state.ui.code_workspace.source_search.as_mut() {
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
    app.state.ui.code_workspace.source_search = Some(CodeSourceSearchState {
        project_id: app.state.workspace.project.id(),
        bundle_id: bundle.id(),
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        generated_reference_revision: app.state.ui.netlist.revision,
        generated_reference_digest: ContentDigest::from_bytes(
            Sha256::digest(app.state.ui.netlist.generated_source.as_bytes()).into(),
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

#[cfg(test)]
pub(crate) fn open_active_source_search(app: &mut RSpiceApp) -> Result<(), String> {
    match app.state.ui.code_workspace.page {
        super::CodeWorkspacePage::Netlist => {
            app.state.ui.netlist.find.open = true;
            Ok(())
        }
        super::CodeWorkspacePage::VerilogA => {
            let selected = super::selected_veriloga_editor_snapshot(app)?;
            let active_path = selected.active_path().to_owned();
            let bundle_id = selected.bundle_id();
            open_source_search_in_bundle(
                app,
                ProjectSourceLanguage::VerilogA,
                bundle_id,
                &active_path,
            )
        }
        super::CodeWorkspacePage::Automation => {
            let bundle = app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                    ProjectSourceLanguage::RSpiceAutomation,
                ))
                .ok_or_else(|| "The Automation source bundle no longer exists.".to_owned())?;
            let active_path = app
                .state
                .ui
                .code_workspace
                .automation
                .selected_file
                .as_deref()
                .filter(|path| bundle.contains_file(path))
                .unwrap_or(bundle.root().logical_path())
                .to_owned();
            open_source_search(app, ProjectSourceLanguage::RSpiceAutomation, &active_path)
        }
    }
}

#[cfg(test)]
pub(crate) fn source_search_results(
    app: &RSpiceApp,
    search: &CodeSourceSearchState,
) -> Result<CodeSourceSearchResults, String> {
    let bundle = current_bundle(app, search)?;
    if search.query.is_empty() {
        return Ok(CodeSourceSearchResults::default());
    }
    let options = search_options(search);
    let mut output = CodeSourceSearchResults::default();
    let documents = search_documents(app, bundle, search)?;
    for (index, document) in documents.iter().enumerate() {
        let searchable_source = searchable_document_source(document, search)?;
        let remaining = SOURCE_SEARCH_STREAM_LIMIT.saturating_sub(output.total_matches);
        if remaining == 0 {
            for later in &documents[index..] {
                let later_source = searchable_document_source(later, search)?;
                if find_all_in_source_bounded(&later_source, &search.query, options, 1)
                    .is_ok_and(|found| !found.matches().is_empty())
                {
                    output.truncated = true;
                    break;
                }
            }
            break;
        }
        let found: BoundedFindMatches =
            find_all_in_source_bounded(&searchable_source, &search.query, options, remaining)
                .map_err(|error| error.to_string())?;
        let editable = document.editable
            && super::source_bundle_document_is_editable(
                app,
                search.language,
                search.bundle_id,
                document.logical_path,
            );
        output.read_only_matches += found.matches().len() * usize::from(!editable);
        output.editable_matches += found.matches().len() * usize::from(editable);
        output.total_matches += found.matches().len();
        let display_remaining = SOURCE_SEARCH_RESULT_LIMIT.saturating_sub(output.matches.len());
        output
            .matches
            .extend(found.matches().iter().take(display_remaining).map(|found| {
                let relative_range = found.byte_range();
                let range = (document.base_byte + relative_range.start)
                    ..(document.base_byte + relative_range.end);
                let (line, column) = line_column_at_byte(document.complete_source, range.start);
                CodeSourceSearchResult {
                    logical_path: document.logical_path.to_owned(),
                    line,
                    column,
                    byte_start: range.start,
                    byte_end: range.end,
                    line_preview: source_line_preview(document.complete_source, line),
                    editable,
                }
            }));
        if found.truncated() {
            output.truncated = true;
            break;
        }
    }
    output.display_truncated = output.total_matches > output.matches.len() || output.truncated;
    Ok(output)
}

#[cfg(test)]
pub(crate) fn commit_source_search_replace(app: &mut RSpiceApp) -> Result<String, String> {
    let search = app
        .state
        .ui
        .code_workspace
        .source_search
        .clone()
        .ok_or_else(|| "No source search transaction is open.".to_owned())?;
    if let Some(reason) = super::source_file_mutation_block_reason(app, search.language) {
        return Err(reason.to_owned());
    }
    let results = source_search_results(app, &search)?;
    if results.truncated {
        return Err(format!(
            "More than {SOURCE_SEARCH_STREAM_LIMIT} matches exist. Narrow the search before replacing so the transaction remains bounded."
        ));
    }
    let bundle = current_bundle(app, &search)?;
    let options = search_options(&search);
    let mut replacements = Vec::new();
    let mut replacement_count = 0_usize;
    for document in search_documents(app, bundle, &search)? {
        if !document.editable
            || !super::source_bundle_document_is_editable(
                app,
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
            "The exact current source bundle contains no matches to replace.".to_owned()
        });
    }
    let changed_files = app
        .state
        .workspace
        .replace_project_source_bundle_files_transactionally(search.bundle_id, replacements)
        .map_err(|error| error.to_string())?;
    match search.language {
        ProjectSourceLanguage::VerilogA => super::veriloga::invalidate_veriloga_evidence(app),
        ProjectSourceLanguage::RSpiceAutomation => {
            super::automation::invalidate_automation_evidence(app);
        }
    }
    app.state.ui.code_workspace.source_search = None;
    let message = format!(
        "Replaced {replacement_count} match{} in {changed_files} project source file{}{}.",
        plural(replacement_count),
        plural(changed_files),
        if results.read_only_matches == 0 {
            String::new()
        } else {
            format!(
                "; left {} read-only match{} unchanged",
                results.read_only_matches,
                plural(results.read_only_matches)
            )
        }
    );
    app.state
        .push_user_message(ConsoleMessage::info(message.clone()));
    Ok(message)
}

#[cfg(test)]
fn current_bundle<'a>(
    app: &'a RSpiceApp,
    search: &CodeSourceSearchState,
) -> Result<&'a ProjectSourceBundle, String> {
    if search.project_id != app.state.workspace.project.id() {
        return Err("The active project changed while search results were open.".to_owned());
    }
    let bundle = app
        .state
        .workspace
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
        && (app.state.ui.netlist.revision != search.generated_reference_revision
            || ContentDigest::from_bytes(
                Sha256::digest(app.state.ui.netlist.generated_source.as_bytes()).into(),
            ) != search.generated_reference_digest)
    {
        return Err(
            "The generated reference deck changed after this search began. Close it and search the current project again."
                .to_owned(),
        );
    }
    Ok(bundle)
}

#[cfg(test)]
fn bundle_documents(bundle: &ProjectSourceBundle) -> impl Iterator<Item = (&str, &str)> {
    std::iter::once((bundle.root().logical_path(), bundle.root().content())).chain(
        bundle
            .files()
            .iter()
            .map(|file| (file.logical_path(), file.content())),
    )
}

#[cfg(test)]
#[derive(Debug, Clone, Copy)]
struct SearchDocument<'a> {
    logical_path: &'a str,
    searchable_source: &'a str,
    complete_source: &'a str,
    base_byte: usize,
    editable: bool,
}

#[cfg(test)]
fn search_documents<'a>(
    app: &'a RSpiceApp,
    bundle: &'a ProjectSourceBundle,
    search: &'a CodeSourceSearchState,
) -> Result<Vec<SearchDocument<'a>>, String> {
    let mut documents =
        bundle_documents(bundle)
            .filter(|(logical_path, _)| match search.scope {
                CodeSourceSearchScope::ActiveLanguageProject => true,
                CodeSourceSearchScope::OpenDocuments => {
                    crate::state::project_source_paths_equal(logical_path, &search.active_path)
                        || app.state.ui.code_workspace.source_carets.keys().any(
                            |(language, path)| {
                                *language == search.language
                                    && crate::state::project_source_paths_equal(path, logical_path)
                            },
                        )
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
        && !app.state.ui.netlist.generated_source.is_empty()
    {
        let generated = app.state.ui.netlist.generated_source.as_str();
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

#[cfg(test)]
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

#[cfg(test)]
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
        ProjectSourceLanguage::RSpiceAutomation => {
            let lower = logical_path.to_ascii_lowercase();
            if lower.ends_with(".yaml") || lower.ends_with(".yml") {
                mask_hash_comments(source, HashSyntax::Yaml)
            } else if lower.ends_with(".toml") || lower.ends_with(".lock") {
                mask_hash_comments(source, HashSyntax::Toml)
            } else {
                mask_hash_comments(source, HashSyntax::Python)
            }
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum HashSyntax {
    Python,
    Yaml,
    Toml,
}

#[cfg(test)]
fn mask_hash_comments(source: &str, syntax: HashSyntax) -> Result<String, String> {
    let bytes = source.as_bytes();
    let mut output = bytes.to_vec();
    let protected = (syntax == HashSyntax::Yaml).then(|| yaml_block_scalar_ranges(source));
    let mut protected_index = 0_usize;
    let mut quote = None;
    let mut triple = false;
    let mut escaped = false;
    let mut index = 0_usize;
    while index < bytes.len() {
        if let Some(ranges) = protected.as_ref() {
            while protected_index < ranges.len() && index >= ranges[protected_index].end {
                protected_index += 1;
            }
            if protected_index < ranges.len() && ranges[protected_index].contains(&index) {
                quote = None;
                triple = false;
                escaped = false;
                index = ranges[protected_index].end;
                continue;
            }
        }
        let byte = bytes[index];
        if let Some(delimiter) = quote {
            if triple
                && index + 2 < bytes.len()
                && bytes[index..index + 3] == [delimiter, delimiter, delimiter]
            {
                quote = None;
                triple = false;
                index += 3;
                continue;
            }
            if !triple && byte == delimiter && !escaped {
                if syntax == HashSyntax::Yaml
                    && delimiter == b'\''
                    && bytes.get(index + 1) == Some(&b'\'')
                {
                    index += 2;
                    continue;
                }
                quote = None;
                index += 1;
                continue;
            }
            escaped = !escaped && byte == b'\\' && delimiter == b'"';
            if byte != b'\\' {
                escaped = false;
            }
            index += 1;
            continue;
        }
        if matches!(byte, b'\'' | b'"') {
            let supports_triple = matches!(syntax, HashSyntax::Python | HashSyntax::Toml);
            triple = supports_triple
                && index + 2 < bytes.len()
                && bytes[index..index + 3] == [byte, byte, byte];
            quote = Some(byte);
            index += if triple { 3 } else { 1 };
            continue;
        }
        let yaml_comment_boundary =
            syntax != HashSyntax::Yaml || index == 0 || bytes[index - 1].is_ascii_whitespace();
        if byte == b'#' && yaml_comment_boundary {
            while index < bytes.len() && !matches!(bytes[index], b'\r' | b'\n') {
                output[index] = b' ';
                index += 1;
            }
            continue;
        }
        index += 1;
    }
    String::from_utf8(output)
        .map_err(|_| "Source-search comment masking produced invalid UTF-8.".to_owned())
}

#[cfg(test)]
fn yaml_block_scalar_ranges(source: &str) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut block_indent = None;
    let mut offset = 0_usize;
    for line in source.split_inclusive('\n') {
        let content = line.trim_end_matches(['\r', '\n']);
        let indentation = content.bytes().take_while(u8::is_ascii_whitespace).count();
        let blank = content.trim().is_empty();
        if let Some(parent_indent) = block_indent {
            if blank || indentation > parent_indent {
                ranges.push(offset..offset + line.len());
                offset += line.len();
                continue;
            }
            block_indent = None;
        }
        let without_comment = content
            .split_once(" #")
            .map_or(content, |(value, _)| value)
            .trim_end();
        let scalar = without_comment
            .rsplit_once(':')
            .map(|(_, value)| value.trim())
            .is_some_and(|value| {
                let mut chars = value.chars();
                matches!(chars.next(), Some('|' | '>'))
                    && chars.all(|character| matches!(character, '+' | '-' | '1'..='9'))
            });
        if scalar {
            block_indent = Some(indentation);
        }
        offset += line.len();
    }
    ranges
}

#[cfg(test)]
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

#[cfg(test)]
fn mask_spice_comments(source: &str) -> Result<String, String> {
    let mut output = source.as_bytes().to_vec();
    let mut offset = 0_usize;
    for line in source.split_inclusive('\n') {
        let bytes = line.as_bytes();
        let first = bytes
            .iter()
            .position(|byte| !byte.is_ascii_whitespace())
            .unwrap_or(bytes.len());
        let mut in_string = false;
        let mut escaped = false;
        let mut comment_start = (first < bytes.len() && bytes[first] == b'*').then_some(first);
        if comment_start.is_none() {
            for index in first..bytes.len() {
                let byte = bytes[index];
                if byte == b'"' && !escaped {
                    in_string = !in_string;
                }
                if !in_string
                    && matches!(byte, b'$' | b';')
                    && (index == first || bytes[index.saturating_sub(1)].is_ascii_whitespace())
                {
                    comment_start = Some(index);
                    break;
                }
                escaped = !escaped && byte == b'\\';
                if byte != b'\\' {
                    escaped = false;
                }
            }
        }
        if let Some(start) = comment_start {
            for index in start..bytes.len() {
                if !matches!(bytes[index], b'\r' | b'\n') {
                    output[offset + index] = b' ';
                }
            }
        }
        offset += line.len();
    }
    String::from_utf8(output)
        .map_err(|_| "Netlist source-search masking produced invalid UTF-8.".to_owned())
}

#[cfg(test)]
fn char_to_byte(source: &str, char_index: usize) -> usize {
    source
        .char_indices()
        .nth(char_index)
        .map_or(source.len(), |(byte, _)| byte)
}

#[cfg(test)]
fn line_column_at_byte(source: &str, byte: usize) -> (usize, usize) {
    let byte = byte.min(source.len());
    let prefix = &source[..byte];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    let column = source[line_start..byte].chars().count() + 1;
    (line, column)
}

#[cfg(test)]
fn search_options(search: &CodeSourceSearchState) -> FindOptions {
    FindOptions {
        direction: FindDirection::Forward,
        match_case: search.match_case,
        whole_word: search.whole_symbol,
        regular_expression: search.regular_expression,
    }
}

#[cfg(test)]
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

#[cfg(test)]
const fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "es" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_masking_preserves_offsets_strings_and_yaml_block_scalars() {
        let python = "value = 'not # comment'  # gain µ\ntext = \"\"\"# body\"\"\"\n";
        let masked = mask_hash_comments(python, HashSyntax::Python).expect("Python mask");
        assert_eq!(masked.len(), python.len());
        assert!(masked.contains("'not # comment'"));
        assert!(masked.contains("\"\"\"# body\"\"\""));
        assert!(!masked.contains("gain"));

        let yaml = "script: |\n  # literal content\n  gain\nvalue: gain # comment gain\n";
        let masked = mask_hash_comments(yaml, HashSyntax::Yaml).expect("YAML mask");
        assert!(masked.contains("# literal content"));
        assert!(masked.contains("value: gain"));
        assert!(!masked.contains("# comment gain"));

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
    }

    #[test]
    fn production_source_search_has_no_panic_shortcuts() {
        let production = crate::source_guard::production_half(include_str!("source_search.rs"));
        for shortcut in [
            ".expect(",
            ".unwrap(",
            "panic!(",
            "unreachable!(",
            "todo!(",
            "unimplemented!(",
        ] {
            assert!(
                !production.contains(shortcut),
                "production code must not contain {shortcut}"
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn selection_scope_reports_absolute_ranges_and_replaces_only_the_capture() {
        let mut app = RSpiceApp::test_instance();
        let (bundle_id, root) = {
            let bundle = app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                    ProjectSourceLanguage::VerilogA,
                ))
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "alpha gain beta gain gamma".to_owned(),
            )
            .unwrap();
        app.state
            .ui
            .code_workspace
            .source_selections
            .insert((ProjectSourceLanguage::VerilogA, root.clone()), (6, 10));
        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &root).unwrap();
        {
            let search = app.state.ui.code_workspace.source_search.as_mut().unwrap();
            search.scope = CodeSourceSearchScope::Selection;
            search.query = "gain".to_owned();
            search.replacement = "stage".to_owned();
        }
        let search = app.state.ui.code_workspace.source_search.clone().unwrap();
        let results = source_search_results(&app, &search).unwrap();
        assert_eq!(results.total_matches, 1);
        assert_eq!(results.matches[0].byte_start, 6);
        assert_eq!(results.matches[0].byte_end, 10);
        assert_eq!((results.matches[0].line, results.matches[0].column), (1, 7));

        commit_source_search_replace(&mut app).unwrap();
        let bundle = app
            .state
            .workspace
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
        let mut app = RSpiceApp::test_instance();
        let (bundle_id, original_path) = {
            let bundle = app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                    ProjectSourceLanguage::VerilogA,
                ))
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        let retained_path = "Models/étage.va";
        app.state
            .workspace
            .rename_project_source_bundle_file(bundle_id, &original_path, retained_path)
            .unwrap();
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                retained_path,
                "module étage; real gain; endmodule\n".to_owned(),
            )
            .unwrap();

        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, "MODELS/ÉTAGE.VA").unwrap();
        let search = app.state.ui.code_workspace.source_search.as_mut().unwrap();
        assert_eq!(search.active_path, retained_path);
        search.scope = CodeSourceSearchScope::CurrentDocument;
        search.query = "gain".to_owned();
        let search = search.clone();
        let results = source_search_results(&app, &search).unwrap();

        assert_eq!(results.total_matches, 1);
        assert_eq!(results.matches[0].logical_path, retained_path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn replacement_rejects_stale_results_and_commits_atomically() {
        let mut app = RSpiceApp::test_instance();
        let active_path = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::VerilogA,
            ))
            .unwrap()
            .root()
            .logical_path()
            .to_owned();
        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &active_path).unwrap();
        {
            let search = app.state.ui.code_workspace.source_search.as_mut().unwrap();
            search.query = "gain".to_owned();
            search.replacement = "stage_gain".to_owned();
        }
        let search = app.state.ui.code_workspace.source_search.clone().unwrap();
        let results = source_search_results(&app, &search).unwrap();
        assert!(!results.matches.is_empty());
        commit_source_search_replace(&mut app).unwrap();
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::VerilogA,
            ))
            .unwrap();
        assert!(bundle.root().content().contains("stage_gain"));
        assert!(app.state.workspace.project_sources_dirty);
        let root = bundle.root().logical_path().to_owned();

        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &active_path).unwrap();
        let bundle_id = app
            .state
            .ui
            .code_workspace
            .source_search
            .as_ref()
            .unwrap()
            .bundle_id;
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module changed; endmodule\n".to_owned(),
            )
            .unwrap();
        let error = commit_source_search_replace(&mut app).unwrap_err();
        assert!(error.contains("changed after this search began"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn replacement_can_exclude_comments_without_rewriting_them() {
        let mut app = RSpiceApp::test_instance();
        let (bundle_id, root) = {
            let bundle = app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                    ProjectSourceLanguage::VerilogA,
                ))
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        let source = "module gain; // gain\nreal gain; /* gain */\nendmodule\n";
        app.state
            .workspace
            .replace_project_source_bundle_file(bundle_id, &root, source.to_owned())
            .unwrap();
        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &root).unwrap();
        {
            let search = app.state.ui.code_workspace.source_search.as_mut().unwrap();
            search.query = "gain".to_owned();
            search.replacement = "stage".to_owned();
            search.include_comments = false;
        }
        let results = source_search_results(
            &app,
            app.state.ui.code_workspace.source_search.as_ref().unwrap(),
        )
        .unwrap();
        assert_eq!(results.total_matches, 2);
        commit_source_search_replace(&mut app).unwrap();
        assert_eq!(
            app.state
                .workspace
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
        let mut app = RSpiceApp::test_instance();
        app.state.ui.netlist.generated_source = "XMODEL generated_reference\n".to_owned();
        app.state.ui.netlist.revision = app.state.ui.netlist.revision.wrapping_add(1);
        let active_path = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::VerilogA,
            ))
            .unwrap()
            .root()
            .logical_path()
            .to_owned();
        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &active_path).unwrap();
        {
            let search = app.state.ui.code_workspace.source_search.as_mut().unwrap();
            search.query = "generated_reference".to_owned();
            search.include_generated_references = true;
        }
        let search = app.state.ui.code_workspace.source_search.clone().unwrap();
        let results = source_search_results(&app, &search).unwrap();
        assert_eq!(results.total_matches, 1);
        assert_eq!(results.read_only_matches, 1);
        assert_eq!(results.matches[0].logical_path, GENERATED_REFERENCE_PATH);
        assert!(!results.matches[0].editable);

        app.state
            .ui
            .netlist
            .generated_source
            .push_str("* changed\n");
        app.state.ui.netlist.revision = app.state.ui.netlist.revision.wrapping_add(1);
        assert!(
            source_search_results(&app, &search)
                .unwrap_err()
                .contains("generated reference deck changed")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn search_streams_the_contract_limit_but_displays_only_five_hundred_rows() {
        let mut app = RSpiceApp::test_instance();
        let (bundle_id, root) = {
            let bundle = app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                    ProjectSourceLanguage::VerilogA,
                ))
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        app.state
            .workspace
            .replace_project_source_bundle_file(bundle_id, &root, "gain ".repeat(600))
            .unwrap();
        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &root).unwrap();
        app.state
            .ui
            .code_workspace
            .source_search
            .as_mut()
            .unwrap()
            .query = "gain".to_owned();
        let search = app.state.ui.code_workspace.source_search.clone().unwrap();
        let results = source_search_results(&app, &search).unwrap();
        assert_eq!(results.total_matches, 600);
        assert_eq!(results.matches.len(), SOURCE_SEARCH_RESULT_LIMIT);
        assert!(results.display_truncated);
        assert!(!results.truncated);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn search_accepts_exactly_fifty_thousand_and_detects_one_more() {
        let mut app = RSpiceApp::test_instance();
        let (bundle_id, root) = {
            let bundle = app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&ProjectSourceOwner::code_workspace(
                    ProjectSourceLanguage::VerilogA,
                ))
                .unwrap();
            (bundle.id(), bundle.root().logical_path().to_owned())
        };
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "scale_token ".repeat(SOURCE_SEARCH_STREAM_LIMIT),
            )
            .unwrap();
        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &root).unwrap();
        app.state
            .ui
            .code_workspace
            .source_search
            .as_mut()
            .unwrap()
            .query = "scale_token".to_owned();
        let exact = app.state.ui.code_workspace.source_search.clone().unwrap();
        let exact_results = source_search_results(&app, &exact).unwrap();
        assert_eq!(exact_results.total_matches, SOURCE_SEARCH_STREAM_LIMIT);
        assert!(!exact_results.truncated);

        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "scale_token ".repeat(SOURCE_SEARCH_STREAM_LIMIT + 1),
            )
            .unwrap();
        open_source_search(&mut app, ProjectSourceLanguage::VerilogA, &root).unwrap();
        app.state
            .ui
            .code_workspace
            .source_search
            .as_mut()
            .unwrap()
            .query = "scale_token".to_owned();
        let over = app.state.ui.code_workspace.source_search.clone().unwrap();
        let over_results = source_search_results(&app, &over).unwrap();
        assert_eq!(over_results.total_matches, SOURCE_SEARCH_STREAM_LIMIT);
        assert!(over_results.truncated);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn active_search_command_routes_to_the_visible_code_document_kind() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
        open_active_source_search(&mut app).unwrap();
        let search = app.state.ui.code_workspace.source_search.as_ref().unwrap();
        assert_eq!(search.language, ProjectSourceLanguage::RSpiceAutomation);
        assert!(!search.active_path.is_empty());

        app.state.ui.code_workspace.source_search = None;
        app.state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        open_active_source_search(&mut app).unwrap();
        assert!(app.state.ui.netlist.find.open);
        assert!(app.state.ui.code_workspace.source_search.is_none());
    }
}

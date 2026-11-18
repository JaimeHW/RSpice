//! Netlist workspace workflows.
//!
//! The parts of validating the visible deck, and composing the source a run
//! executes. Validation is pinned to an exact content digest and project
//! revision, so an edit after validation invalidates it rather than being
//! carried along. The code feature's `netlist_validation` runs the
//! validation and the format over these parts.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::Model;
use crate::workbench::documents::netlist_document::NetlistDocumentState;

mod find;
mod platform;
#[cfg(test)]
mod tests;

pub use find::apply_netlist_find_step;
#[cfg(not(target_arch = "wasm32"))]
pub use platform::{show_deck_save_dialog, show_new_deck_dialog, show_open_deck_dialog};

pub const NETLIST_FILTER: (&str, &[&str]) = (
    "SPICE Deck",
    &[
        "cir", "sp", "spi", "spice", "net", "ckt", "cki", "scs", "lib", "inc", "txt",
    ],
);

pub fn normalize_owned_netlist_whitespace(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let preferred_eol = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut formatted = String::with_capacity(source.len().saturating_add(preferred_eol.len()));
    for (line, segment) in source.split_inclusive('\n').enumerate() {
        let (body, eol) = segment.strip_suffix("\r\n").map_or_else(
            || {
                segment
                    .strip_suffix('\n')
                    .map_or((segment, ""), |body| (body, "\n"))
            },
            |body| (body, "\r\n"),
        );
        // The first physical card is the circuit title and is user data, not
        // executable whitespace. Preserve it byte-for-byte.
        if line == 0 {
            formatted.push_str(body);
        } else {
            formatted.push_str(body.trim_end_matches([' ', '\t']));
        }
        formatted.push_str(eol);
    }
    if !source.ends_with('\n') {
        formatted.push_str(preferred_eol);
    }
    formatted
}

/// What follows the `+` of a continuation line; `None` for any other line.
fn continued(body: &str) -> Option<&str> {
    body.trim_start().strip_prefix('+')
}

/// A deck as Format writes it: what [`normalize_owned_netlist_whitespace`]
/// writes, with each `+` continuation line written `+ ` and its words, and
/// the `name=value` pairs of a card's continuation lines set in columns when
/// every word on them is one. The title, comment lines and inline comments
/// are kept as written, and so are Spectre lines (`spectre` for a Spectre
/// file, and the lines after `simulator lang=spectre`), which `+` does not
/// continue.
pub fn formatted_deck_text(source: &str, spectre: bool) -> String {
    let normalized = normalize_owned_netlist_whitespace(source);
    let lines = normalized
        .split_inclusive('\n')
        .map(|segment| {
            segment.strip_suffix("\r\n").map_or_else(
                || {
                    segment
                        .strip_suffix('\n')
                        .map_or((segment, ""), |body| (body, "\n"))
                },
                |body| (body, "\r\n"),
            )
        })
        .collect::<Vec<_>>();
    let mut spectre = spectre;
    let mut formatted = String::with_capacity(normalized.len());
    let mut index = 0;
    while let Some(&(body, eol)) = lines.get(index) {
        let head = body.trim_start().to_ascii_lowercase();
        if head.starts_with("simulator") {
            if head.contains("lang=spectre") {
                spectre = true;
            } else if head.contains("lang=spice") {
                spectre = false;
            }
        }
        if index == 0 || spectre || continued(body).is_none() {
            formatted.push_str(body);
            formatted.push_str(eol);
            index += 1;
            continue;
        }
        let run = lines[index..]
            .iter()
            .take_while(|(body, _)| continued(body).is_some())
            .collect::<Vec<_>>();
        index += run.len();
        let words = run
            .iter()
            .map(|(body, _)| {
                continued(body)
                    .unwrap_or_default()
                    .split_whitespace()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let pair = |word: &str| {
            word.find('=')
                .is_some_and(|at| at > 0 && at + 1 < word.len())
                && !word.contains(['{', '}', '\'', '"', ';', '$', '*'])
        };
        let tabular = run.len() > 1
            && words
                .iter()
                .all(|line| !line.is_empty() && line.iter().all(|word| pair(word)));
        let mut widths = Vec::<usize>::new();
        if tabular {
            for line in &words {
                for (column, word) in line.iter().enumerate() {
                    let width = word.chars().count();
                    match widths.get_mut(column) {
                        Some(widest) => *widest = (*widest).max(width),
                        None => widths.push(width),
                    }
                }
            }
        }
        for ((body, eol), line) in run.iter().zip(&words) {
            let rest = continued(body).unwrap_or_default().trim_start();
            if tabular {
                let mut cells = String::new();
                for (column, word) in line.iter().enumerate() {
                    if column + 1 < line.len() {
                        let pad = widths[column] - word.chars().count() + 1;
                        cells.push_str(word);
                        cells.extend(std::iter::repeat_n(' ', pad));
                    } else {
                        cells.push_str(word);
                    }
                }
                formatted.push_str("+ ");
                formatted.push_str(&cells);
            } else if rest.is_empty() {
                formatted.push('+');
            } else {
                formatted.push_str("+ ");
                formatted.push_str(rest);
            }
            formatted.push_str(eol);
        }
    }
    formatted
}

pub fn record_validation_failure(model: &mut Model, error: &str) {
    model.record(
        LogSeverity::Error,
        LogSource::Netlist,
        format!("Netlist validation failed: {error}"),
    );
}

/// Retain where each dependency resolved so the navigator and the Problems
/// pipeline can state it. The engine walked the chain; nothing here re-walks
/// it, so the two can never disagree.
pub fn record_include_resolutions(
    include_resolutions: &mut std::collections::BTreeMap<
        String,
        rspice_core::netlist::IncludeResolution,
    >,
    sealed: &[rspice_core::netlist::ResolvedIncludeDependency],
) {
    include_resolutions.clear();
    for dependency in sealed {
        include_resolutions.insert(
            dependency.requested_path().to_owned(),
            dependency.resolution().clone(),
        );
    }
}

/// Bind the editor's settled findings to the generated document they were
/// checked against. A deck is a file, not a document: its check is the
/// validation receipt the run reads.
pub fn acknowledge_canonical_validation(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
) -> Result<(), String> {
    if netlist.active_document
        != crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
    {
        return Ok(());
    }
    let diagnostics = netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.is_current())
        .map(|diagnostic| {
            let severity = match diagnostic.severity {
                crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint
                | crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
                    crate::state::DiagnosticSeverity::Info
                }
                crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => {
                    crate::state::DiagnosticSeverity::Warning
                }
                crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
                    crate::state::DiagnosticSeverity::Error
                }
            };
            crate::state::ValidationDiagnostic::try_new(
                severity,
                diagnostic.message.clone(),
                diagnostic.line.unwrap_or(0) + 1,
                diagnostic.column.unwrap_or(0) + 1,
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    let document = model
        .netlist_artifacts
        .generated_document
        .as_mut()
        .ok_or_else(|| "The active source has no netlist document.".to_owned())?;
    document
        .acknowledge_validation(document.content_digest(), diagnostics)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

/// Withdraw the generated document's validation when it is the document
/// shown.
pub fn invalidate_canonical_validation(model: &mut Model, netlist: &mut NetlistDocumentState) {
    if netlist.active_document
        != crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
    {
        return;
    }
    if let Some(document) = model.netlist_artifacts.generated_document.as_mut() {
        let _ = document.invalidate_validation(document.content_digest());
    }
}

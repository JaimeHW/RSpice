//! Publishing owned source bytes to a durable destination.
//!
//! Generated artifacts are exported through their own immutable-artifact
//! workflow and can never be promoted by a save.

#[cfg(not(target_arch = "wasm32"))]
use super::external_change::stage_external_netlist_change;
use super::import::sha256;
use super::*;

/// Publish the exact project-owned source bytes currently visible in the Code
/// workspace. Generated artifacts are exported through their separate
/// immutable-artifact workflow and can never be promoted by this operation.
///
/// `save_as` always asks for a destination. Ordinary Save reuses a reopenable
/// imported/native origin when one exists; browser downloads have no durable
/// path authority and therefore always use the user-agent download workflow.
pub(crate) fn save_owned_netlist_source(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    save_as: bool,
    save_as_encoding: crate::state::NetlistTextEncoding,
    commit_message: &str,
) -> bool {
    if state.ui.netlist.active_dependency_identity.is_some() {
        state.prompt(
            "Include files are saved with the project. Return to the top-level deck to save it \
             as a file.",
        );
        return false;
    }
    let commit_message = commit_message.trim();
    if commit_message.chars().any(char::is_control) || commit_message.chars().count() > 240 {
        state.prompt("A revision note is one line of at most 240 characters.");
        return false;
    }
    let commit_message = if commit_message.is_empty() {
        "Saved owned SPICE source"
    } else {
        commit_message
    };
    let Some(source) = state.workspace.content.netlist_source.clone() else {
        state.prompt("There is no editable deck to save.");
        return false;
    };
    if source != state.simulation.source.netlist_content {
        state.prompt("The deck was not saved: the editor and the project's deck differ.");
        return false;
    }
    let visible_digest = crate::state::content_digest(&source);
    let default_name = state
        .workspace
        .content
        .netlist_source_path
        .as_deref()
        .and_then(std::path::Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .or_else(|| {
            state
                .workspace
                .content
                .netlist_descriptor
                .as_ref()
                .map(|descriptor| {
                    std::path::Path::new(&descriptor.artifact_name)
                        .file_name()
                        .map_or_else(
                            || descriptor.artifact_name.clone(),
                            |name| name.to_string_lossy().into_owned(),
                        )
                })
        })
        .unwrap_or_else(|| "top.cir".to_owned());
    let reopenable_origin = io
        .saved_paths_are_reopenable()
        .then(|| state.workspace.content.netlist_source_path.clone())
        .flatten();
    let ordinary_save_to_origin = !save_as && reopenable_origin.is_some();
    let expected_external_sha256 = state
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.external_file_sha256);
    let source_encoding = state
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .map_or(crate::state::NetlistTextEncoding::Utf8, |descriptor| {
            descriptor.source_encoding
        });
    let encoding = if save_as {
        save_as_encoding
    } else {
        source_encoding
    };
    let encoded_source = match encoding.encode(&source) {
        Ok(bytes) => bytes,
        Err(error) => {
            record_save_failure(state, &error.to_string());
            return false;
        }
    };
    let picked = if ordinary_save_to_origin {
        Ok(reopenable_origin)
    } else {
        io.show_save_dialog(SaveDialogConfig {
            title: if save_as {
                "Save SPICE Source As"
            } else {
                "Save SPICE Source"
            },
            default_name: &default_name,
            filter_name: NETLIST_FILTER.0,
            filter_extensions: NETLIST_FILTER.1,
        })
    };

    let Some(mut path) = (match picked {
        Ok(path) => path,
        Err(error) => {
            record_save_failure(state, &error);
            return false;
        }
    }) else {
        return false;
    };
    if path.extension().is_none() {
        path.set_extension("cir");
    }

    #[cfg(not(target_arch = "wasm32"))]
    if ordinary_save_to_origin && let Some(expected) = expected_external_sha256 {
        match stage_external_netlist_change(state, &path, expected, &source) {
            Ok(true) => {
                state.prompt(
                    "The source changed outside RSpice. Review the local, external and merged \
                     versions before saving.",
                );
                return false;
            }
            Ok(false) => {}
            Err(error) => {
                record_save_failure(state, &error);
                return false;
            }
        }
    }

    let canonical_publication = io.saved_paths_are_reopenable();
    let next_owned_document = if canonical_publication
        && let Some(document) = state.ui.netlist.owned_document.as_ref()
    {
        let display_name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        let locator =
            crate::state::SourceLocator::try_new(path.display().to_string(), display_name)
                .and_then(|locator| locator.with_native_origin(path.display().to_string()));
        let mut next = document.clone();
        match locator.and_then(|locator| next.acknowledge_save(next.content_digest(), locator)) {
            Ok(_) => Some(next),
            Err(error) => {
                record_save_failure(state, &error.to_string());
                return false;
            }
        }
    } else {
        None
    };
    let next_descriptor = match (
        canonical_publication
            .then_some(state.workspace.content.netlist_descriptor.as_ref())
            .flatten(),
        next_owned_document.as_ref(),
    ) {
        (Some(descriptor), Some(document)) => {
            let mut descriptor = descriptor.clone();
            descriptor.source_encoding = encoding;
            descriptor.source_line_ending = crate::state::NetlistLineEnding::detect(&source);
            descriptor.external_file_sha256 = Some(sha256(&encoded_source));
            let document_revision = document.revision().get();
            if descriptor
                .save_history
                .last()
                .is_none_or(|record| record.document_revision < document_revision)
            {
                descriptor
                    .save_history
                    .push(crate::state::OwnedNetlistSaveRecord {
                        document_revision,
                        content_digest: document.content_digest(),
                        message: commit_message.to_owned(),
                    });
            }
            if let Err(error) = descriptor.retain_revision(document, commit_message) {
                record_save_failure(state, &error);
                return false;
            }
            Some(descriptor)
        }
        (None, None) => None,
        _ => {
            record_save_failure(state, "the deck's save record is incomplete");
            return false;
        }
    };

    let observed = if ordinary_save_to_origin {
        expected_external_sha256.map_or_else(
            || io.observe_destination(&path),
            |expected| io.observe_destination_at_sha256(&path, expected),
        )
    } else {
        io.observe_destination(&path)
    };
    let result = observed.and_then(|destination| {
        if encoding == crate::state::NetlistTextEncoding::Utf8 {
            io.write_text_file_observed(&destination, &source)
        } else {
            io.write_bytes_file_observed(&destination, &encoded_source, "text/plain")
        }
    });
    match result {
        Ok(()) => {
            if canonical_publication {
                state.workspace.content.netlist_source_path = Some(path.clone());
            }
            if let Some(descriptor) = next_descriptor {
                state.workspace.content.netlist_descriptor = Some(descriptor);
            }
            if let Some(document) = next_owned_document {
                state.workspace.content.netlist_document = Some(document.clone());
                state.ui.netlist.owned_document = Some(document);
            }
            if canonical_publication {
                state.ui.netlist.externally_saved_content_digest = Some(visible_digest);
                state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    crate::workbench::workflows::export_workflow::export_completion_message(
                        "SPICE source",
                        &path,
                        None,
                        io,
                    ),
                );
            } else {
                state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!(
                        "Downloaded a copy of the SPICE source: {}. The project's deck stays \
                         modified until the project is saved.",
                        path.display()
                    ),
                );
            }
            true
        }
        Err(error) => {
            #[cfg(not(target_arch = "wasm32"))]
            if ordinary_save_to_origin
                && let Some(expected) = expected_external_sha256
                && matches!(
                    stage_external_netlist_change(state, &path, expected, &source),
                    Ok(true)
                )
            {
                state.prompt(
                    "The source changed while it was being saved; nothing was overwritten. \
                     Review the external version.",
                );
                return false;
            }
            record_save_failure(state, &error);
            false
        }
    }
}

fn record_save_failure(state: &mut AppState, error: &str) {
    state.record(
        LogSeverity::Error,
        LogSource::Studio,
        format!("SPICE source save failed: {error}"),
    );
}

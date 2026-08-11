//! Taking ownership of generated source.
//!
//! Owned source is created from an explicit strategy, never implicitly by
//! typing into a generated view, and opening an existing owned document never
//! rewrites its bytes from the current generated primary.

use crate::diagnostics::ConsoleMessage;
use crate::state::NetlistDocumentId;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::{AppState, MessageCatalog, MessageId, RSpiceApp};

pub(super) fn open_ownership_dialog(
    state: &mut AppState,
    strategy: crate::state::OwnedNetlistEditStrategy,
) {
    if state.workspace.netlist_source.is_some() {
        let _ = open_owned_source(state);
        return;
    }
    if state.ui.netlist.generated_document.is_none() {
        state.push_user_message(ConsoleMessage::warning(
            "Generate a current primary artifact before creating owned source.",
        ));
        return;
    }
    let dialog = &mut state.ui.netlist.ownership_dialog;
    dialog.open = true;
    dialog.strategy = strategy;
    dialog.artifact_name = default_owned_artifact_name(strategy).to_owned();
    dialog.error = None;
}

fn default_owned_artifact_name(strategy: crate::state::OwnedNetlistEditStrategy) -> &'static str {
    match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => "top_override.sp",
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => "top_params.inc",
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => "top_includes.inc",
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => "top_analysis.sp",
    }
}

fn owned_strategy_label(
    messages: MessageCatalog,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> String {
    messages.text(match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
            MessageId::NetlistStrategyOwnedSource
        }
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
            MessageId::NetlistStrategyParameterOverride
        }
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
            MessageId::NetlistStrategyIncludeOverride
        }
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
            MessageId::NetlistStrategyAnalysisDeck
        }
    })
}

fn owned_strategy_description(
    messages: MessageCatalog,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> String {
    messages.text(match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
            MessageId::NetlistStrategyOwnedSourceDescription
        }
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
            MessageId::NetlistStrategyParameterOverrideDescription
        }
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
            MessageId::NetlistStrategyIncludeOverrideDescription
        }
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
            MessageId::NetlistStrategyAnalysisDeckDescription
        }
    })
}

pub(super) fn ownership_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.ownership_dialog.open {
        return;
    }
    let Some(generated) = app.state.ui.netlist.generated_document.as_ref() else {
        app.state.ui.netlist.ownership_dialog.open = false;
        return;
    };
    let base_revision = generated.revision().get();
    let mut dialog = app.state.ui.netlist.ownership_dialog.clone();
    let messages = app.state.ui.messages();
    let owned_deck = dialog.strategy == crate::state::OwnedNetlistEditStrategy::OwnedSource;
    let title = if owned_deck {
        messages.text(MessageId::NetlistOwnershipCreateSourceTitle)
    } else {
        messages.text(MessageId::NetlistOwnershipCreateOverrideTitle)
    };
    let primary = if owned_deck {
        messages.text(MessageId::NetlistOwnershipCreateSource)
    } else {
        messages.text(MessageId::NetlistOwnershipCreatePatch)
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistOwnershipEyebrow),
        title,
        primary,
    )
    .description(messages.text(MessageId::NetlistOwnershipDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost(messages.text(MessageId::CommonCancel))
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(messages.text(MessageId::NetlistOwnershipNotice));
            });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistArtifactName));
        let artifact_name = ui.add(
            egui::TextEdit::singleline(&mut dialog.artifact_name).desired_width(f32::INFINITY),
        );
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(messages.text(MessageId::NetlistBaseRevision));
            ui.monospace(base_revision.to_string());
        });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistEditStrategy));
        egui::ComboBox::from_id_salt("rspice.code.ownership-strategy")
            .selected_text(owned_strategy_label(messages, dialog.strategy))
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                for strategy in crate::state::OwnedNetlistEditStrategy::ALL {
                    ui.selectable_value(
                        &mut dialog.strategy,
                        strategy,
                        owned_strategy_label(messages, strategy),
                    );
                }
            });
        ui.label(owned_strategy_description(messages, dialog.strategy));
        if let Some(error) = &dialog.error {
            ui.add_space(6.0);
            ui.colored_label(Tokens::get(ctx).color.err, error);
        }
        Some(artifact_name.id)
    });
    match choice {
        DialogChoice::Primary => {
            match create_owned_source(&mut app.state, &dialog.artifact_name, dialog.strategy) {
                Ok(()) => dialog.open = false,
                Err(error) => dialog.error = Some(error),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.ownership_dialog = dialog;
}

pub(super) fn create_owned_source(
    state: &mut AppState,
    artifact_name: &str,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> Result<(), String> {
    let name = artifact_name.trim();
    if name.is_empty()
        || name != artifact_name
        || name.chars().any(char::is_control)
        || name.contains('/')
        || name.contains('\\')
    {
        return Err("Artifact name must be one trimmed file name.".to_owned());
    }
    if state.workspace.netlist_source.is_some() {
        return Err("An owned SPICE source artifact already exists.".to_owned());
    }
    let generated = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .ok_or_else(|| "No current generated artifact is available.".to_owned())?;
    let source = initial_owned_source(generated.source(), strategy);
    let mut owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .map_err(|error| error.to_string())?;
    owned
        .replace_editable_source(owned.content_digest(), source.as_bytes().to_vec())
        .map_err(|error| error.to_string())?;

    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_source_path = None;
    state.workspace.netlist_source_dirty = true;
    state.workspace.netlist_document = Some(owned.clone());
    let mut descriptor = crate::state::OwnedNetlistDescriptor {
        artifact_name: name.to_owned(),
        strategy,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::detect(&source),
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    };
    descriptor.retain_revision(&owned, "Created editable source baseline")?;
    state.workspace.netlist_descriptor = Some(descriptor);
    state.ui.netlist.owned_document = Some(owned);
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    state.ui.netlist.externally_saved_content_digest = None;
    state.simulation.netlist_content = source;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    crate::workbench::documents::netlist_document::invalidate_source_evidence(
        &mut state.ui.netlist,
    );
    Ok(())
}

fn initial_owned_source(
    generated: &str,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> String {
    match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => generated.to_owned(),
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => select_source_cards(
            generated,
            "* Parameter and option override derived from generated source",
            |head| matches!(head, ".param" | ".option" | ".options" | ".temp"),
        ),
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => select_source_cards(
            generated,
            "* Include-order override derived from generated source",
            |head| matches!(head, ".include" | ".inc" | ".lib" | ".veriloga"),
        ),
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => select_source_cards(
            generated,
            "* Analysis-only deck derived from generated source",
            is_analysis_card,
        ),
    }
}

fn select_source_cards(source: &str, heading: &str, select: impl Fn(&str) -> bool) -> String {
    let mut selected = Vec::<String>::new();
    let mut retain_continuation = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('+') {
            if retain_continuation {
                selected.push(line.to_owned());
            }
            continue;
        }
        let head = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        retain_continuation = select(&head);
        if retain_continuation {
            selected.push(line.to_owned());
        }
    }
    let mut result = heading.to_owned();
    result.push('\n');
    if selected.is_empty() {
        result.push_str("* No matching cards were present in the base revision.\n");
    } else {
        result.push_str(&selected.join("\n"));
        result.push('\n');
    }
    result
}

fn is_analysis_card(head: &str) -> bool {
    matches!(
        head,
        ".op"
            | ".tran"
            | ".ac"
            | ".dc"
            | ".noise"
            | ".tf"
            | ".pz"
            | ".sens"
            | ".four"
            | ".sp"
            | ".hb"
            | ".pss"
            | ".pac"
            | ".pnoise"
            | ".stb"
            | ".measure"
            | ".meas"
            | ".save"
            | ".probe"
    )
}

/// Open (or explicitly create) the project-owned source derived from the
/// immutable generated primary. Existing owned bytes are never overwritten.
pub(super) fn open_owned_source(state: &mut AppState) -> bool {
    if state.workspace.netlist_source.is_none() {
        return false;
    }
    if state.ui.netlist.owned_document.is_none() {
        let Some(generated) = state.ui.netlist.generated_document.as_ref() else {
            return false;
        };
        let source = state
            .workspace
            .netlist_source
            .as_deref()
            .unwrap_or_else(|| generated.source());
        let mut owned = match generated
            .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        {
            Ok(document) => document,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Could not create owned SPICE source: {error}"
                )));
                return false;
            }
        };
        let transition = if let Some(path) = state.workspace.netlist_source_path.as_ref() {
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            crate::state::SourceLocator::try_new(path.display().to_string(), display_name)
                .and_then(|locator| locator.with_native_origin(path.display().to_string()))
                .and_then(|locator| {
                    owned.import_source(owned.content_digest(), locator, source.as_bytes().to_vec())
                })
                .and_then(|_| owned.make_editable(owned.content_digest()))
        } else {
            owned.replace_editable_source(owned.content_digest(), source.as_bytes().to_vec())
        };
        if let Err(error) = transition {
            state.push_user_message(ConsoleMessage::error(format!(
                "Could not initialize owned SPICE source: {error}"
            )));
            return false;
        }
        state.ui.netlist.owned_document = Some(owned);
    }
    let Some(source) = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .map(|document| document.source().to_owned())
    else {
        return false;
    };
    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_document = state.ui.netlist.owned_document.clone();
    if state.workspace.netlist_descriptor.is_none() {
        let artifact_name = state
            .workspace
            .netlist_source_path
            .as_deref()
            .and_then(std::path::Path::file_name)
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "top_override.sp".to_owned());
        state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            artifact_name,
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::detect(&source),
            imported_dialect: None,
            compatibility_reviewed: false,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
        });
    }
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = source;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    crate::workbench::documents::netlist_document::invalidate_source_evidence(
        &mut state.ui.netlist,
    );
    true
}

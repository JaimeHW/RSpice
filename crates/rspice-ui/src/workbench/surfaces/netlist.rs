//! Mockup-owned Code & Automation netlist document surface.
//!
//! The center well is deliberately flat: one 33-point document toolbar over
//! an exact-entry editor. Generated and owned source are independent retained
//! documents and switching between them never deletes either one.

use egui::{Align, Layout, Ui, vec2};

use crate::common::{AppState, ConsoleMessage, RSpiceApp};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};

use super::super::code_workspace::{
    GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
    NetlistDocument, NetlistDocumentId,
};
use super::super::design_system::{WorkbenchIcon, empty_state, icon_button};
use super::super::netlist_document::{ActiveNetlistDocument, source_content_digest};

const CODE_TOOLBAR_HEIGHT: f32 = 33.0;
const CODE_TOOLBAR_PADDING_X: f32 = 8.0;
const CODE_TOOLBAR_GAP: f32 = 5.0;
const CODE_TOOLBAR_COMPACT_BREAKPOINT: f32 = 720.0;
const PHONE_BREAKPOINT: f32 = 560.0;
const PHONE_PRIMARY_WIDTH: f32 = 154.0;
const PHONE_ACTION_WIDTH: f32 = PHONE_PRIMARY_WIDTH + CODE_TOOLBAR_GAP + 28.0;

pub(super) fn prepare_workspace(app: &mut RSpiceApp) {
    reconcile_documents(app);
    super::super::netlist_document::prepare(&mut app.state);
}

pub(super) fn show_prepared(ui: &mut Ui, app: &mut RSpiceApp) {
    code_toolbar(ui, app);
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        if generated_primary_unavailable(&app.state) {
            empty_state(
                ui,
                WorkbenchIcon::Netlist,
                "Generated netlist unavailable",
                app.state
                    .ui
                    .netlist
                    .generation_error
                    .as_deref()
                    .unwrap_or("Add a circuit before generating the primary netlist."),
            );
        } else {
            super::super::netlist_document::show_editor(ui, &mut app.state);
        }
    });
    find_replace_window(ui.ctx(), app);
    ownership_dialog_window(ui.ctx(), app);
    comparison_dialog_window(ui.ctx(), app);
    save_source_dialog_window(ui.ctx(), app);
    export_generated_dialog_window(ui.ctx(), app);
}

fn generated_primary_unavailable(state: &AppState) -> bool {
    state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && !generated_primary_ready(state)
}

fn generated_primary_ready(state: &AppState) -> bool {
    state.ui.netlist.generated_document.is_some() && !state.ui.netlist.generated_source.is_empty()
}

fn active_document_available(state: &AppState) -> bool {
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => generated_primary_ready(state),
        ActiveNetlistDocument::OwnedSource => state.workspace.netlist_source.is_some(),
        ActiveNetlistDocument::GeneratedDiff => !state.ui.netlist.generated_diff_source.is_empty(),
    }
}

fn generation_block_reason(state: &AppState) -> &str {
    state
        .ui
        .netlist
        .generation_error
        .as_deref()
        .unwrap_or("Generate a primary netlist before using this action.")
}

/// Rebuild the immutable generated primary whenever any authoritative project
/// input changes. Generation happens into a temporary string; a failed build
/// retains the prior valid artifact and marks it stale instead of publishing
/// partial bytes.
fn reconcile_documents(app: &mut RSpiceApp) {
    if app.state.ui.netlist.owned_document.is_none() {
        app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
    }
    if !app.state.ui.netlist.active_document_initialized {
        app.state.ui.netlist.active_document = if app.state.workspace.netlist_source.is_some() {
            ActiveNetlistDocument::OwnedSource
        } else {
            ActiveNetlistDocument::Generated
        };
        app.state.ui.netlist.active_document_initialized = true;
    }

    refresh_generated_artifact(app);

    let projected = match app.state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => Some(app.state.ui.netlist.generated_source.clone()),
        ActiveNetlistDocument::OwnedSource => app.state.workspace.netlist_source.clone(),
        ActiveNetlistDocument::GeneratedDiff => {
            Some(app.state.ui.netlist.generated_diff_source.clone())
        }
    };
    if let Some(projected) = projected
        && app.state.simulation.netlist_content != projected
    {
        app.state.simulation.netlist_content = projected;
        app.state.ui.netlist.revision = app.state.ui.netlist.revision.wrapping_add(1);
        super::super::netlist_document::invalidate_source_evidence(&mut app.state.ui.netlist);
    }
}

fn refresh_generated_artifact(app: &mut RSpiceApp) {
    let input_digest =
        match crate::common::project_lifecycle::generated_netlist_input_digest(&app.state) {
            Ok(digest) => digest,
            Err(error) => {
                app.state.ui.netlist.current_generation_input_digest = None;
                app.state.ui.netlist.generation_error = Some(error.to_string());
                return;
            }
        };
    let authoritative_input_changed = app
        .state
        .ui
        .netlist
        .current_generation_input_digest
        .is_some_and(|previous| previous != input_digest);
    app.state.ui.netlist.current_generation_input_digest = Some(input_digest);
    if authoritative_input_changed {
        app.simulation_controller.clear_prepared_run();
        super::super::netlist_document::invalidate_source_evidence(&mut app.state.ui.netlist);
        if let Some(document) = app.state.ui.netlist.generated_document.as_mut() {
            let _ = document.invalidate_validation(document.content_digest());
        }
        if let Some(document) = app.state.ui.netlist.owned_document.as_mut() {
            let _ = document.invalidate_validation(document.content_digest());
            app.state.workspace.netlist_document = Some(document.clone());
        }
    }
    if app.state.ui.netlist.generated_input_digest == Some(input_digest)
        && !app.state.ui.netlist.generated_source.is_empty()
    {
        return;
    }
    if app.state.schematic.components.is_empty() {
        app.state.ui.netlist.generation_error =
            Some("Add a circuit before generating the primary netlist.".to_owned());
        return;
    }

    let previous_message_count = app.state.log_buffer.len();
    let generated = crate::common::menu_bar::build_menu_netlist(
        &mut app.state,
        crate::io::NetlistFormat::Spice,
    );
    match generated {
        Some(mut source) => {
            if let Err(error) = append_project_veriloga_directive(&app.state, &mut source) {
                app.state.ui.netlist.generation_error = Some(error);
                return;
            }
            match publish_generated_document(&app.state, input_digest, source) {
                Ok((generated_document, owned_document)) => {
                    if let Some(previous) = app.state.ui.netlist.generated_document.as_ref()
                        && previous.generated_artifact().content_digest()
                            != generated_document.generated_artifact().content_digest()
                    {
                        let predecessor = previous.generated_artifact().clone();
                        if app
                            .state
                            .ui
                            .netlist
                            .generated_history
                            .last()
                            .is_none_or(|artifact| {
                                artifact.content_digest() != predecessor.content_digest()
                            })
                        {
                            app.state.ui.netlist.generated_history.push(predecessor);
                            const RETAINED_GENERATED_REVISIONS: usize = 16;
                            let excess = app
                                .state
                                .ui
                                .netlist
                                .generated_history
                                .len()
                                .saturating_sub(RETAINED_GENERATED_REVISIONS);
                            if excess > 0 {
                                app.state.ui.netlist.generated_history.drain(..excess);
                            }
                        }
                    }
                    app.state.ui.netlist.generated_source =
                        generated_document.generated_artifact().source().to_owned();
                    app.state.ui.netlist.generated_document = Some(generated_document);
                    app.state.workspace.netlist_document = owned_document.clone();
                    app.state.ui.netlist.owned_document = owned_document;
                    app.state.ui.netlist.generated_input_digest = Some(input_digest);
                    app.state.ui.netlist.generation_error = None;
                }
                Err(error) => {
                    app.state.ui.netlist.generation_error = Some(error);
                }
            }
        }
        None => {
            let detail = app
                .state
                .log_buffer
                .entries()
                .skip(previous_message_count)
                .filter(|entry| entry.severity == crate::panels::LogSeverity::Error)
                .last()
                .map(|entry| entry.message.clone())
                .unwrap_or_else(|| "Netlist generation failed; review Problems.".to_owned());
            app.state.ui.netlist.generation_error = Some(detail);
        }
    }
}

fn append_project_veriloga_directive(state: &AppState, source: &mut String) -> Result<(), String> {
    let Some(document) = state
        .workspace
        .project_sources
        .get(crate::state::ProjectSourceLanguage::VerilogA)
    else {
        return Ok(());
    };
    let source_key = super::super::code_workspace::project_veriloga_source_key(
        state.workspace.project.id(),
        document,
    );
    let receipt = state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .filter(|receipt| {
            receipt.token.project_id == state.workspace.project.id()
                && receipt.token.revision == document.revision().get()
                && receipt.token.content_digest == document.content_digest()
        })
        .cloned()
        .map(Ok)
        .unwrap_or_else(|| {
            if !document.validation_is_current() {
                return Err(format!(
                    "Compile the exact current project Verilog-A source '{}' before generating the executable netlist",
                    document.file_name()
                ));
            }
            super::super::code_workspace::compile_project_source_receipt(
                state.workspace.project.id(),
                document,
            )
            .map_err(|diagnostics| {
                diagnostics
                    .first()
                    .map(|diagnostic| {
                        format!(
                            "Could not rebuild validated Verilog-A source '{}': {}: {}",
                            document.file_name(),
                            diagnostic.message,
                            diagnostic.detail
                        )
                    })
                    .unwrap_or_else(|| {
                        format!(
                            "Could not rebuild validated Verilog-A source '{}': the compiler returned no diagnostic",
                            document.file_name()
                        )
                    })
            })
        })?;
    super::super::code_workspace::append_project_veriloga_directive(
        source,
        &source_key,
        &receipt.module_name,
    );
    Ok(())
}

fn publish_generated_document(
    state: &AppState,
    input_digest: crate::product::ContentDigest,
    source: String,
) -> Result<(NetlistDocument, Option<NetlistDocument>), String> {
    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-generator/v1",
        GenerationInput::new(state.workspace.project.revision(), input_digest),
    )
    .map_err(|error| error.to_string())?;
    let source_map = generated_source_map(state, &source)?;
    let dependencies = generated_project_source_dependencies(state, &source)?;
    let artifact =
        GeneratedArtifact::try_from_utf8(provenance, source.into_bytes(), dependencies, source_map)
            .map_err(|error| error.to_string())?;

    let generated_document = if let Some(existing) = &state.ui.netlist.generated_document {
        let mut next = existing.clone();
        next.update_generated_artifact(
            existing.generated_artifact().content_digest(),
            artifact.clone(),
        )
        .map_err(|error| error.to_string())?;
        next
    } else {
        NetlistDocument::from_generated(NetlistDocumentId::new(), artifact.clone())
            .map_err(|error| error.to_string())?
    };

    // An owned document retains the exact generated base from which it was
    // created. Current generated revisions live in the primary registry and
    // compare history; silently rebasing owned source would destroy the
    // three-way ownership contract.
    let owned_document = state.ui.netlist.owned_document.clone();
    Ok((generated_document, owned_document))
}

fn generated_project_source_dependencies(
    state: &AppState,
    source: &str,
) -> Result<Vec<super::super::code_workspace::DependencyMetadata>, String> {
    let Some(document) = state
        .workspace
        .project_sources
        .get(crate::state::ProjectSourceLanguage::VerilogA)
    else {
        return Ok(Vec::new());
    };
    let source_key = super::super::code_workspace::project_veriloga_source_key(
        state.workspace.project.id(),
        document,
    );
    let include_index = super::super::code_workspace::parse_include_directives(source)
        .iter()
        .position(|directive| directive.locator() == source_key)
        .ok_or_else(|| {
            "Generated source is missing its authenticated project Verilog-A directive".to_owned()
        })?;
    let locator = super::super::code_workspace::SourceLocator::try_new(
        source_key.clone(),
        document.file_name(),
    )
    .map_err(|error| error.to_string())?;
    let dependency = super::super::code_workspace::DependencyMetadata::unresolved_direct_to(
        include_index,
        source_key,
        locator,
    )
    .map_err(|error| error.to_string())?
    .resolve_utf8(document.content().as_bytes().to_vec())
    .map_err(|error| error.to_string())?;
    Ok(vec![dependency])
}

fn generated_source_map(
    state: &AppState,
    source: &str,
) -> Result<Vec<GeneratedSourceMapEntry>, String> {
    let top = state.workspace.active_view.clone();
    let mut current = top.clone();
    let mut entries = Vec::with_capacity(source.lines().count());

    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        let tokens = trimmed.split_whitespace().collect::<Vec<_>>();
        if tokens
            .first()
            .is_some_and(|head| head.eq_ignore_ascii_case(".subckt"))
            && let Some(cell) = tokens.get(1)
        {
            current = state
                .workspace
                .schematic_buffers
                .keys()
                .find_map(|key| {
                    let mut segments = key.split('/');
                    let library = segments.next()?;
                    let candidate_cell = segments.next()?;
                    let view = segments.next()?;
                    (candidate_cell.eq_ignore_ascii_case(cell))
                        .then(|| crate::state::CellViewRef::new(library, candidate_cell, view))
                })
                .unwrap_or_else(|| {
                    crate::state::CellViewRef::new(&top.library, *cell, "schematic")
                });
        }

        let (instance_identity, component_identity) = source_line_component(state, &current, line)
            .map_or((None, None), |(instance, component)| {
                (Some(instance), Some(component))
            });
        entries.push(
            GeneratedSourceMapEntry::try_new(
                index + 1,
                format!("{}/{}", current.library, current.cell),
                current.key(),
                instance_identity,
                component_identity,
            )
            .map_err(|error| error.to_string())?,
        );

        if tokens
            .first()
            .is_some_and(|head| head.eq_ignore_ascii_case(".ends"))
        {
            current.clone_from(&top);
        }
    }
    Ok(entries)
}

fn source_line_component(
    state: &AppState,
    reference: &crate::state::CellViewRef,
    line: &str,
) -> Option<(String, String)> {
    let token = line.split_whitespace().next()?;
    if token.starts_with(['.', '*']) {
        return None;
    }
    let schematic = state.workspace.schematic_buffers.get(&reference.key())?;
    schematic.components.iter().find_map(|component| {
        let base = component.spice_instance_name();
        let prefix = component.kind.spice_prefix();
        let emitted = if prefix.is_empty()
            || base.is_empty()
            || (base.len() >= prefix.len() && base[..prefix.len()].eq_ignore_ascii_case(prefix))
        {
            base
        } else {
            format!("{prefix}{base}")
        };
        token.eq_ignore_ascii_case(&emitted).then(|| {
            (
                emitted,
                format!("{}/component/{}", reference.key(), component.id),
            )
        })
    })
}

fn code_toolbar(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let compact = code_toolbar_compact(width);
    let phone = width <= PHONE_BREAKPOINT;
    let (rect, _) = ui.allocate_exact_size(vec2(width, CODE_TOOLBAR_HEIGHT), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );

    let content = rect.shrink2(vec2(CODE_TOOLBAR_PADDING_X, 0.0));
    let active = app.state.ui.netlist.active_document;
    let generated_ready = generated_primary_ready(&app.state);
    let active_available = active_document_available(&app.state);
    let action_width: f32 = if compact {
        PHONE_ACTION_WIDTH
    } else {
        match active {
            ActiveNetlistDocument::Generated => {
                if app.state.workspace.netlist_source.is_some() {
                    175.0
                } else {
                    342.0
                }
            }
            ActiveNetlistDocument::OwnedSource => 348.0,
            ActiveNetlistDocument::GeneratedDiff => 152.0,
        }
    };
    let action_width = action_width.min(content.width());
    let right_rect = egui::Rect::from_min_max(
        egui::pos2(content.right() - action_width, content.top()),
        content.right_bottom(),
    );
    let left_rect = egui::Rect::from_min_max(
        content.left_top(),
        egui::pos2(
            (right_rect.left() - CODE_TOOLBAR_GAP).max(content.left()),
            content.bottom(),
        ),
    );
    let language = match active {
        ActiveNetlistDocument::Generated => "SPICE · GENERATED · IMMUTABLE · SOURCE MAPPED",
        ActiveNetlistDocument::OwnedSource => {
            app.state.workspace.netlist_descriptor.as_ref().map_or(
                "SPICE · OWNED · EDITABLE",
                |descriptor| match descriptor.strategy {
                    crate::state::OwnedNetlistEditStrategy::OwnedSource => {
                        "SPICE · OWNED · EDITABLE"
                    }
                    crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
                        "SPICE INCLUDE · PARAMETER OVERRIDE · EDITABLE"
                    }
                    crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
                        "SPICE INCLUDE · INCLUDE ORDER · EDITABLE"
                    }
                    crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
                        "SPICE · ANALYSIS DECK · EDITABLE"
                    }
                },
            )
        }
        ActiveNetlistDocument::GeneratedDiff => "SPICE DIFF · GENERATED COMPARISON · IMMUTABLE",
    };
    let (status, status_tone) = document_syntax_status(&app.state);
    let status_color = match status_tone {
        DocumentStatusTone::Valid => t.color.ok,
        DocumentStatusTone::Warning => t.color.warn,
        DocumentStatusTone::Error => t.color.err,
    };
    let status_visible = toolbar_status_visible(phone, status_tone);
    let advisory_count = (!compact).then(|| {
        app.state
            .ui
            .netlist
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.severity != super::super::netlist_document::DiagnosticSeverity::Error
            })
            .count()
            + app
                .state
                .ui
                .netlist
                .validation
                .as_ref()
                .map_or(0, |receipt| receipt.advisory_count)
            + usize::from(app.state.ui.netlist.validation_error.is_some())
    });
    let advisory_label = advisory_count
        .map(|count| format!("{count} advisor{}", if count == 1 { "y" } else { "ies" }));
    let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let label_width = |label: &str, color| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), status_font.clone(), color)
            .size()
            .x
    };
    let mut status_width = if status_visible {
        11.0 + label_width(&status, status_color)
    } else {
        0.0
    };
    if let Some(label) = advisory_label.as_deref() {
        status_width += CODE_TOOLBAR_GAP + 11.0 + label_width(label, t.color.text_faint);
    }
    status_width = status_width.min(left_rect.width());
    let status_rect = egui::Rect::from_min_max(
        egui::pos2(left_rect.right() - status_width, left_rect.top()),
        left_rect.right_bottom(),
    );
    let language_rect = egui::Rect::from_min_max(
        left_rect.left_top(),
        egui::pos2(
            (status_rect.left() - CODE_TOOLBAR_GAP).max(left_rect.left()),
            left_rect.bottom(),
        ),
    );
    if language_rect.width() > 0.0 {
        let mut language_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(language_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        language_ui.add(
            egui::Label::new(
                egui::RichText::new(language)
                    .font(status_font.clone())
                    .color(t.color.text_dim),
            )
            .truncate(),
        );
    }
    if status_width > 0.0 {
        let mut status_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(status_rect)
                .layout(Layout::right_to_left(Align::Center)),
        );
        status_ui.spacing_mut().item_spacing.x = CODE_TOOLBAR_GAP;
        status_ui.with_layout(Layout::right_to_left(Align::Center), |bar| {
            if let Some(advisory_count) = advisory_count {
                code_status(
                    bar,
                    advisory_label.as_deref().unwrap_or_default(),
                    if advisory_count == 0 {
                        t.color.text_faint
                    } else {
                        t.color.warn
                    },
                );
            }
            if status_visible {
                code_status(bar, &status, status_color);
            }
        });
    }

    let mut action = None;
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(right_rect)
            .layout(Layout::right_to_left(Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = CODE_TOOLBAR_GAP;
    if compact {
        actions.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let mut find_clicked = false;
            ui.add_enabled_ui(active_available, |ui| {
                find_clicked = icon_button(
                    ui,
                    WorkbenchIcon::Search,
                    "Find in active netlist document",
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked();
            })
            .response
            .on_disabled_hover_text("Open or generate a netlist document before searching.");
            if find_clicked {
                action = Some(NetlistToolbarAction::Find);
            }
            match active {
                ActiveNetlistDocument::Generated => {
                    let (label, candidate) = if app.state.workspace.netlist_source.is_some() {
                        ("Open editable source", NetlistToolbarAction::OpenOwned)
                    } else {
                        (
                            "Create editable source from generated netlist…",
                            NetlistToolbarAction::OpenOwnershipDialog(
                                crate::state::OwnedNetlistEditStrategy::OwnedSource,
                            ),
                        )
                    };
                    let primary_ready =
                        app.state.workspace.netlist_source.is_some() || generated_ready;
                    if ui
                        .add_enabled(
                            primary_ready,
                            egui::Button::new(label)
                                .truncate()
                                .min_size(vec2(PHONE_PRIMARY_WIDTH, 28.0)),
                        )
                        .on_disabled_hover_text(generation_block_reason(&app.state))
                        .clicked()
                    {
                        action = Some(candidate);
                    }
                }
                ActiveNetlistDocument::OwnedSource => {
                    let save_ready = owned_source_save_ready(app);
                    if ui
                        .add_enabled(
                            save_ready,
                            egui::Button::new("Save source deck")
                                .truncate()
                                .min_size(vec2(PHONE_PRIMARY_WIDTH, 28.0)),
                        )
                        .on_disabled_hover_text(
                            "Validate a modified source revision before saving it",
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::Save);
                    }
                }
                ActiveNetlistDocument::GeneratedDiff => {
                    if ui
                        .add_sized(
                            [PHONE_PRIMARY_WIDTH, 28.0],
                            egui::Button::new("Return to generated primary").truncate(),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::OpenGenerated);
                    }
                }
            }
        });
    } else {
        actions.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let mut find_clicked = false;
            ui.add_enabled_ui(active_available, |ui| {
                find_clicked = icon_button(
                    ui,
                    WorkbenchIcon::Search,
                    "Find in active netlist document",
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked();
            })
            .response
            .on_disabled_hover_text("Open or generate a netlist document before searching.");
            if find_clicked {
                action = Some(NetlistToolbarAction::Find);
            }

            match active {
                ActiveNetlistDocument::Generated => {
                    let mut override_clicked = false;
                    if !compact && app.state.workspace.netlist_source.is_none() {
                        ui.add_enabled_ui(generated_ready, |ui| {
                            override_clicked = icon_button(
                                ui,
                                WorkbenchIcon::More,
                                "Create a narrow generated-source override",
                                false,
                                vec2(28.0, 28.0),
                            )
                            .clicked();
                        })
                        .response
                        .on_disabled_hover_text(generation_block_reason(&app.state));
                    }
                    if override_clicked {
                        action = Some(NetlistToolbarAction::OpenOwnershipDialog(
                            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
                        ));
                    }
                    let label = if app.state.workspace.netlist_source.is_some() {
                        "Open editable source"
                    } else {
                        "Create editable source from generated netlist…"
                    };
                    let primary_ready =
                        app.state.workspace.netlist_source.is_some() || generated_ready;
                    let response = ui
                        .add_enabled(
                            primary_ready,
                            egui::Button::new(label).min_size(vec2(0.0, 28.0)),
                        )
                        .on_disabled_hover_text(generation_block_reason(&app.state));
                    if response.clicked() {
                        action = Some(if app.state.workspace.netlist_source.is_some() {
                            NetlistToolbarAction::OpenOwned
                        } else {
                            NetlistToolbarAction::OpenOwnershipDialog(
                                crate::state::OwnedNetlistEditStrategy::OwnedSource,
                            )
                        });
                    }
                }
                ActiveNetlistDocument::OwnedSource => {
                    let save_ready = owned_source_save_ready(app);
                    if ui
                        .add_enabled(save_ready, egui::Button::new("Save source deck"))
                        .on_disabled_hover_text(
                            "Validate a modified source revision before saving it",
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::Save);
                    }
                    if ui.button("Validate source").clicked() {
                        action = Some(NetlistToolbarAction::Validate);
                    }
                    if ui.button("Return to primary").clicked() {
                        action = Some(NetlistToolbarAction::OpenGenerated);
                    }
                }
                ActiveNetlistDocument::GeneratedDiff => {
                    if ui.button("Return to primary").clicked() {
                        action = Some(NetlistToolbarAction::OpenGenerated);
                    }
                }
            }
        });
    }

    match action {
        Some(NetlistToolbarAction::OpenOwned) => {
            let _ = open_owned_source(&mut app.state);
        }
        Some(NetlistToolbarAction::OpenGenerated) => {
            let _ = super::super::netlist_document::open_generated_primary(&mut app.state);
        }
        Some(NetlistToolbarAction::OpenOwnershipDialog(strategy)) => {
            open_ownership_dialog(&mut app.state, strategy);
        }
        Some(NetlistToolbarAction::Validate) => {
            crate::common::netlist_workflow::validate_visible_netlist_source(app);
        }
        Some(NetlistToolbarAction::Save) => {
            app.state.ui.netlist.save_dialog.open = true;
            app.state.ui.netlist.save_dialog.error = None;
        }
        Some(NetlistToolbarAction::Find) => {
            app.state.ui.netlist.find.open = true;
        }
        None => {}
    }
}

const fn code_toolbar_compact(width: f32) -> bool {
    width <= CODE_TOOLBAR_COMPACT_BREAKPOINT
}

fn code_status(ui: &mut Ui, label: &str, color: egui::Color32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let (dot, _) = ui.allocate_exact_size(vec2(5.0, 11.0), egui::Sense::hover());
        ui.painter().circle_filled(dot.center(), 2.5, color);
        ui.label(
            egui::RichText::new(label)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(color),
        );
    });
}

const fn toolbar_status_visible(phone: bool, tone: DocumentStatusTone) -> bool {
    !(phone && matches!(tone, DocumentStatusTone::Warning))
}

#[derive(Debug, Clone, Copy)]
enum NetlistToolbarAction {
    OpenOwned,
    OpenGenerated,
    OpenOwnershipDialog(crate::state::OwnedNetlistEditStrategy),
    Validate,
    Save,
    Find,
}

#[derive(Debug, Clone, Copy)]
enum FindWindowAction {
    Select(usize),
    ReplaceNext,
    ReplaceAll,
}

#[derive(Debug, Clone)]
struct NetlistSearchDocument {
    active_document: ActiveNetlistDocument,
    label: String,
    source: String,
}

#[derive(Debug, Clone)]
struct NetlistSearchMatch {
    document: NetlistSearchDocument,
    found: super::super::code_workspace::FindMatch,
}

fn netlist_search_documents(
    state: &AppState,
    scope: super::super::netlist_document::NetlistFindScope,
) -> Vec<NetlistSearchDocument> {
    use super::super::netlist_document::NetlistFindScope;

    let generated = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::Generated,
        label: "generated.sp".to_owned(),
        source: state.ui.netlist.generated_source.clone(),
    };
    let owned = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::OwnedSource,
        label: state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .or_else(|| {
                state
                    .workspace
                    .netlist_source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "owned-source.sp".to_owned()),
        source: state.workspace.netlist_source.clone().unwrap_or_default(),
    };

    match scope {
        NetlistFindScope::CurrentDocument => vec![match state.ui.netlist.active_document {
            ActiveNetlistDocument::Generated => generated(),
            ActiveNetlistDocument::OwnedSource => owned(),
            ActiveNetlistDocument::GeneratedDiff => NetlistSearchDocument {
                active_document: ActiveNetlistDocument::GeneratedDiff,
                label: "generated.diff".to_owned(),
                source: state.ui.netlist.generated_diff_source.clone(),
            },
        }],
        NetlistFindScope::AllOwnedSources => state
            .workspace
            .netlist_source
            .as_ref()
            .map(|_| vec![owned()])
            .unwrap_or_default(),
        NetlistFindScope::ProjectReferences => {
            let mut documents = Vec::with_capacity(2);
            if !state.ui.netlist.generated_source.is_empty() {
                documents.push(generated());
            }
            if state.workspace.netlist_source.is_some() {
                documents.push(owned());
            }
            documents
        }
    }
}

fn open_ownership_dialog(state: &mut AppState, strategy: crate::state::OwnedNetlistEditStrategy) {
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

fn ownership_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.ownership_dialog.open {
        return;
    }
    let Some(generated) = app.state.ui.netlist.generated_document.as_ref() else {
        app.state.ui.netlist.ownership_dialog.open = false;
        return;
    };
    let base_revision = generated.revision().get();
    let mut dialog = app.state.ui.netlist.ownership_dialog.clone();
    let owned_deck = dialog.strategy == crate::state::OwnedNetlistEditStrategy::OwnedSource;
    let title = if owned_deck {
        "Create editable source from generated netlist"
    } else {
        "Create generated-netlist override"
    };
    let primary = if owned_deck {
        "Create owned source"
    } else {
        "Create override patch"
    };
    let choice = Dialog::new("NETLIST OWNERSHIP", title, primary)
        .description(
            "Create a project-owned source artifact while preserving the immutable generated primary, its base revision, and regeneration behavior.",
        )
        .size(DialogSize::Transaction)
        .initial_focus(DialogInitialFocus::BodyControl)
        .ghost("Cancel")
        .show_with_initial_body_focus(ctx, |ui| {
            let t = Tokens::get(ctx);
            egui::Frame::new()
                .fill(t.color.bg_inset)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(10)
                .show(ui, |ui| {
                    ui.label(
                        "The generated schematic netlist remains immutable. The editable artifact records its base revision, ownership, and regeneration behavior.",
                    );
                });
            ui.add_space(8.0);
            ui.label("Artifact name");
            let artifact_name = ui.add(
                egui::TextEdit::singleline(&mut dialog.artifact_name)
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Base revision");
                ui.monospace(base_revision.to_string());
            });
            ui.add_space(8.0);
            ui.label("Edit strategy");
            egui::ComboBox::from_id_salt("rspice.code.ownership-strategy")
                .selected_text(dialog.strategy.label())
                .width(ui.available_width().max(1.0))
                .show_ui(ui, |ui| {
                    for strategy in crate::state::OwnedNetlistEditStrategy::ALL {
                        ui.selectable_value(&mut dialog.strategy, strategy, strategy.label());
                    }
                });
            ui.label(match dialog.strategy {
                crate::state::OwnedNetlistEditStrategy::OwnedSource => {
                    "Own the complete deck as an independently editable source revision."
                }
                crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
                    "Own only .param, .option, and .temp cards; execution composes them with the frozen generated base."
                }
                crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
                    "Own include and library ordering while retaining the frozen generated circuit and analyses."
                }
                crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
                    "Own analysis, measurement, save, and probe cards while retaining the frozen generated circuit."
                }
            });
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

fn comparison_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.comparison_dialog.open {
        return;
    }
    let history_len = app.state.ui.netlist.generated_history.len();
    if history_len == 0 || app.state.ui.netlist.generated_document.is_none() {
        app.state.ui.netlist.comparison_dialog.open = false;
        return;
    }
    let mut dialog = app.state.ui.netlist.comparison_dialog.clone();
    dialog.selected_history_index = dialog.selected_history_index.min(history_len - 1);
    let selected = &app.state.ui.netlist.generated_history[dialog.selected_history_index];
    let selected_label = format!(
        "Input revision {} · {}…",
        selected.provenance().input().revision().get(),
        selected
            .content_digest()
            .to_string()
            .chars()
            .take(12)
            .collect::<String>()
    );
    let choice = Dialog::new(
        "NETLIST · IMMUTABLE REVISION COMPARISON",
        "Compare generated revisions",
        "Compare revisions",
    )
        .description(
            "Select an immutable generated predecessor and open an exact source comparison against the current primary artifact.",
        )
        .size(DialogSize::Transaction)
        .initial_focus(DialogInitialFocus::BodyControl)
        .ghost("Cancel")
        .show_with_initial_body_focus(ctx, |ui| {
            ui.label(
                "Select an immutable generated predecessor to compare with the current primary artifact.",
            );
            ui.add_space(8.0);
            ui.label("Prior revision");
            let revision = egui::ComboBox::from_id_salt("rspice.code.compare-revision-select")
                .selected_text(selected_label)
                .width(ui.available_width().max(1.0))
                .show_ui(ui, |ui| {
                    for (index, artifact) in app
                        .state
                        .ui
                        .netlist
                        .generated_history
                        .iter()
                        .enumerate()
                        .rev()
                    {
                        let label = format!(
                            "Input revision {} · {}…",
                            artifact.provenance().input().revision().get(),
                            artifact
                                .content_digest()
                                .to_string()
                                .chars()
                                .take(12)
                                .collect::<String>()
                        );
                        ui.selectable_value(&mut dialog.selected_history_index, index, label);
                    }
                });
            Some(revision.response.id)
        });
    match choice {
        DialogChoice::Primary => {
            match super::super::netlist_document::compare_generated_revision(
                &mut app.state,
                dialog.selected_history_index,
            ) {
                Ok(()) => dialog.open = false,
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.comparison_dialog = dialog;
}

fn owned_source_save_ready(app: &RSpiceApp) -> bool {
    if app.state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource {
        return false;
    }
    let digest = source_content_digest(&app.state.simulation.netlist_content);
    if app.state.ui.netlist.externally_saved_content_digest == Some(digest) {
        return false;
    }
    app.state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
                && app
                    .simulation_controller
                    .has_retained_manual_authorization(receipt.prepared_snapshot_digest)
        })
}

fn save_source_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.save_dialog.open {
        return;
    }
    if app.state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource {
        app.state.ui.netlist.save_dialog.open = false;
        return;
    }

    let mut dialog = app.state.ui.netlist.save_dialog.clone();
    let current_digest = source_content_digest(&app.state.simulation.netlist_content);
    let validated = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == current_digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
                && app
                    .simulation_controller
                    .has_retained_manual_authorization(receipt.prepared_snapshot_digest)
        });
    let message_valid = {
        let message = dialog.message.trim();
        !message.is_empty()
            && message.chars().count() <= 240
            && !message.chars().any(char::is_control)
    };
    let needs_save = app.state.ui.netlist.externally_saved_content_digest != Some(current_digest);
    let primary_enabled = validated && message_valid && needs_save;
    let footer_hint = if validated {
        "Exact source and execution snapshot validated"
    } else {
        "Validation required"
    };
    let choice = Dialog::new(
        "SOURCE · VALIDATED TRANSACTION",
        "Save owned source revision",
        "Save source",
    )
        .description(
            "Publish the exact validated owned source as a new durable revision without changing or rebasing the generated primary.",
        )
        .size(DialogSize::Transaction)
        .initial_focus(DialogInitialFocus::BodyControl)
        .primary_enabled(primary_enabled)
        .ghost("Cancel")
        .hint(footer_hint)
        .show_with_initial_body_focus(ctx, |ui| {
            let t = Tokens::get(ctx);
            let descriptor = app.state.workspace.netlist_descriptor.as_ref();
            ui.label(
                egui::RichText::new(
                    descriptor
                        .map(|value| value.artifact_name.as_str())
                        .unwrap_or("Owned SPICE source"),
                )
                .font(theme::mono(tokens::FS_1, FontWeight::Medium)),
            );
            egui::Frame::new()
                .fill(t.color.bg_inset)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(10)
                .show(ui, |ui| {
                    ui.label(if validated {
                        "The exact authored bytes, materialized generated dependency, PVT contract, and execution target are validated."
                    } else {
                        "Validation is missing or stale. Close this transaction and validate the current source before saving."
                    });
                    if let Some(document) = app.state.ui.netlist.owned_document.as_ref() {
                        ui.monospace(format!(
                            "Generated base {}… · owned revision {}",
                            document
                                .generated_artifact()
                                .content_digest()
                                .to_string()
                                .chars()
                                .take(12)
                                .collect::<String>(),
                            document.revision().get()
                        ));
                    }
                    ui.label("Saving publishes a new owned-source revision; it never mutates or rebases the generated primary.");
            });
            ui.add_space(8.0);
            ui.label("Revision message");
            let revision_message = ui.add(
                egui::TextEdit::singleline(&mut dialog.message)
                    .desired_width(f32::INFINITY)
                    .char_limit(240),
            );
            if !message_valid {
                ui.colored_label(t.color.err, "Enter a one-line message of 1–240 characters.");
            }
            if !needs_save {
                ui.weak("These exact source bytes are already published.");
            }
            if let Some(error) = &dialog.error {
                ui.colored_label(t.color.err, error);
            }
            Some(revision_message.id)
        });
    match choice {
        DialogChoice::Primary => {
            if crate::common::netlist_workflow::save_owned_netlist_source(
                &mut app.state,
                &app.simulation_controller,
                app.export_workflow_io.as_ref(),
                false,
                &dialog.message,
            ) {
                crate::common::netlist_workflow::validate_visible_netlist_source(app);
                dialog.open = false;
                dialog.error = None;
                dialog.message = "Update owned SPICE source".to_owned();
            } else {
                dialog.error = Some(
                    "Source publication did not complete; review the application log.".to_owned(),
                );
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.save_dialog = dialog;
}

fn export_generated_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.export_dialog.open {
        return;
    }
    let mut dialog = app.state.ui.netlist.export_dialog.clone();
    let current = app.state.ui.netlist.generation_error.is_none()
        && app.state.ui.netlist.generated_document.is_some()
        && app.state.ui.netlist.generated_input_digest
            == app.state.ui.netlist.current_generation_input_digest;
    let (dependency_count, dependencies_sealed) = app
        .state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .map(|document| {
            (
                document.dependencies().len(),
                document.dependency_graph_is_sealed(),
            )
        })
        .unwrap_or_default();
    let requires_bundle = dependency_count > 0;
    if requires_bundle {
        dialog.bundle_dependencies = true;
    }
    let bundle_ready = !dialog.bundle_dependencies || dependencies_sealed;
    let primary = if dialog.bundle_dependencies {
        "Export bundle"
    } else {
        "Export generated deck"
    };
    let footer_hint = format!(
        "{dependency_count} dependenc{} · {}",
        if dependency_count == 1 { "y" } else { "ies" },
        if dependencies_sealed {
            "sealed"
        } else {
            "resolution required"
        }
    );
    let choice = Dialog::new(
        "NETLIST · IMMUTABLE SOURCE-MAPPED ARTIFACT",
        "Export generated netlist",
        primary,
    )
        .description(
            "Export the exact current generated revision as a deck or integrity-bound dependency bundle without changing project source.",
        )
        .size(DialogSize::Transaction)
        .initial_focus(DialogInitialFocus::BodyControl)
        .primary_enabled(current && bundle_ready)
        .ghost("Cancel")
        .hint(&footer_hint)
        .show_with_initial_body_focus(ctx, |ui| {
            let t = Tokens::get(ctx);
            egui::Frame::new()
                .fill(t.color.bg_inset)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(10)
                .show(ui, |ui| {
                    ui.label(
                        "The exported deck is a snapshot of the current generated revision. Editing the export never changes the schematic or the project-owned source graph.",
                    );
                });
            ui.add_space(8.0);
            ui.label("Dialect");
            let dialect = egui::ComboBox::from_id_salt("rspice.code.export-dialect")
                .selected_text(match dialog.format {
                    crate::io::NetlistFormat::Spice => "SPICE",
                    crate::io::NetlistFormat::Spectre => "Spectre",
                    crate::io::NetlistFormat::Hspice => "HSPICE",
                    crate::io::NetlistFormat::Xyce => "Xyce",
                })
                .width(ui.available_width().max(1.0))
                .show_ui(ui, |ui| {
                    for (format, label) in [
                        (crate::io::NetlistFormat::Spice, "SPICE"),
                        (crate::io::NetlistFormat::Spectre, "Spectre"),
                        (crate::io::NetlistFormat::Hspice, "HSPICE"),
                        (crate::io::NetlistFormat::Xyce, "Xyce"),
                    ] {
                        ui.selectable_value(&mut dialog.format, format, label);
                    }
                });
            ui.add_space(6.0);
            let bundle_response = ui.add_enabled(
                !requires_bundle,
                egui::Checkbox::new(
                    &mut dialog.bundle_dependencies,
                    "Create self-contained dependency bundle (.zip)",
                ),
            );
            if requires_bundle {
                bundle_response.clone().on_disabled_hover_text(
                    "This generated deck references project-owned source bytes and must be exported as a reproducible bundle.",
                );
            }
            if bundle_response.changed() && !dialog.bundle_dependencies {
                dialog.include_source_map = false;
            }
            let source_map_supported = dialog.format == crate::io::NetlistFormat::Spice;
            let source_map_response = ui.add_enabled(
                source_map_supported,
                egui::Checkbox::new(
                    &mut dialog.include_source_map,
                    "Include generated source map",
                ),
            );
            if source_map_response.changed() && dialog.include_source_map {
                dialog.bundle_dependencies = true;
            }
            if !source_map_supported {
                dialog.include_source_map = false;
                source_map_response.on_disabled_hover_text(
                    "Source-map coordinates identify the exact SPICE artifact before dialect translation.",
                );
            }
            egui::Frame::new()
                .fill(t.color.bg_inset)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(10)
                .show(ui, |ui| {
                    ui.label(format!(
                        "{dependency_count} external dependenc{} · {}",
                        if dependency_count == 1 { "y" } else { "ies" },
                        if dependencies_sealed {
                            "sealed"
                        } else {
                            "resolution required"
                        }
                    ));
                    ui.label(if dialog.bundle_dependencies {
                        "Bundle paths are rewritten to deterministic internal entries and accompanied by an integrity manifest."
                    } else {
                        "A single dialect deck will be exported without dependency members."
                    });
                });
            if let Some(error) = &dialog.error {
                ui.colored_label(t.color.err, error);
            }
            Some(dialect.response.id)
        });
    match choice {
        DialogChoice::Primary => {
            if crate::common::menu_bar::action_export_generated_netlist_with_options(
                &mut app.state,
                dialog.format,
                dialog.bundle_dependencies,
                dialog.include_source_map,
                app.export_workflow_io.as_ref(),
            ) {
                dialog.open = false;
                dialog.error = None;
            } else {
                dialog.error =
                    Some("Export did not complete; review the application log.".to_owned());
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.export_dialog = dialog;
}

fn create_owned_source(
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
    state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        artifact_name: name.to_owned(),
        strategy,
        save_history: Vec::new(),
    });
    state.ui.netlist.owned_document = Some(owned);
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    state.ui.netlist.externally_saved_content_digest = None;
    state.simulation.netlist_content = source;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    super::super::netlist_document::invalidate_source_evidence(&mut state.ui.netlist);
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

fn find_replace_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.find.open {
        return;
    }

    use super::super::code_workspace::{
        FindDirection, FindOptions, ReplaceScope, find_all_in_source, replace_in_source,
    };
    use super::super::netlist_document::NetlistFindScope;

    let owned = app.state.ui.netlist.active_document == ActiveNetlistDocument::OwnedSource;
    let mut find = app.state.ui.netlist.find.clone();
    let options = FindOptions {
        direction: FindDirection::Forward,
        match_case: find.match_case,
        whole_word: find.whole_symbol,
        regular_expression: find.regular_expression,
    };
    let documents = netlist_search_documents(&app.state, find.scope);
    let matches: Result<Vec<NetlistSearchMatch>, super::super::code_workspace::FindError> =
        if find.find.is_empty() {
            Ok(Vec::new())
        } else {
            documents
                .iter()
                .try_fold(Vec::<NetlistSearchMatch>::new(), |mut all, document| {
                    all.extend(
                        find_all_in_source(&document.source, &find.find, options)?
                            .into_iter()
                            .map(|found| NetlistSearchMatch {
                                document: document.clone(),
                                found,
                            }),
                    );
                    Ok(all)
                })
        };
    find.error = matches.as_ref().err().map(ToString::to_string);
    let matches = matches.unwrap_or_default();
    if matches.is_empty() {
        find.selected_match = 0;
    } else {
        find.selected_match = find.selected_match.min(matches.len() - 1);
    }

    let mut action = None;
    let has_matches = !matches.is_empty();
    let compact_fields = ctx.content_rect().width() < 360.0;
    let find_hint = if find.find.is_empty() {
        "Enter text to search"
    } else if find.error.is_some() {
        "Correct the search expression"
    } else {
        "Generated artifacts remain immutable"
    };
    let choice = Dialog::new(
        "SOURCE EDITOR · SCOPED SEARCH",
        "Find and replace in source",
        "Find next",
    )
        .description(
            "Search editable source and generated references; replacement remains limited to project-owned source artifacts.",
        )
        .size(DialogSize::Transaction)
        .initial_focus(DialogInitialFocus::BodyControl)
        .primary_enabled(has_matches && find.error.is_none())
        .ghost("Close")
        .hint(find_hint)
        .show_with_initial_body_focus(ctx, |ui| {
            let mut find_control_id = None;
            if compact_fields {
                ui.label("Find");
                let response = ui.add(
                    egui::TextEdit::singleline(&mut find.find)
                        .desired_width(ui.available_width())
                        .hint_text("Symbol, text, or expression"),
                );
                find_control_id = Some(response.id);
                ui.label("Replace");
                ui.add_enabled(
                    (owned || find.scope == NetlistFindScope::AllOwnedSources)
                        && find.scope != NetlistFindScope::ProjectReferences,
                    egui::TextEdit::singleline(&mut find.replacement)
                        .desired_width(ui.available_width()),
                );
                ui.label("Scope");
                find_scope_combo(ui, &mut find, app.state.workspace.netlist_source.is_some());
            } else {
                egui::Grid::new("rspice.code.find-fields")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Find");
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut find.find)
                                .desired_width(ui.available_width().max(180.0))
                                .hint_text("Symbol, text, or expression"),
                        );
                        find_control_id = Some(response.id);
                        ui.end_row();
                        ui.label("Replace");
                        ui.add_enabled(
                            (owned || find.scope == NetlistFindScope::AllOwnedSources)
                                && find.scope != NetlistFindScope::ProjectReferences,
                            egui::TextEdit::singleline(&mut find.replacement)
                                .desired_width(ui.available_width().max(180.0)),
                        );
                        ui.end_row();
                        ui.label("Scope");
                        find_scope_combo(
                            ui,
                            &mut find,
                            app.state.workspace.netlist_source.is_some(),
                        );
                        ui.end_row();
                    });
            }

            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut find.match_case, "Match case");
                ui.checkbox(&mut find.whole_symbol, "Whole symbol");
                ui.checkbox(&mut find.regular_expression, "Regular expression");
            });
            ui.separator();

            if let Some(error) = &find.error {
                ui.colored_label(Tokens::get(ctx).color.err, error);
            } else if find.find.is_empty() {
                ui.weak("Enter text to search the exact current artifact.");
            } else {
                ui.label(format!(
                    "{} match{}",
                    matches.len(),
                    if matches.len() == 1 { "" } else { "es" }
                ));
            }

            egui::ScrollArea::vertical()
                .id_salt("rspice.code.find-results")
                .max_height(168.0)
                .show(ui, |ui| {
                    let show_document =
                        documents.len() > 1 || find.scope != NetlistFindScope::CurrentDocument;
                    for (index, result) in matches.iter().enumerate() {
                        let line_text = result
                            .document
                            .source
                            .lines()
                            .nth(result.found.line().saturating_sub(1))
                            .unwrap_or_default()
                            .trim();
                        let location = if show_document {
                            format!(
                                "{}  {}:{}",
                                result.document.label,
                                result.found.line(),
                                result.found.column()
                            )
                        } else {
                            format!("{}:{}", result.found.line(), result.found.column())
                        };
                        if ui
                            .add_sized(
                                [ui.available_width(), 24.0],
                                egui::Button::selectable(
                                    find.selected_match == index,
                                    format!("{location}  {line_text}"),
                                ),
                            )
                            .clicked()
                        {
                            action = Some(FindWindowAction::Select(index));
                        }
                    }
                });

            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(has_matches, egui::Button::new("Previous"))
                    .clicked()
                {
                    let next = if find.selected_match == 0 {
                        matches.len().saturating_sub(1)
                    } else {
                        find.selected_match - 1
                    };
                    action = Some(FindWindowAction::Select(next));
                }
                let selected_owned = matches.get(find.selected_match).is_some_and(|result| {
                    result.document.active_document == ActiveNetlistDocument::OwnedSource
                });
                let replace_enabled = find.scope != NetlistFindScope::ProjectReferences
                    && has_matches
                    && selected_owned
                    && find.error.is_none();
                if ui
                    .add_enabled(replace_enabled, egui::Button::new("Replace"))
                    .clicked()
                {
                    action = Some(FindWindowAction::ReplaceNext);
                }
                if ui
                    .add_enabled(replace_enabled, egui::Button::new("Replace all"))
                    .clicked()
                {
                    action = Some(FindWindowAction::ReplaceAll);
                }
            });
            find_control_id
        });
    match choice {
        DialogChoice::Primary => {
            action = Some(FindWindowAction::Select(
                (find.selected_match + 1) % matches.len().max(1),
            ));
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => find.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.find = find;

    match action {
        Some(FindWindowAction::Select(index)) => {
            if let Some(result) = matches.get(index) {
                app.state.ui.netlist.find.selected_match = index;
                match result.document.active_document {
                    ActiveNetlistDocument::Generated => {
                        let _ =
                            super::super::netlist_document::open_generated_primary(&mut app.state);
                    }
                    ActiveNetlistDocument::OwnedSource => {
                        let _ = open_owned_source(&mut app.state);
                    }
                    ActiveNetlistDocument::GeneratedDiff => {}
                }
                app.state.ui.netlist.requested_line = Some(result.found.line());
            }
        }
        Some(FindWindowAction::ReplaceNext) | Some(FindWindowAction::ReplaceAll) => {
            let selected = matches.get(app.state.ui.netlist.find.selected_match);
            let Some(selected) = selected.filter(|result| {
                result.document.active_document == ActiveNetlistDocument::OwnedSource
            }) else {
                return;
            };
            if !owned {
                let _ = open_owned_source(&mut app.state);
            }
            let scope = if matches!(action, Some(FindWindowAction::ReplaceAll)) {
                ReplaceScope::All
            } else {
                ReplaceScope::Next {
                    caret_byte: selected.found.byte_range().start,
                }
            };
            match replace_in_source(
                &selected.document.source,
                &app.state.ui.netlist.find.find,
                &app.state.ui.netlist.find.replacement,
                options,
                scope,
            ) {
                Ok(outcome) => {
                    let count = outcome.replacement_count();
                    if count > 0
                        && super::super::netlist_document::replace_owned_source(
                            &mut app.state,
                            outcome.into_source(),
                        )
                    {
                        app.state.ui.netlist.find.selected_match = 0;
                        app.state.push_user_message(ConsoleMessage::info(format!(
                            "Replaced {count} match{} in owned SPICE source.",
                            if count == 1 { "" } else { "es" }
                        )));
                    }
                }
                Err(error) => app.state.ui.netlist.find.error = Some(error.to_string()),
            }
        }
        None => {}
    }
}

fn find_scope_combo(
    ui: &mut Ui,
    find: &mut super::super::netlist_document::NetlistFindState,
    has_owned_source: bool,
) {
    use super::super::netlist_document::NetlistFindScope;

    egui::ComboBox::from_id_salt("rspice.code.find-scope")
        .selected_text(match find.scope {
            NetlistFindScope::CurrentDocument => "Current document",
            NetlistFindScope::AllOwnedSources => "All owned source files",
            NetlistFindScope::ProjectReferences => "Project references (find only)",
        })
        .width(ui.available_width().max(180.0))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut find.scope,
                NetlistFindScope::CurrentDocument,
                "Current document",
            );
            if has_owned_source {
                ui.selectable_value(
                    &mut find.scope,
                    NetlistFindScope::AllOwnedSources,
                    "All owned source files",
                );
            }
            ui.selectable_value(
                &mut find.scope,
                NetlistFindScope::ProjectReferences,
                "Project references (find only)",
            );
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentStatusTone {
    Valid,
    Warning,
    Error,
}

fn document_syntax_status(state: &AppState) -> (String, DocumentStatusTone) {
    if state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        return ("comparison ready".to_owned(), DocumentStatusTone::Valid);
    }
    if state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && (state.ui.netlist.generation_error.is_some()
            || !generated_primary_ready(state)
            || state.ui.netlist.generated_input_digest
                != state.ui.netlist.current_generation_input_digest)
    {
        let retained_artifact = generated_primary_ready(state);
        return (
            if retained_artifact {
                "stale · generation blocked"
            } else {
                "generation blocked"
            }
            .to_owned(),
            DocumentStatusTone::Warning,
        );
    }
    let errors = state
        .ui
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == super::super::netlist_document::DiagnosticSeverity::Error
        })
        .count();
    if errors > 0 {
        return (
            format!(
                "{errors} syntax error{}",
                if errors == 1 { "" } else { "s" }
            ),
            DocumentStatusTone::Error,
        );
    }
    ("syntax valid".to_owned(), DocumentStatusTone::Valid)
}

/// Open (or explicitly create) the project-owned source derived from the
/// immutable generated primary. Existing owned bytes are never overwritten.
fn open_owned_source(state: &mut AppState) -> bool {
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
            super::super::code_workspace::SourceLocator::try_new(
                path.display().to_string(),
                display_name,
            )
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
            save_history: Vec::new(),
        });
    }
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = source;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    super::super::netlist_document::invalidate_source_evidence(&mut state.ui.netlist);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn retain_generated(state: &mut AppState, source: &str) {
        let input_digest = crate::product::ContentDigest::from_bytes([0x41; 32]);
        let mut source = source.to_owned();
        append_project_veriloga_directive(state, &mut source)
            .expect("canonical generated Verilog-A dependency is valid");
        let (document, owned) = publish_generated_document(state, input_digest, source.clone())
            .expect("canonical generated fixture");
        state.ui.netlist.generated_source = source;
        state.ui.netlist.generated_document = Some(document);
        state.ui.netlist.owned_document = owned;
        state.ui.netlist.generated_input_digest = Some(input_digest);
        state.ui.netlist.current_generation_input_digest = Some(input_digest);
    }

    #[test]
    fn editable_source_and_generated_primary_coexist_without_overwrite() {
        let mut state = AppState::default();
        retain_generated(&mut state, "generated\n.end\n");
        let retained_generated = state.ui.netlist.generated_source.clone();
        state.simulation.netlist_content = state.ui.netlist.generated_source.clone();

        create_owned_source(
            &mut state,
            "top_override.sp",
            crate::state::OwnedNetlistEditStrategy::OwnedSource,
        )
        .expect("create owned source");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(retained_generated.as_str())
        );
        assert!(crate::workbench::netlist_document::replace_owned_source(
            &mut state,
            "owned edit\n.end\n".to_owned()
        ));

        assert!(super::super::super::netlist_document::open_generated_primary(&mut state));
        assert_eq!(state.simulation.netlist_content, retained_generated);
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("owned edit\n.end\n")
        );
        assert!(!super::super::super::netlist_document::open_generated_primary(&mut state));
    }

    #[test]
    fn opening_existing_owned_source_never_overwrites_its_bytes() {
        let mut state = AppState::default();
        retain_generated(&mut state, "new generated\n.end\n");
        state.workspace.netlist_source = Some("retained owned\n.end\n".to_owned());

        assert!(open_owned_source(&mut state));
        assert_eq!(state.simulation.netlist_content, "retained owned\n.end\n");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("retained owned\n.end\n")
        );
    }

    #[test]
    fn toolbar_geometry_matches_the_mockup_contract() {
        assert_eq!(CODE_TOOLBAR_HEIGHT, 33.0);
        assert_eq!(CODE_TOOLBAR_PADDING_X, 8.0);
        assert_eq!(CODE_TOOLBAR_GAP, 5.0);
        assert_eq!(CODE_TOOLBAR_COMPACT_BREAKPOINT, 720.0);
        assert_eq!(PHONE_BREAKPOINT, 560.0);
        assert_eq!(PHONE_PRIMARY_WIDTH, 154.0);
        assert_eq!(PHONE_ACTION_WIDTH, 187.0);
        assert!(code_toolbar_compact(607.0));
        assert!(!code_toolbar_compact(721.0));
        assert!(!toolbar_status_visible(true, DocumentStatusTone::Warning));
        assert!(toolbar_status_visible(true, DocumentStatusTone::Error));
        assert!(toolbar_status_visible(false, DocumentStatusTone::Warning));
    }

    #[test]
    fn empty_generated_primary_reports_blocked_without_claiming_staleness() {
        let mut state = AppState::default();
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generation_error =
            Some("Add a circuit before generating the primary netlist.".to_owned());

        assert!(generated_primary_unavailable(&state));
        assert!(!generated_primary_ready(&state));
        assert_eq!(
            document_syntax_status(&state),
            ("generation blocked".to_owned(), DocumentStatusTone::Warning)
        );
    }

    #[test]
    fn retained_generated_primary_reports_stale_when_regeneration_is_blocked() {
        let mut state = AppState::default();
        retain_generated(&mut state, "retained\n.end\n");
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generation_error = Some("Regeneration failed.".to_owned());

        assert!(!generated_primary_unavailable(&state));
        assert!(generated_primary_ready(&state));
        assert_eq!(
            document_syntax_status(&state),
            (
                "stale · generation blocked".to_owned(),
                DocumentStatusTone::Warning
            )
        );
    }

    #[test]
    fn split_generated_state_fails_closed_as_unavailable() {
        let mut state = AppState::default();
        retain_generated(&mut state, "retained\n.end\n");
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generated_source.clear();
        state.ui.netlist.generation_error = None;

        assert!(state.ui.netlist.generated_document.is_some());
        assert!(generated_primary_unavailable(&state));
        assert!(!generated_primary_ready(&state));
        assert!(!active_document_available(&state));
        assert_eq!(
            document_syntax_status(&state),
            ("generation blocked".to_owned(), DocumentStatusTone::Warning)
        );
    }

    #[test]
    fn generated_deck_references_project_veriloga_once_before_end() {
        let state = AppState::default();
        let mut source = "R1 1 0 1k\n.end\n".to_owned();

        append_project_veriloga_directive(&state, &mut source).unwrap();
        append_project_veriloga_directive(&state, &mut source).unwrap();

        let document = state
            .workspace
            .project_sources
            .get(crate::state::ProjectSourceLanguage::VerilogA)
            .expect("bootstrapped Verilog-A source");
        let source_key = crate::workbench::code_workspace::project_veriloga_source_key(
            state.workspace.project.id(),
            document,
        );
        assert_eq!(
            source,
            format!("R1 1 0 1k\n.veriloga \"{source_key}\" sensor_bridge\n.end\n")
        );

        let dependencies = generated_project_source_dependencies(&state, &source)
            .expect("project dependency is sealed");
        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0].requested_locator(), source_key);
        assert_eq!(dependencies[0].source(), Some(document.content()));
    }
}

//! Mockup-owned Code & Automation netlist document surface.
//!
//! The center well is deliberately flat: one 33-point document toolbar over
//! an exact-entry editor. Generated and owned source are independent retained
//! documents and switching between them never deletes either one.
//!
//! This module owns the surface's frame — reconciliation entry, the review
//! banner, file drop, deterministic formatting, and the readiness predicates
//! every other part reads. Each submodule owns one transaction of its own.

mod generation;
mod language;
mod ownership;
mod revision;
mod search;
mod toolbar;
mod transfer;

use egui::Ui;

use crate::diagnostics::ConsoleMessage;
use crate::ui::tokens::Tokens;
use crate::workbench::design_system::{WorkbenchIcon, empty_state, empty_state_with_actions};
use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, source_content_digest};
use crate::workbench::{AppState, MessageId, RSpiceApp};

pub(super) fn prepare_workspace(app: &mut RSpiceApp) {
    generation::reconcile_documents(app);
    crate::workbench::documents::netlist_document::prepare(&mut app.state);
}

pub(super) fn show_prepared(ui: &mut Ui, app: &mut RSpiceApp) {
    handle_netlist_file_drop(ui.ctx(), app);
    toolbar::code_toolbar(ui, app);
    execution_profile_review_banner(ui, app);
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui,
        crate::workbench::documents::netlist_document::editor_id(&app.state),
    ) {
        format_owned_netlist(ui.ctx(), app);
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        if generated_primary_unavailable(&app.state) {
            let messages = app.state.ui.messages();
            if app.state.workspace.netlist_source.is_none()
                && app.state.ui.netlist.generated_document.is_none()
            {
                let mut action = None;
                empty_state_with_actions(
                    ui,
                    WorkbenchIcon::Netlist,
                    &messages.text(MessageId::NetlistEmptyWorkspaceTitle),
                    &messages.text(MessageId::NetlistEmptyWorkspaceDescription),
                    |ui| {
                        if ui
                            .button(messages.text(MessageId::NetlistNewTopDeck))
                            .clicked()
                        {
                            action = Some(EmptyNetlistAction::NewTopDeck);
                        }
                        if ui
                            .button(messages.text(MessageId::NetlistImportDeck))
                            .clicked()
                        {
                            action = Some(EmptyNetlistAction::ImportDeck);
                        }
                    },
                );
                match action {
                    Some(EmptyNetlistAction::NewTopDeck) => {
                        let result = crate::workbench::app::open_source_document_dialog(
                            &mut app.state,
                        )
                        .and_then(|()| {
                            crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action(
                                &mut app.state,
                                crate::workbench::documents::code_workspace::CodeSourceFileAction::New,
                            )
                        });
                        if let Err(error) = result {
                            app.state.push_user_message(ConsoleMessage::error(error));
                        }
                    }
                    Some(EmptyNetlistAction::ImportDeck) => {
                        crate::workbench::commands::vocabulary::Command::ImportNetlist.execute(app);
                    }
                    None => {}
                }
            } else {
                let title = messages.text(MessageId::NetlistGeneratedUnavailable);
                let default_description =
                    messages.text(MessageId::NetlistGeneratedUnavailableDescription);
                empty_state(
                    ui,
                    WorkbenchIcon::Netlist,
                    &title,
                    app.state
                        .ui
                        .netlist
                        .generation_error
                        .as_deref()
                        .unwrap_or(&default_description),
                );
            }
        } else {
            crate::workbench::documents::netlist_document::show_editor(ui, &mut app.state);
        }
    });
    search::find_replace_window(ui.ctx(), app);
    language::rename_dialog_window(ui.ctx(), app);
    ownership::ownership_dialog_window(ui.ctx(), app);
    revision::comparison_dialog_window(ui.ctx(), app);
    revision::save_source_dialog_window(ui.ctx(), app);
    transfer::external_change_dialog_window(ui.ctx(), app);
    transfer::export_generated_dialog_window(ui.ctx(), app);
    transfer::import_review_dialog_window(ui.ctx(), app);
}

#[derive(Clone, Copy)]
enum EmptyNetlistAction {
    NewTopDeck,
    ImportDeck,
}

fn execution_profile_review_banner(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(descriptor) = app
        .state
        .workspace
        .netlist_descriptor
        .as_ref()
        .filter(|descriptor| descriptor.execution_profile_review_required())
    else {
        return;
    };
    let dialect = descriptor
        .imported_dialect
        .unwrap_or(crate::state::NetlistSourceDialect::RSpice);
    let messages = app.state.ui.messages();
    let description = messages.format(
        MessageId::NetlistProfileReviewRequiredDescription,
        &[("dialect", dialect.label())],
    );
    let t = Tokens::get(ui.ctx());
    let mut review = false;
    egui::Frame::new()
        .fill(t.color.warn.gamma_multiply(0.10))
        .stroke(egui::Stroke::new(1.0, t.color.warn.gamma_multiply(0.65)))
        .inner_margin(8)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(messages.text(MessageId::NetlistProfileReviewRequired))
                        .strong()
                        .color(t.color.warn),
                );
                ui.label(description);
                review = ui
                    .button(messages.text(MessageId::NetlistProfileReviewAction))
                    .clicked();
            });
        });
    if review {
        crate::workbench::workflows::netlist_workflow::begin_owned_netlist_profile_review(
            &mut app.state,
        );
    }
}

fn handle_netlist_file_drop(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.application_modal_open() {
        return;
    }
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() {
        return;
    }
    if dropped.len() != 1 {
        app.state.push_user_message(ConsoleMessage::warning(
            "Drop one SPICE deck or RSpice netlist bundle at a time so the staged import review has one exact source identity.",
        ));
        return;
    }
    let file = &dropped[0];
    let source_path = file.path.clone();
    let display_name = (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            source_path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "dropped-netlist.spice".to_owned());
    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            source_path
                .as_deref()
                .ok_or_else(|| "Dropped file has neither bytes nor a native path.".to_owned())
                .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err("Browser drop did not provide immutable file bytes.".to_owned())
        }
    };
    match bytes {
        Ok(bytes) => {
            crate::workbench::workflows::netlist_workflow::stage_dropped_netlist_import(
                &mut app.state,
                bytes,
                source_path,
                display_name,
            );
        }
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Dropped SPICE source could not be read: {error}"
        ))),
    }
}

fn format_owned_netlist(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !crate::workbench::documents::netlist_document::active_netlist_source_is_editable(&app.state)
    {
        return;
    }
    let source = app.state.simulation.netlist_content.clone();
    let dependency_document = app.state.ui.netlist.active_dependency_identity.is_some();
    if dependency_document {
        let has_errors = app.state.ui.netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        });
        if has_errors {
            app.state.push_user_message(ConsoleMessage::error(
                "Include formatting is blocked until the current document has no syntax errors.",
            ));
            return;
        }
        let formatted = normalize_owned_netlist_whitespace(&source);
        if formatted == source {
            app.state.push_user_message(ConsoleMessage::info(
                "The owned include already matches the deterministic source format.",
            ));
        } else if crate::workbench::documents::netlist_document::replace_owned_dependency_source(
            &mut app.state,
            formatted,
        ) {
            app.state.push_user_message(ConsoleMessage::info(
                "Formatted the exact project-owned include revision; root-deck validation was invalidated.",
            ));
        }
        return;
    }
    let digest = source_content_digest(&source);
    let validation_current = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
        });
    if !validation_current {
        crate::workbench::workflows::netlist_workflow::validate_visible_netlist_source(app);
    }
    let validation_current = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
        });
    if !validation_current {
        let message = app
            .state
            .ui
            .netlist
            .validation_error
            .clone()
            .unwrap_or_else(|| {
                "Formatting is blocked until the exact owned source passes executable validation."
                    .to_owned()
            });
        app.state
            .push_user_message(ConsoleMessage::error(message.clone()));
        app.state
            .ui
            .toasts
            .error_with_title(ctx, "Netlist format blocked", message);
        return;
    }

    let formatted = normalize_owned_netlist_whitespace(&source);
    if formatted == source {
        app.state.push_user_message(ConsoleMessage::info(
            "The owned netlist already matches the deterministic source format.",
        ));
        return;
    }
    if crate::workbench::documents::netlist_document::replace_owned_source(
        &mut app.state,
        formatted,
    ) {
        app.state.push_user_message(ConsoleMessage::info(
            "Formatted the exact owned netlist revision; prior validation was invalidated.",
        ));
    } else {
        app.state.push_user_message(ConsoleMessage::error(
            "The owned netlist changed before formatting could commit. Review the current revision and retry.",
        ));
    }
}

fn normalize_owned_netlist_whitespace(source: &str) -> String {
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

fn generated_primary_unavailable(state: &AppState) -> bool {
    state.ui.netlist.active_dependency_identity.is_none()
        && state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && !generated_primary_ready(state)
}

fn generated_primary_ready(state: &AppState) -> bool {
    state.ui.netlist.generated_document.is_some() && !state.ui.netlist.generated_source.is_empty()
}

fn active_document_available(state: &AppState) -> bool {
    if state.ui.netlist.active_dependency_identity.is_some() {
        return crate::workbench::documents::netlist_document::active_dependency(state).is_some();
    }
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => generated_primary_ready(state),
        ActiveNetlistDocument::OwnedSource => state.workspace.netlist_source.is_some(),
        ActiveNetlistDocument::GeneratedDiff => !state.ui.netlist.generated_diff_source.is_empty(),
    }
}

fn generation_block_reason(state: &AppState) -> String {
    state
        .ui
        .netlist
        .generation_error
        .clone()
        .unwrap_or_else(|| {
            state
                .ui
                .messages()
                .text(MessageId::NetlistGenerateBeforeAction)
        })
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentStatusTone {
    Valid,
    Warning,
    Error,
}

fn document_syntax_status(state: &AppState) -> (String, DocumentStatusTone) {
    let messages = state.ui.messages();
    if state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        return (
            messages.text(MessageId::NetlistComparisonReady),
            DocumentStatusTone::Valid,
        );
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
                messages.text(MessageId::NetlistGenerationStaleBlocked)
            } else {
                messages.text(MessageId::NetlistGenerationBlocked)
            },
            DocumentStatusTone::Warning,
        );
    }
    let errors = state
        .ui
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        })
        .count();
    if errors > 0 {
        let count = errors.to_string();
        return (
            messages.format(
                if errors == 1 {
                    MessageId::NetlistSyntaxErrorSingular
                } else {
                    MessageId::NetlistSyntaxErrors
                },
                &[("count", &count)],
            ),
            DocumentStatusTone::Error,
        );
    }
    (
        messages.text(MessageId::NetlistSyntaxValid),
        DocumentStatusTone::Valid,
    )
}
#[cfg(test)]
mod tests {
    use egui::vec2;

    use super::*;

    #[test]
    fn deterministic_netlist_format_preserves_line_ending_policy() {
        assert_eq!(
            normalize_owned_netlist_whitespace("deck  \r\nR1 1 0 1k\t\r\n.end"),
            "deck  \r\nR1 1 0 1k\r\n.end\r\n"
        );
        assert_eq!(
            normalize_owned_netlist_whitespace("deck\n.end\n"),
            "deck\n.end\n"
        );
    }

    fn configured_veriloga_state() -> (AppState, String) {
        let mut state = AppState::default();
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "sealed_gain".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let mut cell = crate::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = crate::state::Library::new("behavioral");
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference),
            crate::state::ProjectSourceLanguage::VerilogA,
            "behavioral/gain.va",
            "`ifdef NEVER\n`include \"behavioral/inactive.va\"\n`endif\n`include \"behavioral/gain_constants.va\"\nmodule sealed_gain(p, n); inout p, n; electrical p, n; analog I(p,n) <+ `RSPICE_GAIN * V(p,n); endmodule\n",
            [
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/gain_constants.va",
                    "`define RSPICE_GAIN 1.0\n",
                )
                .expect("valid included source"),
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/inactive.va",
                    "module must_not_enter_provenance; endmodule\n",
                )
                .expect("valid inactive source"),
            ],
            [
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/gain_constants.va",
                )
                .expect("valid dependency edge"),
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/inactive.va",
                )
                .expect("valid inactive dependency edge"),
            ],
        )
        .expect("valid source closure");
        state
            .workspace
            .project_sources
            .insert_bundle(bundle)
            .expect("attach cell-view source");

        let mut placed = crate::state::LibraryCellInstance::new("behavioral", "gain", "schematic");
        placed.terminal_order = vec!["p".to_owned(), "n".to_owned()];
        state
            .schematic
            .add_library_cell_component(crate::state::Point::new(20, 20), placed);
        state
            .workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Mixed-signal".to_owned(),
                root: crate::state::CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["veriloga".to_owned()],
                stop_views: vec!["veriloga".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Mixed-signal design".to_owned(),
            })
            .expect("create mixed-signal configuration");
        let projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
            )
            .expect("resolve configured behavioral view");
        let source_key = projection
            .plan()
            .and_then(|plan| {
                plan.binding(
                    &crate::state::InstancePath::parse("/X1").expect("fixture instance path"),
                )
            })
            .and_then(|binding| binding.project_veriloga())
            .expect("project Verilog-A binding")
            .source_key()
            .to_owned();
        (state, source_key)
    }

    fn retain_generated(state: &mut AppState, source: &str) {
        let input_digest = crate::product::ContentDigest::from_bytes([0x41; 32]);
        let source = source.to_owned();
        let (document, owned) =
            generation::publish_generated_document(state, input_digest, source.clone())
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

        ownership::create_owned_source(
            &mut state,
            "top_override.sp",
            crate::state::OwnedNetlistEditStrategy::OwnedSource,
        )
        .expect("create owned source");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(retained_generated.as_str())
        );
        assert!(
            crate::workbench::documents::netlist_document::replace_owned_source(
                &mut state,
                "owned edit\n.end\n".to_owned()
            )
        );

        assert!(crate::workbench::documents::netlist_document::open_generated_primary(&mut state));
        assert_eq!(state.simulation.netlist_content, retained_generated);
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("owned edit\n.end\n")
        );
        assert!(!crate::workbench::documents::netlist_document::open_generated_primary(&mut state));
    }

    #[test]
    fn opening_existing_owned_source_never_overwrites_its_bytes() {
        let mut state = AppState::default();
        retain_generated(&mut state, "new generated\n.end\n");
        state.workspace.netlist_source = Some("retained owned\n.end\n".to_owned());

        assert!(ownership::open_owned_source(&mut state));
        assert_eq!(state.simulation.netlist_content, "retained owned\n.end\n");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("retained owned\n.end\n")
        );
    }

    #[test]
    fn toolbar_geometry_matches_the_mockup_contract() {
        assert_eq!(toolbar::CODE_TOOLBAR_HEIGHT, 33.0);
        assert_eq!(toolbar::CODE_TOOLBAR_PADDING_X, 8.0);
        assert_eq!(toolbar::CODE_TOOLBAR_GAP, 5.0);
        assert_eq!(toolbar::CODE_TOOLBAR_ACTION_GUTTER, 12.0);
        assert_eq!(toolbar::CODE_TOOLBAR_COMPACT_BREAKPOINT, 720.0);
        assert_eq!(toolbar::CODE_TOOLBAR_TABLET_VIEWPORT_BREAKPOINT, 1024.0);
        assert_eq!(toolbar::CODE_TOOLBAR_FULL_STATUS_MIN_WIDTH, 320.0);
        assert_eq!(toolbar::PHONE_BREAKPOINT, 560.0);
        assert_eq!(toolbar::PHONE_PRIMARY_WIDTH, 154.0);
        assert_eq!(toolbar::CODE_TOOLBAR_ICON_WIDTH, 28.0);
        assert_eq!(toolbar::PHONE_ACTION_WIDTH, 283.0);
        assert_eq!(toolbar::compact_action_width(false, false), 283.0);
        assert_eq!(toolbar::compact_action_width(true, true), 316.0);
        assert_eq!(toolbar::compact_action_width(true, false), 349.0);
        assert_eq!(toolbar::code_toolbar_visible_width(1024.0, 745.0), 745.0);
        assert_eq!(toolbar::code_toolbar_visible_width(700.0, 745.0), 700.0);
        assert!(toolbar::code_toolbar_compact(607.0));
        assert!(!toolbar::code_toolbar_compact(721.0));
        assert!(
            toolbar::code_toolbar_prefers_compact(1024.0, 744.0, 405.0),
            "tablet viewport must retain the compact projection"
        );
        assert!(
            toolbar::code_toolbar_prefers_compact(1280.0, 744.0, 405.0),
            "dock collapse must not re-enable a crowded full action set"
        );
        assert!(!toolbar::code_toolbar_prefers_compact(1280.0, 800.0, 405.0));
        assert!(!toolbar::toolbar_status_visible(
            true,
            DocumentStatusTone::Warning
        ));
        assert!(toolbar::toolbar_status_visible(
            true,
            DocumentStatusTone::Error
        ));
        assert!(toolbar::toolbar_status_visible(
            false,
            DocumentStatusTone::Warning
        ));
        assert!(toolbar::toolbar_advisory_fits(500.0, 260.0, 110.0, 70.0));
        assert!(
            !toolbar::toolbar_advisory_fits(430.0, 260.0, 110.0, 70.0),
            "advisory must yield before language, blocking status, or actions clip"
        );
        let content = egui::Rect::from_min_size(egui::pos2(8.0, 0.0), vec2(526.0, 33.0));
        let (status_and_language, actions) = toolbar::code_toolbar_regions(content, 342.0);
        assert_eq!(actions.right(), content.right());
        assert_eq!(
            actions.left() - status_and_language.right(),
            toolbar::CODE_TOOLBAR_ACTION_GUTTER
        );
        assert!(status_and_language.right() < actions.left());
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
    fn generated_deck_does_not_inject_unreferenced_code_workspace_veriloga() {
        let state = AppState::default();
        let source = "R1 1 0 1k\n.end\n".to_owned();

        let dependencies = generation::generated_project_source_dependencies(&state, &source)
            .expect("unreferenced source does not create a dependency");
        assert!(dependencies.is_empty());
    }

    #[test]
    fn generated_deck_retains_the_exact_transitive_cell_view_source_closure() {
        let (state, source_key) = configured_veriloga_state();
        let source = format!("configured deck\n.veriloga \"{source_key}\" sealed_gain\n.end\n");

        let dependencies = generation::generated_project_source_dependencies(&state, &source)
            .expect("retain exact project source closure");

        assert_eq!(dependencies.len(), 2);
        assert!(dependencies.iter().all(|dependency| {
            !dependency
                .resolution()
                .source()
                .is_some_and(|source| source.contains("must_not_enter_provenance"))
        }));
        let root = dependencies
            .iter()
            .find(|dependency| dependency.direct_include_index().is_some())
            .expect("direct root dependency");
        assert_eq!(root.requested_locator(), source_key);
        assert!(root.parent().is_none());
        assert!(
            root.resolution()
                .source()
                .unwrap()
                .contains("module sealed_gain")
        );
        let included = dependencies
            .iter()
            .find(|dependency| dependency.parent().is_some())
            .expect("transitive included dependency");
        assert_eq!(included.requested_locator(), "behavioral/gain_constants.va");
        assert_eq!(
            included.resolution().source(),
            Some("`define RSPICE_GAIN 1.0\n")
        );
    }
}

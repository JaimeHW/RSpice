//! The netlist document toolbar.
//!
//! One 33-point row: language and ownership status on the left, document
//! actions on the right. Which actions appear is decided by the active
//! document's ownership, and the advisory chip yields its space before the
//! language label or a blocking status can clip.

use egui::containers::menu::MenuButton;
use egui::{Align, Layout, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::{WorkbenchIcon, icon_button};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::{MessageId, RSpiceApp};

use super::ownership::{open_owned_source, open_ownership_dialog};
use super::revision::owned_source_save_ready;
use super::{
    DocumentStatusTone, active_document_available, document_syntax_status, generated_primary_ready,
    generation_block_reason,
};

pub(super) const CODE_TOOLBAR_HEIGHT: f32 = 33.0;
pub(super) const CODE_TOOLBAR_PADDING_X: f32 = 8.0;
pub(super) const CODE_TOOLBAR_GAP: f32 = 5.0;
pub(super) const CODE_TOOLBAR_ACTION_GUTTER: f32 = 12.0;
pub(super) const CODE_TOOLBAR_COMPACT_BREAKPOINT: f32 = 720.0;
pub(super) const CODE_TOOLBAR_TABLET_VIEWPORT_BREAKPOINT: f32 = 1024.0;
pub(super) const CODE_TOOLBAR_FULL_STATUS_MIN_WIDTH: f32 = 320.0;
pub(super) const PHONE_BREAKPOINT: f32 = 560.0;
pub(super) const PHONE_PRIMARY_WIDTH: f32 = 154.0;
pub(super) const CODE_TOOLBAR_ICON_WIDTH: f32 = 28.0;

/// Where the toolbar put its two groups.
///
/// The content rect is what the strip under the toolbar aligns to, so the two
/// rows cannot drift apart. The other two are the layout contract this file
/// used to get wrong: an action group wider than its reservation printed over
/// the status chips, and no constant could be trusted to notice.
pub(super) struct CodeToolbarLayout {
    pub content: egui::Rect,
    /// Exact rect the status chips were painted into.
    #[cfg(test)]
    pub status: egui::Rect,
    /// Exact rect the action group actually occupied, not the reservation.
    #[cfg(test)]
    pub actions: egui::Rect,
}

/// Width of one auto-sized toolbar button, measured the way egui will size it.
fn action_button_width(ui: &Ui, label: &str) -> f32 {
    let font = egui::TextStyle::Button.resolve(ui.style());
    let text = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, ui.visuals().text_color())
        .size()
        .x;
    text + 2.0 * ui.spacing().button_padding.x
}

/// Total width of a group laid out with one gap between each control.
fn action_extent(widths: &[f32]) -> f32 {
    if widths.is_empty() {
        return 0.0;
    }
    widths.iter().sum::<f32>() + CODE_TOOLBAR_GAP * (widths.len() - 1) as f32
}

/// Width the full action set needs, measured from the labels it will lay out.
///
/// This decides whether the full set is affordable at all. Where the group
/// actually lands is read back after it is painted — a predicted width is a
/// good enough answer for "does this fit?" and was never a safe one for "what
/// may the status chips have?".
fn full_action_extent(
    ui: &Ui,
    app: &RSpiceApp,
    messages: crate::workbench::MessageCatalog,
    save_source_label: &str,
    context: ActionSetContext,
) -> f32 {
    let button = |label: &str| action_button_width(ui, label);
    // The editor command menu, then find, source lifecycle and language tools
    // as icon buttons.
    let mut widths = vec![
        button(crate::workbench::documents::text_editor_commands::EDITOR_COMMAND_MENU_LABEL),
        CODE_TOOLBAR_ICON_WIDTH,
        CODE_TOOLBAR_ICON_WIDTH,
        CODE_TOOLBAR_ICON_WIDTH,
    ];
    if context.dependency_visible {
        widths.push(button(&messages.text(MessageId::NetlistReturnRoot)));
        widths.push(button(&messages.text(MessageId::NetlistRelink)));
        if !context.dependency_owned {
            widths.push(button(&messages.text(MessageId::NetlistCopyProject)));
        }
    } else {
        match context.active {
            ActiveNetlistDocument::Generated => {
                if !context.source_exists {
                    widths.push(CODE_TOOLBAR_ICON_WIDTH);
                }
                widths.push(button(&messages.text(if context.source_exists {
                    MessageId::NetlistOpenEditable
                } else if context.generated_ready {
                    MessageId::NetlistCreateEditable
                } else {
                    MessageId::CodeSourceCreateTitle
                })));
            }
            ActiveNetlistDocument::OwnedSource => {
                widths.push(button(save_source_label));
                widths.push(button(&messages.text(MessageId::NetlistValidateSource)));
                widths.push(button(&messages.text(if context.generated_ready {
                    MessageId::NetlistReturnPrimary
                } else {
                    MessageId::CodeSourceLifecycleTitle
                })));
            }
            ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {
                widths.push(button(&messages.text(MessageId::NetlistReturnPrimary)));
            }
        }
    }
    widths.push(run_control_width(ui, app, messages));
    action_extent(&widths)
}

/// What decides which actions the toolbar paints.
#[derive(Clone, Copy)]
struct ActionSetContext {
    active: ActiveNetlistDocument,
    dependency_visible: bool,
    dependency_owned: bool,
    generated_ready: bool,
    source_exists: bool,
}

pub(super) fn code_toolbar(ui: &mut Ui, app: &mut RSpiceApp) -> CodeToolbarLayout {
    let messages = app.state.ui.messages();
    let save_source_label = messages.text(if cfg!(target_arch = "wasm32") {
        MessageId::NetlistDownloadSourceCopy
    } else {
        MessageId::NetlistSaveSourceDeck
    });
    let t = Tokens::get(ui.ctx());
    // A child UI can retain the pre-dock available width while its painter is
    // clipped to the visible center document. Base both allocation and
    // responsive projection on the width a user can actually see.
    let width = code_toolbar_visible_width(ui.available_width(), ui.clip_rect().width());
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
    let dependency_visible = app.state.ui.netlist.active_dependency_identity.is_some();
    let dependency_owned =
        crate::workbench::documents::netlist_document::active_dependency_is_owned(&app.state);
    let dependency_authority =
        crate::workbench::documents::netlist_document::active_dependency(&app.state)
            .map(crate::state::DependencyMetadata::authority);
    let generated_ready = generated_primary_ready(&app.state);
    let active_available = active_document_available(&app.state);
    let active_editable =
        crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
            &app.state,
        );
    let quick_fix_available =
        crate::workbench::documents::netlist_document::language::preferred_quick_fix_available(
            &app.state,
        );
    let full_action_width = full_action_extent(
        ui,
        app,
        messages,
        &save_source_label,
        ActionSetContext {
            active,
            dependency_visible,
            dependency_owned,
            generated_ready,
            source_exists: app.state.workspace.netlist_source.is_some(),
        },
    );
    // Dock collapse can make the document wider at a smaller outer viewport.
    // Decide from both the canonical breakpoint and the space left after the
    // exact full action set, so that discontinuity cannot re-enable a crowded
    // toolbar on tablet landscape.
    let compact = code_toolbar_prefers_compact(
        crate::ui::viewport::root_viewport_width(ui.ctx()),
        width,
        full_action_width,
    );
    let language = match active {
        ActiveNetlistDocument::Generated => messages.text(MessageId::NetlistLanguageGenerated),
        ActiveNetlistDocument::OwnedSource => {
            app.state.workspace.netlist_descriptor.as_ref().map_or_else(
                || messages.text(MessageId::NetlistLanguageOwned),
                |descriptor| {
                    messages.text(match descriptor.strategy {
                        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
                            MessageId::NetlistLanguageOwned
                        }
                        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
                            MessageId::NetlistLanguageParameterOverride
                        }
                        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
                            MessageId::NetlistLanguageIncludeOrderOverride
                        }
                        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
                            MessageId::NetlistLanguageAnalysisDeck
                        }
                    })
                },
            )
        }
        ActiveNetlistDocument::GeneratedDiff => messages.text(MessageId::NetlistLanguageDiff),
        ActiveNetlistDocument::RunSnapshot => messages.text(MessageId::NetlistLanguageRunSnapshot),
    };
    let language = if dependency_visible {
        if dependency_owned {
            messages.text(MessageId::NetlistLanguageOwnedInclude)
        } else {
            messages.text(match dependency_authority.unwrap_or_default() {
                crate::state::DependencySourceAuthority::External => {
                    MessageId::NetlistLanguageExternalInclude
                }
                crate::state::DependencySourceAuthority::Vendor => {
                    MessageId::NetlistLanguageVendorInclude
                }
                crate::state::DependencySourceAuthority::TechnologyPackage => {
                    MessageId::NetlistLanguageTechnologyInclude
                }
                crate::state::DependencySourceAuthority::StandardLibrary => {
                    MessageId::NetlistLanguageStandardInclude
                }
            })
        }
    } else {
        language
    };
    let (status, status_tone) = document_syntax_status(&app.state);
    let status_color = match status_tone {
        DocumentStatusTone::Valid => t.color.ok,
        DocumentStatusTone::Warning => t.color.warn,
        DocumentStatusTone::Error => t.color.err,
    };
    let status_visible = toolbar_status_visible(phone, status_tone);
    let advisory_candidate = (!compact).then(|| {
        app.state
            .ui
            .netlist
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.is_current()
                    && diagnostic.severity
                        != crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
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
    let mut action = None;
    // The actions go down first, over the whole content rect. What the status
    // may occupy is then whatever they left, so the two groups cannot overlap
    // however wide a localized label turns out to be.
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(Layout::right_to_left(Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = CODE_TOOLBAR_GAP;
    if compact {
        actions.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_enabled_ui(active_available, |ui| {
                crate::workbench::documents::text_editor_commands::editor_command_menu(
                    ui,
                    crate::workbench::documents::netlist_document::editor_id(&app.state),
                    crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
                        &app.state,
                    ),
                    false,
                );
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistEditorCommandsUnavailable));
            let mut find_clicked = false;
            ui.add_enabled_ui(active_available, |ui| {
                find_clicked = icon_button(
                    ui,
                    WorkbenchIcon::Search,
                    &messages.text(MessageId::NetlistFindActiveDocument),
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked();
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistSearchUnavailable));
            if find_clicked {
                action = Some(NetlistToolbarAction::Find);
            }
            let manage_source = icon_button(
                ui,
                WorkbenchIcon::Folder,
                &messages.text(MessageId::CodeSourceLifecycleTitle),
                false,
                vec2(28.0, 28.0),
            );
            if manage_source.clicked() {
                action = Some(NetlistToolbarAction::ManageTopDecks);
            }
            if let Some(language_action) = language_tools_menu(
                ui,
                messages,
                active_available && active != ActiveNetlistDocument::GeneratedDiff,
                active_editable,
                quick_fix_available,
            ) {
                action = Some(language_action);
            }
            if dependency_visible {
                let (label, candidate) = if dependency_owned {
                    (
                        messages.text(MessageId::NetlistReturnRoot),
                        NetlistToolbarAction::CloseDependency,
                    )
                } else {
                    (
                        messages.text(MessageId::NetlistCopyProject),
                        NetlistToolbarAction::CopyDependency,
                    )
                };
                if ui
                    .add_sized(
                        [PHONE_PRIMARY_WIDTH, 28.0],
                        egui::Button::new(&label).truncate(),
                    )
                    .clicked()
                {
                    action = Some(candidate);
                }
                if !dependency_owned
                    && icon_button(
                        ui,
                        WorkbenchIcon::ArrowLeft,
                        &messages.text(MessageId::NetlistReturnRoot),
                        false,
                        vec2(28.0, 28.0),
                    )
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::CloseDependency);
                }
                if icon_button(
                    ui,
                    WorkbenchIcon::Refresh,
                    &messages.text(MessageId::NetlistRelinkTooltip),
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked()
                {
                    action = Some(NetlistToolbarAction::RelinkDependency);
                }
            } else {
            match active {
                ActiveNetlistDocument::Generated => {
                    let source_exists = app.state.workspace.netlist_source.is_some();
                    let (label, full_label, candidate) = if source_exists {
                        (
                            messages.text(MessageId::NetlistOpenEditableCompact),
                            messages.text(MessageId::NetlistOpenEditable),
                            NetlistToolbarAction::OpenOwned,
                        )
                    } else if generated_ready {
                        (
                            messages.text(MessageId::NetlistCreateEditableCompact),
                            messages.text(MessageId::NetlistCreateEditable),
                            NetlistToolbarAction::OpenOwnershipDialog(
                                crate::state::OwnedNetlistEditStrategy::OwnedSource,
                            ),
                        )
                    } else {
                        (
                            messages.text(MessageId::CodeSourceCreateTitle),
                            messages.text(MessageId::CodeSourceCreateTitle),
                            NetlistToolbarAction::BeginFirstTopDeck,
                        )
                    };
                    let primary_ready = source_exists
                        || generated_ready
                        || app.state.project_lifecycle.project_open;
                    let primary = ui
                        .add_enabled_ui(primary_ready, |ui| {
                            ui.add_sized(
                                [PHONE_PRIMARY_WIDTH, 28.0],
                                egui::Button::new(&label).truncate(),
                            )
                        })
                        .inner
                        .on_hover_text(full_label)
                        .on_disabled_hover_text(generation_block_reason(&app.state));
                    if primary.clicked()
                    {
                        action = Some(candidate);
                    }
                }
                ActiveNetlistDocument::OwnedSource => {
                    let save_ready = owned_source_save_ready(app);
                    let save = ui
                        .add_enabled_ui(save_ready, |ui| {
                            ui.add_sized(
                                [PHONE_PRIMARY_WIDTH, 28.0],
                                egui::Button::new(
                                    save_source_label.as_str(),
                                )
                                .truncate(),
                            )
                        })
                        .inner
                        .on_disabled_hover_text(
                            messages.text(MessageId::NetlistValidateBeforeSave),
                        );
                    if save.clicked()
                    {
                        action = Some(NetlistToolbarAction::Save);
                    }
                }
                ActiveNetlistDocument::GeneratedDiff => {
                    if ui
                        .add_sized(
                            [PHONE_PRIMARY_WIDTH, 28.0],
                            egui::Button::new(
                                messages.text(MessageId::NetlistReturnGeneratedPrimary),
                            )
                            .truncate(),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::CloseComparison);
                    }
                }
                ActiveNetlistDocument::RunSnapshot => {
                    if ui
                        .add_sized(
                            [PHONE_PRIMARY_WIDTH, 28.0],
                            egui::Button::new(messages.text(MessageId::NetlistReturnPrimary))
                                .truncate(),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::CloseRunSnapshot);
                    }
                }
            }
            }
            if let Some(run) = run_control(ui, app, messages) {
                action = Some(run);
            }
        });
    } else {
        actions.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_enabled_ui(active_available, |ui| {
                crate::workbench::documents::text_editor_commands::editor_command_menu(
                    ui,
                    crate::workbench::documents::netlist_document::editor_id(&app.state),
                    crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
                        &app.state,
                    ),
                    false,
                );
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistEditorCommandsUnavailable));
            let mut find_clicked = false;
            ui.add_enabled_ui(active_available, |ui| {
                find_clicked = icon_button(
                    ui,
                    WorkbenchIcon::Search,
                    &messages.text(MessageId::NetlistFindActiveDocument),
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked();
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistSearchUnavailable));
            if find_clicked {
                action = Some(NetlistToolbarAction::Find);
            }
            let manage_source = icon_button(
                ui,
                WorkbenchIcon::Folder,
                &messages.text(MessageId::CodeSourceLifecycleTitle),
                false,
                vec2(28.0, 28.0),
            );
            if manage_source.clicked() {
                action = Some(NetlistToolbarAction::ManageTopDecks);
            }
            if let Some(language_action) = language_tools_menu(
                ui,
                messages,
                active_available && active != ActiveNetlistDocument::GeneratedDiff,
                active_editable,
                quick_fix_available,
            ) {
                action = Some(language_action);
            }

            if dependency_visible {
                if ui
                    .button(messages.text(MessageId::NetlistReturnRoot))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::CloseDependency);
                }
                if ui.button(messages.text(MessageId::NetlistRelink)).clicked() {
                    action = Some(NetlistToolbarAction::RelinkDependency);
                }
                if !dependency_owned
                    && ui
                        .button(messages.text(MessageId::NetlistCopyProject))
                        .clicked()
                {
                    action = Some(NetlistToolbarAction::CopyDependency);
                }
            } else {
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
                    let source_exists = app.state.workspace.netlist_source.is_some();
                    let (label, candidate) = if source_exists {
                        (
                            messages.text(MessageId::NetlistOpenEditable),
                            NetlistToolbarAction::OpenOwned,
                        )
                    } else if generated_ready {
                        (
                            messages.text(MessageId::NetlistCreateEditable),
                            NetlistToolbarAction::OpenOwnershipDialog(
                                crate::state::OwnedNetlistEditStrategy::OwnedSource,
                            ),
                        )
                    } else {
                        (
                            messages.text(MessageId::CodeSourceCreateTitle),
                            NetlistToolbarAction::BeginFirstTopDeck,
                        )
                    };
                    let primary_ready = source_exists
                        || generated_ready
                        || app.state.project_lifecycle.project_open;
                    let response = ui
                        .add_enabled(
                            primary_ready,
                            egui::Button::new(&label).min_size(vec2(0.0, 28.0)),
                        )
                        .on_disabled_hover_text(generation_block_reason(&app.state));
                    if response.clicked() {
                        action = Some(candidate);
                    }
                }
                ActiveNetlistDocument::OwnedSource => {
                    let save_ready = owned_source_save_ready(app);
                    if ui
                        .add_enabled(
                            save_ready,
                            egui::Button::new(save_source_label.as_str()),
                        )
                        .on_disabled_hover_text(
                            messages.text(MessageId::NetlistValidateBeforeSave),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::Save);
                    }
                    if ui
                        .button(messages.text(MessageId::NetlistValidateSource))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::Validate);
                    }
                    if generated_ready {
                        if ui
                            .button(messages.text(MessageId::NetlistReturnPrimary))
                            .clicked()
                        {
                            action = Some(NetlistToolbarAction::OpenGenerated);
                        }
                    } else if ui
                        .button(messages.text(MessageId::CodeSourceLifecycleTitle))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::ManageTopDecks);
                    }
                }
                ActiveNetlistDocument::GeneratedDiff => {
                    if ui
                        .button(messages.text(MessageId::NetlistReturnPrimary))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::CloseComparison);
                    }
                }
                ActiveNetlistDocument::RunSnapshot => {
                    if ui
                        .button(messages.text(MessageId::NetlistReturnPrimary))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::CloseRunSnapshot);
                    }
                }
            }
            }
            if let Some(run) = run_control(ui, app, messages) {
                action = Some(run);
            }
        });
    }
    // What the group actually occupied, not what it was predicted to need. The
    // action half is only ever read back by this file's layout test, so it stays
    // on the tuple rather than becoming a binding a shipping build never uses.
    let regions = code_toolbar_regions(
        content,
        content.right()
            - actions
                .min_rect()
                .left()
                .clamp(content.left(), content.right()),
    );
    let left_rect = regions.0;

    let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let label_width = |label: &str, color| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), status_font.clone(), color)
            .size()
            .x
    };
    let status_only_width = if status_visible {
        11.0 + label_width(&status, status_color)
    } else {
        0.0
    };
    let language_width = label_width(&language, t.color.text_dim);
    let advisory_count = advisory_candidate.filter(|count| {
        let label = format!("{count} advisor{}", if *count == 1 { "y" } else { "ies" });
        toolbar_advisory_fits(
            left_rect.width(),
            language_width,
            status_only_width,
            11.0 + label_width(&label, t.color.text_faint),
        )
    });
    let advisory_label = advisory_count
        .map(|count| format!("{count} advisor{}", if count == 1 { "y" } else { "ies" }));
    let mut status_width = status_only_width;
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
                egui::RichText::new(&language)
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

    match action {
        Some(NetlistToolbarAction::OpenOwned) => {
            let _ = open_owned_source(&mut app.state);
        }
        Some(NetlistToolbarAction::BeginFirstTopDeck) => {
            let result = crate::workbench::app::open_source_document_dialog(&mut app.state)
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
        Some(NetlistToolbarAction::ManageTopDecks) => {
            if let Err(error) = crate::workbench::app::open_source_document_dialog(&mut app.state) {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        Some(NetlistToolbarAction::OpenGenerated) => {
            let _ = crate::workbench::documents::netlist_document::open_generated_primary(
                &mut app.state,
            );
        }
        Some(NetlistToolbarAction::CloseComparison) => {
            crate::workbench::documents::netlist_document::close_revision_comparison(
                &mut app.state,
            );
        }
        Some(NetlistToolbarAction::CloseRunSnapshot) => {
            crate::workbench::documents::netlist_document::close_run_deck_snapshot(&mut app.state);
        }
        Some(NetlistToolbarAction::RunDeck) => {
            crate::workbench::commands::vocabulary::Command::RunSimulation.execute(app);
        }
        Some(NetlistToolbarAction::StopRun) => {
            crate::workbench::commands::vocabulary::Command::StopSimulation.execute(app);
        }
        Some(NetlistToolbarAction::CloseDependency) => {
            crate::workbench::documents::netlist_document::close_active_dependency(&mut app.state);
        }
        Some(NetlistToolbarAction::CopyDependency) => {
            match crate::workbench::documents::netlist_document::copy_active_dependency_to_project(
                &mut app.state,
            ) {
                Ok(_) => app.state.push_user_message(ConsoleMessage::info(
                    messages.text(MessageId::NetlistCopySucceeded),
                )),
                Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
            }
        }
        Some(NetlistToolbarAction::RelinkDependency) => {
            if let Some(identity) = app.state.ui.netlist.active_dependency_identity.clone() {
                crate::workbench::workflows::netlist_workflow::request_dependency_relink(
                    &mut app.state,
                    &identity,
                );
            }
        }
        Some(NetlistToolbarAction::OpenOwnershipDialog(strategy)) => {
            open_ownership_dialog(&mut app.state, strategy);
        }
        Some(NetlistToolbarAction::Validate) => {
            crate::workbench::commands::vocabulary::Command::ValidateCodeDocument.execute(app);
        }
        Some(NetlistToolbarAction::Save) => {
            crate::workbench::documents::netlist_document::open_netlist_save_dialog(
                &mut app.state,
                false,
            );
        }
        Some(NetlistToolbarAction::Find) => {
            crate::workbench::commands::vocabulary::Command::FindCodeDocument.execute(app);
        }
        Some(NetlistToolbarAction::GoToDefinition) => {
            match crate::workbench::documents::netlist_document::language::go_to_definition_at_cursor(
                &mut app.state,
            ) {
                Ok(message) => app.state.push_user_message(ConsoleMessage::info(message)),
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        Some(NetlistToolbarAction::GoToDeclaration) => {
            match crate::workbench::documents::netlist_document::language::go_to_declaration_at_cursor(
                &mut app.state,
            ) {
                Ok(message) => app.state.push_user_message(ConsoleMessage::info(message)),
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        Some(NetlistToolbarAction::SignatureHelp) => {
            match crate::workbench::documents::netlist_document::language::show_signature_help_at_cursor(
                &mut app.state,
            ) {
                Ok(message) => {
                    ui.ctx().memory_mut(|memory| {
                        memory.request_focus(
                            crate::workbench::documents::netlist_document::editor_id(&app.state),
                        );
                    });
                    app.state.push_user_message(ConsoleMessage::info(message));
                }
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        Some(NetlistToolbarAction::FindReferences) => {
            match crate::workbench::documents::netlist_document::language::find_references_at_cursor(
                &mut app.state,
            ) {
                Ok(message) => app.state.push_user_message(ConsoleMessage::info(message)),
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        Some(NetlistToolbarAction::RenameSymbol) => {
            if let Err(error) = crate::workbench::documents::netlist_document::language::begin_rename_at_cursor(
                &mut app.state,
            ) {
                app.state.push_user_message(ConsoleMessage::warning(error));
            }
        }
        Some(NetlistToolbarAction::WorkspaceSymbols) => {
            match crate::workbench::documents::netlist_document::language::open_workspace_symbols(
                &mut app.state,
            ) {
                Ok(message) => app.state.push_user_message(ConsoleMessage::info(message)),
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        Some(NetlistToolbarAction::ApplyQuickFix) => {
            match crate::workbench::documents::netlist_document::language::apply_preferred_quick_fix(
                &mut app.state,
            ) {
                Ok(message) => app.state.push_user_message(ConsoleMessage::info(message)),
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        Some(NetlistToolbarAction::FormatDocument) => {
            crate::workbench::documents::text_editor_commands::queue_format_document(
                ui.ctx(),
                crate::workbench::documents::netlist_document::editor_id(&app.state),
            );
        }
        None => {}
    }

    CodeToolbarLayout {
        content,
        #[cfg(test)]
        status: status_rect,
        #[cfg(test)]
        actions: regions.1,
    }
}

/// The deck's Run/Stop control.
///
/// Both states dispatch the shared command vocabulary rather than the manual
/// deck gate directly, so the palette, the Simulate menu and this button can
/// never disagree about what Run does or why it is unavailable.
fn run_control_label(
    app: &RSpiceApp,
    messages: crate::workbench::MessageCatalog,
) -> (bool, String) {
    let execution_active = app.state.simulation.has_active_execution();
    let label = messages.text(if execution_active {
        MessageId::NetlistStopRun
    } else {
        MessageId::NetlistRunDeck
    });
    (execution_active, label)
}

/// The run control is sized to its own label like every other full-set button,
/// so "Run deck" can never come out as "Run de…".
fn run_control_width(ui: &Ui, app: &RSpiceApp, messages: crate::workbench::MessageCatalog) -> f32 {
    action_button_width(ui, &run_control_label(app, messages).1)
}

fn run_control(
    ui: &mut Ui,
    app: &RSpiceApp,
    messages: crate::workbench::MessageCatalog,
) -> Option<NetlistToolbarAction> {
    use crate::workbench::commands::CommandAvailability;
    use crate::workbench::commands::vocabulary::Command;

    let (execution_active, label) = run_control_label(app, messages);
    let (command, action) = if execution_active {
        (Command::StopSimulation, NetlistToolbarAction::StopRun)
    } else {
        (Command::RunSimulation, NetlistToolbarAction::RunDeck)
    };
    let availability = command.availability(app);
    let enabled = availability == CommandAvailability::Available;
    // The registry's generic reason names the run set's plan, which this page
    // does not have. The deck gate is the authority here, exactly as the
    // chrome run control resolves it for this workspace.
    let blocked = if execution_active {
        match availability {
            CommandAvailability::Disabled(reason) => reason.to_owned(),
            CommandAvailability::Available | CommandAvailability::Hidden => {
                messages.text(MessageId::NetlistGenerateBeforeAction)
            }
        }
    } else {
        app.manual_deck_run_block_reason()
            .unwrap_or_else(|| messages.text(MessageId::NetlistGenerateBeforeAction))
    };
    let response = ui
        .add_enabled_ui(enabled, |ui| {
            ui.add(egui::Button::new(&label).min_size(vec2(0.0, 28.0)))
        })
        .inner
        .on_disabled_hover_text(blocked);
    response.clicked().then_some(action)
}

fn language_tools_menu(
    ui: &mut Ui,
    messages: crate::workbench::MessageCatalog,
    enabled: bool,
    editable: bool,
    quick_fix_available: bool,
) -> Option<NetlistToolbarAction> {
    let mut action = None;
    let (response, _) = ui
        .add_enabled_ui(enabled, |ui| {
            ui.spacing_mut().button_padding = vec2(0.0, 0.0);
            MenuButton::from_button(
                egui::Button::new(egui::WidgetText::default())
                    .frame(false)
                    .min_size(vec2(28.0, 28.0)),
            )
            .ui(ui, |ui| {
                ui.set_min_width(180.0);
                if ui
                    .button(messages.text(MessageId::NetlistSignatureHelp))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::SignatureHelp);
                    ui.close();
                }
                if ui
                    .button(messages.text(MessageId::NetlistGoToDeclaration))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::GoToDeclaration);
                    ui.close();
                }
                if ui
                    .button(messages.text(MessageId::NetlistGoToDefinition))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::GoToDefinition);
                    ui.close();
                }
                if ui
                    .button(messages.text(MessageId::NetlistFindReferences))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::FindReferences);
                    ui.close();
                }
                if ui
                    .button(messages.text(MessageId::NetlistRenameSymbol))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::RenameSymbol);
                    ui.close();
                }
                if ui
                    .button(messages.text(MessageId::NetlistWorkspaceSymbols))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::WorkspaceSymbols);
                    ui.close();
                }
                ui.separator();
                ui.menu_button(messages.text(MessageId::NetlistCodeActions), |ui| {
                    if ui
                        .add_enabled(
                            editable && quick_fix_available,
                            egui::Button::new(messages.text(MessageId::NetlistApplyQuickFix)),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::ApplyQuickFix);
                        ui.close();
                    }
                    if ui
                        .add_enabled(
                            editable,
                            egui::Button::new(messages.text(MessageId::NetlistFormatDocument)),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::FormatDocument);
                        ui.close();
                    }
                });
            })
        })
        .inner;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            messages.text(MessageId::NetlistLanguageTools),
        )
    });
    ui.painter().rect_filled(
        response.rect,
        2.0,
        if response.hovered() {
            Tokens::get(ui.ctx()).color.bg_hover
        } else {
            egui::Color32::TRANSPARENT
        },
    );
    WorkbenchIcon::Target.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), vec2(16.0, 16.0)),
        Tokens::get(ui.ctx()).color.text_dim,
    );
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_text(messages.text(MessageId::NetlistLanguageTools));
    action
}

pub(super) const fn code_toolbar_compact(width: f32) -> bool {
    width <= CODE_TOOLBAR_COMPACT_BREAKPOINT
}

pub(super) fn code_toolbar_visible_width(available_width: f32, clip_width: f32) -> f32 {
    available_width.min(clip_width).max(0.0)
}

pub(super) fn code_toolbar_prefers_compact(
    viewport_width: f32,
    local_width: f32,
    full_action_width: f32,
) -> bool {
    if viewport_width <= CODE_TOOLBAR_TABLET_VIEWPORT_BREAKPOINT
        || code_toolbar_compact(local_width)
    {
        return true;
    }
    let content_width = (local_width - CODE_TOOLBAR_PADDING_X * 2.0).max(0.0);
    let left_width = (content_width - full_action_width - CODE_TOOLBAR_ACTION_GUTTER).max(0.0);
    left_width < CODE_TOOLBAR_FULL_STATUS_MIN_WIDTH
}

pub(super) fn code_toolbar_regions(
    content: egui::Rect,
    action_width: f32,
) -> (egui::Rect, egui::Rect) {
    let action_width = action_width.clamp(0.0, content.width());
    let right = egui::Rect::from_min_max(
        egui::pos2(content.right() - action_width, content.top()),
        content.right_bottom(),
    );
    let left = egui::Rect::from_min_max(
        content.left_top(),
        egui::pos2(
            (right.left() - CODE_TOOLBAR_ACTION_GUTTER).max(content.left()),
            content.bottom(),
        ),
    );
    (left, right)
}

pub(super) fn toolbar_advisory_fits(
    left_width: f32,
    language_width: f32,
    status_width: f32,
    advisory_width: f32,
) -> bool {
    let status_group_width = status_width
        + if status_width > 0.0 {
            CODE_TOOLBAR_GAP
        } else {
            0.0
        }
        + advisory_width;
    language_width + CODE_TOOLBAR_GAP + status_group_width <= left_width
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

pub(super) const fn toolbar_status_visible(phone: bool, tone: DocumentStatusTone) -> bool {
    !(phone && matches!(tone, DocumentStatusTone::Warning))
}

#[derive(Debug, Clone, Copy)]
enum NetlistToolbarAction {
    OpenOwned,
    BeginFirstTopDeck,
    ManageTopDecks,
    OpenGenerated,
    CloseComparison,
    CloseRunSnapshot,
    RunDeck,
    StopRun,
    CloseDependency,
    CopyDependency,
    RelinkDependency,
    OpenOwnershipDialog(crate::state::OwnedNetlistEditStrategy),
    Validate,
    Save,
    Find,
    SignatureHelp,
    GoToDeclaration,
    GoToDefinition,
    FindReferences,
    RenameSymbol,
    WorkspaceSymbols,
    ApplyQuickFix,
    FormatDocument,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A netlist-first project: an owned deck and no generated primary. This is
    /// the shape whose action set is widest, because the third full-set button
    /// is "Manage source document" rather than "Return to primary".
    fn netlist_first_app() -> RSpiceApp {
        const DECK: &str = "toolbar fixture\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        app.state.workspace.netlist_source = Some(DECK.to_owned());
        app.state.simulation.netlist_content = DECK.to_owned();
        app.state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        app.state.ui.netlist.active_document_initialized = true;
        app
    }

    /// A schematic-first project: a generated primary is retained and active.
    fn schematic_first_app() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        super::super::tests::retain_generated(
            &mut app.state,
            "generated\nR1 out 0 1k\n.op\n.end\n",
        );
        app.state.simulation.netlist_content = app.state.ui.netlist.generated_source.clone();
        app.state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        app.state.ui.netlist.active_document_initialized = true;
        app
    }

    /// Build one application, paint the toolbar over it, and read the layout
    /// back. The application is built and dropped here so a caller never holds
    /// two of them at once.
    fn toolbar_layout(build: fn() -> RSpiceApp, width: f32) -> CodeToolbarLayout {
        let mut app = build();
        let mut captured = None;
        crate::ui::raster::render(vec2(width, 120.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    captured = Some(code_toolbar(ui, &mut app));
                });
        });
        captured.expect("the toolbar paints on every pass")
    }

    /// The action group must never reach the status chips.
    ///
    /// This replaces a set of frozen width tallies. They were hand-tuned, drifted
    /// as controls were added, and by the time the run control arrived the group
    /// over-ran its reservation by more than a hundred points and printed on top
    /// of the status — which no assertion about a constant could catch. The
    /// reservation is now measured from the labels, and this reads back where
    /// the two groups actually landed.
    #[test]
    fn the_action_group_never_reaches_the_status_chips() {
        // One application at a time: two live `AppState`s on a test thread's
        // stack is an overflow, not a fixture.
        for width in [1000.0, 1600.0, 2560.0] {
            for (shape, build) in [
                ("netlist-first", netlist_first_app as fn() -> RSpiceApp),
                ("schematic-first", schematic_first_app as fn() -> RSpiceApp),
            ] {
                let layout = toolbar_layout(build, width);
                assert!(
                    layout.actions.left() >= layout.status.right(),
                    "{shape} at {width}: actions start at {} but the status chips end at {}",
                    layout.actions.left(),
                    layout.status.right()
                );
                assert!(
                    layout.actions.right() <= layout.content.right() + 0.5,
                    "{shape} at {width}: actions overflow the toolbar content rect"
                );
            }
        }
    }
}

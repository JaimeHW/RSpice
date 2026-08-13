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
pub(super) const PHONE_BREAKPOINT: f32 = 560.0;
pub(super) const PHONE_PRIMARY_WIDTH: f32 = 154.0;
pub(super) const EDITOR_MENU_WIDTH: f32 = 58.0;
pub(super) const PHONE_ACTION_WIDTH: f32 =
    PHONE_PRIMARY_WIDTH + CODE_TOOLBAR_GAP * 2.0 + 28.0 + EDITOR_MENU_WIDTH;
pub(super) fn code_toolbar(ui: &mut Ui, app: &mut RSpiceApp) {
    let messages = app.state.ui.messages();
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
    let dependency_visible = app.state.ui.netlist.active_dependency_identity.is_some();
    let dependency_owned =
        crate::workbench::documents::netlist_document::active_dependency_is_owned(&app.state);
    let dependency_authority =
        crate::workbench::documents::netlist_document::active_dependency(&app.state)
            .map(crate::state::DependencyMetadata::authority);
    let generated_ready = generated_primary_ready(&app.state);
    let active_available = active_document_available(&app.state);
    let action_width: f32 = if compact {
        PHONE_ACTION_WIDTH
    } else if dependency_visible {
        if dependency_owned { 280.0 } else { 390.0 }
    } else {
        (match active {
            ActiveNetlistDocument::Generated => {
                if app.state.workspace.netlist_source.is_some() {
                    175.0
                } else {
                    342.0
                }
            }
            ActiveNetlistDocument::OwnedSource => 348.0,
            ActiveNetlistDocument::GeneratedDiff => 152.0,
        }) + EDITOR_MENU_WIDTH
            + CODE_TOOLBAR_GAP
    };
    let (left_rect, right_rect) = code_toolbar_regions(content, action_width);
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

    let mut action = None;
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(right_rect)
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
            if let Some(language_action) = language_tools_menu(
                ui,
                messages,
                active_available && active != ActiveNetlistDocument::GeneratedDiff,
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
                    let (label, candidate) = if app.state.workspace.netlist_source.is_some() {
                        (
                            messages.text(MessageId::NetlistOpenEditable),
                            NetlistToolbarAction::OpenOwned,
                        )
                    } else {
                        (
                            messages.text(MessageId::NetlistCreateEditable),
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
                            egui::Button::new(&label)
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
                            egui::Button::new(messages.text(MessageId::NetlistSaveSourceDeck))
                                .truncate()
                                .min_size(vec2(PHONE_PRIMARY_WIDTH, 28.0)),
                        )
                        .on_disabled_hover_text(
                            messages.text(MessageId::NetlistValidateBeforeSave),
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
            }
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
            if let Some(language_action) = language_tools_menu(
                ui,
                messages,
                active_available && active != ActiveNetlistDocument::GeneratedDiff,
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
                    let label = if app.state.workspace.netlist_source.is_some() {
                        messages.text(MessageId::NetlistOpenEditable)
                    } else {
                        messages.text(MessageId::NetlistCreateEditable)
                    };
                    let primary_ready =
                        app.state.workspace.netlist_source.is_some() || generated_ready;
                    let response = ui
                        .add_enabled(
                            primary_ready,
                            egui::Button::new(&label).min_size(vec2(0.0, 28.0)),
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
                        .add_enabled(
                            save_ready,
                            egui::Button::new(messages.text(MessageId::NetlistSaveSourceDeck)),
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
                    if ui
                        .button(messages.text(MessageId::NetlistReturnPrimary))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::OpenGenerated);
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
            }
            }
        });
    }

    match action {
        Some(NetlistToolbarAction::OpenOwned) => {
            let _ = open_owned_source(&mut app.state);
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
            crate::workbench::commands::vocabulary::Command::Save.execute(app);
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
        None => {}
    }
}

fn language_tools_menu(
    ui: &mut Ui,
    messages: crate::workbench::MessageCatalog,
    enabled: bool,
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
    OpenGenerated,
    CloseComparison,
    CloseDependency,
    CopyDependency,
    RelinkDependency,
    OpenOwnershipDialog(crate::state::OwnedNetlistEditStrategy),
    Validate,
    Save,
    Find,
    GoToDefinition,
    FindReferences,
    RenameSymbol,
}

//! The netlist document toolbar.
//!
//! One row, a compact control high with the bar's padding above and below
//! it: the dialect chip and the check's status on the left, Find, Manage
//! source and the language tools on the right, the same on every tab; what
//! a tab can do with its text is on the workbench toolbar. The warning count
//! yields its space before the dialect chip or a blocking status can clip.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Align, Layout, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::MessageId;
use crate::workbench::code::SourceFeature;
use crate::workbench::code::commands::SourceRequest;
use crate::workbench::design_system::{Problem, WorkbenchIcon, icon_button, paint_problem_mark};
use crate::workbench::documents::netlist_document::NetlistRoot;
use crate::workbench::framework::feature::Cx;
use crate::workbench::state::workbench_session::WidthClass;

use super::{active_document_available, document_error_status};

pub(super) const CODE_TOOLBAR_PADDING_X: f32 = 8.0;
pub(super) const CODE_TOOLBAR_GAP: f32 = 5.0;
pub(super) const CODE_TOOLBAR_ACTION_GUTTER: f32 = 12.0;

/// The strip's height under `metrics`: a compact control, which a finger's
/// target raises, with the bar's padding above and below it.
#[must_use]
/// The toolbar stands where the document bar stands in the other
/// workspaces, so it is the document bar's height and ends on the line the
/// dock headers beside it end on.
pub fn code_toolbar_height(viewport: egui::Vec2, coarse_pointer: bool) -> f32 {
    crate::workbench::layout::document_bar_height(viewport.x, viewport.y, coarse_pointer)
}
/// Side of a status line's problem mark and the air after it: the kit's own,
/// so a marked status matches every other marked line in the product.
use crate::ui::widgets::{
    PROBLEM_MARK_GAP as CODE_STATUS_MARK_GAP, PROBLEM_MARK_SIDE as CODE_STATUS_MARK_SIDE,
};
/// Width that mark takes out of a status line, gap included.
const CODE_STATUS_MARK_COLUMN: f32 = CODE_STATUS_MARK_SIDE + CODE_STATUS_MARK_GAP;
/// Room either side of a count's mark and words, inside the fill it takes
/// under the pointer.
const CODE_STATUS_PRESS_PADDING: f32 = 4.0;

/// Where the toolbar put its two groups.
///
/// The content rect is what the strip under the toolbar aligns to, so the two
/// rows cannot drift apart. The other two are the layout contract this file
/// used to get wrong: an action group wider than its reservation printed over
/// the status chips, and no constant could be trusted to notice.
pub struct NetlistToolbarLayout {
    pub content: egui::Rect,
    /// Exact rect the status chips were painted into.
    #[cfg(any(test, feature = "test-support"))]
    pub status: egui::Rect,
    /// Exact rect the action group actually occupied, not the reservation.
    #[cfg(any(test, feature = "test-support"))]
    pub actions: egui::Rect,
}

/// What the language tools menu offers for the text in front.
#[derive(Clone, Copy)]
struct LanguageTools {
    /// The text in front has a language service at all: a comparison has
    /// none.
    enabled: bool,
    /// The text in front may be rewritten: the quick fix and Format.
    editable: bool,
    quick_fix_available: bool,
    /// The editor in front answers where a name is defined and written.
    navigable: bool,
    /// ...and may rename it.
    renamable: bool,
}

/// Draw the toolbar in the source feature's context and do what was pressed:
/// in place for what the context holds, and raised for what the application
/// carries out once the page has drawn: Find references, Rename symbol and
/// the quick fix. It returns where it drew.
pub fn code_toolbar(ui: &mut Ui, cx: &mut Cx<'_, SourceFeature>) -> NetlistToolbarLayout {
    let messages = cx.wb.messages();
    let t = Tokens::get(ui.ctx());
    // A child UI can retain the pre-dock available width while its painter is
    // clipped to the visible center document. Base both allocation and
    // responsive projection on the width a user can actually see.
    let width = code_toolbar_visible_width(ui.available_width(), ui.clip_rect().width());
    // The shell's layout class, the one every workspace lays out by.
    let phone = WidthClass::for_width(ui.ctx().content_rect().width()) == WidthClass::Phone;
    let (rect, _) = ui.allocate_exact_size(
        vec2(
            width,
            code_toolbar_height(ui.ctx().content_rect().size(), cx.wb.coarse_pointer),
        ),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );

    let content = rect.shrink2(vec2(CODE_TOOLBAR_PADDING_X, 0.0));
    let active = cx.state.netlist.front.root_kind();
    let active_available = active_document_available(cx.model, &cx.state.netlist, cx.runs);
    let editor = cx.wb.active_source_editor;
    let tools = LanguageTools {
        enabled: active_available && active != NetlistRoot::Comparison,
        editable:
            crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in(
                cx.model,
                &cx.state.netlist,
            ),
        quick_fix_available:
            crate::workbench::documents::netlist_document::language::preferred_quick_fix_available(
                &cx.state.netlist,
            ),
        navigable: editor.is_some_and(|editor| editor.navigable),
        renamable: editor.is_some_and(|editor| editor.navigable && editor.editable),
    };
    // Every control in the strip stands at the kit's compact control height
    // and the icons are square at it, so they sit level with the dialect
    // chip, an equal gap above and below them in the row.
    let control_h = crate::ui::widgets::compact_height(&t.metrics);
    let dialect = super::dialect_chip::DeckChip::shown(cx.model, &cx.state.netlist);
    let status = document_error_status(cx.model, &cx.state.netlist, messages)
        .filter(|(_, problem)| toolbar_status_visible(phone, *problem));
    // Nothing is said when there are no warnings, and "0 warnings" is not
    // news: the count appears only once there is something in it, and where
    // it fits beside the chip and the status. A note is not a warning, as
    // the status bar counts it.
    let warnings = cx
        .state
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning
        })
        .count()
        + cx.model
            .netlist_artifacts
            .validation
            .as_ref()
            .map_or(0, |receipt| receipt.advisory_count)
        + usize::from(cx.model.netlist_artifacts.validation_error.is_some());
    let warning_candidate = (warnings > 0).then_some(warnings);
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
    let side = vec2(control_h, control_h);
    let find = actions
        .add_enabled(active_available, |ui: &mut Ui| {
            icon_button(
                ui,
                WorkbenchIcon::Search,
                &messages.text(MessageId::NetlistFindActiveDocument),
                false,
                side,
            )
        })
        .on_disabled_hover_text(messages.text(MessageId::NetlistSearchUnavailable));
    if find.clicked() {
        action = Some(NetlistToolbarAction::Find);
    }
    let manage_source = actions.add(|ui: &mut Ui| {
        icon_button(
            ui,
            WorkbenchIcon::Folder,
            &messages.text(MessageId::NetlistManageDeckTitle),
            false,
            side,
        )
    });
    if manage_source.clicked() {
        action = Some(NetlistToolbarAction::ManageTopDecks);
    }
    if let Some(language_action) = language_tools_menu(&mut actions, messages, side, tools) {
        action = Some(language_action);
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
    let status_only_width = status.as_ref().map_or(0.0, |(label, problem)| {
        status_extent(
            label_width(label, t.color.text_dim),
            counts_findings(*problem),
        )
    });
    let language_width = dialect.map_or(0.0, |chip| chip.width(ui, messages));
    let warning_text = |count: usize| {
        messages.format(
            if count == 1 {
                MessageId::NetlistWarningSingular
            } else {
                MessageId::NetlistWarnings
            },
            &[("count", &count.to_string())],
        )
    };
    let warning_label = warning_candidate.map(warning_text).filter(|label| {
        toolbar_warnings_fit(
            left_rect.width(),
            language_width,
            status_only_width,
            status_extent(label_width(label, t.color.text_dim), true),
        )
    });
    let mut status_width = status_only_width;
    if let Some(label) = warning_label.as_deref() {
        status_width +=
            CODE_TOOLBAR_GAP + status_extent(label_width(label, t.color.text_dim), true);
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
    if let Some(chip) = dialect.filter(|_| language_rect.width() >= language_width) {
        let mut language_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(language_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        chip.show(&mut language_ui, cx.model, &cx.state.netlist, messages);
    }
    let chip_width = dialect.filter(|_| language_rect.width() >= language_width);
    super::read_only::show_word(ui, language_rect, chip_width.map(|_| language_width), cx);
    if status_width > 0.0 {
        let mut status_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(status_rect)
                .layout(Layout::right_to_left(Align::Center)),
        );
        status_ui.spacing_mut().item_spacing.x = CODE_TOOLBAR_GAP;
        let mut open_problems = false;
        status_ui.with_layout(Layout::right_to_left(Align::Center), |bar| {
            if let Some(label) = warning_label.as_deref() {
                open_problems |= code_status(bar, label, Problem::Warning, true);
            }
            if let Some((label, problem)) = status.as_ref() {
                open_problems |= code_status(bar, label, *problem, counts_findings(*problem));
            }
        });
        // A count is read in full on the Problems page, whose rows go to the
        // line: where the inspector is a drawer, the only place it is.
        if open_problems {
            cx.wb.show_problem_findings();
        }
    }

    match action {
        Some(NetlistToolbarAction::ManageTopDecks) => {
            if let Err(error) = crate::workbench::app::open_source_document_dialog_in(
                cx.wb.workspace,
                cx.model,
                &mut cx.state.source_editing,
                &cx.state.netlist,
                cx.platform, cx.wb.messages(),
            ) {
                cx.wb.prompt(error);
            }
        }
        Some(NetlistToolbarAction::GoToDefinition) => {
            let answer =
                crate::workbench::documents::netlist_document::language::go_to_definition_at_cursor_in(
                    cx.model,
                    &mut cx.state.netlist,
                    &mut cx.state.netlist_open_documents,
                    cx.runs,
                    cx.wb,
                );
            cx.wb.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        Some(NetlistToolbarAction::GoToDeclaration) => {
            let answer =
                crate::workbench::documents::netlist_document::language::go_to_declaration_at_cursor(
                    cx.model,
                    &mut cx.state.netlist,
                    &mut cx.state.netlist_open_documents,
                    cx.runs,
                    cx.wb,
                );
            cx.wb.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        Some(NetlistToolbarAction::FindReferences) => {
            cx.execute(crate::workbench::code::commands::SourceCommand::FindReferences.key());
        }
        Some(NetlistToolbarAction::RenameSymbol) => {
            cx.execute(crate::workbench::code::commands::SourceCommand::RenameSymbol.key());
        }
        Some(NetlistToolbarAction::SignatureHelp) => {
            match crate::workbench::documents::netlist_document::language::show_signature_help_at_cursor(
                cx.model,
                &mut cx.state.netlist,
                cx.wb.messages(),
            ) {
                Ok(message) => {
                    ui.ctx().memory_mut(|memory| {
                        memory.request_focus(
                            crate::workbench::documents::netlist_document::editor_id_in(
                                cx.model,
                                &cx.state.netlist,
                            ),
                        );
                    });
                    cx.wb.prompt(message);
                }
                Err(error) => cx.wb.prompt(error),
            }
        }
        Some(NetlistToolbarAction::WorkspaceSymbols) => {
            let answer =
                crate::workbench::documents::netlist_document::language::open_workspace_symbols(
                    cx.model,
                    &mut cx.state.netlist,
                    cx.wb,
                );
            cx.wb.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        Some(NetlistToolbarAction::Find) => {
            // What Find in source does on this page: the deck belongs to no
            // project source bundle, so the command opens the page's own
            // find window, which draws later in this frame.
            cx.state.netlist.find.open = true;
        }
        Some(NetlistToolbarAction::FormatDocument) => {
            crate::workbench::documents::text_editor_commands::queue_format_document(
                ui.ctx(),
                crate::workbench::documents::netlist_document::editor_id_in(
                    cx.model,
                    &cx.state.netlist,
                ),
            );
        }
        Some(NetlistToolbarAction::ApplyQuickFix) => {
            cx.wb.outbox.push(SourceRequest::ApplyNetlistQuickFix);
        }
        None => {}
    }

    NetlistToolbarLayout {
        content,
        #[cfg(any(test, feature = "test-support"))]
        status: status_rect,
        #[cfg(any(test, feature = "test-support"))]
        actions: regions.1,
    }
}

/// The language tools: an icon drawn as Find and Manage source are, which
/// opens the menu of what the language service answers at the caret.
fn language_tools_menu(
    ui: &mut Ui,
    messages: crate::workbench::MessageCatalog,
    side: egui::Vec2,
    tools: LanguageTools,
) -> Option<NetlistToolbarAction> {
    let mut action = None;
    let response = ui.add_enabled(tools.enabled, |ui: &mut Ui| {
        icon_button(
            ui,
            WorkbenchIcon::Target,
            &messages.text(MessageId::NetlistLanguageTools),
            false,
            side,
        )
    });
    // A row the text in front cannot have says why when it is read-only.
    let read_only = (!tools.editable).then_some(MessageId::SourceReadOnlyDocument);
    egui::Popup::menu(&response).show(|ui| {
        ui.set_min_width(180.0);
        let mut row =
            |ui: &mut Ui, enabled: bool, label: MessageId, chosen, why: Option<MessageId>| {
                let mut response = ui.add_enabled(enabled, egui::Button::new(messages.text(label)));
                if let Some(why) = why {
                    response = response.on_disabled_hover_text(messages.text(why));
                }
                if response.clicked() {
                    action = Some(chosen);
                    ui.close();
                }
            };
        row(
            ui,
            true,
            MessageId::NetlistSignatureHelp,
            NetlistToolbarAction::SignatureHelp,
            None,
        );
        row(
            ui,
            true,
            MessageId::NetlistGoToDeclaration,
            NetlistToolbarAction::GoToDeclaration,
            None,
        );
        row(
            ui,
            true,
            MessageId::NetlistGoToDefinition,
            NetlistToolbarAction::GoToDefinition,
            None,
        );
        row(
            ui,
            tools.navigable,
            MessageId::NetlistFindReferences,
            NetlistToolbarAction::FindReferences,
            None,
        );
        row(
            ui,
            true,
            MessageId::NetlistWorkspaceSymbols,
            NetlistToolbarAction::WorkspaceSymbols,
            None,
        );
        ui.separator();
        row(
            ui,
            tools.renamable,
            MessageId::NetlistRenameSymbol,
            NetlistToolbarAction::RenameSymbol,
            read_only,
        );
        rspice_ui_kit::widgets::menu_button(
            ui,
            messages.text(MessageId::NetlistCodeActions),
            |ui| {
                row(
                    ui,
                    tools.editable && tools.quick_fix_available,
                    MessageId::NetlistApplyQuickFix,
                    NetlistToolbarAction::ApplyQuickFix,
                    read_only,
                );
                row(
                    ui,
                    tools.editable,
                    MessageId::NetlistFormatDocument,
                    NetlistToolbarAction::FormatDocument,
                    read_only,
                );
            },
        );
    });
    action
}

pub(super) fn code_toolbar_visible_width(available_width: f32, clip_width: f32) -> f32 {
    available_width.min(clip_width).max(0.0)
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

pub(super) fn toolbar_warnings_fit(
    left_width: f32,
    language_width: f32,
    status_width: f32,
    warnings_width: f32,
) -> bool {
    let status_group_width = status_width
        + if status_width > 0.0 {
            CODE_TOOLBAR_GAP
        } else {
            0.0
        }
        + warnings_width;
    language_width + CODE_TOOLBAR_GAP + status_group_width <= left_width
}

/// Whether a status is a count of the check's findings, which the reader
/// presses to list them: the errors are a failure, and the generation's
/// refusal is a warning that names no finding.
const fn counts_findings(problem: Problem) -> bool {
    matches!(problem, Problem::Failure)
}

/// Width one status takes: its mark and its words, and the room either side
/// of a count the reader presses.
fn status_extent(label_width: f32, pressable: bool) -> f32 {
    CODE_STATUS_MARK_COLUMN
        + label_width
        + if pressable {
            2.0 * CODE_STATUS_PRESS_PADDING
        } else {
            0.0
        }
}

/// One problem the toolbar reports: its mark, then the words in the ordinary
/// tone, at the strip's control height. A count of the check's findings is
/// pressed to list them on the Problems page; whether it was.
fn code_status(ui: &mut Ui, label: &str, problem: Problem, pressable: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let (rect, response) = ui.allocate_exact_size(
        vec2(
            status_extent(galley.size().x, pressable),
            crate::ui::widgets::compact_height(&t.metrics),
        ),
        if pressable {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let mut left = rect.left();
    if pressable {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
        });
        if response.hovered() {
            ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
        }
        left += CODE_STATUS_PRESS_PADDING;
    }
    let mark = egui::Rect::from_min_size(
        egui::pos2(left, rect.center().y - CODE_STATUS_MARK_SIDE * 0.5),
        vec2(CODE_STATUS_MARK_SIDE, CODE_STATUS_MARK_SIDE),
    );
    paint_problem_mark(ui.painter(), mark, problem);
    ui.painter().galley(
        egui::pos2(
            mark.right() + CODE_STATUS_MARK_GAP,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        t.color.text_dim,
    );
    if !pressable {
        return false;
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

pub(super) const fn toolbar_status_visible(phone: bool, problem: Problem) -> bool {
    !(phone && matches!(problem, Problem::Warning))
}

#[derive(Debug, Clone, Copy)]
enum NetlistToolbarAction {
    ManageTopDecks,
    SignatureHelp,
    GoToDeclaration,
    GoToDefinition,
    FindReferences,
    RenameSymbol,
    WorkspaceSymbols,
    Find,
    FormatDocument,
    ApplyQuickFix,
}

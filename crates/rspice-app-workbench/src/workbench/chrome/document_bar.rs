//! Workspace-scoped document tabs. Stable project identities drive every tab;
//! the shell never fabricates datasets, source files, or verification records.

use egui::containers::menu::MenuButton;
use egui::{Frame, Layout, Panel, Rect, Sense, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::framework::host::DocumentHost;
use crate::workbench::framework::workbench_host::{ConsoleHost, WorkbenchHost};

use super::super::layout::LayoutSpec;
use super::super::state::{Workspace, WorkspaceDocumentId};
use crate::ui::panels::{WorkbenchIcon, elide_text};
use crate::workbench::state::workbench_session::WorkspaceDocument;

pub const TAB_MAX_WIDTH: f32 = 190.0;
/// The narrowest a tab shrinks to before the strip scrolls instead: its icon,
/// about eleven characters of its name, and its close button.
pub const TAB_MIN_WIDTH: f32 = 140.0;
const TAB_PADDING_X: f32 = 10.0;
const TAB_ICON_SIZE: f32 = 16.0;
const TAB_CONTENT_GAP: f32 = 7.0;
const TAB_CLOSE_SIZE: f32 = 16.0;
const TAB_DIRTY_SIZE: f32 = 6.0;
const TAB_ACTIVE_EDGE: f32 = 2.0;
const STRIP_FADE_WIDTH: f32 = 24.0;
const HIDDEN_BUTTON_WIDTH: f32 = 40.0;
/// Past this many hidden tabs, their menu offers a filter.
const HIDDEN_MENU_FILTER_THRESHOLD: usize = 8;

/// The session the bar and its commands are handed: the workbench session's
/// record of the open documents, and the shell's answers for the documents
/// each workspace offers. Any session that is both is one.
pub trait DocumentSession:
    ConsoleHost
    + DocumentHost<
        WorkspaceKey = Workspace,
        DocumentId = WorkspaceDocumentId,
        Document = WorkspaceDocument,
    >
{
}

impl<S> DocumentSession for S where
    S: ConsoleHost
        + DocumentHost<
            WorkspaceKey = Workspace,
            DocumentId = WorkspaceDocumentId,
            Document = WorkspaceDocument,
        >
{
}

pub fn show(root: &mut Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    reconcile_document_registry(app.session_mut());
    let documents = visible_documents(app.session());
    if !document_strip_visible(app.workbench().workspace, documents.len()) {
        return;
    }

    let t = Tokens::get(ctx);
    // The strip is window chrome and wears the toolbar's tone. The active tab
    // wears the page's and covers the hairline, so it opens onto the document
    // beneath it; painted in the page's tone, the strip ran into the page.
    Panel::top("workbench.document_bar")
        .exact_size(layout.document_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let rect = ui.max_rect();
            let height = layout.document_bar_height;
            ui.painter().hline(
                rect.x_range(),
                (rect.bottom() - 0.5).max(rect.top()),
                egui::Stroke::new(1.0, t.color.border),
            );
            let natural = documents
                .iter()
                .map(|document| {
                    natural_tab_width(
                        ui.painter(),
                        &document.label,
                        document.dirty,
                        document_is_closable(app.session(), &document.id),
                    )
                })
                .collect::<Vec<_>>();
            let hidden_button_width = HIDDEN_BUTTON_WIDTH.max(t.metrics.ctl_h);
            let fit = StripFit::resolve(&natural, rect.width(), hidden_button_width);
            ui.horizontal(|ui| {
                let scroll = ui
                    .allocate_ui_with_layout(
                        Vec2::new(fit.tabs_width, height),
                        Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            // The track, not the content: the button after it
                            // must sit at the strip's right end.
                            ui.set_min_width(fit.tabs_width);
                            egui::ScrollArea::horizontal()
                                .id_salt("workbench.document_tabs.scroll")
                                .scroll_bar_visibility(
                                    egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                                )
                                .show(ui, |ui| {
                                    let tabs = ui.horizontal(|ui| {
                                        document_tabs(
                                            ui,
                                            app.session_mut(),
                                            &documents,
                                            height,
                                            fit.cap,
                                        )
                                    });
                                    ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
                                        node.set_role(egui::accesskit::Role::TabList);
                                        node.set_label("Open documents");
                                    });
                                    tabs.inner
                                })
                        },
                    )
                    .inner;
                paint_scroll_fades(ui, &scroll, t.color.bg_panel);
                if fit.overflowing {
                    let hidden = documents
                        .iter()
                        .zip(&scroll.inner)
                        .filter(|(_, tab)| !in_view(scroll.inner_rect, **tab))
                        .map(|(document, _)| document)
                        .collect::<Vec<_>>();
                    hidden_documents_button(
                        ui,
                        app.session_mut(),
                        &hidden,
                        Vec2::new(hidden_button_width, height),
                    );
                }
            });
        });
}

/// How the strip's tabs share its width.
///
/// Tabs keep their own widths while they fit. Past that they shrink evenly,
/// the narrow ones keeping theirs, down to [`TAB_MIN_WIDTH`]. Only past that
/// does the strip scroll, and only then does it give up room for the button
/// that lists the tabs scrolled out of view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripFit {
    /// The widest a tab may be, when the tabs do not fit at their own widths.
    pub cap: Option<f32>,
    /// The width the tabs scroll within.
    pub tabs_width: f32,
    /// Whether some tabs are out of view even at the narrowest a tab may be.
    pub overflowing: bool,
}

impl StripFit {
    pub fn resolve(natural: &[f32], strip_width: f32, hidden_button_width: f32) -> Self {
        let narrowest: f32 = natural.iter().map(|width| width.min(TAB_MIN_WIDTH)).sum();
        let overflowing = narrowest > strip_width + 0.5;
        let tabs_width = if overflowing {
            (strip_width - hidden_button_width).max(1.0)
        } else {
            strip_width
        };
        Self {
            cap: shared_width_cap(natural, tabs_width),
            tabs_width,
            overflowing,
        }
    }
}

/// The widest a tab may be for the tabs to fill `available`, or `None` when
/// they fit at their own widths. Tabs narrower than the cap keep their width,
/// and the cap never goes below [`TAB_MIN_WIDTH`].
pub fn shared_width_cap(natural: &[f32], available: f32) -> Option<f32> {
    if natural.iter().sum::<f32>() <= available + 0.5 {
        return None;
    }
    let mut widths = natural.to_vec();
    widths.sort_by(f32::total_cmp);
    let mut remaining = available;
    for (index, width) in widths.iter().enumerate() {
        let share = remaining / (widths.len() - index) as f32;
        if *width > share {
            return Some(share.max(TAB_MIN_WIDTH));
        }
        remaining -= width;
    }
    None
}

/// Fade the strip's ends where tabs carry on out of view, above the hairline.
fn paint_scroll_fades(
    ui: &Ui,
    scroll: &egui::scroll_area::ScrollAreaOutput<Vec<Rect>>,
    color: egui::Color32,
) {
    let band = scroll
        .inner_rect
        .with_max_y(scroll.inner_rect.bottom() - 1.0);
    let before = scroll.state.offset.x;
    let after = scroll.content_size.x - band.width() - before;
    let width = STRIP_FADE_WIDTH.min(band.width() * 0.5);
    if before > 0.5 {
        let edge = Rect::from_min_size(band.left_top(), Vec2::new(width, band.height()));
        paint_horizontal_fade(ui.painter(), edge, color, egui::Color32::TRANSPARENT);
    }
    if after > 0.5 {
        let edge = Rect::from_min_max(egui::pos2(band.right() - width, band.top()), band.max);
        paint_horizontal_fade(ui.painter(), edge, egui::Color32::TRANSPARENT, color);
    }
}

fn paint_horizontal_fade(
    painter: &egui::Painter,
    rect: Rect,
    left: egui::Color32,
    right: egui::Color32,
) {
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(rect.left_top(), left);
    mesh.colored_vertex(rect.right_top(), right);
    mesh.colored_vertex(rect.right_bottom(), right);
    mesh.colored_vertex(rect.left_bottom(), left);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(mesh);
}

/// "+N" at the strip's right end while tabs are out of view; its menu lists
/// those tabs and only those, and a choice opens the document, which scrolls
/// its tab into view.
fn hidden_documents_button(
    ui: &mut Ui,
    state: &mut impl DocumentSession,
    hidden: &[&WorkspaceDocument],
    size: Vec2,
) {
    let t = Tokens::get(ui.ctx());
    let active = active_document_id(state);
    let description = match hidden.len() {
        1 => "1 more open document".to_owned(),
        count => format!("{count} more open documents"),
    };
    let filter_id = egui::Id::new("workbench.document_tabs.hidden_filter");
    let mut chosen = None;
    let response = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            MenuButton::from_button(egui::Button::new("").frame(false).min_size(size))
                .ui(ui, |ui| {
                    ui.set_min_width(240.0);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    let mut filter = String::new();
                    if hidden.len() > HIDDEN_MENU_FILTER_THRESHOLD {
                        filter = ui
                            .data(|data| data.get_temp::<String>(filter_id))
                            .unwrap_or_default();
                        ui.add(
                            egui::TextEdit::singleline(&mut filter)
                                .hint_text("Filter")
                                .desired_width(f32::INFINITY),
                        );
                        ui.data_mut(|data| data.insert_temp(filter_id, filter.clone()));
                        ui.add_space(4.0);
                    }
                    let needle = filter.trim().to_lowercase();
                    let row_height = if size.y >= 44.0 { 44.0 } else { 29.0 };
                    for document in hidden
                        .iter()
                        .filter(|document| document.label.to_lowercase().contains(&needle))
                    {
                        let row = ui.add_sized(
                            [ui.available_width(), row_height],
                            egui::Button::selectable(
                                active.as_ref() == Some(&document.id),
                                &document.label,
                            ),
                        );
                        if row.clicked() {
                            chosen = Some(document.id.clone());
                            ui.close();
                        }
                    }
                })
                .0
        })
        .inner;
    if let Some(document) = chosen {
        ui.data_mut(|data| data.remove::<String>(filter_id));
        activate_document(state, &document);
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &description)
    });
    ui.painter().rect_filled(
        response.rect,
        0.0,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_panel
        },
    );
    ui.painter().vline(
        response.rect.left(),
        response.rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        response.rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("+{}", hidden.len()),
        theme::sans(tokens::FS_1, FontWeight::Medium),
        if response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_text(description);
}

pub fn is_visible(app: &impl WorkbenchHost) -> bool {
    document_strip_visible(
        app.workbench().workspace,
        visible_documents(app.session()).len(),
    )
}

/// Read-only document metadata used by shell-owned document/session managers.
///
/// The descriptor is deliberately derived from the same authoritative source
/// as the tab strip. Consumers cannot mutate project documents by editing a
/// cached presentation model.
#[derive(Debug, Clone)]
pub struct WorkspaceDocumentDescriptor {
    pub id: WorkspaceDocumentId,
    pub label: String,
    pub dirty: bool,
    pub active: bool,
    pub open: bool,
    pub closable: bool,
    pub(crate) owner: crate::workbench::lifecycle::window_session::ApplicationWindowId,
}

pub fn document_descriptors(state: &impl DocumentSession) -> Vec<WorkspaceDocumentDescriptor> {
    let available = available_documents(state);
    let active = authoritative_active_document(state, &available);
    available
        .into_iter()
        .enumerate()
        .map(|(index, document)| WorkspaceDocumentDescriptor {
            open: index == 0
                || !state.workbench().documents.is_closed(&document.id)
                || active.as_ref() == Some(&document.id),
            active: active.as_ref() == Some(&document.id),
            closable: index > 0,
            owner: state.workbench().window_session.owner(&document.id),
            id: document.id,
            label: document.label,
            dirty: document.dirty,
        })
        .collect()
}

pub fn all_document_descriptors(state: &impl DocumentSession) -> Vec<WorkspaceDocumentDescriptor> {
    Workspace::ALL
        .into_iter()
        .flat_map(|workspace| {
            available_documents_for_workspace(state, workspace)
                .into_iter()
                .enumerate()
                .map(move |(index, document)| {
                    let owner = state.workbench().window_session.owner(&document.id);
                    let active = state
                        .workbench()
                        .window_session
                        .state(owner)
                        .and_then(|window| window.active_document.as_ref())
                        == Some(&document.id);
                    WorkspaceDocumentDescriptor {
                        open: index == 0
                            || !state.workbench().documents.is_closed(&document.id)
                            || active,
                        active,
                        closable: index > 0,
                        owner,
                        id: document.id,
                        label: document.label,
                        dirty: document.dirty,
                    }
                })
        })
        .collect()
}

pub fn active_document_id(state: &impl DocumentSession) -> Option<WorkspaceDocumentId> {
    let available = owned_available_documents(state);
    authoritative_active_document(state, &available)
}

pub fn activate_document_by_id(
    state: &mut impl DocumentSession,
    document: &WorkspaceDocumentId,
) -> bool {
    activate_document(state, document)
}

pub fn cycle_document(state: &mut impl DocumentSession, previous: bool) -> bool {
    reconcile_document_registry(state);
    let documents = visible_documents(state);
    if documents.len() < 2 {
        return false;
    }
    let active = authoritative_active_document(state, &documents);
    let current = active
        .as_ref()
        .and_then(|active| documents.iter().position(|document| document.id == *active))
        .unwrap_or_default();
    let next = if previous {
        (current + documents.len() - 1) % documents.len()
    } else {
        (current + 1) % documents.len()
    };
    activate_document(state, &documents[next].id)
}

pub fn close_other_documents(state: &mut impl DocumentSession) -> usize {
    reconcile_document_registry(state);
    let documents = owned_available_documents(state);
    let active = authoritative_active_document(state, &documents);
    let mut closed = 0;
    for document in &documents {
        if document_is_closable(state, &document.id) && active.as_ref() != Some(&document.id) {
            let was_closed = state.workbench().documents.is_closed(&document.id);
            state.workbench_mut().documents.close(&document.id);
            closed += usize::from(!was_closed);
        }
    }
    closed
}

pub fn close_all_documents(state: &mut impl DocumentSession) -> usize {
    if !state.commit_pending_edit() {
        return 0;
    }
    reconcile_document_registry(state);
    let documents = owned_available_documents(state);
    let root = documents
        .iter()
        .find(|document| !document_is_closable(state, &document.id))
        .or_else(|| documents.first())
        .map(|document| document.id.clone());
    let Some(root) = root else {
        return 0;
    };
    if !activate_document(state, &root) {
        return 0;
    }
    let mut closed = 0;
    for document in &documents {
        if !document_is_closable(state, &document.id) {
            continue;
        }
        let was_closed = state.workbench().documents.is_closed(&document.id);
        state.workbench_mut().documents.close(&document.id);
        closed += usize::from(!was_closed);
    }
    closed
}

fn available_documents(state: &impl DocumentSession) -> Vec<WorkspaceDocument> {
    available_documents_for_workspace(state, state.workbench().workspace)
}

pub fn available_documents_for_workspace(
    state: &impl DocumentSession,
    workspace: Workspace,
) -> Vec<WorkspaceDocument> {
    state.workspace_documents(workspace)
}

fn owned_available_documents(state: &impl DocumentSession) -> Vec<WorkspaceDocument> {
    let current = state.workbench().window_session.current();
    let order = state
        .workbench()
        .window_session
        .state(current)
        .map(|window| window.documents.clone())
        .unwrap_or_default();
    let mut documents = available_documents(state)
        .into_iter()
        .filter(|document| {
            state
                .workbench()
                .window_session
                .belongs_to_current(&document.id)
        })
        .collect::<Vec<_>>();
    documents.sort_by_key(|document| {
        order
            .iter()
            .position(|candidate| candidate == &document.id)
            .unwrap_or(usize::MAX)
    });
    documents
}

fn document_is_closable(state: &impl DocumentSession, document: &WorkspaceDocumentId) -> bool {
    available_documents_for_workspace(state, document.workspace())
        .first()
        .is_some_and(|root| root.id != *document)
}

fn authoritative_active_document(
    state: &impl DocumentSession,
    available: &[WorkspaceDocument],
) -> Option<WorkspaceDocumentId> {
    if let Some(document) = state.page_document(state.workbench().workspace)
        && available.iter().any(|candidate| candidate.id == document)
    {
        return Some(document);
    }
    let current_window = state.workbench().window_session.current();
    if let Some(document) = state
        .workbench()
        .window_session
        .active_document(current_window, state.workbench().workspace)
        .filter(|document| {
            available.iter().any(|candidate| candidate.id == **document)
                && state.workbench().window_session.owner(document) == current_window
        })
    {
        return Some(document.clone());
    }
    let candidate = match state.workbench().workspace {
        Workspace::Design => Some(WorkspaceDocumentId::CellView(
            state.model().workspace.content.active_view.clone(),
        )),
        Workspace::Results => state
            .model()
            .simulation
            .active_run()
            .map(|run| WorkspaceDocumentId::ResultDataset(run.dataset_id)),
        workspace => state.workbench().documents.active(workspace).cloned(),
    };
    candidate
        .filter(|candidate| {
            available.iter().any(|document| document.id == *candidate)
                && state.workbench().window_session.owner(candidate) == current_window
        })
        .or_else(|| {
            available
                .iter()
                .find(|document| {
                    state.workbench().window_session.owner(&document.id) == current_window
                })
                .map(|document| document.id.clone())
        })
}

pub fn visible_documents(state: &impl DocumentSession) -> Vec<WorkspaceDocument> {
    let available = owned_available_documents(state);
    let active = authoritative_active_document(state, &available);
    available
        .into_iter()
        .enumerate()
        .filter(|(index, document)| {
            *index == 0
                || !state.workbench().documents.is_closed(&document.id)
                || active.as_ref() == Some(&document.id)
        })
        .map(|(_, document)| document)
        .collect()
}

fn reconcile_document_registry(state: &mut impl DocumentSession) {
    let available = available_documents(state);
    let workspace = state.workbench().workspace;
    state.workbench_mut().documents.retain_available(
        workspace,
        available.iter().map(|document| document.id.clone()),
    );
    if let Some(active) = authoritative_active_document(state, &available) {
        state.workbench_mut().documents.activate(active);
    }
    let all_available = Workspace::ALL
        .into_iter()
        .flat_map(|workspace| available_documents_for_workspace(state, workspace))
        .map(|document| document.id)
        .collect::<Vec<_>>();
    state
        .workbench_mut()
        .window_session
        .retain_documents(all_available);
}

pub fn document_strip_visible(workspace: Workspace, document_count: usize) -> bool {
    document_count > 1
        || (matches!(workspace, Workspace::Design | Workspace::Models) && document_count == 1)
}

/// Draw every tab, at most `cap` wide when the strip has asked them to
/// share it, and return where each one went, in document order.
fn document_tabs(
    ui: &mut egui::Ui,
    state: &mut impl DocumentSession,
    documents: &[WorkspaceDocument],
    height: f32,
    cap: Option<f32>,
) -> Vec<Rect> {
    let active = authoritative_active_document(state, documents);
    let mut open = None;
    let mut close = None;
    let mut focus_index = None;
    let mut tab_ids = Vec::with_capacity(documents.len());
    let mut tab_rects = Vec::with_capacity(documents.len());

    for (index, document) in documents.iter().enumerate() {
        let selected = active.as_ref() == Some(&document.id);
        let tab = ui
            .push_id(("document", &document.id), |ui| {
                document_tab(
                    ui,
                    DocumentTabPresentation {
                        icon: document.icon,
                        label: &document.label,
                        short_label: document.short_label(),
                        selected,
                        dirty: document.dirty,
                        closable: document_is_closable(state, &document.id),
                        height,
                        max_width: cap.map_or(TAB_MAX_WIDTH, |cap| cap.min(TAB_MAX_WIDTH)),
                    },
                )
            })
            .inner;
        tab_ids.push(tab.response.id);
        tab_rects.push(tab.response.rect);
        if selected {
            reveal_newly_active_tab(ui, &tab.response);
        }

        // A pointer press activates a tab without taking keyboard focus, so
        // the focus ring appears only for a reader who is using the keys.
        if tab.close_clicked {
            close = Some((index, document.id.clone(), tab.response.has_focus()));
        } else if tab.response.clicked() && !selected {
            open = Some(document.id.clone());
        }

        if tab.response.has_focus() {
            let key = ui.input_mut(|input| {
                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                    Some(TabKey::Previous)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                    Some(TabKey::Next)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                    Some(TabKey::First)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                    Some(TabKey::Last)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Delete) {
                    Some(TabKey::Close)
                } else {
                    None
                }
            });
            match key {
                Some(TabKey::Previous) => {
                    focus_index = Some((index + documents.len() - 1) % documents.len())
                }
                Some(TabKey::Next) => focus_index = Some((index + 1) % documents.len()),
                Some(TabKey::First) => focus_index = Some(0),
                Some(TabKey::Last) => focus_index = Some(documents.len() - 1),
                Some(TabKey::Close) if document_is_closable(state, &document.id) => {
                    close = Some((index, document.id.clone(), true))
                }
                Some(TabKey::Close) | None => {}
            }
        }
    }

    if let Some(index) = focus_index {
        open = Some(documents[index].id.clone());
        ui.memory_mut(|memory| memory.request_focus(tab_ids[index]));
    }
    if let Some(document) = open {
        activate_document(state, &document);
    }
    if let Some((closed_index, document, had_focus)) = close {
        let focus_after_close = if closed_index + 1 < documents.len() {
            closed_index + 1
        } else {
            closed_index.saturating_sub(1)
        };
        if close_document(state, &document, documents) && had_focus {
            ui.memory_mut(|memory| memory.request_focus(tab_ids[focus_after_close]));
        }
    }
    tab_rects
}

/// Scroll the active tab into view when it becomes active, whether by a
/// press, Ctrl+Tab or the navigator. Only until it is in view: revealing it
/// every frame would undo every turn of the wheel over the strip, and
/// revealing it once can stop short while the scroll is still animating.
fn reveal_newly_active_tab(ui: &Ui, tab: &egui::Response) {
    let revealed = egui::Id::new("workbench.document_tabs.revealed");
    if ui.ctx().data(|data| data.get_temp::<egui::Id>(revealed)) == Some(tab.id) {
        return;
    }
    tab.scroll_to_me(None);
    let viewport = ui.clip_rect();
    if in_view(viewport, tab.rect) || tab.rect.width() >= viewport.width() {
        ui.ctx().data_mut(|data| data.insert_temp(revealed, tab.id));
    }
}

/// Whether all of `tab` is inside the strip's `viewport` across. Only the
/// horizontal extent counts: the strip scrolls one way, and its viewport can
/// be a fraction of a point shorter than the tabs it holds.
fn in_view(viewport: Rect, tab: Rect) -> bool {
    tab.left() >= viewport.left() - 0.5 && tab.right() <= viewport.right() + 0.5
}

pub fn activate_document(state: &mut impl DocumentSession, document: &WorkspaceDocumentId) -> bool {
    if document.workspace() != state.workbench().workspace {
        return false;
    }
    let activated = state.activate_owned_document(document);
    if activated {
        state.workbench_mut().documents.activate(document.clone());
        let window = state.workbench().window_session.current();
        let _ = state
            .workbench_mut()
            .window_session
            .set_active_document(window, document.clone());
    }
    activated
}

pub fn close_document(
    state: &mut impl DocumentSession,
    document: &WorkspaceDocumentId,
    documents: &[WorkspaceDocument],
) -> bool {
    let Some(closed_index) = documents
        .iter()
        .position(|candidate| candidate.id == *document)
    else {
        return false;
    };
    if !document_is_closable(state, document) || document.workspace() != state.workbench().workspace
    {
        return false;
    }
    if !state.commit_pending_edit() {
        return false;
    }
    let was_active = authoritative_active_document(state, documents).as_ref() == Some(document);
    let fallback = documents
        .get(closed_index + 1)
        .or_else(|| documents.get(closed_index.saturating_sub(1)))
        .map(|document| document.id.clone());

    let closed = state.close_owned_document(document).unwrap_or_else(|| {
        state.workbench_mut().documents.close(document);
        true
    });
    if closed
        && (was_active || matches!(document, WorkspaceDocumentId::CellView(_)))
        && let Some(fallback) = fallback
    {
        let source = state.workbench().window_session.owner(document);
        let primary = state.workbench().window_session.primary();
        if source != primary {
            let _ = state.workbench_mut().window_session.move_document(
                document.clone(),
                source,
                primary,
            );
        }
        activate_document(state, &fallback);
    }
    closed
}

#[derive(Clone, Copy)]
enum TabKey {
    Previous,
    Next,
    First,
    Last,
    Close,
}

struct DocumentTabResponse {
    response: egui::Response,
    close_clicked: bool,
}

struct DocumentTabPresentation<'a> {
    icon: WorkbenchIcon,
    label: &'a str,
    short_label: Option<&'a str>,
    selected: bool,
    dirty: bool,
    closable: bool,
    height: f32,
    max_width: f32,
}

/// The text a tab paints at `width`: its label while the label fits, else the
/// short form when it has one. A squeezed tab drops what its icon already
/// says before it cuts the name.
pub fn tab_caption<'a>(
    label: &'a str,
    short: Option<&'a str>,
    natural: f32,
    width: f32,
) -> &'a str {
    match short {
        Some(short) if natural > width + 0.5 => short,
        _ => label,
    }
}

/// The width a tab takes when nothing asks it to share: its whole name, up
/// to [`TAB_MAX_WIDTH`].
fn natural_tab_width(painter: &egui::Painter, label: &str, dirty: bool, closable: bool) -> f32 {
    let label_width = painter
        .layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            egui::Color32::WHITE,
        )
        .size()
        .x;
    (TAB_PADDING_X
        + TAB_ICON_SIZE
        + TAB_CONTENT_GAP
        + label_width
        + suffix_width(dirty, closable)
        + TAB_PADDING_X)
        .min(TAB_MAX_WIDTH)
}

fn document_tab(
    ui: &mut egui::Ui,
    presentation: DocumentTabPresentation<'_>,
) -> DocumentTabResponse {
    let DocumentTabPresentation {
        icon,
        label,
        short_label,
        selected,
        dirty,
        closable,
        height,
        max_width,
    } = presentation;
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let suffix_width = suffix_width(dirty, closable);
    let natural = natural_tab_width(ui.painter(), label, dirty, closable);
    let width = natural.min(max_width);
    let shown = tab_caption(label, short_label, natural, width);
    let mut sense = Sense::click();
    if !selected {
        sense = sense.difference(Sense::focusable_noninteractive());
    }
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
        let description = match (dirty, closable) {
            (true, true) => "Unsaved changes. Press Delete to close.",
            (true, false) => "Unsaved changes.",
            (false, true) => "Press Delete to close.",
            (false, false) => "Pinned primary document.",
        };
        node.set_description(description);
    });

    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.right(), rect.top() + TAB_ACTIVE_EDGE),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );

    let icon_center_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE * 0.5;
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(icon_center_x, rect.center().y),
            Vec2::splat(TAB_ICON_SIZE),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let label_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE + TAB_CONTENT_GAP;
    let suffix_left = rect.right() - TAB_PADDING_X - suffix_width;
    let label_clip = egui::Rect::from_min_max(
        egui::pos2(label_x, rect.top()),
        egui::pos2(suffix_left.max(label_x), rect.bottom()),
    );
    let painted_label = elide_text(ui, shown, &font, (suffix_left - label_x).max(0.0));
    let shortened = painted_label != label;
    let galley = ui
        .painter()
        .layout_no_wrap(painted_label, font, t.color.text);
    ui.painter().with_clip_rect(label_clip).galley(
        egui::pos2(label_x, rect.center().y - galley.size().y * 0.5),
        galley,
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );

    let mut suffix_x = suffix_left;
    if suffix_width > 0.0 {
        suffix_x += TAB_CONTENT_GAP;
    }
    if dirty {
        // The tab's unsaved marker is a live state, not a warning, so it is
        // painted in the secondary text tone rather than in amber.
        let center = egui::pos2(suffix_x + TAB_DIRTY_SIZE * 0.5, rect.center().y);
        ui.painter()
            .circle_filled(center, TAB_DIRTY_SIZE * 0.5, t.color.text_dim);
        suffix_x += TAB_DIRTY_SIZE;
        if closable {
            suffix_x += TAB_CONTENT_GAP;
        }
    }
    let mut close_clicked = false;
    if closable {
        let close_rect = egui::Rect::from_min_size(
            egui::pos2(suffix_x + 2.0, rect.center().y - TAB_CLOSE_SIZE * 0.5),
            Vec2::splat(TAB_CLOSE_SIZE),
        );
        // accessibility-pointer-shim: the close glyph is a pointer shortcut;
        // the semantic tab exposes the equivalent Delete-key action.
        let close_response = ui.interact(
            close_rect,
            response.id.with("close"),
            Sense::click().difference(Sense::focusable_noninteractive()),
        );
        if close_response.hovered() {
            ui.painter().rect_filled(close_rect, 2.0, t.color.bg_active);
        }
        ui.painter().text(
            close_rect.center(),
            egui::Align2::CENTER_CENTER,
            "×",
            theme::sans(14.0, FontWeight::Regular),
            if close_response.hovered() {
                t.color.text
            } else {
                t.color.text_faint
            },
        );
        close_clicked = close_response.clicked();
    }
    theme::paint_focus_ring(ui, &response, rect);
    let response = if shortened {
        response.on_hover_text(label)
    } else {
        response
    };
    DocumentTabResponse {
        response,
        close_clicked,
    }
}

pub fn suffix_width(dirty: bool, closable: bool) -> f32 {
    match (dirty, closable) {
        (false, false) => 0.0,
        (true, false) => TAB_CONTENT_GAP + TAB_DIRTY_SIZE,
        (false, true) => TAB_CONTENT_GAP + 2.0 + TAB_CLOSE_SIZE,
        (true, true) => TAB_CONTENT_GAP + TAB_DIRTY_SIZE + TAB_CONTENT_GAP + 2.0 + TAB_CLOSE_SIZE,
    }
}

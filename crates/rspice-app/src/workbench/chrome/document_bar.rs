//! Workspace-scoped document tabs. Stable project identities drive every tab;
//! the shell never fabricates datasets, source files, or verification records.

use egui::containers::menu::MenuButton;
use egui::{Frame, Layout, Panel, Rect, Sense, Ui, Vec2};

use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::netlist_document::run_deck_snapshot_run_id;
use crate::workbench::{AppState, RSpiceApp};

use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{Workspace, WorkspaceDocumentId};

const TAB_MAX_WIDTH: f32 = 190.0;
/// The narrowest a tab shrinks to before the strip scrolls instead: its icon,
/// about eleven characters of its name, and its close button.
const TAB_MIN_WIDTH: f32 = 140.0;
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

pub fn show(root: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    reconcile_document_registry(&mut app.state);
    let documents = visible_documents(&app.state);
    if !document_strip_visible(app.state.workbench.workspace, documents.len()) {
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
                        document_is_closable(&app.state, &document.id),
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
                                            &mut app.state,
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
                        &mut app.state,
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
struct StripFit {
    /// The widest a tab may be, when the tabs do not fit at their own widths.
    cap: Option<f32>,
    /// The width the tabs scroll within.
    tabs_width: f32,
    /// Whether some tabs are out of view even at the narrowest a tab may be.
    overflowing: bool,
}

impl StripFit {
    fn resolve(natural: &[f32], strip_width: f32, hidden_button_width: f32) -> Self {
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
fn shared_width_cap(natural: &[f32], available: f32) -> Option<f32> {
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
    state: &mut AppState,
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

pub(in crate::workbench) fn is_visible(app: &RSpiceApp) -> bool {
    document_strip_visible(
        app.state.workbench.workspace,
        visible_documents(&app.state).len(),
    )
}

#[derive(Debug, Clone)]
struct WorkspaceDocument {
    id: WorkspaceDocumentId,
    label: String,
    icon: WorkbenchIcon,
    dirty: bool,
}

impl WorkspaceDocument {
    /// What the tab shows when it is too narrow for its label: a cell view's
    /// cell alone, since the tab's icon already names the view.
    fn short_label(&self) -> Option<&str> {
        match &self.id {
            WorkspaceDocumentId::CellView(reference) => Some(reference.cell.as_str()),
            _ => None,
        }
    }
}

/// Read-only document metadata used by shell-owned document/session managers.
///
/// The descriptor is deliberately derived from the same authoritative source
/// as the tab strip. Consumers cannot mutate project documents by editing a
/// cached presentation model.
#[derive(Debug, Clone)]
pub(crate) struct WorkspaceDocumentDescriptor {
    pub(crate) id: WorkspaceDocumentId,
    pub(crate) label: String,
    pub(crate) dirty: bool,
    pub(crate) active: bool,
    pub(crate) open: bool,
    pub(crate) closable: bool,
    pub(crate) owner: crate::workbench::ApplicationWindowId,
}

pub(crate) fn document_descriptors(state: &AppState) -> Vec<WorkspaceDocumentDescriptor> {
    let available = available_documents(state);
    let active = authoritative_active_document(state, &available);
    available
        .into_iter()
        .enumerate()
        .map(|(index, document)| WorkspaceDocumentDescriptor {
            open: index == 0
                || !state.workbench.documents.is_closed(&document.id)
                || active.as_ref() == Some(&document.id),
            active: active.as_ref() == Some(&document.id),
            closable: index > 0,
            owner: state.workbench.window_session.owner(&document.id),
            id: document.id,
            label: document.label,
            dirty: document.dirty,
        })
        .collect()
}

pub(crate) fn all_document_descriptors(state: &AppState) -> Vec<WorkspaceDocumentDescriptor> {
    Workspace::ALL
        .into_iter()
        .flat_map(|workspace| {
            available_documents_for_workspace(state, workspace)
                .into_iter()
                .enumerate()
                .map(move |(index, document)| {
                    let owner = state.workbench.window_session.owner(&document.id);
                    let active = state
                        .workbench
                        .window_session
                        .state(owner)
                        .and_then(|window| window.active_document.as_ref())
                        == Some(&document.id);
                    WorkspaceDocumentDescriptor {
                        open: index == 0
                            || !state.workbench.documents.is_closed(&document.id)
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

pub(crate) fn active_document_id(state: &AppState) -> Option<WorkspaceDocumentId> {
    let available = owned_available_documents(state);
    authoritative_active_document(state, &available)
}

pub(crate) fn activate_document_by_id(
    state: &mut AppState,
    document: &WorkspaceDocumentId,
) -> bool {
    activate_document(state, document)
}

pub(crate) fn cycle_document(state: &mut AppState, previous: bool) -> bool {
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

pub(crate) fn close_other_documents(state: &mut AppState) -> usize {
    reconcile_document_registry(state);
    let documents = owned_available_documents(state);
    let active = authoritative_active_document(state, &documents);
    let mut closed = 0;
    for document in &documents {
        if document_is_closable(state, &document.id) && active.as_ref() != Some(&document.id) {
            let was_closed = state.workbench.documents.is_closed(&document.id);
            state.workbench.documents.close(&document.id);
            closed += usize::from(!was_closed);
        }
    }
    closed
}

pub(crate) fn close_all_documents(state: &mut AppState) -> usize {
    if !state.commit_pending_inspector_edit() {
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
        let was_closed = state.workbench.documents.is_closed(&document.id);
        state.workbench.documents.close(&document.id);
        closed += usize::from(!was_closed);
    }
    closed
}

fn available_documents(state: &AppState) -> Vec<WorkspaceDocument> {
    available_documents_for_workspace(state, state.workbench.workspace)
}

fn available_documents_for_workspace(
    state: &AppState,
    workspace: Workspace,
) -> Vec<WorkspaceDocument> {
    match workspace {
        Workspace::Project => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::Project,
            label: "Project overview".to_owned(),
            icon: WorkbenchIcon::Project,
            dirty: false,
        }],
        Workspace::Design => state
            .workspace
            .content
            .open_views
            .iter()
            .map(|document| WorkspaceDocument {
                id: WorkspaceDocumentId::CellView(document.reference.clone()),
                label: format!("{} · {}", document.reference.cell, document.reference.view),
                icon: icon_for_view(document.view_type),
                dirty: document.dirty,
            })
            .collect(),
        // One plan, one document. The Studio used to open a "<name> · setup"
        // tab per analysis instance too, and activating one only selected that
        // instance on the Analyses page — which the plan's own analysis list
        // does, permanently, on the left. A plan with seven analyses therefore
        // paid a strip of eight tabs for nothing the list did not already say,
        // so the strip is gone: `document_strip_visible` hides a lone
        // non-Design document.
        Workspace::Simulate => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::SimulationPlan,
            label: "Simulation plan".to_owned(),
            icon: WorkbenchIcon::Simulate,
            dirty: false,
        }],
        // The library is one project-owned record, and the shell has no
        // document identity for it yet — the instrument that edits a
        // definition is what will open one. Naming a tab here before that
        // exists would put a document in the window session that nothing can
        // detach, close, or save.
        Workspace::Stimulus => Vec::new(),
        Workspace::Results => state
            .simulation
            .retained
            .runs
            .iter()
            .map(|run| WorkspaceDocument {
                id: WorkspaceDocumentId::ResultDataset(run.dataset_id),
                label: run.label.clone(),
                icon: WorkbenchIcon::Results,
                dirty: false,
            })
            .collect(),
        Workspace::Verify => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::Verification,
            label: "Verification".to_owned(),
            icon: WorkbenchIcon::Verify,
            dirty: false,
        }],
        Workspace::Models => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::Models,
            label: "Models & PDKs".to_owned(),
            icon: WorkbenchIcon::Models,
            dirty: false,
        }],
        Workspace::Netlist => netlist_workspace_documents(state),
    }
}

fn netlist_workspace_documents(state: &AppState) -> Vec<WorkspaceDocument> {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    if state.ui.code_workspace.page != CodeWorkspacePage::Netlist {
        return vec![code_workspace_document(state)];
    }
    let mut documents = Vec::new();
    if let Some(generated) = state.ui.netlist.generated_document.as_ref() {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistGenerated(generated.id()),
            label: "Generated netlist".to_owned(),
            icon: WorkbenchIcon::Netlist,
            dirty: false,
        });
        documents.extend(generated.dependencies().iter().filter_map(|dependency| {
            dependency.source()?;
            let id = WorkspaceDocumentId::NetlistDependency {
                root: generated.id(),
                logical_identity: dependency.locator().logical_identity().to_owned(),
            };
            if !state.workbench.netlist_open_documents.contains(&id) {
                return None;
            }
            Some(WorkspaceDocument {
                id,
                label: format!("{} · generated", dependency.locator().display_name()),
                icon: WorkbenchIcon::Code,
                dirty: false,
            })
        }));
    }

    if let Some(owned) = state.ui.netlist.owned_document.as_ref() {
        let owned_label = state
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .filter(|label| !label.trim().is_empty())
            .unwrap_or_else(|| "Project netlist".to_owned());
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistOwned(owned.id()),
            label: owned_label,
            icon: WorkbenchIcon::Netlist,
            dirty: state.workspace.content.netlist_source_dirty,
        });
        documents.extend(owned.dependencies().iter().filter_map(|dependency| {
            dependency.source()?;
            let identity = dependency.locator().logical_identity();
            let id = WorkspaceDocumentId::NetlistDependency {
                root: owned.id(),
                logical_identity: identity.to_owned(),
            };
            if !state.workbench.netlist_open_documents.contains(&id) {
                return None;
            }
            let project_owned = state
                .workspace
                .content
                .netlist_descriptor
                .as_ref()
                .and_then(|descriptor| descriptor.owned_include(identity))
                .is_some();
            Some(WorkspaceDocument {
                id,
                label: if project_owned {
                    dependency.locator().display_name().to_owned()
                } else {
                    format!("{} · external", dependency.locator().display_name())
                },
                icon: WorkbenchIcon::Code,
                dirty: project_owned && state.workspace.content.netlist_source_dirty,
            })
        }));
    }

    if !state.ui.netlist.generated_diff_source.is_empty() {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistComparison,
            label: "Netlist comparison".to_owned(),
            icon: WorkbenchIcon::Code,
            dirty: false,
        });
    }
    if let Some(run_id) = run_deck_snapshot_run_id(state) {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistRunSnapshot,
            label: state.ui.messages().format(
                crate::workbench::MessageId::NetlistRunSnapshotDocument,
                &[("id", &run_id.to_string())],
            ),
            icon: WorkbenchIcon::Results,
            dirty: false,
        });
    }
    if documents.is_empty() {
        documents.push(code_workspace_document(state));
    }
    documents
}

fn owned_available_documents(state: &AppState) -> Vec<WorkspaceDocument> {
    let current = state.workbench.window_session.current();
    let order = state
        .workbench
        .window_session
        .state(current)
        .map(|window| window.documents.clone())
        .unwrap_or_default();
    let mut documents = available_documents(state)
        .into_iter()
        .filter(|document| {
            state
                .workbench
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

fn document_is_closable(state: &AppState, document: &WorkspaceDocumentId) -> bool {
    available_documents_for_workspace(state, document.workspace())
        .first()
        .is_some_and(|root| root.id != *document)
}

/// The Code & Automation workspace shows one document at a time and the page
/// decides which. Only the netlist page has a dirty bit to report: Automation
/// sources are a project-owned bundle whose dirtiness belongs to the bundle,
/// not to this tab.
fn code_workspace_document(state: &AppState) -> WorkspaceDocument {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let page = state.ui.code_workspace.page;
    WorkspaceDocument {
        id: WorkspaceDocumentId::NetlistSource,
        label: crate::workbench::documents::code_workspace::active_document_label(
            &state.workspace,
            &state.ui.code_workspace,
            state.ui.messages(),
        ),
        icon: match page {
            CodeWorkspacePage::Netlist => WorkbenchIcon::Netlist,
            CodeWorkspacePage::Automation => WorkbenchIcon::Terminal,
        },
        dirty: page == CodeWorkspacePage::Netlist && state.workspace.content.netlist_source_dirty,
    }
}

fn authoritative_active_document(
    state: &AppState,
    available: &[WorkspaceDocument],
) -> Option<WorkspaceDocumentId> {
    if state.workbench.workspace == Workspace::Netlist
        && let Some(document) = active_netlist_document_id(state)
        && available.iter().any(|candidate| candidate.id == document)
    {
        return Some(document);
    }
    let current_window = state.workbench.window_session.current();
    if let Some(document) = state
        .workbench
        .window_session
        .active_document(current_window, state.workbench.workspace)
        .filter(|document| {
            available.iter().any(|candidate| candidate.id == **document)
                && state.workbench.window_session.owner(document) == current_window
        })
    {
        return Some(document.clone());
    }
    let candidate = match state.workbench.workspace {
        Workspace::Design => Some(WorkspaceDocumentId::CellView(
            state.workspace.content.active_view.clone(),
        )),
        Workspace::Results => state
            .simulation
            .active_run()
            .map(|run| WorkspaceDocumentId::ResultDataset(run.dataset_id)),
        workspace => state.workbench.documents.active(workspace).cloned(),
    };
    candidate
        .filter(|candidate| {
            available.iter().any(|document| document.id == *candidate)
                && state.workbench.window_session.owner(candidate) == current_window
        })
        .or_else(|| {
            available
                .iter()
                .find(|document| {
                    state.workbench.window_session.owner(&document.id) == current_window
                })
                .map(|document| document.id.clone())
        })
}

fn active_netlist_document_id(state: &AppState) -> Option<WorkspaceDocumentId> {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;
    use crate::workbench::documents::netlist_document::ActiveNetlistDocument;

    if state.ui.code_workspace.page != CodeWorkspacePage::Netlist {
        return Some(WorkspaceDocumentId::NetlistSource);
    }
    let netlist = &state.ui.netlist;
    if netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        return Some(WorkspaceDocumentId::NetlistComparison);
    }
    if netlist.active_document == ActiveNetlistDocument::RunSnapshot {
        return Some(WorkspaceDocumentId::NetlistRunSnapshot);
    }
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    let document = match root {
        ActiveNetlistDocument::Generated => netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
    }?;
    if let Some(logical_identity) = netlist.active_dependency_identity.as_ref() {
        Some(WorkspaceDocumentId::NetlistDependency {
            root: document.id(),
            logical_identity: logical_identity.clone(),
        })
    } else {
        Some(match root {
            ActiveNetlistDocument::Generated => {
                WorkspaceDocumentId::NetlistGenerated(document.id())
            }
            ActiveNetlistDocument::OwnedSource => WorkspaceDocumentId::NetlistOwned(document.id()),
            ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {
                return None;
            }
        })
    }
}

fn visible_documents(state: &AppState) -> Vec<WorkspaceDocument> {
    let available = owned_available_documents(state);
    let active = authoritative_active_document(state, &available);
    available
        .into_iter()
        .enumerate()
        .filter(|(index, document)| {
            *index == 0
                || !state.workbench.documents.is_closed(&document.id)
                || active.as_ref() == Some(&document.id)
        })
        .map(|(_, document)| document)
        .collect()
}

fn reconcile_document_registry(state: &mut AppState) {
    let available = available_documents(state);
    let workspace = state.workbench.workspace;
    state.workbench.documents.retain_available(
        workspace,
        available.iter().map(|document| document.id.clone()),
    );
    if let Some(active) = authoritative_active_document(state, &available) {
        state.workbench.documents.activate(active);
    }
    let all_available = Workspace::ALL
        .into_iter()
        .flat_map(|workspace| available_documents_for_workspace(state, workspace))
        .map(|document| document.id)
        .collect::<Vec<_>>();
    state
        .workbench
        .window_session
        .retain_documents(all_available);
}

fn document_strip_visible(workspace: Workspace, document_count: usize) -> bool {
    document_count > 1
        || (matches!(workspace, Workspace::Design | Workspace::Models) && document_count == 1)
}

/// Draw every tab, at most `cap` wide when the strip has asked them to
/// share it, and return where each one went, in document order.
fn document_tabs(
    ui: &mut egui::Ui,
    state: &mut AppState,
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

fn activate_document(state: &mut AppState, document: &WorkspaceDocumentId) -> bool {
    if document.workspace() != state.workbench.workspace {
        return false;
    }
    let activated = match document {
        WorkspaceDocumentId::CellView(reference) => {
            if state
                .workspace
                .content
                .open_views
                .iter()
                .any(|open| open.reference == *reference)
            {
                state.open_workspace_view(reference.clone());
                state.workspace.content.active_view == *reference
            } else {
                false
            }
        }
        WorkspaceDocumentId::VisualizationDocument(document_id) => {
            crate::workbench::documents::result_document::activate_persistent_document(
                state,
                *document_id,
            )
        }
        // Re-activating the dataset that is already active must not re-select
        // its run. `select_run` resynchronizes the displayed waveforms, which
        // advances the simulation data version, and every version change
        // retires cursors, the selected trace, the active pane, and pinned
        // readouts. This path runs on every frame the document bar draws, so
        // an unguarded re-selection made those states impossible to hold.
        WorkspaceDocumentId::ResultDataset(dataset_id) => state
            .simulation
            .retained
            .runs
            .iter()
            .position(|run| run.dataset_id == *dataset_id)
            .is_some_and(|index| {
                state.simulation.view.active_run_idx == Some(index)
                    || state.simulation.select_run(index)
            }),
        WorkspaceDocumentId::NetlistGenerated(document_id) => {
            let available = state
                .ui
                .netlist
                .generated_document
                .as_ref()
                .is_some_and(|document| document.id() == *document_id);
            available
                && ((state.ui.netlist.active_document
                    == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
                    && state.ui.netlist.active_dependency_identity.is_none())
                    || crate::workbench::documents::netlist_document::open_generated_primary(state))
        }
        WorkspaceDocumentId::NetlistOwned(document_id) => {
            let available = state
                .ui
                .netlist
                .owned_document
                .as_ref()
                .is_some_and(|document| document.id() == *document_id);
            available && crate::workbench::documents::netlist_document::open_owned_primary(state)
        }
        WorkspaceDocumentId::NetlistDependency {
            root,
            logical_identity,
        } => {
            use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
            let root_kind = if state
                .ui
                .netlist
                .generated_document
                .as_ref()
                .is_some_and(|document| document.id() == *root)
            {
                Some(ActiveNetlistDocument::Generated)
            } else if state
                .ui
                .netlist
                .owned_document
                .as_ref()
                .is_some_and(|document| document.id() == *root)
            {
                Some(ActiveNetlistDocument::OwnedSource)
            } else {
                None
            };
            root_kind.is_some_and(|root_kind| {
                crate::workbench::documents::netlist_document::open_netlist_dependency_from_root(
                    state,
                    root_kind,
                    logical_identity,
                )
                .is_ok()
            })
        }
        WorkspaceDocumentId::NetlistComparison => {
            crate::workbench::documents::netlist_document::open_netlist_comparison(state)
        }
        WorkspaceDocumentId::NetlistRunSnapshot => {
            crate::workbench::documents::netlist_document::open_run_deck_snapshot(state)
        }
        WorkspaceDocumentId::Project
        | WorkspaceDocumentId::SimulationPlan
        | WorkspaceDocumentId::Verification
        | WorkspaceDocumentId::Models
        | WorkspaceDocumentId::NetlistSource => true,
    };
    if activated {
        state.workbench.documents.activate(document.clone());
        let window = state.workbench.window_session.current();
        let _ = state
            .workbench
            .window_session
            .set_active_document(window, document.clone());
    }
    activated
}

fn close_document(
    state: &mut AppState,
    document: &WorkspaceDocumentId,
    documents: &[WorkspaceDocument],
) -> bool {
    let Some(closed_index) = documents
        .iter()
        .position(|candidate| candidate.id == *document)
    else {
        return false;
    };
    if !document_is_closable(state, document) || document.workspace() != state.workbench.workspace {
        return false;
    }
    if !state.commit_pending_inspector_edit() {
        return false;
    }
    let was_active = authoritative_active_document(state, documents).as_ref() == Some(document);
    let fallback = documents
        .get(closed_index + 1)
        .or_else(|| documents.get(closed_index.saturating_sub(1)))
        .map(|document| document.id.clone());

    let closed = match document {
        WorkspaceDocumentId::CellView(_) => {
            if !activate_document(state, document) {
                return false;
            }
            crate::workbench::workflows::project_workflow::close_active_document(state)
        }
        _ => {
            if matches!(document, WorkspaceDocumentId::NetlistDependency { .. }) {
                state.workbench.netlist_open_documents.remove(document);
            }
            state.workbench.documents.close(document);
            true
        }
    };
    if closed
        && (was_active || matches!(document, WorkspaceDocumentId::CellView(_)))
        && let Some(fallback) = fallback
    {
        let source = state.workbench.window_session.owner(document);
        let primary = state.workbench.window_session.primary();
        if source != primary {
            let _ = state
                .workbench
                .window_session
                .move_document(document.clone(), source, primary);
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
fn tab_caption<'a>(label: &'a str, short: Option<&'a str>, natural: f32, width: f32) -> &'a str {
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
    let painted_label =
        ellipsize_document_label(ui.painter(), shown, &font, (suffix_left - label_x).max(0.0));
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
        let center = egui::pos2(suffix_x + TAB_DIRTY_SIZE * 0.5, rect.center().y);
        ui.painter()
            .circle_filled(center, TAB_DIRTY_SIZE * 0.5, t.color.warn);
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

fn ellipsize_document_label(
    painter: &egui::Painter,
    value: &str,
    font: &egui::FontId,
    maximum_width: f32,
) -> String {
    if maximum_width <= 0.0 {
        return String::new();
    }
    if painter
        .layout_no_wrap(value.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return value.to_owned();
    }
    let ellipsis = "\u{2026}";
    if painter
        .layout_no_wrap(ellipsis.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        > maximum_width
    {
        return String::new();
    }
    let characters = value.chars().collect::<Vec<_>>();
    let mut low = 0_usize;
    let mut high = characters.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = characters[..mid]
            .iter()
            .copied()
            .chain(std::iter::once('\u{2026}'))
            .collect::<String>();
        if painter
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x
            <= maximum_width
        {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('\u{2026}'))
        .collect()
}

fn suffix_width(dirty: bool, closable: bool) -> f32 {
    match (dirty, closable) {
        (false, false) => 0.0,
        (true, false) => TAB_CONTENT_GAP + TAB_DIRTY_SIZE,
        (false, true) => TAB_CONTENT_GAP + 2.0 + TAB_CLOSE_SIZE,
        (true, true) => TAB_CONTENT_GAP + TAB_DIRTY_SIZE + TAB_CONTENT_GAP + 2.0 + TAB_CLOSE_SIZE,
    }
}

const fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        // A Verilog-A module is edited in its own tab beside the schematic it
        // describes; borrowing the netlist mark made it read as a deck.
        ViewType::VerilogA => WorkbenchIcon::VerilogA,
        ViewType::Spice | ViewType::Verilog => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ObjectRevision;
    use crate::state::{
        DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput,
        NetlistDocument, NetlistDocumentId, SimulationRun, SourceLocator,
    };

    fn multi_document_netlist_state() -> AppState {
        const ROOT: &str = "tabs\n.include \"models/resistor.inc\"\n.end\n";
        const INCLUDE: &str = ".param r=1k\n";
        let locator = SourceLocator::try_new("models/resistor.inc", "resistor.inc").unwrap();
        let dependency =
            DependencyMetadata::unresolved_direct_to(0, "models/resistor.inc", locator)
                .unwrap()
                .resolve_utf8(INCLUDE.as_bytes().to_vec())
                .unwrap();
        let provenance = GeneratedProvenance::try_new(
            "document-bar-test",
            GenerationInput::new(
                ObjectRevision::INITIAL,
                crate::state::content_digest("document-bar-input"),
            ),
        )
        .unwrap();
        let artifact = GeneratedArtifact::try_from_utf8(
            provenance,
            ROOT.as_bytes().to_vec(),
            vec![dependency],
            Vec::new(),
        )
        .unwrap();
        let generated =
            NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).unwrap();
        let owned = generated
            .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
            .unwrap();
        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Netlist;
        state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        state.ui.netlist.generated_source = ROOT.to_owned();
        state.ui.netlist.generated_document = Some(generated);
        state.ui.netlist.owned_document = Some(owned.clone());
        state.workspace.content.netlist_source = Some(ROOT.to_owned());
        state.workspace.content.netlist_document = Some(owned);
        state.simulation.source.netlist_content = ROOT.to_owned();
        state
    }

    /// Every Design tab is a cell view, so the icon is what tells a module
    /// from the schematic and the symbol beside it. A Verilog-A module wore
    /// the netlist mark, which named a deck that is not what the tab opens.
    #[test]
    fn a_veriloga_tab_wears_its_own_icon() {
        let icon = icon_for_view(ViewType::VerilogA);
        assert_eq!(icon, WorkbenchIcon::VerilogA);
        for other in [
            ViewType::Schematic,
            ViewType::Symbol,
            ViewType::Spice,
            ViewType::Verilog,
        ] {
            assert_ne!(icon, icon_for_view(other), "{other:?} shares the mark");
        }
    }

    #[test]
    fn design_strip_retains_the_single_active_cell_view() {
        assert!(!document_strip_visible(Workspace::Design, 0));
        assert!(document_strip_visible(Workspace::Design, 1));
        assert!(document_strip_visible(Workspace::Design, 2));
    }

    #[test]
    fn models_strip_retains_its_single_pinned_root_document() {
        assert!(!document_strip_visible(Workspace::Models, 0));
        assert!(document_strip_visible(Workspace::Models, 1));

        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Models;
        let descriptors = document_descriptors(&state);
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].id, WorkspaceDocumentId::Models);
        assert_eq!(descriptors[0].label, "Models & PDKs");
        assert!(descriptors[0].open);
        assert!(!descriptors[0].closable);
    }

    #[test]
    fn non_design_strips_remain_reserved_for_multiple_open_documents() {
        assert!(!document_strip_visible(Workspace::Project, 0));
        assert!(!document_strip_visible(Workspace::Project, 1));
        assert!(document_strip_visible(Workspace::Project, 2));
    }

    #[test]
    fn netlist_roots_and_dependencies_are_independent_stable_tabs() {
        let mut state = multi_document_netlist_state();
        assert_eq!(
            document_descriptors(&state).len(),
            2,
            "resolved closures are available in the navigator, not opened as thousands of tabs"
        );
        crate::workbench::documents::netlist_document::open_netlist_dependency_from_root(
            &mut state,
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated,
            "models/resistor.inc",
        )
        .unwrap();
        crate::workbench::documents::netlist_document::open_netlist_dependency_from_root(
            &mut state,
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource,
            "models/resistor.inc",
        )
        .unwrap();
        crate::workbench::documents::netlist_document::open_generated_primary(&mut state);
        let descriptors = document_descriptors(&state);
        assert_eq!(descriptors.len(), 4);
        assert!(matches!(
            descriptors[0].id,
            WorkspaceDocumentId::NetlistGenerated(_)
        ));
        assert!(matches!(
            descriptors[1].id,
            WorkspaceDocumentId::NetlistDependency { .. }
        ));
        assert!(matches!(
            descriptors[2].id,
            WorkspaceDocumentId::NetlistOwned(_)
        ));
        assert!(matches!(
            descriptors[3].id,
            WorkspaceDocumentId::NetlistDependency { .. }
        ));
        assert!(descriptors[0].active);

        let owned_dependency = descriptors[3].id.clone();
        assert!(activate_document(&mut state, &owned_dependency));
        assert_eq!(
            state.ui.netlist.active_document,
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
        );
        assert_eq!(
            state.ui.netlist.active_dependency_identity.as_deref(),
            Some("models/resistor.inc")
        );
        assert_eq!(active_document_id(&state), Some(owned_dependency.clone()));

        let generated_dependency = descriptors[1].id.clone();
        let documents = netlist_workspace_documents(&state);
        assert!(close_document(
            &mut state,
            &generated_dependency,
            &documents
        ));
        assert_eq!(
            active_document_id(&state),
            Some(owned_dependency),
            "closing an inactive include must not switch the active editor"
        );
    }

    #[test]
    fn registry_scopes_active_documents_by_workspace() {
        let mut registry = super::super::super::state::WorkspaceDocumentRegistry::default();
        let dataset = crate::product::DatasetId::new();

        registry.activate(WorkspaceDocumentId::SimulationPlan);
        registry.activate(WorkspaceDocumentId::ResultDataset(dataset));

        assert_eq!(
            registry.active(Workspace::Simulate),
            Some(&WorkspaceDocumentId::SimulationPlan)
        );
        assert_eq!(
            registry.active(Workspace::Results),
            Some(&WorkspaceDocumentId::ResultDataset(dataset))
        );
        assert!(registry.active(Workspace::Design).is_none());
    }

    #[test]
    fn activating_a_result_document_does_not_force_design_workspace() {
        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Results;
        let run = SimulationRun::new(7);
        let dataset = run.dataset_id;
        state.simulation.retained.runs.push(run);

        assert!(activate_document(
            &mut state,
            &WorkspaceDocumentId::ResultDataset(dataset)
        ));
        assert_eq!(state.workbench.workspace, Workspace::Results);
        assert_eq!(state.simulation.view.active_run_idx, Some(0));
    }

    /// The document bar re-activates the open document every frame. Doing so
    /// must be inert for the dataset that is already active: re-selecting its
    /// run resynchronizes waveforms, and the resulting data-version change
    /// retires cursors, the selected trace, the active pane, and pinned
    /// readouts — which would make those states impossible to hold at all.
    #[test]
    fn reactivating_the_active_dataset_disturbs_no_result_state() {
        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Results;
        let mut first = SimulationRun::new(1);
        first.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "TRAN",
        ));
        first.add_analysis(crate::state::AnalysisResult::new(
            2,
            crate::state::AnalysisType::Ac,
            "AC",
        ));
        let dataset = first.dataset_id;
        state.simulation.retained.runs = vec![first].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(1));
        let version = state.simulation.view.data_version;

        for _ in 0..3 {
            assert!(activate_document_by_id(
                &mut state,
                &WorkspaceDocumentId::ResultDataset(dataset)
            ));
        }

        assert_eq!(
            state.simulation.view.data_version, version,
            "re-activating the active dataset must not advance the data version"
        );
        assert_eq!(
            state.simulation.view.active_analysis_idx,
            Some(1),
            "re-activation must not snap the analysis selection back to the first"
        );
    }

    #[test]
    fn closing_a_result_tab_keeps_dataset_and_selects_adjacent_document() {
        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Results;
        let first = SimulationRun::new(1);
        let first_dataset = first.dataset_id;
        let second = SimulationRun::new(2);
        let second_dataset = second.dataset_id;
        state.simulation.retained.runs = vec![first, second].into();
        assert!(state.simulation.select_run(1));
        let documents = visible_documents(&state);

        assert!(close_document(
            &mut state,
            &WorkspaceDocumentId::ResultDataset(second_dataset),
            &documents
        ));
        assert_eq!(
            state.simulation.retained.runs.len(),
            2,
            "closing never deletes result data"
        );
        assert_eq!(state.simulation.view.active_run_idx, Some(0));
        assert_eq!(
            state.simulation.active_run().unwrap().dataset_id,
            first_dataset
        );
        assert!(
            state
                .workbench
                .documents
                .is_closed(&WorkspaceDocumentId::ResultDataset(second_dataset))
        );
        assert_eq!(visible_documents(&state).len(), 1);
    }

    #[test]
    fn document_cycle_and_bulk_close_are_presentation_only() {
        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Results;
        let first = SimulationRun::new(1);
        let first_dataset = first.dataset_id;
        let second = SimulationRun::new(2);
        let second_dataset = second.dataset_id;
        let third = SimulationRun::new(3);
        let third_dataset = third.dataset_id;
        state.simulation.retained.runs = vec![first, second, third].into();
        assert!(state.simulation.select_run(0));

        assert!(cycle_document(&mut state, false));
        assert_eq!(
            state.simulation.active_run().map(|run| run.dataset_id),
            Some(second_dataset)
        );
        assert_eq!(close_other_documents(&mut state), 1);
        assert!(
            state
                .workbench
                .documents
                .is_closed(&WorkspaceDocumentId::ResultDataset(third_dataset))
        );
        assert_eq!(state.simulation.retained.runs.len(), 3);

        assert_eq!(close_all_documents(&mut state), 1);
        assert_eq!(
            state.simulation.active_run().map(|run| run.dataset_id),
            Some(first_dataset)
        );
        assert!(
            state
                .workbench
                .documents
                .is_closed(&WorkspaceDocumentId::ResultDataset(second_dataset))
        );
        assert_eq!(state.simulation.retained.runs.len(), 3);
    }

    #[test]
    fn document_tab_geometry_matches_the_mockup() {
        assert_eq!(TAB_MAX_WIDTH, 190.0);
        assert_eq!(TAB_PADDING_X, 10.0);
        assert_eq!(TAB_ICON_SIZE, 16.0);
        assert_eq!(TAB_CONTENT_GAP, 7.0);
        assert_eq!(TAB_CLOSE_SIZE, 16.0);
        assert_eq!(TAB_DIRTY_SIZE, 6.0);
        assert_eq!(tokens::FS_1, 12.0);
    }

    #[test]
    fn dirty_and_close_suffixes_never_overlap() {
        assert_eq!(suffix_width(false, false), 0.0);
        assert_eq!(suffix_width(true, false), 13.0);
        assert_eq!(suffix_width(false, true), 25.0);
        assert_eq!(suffix_width(true, true), 38.0);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn long_document_labels_are_measured_and_ellipsized() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut fitted = String::new();
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = theme::sans(tokens::FS_1, FontWeight::Regular);
                fitted = ellipsize_document_label(
                    ui.painter(),
                    "precision_sensor_front_end · schematic",
                    &font,
                    72.0,
                );
                let width = ui
                    .painter()
                    .layout_no_wrap(fitted.clone(), font, egui::Color32::WHITE)
                    .size()
                    .x;
                assert!(width <= 72.0);
            });
        });
        assert!(fitted.ends_with('\u{2026}'));
        assert_ne!(fitted, "precision_sensor_front_end · schematic");
    }

    /// The Simulation Studio is one document, whatever the plan holds.
    ///
    /// It used to open a "<name> \u{b7} setup" tab per analysis instance, and
    /// activating one only selected that instance on the Analyses page --
    /// which the plan's analysis list on the left does anyway. So a plan with
    /// three analyses showed four tabs that said nothing the list did not, and
    /// the strip is gone: one document, and `document_strip_visible` hides a
    /// lone non-Design document.
    #[test]
    fn the_simulate_workspace_is_one_document_whatever_the_plan_holds() {
        use crate::simulation::plan::AnalysisKind;

        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Simulate;
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("a default state owns a stable analysis plan");
        plan.insert(AnalysisKind::Transient)
            .expect("a plan takes a second transient");
        plan.insert(AnalysisKind::Ac)
            .expect("a plan takes a small-signal sweep");
        assert_eq!(
            plan.instances().len(),
            3,
            "the fixture holds the instances the strip used to tab"
        );

        let documents = available_documents_for_workspace(&state, Workspace::Simulate);
        assert_eq!(
            documents
                .iter()
                .map(|document| document.label.as_str())
                .collect::<Vec<_>>(),
            ["Simulation plan"]
        );
        assert_eq!(documents[0].id, WorkspaceDocumentId::SimulationPlan);
        assert!(
            !document_strip_visible(Workspace::Simulate, documents.len()),
            "one document paints no strip"
        );
    }

    /// A Design session with the top schematic and `extra` further schematics
    /// open, then a Verilog-A module, which is the active document.
    #[cfg(not(target_arch = "wasm32"))]
    fn design_session(extra: usize) -> RSpiceApp {
        use crate::state::CellViewRef;

        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.activate(Workspace::Design);
        let library = app.state.workspace.content.project.root_library.clone();
        for index in 0..extra {
            app.state.workspace.open_view(
                CellViewRef::new(&library, format!("stage_{index}"), "schematic"),
                ViewType::Schematic,
            );
        }
        app.state.workspace.open_view(
            CellViewRef::new(&library, "testdev", "veriloga"),
            ViewType::VerilogA,
        );
        app
    }

    /// Draw the strip alone across a 1440 pt window, as `frame::show` does.
    #[cfg(not(target_arch = "wasm32"))]
    fn strip_frame(
        ctx: &egui::Context,
        app: &mut RSpiceApp,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let size = egui::vec2(1440.0, 900.0);
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                events,
                ..Default::default()
            },
            |ui| {
                let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
                show(ui, app, layout);
            },
        )
    }

    /// A press on a tab opens its document and leaves keyboard focus where it
    /// was. Taking focus on a press painted the keyboard focus ring around
    /// every tab the pointer chose.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_pressed_tab_activates_without_taking_keyboard_focus() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = design_session(0);
        let top = app.state.workspace.content.open_views[0].reference.clone();
        assert_ne!(app.state.workspace.content.active_view, top);
        strip_frame(&ctx, &mut app, Vec::new());

        // The top schematic is the first tab, at the strip's left edge.
        let at = egui::pos2(40.0, 15.0);
        strip_frame(
            &ctx,
            &mut app,
            vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        strip_frame(
            &ctx,
            &mut app,
            vec![egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        strip_frame(&ctx, &mut app, Vec::new());

        assert_eq!(app.state.workspace.content.active_view, top);
        assert_eq!(ctx.memory(|memory| memory.focused()), None);
    }

    /// The active tab is scrolled into view when the strip holds more tabs
    /// than it can show. With the overflow menu gone, a document activated
    /// off the strip's edge would otherwise have no visible tab.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn an_active_tab_past_the_strips_edge_is_scrolled_into_view() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = design_session(14);

        let nodes = settled_strip(&ctx, &mut app);
        let active = nodes
            .iter()
            .find(|node| {
                node.role() == egui::accesskit::Role::Tab && node.is_selected() == Some(true)
            })
            .and_then(egui::accesskit::Node::bounds)
            .expect("the active tab is in the tree");
        let button = hidden_button(&nodes)
            .and_then(egui::accesskit::Node::bounds)
            .expect("tabs are out of view, so the strip lists them");
        assert!(
            active.x0 >= 0.0 && active.x1 <= button.x0 + 0.5,
            "the active tab spans {}..{}, outside the tabs' track 0..{}",
            active.x0,
            active.x1,
            button.x0
        );
    }

    /// Frame the strip until its scroll has settled, and return what the last
    /// frame named for a screen reader.
    #[cfg(not(target_arch = "wasm32"))]
    fn settled_strip(ctx: &egui::Context, app: &mut RSpiceApp) -> Vec<egui::accesskit::Node> {
        let mut output = strip_frame(ctx, app, Vec::new());
        // The scroll animates; give it the frames it takes.
        for _ in 0..60 {
            output = strip_frame(ctx, app, Vec::new());
        }
        output
            .platform_output
            .accesskit_update
            .expect("accessibility tree")
            .nodes
            .into_iter()
            .map(|(_, node)| node)
            .collect()
    }

    /// What a screen reader calls `node`: a label, or a plain label's text.
    #[cfg(not(target_arch = "wasm32"))]
    fn name(node: &egui::accesskit::Node) -> Option<&str> {
        node.label().or_else(|| node.value())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn hidden_button(nodes: &[egui::accesskit::Node]) -> Option<&egui::accesskit::Node> {
        nodes.iter().find(|node| {
            name(node).is_some_and(|label| {
                label.ends_with(" more open documents") || label == "1 more open document"
            })
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn tab_nodes(nodes: &[egui::accesskit::Node]) -> Vec<&egui::accesskit::Node> {
        nodes
            .iter()
            .filter(|node| node.role() == egui::accesskit::Role::Tab)
            .collect()
    }

    #[test]
    fn a_squeezed_cell_view_tab_shows_its_cell_before_cutting_the_name() {
        let label = "stage_10 · schematic";
        assert_eq!(tab_caption(label, Some("stage_10"), 170.0, 170.0), label);
        assert_eq!(
            tab_caption(label, Some("stage_10"), 170.0, 140.0),
            "stage_10"
        );
        assert_eq!(
            tab_caption("Generated netlist", None, 170.0, 140.0),
            "Generated netlist",
            "a document with no short form is cut as it is"
        );

        let document = WorkspaceDocument {
            id: WorkspaceDocumentId::CellView(crate::state::CellViewRef::new(
                "lib",
                "stage_10",
                "schematic",
            )),
            label: label.to_owned(),
            icon: WorkbenchIcon::Design,
            dirty: false,
        };
        assert_eq!(document.short_label(), Some("stage_10"));
    }

    #[test]
    fn tabs_that_fit_keep_their_own_widths() {
        assert_eq!(shared_width_cap(&[100.0, 120.0], 400.0), None);
        assert_eq!(
            StripFit::resolve(&[100.0, 120.0], 400.0, 40.0).tabs_width,
            400.0
        );
    }

    #[test]
    fn tabs_that_do_not_fit_share_the_strip_and_the_narrow_keep_theirs() {
        assert_eq!(shared_width_cap(&[190.0, 190.0, 190.0], 450.0), Some(150.0));
        // The narrow tab keeps its 60; the other two split what is left.
        assert_eq!(shared_width_cap(&[60.0, 190.0, 190.0], 360.0), Some(150.0));
    }

    #[test]
    fn tabs_never_shrink_past_the_floor_and_the_strip_scrolls_instead() {
        let natural = [190.0; 10];
        assert_eq!(shared_width_cap(&natural, 600.0), Some(TAB_MIN_WIDTH));
        let fit = StripFit::resolve(&natural, 1000.0, 40.0);
        assert!(fit.overflowing);
        assert_eq!(
            fit.tabs_width, 960.0,
            "the hidden-tabs button takes its room"
        );
        assert_eq!(fit.cap, Some(TAB_MIN_WIDTH));

        let fit = StripFit::resolve(&natural[..6], 1000.0, 40.0);
        assert!(!fit.overflowing, "six tabs fit at 166 pt each");
        assert_eq!(fit.tabs_width, 1000.0, "no button, so no room given up");
    }

    /// Ten tabs too wide for the window together shrink until they fit, so
    /// every one stays in view and nothing is offered as hidden.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn tabs_shrink_to_stay_in_view_before_the_strip_scrolls() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = design_session(8);

        let nodes = settled_strip(&ctx, &mut app);
        let tabs = tab_nodes(&nodes);
        assert_eq!(tabs.len(), 10);
        let widest = tabs
            .iter()
            .filter_map(|tab| tab.bounds())
            .map(|bounds| bounds.x1 - bounds.x0)
            .fold(0.0, f64::max);
        assert!(
            widest < f64::from(TAB_MAX_WIDTH) - 1.0,
            "the tabs shrank: the widest is {widest} pt"
        );
        for tab in &tabs {
            let bounds = tab.bounds().expect("a tab has bounds");
            assert!(
                bounds.x0 >= 0.0 && bounds.x1 <= 1440.5,
                "{:?} spans {}..{}",
                tab.label(),
                bounds.x0,
                bounds.x1
            );
        }
        assert!(hidden_button(&nodes).is_none(), "nothing is hidden");
    }

    /// Renders for review, cropped to the chrome above the document: a strip
    /// whose tabs fit, one whose tabs shrank to fit, and one that overflows,
    /// all between the docks of a 1440 x 900 window.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_the_document_strip_for_review() {
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        for (stem, extra) in [
            ("document-strip-fits", 0),
            ("document-strip-shrunk", 3),
            ("document-strip-overflowing", 14),
        ] {
            let mut app = design_session(extra);
            let canvas = crate::ui::raster::render(egui::vec2(1440.0, 900.0), |ui, _| {
                crate::workbench::frame::show(ui, &mut app);
            });
            std::fs::write(directory.join(format!("{stem}.png")), canvas.png(140))
                .expect("write the render");
        }
    }

    /// Past the floor, the strip scrolls and "+N" counts exactly the tabs out
    /// of view; choosing one from its menu opens that document.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_hidden_tabs_button_lists_the_tabs_out_of_view_and_opens_one() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = design_session(14);
        let top = app.state.workspace.content.open_views[0].reference.clone();

        let nodes = settled_strip(&ctx, &mut app);
        let button = hidden_button(&nodes).expect("sixteen tabs overflow 1440 pt");
        let track_end = button.bounds().expect("the button has bounds").x0;
        let out_of_view = tab_nodes(&nodes)
            .into_iter()
            .filter(|tab| {
                let bounds = tab.bounds().expect("a tab has bounds");
                bounds.x0 < -0.5 || bounds.x1 > track_end + 0.5
            })
            .count();
        assert!(out_of_view > 0);
        assert_eq!(
            name(button),
            Some(format!("{out_of_view} more open documents").as_str())
        );

        let press = |ctx: &egui::Context, app: &mut RSpiceApp, at: egui::Pos2| {
            strip_frame(
                ctx,
                app,
                vec![
                    egui::Event::PointerMoved(at),
                    egui::Event::PointerButton {
                        pos: at,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
            strip_frame(
                ctx,
                app,
                vec![egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                }],
            )
        };
        let bounds = button.bounds().expect("the button has bounds");
        let at = egui::pos2(
            ((bounds.x0 + bounds.x1) * 0.5) as f32,
            ((bounds.y0 + bounds.y1) * 0.5) as f32,
        );
        press(&ctx, &mut app, at);
        let row = strip_frame(&ctx, &mut app, Vec::new())
            .platform_output
            .accesskit_update
            .expect("accessibility tree")
            .nodes
            .into_iter()
            .map(|(_, node)| node)
            .find(|node| {
                node.role() != egui::accesskit::Role::Tab && name(node) == Some("top · schematic")
            })
            .expect("the scrolled-out first tab is in the menu");
        let bounds = row.bounds().expect("the menu row has bounds");
        let at = egui::pos2(
            ((bounds.x0 + bounds.x1) * 0.5) as f32,
            ((bounds.y0 + bounds.y1) * 0.5) as f32,
        );
        press(&ctx, &mut app, at);
        assert_eq!(app.state.workspace.content.active_view, top);

        let nodes = settled_strip(&ctx, &mut app);
        let first = tab_nodes(&nodes)
            .into_iter()
            .find(|tab| name(tab) == Some("top · schematic"))
            .and_then(egui::accesskit::Node::bounds)
            .expect("the opened document has a tab");
        assert!(
            first.x0 >= -0.5,
            "the opened document's tab scrolled into view"
        );
    }

    /// A session whose Studio tab was a retired setup document opens on the
    /// plan.
    ///
    /// The registry is where an activated document identity was written, so a
    /// reader upgrading across the deletion has one in their session file.
    /// `ron` refuses an unknown enum variant outright and
    /// `restore_eframe_session` discards the whole AppState on any error, so
    /// the retired identity is read and dropped rather than refused, and the
    /// active document falls back to the one document the Studio now has.
    #[test]
    fn a_session_naming_the_retired_setup_document_opens_on_the_plan() {
        const SESSION: &str = r#"(
            active: {
                Simulate: AnalysisSetup("9f1b6f6c-0f1a-4f2e-9c6d-2a1b3c4d5e6f"),
                Models: Models,
            },
            closed: [AnalysisSetup("9f1b6f6c-0f1a-4f2e-9c6d-2a1b3c4d5e6f")],
        )"#;

        let registry: super::super::super::state::WorkspaceDocumentRegistry =
            ron::from_str(SESSION).expect("a retired identity costs the reader nothing");
        assert!(
            registry.active(Workspace::Simulate).is_none(),
            "the retired identity is dropped, not restored"
        );
        assert_eq!(
            registry.active(Workspace::Models),
            Some(&WorkspaceDocumentId::Models),
            "its neighbours in the same session survive it"
        );

        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Simulate;
        state.workbench.documents = registry;
        assert_eq!(
            active_document_id(&state),
            Some(WorkspaceDocumentId::SimulationPlan)
        );
    }
}

//! Workspace-scoped document tabs. Stable project identities drive every tab;
//! the shell never fabricates datasets, source files, or verification records.

use egui::containers::menu::MenuButton;
use egui::{Frame, Layout, Panel, Sense, Ui, Vec2};

use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, RSpiceApp};

use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{Workspace, WorkspaceDocumentId};

const TAB_MAX_WIDTH: f32 = 190.0;
const TAB_PADDING_X: f32 = 10.0;
const TAB_ICON_SIZE: f32 = 16.0;
const TAB_CONTENT_GAP: f32 = 7.0;
const TAB_CLOSE_SIZE: f32 = 16.0;
const TAB_DIRTY_SIZE: f32 = 6.0;
const TAB_ACTIVE_EDGE: f32 = 2.0;
const OVERFLOW_WIDTH: f32 = 31.0;

pub fn show(root: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    reconcile_document_registry(&mut app.state);
    let documents = visible_documents(&app.state);
    if !document_strip_visible(app.state.workbench.workspace, documents.len()) {
        return;
    }

    let t = Tokens::get(ctx);
    Panel::top("workbench.document_bar")
        .exact_size(layout.document_bar_height)
        .frame(Frame::new().fill(t.color.bg_app))
        .show_separator_line(false)
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let rect = ui.max_rect();
            let overflow_width = if t.metrics.ctl_h >= 44.0 {
                44.0
            } else {
                OVERFLOW_WIDTH
            };
            ui.painter().hline(
                rect.x_range(),
                (rect.bottom() - 0.5).max(rect.top()),
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.horizontal(|ui| {
                let tabs_width = (ui.available_width() - overflow_width).max(1.0);
                ui.allocate_ui_with_layout(
                    Vec2::new(tabs_width, layout.document_bar_height),
                    Layout::left_to_right(egui::Align::Center),
                    |ui| {
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
                                        layout.document_bar_height,
                                    );
                                });
                                ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
                                    node.set_role(egui::accesskit::Role::TabList);
                                    node.set_label("Open documents");
                                });
                            });
                    },
                );
                document_overflow(
                    ui,
                    &mut app.state,
                    &documents,
                    layout.document_bar_height,
                    overflow_width,
                );
            });
        });
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
            .open_views
            .iter()
            .map(|document| WorkspaceDocument {
                id: WorkspaceDocumentId::CellView(document.reference.clone()),
                label: format!("{} · {}", document.reference.cell, document.reference.view),
                icon: icon_for_view(document.view_type),
                dirty: document.dirty,
            })
            .collect(),
        Workspace::Simulate => {
            let mut documents = vec![WorkspaceDocument {
                id: WorkspaceDocumentId::SimulationPlan,
                label: "Simulation plan".to_owned(),
                icon: WorkbenchIcon::Simulate,
                dirty: false,
            }];
            if let Ok(plan) = state.sim_setup.stable_analysis_plan() {
                documents.extend(plan.instances().iter().map(|instance| WorkspaceDocument {
                    id: WorkspaceDocumentId::AnalysisSetup(instance.id()),
                    label: format!(
                        "{} · setup",
                        instance.kind().stable_id().to_ascii_uppercase()
                    ),
                    icon: WorkbenchIcon::Sliders,
                    dirty: false,
                }));
            }
            documents
        }
        Workspace::Results => state
            .simulation
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
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .filter(|label| !label.trim().is_empty())
            .unwrap_or_else(|| "Project netlist".to_owned());
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistOwned(owned.id()),
            label: owned_label,
            icon: WorkbenchIcon::Netlist,
            dirty: state.workspace.netlist_source_dirty,
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
                dirty: project_owned && state.workspace.netlist_source_dirty,
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
/// decides which. Only the netlist page has a dirty bit to report: Verilog-A
/// and Automation sources are project-owned bundles whose dirtiness belongs to
/// the bundle, not to this tab.
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
            CodeWorkspacePage::VerilogA => WorkbenchIcon::Code,
            CodeWorkspacePage::Automation => WorkbenchIcon::Terminal,
        },
        dirty: page == CodeWorkspacePage::Netlist && state.workspace.netlist_source_dirty,
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
            state.workspace.active_view.clone(),
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
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    let document = match root {
        ActiveNetlistDocument::Generated => netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff => None,
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
            ActiveNetlistDocument::GeneratedDiff => return None,
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

fn document_tabs(
    ui: &mut egui::Ui,
    state: &mut AppState,
    documents: &[WorkspaceDocument],
    height: f32,
) {
    let active = authoritative_active_document(state, documents);
    let mut open = None;
    let mut close = None;
    let mut focus_index = None;
    let mut tab_ids = Vec::with_capacity(documents.len());

    for (index, document) in documents.iter().enumerate() {
        let selected = active.as_ref() == Some(&document.id);
        let tab = ui
            .push_id(("document", &document.id), |ui| {
                document_tab(
                    ui,
                    DocumentTabPresentation {
                        icon: document.icon,
                        label: &document.label,
                        selected,
                        dirty: document.dirty,
                        closable: document_is_closable(state, &document.id),
                        height,
                    },
                )
            })
            .inner;
        tab_ids.push(tab.response.id);

        if tab.close_clicked {
            close = Some((index, document.id.clone()));
        } else if tab.response.clicked() {
            focus_index = Some(index);
            if !selected {
                open = Some(document.id.clone());
            }
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
                    close = Some((index, document.id.clone()))
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
    if let Some((closed_index, document)) = close {
        let focus_after_close = if closed_index + 1 < documents.len() {
            closed_index + 1
        } else {
            closed_index.saturating_sub(1)
        };
        if close_document(state, &document, documents) {
            ui.memory_mut(|memory| memory.request_focus(tab_ids[focus_after_close]));
        }
    }
}

fn activate_document(state: &mut AppState, document: &WorkspaceDocumentId) -> bool {
    if document.workspace() != state.workbench.workspace {
        return false;
    }
    let activated = match document {
        WorkspaceDocumentId::CellView(reference) => {
            if state
                .workspace
                .open_views
                .iter()
                .any(|open| open.reference == *reference)
            {
                state.open_workspace_view(reference.clone());
                true
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
        WorkspaceDocumentId::AnalysisSetup(id) => {
            let Some(position) = state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    plan.instances()
                        .iter()
                        .position(|instance| instance.id() == *id)
                })
            else {
                return false;
            };
            state.workbench.active_analysis_instance = Some(*id);
            state.workbench.active_analysis = position;
            true
        }
        // Re-activating the dataset that is already active must not re-select
        // its run. `select_run` resynchronizes the displayed waveforms, which
        // advances the simulation data version, and every version change
        // retires cursors, the selected trace, the active pane, and pinned
        // readouts. This path runs on every frame the document bar draws, so
        // an unguarded re-selection made those states impossible to hold.
        WorkspaceDocumentId::ResultDataset(dataset_id) => state
            .simulation
            .runs
            .iter()
            .position(|run| run.dataset_id == *dataset_id)
            .is_some_and(|index| {
                state.simulation.active_run_idx == Some(index) || state.simulation.select_run(index)
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
    selected: bool,
    dirty: bool,
    closable: bool,
    height: f32,
}

fn document_tab(
    ui: &mut egui::Ui,
    presentation: DocumentTabPresentation<'_>,
) -> DocumentTabResponse {
    let DocumentTabPresentation {
        icon,
        label,
        selected,
        dirty,
        closable,
        height,
    } = presentation;
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let intrinsic_label_width = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), t.color.text)
        .size()
        .x;
    let suffix_width = suffix_width(dirty, closable);
    let width = (TAB_PADDING_X
        + TAB_ICON_SIZE
        + TAB_CONTENT_GAP
        + intrinsic_label_width
        + suffix_width
        + TAB_PADDING_X)
        .min(TAB_MAX_WIDTH);
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
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
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
        ellipsize_document_label(ui.painter(), label, &font, (suffix_left - label_x).max(0.0));
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

fn document_overflow(
    ui: &mut egui::Ui,
    state: &mut AppState,
    documents: &[WorkspaceDocument],
    height: f32,
    width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let active = authoritative_active_document(state, documents);
    let mut selected = None;
    let (response, _) = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            MenuButton::from_button(
                egui::Button::new("")
                    .frame(false)
                    .min_size(Vec2::new(width, height.max(t.metrics.ctl_h))),
            )
            .ui(ui, |ui| {
                ui.set_min_width(220.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                for document in documents {
                    if ui
                        .add_sized(
                            [220.0, if height >= 44.0 { 44.0 } else { 29.0 }],
                            egui::Button::selectable(
                                active.as_ref() == Some(&document.id),
                                &document.label,
                            ),
                        )
                        .clicked()
                    {
                        selected = Some(document.id.clone());
                        ui.close();
                    }
                }
            })
        })
        .inner;
    if let Some(document) = selected {
        activate_document(state, &document);
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Manage open documents",
        )
    });
    ui.painter().rect_filled(
        response.rect,
        0.0,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_app
        },
    );
    ui.painter().vline(
        response.rect.left(),
        response.rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), Vec2::splat(16.0)),
        if response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_text("Manage open documents");
}

const fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => WorkbenchIcon::Netlist,
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
        state.workspace.netlist_source = Some(ROOT.to_owned());
        state.workspace.netlist_document = Some(owned);
        state.simulation.netlist_content = ROOT.to_owned();
        state
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
        let analysis = crate::product::AnalysisInstanceId::new();
        let dataset = crate::product::DatasetId::new();

        registry.activate(WorkspaceDocumentId::AnalysisSetup(analysis));
        registry.activate(WorkspaceDocumentId::ResultDataset(dataset));

        assert_eq!(
            registry.active(Workspace::Simulate),
            Some(&WorkspaceDocumentId::AnalysisSetup(analysis))
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
        state.simulation.runs.push(run);

        assert!(activate_document(
            &mut state,
            &WorkspaceDocumentId::ResultDataset(dataset)
        ));
        assert_eq!(state.workbench.workspace, Workspace::Results);
        assert_eq!(state.simulation.active_run_idx, Some(0));
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
        state.simulation.runs = vec![first];
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(1));
        let version = state.simulation.data_version;

        for _ in 0..3 {
            assert!(activate_document_by_id(
                &mut state,
                &WorkspaceDocumentId::ResultDataset(dataset)
            ));
        }

        assert_eq!(
            state.simulation.data_version, version,
            "re-activating the active dataset must not advance the data version"
        );
        assert_eq!(
            state.simulation.active_analysis_idx,
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
        state.simulation.runs = vec![first, second];
        assert!(state.simulation.select_run(1));
        let documents = visible_documents(&state);

        assert!(close_document(
            &mut state,
            &WorkspaceDocumentId::ResultDataset(second_dataset),
            &documents
        ));
        assert_eq!(
            state.simulation.runs.len(),
            2,
            "closing never deletes result data"
        );
        assert_eq!(state.simulation.active_run_idx, Some(0));
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
        state.simulation.runs = vec![first, second, third];
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
        assert_eq!(state.simulation.runs.len(), 3);

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
        assert_eq!(state.simulation.runs.len(), 3);
    }

    #[test]
    fn document_tab_geometry_matches_the_mockup() {
        assert_eq!(TAB_MAX_WIDTH, 190.0);
        assert_eq!(TAB_PADDING_X, 10.0);
        assert_eq!(TAB_ICON_SIZE, 16.0);
        assert_eq!(TAB_CONTENT_GAP, 7.0);
        assert_eq!(TAB_CLOSE_SIZE, 16.0);
        assert_eq!(TAB_DIRTY_SIZE, 6.0);
        assert_eq!(OVERFLOW_WIDTH, 31.0);
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
}

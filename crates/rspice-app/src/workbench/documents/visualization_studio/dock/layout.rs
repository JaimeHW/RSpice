//! Retained worksheet source preparation and authoritative document transactions.

use super::*;
use rspice_results_ui::studio::dock::layout::{
    self as presentation, LinksHost, PageDraft, PageHost, PropertiesHost,
};

struct Host<'a>(&'a mut RSpiceApp);

pub(super) fn properties_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::properties(ui, &mut Host(app))
}
pub(super) fn reorder_panes_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    let studio = &mut app.state.workbench.visualization_studio;
    let apply = presentation::reorder(ui, studio.active_pane, &mut studio.draft_pane_order);
    if apply {
        save_pane_order(app);
    }
    apply
}
pub(super) fn link_groups_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::links(ui, &mut Host(app))
}
pub(super) fn page_editor_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::page(ui, &mut Host(app))
}

impl PropertiesHost for Host<'_> {
    fn current(&self) -> (u8, bool) {
        let app = &self.0;
        let active_document = active_project_visualization_document_id(&app.state);
        let document_policy = active_document.and_then(|document_id| {
            app.state
                .workspace
                .content
                .visualization_document(document_id)
                .map(|document| document.presentation())
        });
        let current_significant_digits = document_policy.map_or(
            app.state.workbench.visualization_studio.significant_digits,
            |policy| policy.significant_digits,
        );
        let current_phase_continuous = document_policy
            .map_or(app.state.ui.results.session.phase_continuous, |policy| {
                policy.phase_continuous
            });
        (current_significant_digits, current_phase_continuous)
    }
    fn significant_digits(&mut self) -> &mut Option<u8> {
        &mut self
            .0
            .state
            .workbench
            .visualization_studio
            .draft_significant_digits
    }
    fn phase_continuous(&mut self) -> &mut Option<bool> {
        &mut self
            .0
            .state
            .workbench
            .visualization_studio
            .draft_phase_continuous
    }
    fn save(&mut self, significant_digits: u8, phase_continuous: bool) {
        let app = &mut self.0;
        let result = save_document_properties(app, significant_digits, phase_continuous);
        report_visualization_commit(app, result);
    }
}

fn save_pane_order(app: &mut RSpiceApp) {
    let order = app
        .state
        .workbench
        .visualization_studio
        .draft_pane_order
        .clone();
    if let Some(document_id) = active_project_visualization_document_id(&app.state) {
        let project = &app.state.workspace.content;
        let edits = project
            .visualization_document(document_id)
            .map(|document| {
                document
                    .pages()
                    .iter()
                    .filter_map(|page| {
                        let mut current = document
                            .panes()
                            .iter()
                            .filter(|pane| pane.page_id == page.id)
                            .collect::<Vec<_>>();
                        current.sort_by_key(|pane| (pane.order, pane.id));
                        let current = current.iter().map(|pane| pane.id).collect::<Vec<_>>();
                        let desired = order
                            .iter()
                            .filter_map(|pane_id| {
                                document
                                    .panes()
                                    .iter()
                                    .find(|pane| {
                                        pane.id.get() == *pane_id && pane.page_id == page.id
                                    })
                                    .map(|pane| pane.id)
                            })
                            .collect::<Vec<_>>();
                        (desired != current).then_some(DocumentEdit::ReorderPagePanes {
                            page_id: page.id,
                            pane_ids: desired,
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if !edits.is_empty() {
            match transact_active_project_document(app, edits) {
                Ok(_) => reconcile_document(app),
                Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
            }
        }
        return;
    }
    let result = app.state.workbench.visualization_studio.transact(|studio| {
        studio.panes.sort_by_key(|pane| {
            order
                .iter()
                .position(|pane_id| *pane_id == pane.id)
                .unwrap_or(usize::MAX)
        });
        Ok(())
    });
    report_visualization_commit(app, result);
}

impl LinksHost for Host<'_> {
    fn prepare(&mut self) -> Option<u64> {
        let app = &mut self.0;
        let pane_id = app.state.workbench.visualization_studio.active_pane?;
        if app.state.workbench.visualization_studio.draft_link_pane != Some(pane_id) {
            let draft = app
                .state
                .workbench
                .visualization_studio
                .panes
                .iter()
                .find(|pane| pane.id == pane_id)
                .map(|pane| {
                    (
                        pane.x_link.unwrap_or_default(),
                        pane.cursor_group.unwrap_or_default(),
                    )
                });
            if let Some((x_link, cursor_group)) = draft {
                app.state.workbench.visualization_studio.draft_link_pane = Some(pane_id);
                app.state.workbench.visualization_studio.draft_x_link = x_link;
                app.state.workbench.visualization_studio.draft_cursor_group = cursor_group;
            }
        }
        Some(pane_id)
    }
    fn x_link(&mut self) -> &mut u64 {
        &mut self.0.state.workbench.visualization_studio.draft_x_link
    }
    fn cursor_group(&mut self) -> &mut u64 {
        &mut self
            .0
            .state
            .workbench
            .visualization_studio
            .draft_cursor_group
    }
    fn canonical(&self) -> bool {
        active_project_visualization_document_id(&self.0.state).is_some()
    }
    fn linked_cursors(&self) -> bool {
        self.0.state.ui.results.session.linked_cursors
    }
    fn set_cursor_links(&mut self, linked: bool) {
        set_active_project_cursor_links(self.0, linked);
    }
    fn save(&mut self, pane_id: u64) {
        save_link_groups(self.0, pane_id);
    }
}

fn save_link_groups(app: &mut RSpiceApp, pane_id: u64) {
    let x_link = app.state.workbench.visualization_studio.draft_x_link;
    let cursor_group = app.state.workbench.visualization_studio.draft_cursor_group;
    if let Some(document_id) = active_project_visualization_document_id(&app.state) {
        let result = (|| {
            let document = app
                .state
                .workspace
                .content
                .visualization_document(document_id)
                .ok_or_else(|| "The active result document is no longer retained.".to_owned())?;
            let canonical_pane = document
                .panes()
                .iter()
                .find(|pane| pane.id.get() == pane_id)
                .ok_or_else(|| "The selected project result pane no longer exists.".to_owned())?;
            let axis = document
                .axes()
                .iter()
                .find(|axis| {
                    axis.pane_id == canonical_pane.id
                        && axis.orientation
                            == crate::results::visualization_document::AxisOrientation::Horizontal
                })
                .ok_or_else(|| "The selected pane has no horizontal axis to link.".to_owned())?;
            let target_group = if x_link == 0 {
                None
            } else {
                Some(
                        document
                            .link_groups()
                            .iter()
                            .find(|group| {
                                group.id.get() == x_link
                                    && group.kind
                                        == crate::results::visualization_document::LinkKind::HorizontalViewport
                            })
                            .ok_or_else(|| {
                                format!(
                                    "Horizontal viewport group {x_link} does not exist. Use an existing group ID shown by another pane."
                                )
                            })?,
                    )
            };
            let mut edits = Vec::new();
            for group in document.link_groups().iter().filter(|group| {
                group.kind == crate::results::visualization_document::LinkKind::HorizontalViewport
                    && group.members.contains(
                        &crate::results::visualization_document::EntityRef::Axis(axis.id),
                    )
                    && target_group.is_none_or(|target| target.id != group.id)
            }) {
                let members = group
                    .members
                    .iter()
                    .copied()
                    .filter(|member| {
                        *member != crate::results::visualization_document::EntityRef::Axis(axis.id)
                    })
                    .collect::<Vec<_>>();
                if members.len() < 2 {
                    edits.push(DocumentEdit::Remove(
                        crate::results::visualization_document::EntityRef::LinkGroup(group.id),
                    ));
                } else {
                    edits.push(DocumentEdit::SetLinkMembers {
                        link_group_id: group.id,
                        members,
                    });
                }
            }
            if let Some(group) = target_group
                && !group.members.contains(
                    &crate::results::visualization_document::EntityRef::Axis(axis.id),
                )
            {
                let mut members = group.members.clone();
                members.push(crate::results::visualization_document::EntityRef::Axis(
                    axis.id,
                ));
                edits.push(DocumentEdit::SetLinkMembers {
                    link_group_id: group.id,
                    members,
                });
            }
            if edits.is_empty() {
                return Ok(());
            }
            transact_active_project_document(app, edits).map(|_| ())
        })();
        if report_visualization_commit(app, result) {
            reconcile_document(app);
        }
        return;
    }
    let result = app.state.workbench.visualization_studio.transact(|studio| {
        let pane = studio
            .panes
            .iter_mut()
            .find(|pane| pane.id == pane_id)
            .ok_or_else(|| "The selected visualization pane no longer exists".to_owned())?;
        pane.x_link = (x_link != 0).then_some(x_link);
        pane.cursor_group = (cursor_group != 0).then_some(cursor_group);
        studio.applied_link_pane = None;
        Ok(())
    });
    report_visualization_commit(app, result);
}

impl PageHost for Host<'_> {
    fn prepare(&mut self) -> Option<u64> {
        let app = &mut self.0;
        let pane_id = app.state.workbench.visualization_studio.active_pane?;
        if app.state.workbench.visualization_studio.draft_page_pane != Some(pane_id) {
            let page = app
                .state
                .workbench
                .visualization_studio
                .panes
                .iter()
                .find(|pane| pane.id == pane_id)
                .map(|pane| pane.page.clone())
                .unwrap_or_default();
            app.state.workbench.visualization_studio.draft_page_pane = Some(pane_id);
            app.state.workbench.visualization_studio.draft_page = page;
        }
        Some(pane_id)
    }
    fn draft(&mut self) -> PageDraft<'_> {
        let studio = &mut self.0.state.workbench.visualization_studio;
        PageDraft {
            page: &mut studio.draft_page,
            template: &mut studio.draft_report_template,
            freeze: &mut studio.draft_report_freeze,
        }
    }
    fn save(&mut self, pane_id: u64, page: String, template: String, freeze: bool) {
        save_report_page(self.0, pane_id, page, template, freeze);
    }
}

fn save_report_page(
    app: &mut RSpiceApp,
    pane_id: u64,
    page: String,
    template: String,
    freeze: bool,
) {
    if let Some(document_id) = active_project_visualization_document_id(&app.state) {
        let canonical_pane = app
            .state
            .workspace
            .content
            .visualization_document(document_id)
            .and_then(|document| {
                document
                    .panes()
                    .iter()
                    .find(|pane| pane.id.get() == pane_id)
                    .map(|pane| pane.id)
            });
        let result = canonical_pane
            .ok_or_else(|| "The selected project result pane no longer exists.".to_owned())
            .and_then(|pane_id| {
                transact_active_project_document(
                    app,
                    vec![DocumentEdit::AssignPaneToReportPage {
                        pane_id,
                        page_title: page.clone(),
                        template_id: template.clone(),
                        update_policy: if freeze {
                            PageUpdatePolicy::FreezeFigureRevision
                        } else {
                            PageUpdatePolicy::RefreshLinkedFigures
                        },
                    }],
                )
                .map(|_| ())
            });
        if report_visualization_commit(app, result) {
            reconcile_document(app);
        }
        return;
    }
    let result = app.state.workbench.visualization_studio.transact(|studio| {
        let studio = &mut studio.presentation;
        let pane = studio
            .panes
            .iter_mut()
            .find(|pane| pane.id == pane_id)
            .ok_or_else(|| "The selected visualization pane no longer exists".to_owned())?;
        pane.page = page;
        let policy_revision = match studio.report_page_policies.get(&pane.page) {
            Some(policy) => policy
                .revision
                .checked_add(1)
                .ok_or_else(|| "Report page revision space is exhausted".to_owned())?,
            None => 1,
        };
        studio.report_page_policies.insert(
            pane.page.clone(),
            VisualizationReportPagePolicy {
                template,
                update_policy: if freeze {
                    PageUpdatePolicy::FreezeFigureRevision
                } else {
                    PageUpdatePolicy::RefreshLinkedFigures
                },
                revision: policy_revision,
            },
        );
        Ok(())
    });
    report_visualization_commit(app, result);
}

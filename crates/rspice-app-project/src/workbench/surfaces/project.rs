//! Production Project workspace.
//!
//! The persistent section chrome hosts five focused projections of the live
//! project. Every page reads authoritative project state and dispatches real
//! workbench workflows; no mockup fixture data is admitted here.

pub mod configuration;
pub mod dependencies;
pub(crate) mod derived;
pub mod landing;
pub mod library;
pub mod library_publication;
pub mod overview;
mod page;
pub mod recovery;
mod technology;

pub use technology::{AttachableTechnology, attachable_binding, attachable_technologies};

use egui::{
    Align, Align2, Color32, Context, Key, Layout, Margin, Modifiers, Rect, ScrollArea, Sense,
    Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{ProjectTechnologyBinding, model_library::ModelLibraryManager};
use crate::ui::accessibility::plural_suffix;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, Problem, paint_problem_mark};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;

use super::super::design_system::{WorkbenchIcon, elide_text, property_card, property_row};
use super::super::framework::command_registry::ids;
use super::super::state::Workspace;
use crate::workbench::project::workspace_state::ProjectPage;
#[cfg(target_arch = "wasm32")]
use technology::poll_browser_recovery_completions;
use technology::poll_technology_change;
pub use technology::show_technology_attachment_dialog;
use technology::{open_technology_attachment_dialog, open_technology_detach_dialog};

/// The height of a page tab and of a chip beside the tabs: a dense grid's row
/// under a fine pointer, the touch target under a coarse one.
fn project_tab_height(tokens: &Tokens) -> f32 {
    if tokens.metrics.is_touch() {
        tokens.metrics.touch_target()
    } else {
        tokens.metrics.grid_row_h
    }
}

/// The unsaved mark a tab or row carries, the document bar's dot, and the
/// room it takes after the label.
const DIRTY_MARK_SIZE: f32 = 6.0;
const DIRTY_MARK_ADVANCE: f32 = 12.0;

/// Height of one ledger row.
///
/// The density token is the same one the navigator and inspector lists read,
/// so a ledger row, a tree row and a property row line up at every density
/// instead of the ledger holding a desktop-only 36 px that reads loose beside
/// them and still misses a touch target.
fn workspace_table_row_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.row_h
}

/// Width that is both allocated to this UI and actually visible through the
/// current clip. Split-pane children can otherwise inherit the unconstrained
/// central-panel width and paint beneath the Inspector dock.
fn visible_workspace_width(ui: &Ui) -> f32 {
    ui.available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .width()
        .max(1.0)
}
/// Height that is both allocated to this UI and actually visible through the
/// current clip.
///
/// A page rendered inside the surface's vertical `ScrollArea` is offered an
/// unbounded `available_height`, so a panel that wants to reach the bottom of
/// the workspace has to ask the clip how far that is.
fn visible_workspace_height(ui: &Ui) -> f32 {
    ui.available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .height()
        .max(1.0)
}

const TECHNOLOGY_DIALOG_TITLE: &str = "Attach technology";
const TECHNOLOGY_CHANGE_TITLE: &str = "Change technology";
const TECHNOLOGY_DIALOG_PRIMARY: &str = "Attach";
const TECHNOLOGY_CHANGE_PRIMARY: &str = "Change";
const TECHNOLOGY_DIALOG_CURRENT: &str = "Already attached";
const TECHNOLOGY_DETACH_TITLE: &str = "Detach technology";
const TECHNOLOGY_DETACH_PRIMARY: &str = "Detach";
/// The one line a technology change states while its checkpoint is written.
const TECHNOLOGY_CHECKPOINT_PENDING: &str = "Writing the recovery checkpoint\u{2026}";

#[cfg(target_arch = "wasm32")]
struct BrowserTechnologyCheckpointCompletion {
    project_id: String,
    result:
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary, String>,
}

#[cfg(target_arch = "wasm32")]
struct BrowserRecoveryCatalogCompletion {
    project_id: String,
    result:
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointCatalog, String>,
}

#[cfg(target_arch = "wasm32")]
struct BrowserRecoveryCopyCompletion {
    project_id: String,
    filename: String,
    result: Result<Vec<u8>, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_TECHNOLOGY_CHECKPOINT_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserTechnologyCheckpointCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    static BROWSER_RECOVERY_CATALOG_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserRecoveryCatalogCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    static BROWSER_RECOVERY_COPY_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserRecoveryCopyCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

/// Take what background work finished writing or reading for this
/// workspace before it draws, on whichever page is in front.
pub fn poll_completions(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    poll_technology_change(cx);
    #[cfg(target_arch = "wasm32")]
    {
        poll_browser_recovery_completions(ctx, cx);
        recovery::poll_browser_checkpoint_changes(cx);
        recovery::poll_browser_checkpoint_reads(cx);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = ctx;
        recovery::take_checkpoint_job_answer(cx);
        dependencies::take_model_library_read(cx);
    }
}

/// The open project's chrome: the section tabs and nothing else. The band
/// carries no page's actions and no status: each page offers its own, and
/// the status bar states the checks and the run.
pub fn chrome(cx: &mut Cx<'_, ProjectFeature>, ui: &mut Ui) {
    let tokens = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .fill(tokens.color.bg_panel)
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            // A `Frame` shrinks to its content: the band spans the page.
            ui.set_min_width(ui.available_width());
            if ui.available_width() >= project_tabs_desired_width(ui) {
                project_tabs(ui, cx);
            } else {
                // Too narrow for every tab: they scroll under a finger, with
                // no bar under them, and the page in front is kept in view.
                ScrollArea::horizontal()
                    .id_salt("workbench.project.tabs.compact")
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        project_tabs(ui, cx);
                    });
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
}

fn project_tabs_desired_width(ui: &Ui) -> f32 {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let labels = ProjectPage::ALL
        .iter()
        .map(|page| {
            ui.painter()
                .layout_no_wrap(page.label().to_owned(), font.clone(), tokens.color.text)
                .size()
                .x
                + 20.0
        })
        .sum::<f32>();
    labels + 2.0 * (ProjectPage::ALL.len().saturating_sub(1) as f32)
}

fn project_tabs(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let tab_ids = ProjectPage::ALL.map(|page| ui.id().with(("project-section-tab", page)));
    // Edits to a configuration survive leaving its page, so its tab carries
    // the unsaved mark a document tab does until they are applied.
    let configuration_dirty = cx
        .state
        .configuration
        .is_dirty(&cx.model.workspace.content.configuration_sets);
    let height = project_tab_height(&tokens);
    // The page the strip last brought into view: a strip that scrolls brings
    // the page in front into view when it changes, however it was changed,
    // and leaves the strip where a finger put it otherwise.
    let shown_id = ui.id().with("project-section-tab-shown");
    let brought = ui.data(|data| data.get_temp::<ProjectPage>(shown_id));
    let bring_into_view = brought != Some(cx.state.project_page);
    if bring_into_view {
        ui.data_mut(|data| data.insert_temp(shown_id, cx.state.project_page));
    }
    let shown = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (index, page) in ProjectPage::ALL.into_iter().enumerate() {
            let text_width = ui
                .painter()
                .layout_no_wrap(page.label().to_owned(), font.clone(), tokens.color.text)
                .size()
                .x;
            let dirty = page == ProjectPage::Configuration && configuration_dirty;
            let mark_width = if dirty { DIRTY_MARK_ADVANCE } else { 0.0 };
            let (_, rect) = ui.allocate_space(vec2(text_width + 20.0 + mark_width, height));
            let response = ui.interact(rect, tab_ids[index], Sense::click());
            let selected = cx.state.project_page == page;
            if selected && bring_into_view {
                response.scroll_to_me(None);
            }
            response.widget_info(|| {
                WidgetInfo::selected(
                    WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    page.label(),
                )
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Tab);
                node.set_label(page.label());
                node.set_selected(selected);
                if dirty {
                    node.set_description("Unsaved changes.");
                }
            });
            if response.hovered() {
                ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
            }
            ui.painter().text(
                rect.center() - vec2(mark_width * 0.5, 0.0),
                Align2::CENTER_CENTER,
                page.label(),
                font.clone(),
                if selected {
                    tokens.color.text
                } else {
                    tokens.color.text_dim
                },
            );
            if dirty {
                // A live state, not a warning: the secondary text tone.
                ui.painter().circle_filled(
                    pos2(
                        rect.center().x + text_width * 0.5 + DIRTY_MARK_ADVANCE * 0.5,
                        rect.center().y,
                    ),
                    DIRTY_MARK_SIZE * 0.5,
                    tokens.color.text_dim,
                );
            }
            if selected {
                ui.painter().rect_filled(
                    Rect::from_min_max(
                        pos2(rect.left(), rect.bottom() - 2.0),
                        pos2(rect.right(), rect.bottom()),
                    ),
                    0.0,
                    tokens.color.accent,
                );
            }
            theme::paint_focus_ring(ui, &response, rect);
            if response.clicked() {
                cx.state.project_page = page;
            }
            if response.has_focus() {
                let target = ui.input_mut(|input| {
                    if input.consume_key(Modifiers::NONE, Key::ArrowRight) {
                        Some((index + 1) % ProjectPage::ALL.len())
                    } else if input.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                        Some((index + ProjectPage::ALL.len() - 1) % ProjectPage::ALL.len())
                    } else if input.consume_key(Modifiers::NONE, Key::Home) {
                        Some(0)
                    } else if input.consume_key(Modifiers::NONE, Key::End) {
                        Some(ProjectPage::ALL.len() - 1)
                    } else {
                        None
                    }
                });
                if let Some(target) = target {
                    cx.state.project_page = ProjectPage::ALL[target];
                    ui.memory_mut(|memory| memory.request_focus(tab_ids[target]));
                }
            }
            response.on_hover_text(format!("Open project {}", page.label()));
        }
    });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabList);
        node.set_label("Project sections");
    });
}

fn workspace_empty_table_row(ui: &mut Ui, width: f32, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, workspace_table_row_height(ui)), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let text_rect = rect.shrink2(vec2(10.0, 0.0));
    let visible_text = elide_text(ui, text, &font, text_rect.width());
    ui.painter().with_clip_rect(text_rect).text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        visible_text,
        font,
        tokens.color.text_faint,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
}

fn short_identity(identity: &str) -> String {
    identity.chars().take(8).collect()
}

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

use egui::{
    Align, Align2, Color32, Context, Key, Layout, Margin, Modifiers, Rect, Response, ScrollArea,
    Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::command_catalog::CommandKey;
use crate::state::{ProjectTechnologyBinding, model_library::ModelLibraryManager};
use crate::ui::accessibility::plural_suffix;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, Problem, paint_problem_mark, problem_line, select,
};
use crate::workbench::app::dialogs::project::configuration_dialogs::show_configuration_page_in;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::health::{self, DesignSubject};

use super::super::design_system::{WorkbenchIcon, elide_text, property_card, property_row};
use super::super::framework::command_registry::ids;
use super::super::state::Workspace;
use crate::workbench::project::workspace_state::ProjectPage;
use technology::open_technology_attachment_dialog;
pub use technology::show_technology_attachment_dialog;
#[cfg(target_arch = "wasm32")]
use technology::{poll_browser_recovery_completions, poll_browser_technology_checkpoint};

const PROJECT_TAB_HEIGHT: f32 = 24.0;
/// The unsaved mark a tab or row carries, the document bar's dot, and the
/// room it takes after the label.
const DIRTY_MARK_SIZE: f32 = 6.0;
const DIRTY_MARK_ADVANCE: f32 = 12.0;
const WORKSPACE_TABLE_HEADER_HEIGHT: f32 = 37.0;

/// Height of one ledger row.
///
/// The density token is the same one the navigator and inspector lists read,
/// so a ledger row, a tree row and a property row line up at every density
/// instead of the ledger holding a desktop-only 36 px that reads loose beside
/// them and still misses a touch target.
fn workspace_table_row_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.row_h
}

/// Height of a ledger's column header, drawn tighter than its rows.
fn workspace_table_column_header_height(ui: &Ui) -> f32 {
    (workspace_table_row_height(ui) - 4.0).max(22.0)
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

const TECHNOLOGY_SURFACE_ACTION: &str = "Attach model technology\u{2026}";
const TECHNOLOGY_DIALOG_TITLE: &str = "Attach model-library technology";
const TECHNOLOGY_DIALOG_PRIMARY: &str = "Attach binding";
const TECHNOLOGY_DIALOG_CURRENT: &str = "Binding already attached";
const TECHNOLOGY_DIALOG_PENDING: &str = "Writing checkpoint\u{2026}";

#[cfg(target_arch = "wasm32")]
struct BrowserTechnologyCheckpointCompletion {
    project_id: String,
    expected_revision: u64,
    binding: ProjectTechnologyBinding,
    authority: crate::state::ProjectTechnologyChangeAuthority,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
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
    #[cfg(target_arch = "wasm32")]
    {
        poll_browser_technology_checkpoint(ctx, cx);
        poll_browser_recovery_completions(ctx, cx);
        recovery::poll_browser_checkpoint_changes(cx);
        recovery::poll_browser_checkpoint_reads(cx);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = ctx;
        recovery::take_checkpoint_job_answer(cx);
    }
}

/// What the reader asked of the chrome, raised once it has drawn: a command,
/// or what only the shell carries out.
enum ChromeRaise {
    Execute(CommandKey),
    Shell(ProjectRequest),
}

/// The open project's chrome: the section tabs and, off the Overview, the
/// revision and the health chips, in the project feature's context. The
/// band carries no page's actions: each page offers its own. The tabs and
/// the configuration chip change the page where they are drawn; what else
/// the reader asked of it is raised once it has drawn and answered before
/// the page below it draws, in the same frame.
pub fn chrome(cx: &mut Cx<'_, ProjectFeature>, ui: &mut Ui) {
    let tokens = Tokens::get(ui.ctx());
    let mut raised = None;
    // The Overview's own header states the revision and its rows each
    // subject, so the band over it carries the tabs alone.
    let chips = if cx.state.project_page == ProjectPage::Overview {
        Vec::new()
    } else {
        health_chips(ui.ctx(), cx)
    };
    let context_width = project_context_desired_width(ui, cx, &chips);
    let shown = egui::Frame::new()
        .fill(tokens.color.bg_panel)
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            let inline =
                ui.available_width() >= project_tabs_desired_width(ui) + context_width + 10.0;
            if inline {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    project_tabs(ui, cx);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        project_context_cluster(ui, cx, &chips, context_width, &mut raised);
                    });
                });
            } else {
                ScrollArea::horizontal()
                    .id_salt("workbench.project.tabs.compact")
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        project_tabs(ui, cx);
                    });
                if !chips.is_empty() {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            project_context_cluster(ui, cx, &chips, context_width, &mut raised);
                        });
                    });
                }
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    match raised {
        Some(ChromeRaise::Execute(key)) => cx.execute(key),
        Some(ChromeRaise::Shell(request)) => cx.wb.outbox.push(request),
        None => {}
    }
}

fn project_context_cluster(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    chips: &[HealthChip],
    desired: f32,
    raised: &mut Option<ChromeRaise>,
) {
    if chips.is_empty() {
        return;
    }
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().min(desired).max(1.0);
    ui.allocate_ui_with_layout(
        vec2(width, PROJECT_TAB_HEIGHT),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = CHIP_GAP;
            project_revision_label(ui, cx, &tokens);
            for chip in chips {
                let response = project_status_chip(
                    ui,
                    chip.kind.id_salt(),
                    &chip.label,
                    &chip.description,
                    chip.problem,
                    chip.enabled,
                );
                if response.clicked() {
                    chip_clicked(cx, chip.kind, raised);
                }
            }
        },
    );
}

/// Room between the revision and the chips, and between chips.
const CHIP_GAP: f32 = 8.0;
/// A chip's padding either side of its words.
const CHIP_PADDING: f32 = 7.0;

/// What the band to the right of the tabs needs: the revision and every
/// chip at its own width.
fn project_context_desired_width(
    ui: &Ui,
    cx: &Cx<'_, ProjectFeature>,
    chips: &[HealthChip],
) -> f32 {
    if chips.is_empty() {
        return 0.0;
    }
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let measure = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), tokens.color.text_dim)
            .size()
            .x
    };
    let chips_width = chips
        .iter()
        .map(|chip| {
            measure(&chip.label)
                + 2.0 * CHIP_PADDING
                + if chip.problem.is_some() {
                    CHIP_MARK_ADVANCE
                } else {
                    0.0
                }
        })
        .sum::<f32>();
    measure(&revision_label(cx)) + chips_width + CHIP_GAP * chips.len() as f32
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
            let (_, rect) =
                ui.allocate_space(vec2(text_width + 20.0 + mark_width, PROJECT_TAB_HEIGHT));
            let response = ui.interact(rect, tab_ids[index], Sense::click());
            let selected = cx.state.project_page == page;
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

/// The project's revision, as the Overview's header says it.
fn revision_label(cx: &Cx<'_, ProjectFeature>) -> String {
    format!(
        "Revision {}",
        cx.model.workspace.content.project.revision().get()
    )
}

fn project_revision_label(ui: &mut Ui, cx: &Cx<'_, ProjectFeature>, tokens: &Tokens) {
    ui.label(
        egui::RichText::new(revision_label(cx))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(tokens.color.text_faint),
    );
}

/// Which subject a chip states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChipKind {
    Design(DesignSubject),
    Configuration,
    LatestRun,
}

impl ChipKind {
    const fn id_salt(self) -> &'static str {
        match self {
            Self::Design(DesignSubject::SchematicChecks) => "project-checks",
            Self::Design(DesignSubject::SourceDeck) => "project-source",
            Self::Configuration => "project-configuration",
            Self::LatestRun => "project-latest-run",
        }
    }
}

/// One chip over a page other than the Overview: a link to the subject the
/// page leans on, in the subject's own words and mark.
struct HealthChip {
    kind: ChipKind,
    label: String,
    description: String,
    problem: Option<Problem>,
    enabled: bool,
}

/// The chips the band shows: the design subject, the configuration runs
/// use, and the latest run, each read from its subject's status.
fn health_chips(ctx: &Context, cx: &Cx<'_, ProjectFeature>) -> Vec<HealthChip> {
    let model = &*cx.model;
    let (subject, design) = health::design(
        model,
        cx.runs.slot_facts(cx.peers.design().review_markers()),
    );
    let configuration = health::configuration(ctx, model);
    let run = health::latest_run(model);
    vec![
        HealthChip {
            kind: ChipKind::Design(subject),
            label: format!("{} \u{b7} {}", subject.chip_label(), design.state),
            description: format!("{}: {}", subject.label(), subject_text(&design)),
            problem: design.tone.problem(),
            enabled: true,
        },
        // A configuration's name is a name, not a state, and Default is
        // what a project that uses no configuration runs with rather than a
        // fault.
        HealthChip {
            kind: ChipKind::Configuration,
            label: format!("Configuration \u{b7} {}", configuration.state),
            description: format!("Configuration used for runs: {}", configuration.state),
            problem: configuration.tone.problem(),
            enabled: true,
        },
        HealthChip {
            kind: ChipKind::LatestRun,
            description: format!("Latest run: {}", subject_text(&run)),
            label: run.state,
            problem: run.tone.problem(),
            enabled: !model.simulation.retained.runs.is_empty(),
        },
    ]
}

/// A subject's state, then its detail when it has one.
fn subject_text(status: &health::SubjectStatus) -> String {
    if status.detail.is_empty() {
        status.state.clone()
    } else {
        format!("{} \u{b7} {}", status.state, status.detail)
    }
}

/// Carry out what a chip names: check the cell view the checks describe,
/// open the deck, the Configuration page, or the latest run.
fn chip_clicked(cx: &mut Cx<'_, ProjectFeature>, kind: ChipKind, raised: &mut Option<ChromeRaise>) {
    match kind {
        ChipKind::Design(DesignSubject::SchematicChecks) => {
            let root = cx.model.workspace.content.simulation_root_reference();
            *raised = Some(ChromeRaise::Shell(ProjectRequest::OpenRootAndCheck(root)));
        }
        ChipKind::Design(DesignSubject::SourceDeck) => {
            *raised = Some(ChromeRaise::Execute(ids::open_workspace(
                Workspace::Netlist,
            )));
        }
        ChipKind::Configuration => {
            // The Configurations command, carried out where the chip is drawn
            // so the band's action after it reads the page it opens, when the
            // dispatcher's answer for it, taken as the chrome was lent, says
            // it can run.
            if cx
                .foreign_availability(ids::CONFIGURATION_SETS)
                .is_available()
            {
                show_configuration_page_in(cx.platform, cx.wb, &mut cx.state.project_page);
            }
        }
        ChipKind::LatestRun => {
            let Some(run) = cx.model.simulation.retained.runs.first() else {
                return;
            };
            // A run with no dataset sends the reader to wherever the project
            // can actually be run again, which for a netlist-first project is
            // the deck, not Simulate.
            let destination = overview::run_destination(
                !run.analyses.is_empty(),
                cx.model.is_netlist_first_without_schematic(),
            );
            if cx.model.simulation.select_run(0) {
                *raised = Some(ChromeRaise::Execute(ids::open_workspace(destination)));
            }
        }
    }
}

/// Side of a problem mark painted inside a chip, and the advance from the
/// mark's left edge to the text after it: read from the kit, so a marked chip
/// and an unmarked one beside it keep the same text rhythm.
use crate::ui::widgets::{
    PROBLEM_MARK_ADVANCE as CHIP_MARK_ADVANCE, PROBLEM_MARK_SIDE as CHIP_MARK_SIDE,
};

/// One of the title row's health chips: a link to the page that owns the
/// fact, so its words stay neutral. A failure the reader has to act on puts
/// the kit's cross in front of them; nothing else is coloured.
fn project_status_chip(
    ui: &mut Ui,
    id_salt: &'static str,
    label: &str,
    accessible_label: &str,
    problem: Option<Problem>,
    enabled: bool,
) -> Response {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let text_width = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), tokens.color.text_dim)
        .size()
        .x;
    let mark_advance = if problem.is_some() {
        CHIP_MARK_ADVANCE
    } else {
        0.0
    };
    let (_, rect) = ui.allocate_space(vec2(text_width + mark_advance + 2.0 * CHIP_PADDING, 22.0));
    let response = ui.interact(
        rect,
        ui.id().with(("project-status-chip", id_salt)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if enabled && (response.hovered() || response.has_focus()) {
        ui.painter().rect_filled(rect, 3.0, tokens.color.bg_hover);
    }
    let text_left = rect.left() + CHIP_PADDING + mark_advance;
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(
                    rect.left() + CHIP_PADDING + CHIP_MARK_SIDE * 0.5,
                    rect.center().y + 0.5,
                ),
                vec2(CHIP_MARK_SIDE, CHIP_MARK_SIDE),
            ),
            problem,
        );
    }
    ui.painter().text(
        pos2(text_left, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        font,
        if enabled {
            tokens.color.text_dim
        } else {
            tokens.color.text_faint
        },
    );
    let announcement = problem.map_or_else(
        || accessible_label.to_owned(),
        |problem| problem.announce(accessible_label),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, announcement.clone()));
    if enabled {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response.on_hover_text(accessible_label)
}

/// A ledger panel's own header: its name on the left and one fact about the
/// list on the right. The fact is never painted as a verdict; a panel the
/// reader has to look at carries the kit's mark in front of it instead.
fn workspace_table_panel_header(ui: &mut Ui, title: &str, meta: &str, problem: Option<Problem>) {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) =
        ui.allocate_exact_size(vec2(width, WORKSPACE_TABLE_HEADER_HEIGHT), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        Stroke::new(1.0, tokens.color.border_strong),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    let content = rect.shrink2(vec2(11.0, 0.0));
    let gap = 8.0;
    let title_width = ((content.width() - gap) * 0.62).max(0.0);
    let title_rect = Rect::from_min_max(
        content.min,
        pos2(content.left() + title_width, content.bottom()),
    );
    let meta_rect = Rect::from_min_max(
        pos2(
            (title_rect.right() + gap).min(content.right()),
            content.top(),
        ),
        content.max,
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let title = elide_text(ui, title, &title_font, title_rect.width());
    let meta = elide_text(ui, meta, &meta_font, meta_rect.width());
    ui.painter().with_clip_rect(title_rect).text(
        title_rect.left_center(),
        Align2::LEFT_CENTER,
        title,
        title_font,
        tokens.color.text,
    );
    let meta_left = ui
        .painter()
        .with_clip_rect(meta_rect)
        .text(
            meta_rect.right_center(),
            Align2::RIGHT_CENTER,
            meta,
            meta_font,
            tokens.color.text_faint,
        )
        .left();
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(meta_left - 6.0 - CHIP_MARK_SIDE * 0.5, meta_rect.center().y),
                vec2(CHIP_MARK_SIDE, CHIP_MARK_SIDE),
            ),
            problem,
        );
    }
}

/// How one ledger row is painted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum WorkspaceRowKind {
    /// The column header: tighter, filled, and unclickable.
    Header,
    /// An ordinary body row.
    Body,
    /// The body row the page's selection points at.
    Selected,
}

impl WorkspaceRowKind {
    const fn is_header(self) -> bool {
        matches!(self, Self::Header)
    }

    /// The selected row for a boolean the caller already holds.
    const fn body(selected: bool) -> Self {
        if selected { Self::Selected } else { Self::Body }
    }
}

/// One ledger row.
///
/// `faint_columns` are the cells holding a placeholder rather than a value,
/// which take the faint tone so an absent value never competes with a real
/// one. `problem` marks one cell — a resource the reader has to look at —
/// with the kit's mark in front of its words; no cell's text is ever
/// coloured.
fn workspace_table_row<const N: usize>(
    ui: &mut Ui,
    width: f32,
    cells: [&str; N],
    fractions: [f32; N],
    kind: WorkspaceRowKind,
    mono_columns: &[usize],
    faint_columns: &[usize],
    problem: Option<(usize, Problem)>,
) -> (egui::Response, Vec<Rect>) {
    let tokens = Tokens::get(ui.ctx());
    let header = kind.is_header();
    let height = if header {
        workspace_table_column_header_height(ui)
    } else {
        workspace_table_row_height(ui)
    };
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, height),
        if header {
            Sense::hover()
        } else {
            Sense::click()
        },
    );
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    // A row the reader can select has to answer the pointer. Marking the
    // selection with a two-pixel rule alone left the table looking painted on.
    if header {
        painter.rect_filled(rect, 0.0, tokens.color.bg_panel_2);
    } else if kind == WorkspaceRowKind::Selected {
        painter.rect_filled(rect, 0.0, tokens.color.bg_active);
    } else if response.hovered() {
        painter.rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    if kind == WorkspaceRowKind::Selected {
        painter.rect_filled(
            Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
            0.0,
            tokens.color.accent,
        );
    }
    let mut left = rect.left();
    let mut cell_rects = Vec::with_capacity(N);
    for (index, (text, fraction)) in cells.iter().zip(fractions.iter()).enumerate() {
        let right = if index + 1 == N {
            rect.right()
        } else {
            left + rect.width() * fraction
        };
        let cell = Rect::from_min_max(pos2(left, rect.top()), pos2(right, rect.bottom()));
        if index > 0 {
            painter.vline(left, rect.y_range(), Stroke::new(1.0, tokens.color.border));
        }
        let text_color = if header || faint_columns.contains(&index) {
            tokens.color.text_faint
        } else {
            tokens.color.text_dim
        };
        let font = if header {
            theme::sans(tokens::FS_0, FontWeight::Medium)
        } else if mono_columns.contains(&index) {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        } else {
            theme::sans(tokens::FS_0, FontWeight::Regular)
        };
        let mark = problem.and_then(|(column, problem)| (column == index).then_some(problem));
        if let Some(problem) = mark {
            paint_problem_mark(
                &painter,
                Rect::from_center_size(
                    pos2(cell.left() + 8.0 + CHIP_MARK_SIDE * 0.5, cell.center().y),
                    vec2(CHIP_MARK_SIDE, CHIP_MARK_SIDE),
                ),
                problem,
            );
        }
        let text_left = 8.0
            + if mark.is_some() {
                CHIP_MARK_ADVANCE
            } else {
                0.0
            };
        let text_rect = cell.shrink2(vec2(text_left, 0.0));
        let text = elide_text(ui, text, &font, text_rect.width());
        painter.with_clip_rect(text_rect).text(
            cell.left_center() + vec2(text_left, 0.0),
            Align2::LEFT_CENTER,
            text,
            font,
            text_color,
        );
        cell_rects.push(cell);
        left = right;
    }
    let announcement = {
        let joined = cells.join(", ");
        problem.map_or(joined.clone(), |(_, problem)| problem.announce(&joined))
    };
    response.widget_info(|| {
        if header {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
        } else {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                kind == WorkspaceRowKind::Selected,
                &announcement,
            )
        }
    });
    // No-ops for a header, which senses hover only and can never hold focus.
    theme::paint_focus_ring(ui, &response, rect);
    (response, cell_rects)
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

fn muted(ui: &mut Ui, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(tokens.color.text_faint),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn technology_copy_does_not_claim_physical_pdk_qualification() {
        for copy in [
            TECHNOLOGY_SURFACE_ACTION,
            TECHNOLOGY_DIALOG_TITLE,
            TECHNOLOGY_DIALOG_PRIMARY,
            TECHNOLOGY_DIALOG_CURRENT,
            TECHNOLOGY_DIALOG_PENDING,
        ] {
            let copy = copy.to_ascii_lowercase();
            assert!(!copy.contains("qualified"));
            assert!(!copy.contains("rule deck"));
        }
    }
}

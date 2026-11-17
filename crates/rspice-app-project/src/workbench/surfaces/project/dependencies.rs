//! The libraries the project depends on, and its technology.
//!
//! Every row's status is the one the rail and the inspector beside the page
//! give the same library: all three read the rows built once per change of
//! what a status reads, under the workspace navigator's filter, which is the
//! page's one filter. The exported manifest serializes the same rows.
//!
//! Only the rows in view are laid out, so a site install with hundreds of
//! model libraries costs a frame what ten do.

use std::sync::Arc;

use serde::Serialize;

use super::*;
use crate::workbench::design_system::property_row_toned;
use crate::workbench::project::dependency_status::{
    DependencyRow, UNSET, refresh_source_drift, settle_selection, technology_status,
    visible_dependency_rows,
};
use crate::workbench::surfaces::project::overview::Tone;

const DEPENDENCY_SPLIT_BREAKPOINT: f32 = 640.0;
const DEPENDENCY_LEFT_SHARE: f32 = 0.40;
/// Width under which the table drops its digest column.
const DEPENDENCY_COMPACT_WIDTH: f32 = 560.0;
/// Shortest the split panes are ever drawn, so a very short viewport still
/// opens a readable table rather than seating an action bar under a header.
const DEPENDENCY_MIN_PANE_HEIGHT: f32 = 260.0;
/// Vertical padding inside a panel's action bar.
const DEPENDENCY_ACTION_BAR_PAD: i8 = 6;

const LEDGER_COLUMNS: [&str; 6] = ["Library", "Type", "Version", "Source", "Digest", "Status"];
const LEDGER_FRACTIONS: [f32; 6] = [0.24, 0.14, 0.09, 0.26, 0.11, 0.16];
/// Version and digest are identities rather than prose, so both take the mono
/// face and stay comparable down the column.
const LEDGER_MONO_COLUMNS: [usize; 2] = [2, 4];
const LEDGER_COMPACT_COLUMNS: [&str; 5] = ["Library", "Type", "Version", "Source", "Status"];
const LEDGER_COMPACT_FRACTIONS: [f32; 5] = [0.26, 0.15, 0.14, 0.24, 0.21];
const LEDGER_COMPACT_MONO_COLUMNS: [usize; 1] = [2];

#[derive(Debug, Clone, Serialize)]
struct DependencyManifest {
    schema_version: u16,
    project_id: String,
    project_revision: u64,
    technology: Option<TechnologyManifest>,
    libraries: Vec<ManifestLibrary>,
}

#[derive(Debug, Clone, Serialize)]
struct TechnologyManifest {
    label: String,
    model_library: String,
    model_file: String,
    source_files: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ManifestLibrary {
    name: String,
    library_type: &'static str,
    version: String,
    source: String,
    digest: String,
    status: &'static str,
    detail: Option<String>,
}

/// The table as the page reads it: every row, and the rows the filter lets
/// through, by index.
#[derive(Debug, Clone)]
struct LedgerView {
    rows: Arc<[DependencyRow]>,
    visible: Arc<[usize]>,
    filtered: bool,
}

pub fn dependencies(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    // The rail draws first and does both of these; a phone width shows the
    // page without it.
    refresh_source_drift(ui.ctx(), cx.model);
    let filter = cx.wb.navigator_filter().trim().to_owned();
    let (rows, visible) = visible_dependency_rows(ui.ctx(), cx.model, &filter);
    settle_selection(&rows, &visible, &mut cx.state.project_dependency_selection);
    let view = LedgerView {
        rows,
        visible,
        filtered: !filter.is_empty(),
    };
    let width = visible_workspace_width(ui);
    if width < DEPENDENCY_SPLIT_BREAKPOINT {
        dependency_pane(ui, width, 0.0, |ui| {
            technology_binding_panel(ui, cx);
            dependency_inventory(ui, cx, &view, width, 0.0);
        });
        return;
    }
    let pane_height = visible_workspace_height(ui).max(DEPENDENCY_MIN_PANE_HEIGHT);
    let left_width = (width * DEPENDENCY_LEFT_SHARE).floor().max(240.0);
    let right_width = (width - left_width - 1.0).max(320.0);
    let shown = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 1.0;
        dependency_pane(ui, left_width, pane_height, |ui| {
            technology_binding_panel(ui, cx);
        });
        dependency_pane(ui, right_width, pane_height, |ui| {
            dependency_inventory(ui, cx, &view, right_width, pane_height);
        });
    });
    ui.painter().vline(
        shown.response.rect.left() + left_width + 0.5,
        shown.response.rect.y_range(),
        Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
    );
}

/// One side of the split, drawn to the full height of the workspace so the two
/// panels end on the same line and the divider between them runs all the way
/// down rather than stopping under whichever panel ran out of content first.
fn dependency_pane(ui: &mut Ui, width: f32, height: f32, content: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
        // `allocate_ui_with_layout` advances the cursor by the content width
        // rather than the requested track, so the track is asserted inside —
        // and `set_width` is what stops a child from laying itself out against
        // the central panel's unconstrained width and painting past the clip.
        ui.set_width(width);
        ui.set_min_height(height);
        content(ui);
    });
}

/// A panel's action bar, seated on the floor of its pane under a rule.
///
/// Buttons that simply follow the last row read as dropped into the middle of
/// the panel, which is what this page did: the reader cannot tell what they
/// act on, and the space below them belongs to nothing.
fn dependency_action_bar(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, DEPENDENCY_ACTION_BAR_PAD))
        .show(ui, |ui| {
            // A `Frame` shrinks to its content, and an action bar that stops
            // short of the pane edge reads as a floating box.
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                content(ui);
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn dependency_action_bar_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.ctl_h + 2.0 * f32::from(DEPENDENCY_ACTION_BAR_PAD)
}

/// Free height between a panel's content and its action bar, or nothing when
/// the content already reaches the floor.
fn dependency_panel_free_height(ui: &Ui, pane_height: f32, top: f32) -> f32 {
    pane_height - (ui.cursor().top() - top) - dependency_action_bar_height(ui)
}

fn technology_binding_panel(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 0.0;
    let binding = cx
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .cloned();
    // The status the inspector beside the page states. A project with no
    // technology is ordinary unless its setup needs one, so only a status
    // the reader has to act on is named in the header.
    let status = technology_status(cx.model);
    let problem = status.mark.problem();
    workspace_table_panel_header(
        ui,
        "Technology",
        if problem.is_some() { status.label } else { "" },
        problem,
    );
    if let Some(binding) = binding.as_ref() {
        property_row(ui, "Technology", &binding.display_label());
        property_row(ui, "Model library", binding.model_library());
        let source = binding.root_source().display().to_string();
        property_row(ui, "Model file", &source).on_hover_text(source.as_str());
        property_row(
            ui,
            "Source files",
            &binding.source_closure().len().to_string(),
        );
    } else {
        // An unset value is not a value: toning these faint keeps the reader
        // on the rows below that do report live state.
        property_row_toned(ui, "Technology", status.label, t.color.text_faint);
        for unset in ["Model library", "Model file", "Source files"] {
            property_row_toned(ui, unset, UNSET, t.color.text_faint);
        }
    }
    if let Some(pin) = binding
        .as_ref()
        .and_then(crate::state::ProjectTechnologyBinding::pdk_package)
    {
        property_row(
            ui,
            "PDK package",
            &format!(
                "{} \u{00b7} {} \u{00b7} {} nm \u{00b7} {}",
                pin.package_id(),
                pin.revision(),
                pin.process_node_nm(),
                pin.stack_name()
            ),
        );
        property_row(ui, "Publisher", pin.publisher_id());
        property_row(
            ui,
            "Package digests",
            &format!(
                "manifest {} \u{00b7} archive {}",
                short_identity(&pin.manifest_digest().to_string()),
                short_identity(&pin.archive_digest().to_string())
            ),
        );
    } else {
        property_row_toned(ui, "PDK package", "Not attached", t.color.text_faint);
    }
    let libraries = cx.model.model_library_manager.library_count();
    let models = cx.model.model_library_manager.total_model_count();
    property_row(
        ui,
        "Model catalog",
        &format!(
            "{libraries} librar{} \u{00b7} {models} model{}",
            if libraries == 1 { "y" } else { "ies" },
            plural_suffix(models)
        ),
    );
    property_row(
        ui,
        "Execution",
        crate::state::ExecutionTarget::current().label(),
    );
    // What the reader has to act on, in its own words; an attached
    // technology that resolves, and a project that needs none, say nothing.
    if let (Some(problem), Some(detail)) = (problem, status.detail.as_deref()) {
        technology_note(ui, problem, detail);
    }
    let model_paths = cx.commands.label(ids::PDK_SETTINGS);
    dependency_action_bar(ui, |ui| {
        if Button::new(TECHNOLOGY_SURFACE_ACTION)
            .accent()
            .show(ui)
            .clicked()
        {
            open_technology_attachment_dialog(cx);
        }
        if Button::new(model_paths).show(ui).clicked() {
            cx.execute(ids::PDK_SETTINGS);
        }
    });
    // The panel ends here rather than at the floor of the pane, so close it.
    // `min_rect` already reaches the floor — the pane asserts its own height so
    // the divider runs the whole way down — and the cursor is what tracks the
    // content.
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

/// What stands in the way of the project's technology, under a rule that
/// separates it from the property list above and the buttons below.
fn technology_note(ui: &mut Ui, problem: Problem, message: &str) {
    let t = Tokens::get(ui.ctx());
    // `problem_line` carries the panel body's own 10 px inset, which is where
    // the property labels above it start, so the frame adds none of its own.
    let shown = egui::Frame::new()
        .inner_margin(Margin::symmetric(0, 7))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            problem_line(ui, problem, message);
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn dependency_inventory(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    view: &LedgerView,
    pane_width: f32,
    pane_height: f32,
) {
    let top = ui.cursor().top();
    ui.spacing_mut().item_spacing.y = 0.0;
    // The count, out of the whole while the filter hides some; the mark of
    // the worst status, so a library to act on is announced even when its row
    // is scrolled away.
    let total = view.rows.len();
    let count = if view.filtered {
        format!("{} of {total}", view.visible.len())
    } else {
        total.to_string()
    };
    let worst = view
        .rows
        .iter()
        .map(|row| row.status.mark)
        .max_by_key(|mark| match mark {
            Tone::Neutral => 0,
            Tone::Warn => 1,
            Tone::Error => 2,
        })
        .and_then(Tone::problem);
    workspace_table_panel_header(ui, "Libraries and model files", &count, worst);
    // The pane owns the column grid, not the unconstrained central panel: a
    // table bound to the wrong width paints its status column underneath the
    // Inspector dock. Both layouts elide into the pane, so the columns are
    // always complete and the table never scrolls sideways.
    let width = pane_width.min(visible_workspace_width(ui)).max(1.0);
    let compact = width < DEPENDENCY_COMPACT_WIDTH;
    // Beside the technology panel the rows scroll within the pane, so the
    // grid leaves the scroll bar its gutter whether or not the bar shows:
    // the header, drawn outside the scroll, then lines up with the rows in
    // it however long the list grows.
    let width = if pane_height > 0.0 {
        (width - ui.spacing().scroll.allocated_width()).max(1.0)
    } else {
        width
    };
    if compact {
        workspace_table_row(
            ui,
            width,
            LEDGER_COMPACT_COLUMNS,
            LEDGER_COMPACT_FRACTIONS,
            WorkspaceRowKind::Header,
            &[],
            &[],
            None,
        );
    } else {
        workspace_table_row(
            ui,
            width,
            LEDGER_COLUMNS,
            LEDGER_FRACTIONS,
            WorkspaceRowKind::Header,
            &[],
            &[],
            None,
        );
    }
    if view.rows.is_empty() {
        workspace_empty_table_row(ui, width, "The project has no libraries.");
    } else if view.visible.is_empty() {
        workspace_empty_table_row(ui, width, "No library matches the filter.");
    }
    let row_height = workspace_table_row_height(ui);
    let mut selected = None;
    let mut draw_rows = |ui: &mut Ui, range: std::ops::Range<usize>| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for &index in &view.visible[range] {
            let row = &view.rows[index];
            if ledger_row(ui, cx, row, width, compact) {
                selected = Some(row.library.clone());
            }
        }
    };
    let count = view.visible.len();
    if count > 0 && pane_height > 0.0 {
        // Beside the technology panel the table scrolls within its pane,
        // above an action bar that stays in view.
        ScrollArea::vertical()
            .id_salt("workbench.project.dependencies.ledger")
            .auto_shrink([false, true])
            .max_height(dependency_panel_free_height(ui, pane_height, top).max(row_height))
            .show_rows(ui, row_height, count, draw_rows);
    } else if count > 0 {
        // Stacked under it, the table is part of the page's own scroll: a
        // second scroll inside that one would trap a touch drag. It takes the
        // height of every row and lays out only those in the page's view.
        let (rect, _) =
            ui.allocate_exact_size(vec2(width, row_height * count as f32), Sense::hover());
        let shown = ui.clip_rect().intersect(rect);
        if shown.is_positive() {
            let first = (((shown.top() - rect.top()) / row_height).floor() as usize).min(count);
            let last = (((shown.bottom() - rect.top()) / row_height).ceil() as usize).min(count);
            let rows = Rect::from_min_size(
                pos2(rect.left(), rect.top() + first as f32 * row_height),
                vec2(width, (last - first) as f32 * row_height),
            );
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rows)
                    .layout(Layout::top_down(Align::Min)),
                |ui| draw_rows(ui, first..last),
            );
        }
    }
    if let Some(library) = selected {
        cx.state.project_dependency_selection = Some(library);
    }
    if compact {
        ledger_grid_floor(ui, width, pane_height, top, LEDGER_COMPACT_FRACTIONS);
    } else {
        ledger_grid_floor(ui, width, pane_height, top, LEDGER_FRACTIONS);
    }
    dependency_action_bar(ui, |ui| {
        if Button::new("Export manifest\u{2026}").show(ui).clicked() {
            export_dependency_manifest(cx, &view.rows);
        }
    });
}

/// One table row, and whether the reader clicked it.
fn ledger_row(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    row: &DependencyRow,
    width: f32,
    compact: bool,
) -> bool {
    let kind = WorkspaceRowKind::body(
        cx.state.project_dependency_selection.as_ref() == Some(&row.library),
    );
    let problem = row.status.mark.problem();
    let response = if compact {
        let cells = [
            row.library.name.as_str(),
            row.library.kind.label(),
            row.version.as_str(),
            row.source.as_str(),
            row.status.label,
        ];
        workspace_table_row(
            ui,
            width,
            cells,
            LEDGER_COMPACT_FRACTIONS,
            kind,
            &LEDGER_COMPACT_MONO_COLUMNS,
            &unset_columns(&cells),
            problem.map(|problem| (4, problem)),
        )
        .0
    } else {
        let cells = [
            row.library.name.as_str(),
            row.library.kind.label(),
            row.version.as_str(),
            row.source.as_str(),
            row.digest.as_str(),
            row.status.label,
        ];
        workspace_table_row(
            ui,
            width,
            cells,
            LEDGER_FRACTIONS,
            kind,
            &LEDGER_MONO_COLUMNS,
            &unset_columns(&cells),
            problem.map(|problem| (5, problem)),
        )
        .0
    };
    // What the status has to add has no column of its own; the row carries
    // it, as the inspector does under the status.
    let hover = match row.status.detail.as_deref() {
        Some(detail) => format!("{}\n{}", row.library.name, detail),
        None => format!("{}\n{}", row.library.name, row.source),
    };
    response.on_hover_text(hover).clicked()
}

/// Carry the table's column rules down to the action bar.
///
/// A table that simply stops leaves its pane looking half-drawn; continuing
/// the grid says the columns are complete and the rows have run out.
fn ledger_grid_floor<const N: usize>(
    ui: &mut Ui,
    width: f32,
    pane_height: f32,
    top: f32,
    fractions: [f32; N],
) {
    let free = dependency_panel_free_height(ui, pane_height, top);
    if free <= 0.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(width, free), Sense::hover());
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    let mut left = rect.left();
    for fraction in fractions.iter().take(N.saturating_sub(1)) {
        left += rect.width() * fraction;
        painter.vline(left, rect.y_range(), Stroke::new(1.0, t.color.border));
    }
}

/// The columns of one table row holding a placeholder rather than a value.
/// They take the faint tone, so an absent value never competes with a real
/// one down the column.
fn unset_columns<const N: usize>(cells: &[&str; N]) -> Vec<usize> {
    cells
        .iter()
        .enumerate()
        .filter_map(|(index, cell)| (*cell == UNSET).then_some(index))
        .collect()
}

fn dependency_manifest(cx: &Cx<'_, ProjectFeature>, rows: &[DependencyRow]) -> DependencyManifest {
    let technology = cx
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .map(|binding| TechnologyManifest {
            label: binding.display_label(),
            model_library: binding.model_library().to_owned(),
            model_file: binding.root_source().display().to_string(),
            source_files: binding.source_closure().len(),
        });
    DependencyManifest {
        schema_version: 2,
        project_id: cx.model.workspace.content.project.id().to_string(),
        project_revision: cx.model.workspace.content.project.revision().get(),
        technology,
        libraries: rows
            .iter()
            .map(|row| ManifestLibrary {
                name: row.library.name.clone(),
                library_type: row.library.kind.label(),
                version: row.version.clone(),
                source: row.source.clone(),
                digest: row.digest.clone(),
                status: row.status.label,
                detail: row.status.detail.clone(),
            })
            .collect(),
    }
}

fn export_dependency_manifest(cx: &mut Cx<'_, ProjectFeature>, rows: &[DependencyRow]) {
    let contents = match serde_json::to_vec_pretty(&dependency_manifest(cx, rows)) {
        Ok(contents) => contents,
        Err(error) => {
            cx.model.report_failure(
                "Manifest export failed",
                format!("Dependency manifest serialization failed: {error}"),
            );
            return;
        }
    };
    let filename = format!(
        "{}-dependencies.json",
        cx.model
            .workspace
            .content
            .project
            .display_name()
            .chars()
            .map(|character| if character.is_alphanumeric() {
                character
            } else {
                '-'
            })
            .collect::<String>()
            .trim_matches('-')
    );

    match crate::workbench::workflows::export_workflow::publish_project_manifest(
        &filename, &contents,
    ) {
        Ok(Some(exported)) => cx.wb.prompt(exported),
        Ok(None) => {}
        Err(error) => cx.model.report_failure("Manifest export failed", error),
    }
}

//! The libraries the project depends on, and its technology.
//!
//! Every row's status is the one the rail and the inspector beside the page
//! give the same library: all three read the rows built once per change of
//! what a status reads, under the workspace navigator's filter, which is the
//! page's one filter. The exported manifest serializes the same rows.
//!
//! The ledger is the kit's row list: a head click sorts it, a head edge
//! sizes its column, and only the rows in view are drawn, so a site install
//! with hundreds of model libraries costs a frame what ten do. Each row
//! opens in its own workspace and offers, in its menu, what can be done to
//! it from here.

use std::sync::Arc;

use serde::Serialize;

use super::*;
use crate::state::model_library::ModelSourceAuthority;
use crate::ui::widgets::table::{Cell, Column, DataTable, RowKey, SortKey};
use crate::workbench::design_system::property_row_toned;
use crate::workbench::project::dependency_status::{
    DependencyKind, DependencyLibrary, DependencyRow, UNSET, dependency_status_key,
    refresh_source_drift, settle_selection, technology_status, visible_dependency_rows,
};
use crate::workbench::surfaces::project::derived::derived;
use crate::workbench::surfaces::project::overview::Tone;

const DEPENDENCY_SPLIT_BREAKPOINT: f32 = 640.0;
const DEPENDENCY_LEFT_SHARE: f32 = 0.40;
/// Shortest the split panes are ever drawn, so a very short viewport still
/// opens a readable table rather than seating an action bar under a header.
const DEPENDENCY_MIN_PANE_HEIGHT: f32 = 260.0;
/// Vertical padding inside a panel's action bar.
const DEPENDENCY_ACTION_BAR_PAD: i8 = 6;

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
    /// Each visible row's key, in the same order: the table keeps its
    /// selection by it through a sort or a change of the rows.
    keys: Arc<[RowKey]>,
    filtered: bool,
}

pub fn dependencies(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    // The rail draws first and does both of these; a phone width shows the
    // page without it.
    refresh_source_drift(ui.ctx(), cx.model);
    let filter = cx.wb.navigator_filter().trim().to_owned();
    let (rows, visible) = visible_dependency_rows(ui.ctx(), cx.model, &filter);
    settle_selection(&rows, &visible, &mut cx.state.project_dependency_selection);
    let keys = derived(
        ui.ctx(),
        "dependencies.keys",
        (dependency_status_key(cx.model), filter.to_lowercase()),
        || {
            visible
                .iter()
                .map(|&index| ledger_key(&rows[index].library))
                .collect::<Arc<[RowKey]>>()
        },
    );
    let view = LedgerView {
        rows,
        visible,
        keys,
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
/// A zero `height` is the stacked pane: as tall as what it holds, in the
/// page's own scroll, so its table shows every row rather than the sliver a
/// zero-high track left it.
fn dependency_pane(ui: &mut Ui, width: f32, height: f32, content: impl FnOnce(&mut Ui)) {
    let track = if height > 0.0 {
        height
    } else {
        ui.available_height()
    };
    ui.allocate_ui_with_layout(vec2(width, track), Layout::top_down(Align::Min), |ui| {
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
            // A narrow pane takes its buttons onto a second line rather than
            // running the last one over the divider into the next pane.
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
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
        value_row(ui, "Technology", &binding.display_label());
        value_row(ui, "Model library", binding.model_library());
        let source = binding.root_source().display().to_string();
        value_row(ui, "Model file", &source);
        value_row(
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
        value_row(
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
        value_row(ui, "Publisher", pin.publisher_id());
        value_row(
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
    value_row(
        ui,
        "Model catalog",
        &format!(
            "{libraries} librar{} \u{00b7} {models} model{}",
            if libraries == 1 { "y" } else { "ies" },
            plural_suffix(models)
        ),
    );
    value_row(
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
        // Beside Attach, through the same checkpointed change.
        if Button::new(TECHNOLOGY_DETACH_ACTION)
            .enabled(binding.is_some())
            .show(ui)
            .clicked()
        {
            open_technology_detach_dialog(cx);
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

/// A technology value: one line, cut to its column, under a fine pointer,
/// whose hover says the rest; every word of it, wrapped, under a finger,
/// which has no hover.
fn value_row(ui: &mut Ui, label: &str, value: &str) {
    if Tokens::get(ui.ctx()).metrics.is_touch() {
        crate::workbench::design_system::property_row_wrapped(ui, label, value);
    } else {
        property_row(ui, label, value);
    }
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
        .max_by_key(|mark| severity(*mark))
        .and_then(Tone::problem);
    workspace_table_panel_header(ui, "Libraries and model files", &count, worst);
    // The pane owns the table's width, not the unconstrained central panel:
    // a table bound to the wrong width paints its status column underneath
    // the Inspector dock.
    let width = pane_width.min(visible_workspace_width(ui)).max(1.0);
    if view.rows.is_empty() {
        workspace_empty_table_row(ui, width, "The project has no libraries.");
    } else if view.visible.is_empty() {
        workspace_empty_table_row(ui, width, "No library matches the filter.");
    } else {
        // Beside the technology panel the rows scroll inside the pane, above
        // an action bar that stays in view. Stacked under it, the table is
        // part of the page's own scroll: a second scroll inside that one
        // would trap a touch drag.
        let max_height = if pane_height > 0.0 {
            dependency_panel_free_height(ui, pane_height, top)
                .max(workspace_table_row_height(ui) * 2.0)
        } else {
            f32::INFINITY
        };
        let asked = ui
            .scope(|ui| {
                ui.set_width(width);
                ledger_table(ui, cx, view, max_height)
            })
            .inner;
        if let Some((library, action)) = asked {
            run_ledger_action(ui.ctx(), cx, &library, action);
        }
    }
    let free = dependency_panel_free_height(ui, pane_height, top);
    if free > 0.0 {
        ui.add_space(free);
    }
    let reading = cx.state.dependencies.reading();
    dependency_action_bar(ui, |ui| {
        if Button::new("Export manifest\u{2026}").show(ui).clicked() {
            export_dependency_manifest(cx, &view.rows);
        }
        if let Some(reading) = reading.as_deref() {
            muted(ui, reading);
        }
    });
}

/// How bad a status is: the header names the worst, and the status column
/// sorts by it.
const fn severity(mark: Tone) -> u8 {
    match mark {
        Tone::Neutral => 0,
        Tone::Warn => 1,
        Tone::Error => 2,
    }
}

/// The ledger's columns: what tells libraries apart, where each is read
/// from, and where it stands. A library's digest, access and files are in
/// the inspector beside the page.
fn ledger_columns() -> [Column; 5] {
    [
        Column::name("Library", 120.0),
        Column::words("Type", 64.0),
        Column::words("Version", 88.0),
        Column::name("Source", 140.0),
        Column::words("Status", 132.0),
    ]
}

/// The column the status is in, kept in view on a narrow pane.
const STATUS_COLUMN: usize = 4;

/// One library's cells, in the ledger's column order.
fn ledger_cells(row: &DependencyRow) -> Vec<Cell> {
    // The file, by name: a path cut short at its end shows only the folders
    // every row shares. Its hover, and the inspector, give the whole path.
    let file = std::path::Path::new(&row.source)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(&row.source);
    // A design library has no version: what its row once showed there was
    // the technology it names, which the inspector states as that.
    let version = if row.library.kind == DependencyKind::Design || row.version == UNSET {
        Cell::absent().sort_by(SortKey::Missing)
    } else {
        Cell::number(row.version.as_str())
    };
    let source = if row.source == UNSET {
        Cell::absent().sort_by(SortKey::Missing)
    } else {
        Cell::text(file).hint(row.source.as_str())
    };
    let status = match row.status.mark.problem() {
        Some(problem) => Cell::problem(row.status.label, problem),
        None => Cell::text(row.status.label),
    }
    .sort_by(SortKey::Status(severity(row.status.mark)));
    let status = match row.status.detail.as_deref() {
        Some(detail) => status.hint(detail),
        None => status,
    };
    vec![
        Cell::text(row.library.name.as_str()),
        Cell::text(row.library.kind.kind_word()),
        version,
        source,
        status,
    ]
}

/// A row's key: its catalogue and its name, since a design and a model
/// library may share a name.
fn ledger_key(library: &DependencyLibrary) -> RowKey {
    RowKey::Text(format!(
        "{}\u{1f}{}",
        library.kind.kind_word(),
        library.name
    ))
}

/// The ledger, its selection kept with the rail's and the inspector's, and
/// what the reader asked of a row: Enter or a double-click opens it, the
/// context menu offers what can be done to it, Delete removes it.
fn ledger_table(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    view: &LedgerView,
    max_height: f32,
) -> Option<(DependencyLibrary, LedgerAction)> {
    let keys = &view.keys;
    // The page's selection is the rail's: the table follows it.
    let selected = cx
        .state
        .project_dependency_selection
        .as_ref()
        .and_then(|selected| {
            view.visible
                .iter()
                .position(|&index| &view.rows[index].library == selected)
        });
    let table = &mut cx.state.dependencies.table;
    if let Some(row) = selected
        && table.selected_key() != Some(&keys[row])
    {
        table.select(row, Some(keys[row].clone()));
    }
    let columns = ledger_columns();
    let response = DataTable::new(
        "workbench.project.dependencies.ledger",
        &columns,
        view.visible.len(),
    )
    .state(table)
    .keys(keys)
    .freeze(1)
    .pin_when_narrow(STATUS_COLUMN)
    .max_height(max_height)
    .show(ui, |row| ledger_cells(&view.rows[view.visible[row]]));
    let library_at = |row: usize| {
        view.visible
            .get(row)
            .map(|&index| view.rows[index].library.clone())
    };
    if response.selection_changed
        && let Some(library) = cx
            .state
            .dependencies
            .table
            .selected_row()
            .and_then(library_at)
    {
        cx.state.project_dependency_selection = Some(library);
    }
    if let Some(library) = response.activate.and_then(library_at) {
        return Some((library, LedgerAction::Open));
    }
    if response.delete
        && let Some(library) = cx
            .state
            .dependencies
            .table
            .selected_row()
            .and_then(library_at)
    {
        return Some((library, LedgerAction::Remove));
    }
    ledger_menu(
        ui,
        cx,
        response.context_menu.and_then(|request| {
            library_at(request.cell.row).map(|library| (library, request.position))
        }),
    )
}

/// A row's context menu: opened where the reader asked for it, on the row
/// they asked it of, and kept open until an item is taken or they click
/// elsewhere. An item that cannot be taken now is shown, disabled.
fn ledger_menu(
    ui: &Ui,
    cx: &Cx<'_, ProjectFeature>,
    request: Option<(DependencyLibrary, egui::Pos2)>,
) -> Option<(DependencyLibrary, LedgerAction)> {
    let id = ui.id().with("workbench.project.dependencies.row menu");
    let at = id.with("at");
    let opened = request.is_some();
    if let Some(request) = request {
        ui.ctx().data_mut(|data| data.insert_temp(at, request));
    }
    let (library, position) = ui
        .ctx()
        .data(|data| data.get_temp::<(DependencyLibrary, egui::Pos2)>(at))?;
    if !opened && !egui::Popup::is_id_open(ui.ctx(), id) {
        return None;
    }
    let actions = ledger_actions(cx, &library);
    let mut taken = None;
    egui::Popup::new(
        id,
        ui.ctx().clone(),
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    .open_memory(opened.then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .show(|ui| {
        ui.set_min_width(160.0);
        for (action, enabled) in actions {
            if ui
                .add_enabled(enabled, egui::Button::new(action.label(&library)))
                .clicked()
            {
                taken = Some(action);
                ui.close();
            }
        }
    });
    taken.map(|action| (library, action))
}

/// What the reader can ask of one row of the ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LedgerAction {
    /// The library in its own workspace: a design library on the Library
    /// page, a model library in Models.
    Open,
    /// The folder its files are in.
    Reveal,
    /// Read its files from a place the reader chooses.
    Relink,
    /// Read its files again from where they were attached.
    Reload,
    /// Take it out of the project.
    Remove,
}

impl LedgerAction {
    const fn label(self, library: &DependencyLibrary) -> &'static str {
        match (self, library.kind) {
            (Self::Open, DependencyKind::Design) => "Open in Library",
            (Self::Open, DependencyKind::Model) => "Open in Models",
            (Self::Reveal, _) => "Show in folder",
            (Self::Relink, _) => "Relink\u{2026}",
            (Self::Reload, _) => "Reload",
            (Self::Remove, DependencyKind::Design) => "Delete library\u{2026}",
            (Self::Remove, DependencyKind::Model) => "Remove",
        }
    }
}

/// What the row's menu offers, in order, and whether each can be taken now.
/// A library read from a file on this machine can be shown in its folder
/// and read again; one read from a file, or whose bytes the project kept
/// from one, can be read from another file. A browser has no folder to show
/// and no file to read, so it offers only what the project itself holds.
fn ledger_actions(
    cx: &Cx<'_, ProjectFeature>,
    library: &DependencyLibrary,
) -> Vec<(LedgerAction, bool)> {
    let mut actions = vec![(LedgerAction::Open, true)];
    match library.kind {
        DependencyKind::Design => actions.push((LedgerAction::Remove, true)),
        DependencyKind::Model => {
            let Some(found) = cx.model.model_library_manager.get_library(&library.name) else {
                return actions;
            };
            let idle = cx.state.dependencies.reading().is_none();
            if cfg!(not(target_arch = "wasm32")) {
                let on_disk = found.has_external_root()
                    && matches!(found.source_authority, ModelSourceAuthority::External);
                let relinkable = found.pack_id.is_none()
                    && matches!(
                        found.source_authority,
                        ModelSourceAuthority::External
                            | ModelSourceAuthority::RetainedImport { .. }
                    );
                if on_disk {
                    actions.push((LedgerAction::Reveal, true));
                }
                if relinkable {
                    actions.push((LedgerAction::Relink, idle));
                }
                if on_disk {
                    actions.push((LedgerAction::Reload, idle));
                }
            }
            if found.source_authority.has_execution_source() {
                actions.push((LedgerAction::Remove, idle));
            }
        }
    }
    actions
}

/// Open `library` where it is edited, as its row's Open does: a design
/// library on the Library page, a model library in Models.
pub(in crate::workbench) fn open_dependency(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    library: &DependencyLibrary,
) {
    run_ledger_action(ctx, cx, library, LedgerAction::Open);
}

/// Carry out `action` on `library`, if the row still offers it.
fn run_ledger_action(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    library: &DependencyLibrary,
    action: LedgerAction,
) {
    let offered = ledger_actions(cx, library)
        .into_iter()
        .any(|(offered, enabled)| offered == action && enabled);
    if !offered {
        return;
    }
    match (action, library.kind) {
        (LedgerAction::Open, DependencyKind::Design) => {
            super::library::apply_library_intent(
                ctx,
                cx,
                super::library::LibraryIntent::SelectLibrary(library.name.clone()),
            );
            cx.state.project_page = ProjectPage::Library;
        }
        (LedgerAction::Open, DependencyKind::Model) => open_model_library(cx, &library.name),
        (LedgerAction::Remove, DependencyKind::Design) => {
            if let Err(refusal) = cx.state.open_delete_library_review(cx.model, &library.name) {
                cx.wb.prompt(refusal);
            }
        }
        (LedgerAction::Remove, DependencyKind::Model) => remove_model_library(cx, &library.name),
        #[cfg(not(target_arch = "wasm32"))]
        (LedgerAction::Reveal, DependencyKind::Model) => reveal_model_library(cx, &library.name),
        #[cfg(not(target_arch = "wasm32"))]
        (LedgerAction::Reload, DependencyKind::Model) => {
            start_model_library_read(ctx, cx, &library.name, None);
        }
        #[cfg(not(target_arch = "wasm32"))]
        (LedgerAction::Relink, DependencyKind::Model) => {
            if let Some(root) = pick_model_file(&library.name) {
                start_model_library_read(ctx, cx, &library.name, Some(root));
            }
        }
        _ => {}
    }
}

/// Show model library `name` in Models. The catalog lists models, not
/// libraries, so the library opens on its first model in the catalog's own
/// order; a library that holds none opens the catalog with it selected.
fn open_model_library(cx: &mut Cx<'_, ProjectFeature>, name: &str) {
    let first = cx
        .model
        .model_library_manager
        .get_library(name)
        .and_then(|library| {
            library
                .models
                .keys()
                .min_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()).then(a.cmp(b)))
                .cloned()
        });
    match first {
        Some(model) => cx
            .wb
            .outbox
            .push(crate::workbench::framework::requests::RevealModel {
                library: name.to_owned(),
                model,
            }),
        None => match cx.model.model_library_manager.select_library(name) {
            Ok(()) => cx.execute(ids::MODELS_CATALOG),
            Err(refusal) => cx.wb.prompt(refusal),
        },
    }
}

/// Take model library `name` out of the project, as one step of Undo. The
/// library the project's technology simulates from stays until the
/// technology is detached.
fn remove_model_library(cx: &mut Cx<'_, ProjectFeature>, name: &str) {
    if cx
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .is_some_and(|binding| binding.model_library() == name)
    {
        cx.wb.prompt(format!(
            "The project's technology simulates from '{name}'. Detach the technology before removing it."
        ));
        return;
    }
    let mut candidate = cx.model.model_library_manager.clone();
    if candidate.remove_library(name).is_none() {
        cx.wb.prompt(format!(
            "Model library '{name}' is no longer in the project."
        ));
        return;
    }
    let result = if cx.platform.project_lifecycle.is_open() {
        crate::workbench::app_state::design_history::publish_model_library_set_candidate(
            cx.model,
            candidate,
            format!("remove model library {name}"),
        )
        .map(|revision| {
            revision.map_or_else(
                || format!("Removed model library '{name}'."),
                |revision| {
                    format!(
                        "Removed model library '{name}' at project revision {}.",
                        revision.get()
                    )
                },
            )
        })
    } else {
        cx.model.model_library_manager = candidate;
        Ok(format!("Removed model library '{name}'."))
    };
    match result {
        Ok(message) => {
            cx.model
                .record(LogSeverity::Info, LogSource::Studio, message);
        }
        Err(error) => cx.model.report_failure("Model library not removed", error),
    }
}

/// Show the folder model library `name` reads its files from.
#[cfg(not(target_arch = "wasm32"))]
fn reveal_model_library(cx: &mut Cx<'_, ProjectFeature>, name: &str) {
    let Some(folder) = cx
        .model
        .model_library_manager
        .get_library(name)
        .and_then(|library| library.root_path.as_deref())
        .and_then(std::path::Path::parent)
        .map(std::path::Path::to_path_buf)
    else {
        return;
    };
    if let Err(error) = show_folder(&folder) {
        cx.model.report_failure("Folder not shown", error);
    }
}

#[cfg(all(not(any(test, feature = "test-support")), not(target_arch = "wasm32")))]
fn show_folder(folder: &std::path::Path) -> Result<(), String> {
    crate::workbench::lifecycle::recovery::reveal_directory(folder)
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
fn show_folder(folder: &std::path::Path) -> Result<(), String> {
    crate::workbench::app::dialogs::project::SHOWN_FOLDERS
        .with(|shown| shown.borrow_mut().push(folder.to_path_buf()));
    Ok(())
}

/// The model file the reader chooses for library `name`.
#[cfg(all(not(any(test, feature = "test-support")), not(target_arch = "wasm32")))]
fn pick_model_file(name: &str) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_title(format!("Relink model library {name}"))
        .add_filter(
            "SPICE model source",
            crate::state::pdk_config::MODEL_FILE_EXTENSIONS,
        )
        .add_filter("All files", &["*"])
        .pick_file()
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
fn pick_model_file(_: &str) -> Option<std::path::PathBuf> {
    crate::workbench::app::dialogs::project::RELINK_PICK.with(|pick| pick.borrow_mut().take())
}

/// Read model library `name` again, from `relink` when the reader chose a
/// new file and else from where it was attached, on a background thread.
/// The page states the one read in progress; the library changes as one
/// step of Undo when it is in.
#[cfg(not(target_arch = "wasm32"))]
fn start_model_library_read(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    name: &str,
    relink: Option<std::path::PathBuf>,
) {
    let Some(library) = cx.model.model_library_manager.get_library(name) else {
        return;
    };
    let Some(root) = relink.clone().or_else(|| library.root_path.clone()) else {
        return;
    };
    // A library is named by its file: the file it is relinked to keeps the
    // name, so every model bound to it still finds it.
    if relink.is_some() && root.file_stem().and_then(|stem| stem.to_str()) != Some(name) {
        cx.wb.prompt(format!(
            "Choose a file named '{name}': model library '{name}' is named by its file."
        ));
        return;
    }
    let section = library.selected_corner.clone();
    let pack = library.pack_id.clone().filter(|_| relink.is_none());
    let mut candidate = cx.model.model_library_manager.clone();
    let expected = name.to_owned();
    let file = root.file_name().map_or_else(
        || root.display().to_string(),
        |file| file.to_string_lossy().into_owned(),
    );
    let relinked = relink.is_some();
    let reading = crate::workbench::app::dialogs::project::ModelLibraryRead::start(
        ctx,
        name.to_owned(),
        file,
        relinked,
        cx.model.workspace.content.project.revision().get(),
        move || {
            if relinked {
                candidate.remove_library(&expected);
            }
            let loaded = match pack {
                Some(pack) => candidate.refresh_spice_pack(&pack),
                None => candidate.load_library_file(&root, section.as_deref()),
            }?;
            if loaded != expected {
                return Err(format!(
                    "The file read as model library '{loaded}', not '{expected}'."
                ));
            }
            Ok(candidate)
        },
    );
    cx.state.dependencies.read = Some(reading);
}

/// Take a model library read once it is in: the library as its files now
/// say, as one step of Undo, or what stopped it.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn take_model_library_read(cx: &mut Cx<'_, ProjectFeature>) {
    let Some(answer) = cx
        .state
        .dependencies
        .read
        .as_ref()
        .and_then(crate::workbench::app::dialogs::project::ModelLibraryRead::take_answer)
    else {
        return;
    };
    let Some(read) = cx.state.dependencies.read.take() else {
        return;
    };
    let name = read.library;
    let failure = if read.relink {
        "Model library not relinked"
    } else {
        "Model library not reloaded"
    };
    if cx.model.workspace.content.project.revision().get() != read.project_revision {
        cx.model.report_failure(
            failure,
            format!("The project changed while '{name}' was read; nothing was changed."),
        );
        return;
    }
    let candidate = match answer {
        Ok(candidate) => candidate,
        Err(error) => {
            cx.model.report_failure(failure, error);
            return;
        }
    };
    let unchanged = cx
        .model
        .model_library_manager
        .get_library(&name)
        .zip(candidate.get_library(&name))
        .is_some_and(|(before, after)| {
            before.root_path == after.root_path
                && before.source_closure == after.source_closure
                && before.source_authority == after.source_authority
        });
    if unchanged {
        cx.model.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!("Model library '{name}' already matches its files."),
        );
        return;
    }
    let verb = if read.relink { "Relinked" } else { "Reloaded" };
    let result = if cx.platform.project_lifecycle.is_open() {
        crate::workbench::app_state::design_history::publish_model_library_candidate(
            cx.model,
            candidate,
            &name,
            format!("{} model library {name}", verb.to_lowercase()),
        )
        .map(|revision| {
            format!(
                "{verb} model library '{name}' from {} at project revision {}.",
                read.file,
                revision.get()
            )
        })
    } else {
        cx.model.model_library_manager = candidate;
        Ok(format!("{verb} model library '{name}' from {}.", read.file))
    };
    match result {
        Ok(message) => {
            cx.model
                .record(LogSeverity::Info, LogSource::Studio, message);
        }
        Err(error) => cx.model.report_failure(failure, error),
    }
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

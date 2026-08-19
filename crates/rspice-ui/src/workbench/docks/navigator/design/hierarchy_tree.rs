//! The design navigator's hierarchy: the masters a design declares, and the
//! occurrences that instantiate them.
//!
//! Both groups read the frozen [`DesignProjection`]. The execution plan it
//! carries is the one authority on which occurrence exists and what each of
//! them binds to, so the tree never re-resolves the design and never answers
//! from the editor's own buffers.
//!
//! Children are built only under a node that is unfolded, so a design of a
//! million instances costs one row per *visible* row. A typed filter is the
//! exception, because it asks about the whole design rather than about the
//! part on screen; it walks, and [`TREE_ROW_BUDGET`] is where it stops.
//!
//! An occurrence whose master already stands on its own ancestry is a leaf
//! that says so. That is the resolver's own cycle rule, and it is why such an
//! occurrence carries no binding: there is nothing below it to descend into.

use std::collections::BTreeMap;
use std::sync::Arc;

use egui::{Response, Ui};

use crate::state::workspace::{
    ConfigurationExecutionBinding, ConfigurationExecutionPlan, DEFAULT_SCHEMATIC_VIEW,
    DesignProjection,
};
use crate::state::{
    CellViewRef, ComponentType, DesignManagementCatalog, InstancePath, LibraryCellInstance,
    SchematicState, SheetId,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;
use crate::workbench::state::{NavigatorTreeNode, NavigatorTreeState};

use super::super::{SCHEMATIC_NAV_LABEL_SIZE, SCHEMATIC_NAV_META_SIZE, SCHEMATIC_NAV_ROW_HEIGHT};
use super::{
    NavigatorObject, WorkbenchIcon, empty_navigator_row, matches_query,
    navigator_object_context_menu, navigator_section_header, normalized,
};

/// Rows the tree paints before it stops and says that it stopped.
///
/// The expanded hierarchy is bounded by the resolver at a million instances; a
/// navigator column is not a place to put a million rows. The cap is a
/// rendering decision, and the row that states it is part of the contract.
const TREE_ROW_BUDGET: usize = 500;

/// Horizontal offset one nesting level adds, matching the navigator's own
/// indented rows.
const TREE_LEVEL_INDENT: f32 = 14.0;

// -----------------------------------------------------------------------------
// Model
// -----------------------------------------------------------------------------

/// What the tree reads: one frozen projection, the project's sheet catalogs,
/// and the filter the navigator is holding.
pub(super) struct TreeSource<'a> {
    pub projection: &'a DesignProjection,
    pub sheets: &'a DesignManagementCatalog,
    /// Folded filter text, empty when the navigator is unfiltered.
    pub query: &'a str,
}

/// How far one occurrence can be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OccurrenceState {
    /// Bound, with nothing below it.
    Leaf,
    /// Bound, with children this frame did not build.
    Collapsed,
    /// Bound, with its children in the rows that follow it.
    Expanded,
    /// Its master already stands on its own ancestry, so the design stops
    /// here and so does the tree.
    Recursive,
    /// The design places the instance and the plan bound nothing there.
    Unbound,
}

impl OccurrenceState {
    /// Whether descending into this occurrence would land anywhere.
    pub(super) const fn is_bound(self) -> bool {
        matches!(self, Self::Leaf | Self::Collapsed | Self::Expanded)
    }

    /// What the row says about itself when that is a condition rather than
    /// the master it stands on.
    const fn note(self) -> Option<&'static str> {
        match self {
            Self::Recursive => Some("recursive"),
            Self::Unbound => Some("unbound"),
            Self::Leaf | Self::Collapsed | Self::Expanded => None,
        }
    }
}

/// One occurrence, as the tree paints it.
pub(super) struct OccurrenceRow {
    pub path: InstancePath,
    /// The instance stepped through to reach it; the root cell at the top.
    pub label: String,
    pub master: CellViewRef,
    pub state: OccurrenceState,
}

/// One row of the occurrence tree, in paint order.
pub(super) enum DesignTreeRow {
    Occurrence(OccurrenceRow),
    /// One sheet of the occurrence above it, offered only where its cell view
    /// has more than one — a single-sheet cell view is its sheet.
    Sheet {
        occurrence: InstancePath,
        id: SheetId,
        name: String,
        depth: usize,
    },
    /// The walk reached [`TREE_ROW_BUDGET`] and stopped.
    Truncated,
}

/// The occurrence tree, from the design root down to whatever is unfolded.
pub(super) fn occurrence_rows(
    source: &TreeSource<'_>,
    tree: &NavigatorTreeState,
) -> Vec<DesignTreeRow> {
    let Some(plan) = source.projection.plan() else {
        return Vec::new();
    };
    let root = InstancePath::root();
    let Some(binding) = plan.binding(&root) else {
        return Vec::new();
    };
    let walk = Walk { source, plan, tree };
    let mut ancestry = Vec::new();
    let mut budget = TREE_ROW_BUDGET;
    let mut rows = walk
        .visit(&root, binding, &mut ancestry, &mut budget)
        .unwrap_or_default();
    if budget == 0 {
        rows.push(DesignTreeRow::Truncated);
    }
    rows
}

/// One walk of the occurrence tree, against one reading position.
struct Walk<'a> {
    source: &'a TreeSource<'a>,
    plan: &'a ConfigurationExecutionPlan,
    tree: &'a NavigatorTreeState,
}

impl Walk<'_> {
    /// One occurrence and everything unfolded beneath it, or `None` when a
    /// filter is active and neither it nor anything below it answers to it.
    fn visit(
        &self,
        path: &InstancePath,
        binding: &ConfigurationExecutionBinding,
        ancestry: &mut Vec<String>,
        budget: &mut usize,
    ) -> Option<Vec<DesignTreeRow>> {
        if *budget == 0 {
            return None;
        }
        let filtering = !self.source.query.is_empty();
        let master = binding.resolved_reference().clone();
        let label = path
            .segments()
            .last()
            .cloned()
            .unwrap_or_else(|| master.cell.clone());
        let expandable =
            !binding.stop_boundary() && places_instances(self.source.projection, &master);
        // A filter asks about the whole design, so it unfolds what it finds
        // instead of waiting to be unfolded.
        let unfold = expandable
            && (filtering
                || self
                    .tree
                    .is_expanded(&NavigatorTreeNode::Occurrence(path.fold_key())));
        let mut matched = !filtering
            || matches_query(self.source.query, &[label.as_str(), &master.display_path()]);
        let mut children = Vec::new();
        if unfold {
            matched |= self.visit_children(path, &master, ancestry, budget, &mut children);
        }

        if !matched {
            return None;
        }
        *budget = budget.saturating_sub(1);
        let state = if !expandable {
            OccurrenceState::Leaf
        } else if unfold {
            OccurrenceState::Expanded
        } else {
            OccurrenceState::Collapsed
        };
        let mut rows = Vec::with_capacity(children.len() + 1);
        rows.push(DesignTreeRow::Occurrence(OccurrenceRow {
            path: path.clone(),
            label,
            master,
            state,
        }));
        rows.append(&mut children);
        Some(rows)
    }

    /// The sheets and the instances one unfolded occurrence owns. Returns
    /// whether anything below answered to the filter.
    fn visit_children(
        &self,
        path: &InstancePath,
        master: &CellViewRef,
        ancestry: &mut Vec<String>,
        budget: &mut usize,
        rows: &mut Vec<DesignTreeRow>,
    ) -> bool {
        let filtering = !self.source.query.is_empty();
        let mut matched = false;
        for (id, name) in sheets_of(self.source.sheets, master) {
            if filtering && !matches_query(self.source.query, &[name.as_str()]) {
                continue;
            }
            matched = true;
            *budget = budget.saturating_sub(1);
            rows.push(DesignTreeRow::Sheet {
                occurrence: path.clone(),
                id,
                name,
                depth: path.depth() + 1,
            });
        }
        ancestry.push(identity(master));
        for placed in placed_instances(self.source.projection, master) {
            let Ok(child) = path.child(&placed.name) else {
                continue;
            };
            if let Some(binding) = self.plan.binding(&child) {
                if let Some(mut subtree) = self.visit(&child, binding, ancestry, budget) {
                    matched = true;
                    rows.append(&mut subtree);
                }
                continue;
            }
            if filtering
                && !matches_query(
                    self.source.query,
                    &[placed.name.as_str(), &placed.requested.display_path()],
                )
            {
                continue;
            }
            matched = true;
            *budget = budget.saturating_sub(1);
            rows.push(DesignTreeRow::Occurrence(OccurrenceRow {
                state: if ancestry.contains(&identity(&placed.requested)) {
                    OccurrenceState::Recursive
                } else {
                    OccurrenceState::Unbound
                },
                path: child,
                label: placed.name,
                master: placed.requested,
            }));
        }
        ancestry.pop();
        matched
    }
}

/// One instance the design places under a master.
struct PlacedInstance {
    name: String,
    requested: CellViewRef,
}

/// The instances one master places, in the order the design placed them.
///
/// This is the set the hierarchy resolver descends — a cell instance carrying
/// a library binding — read from the projection's materialized buffer.
fn placed_instances(projection: &DesignProjection, master: &CellViewRef) -> Vec<PlacedInstance> {
    materialized(projection, master).map_or_else(Vec::new, |schematic| {
        schematic
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .filter_map(|component| {
                component
                    .library_cell
                    .as_ref()
                    .map(|binding| PlacedInstance {
                        name: component.name.clone(),
                        requested: requested_reference(binding),
                    })
            })
            .collect()
    })
}

/// Whether a master places anything at all. This is what decides a disclosure
/// control, so it is asked without building the list a folded node will never
/// show.
fn places_instances(projection: &DesignProjection, master: &CellViewRef) -> bool {
    materialized(projection, master).is_some_and(|schematic| {
        schematic.components.iter().any(|component| {
            component.kind == ComponentType::CellInstance && component.library_cell.is_some()
        })
    })
}

/// The cell view one placed binding asks for, with the resolver's own symbol
/// substitution applied: placing a symbol requests the schematic behind it.
fn requested_reference(binding: &LibraryCellInstance) -> CellViewRef {
    let view = if binding.view.eq_ignore_ascii_case("symbol") {
        DEFAULT_SCHEMATIC_VIEW
    } else {
        binding.view.as_str()
    };
    CellViewRef::new(&binding.library, &binding.cell, view)
}

/// Two references name one master exactly when their folded cell-view keys
/// agree, which is the comparison the resolver closes its cycles with.
fn identity(reference: &CellViewRef) -> String {
    reference.key().to_ascii_lowercase()
}

/// The projection's materialized buffer for one cell view. The projection
/// keys its buffers in the design's own spelling, so the lookup folds case
/// rather than assuming one.
fn materialized<'a>(
    projection: &'a DesignProjection,
    reference: &CellViewRef,
) -> Option<&'a SchematicState> {
    let key = reference.key();
    projection
        .schematic_buffers()
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(&key))
        .map(|(_, schematic)| schematic)
}

/// The sheets of one cell view, in page order, and only where there is more
/// than one: a single-sheet cell view *is* its sheet, and a node offering to
/// enter what is already on screen is a row that does nothing.
pub(super) fn sheets_of(
    catalog: &DesignManagementCatalog,
    master: &CellViewRef,
) -> Vec<(SheetId, String)> {
    let Some(sheets) = catalog.sheet_catalog(&master.key()) else {
        return Vec::new();
    };
    if sheets.sheets().len() < 2 {
        return Vec::new();
    }
    sheets
        .sheets()
        .iter()
        .map(|sheet| (sheet.id(), sheet.name().to_owned()))
        .collect()
}

/// One master row: a cell view the plan binds, and how many occurrences reach
/// it.
struct MasterRow {
    view: String,
    occurrences: usize,
}

/// Every master the plan declares, grouped library, then cell, then view.
///
/// The plan is asked rather than the library, because a cell view nothing
/// instantiates is a library entry and not a master of this design.
fn declared_masters(
    projection: &DesignProjection,
    query: &str,
) -> BTreeMap<String, BTreeMap<String, Vec<MasterRow>>> {
    let Some(plan) = projection.plan() else {
        return BTreeMap::new();
    };
    let mut counted: BTreeMap<String, BTreeMap<String, BTreeMap<String, usize>>> = BTreeMap::new();
    for binding in plan.bindings() {
        let reference = binding.resolved_reference();
        if !matches_query(
            query,
            &[
                reference.library.as_str(),
                reference.cell.as_str(),
                reference.view.as_str(),
            ],
        ) {
            continue;
        }
        *counted
            .entry(reference.library.clone())
            .or_default()
            .entry(reference.cell.clone())
            .or_default()
            .entry(reference.view.clone())
            .or_default() += 1;
    }
    counted
        .into_iter()
        .map(|(library, cells)| {
            let cells = cells
                .into_iter()
                .map(|(cell, views)| {
                    let views = views
                        .into_iter()
                        .map(|(view, occurrences)| MasterRow { view, occurrences })
                        .collect();
                    (cell, views)
                })
                .collect();
            (library, cells)
        })
        .collect()
}

// -----------------------------------------------------------------------------
// Rendering
// -----------------------------------------------------------------------------

/// One tree row's paint request.
struct TreeRow<'a> {
    id: egui::Id,
    level: usize,
    /// `Some(unfolded)` when the node has children to disclose.
    disclosure: Option<bool>,
    icon: WorkbenchIcon,
    label: &'a str,
    meta: Option<&'a str>,
    /// The meta states a condition rather than a name or a count, so it is
    /// painted as one.
    alert: bool,
    selected: bool,
}

/// What the reader did to one row.
struct TreeRowResponse {
    row: Response,
    /// Present exactly when the row disclosed children.
    disclosure: Option<Response>,
}

/// Paint one tree row, with its disclosure control as its own hit target.
///
/// The two gestures mean different things — unfolding a node reads it,
/// activating it navigates to it — and a tree that conflates them cannot show
/// a subtree without leaving the sheet the reader is on.
fn tree_row(ui: &mut Ui, row: TreeRow<'_>) -> TreeRowResponse {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), SCHEMATIC_NAV_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            row.selected,
            row.label,
        )
    });
    if let Some(unfolded) = row.disclosure {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_expanded(unfolded);
        });
    }
    if row.selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if row.selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if row.selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }

    let indent = TREE_LEVEL_INDENT * row.level as f32;
    let disclosure = row.disclosure.map(|unfolded| {
        let caret = egui::Rect::from_center_size(
            egui::pos2(rect.left() + 14.0 + indent, rect.center().y),
            egui::Vec2::splat(SCHEMATIC_NAV_ROW_HEIGHT),
        );
        let response = ui.interact(caret, row.id.with("disclosure"), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                unfolded,
                if unfolded { "Collapse" } else { "Expand" },
            )
        });
        let glyph = if unfolded {
            WorkbenchIcon::ChevronDown
        } else {
            WorkbenchIcon::ChevronRight
        };
        glyph.paint(
            ui.painter(),
            egui::Rect::from_center_size(caret.center(), egui::Vec2::splat(9.0)),
            if response.hovered() {
                t.color.text
            } else {
                t.color.text_faint
            },
        );
        theme::paint_focus_ring(ui, &response, caret);
        response
    });

    row.icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 33.5 + indent, rect.center().y),
            egui::Vec2::splat(15.0),
        ),
        if row.selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let meta_color = if row.alert {
        t.color.warn
    } else {
        t.color.text_faint
    };
    let meta_width = row.meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(
                meta.to_owned(),
                theme::mono(SCHEMATIC_NAV_META_SIZE, FontWeight::Regular),
                meta_color,
            )
            .size()
            .x
    });
    let label_left = rect.left() + 47.0 + indent;
    let label_right = if row.meta.is_some() {
        rect.right() - 14.0 - meta_width
    } else {
        rect.right() - 8.0
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            row.label,
            theme::sans(SCHEMATIC_NAV_LABEL_SIZE, FontWeight::Regular),
            if row.selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    if let Some(meta) = row.meta {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            theme::mono(SCHEMATIC_NAV_META_SIZE, FontWeight::Regular),
            meta_color,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    TreeRowResponse {
        row: response,
        disclosure,
    }
}

/// The Masters group: every cell view this design binds, library then cell
/// then view, with the number of occurrences that reach each one.
pub(super) fn masters_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(&app.state.workbench.navigator_query);
    let Some(projection) = resolved_projection(ui, app, "Masters") else {
        return;
    };
    let masters = declared_masters(&projection, &query);
    let total = masters
        .values()
        .flat_map(|cells| cells.values())
        .map(|views| views.len())
        .sum::<usize>();
    navigator_section_header(ui, "Masters", &total.to_string());
    if masters.is_empty() {
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "This design binds no cell view"
            } else {
                "No master matches this filter"
            },
        );
        return;
    }

    let workspace = app.state.workbench.workspace;
    let mut tree = std::mem::take(app.state.workbench.navigator_trees.for_workspace(workspace));
    let mut open = None;
    for (library, cells) in &masters {
        let library_node = NavigatorTreeNode::Master(library.to_ascii_lowercase());
        let unfolded = !query.is_empty() || tree.is_expanded(&library_node);
        let cell_count = cells.len().to_string();
        let response = tree_row(
            ui,
            TreeRow {
                id: ui.id().with(("navigator-master-library", library)),
                level: 0,
                disclosure: Some(unfolded),
                icon: WorkbenchIcon::Models,
                label: library.as_str(),
                meta: Some(cell_count.as_str()),
                alert: false,
                selected: false,
            },
        );
        if response.row.clicked() || response.disclosure.is_some_and(|caret| caret.clicked()) {
            tree.toggle(library_node);
        }
        if !unfolded {
            continue;
        }
        for (cell, views) in cells {
            let key = format!("{library}/{cell}").to_ascii_lowercase();
            let cell_node = NavigatorTreeNode::Master(key.clone());
            let unfolded = !query.is_empty() || tree.is_expanded(&cell_node);
            let view_count = views.len().to_string();
            let response = tree_row(
                ui,
                TreeRow {
                    id: ui.id().with(("navigator-master-cell", &key)),
                    level: 1,
                    disclosure: Some(unfolded),
                    icon: WorkbenchIcon::Design,
                    label: cell.as_str(),
                    meta: Some(view_count.as_str()),
                    alert: false,
                    selected: false,
                },
            );
            if response.row.clicked() || response.disclosure.is_some_and(|caret| caret.clicked()) {
                tree.toggle(cell_node);
            }
            if !unfolded {
                continue;
            }
            for master in views {
                let reference = CellViewRef::new(library, cell, &master.view);
                let occurrences = master.occurrences.to_string();
                let response = tree_row(
                    ui,
                    TreeRow {
                        id: ui.id().with(("navigator-master-view", reference.key())),
                        level: 2,
                        disclosure: None,
                        icon: WorkbenchIcon::Layers,
                        label: master.view.as_str(),
                        meta: Some(occurrences.as_str()),
                        alert: false,
                        selected: app.state.workspace.active_view == reference,
                    },
                );
                if response
                    .row
                    .on_hover_text(format!(
                        "Open {}, instantiated {} time(s) in this design",
                        reference.display_path(),
                        master.occurrences
                    ))
                    .clicked()
                {
                    open = Some(reference);
                }
            }
        }
    }
    *app.state.workbench.navigator_trees.for_workspace(workspace) = tree;
    if let Some(reference) = open {
        app.state.open_workspace_view(reference);
    }
}

/// The Occurrences group: the design root and everything unfolded below it.
pub(super) fn occurrences_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(&app.state.workbench.navigator_query);
    let Some(projection) = resolved_projection(ui, app, "Occurrences") else {
        return;
    };
    let workspace = app.state.workbench.workspace;
    let mut tree = std::mem::take(app.state.workbench.navigator_trees.for_workspace(workspace));
    let rows = occurrence_rows(
        &TreeSource {
            projection: projection.as_ref(),
            sheets: &app.state.workspace.design_management,
            query: query.as_str(),
        },
        &tree,
    );
    let occurrences = rows
        .iter()
        .filter(|row| matches!(row, DesignTreeRow::Occurrence(_)))
        .count();

    navigator_section_header(ui, "Occurrences", &occurrences.to_string());
    if rows.is_empty() {
        *app.state.workbench.navigator_trees.for_workspace(workspace) = tree;
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "The design root binds no executable view"
            } else {
                "No occurrence matches this filter"
            },
        );
        return;
    }

    let active = app.state.workspace.occurrence_path();
    let mut descend = None;
    let mut enter_sheet = None;
    for entry in &rows {
        match entry {
            DesignTreeRow::Occurrence(row) => {
                let note = row
                    .state
                    .note()
                    .map_or_else(|| row.master.cell.clone(), str::to_owned);
                let response = tree_row(
                    ui,
                    TreeRow {
                        id: ui.id().with(("navigator-occurrence", row.path.fold_key())),
                        level: row.path.depth(),
                        disclosure: matches!(
                            row.state,
                            OccurrenceState::Collapsed | OccurrenceState::Expanded
                        )
                        .then_some(row.state == OccurrenceState::Expanded),
                        icon: if row.state == OccurrenceState::Recursive {
                            WorkbenchIcon::Refresh
                        } else {
                            WorkbenchIcon::Instance
                        },
                        label: row.label.as_str(),
                        meta: Some(note.as_str()),
                        alert: row.state.note().is_some(),
                        selected: tree.selection() == Some(&row.path) || row.path == active,
                    },
                );
                if response.disclosure.is_some_and(|caret| caret.clicked()) {
                    tree.toggle(NavigatorTreeNode::Occurrence(row.path.fold_key()));
                } else if response.row.clicked() && row.state.is_bound() {
                    tree.select(row.path.clone());
                    descend = Some(row.path.clone());
                }
                occurrence_context_menu(&response.row, app, row);
            }
            DesignTreeRow::Sheet {
                occurrence,
                id,
                name,
                depth,
            } => {
                let response = tree_row(
                    ui,
                    TreeRow {
                        id: ui
                            .id()
                            .with(("navigator-sheet", occurrence.fold_key(), *id)),
                        level: *depth,
                        disclosure: None,
                        icon: WorkbenchIcon::File,
                        label: name.as_str(),
                        meta: Some("sheet"),
                        alert: false,
                        selected: false,
                    },
                );
                if response.row.clicked() {
                    enter_sheet = Some((occurrence.clone(), *id));
                }
            }
            DesignTreeRow::Truncated => {
                empty_navigator_row(
                    ui,
                    "This hierarchy is longer than the navigator lists; filter it to reach the rest",
                );
            }
        }
    }
    *app.state.workbench.navigator_trees.for_workspace(workspace) = tree;

    if let Some((occurrence, sheet)) = enter_sheet {
        open_occurrence(app, &occurrence);
        let _ = crate::workbench::app::sheets::activate_sheet(&mut app.state, sheet);
    } else if let Some(path) = descend {
        open_occurrence(app, &path);
    }
}

/// The design projection, or the reason there is none, stated in the
/// section's own place. A tree built from the editor's buffers would offer
/// occurrences the run has no binding for.
fn resolved_projection(
    ui: &mut Ui,
    app: &RSpiceApp,
    section: &str,
) -> Option<Arc<DesignProjection>> {
    match app.state.workspace.design_projection(
        &app.state.library_manager,
        &app.state.workspace.active_view,
        &app.state.schematic,
    ) {
        Ok(projection) => Some(projection),
        Err(error) => {
            navigator_section_header(ui, section, "\u{2014}");
            empty_navigator_row(ui, &error.to_string());
            None
        }
    }
}

/// The context menu of the object one occurrence row stands for.
///
/// Those commands act on a selected object in the sheet on screen, so the menu
/// is offered exactly where there is one: on a child of the occurrence being
/// edited. A row deeper in the tree names an instance none of them can reach.
fn occurrence_context_menu(response: &Response, app: &mut RSpiceApp, row: &OccurrenceRow) {
    if row.path.parent().as_ref() != Some(&app.state.workspace.occurrence_path()) {
        return;
    }
    let Some(object) = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.name.eq_ignore_ascii_case(&row.label))
        .map(|component| NavigatorObject::Component {
            id: component.id,
            label: component.name.clone(),
            position: component.pos,
        })
    else {
        return;
    };
    navigator_object_context_menu(response, app, object);
}

/// Land the session on one occurrence, through the descent the breadcrumb
/// uses.
///
/// Every master is read from the plan before anything moves, so a path the
/// projection cannot spell end to end navigates nowhere rather than part of
/// the way. Ascending to the shared prefix and descending only the rest is
/// what keeps a click on a sibling from re-entering every level it already
/// stands on.
pub(super) fn open_occurrence(app: &mut RSpiceApp, path: &InstancePath) {
    let Ok(projection) = app.state.workspace.design_projection(
        &app.state.library_manager,
        &app.state.workspace.active_view,
        &app.state.schematic,
    ) else {
        return;
    };
    let Some(plan) = projection.plan() else {
        return;
    };
    let Some(root) = plan
        .binding(&InstancePath::root())
        .map(|binding| binding.resolved_reference().clone())
    else {
        return;
    };
    let mut steps = Vec::with_capacity(path.depth());
    let mut prefix = InstancePath::root();
    for segment in path.segments() {
        let Ok(next) = prefix.child(segment) else {
            return;
        };
        let Some(binding) = plan.binding(&next) else {
            return;
        };
        steps.push((segment.clone(), binding.resolved_reference().clone()));
        prefix = next;
    }
    drop(projection);

    let rooted_here = app
        .state
        .workspace
        .active_occurrence()
        .is_some_and(|occurrence| occurrence.root == root);
    let current = app.state.workspace.occurrence_path();
    let shared = if rooted_here {
        shared_prefix(&current, path)
    } else {
        app.state.descend_into_instance(None, root);
        0
    };
    if rooted_here && shared < current.depth() {
        app.state.focus_workspace_breadcrumb(shared);
    }
    for (instance, master) in steps.into_iter().skip(shared) {
        app.state.descend_into_instance(Some(instance), master);
    }
}

/// How many leading instances two occurrences agree on.
fn shared_prefix(current: &InstancePath, target: &InstancePath) -> usize {
    let mut shared = InstancePath::root();
    for segment in target.segments() {
        let Ok(next) = shared.child(segment) else {
            break;
        };
        if !current.starts_with(&next) {
            break;
        }
        shared = next;
    }
    shared.depth()
}

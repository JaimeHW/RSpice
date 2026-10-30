//! The tables under the global bindings. Instances lists every instance of
//! the expanded design as a tree: what the shown configuration resolved it
//! to, which list chose that, and the view it binds it to; pattern bindings
//! sit above it, since one names many instances. Cells lists each
//! library/cell once, with the cell binding every instance of it follows.

use super::*;

use rspice_design::hierarchy::{
    HierarchyBindingRule, HierarchyBindingStatus, ResolvedHierarchyBinding,
};

use crate::state::{ConfigurationPlatform, ConfigurationSetDefinition, ConfigurationSetOverride};
use crate::ui::widgets::{SegmentedWidth, segmented, select};
use crate::workbench::project::configuration_state::{
    ConfigurationRow, ConfigurationTab, InstanceRow, cell_view_names, governing_binding,
    instance_binding, is_pattern, runs_on_label, set_cell_view, set_instance_view,
};

/// Below this width the table scrolls sideways instead of squeezing.
const TABLE_MIN_WIDTH: f32 = 760.0;
const INDENT: f32 = 14.0;
const CHEVRON: f32 = 14.0;
const INHERITED: &str = "Inherited";

/// The columns, by share of the table's width.
const COLUMNS: [(&str, f32); 8] = [
    ("Instance", 0.16),
    ("Cell", 0.13),
    ("View found", 0.10),
    ("View to use", 0.13),
    ("Bound by", 0.14),
    ("Model section", 0.12),
    ("Runs on", 0.07),
    ("Status", 0.15),
];

/// What the reader did to the tables this frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TableEdit {
    Tab(ConfigurationTab),
    Select(ConfigurationRow),
    Toggle(String),
    View {
        path: String,
        view: Option<String>,
    },
    CellView {
        library: String,
        cell: String,
        view: Option<String>,
    },
    AddPattern,
}

/// Which list chose a row's view, in the page's words.
fn rule_label(rule: HierarchyBindingRule) -> &'static str {
    // Spelled from the crate root: the variant names are also item names in
    // other features, and the full path says which these are.
    match rule {
        rspice_design::hierarchy::HierarchyBindingRule::Default => "Default",
        rspice_design::hierarchy::HierarchyBindingRule::ViewList => "View list",
        rspice_design::hierarchy::HierarchyBindingRule::CellBinding => "Cell binding",
        rspice_design::hierarchy::HierarchyBindingRule::InstanceBinding => "Instance binding",
    }
}

pub(super) fn tables(
    ui: &mut Ui,
    state: &mut AppState,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
) {
    let visible = state.project.configuration.visible_instances();
    let catalog = &state.model.workspace.content.configuration_sets;
    let id = match selection {
        ConfigurationSelection::Configuration(id) => Some(id),
        ConfigurationSelection::Default => None,
    };
    let definition = id.and_then(|id| state.project.configuration.definition(catalog, id));
    let editable = definition.is_some() && !state.model.safe_mode.project_read_only();
    let page = &state.project.configuration;
    let tab = page.tab;
    let mut edit = tab_header(ui, resolution, tab, editable);
    let Ok(hierarchy) = &resolution.hierarchy else {
        apply_table_edit(state, id, edit);
        return;
    };
    if tab == ConfigurationTab::Cells {
        if let Some(cell_edit) = super::cells::cells_table(
            ui,
            &resolution.cells,
            definition,
            &state.model.library_manager,
            page.row.as_ref(),
            editable,
        ) {
            edit = Some(cell_edit);
        }
        apply_table_edit(state, id, edit);
        return;
    }
    if let Some(definition) = definition
        && let Some(pattern_edit) = pattern_bindings(ui, definition, page.row.as_ref())
    {
        edit = Some(pattern_edit);
    }
    if let Some(table_edit) = instance_tree(
        ui,
        TreeInput {
            hierarchy: &hierarchy.bindings,
            rows: &resolution.instances,
            visible: &visible,
            definition,
            libraries: &state.model.library_manager,
            selected: page.row.as_ref(),
            collapsed: &|path| page.is_collapsed(path),
            editable,
        },
    ) {
        edit = Some(table_edit);
    }
    apply_table_edit(state, id, edit);
}

fn apply_table_edit(
    state: &mut AppState,
    id: Option<crate::state::ConfigurationSetId>,
    edit: Option<TableEdit>,
) {
    let Some(edit) = edit else {
        return;
    };
    match edit {
        TableEdit::Tab(tab) => {
            state.project.configuration.tab = tab;
            state.project.configuration.row = None;
        }
        TableEdit::Select(row) => state.project.configuration.row = Some(row),
        TableEdit::CellView {
            library,
            cell,
            view,
        } => {
            if let Some(id) = id {
                state.project.configuration.edit(
                    &state.model.workspace.content.configuration_sets,
                    id,
                    |definition| set_cell_view(definition, &library, &cell, view.as_deref()),
                );
            }
        }
        TableEdit::Toggle(path) => state.project.configuration.toggle_collapsed(&path),
        TableEdit::View { path, view } => {
            if let Some(id) = id {
                state.project.configuration.edit(
                    &state.model.workspace.content.configuration_sets,
                    id,
                    |definition| set_instance_view(definition, &path, view.as_deref()),
                );
            }
        }
        TableEdit::AddPattern => {
            if let Some(id) = id {
                let mut added = None;
                state.project.configuration.edit(
                    &state.model.workspace.content.configuration_sets,
                    id,
                    |definition| added = Some(add_pattern_binding(definition)),
                );
                state.project.configuration.row = added.map(ConfigurationRow::Pattern);
            }
        }
    }
}

/// The band over the tables: which table, how many rows it holds, and on
/// Instances the one action that adds to it.
fn tab_header(
    ui: &mut Ui,
    resolution: &ConfigurationResolution,
    tab: ConfigurationTab,
    editable: bool,
) -> Option<TableEdit> {
    let t = Tokens::get(ui.ctx());
    let meta = match tab {
        ConfigurationTab::Instances => {
            let count = resolution.instances.len();
            format!("{count} instance{}", plural_suffix(count))
        }
        ConfigurationTab::Cells => {
            let count = resolution.cells.len();
            format!("{count} cell{}", plural_suffix(count))
        }
    };
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width().max(1.0), SECTION_HEADER_HEIGHT + 4.0),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    let mut edit = None;
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(8.0, 3.0)))
            .layout(Layout::left_to_right(Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            let mut index = match tab {
                ConfigurationTab::Instances => 0,
                ConfigurationTab::Cells => 1,
            };
            if segmented(
                ui,
                "configuration-tables",
                &["Instances", "Cells"],
                &mut index,
                SegmentedWidth::Natural,
            ) {
                edit = Some(TableEdit::Tab(if index == 0 {
                    ConfigurationTab::Instances
                } else {
                    ConfigurationTab::Cells
                }));
            }
            ui.label(
                egui::RichText::new(&meta)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            if tab == ConfigurationTab::Instances && editable {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if Button::new("Add pattern binding").show(ui).clicked() {
                        edit = Some(TableEdit::AddPattern);
                    }
                });
            }
        },
    );
    edit
}

/// Bind every instance a new pattern names, starting from the first scope
/// no binding uses yet, and return the pattern so the inspector can edit it.
fn add_pattern_binding(definition: &mut ConfigurationSetDefinition) -> String {
    let taken = |pattern: &str| {
        definition
            .overrides
            .iter()
            .any(|binding| binding.instance_path.eq_ignore_ascii_case(pattern))
    };
    let pattern = (1..)
        .map(|depth| format!("/{}", vec!["*"; depth].join("/")))
        .find(|pattern| !taken(pattern))
        .expect("pattern depth is unbounded");
    let views = definition
        .executable_view_policy
        .first()
        .cloned()
        .into_iter()
        .collect();
    definition.overrides.push(ConfigurationSetOverride {
        instance_path: pattern.clone(),
        executable_views: views,
        stop_view: None,
        model_section: None,
        eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
    });
    pattern
}

/// The pattern bindings, one row each. Their properties are edited in the
/// inspector.
fn pattern_bindings(
    ui: &mut Ui,
    definition: &ConfigurationSetDefinition,
    selected: Option<&ConfigurationRow>,
) -> Option<TableEdit> {
    let t = Tokens::get(ui.ctx());
    let row_height = Tokens::get(ui.ctx()).metrics.row_h;
    let mut edit = None;
    for binding in definition
        .overrides
        .iter()
        .filter(|binding| is_pattern(binding))
    {
        let is_selected = matches!(
            selected,
            Some(ConfigurationRow::Pattern(pattern)) if pattern == &binding.instance_path
        );
        let (rect, response) = ui.allocate_exact_size(
            vec2(ui.available_width().max(1.0), row_height),
            Sense::click(),
        );
        let views = binding.executable_views.join(", then ");
        let runs_on = runs_on_label(&binding.eligible_platforms);
        let section = binding.model_section.as_deref().unwrap_or("\u{2014}");
        let label = format!(
            "Pattern binding {}: view list {views}; model section {section}; runs on {runs_on}",
            binding.instance_path
        );
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                is_selected,
                &label,
            )
        });
        row_fill(ui, rect, &response, is_selected);
        theme::paint_focus_ring(ui, &response, rect);
        let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
        let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
        let cells = [
            (binding.instance_path.as_str(), 0.0, 0.22, t.color.text),
            (views.as_str(), 0.22, 0.41, t.color.text_dim),
            (section, 0.63, 0.12, t.color.text_dim),
            (runs_on, 0.75, 0.25, t.color.text_dim),
        ];
        for (text, start, share, color) in cells {
            let cell = Rect::from_min_size(
                pos2(rect.left() + rect.width() * start, rect.top()),
                vec2(rect.width() * share, rect.height()),
            );
            cell_text(ui, &painter, cell, text, &mono, color);
        }
        if response.clicked() {
            edit = Some(TableEdit::Select(ConfigurationRow::Pattern(
                binding.instance_path.clone(),
            )));
        }
    }
    edit
}

/// What the tree draws from, borrowed for one frame.
struct TreeInput<'a> {
    hierarchy: &'a [ResolvedHierarchyBinding],
    rows: &'a [InstanceRow],
    visible: &'a [usize],
    definition: Option<&'a ConfigurationSetDefinition>,
    libraries: &'a crate::state::LibraryManager,
    selected: Option<&'a ConfigurationRow>,
    collapsed: &'a dyn Fn(&str) -> bool,
    editable: bool,
}

fn instance_tree(ui: &mut Ui, input: TreeInput<'_>) -> Option<TableEdit> {
    let t = Tokens::get(ui.ctx());
    let row_height = t.metrics.row_h;
    let header_height = (row_height - 4.0).max(22.0);
    let visible_width = visible_workspace_width(ui);
    let width = visible_width.max(TABLE_MIN_WIDTH);
    let mut edit = None;
    ScrollArea::horizontal()
        .id_salt("workbench.project.configuration.instances.horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(width);
            header_row(ui, width, header_height);
            let height = visible_workspace_height(ui).max(row_height * 4.0);
            ScrollArea::vertical()
                .id_salt("workbench.project.configuration.instances")
                .auto_shrink([false, false])
                .max_height(height)
                .show_rows(ui, row_height, input.visible.len(), |ui, range| {
                    ui.set_min_width(width);
                    for &index in &input.visible[range] {
                        let row = &input.rows[index];
                        if let Some(row_edit) = instance_row(ui, &input, row, width, row_height) {
                            edit = Some(row_edit);
                        }
                    }
                });
        });
    edit
}

fn header_row(ui: &mut Ui, width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "Instance columns: {}",
                COLUMNS.map(|(name, _)| name).join(", ")
            ),
        )
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    for (index, (name, cell)) in column_rects(rect).into_iter().enumerate() {
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        cell_text(ui, ui.painter(), cell, name, &font, t.color.text_faint);
    }
}

/// Each column's name and its rect in `row`.
fn column_rects(row: Rect) -> [(&'static str, Rect); 8] {
    let mut left = row.left();
    COLUMNS.map(|(name, share)| {
        let right = (left + row.width() * share).min(row.right());
        let cell = Rect::from_min_max(pos2(left, row.top()), pos2(right, row.bottom()));
        left = right;
        (name, cell)
    })
}

fn row_fill(ui: &Ui, rect: Rect, response: &egui::Response, selected: bool) {
    let t = Tokens::get(ui.ctx());
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

/// `text` in `cell`, elided to fit rather than clipped mid-glyph.
fn cell_text(
    ui: &Ui,
    painter: &egui::Painter,
    cell: Rect,
    text: &str,
    font: &egui::FontId,
    color: Color32,
) {
    let inner = cell.shrink2(vec2(7.0, 0.0));
    let text = elide_text(ui, text, font, inner.width().max(1.0));
    painter.with_clip_rect(inner).text(
        inner.left_center(),
        Align2::LEFT_CENTER,
        text,
        font.clone(),
        color,
    );
}

/// The short reason a row is not bound, or `None` when it is.
fn unbound_reason(status: HierarchyBindingStatus) -> Option<&'static str> {
    match status {
        HierarchyBindingStatus::Resolved | HierarchyBindingStatus::Modified => None,
        HierarchyBindingStatus::Unresolved => Some("no view found"),
        HierarchyBindingStatus::Recursive => Some("instantiates itself"),
        HierarchyBindingStatus::DepthLimit => Some("hierarchy too deep"),
        HierarchyBindingStatus::InstanceLimit => Some("too many instances"),
    }
}

fn instance_row(
    ui: &mut Ui,
    input: &TreeInput<'_>,
    row: &InstanceRow,
    width: f32,
    height: f32,
) -> Option<TableEdit> {
    let t = Tokens::get(ui.ctx());
    let binding = &input.hierarchy[row.binding];
    let reference = &binding.reference;
    let is_selected = matches!(
        input.selected,
        Some(ConfigurationRow::Instance(path)) if path.eq_ignore_ascii_case(&row.path)
    );
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    let name = if row.depth == 0 {
        reference.cell.clone()
    } else {
        row.path.rsplit('/').next().unwrap_or(&row.path).to_owned()
    };
    let cell = format!("{}/{}", reference.library, reference.cell);
    let found = if binding.status.is_resolved() {
        reference.view.as_str()
    } else {
        "\u{2014}"
    };
    let exact = input
        .definition
        .and_then(|definition| instance_binding(definition, &row.path));
    let to_use = exact
        .and_then(|binding| binding.executable_views.first())
        .map_or(INHERITED, String::as_str);
    let rule = rule_label(binding.rule);
    let section = if binding.model_section.is_empty() {
        "\u{2014}"
    } else {
        binding.model_section.as_str()
    };
    let runs_on = input
        .definition
        .and_then(|definition| governing_binding(definition, &row.path))
        .map_or("All", |binding| runs_on_label(&binding.eligible_platforms));
    let (problem, status) = match unbound_reason(binding.status) {
        Some(reason) => (Some(Problem::Failure), reason),
        None => match binding.warnings.first() {
            Some(warning) => (Some(Problem::Warning), warning.as_str()),
            None => (None, ""),
        },
    };
    let mut label = format!(
        "Instance {}: cell {cell}; view found {found}; view to use {to_use}; \
         bound by {rule}; model section {section}; runs on {runs_on}",
        row.path
    );
    if let Some(problem) = problem {
        label = format!("{label}; {}", problem.announce(status));
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            is_selected,
            &label,
        )
    });
    row_fill(ui, rect, &response, is_selected);
    theme::paint_focus_ring(ui, &response, rect);
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let cells = column_rects(rect);
    for (_, cell) in cells.iter().skip(1) {
        painter.vline(
            cell.left(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    let mut edit = None;

    // Instance: indented by depth, with the expand control when it has
    // children.
    let instance = cells[0].1;
    let chevron_left = instance.left() + 4.0 + INDENT * row.depth as f32;
    if row.children > 0 {
        let chevron = Rect::from_min_size(
            pos2(chevron_left, rect.center().y - CHEVRON * 0.5),
            vec2(CHEVRON, CHEVRON),
        );
        let collapsed = (input.collapsed)(&row.path);
        let toggle = ui.interact(chevron, response.id.with("toggle"), Sense::click());
        toggle.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                true,
                if collapsed {
                    format!("Expand {}", row.path)
                } else {
                    format!("Collapse {}", row.path)
                },
            )
        });
        let icon = if collapsed {
            WorkbenchIcon::ChevronRight
        } else {
            WorkbenchIcon::ChevronDown
        };
        icon.paint(&painter, chevron.shrink(2.5), t.color.text_dim);
        theme::paint_focus_ring(ui, &toggle, chevron);
        let keyboard = response.has_focus()
            && ui.input_mut(|input| {
                let key = if collapsed {
                    egui::Key::ArrowRight
                } else {
                    egui::Key::ArrowLeft
                };
                input.consume_key(egui::Modifiers::NONE, key)
            });
        if toggle.clicked() || keyboard {
            edit = Some(TableEdit::Toggle(row.path.clone()));
        }
    }
    let name_cell = Rect::from_min_max(
        pos2(chevron_left + CHEVRON - 4.0, instance.top()),
        instance.max,
    );
    cell_text(ui, &painter, name_cell, &name, &mono, t.color.text);
    cell_text(ui, &painter, cells[1].1, &cell, &mono, t.color.text_dim);
    cell_text(ui, &painter, cells[2].1, found, &mono, t.color.text_dim);

    // View to use: a choice of the views the cell holds when the shown
    // configuration can be edited; a fact otherwise.
    let view_cell = cells[3].1;
    if input.editable {
        let mut options = vec![INHERITED.to_owned()];
        options.extend(cell_view_names(
            input.libraries,
            &reference.library,
            &reference.cell,
        ));
        let mut picked = None;
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(view_cell.shrink2(vec2(3.0, 2.0)))
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                picked = select(
                    ui,
                    &format!("configuration-view-to-use-{}", row.path),
                    &format!("View to use for {}", row.path),
                    to_use,
                    &options,
                    ui.available_width(),
                );
            },
        );
        if let Some(index) = picked
            && options[index] != to_use
        {
            edit = Some(TableEdit::View {
                path: row.path.clone(),
                view: (index > 0).then(|| options[index].clone()),
            });
        }
    } else {
        cell_text(ui, &painter, view_cell, to_use, &mono, t.color.text_dim);
    }
    cell_text(
        ui,
        &painter,
        cells[4].1,
        rule,
        &theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    cell_text(ui, &painter, cells[5].1, section, &mono, t.color.text_dim);
    cell_text(ui, &painter, cells[6].1, runs_on, &mono, t.color.text_dim);

    // Status: blank when bound; the mark and a short reason otherwise.
    let status_cell = cells[7].1;
    if let Some(problem) = problem {
        paint_problem_mark(
            &painter,
            Rect::from_center_size(
                pos2(status_cell.left() + 7.0 + MARK_SIDE * 0.5, rect.center().y),
                vec2(MARK_SIDE, MARK_SIDE),
            ),
            problem,
        );
        cell_text(
            ui,
            &painter,
            Rect::from_min_max(
                pos2(status_cell.left() + MARK_ADVANCE, status_cell.top()),
                status_cell.max,
            ),
            status,
            &theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    }
    let response = match (&binding.diagnostic, problem) {
        (Some(diagnostic), _) => response.on_hover_text(diagnostic),
        (None, Some(_)) => response.on_hover_text(status),
        (None, None) => response,
    };
    if response.clicked() && edit.is_none() {
        edit = Some(TableEdit::Select(ConfigurationRow::Instance(
            row.path.clone(),
        )));
    }
    edit
}

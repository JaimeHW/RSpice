//! The menu of the Configuration tables' rows, and the edits it makes. Each
//! item acts on every selected row at once as one edit of the configuration:
//! one change to the edits, which Apply commits as one step of Undo, and which
//! is refused whole, with the reason, when the configuration cannot hold it.

use super::*;

use crate::state::{
    CellViewRef, ConfigurationCellBinding, ConfigurationPlatform, ConfigurationSetDefinition,
    ConfigurationSetId, ConfigurationSetOverride, InstancePath, LibraryManager, ViewType,
};
use crate::workbench::project::configuration_state::{
    BindingChange, ConfigurationRow, CopiedBinding, INHERITED, apply_binding_change, cell_binding,
    instance_binding, set_cell_view, set_instance_view,
};

/// Where the open menu is remembered between frames.
const MENU_ID: &str = "workbench.project.configuration.row-menu";

/// What the menu offers, worked out once when it opens.
#[derive(Debug, Clone)]
struct RowMenu {
    at: egui::Pos2,
    /// The selected rows, the one the menu was asked on first.
    rows: Vec<MenuRow>,
    /// The view the first row is bound to, and its type.
    open: Option<(CellViewRef, ViewType)>,
    /// Each view some selected row can be set to, and how many can.
    choices: Vec<(String, usize)>,
    editable: bool,
    /// The first row has a binding of its own.
    copyable: bool,
    /// Some selected row has a binding of its own.
    removable: bool,
    /// What Copy binding put aside can go on some selected row.
    pastable: bool,
}

#[derive(Debug, Clone)]
struct MenuRow {
    row: ConfigurationRow,
    /// The views it can be set to.
    views: Vec<String>,
}

/// The item the reader took.
enum Pick {
    Open,
    SetView(Option<String>),
    Remove,
    Copy,
    Paste,
}

/// Open the menu for `opening`'s rows when the table asked for it, draw it
/// while it is open, and do what the reader takes from it.
pub(super) fn row_menu(
    ui: &Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    id: Option<ConfigurationSetId>,
    resolution: &ConfigurationResolution,
    editable: bool,
    opening: Option<(Vec<ConfigurationRow>, egui::Pos2)>,
) {
    let ctx = ui.ctx();
    let menu_id = egui::Id::new(MENU_ID);
    let opened = opening.is_some();
    if let Some((rows, at)) = opening {
        let menu = facts(cx, id, resolution, editable, rows, at);
        ctx.data_mut(|data| data.insert_temp(menu_id, menu));
    } else if !egui::Popup::is_id_open(ctx, menu_id) {
        return;
    }
    let Some(menu) = ctx.data(|data| data.get_temp::<RowMenu>(menu_id)) else {
        return;
    };
    let Some(pick) = show(ctx, menu_id, &menu, opened) else {
        return;
    };
    egui::Popup::close_id(ctx, menu_id);
    let rows = menu
        .rows
        .iter()
        .map(|row| row.row.clone())
        .collect::<Vec<_>>();
    match pick {
        Pick::Open => {
            if let Some((reference, view_type)) = menu.open {
                cx.wb
                    .outbox
                    .push(ProjectRequest::OpenView(reference, view_type));
            }
        }
        Pick::SetView(view) => {
            if let Some(id) = id {
                edit(cx, id, |definition| {
                    for row in &menu.rows {
                        let holds = view.as_deref().is_none_or(|view| {
                            row.views.iter().any(|held| held.eq_ignore_ascii_case(view))
                        });
                        if holds {
                            set_view(definition, &row.row, view.as_deref());
                        }
                    }
                });
            }
        }
        Pick::Remove => {
            if let Some(id) = id {
                remove_bindings(cx, id, &rows);
            }
        }
        Pick::Copy => {
            let catalog = &cx.model.workspace.content.configuration_sets;
            let copied = id
                .and_then(|id| cx.state.configuration.definition(catalog, id))
                .zip(rows.first())
                .and_then(|(definition, row)| copy(definition, row));
            if copied.is_some() {
                cx.state.configuration.copied_binding = copied;
            }
        }
        Pick::Paste => {
            if let (Some(id), Some(copied)) = (id, cx.state.configuration.copied_binding.clone()) {
                edit(cx, id, |definition| {
                    for row in &rows {
                        paste(definition, row, &copied);
                    }
                });
            }
        }
    }
}

/// Open the view `row` is bound to, if it is bound to one.
pub(super) fn open_view(
    cx: &mut Cx<'_, ProjectFeature>,
    resolution: &ConfigurationResolution,
    row: &ConfigurationRow,
) {
    if let Some((reference, view_type)) = view_to_open(&cx.model.library_manager, resolution, row) {
        cx.wb
            .outbox
            .push(ProjectRequest::OpenView(reference, view_type));
    }
}

/// Remove every binding `rows` hold of their own, as one edit: each
/// instance and cell goes back to what it inherits, and each pattern
/// binding goes.
pub(super) fn remove_bindings(
    cx: &mut Cx<'_, ProjectFeature>,
    id: ConfigurationSetId,
    rows: &[ConfigurationRow],
) {
    edit(cx, id, |definition| {
        for row in rows {
            match row {
                ConfigurationRow::Instance(path) | ConfigurationRow::Pattern(path) => {
                    apply_binding_change(definition, path, &[], BindingChange::Remove);
                }
                ConfigurationRow::Cell { library, cell } => {
                    set_cell_view(definition, library, cell, None);
                }
            }
        }
    });
}

/// Make `change` to the shown configuration's edits: taken whole, or
/// refused with the reason the page states.
fn edit(
    cx: &mut Cx<'_, ProjectFeature>,
    id: ConfigurationSetId,
    change: impl FnOnce(&mut ConfigurationSetDefinition),
) {
    cx.state
        .configuration
        .edit(&cx.model.workspace.content.configuration_sets, id, change);
}

fn facts(
    cx: &Cx<'_, ProjectFeature>,
    id: Option<ConfigurationSetId>,
    resolution: &ConfigurationResolution,
    editable: bool,
    rows: Vec<ConfigurationRow>,
    at: egui::Pos2,
) -> RowMenu {
    let catalog = &cx.model.workspace.content.configuration_sets;
    let definition = id.and_then(|id| cx.state.configuration.definition(catalog, id));
    let own = |row: &ConfigurationRow| definition.is_some_and(|definition| owns(definition, row));
    let rows = rows
        .into_iter()
        .map(|row| MenuRow {
            views: views_for(resolution, &row),
            row,
        })
        .collect::<Vec<_>>();
    let mut choices: Vec<(String, usize)> = Vec::new();
    for view in rows.iter().flat_map(|row| &row.views) {
        match choices
            .iter_mut()
            .find(|(choice, _)| choice.eq_ignore_ascii_case(view))
        {
            Some((_, count)) => *count += 1,
            None => choices.push((view.clone(), 1)),
        }
    }
    let copied = cx.state.configuration.copied_binding.as_ref();
    RowMenu {
        at,
        open: rows
            .first()
            .and_then(|row| view_to_open(&cx.model.library_manager, resolution, &row.row)),
        choices,
        editable,
        copyable: rows.first().is_some_and(|row| own(&row.row)),
        removable: rows.iter().any(|row| own(&row.row)),
        pastable: copied.is_some_and(|copied| rows.iter().any(|row| takes(copied, &row.row))),
        rows,
    }
}

/// Draw the menu; the item taken, if one was.
fn show(ctx: &egui::Context, menu_id: egui::Id, menu: &RowMenu, opened: bool) -> Option<Pick> {
    let mut pick = None;
    egui::Popup::new(
        menu_id,
        ctx.clone(),
        egui::PopupAnchor::Position(menu.at),
        egui::LayerId::new(egui::Order::Foreground, menu_id),
    )
    .kind(egui::PopupKind::Menu)
    .open_memory(opened.then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .show(|ui| {
        let edits = menu.editable;
        if item(ui, "Open view", menu.open.is_some()) {
            pick = Some(Pick::Open);
        }
        ui.add_enabled_ui(edits, |ui| {
            ui.menu_button("Set view to", |ui| {
                if item(ui, INHERITED, true) {
                    pick = Some(Pick::SetView(None));
                }
                let rows = menu.rows.len();
                for (view, count) in &menu.choices {
                    let label = if *count == rows {
                        view.clone()
                    } else {
                        format!("{view} ({count} of {rows})")
                    };
                    if item(ui, &label, true) {
                        pick = Some(Pick::SetView(Some(view.clone())));
                    }
                }
            });
        });
        if item(ui, "Remove binding", edits && menu.removable) {
            pick = Some(Pick::Remove);
        }
        ui.separator();
        if item(ui, "Copy binding", menu.copyable) {
            pick = Some(Pick::Copy);
        }
        if item(ui, "Paste binding", edits && menu.pastable) {
            pick = Some(Pick::Paste);
        }
    });
    pick
}

/// One menu item; whether the reader took it.
fn item(ui: &mut Ui, label: &str, enabled: bool) -> bool {
    let taken = ui.add_enabled(enabled, egui::Button::new(label)).clicked();
    if taken {
        ui.close();
    }
    taken
}

/// The views `row` can be set to: its cell's, or for a pattern every view
/// the project holds.
fn views_for(resolution: &ConfigurationResolution, row: &ConfigurationRow) -> Vec<String> {
    let tables = &resolution.tables;
    let options = match row {
        ConfigurationRow::Instance(path) => resolution
            .instance(path)
            .and_then(|instance| tables.binding_options.get(instance.binding)),
        ConfigurationRow::Cell { library, cell } => tables
            .cell_rows
            .iter()
            .find(|shown| {
                shown.library.eq_ignore_ascii_case(library) && shown.cell.eq_ignore_ascii_case(cell)
            })
            .map(|shown| &shown.options),
        ConfigurationRow::Pattern(_) => return resolution.view_names.clone(),
    };
    // The first choice is Inherited, which the menu offers on its own.
    options
        .and_then(|options| tables.view_options.get(*options))
        .map_or_else(Vec::new, |choices| {
            choices.iter().skip(1).cloned().collect()
        })
}

/// The view `row` is bound to and its type: an instance's resolved view,
/// the first view a cell's instances resolved to. A pattern names many
/// instances and no one view.
fn view_to_open(
    libraries: &LibraryManager,
    resolution: &ConfigurationResolution,
    row: &ConfigurationRow,
) -> Option<(CellViewRef, ViewType)> {
    let reference = match row {
        ConfigurationRow::Instance(path) => {
            let instance = resolution.instance(path)?;
            let binding = resolution
                .hierarchy
                .as_ref()
                .ok()?
                .bindings
                .get(instance.binding)?;
            binding
                .status
                .is_resolved()
                .then(|| binding.reference.clone())?
        }
        ConfigurationRow::Cell { library, cell } => {
            let shown = resolution.cells.iter().find(|shown| {
                shown.library.eq_ignore_ascii_case(library) && shown.cell.eq_ignore_ascii_case(cell)
            })?;
            CellViewRef::new(&shown.library, &shown.cell, shown.views_found.first()?)
        }
        ConfigurationRow::Pattern(_) => return None,
    };
    let view_type = libraries
        .get_library(&reference.library)?
        .get_cell(&reference.cell)?
        .get_view(&reference.view)?
        .view_type;
    Some((reference, view_type))
}

/// Whether `row` holds a binding of its own in `definition`.
fn owns(definition: &ConfigurationSetDefinition, row: &ConfigurationRow) -> bool {
    match row {
        ConfigurationRow::Instance(path) => instance_binding(definition, path).is_some(),
        ConfigurationRow::Pattern(pattern) => pattern_binding(definition, pattern).is_some(),
        ConfigurationRow::Cell { library, cell } => {
            cell_binding(definition, library, cell).is_some()
        }
    }
}

fn pattern_binding<'a>(
    definition: &'a ConfigurationSetDefinition,
    pattern: &str,
) -> Option<&'a ConfigurationSetOverride> {
    definition.overrides.iter().find(|binding| {
        binding
            .instance_path
            .trim()
            .eq_ignore_ascii_case(pattern.trim())
    })
}

/// Make `view` the first view `row` tries, keeping the rest of its list
/// behind it, or with `None` stop it naming views so it inherits them.
fn set_view(
    definition: &mut ConfigurationSetDefinition,
    row: &ConfigurationRow,
    view: Option<&str>,
) {
    match row {
        ConfigurationRow::Instance(path) => set_instance_view(definition, path, view),
        ConfigurationRow::Cell { library, cell } => set_cell_view(definition, library, cell, view),
        ConfigurationRow::Pattern(pattern) => match view {
            None => apply_binding_change(definition, pattern, &[], BindingChange::Inherit),
            Some(view) => {
                if let Some(binding) = definition.overrides.iter_mut().find(|binding| {
                    binding
                        .instance_path
                        .trim()
                        .eq_ignore_ascii_case(pattern.trim())
                }) {
                    binding
                        .executable_views
                        .retain(|listed| !listed.eq_ignore_ascii_case(view));
                    binding.executable_views.insert(0, view.to_owned());
                }
            }
        },
    }
}

/// The binding `row` holds of its own, put aside for Paste binding.
fn copy(definition: &ConfigurationSetDefinition, row: &ConfigurationRow) -> Option<CopiedBinding> {
    let from_override = |binding: &ConfigurationSetOverride| CopiedBinding {
        views: binding.executable_views.clone(),
        stop: binding.stop_view.clone(),
        model_section: binding.model_section.clone(),
        platforms: Some(binding.eligible_platforms.clone()),
    };
    match row {
        ConfigurationRow::Instance(path) => instance_binding(definition, path).map(from_override),
        ConfigurationRow::Pattern(pattern) => {
            pattern_binding(definition, pattern).map(from_override)
        }
        ConfigurationRow::Cell { library, cell } => {
            cell_binding(definition, library, cell).map(|binding| CopiedBinding {
                views: binding.executable_views.clone(),
                stop: binding.stop_view.clone(),
                model_section: None,
                platforms: None,
            })
        }
    }
}

/// Whether `copied` can go on `row`: a cell binding holds nothing but views.
fn takes(copied: &CopiedBinding, row: &ConfigurationRow) -> bool {
    match row {
        ConfigurationRow::Cell { .. } => !copied.views.is_empty(),
        ConfigurationRow::Instance(_) | ConfigurationRow::Pattern(_) => true,
    }
}

/// Give `row` the binding `copied` holds in place of its own: the views, the
/// stop, and on an instance or a pattern the model section and platforms.
fn paste(
    definition: &mut ConfigurationSetDefinition,
    row: &ConfigurationRow,
    copied: &CopiedBinding,
) {
    if !takes(copied, row) {
        return;
    }
    let platforms = copied
        .platforms
        .clone()
        .unwrap_or_else(|| ConfigurationPlatform::ALL.to_vec());
    match row {
        ConfigurationRow::Instance(path) => {
            let Ok(path) = InstancePath::parse(path) else {
                return;
            };
            let key = path.fold_key();
            let binding = ConfigurationSetOverride {
                instance_path: path.to_string(),
                executable_views: copied.views.clone(),
                stop_view: copied.stop.clone(),
                model_section: copied.model_section.clone(),
                eligible_platforms: platforms,
            };
            match definition.overrides.iter().position(|existing| {
                InstancePath::parse(&existing.instance_path)
                    .is_ok_and(|exact| exact.fold_key() == key)
            }) {
                Some(index) => definition.overrides[index] = binding,
                None => definition.overrides.push(binding),
            }
        }
        ConfigurationRow::Pattern(pattern) => {
            if let Some(binding) = definition.overrides.iter_mut().find(|binding| {
                binding
                    .instance_path
                    .trim()
                    .eq_ignore_ascii_case(pattern.trim())
            }) {
                binding.executable_views = copied.views.clone();
                binding.stop_view = copied.stop.clone();
                binding.model_section = copied.model_section.clone();
                binding.eligible_platforms = platforms;
            }
        }
        ConfigurationRow::Cell { library, cell } => {
            let binding = ConfigurationCellBinding {
                library: library.clone(),
                cell: cell.clone(),
                executable_views: copied.views.clone(),
                stop_view: copied.stop.clone(),
            };
            match definition.cell_bindings.iter().position(|existing| {
                existing.library.eq_ignore_ascii_case(library)
                    && existing.cell.eq_ignore_ascii_case(cell)
            }) {
                Some(index) => definition.cell_bindings[index] = binding,
                None => definition.cell_bindings.push(binding),
            }
        }
    }
}

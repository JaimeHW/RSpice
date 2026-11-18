//! Binding a row of the Configuration tables to another cell: which cells a
//! row can take, whether each fits, and the edit that binds it.
//!
//! A cell fits where the Cadence Hierarchy Editor lets a switch master go:
//! it declares every pin the placement connects, by name, at the same width,
//! in a direction that does not contradict the placed one. Pins of its own
//! the placement does not connect are left unconnected and said. The pins are
//! read where the resolver reads them, so a cell the picker offers as fitting
//! is one resolution binds; a cell that stops fitting later is the resolver's
//! to show, on the row and in Problems.

use rspice_design::configuration_set::BindingTarget;
use rspice_design::hierarchy::port_fit::port_fit;
use rspice_design::hierarchy::source_view_ports;
use rspice_design::schematic::port::PortSpec;

use super::*;

/// One cell view a row can be bound to, and how it meets the row's pins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TargetChoice {
    pub(crate) cell: String,
    pub(crate) view: String,
    /// What binding it does: `Ok` with the pins it leaves unconnected, if it
    /// leaves any, or `Err` with the pins that keep it out.
    pub(crate) fit: Result<Option<String>, String>,
}

/// The pins the placements `rows` stand for connect, each set with what it
/// is placed as: an instance by its path, a cell by `library/cell`. A row the
/// resolution does not show places nothing to check.
pub(crate) fn placed_pins(
    resolution: &ConfigurationResolution,
    rows: &[ConfigurationRow],
) -> Vec<(String, Vec<PortSpec>)> {
    let Ok(hierarchy) = resolution.hierarchy.as_ref() else {
        return Vec::new();
    };
    let mut placed: Vec<(String, Vec<PortSpec>)> = Vec::new();
    let mut push = |who: String, pins: &[PortSpec]| {
        if !placed.iter().any(|(_, held)| held.as_slice() == pins) {
            placed.push((who, pins.to_vec()));
        }
    };
    for row in rows {
        match row {
            ConfigurationRow::Instance(path) => {
                if let Some(binding) = resolution
                    .instance(path)
                    .and_then(|instance| hierarchy.bindings.get(instance.binding))
                {
                    push(path.clone(), &binding.placed_ports);
                }
            }
            ConfigurationRow::Cell { library, cell } => {
                for binding in &hierarchy.bindings {
                    let placed = binding.placed.as_ref().unwrap_or(&binding.reference);
                    if placed.library.eq_ignore_ascii_case(library)
                        && placed.cell.eq_ignore_ascii_case(cell)
                        && !binding.placed_ports.is_empty()
                    {
                        push(format!("{library}/{cell}"), &binding.placed_ports);
                    }
                }
            }
            ConfigurationRow::Pattern(_) => {}
        }
    }
    placed
}

/// The cells `rows` place, as library and cell, once each.
pub(crate) fn placed_cells(
    resolution: &ConfigurationResolution,
    rows: &[ConfigurationRow],
) -> Vec<(String, String)> {
    let mut cells: Vec<(String, String)> = Vec::new();
    for row in rows {
        let cell = match row {
            ConfigurationRow::Instance(path) => resolution
                .hierarchy
                .as_ref()
                .ok()
                .zip(resolution.instance(path))
                .and_then(|(hierarchy, instance)| hierarchy.bindings.get(instance.binding))
                .map(|binding| {
                    let placed = binding.placed.as_ref().unwrap_or(&binding.reference);
                    (placed.library.clone(), placed.cell.clone())
                }),
            ConfigurationRow::Cell { library, cell } => Some((library.clone(), cell.clone())),
            ConfigurationRow::Pattern(_) => None,
        };
        if let Some(cell) = cell
            && !cells.iter().any(|(library, name)| {
                library.eq_ignore_ascii_case(&cell.0) && name.eq_ignore_ascii_case(&cell.1)
            })
        {
            cells.push(cell);
        }
    }
    cells
}

/// The pins `reference` declares, read as the resolver reads a master's: a
/// schematic's interface as it stands open, a source view's declared
/// terminals. `None` when the view declares none it can be checked against.
pub(crate) fn view_pins(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
    schematic: &SchematicState,
    reference: &CellViewRef,
) -> Option<Vec<PortSpec>> {
    let cell = libraries
        .get_library(&reference.library)?
        .get_cell(&reference.cell)?;
    let view = cell.get_view(&reference.view)?;
    if matches!(view.view_type, ViewType::Schematic | ViewType::Testbench) {
        if workspace
            .content
            .active_view
            .key()
            .eq_ignore_ascii_case(&reference.key())
        {
            return Some(schematic.document().interface_ports());
        }
        return workspace
            .content
            .schematic_buffers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
            .map(|(_, open)| open.document().interface_ports());
    }
    source_view_ports(cell, view)
}

/// How a cell declaring `pins` meets every placement in `placed`: the pins
/// it leaves unconnected, or the pins that keep it out, each named, with the
/// placement they are missing from when there are several.
pub(crate) fn target_fit(
    placed: &[(String, Vec<PortSpec>)],
    pins: Option<&[PortSpec]>,
) -> Result<Option<String>, String> {
    let Some(pins) = pins else {
        return Err("it declares no pins to connect".to_owned());
    };
    let mut unconnected: Vec<String> = Vec::new();
    for (who, placement) in placed {
        let fit = port_fit(placement, pins);
        if let Some(refusal) = fit.refusal() {
            return Err(if placed.len() > 1 {
                format!("for {who}, {refusal}")
            } else {
                refusal
            });
        }
        for pin in fit.unplaced() {
            if !unconnected
                .iter()
                .any(|held| held.eq_ignore_ascii_case(pin))
            {
                unconnected.push(pin.clone());
            }
        }
    }
    Ok((!unconnected.is_empty()).then(|| format!("leaves {} unconnected", unconnected.join(", "))))
}

/// Every cell view in `library` a row placing `placed` can be bound to, by
/// cell then view, each with how it fits. The cells the rows place are not
/// offered: running a cell in its own place binds nothing.
pub(crate) fn target_choices(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
    schematic: &SchematicState,
    library: &str,
    placed: &[(String, Vec<PortSpec>)],
    own: &[(String, String)],
) -> Vec<TargetChoice> {
    count_scan();
    let Some((name, found)) = libraries
        .libraries_by_key()
        .find(|(key, _)| key.eq_ignore_ascii_case(library))
    else {
        return Vec::new();
    };
    let mut choices = Vec::new();
    for (cell_name, cell) in &found.cells {
        let is_own = own.iter().any(|(own_library, own_cell)| {
            own_library.eq_ignore_ascii_case(name) && own_cell.eq_ignore_ascii_case(cell_name)
        });
        if is_own {
            continue;
        }
        for (view_name, view) in &cell.views {
            if view.view_type == ViewType::Symbol {
                continue;
            }
            let reference = CellViewRef::new(name, cell_name, view_name);
            let pins = view_pins(workspace, libraries, schematic, &reference);
            choices.push(TargetChoice {
                cell: cell_name.clone(),
                view: view_name.clone(),
                fit: target_fit(placed, pins.as_deref()),
            });
        }
    }
    choices.sort_by(|left, right| {
        left.fit
            .is_err()
            .cmp(&right.fit.is_err())
            .then_with(|| left.cell.to_lowercase().cmp(&right.cell.to_lowercase()))
            .then_with(|| left.view.to_lowercase().cmp(&right.view.to_lowercase()))
    });
    choices
}

/// The cell `row`'s own binding runs in its place, if it binds another.
pub(crate) fn bound_target<'a>(
    definition: &'a ConfigurationSetDefinition,
    row: &ConfigurationRow,
) -> Option<&'a BindingTarget> {
    match row {
        ConfigurationRow::Instance(path) => instance_binding(definition, path)?.target.as_ref(),
        ConfigurationRow::Cell { library, cell } => {
            cell_binding(definition, library, cell)?.target.as_ref()
        }
        ConfigurationRow::Pattern(_) => None,
    }
}

/// Bind `row` to the cell view `target`, searched by that view alone, as the
/// Hierarchy Editor binds an instance or a cell to one library, cell and
/// view; or with `None` run the placed cell again, keeping the rest of the
/// binding. A pattern names instances of many cells and is left alone.
fn bind_row_to_cell(
    definition: &mut ConfigurationSetDefinition,
    row: &ConfigurationRow,
    target: Option<&CellViewRef>,
) {
    let bound = target.map(|target| BindingTarget::new(&target.library, &target.cell));
    let views = target.map(|target| vec![target.view.clone()]);
    match row {
        ConfigurationRow::Instance(path) => {
            let Ok(parsed) = InstancePath::parse(path) else {
                return;
            };
            let key = parsed.fold_key();
            let existing = definition.overrides.iter().position(|binding| {
                InstancePath::parse(&binding.instance_path)
                    .is_ok_and(|exact| exact.fold_key() == key)
            });
            let index = match (existing, &bound) {
                (Some(index), _) => index,
                (None, None) => return,
                (None, Some(_)) => {
                    definition.overrides.push(ConfigurationSetOverride {
                        instance_path: parsed.to_string(),
                        executable_views: Vec::new(),
                        stop_view: None,
                        model_section: None,
                        eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
                        target: None,
                    });
                    definition.overrides.len() - 1
                }
            };
            let binding = &mut definition.overrides[index];
            binding.target = bound;
            if let Some(views) = views {
                retarget_views(&mut binding.executable_views, &mut binding.stop_view, views);
            }
            if sets_nothing(&definition.overrides[index]) {
                definition.overrides.remove(index);
            }
        }
        ConfigurationRow::Cell { library, cell } => {
            let existing = definition.cell_bindings.iter().position(|binding| {
                binding.library.eq_ignore_ascii_case(library)
                    && binding.cell.eq_ignore_ascii_case(cell)
            });
            let index = match (existing, &bound) {
                (Some(index), _) => index,
                (None, None) => return,
                (None, Some(_)) => {
                    definition.cell_bindings.push(ConfigurationCellBinding {
                        library: library.clone(),
                        cell: cell.clone(),
                        executable_views: Vec::new(),
                        stop_view: None,
                        target: None,
                    });
                    definition.cell_bindings.len() - 1
                }
            };
            let binding = &mut definition.cell_bindings[index];
            binding.target = bound;
            if let Some(views) = views {
                retarget_views(&mut binding.executable_views, &mut binding.stop_view, views);
            }
            if binding.executable_views.is_empty() && binding.target.is_none() {
                definition.cell_bindings.remove(index);
            }
        }
        ConfigurationRow::Pattern(_) => {}
    }
}

/// A binding to one cell view searches that view alone; a stop on it stays.
fn retarget_views(views: &mut Vec<String>, stop: &mut Option<String>, chosen: Vec<String>) {
    *stop = stop
        .take()
        .filter(|stop| chosen.iter().any(|view| view.eq_ignore_ascii_case(stop)));
    *views = chosen;
}

impl ConfigurationPageState {
    /// Bind every row of `rows` to the cell view `target`, as one edit, or
    /// with `None` run their placed cells again. A cell that does not hold
    /// the pins a row places is refused before anything changes, with the
    /// pins that differ named in [`Self::refusal`]. Returns whether the edit
    /// was taken.
    pub fn bind_to_cell(
        &mut self,
        workspace: &ProjectWorkspace,
        libraries: &LibraryManager,
        schematic: &SchematicState,
        id: ConfigurationSetId,
        rows: &[ConfigurationRow],
        target: Option<&CellViewRef>,
    ) -> bool {
        if let Some(target) = target {
            let placed = self
                .last_resolution()
                .map(|resolution| placed_pins(&resolution, rows))
                .unwrap_or_default();
            let pins = view_pins(workspace, libraries, schematic, target);
            if let Err(pins) = target_fit(&placed, pins.as_deref()) {
                self.refusal = Some(format!(
                    "{} does not fit {}: {pins}.",
                    CellViewRef::new(&target.library, &target.cell, &target.view).display_path(),
                    rows_label(rows)
                ));
                return false;
            }
        }
        self.edit(&workspace.content.configuration_sets, id, |definition| {
            for row in rows {
                bind_row_to_cell(definition, row, target);
            }
        })
    }
}

/// What `rows` are, in the page's words: one row by its name, several by
/// their count.
pub(crate) fn rows_label(rows: &[ConfigurationRow]) -> String {
    match rows {
        [ConfigurationRow::Instance(path)] => path.clone(),
        [ConfigurationRow::Cell { library, cell }] => format!("{library}/{cell}"),
        [ConfigurationRow::Pattern(pattern)] => pattern.clone(),
        rows => format!("{} rows", rows.len()),
    }
}

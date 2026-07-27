//! Deterministic align/distribute transactions for schematic selections.
//!
//! Electrical conductors and net-label anchors are intentionally rejected:
//! moving either as presentation geometry can silently change connectivity.
//! Instances and non-electrical annotations have complete movement semantics
//! and therefore form the compatible selection class for these commands.

use std::fmt;

use crate::workbench::app::AppState;
use crate::state::{Point, Selection};

use super::SchematicSymbolContext;
use super::design_notes;
use super::documentation_shapes;
use super::sheet_visibility::object_is_on_active_sheet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectionLayoutCommand {
    AlignLeft,
    AlignCenter,
    AlignRight,
    AlignTop,
    AlignMiddle,
    AlignBottom,
    DistributeHorizontal,
    DistributeVertical,
}

impl SelectionLayoutCommand {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::AlignLeft => "Align left",
            Self::AlignCenter => "Align horizontal centers",
            Self::AlignRight => "Align right",
            Self::AlignTop => "Align top",
            Self::AlignMiddle => "Align vertical centers",
            Self::AlignBottom => "Align bottom",
            Self::DistributeHorizontal => "Distribute horizontally",
            Self::DistributeVertical => "Distribute vertically",
        }
    }

    const fn minimum_objects(self) -> usize {
        match self {
            Self::DistributeHorizontal | Self::DistributeVertical => 3,
            _ => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SelectionLayoutError {
    ReadOnly,
    IncompatibleSelection,
    TooFewObjects { required: usize, selected: usize },
    StaleSelection,
    OffActiveSheet,
    CoordinateOverflow,
}

impl SelectionLayoutError {
    pub(crate) const fn message(&self) -> &'static str {
        match self {
            Self::ReadOnly => "The active schematic is locked or read-only",
            Self::IncompatibleSelection => {
                "Select only instances and non-electrical annotations; conductors and net-label anchors cannot be aligned safely"
            }
            Self::TooFewObjects { required: 3, .. } => {
                "Select at least three compatible objects to distribute"
            }
            Self::TooFewObjects { .. } => "Select at least two compatible objects to align",
            Self::StaleSelection => "The selection contains an object that no longer exists",
            Self::OffActiveSheet => "Every selected object must belong to the active sheet",
            Self::CoordinateOverflow => "The requested layout would exceed schematic coordinates",
        }
    }
}

impl fmt::Display for SelectionLayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message())
    }
}

impl std::error::Error for SelectionLayoutError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TargetKind {
    Component,
    DesignNote,
    DocumentationShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LayoutTarget {
    kind: TargetKind,
    id: u64,
    min: Point,
    max: Point,
}

impl LayoutTarget {
    fn center_x(self) -> i64 {
        (i64::from(self.min.x) + i64::from(self.max.x)) / 2
    }

    fn center_y(self) -> i64 {
        (i64::from(self.min.y) + i64::from(self.max.y)) / 2
    }

    fn width(self) -> i64 {
        i64::from(self.max.x) - i64::from(self.min.x)
    }

    fn height(self) -> i64 {
        i64::from(self.max.y) - i64::from(self.min.y)
    }
}

pub(crate) fn selection_layout_availability(
    state: &AppState,
    command: SelectionLayoutCommand,
) -> Result<(), SelectionLayoutError> {
    let context = SchematicSymbolContext::from_state(state);
    selection_layout_targets(state, &context, command).map(|_| ())
}

/// Apply one exact layout transform as a single schematic undo transaction.
///
/// Validation constructs all target deltas before the first mutation. Any
/// stale, hidden, incompatible, locked, or overflowing object therefore
/// rejects the entire command without changing design or undo state.
pub(crate) fn apply_selection_layout(
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    command: SelectionLayoutCommand,
) -> Result<bool, SelectionLayoutError> {
    let targets = selection_layout_targets(state, symbol_context, command)?;
    let deltas = layout_deltas(&targets, command)?;
    if deltas.iter().all(|(_, delta)| *delta == Point::origin()) {
        return Ok(false);
    }

    state.schematic.begin_operation(command.label());
    for (target, delta) in deltas {
        match target.kind {
            TargetKind::Component => {
                state
                    .schematic
                    .move_component_with_wires_resolved(target.id, delta, |component| {
                        symbol_context.terminal_points(component)
                    })
            }
            TargetKind::DesignNote => {
                let note = state
                    .schematic
                    .design_notes
                    .iter_mut()
                    .find(|note| note.id == target.id)
                    .expect("validated design-note target remains live");
                note.translate(delta);
            }
            TargetKind::DocumentationShape => {
                let shape = state
                    .schematic
                    .documentation_shapes
                    .iter_mut()
                    .find(|shape| shape.id == target.id)
                    .expect("validated documentation-shape target remains live");
                shape.translate(delta);
            }
        }
    }
    state.schematic.is_dirty = true;
    let recorded = state.schematic.end_operation();
    if recorded {
        state.sync_active_schematic_to_workspace();
    }
    Ok(recorded)
}

fn selection_layout_targets(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    command: SelectionLayoutCommand,
) -> Result<Vec<LayoutTarget>, SelectionLayoutError> {
    if state.schematic.read_only
        || state.active_view_read_only()
        || state.workbench.safe_mode.project_read_only()
    {
        return Err(SelectionLayoutError::ReadOnly);
    }

    let selection = &state.schematic.selection;
    if has_incompatible_selection(selection) {
        return Err(SelectionLayoutError::IncompatibleSelection);
    }

    let mut targets = Vec::with_capacity(selection.count());
    for id in &selection.components {
        let component = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == *id)
            .ok_or(SelectionLayoutError::StaleSelection)?;
        require_active_sheet(state, component.id)?;
        let (min, max) = symbol_context.component_bounds(component);
        targets.push(LayoutTarget {
            kind: TargetKind::Component,
            id: *id,
            min,
            max,
        });
    }
    for id in &selection.design_notes {
        let note = state
            .schematic
            .design_notes
            .iter()
            .find(|note| note.id == *id)
            .ok_or(SelectionLayoutError::StaleSelection)?;
        require_active_sheet(state, note.id)?;
        let (min, max) = design_notes::conservative_world_bounds(note);
        targets.push(LayoutTarget {
            kind: TargetKind::DesignNote,
            id: *id,
            min,
            max,
        });
    }
    for id in &selection.documentation_shapes {
        let shape = state
            .schematic
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == *id)
            .ok_or(SelectionLayoutError::StaleSelection)?;
        require_active_sheet(state, shape.id)?;
        let (min, max) = documentation_shapes::world_bounds(shape);
        targets.push(LayoutTarget {
            kind: TargetKind::DocumentationShape,
            id: *id,
            min,
            max,
        });
    }

    let required = command.minimum_objects();
    if targets.len() < required {
        return Err(SelectionLayoutError::TooFewObjects {
            required,
            selected: targets.len(),
        });
    }
    Ok(targets)
}

fn has_incompatible_selection(selection: &Selection) -> bool {
    !selection.wires.is_empty()
        || !selection.wire_segments.is_empty()
        || !selection.wire_vertices.is_empty()
        || !selection.junctions.is_empty()
        || !selection.buses.is_empty()
        || !selection.bus_taps.is_empty()
        || !selection.net_labels.is_empty()
}

fn require_active_sheet(state: &AppState, object_id: u64) -> Result<(), SelectionLayoutError> {
    if object_is_on_active_sheet(state, object_id) {
        Ok(())
    } else {
        Err(SelectionLayoutError::OffActiveSheet)
    }
}

fn layout_deltas(
    targets: &[LayoutTarget],
    command: SelectionLayoutCommand,
) -> Result<Vec<(LayoutTarget, Point)>, SelectionLayoutError> {
    match command {
        SelectionLayoutCommand::DistributeHorizontal => distribute(targets, true),
        SelectionLayoutCommand::DistributeVertical => distribute(targets, false),
        _ => align(targets, command),
    }
}

fn align(
    targets: &[LayoutTarget],
    command: SelectionLayoutCommand,
) -> Result<Vec<(LayoutTarget, Point)>, SelectionLayoutError> {
    let min_x = targets
        .iter()
        .map(|target| i64::from(target.min.x))
        .min()
        .expect("minimum selection cardinality validated");
    let max_x = targets
        .iter()
        .map(|target| i64::from(target.max.x))
        .max()
        .expect("minimum selection cardinality validated");
    let min_y = targets
        .iter()
        .map(|target| i64::from(target.min.y))
        .min()
        .expect("minimum selection cardinality validated");
    let max_y = targets
        .iter()
        .map(|target| i64::from(target.max.y))
        .max()
        .expect("minimum selection cardinality validated");
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;

    targets
        .iter()
        .copied()
        .map(|target| {
            let (delta_x, delta_y) = match command {
                SelectionLayoutCommand::AlignLeft => (min_x - i64::from(target.min.x), 0),
                SelectionLayoutCommand::AlignCenter => (center_x - target.center_x(), 0),
                SelectionLayoutCommand::AlignRight => (max_x - i64::from(target.max.x), 0),
                SelectionLayoutCommand::AlignTop => (0, min_y - i64::from(target.min.y)),
                SelectionLayoutCommand::AlignMiddle => (0, center_y - target.center_y()),
                SelectionLayoutCommand::AlignBottom => (0, max_y - i64::from(target.max.y)),
                SelectionLayoutCommand::DistributeHorizontal
                | SelectionLayoutCommand::DistributeVertical => unreachable!(),
            };
            Ok((target, checked_delta(target, delta_x, delta_y)?))
        })
        .collect()
}

fn distribute(
    targets: &[LayoutTarget],
    horizontal: bool,
) -> Result<Vec<(LayoutTarget, Point)>, SelectionLayoutError> {
    let mut sorted = targets.to_vec();
    sorted.sort_by_key(|target| {
        let primary = if horizontal {
            target.min.x
        } else {
            target.min.y
        };
        let secondary = if horizontal {
            target.min.y
        } else {
            target.min.x
        };
        (primary, secondary, target.kind, target.id)
    });

    let first = sorted[0];
    let last = sorted[sorted.len() - 1];
    let total_size: i64 = sorted
        .iter()
        .map(|target| {
            if horizontal {
                target.width()
            } else {
                target.height()
            }
        })
        .sum();
    let span = if horizontal {
        i64::from(last.max.x) - i64::from(first.min.x)
    } else {
        i64::from(last.max.y) - i64::from(first.min.y)
    };
    let gap_numerator = span - total_size;
    let gap_count = i64::try_from(sorted.len() - 1).expect("selection length fits i64");
    let origin = if horizontal {
        i64::from(first.min.x)
    } else {
        i64::from(first.min.y)
    };
    let mut preceding_size = 0_i64;
    let mut result = Vec::with_capacity(sorted.len());

    for (index, target) in sorted.into_iter().enumerate() {
        let index = i64::try_from(index).expect("selection index fits i64");
        let desired_min = origin + preceding_size + rounded_ratio(gap_numerator * index, gap_count);
        let current_min = if horizontal {
            i64::from(target.min.x)
        } else {
            i64::from(target.min.y)
        };
        let (delta_x, delta_y) = if horizontal {
            (desired_min - current_min, 0)
        } else {
            (0, desired_min - current_min)
        };
        result.push((target, checked_delta(target, delta_x, delta_y)?));
        preceding_size += if horizontal {
            target.width()
        } else {
            target.height()
        };
    }
    Ok(result)
}

fn rounded_ratio(numerator: i64, denominator: i64) -> i64 {
    debug_assert!(denominator > 0);
    if numerator >= 0 {
        (numerator + denominator / 2) / denominator
    } else {
        -((-numerator + denominator / 2) / denominator)
    }
}

fn checked_delta(
    target: LayoutTarget,
    delta_x: i64,
    delta_y: i64,
) -> Result<Point, SelectionLayoutError> {
    let delta_x = i32::try_from(delta_x).map_err(|_| SelectionLayoutError::CoordinateOverflow)?;
    let delta_y = i32::try_from(delta_y).map_err(|_| SelectionLayoutError::CoordinateOverflow)?;
    for (value, delta) in [
        (target.min.x, delta_x),
        (target.max.x, delta_x),
        (target.min.y, delta_y),
        (target.max.y, delta_y),
    ] {
        value
            .checked_add(delta)
            .ok_or(SelectionLayoutError::CoordinateOverflow)?;
    }
    Ok(Point::new(delta_x, delta_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, SheetDefinition, SheetPortPolicy, SheetTemplate};

    fn selected_components(positions: &[(u64, i32, i32)]) -> AppState {
        let mut state = AppState::default();
        for &(id, x, y) in positions {
            state.schematic.components.push(Component::new(
                id,
                ComponentType::Resistor,
                Point::new(x, y),
            ));
            state.schematic.selection.select_component(id);
        }
        state.schematic.init_undo_history();
        state
    }

    #[test]
    fn all_alignment_edges_are_deterministic_and_one_undo_unit() {
        for command in [
            SelectionLayoutCommand::AlignLeft,
            SelectionLayoutCommand::AlignCenter,
            SelectionLayoutCommand::AlignRight,
            SelectionLayoutCommand::AlignTop,
            SelectionLayoutCommand::AlignMiddle,
            SelectionLayoutCommand::AlignBottom,
        ] {
            let mut state = selected_components(&[(3, 70, 90), (1, 10, 10), (2, 40, 50)]);
            let original: Vec<Point> = state
                .schematic
                .components
                .iter()
                .map(|component| component.pos)
                .collect();
            let context = SchematicSymbolContext::from_state(&state);

            assert!(apply_selection_layout(&mut state, &context, command).unwrap());
            assert_eq!(state.schematic.undo_description(), Some(command.label()));
            assert!(state.schematic.undo());
            assert_eq!(
                state
                    .schematic
                    .components
                    .iter()
                    .map(|component| component.pos)
                    .collect::<Vec<_>>(),
                original
            );
            assert!(
                !state.schematic.can_undo(),
                "layout command is one undo unit"
            );
        }
    }

    #[test]
    fn distribution_uses_equal_visible_gaps_and_stable_tie_breaking() {
        let mut state =
            selected_components(&[(30, 80, 70), (10, 0, 10), (20, 30, 40), (15, 30, 25)]);
        let context = SchematicSymbolContext::from_state(&state);

        assert!(
            apply_selection_layout(
                &mut state,
                &context,
                SelectionLayoutCommand::DistributeHorizontal,
            )
            .unwrap()
        );
        let context = SchematicSymbolContext::from_state(&state);
        let mut bounds: Vec<_> = state
            .schematic
            .components
            .iter()
            .map(|component| {
                let (min, max) = context.component_bounds(component);
                (min.x, max.x, component.id)
            })
            .collect();
        bounds.sort_by_key(|&(min, _, id)| (min, id));
        let gaps: Vec<_> = bounds
            .windows(2)
            .map(|pair| pair[1].0 - pair[0].1)
            .collect();
        assert!(gaps.windows(2).all(|pair| (pair[0] - pair[1]).abs() <= 1));
    }

    #[test]
    fn incompatible_and_read_only_selections_fail_without_undo() {
        let mut state = selected_components(&[(1, 0, 0), (2, 40, 0)]);
        state.schematic.wires.push(crate::state::Wire::new(
            9,
            vec![Point::origin(), Point::new(10, 0)],
        ));
        state.schematic.selection.select_wire(9);
        let context = SchematicSymbolContext::from_state(&state);
        assert_eq!(
            apply_selection_layout(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::IncompatibleSelection)
        );
        assert!(!state.schematic.can_undo());

        state.schematic.selection.wires.clear();
        state.schematic.read_only = true;
        assert_eq!(
            apply_selection_layout(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::ReadOnly)
        );
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn cross_sheet_selection_fails_closed_instead_of_moving_a_visible_subset() {
        let mut state = selected_components(&[(1, 0, 0), (2, 40, 0)]);
        let key = state.workspace.active_schematic_reference().key();
        let first = state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", [1, 2])
            .expect("first sheet");
        let catalog = state
            .workspace
            .design_management
            .sheet_catalog_mut(&key)
            .expect("sheet catalog");
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Sheet 2".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .expect("second sheet");
        catalog
            .assign_objects(catalog.revision(), second, [2])
            .expect("hidden assignment");
        catalog.set_active(first).expect("active sheet");
        let context = SchematicSymbolContext::from_state(&state);
        let before = state.schematic.components.clone();

        assert_eq!(
            apply_selection_layout(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::OffActiveSheet)
        );
        assert_eq!(state.schematic.components, before);
        assert!(!state.schematic.can_undo());
    }
}

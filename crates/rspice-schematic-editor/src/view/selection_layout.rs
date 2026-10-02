//! Checked align/distribute edits over the current design and selection.
//!
//! The host validates the returned source and applies the complete edit list
//! through its connected-movement and undo owner.

use super::{
    design_notes, design_view::DesignView, documentation_shapes,
    symbol_context::SchematicSymbolContext,
};
use crate::requests::EditorRequestSource;
use rspice_design::schematic::selection::Selection;
use rspice_design_model::Point;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionLayoutCommand {
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
    pub const fn label(self) -> &'static str {
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
pub enum SelectionLayoutError {
    ReadOnly,
    IncompatibleSelection,
    TooFewObjects { required: usize, selected: usize },
    StaleSelection,
    OffActiveSheet,
    CoordinateOverflow,
}

impl SelectionLayoutError {
    pub const fn message(&self) -> &'static str {
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
pub enum SelectionLayoutObject {
    Component(u64),
    DesignNote(u64),
    DocumentationShape(u64),
    Probe(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LayoutTarget {
    object: SelectionLayoutObject,
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

pub struct SelectionLayoutView<'a> {
    pub design: DesignView<'a>,
    pub selection: &'a Selection,
    pub symbols: &'a SchematicSymbolContext,
    pub can_edit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionLayoutMove {
    pub object: SelectionLayoutObject,
    pub delta: Point,
}

#[derive(Debug, Clone)]
pub struct SelectionLayoutRequest {
    pub source: EditorRequestSource,
    pub selection: Selection,
    pub command: SelectionLayoutCommand,
    pub moves: Vec<SelectionLayoutMove>,
}

pub fn availability(
    view: &SelectionLayoutView<'_>,
    command: SelectionLayoutCommand,
) -> Result<(), SelectionLayoutError> {
    selection_layout_targets(view, command).map(|_| ())
}

pub fn prepare(
    view: &SelectionLayoutView<'_>,
    source: EditorRequestSource,
    command: SelectionLayoutCommand,
) -> Result<SelectionLayoutRequest, SelectionLayoutError> {
    let targets = selection_layout_targets(view, command)?;
    let moves = layout_deltas(&targets, command)?;
    Ok(SelectionLayoutRequest {
        source,
        selection: view.selection.clone(),
        command,
        moves,
    })
}

fn selection_layout_targets(
    view: &SelectionLayoutView<'_>,
    command: SelectionLayoutCommand,
) -> Result<Vec<LayoutTarget>, SelectionLayoutError> {
    if !view.can_edit {
        return Err(SelectionLayoutError::ReadOnly);
    }

    let selection = view.selection;
    if has_incompatible_selection(selection) {
        return Err(SelectionLayoutError::IncompatibleSelection);
    }

    let mut targets = Vec::with_capacity(selection.count());
    for id in &selection.components {
        let component = view
            .design
            .document
            .components
            .iter()
            .find(|component| component.id == *id)
            .ok_or(SelectionLayoutError::StaleSelection)?;
        require_active_sheet(&view.design, component.id)?;
        let (min, max) = view.symbols.component_bounds(component);
        targets.push(LayoutTarget {
            object: SelectionLayoutObject::Component(*id),
            min,
            max,
        });
    }
    for id in &selection.design_notes {
        let note = view
            .design
            .document
            .design_notes
            .iter()
            .find(|note| note.id == *id)
            .ok_or(SelectionLayoutError::StaleSelection)?;
        require_active_sheet(&view.design, note.id)?;
        let (min, max) = design_notes::conservative_world_bounds(note);
        targets.push(LayoutTarget {
            object: SelectionLayoutObject::DesignNote(*id),
            min,
            max,
        });
    }
    for id in &selection.documentation_shapes {
        let shape = view
            .design
            .document
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == *id)
            .ok_or(SelectionLayoutError::StaleSelection)?;
        require_active_sheet(&view.design, shape.id)?;
        let (min, max) = documentation_shapes::world_bounds(shape);
        targets.push(LayoutTarget {
            object: SelectionLayoutObject::DocumentationShape(*id),
            min,
            max,
        });
    }
    for id in &selection.probes {
        let probe = view
            .design
            .document
            .probes
            .iter()
            .find(|probe| probe.id == *id)
            .ok_or(SelectionLayoutError::StaleSelection)?;
        require_active_sheet(&view.design, probe.id)?;
        let (min, max) = probe.world_bounds();
        targets.push(LayoutTarget {
            object: SelectionLayoutObject::Probe(*id),
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

fn require_active_sheet(view: &DesignView<'_>, object_id: u64) -> Result<(), SelectionLayoutError> {
    if view.object_is_visible(object_id) {
        Ok(())
    } else {
        Err(SelectionLayoutError::OffActiveSheet)
    }
}

fn layout_deltas(
    targets: &[LayoutTarget],
    command: SelectionLayoutCommand,
) -> Result<Vec<SelectionLayoutMove>, SelectionLayoutError> {
    match command {
        SelectionLayoutCommand::DistributeHorizontal => distribute(targets, true),
        SelectionLayoutCommand::DistributeVertical => distribute(targets, false),
        _ => align(targets, command),
    }
}

fn align(
    targets: &[LayoutTarget],
    command: SelectionLayoutCommand,
) -> Result<Vec<SelectionLayoutMove>, SelectionLayoutError> {
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
            Ok(SelectionLayoutMove {
                object: target.object,
                delta: checked_delta(target, delta_x, delta_y)?,
            })
        })
        .collect()
}

fn distribute(
    targets: &[LayoutTarget],
    horizontal: bool,
) -> Result<Vec<SelectionLayoutMove>, SelectionLayoutError> {
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
        (primary, secondary, target.object)
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
        result.push(SelectionLayoutMove {
            object: target.object,
            delta: checked_delta(target, delta_x, delta_y)?,
        });
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

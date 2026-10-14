//! Linked X axes and linked A/B cursors across the panes of a document.
//!
//! A horizontal-viewport link group makes its member axes one X range: the
//! renderer writes a zoom of any member to every member in one transaction.
//! A cursor-position group makes its member cursors one cursor: moving it in
//! any pane moves it in all of them, and clearing it clears all of them.

use std::collections::{BTreeMap, BTreeSet};

use super::*;
use crate::results::visualization_document::{
    Axis, AxisId, AxisOrientation, AxisRange, Cursor, CursorId, LinkKind, TypedValue,
};

const X_LINK_LABEL: &str = "Linked horizontal viewport";

fn horizontal_axis(document: &VisualizationDocument, pane_id: PaneId) -> Option<&Axis> {
    document
        .axes()
        .iter()
        .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal)
}

/// Two X axes can share a range only when they measure the same quantity on
/// the same scale.
fn compatible(left: &Axis, right: &Axis) -> bool {
    left.scale == right.scale && left.unit == right.unit
}

/// The axes `axis_id` shares its X range with, itself excluded.
pub(super) fn x_linked_axes(document: &VisualizationDocument, axis_id: AxisId) -> Vec<&Axis> {
    let member = EntityRef::Axis(axis_id);
    let mut linked = BTreeSet::new();
    for group in document.link_groups().iter().filter(|group| {
        group.kind == LinkKind::HorizontalViewport && group.members.contains(&member)
    }) {
        for entity in &group.members {
            if let EntityRef::Axis(id) = entity
                && *id != axis_id
            {
                linked.insert(*id);
            }
        }
    }
    document
        .axes()
        .iter()
        .filter(|axis| linked.contains(&axis.id))
        .collect()
}

/// Whether `pane_id`'s X axis is linked, or `None` when nothing on its page
/// could share its range.
pub(super) fn x_link_state(document: &VisualizationDocument, pane_id: PaneId) -> Option<bool> {
    let axis = horizontal_axis(document, pane_id)?;
    if !x_linked_axes(document, axis.id).is_empty() {
        return Some(true);
    }
    let page_id = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)?
        .page_id;
    document
        .panes()
        .iter()
        .filter(|pane| pane.page_id == page_id && pane.id != pane_id)
        .filter_map(|pane| horizontal_axis(document, pane.id))
        .any(|other| compatible(axis, other))
        .then_some(false)
}

/// Link or unlink one pane's X axis with the rest of its page.
///
/// Unlinking takes the axis out of its group, and a group left with one axis
/// goes. Linking joins the page's group when it has one, or else links every
/// compatible X axis on the page; the joining axis takes the group's range
/// in the same transaction, so the panes line up at once.
pub(super) fn toggle_x_link(
    state: &mut AppState,
    document_id: ResultDocumentId,
    pane_id: PaneId,
) -> Result<(), String> {
    let edits = x_link_edits(project_document(state, document_id)?, pane_id)?;
    if edits.is_empty() {
        return Ok(());
    }
    transact(state, document_id, edits).map(|_| ())
}

fn x_link_edits(
    document: &VisualizationDocument,
    pane_id: PaneId,
) -> Result<Vec<DocumentEdit>, String> {
    let axis = horizontal_axis(document, pane_id)
        .ok_or_else(|| "This pane has no X axis to link.".to_owned())?;
    let member = EntityRef::Axis(axis.id);
    let joined = document
        .link_groups()
        .iter()
        .filter(|group| {
            group.kind == LinkKind::HorizontalViewport && group.members.contains(&member)
        })
        .collect::<Vec<_>>();
    if !joined.is_empty() {
        return Ok(joined
            .into_iter()
            .map(|group| {
                let members = group
                    .members
                    .iter()
                    .copied()
                    .filter(|candidate| *candidate != member)
                    .collect::<Vec<_>>();
                if members.len() < 2 {
                    DocumentEdit::Remove(EntityRef::LinkGroup(group.id))
                } else {
                    DocumentEdit::SetLinkMembers {
                        link_group_id: group.id,
                        members,
                    }
                }
            })
            .collect());
    }
    let page_id = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .map(|pane| pane.page_id)
        .ok_or_else(|| "The pane is no longer in this document.".to_owned())?;
    let page_axes = document
        .panes()
        .iter()
        .filter(|pane| pane.page_id == page_id && pane.id != pane_id)
        .filter_map(|pane| horizontal_axis(document, pane.id))
        .filter(|other| compatible(axis, other))
        .collect::<Vec<_>>();
    let page_group = document.link_groups().iter().find(|group| {
        group.kind == LinkKind::HorizontalViewport
            && page_axes
                .iter()
                .any(|other| group.members.contains(&EntityRef::Axis(other.id)))
    });
    let mut edits = Vec::new();
    let range = match page_group {
        Some(group) => {
            let mut members = group.members.clone();
            members.push(member);
            edits.push(DocumentEdit::SetLinkMembers {
                link_group_id: group.id,
                members,
            });
            page_axes
                .iter()
                .find(|other| group.members.contains(&EntityRef::Axis(other.id)))
                .and_then(|other| other.range)
        }
        None if !page_axes.is_empty() => {
            let range = page_axes.first().and_then(|other| other.range);
            let mut members = vec![member];
            members.extend(page_axes.iter().map(|other| EntityRef::Axis(other.id)));
            edits.push(DocumentEdit::AddLinkGroup {
                label: X_LINK_LABEL.to_owned(),
                kind: LinkKind::HorizontalViewport,
                members,
            });
            // Every member takes the first one's range, so the new group
            // starts as one X range rather than as several.
            for other in page_axes.iter().skip(1) {
                if other.range != range {
                    edits.push(DocumentEdit::SetAxisRange {
                        axis_id: other.id,
                        range,
                    });
                }
            }
            range
        }
        None => return Err("No other pane on this page has a compatible X axis.".to_owned()),
    };
    if axis.range != range {
        edits.push(DocumentEdit::SetAxisRange {
            axis_id: axis.id,
            range,
        });
    }
    Ok(edits)
}

/// The edits that give `axis_id` the X range `range`, and every axis linked
/// with it the same range.
pub(in crate::workbench::documents::result_document) fn x_range_edits(
    document: &VisualizationDocument,
    axis_id: AxisId,
    range: Option<AxisRange>,
) -> Vec<DocumentEdit> {
    document
        .axes()
        .iter()
        .filter(|axis| axis.id == axis_id)
        .chain(x_linked_axes(document, axis_id))
        .filter(|axis| axis.range != range)
        .map(|axis| DocumentEdit::SetAxisRange {
            axis_id: axis.id,
            range,
        })
        .collect()
}

fn real_position(cursor: &Cursor) -> Option<f64> {
    match cursor.position {
        TypedValue::Real(position) => Some(position),
        _ => None,
    }
}

/// The edits that put `pane_id`'s A and B cursors at `desired`, carrying
/// every cursor linked with them along. A cursor that is cleared takes its
/// links and every cursor in them with it.
pub(in crate::workbench::documents::result_document) fn cursor_pair_edits(
    document: &VisualizationDocument,
    pane_id: PaneId,
    desired: (Option<f64>, Option<f64>),
) -> Result<Vec<DocumentEdit>, String> {
    let axis_id = horizontal_axis(document, pane_id)
        .map(|axis| axis.id)
        .ok_or_else(|| "The pane has no horizontal cursor axis.".to_owned())?;
    if desired
        .0
        .into_iter()
        .chain(desired.1)
        .any(|position| !position.is_finite())
    {
        return Err("A cursor position must be finite.".to_owned());
    }
    let mut removed_groups = BTreeSet::new();
    let mut removed_cursors = BTreeSet::new();
    let mut moves = BTreeMap::<CursorId, f64>::new();
    let mut additions = Vec::new();
    for (label, target) in [("A", desired.0), ("B", desired.1)] {
        let existing = document
            .cursors()
            .iter()
            .find(|cursor| cursor.pane_id == pane_id && cursor.label == label);
        let groups = |cursor: &Cursor| {
            document
                .link_groups()
                .iter()
                .filter(|group| {
                    group.kind == LinkKind::CursorPosition
                        && group.members.contains(&EntityRef::Cursor(cursor.id))
                })
                .collect::<Vec<_>>()
        };
        match (existing, target) {
            (Some(cursor), Some(position)) => {
                let mut targets = BTreeSet::from([cursor.id]);
                for group in groups(cursor) {
                    targets.extend(group.members.iter().filter_map(|member| match member {
                        EntityRef::Cursor(id) => Some(*id),
                        _ => None,
                    }));
                }
                for cursor in document
                    .cursors()
                    .iter()
                    .filter(|candidate| targets.contains(&candidate.id))
                {
                    if real_position(cursor)
                        .is_none_or(|current| current.to_bits() != position.to_bits())
                    {
                        moves.insert(cursor.id, position);
                    }
                }
            }
            (Some(cursor), None) => {
                let linked = groups(cursor);
                removed_cursors.insert(cursor.id);
                for group in linked {
                    removed_groups.insert(group.id);
                    removed_cursors.extend(group.members.iter().filter_map(
                        |member| match member {
                            EntityRef::Cursor(id) => Some(*id),
                            _ => None,
                        },
                    ));
                }
            }
            (None, Some(position)) => additions.push(DocumentEdit::AddCursor {
                pane_id,
                axis_id,
                position: TypedValue::Real(position),
                label: label.to_owned(),
            }),
            (None, None) => {}
        }
    }
    let mut edits = removed_groups
        .into_iter()
        .map(|group_id| DocumentEdit::Remove(EntityRef::LinkGroup(group_id)))
        .collect::<Vec<_>>();
    edits.extend(
        moves
            .into_iter()
            .filter(|(cursor_id, _)| !removed_cursors.contains(cursor_id))
            .map(|(cursor_id, position)| DocumentEdit::MoveCursor {
                cursor_id,
                position: TypedValue::Real(position),
            }),
    );
    edits.extend(
        removed_cursors
            .into_iter()
            .map(|cursor_id| DocumentEdit::Remove(EntityRef::Cursor(cursor_id))),
    );
    edits.extend(additions);
    Ok(edits)
}

/// Whether the document links its A/B cursors across panes.
pub(super) fn cursors_linked(document: &VisualizationDocument) -> bool {
    document
        .link_groups()
        .iter()
        .any(|group| group.kind == LinkKind::CursorPosition)
}

/// Link the A cursors of every pane whose X axis matches the selected pane's,
/// and the B cursors likewise, or take every cursor link away. A link moves
/// each member to the selected pane's position, so the panes agree at once.
pub(super) fn set_cursor_links(
    state: &mut AppState,
    document_id: ResultDocumentId,
    linked: bool,
) -> Result<(), String> {
    let reference_pane = super::super::persistent_document::reading_position(state, document_id)
        .map(|(_, pane_id)| pane_id);
    let document = project_document(state, document_id)?;
    let mut edits = document
        .link_groups()
        .iter()
        .filter(|group| group.kind == LinkKind::CursorPosition)
        .map(|group| DocumentEdit::Remove(EntityRef::LinkGroup(group.id)))
        .collect::<Vec<_>>();
    if linked {
        let mut added = 0_usize;
        for label in ["A", "B"] {
            let candidates = document
                .cursors()
                .iter()
                .filter(|cursor| cursor.label == label)
                .collect::<Vec<_>>();
            let Some(reference) = candidates
                .iter()
                .copied()
                .find(|cursor| Some(cursor.pane_id) == reference_pane)
                .or_else(|| candidates.first().copied())
            else {
                continue;
            };
            let Some(reference_axis) = document
                .axes()
                .iter()
                .find(|axis| axis.id == reference.axis_id)
            else {
                continue;
            };
            let compatible = candidates
                .into_iter()
                .filter(|cursor| {
                    document
                        .axes()
                        .iter()
                        .find(|axis| axis.id == cursor.axis_id)
                        .is_some_and(|axis| {
                            axis.orientation == AxisOrientation::Horizontal
                                && compatible(axis, reference_axis)
                        })
                })
                .collect::<Vec<_>>();
            let Some(position) = real_position(reference) else {
                continue;
            };
            if compatible.len() < 2 {
                continue;
            }
            for cursor in &compatible {
                if real_position(cursor)
                    .is_none_or(|current| current.to_bits() != position.to_bits())
                {
                    edits.push(DocumentEdit::MoveCursor {
                        cursor_id: cursor.id,
                        position: TypedValue::Real(position),
                    });
                }
            }
            edits.push(DocumentEdit::AddLinkGroup {
                label: format!("{label} cursor link"),
                kind: LinkKind::CursorPosition,
                members: compatible
                    .into_iter()
                    .map(|cursor| EntityRef::Cursor(cursor.id))
                    .collect(),
            });
            added += 1;
        }
        if added == 0 {
            return Err(
                "Place the same A or B cursor on at least two panes before linking it.".to_owned(),
            );
        }
    }
    if edits.is_empty() {
        return Ok(());
    }
    transact(state, document_id, edits).map(|_| ())
}

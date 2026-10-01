//! Borrowed geometry and visibility for the active schematic sheet.

use crate::session::{canvas_cache::CanvasCache, visibility::SchematicReviewMarkerVisibility};
use rspice_design::schematic::{
    design_note::{DesignNote, DesignNoteKind, DesignReviewState},
    document::SchematicDocument,
};
use rspice_design_model::{Point, design_management::SheetCatalog};
use std::borrow::Cow;

#[derive(Clone, Copy)]
pub struct DesignView<'a> {
    pub document: &'a SchematicDocument,
    pub canvas_cache: Option<&'a CanvasCache>,
    pub sheet_catalog: Option<&'a SheetCatalog>,
    pub review_markers: SchematicReviewMarkerVisibility,
}

/// Unassigned objects belong to the active sheet; legacy views without an
/// active sheet expose their whole document.
pub fn object_is_on_active_sheet(catalog: Option<&SheetCatalog>, object_id: u64) -> bool {
    let Some(catalog) = catalog else {
        return true;
    };
    let Some(active) = catalog.active_sheet_id() else {
        return true;
    };
    catalog.sheet_for_object(object_id).unwrap_or(active) == active
}

impl<'a> DesignView<'a> {
    pub fn object_is_visible(&self, object_id: u64) -> bool {
        object_is_on_active_sheet(self.sheet_catalog, object_id)
    }
    pub fn objects_on_active_sheet<'b, T: Clone>(
        &self,
        objects: &'b [T],
        id: impl Fn(&T) -> u64,
    ) -> Cow<'b, [T]> {
        if objects
            .iter()
            .all(|object| self.object_is_visible(id(object)))
        {
            Cow::Borrowed(objects)
        } else {
            Cow::Owned(
                objects
                    .iter()
                    .filter(|object| self.object_is_visible(id(object)))
                    .cloned()
                    .collect(),
            )
        }
    }

    pub fn active_wire_at(&self, point: Point) -> Option<u64> {
        if let Some(cache) = self.canvas_cache {
            return cache
                .wire_indices_at_point(point)
                .into_iter()
                .filter_map(|index| self.document.wires.get(index))
                .find(|wire| self.object_is_visible(wire.id) && wire.contains_point(point))
                .map(|wire| wire.id);
        }
        self.document.wires.iter().find_map(|wire| {
            (self.object_is_visible(wire.id) && wire.contains_point(point)).then_some(wire.id)
        })
    }

    pub fn active_junction_at(&self, point: Point) -> Option<u64> {
        self.document
            .junctions
            .iter()
            .find(|junction| junction.pos == point && self.object_is_visible(junction.id))
            .map(|junction| junction.id)
    }

    pub fn active_wire_point_is_draggable(&self, point: Point) -> bool {
        self.active_junction_at(point).is_some()
            || self
                .document
                .wires
                .iter()
                .any(|wire| self.object_is_visible(wire.id) && wire.points.contains(&point))
    }

    pub fn visible_design_notes(&self) -> Cow<'a, [DesignNote]> {
        let active = self.objects_on_active_sheet(&self.document.design_notes, |note| note.id);
        if self.review_markers == SchematicReviewMarkerVisibility::All {
            return active;
        }
        Cow::Owned(
            active
                .iter()
                .filter(|note| design_note_visible(note, self.review_markers))
                .cloned()
                .collect(),
        )
    }
}

pub fn design_note_visible(note: &DesignNote, visibility: SchematicReviewMarkerVisibility) -> bool {
    if note.kind != DesignNoteKind::ReviewNote {
        return true;
    }
    match visibility {
        SchematicReviewMarkerVisibility::Hidden => false,
        SchematicReviewMarkerVisibility::All => true,
        SchematicReviewMarkerVisibility::OpenAndAssigned => {
            note.review.as_ref().is_some_and(|review| {
                review.state == DesignReviewState::Open || review.assignee.is_some()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn review_marker_visibility_preserves_non_review_documentation() {
        let plain = DesignNote::new(
            10,
            Point::origin(),
            DesignNoteKind::PlainText,
            "Bias network",
        )
        .expect("plain note");
        let mut review = DesignNote::new(
            11,
            Point::origin(),
            DesignNoteKind::ReviewNote,
            "Check startup margin",
        )
        .expect("review note");

        assert!(design_note_visible(
            &plain,
            SchematicReviewMarkerVisibility::Hidden
        ));
        assert!(design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::OpenAndAssigned
        ));
        review
            .assign_review(Some("Analog design"))
            .expect("assign note");
        review
            .set_review_state(DesignReviewState::Resolved)
            .expect("resolve note");
        assert!(design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::OpenAndAssigned
        ));
        review
            .assign_review(None::<String>)
            .expect("clear assignment");
        assert!(!design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::OpenAndAssigned
        ));
        assert!(design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::All
        ));
        assert!(!design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::Hidden
        ));
    }
}

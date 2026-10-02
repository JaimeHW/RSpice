//! Borrowed geometry and visibility for the active schematic sheet.

use crate::session::{canvas_cache::CanvasCache, visibility::SchematicReviewMarkerVisibility};
use rspice_design::schematic::{
    design_note::{DesignNote, DesignNoteKind, DesignReviewState},
    document::SchematicDocument,
    junction_candidates::{collect_junction_candidates, nearest_junction_candidate},
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

    /// Only visible conductors contribute explicit-junction targets. A cache
    /// covering the whole document cannot establish active-sheet crossings.
    pub fn nearest_junction_candidate(&self, point: Point, radius: i32) -> Option<Point> {
        let wires = self.objects_on_active_sheet(&self.document.wires, |wire| wire.id);
        let candidates = collect_junction_candidates(wires.as_ref());
        nearest_junction_candidate(&candidates, point, radius)
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
    use rspice_design::schematic::wire::Wire;
    use rspice_design_model::design_management::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    fn view(document: &SchematicDocument) -> DesignView<'_> {
        DesignView {
            document,
            canvas_cache: None,
            sheet_catalog: None,
            review_markers: SchematicReviewMarkerVisibility::All,
        }
    }

    #[test]
    fn junction_candidates_use_only_active_sheet_conductors() {
        let document = SchematicDocument {
            wires: vec![
                Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
                Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
                Wire::new(3, vec![Point::new(0, 0), Point::new(40, 40)]),
            ],
            ..Default::default()
        };
        assert_eq!(
            view(&document).nearest_junction_candidate(Point::new(19, 21), 4),
            Some(Point::new(20, 20))
        );
        assert_eq!(
            view(&document).nearest_junction_candidate(Point::new(100, 100), 4),
            None
        );

        let mut catalog = SheetCatalog::default();
        let mut sheets = Vec::new();
        for page in [1, 2] {
            sheets.push(
                catalog
                    .create_sheet(
                        SheetDefinition {
                            name: format!("Sheet {page}"),
                            template: SheetTemplate::AnalogSchematic,
                            port_policy: SheetPortPolicy::TypedOffSheetPorts,
                            explicit_page_number: Some(page),
                        },
                        sheets.last().copied(),
                    )
                    .unwrap(),
            );
        }
        catalog
            .assign_objects(catalog.revision(), sheets[1], [2, 3])
            .unwrap();
        catalog.set_active(sheets[0]).unwrap();
        assert_eq!(
            DesignView {
                sheet_catalog: Some(&catalog),
                ..view(&document)
            }
            .nearest_junction_candidate(Point::new(20, 20), 4),
            None,
            "hidden conductors cannot create a visible crossing"
        );
        catalog
            .assign_objects(catalog.revision(), sheets[0], [2])
            .unwrap();
        assert_eq!(
            DesignView {
                sheet_catalog: Some(&catalog),
                ..view(&document)
            }
            .nearest_junction_candidate(Point::new(20, 20), 4),
            Some(Point::new(20, 20))
        );
    }

    #[test]
    fn endpoint_and_t_contacts_are_not_explicit_junction_targets() {
        let document = SchematicDocument {
            wires: vec![
                Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
                Wire::new(2, vec![Point::new(20, 20), Point::new(20, 40)]),
            ],
            ..Default::default()
        };
        assert_eq!(
            view(&document).nearest_junction_candidate(Point::new(20, 20), 4),
            None
        );
    }

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

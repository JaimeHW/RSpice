use super::EditHistory;
use crate::schematic::{
    component::Component, component_type::ComponentType, document::SchematicDocument,
    history::SchematicSnapshot, owned::Schematic,
};
use rspice_design_model::Point;
fn document_with(n: usize) -> SchematicDocument {
    SchematicDocument {
        grid_size: 10,
        components: (0..n)
            .map(|i| Component::new(i as u64, ComponentType::Resistor, Point::new(i as i32, 0)))
            .collect(),
        ..SchematicDocument::default()
    }
}
fn snapshot_with(n: usize) -> SchematicSnapshot {
    SchematicSnapshot::capture(&document_with(n))
}
#[test]
fn begin_edit_end_creates_exactly_one_entry() {
    let mut history = EditHistory::default();
    history.initialize();

    history.begin_from(snapshot_with(0), "Add R1");
    let created = history.end(&document_with(1));

    assert!(created);
    assert_eq!(history.committed.undo_count(), 1);
    assert_eq!(history.committed.redo_count(), 0);
    assert!(history.committed.can_undo());
    assert_eq!(history.committed.undo_description(), Some("Add R1"));
    assert!(history.pending_operation_id().is_none());
}

#[test]
fn begin_end_without_change_creates_no_entry() {
    let mut history = EditHistory::default();
    history.initialize();

    history.begin_from(snapshot_with(2), "No-op move");
    let created = history.end(&document_with(2));

    assert!(!created);
    assert_eq!(history.committed.undo_count(), 0);
    assert!(!history.committed.can_undo());
}

#[test]
fn end_without_begin_is_a_noop() {
    let mut history = EditHistory::default();
    history.initialize();

    assert!(!history.end(&document_with(1)));
    assert_eq!(history.committed.undo_count(), 0);
}

#[test]
fn cancel_discards_pending_operation() {
    let mut history = EditHistory::default();
    history.initialize();

    history.begin_from(snapshot_with(0), "Cancelled drag");
    assert!(history.pending_operation_id().is_some());
    assert!(history.cancel().is_some());

    assert!(history.pending_operation_id().is_none());
    // A later end_operation has no pending transaction to commit.
    assert!(!history.end(&document_with(1)));
    assert_eq!(history.committed.undo_count(), 0);
}

#[test]
fn nested_begin_joins_the_outer_atomic_transaction() {
    // A helper can open an undo scope inside an already-atomic caller.
    // The inner end only closes its nesting level; the outer end commits
    // one entry from the original before snapshot.
    let mut history = EditHistory::default();
    history.initialize();

    history.begin_from(snapshot_with(0), "First");
    history.begin_from(snapshot_with(1), "Second");
    assert!(!history.end(&document_with(2)));
    let created = history.end(&document_with(2));

    assert!(created);
    assert_eq!(history.committed.undo_count(), 1);
    assert_eq!(history.committed.undo_description(), Some("First"));
    let restored = history
        .committed
        .undo(snapshot_with(2), None)
        .unwrap()
        .snapshot;
    assert!(restored.is_equal(&snapshot_with(0)));
}

/// A baseline copied before its operation began stands in for the capture
/// only while the document is unchanged since: an edit in between leaves the
/// copy behind, and undo returns to the document the operation began on.
#[test]
fn a_prepared_baseline_stands_in_only_for_the_document_it_copied() {
    let one_part = || {
        Schematic::from_document(SchematicDocument {
            grid_size: 10,
            components: vec![Component::new(1, ComponentType::Resistor, Point::new(0, 0))],
            ..SchematicDocument::default()
        })
    };

    let mut unchanged = one_part();
    let prepared = unchanged.prepare_operation();
    unchanged.begin_prepared_operation(Some(prepared), "Move R1");
    unchanged.document.components[0].pos = Point::new(50, 0);
    assert!(unchanged.end_operation());
    assert!(unchanged.undo().is_some());
    assert_eq!(unchanged.document().components[0].pos, Point::new(0, 0));

    let mut changed = one_part();
    let prepared = changed.prepare_operation();
    changed.document.components.push(Component::new(
        2,
        ComponentType::Resistor,
        Point::new(70, 0),
    ));
    changed.begin_prepared_operation(Some(prepared), "Move R1");
    changed.document.components[0].pos = Point::new(50, 0);
    assert!(changed.end_operation());
    assert!(changed.undo().is_some());
    let parts: Vec<_> = changed
        .document()
        .components
        .iter()
        .map(|component| (component.id, component.pos))
        .collect();
    assert_eq!(
        parts,
        [(1, Point::new(0, 0)), (2, Point::new(70, 0))],
        "undo keeps the part added after the copy was made"
    );
}

#[test]
fn clear_resets_everything_including_initialized() {
    let mut history = EditHistory::default();
    history.initialize();
    history.begin_from(snapshot_with(0), "Add R1");
    history.end(&document_with(1));
    history.committed.undo(snapshot_with(1), None).unwrap();
    history.begin_from(snapshot_with(0), "Pending");

    history.clear();

    assert_eq!(history.committed.undo_count(), 0);
    assert_eq!(history.committed.redo_count(), 0);
    assert!(history.pending_operation_id().is_none());
    assert!(!history.committed.is_initialized());
}

fn edited(schematic: &mut Schematic, parts: usize) {
    schematic.begin_operation("Edit");
    schematic.apply_snapshot(&snapshot_with(parts));
    assert!(schematic.end_operation());
}

/// A commit names the content it leaves afresh; an undo gives the drawing
/// back the name it had before the edit, and a redo the name it had after.
#[test]
fn undo_and_redo_give_the_drawing_back_its_name() {
    let mut schematic = Schematic::from_document(document_with(1));
    edited(&mut schematic, 2);
    let after = schematic
        .history_state()
        .expect("a commit names what it leaves");

    assert!(schematic.undo().is_some());
    let before = schematic
        .history_state()
        .expect("an undo gives back the name of what it restores");
    assert_ne!(before, after, "an edit is content of a name of its own");
    assert!(schematic.redo().is_some());
    assert_eq!(schematic.history_state(), Some(after));
    assert!(schematic.undo().is_some());
    assert_eq!(schematic.history_state(), Some(before));

    edited(&mut schematic, 2);
    assert_ne!(
        schematic.history_state(),
        Some(after),
        "the same edit made again is a new name"
    );
}

/// An operation that changes nothing, or is cancelled, leaves the drawing
/// the name it began with.
#[test]
fn an_operation_that_leaves_the_drawing_as_it_was_keeps_its_name() {
    let mut schematic = Schematic::from_document(document_with(1));
    edited(&mut schematic, 2);
    let named = schematic.history_state();
    assert!(named.is_some());

    schematic.begin_operation("Nothing");
    assert!(!schematic.end_operation());
    assert_eq!(schematic.history_state(), named);

    schematic.begin_operation("Cancelled");
    schematic.apply_snapshot(&snapshot_with(3));
    assert!(schematic.cancel_operation().is_some());
    assert_eq!(schematic.history_state(), named);
}

/// An edit outside the history leaves the drawing without a name, and no
/// step recorded before it returns to a name: it restores the drawing it
/// holds, not what moved beside it.
#[test]
fn an_edit_outside_the_history_leaves_no_name_to_return_to() {
    let mut schematic = Schematic::from_document(document_with(1));
    edited(&mut schematic, 2);
    schematic.apply_snapshot(&snapshot_with(3));
    assert_eq!(schematic.history_state(), None);

    assert!(schematic.undo().is_some());
    assert_eq!(schematic.history_state(), None);
    assert!(schematic.redo().is_some());
    assert!(
        schematic.history_state().is_some(),
        "a redo returns to the content the undo named"
    );
}

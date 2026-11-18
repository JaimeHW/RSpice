//! The design projection held through a canvas gesture.
//!
//! The Design rails read the configured design's projection of the open
//! drawing on every frame: the navigator's hierarchy, nets and objects, the
//! inspector's nets and the strip's occurrences. A gesture moves the drawing
//! on every frame, and each move would project the open cell view again.
//! Through a gesture they read the projection from before it began, and the
//! design is projected again once, at the drop.

use std::cell::RefCell;
use std::sync::Arc;

use super::Model;
use crate::state::workspace::{ConfigurationExecutionPlanError, DesignProjection};

/// The projection the rails last read, or why the design did not resolve,
/// with the open view and the stamps of everything but the drawing it was
/// read under. Runtime only: a copy of the model reads its own.
#[derive(Debug, Default)]
pub struct HeldProjection(RefCell<Option<(HoldKey, Projected)>>);

type Projected = Result<Arc<DesignProjection>, ConfigurationExecutionPlanError>;

impl Clone for HeldProjection {
    fn clone(&self) -> Self {
        Self::default()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HoldKey {
    view: String,
    /// The workspace, the library catalog and the libraries it references.
    stamps: [u64; 3],
}

impl Model {
    /// The configured design's projection of the open drawing, as the
    /// workspace answers it, except through a canvas gesture: then the one
    /// read before the gesture moved the drawing, while nothing but the
    /// drawing has moved since. A design that did not resolve before the
    /// gesture is not projected again until it ends either.
    pub fn design_projection_through_gesture(&self) -> Projected {
        let key = HoldKey {
            view: self.workspace.content.active_view.key(),
            stamps: [
                self.workspace.content.stamp(),
                self.library_manager.content_stamp(),
                self.library_manager.references_stamp(),
            ],
        };
        if self.canvas_gesture_in_progress()
            && let Some((held_key, held)) = &*self.held_projection.0.borrow()
            && *held_key == key
        {
            return held.clone();
        }
        let projection = self.blame_unread_library(self.workspace.inspect_design_projection(
            &self.library_manager,
            &self.workspace.content.active_view,
            &self.schematic,
        ));
        *self.held_projection.0.borrow_mut() = Some((key, projection.clone()));
        projection
    }

    /// An inspected projection admitted for execution, as a reader states
    /// it: one that does not resolve because it places a cell of a library
    /// this session could not read says that library is what is missing,
    /// not a view of the cell.
    pub(crate) fn blame_unread_library(&self, inspected: Projected) -> Projected {
        let inspected = inspected?;
        let error = match Arc::clone(&inspected).into_execution() {
            Ok(projection) => return Ok(projection),
            Err(error) => error,
        };
        let unread = self.unread_libraries();
        let blamed = inspected
            .hierarchy_resolution()
            .bindings
            .iter()
            .filter(|binding| !binding.status.is_resolved())
            .find_map(|binding| {
                let library = &binding.reference.library;
                unread.get(library).map(|unread| unread.sentence.clone())
            });
        Err(blamed.map_or(error, ConfigurationExecutionPlanError::LibraryUnavailable))
    }
}

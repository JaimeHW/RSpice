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

/// The projection the rails last read, with the open view and the stamps of
/// everything but the drawing it was read under. Runtime only: a copy of
/// the model reads its own.
#[derive(Debug, Default)]
pub struct HeldProjection(RefCell<Option<(HoldKey, Arc<DesignProjection>)>>);

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
    /// drawing has moved since.
    pub fn design_projection_through_gesture(
        &self,
    ) -> Result<Arc<DesignProjection>, ConfigurationExecutionPlanError> {
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
            return Ok(Arc::clone(held));
        }
        let projection = self.workspace.design_projection(
            &self.library_manager,
            &self.workspace.content.active_view,
            &self.schematic,
        );
        *self.held_projection.0.borrow_mut() = projection
            .as_ref()
            .ok()
            .map(|projection| (key, Arc::clone(projection)));
        projection
    }
}

//! DC Annotation Layer Component
//!
//! Separate component for DC annotation overlay to ensure hooks are called
//! unconditionally at component level (not inside conditional RSX blocks).
//! This follows Dioxus/React rules of hooks.

use dioxus::prelude::*;

use crate::components::annotation_overlay::AnnotationOverlay;
use crate::state::dc_annotation::{Annotation, AnnotationMode};
use crate::state::{DocumentManager, SimulationState};

/// DC Annotation Layer - renders operating point annotations
///
/// This is a separate component to isolate use_context() calls from
/// conditional rendering paths, avoiding hooks ordering violations.
#[component]
pub fn DcAnnotationLayer(grid_size: i32, current_zoom: f64) -> Element {
    // Hooks are called unconditionally at component top level
    let sim_state: Signal<SimulationState> = use_context();
    let doc_manager: Signal<DocumentManager> = use_context();

    // Read states
    let sim_read = sim_state.read();
    let dc_state = &sim_read.dc_annotations;
    let mode = dc_state.mode;

    // Only render if mode is not hidden
    if matches!(mode, AnnotationMode::Hidden) {
        return rsx! {};
    }

    // Get visible annotations based on current mode
    let annotations: Vec<Annotation> = dc_state
        .visible_annotations()
        .into_iter()
        .cloned()
        .collect();
    let is_stale = dc_state.is_stale;

    // Render overlay if there are annotations
    if annotations.is_empty() {
        return rsx! {};
    }

    // Get current wires for live position tracking
    // When annotations have wire_id set, they'll look up current positions from this
    let wires = {
        let docs = doc_manager.read();
        docs.active().schematic.wires.clone()
    };

    rsx! {
        AnnotationOverlay {
            annotations: annotations,
            zoom: current_zoom,
            grid_size: grid_size,
            is_stale: is_stale,
            wires: wires,
        }
    }
}

//! Canvas Focus Management
//!
//! Provides a shared context for focusing the schematic canvas from any component.
//! This enables keyboard shortcuts to work immediately after selecting components
//! from the library panel, without requiring the user to click the canvas first.

use dioxus::prelude::*;
use std::rc::Rc;

/// Shared canvas focus state accessible via context
#[derive(Clone, Default)]
pub struct CanvasFocusState {
    /// The mounted canvas element, if available
    element: Option<Rc<MountedData>>,
}

impl CanvasFocusState {
    /// Create a new canvas focus state
    pub fn new() -> Self {
        Self { element: None }
    }

    /// Set the mounted canvas element
    pub fn set_element(&mut self, element: Rc<MountedData>) {
        self.element = Some(element);
    }

    /// Focus the canvas if available (best-effort, may fail silently on some elements)
    pub fn focus(&self) {
        if let Some(ref el) = self.element {
            let el = el.clone();
            spawn(async move {
                // Ignore result - focus is best-effort since some elements don't support it
                let _ = el.set_focus(true).await;
            });
        }
    }
}

/// Signal wrapper for canvas focus that can be used via context
pub type CanvasFocus = Signal<CanvasFocusState>;

/// Hook to access the canvas focus context
pub fn use_canvas_focus() -> CanvasFocus {
    use_context::<CanvasFocus>()
}

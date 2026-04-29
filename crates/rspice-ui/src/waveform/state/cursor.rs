use serde::{Deserialize, Serialize};

/// Cursor mode for waveform measurements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CursorMode {
    /// No cursors active
    #[default]
    None,
    /// Single cursor for point measurement
    Single,
    /// Dual cursors for delta measurement
    Delta,
}

/// Cursor state for waveform measurements
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CursorState {
    /// Current cursor mode
    pub mode: CursorMode,
    /// Position of cursor 1 (X coordinate in data space)
    pub cursor1_x: Option<f64>,
    /// Position of cursor 2 (X coordinate in data space)
    pub cursor2_x: Option<f64>,
    /// Whether cursor 1 is being dragged
    pub dragging_cursor1: bool,
    /// Whether cursor 2 is being dragged
    pub dragging_cursor2: bool,
}

impl CursorState {
    /// Place a cursor at the given X position
    ///
    /// If no cursor is active, places cursor 1.
    /// If cursor 1 is active, places cursor 2 and enters delta mode.
    pub fn place(&mut self, x: f64) {
        match self.mode {
            CursorMode::None => {
                self.cursor1_x = Some(x);
                self.mode = CursorMode::Single;
            }
            CursorMode::Single => {
                self.cursor2_x = Some(x);
                self.mode = CursorMode::Delta;
            }
            CursorMode::Delta => {
                // In delta mode, click replaces the second cursor
                self.cursor2_x = Some(x);
            }
        }
    }

    /// Clear all cursors
    pub fn clear(&mut self) {
        self.mode = CursorMode::None;
        self.cursor1_x = None;
        self.cursor2_x = None;
        self.dragging_cursor1 = false;
        self.dragging_cursor2 = false;
    }

    /// Get delta between cursors (if both are set)
    pub fn delta_x(&self) -> Option<f64> {
        match (self.cursor1_x, self.cursor2_x) {
            (Some(x1), Some(x2)) => Some((x2 - x1).abs()),
            _ => None,
        }
    }

    /// Check if any cursor is active
    pub fn is_active(&self) -> bool {
        self.cursor1_x.is_some()
    }
}

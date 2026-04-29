use serde::{Deserialize, Serialize};

/// Box selection state for zoom-to-region
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BoxSelection {
    /// Whether selection is in progress
    pub is_selecting: bool,
    /// Start X in data coordinates
    pub start_x: f64,
    /// Start Y in data coordinates
    pub start_y: f64,
    /// End X in data coordinates
    pub end_x: f64,
    /// End Y in data coordinates
    pub end_y: f64,
    /// Start X in screen coordinates (for rendering)
    pub screen_start_x: f64,
    /// Start Y in screen coordinates
    pub screen_start_y: f64,
    /// Plot rectangle (left, top, width, height) for global coordinate conversion
    pub plot_rect: (f64, f64, f64, f64),
}

impl BoxSelection {
    /// Start a new box selection
    pub fn start(
        &mut self,
        data_x: f64,
        data_y: f64,
        screen_x: f64,
        screen_y: f64,
        plot_rect: (f64, f64, f64, f64),
    ) {
        self.is_selecting = true;
        self.start_x = data_x;
        self.start_y = data_y;
        self.end_x = data_x;
        self.end_y = data_y;
        self.screen_start_x = screen_x;
        self.screen_start_y = screen_y;
        self.plot_rect = plot_rect;
    }

    /// Update the selection endpoint
    pub fn update(&mut self, data_x: f64, data_y: f64) {
        self.end_x = data_x;
        self.end_y = data_y;
    }

    /// Finish selection and return the selected region (x_min, x_max, y_min, y_max)
    ///
    /// Returns None if selection was too small (click without drag).
    pub fn finish(&mut self) -> Option<(f64, f64, f64, f64)> {
        if !self.is_selecting {
            return None;
        }

        self.is_selecting = false;

        let x_min = self.start_x.min(self.end_x);
        let x_max = self.start_x.max(self.end_x);
        let y_min = self.start_y.min(self.end_y);
        let y_max = self.start_y.max(self.end_y);

        // Require minimum selection size (avoid accidental zoom)
        let min_range = 1e-12;
        if (x_max - x_min) < min_range || (y_max - y_min) < min_range {
            return None;
        }

        Some((x_min, x_max, y_min, y_max))
    }

    /// Cancel the selection
    pub fn cancel(&mut self) {
        self.is_selecting = false;
    }
}

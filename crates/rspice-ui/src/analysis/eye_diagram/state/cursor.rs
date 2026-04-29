/// Cursor mode for eye viewer measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EyeCursorMode {
    /// No cursors active.
    #[default]
    None,
    /// Single cursor active.
    Single,
    /// Two cursors active (delta mode).
    Delta,
}

/// Cursor state for eye viewer measurements.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EyeCursorState {
    /// Current cursor mode.
    pub mode: EyeCursorMode,
    /// Cursor 1 time in seconds.
    pub cursor1_time_s: Option<f64>,
    /// Cursor 2 time in seconds.
    pub cursor2_time_s: Option<f64>,
}

impl EyeCursorState {
    /// Place cursor in waveform-style sequential behavior.
    pub fn place(&mut self, time_s: f64) {
        if !time_s.is_finite() {
            return;
        }
        match self.mode {
            EyeCursorMode::None => {
                self.cursor1_time_s = Some(time_s);
                self.mode = EyeCursorMode::Single;
            }
            EyeCursorMode::Single => {
                self.cursor2_time_s = Some(time_s);
                self.mode = EyeCursorMode::Delta;
            }
            EyeCursorMode::Delta => {
                self.cursor2_time_s = Some(time_s);
            }
        }
    }

    /// Clear both cursors.
    pub fn clear(&mut self) {
        self.mode = EyeCursorMode::None;
        self.cursor1_time_s = None;
        self.cursor2_time_s = None;
    }

    /// Delta time between two active cursors.
    pub fn delta_time(&self) -> Option<f64> {
        match (self.cursor1_time_s, self.cursor2_time_s) {
            (Some(a), Some(b)) => Some((b - a).abs()),
            _ => None,
        }
    }
}

/// Active eye-plot view range (time and voltage).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyeViewRange {
    /// Visible time minimum in seconds.
    pub time_min_s: f64,
    /// Visible time maximum in seconds.
    pub time_max_s: f64,
    /// Visible voltage minimum.
    pub voltage_min: f64,
    /// Visible voltage maximum.
    pub voltage_max: f64,
}

impl Default for EyeViewRange {
    fn default() -> Self {
        Self {
            time_min_s: 0.0,
            time_max_s: 1e-9,
            voltage_min: -0.5,
            voltage_max: 0.5,
        }
    }
}

impl EyeViewRange {
    /// Time span in seconds.
    pub fn time_span(self) -> f64 {
        self.time_max_s - self.time_min_s
    }

    /// Voltage span.
    pub fn voltage_span(self) -> f64 {
        self.voltage_max - self.voltage_min
    }

    /// Enforce finite non-degenerate range.
    pub fn sanitize(&mut self) {
        if !self.time_min_s.is_finite()
            || !self.time_max_s.is_finite()
            || self.time_max_s <= self.time_min_s
        {
            self.time_min_s = 0.0;
            self.time_max_s = 1e-9;
        }
        if !self.voltage_min.is_finite()
            || !self.voltage_max.is_finite()
            || self.voltage_max <= self.voltage_min
        {
            self.voltage_min = -0.5;
            self.voltage_max = 0.5;
        }
    }
}

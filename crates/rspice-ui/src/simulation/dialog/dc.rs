//! DC Analysis Configuration
//!
//! Configuration for DC sweep analysis (.dc).

// =============================================================================
// DC Sweep Source
// =============================================================================

/// DC sweep source type
#[derive(Debug, Clone, PartialEq)]
pub struct DcSweepSource {
    /// Source name (e.g., "VIN", "R1")
    pub name: String,
    /// Start value
    pub start: f64,
    /// Stop value
    pub stop: f64,
    /// Step value
    pub step: f64,
}

impl Default for DcSweepSource {
    /// Commercial simulator default: sweep VIN from 0 to 5V with 0.1V step
    fn default() -> Self {
        Self {
            name: "VIN".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.1,
        }
    }
}

impl DcSweepSource {
    /// Create new source
    pub fn new(name: &str, start: f64, stop: f64, step: f64) -> Self {
        Self {
            name: name.to_string(),
            start,
            stop,
            step,
        }
    }

    /// Number of points
    pub fn num_points(&self) -> u32 {
        if self.step.abs() < 1e-15 {
            return 1;
        }
        ((self.stop - self.start) / self.step).abs().ceil() as u32 + 1
    }

    /// Validate source
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Source name is required".to_string());
        }

        if self.step.abs() < 1e-15 {
            return Err("Step size cannot be zero".to_string());
        }

        if (self.stop > self.start && self.step < 0.0)
            || (self.stop < self.start && self.step > 0.0)
        {
            return Err("Step sign must match sweep direction".to_string());
        }

        Ok(())
    }

    /// Generate SPICE source string
    pub fn to_spice(&self) -> String {
        format!("{} {} {} {}", self.name, self.start, self.stop, self.step)
    }
}

// =============================================================================
// DC Config
// =============================================================================

/// DC analysis configuration
#[derive(Debug, Clone, Default)]
pub struct DcConfig {
    /// Primary sweep source
    pub source1: DcSweepSource,
    /// Optional secondary sweep source (nested sweep)
    pub source2: Option<DcSweepSource>,
    /// Save all nodes
    pub save_all: bool,
}

impl DcConfig {
    /// Create new config with single source
    pub fn new(source_name: &str, start: f64, stop: f64, step: f64) -> Self {
        Self {
            source1: DcSweepSource::new(source_name, start, stop, step),
            source2: None,
            save_all: true,
        }
    }

    /// Add second sweep source (nested)
    pub fn with_second_source(mut self, source: DcSweepSource) -> Self {
        self.source2 = Some(source);
        self
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(".dc {}", self.source1.to_spice());

        if let Some(ref src2) = self.source2 {
            cmd.push_str(&format!(" {}", src2.to_spice()));
        }

        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        self.source1.validate()?;

        if let Some(ref src2) = self.source2 {
            src2.validate()?;
        }

        Ok(())
    }

    /// Total number of analysis points
    pub fn total_points(&self) -> u32 {
        let p1 = self.source1.num_points();
        match &self.source2 {
            Some(src2) => p1 * src2.num_points(),
            None => p1,
        }
    }

    /// Has nested sweep?
    pub fn is_nested(&self) -> bool {
        self.source2.is_some()
    }
}

// =============================================================================
// Tests
// =============================================================================

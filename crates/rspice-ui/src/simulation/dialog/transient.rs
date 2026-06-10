//! Transient Analysis Configuration
//!
//! Configuration for time-domain transient analysis (.tran).


// =============================================================================
// Integration Method
// =============================================================================

/// Time integration method
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IntegrationMethod {
    /// Trapezoidal rule
    #[default]
    Trapezoidal,
    /// Backward Euler
    BackwardEuler,
    /// Gear method (order 2)
    Gear2,
    /// Variable order Gear
    GearVariable,
}

impl IntegrationMethod {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Trapezoidal => "Trapezoidal",
            Self::BackwardEuler => "Backward Euler",
            Self::Gear2 => "Gear-2",
            Self::GearVariable => "Gear Variable",
        }
    }

    /// All methods
    pub fn all() -> &'static [IntegrationMethod] {
        &[
            Self::Trapezoidal,
            Self::BackwardEuler,
            Self::Gear2,
            Self::GearVariable,
        ]
    }
}

// =============================================================================
// Transient Config
// =============================================================================

/// Transient analysis configuration
#[derive(Debug, Clone)]
pub struct TransientConfig {
    /// Stop time
    pub stop_time: f64,
    /// Start time for output
    pub start_time: f64,
    /// Maximum timestep
    pub max_step: Option<f64>,
    /// Integration method
    pub method: IntegrationMethod,
    /// Use initial conditions
    pub use_ic: bool,
    /// Skip initial operating point
    pub skip_op: bool,
    /// Enable UIC (use initial conditions)
    pub uic: bool,
    /// Relative tolerance
    pub reltol: f64,
    /// Absolute tolerance
    pub abstol: f64,
    /// Save all nodes
    pub save_all: bool,
    /// Strobe period (for periodic saving)
    pub strobe: Option<f64>,
}

impl Default for TransientConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-6,
            start_time: 0.0,
            max_step: None,
            method: IntegrationMethod::default(),
            use_ic: false,
            skip_op: false,
            uic: false,
            reltol: 1e-3,
            abstol: 1e-12,
            save_all: true,
            strobe: None,
        }
    }
}

impl TransientConfig {
    /// Create new config
    pub fn new(stop_time: f64) -> Self {
        Self {
            stop_time,
            ..Default::default()
        }
    }

    /// Set max step
    pub fn with_max_step(mut self, step: f64) -> Self {
        self.max_step = Some(step);
        self
    }

    /// Set method
    pub fn with_method(mut self, method: IntegrationMethod) -> Self {
        self.method = method;
        self
    }

    /// Set tolerances
    pub fn with_tolerances(mut self, reltol: f64, abstol: f64) -> Self {
        self.reltol = reltol;
        self.abstol = abstol;
        self
    }

    /// Enable UIC
    pub fn with_uic(mut self) -> Self {
        self.uic = true;
        self
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = ".tran ".to_string();

        if let Some(step) = self.max_step {
            cmd.push_str(&format!("{} ", format_time(step)));
        }

        cmd.push_str(&format_time(self.stop_time));

        if self.start_time > 0.0 {
            cmd.push_str(&format!(" {}", format_time(self.start_time)));
        }

        if self.uic {
            cmd.push_str(" uic");
        }

        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.stop_time <= 0.0 {
            return Err("Stop time must be positive".to_string());
        }

        if self.start_time < 0.0 {
            return Err("Start time cannot be negative".to_string());
        }

        if self.start_time >= self.stop_time {
            return Err("Start time must be less than stop time".to_string());
        }

        if let Some(step) = self.max_step {
            if step <= 0.0 {
                return Err("Max step must be positive".to_string());
            }
            if step > self.stop_time {
                return Err("Max step cannot exceed stop time".to_string());
            }
        }

        if self.reltol <= 0.0 || self.reltol >= 1.0 {
            return Err("Relative tolerance must be between 0 and 1".to_string());
        }

        if self.abstol <= 0.0 {
            return Err("Absolute tolerance must be positive".to_string());
        }

        Ok(())
    }
}


fn format_time(time: f64) -> String {
    let abs = time.abs();
    if abs >= 1.0 {
        format!("{}s", time)
    } else if abs >= 1e-3 {
        format!("{}ms", time * 1e3)
    } else if abs >= 1e-6 {
        format!("{}us", time * 1e6)
    } else if abs >= 1e-9 {
        format!("{}ns", time * 1e9)
    } else {
        format!("{}ps", time * 1e12)
    }
}

// =============================================================================
// Tests
// =============================================================================

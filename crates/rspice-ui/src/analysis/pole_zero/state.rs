//! Pole-Zero State Management
//!
//! Viewer state for pole-zero plot display.

use super::data::PoleZeroData;

// =============================================================================
// Display Options
// =============================================================================

/// What domain to display
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DomainType {
    /// S-domain (continuous-time, Laplace)
    #[default]
    SDomain,
    /// Z-domain (discrete-time)
    ZDomain,
}

impl DomainType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::SDomain => "s-domain",
            Self::ZDomain => "z-domain",
        }
    }

    /// All options
    pub fn all() -> &'static [DomainType] {
        &[Self::SDomain, Self::ZDomain]
    }
}

// =============================================================================
// Pole-Zero State
// =============================================================================

/// Complete pole-zero viewer state
#[derive(Debug, Clone)]
pub struct PoleZeroState {
    /// Pole-zero data sets
    pub datasets: Vec<PoleZeroData>,
    /// Selected dataset index
    pub selected: usize,
    /// Domain type
    pub domain: DomainType,
    /// Show grid
    pub show_grid: bool,
    /// Show unit circle (z-domain)
    pub show_unit_circle: bool,
    /// Show stability region shading
    pub show_stability_region: bool,
    /// Show dominant poles
    pub show_dominant: bool,
    /// Show root annotations
    pub show_annotations: bool,
    /// Equal axis scaling
    pub equal_axes: bool,
    /// Real axis range
    pub real_min: f64,
    pub real_max: f64,
    /// Imaginary axis range
    pub imag_min: f64,
    pub imag_max: f64,
    /// Auto-scale
    pub auto_scale: bool,
}

impl Default for PoleZeroState {
    fn default() -> Self {
        Self {
            datasets: Vec::new(),
            selected: 0,
            domain: DomainType::SDomain,
            show_grid: true,
            show_unit_circle: true,
            show_stability_region: true,
            show_dominant: true,
            show_annotations: true,
            equal_axes: true,
            real_min: -3.0,
            real_max: 1.0,
            imag_min: -2.0,
            imag_max: 2.0,
            auto_scale: true,
        }
    }
}

impl PoleZeroState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load data
    pub fn load_data(&mut self, data: PoleZeroData) {
        self.domain = if data.z_domain {
            DomainType::ZDomain
        } else {
            DomainType::SDomain
        };
        self.datasets = vec![data];
        self.selected = 0;
        self.update_auto_scale();
    }

    /// Add dataset
    pub fn add_data(&mut self, data: PoleZeroData) {
        self.datasets.push(data);
        self.update_auto_scale();
    }

    /// Clear all datasets
    pub fn clear(&mut self) {
        self.datasets.clear();
        self.selected = 0;
    }

    /// Current dataset
    pub fn current(&self) -> Option<&PoleZeroData> {
        self.datasets.get(self.selected)
    }

    /// Number of datasets
    pub fn dataset_count(&self) -> usize {
        self.datasets.len()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.datasets.is_empty()
    }

    /// Update auto-scale ranges
    pub fn update_auto_scale(&mut self) {
        if !self.auto_scale || self.datasets.is_empty() {
            return;
        }

        let mut real_min = f64::MAX;
        let mut real_max = f64::MIN;
        let mut imag_min = f64::MAX;
        let mut imag_max = f64::MIN;

        for data in &self.datasets {
            let (rmin, rmax) = data.real_range();
            let (imin, imax) = data.imag_range();
            real_min = real_min.min(rmin);
            real_max = real_max.max(rmax);
            imag_min = imag_min.min(imin);
            imag_max = imag_max.max(imax);
        }

        // Include origin and stability boundary
        match self.domain {
            DomainType::SDomain => {
                // Include jω axis
                real_min = real_min.min(-0.5);
                real_max = real_max.max(0.5);
            }
            DomainType::ZDomain => {
                // Include unit circle
                real_min = real_min.min(-1.5);
                real_max = real_max.max(1.5);
                imag_min = imag_min.min(-1.5);
                imag_max = imag_max.max(1.5);
            }
        }

        self.real_min = real_min;
        self.real_max = real_max;
        self.imag_min = imag_min;
        self.imag_max = imag_max;

        // Make square if equal axes
        if self.equal_axes {
            let real_range = self.real_max - self.real_min;
            let imag_range = self.imag_max - self.imag_min;
            let max_range = real_range.max(imag_range);

            let real_center = (self.real_max + self.real_min) / 2.0;
            let imag_center = (self.imag_max + self.imag_min) / 2.0;

            self.real_min = real_center - max_range / 2.0;
            self.real_max = real_center + max_range / 2.0;
            self.imag_min = imag_center - max_range / 2.0;
            self.imag_max = imag_center + max_range / 2.0;
        }
    }

    /// Set domain type
    pub fn set_domain(&mut self, domain: DomainType) {
        self.domain = domain;
        self.update_auto_scale();
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Toggle unit circle
    pub fn toggle_unit_circle(&mut self) {
        self.show_unit_circle = !self.show_unit_circle;
    }

    /// Toggle stability region
    pub fn toggle_stability_region(&mut self) {
        self.show_stability_region = !self.show_stability_region;
    }

    /// Toggle annotations
    pub fn toggle_annotations(&mut self) {
        self.show_annotations = !self.show_annotations;
    }

    /// Is current system stable?
    pub fn is_stable(&self) -> Option<bool> {
        self.current().map(|d| d.is_stable())
    }

    /// System order
    pub fn system_order(&self) -> Option<usize> {
        self.current().map(|d| d.system_order())
    }
}

// =============================================================================
// Tests
// =============================================================================


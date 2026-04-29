/// Eye mask for compliance testing
#[derive(Debug, Clone)]
pub struct EyeMask {
    /// Mask is enabled
    pub enabled: bool,
    /// Mask name (e.g., "100GBASE-KR4")
    pub name: String,
    /// Inner polygon (forbidden region)
    pub inner: MaskPolygon,
    /// Outer polygon (boundary)
    pub outer: Option<MaskPolygon>,
    /// Mask violation count
    pub violation_count: usize,
    /// Total samples tested
    pub total_samples: usize,
}

impl Default for EyeMask {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "Generic".to_string(),
            inner: MaskPolygon::default_inner(),
            outer: None,
            violation_count: 0,
            total_samples: 0,
        }
    }
}

impl EyeMask {
    /// Create new mask
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Check if a point violates the mask
    pub fn check_violation(&self, t_normalized: f64, v_normalized: f64) -> bool {
        self.inner.contains(t_normalized, v_normalized)
    }

    /// Get mask margin (minimum distance to mask)
    pub fn get_margin(&self) -> f64 {
        if self.total_samples == 0 {
            return 1.0;
        }
        1.0 - (self.violation_count as f64 / self.total_samples as f64)
    }

    /// Is mask passing (no violations)?
    pub fn is_passing(&self) -> bool {
        self.violation_count == 0
    }
}

/// Polygon for mask definition
#[derive(Debug, Clone, Default)]
pub struct MaskPolygon {
    /// Points as (t_normalized, v_normalized) pairs
    pub points: Vec<(f64, f64)>,
}

impl MaskPolygon {
    /// Default inner mask (hexagonal eye opening)
    pub fn default_inner() -> Self {
        Self {
            points: vec![
                (0.35, 0.0),
                (0.40, 0.25),
                (0.60, 0.25),
                (0.65, 0.0),
                (0.60, -0.25),
                (0.40, -0.25),
            ],
        }
    }

    /// Check if point is inside polygon (ray casting)
    pub fn contains(&self, x: f64, y: f64) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut inside = false;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let (xi, yi) = self.points[i];
            let (xj, yj) = self.points[j];

            if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
        }

        inside
    }
}

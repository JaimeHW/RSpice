/// Cache key for persistence rendering.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyePersistenceCacheKey {
    pub width: usize,
    pub height: usize,
    pub trace_count: usize,
    pub total_points: usize,
    pub ui_count: u32,
    pub time_min_s: f64,
    pub time_max_s: f64,
    pub voltage_min: f64,
    pub voltage_max: f64,
    pub decay_quantized: u16,
}

/// Cached persistence grid.
#[derive(Debug, Clone)]
pub struct EyePersistenceCache {
    pub key: EyePersistenceCacheKey,
    pub width: usize,
    pub height: usize,
    pub nonzero_bins: Vec<(usize, usize, u32)>,
    pub max_count: u32,
}

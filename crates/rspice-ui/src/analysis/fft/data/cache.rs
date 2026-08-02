//! FFT plan and window caches.
//!
//! Both are keyed by length so a repeated transform reuses its plan and its
//! window coefficients instead of rebuilding them each frame.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, PoisonError},
};

use once_cell::sync::Lazy;
use rustfft::{Fft, FftPlanner};

use crate::analysis::fft::window::{WindowFunction, generate_window};
#[derive(Debug, Clone)]
pub(super) struct WindowCacheEntry {
    pub(super) coefficients: Arc<[f64]>,
    pub(super) coherent_gain: f64,
    pub(super) equivalent_noise_bandwidth_bins: f64,
}

static WINDOW_CACHE: Lazy<Mutex<HashMap<(WindowFunction, usize), WindowCacheEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static FFT_PLAN_CACHE: Lazy<Mutex<HashMap<usize, Arc<dyn Fft<f64>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn cached_window(window: WindowFunction, length: usize) -> WindowCacheEntry {
    // Poison recovery: the map holds only completed entries (a panic in
    // generate_window happens before insert), so the data stays valid and
    // a panicked analysis thread must not take the render path down too.
    let mut cache = WINDOW_CACHE.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(entry) = cache.get(&(window, length)) {
        return entry.clone();
    }

    let coefficients = generate_window(window, length);
    let coefficient_sum = coefficients.iter().sum::<f64>();
    let coherent_gain = if length == 0 {
        0.0
    } else {
        coefficient_sum / length as f64
    };
    let sum_squares = coefficients
        .iter()
        .map(|coefficient| coefficient * coefficient)
        .sum::<f64>();
    let equivalent_noise_bandwidth_bins = if coefficient_sum.abs() <= f64::EPSILON {
        0.0
    } else {
        length as f64 * sum_squares / (coefficient_sum * coefficient_sum)
    };
    let entry = WindowCacheEntry {
        coefficients: Arc::from(coefficients),
        coherent_gain,
        equivalent_noise_bandwidth_bins,
    };
    cache.insert((window, length), entry.clone());
    entry
}

pub(super) fn cached_fft_plan(length: usize) -> Arc<dyn Fft<f64>> {
    let mut cache = FFT_PLAN_CACHE
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    if let Some(plan) = cache.get(&length) {
        return Arc::clone(plan);
    }

    let mut planner = FftPlanner::new();
    let plan = planner.plan_fft_forward(length);
    cache.insert(length, Arc::clone(&plan));
    plan
}

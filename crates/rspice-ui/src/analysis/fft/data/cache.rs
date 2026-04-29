use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;
use rustfft::{Fft, FftPlanner};

use crate::analysis::fft::window::{WindowFunction, generate_window};
#[derive(Debug, Clone)]
pub(super) struct WindowCacheEntry {
    pub(super) coefficients: Arc<[f64]>,
    pub(super) coherent_gain: f64,
}

static WINDOW_CACHE: Lazy<Mutex<HashMap<(WindowFunction, usize), WindowCacheEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static FFT_PLAN_CACHE: Lazy<Mutex<HashMap<usize, Arc<dyn Fft<f64>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn cached_window(window: WindowFunction, length: usize) -> WindowCacheEntry {
    let mut cache = WINDOW_CACHE.lock().expect("window cache lock poisoned");
    if let Some(entry) = cache.get(&(window, length)) {
        return entry.clone();
    }

    let coefficients = generate_window(window, length);
    let coherent_gain = if length == 0 {
        0.0
    } else {
        coefficients.iter().sum::<f64>() / length as f64
    };
    let entry = WindowCacheEntry {
        coefficients: Arc::from(coefficients),
        coherent_gain,
    };
    cache.insert((window, length), entry.clone());
    entry
}

pub(super) fn cached_fft_plan(length: usize) -> Arc<dyn Fft<f64>> {
    let mut cache = FFT_PLAN_CACHE.lock().expect("fft plan cache lock poisoned");
    if let Some(plan) = cache.get(&length) {
        return Arc::clone(plan);
    }

    let mut planner = FftPlanner::new();
    let plan = planner.plan_fft_forward(length);
    cache.insert(length, Arc::clone(&plan));
    plan
}

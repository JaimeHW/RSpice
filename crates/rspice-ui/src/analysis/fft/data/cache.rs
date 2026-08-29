//! FFT plan and window caches.
//!
//! Both are keyed by length so a repeated transform reuses its plan and its
//! window coefficients instead of rebuilding them each frame. Retention is
//! deliberately bounded even when a long-running session sees many lengths.

use std::sync::{Arc, Mutex, MutexGuard};

use once_cell::sync::Lazy;
use rspice_core::numerics::rustfft_qualification::qualify_rustfft_forward_length;
use rustfft::{Fft, FftPlanner};

use super::{FftAllocationStage, FftBuildError};
use crate::analysis::fft::window::{WindowFunction, try_generate_window};

const MAX_RETAINED_WINDOW_ENTRIES: usize = 16;
// 2,097,152 f64 coefficients retain at most 16 MiB of cache-owned coefficient
// payload; allocation metadata and Arcs still held by callers are outside it.
const MAX_RETAINED_WINDOW_COEFFICIENTS: usize = 2_097_152;
const MAX_RETAINED_FFT_PLAN_ENTRIES: usize = 8;
// RustFFT does not expose retained heap bytes. Length plus reported in-place
// scratch is a deterministic retention proxy, not an actual byte bound.
const MAX_RETAINED_FFT_PLAN_WEIGHT: usize = 2_097_152;

#[derive(Debug, Clone)]
pub(super) struct WindowCacheEntry {
    pub(super) coefficients: Arc<Vec<f64>>,
    pub(super) coherent_gain: f64,
    pub(super) equivalent_noise_bandwidth_bins: f64,
}

struct RetainedWindow {
    key: (WindowFunction, usize),
    coefficient_count: usize,
    last_used: u64,
    entry: WindowCacheEntry,
}

struct WindowCache {
    entries: [Option<RetainedWindow>; MAX_RETAINED_WINDOW_ENTRIES],
    retained_coefficients: usize,
    access_clock: u64,
    coefficient_limit: usize,
}

impl Default for WindowCache {
    fn default() -> Self {
        Self::with_coefficient_limit(MAX_RETAINED_WINDOW_COEFFICIENTS)
    }
}

impl WindowCache {
    fn with_coefficient_limit(coefficient_limit: usize) -> Self {
        Self {
            entries: std::array::from_fn(|_| None),
            retained_coefficients: 0,
            access_clock: 0,
            coefficient_limit,
        }
    }

    fn clear(&mut self) {
        self.entries = std::array::from_fn(|_| None);
        self.retained_coefficients = 0;
        self.access_clock = 0;
    }

    fn get_or_generate(
        &mut self,
        window: WindowFunction,
        length: usize,
        generate: impl FnOnce() -> Result<WindowCacheEntry, FftBuildError>,
    ) -> Result<WindowCacheEntry, FftBuildError> {
        let access = self.next_access();
        let key = (window, length);
        if let Some(retained) = self
            .entries
            .iter_mut()
            .flatten()
            .find(|retained| retained.key == key)
        {
            retained.last_used = access;
            return Ok(retained.entry.clone());
        }

        let entry = generate()?;
        let coefficient_count = entry.coefficients.len();
        if coefficient_count > self.coefficient_limit {
            return Ok(entry);
        }
        while self.entry_count() == MAX_RETAINED_WINDOW_ENTRIES
            || self.retained_coefficients > self.coefficient_limit - coefficient_count
        {
            if !self.evict_least_recently_used() {
                self.clear();
                break;
            }
        }
        let insertion_index = self.first_free_or_reset();
        self.entries[insertion_index] = Some(RetainedWindow {
            key,
            coefficient_count,
            last_used: access,
            entry: entry.clone(),
        });
        self.retained_coefficients += coefficient_count;
        Ok(entry)
    }

    fn next_access(&mut self) -> u64 {
        let Some(next) = self.access_clock.checked_add(1) else {
            self.clear();
            self.access_clock = 1;
            return 1;
        };
        self.access_clock = next;
        next
    }

    fn evict_least_recently_used(&mut self) -> bool {
        let Some(index) = least_recently_used_index(&self.entries, |entry| entry.last_used) else {
            return false;
        };
        self.entries[index] = None;
        self.retained_coefficients = self
            .entries
            .iter()
            .flatten()
            .map(|entry| entry.coefficient_count)
            .sum();
        true
    }

    fn first_free_or_reset(&mut self) -> usize {
        if let Some(index) = self.entries.iter().position(Option::is_none) {
            index
        } else {
            self.clear();
            0
        }
    }

    fn entry_count(&self) -> usize {
        self.entries.iter().flatten().count()
    }

    #[cfg(test)]
    fn contains(&self, window: WindowFunction, length: usize) -> bool {
        self.entries
            .iter()
            .flatten()
            .any(|entry| entry.key == (window, length))
    }
}

struct RetainedFftPlan {
    length: usize,
    weight: usize,
    last_used: u64,
    plan: Arc<dyn Fft<f64>>,
}

struct FftPlanCache {
    entries: [Option<RetainedFftPlan>; MAX_RETAINED_FFT_PLAN_ENTRIES],
    retained_weight: usize,
    access_clock: u64,
    weight_limit: usize,
}

impl Default for FftPlanCache {
    fn default() -> Self {
        Self::with_weight_limit(MAX_RETAINED_FFT_PLAN_WEIGHT)
    }
}

impl FftPlanCache {
    fn with_weight_limit(weight_limit: usize) -> Self {
        Self {
            entries: std::array::from_fn(|_| None),
            retained_weight: 0,
            access_clock: 0,
            weight_limit,
        }
    }

    fn clear(&mut self) {
        self.entries = std::array::from_fn(|_| None);
        self.retained_weight = 0;
        self.access_clock = 0;
    }

    fn get_or_plan(
        &mut self,
        length: usize,
        plan: impl FnOnce() -> Arc<dyn Fft<f64>>,
    ) -> Arc<dyn Fft<f64>> {
        let access = self.next_access();
        if let Some(retained) = self
            .entries
            .iter_mut()
            .flatten()
            .find(|retained| retained.length == length)
        {
            retained.last_used = access;
            return Arc::clone(&retained.plan);
        }

        let plan = plan();
        let Some(weight) = length.checked_add(plan.get_inplace_scratch_len()) else {
            return plan;
        };
        if weight > self.weight_limit {
            return plan;
        }
        while self.entry_count() == MAX_RETAINED_FFT_PLAN_ENTRIES
            || self.retained_weight > self.weight_limit - weight
        {
            if !self.evict_least_recently_used() {
                self.clear();
                break;
            }
        }
        let insertion_index = self.first_free_or_reset();
        self.entries[insertion_index] = Some(RetainedFftPlan {
            length,
            weight,
            last_used: access,
            plan: Arc::clone(&plan),
        });
        self.retained_weight += weight;
        plan
    }

    fn next_access(&mut self) -> u64 {
        let Some(next) = self.access_clock.checked_add(1) else {
            self.clear();
            self.access_clock = 1;
            return 1;
        };
        self.access_clock = next;
        next
    }

    fn evict_least_recently_used(&mut self) -> bool {
        let Some(index) = least_recently_used_index(&self.entries, |entry| entry.last_used) else {
            return false;
        };
        self.entries[index] = None;
        self.retained_weight = self
            .entries
            .iter()
            .flatten()
            .map(|entry| entry.weight)
            .sum();
        true
    }

    fn first_free_or_reset(&mut self) -> usize {
        if let Some(index) = self.entries.iter().position(Option::is_none) {
            index
        } else {
            self.clear();
            0
        }
    }

    fn entry_count(&self) -> usize {
        self.entries.iter().flatten().count()
    }

    #[cfg(test)]
    fn contains(&self, length: usize) -> bool {
        self.entries
            .iter()
            .flatten()
            .any(|entry| entry.length == length)
    }
}

fn least_recently_used_index<T, const N: usize>(
    entries: &[Option<T>; N],
    age: impl Fn(&T) -> u64,
) -> Option<usize> {
    entries
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| entry.as_ref().map(|entry| (index, age(entry))))
        .min_by_key(|(_, age)| *age)
        .map(|(index, _)| index)
}

static WINDOW_CACHE: Lazy<Mutex<WindowCache>> = Lazy::new(|| Mutex::new(WindowCache::default()));
static FFT_PLAN_CACHE: Lazy<Mutex<FftPlanCache>> =
    Lazy::new(|| Mutex::new(FftPlanCache::default()));

fn lock_window_cache(cache: &Mutex<WindowCache>) -> MutexGuard<'_, WindowCache> {
    match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            let mut guard = poisoned.into_inner();
            *guard = WindowCache::default();
            cache.clear_poison();
            guard
        }
    }
}

fn lock_fft_plan_cache(cache: &Mutex<FftPlanCache>) -> MutexGuard<'_, FftPlanCache> {
    match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            let mut guard = poisoned.into_inner();
            *guard = FftPlanCache::default();
            cache.clear_poison();
            guard
        }
    }
}

fn generate_window_entry(
    window: WindowFunction,
    length: usize,
) -> Result<WindowCacheEntry, FftBuildError> {
    let coefficients =
        try_generate_window(window, length).map_err(|_| FftBuildError::Allocation {
            stage: FftAllocationStage::WindowCoefficients,
            requested: length,
        })?;
    let coefficient_sum = compensated_sum(coefficients.iter().copied());
    let coherent_gain = if length == 0 {
        0.0
    } else {
        coefficient_sum / length as f64
    };
    let sum_squares = compensated_sum(
        coefficients
            .iter()
            .map(|coefficient| coefficient * coefficient),
    );
    let equivalent_noise_bandwidth_bins = if coefficient_sum.abs() <= f64::EPSILON {
        0.0
    } else {
        length as f64 * sum_squares / (coefficient_sum * coefficient_sum)
    };
    Ok(WindowCacheEntry {
        // Moving the Vec into an Arc avoids Arc<[T]>'s second large
        // infallible allocation/copy. Only the small Arc control block remains
        // outside stable Rust's fallible-allocation surface.
        coefficients: Arc::new(coefficients),
        coherent_gain,
        equivalent_noise_bandwidth_bins,
    })
}

fn compensated_sum(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut sum = 0.0;
    let mut correction = 0.0;
    for value in values {
        let next = sum + value;
        correction += if sum.abs() >= value.abs() {
            (sum - next) + value
        } else {
            (value - next) + sum
        };
        sum = next;
    }
    sum + correction
}

pub(super) fn cached_window(
    window: WindowFunction,
    length: usize,
) -> Result<WindowCacheEntry, FftBuildError> {
    // Generation remains under the lock so concurrent misses cannot multiply
    // the largest retained allocation. Returned Arc data outlives eviction.
    lock_window_cache(&WINDOW_CACHE)
        .get_or_generate(window, length, || generate_window_entry(window, length))
}

pub(super) fn cached_fft_plan(length: usize) -> Result<Arc<dyn Fft<f64>>, FftBuildError> {
    qualify_rustfft_forward_length(length)?;
    // Planning remains under the lock to prevent duplicate infallible rustfft
    // allocations. The Arc is returned after the guard drops, so transforms
    // never execute while holding the cache lock.
    Ok(lock_fft_plan_cache(&FFT_PLAN_CACHE)
        .get_or_plan(length, || FftPlanner::new().plan_fft_forward(length)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustfft::num_complex::Complex;

    fn local_window(
        cache: &mut WindowCache,
        window: WindowFunction,
        length: usize,
    ) -> WindowCacheEntry {
        cache
            .get_or_generate(window, length, || generate_window_entry(window, length))
            .expect("qualified local window fixture")
    }

    fn local_plan(cache: &mut FftPlanCache, length: usize) -> Arc<dyn Fft<f64>> {
        cache.get_or_plan(length, || FftPlanner::new().plan_fft_forward(length))
    }

    #[test]
    fn window_cache_reuses_exact_keys_only() {
        let mut cache = WindowCache::default();
        let first = local_window(&mut cache, WindowFunction::Hanning, 32);
        let reused = local_window(&mut cache, WindowFunction::Hanning, 32);
        let other_window = local_window(&mut cache, WindowFunction::Hamming, 32);
        let other_length = local_window(&mut cache, WindowFunction::Hanning, 33);

        assert!(Arc::ptr_eq(&first.coefficients, &reused.coefficients));
        assert!(!Arc::ptr_eq(
            &first.coefficients,
            &other_window.coefficients
        ));
        assert!(!Arc::ptr_eq(
            &first.coefficients,
            &other_length.coefficients
        ));
    }

    #[test]
    fn window_cache_evicts_deterministically_by_count_and_weight() {
        let mut cache = WindowCache::default();
        let evicted_but_live = local_window(&mut cache, WindowFunction::Rectangular, 2);
        for length in 3..=17 {
            local_window(&mut cache, WindowFunction::Rectangular, length);
        }
        local_window(&mut cache, WindowFunction::Rectangular, 3);
        local_window(&mut cache, WindowFunction::Rectangular, 18);
        assert!(!cache.contains(WindowFunction::Rectangular, 2));
        assert!(cache.contains(WindowFunction::Rectangular, 3));
        assert_eq!(cache.entry_count(), MAX_RETAINED_WINDOW_ENTRIES);
        assert_eq!(
            cache.retained_coefficients,
            cache
                .entries
                .iter()
                .flatten()
                .map(|entry| entry.coefficient_count)
                .sum::<usize>()
        );
        assert_eq!(evicted_but_live.coefficients.as_ref(), &[1.0, 1.0]);

        let mut weighted = WindowCache::with_coefficient_limit(7);
        local_window(&mut weighted, WindowFunction::Rectangular, 4);
        local_window(&mut weighted, WindowFunction::Rectangular, 5);
        assert!(!weighted.contains(WindowFunction::Rectangular, 4));
        assert!(weighted.contains(WindowFunction::Rectangular, 5));
        assert_eq!(weighted.retained_coefficients, 5);

        let mut oversized = WindowCache::with_coefficient_limit(3);
        let entry = local_window(&mut oversized, WindowFunction::Rectangular, 4);
        assert_eq!(entry.coefficients.len(), 4);
        assert_eq!(oversized.entry_count(), 0);
    }

    #[test]
    fn plan_cache_reuses_exact_lengths_and_evicted_arcs_remain_usable() {
        let mut cache = FftPlanCache::default();
        let first = local_plan(&mut cache, 8);
        let reused = local_plan(&mut cache, 8);
        let evicted_but_live = local_plan(&mut cache, 9);
        assert!(Arc::ptr_eq(&first, &reused));
        assert!(!Arc::ptr_eq(&first, &evicted_but_live));

        for length in 10..=15 {
            local_plan(&mut cache, length);
        }
        local_plan(&mut cache, 8);
        local_plan(&mut cache, 16);
        assert!(cache.contains(8));
        assert!(!cache.contains(9));
        assert_eq!(cache.entry_count(), MAX_RETAINED_FFT_PLAN_ENTRIES);
        assert_eq!(
            cache.retained_weight,
            cache
                .entries
                .iter()
                .flatten()
                .map(|entry| entry.weight)
                .sum::<usize>()
        );

        let mut buffer = vec![Complex::new(0.0, 0.0); 9];
        buffer[1].re = 1.0;
        let mut scratch = vec![Complex::new(0.0, 0.0); evicted_but_live.get_inplace_scratch_len()];
        evicted_but_live.process_with_scratch(&mut buffer, &mut scratch);
        assert!(
            buffer
                .iter()
                .all(|value| value.re.is_finite() && value.im.is_finite())
        );
    }

    #[test]
    fn plan_cache_enforces_weight_and_skips_oversized_entries() {
        let plan8 = FftPlanner::<f64>::new().plan_fft_forward(8);
        let plan9 = FftPlanner::<f64>::new().plan_fft_forward(9);
        let weight8 = 8 + plan8.get_inplace_scratch_len();
        let weight9 = 9 + plan9.get_inplace_scratch_len();
        let mut cache = FftPlanCache::with_weight_limit(weight8.max(weight9));
        local_plan(&mut cache, 8);
        local_plan(&mut cache, 9);
        assert!(cache.entry_count() <= 1);
        assert!(cache.contains(9));
        assert_eq!(cache.retained_weight, weight9);

        let mut oversized = FftPlanCache::with_weight_limit(weight8 - 1);
        let plan = local_plan(&mut oversized, 8);
        assert_eq!(plan.len(), 8);
        assert_eq!(oversized.entry_count(), 0);
    }

    #[test]
    fn poisoned_caches_reset_and_resume_operation() {
        let windows = Mutex::new(WindowCache::default());
        let window_poison = std::panic::catch_unwind(|| {
            let mut guard = windows.lock().expect("fresh window-cache lock");
            local_window(&mut guard, WindowFunction::Rectangular, 8);
            panic!("intentional window-cache poison");
        });
        assert!(window_poison.is_err());
        let mut guard = lock_window_cache(&windows);
        assert_eq!(guard.entry_count(), 0);
        let recovered = local_window(&mut guard, WindowFunction::Rectangular, 8);
        assert_eq!(recovered.coefficients.len(), 8);
        drop(guard);
        assert!(!windows.is_poisoned());

        let plans = Mutex::new(FftPlanCache::default());
        let plan_poison = std::panic::catch_unwind(|| {
            let mut guard = plans.lock().expect("fresh plan-cache lock");
            local_plan(&mut guard, 8);
            panic!("intentional plan-cache poison");
        });
        assert!(plan_poison.is_err());
        let mut guard = lock_fft_plan_cache(&plans);
        assert_eq!(guard.entry_count(), 0);
        assert_eq!(local_plan(&mut guard, 8).len(), 8);
        drop(guard);
        assert!(!plans.is_poisoned());
    }

    #[test]
    fn access_clock_overflow_resets_entries_without_invalidating_returned_arcs() {
        let mut windows = WindowCache::default();
        let old_window = local_window(&mut windows, WindowFunction::Rectangular, 8);
        windows.access_clock = u64::MAX;
        local_window(&mut windows, WindowFunction::Rectangular, 9);
        assert_eq!(windows.access_clock, 1);
        assert_eq!(windows.entry_count(), 1);
        assert!(!windows.contains(WindowFunction::Rectangular, 8));
        assert!(windows.contains(WindowFunction::Rectangular, 9));
        assert_eq!(old_window.coefficients.as_slice(), &[1.0; 8]);

        let mut plans = FftPlanCache::default();
        let old_plan = local_plan(&mut plans, 8);
        plans.access_clock = u64::MAX;
        local_plan(&mut plans, 9);
        assert_eq!(plans.access_clock, 1);
        assert_eq!(plans.entry_count(), 1);
        assert!(!plans.contains(8));
        assert!(plans.contains(9));

        let mut buffer = vec![Complex::new(0.0, 0.0); 8];
        buffer[1].re = 1.0;
        let mut scratch = vec![Complex::new(0.0, 0.0); old_plan.get_inplace_scratch_len()];
        old_plan.process_with_scratch(&mut buffer, &mut scratch);
        assert!(
            buffer
                .iter()
                .all(|value| value.re.is_finite() && value.im.is_finite())
        );
    }
}

//! State Synchronization
//!
//! Keep state synchronized across multiple views and components.
//! Provides reactive state updates with change notifications.
//!
//! # Features
//!
//! - Shared state container
//! - Change listeners
//! - Atomic updates
//! - State history/undo

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use super::lock::{read_lock, write_lock};

// =============================================================================
// Sync State Container
// =============================================================================

type SyncListener<T> = Arc<dyn Fn(&T) + Send + Sync>;

/// A synchronized state value
#[derive(Clone)]
pub struct SyncState<T> {
    /// Current value
    value: Arc<RwLock<T>>,
    /// Change listeners
    listeners: Arc<RwLock<Vec<SyncListener<T>>>>,
    /// Version counter
    version: Arc<AtomicU64>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for SyncState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = read_lock(&self.value, "SyncState::fmt(value)");
        let version = self.version.load(Ordering::Relaxed);
        let listener_count = read_lock(&self.listeners, "SyncState::fmt(listeners)").len();

        f.debug_struct("SyncState")
            .field("value", &*value)
            .field("version", &version)
            .field("listener_count", &listener_count)
            .finish()
    }
}

impl<T: Clone + Send + Sync + 'static> SyncState<T> {
    /// Create new sync state
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial)),
            listeners: Arc::new(RwLock::new(Vec::new())),
            version: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get current value
    pub fn get(&self) -> T {
        let val = read_lock(&self.value, "SyncState::get");
        val.clone()
    }

    /// Set value and notify listeners
    pub fn set(&self, new_value: T) {
        // Update value
        {
            let mut val = write_lock(&self.value, "SyncState::set(value)");
            *val = new_value;
        }

        // Increment version
        self.version.fetch_add(1, Ordering::Relaxed);

        // Notify listeners
        self.notify();
    }

    /// Update value with function
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut val = write_lock(&self.value, "SyncState::update(value)");
            f(&mut *val);
        }

        self.version.fetch_add(1, Ordering::Relaxed);

        self.notify();
    }

    /// Get current version
    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }

    /// Add change listener
    pub fn on_change<F>(&self, listener: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut listeners = write_lock(&self.listeners, "SyncState::on_change(listeners)");
        listeners.push(Arc::new(listener));
    }

    /// Notify all listeners
    fn notify(&self) {
        // Snapshot listener list and value before invoking callbacks to avoid lock re-entrancy
        // deadlocks when listeners mutate state or listener registrations.
        let val = self.get();
        let listeners = read_lock(&self.listeners, "SyncState::notify(listeners)").clone();

        for listener in listeners {
            listener(&val);
        }
    }

    /// Clear all listeners
    pub fn clear_listeners(&self) {
        let mut listeners = write_lock(&self.listeners, "SyncState::clear_listeners");
        listeners.clear();
    }
}

impl<T: Default + Clone + Send + Sync + 'static> Default for SyncState<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// =============================================================================
// State Synchronizer
// =============================================================================

/// Manages multiple named states
pub struct StateSynchronizer {
    /// Named selection states
    selections: HashMap<String, SyncState<Vec<String>>>,
    /// Named value states  
    values: HashMap<String, SyncState<f64>>,
    /// Named flag states
    flags: HashMap<String, SyncState<bool>>,
    /// String states
    strings: HashMap<String, SyncState<String>>,
}

impl Default for StateSynchronizer {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSynchronizer {
    /// Create new synchronizer
    pub fn new() -> Self {
        Self {
            selections: HashMap::new(),
            values: HashMap::new(),
            flags: HashMap::new(),
            strings: HashMap::new(),
        }
    }

    // =========================================================================
    // Selection States
    // =========================================================================

    /// Get or create selection state
    pub fn selection(&mut self, name: &str) -> &SyncState<Vec<String>> {
        self.selections
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(Vec::new()))
    }

    /// Set selection
    pub fn set_selection(&mut self, name: &str, items: Vec<String>) {
        let state = self
            .selections
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(Vec::new()));
        state.set(items);
    }

    /// Get selection
    pub fn get_selection(&self, name: &str) -> Vec<String> {
        self.selections
            .get(name)
            .map(|s| s.get())
            .unwrap_or_default()
    }

    /// Add to selection
    pub fn add_to_selection(&mut self, name: &str, item: String) {
        let state = self
            .selections
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(Vec::new()));

        state.update(|items| {
            if !items.contains(&item) {
                items.push(item);
            }
        });
    }

    /// Remove from selection
    pub fn remove_from_selection(&mut self, name: &str, item: &str) {
        if let Some(state) = self.selections.get(name) {
            state.update(|items| {
                items.retain(|x| x != item);
            });
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self, name: &str) {
        if let Some(state) = self.selections.get(name) {
            state.set(Vec::new());
        }
    }

    // =========================================================================
    // Value States
    // =========================================================================

    /// Get or create value state
    pub fn value(&mut self, name: &str) -> &SyncState<f64> {
        self.values
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(0.0))
    }

    /// Set value
    pub fn set_value(&mut self, name: &str, val: f64) {
        let state = self
            .values
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(0.0));
        state.set(val);
    }

    /// Get value
    pub fn get_value(&self, name: &str) -> f64 {
        self.values.get(name).map(|s| s.get()).unwrap_or(0.0)
    }

    // =========================================================================
    // Flag States
    // =========================================================================

    /// Get or create flag state
    pub fn flag(&mut self, name: &str) -> &SyncState<bool> {
        self.flags
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(false))
    }

    /// Set flag
    pub fn set_flag(&mut self, name: &str, val: bool) {
        let state = self
            .flags
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(false));
        state.set(val);
    }

    /// Get flag
    pub fn get_flag(&self, name: &str) -> bool {
        self.flags.get(name).map(|s| s.get()).unwrap_or(false)
    }

    /// Toggle flag
    pub fn toggle_flag(&mut self, name: &str) {
        let current = self.get_flag(name);
        self.set_flag(name, !current);
    }

    // =========================================================================
    // String States
    // =========================================================================

    /// Set string
    pub fn set_string(&mut self, name: &str, val: String) {
        let state = self
            .strings
            .entry(name.to_string())
            .or_insert_with(|| SyncState::new(String::new()));
        state.set(val);
    }

    /// Get string
    pub fn get_string(&self, name: &str) -> String {
        self.strings.get(name).map(|s| s.get()).unwrap_or_default()
    }
}

// =============================================================================
// Common Synchronized States
// =============================================================================

/// Pre-defined state keys for common use cases
pub mod keys {
    /// Currently selected components in schematic
    pub const SCHEMATIC_SELECTION: &str = "schematic.selection";
    /// Currently selected signals in waveform
    pub const WAVEFORM_SELECTION: &str = "waveform.selection";
    /// Highlighted signals (for cross-probing)
    pub const HIGHLIGHTED_SIGNALS: &str = "signals.highlighted";
    /// Current time cursor position
    pub const TIME_CURSOR: &str = "cursor.time";
    /// Current frequency cursor
    pub const FREQ_CURSOR: &str = "cursor.frequency";
    /// Simulation running flag
    pub const SIM_RUNNING: &str = "simulation.running";
    /// Current temperature
    pub const TEMPERATURE: &str = "simulation.temperature";
    /// Current corner
    pub const CORNER: &str = "simulation.corner";
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // =========================================================================
    // SyncState Tests
    // =========================================================================

    #[test]
    fn test_sync_state_new() {
        let state = SyncState::new(42);
        assert_eq!(state.get(), 42);
        assert_eq!(state.version(), 0);
    }

    #[test]
    fn test_sync_state_set() {
        let state = SyncState::new(0);
        state.set(100);
        assert_eq!(state.get(), 100);
        assert_eq!(state.version(), 1);
    }

    #[test]
    fn test_sync_state_update() {
        let state = SyncState::new(vec![1, 2, 3]);
        state.update(|v| v.push(4));
        assert_eq!(state.get(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sync_state_listener() {
        let state = SyncState::new(0);
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        state.on_change(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        state.set(1);
        state.set(2);

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_sync_state_clear_listeners() {
        let state = SyncState::new(0);
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        state.on_change(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        state.set(1);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        state.clear_listeners();
        state.set(2);
        assert_eq!(counter.load(Ordering::SeqCst), 1); // No increment
    }

    #[test]
    fn test_sync_state_listener_can_reenter_state_update() {
        let state = SyncState::new(0_u64);
        let state_clone = state.clone();

        state.on_change(move |value| {
            if *value == 1 {
                state_clone.set(2);
            }
        });

        state.set(1);

        assert_eq!(state.get(), 2);
        assert_eq!(state.version(), 2);
    }

    #[test]
    fn test_sync_state_listener_can_clear_listeners_during_callback() {
        let state = SyncState::new(0_u64);
        let state_clone = state.clone();
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = calls.clone();

        state.on_change(move |_| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            state_clone.clear_listeners();
        });

        state.set(1);
        state.set(2);

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_sync_state_recovers_from_poisoned_value_lock() {
        let state = SyncState::new(7_i32);
        let poison_state = state.clone();
        let _ = std::thread::spawn(move || {
            let _guard = poison_state
                .value
                .write()
                .expect("value lock should be writable before poison");
            panic!("intentional lock poison for recovery test");
        })
        .join();

        assert_eq!(state.get(), 7);
        state.set(11);
        assert_eq!(state.get(), 11);
    }

    // =========================================================================
    // StateSynchronizer Tests
    // =========================================================================

    #[test]
    fn test_synchronizer_selection() {
        let mut sync = StateSynchronizer::new();

        sync.set_selection("test", vec!["a".to_string(), "b".to_string()]);
        assert_eq!(sync.get_selection("test"), vec!["a", "b"]);

        sync.add_to_selection("test", "c".to_string());
        assert_eq!(sync.get_selection("test").len(), 3);

        sync.remove_from_selection("test", "b");
        assert_eq!(sync.get_selection("test"), vec!["a", "c"]);

        sync.clear_selection("test");
        assert!(sync.get_selection("test").is_empty());
    }

    #[test]
    fn test_synchronizer_values() {
        let mut sync = StateSynchronizer::new();

        sync.set_value("temp", 27.0);
        assert_eq!(sync.get_value("temp"), 27.0);

        assert_eq!(sync.get_value("nonexistent"), 0.0);
    }

    #[test]
    fn test_synchronizer_flags() {
        let mut sync = StateSynchronizer::new();

        assert!(!sync.get_flag("test"));

        sync.set_flag("test", true);
        assert!(sync.get_flag("test"));

        sync.toggle_flag("test");
        assert!(!sync.get_flag("test"));
    }

    #[test]
    fn test_synchronizer_strings() {
        let mut sync = StateSynchronizer::new();

        sync.set_string("corner", "tt".to_string());
        assert_eq!(sync.get_string("corner"), "tt");

        assert!(sync.get_string("nonexistent").is_empty());
    }

    #[test]
    fn test_synchronizer_version_tracking() {
        let mut sync = StateSynchronizer::new();

        let state = sync.value("test");
        let v0 = state.version();

        sync.set_value("test", 1.0);
        let state = sync.value("test");
        let v1 = state.version();

        assert!(v1 > v0);
    }
}

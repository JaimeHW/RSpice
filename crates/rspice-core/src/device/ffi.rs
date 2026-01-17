//! FFI (Foreign Function Interface) Module
//!
//! Provides an interface for loading and using external device models
//! compiled as dynamic libraries (.dll on Windows, .so on Linux, .dylib on macOS).
//!
//! This enables integration of Verilog-A models compiled with external tools,
//! or custom device models written in C/C++.
//!
//! # SPICE Syntax
//! ```text
//! .MODEL mydev FFI_MODEL LIB="path/to/model.dll" ENTRY="create_device"
//! X1 in out mydev
//! ```
//!
//! # Example: External Model Interface
//!
//! External models must export functions with these signatures:
//! ```c
//! // Create device instance, returns opaque pointer
//! void* create_device(const char* name, size_t num_nodes);
//!
//! // Destroy device instance
//! void destroy_device(void* device);
//!
//! // Stamp device into matrix (called during DC/transient)
//! void stamp_device(void* device, const FfiContext* ctx);
//!
//! // Update device state after convergence
//! void update_device(void* device, const FfiContext* ctx);
//! ```

use crate::Value;
use std::collections::HashMap;

//=============================================================================
// C-compatible Structures
//=============================================================================

/// Context passed to external devices during stamping
///
/// This structure is designed to be ABI-compatible with C code.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FfiContext {
    /// Simulation time (s)
    pub time: Value,
    /// Timestep (s)
    pub dt: Value,
    /// Temperature (K)
    pub temperature: Value,
    /// Number of terminal nodes
    pub num_nodes: usize,
    /// Pointer to node voltages array
    pub node_voltages: *const Value,
    /// Pointer to matrix conductance stamping callback
    pub stamp_conductance: Option<extern "C" fn(row: usize, col: usize, value: Value)>,
    /// Pointer to RHS current stamping callback  
    pub stamp_current: Option<extern "C" fn(row: usize, value: Value)>,
}

impl Default for FfiContext {
    fn default() -> Self {
        Self {
            time: 0.0,
            dt: 0.0,
            temperature: 300.0,
            num_nodes: 0,
            node_voltages: std::ptr::null(),
            stamp_conductance: None,
            stamp_current: None,
        }
    }
}

impl FfiContext {
    /// Create a new context for simulation
    pub fn new(time: Value, dt: Value, temperature: Value) -> Self {
        Self {
            time,
            dt,
            temperature,
            ..Default::default()
        }
    }

    /// Set node voltages from a slice
    pub fn with_voltages(mut self, voltages: &[Value]) -> Self {
        self.num_nodes = voltages.len();
        self.node_voltages = voltages.as_ptr();
        self
    }
}

//=============================================================================
// External Device Trait
//=============================================================================

/// Trait for external device implementations
///
/// This can be implemented by:
/// 1. Rust code directly
/// 2. Wrapper around FFI-loaded functions
/// 3. WASM modules (future)
pub trait ExternalDevice: std::fmt::Debug + Send + Sync {
    /// Get device name
    fn name(&self) -> &str;

    /// Get number of terminal nodes
    fn num_terminals(&self) -> usize;

    /// Get terminal node indices
    fn terminals(&self) -> &[usize];

    /// Stamp device into MNA matrix
    ///
    /// Called during Newton-Raphson iteration. The device should
    /// linearize its equations and stamp conductances/currents.
    fn stamp(&self, ctx: &FfiContext, matrix: &mut dyn FfiMatrixStamper, rhs: &mut [Value]);

    /// Update internal state after convergence
    ///
    /// Called after Newton-Raphson converges. Used to update
    /// charge storage, history, etc.
    fn update(&mut self, ctx: &FfiContext);

    /// Reset device state
    fn reset(&mut self);

    /// Get parameter value
    fn get_param(&self, name: &str) -> Option<Value>;

    /// Set parameter value
    fn set_param(&mut self, name: &str, value: Value) -> bool;
}

/// Matrix stamping interface for FFI devices
pub trait FfiMatrixStamper {
    /// Stamp a conductance value at (row, col)
    fn stamp(&mut self, row: usize, col: usize, value: Value);
}

//=============================================================================
// FFI Model Registry
//=============================================================================

/// Registry for loaded FFI models
#[derive(Debug, Default)]
pub struct FfiModelRegistry {
    /// Loaded model factories by name
    factories: HashMap<String, FfiModelFactory>,
}

/// Factory for creating device instances
#[derive(Debug, Clone)]
pub struct FfiModelFactory {
    /// Model name
    pub name: String,
    /// Library path
    pub library_path: String,
    /// Entry point function name
    pub entry_point: String,
    /// Model parameters
    pub params: HashMap<String, Value>,
}

impl FfiModelRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a model factory
    pub fn register(&mut self, name: &str, factory: FfiModelFactory) {
        self.factories.insert(name.to_uppercase(), factory);
    }

    /// Get a model factory by name
    pub fn get(&self, name: &str) -> Option<&FfiModelFactory> {
        self.factories.get(&name.to_uppercase())
    }

    /// Check if a model is registered
    pub fn contains(&self, name: &str) -> bool {
        self.factories.contains_key(&name.to_uppercase())
    }

    /// List all registered model names
    pub fn list_models(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }
}

//=============================================================================
// Placeholder Device Implementation
//=============================================================================

/// A placeholder external device for testing
///
/// Represents a simple two-terminal resistor that can be
/// controlled by FFI. Useful for testing the interface.
#[derive(Debug)]
pub struct PlaceholderDevice {
    name: String,
    terminals: Vec<usize>,
    resistance: Value,
    params: HashMap<String, Value>,
}

impl PlaceholderDevice {
    /// Create a new placeholder device
    pub fn new(name: &str, node_pos: usize, node_neg: usize, resistance: Value) -> Self {
        let mut params = HashMap::new();
        params.insert("R".to_string(), resistance);

        Self {
            name: name.to_string(),
            terminals: vec![node_pos, node_neg],
            resistance,
            params,
        }
    }
}

impl ExternalDevice for PlaceholderDevice {
    fn name(&self) -> &str {
        &self.name
    }

    fn num_terminals(&self) -> usize {
        2
    }

    fn terminals(&self) -> &[usize] {
        &self.terminals
    }

    fn stamp(&self, _ctx: &FfiContext, matrix: &mut dyn FfiMatrixStamper, rhs: &mut [Value]) {
        let n_pos = self.terminals[0];
        let n_neg = self.terminals[1];

        // Stamp resistor conductance
        let g = 1.0 / self.resistance;

        if n_pos > 0 {
            matrix.stamp(n_pos - 1, n_pos - 1, g);
            if n_neg > 0 {
                matrix.stamp(n_pos - 1, n_neg - 1, -g);
            }
        }
        if n_neg > 0 {
            matrix.stamp(n_neg - 1, n_neg - 1, g);
            if n_pos > 0 {
                matrix.stamp(n_neg - 1, n_pos - 1, -g);
            }
        }

        // No RHS contribution for linear resistor
        let _ = rhs;
    }

    fn update(&mut self, _ctx: &FfiContext) {
        // No state to update for resistor
    }

    fn reset(&mut self) {
        // Nothing to reset
    }

    fn get_param(&self, name: &str) -> Option<Value> {
        self.params.get(&name.to_uppercase()).copied()
    }

    fn set_param(&mut self, name: &str, value: Value) -> bool {
        let upper = name.to_uppercase();
        if upper == "R" {
            self.resistance = value;
            self.params.insert("R".to_string(), value);
            true
        } else {
            false
        }
    }
}

//=============================================================================
// FFI Error Types
//=============================================================================

/// Errors that can occur during FFI operations
#[derive(Debug)]
pub enum FfiError {
    /// Library not found or failed to load
    LibraryError(String),
    /// Symbol not found in library
    SymbolError(String),
    /// Invalid device handle
    InvalidHandle,
    /// Parameter error
    ParameterError(String),
    /// Model not registered
    ModelNotFound(String),
}

impl std::fmt::Display for FfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiError::LibraryError(s) => write!(f, "Library error: {}", s),
            FfiError::SymbolError(s) => write!(f, "Symbol error: {}", s),
            FfiError::InvalidHandle => write!(f, "Invalid device handle"),
            FfiError::ParameterError(s) => write!(f, "Parameter error: {}", s),
            FfiError::ModelNotFound(s) => write!(f, "Model not found: {}", s),
        }
    }
}

impl std::error::Error for FfiError {}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStamper {
        stamps: Vec<(usize, usize, Value)>,
    }

    impl FfiMatrixStamper for TestStamper {
        fn stamp(&mut self, row: usize, col: usize, value: Value) {
            self.stamps.push((row, col, value));
        }
    }

    #[test]
    fn test_ffi_context_creation() {
        let ctx = FfiContext::new(1.0e-6, 1.0e-9, 300.0);
        assert!((ctx.time - 1.0e-6).abs() < 1e-15);
        assert!((ctx.dt - 1.0e-9).abs() < 1e-15);
        assert!((ctx.temperature - 300.0).abs() < 1e-10);
    }

    #[test]
    fn test_ffi_context_with_voltages() {
        let voltages = vec![1.0, 2.0, 3.0];
        let ctx = FfiContext::new(0.0, 0.0, 300.0).with_voltages(&voltages);
        assert_eq!(ctx.num_nodes, 3);
        assert!(!ctx.node_voltages.is_null());
    }

    #[test]
    fn test_placeholder_device_creation() {
        let dev = PlaceholderDevice::new("R1", 1, 0, 1000.0);
        assert_eq!(dev.name(), "R1");
        assert_eq!(dev.num_terminals(), 2);
        assert_eq!(dev.terminals(), &[1, 0]);
    }

    #[test]
    fn test_placeholder_device_stamping() {
        let dev = PlaceholderDevice::new("R1", 1, 2, 1000.0);
        let ctx = FfiContext::default();
        let mut stamper = TestStamper { stamps: vec![] };
        let mut rhs = vec![0.0; 3];

        dev.stamp(&ctx, &mut stamper, &mut rhs);

        // Should have 4 stamps for resistor between nodes 1 and 2
        // (0,0), (0,1), (1,0), (1,1) with +/- 1mS
        assert_eq!(stamper.stamps.len(), 4);
    }

    #[test]
    fn test_placeholder_device_params() {
        let mut dev = PlaceholderDevice::new("R1", 1, 0, 1000.0);

        assert_eq!(dev.get_param("R"), Some(1000.0));
        assert!(dev.set_param("R", 2000.0));
        assert_eq!(dev.get_param("R"), Some(2000.0));
    }

    #[test]
    fn test_ffi_registry() {
        let mut registry = FfiModelRegistry::new();

        let factory = FfiModelFactory {
            name: "TestModel".to_string(),
            library_path: "test.dll".to_string(),
            entry_point: "create_test".to_string(),
            params: HashMap::new(),
        };

        registry.register("TESTMODEL", factory);

        assert!(registry.contains("testmodel"));
        assert!(registry.get("testmodel").is_some());
        assert_eq!(registry.list_models(), vec!["TESTMODEL"]);
    }
}

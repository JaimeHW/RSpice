//! FFI (Foreign Function Interface) Module
//!
//! Provides an interface for loading and using external device models
//! compiled as dynamic libraries (.dll on Windows, .so on Linux, .dylib on macOS).
//!
//! This enables integration of Verilog-A models compiled with external tools,
//! or custom device models written in C/C++.
//!
//! # Stability
//!
//! This module is an experimental integration boundary behind the optional
//! `ffi` feature. It is not part of the production-stable device ABI until the
//! callback ownership model, dynamic-library lifetime, and matrix/RHS stamping
//! path have a documented safety review and conformance suite.
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
#[cfg(feature = "ffi")]
use std::path::Path;

//=============================================================================
// C-compatible Structures
//=============================================================================

/// Context passed to external devices during stamping
///
/// This structure is designed to be ABI-compatible with C code.
#[repr(C)]
#[derive(Debug)]
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

/// Lifetime-bound wrapper that keeps the voltage buffer alive while exposing a
/// C-compatible `FfiContext` view to external devices.
#[derive(Debug)]
pub struct FfiContextWithVoltages<'a> {
    raw: FfiContext,
    _voltages: &'a [Value],
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

    /// Bind node voltages to the context for the lifetime of the returned wrapper.
    pub fn with_voltages<'a>(mut self, voltages: &'a [Value]) -> FfiContextWithVoltages<'a> {
        self.num_nodes = voltages.len();
        self.node_voltages = voltages.as_ptr();
        FfiContextWithVoltages {
            raw: self,
            _voltages: voltages,
        }
    }
}

impl<'a> std::ops::Deref for FfiContextWithVoltages<'a> {
    type Target = FfiContext;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<'a> FfiContextWithVoltages<'a> {
    /// Access the underlying C-compatible context.
    pub fn as_ffi(&self) -> &FfiContext {
        &self.raw
    }
}

impl<'a> Clone for FfiContextWithVoltages<'a> {
    fn clone(&self) -> Self {
        Self {
            raw: FfiContext {
                time: self.raw.time,
                dt: self.raw.dt,
                temperature: self.raw.temperature,
                num_nodes: self.raw.num_nodes,
                node_voltages: self.raw.node_voltages,
                stamp_conductance: self.raw.stamp_conductance,
                stamp_current: self.raw.stamp_current,
            },
            _voltages: self._voltages,
        }
    }
}

//=============================================================================
// C Function Type Definitions
//=============================================================================

/// C function pointer types for external device interface
pub mod ffi_types {
    use super::*;
    use std::ffi::c_char;
    use std::os::raw::c_void;

    /// Create device: (name, num_nodes) -> device_ptr
    pub type CreateDeviceFn = unsafe extern "C" fn(*const c_char, usize) -> *mut c_void;

    /// Destroy device: (device_ptr) -> ()
    pub type DestroyDeviceFn = unsafe extern "C" fn(*mut c_void);

    /// Stamp device: (device_ptr, context_ptr) -> ()
    pub type StampDeviceFn = unsafe extern "C" fn(*mut c_void, *const FfiContext);

    /// Update device: (device_ptr, context_ptr) -> ()
    pub type UpdateDeviceFn = unsafe extern "C" fn(*mut c_void, *const FfiContext);

    /// Reset device: (device_ptr) -> ()
    pub type ResetDeviceFn = unsafe extern "C" fn(*mut c_void);

    /// Get parameter: (device_ptr, name) -> value
    pub type GetParamFn = unsafe extern "C" fn(*mut c_void, *const c_char) -> Value;

    /// Set parameter: (device_ptr, name, value) -> success
    pub type SetParamFn = unsafe extern "C" fn(*mut c_void, *const c_char, Value) -> i32;
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
pub trait ExternalDevice: std::fmt::Debug {
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
// Dynamic Library Loading (requires 'ffi' feature)
//=============================================================================

#[cfg(feature = "ffi")]
mod dynamic {
    use super::*;
    use libloading::{Library, Symbol};
    use std::ffi::CString;
    use std::os::raw::c_void;
    use std::path::Path;
    use std::ptr::NonNull;
    use std::sync::Arc;

    #[derive(Debug, Clone, Copy)]
    struct DeviceHandle(NonNull<c_void>);

    impl DeviceHandle {
        fn new(handle: *mut c_void) -> Result<Self, FfiError> {
            NonNull::new(handle)
                .map(Self)
                .ok_or(FfiError::InvalidHandle)
        }

        fn as_ptr(self) -> *mut c_void {
            self.0.as_ptr()
        }
    }

    /// A dynamically loaded library containing external device models
    pub struct DynamicLibrary {
        /// The loaded library (kept alive)
        _library: Arc<Library>,
        /// Library path for debugging
        path: String,
        /// Loaded function pointers
        create_fn: ffi_types::CreateDeviceFn,
        destroy_fn: ffi_types::DestroyDeviceFn,
        stamp_fn: Option<ffi_types::StampDeviceFn>,
        update_fn: Option<ffi_types::UpdateDeviceFn>,
        reset_fn: Option<ffi_types::ResetDeviceFn>,
        get_param_fn: Option<ffi_types::GetParamFn>,
        set_param_fn: Option<ffi_types::SetParamFn>,
    }

    impl std::fmt::Debug for DynamicLibrary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("DynamicLibrary")
                .field("path", &self.path)
                .finish()
        }
    }

    impl DynamicLibrary {
        /// Load a dynamic library from the given path
        ///
        /// # Arguments
        /// * `path` - Path to the library (.dll, .so, or .dylib)
        /// * `create_symbol` - Name of the create_device function
        /// * `destroy_symbol` - Name of the destroy_device function
        ///
        /// # Safety
        /// The library must contain valid C functions matching the expected signatures.
        pub unsafe fn load(
            path: &Path,
            create_symbol: &str,
            destroy_symbol: &str,
        ) -> Result<Self, FfiError> {
            let library = unsafe { Library::new(path) }.map_err(|e| {
                FfiError::LibraryError(format!("Failed to load {}: {}", path.display(), e))
            })?;

            let library = Arc::new(library);

            // Load required functions
            let create_fn: Symbol<ffi_types::CreateDeviceFn> =
                unsafe { library.get(create_symbol.as_bytes()) }
                    .map_err(|e| FfiError::SymbolError(format!("{}: {}", create_symbol, e)))?;
            let create_fn = *create_fn;

            let destroy_fn: Symbol<ffi_types::DestroyDeviceFn> =
                unsafe { library.get(destroy_symbol.as_bytes()) }
                    .map_err(|e| FfiError::SymbolError(format!("{}: {}", destroy_symbol, e)))?;
            let destroy_fn = *destroy_fn;

            // Load optional functions
            let stamp_fn = unsafe { library.get::<ffi_types::StampDeviceFn>(b"stamp_device") }
                .ok()
                .map(|s| *s);

            let update_fn = unsafe { library.get::<ffi_types::UpdateDeviceFn>(b"update_device") }
                .ok()
                .map(|s| *s);

            let reset_fn = unsafe { library.get::<ffi_types::ResetDeviceFn>(b"reset_device") }
                .ok()
                .map(|s| *s);

            let get_param_fn = unsafe { library.get::<ffi_types::GetParamFn>(b"get_param") }
                .ok()
                .map(|s| *s);

            let set_param_fn = unsafe { library.get::<ffi_types::SetParamFn>(b"set_param") }
                .ok()
                .map(|s| *s);

            Ok(Self {
                _library: library,
                path: path.display().to_string(),
                create_fn,
                destroy_fn,
                stamp_fn,
                update_fn,
                reset_fn,
                get_param_fn,
                set_param_fn,
            })
        }

        /// Create a device instance from this library
        pub fn create_device(
            &self,
            name: &str,
            terminals: Vec<usize>,
        ) -> Result<DynamicExternalDevice, FfiError> {
            let c_name = CString::new(name).map_err(|_| {
                FfiError::ParameterError("Device name contains null byte".to_string())
            })?;

            let handle =
                DeviceHandle::new(unsafe { (self.create_fn)(c_name.as_ptr(), terminals.len()) })?;

            Ok(DynamicExternalDevice {
                name: name.to_string(),
                terminals,
                handle,
                destroy_fn: self.destroy_fn,
                stamp_fn: self.stamp_fn,
                update_fn: self.update_fn,
                reset_fn: self.reset_fn,
                get_param_fn: self.get_param_fn,
                set_param_fn: self.set_param_fn,
            })
        }

        /// Get the library path
        pub fn path(&self) -> &str {
            &self.path
        }
    }

    /// An external device loaded from a dynamic library
    pub struct DynamicExternalDevice {
        name: String,
        terminals: Vec<usize>,
        handle: DeviceHandle,
        destroy_fn: ffi_types::DestroyDeviceFn,
        stamp_fn: Option<ffi_types::StampDeviceFn>,
        update_fn: Option<ffi_types::UpdateDeviceFn>,
        reset_fn: Option<ffi_types::ResetDeviceFn>,
        get_param_fn: Option<ffi_types::GetParamFn>,
        set_param_fn: Option<ffi_types::SetParamFn>,
    }

    impl std::fmt::Debug for DynamicExternalDevice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("DynamicExternalDevice")
                .field("name", &self.name)
                .field("terminals", &self.terminals)
                .finish()
        }
    }

    impl Drop for DynamicExternalDevice {
        fn drop(&mut self) {
            unsafe { (self.destroy_fn)(self.handle.as_ptr()) };
        }
    }

    impl ExternalDevice for DynamicExternalDevice {
        fn name(&self) -> &str {
            &self.name
        }

        fn num_terminals(&self) -> usize {
            self.terminals.len()
        }

        fn terminals(&self) -> &[usize] {
            &self.terminals
        }

        fn stamp(&self, ctx: &FfiContext, _matrix: &mut dyn FfiMatrixStamper, _rhs: &mut [Value]) {
            if let Some(stamp_fn) = self.stamp_fn {
                unsafe { stamp_fn(self.handle.as_ptr(), ctx as *const FfiContext) };
            }
        }

        fn update(&mut self, ctx: &FfiContext) {
            if let Some(update_fn) = self.update_fn {
                unsafe { update_fn(self.handle.as_ptr(), ctx as *const FfiContext) };
            }
        }

        fn reset(&mut self) {
            if let Some(reset_fn) = self.reset_fn {
                unsafe { reset_fn(self.handle.as_ptr()) };
            }
        }

        fn get_param(&self, name: &str) -> Option<Value> {
            if let Some(get_fn) = self.get_param_fn {
                let c_name = CString::new(name).ok()?;
                let value = unsafe { get_fn(self.handle.as_ptr(), c_name.as_ptr()) };
                if value.is_nan() { None } else { Some(value) }
            } else {
                None
            }
        }

        fn set_param(&mut self, name: &str, value: Value) -> bool {
            if let Some(set_fn) = self.set_param_fn {
                if let Ok(c_name) = CString::new(name) {
                    let result = unsafe { set_fn(self.handle.as_ptr(), c_name.as_ptr(), value) };
                    return result != 0;
                }
            }
            false
        }
    }

    /// Load a library from the given path
    ///
    /// # Safety
    /// Caller must ensure the library contains valid C functions.
    pub unsafe fn load_library(
        path: &Path,
        create_symbol: &str,
        destroy_symbol: &str,
    ) -> Result<DynamicLibrary, FfiError> {
        unsafe { DynamicLibrary::load(path, create_symbol, destroy_symbol) }
    }
}

#[cfg(feature = "ffi")]
pub use dynamic::{DynamicExternalDevice, DynamicLibrary, load_library};

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
    /// Entry point function name for create
    pub create_entry: String,
    /// Entry point function name for destroy
    pub destroy_entry: String,
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

    /// Load a model from a library path and register it
    #[cfg(feature = "ffi")]
    pub unsafe fn load_from_path(
        &mut self,
        name: &str,
        library_path: &Path,
        create_entry: &str,
        destroy_entry: &str,
    ) -> Result<(), FfiError> {
        // Verify the library can be loaded
        let _lib = unsafe { load_library(library_path, create_entry, destroy_entry) }?;

        let factory = FfiModelFactory {
            name: name.to_string(),
            library_path: library_path.display().to_string(),
            create_entry: create_entry.to_string(),
            destroy_entry: destroy_entry.to_string(),
            params: HashMap::new(),
        };

        self.register(name, factory);
        Ok(())
    }
}

//=============================================================================
// Test Device Implementation
//=============================================================================

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

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
//! // Stamp device into matrix (called during DC/transient). Return 0 on success.
//! int32_t stamp_device(void* device, const FfiContext* ctx);
//!
//! // Update device state after convergence. Return 0 on success.
//! int32_t update_device(void* device, const FfiContext* ctx);
//! ```

use crate::Value;
use std::collections::HashMap;
use std::ffi::c_void;
#[cfg(feature = "ffi")]
use std::path::Path;

/// Version of the C device ABI implemented by this crate.
pub const RSPICE_FFI_ABI_VERSION: u32 = 1;

/// Successful return code for fallible ABI entry points and callbacks.
pub const RSPICE_FFI_OK: i32 = 0;

/// Generic failure return code for ABI entry points and callbacks.
pub const RSPICE_FFI_ERROR: i32 = -1;

//=============================================================================
// C-compatible Structures
//=============================================================================

/// Context passed to external devices during stamping
///
/// This structure is designed to be ABI-compatible with C code.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiContext {
    /// ABI version. External models must reject versions they do not support.
    pub abi_version: u32,
    /// Size of this structure in bytes, allowing compatible tail extension.
    pub struct_size: usize,
    /// Simulation time (s)
    pub time: Value,
    /// Timestep (s)
    pub dt: Value,
    /// Temperature (K)
    pub temperature: Value,
    /// Number of terminal nodes
    num_nodes: usize,
    /// Pointer to node voltages array
    node_voltages: *const Value,
    /// Opaque call-scoped stamping state. It must not be retained by a model.
    stamp_user_data: *mut c_void,
    /// Matrix callback. Row and column are zero-based terminal ordinals.
    stamp_conductance: Option<unsafe extern "C" fn(*mut c_void, usize, usize, Value) -> i32>,
    /// RHS callback. Row is a zero-based terminal ordinal.
    stamp_current: Option<unsafe extern "C" fn(*mut c_void, usize, Value) -> i32>,
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
            abi_version: RSPICE_FFI_ABI_VERSION,
            struct_size: std::mem::size_of::<Self>(),
            time: 0.0,
            dt: 0.0,
            temperature: 300.0,
            num_nodes: 0,
            node_voltages: std::ptr::null(),
            stamp_user_data: std::ptr::null_mut(),
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

    /// Number of voltages exposed to the external model.
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
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

    /// Voltages backing the C-compatible view.
    pub fn voltages(&self) -> &'a [Value] {
        self._voltages
    }
}

impl Clone for FfiContextWithVoltages<'_> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw,
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

    /// Stamp device: (device_ptr, context_ptr) -> status (`0` is success).
    pub type StampDeviceFn = unsafe extern "C" fn(*mut c_void, *const FfiContext) -> i32;

    /// Update device: (device_ptr, context_ptr) -> status (`0` is success).
    pub type UpdateDeviceFn = unsafe extern "C" fn(*mut c_void, *const FfiContext) -> i32;

    /// Reset device: (device_ptr) -> status (`0` is success).
    pub type ResetDeviceFn = unsafe extern "C" fn(*mut c_void) -> i32;

    /// Get parameter: (device_ptr, name) -> value
    pub type GetParamFn = unsafe extern "C" fn(*mut c_void, *const c_char) -> Value;

    /// Set parameter: (device_ptr, name, value) -> status (`0` is success).
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
    fn stamp(
        &self,
        ctx: &FfiContextWithVoltages<'_>,
        matrix: &mut dyn FfiMatrixStamper,
        rhs: &mut [Value],
    ) -> Result<(), FfiError>;

    /// Update internal state after convergence
    ///
    /// Called after Newton-Raphson converges. Used to update
    /// charge storage, history, etc.
    fn update(&mut self, ctx: &FfiContextWithVoltages<'_>) -> Result<(), FfiError>;

    /// Reset device state
    fn reset(&mut self) -> Result<(), FfiError>;

    /// Get parameter value
    fn get_param(&self, name: &str) -> Option<Value>;

    /// Set parameter value
    fn set_param(&mut self, name: &str, value: Value) -> bool;
}

/// Matrix stamping interface for FFI devices
pub trait FfiMatrixStamper {
    /// Stamp a value using global, one-based circuit node identifiers.
    /// Ground is removed by the ABI bridge before this method is called.
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

    struct LoadedLibrary {
        _library: Library,
        /// Loaded function pointers
        create_fn: ffi_types::CreateDeviceFn,
        destroy_fn: ffi_types::DestroyDeviceFn,
        stamp_fn: Option<ffi_types::StampDeviceFn>,
        update_fn: Option<ffi_types::UpdateDeviceFn>,
        reset_fn: Option<ffi_types::ResetDeviceFn>,
        get_param_fn: Option<ffi_types::GetParamFn>,
        set_param_fn: Option<ffi_types::SetParamFn>,
    }

    /// A dynamically loaded library containing external device models.
    /// Clones share one live OS library handle.
    #[derive(Clone)]
    pub struct DynamicLibrary {
        inner: Arc<LoadedLibrary>,
        /// Library path for diagnostics.
        path: Arc<str>,
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
                inner: Arc::new(LoadedLibrary {
                    _library: library,
                    create_fn,
                    destroy_fn,
                    stamp_fn,
                    update_fn,
                    reset_fn,
                    get_param_fn,
                    set_param_fn,
                }),
                path: Arc::from(path.display().to_string()),
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

            let handle = DeviceHandle::new(unsafe {
                (self.inner.create_fn)(c_name.as_ptr(), terminals.len())
            })?;

            Ok(DynamicExternalDevice {
                name: name.to_string(),
                terminals,
                handle,
                library: Arc::clone(&self.inner),
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
        /// Owns the OS library handle for at least as long as every copied
        /// function pointer and the opaque device handle can be used.
        library: Arc<LoadedLibrary>,
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
            unsafe { (self.library.destroy_fn)(self.handle.as_ptr()) };
        }
    }

    struct StampBridge<'a> {
        terminals: &'a [usize],
        matrix: &'a mut dyn FfiMatrixStamper,
        rhs: &'a mut [Value],
        error: Option<FfiError>,
    }

    impl StampBridge<'_> {
        fn terminal_node(&mut self, terminal: usize) -> Option<usize> {
            match self.terminals.get(terminal).copied() {
                Some(node) => Some(node),
                None => {
                    self.record_error(FfiError::CallbackError(format!(
                        "terminal index {terminal} is out of range for {} terminal(s)",
                        self.terminals.len()
                    )));
                    None
                }
            }
        }

        fn record_error(&mut self, error: FfiError) {
            if self.error.is_none() {
                self.error = Some(error);
            }
        }
    }

    unsafe extern "C" fn stamp_matrix_callback(
        user_data: *mut c_void,
        row: usize,
        col: usize,
        value: Value,
    ) -> i32 {
        let Some(bridge) = (unsafe { (user_data as *mut StampBridge<'_>).as_mut() }) else {
            return RSPICE_FFI_ERROR;
        };
        if !value.is_finite() {
            bridge.record_error(FfiError::CallbackError(
                "matrix stamp value must be finite".to_string(),
            ));
            return RSPICE_FFI_ERROR;
        }
        let Some(global_row) = bridge.terminal_node(row) else {
            return RSPICE_FFI_ERROR;
        };
        let Some(global_col) = bridge.terminal_node(col) else {
            return RSPICE_FFI_ERROR;
        };
        if global_row != 0 && global_col != 0 {
            bridge.matrix.stamp(global_row, global_col, value);
        }
        RSPICE_FFI_OK
    }

    unsafe extern "C" fn stamp_rhs_callback(
        user_data: *mut c_void,
        row: usize,
        value: Value,
    ) -> i32 {
        let Some(bridge) = (unsafe { (user_data as *mut StampBridge<'_>).as_mut() }) else {
            return RSPICE_FFI_ERROR;
        };
        if !value.is_finite() {
            bridge.record_error(FfiError::CallbackError(
                "RHS stamp value must be finite".to_string(),
            ));
            return RSPICE_FFI_ERROR;
        }
        let Some(global_row) = bridge.terminal_node(row) else {
            return RSPICE_FFI_ERROR;
        };
        if global_row == 0 {
            return RSPICE_FFI_OK;
        }
        let Some(slot) = bridge.rhs.get_mut(global_row - 1) else {
            bridge.record_error(FfiError::CallbackError(format!(
                "global node {global_row} has no RHS row (RHS length {})",
                bridge.rhs.len()
            )));
            return RSPICE_FFI_ERROR;
        };
        *slot += value;
        RSPICE_FFI_OK
    }

    fn invoke_stamp(
        handle: DeviceHandle,
        stamp_fn: ffi_types::StampDeviceFn,
        terminals: &[usize],
        ctx: &FfiContextWithVoltages<'_>,
        matrix: &mut dyn FfiMatrixStamper,
        rhs: &mut [Value],
    ) -> Result<(), FfiError> {
        let terminal_voltages = terminals
            .iter()
            .map(|&node| {
                if node == 0 {
                    Ok(0.0)
                } else {
                    ctx.voltages().get(node - 1).copied().ok_or_else(|| {
                        FfiError::ParameterError(format!(
                            "terminal references global node {node}, but only {} voltage row(s) were supplied",
                            ctx.voltages().len()
                        ))
                    })
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut bridge = StampBridge {
            terminals,
            matrix,
            rhs,
            error: None,
        };
        let mut plugin_ctx =
            FfiContext::new(ctx.time, ctx.dt, ctx.temperature).with_voltages(&terminal_voltages);
        plugin_ctx.raw.stamp_user_data = (&mut bridge as *mut StampBridge<'_>).cast();
        plugin_ctx.raw.stamp_conductance = Some(stamp_matrix_callback);
        plugin_ctx.raw.stamp_current = Some(stamp_rhs_callback);

        let status = unsafe { stamp_fn(handle.as_ptr(), plugin_ctx.as_ffi()) };
        if let Some(error) = bridge.error {
            return Err(error);
        }
        if status != RSPICE_FFI_OK {
            return Err(FfiError::DeviceError {
                operation: "stamp",
                status,
            });
        }
        Ok(())
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

        fn stamp(
            &self,
            ctx: &FfiContextWithVoltages<'_>,
            matrix: &mut dyn FfiMatrixStamper,
            rhs: &mut [Value],
        ) -> Result<(), FfiError> {
            let Some(stamp_fn) = self.library.stamp_fn else {
                return Err(FfiError::MissingEntryPoint("stamp_device"));
            };
            invoke_stamp(self.handle, stamp_fn, &self.terminals, ctx, matrix, rhs)
        }

        fn update(&mut self, ctx: &FfiContextWithVoltages<'_>) -> Result<(), FfiError> {
            if let Some(update_fn) = self.library.update_fn {
                let status = unsafe { update_fn(self.handle.as_ptr(), ctx.as_ffi()) };
                if status != RSPICE_FFI_OK {
                    return Err(FfiError::DeviceError {
                        operation: "update",
                        status,
                    });
                }
            }
            Ok(())
        }

        fn reset(&mut self) -> Result<(), FfiError> {
            if let Some(reset_fn) = self.library.reset_fn {
                let status = unsafe { reset_fn(self.handle.as_ptr()) };
                if status != RSPICE_FFI_OK {
                    return Err(FfiError::DeviceError {
                        operation: "reset",
                        status,
                    });
                }
            }
            Ok(())
        }

        fn get_param(&self, name: &str) -> Option<Value> {
            if let Some(get_fn) = self.library.get_param_fn {
                let c_name = CString::new(name).ok()?;
                let value = unsafe { get_fn(self.handle.as_ptr(), c_name.as_ptr()) };
                if value.is_nan() { None } else { Some(value) }
            } else {
                None
            }
        }

        fn set_param(&mut self, name: &str, value: Value) -> bool {
            if let Some(set_fn) = self.library.set_param_fn {
                if let Ok(c_name) = CString::new(name) {
                    let result = unsafe { set_fn(self.handle.as_ptr(), c_name.as_ptr(), value) };
                    return result == RSPICE_FFI_OK;
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicBool, Ordering};

        #[derive(Default)]
        struct RecordingStamper(Vec<(usize, usize, Value)>);

        impl FfiMatrixStamper for RecordingStamper {
            fn stamp(&mut self, row: usize, col: usize, value: Value) {
                self.0.push((row, col, value));
            }
        }

        unsafe extern "C" fn bridge_probe(_device: *mut c_void, ctx: *const FfiContext) -> i32 {
            let Some(ctx) = (unsafe { ctx.as_ref() }) else {
                return RSPICE_FFI_ERROR;
            };
            assert_eq!(ctx.abi_version, RSPICE_FFI_ABI_VERSION);
            assert_eq!(ctx.num_nodes, 3);
            let voltages = unsafe { std::slice::from_raw_parts(ctx.node_voltages, ctx.num_nodes) };
            assert_eq!(voltages, &[2.5, 0.0, 1.5]);
            let matrix = ctx.stamp_conductance.expect("matrix callback");
            let rhs = ctx.stamp_current.expect("RHS callback");
            if unsafe { matrix(ctx.stamp_user_data, 0, 2, 4.0) } != RSPICE_FFI_OK {
                return RSPICE_FFI_ERROR;
            }
            unsafe { rhs(ctx.stamp_user_data, 2, -3.0) }
        }

        #[test]
        fn stamp_bridge_maps_terminal_ordinals_and_rhs_rows() {
            let handle = DeviceHandle::new(NonNull::<u8>::dangling().as_ptr().cast())
                .expect("dangling non-null test handle");
            let ctx = FfiContext::new(1.0, 0.1, 325.0).with_voltages(&[1.5, 2.5]);
            let mut matrix = RecordingStamper::default();
            let mut rhs = vec![0.0; 2];
            invoke_stamp(
                handle,
                bridge_probe,
                &[2, 0, 1],
                &ctx,
                &mut matrix,
                &mut rhs,
            )
            .expect("bridge stamp succeeds");
            assert_eq!(matrix.0, vec![(2, 1, 4.0)]);
            assert_eq!(rhs, vec![-3.0, 0.0]);
        }

        unsafe extern "C" fn invalid_terminal_probe(
            _device: *mut c_void,
            ctx: *const FfiContext,
        ) -> i32 {
            let ctx = unsafe { &*ctx };
            let rhs = ctx.stamp_current.expect("RHS callback");
            unsafe { rhs(ctx.stamp_user_data, 99, 1.0) }
        }

        #[test]
        fn stamp_bridge_turns_callback_contract_violations_into_errors() {
            let handle = DeviceHandle::new(NonNull::<u8>::dangling().as_ptr().cast())
                .expect("dangling non-null test handle");
            let ctx = FfiContext::default().with_voltages(&[0.0]);
            let mut matrix = RecordingStamper::default();
            let mut rhs = vec![0.0];
            let error = invoke_stamp(
                handle,
                invalid_terminal_probe,
                &[1],
                &ctx,
                &mut matrix,
                &mut rhs,
            )
            .expect_err("invalid terminal must fail");
            assert!(error.to_string().contains("terminal index 99"));
        }

        static DESTROYED: AtomicBool = AtomicBool::new(false);

        unsafe extern "C" fn create_probe(_name: *const i8, _terminals: usize) -> *mut c_void {
            Box::into_raw(Box::new(7_u8)).cast()
        }

        unsafe extern "C" fn destroy_probe(device: *mut c_void) {
            drop(unsafe { Box::from_raw(device.cast::<u8>()) });
            DESTROYED.store(true, Ordering::SeqCst);
        }

        #[cfg(windows)]
        fn current_process_library() -> Library {
            libloading::os::windows::Library::this()
                .expect("open current process")
                .into()
        }

        #[cfg(unix)]
        fn current_process_library() -> Library {
            libloading::os::unix::Library::this().into()
        }

        #[test]
        fn device_retains_library_owner_until_after_destroy() {
            DESTROYED.store(false, Ordering::SeqCst);
            let inner = Arc::new(LoadedLibrary {
                _library: current_process_library(),
                create_fn: create_probe,
                destroy_fn: destroy_probe,
                stamp_fn: None,
                update_fn: None,
                reset_fn: None,
                get_param_fn: None,
                set_param_fn: None,
            });
            let library = DynamicLibrary {
                inner: Arc::clone(&inner),
                path: Arc::from("current-process"),
            };
            let device = library
                .create_device("probe", vec![1])
                .expect("create test device");
            drop(library);
            drop(inner);
            assert!(!DESTROYED.load(Ordering::SeqCst));
            drop(device);
            assert!(DESTROYED.load(Ordering::SeqCst));
        }
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
    #[cfg(feature = "ffi")]
    library: Option<DynamicLibrary>,
}

impl FfiModelFactory {
    /// Create metadata for a model that has not yet loaded its dynamic library.
    pub fn new(
        name: impl Into<String>,
        library_path: impl Into<String>,
        create_entry: impl Into<String>,
        destroy_entry: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            library_path: library_path.into(),
            create_entry: create_entry.into(),
            destroy_entry: destroy_entry.into(),
            params: HashMap::new(),
            #[cfg(feature = "ffi")]
            library: None,
        }
    }
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

    /// Create an instance from a loaded model and apply its registered parameters.
    #[cfg(feature = "ffi")]
    pub fn create_device(
        &self,
        model_name: &str,
        instance_name: &str,
        terminals: Vec<usize>,
    ) -> Result<DynamicExternalDevice, FfiError> {
        let factory = self
            .get(model_name)
            .ok_or_else(|| FfiError::ModelNotFound(model_name.to_string()))?;
        let library = factory
            .library
            .as_ref()
            .ok_or_else(|| FfiError::ModelNotLoaded(model_name.to_string()))?;
        let mut device = library.create_device(instance_name, terminals)?;
        for (name, &value) in &factory.params {
            if !device.set_param(name, value) {
                return Err(FfiError::ParameterError(format!(
                    "model '{model_name}' parameter '{name}' was rejected by the external device"
                )));
            }
        }
        Ok(device)
    }

    /// Load a model from a library path and register it
    ///
    /// # Safety
    /// The named library entry points must use the RSpice FFI ABI version
    /// declared by [`RSPICE_FFI_ABI_VERSION`]. Calling a symbol through an
    /// incompatible C signature is undefined behavior.
    #[cfg(feature = "ffi")]
    pub unsafe fn load_from_path(
        &mut self,
        name: &str,
        library_path: &Path,
        create_entry: &str,
        destroy_entry: &str,
    ) -> Result<(), FfiError> {
        let library = unsafe { load_library(library_path, create_entry, destroy_entry) }?;
        let mut factory = FfiModelFactory::new(
            name,
            library_path.display().to_string(),
            create_entry,
            destroy_entry,
        );
        factory.library = Some(library);

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
    /// Model metadata exists but no live library has been loaded.
    ModelNotLoaded(String),
    /// Optional entry point required for this operation is absent.
    MissingEntryPoint(&'static str),
    /// External device returned an error status.
    DeviceError {
        /// Operation that failed.
        operation: &'static str,
        /// Status returned by the external model.
        status: i32,
    },
    /// External model violated the call-scoped stamping contract.
    CallbackError(String),
}

impl std::fmt::Display for FfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiError::LibraryError(s) => write!(f, "Library error: {}", s),
            FfiError::SymbolError(s) => write!(f, "Symbol error: {}", s),
            FfiError::InvalidHandle => write!(f, "Invalid device handle"),
            FfiError::ParameterError(s) => write!(f, "Parameter error: {}", s),
            FfiError::ModelNotFound(s) => write!(f, "Model not found: {}", s),
            FfiError::ModelNotLoaded(s) => write!(f, "Model library is not loaded: {}", s),
            FfiError::MissingEntryPoint(s) => write!(f, "Missing external-model entry point: {s}"),
            FfiError::DeviceError { operation, status } => {
                write!(f, "External device {operation} failed with status {status}")
            }
            FfiError::CallbackError(s) => write!(f, "External device callback error: {s}"),
        }
    }
}

impl std::error::Error for FfiError {}

//=============================================================================
// Tests
//=============================================================================

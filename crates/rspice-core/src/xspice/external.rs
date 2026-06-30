//! External host interfaces used by XSPICE code models.

use crate::Value;
use crate::xspice::{CmError, CmResult, DigitalValue};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

/// Startup configuration for the official XSPICE `d_process` code model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalProcessSpec {
    /// Executable path or provider-defined process identifier.
    pub process_file: String,
    /// Arguments passed to the external process/provider.
    pub process_params: Vec<String>,
    /// Number of digital input bits before byte packing.
    pub input_count: usize,
    /// Number of digital output bits before byte packing.
    pub output_count: usize,
}

/// Runtime instance backing one `d_process` code-model instance.
pub trait DigitalProcessRuntime: Send {
    /// Exchange one clocked packet with the external process/provider.
    fn exchange(
        &mut self,
        signed_time: Value,
        input_bytes: &[u8],
        output_bytes: &mut [u8],
    ) -> CmResult<()>;
}

/// Factory for creating `d_process` runtimes.
pub trait DigitalProcessRuntimeFactory: Send + Sync {
    /// Start a new runtime for one `d_process` instance.
    fn start(&self, spec: &DigitalProcessSpec) -> CmResult<Box<dyn DigitalProcessRuntime>>;
}

/// Startup configuration for the official XSPICE `d_cosim` code model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalCosimSpec {
    /// Shared-library path or provider-defined simulation identifier.
    pub simulation: String,
    /// Arguments made available to the host library/shim.
    pub lib_args: Vec<String>,
    /// Arguments made available to the co-simulation payload.
    pub sim_args: Vec<String>,
    /// Number of digital input ports.
    pub input_count: usize,
    /// Number of digital output ports.
    pub output_count: usize,
    /// Number of bidirectional digital ports.
    pub inout_count: usize,
    /// Input event queue size requested by the model card.
    pub queue_size: usize,
    /// Official irreversible flag value.
    pub irreversible: i64,
}

/// Output snapshot returned by a `d_cosim` runtime after advancing.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalCosimStep {
    /// Co-simulator time corresponding to these output values.
    pub vtime: Value,
    /// Digital output-port values.
    pub outputs: Vec<DigitalValue>,
    /// Digital bidirectional-port drive values.
    pub inouts: Vec<DigitalValue>,
}

/// One timestamped input event delivered to a `d_cosim` runtime.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalCosimInputEvent {
    /// Accepted simulator time for this input change.
    pub time: Value,
    /// Unified input index: `d_in` bits first, followed by `d_inout` bits.
    pub index: usize,
    /// New digital value.
    pub value: DigitalValue,
}

/// Runtime instance backing one `d_cosim` code-model instance.
pub trait DigitalCosimRuntime: Send {
    /// Maximum unified input event index accepted by this runtime.
    ///
    /// The official ngspice `d_cosim` model warns on XSPICE/co-simulator
    /// input-count mismatches, then clips event delivery to the co-simulator's
    /// advertised `in_count + inout_count`. Providers that use the connected
    /// port counts from `DigitalCosimSpec` can keep the default.
    fn input_event_limit(&self) -> Option<usize> {
        None
    }

    /// Initialize co-simulator inputs at time zero.
    fn initialize(
        &mut self,
        time: Value,
        inputs: &[DigitalValue],
        inouts: &[DigitalValue],
    ) -> CmResult<DigitalCosimStep>;

    /// Run the official startup advance after time-zero input initialization.
    fn startup_step(&mut self, time: Value) -> CmResult<DigitalCosimStep> {
        self.step(time, &[], &[], &[])
    }

    /// Advance the co-simulator to an accepted/direct simulator time.
    fn step(
        &mut self,
        time: Value,
        inputs: &[DigitalValue],
        inouts: &[DigitalValue],
        events: &[DigitalCosimInputEvent],
    ) -> CmResult<DigitalCosimStep>;
}

/// Factory for creating `d_cosim` runtimes.
pub trait DigitalCosimRuntimeFactory: Send + Sync {
    /// Start a new runtime for one `d_cosim` instance.
    fn start(&self, spec: &DigitalCosimSpec) -> CmResult<Box<dyn DigitalCosimRuntime>>;
}

static DIGITAL_PROCESS_FACTORY: OnceLock<Mutex<Option<Arc<dyn DigitalProcessRuntimeFactory>>>> =
    OnceLock::new();
static DIGITAL_COSIM_FACTORY: OnceLock<Mutex<Option<Arc<dyn DigitalCosimRuntimeFactory>>>> =
    OnceLock::new();

fn digital_process_factory() -> &'static Mutex<Option<Arc<dyn DigitalProcessRuntimeFactory>>> {
    DIGITAL_PROCESS_FACTORY.get_or_init(|| Mutex::new(None))
}

fn digital_cosim_factory() -> &'static Mutex<Option<Arc<dyn DigitalCosimRuntimeFactory>>> {
    DIGITAL_COSIM_FACTORY.get_or_init(|| Mutex::new(None))
}

fn lock_external_factory<T>(mutex: &'static Mutex<T>) -> MutexGuard<'static, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Install or clear the process-runtime provider used by `d_process`.
///
/// Returns the previously installed provider so callers can restore it after a
/// scoped test or host integration.
pub fn set_digital_process_runtime_factory(
    factory: Option<Arc<dyn DigitalProcessRuntimeFactory>>,
) -> Option<Arc<dyn DigitalProcessRuntimeFactory>> {
    std::mem::replace(
        &mut *lock_external_factory(digital_process_factory()),
        factory,
    )
}

/// Install or clear the co-simulation runtime provider used by `d_cosim`.
pub fn set_digital_cosim_runtime_factory(
    factory: Option<Arc<dyn DigitalCosimRuntimeFactory>>,
) -> Option<Arc<dyn DigitalCosimRuntimeFactory>> {
    std::mem::replace(
        &mut *lock_external_factory(digital_cosim_factory()),
        factory,
    )
}

pub(crate) fn start_digital_process_runtime(
    spec: &DigitalProcessSpec,
) -> CmResult<Box<dyn DigitalProcessRuntime>> {
    if let Some(factory) = lock_external_factory(digital_process_factory())
        .as_ref()
        .cloned()
    {
        return factory.start(spec);
    }

    default_digital_process_factory().start(spec)
}

pub(crate) fn start_digital_cosim_runtime(
    spec: &DigitalCosimSpec,
) -> CmResult<Box<dyn DigitalCosimRuntime>> {
    if let Some(factory) = lock_external_factory(digital_cosim_factory())
        .as_ref()
        .cloned()
    {
        return factory.start(spec);
    }

    default_digital_cosim_factory().start(spec)
}

#[cfg(not(target_arch = "wasm32"))]
fn default_digital_process_factory() -> NativeDigitalProcessFactory {
    NativeDigitalProcessFactory
}

#[cfg(target_arch = "wasm32")]
fn default_digital_process_factory() -> WasmDigitalProcessFactory {
    WasmDigitalProcessFactory
}

#[cfg(all(feature = "ffi", not(target_arch = "wasm32")))]
fn default_digital_cosim_factory() -> NativeDigitalCosimFactory {
    NativeDigitalCosimFactory
}

#[cfg(not(all(feature = "ffi", not(target_arch = "wasm32"))))]
fn default_digital_cosim_factory() -> HostDigitalCosimFactory {
    HostDigitalCosimFactory
}

struct HostDigitalCosimFactory;

impl DigitalCosimRuntimeFactory for HostDigitalCosimFactory {
    fn start(&self, _spec: &DigitalCosimSpec) -> CmResult<Box<dyn DigitalCosimRuntime>> {
        Err(CmError::EvaluationError(
            "d_cosim requires a host co-simulation runtime provider".to_string(),
        ))
    }
}

fn packed_byte_len(bit_count: usize) -> usize {
    if bit_count == 0 {
        0
    } else {
        (bit_count - 1) / 8 + 1
    }
}

fn validate_digital_process_spec(spec: &DigitalProcessSpec) -> CmResult<(usize, usize)> {
    if spec.process_file.trim().is_empty() {
        return Err(CmError::InvalidParameter {
            name: "process_file".to_string(),
            message: "must not be empty".to_string(),
        });
    }
    if spec.input_count > u8::MAX as usize {
        return Err(CmError::InvalidParameter {
            name: "in".to_string(),
            message: "d_process supports at most 255 input bits".to_string(),
        });
    }
    if spec.output_count == 0 || spec.output_count > u8::MAX as usize {
        return Err(CmError::InvalidParameter {
            name: "out".to_string(),
            message: "d_process requires 1 to 255 output bits".to_string(),
        });
    }

    Ok((
        packed_byte_len(spec.input_count),
        packed_byte_len(spec.output_count),
    ))
}

fn d_process_fifo_base_name(process_file: &str) -> Option<&str> {
    if process_file.ends_with("||") {
        Some(&process_file[..process_file.len() - 2])
    } else if process_file.ends_with('|') {
        Some(&process_file[..process_file.len() - 1])
    } else {
        None
    }
}

fn d_process_fifo_endpoint_paths(process_file: &str) -> Option<(String, String)> {
    d_process_fifo_base_name(process_file).map(|base| (format!("{base}_in"), format!("{base}_out")))
}

#[cfg(target_arch = "wasm32")]
struct WasmDigitalProcessFactory;

#[cfg(target_arch = "wasm32")]
impl DigitalProcessRuntimeFactory for WasmDigitalProcessFactory {
    fn start(&self, _spec: &DigitalProcessSpec) -> CmResult<Box<dyn DigitalProcessRuntime>> {
        Err(CmError::EvaluationError(
            "d_process requires a host digital-process runtime provider on this platform"
                .to_string(),
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::{
        CmError, CmResult, DigitalProcessRuntime, DigitalProcessRuntimeFactory, DigitalProcessSpec,
        Value, validate_digital_process_spec,
    };
    use std::io::{Read, Write};
    use std::process::{Child, Command, Stdio};

    const D_PROCESS_FORMAT_VERSION: u8 = 0x01;

    pub(super) struct NativeDigitalProcessFactory;

    impl DigitalProcessRuntimeFactory for NativeDigitalProcessFactory {
        fn start(&self, spec: &DigitalProcessSpec) -> CmResult<Box<dyn DigitalProcessRuntime>> {
            let (input_byte_count, output_byte_count) = validate_digital_process_spec(spec)?;
            let mut runtime = NativeDigitalProcessRuntime::start(spec)?;
            runtime.input_byte_count = input_byte_count;
            runtime.output_byte_count = output_byte_count;
            runtime.send_header(spec)?;
            Ok(Box::new(runtime))
        }
    }

    struct NativeDigitalProcessRuntime {
        writer: Box<dyn Write + Send>,
        reader: Box<dyn Read + Send>,
        child: Option<Child>,
        input_byte_count: usize,
        output_byte_count: usize,
    }

    impl NativeDigitalProcessRuntime {
        fn start(spec: &DigitalProcessSpec) -> CmResult<Self> {
            if spec.process_file.ends_with('|') {
                return Self::start_fifo(spec);
            }

            let mut child = Command::new(&spec.process_file)
                .args(&spec.process_params)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|err| {
                    CmError::EvaluationError(format!(
                        "d_process failed to start '{}': {err}",
                        spec.process_file
                    ))
                })?;

            let writer = child.stdin.take().ok_or_else(|| {
                CmError::EvaluationError(format!(
                    "d_process failed to open stdin for '{}'",
                    spec.process_file
                ))
            })?;
            let reader = child.stdout.take().ok_or_else(|| {
                CmError::EvaluationError(format!(
                    "d_process failed to open stdout for '{}'",
                    spec.process_file
                ))
            })?;

            Ok(Self {
                writer: Box::new(writer),
                reader: Box::new(reader),
                child: Some(child),
                input_byte_count: 0,
                output_byte_count: 0,
            })
        }

        fn start_fifo(spec: &DigitalProcessSpec) -> CmResult<Self> {
            use std::fs::OpenOptions;

            let trimmed = super::d_process_fifo_base_name(&spec.process_file).unwrap_or("");
            if trimmed.is_empty() {
                return Err(CmError::InvalidParameter {
                    name: "process_file".to_string(),
                    message: "FIFO base name must not be empty".to_string(),
                });
            }

            let (writer_path, reader_path) =
                super::d_process_fifo_endpoint_paths(&spec.process_file).ok_or_else(|| {
                    CmError::InvalidParameter {
                        name: "process_file".to_string(),
                        message: "FIFO process file must end in '|' or '||'".to_string(),
                    }
                })?;
            let writer = OpenOptions::new()
                .write(true)
                .open(&writer_path)
                .map_err(|err| {
                    CmError::EvaluationError(format!(
                        "d_process failed to open input FIFO or named pipe '{writer_path}': {err}"
                    ))
                })?;
            let reader = OpenOptions::new()
                .read(true)
                .open(&reader_path)
                .map_err(|err| {
                    CmError::EvaluationError(format!(
                        "d_process failed to open output FIFO or named pipe '{reader_path}': {err}"
                    ))
                })?;

            Ok(Self {
                writer: Box::new(writer),
                reader: Box::new(reader),
                child: None,
                input_byte_count: 0,
                output_byte_count: 0,
            })
        }

        fn send_header(&mut self, spec: &DigitalProcessSpec) -> CmResult<()> {
            let header = [
                D_PROCESS_FORMAT_VERSION,
                spec.input_count as u8,
                spec.output_count as u8,
            ];
            self.writer.write_all(&header).map_err(|err| {
                CmError::EvaluationError(format!("d_process failed to send header: {err}"))
            })?;
            self.writer.flush().map_err(|err| {
                CmError::EvaluationError(format!("d_process failed to flush header: {err}"))
            })?;

            let mut echoed = [0u8; 3];
            self.reader.read_exact(&mut echoed).map_err(|err| {
                CmError::EvaluationError(format!("d_process did not echo header: {err}"))
            })?;
            if echoed[0] != D_PROCESS_FORMAT_VERSION {
                return Err(CmError::EvaluationError(format!(
                    "d_process returned unsupported protocol version {}",
                    echoed[0]
                )));
            }
            if echoed[1] as usize != spec.input_count || echoed[2] as usize != spec.output_count {
                return Err(CmError::EvaluationError(format!(
                    "d_process I/O mismatch: in {} vs {}, out {} vs {}",
                    spec.input_count, echoed[1], spec.output_count, echoed[2]
                )));
            }

            Ok(())
        }
    }

    impl DigitalProcessRuntime for NativeDigitalProcessRuntime {
        fn exchange(
            &mut self,
            signed_time: Value,
            input_bytes: &[u8],
            output_bytes: &mut [u8],
        ) -> CmResult<()> {
            if input_bytes.len() != self.input_byte_count {
                return Err(CmError::EvaluationError(format!(
                    "d_process input packet length mismatch: expected {}, got {}",
                    self.input_byte_count,
                    input_bytes.len()
                )));
            }
            if output_bytes.len() != self.output_byte_count {
                return Err(CmError::EvaluationError(format!(
                    "d_process output packet length mismatch: expected {}, got {}",
                    self.output_byte_count,
                    output_bytes.len()
                )));
            }

            self.writer
                .write_all(&signed_time.to_ne_bytes())
                .and_then(|_| self.writer.write_all(input_bytes))
                .and_then(|_| self.writer.flush())
                .map_err(|err| {
                    CmError::EvaluationError(format!("d_process failed to write packet: {err}"))
                })?;
            self.reader.read_exact(output_bytes).map_err(|err| {
                CmError::EvaluationError(format!("d_process failed to read packet: {err}"))
            })?;
            Ok(())
        }
    }

    impl Drop for NativeDigitalProcessRuntime {
        fn drop(&mut self) {
            if let Some(child) = self.child.as_mut() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
use native::NativeDigitalProcessFactory;

#[cfg(all(feature = "ffi", not(target_arch = "wasm32")))]
mod native_cosim {
    use super::{
        CmError, CmResult, DigitalCosimInputEvent, DigitalCosimRuntime, DigitalCosimRuntimeFactory,
        DigitalCosimSpec, DigitalCosimStep, DigitalValue, Value,
    };
    use crate::xspice::{DigitalState, DigitalStrength};
    use libloading::Library;
    use std::ffi::{CStr, CString, c_char, c_uint, c_void};

    #[cfg(unix)]
    type PlatformLibrary = libloading::os::unix::Library;
    #[cfg(windows)]
    type PlatformLibrary = libloading::os::windows::Library;

    #[cfg(windows)]
    const COSIM_LIBRARY_EXTENSIONS: &[&str] = &["", ".so", ".DLL"];
    #[cfg(target_os = "macos")]
    const COSIM_LIBRARY_EXTENSIONS: &[&str] = &["", ".so", ".dylib"];
    #[cfg(all(unix, not(target_os = "macos")))]
    const COSIM_LIBRARY_EXTENSIONS: &[&str] = &["", ".so"];

    const COSIM_METHOD_NORMAL: i32 = 0;
    const COSIM_METHOD_AFTER_INPUT: i32 = 1;
    const COSIM_METHOD_BOTH: i32 = 2;

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct NgDigital {
        state: i32,
        strength: i32,
    }

    #[repr(C)]
    struct CoInfo {
        in_count: c_uint,
        out_count: c_uint,
        inout_count: c_uint,
        cleanup: Option<unsafe extern "C" fn(*mut CoInfo)>,
        step: Option<unsafe extern "C" fn(*mut CoInfo)>,
        in_fn: Option<unsafe extern "C" fn(*mut CoInfo, c_uint, *mut NgDigital)>,
        out_fn: Option<unsafe extern "C" fn(*mut CoInfo, c_uint, *mut NgDigital)>,
        handle: *mut c_void,
        vtime: Value,
        method: i32,
        lib_argc: c_uint,
        sim_argc: c_uint,
        lib_argv: *const *const c_char,
        sim_argv: *const *const c_char,
        dlopen_fn: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    }

    pub(super) struct NativeDigitalCosimFactory;

    impl DigitalCosimRuntimeFactory for NativeDigitalCosimFactory {
        fn start(&self, spec: &DigitalCosimSpec) -> CmResult<Box<dyn DigitalCosimRuntime>> {
            NativeDigitalCosimRuntime::start(spec)
                .map(|runtime| Box::new(runtime) as Box<dyn DigitalCosimRuntime>)
        }
    }

    #[repr(C)]
    struct NativeDigitalCosimRuntime {
        info: CoInfo,
        output_count: usize,
        inout_count: usize,
        output_values: Vec<DigitalValue>,
        pending_values: Vec<Option<DigitalValue>>,
        _library: Library,
        _lib_arg_strings: Vec<CString>,
        _sim_arg_strings: Vec<CString>,
        _lib_arg_ptrs: Vec<*const c_char>,
        _sim_arg_ptrs: Vec<*const c_char>,
    }

    unsafe impl Send for NativeDigitalCosimRuntime {}

    fn has_cosim_library_extension(filename: &str, extension: &str) -> bool {
        if extension.is_empty() {
            return false;
        }
        if cfg!(windows) {
            filename
                .get(filename.len().saturating_sub(extension.len())..)
                .is_some_and(|suffix| suffix.eq_ignore_ascii_case(extension))
        } else {
            filename.ends_with(extension)
        }
    }

    fn cosim_library_candidates(filename: &str) -> Vec<String> {
        let mut candidates = Vec::with_capacity(COSIM_LIBRARY_EXTENSIONS.len());
        for extension in COSIM_LIBRARY_EXTENSIONS {
            if has_cosim_library_extension(filename, extension) {
                continue;
            }
            let candidate = format!("{filename}{extension}");
            if !candidates.iter().any(|existing| existing == &candidate) {
                candidates.push(candidate);
            }
        }
        candidates
    }

    #[cfg(unix)]
    unsafe fn open_platform_cosim_library(
        filename: &str,
    ) -> Result<PlatformLibrary, libloading::Error> {
        unsafe {
            PlatformLibrary::open(
                Some(filename),
                libloading::os::unix::RTLD_GLOBAL | libloading::os::unix::RTLD_NOW,
            )
        }
    }

    #[cfg(windows)]
    unsafe fn open_platform_cosim_library(
        filename: &str,
    ) -> Result<PlatformLibrary, libloading::Error> {
        unsafe { PlatformLibrary::new(filename) }
    }

    fn open_cosim_platform_library(filename: &str) -> Result<PlatformLibrary, libloading::Error> {
        let mut last_error = None;
        for candidate in cosim_library_candidates(filename) {
            match unsafe { open_platform_cosim_library(&candidate) } {
                Ok(library) => return Ok(library),
                Err(error) => last_error = Some(error),
            }
        }
        Err(last_error.expect("cosim library candidate list is never empty"))
    }

    fn open_cosim_library(filename: &str) -> Result<Library, libloading::Error> {
        open_cosim_platform_library(filename).map(Into::into)
    }

    #[cfg(unix)]
    fn platform_library_into_raw(library: PlatformLibrary) -> *mut c_void {
        library.into_raw()
    }

    #[cfg(windows)]
    fn platform_library_into_raw(library: PlatformLibrary) -> *mut c_void {
        library.into_raw() as *mut c_void
    }

    unsafe extern "C" fn cosim_dlopen(filename: *const c_char) -> *mut c_void {
        if filename.is_null() {
            return std::ptr::null_mut();
        }
        let filename = unsafe { CStr::from_ptr(filename) }.to_string_lossy();
        match open_cosim_platform_library(filename.as_ref()) {
            Ok(library) => platform_library_into_raw(library),
            Err(error) => {
                log::warn!("d_cosim dlopen_fn failed to load '{filename}': {error}");
                std::ptr::null_mut()
            }
        }
    }

    fn event_batch_end(events: &[DigitalCosimInputEvent], start: usize) -> usize {
        let time = events[start].time;
        let mut end = start + 1;
        while end < events.len() && events[end].time == time {
            end += 1;
        }
        end
    }

    impl NativeDigitalCosimRuntime {
        fn start(spec: &DigitalCosimSpec) -> CmResult<Self> {
            if spec.simulation.trim().is_empty() {
                return Err(CmError::InvalidParameter {
                    name: "simulation".to_string(),
                    message: "must not be empty".to_string(),
                });
            }

            let library = open_cosim_library(&spec.simulation).map_err(|err| {
                CmError::EvaluationError(format!(
                    "d_cosim failed to load simulation '{}': {err}",
                    spec.simulation
                ))
            })?;
            let (lib_arg_strings, lib_arg_ptrs) = c_arg_block("lib_args", &spec.lib_args)?;
            let (sim_arg_strings, sim_arg_ptrs) = c_arg_block("sim_args", &spec.sim_args)?;

            let mut runtime = Self {
                info: CoInfo {
                    in_count: spec.input_count as c_uint,
                    out_count: spec.output_count as c_uint,
                    inout_count: spec.inout_count as c_uint,
                    cleanup: None,
                    step: None,
                    in_fn: None,
                    out_fn: Some(accept_output),
                    handle: std::ptr::null_mut(),
                    vtime: 0.0,
                    method: COSIM_METHOD_NORMAL,
                    lib_argc: spec.lib_args.len() as c_uint,
                    sim_argc: spec.sim_args.len() as c_uint,
                    lib_argv: lib_arg_ptrs.as_ptr(),
                    sim_argv: sim_arg_ptrs.as_ptr(),
                    dlopen_fn: Some(cosim_dlopen),
                },
                output_count: spec.output_count,
                inout_count: spec.inout_count,
                output_values: vec![DigitalValue::zero(); spec.output_count + spec.inout_count],
                pending_values: vec![None; spec.output_count + spec.inout_count],
                _library: library,
                _lib_arg_strings: lib_arg_strings,
                _sim_arg_strings: sim_arg_strings,
                _lib_arg_ptrs: lib_arg_ptrs,
                _sim_arg_ptrs: sim_arg_ptrs,
            };

            unsafe {
                let setup: libloading::Symbol<'_, unsafe extern "C" fn(*mut CoInfo)> =
                    runtime._library.get(b"Cosim_setup\0").map_err(|err| {
                        CmError::EvaluationError(format!(
                            "d_cosim simulation '{}' does not export Cosim_setup: {err}",
                            spec.simulation
                        ))
                    })?;
                setup(&mut runtime.info);
            }

            if runtime.info.step.is_none() {
                return Err(CmError::EvaluationError(format!(
                    "d_cosim simulation '{}' did not provide step()",
                    spec.simulation
                )));
            }
            if runtime.info.in_fn.is_none() {
                return Err(CmError::EvaluationError(format!(
                    "d_cosim simulation '{}' did not provide in_fn()",
                    spec.simulation
                )));
            }
            if runtime.info.in_count as usize != spec.input_count {
                log::warn!(
                    "d_cosim input count mismatch for '{}': RSpice={} runtime={}",
                    spec.simulation,
                    spec.input_count,
                    runtime.info.in_count
                );
            }
            if runtime.info.out_count as usize != spec.output_count {
                log::warn!(
                    "d_cosim output count mismatch for '{}': RSpice={} runtime={}",
                    spec.simulation,
                    spec.output_count,
                    runtime.info.out_count
                );
            }
            if runtime.info.inout_count as usize != spec.inout_count {
                log::warn!(
                    "d_cosim inout count mismatch for '{}': RSpice={} runtime={}",
                    spec.simulation,
                    spec.inout_count,
                    runtime.info.inout_count
                );
            }

            Ok(runtime)
        }

        fn feed_inputs(
            &mut self,
            inputs: &[DigitalValue],
            inouts: &[DigitalValue],
        ) -> CmResult<()> {
            let Some(in_fn) = self.info.in_fn else {
                return Err(CmError::EvaluationError(
                    "d_cosim runtime is missing in_fn()".to_string(),
                ));
            };
            for (index, value) in inputs.iter().copied().enumerate() {
                let mut value = to_ng_digital(value);
                unsafe { in_fn(&mut self.info, index as c_uint, &mut value) };
            }
            for (index, value) in inouts.iter().copied().enumerate() {
                let mut value = to_ng_digital(value);
                unsafe { in_fn(&mut self.info, (inputs.len() + index) as c_uint, &mut value) };
            }
            Ok(())
        }

        fn feed_event(&mut self, event: &DigitalCosimInputEvent) -> CmResult<()> {
            let Some(in_fn) = self.info.in_fn else {
                return Err(CmError::EvaluationError(
                    "d_cosim runtime is missing in_fn()".to_string(),
                ));
            };
            let mut value = to_ng_digital(event.value);
            unsafe { in_fn(&mut self.info, event.index as c_uint, &mut value) };
            Ok(())
        }

        fn feed_event_batch(&mut self, events: &[DigitalCosimInputEvent]) -> CmResult<()> {
            for event in events {
                self.feed_event(event)?;
            }
            Ok(())
        }

        fn run_normal_event_batches(
            &mut self,
            time: Value,
            events: &[DigitalCosimInputEvent],
        ) -> CmResult<()> {
            let mut index = 0;
            while index < events.len() {
                let event_time = events[index].time;
                let end = event_batch_end(events, index);
                self.call_step(event_time)?;
                self.feed_event_batch(&events[index..end])?;
                index = end;
            }
            if time > self.info.vtime {
                self.call_step(time)?;
            }
            Ok(())
        }

        fn run_after_input_event_batches(
            &mut self,
            events: &[DigitalCosimInputEvent],
        ) -> CmResult<()> {
            let mut index = 0;
            while index < events.len() {
                let event_time = events[index].time;
                let end = event_batch_end(events, index);
                self.feed_event_batch(&events[index..end])?;
                self.call_step(event_time)?;
                index = end;
            }
            Ok(())
        }

        fn run_both_event_batches(&mut self, events: &[DigitalCosimInputEvent]) -> CmResult<()> {
            let mut index = 0;
            while index < events.len() {
                let event_time = events[index].time;
                let end = event_batch_end(events, index);
                self.call_step(event_time)?;
                self.feed_event_batch(&events[index..end])?;
                self.call_step(event_time)?;
                index = end;
            }
            Ok(())
        }

        fn call_step(&mut self, time: Value) -> CmResult<()> {
            let Some(step) = self.info.step else {
                return Err(CmError::EvaluationError(
                    "d_cosim runtime is missing step()".to_string(),
                ));
            };
            self.info.vtime = time;
            unsafe { step(&mut self.info) };
            Ok(())
        }

        fn current_snapshot(&self) -> DigitalCosimStep {
            DigitalCosimStep {
                vtime: self.info.vtime,
                outputs: self.output_values[..self.output_count].to_vec(),
                inouts: self.output_values[self.output_count..self.output_count + self.inout_count]
                    .to_vec(),
            }
        }

        fn snapshot(&mut self) -> DigitalCosimStep {
            for (index, pending) in self.pending_values.iter_mut().enumerate() {
                if let Some(value) = pending.take() {
                    self.output_values[index] = value;
                }
            }
            self.current_snapshot()
        }
    }

    impl DigitalCosimRuntime for NativeDigitalCosimRuntime {
        fn input_event_limit(&self) -> Option<usize> {
            Some(self.info.in_count as usize + self.info.inout_count as usize)
        }

        fn initialize(
            &mut self,
            time: Value,
            inputs: &[DigitalValue],
            inouts: &[DigitalValue],
        ) -> CmResult<DigitalCosimStep> {
            self.info.vtime = time;
            self.feed_inputs(inputs, inouts)?;
            Ok(self.current_snapshot())
        }

        fn startup_step(&mut self, time: Value) -> CmResult<DigitalCosimStep> {
            self.call_step(time)?;
            Ok(self.snapshot())
        }

        fn step(
            &mut self,
            time: Value,
            _inputs: &[DigitalValue],
            _inouts: &[DigitalValue],
            events: &[DigitalCosimInputEvent],
        ) -> CmResult<DigitalCosimStep> {
            if events.is_empty() {
                self.call_step(time)?;
                return Ok(self.snapshot());
            }

            match self.info.method {
                COSIM_METHOD_NORMAL => {
                    self.run_normal_event_batches(time, events)?;
                }
                COSIM_METHOD_AFTER_INPUT => {
                    self.run_after_input_event_batches(events)?;
                }
                COSIM_METHOD_BOTH => {
                    self.run_both_event_batches(events)?;
                }
                other => {
                    log::warn!("d_cosim runtime returned invalid method {other}; using Normal");
                    self.run_normal_event_batches(time, events)?;
                }
            }
            Ok(self.snapshot())
        }
    }

    impl Drop for NativeDigitalCosimRuntime {
        fn drop(&mut self) {
            if let Some(cleanup) = self.info.cleanup {
                unsafe { cleanup(&mut self.info) };
            }
        }
    }

    fn c_arg_block(
        parameter_name: &str,
        args: &[String],
    ) -> CmResult<(Vec<CString>, Vec<*const c_char>)> {
        let mut strings = Vec::with_capacity(args.len());
        for arg in args {
            strings.push(
                CString::new(arg.as_str()).map_err(|_| CmError::InvalidParameter {
                    name: parameter_name.to_string(),
                    message: "argument contains an interior NUL byte".to_string(),
                })?,
            );
        }
        let mut pointers: Vec<*const c_char> = strings.iter().map(|arg| arg.as_ptr()).collect();
        pointers.push(std::ptr::null());
        Ok((strings, pointers))
    }

    fn to_ng_digital(value: DigitalValue) -> NgDigital {
        let state = match value.state.logic_level() {
            Some(false) => 0,
            Some(true) => 1,
            None => 2,
        };
        let strength = match value.strength {
            DigitalStrength::Strong => 0,
            DigitalStrength::Resistive => 1,
            DigitalStrength::HighZ => 2,
            DigitalStrength::Undetermined => 3,
        };
        NgDigital { state, strength }
    }

    fn from_ng_digital(value: NgDigital) -> DigitalValue {
        let state = match value.state {
            0 => DigitalState::Zero,
            1 => DigitalState::One,
            _ => DigitalState::Unknown,
        };
        let strength = match value.strength {
            0 => DigitalStrength::Strong,
            1 => DigitalStrength::Resistive,
            2 => DigitalStrength::HighZ,
            _ => DigitalStrength::Undetermined,
        };
        DigitalValue::new(state, strength)
    }

    unsafe extern "C" fn accept_output(pinfo: *mut CoInfo, bit_num: c_uint, value: *mut NgDigital) {
        if pinfo.is_null() || value.is_null() {
            return;
        }
        let runtime = pinfo.cast::<NativeDigitalCosimRuntime>();
        let runtime = unsafe { &mut *runtime };
        let index = bit_num as usize;
        if let Some(slot) = runtime.pending_values.get_mut(index) {
            *slot = Some(from_ng_digital(unsafe { *value }));
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[derive(Default)]
        struct CallbackCounts {
            input_calls: Vec<(c_uint, NgDigital)>,
            step_times: Vec<Value>,
            calls: Vec<CallbackCall>,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        enum CallbackCall {
            Input(c_uint, NgDigital),
            Step(Value),
        }

        unsafe extern "C" fn record_input(
            pinfo: *mut CoInfo,
            which: c_uint,
            value: *mut NgDigital,
        ) {
            let counts = unsafe { &mut *((*pinfo).handle as *mut CallbackCounts) };
            let value = unsafe { *value };
            counts.input_calls.push((which, value));
            counts.calls.push(CallbackCall::Input(which, value));
        }

        unsafe extern "C" fn record_input_and_output_one(
            pinfo: *mut CoInfo,
            which: c_uint,
            value: *mut NgDigital,
        ) {
            unsafe { record_input(pinfo, which, value) };
            let mut output = NgDigital {
                state: 1,
                strength: 0,
            };
            let out_fn = unsafe { (*pinfo).out_fn }.expect("native test runtime has out_fn");
            unsafe { out_fn(pinfo, 0, &mut output) };
        }

        unsafe extern "C" fn record_step(pinfo: *mut CoInfo) {
            let counts = unsafe { &mut *((*pinfo).handle as *mut CallbackCounts) };
            let time = unsafe { (*pinfo).vtime };
            counts.step_times.push(time);
            counts.calls.push(CallbackCall::Step(time));
        }

        #[cfg(windows)]
        fn current_process_library() -> Library {
            libloading::os::windows::Library::this()
                .expect("open current process library")
                .into()
        }

        #[cfg(unix)]
        fn current_process_library() -> Library {
            libloading::os::unix::Library::this().into()
        }

        fn test_runtime_with_method(
            counts: &mut CallbackCounts,
            method: i32,
            input_count: usize,
            inout_count: usize,
        ) -> NativeDigitalCosimRuntime {
            let output_count = 1;
            NativeDigitalCosimRuntime {
                info: CoInfo {
                    in_count: input_count as c_uint,
                    out_count: output_count as c_uint,
                    inout_count: inout_count as c_uint,
                    cleanup: None,
                    step: Some(record_step),
                    in_fn: Some(record_input),
                    out_fn: Some(accept_output),
                    handle: (counts as *mut CallbackCounts).cast(),
                    vtime: 0.0,
                    method,
                    lib_argc: 0,
                    sim_argc: 0,
                    lib_argv: std::ptr::null(),
                    sim_argv: std::ptr::null(),
                    dlopen_fn: Some(cosim_dlopen),
                },
                output_count,
                inout_count,
                output_values: vec![DigitalValue::zero(); output_count + inout_count],
                pending_values: vec![None; output_count + inout_count],
                _library: current_process_library(),
                _lib_arg_strings: Vec::new(),
                _sim_arg_strings: Vec::new(),
                _lib_arg_ptrs: Vec::new(),
                _sim_arg_ptrs: Vec::new(),
            }
        }

        fn test_runtime(counts: &mut CallbackCounts) -> NativeDigitalCosimRuntime {
            test_runtime_with_method(counts, COSIM_METHOD_AFTER_INPUT, 1, 0)
        }

        #[test]
        fn cosim_library_candidates_follow_ngspice_extension_order() {
            let candidates = cosim_library_candidates("cosim_model");
            #[cfg(windows)]
            assert_eq!(
                candidates,
                ["cosim_model", "cosim_model.so", "cosim_model.DLL"]
            );
            #[cfg(target_os = "macos")]
            assert_eq!(
                candidates,
                ["cosim_model", "cosim_model.so", "cosim_model.dylib"]
            );
            #[cfg(all(unix, not(target_os = "macos")))]
            assert_eq!(candidates, ["cosim_model", "cosim_model.so"]);

            let with_so = cosim_library_candidates("cosim_model.so");
            assert_eq!(with_so[0], "cosim_model.so");
            assert!(
                !with_so
                    .iter()
                    .any(|candidate| candidate == "cosim_model.so.so"),
                "ngspice does not append a matching shared-library extension twice"
            );
        }

        #[test]
        fn native_cosim_exposes_official_dlopen_helper() {
            let mut counts = CallbackCounts::default();
            let runtime = test_runtime(&mut counts);
            let dlopen_fn = runtime
                .info
                .dlopen_fn
                .expect("native d_cosim must expose co_info.dlopen_fn like ngspice");

            assert!(
                unsafe { dlopen_fn(std::ptr::null()) }.is_null(),
                "d_cosim dlopen helper must tolerate null pointers from C shims"
            );
        }

        #[test]
        fn after_input_no_event_step_does_not_refeed_inputs_like_ngspice() {
            let mut counts = CallbackCounts::default();
            let mut runtime = test_runtime(&mut counts);

            runtime
                .initialize(0.0, &[DigitalValue::one()], &[])
                .expect("initial input feed succeeds");
            assert_eq!(counts.input_calls.len(), 1);

            runtime
                .step(1.0e-9, &[DigitalValue::zero()], &[], &[])
                .expect("no-event step succeeds");

            assert_eq!(
                counts.input_calls.len(),
                1,
                "ngspice advances an empty d_cosim queue with step() only"
            );
            assert_eq!(counts.step_times, [1.0e-9]);
        }

        #[test]
        fn initialize_defers_input_settling_outputs_until_startup_step_like_ngspice() {
            let mut counts = CallbackCounts::default();
            let mut runtime = test_runtime(&mut counts);
            runtime.info.in_fn = Some(record_input_and_output_one);

            let initial = runtime
                .initialize(0.0, &[DigitalValue::one()], &[])
                .expect("time-zero input feed succeeds");
            assert_eq!(
                initial.outputs,
                [DigitalValue::zero()],
                "ngspice feeds d_cosim inputs at TIME=0 but returns before output() drains pending outputs"
            );

            let startup = runtime
                .startup_step(0.0)
                .expect("startup step drains pending output");
            assert_eq!(startup.outputs, [DigitalValue::one()]);
            assert_eq!(
                counts.calls,
                [
                    CallbackCall::Input(0, to_ng_digital(DigitalValue::one())),
                    CallbackCall::Step(0.0),
                ]
            );
        }

        #[test]
        fn normal_batches_same_time_events_before_single_final_step_like_ngspice() {
            let mut counts = CallbackCounts::default();
            let mut runtime = test_runtime_with_method(&mut counts, COSIM_METHOD_NORMAL, 2, 0);
            let events = [
                DigitalCosimInputEvent {
                    time: 1.0e-9,
                    index: 0,
                    value: DigitalValue::one(),
                },
                DigitalCosimInputEvent {
                    time: 1.0e-9,
                    index: 1,
                    value: DigitalValue::zero(),
                },
            ];

            runtime
                .step(
                    3.0e-9,
                    &[DigitalValue::one(), DigitalValue::zero()],
                    &[],
                    &events,
                )
                .expect("same-time Normal event step succeeds");

            assert_eq!(
                counts.calls,
                [
                    CallbackCall::Step(1.0e-9),
                    CallbackCall::Input(0, to_ng_digital(DigitalValue::one())),
                    CallbackCall::Input(1, to_ng_digital(DigitalValue::zero())),
                    CallbackCall::Step(3.0e-9),
                ],
                "ngspice advances once before a same-time event batch, feeds all changed inputs, then advances to TIME for Normal"
            );
        }

        #[test]
        fn after_input_batches_same_time_events_without_final_step_like_ngspice() {
            let mut counts = CallbackCounts::default();
            let mut runtime = test_runtime_with_method(&mut counts, COSIM_METHOD_AFTER_INPUT, 2, 0);
            let events = [
                DigitalCosimInputEvent {
                    time: 1.0e-9,
                    index: 0,
                    value: DigitalValue::one(),
                },
                DigitalCosimInputEvent {
                    time: 1.0e-9,
                    index: 1,
                    value: DigitalValue::zero(),
                },
            ];

            runtime
                .step(
                    3.0e-9,
                    &[DigitalValue::one(), DigitalValue::zero()],
                    &[],
                    &events,
                )
                .expect("same-time After_input event step succeeds");

            assert_eq!(
                counts.calls,
                [
                    CallbackCall::Input(0, to_ng_digital(DigitalValue::one())),
                    CallbackCall::Input(1, to_ng_digital(DigitalValue::zero())),
                    CallbackCall::Step(1.0e-9),
                ],
                "ngspice feeds every same-time input before one After_input advance and does not advance again to TIME"
            );
        }

        #[test]
        fn both_batches_same_time_events_without_final_step_like_ngspice() {
            let mut counts = CallbackCounts::default();
            let mut runtime = test_runtime_with_method(&mut counts, COSIM_METHOD_BOTH, 2, 0);
            let events = [
                DigitalCosimInputEvent {
                    time: 1.0e-9,
                    index: 0,
                    value: DigitalValue::one(),
                },
                DigitalCosimInputEvent {
                    time: 1.0e-9,
                    index: 1,
                    value: DigitalValue::zero(),
                },
            ];

            runtime
                .step(
                    3.0e-9,
                    &[DigitalValue::one(), DigitalValue::zero()],
                    &[],
                    &events,
                )
                .expect("same-time Both event step succeeds");

            assert_eq!(
                counts.calls,
                [
                    CallbackCall::Step(1.0e-9),
                    CallbackCall::Input(0, to_ng_digital(DigitalValue::one())),
                    CallbackCall::Input(1, to_ng_digital(DigitalValue::zero())),
                    CallbackCall::Step(1.0e-9),
                ],
                "ngspice advances before and after each same-time input batch for Both, with no final advance to TIME"
            );
        }
    }
}

#[cfg(all(feature = "ffi", not(target_arch = "wasm32")))]
use native_cosim::NativeDigitalCosimFactory;

#[cfg(test)]
mod tests {
    use super::*;

    static PANIC_HOOK_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct FailingProcessFactory;

    impl DigitalProcessRuntimeFactory for FailingProcessFactory {
        fn start(&self, _spec: &DigitalProcessSpec) -> CmResult<Box<dyn DigitalProcessRuntime>> {
            Err(CmError::EvaluationError(
                "process factory recovered after poison".to_string(),
            ))
        }
    }

    struct FailingCosimFactory;

    impl DigitalCosimRuntimeFactory for FailingCosimFactory {
        fn start(&self, _spec: &DigitalCosimSpec) -> CmResult<Box<dyn DigitalCosimRuntime>> {
            Err(CmError::EvaluationError(
                "cosim factory recovered after poison".to_string(),
            ))
        }
    }

    fn suppress_expected_panic_output(f: impl FnOnce() + std::panic::UnwindSafe) {
        let _hook_guard = PANIC_HOOK_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(f);
        std::panic::set_hook(previous_hook);
        assert!(result.is_err(), "recovery test must poison the mutex");
    }

    fn poison_process_factory_lock() {
        suppress_expected_panic_output(|| {
            let _guard = lock_external_factory(digital_process_factory());
            panic!("poison process factory lock for recovery test");
        });
    }

    fn poison_cosim_factory_lock() {
        suppress_expected_panic_output(|| {
            let _guard = lock_external_factory(digital_cosim_factory());
            panic!("poison cosim factory lock for recovery test");
        });
    }

    fn process_spec() -> DigitalProcessSpec {
        DigitalProcessSpec {
            process_file: "fake-process".to_string(),
            process_params: Vec::new(),
            input_count: 1,
            output_count: 1,
        }
    }

    fn cosim_spec() -> DigitalCosimSpec {
        DigitalCosimSpec {
            simulation: "fake-sim".to_string(),
            lib_args: Vec::new(),
            sim_args: Vec::new(),
            input_count: 1,
            output_count: 1,
            inout_count: 0,
            queue_size: 1,
            irreversible: 1,
        }
    }

    #[test]
    fn d_process_fifo_base_name_strips_only_ngspice_terminal_pipe_marker() {
        assert_eq!(d_process_fifo_base_name("proc"), None);
        assert_eq!(d_process_fifo_base_name("proc|"), Some("proc"));
        assert_eq!(d_process_fifo_base_name("proc||"), Some("proc"));
        assert_eq!(
            d_process_fifo_base_name("proc|||"),
            Some("proc|"),
            "ngspice strips one trailing pipe, or the two-pipe PSpice-compat marker, not every trailing pipe"
        );
        assert_eq!(d_process_fifo_base_name("|"), Some(""));
    }

    #[test]
    fn d_process_fifo_endpoint_paths_append_ngspice_suffixes() {
        assert_eq!(d_process_fifo_endpoint_paths("proc"), None);
        assert_eq!(
            d_process_fifo_endpoint_paths("proc|"),
            Some(("proc_in".to_string(), "proc_out".to_string()))
        );
        assert_eq!(
            d_process_fifo_endpoint_paths("proc||"),
            Some(("proc_in".to_string(), "proc_out".to_string()))
        );
        assert_eq!(
            d_process_fifo_endpoint_paths(r"\\.\pipe\rspice-process|"),
            Some((
                r"\\.\pipe\rspice-process_in".to_string(),
                r"\\.\pipe\rspice-process_out".to_string(),
            )),
            "Windows named-pipe bases should keep their host path prefix"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_d_process_fifo_mode_exchanges_protocol_over_endpoint_paths() {
        use std::convert::TryInto;

        let unique = format!(
            "rspice-d-process-fifo-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock after epoch")
                .as_nanos()
        );
        let dir = std::env::temp_dir().join(unique);
        std::fs::create_dir(&dir).expect("create d_process FIFO endpoint test directory");

        let base = dir.join("proc");
        let base = base
            .to_str()
            .expect("temporary path should be valid Unicode for process_file");
        let input_path = format!("{base}_in");
        let output_path = format!("{base}_out");
        let process_file = format!("{base}|");
        std::fs::File::create(&input_path).expect("create input endpoint file");
        std::fs::write(&output_path, [0x01, 1, 1, 1])
            .expect("seed output endpoint with header echo and one response byte");

        let spec = DigitalProcessSpec {
            process_file,
            process_params: Vec::new(),
            input_count: 1,
            output_count: 1,
        };
        let factory = native::NativeDigitalProcessFactory;
        let mut runtime = factory
            .start(&spec)
            .expect("native FIFO endpoint runtime starts");
        let mut output = [0u8; 1];
        runtime
            .exchange(2.0e-9, &[1], &mut output)
            .expect("native FIFO endpoint runtime exchanges one packet");
        assert_eq!(output, [1]);
        drop(runtime);

        let input = std::fs::read(&input_path).expect("read input endpoint protocol bytes");
        assert_eq!(&input[..3], &[0x01, 1, 1]);
        let time = Value::from_ne_bytes(input[3..11].try_into().expect("time packet bytes"));
        assert!(
            (time - 2.0e-9).abs() <= f64::EPSILON,
            "exchange should write the native-endian signed time, got {time:e}"
        );
        assert_eq!(input[11], 1);

        std::fs::remove_dir_all(&dir).expect("remove d_process FIFO endpoint test directory");
    }

    #[test]
    fn digital_process_factory_lock_recovers_after_poison() {
        let _ = set_digital_process_runtime_factory(None);
        poison_process_factory_lock();
        let _ = set_digital_process_runtime_factory(Some(Arc::new(FailingProcessFactory)));

        let err = match start_digital_process_runtime(&process_spec()) {
            Ok(_) => panic!("poisoned process factory lock should still use installed provider"),
            Err(err) => err,
        };

        let _ = set_digital_process_runtime_factory(None);
        assert!(
            format!("{err:?}").contains("process factory recovered after poison"),
            "{err:?}"
        );
    }

    #[test]
    fn digital_cosim_factory_lock_recovers_after_poison() {
        let _ = set_digital_cosim_runtime_factory(None);
        poison_cosim_factory_lock();
        let _ = set_digital_cosim_runtime_factory(Some(Arc::new(FailingCosimFactory)));

        let err = match start_digital_cosim_runtime(&cosim_spec()) {
            Ok(_) => panic!("poisoned cosim factory lock should still use installed provider"),
            Err(err) => err,
        };

        let _ = set_digital_cosim_runtime_factory(None);
        assert!(
            format!("{err:?}").contains("cosim factory recovered after poison"),
            "{err:?}"
        );
    }
}

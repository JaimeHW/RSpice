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

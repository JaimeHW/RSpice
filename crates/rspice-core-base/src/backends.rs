//! Which code ran a result: the analog backends that evaluated its devices
//! and the digital engines that ran its discrete half.
//!
//! A run says so in its result because the answer is not the same in every
//! build. A runtime-compiled Verilog-A module runs on the native JIT in the
//! desktop builds, on the bytecode interpreter in the mobile and
//! no-default-feature builds, and on the WebAssembly JIT in the browser; a
//! shipped compact model runs as generated Rust only where its feature is
//! compiled in. A reader comparing two results has to know whether they came
//! from the same code.
//!
//! The circuit build records what it built into the run's
//! [`log_scope`], which the engine enters on every thread it hands part of
//! the run to; the host that runs the analysis keeps a [`BackendRecorder`]
//! there and reads it when the run ends. Where no run records, recording
//! costs one thread-local read.

use std::sync::{Arc, Mutex};

use crate::log_scope::{self, LogScope, LogScopeGuard};

/// What evaluated a result's analog devices, beyond the built-in SPICE
/// device models every build runs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum AnalogBackend {
    /// Runtime-compiled Verilog-A as host machine code (x86-64 or AArch64).
    NativeJit,
    /// Runtime-compiled Verilog-A on the portable bytecode interpreter.
    Interpreter,
    /// Runtime-compiled Verilog-A as WebAssembly, in the browser worker.
    WasmJit,
    /// A shipped compact model compiled into the build as generated Rust.
    GeneratedModel,
}

impl AnalogBackend {
    /// The backend as a person reads it.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NativeJit => "native JIT",
            Self::Interpreter => "interpreter",
            Self::WasmJit => "wasm JIT",
            Self::GeneratedModel => "generated model",
        }
    }
}

/// What ran a result's discrete half.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum DigitalBackend {
    /// The Verilog host: the discrete processes of HDL and mixed
    /// Verilog-AMS modules.
    VerilogHost,
    /// XSPICE event-driven code models.
    Xspice,
}

impl DigitalBackend {
    /// The engine as a person reads it.
    pub const fn label(self) -> &'static str {
        match self {
            Self::VerilogHost => "Verilog host",
            Self::Xspice => "XSPICE",
        }
    }
}

/// A card that asked for a shipped compact model this build does not
/// include, and ran a native port of the same model instead.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelSubstitution {
    /// The instance, as the engine names it (`XAMP.M1`).
    pub instance: String,
    /// The model card it is bound to, as the deck names it.
    pub model: String,
    /// The generated module a build with the model runs.
    pub requested: String,
    /// The feature that compiles that module in.
    pub feature: String,
    /// The native device that ran instead (`EKV 2.6`).
    pub ran: String,
}

/// Which code ran one result: the analog backends, the digital engines, the
/// generated compact models it instantiated, and every card that ran a
/// native port in place of a generated model this build lacks.
///
/// Every list is sorted and holds each entry once, so two runs of one deck
/// on one build compare equal. An empty `analog` list means only the
/// built-in device models ran; an empty `digital` list means the design has
/// no discrete half.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ExecutionBackends {
    pub analog: Vec<AnalogBackend>,
    pub digital: Vec<DigitalBackend>,
    /// The generated compact models, by module name.
    pub generated_models: Vec<String>,
    pub substitutions: Vec<ModelSubstitution>,
}

impl ExecutionBackends {
    /// Add everything `other` ran.
    pub fn merge(&mut self, other: &Self) {
        fn union<T: Ord + Clone>(into: &mut Vec<T>, from: &[T]) {
            for item in from {
                if let Err(at) = into.binary_search(item) {
                    into.insert(at, item.clone());
                }
            }
        }
        union(&mut self.analog, &other.analog);
        union(&mut self.digital, &other.digital);
        union(&mut self.generated_models, &other.generated_models);
        union(&mut self.substitutions, &other.substitutions);
    }

    /// The Verilog-A runtime that ran the result's runtime-compiled modules,
    /// if any ran.
    pub fn verilog_a_runtime(&self) -> Option<AnalogBackend> {
        self.analog
            .iter()
            .copied()
            .find(|backend| *backend != AnalogBackend::GeneratedModel)
    }
}

/// `analog: native JIT, generated model (JUNCAP200); digital: Verilog host`,
/// then one clause per substitution.
impl std::fmt::Display for ExecutionBackends {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("analog: ")?;
        if self.analog.is_empty() {
            f.write_str("built-in devices")?;
        }
        for (index, backend) in self.analog.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            f.write_str(backend.label())?;
            if *backend == AnalogBackend::GeneratedModel && !self.generated_models.is_empty() {
                write!(f, " ({})", self.generated_models.join(", "))?;
            }
        }
        f.write_str("; digital: ")?;
        if self.digital.is_empty() {
            f.write_str("none")?;
        }
        for (index, backend) in self.digital.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            f.write_str(backend.label())?;
        }
        for substitution in &self.substitutions {
            write!(
                f,
                "; {} ran the native {} in place of {} (feature {})",
                substitution.instance,
                substitution.ran,
                substitution.requested,
                substitution.feature
            )?;
        }
        Ok(())
    }
}

/// Where one run records what ran it.
///
/// Shared, because a run may build its circuit on more than one thread: a
/// Monte Carlo run elaborates its samples on workers, each inside the run's
/// log scope. Empty until the run builds a circuit, so a run refused before
/// it built one does not claim it ran on the built-in devices.
#[derive(Debug, Clone, Default)]
pub struct BackendRecorder(Arc<Mutex<Option<ExecutionBackends>>>);

impl BackendRecorder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record on this thread, and on every thread it hands work to, until
    /// the guard is dropped: into the run this thread is executing, or, on a
    /// thread outside any run, into a scope of the recorder's own.
    #[must_use]
    pub fn install(&self) -> LogScopeGuard {
        let scope = log_scope::capture().unwrap_or_else(|| LogScope::new(Arc::new(())));
        log_scope::enter(Some(scope.recording_backends(self.clone())))
    }

    /// Record what a run elsewhere reported it ran: a worker's answer, say.
    pub fn add(&self, backends: &ExecutionBackends) {
        self.lock().get_or_insert_default().merge(backends);
    }

    /// Everything recorded so far, or `None` before the run built a circuit.
    pub fn snapshot(&self) -> Option<ExecutionBackends> {
        self.lock().clone()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Option<ExecutionBackends>> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl LogScope {
    /// The same run, recording what ran it in `recorder`, on this thread and
    /// on every thread it hands work to.
    #[must_use]
    pub fn recording_backends(mut self, recorder: BackendRecorder) -> Self {
        self.backends = Some(Arc::new(recorder));
        self
    }

    fn backend_recorder(&self) -> Option<&BackendRecorder> {
        self.backends.as_deref()?.downcast_ref()
    }
}

/// Whether the run on this thread records which code ran it, so a caller
/// can skip working out what it would record.
#[doc(hidden)]
pub fn is_recording() -> bool {
    log_scope::with_current(|scope| scope.backend_recorder().is_some()).unwrap_or(false)
}

/// Record that the run on this thread ran `backends`, if the run records.
#[doc(hidden)]
pub fn record(backends: &ExecutionBackends) {
    let _ = log_scope::with_current(|scope| {
        if let Some(recorder) = scope.backend_recorder() {
            recorder.add(backends);
        }
    });
}

/// What the run on this thread has recorded so far, for a host that names
/// the code in each result it writes while the run goes on.
pub fn recorded() -> Option<ExecutionBackends> {
    log_scope::with_current(|scope| scope.backend_recorder()?.snapshot()).flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn backends(analog: &[AnalogBackend], generated: &[&str]) -> ExecutionBackends {
        ExecutionBackends {
            analog: analog.to_vec(),
            generated_models: generated.iter().map(|name| (*name).to_owned()).collect(),
            ..ExecutionBackends::default()
        }
    }

    #[test]
    fn merging_keeps_each_entry_once_in_order() {
        let mut ran = backends(&[AnalogBackend::Interpreter], &["R2_CMC"]);
        ran.merge(&backends(
            &[AnalogBackend::GeneratedModel, AnalogBackend::Interpreter],
            &["JUNCAP200", "R2_CMC"],
        ));
        assert_eq!(
            ran,
            backends(
                &[AnalogBackend::Interpreter, AnalogBackend::GeneratedModel],
                &["JUNCAP200", "R2_CMC"],
            )
        );
    }

    #[test]
    fn a_run_records_on_every_thread_it_hands_work_to() {
        let recorder = BackendRecorder::new();
        {
            let _run = recorder.install();
            // A run that has built nothing has run nothing yet.
            assert_eq!(recorded(), None);
            let scope = log_scope::capture();
            std::thread::scope(|threads| {
                threads.spawn(|| {
                    let _entered = log_scope::enter(scope);
                    record(&backends(&[AnalogBackend::NativeJit], &[]));
                });
            });
            record(&ExecutionBackends {
                digital: vec![DigitalBackend::VerilogHost],
                ..ExecutionBackends::default()
            });
        }
        // Outside the run nothing is recorded.
        record(&backends(&[AnalogBackend::WasmJit], &[]));
        assert_eq!(recorded(), None);
        let ran = recorder.snapshot().expect("the run recorded");
        assert_eq!(ran.analog, [AnalogBackend::NativeJit]);
        assert_eq!(ran.digital, [DigitalBackend::VerilogHost]);
    }

    #[test]
    fn the_record_reads_as_one_line() {
        let ran = ExecutionBackends {
            analog: vec![AnalogBackend::NativeJit, AnalogBackend::GeneratedModel],
            digital: vec![DigitalBackend::VerilogHost],
            generated_models: vec!["JUNCAP200".into()],
            substitutions: vec![ModelSubstitution {
                instance: "M1".into(),
                model: "nch".into(),
                requested: "ekv_va".into(),
                feature: "veriloga-model-ekv-va".into(),
                ran: "EKV 2.6".into(),
            }],
        };
        assert_eq!(
            ran.to_string(),
            "analog: native JIT, generated model (JUNCAP200); digital: Verilog host; M1 ran the \
             native EKV 2.6 in place of ekv_va (feature veriloga-model-ekv-va)"
        );
        assert_eq!(
            ExecutionBackends::default().to_string(),
            "analog: built-in devices; digital: none"
        );
    }
}

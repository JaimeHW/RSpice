//! Deadlines and the cooperative cancellation control.
//!
//! A browser call composes an optional `timeoutMilliseconds` deadline with an
//! optional `SharedArrayBuffer`-backed control word into one
//! [`ConfiguredAbort`], and passes that object to an abort-aware core
//! entrypoint. There is no non-abort execution path.

use rspice_core::AbortSignal;

use crate::DetailedWasmResult;
use crate::errors::WasmError;
use crate::options::{MAX_TIMEOUT_MILLISECONDS, WasmExecutionOptions};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExecutionDeadline(Option<std::time::Instant>);

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExecutionDeadline(Option<f64>);

impl ExecutionDeadline {
    pub(crate) fn new(timeout_milliseconds: Option<u32>) -> DetailedWasmResult<Self> {
        if let Some(timeout) = timeout_milliseconds
            && timeout > MAX_TIMEOUT_MILLISECONDS
        {
            return Err(Box::new(WasmError::invalid_argument(format!(
                "timeoutMilliseconds must not exceed {MAX_TIMEOUT_MILLISECONDS}, got {timeout}"
            ))));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(Self(timeout_milliseconds.map(|timeout| {
                std::time::Instant::now() + std::time::Duration::from_millis(u64::from(timeout))
            })))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let deadline = timeout_milliseconds
                .map(|timeout| monotonic_now_milliseconds().map(|now| now + f64::from(timeout)))
                .transpose()?;
            Ok(Self(deadline))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn expired(&self) -> bool {
        self.0
            .is_some_and(|deadline| std::time::Instant::now() >= deadline)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn expired(&self) -> bool {
        self.0.is_some_and(|deadline| {
            monotonic_now_milliseconds().map_or(true, |now| now >= deadline)
        })
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn monotonic_now_milliseconds() -> DetailedWasmResult<f64> {
    let global = js_sys::global();
    let performance =
        js_sys::Reflect::get(&global, &JsValue::from_str("performance")).map_err(|_| {
            Box::new(WasmError::invalid_argument(
                "timeoutMilliseconds requires a host performance clock".to_string(),
            ))
        })?;
    let now = js_sys::Reflect::get(&performance, &JsValue::from_str("now"))
        .ok()
        .and_then(|value| value.dyn_into::<js_sys::Function>().ok())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "timeoutMilliseconds requires performance.now()".to_string(),
            ))
        })?;
    let value = now
        .call0(&performance)
        .ok()
        .and_then(|value| value.as_f64())
        .filter(|value| value.is_finite())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "performance.now() did not return a finite timestamp".to_string(),
            ))
        })?;
    Ok(value)
}

/// Compose one frontend cancellation source with the per-call deadline.
/// Every browser analysis passes this object to an abort-aware core entrypoint.
pub(crate) struct ConfiguredAbort<'a> {
    external: &'a dyn AbortSignal,
    deadline: ExecutionDeadline,
}

impl<'a> ConfiguredAbort<'a> {
    /// Compose an already-started deadline with a cancellation source.
    ///
    /// [`ExecutionScope`] starts the deadline once, when the options object
    /// has been validated and before any parser work, and then hands the same
    /// deadline to the abort every runner polls.
    pub(crate) const fn with_deadline(
        deadline: ExecutionDeadline,
        external: &'a dyn AbortSignal,
    ) -> Self {
        Self { external, deadline }
    }
}

impl AbortSignal for ConfiguredAbort<'_> {
    fn is_aborted(&self) -> bool {
        self.external.is_aborted() || self.deadline.expired()
    }
}

pub(crate) fn aborted_error() -> Box<WasmError> {
    Box::new(WasmError::from_simulation_error(
        rspice_core::engine::SimulationError::Aborted,
    ))
}

pub(crate) fn ensure_not_aborted(abort: &dyn AbortSignal) -> DetailedWasmResult<()> {
    if abort.is_aborted() {
        Err(aborted_error())
    } else {
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct JsSharedCancellationControl {
    pub(crate) view: js_sys::Int32Array,
    pub(crate) index: u32,
}

pub(crate) struct JsExecutionRequest {
    pub(crate) options: WasmExecutionOptions,
    pub(crate) timeout_milliseconds: Option<u32>,
    pub(crate) cancellation: Option<JsSharedCancellationControl>,
}

thread_local! {
    static ACTIVE_SHARED_CANCELLATION: std::cell::RefCell<Option<JsSharedCancellationControl>> =
        const { std::cell::RefCell::new(None) };
}

/// The signal itself owns no JavaScript handle and therefore satisfies the
/// core's Send + Sync contract without unsafe code. The browser build is
/// deliberately single-threaded; the per-agent control view lives in TLS.
pub(crate) struct JsSharedAbortSignal {
    pub(crate) enabled: bool,
}

impl AbortSignal for JsSharedAbortSignal {
    fn is_aborted(&self) -> bool {
        if !self.enabled {
            return false;
        }
        ACTIVE_SHARED_CANCELLATION.with(|active| {
            active
                .borrow()
                .as_ref()
                .is_none_or(|control| js_sys::Atomics::load(&control.view, control.index) != Ok(0))
        })
    }
}

pub(crate) struct ActiveSharedCancellationGuard {
    installed: bool,
}

impl ActiveSharedCancellationGuard {
    pub(crate) fn install(
        control: Option<JsSharedCancellationControl>,
    ) -> DetailedWasmResult<Self> {
        let installed = control.is_some();
        ACTIVE_SHARED_CANCELLATION.with(|active| {
            let mut active = active.borrow_mut();
            if active.is_some() {
                return Err(Box::new(WasmError::invalid_argument(
                    "nested WASM execution is not supported".to_string(),
                )));
            }
            *active = control;
            Ok(Self { installed })
        })
    }
}

impl Drop for ActiveSharedCancellationGuard {
    fn drop(&mut self) {
        if self.installed {
            ACTIVE_SHARED_CANCELLATION.with(|active| {
                *active.borrow_mut() = None;
            });
        }
    }
}

/// One decoded browser call: validated policy, installed cancellation
/// control, and a started deadline.
///
/// Every `#[wasm_bindgen]` export opens exactly one of these and then calls a
/// Rust entry point with [`Self::options`] and [`Self::abort`]. Keeping the
/// sequence in one place is what makes "a frontend may not call a non-abort
/// wrapper" checkable by reading one function instead of every export.
pub(crate) struct ExecutionScope {
    options: WasmExecutionOptions,
    shared: JsSharedAbortSignal,
    deadline: ExecutionDeadline,
    /// Uninstalls the shared control word when the call returns, including on
    /// every error path.
    _cancellation: ActiveSharedCancellationGuard,
}

impl ExecutionScope {
    /// Decode the options object, install its cancellation control, and start
    /// the deadline, in that order.
    pub(crate) fn open(options: wasm_bindgen::JsValue) -> DetailedWasmResult<Self> {
        let request = crate::js_interop::execution_request_from_js(options)?;
        let shared = JsSharedAbortSignal {
            enabled: request.cancellation.is_some(),
        };
        let cancellation = ActiveSharedCancellationGuard::install(request.cancellation)?;
        let deadline = ExecutionDeadline::new(request.timeout_milliseconds)?;
        Ok(Self {
            options: request.options,
            shared,
            deadline,
            _cancellation: cancellation,
        })
    }

    /// The validated browser execution policy for this call.
    pub(crate) const fn options(&self) -> &WasmExecutionOptions {
        &self.options
    }

    /// The core resource policy for this call.
    pub(crate) fn resource_limits(&self) -> rspice_core::ResourceLimits {
        self.options.resource_limits.to_core()
    }

    /// The single abort source every runner in this call polls.
    pub(crate) fn abort(&self) -> ConfiguredAbort<'_> {
        ConfiguredAbort::with_deadline(self.deadline, &self.shared)
    }
}

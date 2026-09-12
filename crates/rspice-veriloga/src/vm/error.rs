//! Failures raised while executing or constructing a model.
//!
//! Covers both bytecode execution faults and the construction-time failures
//! that must not fall back — a requested native compilation that could not be
//! satisfied surfaces here rather than quietly running the interpreter.

/// VM execution errors.
#[derive(Debug, Clone, PartialEq)]
pub enum VmError {
    StackUnderflow(&'static str),
    InvalidInstruction(&'static str),
    /// Native JIT compilation failed while native execution was required.
    NativeJit(String),
    /// The caller cancelled device construction or a wait for shared native code.
    CompilationCancelled,
    /// Browser secondary-WASM compilation or dispatch failed while WASM JIT
    /// execution was required.
    WasmJit(String),
    /// Invalid model or instance parameter value.
    ParameterValue(String),
    /// Structurally invalid or corrupted compiled-model artifact.
    InvalidModel(String),
    /// A model expression produced NaN or infinity at a solver boundary.
    ///
    /// This is a property of the *point* the solver handed the device, not of
    /// the model: `ln(V(p,n)+0.1)` is finite at every accepted operating point
    /// of a well-posed deck and undefined at an overshooting Newton iterate.
    /// A consumer may therefore reject the iterate and retry from a smaller
    /// step or a stepped source. Every refusal that would fail identically at
    /// every other iterate belongs in [`VmError::InvalidRuntimeOperation`].
    InvalidNumericResult(String),
    /// A runtime refusal that shares the shape of an invalid numeric result
    /// but does not depend on the values of this iterate.
    ///
    /// A compiled layout the runtime will not execute, or an index that is
    /// finite and still outside the representable range, fails the same way at
    /// every point. Retrying such a refusal spends the whole convergence
    /// ladder to report the message it already had.
    InvalidRuntimeOperation(String),
    /// Invalid simulator-to-device runtime configuration.
    InvalidRuntimeConfiguration(String),
    /// An analog system task could not be staged or delivered.
    AnalogTask(String),
    /// Runtime array index outside the declared bounds
    IndexOutOfBounds {
        index: i64,
        lower: i64,
        upper: i64,
    },
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmError::StackUnderflow(msg) => write!(f, "Stack underflow: {}", msg),
            VmError::InvalidInstruction(msg) => write!(f, "Invalid instruction: {}", msg),
            VmError::NativeJit(msg) => write!(f, "native JIT error: {}", msg),
            VmError::CompilationCancelled => f.write_str("model compilation cancelled"),
            VmError::WasmJit(msg) => write!(f, "browser WASM JIT error: {msg}"),
            VmError::ParameterValue(msg) => write!(f, "parameter value error: {msg}"),
            VmError::InvalidModel(msg) => write!(f, "invalid compiled model: {msg}"),
            VmError::InvalidNumericResult(msg) => write!(f, "invalid numeric result: {msg}"),
            VmError::InvalidRuntimeOperation(msg) => {
                write!(f, "invalid runtime operation: {msg}")
            }
            VmError::InvalidRuntimeConfiguration(msg) => {
                write!(f, "invalid runtime configuration: {msg}")
            }
            VmError::AnalogTask(msg) => write!(f, "analog system-task error: {msg}"),
            VmError::IndexOutOfBounds {
                index,
                lower,
                upper,
            } => write!(
                f,
                "Array index {} outside declared bounds [{}:{}]",
                index, lower, upper
            ),
        }
    }
}

impl std::error::Error for VmError {}

#[cfg(test)]
mod tests {
    use super::VmError;

    #[test]
    fn native_jit_error_display_mentions_no_fallback() {
        let err = VmError::NativeJit(
            "model rjit: unsupported canonical op EvaluateEquation; no interpreter fallback"
                .to_string(),
        );
        let msg = err.to_string();
        assert!(msg.contains("native JIT"));
        assert!(msg.contains("no interpreter fallback"));
    }

    #[test]
    fn invalid_model_error_identifies_compiled_artifact_failure() {
        let err = VmError::InvalidModel("assignment range overflow".into());
        assert_eq!(
            err.to_string(),
            "invalid compiled model: assignment range overflow"
        );
    }
}

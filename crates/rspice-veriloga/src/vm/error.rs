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
    /// Browser secondary-WASM compilation or dispatch failed while WASM JIT
    /// execution was required.
    WasmJit(String),
    /// Invalid model or instance parameter value.
    ParameterValue(String),
    /// A model expression produced NaN or infinity at a solver boundary.
    InvalidNumericResult(String),
    /// Invalid simulator-to-device runtime configuration.
    InvalidRuntimeConfiguration(String),
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
            VmError::WasmJit(msg) => write!(f, "browser WASM JIT error: {msg}"),
            VmError::ParameterValue(msg) => write!(f, "parameter value error: {msg}"),
            VmError::InvalidNumericResult(msg) => write!(f, "invalid numeric result: {msg}"),
            VmError::InvalidRuntimeConfiguration(msg) => {
                write!(f, "invalid runtime configuration: {msg}")
            }
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
}

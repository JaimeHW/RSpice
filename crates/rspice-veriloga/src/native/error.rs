use smol_str::SmolStr;

pub type JitResult<T> = Result<T, JitError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JitError {
    UnsupportedTarget { target: SmolStr, reason: SmolStr },
    UnsupportedCanonicalOp { model: SmolStr, op: SmolStr },
    InvalidCanonicalIr { model: SmolStr, detail: SmolStr },
    Lowering { model: SmolStr, detail: SmolStr },
    Verifier { model: SmolStr, detail: SmolStr },
    RegisterAllocation { model: SmolStr, detail: SmolStr },
    Encoding { model: SmolStr, detail: SmolStr },
    Relocation { model: SmolStr, detail: SmolStr },
    ExecutableMemory { detail: SmolStr },
    AbiMismatch { model: SmolStr, detail: SmolStr },
    MissingEntryPoint { model: SmolStr, entry: SmolStr },
    InternalCompilerError { model: SmolStr, detail: SmolStr },
}

impl JitError {
    pub fn unsupported_current_optir(model: impl Into<SmolStr>) -> Self {
        Self::UnsupportedCanonicalOp {
            model: model.into(),
            op: SmolStr::new("EvaluateEquation"),
        }
    }
}

impl std::fmt::Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitError::UnsupportedTarget { target, reason } => write!(
                f,
                "target {target} is not supported by the native JIT: {reason}; no interpreter fallback"
            ),
            JitError::UnsupportedCanonicalOp { model, op } => write!(
                f,
                "model {model}: native JIT does not support canonical op {op}; no interpreter fallback"
            ),
            JitError::InvalidCanonicalIr { model, detail } => write!(
                f,
                "model {model}: invalid canonical IR for native JIT: {detail}; no interpreter fallback"
            ),
            JitError::Lowering { model, detail } => write!(
                f,
                "model {model}: native JIT lowering failed: {detail}; no interpreter fallback"
            ),
            JitError::Verifier { model, detail } => write!(
                f,
                "model {model}: native JIT verifier failed: {detail}; no interpreter fallback"
            ),
            JitError::RegisterAllocation { model, detail } => write!(
                f,
                "model {model}: native JIT register allocation failed: {detail}; no interpreter fallback"
            ),
            JitError::Encoding { model, detail } => write!(
                f,
                "model {model}: native JIT encoding failed: {detail}; no interpreter fallback"
            ),
            JitError::Relocation { model, detail } => write!(
                f,
                "model {model}: native JIT relocation failed: {detail}; no interpreter fallback"
            ),
            JitError::ExecutableMemory { detail } => {
                write!(
                    f,
                    "native JIT executable memory failed: {detail}; no interpreter fallback"
                )
            }
            JitError::AbiMismatch { model, detail } => write!(
                f,
                "model {model}: native JIT ABI mismatch: {detail}; no interpreter fallback"
            ),
            JitError::MissingEntryPoint { model, entry } => write!(
                f,
                "model {model}: native JIT missing entry point {entry}; no interpreter fallback"
            ),
            JitError::InternalCompilerError { model, detail } => write!(
                f,
                "model {model}: native JIT internal compiler error: {detail}; no interpreter fallback"
            ),
        }
    }
}

impl std::error::Error for JitError {}

#[cfg(test)]
mod tests {
    use super::JitError;

    #[test]
    fn unsupported_op_error_names_hard_fail_contract() {
        let msg = JitError::unsupported_current_optir("rjit").to_string();
        assert!(msg.contains("EvaluateEquation"));
        assert!(msg.contains("no interpreter fallback"));
    }
}

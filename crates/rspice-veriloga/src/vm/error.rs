/// VM execution errors.
#[derive(Debug, Clone, PartialEq)]
pub enum VmError {
    StackUnderflow(&'static str),
    InvalidInstruction(&'static str),
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmError::StackUnderflow(msg) => write!(f, "Stack underflow: {}", msg),
            VmError::InvalidInstruction(msg) => write!(f, "Invalid instruction: {}", msg),
        }
    }
}

impl std::error::Error for VmError {}

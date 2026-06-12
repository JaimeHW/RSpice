/// VM execution errors.
#[derive(Debug, Clone, PartialEq)]
pub enum VmError {
    StackUnderflow(&'static str),
    InvalidInstruction(&'static str),
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

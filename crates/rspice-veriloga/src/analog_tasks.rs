//! Ordered analog task calls shared by frontend and executable lowerings.

pub use rspice_veriloga_runtime::AnalogTaskKind;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Hidden scalar results read by simulator-control task consumers.
pub(crate) const SIMULATOR_CONTROL_TASK_VARIABLES: [&str; 2] = ["$bound_step", "$discontinuity"];

/// Where one task call was written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalogTaskOrigin {
    pub site: u32,
    pub file: SmolStr,
    /// One-based.
    pub line: u32,
}

/// An argument's language type must survive numerical backend lowering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalogTaskOperand<E> {
    Real(E),
    Integer(E),
    String(SmolStr),
}

/// A call at one source site. The enclosing guard belongs to the flattened
/// representation; structured regions carry the guard in their control flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalogTaskCall<E, S> {
    pub kind: AnalogTaskKind,
    pub site: u32,
    pub guard: Option<E>,
    pub arguments: Vec<AnalogTaskOperand<E>>,
    pub span: S,
    pub initialization: bool,
}

impl<E, S> AnalogTaskCall<E, S> {
    pub(crate) fn finish_operand(&self) -> Option<&E> {
        match (self.kind, self.arguments.as_slice()) {
            (AnalogTaskKind::Finish, [AnalogTaskOperand::Integer(value)]) => Some(value),
            _ => None,
        }
    }

    /// Every backend executes a `$finish` with its level and the output tasks.
    pub(crate) fn is_executable(&self) -> bool {
        self.finish_operand().is_some() || self.kind.is_output()
    }

    /// An output task's numeric operands in source order; the string literals
    /// stay in the task table for the host's formatter.
    pub(crate) fn output_operands(&self) -> Option<Vec<&E>> {
        self.kind.is_output().then(|| {
            self.arguments
                .iter()
                .filter_map(|argument| match argument {
                    AnalogTaskOperand::Real(value) | AnalogTaskOperand::Integer(value) => {
                        Some(value)
                    }
                    AnalogTaskOperand::String(_) => None,
                })
                .collect()
        })
    }

    /// The one program a native plan evaluates for this call: the `$finish`
    /// level, or the program that evaluates an output task's operands and
    /// journals them itself (a plan merges them into one, see
    /// `NativeProgram::analog_output`).
    #[cfg(any(feature = "native", feature = "wasm-jit"))]
    pub(crate) fn plan_operand(&self) -> Option<&E> {
        self.finish_operand().or_else(
            || match (self.kind.is_output(), self.arguments.as_slice()) {
                (true, [AnalogTaskOperand::Real(value)]) => Some(value),
                _ => None,
            },
        )
    }

    pub fn map<T, U>(&self, span: U, mut lower: impl FnMut(&E) -> T) -> AnalogTaskCall<T, U> {
        self.try_map(span, |expression| {
            Ok::<_, std::convert::Infallible>(lower(expression))
        })
        .expect("infallible expression lowering")
    }

    /// Guard first, followed by numeric arguments in source order.
    pub fn expressions(&self) -> impl Iterator<Item = &E> {
        self.guard
            .iter()
            .chain(self.arguments.iter().filter_map(|argument| match argument {
                AnalogTaskOperand::Real(value) | AnalogTaskOperand::Integer(value) => Some(value),
                AnalogTaskOperand::String(_) => None,
            }))
    }

    pub fn expressions_mut(&mut self) -> impl Iterator<Item = &mut E> {
        self.guard
            .iter_mut()
            .chain(
                self.arguments
                    .iter_mut()
                    .filter_map(|argument| match argument {
                        AnalogTaskOperand::Real(value) | AnalogTaskOperand::Integer(value) => {
                            Some(value)
                        }
                        AnalogTaskOperand::String(_) => None,
                    }),
            )
    }

    /// Change the expression representation without losing argument types or
    /// call identity. Every lowering uses the same traversal order.
    pub fn try_map<T, U, Error>(
        &self,
        span: U,
        mut lower: impl FnMut(&E) -> Result<T, Error>,
    ) -> Result<AnalogTaskCall<T, U>, Error> {
        let guard = self.guard.as_ref().map(&mut lower).transpose()?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| {
                Ok(match argument {
                    AnalogTaskOperand::Real(value) => AnalogTaskOperand::Real(lower(value)?),
                    AnalogTaskOperand::Integer(value) => AnalogTaskOperand::Integer(lower(value)?),
                    AnalogTaskOperand::String(value) => AnalogTaskOperand::String(value.clone()),
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(AnalogTaskCall {
            kind: self.kind,
            site: self.site,
            guard,
            arguments,
            span,
            initialization: self.initialization,
        })
    }
}

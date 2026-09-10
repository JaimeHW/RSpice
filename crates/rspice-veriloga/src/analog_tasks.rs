//! Ordered analog task calls shared by frontend and executable lowerings.

pub use rspice_veriloga_runtime::AnalogTaskKind;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Hidden scalar results read by simulator-control task consumers.
pub(crate) const SIMULATOR_CONTROL_TASK_VARIABLES: [&str; 2] = ["$bound_step", "$discontinuity"];

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

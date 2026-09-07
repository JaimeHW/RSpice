//! Execution phases shared by the compiler and simulator backends.

/// Declaration assignments precede analog initial blocks, and both precede
/// ordinary analog evaluation. These phases repeat when a new analysis starts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum AnalogEvaluationPhase {
    #[default]
    Evaluation = 0,
    Declarations = 1,
    Initialization = 2,
}

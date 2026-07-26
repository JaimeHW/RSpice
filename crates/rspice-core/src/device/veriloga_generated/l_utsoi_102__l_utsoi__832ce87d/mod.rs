#[cfg(feature = "veriloga-builtins-noise")]
pub mod noise;
pub mod state;
mod stamp;

#[cfg(feature = "veriloga-builtins-noise")]
pub use noise::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor, NOISE_SOURCES};
pub use state::{Instance, Parameters};

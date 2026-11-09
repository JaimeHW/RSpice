//! LTspice special-function devices.
//!
//! Each device is written `Annn n1 ... n8 FUNCTION [params]`. Terminal 8 is
//! the device common: every input is sensed and every output is driven
//! relative to it, and any other terminal tied to it is unused. Inputs draw
//! no current.
//!
//! This module holds the device equations and their LTspice parameter sets;
//! the engine hosts each instance as an analog behavioral model.

mod logic;
mod mixed;
mod ota;
mod sequential;

#[doc(hidden)]
pub use logic::{Gate, GateFunction, OutputStage, Schmitt};
#[doc(hidden)]
pub use mixed::{Modulator, PhaseDetector, SampleHold, Varistor};
#[doc(hidden)]
pub use ota::{Ota, OtaDrive, OtaNoise};
#[doc(hidden)]
pub use sequential::{Sequential, SequentialFunction};

use crate::Value;
use std::fmt;

/// A special-function instance whose parameters LTspice would not accept,
/// or that names a terminal configuration it does not simulate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(hidden)]
pub struct SpecialFunctionError(String);

impl SpecialFunctionError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for SpecialFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SpecialFunctionError {}

/// The instance parameters of one device, each read at most once; a name the
/// device does not define is refused when the reading is finished.
struct InstanceParams<'a> {
    device: &'static str,
    params: &'a [(String, Value)],
    read: Vec<bool>,
}

impl<'a> InstanceParams<'a> {
    fn new(device: &'static str, params: &'a [(String, Value)]) -> Self {
        Self {
            device,
            params,
            read: vec![false; params.len()],
        }
    }

    /// The last assignment of `name`, ignoring ASCII case.
    fn get(&mut self, name: &str) -> Result<Option<Value>, SpecialFunctionError> {
        self.get_any(&[name])
    }

    /// The last assignment of any of `names`, the spellings of one parameter.
    fn get_any(&mut self, names: &[&str]) -> Result<Option<Value>, SpecialFunctionError> {
        match self.last(names) {
            Some((name, value)) if !value.is_finite() => Err(SpecialFunctionError::new(format!(
                "{} parameter {name} is not finite ({value})",
                self.device
            ))),
            found => Ok(found.map(|(_, value)| value)),
        }
    }

    /// A compliance rail: `name`, or `default` when it is not given. It may
    /// be infinite, as LTspice reads `1e309`, and then limits nothing.
    fn rail(&mut self, name: &str, default: Value) -> Result<Value, SpecialFunctionError> {
        match self.last(&[name]) {
            Some((name, value)) if value.is_nan() => Err(SpecialFunctionError::new(format!(
                "{} parameter {name} is not a number",
                self.device
            ))),
            found => Ok(found.map_or(default, |(_, value)| value)),
        }
    }

    /// The last assignment of any of `names`, marked read.
    fn last(&mut self, names: &[&str]) -> Option<(&'a str, Value)> {
        let mut found = None;
        for (index, (candidate, value)) in self.params.iter().enumerate() {
            if names
                .iter()
                .any(|name| candidate.eq_ignore_ascii_case(name))
            {
                self.read[index] = true;
                found = Some((candidate.as_str(), *value));
            }
        }
        found
    }

    fn or(&mut self, name: &str, default: Value) -> Result<Value, SpecialFunctionError> {
        Ok(self.get(name)?.unwrap_or(default))
    }

    /// Take a Schmitt trigger's `Vt` and `Vh`. LTspice 26.1.1 accepts them on
    /// every special function and reads them only on the Schmitt trigger:
    /// anywhere else neither moves a threshold or an output.
    fn schmitt_trip_points(&mut self) -> Result<(), SpecialFunctionError> {
        self.get("VT")?;
        self.get("VH")?;
        Ok(())
    }

    /// A bare flag such as `linear`, or `linear=1`.
    fn flag(&mut self, name: &str) -> Result<bool, SpecialFunctionError> {
        Ok(self.get(name)?.is_some_and(|value| value != 0.0))
    }

    fn finish(self) -> Result<(), SpecialFunctionError> {
        let unknown: Vec<&str> = self
            .params
            .iter()
            .zip(&self.read)
            .filter(|(_, read)| !**read)
            .map(|((name, _), _)| name.as_str())
            .collect();
        if unknown.is_empty() {
            Ok(())
        } else {
            Err(SpecialFunctionError::new(format!(
                "{} does not define parameter {}",
                self.device,
                unknown.join(", ")
            )))
        }
    }
}

fn require(condition: bool, message: impl FnOnce() -> String) -> Result<(), SpecialFunctionError> {
    if condition {
        Ok(())
    } else {
        Err(SpecialFunctionError::new(message()))
    }
}

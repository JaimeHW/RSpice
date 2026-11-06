//! LTspice special-function devices.
//!
//! Each device is written `Annn n1 ... n8 FUNCTION [params]`. Terminal 8 is
//! the device common: every input is sensed and every output is driven
//! relative to it, and any other terminal tied to it is unused. Inputs draw
//! no current.
//!
//! This module holds the device equations and their LTspice parameter sets;
//! the engine hosts each instance as an analog behavioral model.

mod ota;

pub use ota::{Ota, OtaDrive, OtaNoise};

use crate::Value;
use std::fmt;

/// A special-function instance whose parameters LTspice would not accept,
/// or that names a terminal configuration it does not simulate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialFunctionError(String);

impl SpecialFunctionError {
    pub fn new(message: impl Into<String>) -> Self {
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
        let mut found = None;
        for (index, (candidate, value)) in self.params.iter().enumerate() {
            if candidate.eq_ignore_ascii_case(name) {
                self.read[index] = true;
                found = Some(*value);
            }
        }
        match found {
            Some(value) if !value.is_finite() => Err(SpecialFunctionError::new(format!(
                "{} parameter {name} is not finite ({value})",
                self.device
            ))),
            found => Ok(found),
        }
    }

    fn or(&mut self, name: &str, default: Value) -> Result<Value, SpecialFunctionError> {
        Ok(self.get(name)?.unwrap_or(default))
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

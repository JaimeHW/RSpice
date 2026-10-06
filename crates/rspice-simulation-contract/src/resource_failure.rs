//! Portable resource failures that retain device and allocation context.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Resource failures other than the existing context-free policy limit.
/// Neither case is a convergence failure or eligible for automatic retry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceFailure {
    DeviceLimit {
        instance: String,
        resource: String,
        requested: usize,
        limit: usize,
    },
    Allocation {
        object: String,
        detail: String,
    },
}

impl ResourceFailure {
    /// The stable core error code, retained without parsing display text.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::DeviceLimit { .. } => "resource_limit",
            Self::Allocation { .. } => "allocation_failed",
        }
    }
}

impl fmt::Display for ResourceFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceLimit {
                instance,
                resource,
                requested,
                limit,
            } => write!(
                formatter,
                "Device '{instance}': {resource} limit exceeded: requested {requested}, limit {limit}"
            ),
            Self::Allocation { object, detail } => {
                write!(formatter, "Unable to allocate {object}: {detail}")
            }
        }
    }
}

impl std::error::Error for ResourceFailure {}

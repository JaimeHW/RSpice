//! The build this process runs: its source commit and the target it was
//! compiled for.
//!
//! The shell's build script stamps these values, and the shell installs them
//! here once at startup. The services and runs below the shell read the build
//! here instead of compiling in variables that only the shell's build emits.
//! The managed runtime's verification key is deliberately not part of it: a
//! trust anchor stays a compile-time constant where it is verified, so an
//! uninstalled key can never be read.

use std::sync::OnceLock;

/// What the shell's build script stamped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildIdentity {
    /// The source commit, or `"unknown"` for a build made outside a checkout.
    pub hash: &'static str,
    /// The Rust target triple the build was compiled for.
    pub target: &'static str,
    /// The architecture of that target.
    pub arch: &'static str,
}

static INSTALLED: OnceLock<BuildIdentity> = OnceLock::new();

impl BuildIdentity {
    /// Install the identity of this process. The shell does this once at
    /// startup, before anything reads it; a process has one build, so a later
    /// call keeps the first identity.
    pub fn install(identity: Self) {
        let installed = INSTALLED.get_or_init(|| identity);
        debug_assert_eq!(*installed, identity, "a process has one build identity");
    }

    /// The installed identity, or `None` before the shell installed one.
    pub fn installed() -> Option<&'static Self> {
        INSTALLED.get()
    }

    /// The source commit, or `"unknown"` when no identity is installed: the
    /// same answer a build made outside a checkout gives.
    pub fn source_hash() -> &'static str {
        Self::installed().map_or("unknown", |identity| identity.hash)
    }
}

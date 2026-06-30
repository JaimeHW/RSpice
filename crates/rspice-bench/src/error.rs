//! Error type for the benchmark rig.
//!
//! The rig deliberately avoids `unwrap()`/`expect()` on I/O and process
//! paths: every failure is routed through [`BenchError`] with enough context
//! to act on (which file, which executable, which phase).

use std::fmt;
use std::path::PathBuf;

/// All failures the benchmark rig can report.
#[derive(Debug)]
pub enum BenchError {
    /// An I/O operation failed; `context` says what was being attempted.
    Io {
        /// Human-readable description of the failed operation.
        context: String,
        /// Underlying OS error.
        source: std::io::Error,
    },
    /// The RSpice executable could not be located.
    MissingRspice {
        /// Candidate paths that were probed, in order.
        tried: Vec<PathBuf>,
    },
    /// An explicitly configured executable path does not exist.
    ExeNotFound {
        /// Environment variable that supplied the path.
        env_var: &'static str,
        /// The configured path that was not a file.
        path: PathBuf,
    },
    /// The circuits directory contained no `*.cir` decks.
    NoDecks {
        /// Directory that was scanned.
        dir: PathBuf,
    },
    /// Scoreboard serialization failed.
    Json {
        /// Human-readable description of what was being serialized.
        context: String,
        /// Underlying serde_json error.
        source: serde_json::Error,
    },
    /// Native JIT benchmark setup or execution failed.
    NativeJit {
        /// Human-readable benchmark failure.
        message: String,
    },
    /// Internal invariant violation (a bug in the rig itself).
    Internal(&'static str),
}

impl fmt::Display for BenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { context, .. } => write!(f, "{context}"),
            Self::MissingRspice { tried } => {
                write!(
                    f,
                    "could not locate the RSpice executable; set RSPICE_BENCH_RSPICE or build it \
                     with `cargo build --release -p rspice-cli`. Paths probed:"
                )?;
                for path in tried {
                    write!(f, " `{}`", path.display())?;
                }
                Ok(())
            }
            Self::ExeNotFound { env_var, path } => write!(
                f,
                "{env_var} points at `{}`, which is not an existing file",
                path.display()
            ),
            Self::NoDecks { dir } => write!(
                f,
                "no benchmark decks (*.cir) found in `{}`; run `rspice-bench gen` and check the \
                 directory path",
                dir.display()
            ),
            Self::Json { context, .. } => write!(f, "{context}"),
            Self::NativeJit { message } => write!(f, "{message}"),
            Self::Internal(message) => write!(f, "internal error: {message}"),
        }
    }
}

impl std::error::Error for BenchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl BenchError {
    /// Builds an [`BenchError::Io`] from a context string and an OS error.
    pub fn io(context: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }
}

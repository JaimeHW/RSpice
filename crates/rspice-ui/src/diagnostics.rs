//! Diagnostics the application reports about itself.
//!
//! Two models had grown for the same concern in two different places: the
//! console message the shell prints, and the structured, filterable log the
//! console panel reads. They lived three layers apart — one in the
//! application root, one under the panel that renders it — so the simulation
//! and schematic layers had to reach up through the whole shell to report a
//! diagnostic about their own work.
//!
//! They are one model and they live here, below everything that produces a
//! diagnostic and above `state`, which a log entry may anchor to.

mod console;
mod log;

pub use console::{ConsoleLevel, ConsoleMessage};
pub use log::{LogAnchor, LogBuffer, LogEntry, LogSeverity, LogSource};

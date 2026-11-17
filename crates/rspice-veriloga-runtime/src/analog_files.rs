//! The open files of one analog instance (LRM 2.4 section 9.5), as the host
//! keeps them.
//!
//! The table is the host's: the run's files, path confinement, descriptor
//! numbering and formatting belong to the simulator, which shares them with
//! its digital kernel. A model reaches it only through this interface.
//!
//! # Transactions
//!
//! What an evaluation does to the table is a candidate. Every evaluation
//! starts again from the accepted table, so a `$fopen` or a `$fscanf` of a
//! rejected point leaves no descriptor, position or read behind (LRM 2.4
//! section 9.5.9); accepting the point makes its last evaluation's table the
//! accepted one. Writes never happen during an evaluation: the file output
//! tasks are journaled with the other analog tasks, and the host delivers an
//! accepted point's calls to [`AnalogFileTable::deliver`] after the point
//! is accepted.

use std::fmt;

/// One write a delivered file task made, for the run's output records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalogFileWrite {
    /// To the standard output: descriptor `32'h8000_0001` or multichannel
    /// bit 0.
    Stdout(String),
    /// To the standard error, descriptor `32'h8000_0002`.
    Stderr(String),
    /// To a file the instance opened: the descriptor and the name `$fopen`
    /// was given.
    File {
        handle: u32,
        name: String,
        text: String,
    },
    /// The call could not write: a closed or invalid descriptor, or a file
    /// the host could not write.
    Warning(String),
}

/// The result of `$fopen`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalogFileOpen {
    /// The descriptor.
    Opened(u32),
    /// Zero, the way IEEE 1364-2005 section 17.2.1 fails, with the warning
    /// the host reports if the point is accepted, if any.
    Failed(Option<String>),
}

/// One converted `$fscanf` field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnalogScanValue {
    Integer(i32),
    Real(f64),
}

/// The result of one `$fscanf` (IEEE 1364-2005 section 17.2.4.3).
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogScan {
    /// The number of fields converted and assigned, or `-1` at the end of the
    /// file before the first conversion.
    pub count: i32,
    pub values: Vec<AnalogScanValue>,
    /// A warning the host reports if the point is accepted.
    pub warning: Option<String>,
}

/// One analog instance's open files. See the module documentation for the
/// transaction it keeps.
pub trait AnalogFileTable: fmt::Debug + Send + Sync {
    /// A copy for a rollback image.
    fn clone_table(&self) -> Box<dyn AnalogFileTable>;

    /// An evaluation begins: the candidate restarts from the accepted table.
    fn begin_evaluation(&mut self);

    /// The point is accepted: the candidate becomes the accepted table.
    fn accept(&mut self);

    /// The analysis starts over: nothing an earlier analysis left pending
    /// remains, but files opened by it stay open (LRM 2.4 section 9.5.1.1).
    fn reset_analysis(&mut self);

    /// `$fopen(name)` or `$fopen(name, mode)`, on the candidate.
    fn open(&mut self, name: &str, mode: Option<&str>) -> AnalogFileOpen;

    /// `$fscanf(descriptor, format, ...)` for `targets` fields, each an
    /// integer (`true`) or a real target, on the candidate.
    fn scan(&mut self, descriptor: u32, format: &str, targets: &[bool]) -> AnalogScan;

    /// Deliver one accepted file task: `Some(text)` writes it to the
    /// descriptor's targets, `None` closes the descriptor.
    fn deliver(&mut self, descriptor: u32, text: Option<&str>) -> Vec<AnalogFileWrite>;

    /// Warnings accepted evaluations raised, in order, to report once.
    fn take_warnings(&mut self) -> Vec<String>;

    /// The accepted table, serialized, for a checkpoint; `Err` when it cannot
    /// be saved exactly.
    fn checkpoint(&self) -> Result<String, String>;

    /// Reopen a checkpoint's table as the accepted one.
    fn restore(&mut self, image: &str) -> Result<(), String>;
}

impl Clone for Box<dyn AnalogFileTable> {
    fn clone(&self) -> Self {
        self.clone_table()
    }
}

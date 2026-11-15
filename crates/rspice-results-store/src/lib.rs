//! The results store: one segment per run, appended while the run solves and read on demand.
//!
//! A segment is a 64-byte superblock followed by self-delimiting, checksummed frames: the schema,
//! the run's metadata, chunks of rows stored column by column with per-bucket min/max summaries,
//! event changes, coarser summary levels, a backward chain of index frames and a closing `END`.
//! Memory while writing is one chunk's buffers and one index's entries whatever the run's length;
//! a reader opens any prefix a crash left behind by scanning forward from the last durable index.
//!
//! The crate depends only on `std`, `lz4_flex`, `crc32fast` and `sha2`, and does all I/O through
//! [`SegmentIo`], so the same code writes a file, a buffer in memory, or a browser's origin-private
//! file system.

mod codec;
mod digest;
mod error;
mod events;
mod format;
mod io;
mod manifest;
mod reader;
mod run_writer;
mod schema;
mod summary;
mod tile;
mod writer;

#[cfg(test)]
mod tests;

pub use digest::Digest;
pub use error::StoreError;
pub use events::{EventChange, EventValue, Logic};
#[cfg(any(unix, windows))]
pub use io::FileIo;
pub use io::{MemoryIo, SegmentIo};
#[cfg(any(unix, windows))]
pub use manifest::StoreDir;
pub use manifest::{Manifest, Member};
pub use reader::{SegmentReader, SegmentState};
pub use run_writer::{RunIo, RunWriter};
pub use schema::{Axis, RunMeta, Schema, Signal, SignalKind};
pub use tile::{Tile, pixel_of};
pub use writer::{RunStatus, SegmentWriter, StopReason, StoreReceipt};

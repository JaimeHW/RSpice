//! Long-run transient controls: compression and resumable checkpoints.
//!
//! `CompressedTransientResult` carries an error-bounded reduction of the full
//! analog waveform inventory, so a multi-hour run stays addressable without
//! holding every timepoint.
//! `TransientCheckpoint` carries the netlist-fingerprinted state a resumed run
//! restarts from; the fingerprint is what stops a checkpoint being replayed
//! against a deck it was not produced from.
//!
//! The two classes live in sibling modules because they are two containers,
//! not two views of one: [`compressed`] holds the decimated result and its
//! accessors, [`checkpoint`] the resumable solver state. [`pickle`] holds the
//! versioned round-trip codec the compressed container persists through, kept
//! apart from the accessors because a state contract changes on its own
//! schedule and has to stay readable as a whole.

use super::*;

mod checkpoint;
mod compressed;
mod pickle;

use pickle::*;

pub(crate) use checkpoint::PyTransientCheckpoint;
pub(crate) use compressed::PyCompressedTransientResult;

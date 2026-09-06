# rspice-output

Transactional publication of native artifacts. Result files RSpice serializes
itself (raw files, tables, VCD, Touchstone, transient checkpoints) are published
through this crate, so a crashed or cancelled run never leaves a half-written
artifact where a complete one used to be.

The crate depends on `thiserror` and nothing else. No serialization format, no
clock, no configuration.

## How a publication works

A writer receives a buffered stream backed by a uniquely named sibling of the
destination. The destination is untouched until serialization and durability
work have both completed; publication is then a same-directory atomic replace.

The policy is deliberately not a parameter, so no caller can weaken one result's
durability:

- the complete staging file is flushed and synchronized;
- the replace itself is durable (`MOVEFILE_WRITE_THROUGH` on Windows);
- the published directory entry is synchronized where the host exposes that
  (`fsync` of the parent directory on Unix);
- a failed replace removes the staging file rather than leaving a successor.

## One file or a set

`AtomicArtifactFile` and `write_atomic` publish a single file.

`AtomicArtifactSet` publishes several files as one transaction: every member is
staged and completed first, every predecessor is snapshotted, and a commit
failure restores every predecessor byte-identically, or removes destinations
that did not exist, before the error is returned. `RollbackOutcome` reports
whether that restoration succeeded, because a rollback that itself fails is a
different operational problem from a rejected write.

## Recovery

Staging files left behind by a process crash carry `STAGING_MARKER`, the
destination file name, and the owning process id. `stale_artifacts` enumerates
them for one destination and `recover_stale_artifacts` sweeps a directory, up to
`MAX_RECOVERY_ENTRIES`. Recovery never resurrects partial content: it identifies
and removes debris, and the artifact stays absent until a run republishes it.

## Fault injection

`fault` arms the commit path to fail at a named `ArtifactFaultPoint` with a
chosen `ArtifactFaultKind`. It exists so the crash-consistency properties above
are tested rather than asserted; `tests/transaction_properties.rs` drives the
set transaction through every injection point.

## Testing

```text
cargo test -p rspice-output
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

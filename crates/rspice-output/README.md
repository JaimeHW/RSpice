# rspice-output

Transactional publication of native artifacts. A writer streams into a
uniquely named sibling of the destination, and the destination changes only
after serialization and durability work have both completed, so a failed,
cancelled or crashed run never leaves a half-written artifact where a complete
one used to be. The engine's rawfile, waveform-stream and transient-checkpoint
writers, every result file the CLI's `run` and `export` commands produce, the
Python exports and the engine adapter publish through this one crate, so none
of them can weaken the policy. It owns no serialization format; callers write
the bytes.

## What's in here

- **Crate root**: `write_atomic` and `AtomicArtifactFile` for one file;
  `stale_artifacts`; `AtomicArtifactError`, which names the failed phase and,
  for a commit, whether the destination is unchanged or already published
  with uncertain durability.
- **`set`**: `AtomicArtifactSet` and `StagedArtifact`, several files published
  as one transaction, and `RollbackOutcome`, which reports whether a failed
  commit restored every predecessor.
- **`recovery`**: `recover_stale_artifacts`, a bounded pass over one directory
  that removes staging files whose owning process is gone and reports the rest.
- **`fault`**: `ArmedFaults::arm` with an `ArtifactFault` fails the next
  publication on the arming thread at a named `ArtifactFaultPoint` with a
  chosen `ArtifactFaultKind`. It ships rather than sitting behind `cfg(test)`
  so qualification tests in the CLI, the engine and this crate drive the code
  the product runs; nothing is armed unless a caller constructs a guard.
- **`durable_file`** (`durable-file`, native targets only): the protocol for
  project, schematic and recovery files. Its compare-and-exchange publication
  checks the destination against the digest the caller accepted, holds a
  per-destination lease against other RSpice writers, and leaves a recovery
  journal that explains any interrupted publication.

## Publication contract

- The staging file is flushed and synchronized, the replace is durable
  (`MOVEFILE_WRITE_THROUGH` on Windows), and the parent directory is
  synchronized where the host exposes that (`fsync` on Unix). The policy is
  not a parameter.
- A failed replace removes its staging file. A failed set commit restores
  every predecessor byte-identically, or removes destinations that did not
  exist, before the error is returned.
- Staging files carry `STAGING_MARKER`, the destination file name and the
  owning process id. Predecessor snapshots taken by a set commit carry
  `PREDECESSOR_MARKER` and are never removed automatically, because one can be
  the last copy of a published result.
- `recover_stale_artifacts` does not recurse, stops after
  `MAX_RECOVERY_ENTRIES`, and leaves any stage whose owner is alive or whose
  liveness the host cannot answer.

## Entry points

- `write_atomic(destination, |writer| ...)` for one file written in a closure.
- `AtomicArtifactFile::prepare(destination)`, write, then `commit`, for a
  seekable or long-running write.
- `AtomicArtifactSet::new`, `stage` each member, then `commit`.
- `stale_artifacts(destination)`, whose results may belong to a live writer,
  and `recover_stale_artifacts(directory)` for crash debris.
- `durable_file::observe_expected_content`, then
  `durable_file::compare_exchange_bytes`, for a save that must not overwrite a
  concurrent change.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `durable-file` | Builds `durable_file`, adding `sha2`, `uuid`, `rustix` on Unix and the Windows security, authorization and I/O APIs. Enabled by `rspice-app` |

## Testing

```bash
cargo test -p rspice-output
cargo test -p rspice-output --features durable-file
```

`tests/transaction_properties.rs` drives the set transaction through every
`ArtifactFaultPoint`; the second run adds the `durable_file` tests.

Licensed under the [RSpice Personal Use License](../../LICENSE).

# rspice-automation-protocol

The versioned, bounded protocol the RSpice Automation worker speaks. The UI and
the worker exchange immutable source snapshots and typed events; this crate
defines both sides of that exchange and nothing else: no transport, no process
management, no interpreter.

One protocol serves both worker flavours. The native worker
([`rspice-automation-runtime`](../rspice-automation-runtime)) and the browser
worker exchange the same envelopes, so a script does not behave differently
depending on where it ran.

`PROTOCOL_VERSION` is 1.4.

## No ambient authority

No request carries a host path or an implicit permission. Access to project,
result, artifact, external file, network, process, environment, or clipboard
resources is represented by an opaque `CapabilityGrant`, and `CapabilityKind`
enumerates the complete set:

```text
ProjectRead        ProjectWrite       SimulationExecute
ResultRead         ArtifactWrite      ExternalFileRead
ExternalFileWrite  Network            ProcessSpawn
EnvironmentRead    ClipboardRead      ClipboardWrite
```

A `SourceSnapshot` is what a run is *about*: the documents, the grants, and the
digests that pin them: `closure_digest`, `environment_digest`, and
`permission_digest`. Two runs of the same digests are the same run.

## Message families

- `RuntimeRequest` in a `RequestEnvelope`: launch (`LaunchMode`), debug control
  (`DebugControl`, `Breakpoint`, `ExceptionPolicy`), and cancellation
- `RuntimeEvent` in an `EventEnvelope`: state (`RuntimeState`), diagnostics
  (`RuntimeDiagnostic` with `SourceRange`), stops (`StopReason`, `StackFrame`,
  `Variable`), and completion
- `HostCall` / `HostResponse`: the worker asking the host to exercise a
  capability it was granted

Every structured payload is `#[serde(deny_unknown_fields)]`, so an envelope from
a newer peer is rejected by name rather than silently half-understood.

## Bounds

The protocol is bounded before it is parsed, because a worker is a place
untrusted script runs: `MAX_DOCUMENTS` (10 000), `MAX_SOURCE_BYTES` (64 MiB),
`MAX_PATH_BYTES`, `MAX_TEXT_FIELD_BYTES`, `MAX_BREAKPOINTS` (100 000),
`MAX_CAPABILITIES` (4 096), `MAX_WATCH_EXPRESSION_BYTES`, and
`MAX_ENVELOPE_BYTES`.

`native_codec` frames envelopes for a pipe as a big-endian `u32` length followed
by JSON, refusing to write or read past `MAX_ENVELOPE_BYTES` so a malformed
length cannot make the host allocate on a peer's say-so. The browser worker
carries the same envelopes over `postMessage` instead.

## Testing

```text
cargo test -p rspice-automation-protocol
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

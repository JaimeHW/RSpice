# rspice-automation-protocol

The versioned, bounded protocol between the RSpice application and an
Automation worker running user Python. The native worker
([`rspice-automation-runtime`](../rspice-automation-runtime)) and the browser
worker in [`rspice-app`](../rspice-app) exchange the same envelopes, so this
crate is the one definition both sides compile against. It defines the messages
and their validation and nothing else: no process, no interpreter, no
capability broker.

## What's in here

Everything is in `src/lib.rs`:

- **Envelopes.** `RequestEnvelope` carries a `RuntimeRequest` from the host to
  the worker: probe, launch, breakpoint updates, debug control, stack,
  variable and watch queries, the answer to a host call, cancellation and
  shutdown. `EventEnvelope` carries a `RuntimeEvent` back: identity, state,
  diagnostics, output, stops, progress, published results and artifacts, host
  calls and termination.
- **Launch input.** `SourceSnapshot` is what a launch runs: its
  `SourceDocument`s, each with a `DocumentRole`, the granted capabilities, and
  the `closure_digest`, `environment_digest` and `permission_digest` that pin
  them. `ResourceLimits`, `Breakpoint` and `ExceptionPolicy` travel beside it.
- **Capabilities.** A `CapabilityGrant` pairs one `CapabilityKind` with an
  opaque broker scope and token. A `HostCall` is the worker asking the host to
  act under a grant; `HostResponse` is the answer.
- **`native_codec`.** Pipe framing for the native worker.

## Boundaries

- Verifying a signed Python runtime, and launching and containing the native
  worker process: `rspice-automation-runtime`.
- The browser worker and its transport, the broker that answers host calls,
  and the debugger UI: `rspice-app`.

## Entry points

- `PROTOCOL_VERSION` is the version every envelope carries.
- `RequestEnvelope::validate` and `EventEnvelope::validate` are the checks a
  receiver runs before acting on a decoded envelope.
- `native_codec::write_frame` and `native_codec::read_frame` frame envelopes on
  a byte stream.

## Wire contract

- **Version.** `ProtocolVersion::require_compatible` accepts a peer with the
  same major version as `PROTOCOL_VERSION` and a minor version no newer than
  it. The managed Python worker
  (`rspice-automation-runtime/worker/rspice_worker.py`) spells the same version
  as `PROTOCOL` and requires an exact match, so the two change together.
- **No ambient authority.** No request carries a host path or an implicit
  permission. Project, result, artifact, external-file, network, process,
  environment and clipboard access is an opaque `CapabilityGrant`.
  `CapabilityKind` is the complete set, and a `HostCall` must name the kind its
  `HostOperation` requires.
- **Strict decoding.** Every struct payload denies unknown fields, so content
  from a newer peer is refused rather than half-read.
- **Bounds.** `MAX_DOCUMENTS`, `MAX_SOURCE_BYTES`, `MAX_PATH_BYTES`,
  `MAX_TEXT_FIELD_BYTES`, `MAX_BREAKPOINTS`, `MAX_CAPABILITIES`,
  `MAX_WATCH_EXPRESSION_BYTES` and `MAX_ENVELOPE_BYTES` bound the collections
  and text fields, and `validate` enforces them.
- **Native framing.** A big-endian `u32` byte length, then that many bytes of
  UTF-8 JSON. A zero length, or one above `MAX_ENVELOPE_BYTES`, is refused
  before the payload buffer is allocated. The browser worker passes the same
  serde representation through `postMessage`.

## Testing

```text
cargo test -p rspice-automation-protocol
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

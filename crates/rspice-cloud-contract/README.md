# rspice-cloud-contract

The RSpice Cloud HTTP contract as Rust types. The API serializes these types and
every Rust client deserializes these same types, so the wire format has one
definition rather than two that drift.

It sits directly on `rspice-cloud-domain` and adds no machinery: no HTTP client,
database, async runtime, platform binding, credential handling, or secret
storage. `#![forbid(unsafe_code)]`.

## Surface

`API_VERSION` is `v1` and `SERVICE_NAME` is `rspice-cloud-api`. Errors are
problem details (`ProblemDetails`: `type`, `title`, `status`, `detail`,
`instance`); list endpoints return a cursor-paginated `Page<T>`. Beyond that the
types group into the resource families the service exposes:

- **Identity and entitlement**: `CurrentPrincipal`, `Entitlement`,
  `LicenseJwkSet`, license lease issuance
- **Workspaces**: membership, roles, invitations, `AuditEvent`
- **Circuits and revisions**: creation, update, immutable revision records
- **Collaboration**: Automerge document sessions and live sessions with their
  policy, participants, and join codes
- **Artifacts**: checksum-bound direct-to-object-storage upload and download
  sessions
- **Simulation runs**: queueing one immutable revision and reading its status
- **Publications**: sealed public pages and their published simulations
- **Shares**: capability-token links to a circuit revision
- **Model packs**: release upload and signed catalog download

## Protocol constants worth knowing

WebSocket subprotocols are named here so client and relay cannot disagree:
`COLLABORATION_PROTOCOL` (`rspice.automerge.v1`) and `LIVE_SESSION_PROTOCOL`
(`rspice.live-session.v2`). Every live-session relay frame is prefixed with a
`LiveSessionFrameClass` byte; the relay enforces participant capability by that
class alone and never interprets payload bytes, which is what keeps a viewer
from sending document edits without the relay parsing Automerge.

Capability tokens are exactly `CAPABILITY_TOKEN_BYTES` (32) of entropy encoded
as `CAPABILITY_TOKEN_LENGTH` (43) base64url characters;
`is_canonical_capability_token` rejects any other spelling of the same value.

License verification keys are RSA JWKs bounded by
`MIN_LICENSE_RSA_MODULUS_BITS` and `MAX_LICENSE_RSA_MODULUS_BITS`, with the
exponent pinned to `LICENSE_RSA_PUBLIC_EXPONENT`. The `is_valid_license_*`
predicates are the shared judgment, so client and service accept the same key
sets.

## Testing

```text
cargo test -p rspice-cloud-contract
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

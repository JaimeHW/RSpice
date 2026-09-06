# rspice-cloud-client

The HTTP client for RSpice Cloud, for native Rust applications and browser
WebAssembly alike. It sits directly on
[`rspice-cloud-contract`](../rspice-cloud-contract) and re-exports it as
`contract`, so the types on the wire have one definition.

`#![forbid(unsafe_code)]`, `#![deny(missing_docs)]`.

## What this client refuses to do

The constraints are the design. This crate does **not**:

- **own credentials**: `BearerToken<'a>` validates and *borrows* the token for
  one request without copying or retaining it; construct it immediately before
  the call from a platform credential store;
- **retry mutations**: every mutating method makes exactly one network attempt
  and returns. Retry scheduling and durable command state belong to the caller,
  who owns the `IdempotencyKey`;
- **accept redirects**: a redirected request is a failure, not a hop;
- **buffer unbounded bodies**: `ClientConfig::max_response_bytes` caps every
  response, and artifact transfers stream in chunks of at most
  `MAX_ARTIFACT_SINK_CHUNK_BYTES`.

`ClientConfig` is validated at construction, not at request time.
`EndpointMode::Production` requires HTTPS everywhere and *distinct* origins for
the application and for customer-controlled object bytes;
`SharedProductionOrigin` is a rejected configuration, not a warning. Embedded
user information in an endpoint URL is rejected outright.
`EndpointMode::LoopbackDevelopment` relaxes the scheme only on loopback.

## Idempotency

Mutations that can be replayed take a validated `IdempotencyKey`: generate a
high-entropy value once per logical mutation, persist it with the pending
command, and reuse it only when retrying *that* command. A successful response
must carry authenticated replay metadata from an idempotency-capable API, and
responses are bound back to the request: a returned workspace member must match
the exact workspace, principal, and role that were asked for.

## Coverage

One module per resource family: `identity`, `governance`, `workspaces`,
`invitations`, `circuits`, `collaboration`, `live_sessions`, `artifacts`,
`simulations`, `publications`, `shares`, `model_hub`, `licensing`, plus
`pagination` and `transfer`.

Newtypes stop malformed values before a request is made rather than after:
`PackId`, `PackVersion`, `PublicationSlug`, `ShareToken`, `InvitationToken`,
`PageRequest`.

## Native and browser

Transport is target-split. Native builds use `reqwest` with `tokio`, and add
`native_licensing`, offline PS256 verification of license leases with pinned
issuer and audience. WebAssembly builds use `wasm_transport` and `wasm_transfer`
over `web-sys` and `wasm-streams`. Everything above the transport is shared, so
the two targets cannot disagree about the protocol.

## Testing

```text
cargo test -p rspice-cloud-client
```

The browser transport, transfer, clock, collaboration, governance, and share
modules also carry `wasm_bindgen_test` cases, run under a wasm test runner
against the `wasm32-unknown-unknown` target.

Licensed under the [RSpice Personal Use License](../../LICENSE).

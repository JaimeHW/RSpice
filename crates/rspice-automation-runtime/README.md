# rspice-automation-runtime

Verification and launch support for RSpice-owned native Python runtimes: the
native half of the transport that
[`rspice-automation-protocol`](../rspice-automation-protocol) defines.

The crate deliberately has **no system-interpreter discovery**. A caller
supplies an application-local runtime directory and a release trust store;
every executable input is signed, enumerated, and content verified before a
Python worker can be launched. `NativeWorker` starts only the verified
application-local executable: no `PATH` lookup, shell, user site, startup hook,
or ambient working directory is involved.

## Verifying a runtime

A managed runtime directory carries `runtime-manifest.json` and
`runtime-manifest.ed25519.json`. Verification walks:

1. the signature document, against an Ed25519 key in the caller's
   `RuntimeTrustStore` (an unlisted key id is `UntrustedSigningKey`, not a
   warning);
2. the manifest schema, protocol major/minor against `PROTOCOL_VERSION`, target
   triple, Python version, and ABI against the requirement;
3. `runtime_digest_sha256` recomputed by `runtime_inventory_digest` over the
   enumerated file set, the same implementation release assembly uses, so the
   packager and the installed verifier cannot drift on framing, ordering, or
   executable-bit semantics;
4. every `RuntimeFile` digest on disk.

The result is a `VerifiedRuntime`. `RuntimeError` names which of those steps
failed and with what value, so a broken install is diagnosable without a debug
build. Python environments inside a runtime are content-addressed: a project
lock names one of `environment_digests_sha256` exactly.

## Launching and containing a worker

`NativeWorker` owns the child process, frames envelopes through
`rspice_automation_protocol::native_codec`, and applies containment *before* any
untrusted source is sent:

- OS-enforced CPU, memory, process-count, and kill-on-close limits (a Windows
  job object, or the equivalent rlimits inside the POSIX trusted bootstrap);
- a process-owning watchdog deadline per launch request, which unlike UI-frame
  polling stays effective if user Python loops forever or the UI stops
  repainting;
- bounded stderr capture, so a chatty failure cannot exhaust host memory.

## Testing

```text
cargo test -p rspice-automation-runtime
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

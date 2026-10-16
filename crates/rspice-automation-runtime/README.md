# rspice-automation-runtime

Verification and launch of the RSpice-owned native Python runtime that runs
Automation scripts: the native side of
[`rspice-automation-protocol`](../rspice-automation-protocol). There is no
system-interpreter discovery. A caller supplies an application-local runtime
directory and a release trust store; every file in the runtime is signed,
enumerated and content-verified before a worker can start, and the launch uses
only that verified executable, with no `PATH` lookup, shell, user site,
startup hook or ambient working directory.

## What's in here

- `src/lib.rs`: `RuntimeTrustStore`, `VerifiedRuntime`, `NativeWorker`, the
  signed manifest types, and `runtime_inventory_digest`.
- `worker/rspice_worker.py`: the trusted protocol worker the runtime runs.
  Release assembly copies it into the runtime as `worker/rspice_worker.py`.
- `src/bin/rspice-managed-runtime-packager.rs`: inventories and signs a
  staged runtime, then verifies the result with the code the application uses.
- `src/bin/rspice-managed-runtime-qualifier.rs`: release qualification of one
  signed runtime: verification, a validate and a run launch, hard
  cancellation and watchdog termination, reported as one JSON evidence line.

## Boundaries

- The messages the worker speaks: `rspice-automation-protocol`.
- Locating the installed runtime (`runtimes/python` beside the executable), the
  compiled-in trust key, and the browser (Pyodide) worker: `rspice-app`.
- Staging a CPython build into a runtime directory:
  `tools/release/stage_managed_python.py`.

## Entry points

```rust
let mut trust = RuntimeTrustStore::new();
trust.add_key_bytes(key_id, public_key)?;
let runtime = trust.verify(runtime_root, &requirement)?; // VerifiedRuntime
let mut worker = runtime.launch(scratch_dir)?;          // NativeWorker
worker.apply_resource_limits(limits)?;                  // before any source
worker.arm_wall_time_limit(limits.wall_time_ms)?;
worker.send(&request)?;
let event = worker.try_event()?;
```

`VerifiedRuntime::supports_environment` says whether a project lock's
environment digest is one the runtime installs. `RuntimeError` names the check
that failed and the value it found, so a broken install is diagnosable from a
release build.

## What a signature covers

A runtime directory carries `runtime-manifest.json` (`MANIFEST_FILE`, schema
`rspice.managed-python-runtime/v2`) and `runtime-manifest.ed25519.json`
(`SIGNATURE_FILE`: a `key_id` and the Ed25519 signature over the manifest's
exact bytes). `RuntimeTrustStore::verify` refuses the runtime unless:

1. the signing key id is in the trust store and the signature verifies;
2. the manifest's protocol version is compatible with `PROTOCOL_VERSION`, its
   target triple and architecture match the `RuntimeRequirement`, its Python
   and RSpice API versions satisfy it, and its Python is 3.14 with the `cp314`
   ABI;
3. every listed file is a regular file inside the root with the declared size
   and SHA-256;
4. `runtime_inventory_digest` over the manifest equals its
   `runtime_digest_sha256`;
5. the directory holds no file the manifest does not list, and no symlink.

The packager computes the digest with the same `runtime_inventory_digest`, so
release assembly and the installed verifier cannot disagree on framing,
ordering or executable-bit semantics. A project lock selects one of the
manifest's `environment_digests_sha256` exactly.

## Containment

`NativeWorker` owns the child process and frames envelopes through
`rspice_automation_protocol::native_codec`. Limits are installed before any
untrusted source is sent. On Windows, `apply_resource_limits` puts the process
in a job object with CPU, memory, process-count and kill-on-close limits; on
POSIX the worker applies the equivalent rlimits in its trusted bootstrap. The
limits are process-lifetime state, so a second call is refused and each launch
needs a fresh process. `arm_wall_time_limit` starts a watchdog thread that
kills the process at the deadline even if the script loops forever or the UI
stops repainting. Captured stderr is bounded.

## Release packaging

`.github/workflows/native-release.yml` is the reference procedure: it stages a
uv-managed CPython with `tools/release/stage_managed_python.py`, signs it with
`rspice-managed-runtime-packager`, and runs `rspice-managed-runtime-qualifier`
against the signed result before the application is built. The packager reads
the Ed25519 seed, 64 hexadecimal characters, only from the file named by
`--signing-key-file`, never from the command line or the environment, and
refuses a staging directory that already holds manifest outputs. It prints the
`signing_key_id` and `public_key_hex` that the `rspice-app` build compiles in
through `RSPICE_AUTOMATION_RUNTIME_KEY_ID` and
`RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX`. A build without them reports native
Automation as unavailable; it never falls back to a system Python.

## Testing

```text
cargo test -p rspice-automation-runtime
python3 tools/ci/test_automation_runtime_assets.py
```

The second command checks the worker script and the release workflow's
qualification wiring alongside the browser runtime.

Licensed under the [RSpice Personal Use License](../../LICENSE).

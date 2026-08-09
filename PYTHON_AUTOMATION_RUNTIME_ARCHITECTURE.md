# RSpice Python automation runtime architecture

Status: accepted implementation direction for the Netlist & Script Editor workspace.

## Decision

RSpice owns the default Python runtime.

- Native desktop packages ship an application-local, pinned CPython interpreter, standard library,
  RSpice automation API, and approved binary dependencies.
- Browser packages ship a pinned Python/WASM distribution and the browser-compatible RSpice
  automation API as local versioned assets. Python runs in a Web Worker, never on the UI thread.
- A customer does not need to install Python, modify `PATH`, create a virtual environment, or fetch
  an interpreter to use supported automation workflows.
- An arbitrary system interpreter is never selected automatically for execution.
- System-interpreter discovery is an optional, explicitly selected developer integration. It is
  disabled by default and does not produce qualification/signoff-capable results unless an
  organization policy deliberately authorizes that exact interpreter and environment.

This policy is independent of demonstration filenames and values in the mockup. A project can use
arbitrary source paths and helper modules. The project records semantic roles and a versioned,
content-addressed environment description; it does not require files to be named
`characterize.py`, `runplan.rspice.yaml`, `requirements.lock`, or `permissions.toml`.

## Why system-Python detection is not the primary design

Selecting a user's `python`, `python3`, or Windows launcher result would make execution depend on
uncontrolled patch levels, architecture, site packages, environment variables, startup hooks,
DLL/shared-library search paths, encodings, and local policy. It also has no browser equivalent.
That would make two runs of the same project capable of importing different code or producing
different results.

The official CPython distribution provides an embeddable Windows package, and CPython's isolated
configuration is specifically intended for embedding. RSpice must set the interpreter home and
module search path explicitly and ignore user site directories, Python environment variables, and
startup configuration. PyO3 can expose the Rust engine to a native CPython runtime, but attaching
an interpreter directly to the EGUI process is not sufficient isolation. Python runs in a worker
process so cancellation, resource enforcement, crashes, and native-extension failures do not take
down the UI or corrupt its live project state.

For the browser, Python/WASM runs in a module Web Worker. The runtime and supported wheels are
served from the RSpice application package, not an unpinned CDN. Python packages containing native
extensions must be pure Python or have an approved WASM build; an ordinary desktop wheel is not
browser-compatible.

## Runtime topology

```text
EGUI workspace
  -> AutomationRuntimeClient (platform-neutral Rust API)
      -> versioned request/event protocol
          -> native: sandboxed rspice-python-worker process + managed CPython
          -> browser: dedicated Web Worker + managed Python/WASM
              -> capability broker
                  -> immutable project snapshot / simulator / results / artifact store
```

The UI owns no interpreter objects. It submits immutable requests and applies versioned events to a
session identified by project, document, source-closure digest, revision, environment digest,
permission digest, API version, platform, and runtime build identity.

## Shared protocol

The native and browser workers implement the same versioned protocol. Requests include:

- probe runtime and capabilities
- materialize/verify environment
- parse and validate a source closure
- start, dry-run, restart, pause, continue, step in/over/out, and stop
- add/update/remove enabled, conditional, log, and hit-count breakpoints
- set exception policy
- enumerate threads, stack frames, scopes, variables, children, and watches
- evaluate a watch expression in a selected paused frame
- acknowledge output and artifact chunks
- cancel a request and terminate a session

Events include:

- state transitions with monotonically increasing sequence numbers
- canonical diagnostics and tracebacks with exact document/range/revision binding
- stdout/stderr and typed structured-output records
- stopped/continued/terminated/worker-failed notifications
- stack/scope/value updates
- progress, resource usage, permission requests/denials, simulator run events, result handles, and
  artifact handles

Messages use bounded, length-prefixed binary envelopes on native and structured-clone-compatible
envelopes in the browser. All untrusted sizes, counts, strings, paths, and nesting depths are
validated before allocation. Unknown required protocol fields or versions fail closed.

## Managed native runtime

Each native release contains a signed, content-addressed runtime payload for every supported host
architecture. It includes:

- the pinned CPython patch release and standard library
- the RSpice Python package and generated type information for the versioned automation API
- the native RSpice extension/bridge built for the shipped ABI
- approved baseline packages and their licenses/notices
- a runtime manifest containing hashes, target triple, ABI, build identity, and compatibility range

The interpreter starts in isolated mode with an explicit home, search path, UTF-8 policy, and no
user site directory. The worker receives a virtual project snapshot and opaque broker handles; it
does not inherit the full parent environment, arbitrary handles, current directory, or unrestricted
filesystem/network/process access.

The worker is constrained with platform facilities appropriate to the OS, including a restricted
token/job object and resource limits on Windows, sandbox profiles and process limits on macOS, and
namespaces/seccomp/rlimits where available on Linux. Platform implementations must meet the same
capability and failure semantics even when their mechanisms differ.

## Managed browser runtime

The browser worker loads a pinned Python/WASM runtime, standard library, RSpice API bridge, and
approved wheels from versioned application assets with integrity verification. It receives a
virtual read-only source snapshot and a bounded scratch filesystem. Durable project and artifact
writes go through the host broker and are committed transactionally.

The browser worker has no DOM access. Network access is unavailable unless the project manifest,
user decision, deployment policy, and browser policy all permit a brokered request. Long-running
or unresponsive execution is cancelled by terminating and recreating the worker; project state is
not mutated until the host validates and commits a complete transaction.

Browser compatibility is explicit in the environment resolver. Pure-Python wheels and approved
WASM wheels can be supported. A dependency that only supplies a native desktop extension yields a
diagnostic naming the package, required platform, available artifacts, and remediation choices.

## Environment model

The environment description is a generalized, versioned schema. Its path is project-configured.
It records at least:

- schema version and environment identity
- required Python language/ABI range
- required RSpice automation API range
- package names, exact versions, artifact hashes, source/index identity, markers, and transitive
  dependency graph
- target-platform availability and approved runtime build identities
- resolver identity and lock generation metadata

All executable inputs are part of the source-closure or environment digest. Editing any bound file,
dependency, permission, run plan, or project binding invalidates the validation receipt and prevents
execution until revalidated.

Environment materialization uses a content-addressed cache. It is staged, hash-verified, quota
checked, and committed atomically. Offline use succeeds when every required artifact is already in
the signed product payload or verified cache; otherwise it fails with an exact missing-artifact
diagnostic. It never mutates a system Python installation.

## Permission and security model

Python itself is not treated as a security sandbox. The worker can only affect host resources
through a capability broker. The project permission manifest is a semantic role, not a reserved
filename, and is bound to the execution receipt.

Capabilities are deny-by-default and independently scoped for:

- project reads and writes
- external file reads and writes
- result reads and artifact creation
- network destinations and methods
- process creation
- environment-variable reads
- clipboard access
- clocks/randomness and reproducibility policy

Permission prompts identify the requesting source, requested operation, exact resource scope, and
whether the decision applies once, for the project, or by organization policy. A denial becomes a
typed Python exception and a canonical diagnostic/event; it is never converted to silent success.

## Parsing, language services, execution, and debugging

Live editing uses an error-tolerant native/WASM Python parser for immediate syntax structure and
incremental document services. Authoritative validation is performed by the exact managed runtime
that will execute the code. A lightweight delimiter scanner or matching a list of demonstration
statements is not valid Python validation.

The RSpice automation API is a real versioned package with explicit objects for projects, source
snapshots, simulation plans/runs, results, comparisons, measurements, artifacts, cancellation, and
structured logging. The Python layer calls stable Rust application services through broker handles;
it does not reimplement the simulator or bypass project validation.

Debugging is implemented inside the worker with a versioned debugger adapter. Breakpoint locations
are resolved against the runtime's compiled code and mapped back to exact source revisions. Paused
frames expose bounded, lazily expanded values; evaluating watches observes the declared policy and
cannot silently acquire new capabilities. A worker crash or forced termination yields a recoverable
session failure and never fabricates a completed run.

## Optional external-interpreter discovery

External discovery may be added for advanced development after its UI is approved. It must be an
explicit setting, not automatic execution behavior.

Discovery probes known platform registration mechanisms and user-selected paths without executing
shell command strings. Every candidate is launched in an isolated probe mode with a timeout and a
minimal environment and must return a bounded machine-readable record containing executable and
base-prefix identity, implementation, exact version, ABI, architecture, platform, environment
paths, and RSpice API availability. RSpice displays the resolved executable and compatibility
result before the user can select it.

Selecting an external interpreter creates or verifies an RSpice-owned project environment; RSpice
does not install packages into the user's global interpreter. The executable and resolved
environment are fingerprinted. Any change invalidates prior receipts. External runtimes remain
visibly non-managed and non-qualifying by default, and they have no browser parity guarantee.

## Packaging and update requirements

- Runtime assets are part of release assembly, signing, SBOM, license notices, integrity checks,
  differential update, rollback, and offline-install qualification.
- Runtime updates are atomic and side-by-side. Existing projects continue to resolve their pinned
  compatible runtime until deliberately migrated.
- The application verifies runtime hashes before launch and refuses a corrupted or mismatched
  payload with a repair action.
- Release qualification covers native x64/ARM64 targets, browser targets, worker crash/timeout,
  cancellation, capability denial, corrupted caches, missing assets, incompatible packages, and
  deterministic replay of accepted reference workflows.

The repository-owned `rspice-managed-runtime-packager` binary inventories a fresh staged runtime,
calculates the same framed content digest used by the installed verifier, signs the exact manifest
with a file-supplied Ed25519 release seed, writes no private material into the package, and
self-verifies the completed closed-world directory. Its staging contract and invocation are in
`assets/automation_runtime/README.md`.

## Immediate implementation consequences

1. Remove demonstration-statement validators and simulated debuggers from authorization paths.
2. Introduce the platform-neutral runtime protocol and immutable request/event domain types.
3. Refactor the existing PyO3 extension behind application services suitable for the native worker.
4. Build and package the isolated native worker and its managed runtime.
5. Build the browser Web Worker backend and locally package a compatible Python/WASM distribution.
6. Implement the versioned RSpice automation API and generate matching stubs/documentation.
7. Replace filename heuristics with persisted semantic role bindings and full source-closure rules.
8. Add capability brokerage, environment locking/materialization, debugger services, and failure
   recovery before enabling Automation Run as a production action.

Until these conditions are met, the UI must report automation execution as unavailable with an
actionable reason; it must never substitute a simulated run or debugger and call it successful.

## Qualified runtime line

- Native releases package an exact signed CPython 3.14 patch build. The current release input is
  CPython 3.14.6; changing it requires rebuilding, signing, and requalifying the native payload.
- Browser releases package exact self-hosted Pyodide assets from the 314 line. The current browser
  runtime is Pyodide 314.0.2, which reports CPython 3.14.2. Browser and native patch identities are
  therefore allowed to differ while both satisfy the locked Python 3.14 ABI range.
- A project lock names a compatible Python range and exact approved runtime identities. It must not
  infer compatibility from a demonstration filename or from whatever interpreter happens to be on
  the customer's system.

## Primary technical references

- CPython 3.14 initialization and isolated embedding configuration:
  <https://docs.python.org/3.14/c-api/init_config.html>
- CPython's application-local Windows embeddable package:
  <https://docs.python.org/3.14/using/windows.html#the-embeddable-package>
- PyO3 calling Python from Rust and embedding feature behavior:
  <https://pyo3.rs/main/python-from-rust> and <https://pyo3.rs/main/features>
- Pyodide Web Worker execution guidance:
  <https://pyodide.org/en/stable/usage/webworker.html>
- Pyodide/micropip package compatibility, including the requirement for WASM builds of native
  extension packages: <https://micropip.pyodide.org/en/stable/project/usage.html>

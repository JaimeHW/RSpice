# rspice-engine-adapter

The self-contained engine executor the credentialless RSpice Cloud worker
launches. It reads exactly one protocol-4 JSON request on standard input,
solves it with `rspice-core`, writes the declared result files under
`results/`, and emits exactly one JSON response on standard output. It opens no
network connection and launches no subprocess, and a deck's includes resolve
only against the request's manifested artifacts and the model-library bundle
a deployment may bind. The binary is the deliverable; the library exists so
tests and release tooling build byte-exact requests through the code the
executor validates with.

## What's in here

| Module | Contents |
| --- | --- |
| `wire` | The protocol-4 request and response envelopes, the request and revision digests, and the byte bounds |
| `document` | `rspice-circuit-v1` interpretation and `.include` binding against verified sources |
| `execute` | Request-kind resolution, canonical deck planning, the run-axis loop, the solve budget, and atomic staging of the result files |
| `family` | The wire name of each analysis family, the refused names, and the projection of one card into its result documents |
| `result_artifact` | One shared result document encoded as an artifact, and the manifest measurements projected from it |
| `measure` | Deterministic measurement formatting and the wire grammar of measurement names |
| `axis_execution_document` | The `rspice-axis-execution` STEP/TEMP orchestration record |
| `src/main.rs` | The executor binary: launch-environment checks, termination handling, and the one-request lifecycle |

## Boundaries

- Analysis runners and the result document schema: `rspice-core`. The adapter
  defines no result schema of its own beyond the run-axis record.
- Atomic publication of the result file set: `rspice-output`.
- The controller side of the protocol: the cloud worker's
  `rspice-cloud-engine-contract` crate. `wire` reimplements it rather than
  depending on it across repositories, and pinned digest vectors in the
  `wire` tests hold the two byte-for-byte equal.

## Entry points

- `rspice-engine-adapter` with no arguments serves one request;
  `rspice-engine-adapter component-info` prints the engine build, the protocol
  versions and the schemas it serves as one JSON line.
- `wire::parse_engine_request` is the validation the executor applies to its
  input, and `wire::simulation_request_digest` and
  `wire::revision_content_digest` compute the digests a request carries.
- `document::interpret_document` followed by `execute::execute_with_abort`
  runs a request in process.

## Wire contract

| Element | Value |
| --- | --- |
| Protocol version | `4` (`RSPICE_ENGINE_PROTOCOL_VERSION=4`) |
| Request document schema | `rspice-circuit-v1` |
| Request digest version | `1` |
| Revision content digest version | `2` |
| Result manifest format | `rspice-result-v3` |
| Typed result document | `rspice-analysis-result` v18 (`rspice-core`) |
| Run-axis orchestration record | `rspice-axis-execution` v1 |

A request whose `protocol_version` is anything other than `4` is refused as
controller drift before any deck is parsed, and a launch environment that
declares another protocol is refused before a request is read. Neither an older
nor a newer protocol is served a response its reader cannot interpret.

## Launch contract

The worker starts the executor with a cleared environment that sets
`RSPICE_ENGINE_PROTOCOL_VERSION=4`, `RSPICE_ENGINE_INPUT=stdin-json` and
`RSPICE_ENGINE_OUTPUT=stdout-json`, and does not set
`RSPICE_ENGINE_SOLVER_PATH`. `RSPICE_ENGINE_MODEL_LIBRARY_PATH` optionally names
the model-library bundle a deck includes as `rspice-model-library`.
`RSPICE_ENGINE_SOLVE_BUDGET_SECONDS` optionally replaces
`execute::DEFAULT_SOLVE_BUDGET`; a malformed or non-positive value is a
launch-contract violation, not a fall back to the default.

Exit status `0` means a response was written, including a `status: failed`
response for a circuit the engine rejects. Any other status is a controller or
sandbox fault, never a customer result:

| Status | Cause |
| --- | --- |
| `10` | The launch environment or arguments violate the launch contract |
| `11` | Standard input could not be read |
| `12` | The request violates the wire contract |
| `13` | A manifested artifact or the model bundle failed re-verification, or the result files could not be written |
| `14` | The response could not be serialized or written |

## Analysis kinds

The request's `analysis.kind` selects one family by a wire name listed in
`family::REQUEST_KINDS`, and every authored card of that family runs at every
coordinate of the deck's `.STEP` and `.TEMP` axes. Directive parameters come
from the deck, which is parsed in the ngspice dialect, so the request digest
covers the whole simulation definition. `family::REFUSED_REQUEST_KINDS` lists
the names this build recognizes and refuses, each with the reason the response
carries. A mixed-signal deck is requested as `transient`, and `fourier`, `fft`
and `port_noise` results are published by requesting the `transient` or
`s_parameters` family whose card produces them. A deck that authors a card the
adapter cannot run without loss, such as a quasiperiodic card or a small-signal
card linearized around a transient state (`ACTIMES=`, `CHECKPOINT=`), is
refused whole before anything solves.
`rspice_core::execution::ANALYSIS_CAPABILITY_MATRIX` declares, per result
family, what the engine-adapter surface maps.

## Result files

Each analysis publishes at least one `rspice-analysis-result` document, at
`results/<analysis>.result.json`, or at
`results/<coordinate>__<analysis>.result.json` when the deck has a run axis.
`<analysis>` and `<coordinate>` are canonical namespace components such as
`tran-001` and `run-<id>`, and the declared content type derives from the
document's schema and version. A
card that produces more than one document publishes the rest beside its own,
named by its stem plus their component: `tran-001.fft-001.result.json` and
`tran-001.four-001.result.json` for `.FFT` and `.FOUR`, and
`sp-001.port-noise.result.json` for `.SP DONOISE`. A deck with a run axis also
carries an `rspice-axis-execution` record in the result manifest, naming each
coordinate, its axis assignments, and the documents each analysis published.

Result size is checked against the artifact limit before any solve, and each
document's retained value count against the remaining byte budget before it is
serialized, so an oversized set fails with a `resource.*` code rather than an
allocation failure. Every file is staged in memory and committed as one
transaction, so a failed run publishes none of them. A termination request
(SIGINT, SIGTERM, or a Windows console-control event) ends the request with
`engine.cancelled`, and an exhausted solve budget with `engine.time_limit`.

## Testing

```text
cargo test -p rspice-engine-adapter
```

`tests/readme_contract.rs` holds the wire-contract table above to the constants
it names.

Licensed under the [RSpice Personal Use License](../../LICENSE).

# RSpice WASM

WebAssembly bindings for the RSpice simulation engine. The crate is
deliberately thin, and it owns **no result schema of its own**: every analysis
produces the shared `rspice_core::execution::AnalysisResultDocument`, and this
crate only decides how that document crosses the JavaScript boundary. All
numerical work happens in `rspice-core`.

This is what powers the "run it in your browser" demo on the project site.
The client-owned [`web/`](web/) shell is overlaid onto the standalone
`RSpice-Site` static source during the verified deployment build and loads the
`pkg/rspice_wasm.js` module built from this crate.

## Public API (the contract JavaScript sees)

Every analysis export returns a `WasmResultHandle`. Errors are thrown as an
`RSpiceError` with stable structured fields.

| JS function | Arguments | Returns |
| :--- | :--- | :--- |
| `defaultResourceLimits()` | none | camelCase browser resource policy object |
| `healthCheck([options])` | optional execution options | parser-to-solver readiness report with timing and probe counts |
| `summarizeNetlist(source[, options])` | netlist text and optional execution options | `{title, element_count, analysis_count, model_count, subcircuit_count, parameter_count, diagnostics, startup_diagnostics}` |
| `runOperatingPointDocument(source[, options])` | netlist text | result handle holding one `op` document |
| `runDcSweepDocument(source, sourceName, start, stop, step[, options])` | scalar linear source sweep | result handle holding one `dc` document |
| `runAcAnalysisDocument(source, frequencies[, options])` | explicit frequency grid | result handle holding one `ac` document |
| `runTransientAnalysisDocument(source, tstop, maxStep[, options])` | explicit transient interval | result handle holding one `tran` document |
| `runNoiseAnalysisDocument(source, outputNode, referenceNode, inputSource, frequencies[, options])` | named output/input and an explicit positive frequency grid | result handle holding one `noise` document |
| `runAuthoredDeckDocument(source[, options])` | a complete authored deck with optional DATA/STEP/TEMP axes | result handle holding every coordinate-local result the deck produced |

### The result handle

`WasmResultHandle` retains its results in WebAssembly memory. Only descriptors
and caller-bounded numeric windows cross into JavaScript.

- `resultCount()` / `coordinateCount()`.
- `metadata()` — the executed plan: `schema` (`rspice-browser-result`),
  `schemaVersion`, ordered `axes` run-axis descriptors, `plannedAnalyses`
  (canonical identities such as `ac-001`/`ac-002`), the canonical
  `coordinates`, a compact `results` summary array, `maximumWindowValues`, and
  `maximumResultJsonBytes`.
- `resultMetadata(resultIndex)` — one result's identity, provenance, and
  descriptors: `resultKind`, `analysis`, `parentAnalysis`, `coordinateId`, the
  full `coordinate`, `topologyFingerprint`, `namespaces`, `pointCount`, `axes`,
  `signals`, `scalars`, `deviceStates`, a `payload` descriptor carrying the
  family tag plus the compression certificate and the FFT children, and the
  `valuesPerPoint` / `totalValueCount` / `maximumWindowValues` budget figures.
- `readWindow(resultIndex, start, count)` — a half-open aligned slice. Axis and
  real/complex sample columns are `Float64Array`; every signal carries a
  `Uint8Array` `validity` mask. **A zero validity entry is an explicitly
  unavailable sample, so the aligned numeric placeholder must not be
  interpreted.** Empty, out-of-range, and over-budget windows fail with
  `code: "invalid_result_window"`; an unknown result index fails with
  `code: "invalid_result_index"`.
- `resultJson(resultIndex)` — the complete core document as JSON. This is the
  lossless export path: it is bounded by an explicit byte budget and fails
  closed rather than truncating.

The default transfer ceiling is 262,144 numeric/validity values and is further
reduced by `maxResultValues`. A window charges for its numeric columns plus one
validity byte per signal per point, because a transfer without the mask cannot
tell a placeholder from a measurement.

### Result documents

Documents are `rspice-analysis-result` version 1, defined and validated by
`rspice-core`. Their JSON encoding, missingness rules, per-family payloads, and
identity fields are documented on
`rspice_core::execution::result_document`; this crate does not restate or
reinterpret them.

A direct request is planned as a one-analysis `DeckPlan`, so its result carries
the same canonical `AnalysisInstanceId` and the same single run coordinate an
authored deck would give it. Nothing here mints an identity of its own.

### Analysis coverage

`runAuthoredDeckDocument` executes every planned analysis in a deck over its
canonical DATA/`.STEP`/`.TEMP` coordinate product:

| Executed | `.OP`, `.DC` (single source), `.AC` (including `DATA=`), `.TRAN` (including attached `.FFT` spectra and optional compression), `.NOISE` (including `DATA=`), `.STB`, `.TF`, `.DISTO`, `.MC`, `.PSS`, `.PAC`, `.HB`, `.ENVELOPE` |
| :--- | :--- |
| Refused by name | `.SP` and its port noise, `.PNOISE`, `.SENS`, `.PZ`, `.FOUR`, and a nested two-source `.DC` |

Every refusal is raised before any solver work, names the authored card and its
canonical instance identity, and quotes the exact `rspice-core` entry point
that is missing. The browser API never turns an unsupported family into an
operating point, an empty table, or a nominal-temperature run. Textual `.ALTER`
is refused by the core deck materializer, which owns that contract.

The declared coverage of every family on this surface lives in
`rspice_core::execution::capability`, and the browser test suite uses that
registry as its input: a cell that says `Mapped` must publish a document of
that family, and a cell that says `Unsupported` must refuse by name.

### Renames from the previous browser schemas

This build replaces the three browser-owned result schemas
(`rspice-analog-result`, `rspice-deck-result`, `rspice-stb-result`) with the
shared core document. Consumers of the old handles should note:

| Was | Now |
| :--- | :--- |
| `WasmAnalogResultHandle`, `WasmDeckResultHandle`, `WasmStbResultHandle` | one `WasmResultHandle` |
| `handle.readWindow(start, count)` | `handle.readWindow(resultIndex, start, count)` |
| `handle.metadata()` returning one result's descriptors | `handle.resultMetadata(resultIndex)`; `metadata()` now describes the plan |
| coordinate `index` / `namespace` / assignment `target` | coordinate `ordinal` / `label` / assignment `stepTarget` |
| coordinate `id` as a string | `coordinate.id` as `{semantic, occurrence}`; the flat string stays available as `coordinateId` |
| `analysis: {id, kind, request_kind, ordinal}` | `analysis: {id, kind, ordinal}`, with the core's own kind tags |
| scalar calls taking a one-based `ordinal` argument | removed; a one-analysis request is `kind-001` by construction |
| `runStbAnalysisDocument(...)` | author `.STB` in the deck and call `runAuthoredDeckDocument` |
| `runDcOperatingPoint`, `runAcAnalysis`, `runTransientAnalysis`, `runTransientAnalysisCompressed` | removed; they copied whole results into JavaScript arrays. Use the corresponding `*Document` call, and request compression through `options.transientCompression` |
| the separate `TransientFftSnapshot` DTO family | each `.FFT` spectrum is its own `fft` document naming its parent `tran` analysis |

### Execution options

The optional object is additive; existing calls need no changes:

```js
const options = {
  timeoutMilliseconds: 30_000,
  resourceLimits: {
    maxNetlistBytes: 2 * 1024 * 1024,
    maxAnalysisPoints: 50_000,
    maxResultValues: 1_000_000,
  },
  transientCompression: {
    absoluteTolerance: 1e-6,
    relativeTolerance: 1e-6,
    maximumInterval: 0,
    enabled: true,
  },
};
const handle = runAcAnalysisDocument(source, frequencies, options);
```

Omitted resource fields inherit browser-safe defaults. Unknown option or
resource fields are rejected, so a misspelled control cannot silently fall back
to a looser policy. `defaultResourceLimits()` returns all 16 current ceilings.
The defaults cap netlists at 8 MiB, circuit matrices at 2,000 unknowns,
analysis grids at 200,000 points, retained results at 2,000,000 scalar values,
shared caches at 64 MiB, and parallel workers at one because the browser build
is single-threaded.

`transientCompression` is a browser transfer policy, not a deck semantic: the
solver and the authored output projection are identical either way, and the
published result always carries the compression certificate that says which
grid it is on. Tolerances must be finite and non-negative;
`maximumInterval: 0` disables the time-axis gap ceiling.

### Cancellation and deadlines

Every export decodes its options, installs its cancellation control, and starts
its deadline in one place, then hands the composed abort source to an
abort-aware `rspice-core` entrypoint. There is no non-abort execution path, and
parsing uses the core abort-aware parser as well. There are two supported
controls:

- `timeoutMilliseconds` is an integer from 0 through 86,400,000. It starts
  after the options object is validated and before parser work. Zero requests
  immediate cancellation. The worker's monotonic `performance.now()` clock
  drives the deadline.
- `cancellation` supports exactly the `sharedInt32` mechanism shown below. Its
  `view` must be an `Int32Array` backed by `SharedArrayBuffer`; an ordinary
  `ArrayBuffer`, a DOM `AbortSignal`, an unknown mechanism, an out-of-range
  index, and unknown fields are rejected before parser or solver work.

```js
// Create this on the caller/main thread, then include `options` in the worker
// run message. Structured cloning retains the same SharedArrayBuffer storage.
const cancelBuffer = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT);
const cancelView = new Int32Array(cancelBuffer);
const options = {
  timeoutMilliseconds: 30_000,
  cancellation: {
    mechanism: "sharedInt32",
    view: cancelView,
    index: 0, // optional; defaults to zero
  },
};

// From the caller while the engine worker is synchronously inside WebAssembly:
Atomics.store(cancelView, 0, 1);
```

The control word is caller-owned: `0` means continue and any nonzero value
means cancel. Set it to zero before each new call and do not reuse the same word
for concurrent calls. In browsers, `SharedArrayBuffer` requires the normal
cross-origin-isolated deployment headers. The standalone Node contract uses
the same object without special flags on supported Node releases.

Cancellation is cooperative, not asynchronous exception injection. The core
observes it at bounded parser chunks and natural numerical checkpoints (for
example Newton/continuation, frequency, accepted-step, FFT, and compression
boundaries), and result-document validation, encoding, and the JSON export poll
it too. The exact wall-clock latency therefore depends on the cost of one
uncancellable numerical kernel. A successful result is never returned after a
poll observes cancellation; the export throws `RSpiceError` with
`code: "aborted"`, `category: "cancellation"`, and `retryable: true`.

A DOM `AbortSignal` cannot interrupt a synchronous WebAssembly call on the
same worker event loop, so claiming support for it would be misleading. The
binding rejects `mechanism: "abortSignal"` with
`code: "unsupported_cancellation"`. If `SharedArrayBuffer` is unavailable,
use `timeoutMilliseconds` for cooperative engine cancellation or terminate a
dedicated worker for unconditional process-isolation cancellation.

### Errors

Thrown errors expose the cross-interface `code`, compatibility `kind`,
`category`, conservative `retryable` policy, and `details`. `code`, `category`,
and `retryable` are taken from the core error descriptor, so a new core failure
taxonomy reaches JavaScript without a per-error edit in this crate.

The core failure taxonomy is execution-context free by design, so the deck
runner attaches `analysisId` and `coordinateId` to any failure raised inside
one coordinate-local analysis: a stepped deck that fails to converge says which
run and which card failed. Source locations are reported as `primarySource` /
`primaryLine` (and `relatedSource` / `relatedLine`). Resource failures add
`resource`, `requested`, and `limit`; convergence errors add `iterations`;
diagnostic errors retain unresolved output symbols.

### Rust API

The same operations are exported as plain Rust functions, since the crate
builds as both `cdylib` and `rlib`. `*_detailed` variants return `WasmError`,
`*_with_options_detailed` accept the typed `WasmExecutionOptions` policy, and
`*_with_options_and_abort_detailed` accept any core `AbortSignal`. They return
a `DeckExecution` holding the plan, the coordinates, and the core documents, so
a Rust embedder can consume the shared document directly instead of going
through the JavaScript handle.

## Module layout

- `options`: browser resource policy, execution options, compression policy
- `abort`: deadlines, the shared-memory cancellation control, and the composed
  abort source every runner polls
- `errors`: the structured `WasmError` and its JavaScript projection
- `dto`: parser-diagnostic and readiness summaries
- `document`: the descriptor-only projection of one core result document
- `hb_config`: the authored `.HB` tone list, resolved through the core constructors
- `handles`: the retained handle that publishes bounded typed-array windows
- `js_interop`: JavaScript value decoding and typed-array publication
- `runners::{deck, direct}`: the authored-deck route and the direct entry points
- `exports`: the `#[wasm_bindgen]` shims

Tests live beside what they test.

## Relationship to rspice-core

The dependency is declared as:

```toml
rspice-core = { path = "../rspice-core", default-features = false, features = ["wasm"] }
```

`default-features = false` drops `parallel`, `faer-parallel`, and `simd`
(no rayon or SIMD on this target); the `wasm` feature wires up
`wasm-bindgen` and the `getrandom/js` browser entropy source that faer
needs on `wasm32-unknown-unknown`. The browser playground runs engine calls
inside a dedicated module Web Worker so long solves do not block the page's
UI event loop. Inside that worker the solve is still single-threaded; Verilog-A
is not enabled here.

The full application surface is the [CLI](../rspice-cli/README.md), the
[Python bindings](../rspice-python/README.md), and the
[GUI](../rspice-ui/README.md).

## Building and the browser playground

Build instructions, the wasm-bindgen CLI version requirement, serving
notes, and a walkthrough of the demo page live in
[`web/README.md`](web/README.md) — see that file rather than duplicating
the steps. In short, it is a two-stage build:

```bash
cargo build -p rspice-wasm --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rspice_wasm.wasm \
  --out-dir crates/rspice-wasm/web/pkg --target web --no-typescript
```

`web/pkg/` is a build artifact and is not committed. Known gaps tracked in
`web/README.md`: `wasm-opt` is not applied, no TypeScript definitions are
generated, and the worker uses one single-threaded engine instance rather than
wasm threads.

## Testing

`cargo test -p rspice-wasm` runs native unit and integration tests for browser
defaults, option decoding, fail-closed field handling, structured error
contracts, abort propagation across every entry point, the descriptor
projection, and bounded window transfer.

The central test drives the shared capability registry: for every core
`AnalysisResultKind`, an exhaustive match supplies a deck, and the test asserts
that the family behaves the way `rspice_core::execution::capability` declares —
executing and round-tripping through the handle unchanged, or refusing by name
with the missing core API quoted. Further tests cover `.STEP`/`.TEMP`
coordinate products and collision-free artifact namespaces, `.ALTER` refusal
through the core materializer, coordinate-local failure attribution,
compression certificates, direct-entry-point planning and argument validation,
and the periodic large-signal cards.

CI also builds the real `wasm32-unknown-unknown` artifact. The static browser
contract is guarded by `tools/ci/test_wasm_playground.py`, which verifies that
the canonical playground routes engine calls through `engine-worker.js`, that
the worker reads bounded windows from the retained handle rather than copying
whole results, that AC controls are present, and that synchronous solve exports
stay off the main page. Numerical engine behavior remains covered in
`rspice-core`.

## License

RSpice WASM is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).

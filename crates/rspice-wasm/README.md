# rspice-wasm

WebAssembly bindings that run the RSpice engine in a browser or under Node, and
the engine behind the browser playground in [`web/`](web/). The crate owns no
result schema: every analysis produces the shared `rspice-analysis-result`
document from `rspice-core`, and this crate decides only how that document
crosses the JavaScript boundary. Results stay in WebAssembly memory and
JavaScript reads bounded, typed windows of them; all numerical work happens in
`rspice-core`.

## What's in here

| Module | Contents |
| :--- | :--- |
| `exports` | The `#[wasm_bindgen]` functions JavaScript calls |
| `runners` | `deck`, the authored-deck route and its control-script host, and `direct`, the single-analysis entry points |
| `handles` | `WasmResultHandle`, which retains results and publishes bounded windows |
| `document` | The descriptor-only projection of one core result document |
| `events` | Digital event rows, bus histories and the Value Change Dump |
| `options` | The browser resource policy, execution options and compression policy |
| `abort` | Deadlines, the shared-memory cancellation word and the composed abort source |
| `errors` | `WasmError` and its JavaScript projection |
| `dto` | Netlist summaries, parser diagnostics and the readiness report |
| `hb_config` | The authored `.HB` tone list, resolved through the core constructors |
| `js_interop`, `support` | JavaScript value decoding, typed-array publication and shared helpers |

## Boundaries

- The engine, card grammar and result document belong to `rspice-core`.
- The browser IDE is `rspice-app`, which executes through the separate
  `rspice-worker` image rather than through this crate.
- The command-line and Python surfaces are `rspice-cli` and `rspice-python`.

## Entry points

Every analysis export returns a `WasmResultHandle`, and every failure throws an
`RSpiceError` with structured fields.

| JavaScript function | Runs |
| :--- | :--- |
| `runAuthoredDeckDocument(source[, options])` | A complete deck, over its `.DATA`, `.STEP` and `.TEMP` coordinates |
| `runOperatingPointDocument(source[, options])` | One operating point |
| `runDcSweepDocument(source, sourceName, start, stop, step[, options])` | One linear single-source DC sweep |
| `runAcAnalysisDocument(source, frequencies[, options])` | AC over an explicit frequency grid |
| `runTransientAnalysisDocument(source, tstop, maxStep[, options])` | One transient |
| `runNoiseAnalysisDocument(source, outputNode, referenceNode, inputSource, frequencies[, options])` | Noise over an explicit positive frequency grid |
| `summarizeNetlist(source[, options])` | Counts and diagnostics of a parsed deck, without simulating |
| `healthCheck([options])` | A parser-to-solver readiness probe |
| `defaultResourceLimits()` | The browser resource policy, in camelCase |

The handle reports `resultCount()`, `coordinateCount()`, the executed plan
(`metadata()`) and each result's identity and descriptors
(`resultMetadata(i)`), and transfers data only through bounded calls:
`readWindow(i, start, count)` for result samples, `controlWindow(…)` for
control-script plot and print traces, `resultJson(i)` for the complete document,
`digitalNodes`, `digitalEvents`, `digitalBuses` and `busEvents` for event
histories, and `toVcd(i)` for the Value Change Dump `rspice run -f vcd`
publishes for the same run.

The same operations are plain Rust functions too, since the crate builds as
both `cdylib` and `rlib`: `run_*_document_detailed` and
`run_*_document_with_options_and_abort_detailed` return a `DeckExecution`
holding the plan, the coordinates and the core documents, for an embedder that
consumes the shared document directly.

## Result contract

Documents are `rspice-analysis-result` version 18, defined and validated by
`rspice-core`; `rspice_core::execution::result_document` documents their
encoding, missingness rules and payloads, and this crate does not restate them.
A direct call is planned as a one-analysis deck, so its result carries the same
canonical identity and run coordinate an authored deck would give it. The
handle's `metadata()` is the `rspice-browser-result` schema, version 3.

- `readWindow` returns axis and sample columns as `Float64Array` and a
  `Uint8Array` `validity` mask per signal. A zero validity entry marks an
  unavailable sample, whose numeric placeholder must not be read.
- A transfer is charged against a ceiling of 262,144 values by default, further
  reduced by `maxResultValues`, counting one validity byte per signal per point.
  An empty, out-of-range or over-budget window fails with
  `code: "invalid_result_window"`, and an unknown result with
  `"invalid_result_index"`. `resultJson` has its own byte budget and fails
  rather than truncating.
- Event calls fail with `"unsupported_result_family"`, `"unknown_event_node"`,
  `"unknown_event_bus"` or `"no_event_history"`. Event rows carry the `0..=12`
  XSPICE event code, so the drive strength the VCD drops survives here.

### Deck coverage

`runAuthoredDeckDocument` executes every planned analysis over the deck's
canonical `.DATA`, `.STEP` and `.TEMP` coordinate product: `.OP`, `.DC`
(including a second source), `.AC` and `.NOISE` (including `DATA=`), `.TRAN`
with its `.FFT` and `.FOUR` post-processing and optional compression, `.DISTO`,
`.TF`, `.PZ`, `.SENS`, `.SP` with its port noise, `.STB`, `.DCMATCH`, `.MC`,
`.PSS`, `.HB`, `.PAC`, `.PXF`, `.PNOISE`, `.PSTB` and `.ENVELOPE`.

These are refused by name before any solver work, naming the card and its
canonical instance identity, and never turned into an operating point, an empty
table or a nominal-temperature run:

- `.QPSS`, `.QPAC`, `.QPXF` and `.QPNOISE`;
- a small-signal card carrying `ACTIMES=` or `CHECKPOINT=`
  (`code: "unsupported_small_signal_state"`);
- textual `.ALTER`, which the core deck materializer refuses.

`rspice_core::execution::capability` declares this surface's coverage of every
result family, and the test suite takes that registry as its input.

A deck's `.control` script runs through the shared core control host, with the
same commands the CLI accepts. Each completed analysis keeps its own document
and identity; `metadata()` lists the script's `controlDatasets` (`ac1`, …) and
`controlPresentations`, whose trace data `controlWindow` transfers. A script
combined with run axes, `.FOUR` or `.FFT`, `.OPTIONS RESTART` or compression is
refused.

### Execution options

Every export takes an optional final options object:

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

Unknown option and resource fields are rejected, so a misspelled control cannot
fall back to a looser policy, and omitted resource fields take the browser
defaults `defaultResourceLimits()` returns: among them 8 MiB netlists, 2,000
matrix unknowns, 200,000 analysis points, 2,000,000 retained values, 64 MiB of
shared cache and one worker, because the browser build is single-threaded.
`transientCompression` is a transfer policy, not a deck semantic: the solver
runs identically either way, and the result carries the compression certificate
that says which grid it is on. Its tolerances must be finite and non-negative,
and `maximumInterval: 0` disables the time-axis gap ceiling.

### Cancellation and deadlines

Every export runs through an abort-aware `rspice-core` entry point, parser
included, under two controls:

- `timeoutMilliseconds`, an integer from 0 through 86,400,000, starts once the
  options are validated and is measured on `performance.now()`; zero cancels
  immediately.
- `cancellation` accepts only the `sharedInt32` mechanism, whose `view` must be
  an `Int32Array` over a `SharedArrayBuffer`. An ordinary `ArrayBuffer`, a DOM
  `AbortSignal` (`code: "unsupported_cancellation"`), an unknown mechanism, an
  out-of-range index and unknown fields are rejected before any work.

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

The control word belongs to the caller: `0` continues and any nonzero value
cancels. Clear it before each call and do not share one word between
concurrent calls. In a browser, `SharedArrayBuffer` needs cross-origin-isolated
deployment headers. Cancellation is cooperative: the core polls at bounded
parser chunks and numerical checkpoints, and document encoding and the JSON
export poll too, so the latency is one uncancellable kernel. A cancelled call
never returns a result; it throws with `code: "aborted"`,
`category: "cancellation"` and `retryable: true`. A DOM `AbortSignal` cannot
interrupt a synchronous WebAssembly call on its own event loop; without
`SharedArrayBuffer`, use `timeoutMilliseconds` or terminate the worker.

### Errors

A thrown `RSpiceError` carries `code`, `kind`, `category`, `retryable` and
`details`, with `code`, `category` and `retryable` taken from the core error
descriptor. A failure inside one coordinate-local analysis also names its
`analysisId` and `coordinateId`. Source locations are `primarySource` and
`primaryLine` (with `relatedSource` and `relatedLine`), resource failures add
`resource`, `requested` and `limit`, convergence failures add `iterations`, and
diagnostic errors keep their unresolved output symbols.

## Feature flags

The crate declares none. It depends on `rspice-core` with
`default-features = false`, which drops the `parallel`, `faer-parallel` and
`simd` backends this target cannot use, and with the `wasm` feature, which
wires up `wasm-bindgen`. The build is single-threaded and enables no Verilog-A
backend.

## Building and testing

```bash
cargo build -p rspice-wasm --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rspice_wasm.wasm \
  --out-dir crates/rspice-wasm/web/pkg --target web --no-typescript
```

[`web/README.md`](web/README.md) covers the wasm-bindgen CLI version and
serving the playground; `web/pkg/` is a build artifact and is not committed.

`cargo test -p rspice-wasm` runs the native tests: option decoding and
fail-closed fields, the structured error contract, abort propagation through
every entry point, bounded window transfer, the event accessors (including a
dump identical to the one `rspice run -f vcd` writes), and every result family
against the declared capability registry, executing or refusing by name.

The `wasm-bindgen-test` module in `exports` checks what only a real JavaScript
boundary shows: typed-array columns, explicit `null` metadata, `sharedInt32`
cancellation, and the rejection of a DOM `AbortSignal` or an ordinary
`ArrayBuffer`. CI runs it in Firefox through `wasm-bindgen-test-runner`:

```bash
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  WASM_BINDGEN_USE_BROWSER=1 \
  cargo test -p rspice-wasm --lib --target wasm32-unknown-unknown
```

`tools/ci/test_wasm_playground.py` guards the playground: engine calls run in
`engine-worker.js`, the worker reads bounded windows from the retained handle
rather than copying whole results, and no synchronous solve export is imported
by the page.

Licensed under the [RSpice Personal Use License](../../LICENSE).

# WASM Worker Transferable Waveforms Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move high-volume browser IDE simulation result arrays through transferable `Float64Array` buffers instead of structured-cloning nested JavaScript arrays.

**Architecture:** The worker contract keeps the existing `WorkerResponse` as the internal compatibility model, but adds a protocol-v2 transport envelope for browser messages. The envelope serializes metadata with serde and stores large `Vec<f64>` payloads as indexed buffers; the worker JS posts those buffers with a transfer list, and the main-thread wasm bridge reconstructs a normal `WorkerResponse` before handing results to existing UI code.

**Tech Stack:** Rust, wasm-bindgen, serde/serde-wasm-bindgen, js-sys `Float64Array`, browser module workers, Python static CI gates.

---

### Task 1: Static Gates For Transfer Lists

**Files:**
- Modify: `tools/ci/test_ide_worker.py`
- Modify: `tools/deploy/build_site.py`

- [x] **Step 1: Write failing tests**

Add assertions that both IDE workers define `responseTransferList(response)`, pass `responseTransferList(response)` as the second `postMessage` argument for result messages, and collect `view.buffer` from transferable typed-array views. Add source-contract gates requiring `WorkerResponseTransport`, `worker_response_transport_value`, and `worker_response_from_value`.

- [x] **Step 2: Run tests to verify failure**

Run: `uv run python tools/ci/test_ide_worker.py`

Expected: FAIL because result messages still call `postMessage({ type: "result", ... })` without a transfer list and the Rust transport symbols do not exist.

- [x] **Step 3: Implement gates**

Keep the checks textual and exact enough to catch regression in both `crates/rspice-ui/web/simulation-worker.js` and `site/ide/simulation-worker.js`, plus deploy-time checks in `tools/deploy/build_site.py`.

- [x] **Step 4: Run tests to verify pass after implementation tasks**

Run: `uv run python tools/ci/test_ide_worker.py` and `uv run python tools/deploy/test_build_site.py`.

Expected: PASS.

### Task 2: Pure Rust Protocol-V2 Transport

**Files:**
- Modify: `crates/rspice-ui/src/simulation/runner/worker_contract.rs`

- [x] **Step 1: Write failing tests**

Add worker-contract tests that convert transient, AC, and noise `WorkerResponse` values into `WorkerResponseTransport`, assert the high-volume arrays became buffer references, then reconstruct the original `WorkerResponse`. Add validation tests for missing buffers and length mismatches.

- [x] **Step 2: Run tests to verify failure**

Run: `cargo test -p rspice-ui --release --lib worker_transport`

Expected: FAIL because `WorkerResponseTransport` and reconstruction helpers do not exist.

- [x] **Step 3: Implement pure transport model**

Add `WorkerResponseTransport`, `WorkerOutcomeTransport`, `WorkerSimulationResultTransport`, `WorkerWaveformTransport`, `WorkerF64Series`, and `WorkerTransportBuffers`. Move high-volume `Vec<f64>` fields into buffers for DC sweep, transient, AC, noise, parametric, corner, reliability, optimization, and SOA results. Keep scalar/small variants inline and validate all referenced buffer IDs and lengths when reconstructing.

- [x] **Step 4: Run tests to verify pass**

Run: `cargo test -p rspice-ui --release --lib worker_transport`.

Expected: PASS.

### Task 3: WASM JsValue Encoding And Decoding

**Files:**
- Modify: `crates/rspice-ui/src/simulation/runner/worker_contract.rs`
- Modify: `crates/rspice-ui/src/simulation/runner/wasm_worker.rs`

- [x] **Step 1: Write failing tests/gates**

Extend `tools/ci/test_ide_worker.py` to require `Float64Array`, `worker_response_transport_value`, and `worker_response_from_value` symbols. The Rust unit tests from Task 2 cover pure reconstruction; wasm target check covers JS bindings.

- [x] **Step 2: Run tests to verify failure**

Run: `uv run python tools/ci/test_ide_worker.py`.

Expected: FAIL before implementation.

- [x] **Step 3: Implement wasm value helpers**

In `run_worker_request_value`, build the normal `WorkerResponse`, convert it to `WorkerResponseTransport`, then return a JS object `{ protocol: 2, response, buffers }` where each buffer is a `js_sys::Float64Array`. Add `worker_response_from_value` that detects protocol 2, reads typed arrays from `buffers`, reconstructs `WorkerResponse`, and falls back to v1 `serde_wasm_bindgen::from_value::<WorkerResponse>` when no protocol marker is present.

- [x] **Step 4: Update main-thread decode**

Change `wasm_worker.rs` result handling to call `worker_response_from_value(value)` instead of directly deserializing `WorkerResponse`.

- [x] **Step 5: Run verification**

Run: `cargo check -p rspice-ui --target wasm32-unknown-unknown --lib -j1`.

Expected: PASS.

### Task 4: Worker JavaScript Transfer Lists

**Files:**
- Modify: `crates/rspice-ui/web/simulation-worker.js`
- Modify: `site/ide/simulation-worker.js`

- [x] **Step 1: Write failing static gate**

Task 1 already adds this gate: both workers must post result messages with `responseTransferList(response)`.

- [x] **Step 2: Implement transfer-list helper**

Add a small `responseTransferList(response)` helper that returns `[]` for v1/errors and returns `response.buffers.map((view) => view.buffer)` only for typed-array views with `ArrayBuffer` backing stores. Use that helper as the second argument to result `postMessage`.

- [x] **Step 3: Run verification**

Run: `uv run python tools/ci/test_ide_worker.py`.

Expected: PASS.

### Task 5: Final Verification

**Files:**
- Verify all files touched above.

- [x] **Step 1: Rust focused tests**

Run: `cargo test -p rspice-ui --release --lib worker_contract`.

Expected: PASS.

- [x] **Step 2: Full UI library tests**

Run: `cargo test -p rspice-ui --release --lib -j1`.

Expected: PASS.

- [x] **Step 3: WASM build check**

Run: `cargo check -p rspice-ui --target wasm32-unknown-unknown --lib -j1`.

Expected: PASS.

- [x] **Step 4: Static site/worker gates**

Run: `uv run python tools/ci/test_ide_worker.py`, `uv run python tools/deploy/test_build_site.py`, and `uv run python tools/ci/test_wasm_playground.py`.

Expected: PASS.

Verification update:

```powershell
cargo test -p rspice-ui --release --lib worker_contract
cargo check -p rspice-ui --target wasm32-unknown-unknown --lib -j1
cargo clippy -p rspice-ui --lib --message-format short -- -D warnings
cargo fmt --all -- --check
uv run python tools/ci/test_ide_worker.py
uv run python tools/deploy/test_build_site.py
uv run python tools/ci/test_wasm_playground.py
uv run python tools/deploy/build_site.py --skip-headless --out $env:TEMP\rspice-site-verify
```

Result: all passed. The live Chrome smoke against the built site reported
worker readiness, protocol version 2, outer and inner response id agreement,
one transferred `Float64Array` buffer, and a populated result. After the user
clarified verification policy, subsequent non-ngspice checks for this slice
were run in debug:

```powershell
cargo test -p rspice-ui --lib
cargo check -p rspice-ui --lib
cargo clippy -p rspice-ui --lib -- -D warnings
cargo check -p rspice-ui --target wasm32-unknown-unknown --lib
```

Result: all passed; `rspice-ui --lib` passed 466/466 after the follow-up
owned waveform-storage optimization and review coverage.

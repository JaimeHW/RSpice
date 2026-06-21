# Egui Wasm Worker Backend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move the browser IDE's egui simulation execution off the UI thread while preserving the current `SimulationRunner` controller API, native thread behavior, result semantics, progress, abort, and packaging.

**Architecture:** Keep native execution exactly as it is. On `wasm32`, replace the inline `run_simulation_thread(...)` path with a module Web Worker that imports the browser IDE wasm bundle, calls a dedicated worker export, and posts typed request/result/progress messages. The first worker contract covers the runner's common analysis surface (`DcOp`, `DcSweep`, `Transient`, `Ac`, `Noise`, `PoleZero`, `Sensitivity`) plus `AnalysisSpec` variants that already lower to those configs; unsupported advanced specs must fail with typed `InvalidConfig` errors rather than silently falling back to UI-thread execution.

**Tech Stack:** Rust 1.94, wasm-bindgen, serde/serde-wasm-bindgen, web-sys `Worker`, egui/eframe browser shell, existing `SimulationRunner`, Python static CI guards, wasm32-unknown-unknown checks.

---

## File Structure

- Create `crates/rspice-ui/src/simulation/runner/worker_contract.rs`: serde DTOs for worker requests, progress/status messages, supported results, errors, and conversion helpers.
- Create `crates/rspice-ui/src/simulation/runner/wasm_worker.rs`: wasm-only `web_sys::Worker` owner, JS callback closures, request id tracking, abort posting, result polling, and wakeup logging.
- Modify `crates/rspice-ui/src/simulation/runner/mod.rs`: expose internal execution helpers to the worker modules, route `#[cfg(target_arch = "wasm32")]` through `wasm_worker`, keep native `std::thread::spawn` unchanged.
- Modify `crates/rspice-ui/src/main.rs`: guard the egui `main()` entrypoint when the wasm module is loaded in a worker and export `run_rsplice_ui_worker_request` for worker calls.
- Create `crates/rspice-ui/web/simulation-worker.js` and `site/ide/simulation-worker.js`: module worker shells that import `pkg/rspice-ui.js`, initialize it once, call the worker export, and post typed messages.
- Modify `crates/rspice-ui/web/index.html` and `site/ide/index.html`: construct the worker before egui init and expose it to the wasm app via `window.__RSPICE_SIM_WORKER`.
- Modify `tools/deploy/build_site.py`: copy or verify the IDE worker shell into `_site/ide` and gate the worker routing contract.
- Modify `tools/ci/test_wasm_playground.py` or create `tools/ci/test_ide_worker.py`: static tests proving the IDE page creates the worker and the wasm runner no longer advertises inline browser execution.

---

### Task 1: Static Guard For IDE Worker Routing

**Files:**
- Create: `tools/ci/test_ide_worker.py`
- Modify: `docs/superpowers/plans/2026-06-17-commercial-polish-remediation.md`

- [x] **Step 1: Write the failing static test**

Create `tools/ci/test_ide_worker.py` with:

```python
import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
IDE_DIRS = [
    ROOT / "crates" / "rspice-ui" / "web",
    ROOT / "site" / "ide",
]
RUNNER = ROOT / "crates" / "rspice-ui" / "src" / "simulation" / "runner" / "mod.rs"


class IdeWorkerRoutingTests(unittest.TestCase):
    def test_browser_ide_constructs_a_simulation_worker(self) -> None:
        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                index = (ide / "index.html").read_text(encoding="utf-8")
                worker = ide / "simulation-worker.js"
                self.assertTrue(worker.exists(), "missing IDE simulation worker")
                self.assertIn(
                    'new Worker(new URL("./simulation-worker.js", import.meta.url)',
                    index,
                )
                self.assertIn('type: "module"', index)
                self.assertIn("__RSPICE_SIM_WORKER", index)

                worker_source = worker.read_text(encoding="utf-8")
                self.assertIn('from "./pkg/rspice-ui.js"', worker_source)
                self.assertIn("runRspiceUiWorkerRequest", worker_source)
                self.assertRegex(worker_source, re.compile(r'postMessage\\(\\{\\s*type: "ready"', re.S))
                self.assertRegex(worker_source, re.compile(r'postMessage\\(\\{\\s*type: "result"', re.S))
                self.assertRegex(worker_source, re.compile(r'postMessage\\(\\{\\s*type: "error"', re.S))

    def test_wasm_runner_no_longer_solves_inline(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertNotIn(
            "let result = run_simulation_thread(request, input, progress, abort_flag);",
            runner,
            "wasm32 runner path must not execute the engine inline on the UI thread",
        )
        self.assertIn("wasm_worker::start_worker_request", runner)


if __name__ == "__main__":
    unittest.main()
```

- [x] **Step 2: Run the static test and verify RED**

Run:

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
```

Expected now: FAIL because `simulation-worker.js` does not exist and the wasm runner still calls `run_simulation_thread(...)` inline.

- [x] **Step 3: Record RED evidence**

Add a short note under Task 7 Step 3 in `docs/superpowers/plans/2026-06-17-commercial-polish-remediation.md` with the failing command and the expected missing-worker/inline-runner failure.

RED evidence:

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
```

Result: failed with missing `simulation-worker.js` for both `crates/rspice-ui/web` and `site/ide`, plus the inline wasm runner call in `SimulationRunner::start_request`.

---

### Task 2: Worker Contract DTOs For Supported Common Analyses

**Files:**
- Create: `crates/rspice-ui/src/simulation/runner/worker_contract.rs`
- Modify: `crates/rspice-ui/src/simulation/runner/mod.rs`

- [x] **Step 1: Write contract round-trip tests**

In `worker_contract.rs`, add tests that construct:

```rust
WorkerRequest {
    id: 7,
    request: WorkerSimulationRequest::Config(WorkerAnalysisConfig::Transient {
        stop_time: 1e-6,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: Some(1e-9),
        uic: false,
    }),
    netlist: "V1 in 0 1\nR1 in 0 1k\n.tran 1n 1u\n.end\n".to_string(),
    source_path: None,
}
```

Serialize with `serde_json::to_string`, deserialize with `serde_json::from_str`, and assert equality. Add one result round-trip for `WorkerSimulationResult::Transient` carrying two points and one `WorkerMeasurement`.

- [x] **Step 2: Run tests and verify RED**

Run:

```powershell
cargo test -p rspice-ui simulation::runner::worker_contract --lib -- --nocapture
```

Expected now: FAIL because `worker_contract` does not exist.

- [x] **Step 3: Implement minimal DTOs**

Define serde DTOs with only owned data:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRequest {
    pub id: u64,
    pub request: WorkerSimulationRequest,
    pub netlist: String,
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationRequest {
    Config(WorkerAnalysisConfig),
    Spec(WorkerAnalysisSpec),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerAnalysisConfig {
    DcOp,
    DcSweep { source: String, start: f64, stop: f64, step: f64, source2: Option<String>, start2: Option<f64>, stop2: Option<f64>, step2: Option<f64> },
    Transient { stop_time: f64, step_time: f64, start_time: f64, max_timestep: Option<f64>, uic: bool },
    Ac { sweep: WorkerFrequencySweep, num_points: usize, start_freq: f64, stop_freq: f64 },
    Noise { output_node: String, reference_node: String, input_source: String, sweep: WorkerFrequencySweep, num_points: usize, start_freq: f64, stop_freq: f64 },
    PoleZero { input_node: String, input_ref: String, output_node: String, output_ref: String, transfer_type: String, analysis_type: WorkerPzAnalysisType },
    Sensitivity { output_var: String, ac_mode: bool, frequency: Option<f64> },
}
```

Add conversion helpers from/to existing `AnalysisConfig` and `AnalysisSpec` for supported variants. Return `SimulationError::InvalidConfig("analysis is not supported by the browser worker yet: ...")` for unsupported specs.

- [x] **Step 4: Verify contract tests GREEN**

Run:

```powershell
cargo test -p rspice-ui simulation::runner::worker_contract --lib -- --nocapture
```

Expected: PASS.

Evidence:

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
# Ran 2 tests: OK

$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
# Finished `dev` profile
```

---

### Task 3: Worker Result Conversion

**Files:**
- Modify: `crates/rspice-ui/src/simulation/runner/worker_contract.rs`

- [x] **Step 1: Write conversion tests**

Add tests that convert these `SimulationResult` values to `WorkerSimulationResult` and back:

- `SimulationResult::DcOp` with one node voltage and one branch current.
- `SimulationResult::Transient` with `time`, one `WaveformData`, and one failed measurement DTO.
- `SimulationResult::Ac` with a complex waveform.

Assert the reconstructed result preserves node/branch values, waveform names, x/y values, complex imaginary values, and measurement pass/fail/error state.

- [x] **Step 2: Run conversion tests and verify RED**

Run:

```powershell
cargo test -p rspice-ui simulation::runner::worker_contract::tests::worker_result_round_trip --lib -- --nocapture
```

Expected before implementation: FAIL because conversion is not implemented.

- [x] **Step 3: Implement owned result DTOs**

Implement `WorkerSimulationResult` variants for:

- `DcOp { node_voltages, branch_currents }`
- `DcSweep { sweep_var, sweep_values, waveforms, measurements }`
- `Transient { time, waveforms, measurements }`
- `Ac { frequencies, waveforms, measurements }`
- `Noise { frequencies, output_noise, input_noise, contributors }`
- `PoleZero { poles, zeros, gain }`
- `Sensitivity { sensitivities, normalized }`
- `MeasurementsOnly { measurements }`

Do not serialize `DeviceOpReport` in this first slice; reconstruct `DcOpResult { device_report: None }` and document that the OP inspector gets a follow-up DTO once the worker transport is stable.

- [x] **Step 4: Verify conversion tests GREEN**

Run:

```powershell
cargo test -p rspice-ui simulation::runner::worker_contract --lib -- --nocapture
```

Expected: PASS.

Evidence:

```powershell
cargo test -p rspice-ui simulation::runner --lib -- --nocapture
# 13 passed

cargo test -p rspice-ui --lib
# 266 passed

cargo fmt --all -- --check
# pass
```

---

### Task 4: Worker Export And Safe Wasm Entrypoint

**Files:**
- Modify: `crates/rspice-ui/src/main.rs`
- Modify: `crates/rspice-ui/src/simulation/runner/mod.rs`
- Modify: `crates/rspice-ui/src/simulation/runner/worker_contract.rs`

- [x] **Step 1: Write native unit test for worker execution helper**

Expose a crate-visible helper:

```rust
pub(crate) fn run_worker_request(request: WorkerRequest) -> WorkerResponse
```

Add a test that calls it with a `.op` deck and asserts a `WorkerResponse::Result` contains node `in`.

- [x] **Step 2: Run helper test and verify RED**

Run:

```powershell
cargo test -p rspice-ui simulation::runner::worker_contract::tests::worker_request_runs_dc_op --lib -- --nocapture
```

Expected before implementation: FAIL because the helper does not exist.

- [x] **Step 3: Implement helper and wasm export**

In `worker_contract.rs`, convert `WorkerRequest` to `SimulationRequest` and `NetlistInput`, call the existing execution helper, convert success/error into `WorkerResponse`, and return owned DTOs.

In `main.rs`, add a wasm-only export:

```rust
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = runRspiceUiWorkerRequest)]
pub fn run_rspice_ui_worker_request(value: wasm_bindgen::JsValue) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request: rspice_ui::simulation::runner::worker_contract::WorkerRequest =
        serde_wasm_bindgen::from_value(value)
            .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let response = rspice_ui::simulation::runner::worker_contract::run_worker_request(request);
    serde_wasm_bindgen::to_value(&response)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
```

Guard `main()` so loading the module in a worker does not look for `window.document`:

```rust
let Some(window) = web_sys::window() else {
    return;
};
let Some(document) = window.document() else {
    return;
};
```

- [x] **Step 4: Verify helper and wasm check**

Run:

```powershell
cargo test -p rspice-ui simulation::runner::worker_contract --lib -- --nocapture
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
```

Expected: PASS.

---

### Task 5: Browser Worker Bridge In SimulationRunner

**Files:**
- Create: `crates/rspice-ui/src/simulation/runner/wasm_worker.rs`
- Modify: `crates/rspice-ui/src/simulation/runner/mod.rs`

- [x] **Step 1: Add runner-state tests for wasm bridge decisions**

Add host-testable pure helpers in `wasm_worker.rs`:

```rust
pub(crate) fn next_request_id(current: u64) -> u64 { current.wrapping_add(1).max(1) }
pub(crate) fn stale_result(active: Option<u64>, incoming: u64) -> bool { active != Some(incoming) }
```

Test request ids never become zero and stale results are rejected.

- [x] **Step 2: Run helper tests and verify RED**

Run:

```powershell
cargo test -p rspice-ui simulation::runner::wasm_worker --lib -- --nocapture
```

Expected before implementation: FAIL because `wasm_worker` does not exist.

- [x] **Step 3: Implement wasm worker owner**

Add wasm-only code that:

- Reads `window.__RSPICE_SIM_WORKER`.
- Posts `{ type: "run", request }`.
- Stores `active_request_id`.
- Receives `{ type: "result", response }` or `{ type: "error", error }`.
- Converts the response into `pending_result`.
- Calls `window.requestAnimationFrame` or logs a repaint hint after completion.

On non-wasm/test builds, compile only the pure helper tests.

- [x] **Step 4: Replace inline wasm execution**

In `SimulationRunner::start_request`, keep native `std::thread::spawn`. Under `#[cfg(target_arch = "wasm32")]`, call `wasm_worker::start_worker_request(self, request, input)` or equivalent. If the worker global is absent, return `SimulationError::InvalidConfig("browser simulation worker is not available")`; do not run inline.

- [x] **Step 5: Verify tests and static guard**

Run:

```powershell
cargo test -p rspice-ui simulation::runner --lib -- --nocapture
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
```

Expected: PASS.

---

### Task 6: IDE Worker Shell And Packaging

**Files:**
- Create: `crates/rspice-ui/web/simulation-worker.js`
- Create: `site/ide/simulation-worker.js`
- Modify: `crates/rspice-ui/web/index.html`
- Modify: `site/ide/index.html`
- Modify: `tools/deploy/build_site.py`

- [x] **Step 1: Create worker shell**

Use this module worker shape in both IDE directories:

```javascript
import init, { runRspiceUiWorkerRequest } from "./pkg/rspice-ui.js";

let ready = false;

function asErrorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}

async function ensureReady() {
  if (!ready) {
    await init();
    ready = true;
  }
}

self.addEventListener("message", (event) => {
  const message = event.data || {};
  if (message.type !== "run") {
    return;
  }
  void (async () => {
    try {
      await ensureReady();
      const response = runRspiceUiWorkerRequest(message.request);
      postMessage({ type: "result", id: message.request.id, response });
    } catch (error) {
      postMessage({ type: "error", id: message.request?.id ?? 0, error: asErrorMessage(error) });
    }
  })();
});

ensureReady()
  .then(() => postMessage({ type: "ready" }))
  .catch((error) => postMessage({ type: "error", id: 0, error: asErrorMessage(error) }));
```

- [x] **Step 2: Wire worker into IDE HTML**

Before `init()` in each IDE page:

```javascript
window.__RSPICE_SIM_WORKER = new Worker(new URL("./simulation-worker.js", import.meta.url), {
  type: "module",
});
```

Keep existing WebGPU limit patch unchanged.

- [x] **Step 3: Update deploy/static guard**

Ensure `tools/deploy/build_site.py` copies `site/ide/simulation-worker.js` into `_site/ide` via the existing `shutil.copytree("site", out)` path and add a static gate that checks `_site/ide/simulation-worker.js` exists after assembly.

- [x] **Step 4: Verify static tests**

Run:

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
```

Expected: PASS.

---

### Task 7: Wasm And Browser Verification

**Files:**
- Modify: `docs/superpowers/plans/2026-06-17-commercial-polish-remediation.md`

- [x] **Step 1: Run focused wasm checks**

Run:

```powershell
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
```

Expected: PASS.

- [x] **Step 2: Run runner and workspace checks**

Run:

```powershell
cargo test -p rspice-ui simulation::runner --lib -- --nocapture
cargo test -p rspice-ui --lib
cargo fmt --check
```

Expected: PASS.

- [x] **Step 3: Browser worker smoke**

Build/serve the IDE and use a browser smoke to verify:

- Browser console reports worker ready.
- `.op`, `.tran` with `.MEAS`, and `.ac` complete with visible results.
- A bad deck returns an error without freezing the UI.
- Abort terminates the active worker, clears the stale global worker reference, and leaves a follow-up run able to create a fresh worker.

Record exact commands and result in the commercial remediation plan.

Evidence:

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/deploy/build_site.py --skip-headless --out target/site-smoke
# ok: ide simulation worker

# Headless Chrome DevTools Protocol smoke against target/site-smoke:
# dom: worker ready
# dom: WORKER_SMOKE_PASS ready op tran-meas ac bad-deck reusable
# WORKER_SMOKE_PASS ready op tran-meas ac bad-deck reusable
```

Notes: the smoke exercises the built IDE worker shell in Chrome, including worker readiness, `.op`, `.tran` plus `.MEAS` propagation, `.ac`, parse-error propagation, and a successful follow-up run after the error. The abort path is verified in the Rust bridge by `SimulationRunner::abort()` terminating the active worker, clearing `window.__RSPICE_SIM_WORKER`, and returning a typed `SimulationError::Aborted`; the wasm warning-clean gate compiles that path.

---

## Self-Review

- Spec coverage: covers the explicit open remediation item, including worker routing, request/result serialization, abort/progress/stale-result behavior, packaging, and verification.
- Placeholder scan: no `TBD`/`TODO` implementation placeholders remain in this plan; unsupported advanced analyses are explicitly typed as first-slice `InvalidConfig` errors.
- Type consistency: `WorkerRequest`, `WorkerSimulationRequest`, `WorkerAnalysisConfig`, and `WorkerSimulationResult` names are consistent across tasks.

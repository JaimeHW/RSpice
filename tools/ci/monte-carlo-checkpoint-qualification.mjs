// A small real-worker scenario: receive a journal, terminate, transfer it into
// another worker, resume missing trials, then compare against a fresh population.
const workerUrl = new URL(new URL(location.href).searchParams.get("worker"), location.href);
if (workerUrl.origin !== location.origin) throw new Error("Worker must use this origin");
const workers = new Set();
const assert = (condition, message) => { if (!condition) throw new Error(message); };
const json = value => JSON.stringify(value, (_, item) => typeof item === "bigint" ? item.toString() : item);
const deck = "Browser checkpoint\n.param r=1k\nV1 in 0 1\nR1 in out {r}\nR2 out 0 1k\n.mc 6 uniform 0.2 seed 37\n.end\n";

async function openWorker() {
  const worker = new Worker(workerUrl, {type: "module"});
  workers.add(worker);
  await new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("Worker startup timed out")), 90000);
    worker.onerror = event => { clearTimeout(timer); reject(new Error(event.message)); };
    worker.onmessage = ({data}) => {
      if (data.type === "ready") { clearTimeout(timer); resolve(); }
      if (data.type === "error") { clearTimeout(timer); reject(new Error(data.error)); }
    };
  });
  return worker;
}

// Independently encode the public content identity used by checkpoint inputs.
async function digest(bytes) {
  const encoder = new TextEncoder();
  const domain = encoder.encode("rspice.studio-monte-carlo-checkpoint/v1");
  const size = value => {
    const result = new Uint8Array(8);
    new DataView(result.buffer).setBigUint64(0, BigInt(value));
    return result;
  };
  const parts = [encoder.encode("RSPICE-CANONICAL"), [0, 1, 0xd0], size(domain.length), domain,
    [0x08], size(bytes.length), bytes];
  const encoded = new Uint8Array(parts.reduce((count, part) => count + part.length, 0));
  let offset = 0;
  for (const part of parts) { encoded.set(part, offset); offset += part.length; }
  return Array.from(new Uint8Array(await crypto.subtle.digest("SHA-256", encoded)),
    byte => byte.toString(16).padStart(2, "0")).join("");
}

function request(id, bins, resume = null, source = deck) {
  return {
    protocolVersion: 34,
    request: {
      request: {
        id, request: {Spec: {spec: {MonteCarlo: {variation_source: "parameter_tolerance", params: []}},
          options: {mc_histogram_bins: bins, mc_checkpoint: {publish_every: 1,
            trial_range: {start: 0, end: 6}, resume: resume ? {digest: resume.digest} : null}}}},
        netlist: source, source_path: null,
        project_veriloga_runtimes: {runtimes: [], connections: []},
      },
      dependency_metadata: '{"snapshot_digest":null,"bindings":[],"artifacts":[]}',
      dependency_buffer_count: 0,
    },
    buffers: [], byteBuffers: resume ? [resume.bytes.slice()] : [],
  };
}

function run(worker, packet, stopAfterCheckpoint = false) {
  const id = packet.request.request.id;
  return new Promise((resolve, reject) => {
    const checkpoints = [];
    const timer = setTimeout(() => reject(new Error(`Run ${id} timed out`)), 30000);
    const finish = result => { clearTimeout(timer); resolve({...result, checkpoints}); };
    worker.onerror = event => { clearTimeout(timer); reject(new Error(event.message)); };
    worker.onmessage = ({data}) => {
      try {
        assert(Number(data.id) === id, `Run ${id} received a different request identity`);
        if (data.type === "monteCarloCheckpoint") {
          assert(data.checkpoint instanceof Uint8Array && data.checkpoint.length > 0,
            "Checkpoint did not arrive as nonempty Uint8Array");
          checkpoints.push(data.checkpoint);
          if (stopAfterCheckpoint) {
            worker.terminate(); workers.delete(worker);
            finish({terminated: true});
          }
        } else if (data.type === "result") {
          assert(data.response.protocolVersion === 28, "Unexpected response protocol");
          assert(Number(data.response.response.id) === id, "Nested response identity differs");
          finish({outcome: data.response.response.outcome});
        } else if (data.type === "error") {
          finish({error: data.error});
        }
      } catch (error) { clearTimeout(timer); reject(error); }
    };
    const transfer = packet.byteBuffers.map(view => view.buffer);
    worker.postMessage({type: "run", id, request: packet}, transfer);
    assert(transfer.every(buffer => buffer.byteLength === 0), "Resume bytes were not transferred");
  });
}

function population(run) {
  const value = run.outcome?.Success?.Inline?.MonteCarlo;
  assert(value, `Worker did not return Monte Carlo success: ${json(run)}`);
  assert(Number(value.runs_completed) === 6 && Number(value.num_failures) === 0,
    `Incomplete trial population: ${json([value.runs_completed, value.num_failures])}`);
  assert(value.member_measurements.length === 6, "Lost per-trial evidence");
  return value;
}

async function qualify() {
  const first = await openWorker();
  const interrupted = await run(first, request(1, 3), true);
  assert(interrupted.terminated && interrupted.checkpoints.length === 1,
    "Did not retain a checkpoint before hard termination");
  const bytes = interrupted.checkpoints[0];
  const partial = {bytes, digest: await digest(bytes)};
  const worker = await openWorker();
  const resumed = await run(worker, request(2, 5, partial));
  const resumedPopulation = population(resumed);
  assert(resumed.checkpoints.length === 5, "Resume did not solve only the five missing trials");
  const varying = resumedPopulation.variables.find(value => value.name.toLowerCase() === "v(out)");
  assert(varying?.histogram.length === 5 && varying.max > varying.min,
    "Changed histogram configuration did not reach the resumed result");

  const completeBytes = resumed.checkpoints.at(-1);
  const complete = {bytes: completeBytes, digest: await digest(completeBytes)};
  const cached = await run(worker, request(3, 5, complete));
  assert(cached.checkpoints.length === 1, "Cached population must publish exactly one journal");
  assert(json(population(cached)) === json(resumedPopulation), "Cached observations changed");
  assert(await digest(cached.checkpoints[0]) === complete.digest, "Cached journal bytes changed");

  const fresh = await run(worker, request(4, 5));
  assert(fresh.checkpoints.length === 6, "Fresh run did not publish each trial");
  assert(json(population(fresh)) === json(resumedPopulation), "Resume differs from fresh results");

  const changed = await run(worker, request(5, 5, complete, deck.replace("R2 out 0 1k", "R2 out 0 2k")));
  assert(changed.outcome?.Failure && changed.checkpoints.length === 0,
    "Changed circuit reused incompatible trials");
  const corruptBytes = bytes.slice();
  corruptBytes[corruptBytes.length - 1] ^= 1;
  const corrupt = await run(worker, request(6, 5, {digest: partial.digest, bytes: corruptBytes}));
  assert(corrupt.error?.includes("content identity") && corrupt.checkpoints.length === 0,
    "Worker accepted corrupted transferred bytes");
  return {status: "passed", retainedBeforeTermination: 1, resumedTrials: 5, freshTrials: 6,
    cachedPublications: 1, histogramBins: varying.histogram.length,
    completeCheckpointBytes: completeBytes.length, refusedChangedCircuit: true, refusedCorruption: true};
}

let verdict;
try { verdict = await qualify(); }
catch (error) { verdict = {status: "failed", error: error.stack || String(error)}; }
finally { for (const worker of workers) worker.terminate(); }
document.body.textContent = json(verdict);
await fetch("/mc-checkpoint-verdict", {method: "POST", headers: {"Content-Type": "application/json"},
  body: json(verdict)});

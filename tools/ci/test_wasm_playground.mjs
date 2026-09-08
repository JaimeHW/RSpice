import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';

const web = new URL('../../crates/rspice-wasm/web/', import.meta.url);
const page = readFileSync(new URL('index.html', web), 'utf8');
const script = page.match(/<script type="module">([\s\S]*?)<\/script>/)[1]
  .replaceAll('import.meta.url', JSON.stringify(web.href));

function playground() {
  const elements = new Map();
  const element = () => ({ children: [], attributes: {}, textContent: '',
    appendChild(child) { this.children.push(child); },
    setAttribute(name, value) { this.attributes[name] = value; }, addEventListener() {} });
  const context = vm.createContext({ URL, Float64Array, Uint8Array,
    document: { getElementById(id) {
      if (!elements.has(id)) elements.set(id, element());
      return elements.get(id);
    }, createElement: element, createElementNS: element, addEventListener() {} },
    window: { addEventListener() {} }, Worker: class { addEventListener() {} } });
  vm.runInContext(script, context);
  for (const [id, value] of Object.entries({ tstop: '6m', hmax: '10u', fstart: '10', fstop: '100k', fpoints: '61' })) {
    context.document.getElementById(id).value = value;
  }
  return { context, elements };
}

test('playground reads flat typed axes and rejects missing or non-finite axes', () => {
  const { context } = playground();
  for (const name of ['time', 'frequency']) {
    context.entry = { window: { axes: [{ name, values: new Float64Array([0, 1, 2]) }] } };
    assert.deepEqual(Array.from(vm.runInContext(`axisValues(entry, '${name}')`, context)), [0, 1, 2]);
  }
  for (const axes of [[], [{ name: 'time', values: { values: [0, 1] } }],
    [{ name: 'time', values: new Float64Array([0, NaN]) }]]) {
    context.entry = { window: { axes } };
    assert.throws(() => vm.runInContext("axisValues(entry, 'time')", context), /result axis/);
  }
});

test('empty AC and transient windows cannot publish a solved notice', async () => {
  for (const [kind, action, axis] of [['ac', 'doAc', 'frequency'], ['tran', 'doTran', 'time']]) {
    const { context, elements } = playground();
    context.response = { elapsedMs: 1, result: { results: [{ metadata: { resultKind: kind,
      signals: [{ kind: 'voltage', displayName: 'V(out)', owner: { name: 'out' } }] },
      window: { axes: [{ name: axis, values: new Float64Array() }], signals: [] } }] } };
    await vm.runInContext(`runEngine = async () => response; ${action}()`, context);
    const lines = elements.get('log').children;
    assert.ok(lines.some(line => line.className === 'err' && line.textContent.includes('No available')));
    assert.ok(lines.every(line => !line.textContent.includes('solved in')));
  }
});

test('worker releases retained engine documents after reading or failing a window', () => {
  const worker = readFileSync(new URL('engine-worker.js', web), 'utf8')
    .replace(/^import[\s\S]*?from "\.\/pkg\/rspice_wasm\.js";/, '')
    .split('async function ensureReady()')[0];
  for (const failure of [false, true]) {
    let freed = 0;
    const handle = { metadata: () => ({ results: [{ index: 0 }] }),
      resultMetadata: () => ({ pointCount: 2, valuesPerPoint: 1, maximumWindowValues: 10, signals: [] }),
      readWindow() { if (failure) throw new Error('window failed'); return { count: 2 }; },
      free() { freed++; } };
    const context = vm.createContext({ handle });
    vm.runInContext(worker, context);
    if (failure) assert.throws(() => vm.runInContext('readHandle(handle)', context), /window failed/);
    else assert.equal(vm.runInContext('readHandle(handle).results[0].window.count', context), 2);
    assert.equal(freed, 1);
  }
});

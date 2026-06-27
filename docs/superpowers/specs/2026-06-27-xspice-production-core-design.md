# XSPICE Production Core Design

## Goal

Implement the production-core XSPICE slice that makes RSpice handle the
high-value ngspice46 XSPICE example groups before attempting full catalog
parity. The target is practical mixed-signal coverage for commercial use:
analog behavioral blocks, file-driven stimuli, lookup/transfer functions,
oscillator and one-shot blocks, and the PLL/delta-sigma building blocks that
exercise the existing digital event engine.

## Scope

This phase targets the upstream ngspice46 example groups under:

- `examples/xspice/original-examples`
- `examples/xspice/filesource`
- `examples/xspice/various`
- `examples/xspice/pll`
- `examples/xspice/delta-sigma`
- directly required helper data files from those directories

The existing vendored `tests/ngspice/xspice/digital` suite remains a required
green gate.

## Non-Goals

This phase is not full ngspice46 XSPICE catalog parity. It will not implement
external co-simulation models such as `d_cosim`, `d_process`, GHDL, Icarus,
Verilator, or any runtime that requires a foreign simulator process.

This phase will not port ngspice GPL `xspice/icm/table` implementation code.
Any table-like behavior must be implemented from public behavior descriptions,
black-box oracle runs, and checked-in test decks.

This phase will not rewrite the whole event engine unless a production-core
model requires a specific missing capability. Engine work must stay scoped to
the minimum abstractions needed by the selected models.

## Current State

RSpice already has a native XSPICE framework in
`crates/rspice-core/src/xspice` with:

- built-in code-model registry
- A-device parsing with analog and digital ports
- scalar numeric and string model parameters
- digital event queue and transient breakpoint integration
- digital traces and ngspice `eprint` comparison for the vendored digital
  regression suite
- native implementations for the existing digital gates, latches, flip-flops,
  `d_source`, `d_state`, `d_ram`, `adc_bridge`, `dac_bridge`, and a small set
  of analog blocks

The main production-core gaps are:

- official ngspice code-model names are not all registered as aliases
- model-card vector parameters are not represented in `ModelDef`,
  `XspiceInstance`, or `CmContext`
- A-device inline params remain scalar-only
- `pwl`, `pwlts`, `filesource`, waveform generators, transfer blocks, and
  several PLL-oriented models are missing
- the test corpus currently vendors only the small ngspice XSPICE digital
  regression directory, not the production-core examples

## Architecture

### Parameter Representation

Extend model parameter plumbing to support:

- scalar numeric params
- string params
- real vector params
- integer vector params when needed by a selected model

Vector support must be centralized in:

- `crates/rspice-core/src/netlist/ast.rs`
- `crates/rspice-core/src/netlist/parser/values.rs`
- `crates/rspice-core/src/xspice/traits.rs`
- `crates/rspice-core/src/xspice/context.rs`
- `crates/rspice-core/src/xspice/instance.rs`
- `crates/rspice-core/src/engine/builder/model_resolution/xspice.rs`

The public `ParamSpec` contract must describe vector defaults and validation
rules rather than forcing every model to parse vector syntax manually.

### Parser Behavior

The parser must accept ngspice-style vector model parameters such as:

```spice
.model pwl1 pwl (x_array=[-1 0 1] y_array=[0 1 0])
.model filter1 s_xfer (num_coeff=[1] den_coeff=[1 1])
```

Model-card vector params are the priority because the production-core examples
use them heavily. Inline A-device vector params are included in this phase for
the same bracket-array syntax used by model cards, so deck authors get one
consistent XSPICE parameter language across model and instance overrides.

### Code Model Registry

Register official ngspice names where RSpice already has equivalent behavior:

- `int` maps to the existing integrator implementation
- `d_dt` maps to the existing differentiator implementation
- `divide` maps to the existing divider implementation if port semantics match
- `file_source` and `filesource` must resolve to the same implementation

Aliases must be explicit registered models or a registry alias table. They
must not be implemented as deck-name special cases.

### Model Modules

Add focused production-core modules under `crates/rspice-core/src/xspice/models`
or split existing modules when files become too broad:

- `lookup.rs`: `pwl`, `pwlts`, and any small interpolation helpers
- `sources.rs` or `analog_sources.rs`: `filesource`, `sine`, `square`,
  `triangle`
- `transfer.rs`: `xfer`, `s_xfer`, and transfer-function state helpers
- `timing.rs`: `oneshot`, `hyst`, `slew`, and related stateful timing blocks

Each model must expose its official XSPICE model name through `CodeModel`.
Shared helpers must be private to the model group unless multiple groups
need them.

### Runtime Semantics

Analog-only models must stamp through the existing analog output path where
possible. If a model needs a conductance or controlled source stamp, it must
use the existing deferred-stamp API in `CmContext` rather than adding a parallel
stamping surface.

Stateful transient models must store accepted state in `CmContext` and rely
on the existing timestep accept path. Models that require discontinuity times
must schedule events or breakpoints through the same XSPICE/transient
integration used by `d_source`.

Models must fail closed with precise `CmError` messages for malformed files,
invalid array shapes, non-monotonic time arrays where monotonicity is required,
empty lookup tables, incompatible port widths, and unsupported parameter
combinations.

## Acceptance Tests

Add a new focused production-core XSPICE suite in the ngspice runner rather
than broadening the existing digital test name. The suite must run checked-in
copies of selected upstream example decks and required data files.

The first acceptance set must include minimal decks for:

- official name aliasing: `int`, `d_dt`, `divide`
- vector model parameters: `pwl` and `s_xfer`
- file stimulus: `filesource`
- stateful analog behavior: `hyst`, `slew`, `oneshot`
- waveform generators: `sine`, `square`, `triangle`
- PLL/delta-sigma support decks after their primitive models pass focused tests

For each model, the implementation order is:

1. Add a failing focused test against a minimal ngspice-backed deck.
2. Implement the smallest production behavior that passes the test.
3. Add edge-case tests for invalid params or shape errors.
4. Add the relevant upstream example deck to the suite.
5. Run the focused suite, the existing XSPICE digital suite, and the full
   ngspice regression summary.

## Browser/WASM Parity

XSPICE production-core models are included in browser builds by default. The
wasm build must not omit XSPICE models behind native-only feature gates. File
source models must keep a clean abstraction for file loading so browser-hosted
netlists can provide bundled or virtual-file data without depending on native
filesystem APIs in the model logic.

## Provenance

Behavior can be validated against the local ngspice46 binary and checked-in
oracle output. Implementation must be original Rust. The ngspice GPL
`xspice/icm/table` implementation is not a permissible source for translation
or close porting. If a model's only practical documentation is oracle behavior,
derive the implementation from black-box input/output tests and record that in
the model tests or support notes.

## Completion Criteria

This phase is complete when:

- selected production-core ngspice XSPICE example decks are vendored with only
  files required to run them
- the new production-core XSPICE suite passes
- the existing `tests/ngspice/xspice/digital` suite still passes
- the full ngspice regression summary remains green
- `rspice-core`, `rspice-wasm`, and `rspice-ui` wasm checks pass
- unsupported full-catalog XSPICE models still fail closed with clear errors
  instead of silent no-op behavior

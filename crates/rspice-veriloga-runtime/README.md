# rspice-veriloga-runtime

The runtime ABI of RSpice's generated Verilog-A models, and the shared rules
every Verilog-A backend executes. `rspice-veriloga`'s offline Rust backend
emits model code against this crate, each generated model crate under
`crates/rspice-veriloga-models/models/` depends on it and nothing else, and
the compiler's in-process backends (bytecode VM, native JIT, browser JIT) take
their integer, arithmetic, operator and phase rules from it, so all four
backends agree. It must not depend on `rspice-core`: the edges run
`rspice-core` to `rspice-veriloga-models` to each generated model to this
crate, and an edge back to the engine would close a cycle.

## What's in here

| Module | Contents |
| :--- | :--- |
| crate root | Model, terminal and parameter descriptors (`GeneratedVerilogAModelDescriptor`); parameter assignment with model-card or instance provenance, and declared-range validation; the evaluation context and the static and reactive stampers generated code writes through; noise descriptors and evaluation; persistent and checkpoint state; `limexp` and the limited exponentials; evaluation errors |
| `integration`, `integration_state` | `ddt`, `idt` and `idtmod` candidates over accepted history, and transactional `idtmod` state |
| `event_control` | `cross`, `above`, `timer` and `last_crossing` evaluation over accepted crossing history |
| `transport_delay` | Accepted `absdelay` history, shared with native devices in `rspice-core` |
| `absdelta` | Interpolated A/D observations for `absdelta` |
| `integer`, `array_index` | Verilog-AMS `integer` semantics (signed 32-bit wrapping, round-half-away conversion, logical shifts) and checked runtime array indexing |
| `arithmetic`, `polynomial` | Shared scalar arithmetic with extended-range intermediates; exact complex polynomial evaluation for bounded unit-circle responses |
| `analog_lifecycle`, `simparam` | Evaluation and analysis phases behind `analysis()`, and the `$simparam` set |
| `analog_effects` | The journal of analog system-task calls, published only when a step is accepted |
| `ground_reference` | Whether a module's contributions connect one of its ports to node 0 |
| `noise_frequency` | Frequency substitution for coherently routed noise tangents |
| `compatibility_catalog` | `GENERATED_VERILOGA_COMPATIBILITY_CATALOG`: each explicitly authenticated migration from an earlier generated format |

## Boundaries

- The compiler and the generator that writes code against this ABI:
  [`rspice-veriloga`](../rspice-veriloga/README.md), whose
  [Generating the built-in device models](../rspice-veriloga/README.md#generating-the-built-in-device-models)
  covers regenerating the catalog.
- The generated models and their feature-selectable catalog:
  [`rspice-veriloga-models`](../rspice-veriloga-models/README.md), whose
  README is generated with it.
- Routing a card or an `X` line to a generated model, and driving the
  instance in a circuit: [`rspice-core`](../rspice-core/README.md), through
  its `device::veriloga_builtins` adapter.
- The matrices the stampers write into: [`rspice-matrix`](../rspice-matrix).

## Entry points

The callers are generated code and the engine-side adapter in `rspice-core`.

- `GeneratedVerilogAModelDescriptor` — a model's identity and external
  contract: model and module names, source and checkpoint identities,
  terminals, parameters and internal nodes.
- `GeneratedParameterAssignment` and `GeneratedParameterOrigin` — a value with
  its model-card or instance provenance; a value written through the wrong
  storage class is rejected, because `$param_given` depends on it.
- `validate_generated_parameter` — a declared parameter's finiteness, integer
  type, inclusive or exclusive bounds, and excluded values.
- `GeneratedEvalContext`, `GeneratedStamper` and `GeneratedReactiveStamper` —
  what one evaluation reads, and where its contributions go.
- `rspice_eval_ddt` and `rspice_eval_idt` — the analog operators over accepted
  history.
- `GeneratedVerilogAInstanceCheckpoint` — the per-instance state a transient
  checkpoint carries.

## Persisted identities

Saved schematic bindings and transient checkpoints record these, and a build
checks them before reconstructing anything against its own catalog:

- `GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION` versions the descriptor
  contract, independently of checkpoint and stamp-workspace versions. It is
  what decides whether a persisted schematic binding can be reconstructed
  from the catalog a build contains.
- `GeneratedVerilogAAcceptedStateShapeIdentity` is the exact shape of a
  model's accepted dynamic state, and `GENERATED_PERSISTENT_STATE_VERSION`
  versions the persistent-state record a checkpoint carries.
- `GENERATED_VERILOGA_COMPATIBILITY_CATALOG` lists each earlier identity
  explicitly authenticated as equivalent. Matching source text alone is never
  enough to claim one.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `serde` | `Serialize`/`Deserialize` for the phase, task, integer-operation, packed-array and `$simparam` types; `rspice-veriloga` enables it |
| `faer-parallel` | Forwards to `rspice-matrix/faer-parallel` |

## Testing

```bash
cargo test -p rspice-veriloga-runtime
cargo run -p rspice-veriloga --bin rspice-veriloga-gen -- check-builtins
```

This crate's `Cargo.toml` and `src/` are inputs to the generator digest
recorded in the catalog's `manifest.txt`, so any change to them fails
`check-builtins` until the catalog is regenerated.

Licensed under the [RSpice Personal Use License](../../LICENSE).

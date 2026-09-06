# rspice-veriloga-models

The precompiled Verilog-A model catalog: 43 CMC compact models, compiled from
their Verilog-A sources to Rust ahead of time by
[`rspice-veriloga`](../rspice-veriloga)'s generator and selected by Cargo
feature.

**Everything under `src/` and `models/` is generated. Do not edit it.**
Regenerate from the sources under `models/veriloga/` instead:

```bash
cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- \
    regenerate-builtins

# Verify the checked-in output is current; no writes
cargo run -p rspice-veriloga --bin rspice-veriloga-gen -- check-builtins
```

`check-builtins` compares `manifest.txt`'s `source_tree_digest` and
`generator_digest` and fails with the exact regeneration command when either has
moved, so editing the compiler invalidates the corpus exactly like editing a
model does. See [rspice-veriloga's README][gen] for the full generation
contract.

[gen]: ../rspice-veriloga/README.md#generating-the-built-in-device-models

## Shape

Each model is its **own Cargo package** under `models/<name>/`, depending only
on [`rspice-veriloga-runtime`](../rspice-veriloga-runtime) for its ABI. This
crate is the aggregator: `src/registry.rs` is the feature-gated catalog, and
`src/lib.rs` re-exports each enabled model's device module.

The split is not cosmetic. Because the leaves stop at the runtime ABI crate,
Cargo compiles and caches all 43 independently instead of folding them into one
`rspice-core` translation unit, which cut peak single-`rustc` memory for the
full corpus from 9.96 GB to 2.59 GB, back under the 3 GB build gate.

## Selecting models

`default = []`: nothing is compiled unless asked for. Reach the catalog through
`rspice-core`'s features rather than depending on this crate directly:

| Feature | Effect |
| :--- | :--- |
| `rspice-core/veriloga-model-<name>` | One model. Prefer this for small binaries and fast builds. |
| `rspice-core/veriloga-builtins-models` | Every model, no noise schedules. |
| `rspice-core/veriloga-builtins-noise` | The generated noise schedules, independently selectable because they are more than half of the checked-in Rust source. |
| `rspice-core/veriloga-builtins` | Both of the above: the historical all-model, noise-enabled behaviour. |

Building the complete corpus is a heavyweight operation: 43 packages over 215
generated Rust files and ~33 MB of source. Select the models a build actually
needs.

## Testing

```text
cargo test -p rspice-veriloga-models --features veriloga-model-vbic13
```

Correctness of the generated code is not established here. It is established by
the oracle and golden suites in
[`rspice-conformance`](../rspice-conformance), which run the generated models
against reference simulators.

Licensed under the [RSpice Personal Use License](../../LICENSE).

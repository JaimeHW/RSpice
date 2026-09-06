# rspice-veriloga-runtime

The stable, engine-neutral runtime ABI that every precompiled Verilog-A model
calls. Generated model crates depend on this and nothing else.

## Why it is a separate crate

The dependency edges run `rspice-core` to
[`rspice-veriloga-models`](../rspice-veriloga-models) to the 43 generated
model crates, and each of those leaves depends on exactly this crate for its
ABI. **Moving these types into `rspice-core`, or adding a
`rspice-core` dependency here, closes that path into a cycle.**

That separation is also what buys the build headroom. Because the leaves stop
here, Cargo compiles and caches all 43 independently instead of folding them
into one `rspice-core` translation unit. Measured 2026-08-01, the split cut peak
single-`rustc` memory for the full corpus from 9.96 GB to 2.59 GB, back under
the 3 GB build gate.

## What the ABI covers

- **Descriptors**: `GeneratedVerilogAModelDescriptor` with its terminal,
  parameter, and bound descriptors, versioned by
  `GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION` (currently 3).
- **Parameter handling**: origin tracking, alias installation, index lookup,
  and the `validate_generated_parameter*` family that enforces declared bounds
  with inclusive/exclusive flags.
- **Derivative lanes**: `Lanes<N>`, the packed derivative representation
  generated code stamps through. One-lane values emit as plain `f64`; widths of
  two and above stay packed, avoiding both scalar source explosion and
  one-element array overhead.
- **Analog operators**: `rspice_eval_ddt` and `rspice_eval_idt` with their
  accepted-history and candidate types, and `GENERATED_DDT_TIMESTEP_FLOOR`.
- **Limited exponentials**: `rspice_limexp` and `rspice_limited_exp` with the
  thresholds and floors that keep an overflowing bias excursion recoverable
  instead of infinite.
- **Compatibility**: `GENERATED_VERILOGA_COMPATIBILITY_CATALOG` records each
  explicitly authenticated migration from an older generated format to the
  current split-provenance contract. Every target field and wire alias is
  exact; matching source text alone is never sufficient to claim a historical
  identity.

`GeneratedVerilogAAcceptedStateShapeIdentity` is what lets a persisted
checkpoint or schematic binding decide whether it can be reconstructed against
the model catalog a given build actually contains.

## Features

`faer-parallel` forwards to `rspice-matrix`. There is no `default`.

## Testing

```text
cargo test -p rspice-veriloga-runtime
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

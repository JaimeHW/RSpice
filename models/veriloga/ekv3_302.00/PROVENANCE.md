# EKV3 302.00 Provenance

Source: `VA-Models/code/ekv3/vacode` from
https://github.com/dwarning/VA-Models

Downloaded on 2026-06-25 from upstream commit
`ba3d04319aae2806962848bb712fd90501488d80`.

The upstream Verilog-A sources, the `ekv3_include/` headers, and the upstream
`LICENSE` are preserved in `vacode/`.

License stated by the upstream `LICENSE` file: Educational Community License,
Version 2.0 (ECL-2.0). The preamble above the ECL-2.0 grant restates a separate
License Agreement's copying restriction; the operative grant in the same file
is ECL-2.0.

RSpice integration status:

- Generator-ready. `ekv3.va` selects its mode from a `` `define `` block at the
  top of the file, and upstream ships it with `RF` defined, so discovery finds
  one module — `ekv3_rf`, four terminals and seven internal nodes. It is
  transpiled to `crates/rspice-veriloga-models/models/ekv3-rf/` and compiles in
  behind the `veriloga-model-ekv3-rf` feature.
- The other four modes (`ekv3_s`, `ekv3`, `ekv3_r4`, `ekv3_nqs`) would each
  need that `` `define `` block edited, which is why only the shipped one is
  generated: these sources are byte-for-byte upstream.
- `ekv3_oppoints.va` reports transconductances through `ddx`, which the Rust
  backend resolves from the automatic-differentiation lanes it already
  computes. The stamped currents and charges do not go through `ddx` at all.

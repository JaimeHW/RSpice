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

- Not generator-ready: the canonical Verilog-A parser rejects the source before
  Rust transpilation; enablement is planned work.
- `.rspice-veriloga-skip` in this folder keeps builtin generation from scanning,
  fingerprinting, transpiling, or compiling these sources. Remove it only once
  the parser accepts the source and the model has passed simulator validation.

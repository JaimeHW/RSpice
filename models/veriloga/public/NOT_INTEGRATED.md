# Public Sources Not Integrated Yet

This list covers public sources downloaded under `models/veriloga/public` that
are not wired into generated RSpice builtins.

## Integrated From This Archive

- `ekv26/model`: promoted to `models/veriloga/ekv26_2.6`; generated as
  `ekv_va`; routed from `.model ... NMOS/PMOS LEVEL=260`.
- `va_models/bsimsoi_4.6.1`: promoted to `models/veriloga/bsimsoi_4.6.1`;
  generated as `bsimsoi_va`; routed from legacy BSIM-SOI
  `.model ... LEVEL=10/55/56/57`.

## Not Integrated

- `cogenda/VA-BSIM3v3`: public, but upstream README states `CC-BY-NC 4.0`.
  This is not acceptable for commercial RSpice shipping without a separate
  commercial license or replacement source.
- `cogenda/VA-BSIM48`: public, but upstream README states `CC-BY-NC 4.0`.
  This is not acceptable for commercial RSpice shipping without a separate
  commercial license or replacement source.
- `va_models/bsim4_4.8.0`: adapted from Cogenda VA-BSIM48 and its header
  states `CC-BY-NC 4.0`; not acceptable for commercial RSpice shipping without
  a separate commercial license or replacement source.
- `va_models/ekv3_302.00`: ECL-2.0 candidate, but not generator-ready. The
  current canonical parser rejects the source before Rust transpilation.
- `va_models/epfl_hemt_3.0.0`: public HEMT source with redistribution terms,
  but not generator-ready. The current Rust backend rejects a `Pow` operator in
  the model.
- `va_models/angelov_2.0`: public Angelov/Angelov-GaN HEMT sources. The main
  model files use an MIT header, but they include `compact.vams`, whose header
  refers to a separate license agreement. They are also not generator-ready yet:
  the current Rust backend rejects branch-current access `I(...)` in expressions.
- `va_models/vbic_1.3`: not generator-ready because it uses the Verilog-A
  `analysis()` function, which the Rust backend does not support yet. Its
  copied source also does not carry clear enough commercial redistribution
  terms to treat as production-ready.
- VDMOS: no exact public Verilog-A source was retrieved.
- Native JFET/MESFET/HFET replacements: public HEMT/HFET-class sources do exist
  in VA-Models and CMC sources. RSpice already generates and routes the active
  CMC ASM-HEMT and MVSG models by explicit model family. They are not currently
  treated as exact drop-in replacements for RSpice's classic native
  JFET/MESFET/HFET directives.
- VADistiller/VACASK: public material suggests converted SPICE3 models may
  exist for JFET/MESFET/VDMOS, but Codeberg returned 504 errors during retrieval
  on 2026-06-25.

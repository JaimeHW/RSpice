# Public Verilog-A Source Candidates

This directory contains public third-party Verilog-A compact-model sources that are
useful candidates for replacing older native RSpice model implementations.

The directory is intentionally marked with `.rspice-veriloga-skip`, so the RSpice
builtin generation path does not scan, fingerprint, transpile, or compile these
sources yet. Do not remove that marker until a model has passed licensing review,
transpiler compatibility work, and simulator validation.

Downloaded on 2026-06-25.

See `NOT_INTEGRATED.md` for the current integration status and blockers for
sources that remain quarantined here.

## Contents

- `va_models/`: selected sources from `dwarning/VA-Models` at commit
  `ba3d04319aae2806962848bb712fd90501488d80`.
- `cogenda/VA-BSIM3v3/`: Cogenda BSIM3v3 Verilog-A source at commit
  `e16cbec5e77bc7bad1d29b8fd392860bdc180a19`.
- `cogenda/VA-BSIM48/`: Cogenda BSIM4.8 Verilog-A source at commit
  `0a854e0064e10b6621deff8a50f0c954e337234b`.
- `ekv26/model/`: official FOSS EKV2.6 Verilog-A source at commit
  `137e7779c66d282113ce5802843f546869cb8c05`.

## Commercial-Licensing Notes

Public availability is not the same thing as production eligibility for a
commercial simulator. In particular, Cogenda BSIM3v3 and BSIM4.8 are marked
CC-BY-NC 4.0 and must not be shipped in a commercial RSpice build without a
separate commercial license or replacement source.

The current generated source tree already includes newer commercial-candidate
CMC families such as BSIM-BULK, BSIM-CMG, BSIM-IMG, BSIM-SOI, ASM-HEMT, MVSG,
and others. Commercial-ready candidates promoted from this archive are copied
into unskipped sibling folders under `models/veriloga`.

## Coverage Notes

- VBIC 1.3, BSIM-SOI 4.6.1, EKV3 302.00, EPFL-HEMT 3.0.0,
  Angelov/Angelov-GaN 2.0, and a VA-Models-adapted BSIM4 4.8.0 copy are under
  `va_models/`.
- EKV2.6 has been promoted to `models/veriloga/ekv26_2.6`.
- BSIM-SOI 4.6.1 has been promoted to `models/veriloga/bsimsoi_4.6.1`.
- BSIM3v3 is available from Cogenda, but the public source found is
  non-commercially licensed.
- No exact public Verilog-A replacement was added for native VDMOS.
- Public HEMT/HFET-class sources do exist. Existing CMC ASM-HEMT and MVSG
  sources are already generated and explicitly routed, while EPFL-HEMT and
  Angelov remain public candidates. These are not currently treated as exact
  replacements for RSpice's classic native JFET/MESFET/HFET directives.
- VADistiller/VACASK appears to have converted classic SPICE3 models, including
  JFET/MESFET/VDMOS in public presentations, but Codeberg returned 504 errors
  during retrieval on 2026-06-25.

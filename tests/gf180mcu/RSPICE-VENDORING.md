# RSpice GF180MCU Vendoring Notes

This directory vendors GlobalFoundries' open GF180MCU 180nm PDK model library
together with a normalized device-characterisation corpus derived from the
PDK's own `models/ngspice/testing` tree. Run by
`crates/rspice-conformance/src/suites/gf180mcu.rs`.

## Source

- `models/design.ngspice` and `models/sm141064.ngspice` copied verbatim from
  <https://github.com/google/globalfoundries-pdk-libs-gf180mcu_fd_pr>
  (`models/ngspice/`), sparse-checked-out on 2026-07-28.
- Cases derived from that repository's
  `models/ngspice/testing/regression/{diode,mos_iv_vgs}` netlist templates and
  `180MCU_SPICE_DATA` device tables.

## License

Apache-2.0, reproduced in `LICENSE`. This is the first vendored corpus RSpice
may redistribute without qualification — `tests/ngspice/` and `tests/xyce/`
are copyleft and `tests/iscas85/` has no stated terms at all. Preserve the
upstream copyright headers inside the model files.

## What was normalized, and why nothing upstream is retained

Upstream drives its comparison from Python: `models_regression.py` per device
group, rendering Jinja2 templates, shelling out to ngspice, and reducing the
result with pandas. None of that is vendored. The vendoring step ran once and
emitted plain text:

```
cases/<group>/<case>.spice   a concrete, self-contained deck
cases/<group>/<case>.tsv     the reference curve, two tab-separated columns
device-manifest.tsv          per-case contract
```

Every deck states its own analysis with a `.dc` card instead of a `.control`
block, so it can be run directly by RSpice, by ngspice, or by hand. Two
further departures from the upstream templates are deliberate:

- **The bias sweep comes from the device's own characterisation range.**
  Upstream's `iv.spice` hardcodes `dc Vn -12.13 1.13 0.13` for every diode,
  but the devices were not characterised over a common range — `dnwps` was
  swept `-40.41 .. 1.41`. Simulating one range and comparing it point-by-point
  against another is what produces upstream's implausible error figures, and
  is presumably why its report is printed rather than asserted.
- **Each gate bias is its own MOS case.** Upstream sweeps six gate biases in
  one two-source run. Split, a failure names one bias condition rather than a
  family, and the comparison cannot depend on how a simulator orders a nested
  sweep.
- **Statistical mismatch is switched off.** Every MOS deck ends with
  `.param sw_stat_mismatch=0` / `sw_stat_global=0`, placed *after* the `.lib`
  line because `design.ngspice` would otherwise overwrite them. See the
  section below for why this is not optional.

## ⚠ These decks are not reproducible with the PDK's default switches

`design.ngspice` ships `sw_stat_global=1` and `sw_stat_mismatch=1`, and the
device wrappers in `sm141064.ngspice` use them:

```
.param var_vth='0.7071*par_vth*1e-06/p_sqrtarea'
.param mis_vth=agauss(0,var_vth,1)
m0 d g s b nmos_3p3 ... delvto='mis_vth*sw_stat_mismatch' ...
```

So every instance gets a **random threshold-voltage offset**. For a
1 µm × 0.5 µm device that is σ ≈ 8 mV, which in subthreshold is ±19% of drain
current — larger than any tolerance worth setting. ngspice does not fix its
seed, so it answers differently on consecutive runs of the same deck:

```
$ ngspice -b nmos_3p3_W1_L0.5_T25_Vg0.8   ->  I(0.35 V) = 2.72324e-06
$ ngspice -b nmos_3p3_W1_L0.5_T25_Vg0.8   ->  I(0.35 V) = 2.47355e-06
```

A reference captured that way is one random draw that nothing can reproduce,
including the tool that produced it. Turning the switches off makes both
simulators deterministic — ngspice then returns `2.51440e-06` every time — and
only then does a sub-percent comparison mean anything. This is also what a
device-characterisation flow does: mismatch belongs in a Monte-Carlo analysis,
not in a nominal I-V curve.

## ⚠ The reference is ngspice, not silicon

The upstream tree presents `180MCU_SPICE_DATA/**.nl_out.xlsx` as measured
data, and this corpus was first built on that reading. **It does not hold.**
The value columns are named after corner libraries — `diode_typical`,
`diode_ff`, `diode_ss` — and reproduce ngspice's answer on the same deck to
four significant figures through the reverse-bias region. They are simulation
output captured per corner, not wafer measurements. Upstream never asserts on
them, which is consistent with the distinction never having mattered there.

So the vendored `.tsv` references are **ngspice-46's output on the exact deck
beside them**, captured once at vendoring time — the same arrangement
`tests/ngspice/` uses for its `.out` files. That is a weaker claim than
silicon conformance and a much tighter numeric gate: both sides evaluate the
same model card, so agreement should be sub-percent, and it is.

If real GF180MCU silicon data becomes available, the columns to look for are
not the ones this corpus used.

## What is covered

| Group | Cases | What it exercises |
| --- | --- | --- |
| `mos_iv_vds` | 612 | BSIM4 (LEVEL=54) through the PDK's subcircuit wrappers: 5 device flavours × 4 geometries × 4 temperatures × 6 gate biases |
| `diode_iv` | 216 | 9 diode types × 3 corners × 2 geometries × 4 temperatures |

## Known gaps, recorded in `device-manifest.tsv`

The gap list is empty: both groups assert against their reference curve.

- **Diode LEVEL=3 is implemented.** All 216 diode cases agree with ngspice-46
  to under 1%. What this corpus forced into `rspice-core` is recorded under
  "Ingestion fixes" below.
- **C-V measures are not covered.** Upstream probes device internal
  capacitance with ngspice's `@dn[cd]` device-parameter form, which this
  suite does not express. The `mos_cv`, `moscap_c`, `mimcap_c`, `bjt_cj`
  groups are untouched for the same reason.
- **`resistor_r`, `bjt_iv`, `bjt_beta`, `mos_iv_vbs` are not yet derived.**
  Nothing blocks them; they were scoped out to land the two groups above
  properly. Each is a spreadsheet decode plus a template render.

## Ingestion fixes this corpus forced

A released Apache-2.0 foundry PDK could not be loaded at all before this
corpus was wired in. Three defects in `rspice-core`, each verified against
ngspice-46 before changing anything:

1. `sm141064.ngspice` opens `.LIB dio` and closes it `.endl diode`. Every
   other simulator ignores the label; RSpice rejected the file.
2. `.LIB typical` pulls in the MOS model cards *before* the `.LIB
   noise_corner` section defining the flicker-noise parameters they cite, so
   a single forward parse pass cannot resolve them.
3. Instance parameter lists are written `W = 10u L = 0.28u`; RSpice read the
   first `=` one field too late and concluded the subcircuit was named `W`.

And then the diode model itself, which RSpice rejected outright. ngspice
LEVEL=3 is the same `dio` device as LEVEL=1 — only three behaviours branch on
the level (W/L-derived AREA and PJ, area-scaled breakdown knee current, and
metal/poly overlap capacitance). The reason a foundry card needs it is the
*parameters*, which are level-independent in ngspice and were simply missing:

- **Band-to-band tunneling (JTUN, JTUNSW, NTUN, XTITUN, KEG).** Not a
  correction — it is the dominant reverse mechanism here. On `np_3p3` at
  −12.13 V, ngspice-46 gives 1.61 A with tunneling and 0.19 A without.
- **The TLEV/TLEVC temperature families** (TCV, TPB, TPHP, CTA, CTP,
  TRS1/TRS2, TM1/TM2, TTT1/TTT2, GAP1/GAP2). Every card here sets
  `tlev=1 tlevc=1`.
- **Aliases** PB, MJ, TREF, ISW, IB, NZ, VRB/VAR, CTC, TVJ, and model-card
  AREA/PJ defaults.

Two reference behaviours are reproduced deliberately rather than corrected,
both documented at their implementation sites in
`crates/rspice-core/src/device/semiconductor/diode.rs`:

- `BV=0` means *breakdown matched to a negative voltage*, not *no breakdown*.
  `nwp_3p3` relies on it — 5.65 A at −12.13 V against 81 µA with BV omitted.
- A sidewall without its own NS contributes nothing in the breakdown region,
  because `dioload.c` evaluates it from `vdsw`, which is only assigned when
  the model gives RSW. The intent was plainly the common voltage, but the
  PDK's JSW and BV were *extracted against this implementation*, so applying
  the intended equation would overstate the knee by `(IS + JSW·PJ)/IS` —
  3.6x on the high-perimeter geometries in this corpus.

Xyce 7.10 disagrees with ngspice on both the sidewall merge and the
breakdown-matching basis, and has no LEVEL=3 at all, so the diode carries an
explicit dialect flag rather than a compromise between them.

Then the MOS side, which this corpus is the only local exercise of:

4. **Model-bin bounds are inclusive at both ends, within 1 nm.** RSpice used
   `min <= W < max` compared exactly. `W=0.22u` is `0.22 × 1e-6`, one ULP
   below the card's `wmin = 2.2e-007`, so the PDK's minimum-width devices
   matched no bin and the family failed to resolve at all — 60 cases.
5. **Bin ties take the first match in declaration order.** Inclusive bounds
   make adjacent bins overlap on their shared edge, and ngspice returns the
   first bin whose ranges contain the geometry, which for an ascending foundry
   table is the lower one. RSpice picked by smallest bin range instead, giving
   a different parameter set to every device with `W` or `L` on a boundary.
6. **A rebuild must redraw the same statistical values.** Subcircuit `.param`s
   calling `agauss` are evaluated during flattening, and the draw counter was
   shared across builds — so simulating one netlist twice in a process gave
   two different circuits. Now each build rewinds the stream.

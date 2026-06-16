# 2 · Netlists

RSpice reads SPICE decks in the ngspice dialect, plus the HSPICE
multi-run constructs (`.alter`, `.data`) and Spectre-style SPEF
inclusion. Syntax is case-insensitive. The **first line is the title**;
the deck ends at `.end`. A leading `+` continues the previous line;
`*` starts a comment line; `;` and `$` start end-of-line comments.

## Devices

The first letter of an element name selects the device:

| Letter | Device | Example |
|---|---|---|
| `R` | Resistor (value, model-based, or expression) | `R1 a b 4.7k` · `R2 a b RMOD L=10u W=2u` |
| `C` | Capacitor (`IC=` initial voltage) | `C1 out 0 1u IC=0` |
| `L` | Inductor (`IC=` initial current; Jiles-Atherton core via model) | `L1 a b 10u` |
| `K` | Inductor coupling / transformer | `K1 L1 L2 0.98` |
| `V` | Voltage source | `V1 in 0 DC 1 AC 1 SIN(0 1 1k)` |
| `I` | Current source | `I1 0 n1 PULSE(0 1m 0 1n 1n 1u 2u)` |
| `E` | VCVS (linear, `POLY(n)`, `VALUE=`, `TABLE`, `LAPLACE`) | `E1 y 0 a 0 10` |
| `G` | VCCS (same extended forms) | `G1 y 0 a 0 1m` |
| `F` | CCCS (linear, `POLY(n)`) | `F1 y 0 V1 2` |
| `H` | CCVS (linear, `POLY(n)`) | `H1 y 0 V1 1k` |
| `B` | Behavioral source (`V=expr` / `I=expr`) | `B1 y 0 V=v(a)*v(a)` |
| `D` | Diode | `D1 a k DMOD` |
| `Q` | BJT (Gummel-Poon, VBIC via model level) | `Q1 c b e QMOD` |
| `M` | MOSFET (native BSIM4/BSIM3/B3SOI plus classic and opt-in fallback levels) | `M1 d g s b NCH W=10u L=1u` |
| `J` | JFET level 1; `LEVEL=2` warns and falls back to level 1 | `J1 d g s JMOD` |
| `Z` | MES/MESA/HFET-family devices | `Z1 d g s ZMOD` |
| `S` / `W` | Voltage-/current-controlled switch | `S1 a b c d SWMOD` |
| `T` | Ideal transmission line | `T1 a 0 b 0 Z0=50 TD=1n` |
| `O` | Lossy line (LTRA) | `O1 a 0 b 0 LMOD` |
| `Y` | Lossy line (TXL) | `Y1 a 0 b 0 YMOD LEN=2` |
| `P` | Coupled lines (CPL) | `P1 a1 a2 0 b1 b2 0 PMOD LEN=1` |
| `X` | Subcircuit instance (`PARAMS:` overrides) | `XA in out amp gain=20` |
| `A` | XSPICE code-model instance | `A1 [in] out gate_model` |

Instance multiplicity `m=` and `AREA` scaling compose hierarchically
through subcircuits.

Controlled-source compatibility is intentionally bounded: voltage-controlled
`E`/`G` sources support linear, `POLY`, `VALUE`, `TABLE`, and `LAPLACE`
forms; `FREQ` is recognized and rejected as unsupported. Current-controlled
`F`/`H` sources support linear and `POLY` forms only.

MOS model routing is explicit. Native production paths cover BSIM4 v4.8
(`LEVEL=14/54`, canonical mode set), BSIM3v3.3 (`LEVEL=8/49`), BSIM3-SOI
FD/DD/PD (`LEVEL=55/56/57`), Berkeley MOS1/MOS2/MOS6, legacy BSIM1/BSIM2,
EKV, and VDMOS. Unsupported BSIM4 mode selectors such as external
source/drain resistance networks, gate/body resistance networks, NQS,
non-default charge paths, gate tunneling, WPE/stress, and unsupported
geometry modes fail with typed errors rather than being silently simplified.
Other non-native bulk-MOS levels require `.options allow_simplified_mos=1`
and use the documented simplified fallback.

## Sources

Transient source functions: `PULSE`, `SIN`, `EXP`, `PWL`, `SFFM`, `AM`,
`TRNOISE` (white Gaussian + 1/f). DC and `AC mag [phase]` specifications
combine with any transient function. `TRNOISE` RTS-tail parameters and
`TRRANDOM` are not implemented; decks requesting those forms should fail
explicitly instead of being treated as supported noise sources.

## Parameters and expressions

```spice
.param vdd=3.3 rl={vdd*1k}
R1 out 0 {rl}
.func db(x) {20*log10(x)}
```

- `.param` assignments evaluate **in deck order**; later assignments
  override earlier ones.
- `{…}` (or quoted) expressions may use parameters, arithmetic, the
  standard function set, and the statistical functions `gauss`,
  `agauss`, `unif`, `aunif`, `flat`, `limit` (driven by per-run RNG
  streams under Monte Carlo).
- `.csparam` behaves as `.param`.

## Hierarchy

```spice
.subckt amp in out params: gain=10
E1 out 0 in 0 {gain}
.ends
XA a b amp gain=40
```

Nested subcircuits, local parameter scopes, and `.global` nodes are
supported. `.if` / `.elseif` / `.else` / `.endif` gate deck regions on
parameter expressions (PDK cards use this heavily).

## Dot-command summary

| Command | Purpose | Chapter |
|---|---|---|
| `.op` `.dc` `.ac` `.tran` `.noise` `.disto` `.pz` `.sens` `.tf` `.four` | Analyses | [3](03-analyses.md) |
| `.step` `.mc` `.temp` `.alter` `.data`/`.enddata` | Multi-run | [4](04-multi-run.md) |
| `.model` `.subckt`/`.ends` `.global` `.param`/`.csparam` `.func` | Definitions | this chapter |
| `.ic` `.nodeset` | Initial conditions / DC hints | [3](03-analyses.md) |
| `.include`/`.inc` `.lib` | File inclusion (search paths: `-I`) | this chapter |
| `.spef_include` | Post-layout parasitics | [5](05-post-layout.md) |
| `.veriloga` | Verilog-A compact model | `rspice compile-va --help` |
| `.options` | Simulator options (`reltol`, `abstol`, `temp`, `seed`, …) | [3](03-analyses.md) |
| `.meas`/`.measure` | Post-run measurements | [6](06-measurements.md) |
| `.save` `.probe` `.print` `.plot` | Output selection | [6](06-measurements.md) |
| `.if`/`.elseif`/`.else`/`.endif` | Conditional deck regions | this chapter |
| `.end` | End of deck | — |

Unknown dot-commands are ignored with a log entry rather than failing
the parse.

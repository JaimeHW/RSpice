# External device-stamp references

This study estimates compact-model evaluation plus stamping cost in ngspice
and Xyce. It provides architectural context for `rspice-bench generated-stamp`;
it is not accepted by a native RSpice regression gate.

## Method

Each deck drives every terminal of one device with an independent source and
sweeps roughly 30,000 DC points. A paired no-device deck replaces the model
with a 1 Gohm dummy resistor. Subtracting the paired result removes source,
matrix, and simulator-loop overhead before normalizing by iterations.

- ngspice supplies matrix-load time and total iterations through `.options
  acct`; matrix load covers device evaluation and stamping.
- Xyce supplies residual-load time, Jacobian-load time, and Jacobian count. The
  two load times are summed to match one RSpice evaluation/stamp operation.

`decks/` contains the study inputs. `anchors/` contains an independently
maintained focused comparison for hand-written ngspice BSIM4/VBIC versus
RSpice's handwritten BSIM4. Read `anchors/README.md` before reproducing it.

## Recorded results (2026-07-27)

Per device evaluation plus stamp after paired-baseline subtraction:

| Model | ngspice (C) | Xyce (Sacado AD) | Historical RSpice generated result |
| :--- | ---: | ---: | ---: |
| VBIC 1.3, four terminals | 859 ns | 2,658 ns | 1,501 ns (retired scalar emitter) |
| HiSIM-HV2, default configuration | 1,169 ns | unavailable | unavailable |
| HiSIM-HV2, internal nodes enabled | 1,694 ns | unavailable | 10,465 ns (retired sparse-local emitter) |
| HiSIM-HV2, internal nodes enabled | 1,694 ns | unavailable | 44,000 ns (retired indexed-workspace emitter) |

These figures are preserved research evidence, not current release claims. The
original records do not carry the complete host, executable digest, raw-sample,
and immutable-schema provenance required by the current benchmark policy.

Important comparability limits:

- ngspice uses HiSIM-HV2 2.2 while the recorded RSpice model is 2.5.1.
- ngspice conditionally creates internal nodes; the historical RSpice topology
  was fixed at 19 nodes and 13 branches.
- Every reference is single-threaded and single-device, so it says nothing
  about instance scaling or full-circuit time.

## Reproduction

Run from this directory with explicitly versioned release binaries and retain
their version output, executable hashes, complete stdout/stderr, host identity,
and raw results alongside any new analysis:

```text
ngspice_con.exe -b decks/vbic_dense.cir
ngspice_con.exe -b decks/vbic_nodev.cir
ngspice_con.exe -b decks/hv_dense.cir
ngspice_con.exe -b decks/hv_full.cir
ngspice_con.exe -b decks/hv_nodev.cir
Xyce.exe decks/vbic_xyce.cir
Xyce.exe decks/vbic_xyce_nodev.cir
```

`hv_dense.cir` carries the ngspice `tests/hisimhv2/nmos` parameter set;
`hv_full.cir` enables its internal-node configuration flags. Do not overwrite
the historical records in place. Add a dated, reviewed study version if this
evidence is refreshed.

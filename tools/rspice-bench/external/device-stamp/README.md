# External device-stamp comparisons

These workloads estimate compact-model evaluation and stamping cost in
ngspice and Xyce. They provide context for `rspice-bench generated-stamp`;
they are not inputs to a native RSpice regression gate.

## Method and limits

Each deck drives device terminals with independent sources and sweeps DC
points. A paired no-device deck uses dummy resistors to measure simulator,
source, and matrix overhead.

ngspice reports matrix-load time and total iterations through `.options acct`.
Xyce reports residual-load time, Jacobian-load time, and Jacobian count.
Normalize each side by its own iteration count before comparing device costs.

Model versions, internal-node configurations, matrix representations, and
terminal topology differ between simulators. Single-device measurements do
not establish instance scaling or full-circuit performance. Do not present
cross-simulator ratios as equivalent-model latency without establishing those
conditions.

## Reproduction

Run from this directory using explicitly selected release binaries:

```sh
ngspice_con.exe -b decks/vbic_dense.cir
ngspice_con.exe -b decks/vbic_nodev.cir
ngspice_con.exe -b decks/hv_dense.cir
ngspice_con.exe -b decks/hv_full.cir
ngspice_con.exe -b decks/hv_nodev.cir
Xyce.exe decks/vbic_xyce.cir
Xyce.exe decks/vbic_xyce_nodev.cir
```

`hv_dense.cir` uses the ngspice `tests/hisimhv2/nmos` parameter set;
`hv_full.cir` enables its internal-node configuration flags.

The [anchor script](anchors/README.md) automates a repeated, paired comparison
for ngspice BSIM4 and VBIC. Retain raw observations and complete provenance in
ignored output or CI artifacts. Historical timing tables and reports remain
in Git history.

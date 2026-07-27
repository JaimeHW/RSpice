# Device-evaluation reference baselines

What a compact-model evaluation costs in other simulators, so that RSpice's own
`rspice-bench generated-stamp` numbers can be judged against something rather
than tracked in isolation.

Two references are measured, and they answer different questions:

- **ngspice** — hand-written C with hand-derived analytic Jacobians. This is the
  performance ceiling a generated backend is trying to reach.
- **Xyce** — C++ model equations differentiated by Sacado. This is the closer
  architectural analogue to RSpice, which also generates derivative code rather
  than having a human write it, so it is the more honest peer comparison.

## Method

Isolating device evaluation from everything else is done by subtraction, not by
instrumenting either simulator:

1. A deck holds **one** device with **every terminal driven by its own source**,
   so no terminal is grounded away and the Jacobian block stays dense. That is
   the same worst case `generated-stamp` uses, and the same case across
   revisions.
2. A DC sweep of ~30k points keeps the matrix tiny (≈10–18 equations) so device
   evaluation dominates and the solver does not.
3. The identical deck is run with the device deleted and replaced by a 1 GΩ
   dummy resistor, giving the fixed per-iteration overhead of the sources, the
   matrix, and the simulator's own loop.
4. Device cost is the difference, divided by iterations.

Both simulators report the numbers this needs directly, so nothing is inferred
from wall-clock:

- ngspice `.options acct` → `Matrix load time` and `Total iterations`. Its
  "matrix load" *is* device evaluation plus stamping.
- Xyce → `Total Residual Load Time` + `Total Jacobian Load Time`, normalized by
  `Number Jacobians Evaluated`. Xyce loads residual and Jacobian separately, so
  both are summed to match what one RSpice `stamp()` call does.

## Results (2026-07-27)

Per device evaluation + stamp, after subtracting the no-device baseline.

| Model | ngspice (C) | Xyce (Sacado AD) | RSpice generated |
|---|---|---|---|
| VBIC 1.3, 4 terminals | **859 ns** | 2,658 ns | **1,501 ns** (`ScalarOptIr`) |
| HiSIM-HV2, default config (10 eq) | **1,169 ns** | — | — |
| HiSIM-HV2, internal nodes on (16 eq) | **1,694 ns** | — | 10,465 ns (`SparseLocalKernel`) |
| | | | 44,000 ns (`StructuredKernel`) |

Reading these:

- On VBIC, RSpice's scalar tier is **1.75× hand-written C and 1.8× faster than
  Xyce**. Generating derivative code is not itself the problem — that tier is
  already competitive with a shipping commercial AD-based simulator.
- On HiSIM-HV the same backend is **6× C** at best and **26× C** on the tier it
  actually falls back to. The gap is the tier, not the approach.

## Caveats that matter when quoting these

- **ngspice's HiSIM-HV2 is version 2.2; RSpice's is 2.5.1.** Same family and
  comparable complexity, not identical equations.
- **Topology is not matched exactly.** ngspice creates internal nodes only when
  the `CO*` configuration flags ask for them — 10 circuit equations by default,
  16 with `CORSRD/CORG/CONQS/COSELFHEAT/COSUBNODE/CORBNET` enabled. RSpice's
  generated device carries a fixed 19-node/13-branch topology regardless, so it
  is always paying for the densest configuration. Some of the HiSIM-HV gap is
  this, not code quality; collapsing unused internal nodes is a separate win
  from anything in the codegen.
- Both references are single-threaded, single-device. Neither says anything
  about how the simulators scale across instances.

## Reproducing

```
ngspice_con.exe -b decks/vbic_dense.cir      # and vbic_nodev.cir
ngspice_con.exe -b decks/hv_dense.cir        # and hv_full.cir, hv_nodev.cir
Xyce.exe decks/vbic_xyce.cir                 # and vbic_xyce_nodev.cir
cargo run -p rspice-bench --release --features generated-stamp -- generated-stamp
```

`hv_dense.cir` carries the HiSIM-HV parameter set from ngspice's own
`tests/hisimhv2/nmos` converted to a `.model` card; `hv_full.cir` is that same
card with the internal-node flags turned on.

## Lowering probe

`lowering-probe/` answers the question Phase 1 of the backend rewrite rests on:
whether LLVM promotes a fixed-width `[f64; L]` derivative local to registers, so
that a compact `array::from_fn` chain rule compiles to the same straight-line
FMAs the flattened emitter writes out by hand. If it did not, generated source
would have to keep scaling by operations x lanes.

It times three forms of identical arithmetic: `workspace` (today's
`StructuredKernel` — indexed values behind a `&mut`, runtime-masked derivative
loop, `#[inline(never)]` per op), `array` (the proposal), and `flat` (today's
`SparseLocalKernel` — one named scalar per lane).

Result, 2026-07-27:

| Form | L=12 | L=32 |
|---|---|---|
| workspace | 9.819 ns/op | 23.753 ns/op |
| **array** | **3.692 ns/op** | **4.491 ns/op** |
| flat | 3.744 ns/op | — |

The array form matches the flattened form, beats the workspace form 2.7x at
L=12 and 5.3x at L=32, and scales sub-linearly in L — 8x the lanes costs 1.33x
the time, against 2.4x for the workspace form. Dense models therefore cost
little more than sparse ones, which is why the emitter uses one device-wide
lane width rather than per-value masks.

Every lane is summed into the returned value on purpose. An earlier revision
returned only lane 0, and LLVM dead-code-eliminated the rest: the array form
then reported an identical time for L=4 through L=32, which is the signature of
measuring nothing. If this probe is ever changed, keep all lanes live.

```
cd lowering-probe && cargo run --release
```

## Split probe

`split-probe/` answers the other question the packed lowering had open: whether
a compact model's worth of values can be emitted as one function, or has to be
cut into blocks the way the existing backends cut theirs into 483
`#[inline(never)]` pieces.

It generates a synthetic body at bsimbulk's scale — 12,028 values at lane width
17, with bsimbulk's own operator mix — either monolithic or split into functions
of a given size, each passing the live values across the boundary as arguments.

Result, 2026-07-27:

| block | source | rustc | ns/eval |
|---|---|---|---|
| **0 (monolithic)** | 1670 KB | 51.6 s | **25,233** |
| 500 | 1677 KB | 30.2 s | 61,681 |
| 2000 | 1672 KB | 34.4 s | 61,875 |

Splitting costs 2.4x at run time and buys about twenty seconds of compile time.
The cost is what it looks like: `[f64; 17]` arrays crossing a function boundary
are forced to memory, which is the round-tripping the packed form exists to
avoid. A monolithic 12,028-value function compiles in under a minute and does
not blow up, so the lowering does not split and needs no liveness partitioner.

Only the *ratio* is meaningful. The generated chain is fully serial — every
value depends on the one before it — so roughly half the absolute time is
dependency latency that a real model's DAG would overlap. Both variants carry
the identical chain, which is what makes comparing them fair; neither number
predicts what bsimbulk will do.

The operator mix is load-bearing. An earlier revision made every fourth
operation an `exp`, putting 3,007 transcendental calls in the body and measuring
libm throughput rather than the lowering. Keep transcendentals rare, as they are
in a compact model's inner graph.

```
cd split-probe && cargo run --release -- 12028 17 0 /tmp/mono.rs
rustc -O -o /tmp/mono.exe /tmp/mono.rs && /tmp/mono.exe
```

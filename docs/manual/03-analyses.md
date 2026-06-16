# 3 · Analyses

Every analysis card in the deck runs in order; with several analyses,
output files are tagged per analysis (`out.tran.csv`, `out.ac.csv`, …).
A deck with no analysis card runs a DC operating point.

## Core set (deck cards)

| Card | Meaning |
|---|---|
| `.op` | DC operating point. |
| `.dc SRC start stop step` | DC transfer sweep (nested sweep supported via a second source tuple). |
| `.ac dec\|oct\|lin N fstart fstop` | Small-signal frequency sweep. |
| `.tran tstep tstop [tstart [tmax]]` | Transient; `tstep` is the output interval, integration is variable-step (trap/Gear with LTE control). |
| `.noise v(out[,ref]) SRC dec\|oct\|lin N f0 f1` | Small-signal noise: output/input-referred spectra, per-contributor breakdown, band-integrated totals. |
| `.disto dec\|oct\|lin N f0 f1 [f2/f1]` | Compatibility card; currently runs the matching small-signal AC sweep and does not emit Volterra distortion products. |
| `.pz in+ in- out+ out- cur\|vol pol\|zer\|pz` | Pole-zero extraction. |
| `.sens v(out[,ref]) [AC sweep]` | DC or AC sensitivity. |
| `.tf v(out) SRC` / `.tf i(VSRC) SRC` | DC transfer function (gain, Rin, Rout). |
| `.four f0 v(out) …` | Fourier analysis of the last transient. |
| `.stb dec\|oct\|lin N fstart fstop probe=VNAME` | Loop stability: Tian double-injection loop gain at a 0 V probe source placed in series with the feedback path (both terminals off ground). Prints phase/gain margins; exports `loopgain` (complex), `loopgain_mag_db`, `loopgain_phase_deg` under the `stb` tag. |

Initial conditions: `.ic v(node)=value …` seeds the transient start;
`.nodeset` hints the DC solve. `.TRAN ... UIC` skips the operating point
and starts integration from the explicit `.IC` / element `IC=` state.

## Convergence aids

The DC solve escalates automatically: Newton with damping, then
gmin-stepping, then source-stepping/pseudo-transient continuation.
Transient steps that hit a stiff knife-edge engage a gmin-continuation
rescue and, if the step controller livelocks at the minimum timestep, a
breakpoint-style integration restart. Tuning knobs (`.options` or CLI):
`reltol`, `abstol`, `vntol`, `gmin`, `itl…`, `temp`, `trtol`, `method`
(`trap`/`gear`), plus `--convergence <preset>` on the CLI.

## Advanced / RF set

These run through CLI flags, the IDE's analysis setup, or engine APIs
(they are configured per run rather than as deck cards, except for `.stb`
in the core table above):

| Analysis | CLI / IDE |
|---|---|
| Periodic steady state (shooting) | `--pss-freq F [--pss-harmonics N] [--pss-tstab T]` |
| Harmonic balance (Krylov for ≥256 unknowns) | `--hb-freq F [--hb-harmonics N]` |
| S-parameters (Touchstone export) | `--sparam "P1+,P1-,P2+,P2-" [--sparam-z0 Z0]` over the deck's AC sweep, or IDE analysis setup |
| Monte Carlo | Operating-point parameter variation via `--monte-carlo N [--seed S]` or `.mc` card |
| Process corners | `--corners tt,ss,ff [--corner-lib FILE]` or IDE analysis setup |
| Pole-zero via nodes | `--pz-input N --pz-output M` |
| Sensitivity | `--sens-output N [--sens-param NAME]` |
| PAC / PXF / PSTB / periodic noise / envelope | IDE / engine-runner surface; not CLI flags or deck cards in the current release |

## Transient checkpointing

Long transients can be segmented:

```sh
rspice run long.cir --checkpoint seg1.ckpt          # runs to tstop, saves state
rspice run long2.cir --resume seg1.ckpt             # continues from the checkpoint
```

The checkpoint stores the exact reactive integrator state and is
fingerprint-guarded against a mismatched deck. `--resume` requires the
deck's `tstop` to exceed the checkpoint time, and combines with
`--checkpoint` to chain segments. TRNOISE decks regenerate their noise
train per segment — run those unsegmented when one continuous sample
path matters.

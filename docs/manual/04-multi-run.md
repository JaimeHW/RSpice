# 4 · Multi-run simulation

## `.step` — parametric sweep

```spice
.step param rload 500 2k 500
```

Re-runs the deck's analyses across the sweep; results carry the sweep
value per run. `.step temp …` sweeps temperature.

## `.temp`

```spice
.temp 27 85 125
```

One run per listed temperature.

## `.alter` — HSPICE sequential variants

```spice
* base deck …
.tran 1u 10m
.alter heavy load
.param rload=250
.alter low supply
V1 in 0 DC 2.5
.end
```

Each `.alter` block edits the deck **cumulatively** (HSPICE semantics —
block 2 sees block 1's changes) and re-runs everything:

- an element line replaces the element with the same name (its
  continuation lines go with it);
- `.model` cards replace the model with the same name;
- `.param` assignments override the existing assignment **in place**
  (preserving in-order evaluation);
- anything else appends.

The CLI prints one `=== run: <title> ===` banner per variant, tags
output files (`out.csv` → `out.heavy_load.csv`), and aggregates every
run into the JUnit/TAP/measurement reports. A failing variant doesn't
stop the remaining ones; the exit code reflects any failure.

## `.data` — table-driven sweeps

```spice
.param vdd=1 rl=1k
.data corners vdd rl
1.0 1k
1.8 2k
3.3 500
.enddata
.dc data=corners
```

Each row binds the listed parameters and re-runs. A bare
`.dc data=name` is one operating point per row; `.tran … sweep
data=name` / `.ac … sweep data=name` re-run that analysis per row.
`.alter` and `.data` compose (every variant runs the full table).

## Monte Carlo

```spice
.param r={agauss(1k, 50, 1)}
.mc 500 tran 1n 10u
```

or from the CLI: `rspice run deck.cir --monte-carlo 500 --seed 1337`.
Statistical expression functions (`gauss/agauss/unif/aunif`) draw from
per-run, seed-stable RNG streams, so a run is reproducible regardless of
machine. Results aggregate into per-variable statistics and histograms.

## Reports across runs

`--report-file out.xml` (JUnit) or `--report-format tap`, and
`--meas-file meas.json|csv`, cover **all** runs of a multi-run plan —
one entry per run, named `deck [variant]`.

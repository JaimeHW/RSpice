# 6 · Measurements and outputs

## `.meas`

```spice
.meas tran vmax   MAX  v(out)
.meas tran vavg   AVG  v(out) FROM=1u TO=9u
.meas tran trise  TRIG v(in)  VAL=0.5 RISE=1  TARG v(out) VAL=0.5 RISE=1
.meas tran vfind  FIND v(out) AT=2u
.meas tran vwhen  FIND v(out) WHEN v(in)=0.5
.meas ac   peak   MAX  v(out)
```

Measurement types: `MAX`, `MIN`, `PP`, `AVG`, `RMS`, `INTEG`,
`FIND … AT=`/`WHEN`, `TRIG…TARG` delay, `RISE`/`FALL` times. `.meas
tran|ac|dc` statements evaluate after the matching analysis. Signal lookup
is case-insensitive and accepts both `V(out)` and bare `out` spellings. For
AC, plain `V(out)` / `VM(out)` measure magnitude; `VDB(out)` measures dB
magnitude, `VP(out)` phase in degrees, `VR(out)` real, and `VI(out)`
imaginary. The AC sweep axis is available as `TIME`, `FREQUENCY`, or `FREQ`;
branch currents use the same `I*` variants.

Results print with `--meas`, and export for CI with
`--meas-file results.json` (or `.csv` via `--meas-format csv`). In
multi-run plans the report carries every run.

In the IDE, measurements feed the **specs matrix**: rows are `.meas`
results, columns are runs, bounds (min/max per measurement) persist with
the project, failing cells tint, and clicking a cell focuses that run.

## Output selection

`.save v(out) i(V1)` / `.probe` restrict what is recorded; `.print` /
`.plot` select tabular output signals. Without any, all node voltages
(and branch currents for sources/inductors) are recorded.

## Output files

`-o FILE -f FORMAT` writes waveforms. Formats: `raw` (ngspice
rawfile), `csv`, `json`, and the binary/compressed variants listed by
`rspice run --help`. `rspice convert` translates between formats after
the fact; `rspice compare` diffs a result against a golden file with
tolerances (regression testing).

# rspice-cli

The `rspice` command-line simulator. It runs SPICE decks and typed studies,
sweeps corners and Monte Carlo populations, and compares results against golden
references, over the same `rspice-core` engine as every other RSpice surface;
its exit status is a verification verdict that automation can branch on. This
document is the reference for the shipped binary: commands, flags, output
files, configuration and exit codes. `rspice <command> --help` lists the same
flags.

```bash
cargo build --release -p rspice-cli      # target/release/rspice
cargo install --path crates/rspice-cli   # puts rspice on PATH
```

## Commands

| Command | Purpose |
| :--- | :--- |
| `run` | Execute the analyses a netlist requests, or one requested on the command line |
| `study` | `check`, `plan` or `run` a typed JSON study with explicit task dependencies |
| `check` | Validate syntax, analysis requests, output symbols, topology and device construction without simulating |
| `info` | Summarize a netlist without simulating it |
| `models` | List the shipped SPICE model packs and look up parts in them |
| `compare` | Compare a result against a golden reference |
| `convert` | Convert a result between file formats |
| `compile-va` | Compile a Verilog-A model and report its interface |
| `health` | Probe liveness or numerical readiness |
| `capabilities` | Report this build's execution routes, control-script coverage, result mappings and unsupported workflows |
| `completions` | Print a shell completion script for `bash`, `zsh`, `fish`, `powershell` or `elvish` |

### Global options

| Option | Effect |
| :--- | :--- |
| `-v`, `--verbose` | More console detail, and debug logging unless `--log-level` is set |
| `-q`, `--quiet` | Suppress all non-error output |
| `--config FILE` | Use this configuration file instead of the user and project files |
| `--log-level LEVEL` | `off`, `error`, `warn`, `info`, `debug` or `trace`; overrides `--verbose` and `RUST_LOG` |
| `--log-format FORMAT` | `text` (default) or `json` log records on stderr |
| `--error-format FORMAT` | `text` (default) or `json` diagnostics on stderr; `json` also selects JSON logs |

### Netlist options

`run`, `check` and `info` share these options:

| Option | Effect |
| :--- | :--- |
| `-I`, `--include DIR` | Extra search directory for `.include` and `.lib` (repeatable) |
| `-D`, `--define NAME=VALUE` | Override or define a root-scope parameter (repeatable) |
| `--spice-dialect DIALECT` | Compatibility dialect, applied to parsing and simulation alike |
| `--redefined-params MODE` | What a `.PARAM` defined twice in one scope means |

| Dialect | Semantics |
| :--- | :--- |
| `best` | RSpice's most accurate available device evaluator, with ngspice expression syntax |
| `ngspice` | ngspice-compatible expression and device semantics |
| `xyce` | Xyce-compatible expression, measurement and device semantics |

`--redefined-params` follows Xyce: `ignore` or `usefirst` keeps the first
definition silently, `warning` (or `warn`) and `usefirstwarn` keep it with a
warning, `uselast` and `uselastwarn` keep the last definition silently or with
a warning, and `error` rejects the redefinition.

Pass `-` as the netlist path to read the deck from stdin; includes then resolve
against the working directory. `-D` values apply before conditional parsing and
expression evaluation, and subcircuit-local parameters keep their own scope.
Numeric flag values accept SPICE magnitude suffixes (`--pss-freq 2.4G`,
`--max-step 1u`, `-D RLOAD=4.7k`); `--temp` and `--sens-value` also take a
negative value as a separate word (`--temp -40`). A node named on the command
line is an authored node name, never a solver index.

## Running a deck

`rspice run DECK` parses the deck, expands its variants and run axes, executes
its analyses and publishes what they produce. Every `run` option is in the flag
reference at the end of this section.

### Analysis cards

Without a `.control` block, `run` executes every physical analysis card in
authored order and then the transient post-processors. A deck that authors no
analysis runs a DC operating point.

| Card | Executes |
| :--- | :--- |
| `.OP` | DC operating point |
| `.DC` | DC sweep, with an optional second source |
| `.TRAN`, `.TR` | Transient, including transient noise |
| `.AC`, `.NOISE` | Small-signal AC and noise over a generated sweep, or over the frequencies of a `.DATA` table with `DATA=<table>` |
| `.DISTO` | Harmonic or two-tone distortion |
| `.TF`, `.PZ`, `.SENS` | Transfer function, pole-zero, and DC or AC sensitivity |
| `.SP` | S-parameters over ports declared by `portnum=` source annotations or named on the card; `DONOISE` adds port noise |
| `.STB` | Loop stability |
| `.DCMATCH` | DC mismatch |
| `.PSS`, `.HB` | Periodic carriers: shooting steady state and harmonic balance |
| `.PAC`, `.PXF`, `.PNOISE` | Periodic AC, transfer function and noise around the nearest preceding `.PSS` or `.HB`; `FROM=` pins the family |
| `.PSTB` | Periodic stability around the nearest preceding `.PSS` |
| `.ENVELOPE` | Envelope following from the nearest preceding `.HB` |
| `.QPSS` | Quasi-periodic steady state |
| `.QPAC`, `.QPXF`, `.QPNOISE` | Quasi-periodic AC, transfer function and noise around the nearest preceding `.QPSS` |
| `.MC` | Monte Carlo |
| `.FOUR`, `.FOURIER` | Fourier analysis of the transient authored before the card, or of the first transient when none precedes it |
| `.FFT` | FFT of the deck's first transient |
| `.STEP`, `.TEMP` | Run axes: the other cards run at every coordinate |
| `.LIN SPARCALC=0` | Accepted; the deck's ordinary `.AC` runs. Every other `.LIN` form is refused |

`.AC`, `.NOISE`, `.TF`, `.SP`, `.STB`, `.PZ`, `.SENS` and `.DISTO` cards that
end in `ACTIMES=` or `CHECKPOINT=` linearize around a transient state instead
of the operating point.

The carrier a periodic or quasi-periodic card binds to is decided by the
canonical deck plan, the same binding the Python, browser and engine-adapter
surfaces use. A dependent card whose carrier is missing, such as `.PSTB` with no
preceding `.PSS` or `.ENVELOPE` with no preceding `.HB`, refuses the deck before
anything is solved. Each carrier is solved once, and every card bound to it
reuses its exact numerical state.

### Control scripts

With `.control` regions, including those reached through `.include` and `-I`,
`run` executes their commands in order instead of the declarative cards:

- **Analyses:** `op`, `dc`, `ac`, `noise`, `tran`, `tf`, `pz`, `sens`, `disto`,
  `stb` and `dcmatch`. `run` executes the deck's declarative cards, including
  the AC and noise `DATA=` forms; a card of any other kind, or a small-signal
  card carrying `ACTIMES=` or `CHECKPOINT=`, fails the command.
- **Circuit and options:** `alter`; `option` or `options` for `reltol`,
  `abstol`, `vntol`, `gmin`, `chgtol`, `eventfluxtol`, `trtol`, `xmu`, `method`,
  `itl1`, `itl2`, `itl4`, `temp` and `tnom`, each applied atomically to later
  analyses; and `set`, which accepts `num_threads=N` to cap later parallel work
  and `noinit` to suppress the initial listing.
- **Flow:** `let`, `if`/`else`/`end`, `foreach`, `repeat`, `while`, `dowhile`,
  `break` and `continue` over scalars.
- **Presentation:** `print`, `plot` and `settype`.

Any other command fails with its source location. Each completed analysis is a
named dataset (`ac1`, `tf1`, `pz1`, `disto1`, …) with its own output artifact
(`results.ac-001.csv`, `results.ac-002.csv`), and a qualified expression such
as `abs(ac1.v(out))` reads an earlier one. Transfer-function datasets expose
`transfer_function`, `input_impedance` and `output_impedance`; pole-zero
datasets expose one-based `pole(i)` and `zero(i)` and the `dc_gain` and
`hf_gain` scalars; distortion datasets select a product with
`disto("2f1",i(V1))` or `disto("f1+f2",v(out))`, and `disto("2f1",frequency)`
is that product's physical frequency.

`print` writes complex samples to stdout, including under `--quiet`; with
`--summary -` the data goes instead to a companion `*.control-NNN.json` listed
in the summary's `outputs`. `plot` writes an SVG and a companion
`*.control-NNN.json` holding the exact samples, units and axes, beside `-o` or
else beside the deck. SVG plots need real expressions, so apply `real()` or
`abs()` to complex data. A failed script publishes none of its files.

Scripts run independently at every `.STEP` or `.TEMP` coordinate. Their
transients honour `--checkpoint`, `--resume`, `--tran-stop`, `.OPTIONS RESTART`
and `--compress` exactly as `.TRAN` cards do, and the deck's `.FOUR` and `.FFT`
cards post-process the transients a script runs.

### Analyses requested on the command line

Each of these flags replaces the deck's analysis cards with one analysis. They
are mutually exclusive, and none combines with a `.STEP` or `.TEMP` axis.
Because the deck's cards do not run, an authored `.PAC`, `.PXF`, `.PNOISE` or
`.ENVELOPE` cannot use a mode's carrier; author the `.PSS` or `.HB` in the deck
when a dependent card needs one.

| Flag | Runs | Related flags |
| :--- | :--- | :--- |
| `--monte-carlo N` | N Monte Carlo trials | `--mc-distribution`, `--mc-spread`, `--mc-param`, `--seed` |
| `--pss-freq F` | Periodic steady state at fundamental F; refused beside an authored `.PSS` | `--pss-harmonics N` (default 9), `--pss-tstab TIME` |
| `--hb-freq F` | Harmonic balance at fundamental F | `--hb-harmonics N` (default 9) |
| `--pz-input NODE --pz-output NODE` | Pole-zero between two nodes | `--pz-transfer voltage` (default) or `current` |
| `--sens-output NODE --sens-param NAME` | DC sensitivity of `V(NODE)` to one `.PARAM` | `--sens-value VALUE`, the nominal value (default: the deck's) |
| `--sparam "P1+,P1-,P2+,P2-"` | Two-port S-parameters over the deck's first `.AC` sweep; `0` names ground | `--sparam-z0 OHMS` (default 50) |

`--monte-carlo` samples parameters from `--mc-distribution` (`gaussian` by
default, `uniform` or `worst-case`) with relative spread `--mc-spread` (the
sigma for gaussian, the tolerance otherwise; default `0.01`, and `0` gives
deterministic nominal samples), optionally only the parameters named by
repeated `--mc-param`. Trials run in parallel within
`resources.max_parallel_workers` rather than `-j`, and sampling is seed-stable,
so the statistics match a serial run exactly. Non-converging trials are dropped
from the statistics and counted; all trials failing is an error.

`--seed N` sets the seed of `--monte-carlo` (default 1), or overrides the seed
of every authored Monte Carlo card while keeping each card's run count, starting
trial, distribution, parameter selection and confidence settings. It is refused
before execution unless every run executes a Monte Carlo analysis.

### Process corners

`--corners tt,ss,ff` runs the deck's own analyses once per corner. With
`--corner-lib FILE`, each corner's deck gains `.lib "FILE" <corner>` after its
title line; without it every corner runs the nominal models and the sweep only
checks convergence. Corner names are letters, digits, `_` and `-`, unique
without regard to case. Outputs are tagged per corner (`res.csv` becomes
`res.tt.csv`), measurement names gain a `<corner>:` prefix, each corner keeps
its own report entry and failure category, and the run exits nonzero if any
corner fails. `-j` runs corners in parallel. `--corners` does not combine with
the analysis modes above or with a `.STEP` or `.TEMP` axis.

### Decks that expand into several runs

HSPICE `.ALTER` blocks and textual `.DATA` variants expand into a plan of
concrete decks, each parsed and solved independently. Each run tags its own
artifacts: `.ALTER` runs are named `base` and then by block title, `.DATA` runs
by table row, reduced to a file-safe tag (`out.csv` becomes `out.base.csv`,
`out.hot.csv`, `out.tbl_row_1.csv`). `-j N` runs the plan on N workers, and
`-j 0` on every core up to `resources.max_parallel_workers`; an explicit request
above that ceiling is refused rather than clamped, and with more than one worker
the console shows one status line per run. A failing run does not stop the
rest, as in HSPICE: every run lands in the reports, and the exit status
reflects the whole plan. `resources.max_batch_runs` bounds the plan, counting
every variant, corner and run-axis coordinate.

`.PREPROCESS ADDRESISTORS` writes its derived Xyce-compatible deck beside the
input as `<input file name>_xyce.cir`. It needs a file-backed deck, and it is
refused in an `.ALTER`, `.DATA`, corner or `.STEP` deck, where one sibling file
cannot represent several rewritten decks.

### Measurements and the exit status

`.MEAS` and `.MEASURE` statements evaluate against every analysis that produces
data:

- **TRAN:** node voltages, branch currents as `I(name)`, and time as `TIME`, so
  `FIND TIME WHEN V(out)=...` works
- **DC:** the sweep, with the swept value as the abscissa
- **AC:** `V(x)`/`VM(x)` magnitude, `VDB(x)`, `VP(x)` phase in degrees, `VR(x)`,
  `VI(x)` and the matching `I…` forms, with frequency addressable as
  `FREQUENCY`, `FREQ` or `TIME`
- **NOISE:** the `ONOISE` and `INOISE` spectral densities (also `*_SPECTRUM`)

`GOAL=value [TOL=value]` turns a measurement into a check; `TOL` defaults to
`max(1% of |goal|, 1e-12)`. Under `--spice-dialect xyce`, the continuous
measurements `TRAN_CONT`, `DC_CONT`, `AC_CONT` and `NOISE_CONT` accept
`FAILVALUE=value`: a record fails when `abs(raw_value) >= FAILVALUE`, a
non-finite value or threshold fails closed, and the stream passes only when it
produced at least one record and every record passed. Each record is reported
on its own, with its raw value, threshold, verdict and event or trigger/target
coordinates; JUnit and TAP name it `name[record N]`. With `--compress`,
measurements still read the exact accepted samples.

A failed measurement fails the run with exit 3, whether it missed its goal, did
not evaluate, or belongs to an analysis that never ran. `--allow-failed-meas`
restores exit 0 while every report keeps the individual verdicts. Results
containing NaN or Inf fail the run with exit 80 unless `--allow-nonfinite` is
given. `--meas` prints the results; `--meas-file FILE` writes them as JSON, or
as CSV with `--meas-format csv`, to enough digits to recover every stored value
exactly.

### HDL designs

For a deck that instantiates Verilog designs:

| Option | Effect |
| :--- | :--- |
| `+PLUSARG …` | Plusargs after the netlist (`rspice run tb.cir +VERBOSE +N=4`) for `$test$plusargs` and `$value$plusargs`; they precede the deck's `.options hdl_plusargs`, and the first match wins |
| `--hdl-run-dir DIR` | Directory that relative HDL file paths resolve against (default: the deck's directory) |
| `--hdl-unrestricted-io` | Let HDL file writes leave the run directory |
| `--hdl-delay-select WHICH` | `typ` (default); `min` and `max` are refused with capability `hdl.option.delay_select`, because every HDL delay uses its typical value |

A positional argument after the netlist that does not begin with `+` is a usage
error. An HDL `$error` or `$fatal` fails the run with exit 3. With `-o`, the
designs' console transcript is also written as `<output stem>.hdl.log`; when
`--summary -` reserves stdout, design text goes to stderr.

### `run` flag reference

Output and reports:

| Flag | Effect |
| :--- | :--- |
| `-o`, `--output FILE` | Result file; see Output files below |
| `-f`, `--format FORMAT` | `raw`, `ascii`, `csv`, `tsv`, `json`, `hdf5` or `vcd` (default: `output.format`, else `raw`) |
| `--save SIGNAL` | Export only these signals, replacing the deck's `.SAVE`, `.PROBE`, `.PRINT` and `.PLOT` selection (repeatable; `V(out)`, `V(a,b)`, `I(v1)`, `@m1[id]`, `all`) |
| `--expand-buses` | With `-f vcd`, write each digital-bus member as its own one-bit `$var` |
| `--compress`, `--compress-tol TOL` | Decimate the published transient waveform within TOL (default `1e-4`) |
| `--meas` | Print `.MEAS` results |
| `--meas-file FILE`, `--meas-format FORMAT` | Write `.MEAS` results as `json` (default) or `csv` |
| `--report-file FILE`, `--report-format FORMAT` | Write a `junit` or `tap` CI report; each flag requires the other |
| `--summary FILE` | Write a JSON run summary, or print it to stdout with `-` |
| `--allow-failed-meas` | Exit 0 when only measurements fail |
| `--allow-nonfinite` | Export results containing NaN or Inf instead of failing |
| `--progress` | Show a progress bar during transient analysis |

Execution:

| Flag | Effect |
| :--- | :--- |
| `--timeout SECONDS` | Abort after this long (exit 124); transient and DC sweeps stop at the next safe point |
| `-j`, `--jobs N` | Workers for `.ALTER`/`.DATA` variants and corners (default 1; `0` for every core) |
| `--temp CELSIUS` | Simulation temperature; refused beside a `.TEMP` or `.STEP TEMP` axis |

Solver:

| Flag | Effect |
| :--- | :--- |
| `--maxiter N` | Newton-Raphson iteration limit |
| `--reltol TOL` | Relative tolerance |
| `--abstol TOL` | Absolute tolerance for both node voltages and branch currents |
| `--voltage-abstol TOL`, `--current-abstol TOL` | Voltage (`VNTOL`) or current absolute tolerance alone; each takes precedence over `--abstol` |
| `--charge-abstol TOL` | Charge tolerance (`CHGTOL`) |
| `--event-flux-abstol TOL` | Flux-linkage conservation tolerance at physical transient events, in weber-turns (`EVENTFLUXTOL`, default `1e-24`) |
| `--residual-reltol TOL` | Relative residual tolerance for equation convergence checks |
| `--gmin G` | Initial GMIN for convergence aids |
| `--trtol TOL` | Transient truncation-error tolerance (`TRTOL`) |
| `--min-step TIME`, `--max-step TIME` | Transient timestep bounds |
| `--integration-method METHOD` | `trap`, `gear`, `euler` or `trapgear` (the default, which switches automatically) |
| `--convergence MODE` | DC convergence preset: `fast`, `default` or `robust` |

Transient state:

| Flag | Effect |
| :--- | :--- |
| `--tran-stop TIME` | Override the `.TRAN` stop time without editing the deck |
| `--checkpoint FILE` | Save the transient integrator state when the run completes |
| `--resume FILE` | Continue a transient from a checkpoint; the deck must be byte-identical to the one that wrote it |

The analysis-mode, corner and HDL flags are described above. Command-line
values override the deck's `.OPTIONS`, which override the configuration file;
`--convergence` replaces the deck's stepping and damping controls, and without
it the configured `convergence_mode` is the base those controls refine.
Tolerances, timesteps and `--timeout` must be positive and finite, `--gmin` and
`--mc-spread` non-negative, and `--min-step` no larger than `--max-step`.

## Output files

With `-o FILE`, `run` publishes machine-readable results in the `-f` format:
`raw` (binary SPICE rawfile), `ascii` (ASCII rawfile), `csv`, `tsv`, `json`,
`hdf5` or `vcd`. A relative `-o` path lands in `output.output_directory` when
the configuration sets one.

### Artifact naming

Every artifact is namespaced by the canonical identity the planner minted for
the analysis that produced it, so nothing a deck publishes can overwrite
anything else it publishes:

```
<output stem>[.<run label>][.<coordinate>].<analysis id>.<extension>
```

- `<analysis id>` is `<family>-<NNN>`, one-based in authored source order:
  `op-001`, `tran-001`, `ac-001`, `ac-002`, `four-001`, `pss-001`, `pac-001`,
  `pxf-001`, `pnoise-001`, `env-001`. A deck with exactly one analysis and no
  run axis writes to the `-o` path unchanged.
- `<coordinate>` appears only under a `.STEP`, `.TEMP` or `.DATA` axis. It is
  the deterministic coordinate identity `run_<32 hex>_<NNN>`, derived from the
  coordinate's semantic axis assignment, so it does not change when the
  authored order of the axis values does.
- `<run label>` appears only for an outer `.ALTER` or textual `.DATA` variant.

`.FOUR` publishes one artifact per operand, because it evaluates one spectrum
per operand: `.FOUR 1k V(out) I(V1)` writes `four-001` and `four-002`.

### `json`: the shared typed result document

`-f json` publishes the shared `rspice-analysis-result` document, the typed
result the Python, browser and engine-adapter surfaces also publish. It carries
the analysis instance, the coordinate and topology fingerprint it was produced
at, typed axes, one series per signal with a descriptor (canonical name, display
spelling, kind, unit, value type and owner), family-specific scalars and a
per-family payload. A sample that does not exist is encoded as absent, never as
zero, and a channel the output selection did not retain keeps its descriptor
and says so.

- `.SP DONOISE` writes the scattering sweep and, beside it, a port-noise
  document under the same analysis identity (`out.json` and
  `out.port-noise.json`, or `out.sp-001.json` and `out.sp-001.port-noise.json`
  in a multi-analysis deck). It carries the noise covariance, its reference
  temperature and the `4kT` thermal reference, and for a two-port the `Rn`, `F`,
  `Fmin` and `Sopt` figures at every frequency.
- `.PAC`, `.PXF`, `.PNOISE`, `.PSTB` and `.ENVELOPE` documents name the `.PSS`
  or `.HB` instance they were computed around as their parent analysis. The flat
  formats carry that pairing only in the artifact names.
- `.SENS` attributes each derivative to the parameter rather than to a device,
  with its nominal value and normalized derivative. A card's specifications are
  one filter list: a glob selects a device, one owner's parameter (`M1:W`,
  `RMOD:*`) or a vector, and only `PARAM:<glob>` selects design parameters,
  whose rows are named `PARAM:<NAME>`; `--sens-param` uses the same
  `PARAM:<NAME>` identity. A specification that selects nothing is refused by
  name.
- `.FFT` publishes its own bundle (`schema_version: 3`) beside its parent
  transient, with the transform settings and a status per request: `"complete"`
  for a computed spectrum, or `"incomplete-history"`, with the available time
  range and no bins, when the transient finished before the requested window.
  CSV, TSV, RAW and HDF5 carry the same status contract.

### `csv`, `tsv`, `raw`, `ascii`: the flat authored projection

These formats carry the columns the deck's `.PRINT`, `.PLOT` and `.SAVE` cards
select, in authored order and exactly as authored, including a column a card
deliberately repeats. They cannot represent an absent sample, so the
coordinate-set manifest below carries missingness for them.

| Mode | Contents |
| :--- | :--- |
| `.OP` | `signal,value` rows |
| `.DC`, `.TRAN`, `.AC`, `.NOISE` | Node and branch columns over the family's own scale |
| Aggregated axis sweep | One row per coordinate, node voltages as columns |
| `.FOUR` | One row per harmonic: frequency, magnitude, phase, DC component, THD |
| `.MC`, `--monte-carlo` | Per-trial samples over `trial_index`, statistics, histograms, confidence states and campaign context |
| `.PSS`, `--pss-freq` | One period of the steady-state waveforms |
| `.HB`, `--hb-freq` | Complex spectrum per node over the harmonic frequencies |
| `.PAC` | Complex node and branch columns per retained sideband (`V(out):sb0`), scaled by `PACMAG`, over the offset frequency, then the fundamental and each sideband's frequency in Hz; `INCLUDEDC=NO` omits sideband zero |
| `.PXF` | Complex transfer columns `<signal>:sb<in>->sb<out>` over the offset frequency |
| `.PNOISE` | Real densities over the offset frequency: `output_noise` (V²/Hz) and each source's `contribution:<label>` around a driven carrier, `phase_noise` (dBc/Hz) around an autonomous one |
| `.PSTB` | One row per Floquet mode: multiplier magnitude and phase, stability margin, damping, frequency and probe participation |
| `.ENVELOPE` | The continued slow-time trajectory, shaped like a transient table |
| `.QPSS`, `.QPAC`, `.QPXF`, `.QPNOISE` | The primary result series; selecting an unavailable sample fails, and JSON keeps the full evidence |
| `.STB` | Complex `loopgain`, `loopgain_mag_db`, `loopgain_phase_deg` and the `stb:` evidence columns |
| `.TF` | Gain, input impedance, output impedance |
| `.PZ`, `--pz-input` | `pole(i)`/`zero(i)` complex columns, `dc_gain`, `high_frequency_gain` and the `pz:` evidence columns |
| `.SENS`, `--sens-output` | `dV/d(param)` columns: one point for DC, a series over frequency for AC |
| `.DCMATCH` | Output statistics and per-contributor `sigma_parameter(…)`, `sensitivity(…)`, `contribution(…)` and `share(…)` columns, each named `<scope>:<instance>/<parameter>` |
| `.DISTO` | `peak(<product>:<signal>)` phasors and each product's `frequency(<product>)` |
| `.SP` | `S_i_j` complex columns and `Z0(port)` reference impedances in ohms; Touchstone instead when `-o` ends in `.sNp`, `.snp` or `.ts` |
| `--sparam` | `S11`, `S21`, `S12`, `S22` and `Z0(1)`, `Z0(2)`; Touchstone instead when `-o` ends in `.s2p`, `.snp` or `.ts` |

A numbered `.sNp` name writes Touchstone version 1 and must match the
network's port count; `.snp` and `.ts` write Touchstone 2.0, which also carries
unequal port reference impedances. Touchstone cannot hold `.SP DONOISE`
covariance, so noise results need another format. TF, pole-zero, sensitivity,
DC-mismatch and periodic-stability results reach HDF5 through a general
`table` section that keeps the analysis identity, the coordinate and each
column's quantity type.

### `vcd`

A Value Change Dump of the transient's event timelines: one `wire` per XSPICE
digital node and one `real` per real event node under `$scope module events`,
at the times the events happened rather than on the analysis grid. The
`$timescale` is the coarsest period that keeps every event time an exact
integer tick, so a run whose edges land on nanoseconds is written `1 ns`. A
time that is not a whole number of femtoseconds, event times that would merge
on the tick grid, and a duration that no exact timescale can span are refused
rather than quantised. VCD has four bit states and no drive strength, so the
level survives and the XSPICE strength band does not.

A declared digital bus, such as a vector discrete port of a mixed Verilog-AMS
module, is written as one `$var wire N` named `bus [msb:lsb]`, bits most
significant first and `x` for a member not yet stated, with no member scalars
beside it. `--expand-buses` writes the members as N one-bit `$var`s instead,
and every other format refuses that flag. It is the same dump
`TransientResult.to_vcd()` in Python and `WasmResultHandle.toVcd()` in the
browser write, and the one `convert --to vcd` builds from the same run's
rawfile or typed document.

Only a transient captures events, so every other analysis refuses `-f vcd`. A
transient that captured no event node publishes an empty dump and says so on
stderr.

### `hdf5`

One document per analysis, whose section group is named by the analysis
identity rather than by the result family, so repeated cards cannot collide.
The group declares its family in its own `section_type` attribute, and the root
carries `analysis_id` plus, for an axis coordinate, `coordinate_id`,
`coordinate_tag`, `coordinate_assignment`, and `topology_fingerprint`.

### Coordinate sets

`.STEP`, `.TEMP` and `.DATA` axes publish one artifact per coordinate and
analysis, plus two manifests:

- `<output stem>.step_schema.json` (version 3) groups by analysis instance. It
  records the union of the coordinates' signal schemas and, for each
  coordinate, its ID, topology fingerprint, artifact filename and a validity
  bitmap over the union. A signal a conditional removed is omitted from that
  coordinate's artifact and marked invalid; it is never inferred from another
  coordinate or written as zero.
- `<output stem>.run_set.json` is published last and names every artifact of
  the complete set, so a reader that finds it can load everything it names.

The set is one transaction: a failed or cancelled run leaves either the
previous complete set or nothing. An implicit axis sweep whose topology and
signal schema are identical at every coordinate keeps a single aggregated table
instead.

Every analysis family runs under a run axis. An authored `.MC` card under
`.STEP` or `.TEMP` runs at every coordinate, with its random stream derived from
the authored seed and the coordinate's identity, so each coordinate reproduces
exactly whatever else ran and no two coordinates draw the same variation vector.
`.MC N START n` (or `START=n`) runs the batch that begins at zero-based trial
index `n`: `.MC 100 START 1000 SEED 42` runs trials 1000 to 1099, and the same
deck, seed, distribution and coordinate replay the same samples. Exports keep
the original `trial_index`, with gaps where trials failed.

### Checkpoints and compression

`--checkpoint`, `--resume` and `--tran-stop` need an executed `.TRAN`. When a
run holds more than one transient, coordinate, variant or corner, the checkpoint
base gains the same identity components as the result artifacts (for example
`state.tran-001.chk` or `state.tt.chk`), so no run overwrites or resumes another
run's state; pass the same base to `--resume`. Scheduled `.OPTIONS RESTART`
files keep these names too. Resume inputs are reserved before anything is
published, so results and reports cannot replace them even when a run fails. To
renew a checkpoint in place, pass the same base to both `--resume` and
`--checkpoint` with a separate result destination.

`--compress` decimates only the published transient waveform and prints the
worst reconstruction error it accepted beside the compression ratio. `.MEASURE`,
`.FOUR` and `.FFT` are evaluated on the exact accepted trajectory first, so
their artifacts are identical with and without the flag; every sample an
authored `.OPTIONS OUTPUT INITIAL_INTERVAL` lattice names is kept exactly, and
`.OPTIONS RESTART` files are written from the accepted trajectory.

### Reserved columns

Flat exports and `convert` keep typed evidence in reserved, dimensionless
columns, so a CSV or RAW file keeps what the JSON document states:

| Column | Meaning |
| :--- | :--- |
| `<name>:unavailable(<reason>)` | `1` beside a missing scalar whose absence is proven: `positive_infinity`, `negative_infinity`, `no_crossover` or `empty_domain` |
| `<name>:sensitivity_status` | Per sample: `0` available, `1` zero output, `2` nondifferentiable magnitude, `3` outside the representable range |
| `<name>:monte_carlo_status` | A missing statistic: `1` empty population, `2` insufficient samples, `3` unrepresentable result |
| `mc:identity:…` | Exact text, integer and unit identities of a Monte Carlo report, text hex-encoded as UTF-8 |
| `stb:pole(N)`, `stb:completed(…)`, `stb:circuit_stability(…)`, `stb:circuit_poles(…)` | STB natural poles in rad/s, and the completion, stability and pole-spectrum verdicts |
| `pz:…` | Qualification of a pole-zero root set |

A CSV or TSV file whose layout cannot be inferred from its headers ends with one
metadata record: `# RSpiceTableLayoutV1` marks each column as real or as the
real or imaginary part of a complex signal, and `# RSpiceTableLayoutV2` declares
the analysis, title and each column's name, kind, quantity and unit. Other table
readers should skip that record. RAW files carry the same metadata in numbered,
hex-encoded `Option:` chunks.

## Structured studies (`study`)

A study is a version-1 JSON document naming a circuit and a graph of typed
analysis tasks:

```json
{
  "schema_version": 1,
  "id": "54dac9d3-e7c4-46d3-b2ae-9db058426b19",
  "circuit": { "path": "circuits/divider.cir" },
  "tasks": [
    { "id": "bias", "analysis": "DcOp" },
    {
      "id": "response",
      "depends_on": ["bias"],
      "analysis": {
        "Ac": {
          "start_freq": 1.0,
          "stop_freq": 1000000.0,
          "points_per_unit": 20,
          "sweep": "Decade"
        }
      }
    }
  ]
}
```

| Command | Effect |
| :--- | :--- |
| `rspice study check STUDY` | Validate the study, its captured circuit sources, typed settings and dependency graph without solving |
| `rspice study plan STUDY` | Also report the prepared task order, stable instance IDs, configuration digests, and source and snapshot digests |
| `rspice study run STUDY -o RESULTS` | Execute the graph and publish one `rspice.study.results` document |

All three take `--timeout SECONDS` and `--json`, which prints one versioned
document even under `--quiet`; a validation failure prints `valid: false` and
exits nonzero. `study run --progress-json` writes versioned task-start, progress
and completion records to stderr and switches logs and errors to JSON, so
stderr stays machine-readable.

- The study `id` is a nonzero UUID that namespaces task identities; keep it
  and each task's `id` when editing settings or labels. Task names are
  case-sensitive and hold 1 to 128 ASCII letters, digits, dots, underscores or
  hyphens. `depends_on` names prerequisites explicitly, and array order breaks
  ties between independent tasks.
- Unknown fields, repeated task names, missing or cyclic dependencies and
  task-specific numeric settings an analysis cannot honour are errors.
- `circuit.path` is relative to the study file, so a study cannot come from
  stdin, and `circuit.include_search_paths` are relative to the circuit's
  directory; configured include and library paths also apply. The circuit
  supplies devices, models, options, measurements and saves. Analysis cards and
  control scripts in it are refused, because the study owns the task graph.
- `analysis` uses the typed representation the RSpice application executes.
  `Pac`, `Pxf`, `Pnoise` and `Pstb` tasks also need a matching `periodic` object
  with `kind` and complete `settings`. Monte Carlo, parametric and corner tasks
  are refused with capability `study.structured_sweeps`. SOA, optimization,
  PSP, HBSP and HB-noise analyses are available in the CLI only as study tasks.
- The study's `numeric` defaults apply to the tasks that consume them, and each
  task's `numeric` overrides them; the configuration's `[simulation]` settings
  belong to `run`. `[resources]` limits are captured in the prepared snapshot
  and enforced during execution, and a task whose analysis cannot enforce
  custom limits is refused with capability `study.execution_resource_overrides`.

The results document embeds the typed study, the preparation report, the exact
netlist each task executed, and the retained typed results with their native
payloads, complex samples, units, result digests and the prepared-run receipt.
It is published only after every task and its results validate, so a failure
leaves an existing output unchanged, and it may not replace the study, circuit,
include or configuration files. `save_policy.maximum_storage_bytes` bounds the
retained evidence and the final document. Ctrl-C and `--timeout` cancel the run
(exit 130 and 124), and solver failures and failed authored measurements exit
nonzero. STB margins that fall outside the swept band are recorded as
unavailable without failing the study. Successful preparation is not evidence
of convergence.

## Checking a deck (`check`)

`rspice check DECK` validates without simulating. It expands every `.ALTER` and
`.DATA` variant and every `.STEP` or `.TEMP` coordinate, and for each one:

- validates every analysis request, including AC and noise `DATA=` tables,
  which must exist, hold one frequency column of finite rows, and fit the
  configured limits;
- resolves every output symbol `.PRINT`, `.PLOT`, `.SAVE` and `.PROBE` name, so
  an undefined `V(x)` or `I(rbogus)` is an error;
- reports a loop of ideal voltage sources or inductors as an error, and a node
  connected only to current sources as a warning;
- builds the circuit with external runtimes stubbed out, under the selected
  dialect, the configured settings and the coordinate's own options and
  temperature, so native, XSPICE and Verilog-A construction errors fail
  validation;
- reports parser diagnostics and Verilog-A compiler warnings, with their
  original source locations, as warnings.

| Option | Effect |
| :--- | :--- |
| `--connectivity` | Also warn about nodes with one device terminal (`TOPOLOGY_ONE_DEVICE_TERMINAL`) or no DC path to ground (`TOPOLOGY_NO_DC_PATH`) |
| `--models` | Also warn about undefined model references |
| `--strict` | Fail on any warning (exit 2) |
| `--json` | Print one `rspice.check` document: `valid` follows the non-strict verdict, and `strict_valid` is false whenever there are warnings |

Validation errors exit 65, and the process error lists every issue, so the cause
survives a redirected stdout. Control scripts are checked without running them:
every command needs a handler, and literal OP, AC and TRAN requests are
validated in every branch. Arguments built with `$` substitution get a
`CONTROL_DYNAMIC_ANALYSIS` warning, because their values exist only at run time.

## Inspecting a netlist (`info`)

`rspice info DECK` summarizes the parsed deck without elaborating or simulating
it: title, element counts by kind, analysis and measurement counts, and parser
diagnostics.

| Option | Adds |
| :--- | :--- |
| `-d`, `--detailed` | Each element's nodes and typed device or source specification |
| `--models` | Each model's type and parameters |
| `--hierarchy` | Root instances with their connections and overrides, and each subcircuit definition with its ports, defaults, local parameters and instance references, by scope |
| `--params` | Effective root parameter bindings, including strings, complex values and analysis-dependent expressions |
| `--json` | Print one `rspice.info` document |

In JSON the reports are `element_details`, `models` and `model_definitions`,
`subcircuits` and `hierarchy`, and `params` and `parameter_definitions`; a
report whose flag is absent is `null`. `params` holds real numeric values only,
while `parameter_definitions` is the complete typed list. A value that needs
expression or model evaluation is `null` beside its expression or model
reference: `info` evaluates nothing for a particular instance, does not expand
references recursively, and draws no statistical sample.

## Comparing results (`compare`)

`rspice compare RESULT GOLDEN` reads either side in any supported format,
detected by extension (`.csv`, `.tsv`, `.json`, `.h5` or `.hdf5`, `.vcd`, and
Touchstone `.sNp`, `.snp` or `.ts`; anything else is read as a rawfile), so a
binary rawfile can be checked against a CSV golden. The golden defines the
contract:

- a golden variable missing from the result fails, unless `--ignore-missing`;
- a point-count mismatch fails, so a truncated result cannot pass on its
  overlap, unless `--allow-truncated`;
- a finite sample passes when its absolute error is at most `--abstol`
  (default `1e-9`) or its error relative to the golden value is at most
  `--reltol` (default `1e-6`); against a golden zero only `--abstol` applies;
- NaN never matches, undefined samples must sit at the same positions on both
  sides, and every compared signal needs at least one pair of defined samples;
- units must match exactly, including the case of SI prefixes (`mV` is not
  `MV`), and no unit conversion is performed;
- complex data compares as separate `Re(..)` and `Im(..)` series, and a `.vcd`
  file compares through the table `convert` builds from it.

| Option | Effect |
| :--- | :--- |
| `--variables VAR` | Compare only these signals (repeatable; see `convert`) |
| `--interpolate` | Resample the result onto the golden's scale: analog signals linearly, digital and real event signals by holding their last value. The result must cover the golden range, within four ULPs at the endpoints; nothing is extrapolated |
| `--fail-fast` | Stop at the first difference |
| `--json` | Print one `rspice.comparison` document |
| `--section NAME_OR_INDEX` | Select one result from a RAW, HDF5 or Touchstone container (see `convert`) |
| `--bless` | Accept the result as the new golden |

Reports keep exact mismatch counts and maximum errors, and show the first ten
differences in JSON and five in text. Mismatches exit 3. `--bless` first checks
that the result satisfies the requested selection and options, then writes the
exact bytes it validated; with no golden yet it creates one. It needs the result
and golden to share a file format, does not combine with `--section`, and
refuses a Touchstone destination name that would change the decoded network,
such as a two-port `.s2p` renamed `.s1p`.

Typed results compare by identity. FFT bundles compare request by request and
cannot be interpolated, and an incomplete spectrum never passes. STB,
pole-zero, Monte Carlo, sensitivity and DC-mismatch results include their
reserved evidence columns, so a changed verdict fails whatever the tolerances.
An older flat baseline without those columns verifies only what it retains;
regenerate it for a complete comparison.

## Converting results (`convert`)

`rspice convert INPUT OUTPUT --to FORMAT` converts a result to `raw`, `ascii`,
`csv`, `tsv`, `json`, `hdf5` or `vcd`; Touchstone is read but not written. The
input format is detected by extension, as for `compare`, unless `--from` names
it. Complex AC data survives every round trip (`Re(..)`/`Im(..)` column pairs
in CSV and TSV, `Flags: complex` in rawfiles, real and imaginary arrays in JSON
and HDF5), declared signal and coordinate units are preserved, and RAW and HDF5
keep an explicit validity mask, so an undefined sample stays distinct from
zero. An operating point is a one-sample result and converts like any other; an
empty coordinate is refused when the file is read.

| Option | Effect |
| :--- | :--- |
| `--variables VAR` | Keep only these signals (repeatable). Names match without regard to case; a qualified name such as `V(out)` or `D(clk)` selects exactly that signal, and a bare name must be unambiguous |
| `--start VALUE`, `--stop VALUE` | Clip the scale; `--start` must not exceed `--stop` |
| `--section NAME_OR_INDEX` | Select one result from a RAW, HDF5 or Touchstone input that holds several: an exact plot or group name, or a one-based index from the list the diagnostic prints. Required when there are several; refused for a single-document input |
| `--expand-buses` | With `--to vcd`, write each bus member as its own one-bit `$var` |

FFT bundles convert whole: `--variables`, `--start` and `--stop` are refused
for them, and so is VCD output. Typed periodic, distortion, STB, pole-zero,
Monte Carlo, sensitivity and DC-mismatch results keep their response identities
(`<signal>:sb<index>`, `<signal>:sb<in>->sb<out>`, `peak(<product>:<signal>)`)
and reserved columns. Inputs must be regular files (a symlink to one is
accepted); each is validated in full against `max_external_data_bytes` and the
value budgets before anything is selected or written, and a refused conversion
leaves an existing destination unchanged.

For example,
`convert mixed.raw grid.csv --to csv --section "Transient Analysis"` takes the
sampled grid of a mixed-signal result, and
`convert mixed.raw events.vcd --to vcd` keeps its complete event timelines.

Touchstone input (`--from touchstone` for another extension) reads version 1
and 2.0 files with RI, magnitude/angle or dB/angle data, full or triangular
matrices, and LF, CRLF or CR line endings. Omitted option fields default to GHz,
S, magnitude/angle and 50 ohms, and only the first option line counts, as the
specification requires. Reference impedances stay numeric `Z0(port)` columns, so
equal coefficients with unequal references do not compare equal. A file with
noise data has a `network` and a `noise` section.

### VCD conversion

`--to vcd` writes an event dump rather than a table. A rawfile carrying event
plots and a typed JSON result document hold the timelines themselves and
convert exactly, producing the file `run -f vcd` publishes. Any other source is
converted from its grid `D(node)` and `E(node)` columns, which is lossy: `0`,
`1` and `0.5` become `0`, `1` and `x`, with one change per level held, and the
drive strength those columns already dropped is not recovered. Sampled time
coordinates in `s`, `ms`, `us`, `ns`, `ps` or `fs` are converted to seconds,
and `--start` and `--stop` are always in seconds; clipping keeps the held value
of every selected signal at `--start`. A bus selector accepts the bus name, its
declared range or one bit (`D(top.data[7:0])`, `D(top.data[3])`); without
`--expand-buses`, selecting one bit keeps the whole vector and prints a note.

Converting from `vcd` builds a table whose rows are the dump's distinct ticks:
each signal holds its last value, `x` and `z` become `0.5` in a digital column,
a vector becomes one `D(bus[k])` column per bit, and a real signal stays
undefined until its first value. VCD-to-VCD conversion keeps the original
integer timeline.

## Verilog-A models (`compile-va`)

`rspice compile-va MODEL.va` compiles a Verilog-A source and prints its
interface table unless `--quiet` is set.

| Option | Effect |
| :--- | :--- |
| `-o`, `--output FILE` | Write a versioned `rspice.compile_va` JSON summary: source, model, terminals, internal nodes, parameters with defaults and bounds, and compiler warnings |
| `-I`, `--include DIR` | Extra include directory (repeatable) |
| `--module NAME` | Select one module of a multi-module source |
| `--detailed` | Also print per-branch stamp and Jacobian entry counts |
| `--show-usage` | Print an example netlist instantiation, with the absolute source path and module selection |
| `--strict` | Refused with capability `veriloga.strict_lrm` (exit 69): strict LRM checking is not implemented |

Includes resolve against the including file first, then the directories
preprocessing discovered, `-I` directories, configured
`paths.veriloga_includes`, and finally the built-in standard headers; an
unreadable entry or broken link stops resolution rather than selecting another
file. Diagnostics name the original file, line and column after include and
macro expansion, and under `--error-format json` carry their code, phase and
UTF-8 byte range in `error.diagnostics`. `run` and `check` report the same
compiler findings. The `[resources]` source, dependency and include-depth
budgets apply, and Ctrl-C cancels compilation.

## Model packs (`models`)

`rspice models` lists the SPICE model packs shipped beside the binary, or the
tree `--models-dir DIR` or `RSPICE_MODELS_DIR` names: a directory holding
`PACKS.tsv` and `CATALOG.tsv`. The foundation library is compiled in, so
simulation does not need this tree; only this command does.

| Option | Effect |
| :--- | :--- |
| `--part NAME` | Every pack that defines this exact part name, ignoring case |
| `--device CLASS` | Every definition of a canonical device class, such as `diode` or `jfet-n` |
| `--search TEXT` | Definitions whose name contains the text, ignoring case |
| `--shippable-only` | Only redistributable packs and unrestricted definitions, applied before results are limited |

`--part` excludes `--device` and `--search`, and `--device` excludes
`--search`. `--device` and `--search` print at most 200 rows. A malformed index
or catalog record stops the query with its file and line.

## Health and capabilities

`rspice health` is a deployment probe. The default `--mode readiness` validates
the effective configuration and runs a small built-in circuit through parsing,
construction, matrix assembly and a DC solve, with no filesystem or network
I/O. `--mode liveness` skips that workload, so a deliberately narrow admission
policy cannot make the process look dead. `--json` prints a versioned document
with `status`, `ready`, the probe duration, build and runtime identity,
per-check verdicts and the `run_id`.

`rspice capabilities` reports, for every canonical analysis kind, whether this
CLI executes it, whether the control-script host runs it and which typed result
family it maps to, followed by the workflows the CLI does not offer. `--json`
prints the `rspice.capabilities` document, also under `--quiet`, without
reading a netlist; it adds the result-document schema and version, the format
names and the build identity. These declarations describe adapters, not whether
a particular circuit is admitted or numerically qualified. The CLI does not
offer:

- standalone remeasurement, transformation or plotting of a saved result
  (`convert` and `compare` exist);
- durable multi-netlist job manifests with selective retry (variants and
  corners run in parallel within one invocation);
- Monte Carlo checkpoint continuation or population aggregation (indexed
  `START` batches exist);
- conversion of a subset of an FFT spectrum;
- strict Verilog-A LRM checking;
- SOA, optimization, PSP, HBSP and HB-noise analyses through `run` or the flat
  formats; they run as `study run` tasks.

## Observability

Logging uses `--log-level` first, then debug logging for `--verbose`, then
`RUST_LOG`, and finally `warn`. `RUST_LOG` supports module directives and
message regexes, such as `rspice=info/Loading`. `RUST_LOG_STYLE` accepts `auto`,
`always` or `never`; JSON logs never carry terminal colour codes. An invalid
active logging setting exits 78. `--quiet` skips logging and its environment
settings.

`rspice --version` reports the version, build target, profile and exact source
commit; the same commit appears in health documents, fatal diagnostics and run
summaries, so an installed binary can be traced to its release. The commit is
read from `git rev-parse HEAD` at build time; set `RSPICE_BUILD_COMMIT` to a
full 40-character hash to stamp it when building from a source archive.

Every JSON document the CLI writes, from `health`, a fatal diagnostic,
`--summary` or `--log-format json`, carries the process `run_id`, so one run's
logs, failure and summary can be correlated afterwards. The documents below
also carry the tool identity, a `schema` name and `schema_version` 1:

| Schema | Written by |
| :--- | :--- |
| `rspice.check` | `check --json` |
| `rspice.info` | `info --json` |
| `rspice.comparison` | `compare --json` |
| `rspice.compile_va` | `compile-va -o` |
| `rspice.capabilities` | `capabilities --json` |
| `rspice.measurements` | `run --meas-file` in JSON |
| `rspice.diagnostic` | Nonfatal stderr diagnostics under `--error-format json` |
| `rspice.study.check`, `rspice.study.plan`, `rspice.study.run` | `study check`, `plan` and `run` with `--json` |
| `rspice.study.progress` | `study run --progress-json`, on stderr |
| `rspice.study.results` | `study run -o` |

`--error-format json` writes the fatal diagnostic (`schema_version: 2`) with a
stable code and category, retry policy, exit code, the failing analysis and
run-coordinate IDs, the deck path and line, the refused capability token, and
numeric resource or convergence details; it also covers argument-parsing and
timeout failures. `check --json`, `compare --json` and `run --summary -`
reserve stdout for one JSON document without needing `--quiet`.

## Configuration file

Configuration merges, in increasing priority: built-in defaults, the user
file, the project file `./.rspicerc`, environment variables, and command-line
flags. The user file is `rspice/config.toml` in the platform configuration
directory (`$XDG_CONFIG_HOME` or `~/.config` on Linux,
`~/Library/Application Support` on macOS, `%APPDATA%` on Windows), falling back
to `~/.rspicerc`. `--config FILE` replaces the user and project files with that
one file. Only an absent default file is skipped; an unreadable file or a broken
symlink fails with its path and the filesystem reason. Scalar keys override the
layer below, while the `[paths]` lists accumulate, so a project file adds search
directories rather than discarding the user's. For simulation settings, a
deck's `.OPTIONS` sit between the configuration and the command line.

Files are TOML. Unknown keys are rejected rather than ignored, and values are
validated on load (positive tolerances and timesteps,
`min_timestep <= max_timestep`, known enum names), so a bad file or environment
override exits 78 naming the key. Every recognized key, with its default:

```toml
[simulation]
temperature = 27.0            # Celsius
max_iterations = 50
abstol = 1e-12
reltol = 1e-3
residual_reltol = 1e-3
min_timestep = 1e-12
max_timestep = 1e-3
compress_waveforms = false
compression_tolerance = 1e-4
convergence_mode = "default"  # "fast" | "default" | "robust"

[output]
format = "raw"                   # default for -f/--format
show_progress = false            # default for --progress
# output_directory = "results"   # relative -o paths land here (created on demand)

[paths]
include_paths = ["./models", "./lib"]   # extra .include/.lib search dirs for run/check/info
library_paths = []                      # extra .include/.lib search dirs for run/check/info
veriloga_includes = []                  # extra include dirs for compile-va

[resources]
max_netlist_bytes = 67108864             # 64 MiB root deck
max_netlist_lines = 2000000
max_expanded_source_bytes = 268435456    # includes and retained multi-run decks
max_dependency_source_bytes = 268435456
max_external_data_bytes = 268435456
max_external_data_values = 25000000
max_shared_cache_bytes = 536870912
max_include_depth = 64
max_hierarchy_depth = 100
max_flattened_elements = 250000
max_circuit_nodes = 250000
max_matrix_unknowns = 250000
max_analysis_points = 2000000
max_result_values = 25000000
max_parallel_workers = 64               # cap batch/analysis worker fan-out
max_batch_runs = 10000
```

The configured `convergence_mode` is the base convergence policy for `run`,
`check` and `health`; a deck's `.OPTIONS` convergence controls refine it, and
`run --convergence` replaces both.

`[resources]` applies to every command that reads a deck, a study or a result
file, including stdin, include expansion, `.ALTER` and `.DATA` materialization,
derived corner and S-parameter decks, circuit construction and result
retention; the worker ceiling also bounds `-j 0`. Retained measurement and
report records count against `max_result_values` across every step, variant,
corner and control dataset, and exceeding it exits 75 even with
`--allow-failed-meas`. Lower the ceilings for a service with a fixed CPU or
memory budget.

Environment variable overrides:

| Variable | Effect |
| :--- | :--- |
| `RSPICE_TEMPERATURE` | Default simulation temperature (Celsius) |
| `RSPICE_OUTPUT_FORMAT` | Default output format |
| `RSPICE_INCLUDE_PATH` | Include search paths, platform path separator (`;` on Windows, `:` elsewhere) |
| `RSPICE_LIBRARY_PATH` | Model library paths, platform path separator |
| `RSPICE_MODELS_DIR` | Model tree `rspice models` reads |
| `RSPICE_MAX_<RESOURCE>` | Override any `[resources]` key in uppercase, for example `RSPICE_MAX_BATCH_RUNS` or `RSPICE_MAX_NETLIST_BYTES` |

## Exit Codes

The exit status is the verification contract: a deck whose measurements fail,
or whose results are non-finite, does not exit 0.

Every nonzero code is derived from one failure category, and for anything the
engine produced that category is the engine's own
(`rspice_core::SimulationErrorCategory`), so the exit status, the
`--error-format json` `category` field and the `--summary` report always agree.
Automation can branch on the number alone:

| Code | Category | Meaning |
| :--- | :--- | :--- |
| 0 | — | Success: simulation ran and every check passed |
| 1 | `compilation`, `conversion` | A failure with no engine category: Verilog-A compile failure, result-format conversion failure |
| 2 | `usage` | Usage error (invalid arguments, or warnings under `check --strict`) |
| 3 | `verification` | A `.MEAS` failed or did not evaluate, or `compare` found mismatches |
| 65 | `netlist` | Invalid authored input: netlist parse failure, singular topology from `check` |
| 66 | `input_not_found` | Input file not found |
| 69 | `capability` | The deck is well formed and this build does not execute it: an unsupported analysis/device combination, model family, or netlist construct |
| 70 | `internal` | Internal error |
| 73 | `output_commit` | The run produced correct results and publishing them failed; the previous artifact is intact unless the message says otherwise |
| 74 | `io` | I/O error outside a publication transaction (input read or console output failure) |
| 75 | `resource_limit` | A configured resource budget was exceeded; the same workload succeeds under a larger budget |
| 76 | `persistence` | A checkpoint or other persisted artifact was written by an incompatible format version |
| 78 | `configuration` | Configuration error |
| 80 | `simulation` | Circuit construction or device evaluation failed |
| 81 | `solver` | The numerical solver failed |
| 82 | `convergence` | An iterative analysis exhausted its convergence strategy |
| 83 | `signal_unavailable` | A valid authored output symbol is absent from the produced result |
| 84 | `result_schema` | A produced result violates its own published schema |
| 85 | `materialization` | A materialized `.STEP`/`.TEMP` run disagrees with the plan that produced it |
| 124 | `timeout` | Run exceeded `--timeout` |
| 130 | `cancellation` | Interrupted (Ctrl-C) |

65-78 keep their `sysexits.h` meanings; 80-85 are an RSpice block for
engine-domain outcomes `sysexits` has no name for. No engine category exits 1,
so a `1` means only the two frontend failures listed above.

## CI integration

`--report-format junit` writes JUnit XML that CI systems ingest as test
results: one test suite per run, holding a `simulation` case and one case per
`.MEAS` statement, so a missed goal shows up as a named failing test rather than
a log line. `--report-format tap` reports the same content as TAP. The
`--summary` JSON carries the run's verdict, per-run status, the full
measurement records (the same fields as `--meas-file` JSON), the resource
limits in force and an `outputs` list naming only the artifacts this invocation
published.

A typical verification pipeline:

```bash
# 1. Validate the deck, treating connectivity warnings as failures (exit 2)
rspice check circuit.sp --connectivity --strict

# 2. Run with a time budget; failed .MEAS checks exit 3, non-finite results 80
rspice run circuit.sp -q -o results.csv -f csv --timeout 600 \
        --summary summary.json --report-format junit --report-file junit.xml

# 3. Compare against the golden reference (mismatches exit 3)
rspice compare results.csv golden.csv --abstol 1e-9

# 4. After a reviewed, intentional change: accept the new waveforms
rspice compare results.csv golden.csv --bless
```

Every failure category maps to a documented nonzero exit code, so a pipeline
step can gate on the exit status alone.

Licensed under the [RSpice Personal Use License](../../LICENSE).

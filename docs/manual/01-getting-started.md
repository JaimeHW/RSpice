# 1 · Getting started

## Build

```sh
cargo build --release -p rspice-cli      # the `rspice` CLI
cargo build --release -p rspice-ui       # the VOLTA desktop IDE
```

Binaries land in `target/release/`.

## A first deck

Save as `divider.cir`:

```spice
resistive divider with a step input
V1 in 0 DC 0 PULSE(0 5 1u 1n 1n 1m 2m)
R1 in out 4.7k
R2 out 0 4.7k
C1 out 0 1n
.tran 100n 3m
.meas tran vpeak MAX v(out)
.end
```

The first line of a deck is always its **title** (it is not parsed as a
component). The deck must end with `.end`.

## Run it

```sh
rspice run divider.cir --meas -o divider.csv -f csv
```

- The transient runs and prints `✓ Transient complete: N time points`.
- `--meas` prints the `.meas` results (`vpeak = 2.5…`).
- `-o`/`-f` write the waveforms (`raw`, `csv`, `json`, … — see
  [chapter 7](07-cli-reference.md)).

Other day-one commands:

```sh
rspice check divider.cir      # parse + structural checks, no simulation
rspice info  divider.cir      # deck summary: elements, nodes, analyses
```

## The VOLTA IDE

`rspice-ui` opens the desktop IDE: schematic capture, a text-first
netlist editor with live parameter tuning, simulation setup, and the
results workspace (waveforms, Bode, FFT, eye, histogram, Smith,
operating-point inspector, noise contributors, specs matrix). The IDE
runs the same engine as the CLI; anything in this manual about deck
syntax and analyses applies in both.

## Where to go next

- Writing decks: [chapter 2](02-netlists.md)
- Choosing analyses: [chapter 3](03-analyses.md)
- Sweeps, corners, and statistical runs: [chapter 4](04-multi-run.md)

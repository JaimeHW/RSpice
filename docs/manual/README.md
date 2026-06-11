# RSpice User Manual

RSpice is a high-performance SPICE-class analog/RF circuit simulator with
a command-line interface, a desktop IDE (VOLTA), Python bindings, and a
WebAssembly build.

| Chapter | Covers |
|---|---|
| [1 · Getting started](01-getting-started.md) | Install, first deck, first plot |
| [2 · Netlists](02-netlists.md) | Devices, sources, parameters, expressions, dot-commands |
| [3 · Analyses](03-analyses.md) | OP, DC, AC, transient, noise, and the advanced/RF set |
| [4 · Multi-run simulation](04-multi-run.md) | `.step`, `.alter`, `.data`, Monte Carlo, corners |
| [5 · Post-layout simulation](05-post-layout.md) | SPEF/DSPF parasitic ingestion |
| [6 · Measurements](06-measurements.md) | `.meas`, reports, the specs matrix |
| [7 · CLI reference](07-cli-reference.md) | Every `rspice` command and flag |
| [8 · Performance](08-performance.md) | Benchmarking, checkpoints, solver backends |

Conventions used throughout:

- Shell examples use `rspice` for the CLI binary (`target/release/rspice`
  in a source build).
- Netlist syntax is case-insensitive; the manual writes dot-commands in
  lower case and device letters in upper case for readability.
- Engineering suffixes follow SPICE semantics everywhere: `k` = 1e3,
  `meg` = 1e6, `m` = 1e-3, `u` = 1e-6, `n` = 1e-9, `p` = 1e-12,
  `f` = 1e-15. **`M` means milli, not mega.**

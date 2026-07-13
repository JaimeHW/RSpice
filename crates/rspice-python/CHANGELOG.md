# RSpice Python Changelog

All notable changes to the Python distribution are recorded here. Entries use
the categories from [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Release numbers remain tied to the workspace version.

## Unreleased

- Marked the package as private/Do Not Upload, documented that release
  artifacts are not published to PyPI, and aligned distribution language with
  the repository-wide RSpice Personal Use License.

### Added

- Complete third-order Volterra `.DISTO` analysis, including harmonic 2F1/3F1
  products, two-tone F1+F2/F1-F2/2F1-F2 products, complex peak phasors,
  relative voltage/current metrics, direct sweep APIs, deck execution, and
  cooperative cancellation.
- Complete complex AC `.SENS` analysis across flattened hierarchical devices,
  explicit instance/model/source and real-vector parameters, with device
  filters, differential-voltage and branch-current outputs, normalized,
  magnitude, phase, and dB derivatives, direct APIs, deck execution, and
  cooperative cancellation.
- Standards-equivalent `.SP donoise` analysis with a full complex Hermitian
  N-port Norton current-noise correlation matrix (`Cy`) and two-port `Rn`,
  matched `NF`, `NFmin`, and `Sopt`, available from deck execution and the
  direct S-parameter API.
- Typed results and direct APIs for AC data sweeps, N-port S-parameters, STB,
  pole-zero, PSS, HB, PAC, driven and oscillator noise, Monte Carlo,
  sensitivity, parameter stepping, Fourier analysis, transient compression,
  and checkpoint/resume.
- Cooperative `KeyboardInterrupt` handling across every exposed simulation
  family, including transfer-function, STB, and pole-zero solver boundaries.
- Thread-safe `Engine.cancel()`, `is_running`, and `active_run_count` controls
  with a dedicated `CancelledError` for application-managed cancellation and
  single-analysis progress reporting.
- Free-threaded CPython 3.14 support with dedicated `cp314t` wheels for Linux,
  macOS, and Windows on all shipped architectures.
- Strict runtime/type-stub parity checks, structured exception attributes,
  per-device operating-point data, and directive-level execution records.
- Locked, offline-verifiable source distributions and provenance attestations
  for wheel and source artifacts.

### Changed

- `RunReport.assert_passed()` now requires at least one executed measurement
  and rejects reports containing skipped directives.
- NumPy result arrays have explicit owned-copy semantics so their lifetime is
  independent of the Rust result object.
- Long simulations release the GIL and may share immutable Engine, Netlist,
  and result objects between Python threads.

### Fixed

- Corrected transient `TSTART` clipping, DC sweep validation and nested axes,
  percentile validation, zero/noise logarithms, pole stability and bandwidth,
  and multi-channel waveform compression error bounds.
- Corrected sensitivity percent-per-percent scaling and transfer-function UI
  source phase conversion while preserving RF-port, distortion, DC, and
  transient source annotations.
- Replaced the former `.SP donoise` rejection with the complete correlated
  port-noise solve; scalar `.NOISE` output is never substituted for `Cy`.
- Fixed a parallel XSPICE virtual-file test race and workspace-sdist lockfile
  reconciliation without permitting dependency upgrades.

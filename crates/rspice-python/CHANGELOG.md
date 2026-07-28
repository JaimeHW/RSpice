# RSpice Python Changelog

All notable changes to the Python distribution are recorded here. Entries use
the categories from [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Release numbers remain tied to the workspace version.

## Unreleased

### Added

- Result export: Touchstone v1 (`SParameterResult.to_touchstone` /
  `write_touchstone`, with `ri`/`ma`/`db` formats and Hz–GHz axes),
  ngspice-compatible SPICE raw files in ASCII and binary for transient, AC,
  and DC-sweep results (AC written with `Flags: complex`), and RFC 4180 CSV.
  A Touchstone export whose ports do not share one reference impedance is
  refused rather than written with a silently wrong `R`.
- Pickle and `copy.deepcopy` support for `Netlist`, every configuration class
  and enum, and every result type, so netlists and results can cross a
  process boundary with `multiprocessing`. A `Netlist` pickles by replaying
  its parse from the retained source text under the same `ResourceLimits`.
  A result carries the state behind everything its own accessors expose, so
  each readable quantity, and each quantity derived from one, is unchanged
  across a round trip; internal traces no accessor reaches are not carried.
- Pickle support for the periodic and RF results: `PssResult`, `HbResult`,
  `PacResult`, `CompressedTransientResult`, and `DistortionResult`. `PacResult`
  restores its per-sideband spectra and its conversion matrix independently,
  and `HbResult` retains the branch-current and reactive spectra that
  `is_valid` tests so the verdict cannot change across a round trip.
- Netlist introspection: an `Element` projection (name, kind, nodes, value,
  value expression, model, instance parameters) behind `Netlist.elements` and
  `Netlist.element()`, plus `Netlist.node_names`, `Netlist.parameters`,
  `Netlist.parameter()`, `Netlist.source`, and `Netlist.source_path`.
- `Netlist.with_parameters()` derives a new netlist with different `.PARAM`
  values by rewriting top-level assignments in place and re-parsing.
  Subcircuit-scoped definitions are deliberately left alone.
- `DcSweep` and `Engine.run_dc_sweep_spec()`: the general `.DC` form with
  linear, explicit-list, and logarithmic decade/octave axes, and nested
  two-source sweeps. `run_dc_sweep` remains the linear single-source
  shorthand.
- `.FOUR` and `TransientResult` now resolve branch currents and differential
  node pairs, matching the probe grammar the GUI already accepted. Adds
  `TransientResult.fourier_current()`, `TransientResult.signal()`, and a
  `reference=` argument to `fourier()`.
- `RSpiceKeyError`, `RSpiceIndexError`, `RSpiceValueError`, and
  `RSpiceTypeError`, each deriving from both `RSpiceError` and the builtin
  exception a caller already expects.
- `RunReport.all_op`, `all_dc`, `all_tran`, `all_ac`, and `all_noise` retain
  every result when a deck carries more than one directive of a kind.
- `MonteCarloResult.try_variable()`, `__getitem__`, and `__contains__`.
- Periodic operating-point reuse and continuation: `run_pss_operating_point()`
  produces a `PssOperatingPoint` that `run_pac(..., pss=)` and
  `run_pnoise(..., pss=)` reuse instead of repeating the shooting solve;
  `run_pss_continuation()` and `run_tran_from_pss()` start a transient from a
  converged orbit; `run_hb_envelope()` and `run_tran_from_hb_envelope()` do
  the same for harmonic balance. The envelope state carries the HB
  configuration and frozen-source list that produced it, so a mismatched
  continuation is rejected rather than silently accepted.
- Two-port stability and gain figures on `SParameterResult`: Rollett `K`,
  Edwards-Sinsky `mu` and `mu_prime`, the scattering determinant `delta`, an
  `unconditionally_stable` verdict, MAG/MSG/Mason-U/transducer-gain/reverse
  isolation in dB, and source and load `stability_circles()`. These are
  two-port quantities and read `None` for any other port count rather than
  being derived from a sub-matrix.
- `Engine.health_check()` and a typed `HealthReport`: a readiness probe that
  exercises the configured parser-to-solver path against a fixed in-memory
  circuit, with no filesystem or network I/O.
- A typed `ResourceLimits` policy shared by every `Netlist.parse*` entry point
  and `SimulationConfig`, with structured resource/requested/limit attributes
  on parsing and simulation errors.
- Deck execution of `.NOISE ... DATA=<table>` alongside the existing swept
  `.NOISE`, reported as its own `noise_data` directive record.
- Complete third-order Volterra `.DISTO` analysis, including harmonic 2F1/3F1
  products, two-tone F1+F2/F1-F2/2F1-F2 products, complex peak phasors,
  relative voltage/current metrics, direct sweep APIs, deck execution, and
  cooperative cancellation.
- Complete complex AC `.SENS` analysis across flattened hierarchical devices,
  explicit instance/model/source and real-vector parameters, with device
  filters, differential-voltage and branch-current outputs, normalized,
  magnitude, phase, and dB derivatives, direct APIs, deck execution, and
  cooperative cancellation.
- Complete netlist-wide DC `.SENS` analysis with nonlinear instance/model
  parameters, flattened hierarchy, differential-voltage and branch-current
  outputs, device/parameter filters, normalized derivatives, direct APIs,
  deck execution, and cooperative cancellation.
- Complete periodic/RF numerical controls in Python: PSS and oscillator-noise
  absolute tolerances, damping, period-update limits, integration-method
  overrides, and diagnostics; PAC operating-point tolerances; HB verbosity;
  and PSS harmonic phasors, magnitudes, phases, records, and THD.
- Standards-equivalent `.SP donoise` analysis with a full complex Hermitian
  N-port Norton current-noise correlation matrix (`Cy`) and two-port `Rn`,
  matched `NF`, `NFmin`, and `Sopt`, available from deck execution and the
  direct S-parameter API.
- Deck-driven single- and multi-tone `.HB` execution, a typed `RunReport.hb`
  result, per-tone direct API with optional source mapping, and complete HB
  numerical controls for collocation, damping, Krylov, mixing, and Jacobian
  selection.
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
- The cross-interface simulation-error contract on every `SimulationError`:
  stable `code` and `category` tags plus a conservative `retryable` flag,
  alongside the existing `kind`, `iterations`, and resource-limit attributes.
- Structured netlist diagnostics: `Netlist.startup_diagnostics` with typed
  `StartupDiagnostic`/`StartupDirectiveScope` entries, and
  `ParseError.unresolved_output_symbols` carrying typed
  `UnresolvedOutputSymbol` items, so automation never parses display text.
- Strict runtime/type-stub parity checks, per-device operating-point data,
  and directive-level execution records.
- Locked, offline-verifiable source distributions and provenance attestations
  for wheel and source artifacts.

### Changed

- `Engine.run()` records a directive that fails as `skipped=True` with its
  error text and continues with the remaining directives, which is what the
  documentation already described. `continue_on_error=False` restores the
  previous abort-and-raise behaviour.
- Lookup and argument failures now derive from `RSpiceError` in addition to
  their builtin base, so `except rspice.RSpiceError` covers every failure the
  library raises. `except KeyError` and friends keep working unchanged.
- `MonteCarloResult.get_variable()`, `mean()`, `std_dev()`, and `range()`
  raise `KeyError` for an unknown variable instead of returning `None`.
- `SimulationConfig`, `ConvergenceConfig`, `BypassConfig`, and
  `ResourceLimits` compare by value.
- Marked the package as private/Do Not Upload, documented that release
  artifacts are not published to PyPI, and aligned distribution language with
  the repository-wide RSpice Personal Use License.
- `RunReport.assert_passed()` now requires at least one executed measurement
  and rejects reports containing skipped directives.
- `SimulationConfig.max_timestep` defaults to unbounded, so a large
  `min_timestep` is no longer rejected against a built-in ceiling. Pass a
  finite `max_timestep` when the embedding application requires one.
- NumPy result arrays have explicit owned-copy semantics so their lifetime is
  independent of the Rust result object.
- Long simulations release the GIL and may share immutable Engine, Netlist,
  and result objects between Python threads.

### Fixed

- `DcSweepResult.result_at()` carries the per-device operating points that
  `sweep[i]` and `points()` already provided.
- `.four 1k I(V1)` and `.four 1k V(a,b)` are evaluated instead of failing
  with "unknown node", which previously made `assert_passed()` fail on decks
  those directives are valid in.
- `SimulationConfig.max_timestep` now accepts `float('inf')` in both the
  constructor and the setter. The getter already returned `inf` for the
  unbounded default, so the value could be read but never written back: a
  timestep ceiling was a one-way door and round-tripping a config through
  `Engine.config` raised `ValueError`.
- Normalized Windows extended-length paths before exposing source provenance
  through public exception and netlist-location attributes.
- Isolated the validated fixed-BDF2 compatibility coefficients to native
  BSIM4 transient-NQS charge-deficit states, preserving variable-step Gear2
  behavior for BSIM3, quasi-static BSIM4, and Xyce accepted-reference modes.
- Replaced the oscillator phase-noise solver's resistor/diode-only source
  subset with the complete device-noise model, including colored-source PPV
  averaging, correlated BSIM4 noise, resistor `NOISY` controls, and strict
  offset validation.
- Corrected transient `TSTART` clipping, DC sweep validation and nested axes,
  percentile validation, zero/noise logarithms, pole stability and bandwidth,
  and multi-channel waveform compression error bounds.
- Corrected sensitivity percent-per-percent scaling and transfer-function UI
  source phase conversion while preserving RF-port, distortion, DC, and
  transient source annotations.
- Corrected multi-tone HB collocation sizing so the FFT grid represents the
  highest common-basis harmonic and rejects truncated spectra explicitly.
- Replaced the former `.SP donoise` rejection with the complete correlated
  port-noise solve; scalar `.NOISE` output is never substituted for `Cy`.
- Fixed workspace-sdist lockfile reconciliation without permitting dependency
  upgrades.

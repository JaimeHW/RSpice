# rspice-engine-adapter

`rspice-engine-adapter` is the self-contained RSpice engine executor the
credentialless RSpice Cloud worker launches. It reads exactly one protocol-4
JSON request on standard input, solves it with `rspice-core`, writes the
declared result files under `results/`, and emits exactly one JSON response on
standard output. It opens no network connection, launches no subprocess, and
reads nothing outside the request's own manifested artifacts.

## Wire contract

| Element | Value |
| --- | --- |
| Protocol version | `4` (`RSPICE_ENGINE_PROTOCOL_VERSION=4`) |
| Request document schema | `rspice-circuit-v1` |
| Request digest version | `1` |
| Revision content digest version | `2` |
| Result manifest format | `rspice-result-v3` |
| Typed result document | `rspice-analysis-result` v2 (`rspice-core`) |
| Run-axis orchestration record | `rspice-axis-execution` v1 |

A request whose `protocol_version` is anything other than `4` is refused as
controller drift before any deck is parsed, and a launch environment that
declares a different protocol is refused before a request is read. Neither an
older nor a newer protocol is served a response its reader cannot interpret.

## Analysis kinds

The request selects one analysis family by wire name; the deck supplies the
directive parameters, so the request digest covers the complete simulation
definition. Every authored instance of the requested family runs at every
coordinate of the deck's `.STEP` and `.TEMP` axes.

Runnable kinds: `operating_point`, `dc_sweep`, `ac_small_signal`, `transient`,
`noise`, `distortion`, `transfer_function`, `stability`, `sensitivity`,
`pole_zero`, `monte_carlo`, `harmonic_balance`, `pss`, `pac`, `pnoise`,
`s_parameters`, `envelope`.

Refused kinds, each with the reason on the wire: `mixed_signal`, `port_noise`,
`fourier`, and `fft`. A mixed-signal deck is a transient and is requested as
`transient`. The other three are results a card produces beside its own rather
than analyses of their own: the canonical plan mints no analysis slot for
them, so no deck contains a directive such a request could select. They are
published — see below — by requesting the parent family.

The authoritative per-family declaration, including the exact reason for every
gap, is `rspice_core::execution::capability`'s engine-adapter column.

## Results

Every analysis publishes at least one
`rspice_core::execution::AnalysisResultDocument` as
`results/<analysis>.result.json`, or
`results/<coordinate>__<analysis>.result.json` when the deck has a run axis.
The document names the canonical analysis identity, the shared-deck
coordinate, the elaborated topology fingerprint, and the output and checkpoint
namespaces, and represents a sample that does not exist as `null` rather than
as a plausible number. The artifact's declared content type is derived from
that document's own schema identifier and version.

A card whose result is more than one document publishes them all, staged in
the same artifact transaction as its own, so a failure part-way through
publishes none of them. A child artifact's name is the parent's stem plus its
own namespace component:

* one `fft` document per authored `.FFT` card beside its parent transient,
  under the `fft-NNN` identity the canonical plan minted for that card —
  `results/tran-001.fft-001.result.json`. The transient's own document lists
  every one of them by identity and probed column;
* one Fourier document per authored `.FOUR` operand beside its parent
  transient, under the `four-NNN` identity the canonical plan minted for that
  operand — `results/tran-001.four-001.result.json`;
* the port-noise sweep beside the scattering sweep of a `.SP DONOISE` card.
  Port noise is that card's second result and carries the card's own analysis
  identity, so its result family is the component that separates the two —
  `results/sp-001.port-noise.result.json`.

A deck with a run axis also carries an `rspice-axis-execution` record inside
the result manifest. It is an orchestration record, not a second result
schema: it names each coordinate, its axis assignments, and the typed
documents each analysis published, by path, content type, schema, schema
version, and result family.

## Migrating from protocol 3

* `analysis.kind` `mixed_signal` is gone; request `transient`.
* The per-analysis CSV artifact and the `rspice-analog-result-v1` document are
  replaced by one `rspice-analysis-result` document.
* Result files are named from the canonical namespace components
  (`op-001.result.json`, `run-<id>__tran-001.result.json`) rather than from the
  request kind and a one-based ordinal, and a second document a card publishes
  extends that stem with its own component
  (`tran-001.fft-001.result.json`, `tran-001.four-001.result.json`,
  `sp-001.port-noise.result.json`).
* every published document is an `rspice-analysis-result`: the adapter defines
  no result schema of its own, so there is no separate transient-FFT bundle and
  no `result_manifest.typed_fft_result_schema` entry to read.
* `result_manifest.format` is `rspice-result-v3` for every run, with or without
  a run axis.
* Manifest measurements are projected generically from the typed document, so
  their names are `axis:<name>`, `signal:<canonical>`, and `scalar:<name>`, and
  a complex series contributes `.re` and `.im` entries instead of a magnitude
  and a phase.
* The axis record's `artifacts` entries are objects describing each document
  rather than bare path strings.

## Resource contract

Result shape is preflighted before any solver work: the planned
coordinate/analysis product is checked against the artifact ceiling, and each
document's own retained value count is checked against the remaining byte
budget before it is serialized. An oversized family stops with
`resource.result_set_bytes`, not an allocation failure. Every result file is
staged in memory and committed as one transaction, so a failed or cancelled
run leaves either the previous complete set or none of it.

Two things stop engine work, and the response says which: the worker's
termination request produces `engine.cancelled`, and the exhausted solve budget
produces `engine.time_limit`. The budget is a launch input
(`RSPICE_ENGINE_SOLVE_BUDGET_SECONDS`); a malformed value is a launch-contract
violation rather than a silent fall back to the default.

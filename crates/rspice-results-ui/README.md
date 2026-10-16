# rspice-results-ui

The egui result viewers and the display sessions behind them: waveform
stacks, tables and readouts, the frequency-domain, spectrum, RF, statistical,
digital and verification viewers, and the result-document and report forms.
A session owns derived display caches and viewer preferences only. The host
selects the retained source, decides whether it is current, and coordinates
simulation, persistence and navigation; measurements, transforms and
statistics keep their owners in `rspice-results` and `rspice-core`. The crate
stands apart so that boundary can be enforced:
`tools/ci/check_app_crate_dependencies.py` allows it only `rspice-app-types`,
`rspice-results` and `rspice-ui-kit` among the application crates, and
refuses `rspice-app`, `rspice-project`, `rspice-project-contract`,
`rspice-simulation`, `rspice-worker`, eframe, winit, wgpu and rfd in its
closure.

## What's in here

| Module | Contents |
| :--- | :--- |
| `session` | `ResultViewerState`: the device-local viewer state (active viewer, per-document page and pane selection, cursor, marker and plot tools, per-sheet controls); it is not part of any result document |
| `selection` | Stable selection keys over exact retained result identities |
| `waves` | `StripModel` and `build_models`: the waveform projection that plots, tables and readouts share; the waveform stack, unit panes, cursor readouts, markers, expression editor and projection cache |
| `waveform`, `derived` | Display choices and bounded drawing caches over retained waveform samples; derived display columns and window statistics |
| `chrome`, `strip`, `presentation` | Results tab strips, viewer picker, instrument controls and export menu; the strip header every plot wears; shared plot navigation, readout cards and notes |
| `table`, `virtual_rows` | The retained samples of one analysis as rows, and viewport-bounded row plans for the evidence tables |
| `operational`, `manifest` | Operational status over host-classified evidence; dataset manifest tables and provenance readouts |
| `create_document`, `document_dialogs`, `report` | The Create Result Document form; the forms opened over a result document (add pane, compare, family, measurement, properties); the report composer, inspector, preview and dialogs over a borrowed `ReportDocument` |
| `bode`, `nyquist`, `pole_zero`, `transfer_function` | Frequency-response, Nyquist, pole-zero and transfer-function viewers and inspectors |
| `fft`, `harmonic_balance`, `phase_noise`, `noise_contrib`, `qpnoise` | The FFT viewer; retained coefficient spectra for HB, Fourier, recorded FFT and QPSS; phase noise; noise contributors and noise figure; QPNOISE integrals and rankings |
| `smith_chart`, `polar`, `network_matrix` | Smith chart and polar views of retained S-parameter and complex responses; network-matrix cells |
| `eye_diagram`, `events` | Eye diagrams over `rspice-core` waveform folding and `rspice-results` mask compliance; digital and real event tables and bus controls |
| `histogram`, `scatter`, `box_violin`, `sensitivity`, `optimization` | Monte Carlo histograms, scatter correlation with trial brushing, and box and violin plots against requirements; sensitivity and mismatch contribution sheets; optimization convergence |
| `op_inspector`, `specs`, `soa` | Operating-point cards and tables; the specification result table and inspector; SOA rule projections and stress history |

## Boundaries

- Result data, calculations, and the visualization and report documents
  belong to [`rspice-results`](../rspice-results).
- Binding viewers to the project's run and dataset selection and committing
  result-document edits belong to [`rspice-app`](../rspice-app), in
  `workbench/documents/result_document/`.
- Plot painting primitives, design tokens and `CommandForm` belong to
  [`rspice-ui-kit`](../rspice-ui-kit).

## Entry points

- `session::ResultViewerState`, the viewer state a host keeps per results
  surface.
- `waves::build_models` for the `StripModel`s of the displayed runs, with
  `waves::cache::ModelsCache` to bound rebuilds.
- Per-viewer state the host keeps, such as `fft::FftState`,
  `bode::BodePlotState`, `nyquist::NyquistState`,
  `smith_chart::SmithChartState` and `histogram::HistogramState`.
- A viewer's `show`, which paints the document well, and its `right_panel`,
  which paints the inspector: `fft::view`, `nyquist::view`,
  `smith_chart::view`, `histogram::view`, `eye_diagram::view`,
  `op_inspector` and `specs`.
- `chrome::viewer_tab` and `chrome::viewer_picker` for the viewer strip.
- `create_document::show` with a `CreateDocumentHost`, which returns a create
  request the host revalidates and commits.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `test-support` | Exposes viewer fixtures and cache observations to integration tests outside this crate. `rspice-app` enables it as a dev-dependency |

## Testing

```bash
cargo test -p rspice-results-ui
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

# rspice-formats

Portable engineering-data codecs for the RSpice application: bounded decoders
for waveform, digital and tabular result files, encoders for selected tables
and retained results, the native RSpice result and dataset bundles, and the
versioned result history a project file persists. Codecs over retained
results or the engine, and those with large third-party dependencies (Arrow and
Parquet, HDF5, FST, XLSX), sit behind their own features, so a consumer links
only the codecs it uses. Without optional features, and with only
`native-bundle` or only `table-schema`, the crate reaches no simulator
package; `tools/ci/check_app_crate_dependencies.py` checks those three builds,
allows the crate only `rspice-app-types`, `rspice-results` and
`rspice-model-library` among the application crates, and refuses egui,
eframe, winit, wgpu and rfd in its closure.

## What's in here

| Module | Contents |
| :--- | :--- |
| `waveform_io` | `WaveformDataset` and `WaveformWriter` for CSV, TSV and Touchstone text; the bounded Touchstone v1/v2 reader. With `result-waveform`, `waveform_io::result` projects selected retained waveforms into export datasets |
| `delimited`, `csv_sheet` | Strict CSV/TSV waveform decoding with canonical engineering units; bounded CSV text sheets that keep physical source-line identities |
| `psf` | PSF ASCII declarations and finite numeric columns |
| `spice_raw` | Waveform interpretation over `rspice-core`'s bounded SPICE RAW reader |
| `matlab` | A MAT-file Level 5 writer built to MathWorks' published layout, and a bounded Level 5 reader. MATLAB 7.3 files are HDF5 and are read by `hdf5` |
| `numpy` | NPY encoding of real and complex arrays, NPZ archives, and bounded NPY decoding |
| `hdf5` | HDF5 and MATLAB 7.3 decoding; with `result-hdf5`, HDF5 export through the writer `rspice-core` shares with the CLI |
| `columnar` | Arrow IPC and Parquet decoding of numeric results, and Parquet encoding of selected tables |
| `fst`, `digital`, `vcd` | Bounded FST framing and event decoding; VCD and FST import into retained event evidence with a sampled projection; VCD encoding of retained digital and real events through the projection and writer the CLI also uses |
| `numeric` | Decoded numeric columns, nullable samples, and exact integer-to-`f64` conversion bounded by `MAX_EXACT_F64_INTEGER` |
| `table`, `table_schema`, `xlsx` | `EngineeringTableSource` and delimited encoding of an already-selected table, its versioned schema JSON, and XLSX encoding |
| `result_csv` | CSV and exact text encoders over selected retained records: typed results, spectra, histograms, eye measurements, specifications, optimization, correlation, network matrices, and noise and operating-point reports |
| `result_import` | `identify_result_import_format`: identification by extension and file signature, refusing an extension the signature contradicts |
| `native_bundle`, `zip` | The native result and dataset bundles; deterministic stored ZIP32 encoding |
| `monte_carlo_checkpoint` | Bounded portable Monte Carlo checkpoint files |
| `project_results` | `ProjectSimulationResults`: the retained run history, provenance and executed decks a project file persists, versioned by the private `PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION`, with a separate validator for each earlier schema so an older file is checked against the fields it actually sealed |

## Boundaries

- Retained result values and their validation belong to
  [`rspice-results`](../rspice-results).
- The project file that embeds `project_results` belongs to
  [`rspice-project`](../rspice-project).
- The engine's RAW reader and HDF5 writer belong to
  [`rspice-core`](../rspice-core); this crate interprets and projects through
  them.
- Filenames, view selection and publication belong to
  [`rspice-app`](../rspice-app).

## Entry points

- `result_import::identify_result_import_format(source_name, bytes)`, then the
  matching decoder: `delimited::decode_delimited_waveforms`,
  `psf::decode_psf_ascii`, `spice_raw::decode_spice_raw`, `hdf5::decode_hdf5`,
  `columnar::decode_parquet`, `digital::decode_vcd` or
  `native_bundle::decode_native_bundle`. Every decoder takes explicit limits.
- `read_touchstone_bytes`, and
  `WaveformWriter::new(format).write_text(&dataset)`.
- `native_bundle::encode_native_bundle(kind, &dataset, max_bytes)`.
- `table::encode_delimited_table`, `xlsx::encode_xlsx_table` and
  `columnar::encode_parquet_table` for a selected table.
- `project_results::ProjectSimulationResults` for retained history.

## File contracts

| Element | Contract |
| :--- | :--- |
| Native bundle container | A ZIP holding `manifest.json` and `dataset.json`, written stored and deterministic. Reading refuses duplicate, directory, absolute and parent-relative members and bounds member count and expanded size by `NativeBundleReadLimits` |
| Native bundle kinds | `NativeBundleKind::Result`: `.rspiceresult`, `application/vnd.rspice.result+zip`, manifest schema `rspice-result-bundle/1`. `NativeBundleKind::Dataset`: `.rspicedata`, `application/vnd.rspice.dataset+zip`, `rspice-dataset-bundle/1` |
| Native bundle manifest | `schema`, `dataset_member` (always `dataset.json`) and `dataset_sha256`, the hex SHA-256 of the exact `dataset.json` bytes |
| Native bundle dataset | `rspice-waveform-dataset/1`; `/2` when the coordinate carries a unit; `/3` when any sample is unavailable. One analysis domain, a named coordinate and real or complex signals of the coordinate's length |
| Monte Carlo checkpoint | JSON with `format` `rspice.monte-carlo-checkpoint`, `version` 1 and the checkpoint evidence; unknown fields are refused and input is bounded by `monte_carlo_checkpoint::size_limit` |

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `project-results` | Builds `project_results`, pulling in `rspice-results`, `rspice-core`, `rspice-design-model`, `rspice-model-library` and `rspice-app-types`. Enabled by `rspice-project`, `rspice-hardcopy`, `rspice-cli` and `rspice-app` |
| `result-validation-observation` | Implies `project-results` and exposes `project_results::RESULT_VALIDATIONS`, a per-thread count of result validations. Enabled by `rspice-app`'s dev-dependency |
| `table-schema` | Builds `table_schema` |
| `result-csv` | Builds `result_csv` |
| `result-digital` | Builds `digital` |
| `result-vcd` | Builds `vcd` |
| `result-waveform` | Builds the retained-waveform projections in `waveform_io`, `matlab`, `numpy` and `native_bundle`; with `fst` also builds `result_import` |
| `result-hdf5` | Implies `hdf5` and `result-waveform`, and adds HDF5 export through `rspice-core` |
| `native-bundle` | Builds `native_bundle` |
| `monte-carlo-checkpoint` | Builds `monte_carlo_checkpoint` |
| `columnar` | Builds `columnar` (Arrow and Parquet) |
| `fst` | Adds FST decoding to `fst` and `digital` |
| `hdf5` | Builds `hdf5` decoding |
| `spice-raw` | Builds `spice_raw` |
| `xlsx` | Builds `xlsx` |

## Testing

```bash
cargo test -p rspice-formats --all-features

# CI also tests the codec features one at a time, which catches a codec that
# compiles only alongside another flag
cargo test -p rspice-formats --no-default-features --features result-digital,fst --lib
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

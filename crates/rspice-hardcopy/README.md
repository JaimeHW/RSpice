# rspice-hardcopy

Headless hardcopy for the RSpice application. It resolves schematics, symbols,
plots, result views, reports and multi-document source sets into frozen,
authenticated semantic sources with a physical extent, and renders a
`HardcopyPlan` from `rspice-hardcopy-contract` into PDF, PDF/A-2b, SVG, PNG,
TIFF, printer rasters or an HTML print document, independent of egui,
filesystems, browsers and printer APIs. It is its own crate because the
renderer brings krilla, resvg, usvg, tiff and embedded fonts that the
persisted contract does not need; `rspice-app` links it directly and the
browser `rspice-worker` runs it through the `worker` protocol.
`tools/ci/check_app_crate_dependencies.py` allows it only `rspice-app-types`,
`rspice-hardcopy-contract`, `rspice-design`, `rspice-project-contract`,
`rspice-results` and `rspice-formats` among the application crates, and
refuses egui, eframe, winit, wgpu and rfd in its closure.

## What's in here

- **`sources`**: the document adapters. Each resolves one kind of document
  into a `ResolvedHardcopyDocument`: its `ActiveHardcopySource` authority, a
  `HardcopySemanticDocument` (schematic, symbol, plot, result summary, report
  or aggregate), its bounds and its natural content extent in micrometres.
  `PreparedRetainedHardcopyResolution` captures the inputs a worker needs and
  round-trips them as a bounded snapshot.
- **`render`**: `HardcopyRenderer`. The scene is validated before a byte is
  written: every primitive lies inside the page's content extent and every
  glyph is covered by the embedded IBM Plex faces from
  [`assets/fonts/`](../../assets/fonts); a page
  that fails is refused rather than emitted clipped. PDF output is written with
  krilla, and PDF/A-2b is returned only after krilla's archival validator
  accepts the complete document. Publication timestamps are explicit, never
  the wall clock, so output is reproducible.
- **`worker`**: the browser-worker protocol, versioned by
  `HARDCOPY_WORKER_PROTOCOL_VERSION`, for source resolution, preview and
  publication. A response is bound to the plan digest and the part count it
  was issued for; per-page SVG and PNG parts can be packaged as one
  deterministic stored ZIP.

## Boundaries

- Page setup, plans, receipts, mapping catalogs and source sets belong to
  [`rspice-hardcopy-contract`](../rspice-hardcopy-contract).
- The Page Setup and print dialogs, platform printing and file publication
  belong to [`rspice-app`](../rspice-app).
- Report and visualization documents, their frozen figure artifacts and the
  Cartesian line scenes plots are drawn from belong to
  [`rspice-results`](../rspice-results); this crate authenticates and places
  them.

## Entry points

- `sources::resolve_schematic_source`, `resolve_all_schematic_sheets`,
  `resolve_symbol_document`, `resolve_plot_source`,
  `resolve_visualization_document_source`, `resolve_report_source` and
  `resolve_hardcopy_source_set_with`, each returning a
  `ResolvedHardcopyDocument`.
- `ResolvedHardcopyDocument::content_extent_for_setup`, then
  `HardcopyPlan::compile(source.authority().clone(), setup, extent)`.
- `render::HardcopyRenderer::render_resolved(&plan, &source, metadata)` for a
  `RenderedHardcopyPublication`; `render_preview_pages_resolved` for dialog
  previews; `render_printer_pages_resolved` for native printer rasters (not
  built for `wasm32`).
- `worker::execute_request(request, buffers)` on the worker side, and
  `worker::decode_packaged_publication` on the requesting side.

## Testing

```bash
cargo test -p rspice-hardcopy
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

# rspice-ui-kit

The RSpice design system and shared egui widget layer: design tokens, the dark
and light palettes, the embedded fonts and vector icons, panel and form
primitives, dialogs, notices and the strip-plot engine. Widgets take display
data in and report interaction outcomes out; none reads application state,
owns a command or decides what a feature offers. It is a crate of its own so
that `rspice-app`, `rspice-schematic-editor` and `rspice-results-ui` paint from
one token set, and so the boundary can be enforced:
`tools/ci/check_app_crate_dependencies.py` refuses eframe, winit, wgpu, rfd,
every simulator package and every application crate in its closure except
`rspice-app-types`, `rspice-results` and the `rspice-units` beneath it, which
`plot` needs.

## What's in here

| Module | Contents |
| :--- | :--- |
| `tokens` | `Tokens`, the single source of the visual system: the active palette plus the spacing, type, control and radius scales for one `Mode` × `Density`, installed into the egui context and read back with `Tokens::get` |
| `theme` | `Theme`, the persisted visual choice (mode, density, color-blind-safe traces, canvas contrast, `EngineeringCanvasTheme`); `Theme::apply` maps the tokens onto egui's style and publishes them; focus-ring and font helpers |
| `palette` | The exact dark and light palettes, `INSTRUMENT_DARK` and `INSTRUMENT_LIGHT` |
| `fonts` | IBM Plex Sans and Mono compiled in from `assets/fonts/`, with the medium and semibold weights registered as named families |
| `icons` | `Icon`, vector icons painted as polylines in the caller's color, each with an accessible label |
| `panels` | The panel grammar: body inset and rhythm, section headers, the `property_row` family, cards, notes, empty states and `WorkbenchIcon`. `rspice-app` imports it as `workbench::design_system` |
| `widgets` | `Button`, `Dialog`, `CommandForm`, `Toasts` and notices, `select`, `segmented`, `view_switch`, tree rows, chips, switches, two-pane chrome, status and problem marks, and the presentation shells for schematic placement and selected-object commands |
| `plot` | The strip-plot engine: linear and log axes, dual Y axes, per-pixel-column min/max trace decimation cached per view, reference and limit lines, bands, markers, hover readout and A/B cursors. Requires `plot` |
| `input` | Ordered handover of pending keyboard input when a command moves focus to another surface |
| `viewport` | The root viewport rectangle, captured before chrome is laid out, for viewport-scoped breakpoints |
| `accessibility` | Count and plural helpers for UI copy and assistive-technology text |
| `raster` | A software rasterizer that renders a headless egui pass to pixels, so tests assert on painted geometry. Requires `test-support` outside this crate's own tests |

## Boundaries

- Application chrome, commands, routing, state and the eframe host belong to
  `rspice-app`.
- Result viewers belong to `rspice-results-ui`. Axis scales, number
  formatting and sweep sampling belong to `rspice-results`; `plot` re-exports
  them as `plot::scale`, `plot::format` and `plot::sample`.
- Schematic painting and editor interaction belong to
  `rspice-schematic-editor`. The command shells here are presentation only;
  the caller keeps document authority and every mutation.

## Entry points

- `Theme::apply(ctx)`, which also installs the fonts on first use, and
  `Tokens::get(ctx)` inside a custom widget.
- `widgets::Dialog::new(title, primary)` with its builder methods, then
  `show(ctx, body)` for a `DialogChoice`.
- `widgets::CommandForm::new(title, primary)` with `FormRows` for a compact
  command form.
- `panels::panel_body`, `panels::section_header` and `panels::property_row`.
- `icons::Icon::paint`.
- `plot::show` with a `PlotSpec` and a `DecimationCache`.
- `viewport::capture_root_viewport` before the shell lays out chrome, and
  `viewport::root_viewport_rect` wherever a breakpoint is viewport-scoped.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `plot` | Compiles the `plot` module and depends on `rspice-app-types` and `rspice-results`, so plots share the results crate's exact axis and sampling contracts. `rspice-app` and `rspice-results-ui` enable it |
| `test-support` | Exposes `raster` and, off `wasm32`, `widgets::painted_runs`, which records every text run a headless pass painted with its clip rectangle. Enabled by the dev-dependencies of `rspice-app` and `rspice-schematic-editor` |

## Testing

```bash
cargo test -p rspice-ui-kit --features plot

# Writes the status-grammar review PNGs to RSPICE_RASTER_DIR (default: the
# system temp directory); nothing in it asserts.
cargo test -p rspice-ui-kit --lib render_the_status_grammar_for_review -- --ignored
```

`plot` is off by default, so its tests compile only with the feature.

Licensed under the [RSpice Personal Use License](../../LICENSE).

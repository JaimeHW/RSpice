# RSpice App-Icon Export Set

Rasterized from the final brand SVGs in `design/brand/` (viewBox 0 0 96 96) by
`design/tools/export_icons.py`. Headless Chrome renders **one master per
distinct cut** (512 standard, 512 dark, 32 dot-tip); the smaller standard sizes
are **derived by Lanczos-downscaling the 512 master**, not screenshotted
individually. Sources are unmodified by the script.

Why derive instead of render each size: Chrome's `--screenshot` capture races
the SVG paint on large canvases and once silently shipped a horizontally
shifted, right-clipped `icon-256.png` (the running-app taskbar icon) and a
near-blank `icon-128.png` — both correctly *sized*, so the old size-only check
missed it. Every frame is now validated for opaque coverage and a centred
bounding box, and Chrome renders are retried until they pass. Pass
`--reuse-masters` to rebuild the derived set + ICO from the on-disk 512/32/16
masters with no Chrome — a deterministic repair if a render ever regresses.

The mark is the June 2026 terminal recut: open-port ring terminals with all
three pins equal at 5.5 grid units (standard cut), dot-tip terminals on the
heavy stroke for the 24–32 px band, and the original bare heavy cut at 16 px.

## PNG provenance

| File | Source SVG | Size | Notes |
| --- | --- | --- | --- |
| `icon-512.png` | `run-icon.svg` (master) | 512x512 | standard cut (yellow tile, port rings) |
| `icon-256.png` | ↓ `icon-512.png` | 256x256 | standard cut, derived |
| `icon-128.png` | ↓ `icon-512.png` | 128x128 | standard cut, derived |
| `icon-64.png` | ↓ `icon-512.png` | 64x64 | standard cut, derived |
| `icon-48.png` | ↓ `icon-512.png` | 48x48 | standard cut, derived — smallest size where ring holes stay open (~2.8 px) |
| `icon-32.png` | `run-icon-32.svg` (master) | 32x32 | **dot-tip cut** — filled terminals on the 10.5 stroke; rings would smear shut here |
| `icon-16.png` | `run-icon-16.svg` | 16x16 | **dedicated small cut, unchanged from the original set** — no terminals, heavier strokes, wider input spacing; reused on disk, not re-rendered |
| `icon-dark-512.png` | `run-icon-dark.svg` (master) | 512x512 | dark bezel tile, yellow ringed mark |
| `icon-dark-256.png` | ↓ `icon-dark-512.png` | 256x256 | dark bezel tile, derived |
| `og-card.png` | `og-card.html` | 1200x630 | social card, ringed mark at 168 px tile |

## rspice.ico

Multi-size Windows icon containing 256, 128, 64, 48, 32, and 16 px entries
(PNG-compressed frames, largest first), assembled by the same script from the
PNGs above — the 256/128/64/48 entries are the derived standard cut, the 32 px
entry the dot-tip cut, the 16 px entry the original small cut. This is the icon
embedded in the `.exe` (`crates/rspice-ui/build.rs`, Explorer/pinned shortcuts);
the running app's taskbar/alt-tab icon is `icon-256.png`, decoded at startup by
`crates/rspice-ui/src/main.rs::load_window_icon`. Re-run
`design/tools/export_icons.py` after any change to the brand SVGs or the OG
card; the `.exe` must be rebuilt for an icon change to take effect.

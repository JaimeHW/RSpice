# RSpice App-Icon Export Set

Rasterized from the final brand SVGs in `design/brand/` (viewBox 0 0 96 96) by
`design/tools/export_icons.py` — headless Chrome at exact target pixel sizes,
transparent rounded corners preserved. Sources are unmodified by the script.

The mark is the June 2026 terminal recut: open-port ring terminals with all
three pins equal at 5.5 grid units (standard cut), dot-tip terminals on the
heavy stroke for the 24–32 px band, and the original bare heavy cut at 16 px.

## PNG provenance

| File | Source SVG | Size | Notes |
| --- | --- | --- | --- |
| `icon-512.png` | `run-icon.svg` | 512x512 | standard cut (yellow tile, port rings) |
| `icon-256.png` | `run-icon.svg` | 256x256 | standard cut |
| `icon-128.png` | `run-icon.svg` | 128x128 | standard cut |
| `icon-64.png` | `run-icon.svg` | 64x64 | standard cut |
| `icon-48.png` | `run-icon.svg` | 48x48 | standard cut — smallest size where ring holes stay open (~2.8 px) |
| `icon-32.png` | `run-icon-32.svg` | 32x32 | **dot-tip cut** — filled terminals on the 10.5 stroke; rings would smear shut here |
| `icon-16.png` | `run-icon-16.svg` | 16x16 | **dedicated small cut, unchanged from the original set** — no terminals, heavier strokes, wider input spacing |
| `icon-dark-512.png` | `run-icon-dark.svg` | 512x512 | dark bezel tile, yellow ringed mark |
| `icon-dark-256.png` | `run-icon-dark.svg` | 256x256 | dark bezel tile, yellow ringed mark |
| `og-card.png` | `og-card.html` | 1200x630 | social card, ringed mark at 168 px tile |

## rspice.ico

Multi-size Windows icon containing 256, 128, 64, 48, 32, and 16 px entries
(PNG-compressed frames, largest first), assembled by the same script from the
PNGs above — the 32 px entry uses the dot-tip cut, the 16 px entry the
original small cut. Re-run `design/tools/export_icons.py` after any change to
the brand SVGs or the OG card.
